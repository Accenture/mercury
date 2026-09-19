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

//! The `v1.cache.redis` contract end to end against the in-process RESP
//! double, THROUGH the event system so the whole path is real — the Rust twin
//! of the Java `RedisCacheTest` (the function contract), `RedisCacheStoreTest`
//! (every operation, the key-prefix namespace, the TTL-from-birth discipline)
//! and `CacheRedisHealthCheckTest` (the `redis.health` binding). One booted
//! platform, sequential scenarios (repo convention: the platform boots once
//! per process).

use redis_test_double as common;

use std::collections::HashMap;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use distributed_cache::{handle, runtime, RedisCacheStore, CACHE_ROUTE, HEALTH_ROUTE};
use platform_core::{
    main_application, overrides, AppError, AutoStart, EntryPoint, EventEnvelope, Platform,
    PostOffice,
};
use redis_connection::{RedisBackend, RedisConfig};
use rmpv::Value;

const TIMEOUT: Duration = Duration::from_secs(8);
const PREFIX: &str = "app1:";

#[main_application]
struct CacheTestApp;

#[async_trait]
impl EntryPoint for CacheTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

// ---- helpers ----

fn headers(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn bytes(text: &str) -> Value {
    Value::Binary(text.as_bytes().to_vec())
}

async fn call(po: &PostOffice, pairs: &[(&str, &str)], body: Value) -> EventEnvelope {
    let mut event = EventEnvelope::new().set_to(CACHE_ROUTE).set_raw_body(body);
    for (k, v) in pairs {
        event = event.set_header(k, v);
    }
    po.request(event, TIMEOUT).await.expect("cache reply")
}

fn body_text(reply: &EventEnvelope) -> String {
    match reply.body() {
        Value::String(text) => text.as_str().unwrap_or_default().to_string(),
        other => format!("{other}"),
    }
}

/// The wire-visible record for a key (prefixed) and the seconds left on it.
fn ttl_of(store: &common::SharedStore, key: &str) -> Option<Duration> {
    let map = store.lock().expect("raw store");
    map.get(&format!("{PREFIX}{key}").into_bytes())
        .map(|entry| {
            entry
                .expires_at
                .map(|at| at.saturating_duration_since(Instant::now()))
                .unwrap_or(Duration::ZERO)
        })
}

fn assert_ttl_within(store: &common::SharedStore, key: &str, max: Duration, what: &str) {
    let ttl = ttl_of(store, key).unwrap_or_else(|| panic!("{what}: {key} must be stored"));
    assert!(
        ttl > Duration::ZERO && ttl <= max,
        "{what}: unexpected ttl {ttl:?} for {key}"
    );
}

fn journal_count(journal: &common::CommandJournal, command: &str) -> usize {
    journal
        .lock()
        .expect("journal")
        .iter()
        .filter(|c| *c == command)
        .count()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn distributed_cache_contract() {
    let (port, raw_store, journal) = common::start_resp_double("7.4.1").await;
    platform_core::resources::prepend_resource_root("tests/resources");
    overrides::set("redis.cache.enabled", "true");
    overrides::set("redis.host", "127.0.0.1");
    overrides::set("redis.port", &port.to_string());
    overrides::set("redis.cache.key.prefix", PREFIX);
    overrides::set("redis.cache.default.ttl", "10m");
    overrides::set("redis.health.startup.grace", "0s");
    AutoStart::main(vec![]).await.expect("lifecycle");
    let platform = Platform::get_instance();
    let po = PostOffice::new(&platform);

    // 1) the deployment story: link the crate, enable the switch, and both
    // functions self-register through the preload inventory
    assert!(
        platform.has_route(CACHE_ROUTE),
        "{CACHE_ROUTE} must self-register"
    );
    assert!(
        platform.has_route(HEALTH_ROUTE),
        "{HEALTH_ROUTE} must self-register"
    );
    assert!(
        runtime::current().is_none(),
        "the connection is built lazily - nothing opened at start-up"
    );

    // 2) PUT returns true and GET round-trips the bytes; the record lives under
    // the PREFIXED key with the default TTL from creation (Java
    // putReturnsTrueAndGetRoundTrips + putThenGetRoundTripsBytesWithATtl)
    let stored = call(&po, &[("action", "PUT"), ("key", "k1")], bytes("hi")).await;
    assert_eq!(200, stored.status(), "{}", body_text(&stored));
    assert_eq!(&Value::Boolean(true), stored.body());
    assert_ttl_within(
        &raw_store,
        "k1",
        Duration::from_secs(600),
        "PUT sets the default TTL",
    );
    assert!(
        !raw_store
            .lock()
            .expect("raw store")
            .contains_key(&b"k1".to_vec()),
        "the key must be namespaced by the prefix"
    );
    let read = call(&po, &[("action", "GET"), ("key", "k1")], Value::Nil).await;
    assert_eq!(&bytes("hi"), read.body());
    assert!(
        runtime::current().is_some(),
        "the first call opened the shared connection"
    );

    // 3) the action is case-insensitive (Java actionIsCaseInsensitive)
    let lower = call(&po, &[("action", "get"), ("key", "k1")], Value::Nil).await;
    assert_eq!(&bytes("hi"), lower.body());

    // 4) a miss is a null body, never an error (Java getMissReturnsNull)
    let miss = call(&po, &[("action", "GET"), ("key", "absent")], Value::Nil).await;
    assert_eq!(200, miss.status());
    assert_eq!(&Value::Nil, miss.body());

    // 5) a String body is stored as its UTF-8 bytes (Java aStringBodyIsStoredAsUtf8Bytes)
    call(
        &po,
        &[("action", "PUT"), ("key", "text")],
        Value::from("text-value"),
    )
    .await;
    let text = call(&po, &[("action", "GET"), ("key", "text")], Value::Nil).await;
    assert_eq!(&bytes("text-value"), text.body());

    // 6) the ttl header bounds the key's TTL (Java ttlHeaderIsHonoured)
    call(
        &po,
        &[("action", "PUT"), ("key", "short"), ("ttl", "30s")],
        bytes("v"),
    )
    .await;
    assert_ttl_within(
        &raw_store,
        "short",
        Duration::from_secs(30),
        "the 30s ttl header",
    );
    let bad_ttl = call(
        &po,
        &[("action", "PUT"), ("key", "x"), ("ttl", "soon")],
        bytes("v"),
    )
    .await;
    assert_eq!(400, bad_ttl.status());
    assert!(body_text(&bad_ttl).contains("Invalid 'ttl'"));

    // 7) DELETE returns the count removed, then nothing (Java deleteReturnsCount
    // + deleteReturnsTheCountRemoved)
    let removed = call(&po, &[("action", "DELETE"), ("key", "k1")], Value::Nil).await;
    assert_eq!(&Value::from(1), removed.body());
    let again = call(&po, &[("action", "DELETE"), ("key", "k1")], Value::Nil).await;
    assert_eq!(
        &Value::from(0),
        again.body(),
        "deleting an absent key removes nothing"
    );
    let gone = call(&po, &[("action", "GET"), ("key", "k1")], Value::Nil).await;
    assert_eq!(&Value::Nil, gone.body());

    // 8) PUT_IF_NOT_PRESENT is atomic SET NX EX: stored once, then refused, the
    // original untouched, a TTL on the key (Java putIfNotPresentReturnsBoolean +
    // putIfAbsentStoresOnlyWhenAbsentAndIsAtomicWithTtl)
    let first = call(
        &po,
        &[("action", "PUT_IF_NOT_PRESENT"), ("key", "once")],
        bytes("first"),
    )
    .await;
    assert_eq!(&Value::Boolean(true), first.body(), "stored when absent");
    let second = call(
        &po,
        &[("action", "PUT_IF_NOT_PRESENT"), ("key", "once")],
        bytes("second"),
    )
    .await;
    assert_eq!(
        &Value::Boolean(false),
        second.body(),
        "not stored when present"
    );
    let kept = call(&po, &[("action", "GET"), ("key", "once")], Value::Nil).await;
    assert_eq!(
        &bytes("first"),
        kept.body(),
        "the original value is untouched"
    );
    assert_ttl_within(
        &raw_store,
        "once",
        Duration::from_secs(600),
        "SET NX EX sets a TTL atomically",
    );
    assert!(
        journal_count(&journal, "SET") >= 2,
        "put-if-absent must be ONE SET command"
    );

    // 9) MGET returns a map of the present keys, misses omitted, keys
    // unprefixed, request order (Java mgetReturnsAMapOfPresentKeys +
    // mgetReturnsPresentKeysAndOmitsMisses + keyPrefix...IsStrippedFromMget)
    call(&po, &[("action", "PUT"), ("key", "a")], bytes("1")).await;
    call(&po, &[("action", "PUT"), ("key", "c")], bytes("3")).await;
    let found = call(
        &po,
        &[("action", "MGET")],
        Value::Array(vec![Value::from("a"), Value::from("b"), Value::from("c")]),
    )
    .await;
    assert_eq!(
        &Value::Map(vec![
            (Value::from("a"), bytes("1")),
            (Value::from("c"), bytes("3")),
        ]),
        found.body(),
        "misses are omitted, not null entries; keys come back without the prefix"
    );
    assert_eq!(1, journal_count(&journal, "MGET"), "one MGET round trip");

    // 10) MPUT writes every entry with its own TTL in one pipelined round trip,
    // non-atomic (no MULTI) - then reads back (Java mputWritesEveryEntryThenReadsBack
    // + mputPipelinesEveryEntryWithItsTtl)
    let setex_before = journal_count(&journal, "SETEX");
    let multi_before = journal_count(&journal, "MULTI");
    let mput = call(
        &po,
        &[("action", "MPUT"), ("ttl", "2m")],
        Value::Map(vec![
            (Value::from("m1"), bytes("one")),
            (Value::from("m2"), Value::from("two")),
        ]),
    )
    .await;
    assert_eq!(&Value::Boolean(true), mput.body());
    assert_eq!(
        setex_before + 2,
        journal_count(&journal, "SETEX"),
        "one SETEX per entry"
    );
    assert_eq!(
        multi_before,
        journal_count(&journal, "MULTI"),
        "MPUT is pipelined, not a transaction"
    );
    assert_ttl_within(
        &raw_store,
        "m1",
        Duration::from_secs(120),
        "each MPUT entry carries the ttl",
    );
    assert_ttl_within(
        &raw_store,
        "m2",
        Duration::from_secs(120),
        "each MPUT entry carries the ttl",
    );
    let m2 = call(&po, &[("action", "GET"), ("key", "m2")], Value::Nil).await;
    assert_eq!(
        &bytes("two"),
        m2.body(),
        "a String entry value is stored as UTF-8"
    );

    // 11) the FIFO list: push returns the length, len counts, pop drains oldest
    // first, a drained list ceases to exist, and the key carries a TTL from the
    // atomic RPUSH+EXPIRE step (Java listPushPopLen + listPushPopLenAreFifoWithATtl)
    let multi_before = journal_count(&journal, "MULTI");
    let n1 = call(
        &po,
        &[("action", "LIST_PUSH"), ("key", "q")],
        bytes("first"),
    )
    .await;
    assert_eq!(&Value::from(1), n1.body());
    let n2 = call(
        &po,
        &[("action", "LIST_PUSH"), ("key", "q")],
        bytes("second"),
    )
    .await;
    assert_eq!(&Value::from(2), n2.body());
    assert_eq!(
        multi_before + 2,
        journal_count(&journal, "MULTI"),
        "each push is one MULTI/EXEC step"
    );
    assert_ttl_within(
        &raw_store,
        "q",
        Duration::from_secs(600),
        "LIST_PUSH sets a TTL atomically",
    );
    let len = call(&po, &[("action", "LIST_LEN"), ("key", "q")], Value::Nil).await;
    assert_eq!(&Value::from(2), len.body());
    let p1 = call(&po, &[("action", "LIST_POP"), ("key", "q")], Value::Nil).await;
    assert_eq!(&bytes("first"), p1.body(), "FIFO: the oldest value first");
    let p2 = call(&po, &[("action", "LIST_POP"), ("key", "q")], Value::Nil).await;
    assert_eq!(&bytes("second"), p2.body());
    let empty = call(&po, &[("action", "LIST_POP"), ("key", "q")], Value::Nil).await;
    assert_eq!(
        &Value::Nil,
        empty.body(),
        "popping an empty list returns null"
    );
    let drained = call(&po, &[("action", "LIST_LEN"), ("key", "q")], Value::Nil).await;
    assert_eq!(
        &Value::from(0),
        drained.body(),
        "a drained list ceases to exist"
    );

    // 12) the rejections, surfaced as the event's error (Java missingActionIsRejected,
    // unsupportedActionIsRejected, putWithoutAValueIsRejected, mgetWithoutAListBodyIsRejected)
    let no_action = call(&po, &[("key", "k1")], Value::Nil).await;
    assert_eq!(400, no_action.status());
    assert!(
        body_text(&no_action).contains("action"),
        "{}",
        body_text(&no_action)
    );
    let incr = call(&po, &[("action", "INCR"), ("key", "k1")], Value::Nil).await;
    assert_eq!(400, incr.status());
    assert!(
        body_text(&incr).contains("INCR"),
        "the unsupported action is named"
    );
    let no_value = call(&po, &[("action", "PUT"), ("key", "k1")], Value::Nil).await;
    assert_eq!(400, no_value.status());
    assert!(
        body_text(&no_value).contains("value"),
        "{}",
        body_text(&no_value)
    );
    let no_list = call(&po, &[("action", "MGET")], bytes("not-a-list")).await;
    assert_eq!(400, no_list.status());
    assert!(
        body_text(&no_list).contains("MGET requires a List"),
        "{}",
        body_text(&no_list)
    );
    let no_map = call(&po, &[("action", "MPUT")], bytes("not-a-map")).await;
    assert_eq!(400, no_map.status());
    assert!(
        body_text(&no_map).contains("MPUT requires a Map"),
        "{}",
        body_text(&no_map)
    );
    let no_key = call(&po, &[("action", "GET")], Value::Nil).await;
    assert_eq!(400, no_key.status());
    assert!(
        body_text(&no_key).contains("Missing 'key'"),
        "{}",
        body_text(&no_key)
    );

    // 13) redis.health: info names the dependency from the plain namespace, and
    // a live probe finds the server reachable (Java infoResolvesThePlainRedisNamespaceHref;
    // the grace was set to 0s so the probe is live at once)
    let info = po
        .request(
            EventEnvelope::new()
                .set_to(HEALTH_ROUTE)
                .set_header("type", "info"),
            TIMEOUT,
        )
        .await
        .expect("info reply");
    let info: serde_json::Value = info.body_as().expect("json body");
    assert_eq!("redis", info["service"]);
    assert_eq!(format!("127.0.0.1:{port}"), info["href"]);
    let health = po
        .request(
            EventEnvelope::new()
                .set_to(HEALTH_ROUTE)
                .set_header("type", "health"),
            TIMEOUT,
        )
        .await
        .expect("health reply");
    assert_eq!(200, health.status());
    let health: serde_json::Value = health.body_as().expect("json body");
    assert_eq!("Redis is reachable", health["status"]);

    // 14) the empty bulk cases are no-ops at the store level (Java mgetOnEmptyKeysIsAnEmptyMap
    // + mputOnEmptyMapIsANoOp) - driven through the reuse seam with an explicit store
    let backend = RedisBackend::connect(&RedisConfig::new("127.0.0.1", port, "", false, 0, 2000))
        .await
        .expect("connect");
    let store = RedisCacheStore::new(backend, "", 60);
    let empty_mget = handle(
        &headers(&[("action", "MGET")]),
        &Value::Array(vec![]),
        &store,
    )
    .await
    .expect("empty mget");
    assert_eq!(&Value::Map(vec![]), empty_mget.body());
    let empty_mput = handle(&headers(&[("action", "MPUT")]), &Value::Map(vec![]), &store)
        .await
        .expect("empty mput");
    assert_eq!(&Value::Boolean(true), empty_mput.body());
    assert_eq!(60, store.default_ttl_seconds());
    assert!(!store.backend().cluster());

    // 15) shutdown releases the shared connection; the next call rebuilds it
    // from live configuration (the lazy runtime's whole point)
    runtime::shutdown();
    assert!(runtime::current().is_none());
    let rebuilt = call(&po, &[("action", "GET"), ("key", "once")], Value::Nil).await;
    assert_eq!(
        &bytes("first"),
        rebuilt.body(),
        "the store rebuilt on demand"
    );
    assert!(runtime::current().is_some());
}
