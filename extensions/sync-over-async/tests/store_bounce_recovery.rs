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

//! The bounce-recovery contract (port spec §5 item 6, maintainer ruling on the
//! R4 report's note 3): after a Redis connection loss, the store's
//! **idempotent** operations heal in a single call via one lifecycle-aware
//! retry, its **non-idempotent** operations stay fail-fast, and the retry is
//! bounded — one second attempt, never a loop.
//!
//! The bounce itself is simulated by a test-local TCP proxy between the store
//! and the in-process RESP double: killing the proxy's live links is a server
//! bounce (connections die, the server is immediately back), and dropping its
//! listener is an outage (reconnects refused).

use std::time::Duration;

use redis_test_double::{start_resp_double, BounceProxy};
use sync_over_async::{RedisSettings, ReturnRouteStore};

/// The store through a severable relay. The heartbeat monitor is OFF here on
/// purpose: these tests pin the retry semantics of a command that meets the
/// lost connection itself, which the heartbeat would otherwise heal ahead of
/// it (the foundation's own suite proves the heartbeat).
async fn store_through_proxy(timeout_ms: u64) -> (ReturnRouteStore, BounceProxy) {
    let (redis_port, _store, _journal) = start_resp_double("7.4.0").await;
    let proxy = BounceProxy::start(redis_port).await;
    let settings =
        RedisSettings::new("127.0.0.1", proxy.port(), "", false, 0, timeout_ms).with_heartbeat(0);
    let store = ReturnRouteStore::connect(&settings)
        .await
        .expect("store connects");
    (store, proxy)
}

/// Every idempotent operation heals in ONE call across a server bounce: the
/// first attempt's connection is dead, the manager swaps in its reconnection
/// future, and the store's single retry awaits the fresh connection.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn idempotent_operations_heal_across_a_server_bounce() {
    let (store, proxy) = store_through_proxy(5000).await;
    store
        .save_route("cid-1", "svc-return:pod", 60)
        .await
        .expect("healthy save");

    proxy.bounce().await;
    store
        .save_route("cid-1", "svc-return:pod", 60)
        .await
        .expect("save_route heals in one call");

    proxy.bounce().await;
    assert_eq!(
        Some("svc-return:pod".to_string()),
        store.get_route("cid-1").await.expect("get_route heals"),
        "the value survives the bounce (server state, not connection state)"
    );

    proxy.bounce().await;
    assert_eq!(
        0,
        store
            .queue_length("cid-1")
            .await
            .expect("queue_length heals")
    );

    proxy.bounce().await;
    store.cleanup("cid-1").await.expect("cleanup heals");
    assert_eq!(None, store.get_route("cid-1").await.expect("route gone"));
}

/// The non-idempotent operations stay fail-fast across a bounce — replaying an
/// ambiguous `RPUSH` risks an undetectable duplicate segment and replaying an
/// ambiguous `LPOP` could silently discard one — and the same call succeeds
/// once the caller retries (the manager healed in the background).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn append_and_pop_stay_fail_fast_across_a_bounce() {
    let (store, proxy) = store_through_proxy(5000).await;
    store
        .append_segment("cid-2", "{\"type\":\"data\",\"body\":\"one\"}", 60)
        .await
        .expect("healthy append");

    proxy.bounce().await;
    store
        .append_segment("cid-2", "{\"type\":\"data\",\"body\":\"two\"}", 60)
        .await
        .expect_err("append fails fast on the bounced connection");
    store
        .append_segment("cid-2", "{\"type\":\"data\",\"body\":\"two\"}", 60)
        .await
        .expect("the caller's own retry lands on the healed connection");

    proxy.bounce().await;
    store
        .pop_segment("cid-2")
        .await
        .expect_err("pop fails fast on the bounced connection");
    assert_eq!(
        Some("{\"type\":\"data\",\"body\":\"one\"}".to_string()),
        store
            .pop_segment("cid-2")
            .await
            .expect("pop heals on the caller's retry"),
        "nothing was popped into the void by the failed attempt"
    );
}

/// The retry is bounded: one second attempt, never a loop. Under a full
/// outage (reconnects refused) an idempotent call fails after its two bounded
/// attempts instead of hanging.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn retry_is_single_and_bounded_under_a_full_outage() {
    let (store, proxy) = store_through_proxy(1500).await;
    store
        .save_route("cid-3", "svc-return:pod", 60)
        .await
        .expect("healthy save");

    proxy.refuse().await;
    let started = std::time::Instant::now();
    store
        .save_route("cid-3", "svc-return:pod", 60)
        .await
        .expect_err("a full outage still fails");
    assert!(
        started.elapsed() < Duration::from_secs(6),
        "two bounded attempts, not an open-ended retry loop (took {:?})",
        started.elapsed()
    );
}
