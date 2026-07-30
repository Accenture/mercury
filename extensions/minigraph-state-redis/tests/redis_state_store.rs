//
// Copyright 2018-2026 Accenture Technology
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Contract tests for the Redis state store, exercised THROUGH the event
//! system so the whole path is real: preload auto-registration, MsgPack
//! transit of the persistence envelope, SETEX with native expiry, and atomic
//! GETDEL consumption — the Rust twin of the Java `RedisStateStoreTest`.
//!
//! The Java suite runs against an embedded redis-server binary; this
//! environment has no Redis binary or Docker daemon, so the suite listens
//! with an in-process **RESP2 test double** (real TCP, real protocol frames
//! through the real `redis` client — only the server side is simulated:
//! SETEX/GETDEL/GET/TTL plus handshake commands). Point `REDIS_HOST`/
//! `REDIS_PORT` style config at a real server and the same functions run
//! unchanged — the double stands in for redis-server, not for the client.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use platform_core::{
    main_application, overrides, AppError, AutoStart, EntryPoint, EventEnvelope, Platform,
    PostOffice,
};
use rmpv::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// Linking the store crate carries its preload inventory into this test
// binary — the "include the crate and the functions self-register" story.
use minigraph_state_redis::{PERSIST_ROUTE, RETRIEVE_ROUTE};

const TIMEOUT: Duration = Duration::from_secs(8);
const KEY_PREFIX: &str = "graph:state:";

#[main_application]
struct RedisStoreTestApp;

#[async_trait]
impl EntryPoint for RedisStoreTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

// ---- the RESP2 test double ----

#[derive(Clone)]
struct StoredValue {
    value: Vec<u8>,
    expires_at: Option<Instant>,
}

type SharedStore = Arc<Mutex<HashMap<Vec<u8>, StoredValue>>>;

/// Bind an ephemeral port and serve just enough RESP2 for the store
/// functions: SETEX, GETDEL, GET, TTL, DEL, PING and tolerant handshake
/// replies (CLIENT SETINFO etc.). Values are binary-safe; expiry is honored
/// on read like a real server.
async fn start_resp_double() -> (u16, SharedStore) {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().expect("addr").port();
    let store: SharedStore = Arc::new(Mutex::new(HashMap::new()));
    let shared = store.clone();
    tokio::spawn(async move {
        loop {
            let Ok((socket, _)) = listener.accept().await else {
                return;
            };
            let store = shared.clone();
            tokio::spawn(async move { serve_connection(socket, store).await });
        }
    });
    (port, store)
}

async fn serve_connection(mut socket: tokio::net::TcpStream, store: SharedStore) {
    let mut buffer: Vec<u8> = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        // parse as many complete commands as the buffer holds
        while let Some((args, consumed)) = parse_resp_array(&buffer) {
            buffer.drain(..consumed);
            let reply = dispatch(&args, &store);
            if socket.write_all(&reply).await.is_err() {
                return;
            }
        }
        match socket.read(&mut chunk).await {
            Ok(0) | Err(_) => return,
            Ok(n) => buffer.extend_from_slice(&chunk[..n]),
        }
    }
}

/// Parse one RESP2 array of bulk strings; None when incomplete.
fn parse_resp_array(buffer: &[u8]) -> Option<(Vec<Vec<u8>>, usize)> {
    let mut pos = 0;
    let (count, next) = parse_prefixed_int(buffer, pos, b'*')?;
    pos = next;
    let mut args = Vec::with_capacity(count.max(0) as usize);
    for _ in 0..count {
        let (len, next) = parse_prefixed_int(buffer, pos, b'$')?;
        pos = next;
        let end = pos + len as usize;
        if buffer.len() < end + 2 {
            return None;
        }
        args.push(buffer[pos..end].to_vec());
        pos = end + 2; // skip trailing \r\n
    }
    Some((args, pos))
}

fn parse_prefixed_int(buffer: &[u8], pos: usize, prefix: u8) -> Option<(i64, usize)> {
    if buffer.len() <= pos || buffer[pos] != prefix {
        return None;
    }
    let line_end = buffer[pos..].windows(2).position(|w| w == b"\r\n")? + pos;
    let text = std::str::from_utf8(&buffer[pos + 1..line_end]).ok()?;
    Some((text.parse().ok()?, line_end + 2))
}

