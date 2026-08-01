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
//! with the shared in-process **RESP2 test double** (see `common`). The
//! double reports Redis 7.4.1 here, so this suite exercises the native
//! GETDEL consume strategy end to end — including its detection from
//! `INFO server`; the MULTI/EXEC fallback for older servers has its own
//! suite (`redis_state_store_legacy.rs`, a separate process so the
//! once-per-process strategy detection runs fresh).

mod common;

use std::time::{Duration, Instant};

use async_trait::async_trait;
use platform_core::{
    main_application, overrides, AppError, AutoStart, EntryPoint, EventEnvelope, Platform,
    PostOffice,
};
use rmpv::Value;

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
    let (port, raw_store, _journal) = common::start_resp_double("7.4.1").await;
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
