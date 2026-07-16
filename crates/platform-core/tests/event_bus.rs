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

//! Integration tests for increments 2–3 — the event-bus foundation and the
//! manager-worker back-pressure dispatch
//! (`docs/design/platform-core-port.md` §5.7 / §6).

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::{
    overrides, resources, AppConfigReader, AppError, ComposableFunction, EventEnvelope, Platform,
    PostOffice, TypedAdapter, TypedFunction,
};
use serde::{Deserialize, Serialize};
use std::sync::Once;

/// Every test calls this first: `Platform::register` reads configuration (the
/// dispatch mailbox size, the elastic queue's segment size and holding area),
/// so the resource root, spill-dir override, and singleton init must be fixed
/// before the first registration in this process.
fn setup_config() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let holding = std::env::temp_dir().join(format!("mercury-bus-test-{}", std::process::id()));
        overrides::set("transient.data.store", &holding.display().to_string());
        overrides::set("elastic.queue.segment.size.bytes", "512");
        let _ = AppConfigReader::get_instance();
    });
}

// ---- test functions ----

/// Untyped echo: replies with the request body and a worker-instance header.
struct Echo;

#[async_trait]
impl ComposableFunction for Echo {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Ok(EventEnvelope::new()
            .set_header("instance", &instance.to_string())
            .set_raw_body(input.body().clone()))
    }
}

/// Counter with configurable delay: tracks invocations and concurrency.
struct Counter {
    calls: Arc<AtomicUsize>,
    in_flight: Arc<AtomicUsize>,
    max_in_flight: Arc<AtomicUsize>,
    delay: Duration,
}

#[async_trait]
impl ComposableFunction for Counter {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let now = self.in_flight.fetch_add(1, Ordering::SeqCst) + 1;
        self.max_in_flight.fetch_max(now, Ordering::SeqCst);
        tokio::time::sleep(self.delay).await;
        self.in_flight.fetch_sub(1, Ordering::SeqCst);
        Ok(EventEnvelope::new())
    }
}

/// Always fails with a structured error (the AppException analog).
struct AlwaysFails;

#[async_trait]
impl ComposableFunction for AlwaysFails {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Err(AppError::new(404, "resource not here"))
    }
}

// typed function exercising TypedAdapter (the TypedLambdaFunction<I,O> analog)

#[derive(Serialize, Deserialize)]
struct GreetingRequest {
    user: String,
}

#[derive(Serialize, Deserialize)]
struct GreetingResponse {
    message: String,
}

struct Greeting;

#[async_trait]
impl TypedFunction<GreetingRequest, GreetingResponse> for Greeting {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: GreetingRequest,
        _instance: usize,
    ) -> Result<GreetingResponse, AppError> {
        Ok(GreetingResponse {
            message: format!("Welcome, {}", input.user),
        })
    }
}

// ---- envelope wire format ----

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Pojo {
    name: String,
    count: i64,
}

#[test]
fn envelope_round_trips_through_msgpack() {
    let pojo = Pojo {
        name: "mercury".to_string(),
        count: 42,
    };
    let envelope = EventEnvelope::new()
        .set_to("v1.get.profile")
        .set_header("x-demo", "yes")
        .set_status(201)
        .set_correlation_id("cid-1")
        .set_body(pojo.clone())
        .unwrap();
    let bytes = envelope.to_bytes().unwrap();
    let restored = EventEnvelope::from_bytes(&bytes).unwrap();
    assert_eq!(restored.id(), envelope.id());
    assert_eq!(restored.to(), Some("v1.get.profile"));
    assert_eq!(restored.header("x-demo"), Some("yes"));
    assert_eq!(restored.status(), 201);
    assert_eq!(restored.correlation_id(), Some("cid-1"));
    assert_eq!(restored.body_as::<Pojo>().unwrap(), pojo);
    assert!(!restored.has_error());
}

// ---- registry ----