fn dispatch(args: &[Vec<u8>], store: &SharedStore) -> Vec<u8> {
    let command = args
        .first()
        .map(|c| String::from_utf8_lossy(c).to_ascii_uppercase())
        .unwrap_or_default();
    let mut map = store.lock().expect("store");
    match command.as_str() {
        "PING" => b"+PONG\r\n".to_vec(),
        // handshake chatter the client may send (CLIENT SETINFO, SELECT 0...)
        "CLIENT" | "SELECT" | "AUTH" => b"+OK\r\n".to_vec(),
        "SETEX" if args.len() == 4 => {
            let seconds: u64 = String::from_utf8_lossy(&args[2]).parse().unwrap_or(0);
            map.insert(
                args[1].clone(),
                StoredValue {
                    value: args[3].clone(),
                    expires_at: Some(Instant::now() + Duration::from_secs(seconds)),
                },
            );
            b"+OK\r\n".to_vec()
        }
        "GETDEL" | "GET" if args.len() == 2 => {
            let live = match map.get(&args[1]) {
                Some(entry) if !expired(entry) => Some(entry.value.clone()),
                Some(_) => {
                    map.remove(&args[1]);
                    None
                }
                None => None,
            };
            match live {
                Some(value) => {
                    if command == "GETDEL" {
                        map.remove(&args[1]);
                    }
                    let mut reply = format!("${}\r\n", value.len()).into_bytes();
                    reply.extend_from_slice(&value);
                    reply.extend_from_slice(b"\r\n");
                    reply
                }
                None => b"$-1\r\n".to_vec(),
            }
        }
        "TTL" if args.len() == 2 => match map.get(&args[1]) {
            Some(entry) if !expired(entry) => {
                let remaining = entry
                    .expires_at
                    .map(|at| at.saturating_duration_since(Instant::now()).as_secs() as i64)
                    .unwrap_or(-1);
                format!(":{remaining}\r\n").into_bytes()
            }
            _ => b":-2\r\n".to_vec(),
        },
        "DEL" => {
            let mut removed = 0;
            for key in &args[1..] {
                if map.remove(key).is_some() {
                    removed += 1;
                }
            }
            format!(":{removed}\r\n").into_bytes()
        }
        _ => format!("-ERR unknown command '{command}'\r\n").into_bytes(),
    }
}

fn expired(entry: &StoredValue) -> bool {
    entry
        .expires_at
        .map(|at| Instant::now() >= at)
        .unwrap_or(false)
}

// ---- helpers ----

fn sample_envelope(cid: &str, ttl_seconds: i64) -> Value {
    let model = Value::Map(vec![
        (Value::from("amount"), Value::from(42)),
        (Value::from("binary"), Value::from(vec![1u8, 2, 3])),
        (
            Value::from("nested"),
            Value::Map(vec![(Value::from("stage"), Value::from("approval"))]),
        ),
    ]);
    Value::Map(vec![
        (Value::from("cid"), Value::from(cid)),
        (Value::from("node"), Value::from("step-1")),
        (Value::from("ttl"), Value::from(ttl_seconds)),
        (Value::from("model"), model),
        (
            Value::from("seen"),
            Value::Map(vec![
                (Value::from("root"), Value::from(true)),
                (Value::from("resume"), Value::from(true)),
                (Value::from("step-1"), Value::from(true)),
            ]),
        ),
        (
            Value::from("run"),
            Value::Map(vec![
                (Value::from("resume"), Value::from(true)),
                (Value::from("step-1"), Value::from(true)),
            ]),
        ),
    ])
}

async fn request(po: &PostOffice, route: &str, kind: &str, body: Value) -> EventEnvelope {
    po.request(
        EventEnvelope::new()
            .set_to(route)
            .set_header("type", kind)
            .set_raw_body(body),
        TIMEOUT,
    )
    .await
    .expect("store reply")
}

fn get_element<'a>(body: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = body;
    for key in path {
        let Value::Map(entries) = current else {
            return None;
        };
        current = &entries.iter().find(|(k, _)| k.as_str() == Some(*key))?.1;
    }
    Some(current)
}

fn body_text(reply: &EventEnvelope) -> String {
    match reply.body() {
        Value::String(text) => text.as_str().unwrap_or_default().to_string(),
        other => format!("{other}"),
    }
}

fn is_empty_map(value: &Value) -> bool {
    matches!(value, Value::Map(entries) if entries.is_empty())
}

