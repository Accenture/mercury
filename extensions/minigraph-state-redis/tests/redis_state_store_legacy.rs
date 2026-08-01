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

//! The MULTI/EXEC GET+DEL fallback for Redis servers older than 6.2 — the
//! Rust twin of the Java `RedisStateStoreTest` legacy scenarios (field
//! report: the redis-standalone Windows binary is 5.0.14, and enterprise
//! managed Redis versions on AWS/Azure/GCP are outside our control).
//!
//! A separate test binary on purpose: the consume strategy is detected once
//! per process when the shared connection manager is first built, so this
//! suite gets a fresh detection against a double reporting Redis 5.0.14 —
//! the FALLBACK path runs for real, not by forcing a flag. The command
//! journal proves which strategy went over the wire.

mod common;

use std::time::Duration;

use async_trait::async_trait;
use platform_core::{
    main_application, overrides, AppError, AutoStart, EntryPoint, EventEnvelope, Platform,
    PostOffice,
};
use rmpv::Value;

use minigraph_state_redis::{PERSIST_ROUTE, RETRIEVE_ROUTE};

const TIMEOUT: Duration = Duration::from_secs(8);
const KEY_PREFIX: &str = "graph:state:";

#[main_application]
struct RedisLegacyTestApp;

#[async_trait]
impl EntryPoint for RedisLegacyTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
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

fn is_empty_map(value: &Value) -> bool {
    matches!(value, Value::Map(entries) if entries.is_empty())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn transactional_consume_on_servers_older_than_6_2() {
    let (port, raw_store, journal) = common::start_resp_double("5.0.14").await;
    platform_core::resources::prepend_resource_root("tests/resources");
    overrides::set("redis.host", "127.0.0.1");
    overrides::set("redis.port", &port.to_string());
    AutoStart::main(vec![]).await.expect("lifecycle");
    let platform = Platform::get_instance();
    let po = PostOffice::new(&platform);

    // persist -> retrieve round trip through the TRANSACTIONAL consume path
    let cid = uuid::Uuid::new_v4().simple().to_string();
    let envelope = Value::Map(vec![
        (Value::from("cid"), Value::from(cid.as_str())),
        (Value::from("node"), Value::from("step-1")),
        (Value::from("ttl"), Value::from(30)),
        (
            Value::from("model"),
            Value::Map(vec![
                (Value::from("amount"), Value::from(42)),
                (Value::from("binary"), Value::from(vec![1u8, 2, 3])),
            ]),
        ),
    ]);
    let stored = request(&po, PERSIST_ROUTE, "put", envelope).await;
    assert_eq!(200, stored.status(), "persist ack: {:?}", stored.body());

    // full fidelity through MULTI/EXEC GET+DEL, including binary values
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
        Some(&Value::from(vec![1u8, 2, 3])),
        get_element(restored.body(), &["model", "binary"])
    );

    // the record is consumed atomically - a duplicate resume finds nothing
    // and the key is gone from the server (same post-conditions as GETDEL)
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
        "the transaction must delete the key"
    );

    // an absent correlation id is a normal empty result on this path too
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

    // the journal PROVES the strategy: MULTI/GET/DEL/EXEC went over the
    // wire and GETDEL never did - the detection, not a forced flag, chose it
    let commands = journal.lock().expect("journal").clone();
    assert!(
        commands.iter().any(|c| c == "INFO"),
        "strategy detection must probe INFO server: {commands:?}"
    );
    for expected in ["MULTI", "GET", "DEL", "EXEC"] {
        assert!(
            commands.iter().any(|c| c == expected),
            "{expected} must appear on the wire: {commands:?}"
        );
    }
    assert!(
        !commands.iter().any(|c| c == "GETDEL"),
        "GETDEL must never be sent to a pre-6.2 server: {commands:?}"
    );
}
