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

//! End-to-end drive of the tutorial-14 purchase workflow — the Rust twin of
//! the Java example app's `SuspendResumeTutorialTest`: three human
//! checkpoints (order, approval, delivery release) expressed as four short
//! graph runs sharing one X-Correlation-Id, each run resuming past the
//! previous checkpoint through the REAL Redis client and the
//! `v1.redis.persist.model` / `v1.redis.retrieve.model` store functions,
//! with the final response carrying the state captured across all of them.
//!
//! The Java suite embeds a real redis-server binary; this environment has
//! neither the binary nor a Docker daemon, so the server side is the same
//! in-process RESP2 test double the `minigraph-state-redis` crate's contract
//! tests use — the client wire path (TCP, RESP2 frames, SETEX/GETDEL) stays
//! fully real.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use platform_core::automation::AsyncHttpRequest;
use platform_core::{automation, overrides, AutoStart, EventEnvelope, Platform, PostOffice};
use rmpv::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// The application under test is a BIN crate — include its source so the
// link-time inventory in this test binary carries the engine skills AND the
// Redis state-store functions the app links.
#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

const TIMEOUT: Duration = Duration::from_secs(8);

// ---- the RESP2 test double (same approach as the minigraph-state-redis
// contract tests — the double stands in for redis-server, never the client) ----

#[derive(Clone)]
struct StoredValue {
    value: Vec<u8>,
    expires_at: Option<Instant>,
}

type SharedStore = Arc<Mutex<HashMap<Vec<u8>, StoredValue>>>;

async fn start_resp_double() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().expect("addr").port();
    let store: SharedStore = Arc::new(Mutex::new(HashMap::new()));
    tokio::spawn(async move {
        loop {
            let Ok((socket, _)) = listener.accept().await else {
                return;
            };
            let store = store.clone();
            tokio::spawn(async move { serve_connection(socket, store).await });
        }
    });
    port
}

async fn serve_connection(mut socket: tokio::net::TcpStream, store: SharedStore) {
    let mut buffer: Vec<u8> = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
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
        pos = end + 2;
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
        "CLIENT" | "SELECT" | "AUTH" => b"+OK\r\n".to_vec(),
        // the store detects its consume strategy from INFO server once per
        // process; report a 6.2+ version so this suite keeps exercising the
        // native GETDEL path (the transactional fallback has its own suite
        // in the store crate)
        "INFO" => {
            let body = "# Server\r\nredis_version:7.4.1\r\n";
            let mut reply = format!("${}\r\n", body.len()).into_bytes();
            reply.extend_from_slice(body.as_bytes());
            reply.extend_from_slice(b"\r\n");
            reply
        }
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

fn text_of(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(text)) => text.as_str().unwrap_or_default().to_string(),
        Some(other) => format!("{other}"),
        None => String::new(),
    }
}

async fn run_graph(po: &PostOffice, target: &str, cid: &str, body: Value) -> EventEnvelope {
    let request = AsyncHttpRequest::new()
        .set_method("POST")
        .set_target_host(target)
        .set_url("/api/graph/tutorial-14")
        .set_header("content-type", "application/json")
        .set_header("accept", "application/json")
        .set_header("x-correlation-id", cid)
        .set_body(body);
    po.request(
        EventEnvelope::new()
            .set_to("async.http.request")
            .set_raw_body(request.to_value()),
        TIMEOUT,
    )
    .await
    .expect("graph run reply")
}

fn stage(response: &EventEnvelope, cid: &str) -> Value {
    assert_eq!(200, response.status(), "run reply: {:?}", response.body());
    assert_eq!(
        cid,
        text_of(get_element(response.body(), &["cid"])),
        "every reply carries the business correlation id"
    );
    response.body().clone()
}

fn json_map(entries: &[(&str, Value)]) -> Value {
    Value::Map(
        entries
            .iter()
            .map(|(k, v)| (Value::from(*k), v.clone()))
            .collect(),
    )
}