// One test function on purpose (the repo convention): the platform boots
// ONCE per process, so all contract scenarios run sequentially against the
// same runtime and the same test double.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn redis_state_store_contract() {
    let (port, raw_store) = start_resp_double().await;
    platform_core::resources::prepend_resource_root("tests/resources");
    overrides::set("redis.host", "127.0.0.1");
    overrides::set("redis.port", &port.to_string());
    AutoStart::main(vec![]).await.expect("lifecycle");
    let platform = Platform::get_instance();
    let po = PostOffice::new(&platform);

    // 1) the deployment story: link the crate and the two functions
    // self-register through the normal preload inventory
    assert!(
        platform.has_route(PERSIST_ROUTE),
        "{PERSIST_ROUTE} must self-register"
    );
    assert!(
        platform.has_route(RETRIEVE_ROUTE),
        "{RETRIEVE_ROUTE} must self-register"
    );

    // 2) persist -> retrieve round trip with atomic consumption
    let cid = uuid::Uuid::new_v4().simple().to_string();
    let stored = request(&po, PERSIST_ROUTE, "put", sample_envelope(&cid, 30)).await;
    assert_eq!(200, stored.status(), "persist ack: {:?}", stored.body());
    assert_eq!(
        Some(&Value::from(true)),
        get_element(stored.body(), &["stored"])
    );
    // native expiry is set on the wire-visible record
    {
        let key = format!("{KEY_PREFIX}{cid}").into_bytes();
        let map = raw_store.lock().expect("raw store");
        let entry = map.get(&key).expect("record stored under graph:state:cid");
        let remaining = entry
            .expires_at
            .expect("expiry must be set")
            .saturating_duration_since(Instant::now());
        assert!(
            remaining > Duration::ZERO && remaining <= Duration::from_secs(30),
            "unexpected ttl: {remaining:?}"
        );
    }
    // retrieve returns the record with full fidelity, including binary values
    let restored = request(
        &po,
        RETRIEVE_ROUTE,
        "get",
        Value::Map(vec![(Value::from("cid"), Value::from(cid.as_str()))]),
    )
    .await;
    assert_eq!(200, restored.status());
    assert_eq!(
        Some(&Value::from("step-1")),
        get_element(restored.body(), &["node"])
    );
    assert_eq!(
        Some(&Value::from(42)),
        get_element(restored.body(), &["model", "amount"])
    );
    assert_eq!(
        Some(&Value::from("approval")),
        get_element(restored.body(), &["model", "nested", "stage"])
    );
    assert_eq!(
        Some(&Value::from(vec![1u8, 2, 3])),
        get_element(restored.body(), &["model", "binary"])
    );
    assert_eq!(
        Some(&Value::from(true)),
        get_element(restored.body(), &["run", "step-1"])
    );
    // the record is consumed atomically - a duplicate resume finds nothing
    let again = request(
        &po,
        RETRIEVE_ROUTE,
        "get",
        Value::Map(vec![(Value::from("cid"), Value::from(cid.as_str()))]),
    )
    .await;
    assert_eq!(200, again.status());
    assert!(
        is_empty_map(again.body()),
        "the record must be consumed on read: {:?}",
        again.body()
    );
    assert!(
        !raw_store
            .lock()
            .expect("raw store")
            .contains_key(&format!("{KEY_PREFIX}{cid}").into_bytes()),
        "GETDEL must delete the key"
    );

    // 3) an absent correlation id is a normal empty result, not an error
    let absent = request(
        &po,
        RETRIEVE_ROUTE,
        "get",
        Value::Map(vec![(
            Value::from("cid"),
            Value::from(uuid::Uuid::new_v4().simple().to_string()),
        )]),
    )
    .await;
    assert_eq!(200, absent.status());
    assert!(is_empty_map(absent.body()));

    // 4) an expired record is gone (native expiry, no sweeper)
    let cid = uuid::Uuid::new_v4().simple().to_string();
    let short = request(&po, PERSIST_ROUTE, "put", sample_envelope(&cid, 1)).await;
    assert_eq!(200, short.status());
    tokio::time::sleep(Duration::from_millis(1300)).await;
    let gone = request(
        &po,
        RETRIEVE_ROUTE,
        "get",
        Value::Map(vec![(Value::from("cid"), Value::from(cid.as_str()))]),
    )
    .await;
    assert_eq!(200, gone.status());
    assert!(is_empty_map(gone.body()), "the record must expire natively");

    // 5) wrong request type is rejected
    let wrong = request(
        &po,
        PERSIST_ROUTE,
        "get",
        sample_envelope(&uuid::Uuid::new_v4().simple().to_string(), 30),
    )
    .await;
    assert_ne!(200, wrong.status());
    assert!(
        body_text(&wrong).contains("Type must be put"),
        "unexpected: {}",
        body_text(&wrong)
    );

    // 6) a missing correlation id is rejected
    let missing = request(
        &po,
        PERSIST_ROUTE,
        "put",
        Value::Map(vec![(Value::from("ttl"), Value::from(30))]),
    )
    .await;
    assert_ne!(200, missing.status());
    assert!(
        body_text(&missing).contains("Missing cid"),
        "unexpected: {}",
        body_text(&missing)
    );

    // 7) an invalid ttl is rejected
    let invalid = request(
        &po,
        PERSIST_ROUTE,
        "put",
        Value::Map(vec![(
            Value::from("cid"),
            Value::from(uuid::Uuid::new_v4().simple().to_string()),
        )]),
    )
    .await;
    assert_ne!(200, invalid.status());
    assert!(
        body_text(&invalid).contains("Invalid ttl"),
        "unexpected: {}",
        body_text(&invalid)
    );
}