#[tokio::test]
async fn route_validation_and_duplicates_are_rejected() {
    setup_config();
    let platform = Platform::new();
    assert!(platform.register("badroute", Arc::new(Echo), 1).is_err());
    assert!(platform.register("UPPER.case", Arc::new(Echo), 1).is_err());
    assert!(platform.register("v1.echo", Arc::new(Echo), 0).is_err());
    platform.register("v1.echo", Arc::new(Echo), 1).unwrap();
    assert!(platform.has_route("v1.echo"));
    // duplicate registration is refused
    assert!(platform.register("v1.echo", Arc::new(Echo), 1).is_err());
    // release closes the route
    assert!(platform.release("v1.echo"));
    assert!(!platform.has_route("v1.echo"));
    assert!(!platform.release("v1.echo"));
}

#[tokio::test]
async fn send_reaches_exactly_one_worker() {
    setup_config();
    let platform = Platform::new();
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Counter {
        calls: calls.clone(),
        in_flight: Arc::new(AtomicUsize::new(0)),
        max_in_flight: Arc::new(AtomicUsize::new(0)),
        delay: Duration::ZERO,
    };
    platform
        .register("v1.count.once", Arc::new(counter), 2)
        .unwrap();
    let po = PostOffice::new(&platform);
    po.send(EventEnvelope::new().set_to("v1.count.once"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    // point-to-point: exactly one of the two workers took it
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn send_to_unknown_route_is_404() {
    setup_config();
    let po = PostOffice::new(&Platform::new());
    let err = po
        .send(EventEnvelope::new().set_to("v1.no.where"))
        .await
        .unwrap_err();
    assert_eq!(err.status(), 404);
    let err = po.send(EventEnvelope::new()).await.unwrap_err();
    assert_eq!(err.status(), 400); // missing 'to'
}

// ---- RPC ----

#[tokio::test]
async fn rpc_round_trip_with_correlation() {
    setup_config();
    let platform = Platform::new();
    platform.register("v1.echo", Arc::new(Echo), 1).unwrap();
    let po = PostOffice::new(&platform);
    let request = EventEnvelope::new()
        .set_to("v1.echo")
        .set_correlation_id("cid-42")
        .set_body(Pojo {
            name: "ping".to_string(),
            count: 1,
        })
        .unwrap();
    let response = po.request(request, Duration::from_secs(2)).await.unwrap();
    assert_eq!(response.body_as::<Pojo>().unwrap().name, "ping");
    // the reply carries the request's correlation id (inbox correlation)
    assert_eq!(response.correlation_id(), Some("cid-42"));
    assert_eq!(response.from(), Some("v1.echo"));
    assert!(response.exec_time().is_some());
    // the temporary inbox was released
    assert_eq!(platform.routes(), vec!["v1.echo".to_string()]);
}

#[tokio::test]
async fn typed_function_via_adapter() {
    setup_config();
    let platform = Platform::new();
    platform
        .register("v1.greeting", TypedAdapter::arc(Greeting), 1)
        .unwrap();
    let po = PostOffice::new(&platform);
    let request = EventEnvelope::new()
        .set_to("v1.greeting")
        .set_body(GreetingRequest {
            user: "eric".to_string(),
        })
        .unwrap();
    let response = po.request(request, Duration::from_secs(2)).await.unwrap();
    let body: GreetingResponse = response.body_as().unwrap();
    assert_eq!(body.message, "Welcome, eric");
}

#[tokio::test]
async fn rpc_timeout_is_408() {
    setup_config();
    let platform = Platform::new();
    let slow = Counter {
        calls: Arc::new(AtomicUsize::new(0)),
        in_flight: Arc::new(AtomicUsize::new(0)),
        max_in_flight: Arc::new(AtomicUsize::new(0)),
        delay: Duration::from_millis(500),
    };
    platform.register("v1.slow", Arc::new(slow), 1).unwrap();
    let po = PostOffice::new(&platform);
    let err = po
        .request(
            EventEnvelope::new().set_to("v1.slow"),
            Duration::from_millis(100),
        )
        .await
        .unwrap_err();
    assert_eq!(err.status(), 408);
    // the temporary inbox was released even on timeout
    assert_eq!(platform.routes(), vec!["v1.slow".to_string()]);
}

#[tokio::test]
async fn function_error_becomes_response_status() {
    setup_config();
    let platform = Platform::new();
    platform
        .register("v1.fails", Arc::new(AlwaysFails), 1)
        .unwrap();
    let po = PostOffice::new(&platform);
    let response = po
        .request(
            EventEnvelope::new().set_to("v1.fails"),
            Duration::from_secs(2),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
    assert!(response.has_error());
    assert_eq!(response.body_as::<String>().unwrap(), "resource not here");
}

// ---- concurrency ----

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn n_instances_bound_concurrency() {
    setup_config();
    let platform = Platform::new();
    let calls = Arc::new(AtomicUsize::new(0));
    let in_flight = Arc::new(AtomicUsize::new(0));
    let max_in_flight = Arc::new(AtomicUsize::new(0));
    let counter = Counter {
        calls: calls.clone(),
        in_flight,
        max_in_flight: max_in_flight.clone(),
        delay: Duration::from_millis(100),
    };
    platform.register("v1.pool", Arc::new(counter), 4).unwrap();
    let po = PostOffice::new(&platform);
    let requests = (0..8).map(|_| {
        let po = po.clone();
        async move {
            po.request(
                EventEnvelope::new().set_to("v1.pool"),
                Duration::from_secs(5),
            )
            .await
        }
    });
    let results = futures_join_all(requests).await;
    assert!(results.iter().all(Result::is_ok));
    assert_eq!(calls.load(Ordering::SeqCst), 8);
    let max = max_in_flight.load(Ordering::SeqCst);
    assert!(max <= 4, "at most 4 workers in flight, saw {max}");
    assert!(max >= 2, "expected observable concurrency, saw {max}");
}

/// Minimal join_all (avoids a futures-crate dependency for one call site).
async fn futures_join_all<F, T>(futures: impl IntoIterator<Item = F>) -> Vec<T>
where
    F: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let handles: Vec<_> = futures.into_iter().map(tokio::spawn).collect();
    let mut out = Vec::with_capacity(handles.len());
    for handle in handles {
        out.push(handle.await.expect("task panicked"));
    }
    out
}

// ---- back-pressure (increment 3: manager-worker + elastic FIFO) ----

/// Slow single consumer: records each event's sequence number in arrival order.
struct SlowRecorder {
    seen: Arc<Mutex<Vec<u64>>>,
    delay: Duration,
}

#[async_trait]
impl ComposableFunction for SlowRecorder {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let n: u64 = input.body_as().unwrap_or(u64::MAX);
        tokio::time::sleep(self.delay).await;
        self.seen.lock().expect("recorder mutex").push(n);
        Ok(EventEnvelope::new())
    }
}

fn segment_files(route_fragment: &str) -> Vec<String> {
    std::fs::read_dir(platform_core::util::elastic_queue::base_dir())
        .map(|entries| {
            entries
                .flatten()
                .map(|e| e.file_name().to_string_lossy().to_string())
                .filter(|n| {
                    n.starts_with("eq-") && n.contains(route_fragment) && n.ends_with(".dat")
                })
                .collect()
        })
        .unwrap_or_default()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn slow_consumer_spills_to_disk_and_preserves_fifo_order() {
    setup_config();
    let platform = Platform::new();
    let seen = Arc::new(Mutex::new(Vec::new()));
    let recorder = SlowRecorder {
        seen: seen.clone(),
        delay: Duration::from_millis(20),
    };
    platform
        .register("v1.slow.recorder", Arc::new(recorder), 1)
        .unwrap();
    let po = PostOffice::new(&platform);
    let total: u64 = 60;
    for n in 0..total {
        po.send(
            EventEnvelope::new()
                .set_to("v1.slow.recorder")
                .set_body(n)
                .unwrap(),
        )
        .await
        .unwrap();
    }
    // the burst outran the single worker (20 ms/event): the overflow beyond the
    // 20-event memory tier must spill to 512-byte segment files. send() returns
    // when the event is in the manager's mailbox, so poll briefly while the
    // manager drains the mailbox into the elastic queue (the ~1s backlog keeps
    // segments on disk far longer than this poll needs)
    let spill_deadline = tokio::time::Instant::now() + Duration::from_secs(2);
    while segment_files("v1.slow.recorder").is_empty() {
        assert!(
            tokio::time::Instant::now() < spill_deadline,
            "expected disk spill during the burst"
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    // wait for the full drain
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while (seen.lock().expect("recorder mutex").len() as u64) < total {
        assert!(
            tokio::time::Instant::now() < deadline,
            "drain timed out: {} of {total}",
            seen.lock().expect("recorder mutex").len()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    // a single worker + the elastic FIFO = strict end-to-end ordering
    let recorded = seen.lock().expect("recorder mutex").clone();
    assert_eq!(recorded, (0..total).collect::<Vec<_>>());
    // once drained, the elastic queue closed and every segment was reclaimed
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(
        segment_files("v1.slow.recorder").is_empty(),
        "segments should be reclaimed after the queue drains"
    );
}

// ---- config + bus layering (everything relies on config management) ----

#[tokio::test]
async fn worker_count_can_come_from_configuration() {
    setup_config();
    let instances: usize = AppConfigReader::get_instance()
        .get_property("demo.instances")
        .expect("demo.instances in application.yml")
        .parse()
        .unwrap();
    assert_eq!(instances, 3);
    let platform = Platform::new();
    platform
        .register("v1.configured", Arc::new(Echo), instances)
        .unwrap();
    assert_eq!(platform.instances("v1.configured"), Some(3));
    let po = PostOffice::new(&platform);
    let response = po
        .request(
            EventEnvelope::new().set_to("v1.configured"),
            Duration::from_secs(2),
        )
        .await
        .unwrap();
    assert!(!response.has_error());
}

// ---- manual reply (the @EventInterceptor pattern; increment 9 inbox) ----

/// Replies MANUALLY via po.send() to the request's reply_to, returning nothing
/// useful itself — the Java @EventInterceptor pattern. Works because an RPC
/// inbox is addressable through the normal delivery path (TemporaryInbox
/// parity), not only via the worker's internal reply short-circuit.
struct ManualReplier {
    platform: Platform,
}

#[async_trait]
impl ComposableFunction for ManualReplier {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if let Some(reply_to) = input.reply_to() {
            let po = PostOffice::new(&self.platform);
            let reply = EventEnvelope::new()
                .set_to(reply_to)
                .set_correlation_id(input.correlation_id().unwrap_or_default())
                .set_body("manual reply")?;
            po.send(reply).await?;
        }
        // suppress the automatic reply: without reply_to set on the returned
        // envelope... the worker would also auto-reply; return an envelope the
        // caller ignores (first reply wins on a oneshot inbox)
        EventEnvelope::new().set_body("ignored auto reply")
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn manual_reply_reaches_the_rpc_inbox() {
    setup_config();
    let platform = Platform::new();
    platform
        .register(
            "v1.manual.replier",
            Arc::new(ManualReplier {
                platform: platform.clone(),
            }),
            1,
        )
        .unwrap();
    let po = PostOffice::new(&platform);
    let response = po
        .request(
            EventEnvelope::new()
                .set_to("v1.manual.replier")
                .set_body("go")
                .unwrap(),
            Duration::from_secs(2),
        )
        .await
        .unwrap();
    // the MANUAL reply won the oneshot (it was sent before the function returned)
    assert_eq!(response.body_as::<String>().unwrap(), "manual reply");
}
