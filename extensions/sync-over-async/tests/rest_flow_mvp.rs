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

//! End-to-end proof of the whole sync-over-async MVP, fully composable (the
//! Java `RestFlowMvpTest` twin):
//!
//! ```text
//!   HTTP POST /api/sync-to-async -> http.flow.adapter -> flow sync-to-async
//!     sync.prepare: begin(cid) [Redis] -> simple.kafka.notification -> topic-1 -> sync.await (parks)
//!       -> Kafka Flow Adapter -> flow system-of-record (echo + notify topic-2)
//!         -> topic-2 -> Kafka Flow Adapter -> flow soa-reply -> soa.reply: deliver(cid)
//!           -> the Redis return route wakes sync.await -> HTTP 200 + body
//! ```
//!
//! Configuration only: the REST endpoint, the three flows and the two topic
//! bindings are the Java test's resources verbatim; the Kafka publisher and the
//! per-topic flow consumers are built by the minimalist-kafka library's own
//! auto-start, the coordinator by this extension's — both collected from the
//! linked crates, as an application would. The broker is the in-process mock
//! cluster, Redis the in-process RESP double. Because every hop runs as a flow
//! task carrying the `traceparent`, the whole path is one trace.

use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;

use async_trait::async_trait;
// link the engines this suite drives by configuration alone: their inventory
// entries (auto-start hooks, preloaded functions) register at link time
use event_script as _;
use minimalist_kafka as _;
use platform_core::automation::{AsyncHttpRequest, ASYNC_HTTP_REQUEST};
use platform_core::{
    preload, resources, AppConfigReader, AppError, AutoStart, ComposableFunction, EventEnvelope,
    Platform, PostOffice,
};
use rdkafka::mocking::MockCluster;
use redis_test_double::{start_resp_double, CommandJournal, SharedStore};
use sync_over_async::{runtime, CID};

/// Matches `rest.server.port` in the suite's application.yml (the Java test's 8305).
const REST_PORT: u16 = 8305;
const TRACE_PARENT: &str = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";
const TRACE_ID: &str = "0af7651916cd43dd8448eb211c80319c";
const REQUEST_TOPIC: &str = "topic-1";
const RESPONSE_TOPIC: &str = "topic-2";

// ---------------------------------------------------------------------------
// the application's own functions (Java mock package twins)
// ---------------------------------------------------------------------------

/// The backend business logic — the only application-specific task: it echoes
/// the request it received over Kafka, carrying the correlation-id and its own
/// trace id (continuous across the hops), and asks the flow to publish the
/// reply. A `no-reply` action simulates a backend that never answers, so the
/// facade's timeout path can be exercised: the flow's decision task routes a
/// numeric decision to `next[n-1]` — 1 drops (`no.op`), 2 publishes.
#[preload(route = "system.of.record", instances = 10)]
#[derive(Default)]
struct SystemOfRecord;

#[async_trait]
impl ComposableFunction for SystemOfRecord {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let cid = headers.get(CID).cloned();
        let request: serde_json::Value = match input.body() {
            rmpv::Value::Binary(bytes) => serde_json::from_slice(bytes)
                .map_err(|e| AppError::new(400, format!("request is not JSON - {e}")))?,
            _ => input.body_as()?,
        };
        if request["action"] == "no-reply" {
            return Ok(EventEnvelope::new().set_header("decision", "1"));
        }
        let response = serde_json::json!({
            "cid": cid,
            // continuous across the Kafka hops: the worker-injected trace id
            "traceId": headers.get("my_trace_id"),
            "echo": request,
        });
        let payload = serde_json::to_vec(&response)
            .map_err(|e| AppError::new(500, format!("unable to render the reply - {e}")))?;
        let mut result = EventEnvelope::new()
            .set_header("decision", "2")
            .set_raw_body(rmpv::Value::Binary(payload));
        if let Some(cid) = cid {
            result = result.set_header(CID, &cid);
        }
        Ok(result)
    }
}

/// The facade flow's exception handler: the HTTP status policy. A 408 from
/// `sync.await` passes through; any 5xx (a publish or registration failure —
/// the async backend is unreachable) is re-mapped to a retriable 503; the
/// pending entry registered by `sync.prepare` is cancelled on the fail-fast path.
#[preload(route = "sync.error.handler", instances = 10)]
#[derive(Default)]
struct SyncErrorHandler;

#[async_trait]
impl ComposableFunction for SyncErrorHandler {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let error: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        if let Some(cid) = error.get(CID).and_then(serde_json::Value::as_str) {
            if let Some(coordinator) = runtime::coordinator() {
                coordinator.abort(cid);
            }
        }
        let mut status = error
            .get("status")
            .and_then(|s| {
                s.as_i64()
                    .or_else(|| s.as_str().and_then(|t| t.parse().ok()))
            })
            .unwrap_or(500);
        if status >= 500 {
            status = 503;
        }
        let message = error
            .get("message")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("Internal error");
        EventEnvelope::new().set_body(serde_json::json!({
            "type": "error",
            "status": status,
            "message": message,
        }))
    }
}

