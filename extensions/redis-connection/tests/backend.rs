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

//! The backend seam against the in-process RESP double — the Rust twin of the
//! Java `RedisBackendFactoryTest`: the two-key topology selection (auto-detect
//! resolves a standalone server; explicit mode skips detection; explicit
//! cluster mode routes to the cluster branch), the one multiplexed connection
//! that pipelines, and opaque bytes round-tripping unchanged. A real cluster
//! cannot be stood up in-process, so — exactly as the Java suite — the cluster
//! branch is pinned by its selection and its first wire exchange, not by a
//! live `MOVED` storm.

use std::time::Duration;

use redis_connection::{ConnectError, RedisBackend, RedisConfig};
use redis_test_double as common;

fn standalone(port: u16) -> RedisConfig {
    // the constructor form sync-over-async always used: detection OFF
    RedisConfig::new("127.0.0.1", port, "", false, 0, 2000)
}

fn journal_has(journal: &common::CommandJournal, command: &str) -> bool {
    journal
        .lock()
        .expect("journal")
        .iter()
        .any(|c| c == command)
}

/// Java `autoDetectResolvesAStandaloneServerAndRoundTrips`: `cluster.detect=auto`
/// probes `INFO cluster`, finds no `cluster_enabled:1`, and builds the
/// standalone manager — which then round-trips a value.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn auto_detect_resolves_a_standalone_server_and_round_trips() {
    let (port, _store, journal) = common::start_resp_double("7.4.1").await;
    let config = standalone(port).with_cluster(true, false, "");
    let backend = RedisBackend::connect(&config).await.expect("connect");
    assert!(!backend.cluster(), "a single node is standalone");
    assert!(
        journal_has(&journal, "INFO"),
        "auto-detect must probe INFO: {:?}",
        journal.lock().expect("journal")
    );
    let ok: String = backend
        .query(redis::cmd("SETEX").arg("k1").arg(60).arg("hello"))
        .await
        .expect("setex");
    assert_eq!("OK", ok);
    let value: Option<String> = backend
        .query(redis::cmd("GET").arg("k1"))
        .await
        .expect("get");
    assert_eq!(Some("hello".to_string()), value);
    assert_eq!(format!("127.0.0.1:{port}"), backend.endpoint());
    assert_eq!(Duration::from_millis(2000), backend.timeout());
}

/// Java `explicitStandaloneModeSkipsDetectionAndRoundTrips`.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn explicit_standalone_mode_skips_detection() {
    let (port, _store, journal) = common::start_resp_double("7.4.1").await;
    let backend = RedisBackend::connect(&standalone(port))
        .await
        .expect("connect");
    assert!(!backend.cluster());
    assert!(
        !journal_has(&journal, "INFO"),
        "explicit mode must not probe: {:?}",
        journal.lock().expect("journal")
    );
    backend.ping().await.expect("ping");
    let missing: Option<String> = backend
        .query(redis::cmd("GET").arg("absent"))
        .await
        .expect("get");
    assert_eq!(None, missing, "a miss is a None, never an error");
}

/// Java `asyncCommandsPipelineOverTheSameConnection`: a batch of `SETEX` in one
/// round trip, then `MGET` reads them all back (misses omitted as nulls).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_pipeline_rides_the_one_connection() {
    let (port, _store, _journal) = common::start_resp_double("7.4.1").await;
    let backend = RedisBackend::connect(&standalone(port))
        .await
        .expect("connect");
    let mut pipe = redis::pipe();
    for (key, value) in [("a", "1"), ("b", "2"), ("c", "3")] {
        pipe.cmd("SETEX").arg(key).arg(60).arg(value);
    }
    let acks: Vec<String> = backend.query_pipeline(&pipe).await.expect("pipeline");
    assert_eq!(vec!["OK", "OK", "OK"], acks);
    let values: Vec<Option<String>> = backend
        .query(redis::cmd("MGET").arg("a").arg("missing").arg("c"))
        .await
        .expect("mget");
    assert_eq!(
        vec![Some("1".to_string()), None, Some("3".to_string())],
        values
    );
}

/// Java `byteArrayCodecStoresOpaqueBytes`: bytes go in and come out
/// unchanged — no text decoding in between.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn opaque_bytes_round_trip_unchanged() {
    let (port, _store, _journal) = common::start_resp_double("7.4.1").await;
    let backend = RedisBackend::connect(&standalone(port))
        .await
        .expect("connect");
    let payload: Vec<u8> = vec![0, 255, 1, 2, 3, 0x93, 0xa1];
    let _: String = backend
        .query(redis::cmd("SETEX").arg("bin").arg(60).arg(payload.clone()))
        .await
        .expect("setex");
    let stored: Vec<u8> = backend
        .query(redis::cmd("GET").arg("bin"))
        .await
        .expect("get");
    assert_eq!(payload, stored);
}

/// Java `explicitClusterModeRoutesToTheClusterBranch`: `cluster.mode=true`
/// with detection off builds the cluster client, whose first exchange asks the
/// node for its slot map — the double is a single node, so the branch fails
/// there, which is exactly the proof that it was taken.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn explicit_cluster_mode_routes_to_the_cluster_branch() {
    let (port, _store, journal) = common::start_resp_double("7.4.1").await;
    let config = standalone(port).with_cluster(false, true, "");
    let outcome = RedisBackend::connect(&config).await;
    assert!(
        matches!(
            outcome,
            Err(ConnectError::Redis(_)) | Err(ConnectError::TimedOut(_))
        ),
        "a single node cannot serve a cluster client"
    );
    assert!(
        journal_has(&journal, "CLUSTER"),
        "the cluster client must have asked for the topology: {:?}",
        journal.lock().expect("journal")
    );
}

/// The inconclusive-probe fallback: an unreachable seed cannot answer `INFO`,
/// so auto-detect resolves to the configured `cluster.mode` boolean.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn inconclusive_detection_falls_back_to_the_configured_mode() {
    let dead_port = {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
        listener.local_addr().expect("addr").port()
    };
    let config = RedisConfig::new("127.0.0.1", dead_port, "", false, 0, 500);
    assert!(RedisBackend::detect_cluster(&config, true).await);
    assert!(!RedisBackend::detect_cluster(&config, false).await);
}

/// The probe's shape: a plain connection that answers PING, and reports a
/// refused connection as a Redis error rather than healing it underneath.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn connect_once_builds_a_plain_connection() {
    let (port, _store, _journal) = common::start_resp_double("7.4.1").await;
    let backend = RedisBackend::connect_once(&standalone(port))
        .await
        .expect("connect");
    backend.ping().await.expect("ping");
    let dead_port = {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
        listener.local_addr().expect("addr").port()
    };
    let refused =
        RedisBackend::connect_once(&RedisConfig::new("127.0.0.1", dead_port, "", false, 0, 500))
            .await;
    assert!(matches!(
        refused,
        Err(ConnectError::Redis(_)) | Err(ConnectError::TimedOut(_))
    ));
}