// One test function on purpose (the repo convention): the app boots ONCE per
// process, so all scenarios run sequentially against the same server.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn tutorial_14_purchase_workflow() {
    let redis_port = start_resp_double().await;
    // the state-store connection is lazy and reads configuration at first
    // use, so an override set before the first suspension is honored
    overrides::set("redis.host", "127.0.0.1");
    overrides::set("redis.port", &redis_port.to_string());
    overrides::set("rest.server.port", "0");
    let holding = std::env::temp_dir().join(format!("tutorial-14-{}", std::process::id()));
    overrides::set("transient.data.store", &holding.display().to_string());
    AutoStart::main(vec![]).await.expect("app lifecycle");
    let port = automation::server_address().expect("server started").port();
    let target = format!("http://127.0.0.1:{port}");
    let po = PostOffice::new(&Platform::get_instance());

    // --- the purchase workflow: three checkpoints, four runs, ONE cid
    let cid = uuid::Uuid::new_v4().simple().to_string();
    // run 1: the customer orders a laptop - suspend for the store manager
    let ordered = stage(
        &run_graph(
            &po,
            &target,
            &cid,
            json_map(&[
                ("item", Value::from("laptop")),
                ("amount", Value::from(2000)),
            ]),
        )
        .await,
        &cid,
    );
    assert!(
        text_of(get_element(&ordered, &["stage"])).starts_with("order-submitted"),
        "run 1: {ordered:?}"
    );
    assert_eq!(
        "fresh",
        text_of(get_element(&ordered, &["run"])),
        "run 1 is a fresh transaction"
    );
    // run 2: the store manager approves - suspend for the delivery department
    let approved = stage(
        &run_graph(
            &po,
            &target,
            &cid,
            json_map(&[
                ("decision", Value::from("approved")),
                ("manager", Value::from("store-88")),
            ]),
        )
        .await,
        &cid,
    );
    assert!(
        text_of(get_element(&approved, &["stage"])).starts_with("approved"),
        "run 2: {approved:?}"
    );
    // run 3: the delivery department releases the shipment
    let released = stage(
        &run_graph(
            &po,
            &target,
            &cid,
            json_map(&[
                ("release", Value::from(true)),
                ("courier", Value::from("express")),
            ]),
        )
        .await,
        &cid,
    );
    assert!(
        text_of(get_element(&released, &["stage"])).starts_with("released"),
        "run 3: {released:?}"
    );
    // run 4: shipment confirmation - the workflow completes with full history
    let shipped = stage(
        &run_graph(
            &po,
            &target,
            &cid,
            json_map(&[("tracking", Value::from("TRK-12345"))]),
        )
        .await,
        &cid,
    );
    assert_eq!("shipped", text_of(get_element(&shipped, &["stage"])));
    assert_eq!(
        "resume",
        text_of(get_element(&shipped, &["run"])),
        "runs 2-4 are resumed continuations"
    );
    // state captured across all four runs survived every suspension
    assert_eq!("laptop", text_of(get_element(&shipped, &["order", "item"])));
    assert_eq!(
        Some(&Value::from(2000)),
        get_element(&shipped, &["order", "amount"])
    );
    assert_eq!(
        "approved",
        text_of(get_element(&shipped, &["approval", "decision"]))
    );
    assert_eq!(
        "store-88",
        text_of(get_element(&shipped, &["approval", "manager"]))
    );
    assert_eq!(
        Some(&Value::from(true)),
        get_element(&shipped, &["delivery", "release"])
    );
    assert_eq!(
        "express",
        text_of(get_element(&shipped, &["delivery", "courier"]))
    );
    assert_eq!(
        "TRK-12345",
        text_of(get_element(&shipped, &["shipment", "tracking"]))
    );
    log::info!("tutorial-14 purchase workflow shipped for cid {cid}");

    // --- a transaction that never suspended simply flows through the resume
    // node (fresh correlation id runs from the start)
    let fresh_cid = uuid::Uuid::new_v4().simple().to_string();
    let fresh = run_graph(
        &po,
        &target,
        &fresh_cid,
        json_map(&[("item", Value::from("mouse"))]),
    )
    .await;
    assert_eq!(200, fresh.status());
    assert!(
        text_of(get_element(fresh.body(), &["stage"])).starts_with("order-submitted"),
        "fresh run: {:?}",
        fresh.body()
    );

    // --- input validation: an approval decision for a transaction that was
    // never submitted (or has expired) must be rejected - submission first
    let unknown_cid = uuid::Uuid::new_v4().simple().to_string();
    let rejected = run_graph(
        &po,
        &target,
        &unknown_cid,
        json_map(&[("decision", Value::from("approved"))]),
    )
    .await;
    assert_eq!(404, rejected.status(), "gate: {:?}", rejected.body());
    assert_eq!("rejected", text_of(get_element(rejected.body(), &["type"])));
    assert_eq!(
        "fresh",
        text_of(get_element(rejected.body(), &["run"])),
        "the rejection advises the UI of the fresh condition"
    );
    assert!(
        text_of(get_element(rejected.body(), &["message"])).contains("Submit the order"),
        "unexpected rejection message: {:?}",
        rejected.body()
    );
}