// ---------------------------------------------------------------------------
// harness
// ---------------------------------------------------------------------------

struct Infra {
    _store: SharedStore,
    _journal: CommandJournal,
}

/// Mock cluster + RESP double + env + config, pinned BEFORE the configuration
/// snapshot; then the whole application lifecycle (both libraries' auto-starts).
async fn boot() {
    static INFRA: OnceLock<Infra> = OnceLock::new();
    if INFRA.get().is_some() {
        return;
    }
    let cluster = MockCluster::new(1).expect("mock cluster");
    for topic in [REQUEST_TOPIC, RESPONSE_TOPIC] {
        cluster.create_topic(topic, 1, 1).expect("topic");
    }
    std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", cluster.bootstrap_servers());
    std::mem::forget(cluster);
    let (redis_port, store, journal) = start_resp_double("7.4.0").await;
    std::env::set_var("SOA_REDIS_PORT", redis_port.to_string());
    let _ = INFRA.set(Infra {
        _store: store,
        _journal: journal,
    });
    resources::prepend_resource_root("tests/resources-mvp");
    let _ = AppConfigReader::get_instance();
    AutoStart::main(vec![]).await.expect("lifecycle");
}

async fn post(body: serde_json::Value, timeout_header: Option<&str>) -> EventEnvelope {
    let mut request = AsyncHttpRequest::new()
        .set_method("POST")
        .set_target_host(&format!("http://127.0.0.1:{REST_PORT}"))
        .set_url("/api/sync-to-async")
        .set_header("accept", "application/json")
        .set_header("content-type", "application/json")
        .set_header("traceparent", TRACE_PARENT)
        .set_body(rmpv::ext::to_value(&body).expect("body"))
        .set_timeout_seconds(20);
    if let Some(timeout) = timeout_header {
        request = request.set_header("x-sync-timeout", timeout);
    }
    let po = PostOffice::new(&Platform::get_instance());
    po.request(
        EventEnvelope::new()
            .set_to(ASYNC_HTTP_REQUEST)
            .set_raw_body(request.to_value()),
        Duration::from_secs(25),
    )
    .await
    .expect("http round trip")
}

fn json(value: &rmpv::Value) -> serde_json::Value {
    rmpv::ext::from_value(value.clone()).unwrap_or(serde_json::Value::Null)
}

// ---------------------------------------------------------------------------
// the scenarios (one runtime, one lifecycle - sequential)
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn synchronous_round_trip_and_timeout() {
    boot().await;

    // --- the facade returns the async response synchronously
    let response = post(serde_json::json!({"action": "create"}), None).await;
    assert_eq!(
        200,
        response.status(),
        "the facade should return the async response synchronously: {:?}",
        json(response.body())
    );
    let body = json(response.body());
    assert!(
        body["cid"].is_string(),
        "response carries the correlation-id"
    );
    // the system-of-record flow echoes the request it received over Kafka;
    // getting it back proves the full request -> Kafka -> SoR flow -> Kafka ->
    // reply flow -> Redis return route -> await round trip
    assert_eq!(
        serde_json::json!({"action": "create"}),
        body["echo"],
        "the request round-tripped through Kafka"
    );
    // span-ids change per hop; the trace-id must stay continuous end to end
    assert_eq!(
        TRACE_ID, body["traceId"],
        "trace-id stayed continuous from REST through Kafka to the system of record"
    );

    // --- the backend drops "no-reply" requests; with a short await budget the
    // facade returns 408 through sync.error.handler
    let timeout = post(serde_json::json!({"action": "no-reply"}), Some("1500")).await;
    assert_eq!(
        408,
        timeout.status(),
        "an unanswered request should time out as HTTP 408: {:?}",
        json(timeout.body())
    );
    let error = json(timeout.body());
    assert_eq!("error", error["type"]);
    assert_eq!(408, error["status"]);
    assert!(
        error["message"]
            .as_str()
            .unwrap_or_default()
            .starts_with("Timed out awaiting response for "),
        "{error}"
    );
    assert_eq!(
        0,
        runtime::pending_count(),
        "the timed-out entry was released (and the fail-fast handler aborts the rest)"
    );

    // --- stop what the lifecycle started
    let still_running = tokio::task::spawn_blocking(minimalist_kafka::runtime::stop_flow_consumers)
        .await
        .expect("stop runs");
    assert_eq!(0, still_running);
    runtime::shutdown();
}
