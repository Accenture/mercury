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

//! Integration tests for increment 5 — distributed tracing (OpenTelemetry-
//! compatible span lineage), business correlation-id propagation, and the
//! application log context.
//!
//! Each test uses an **isolated** `Platform::new()` and registers a capture
//! function at `distributed.tracing`, so the telemetry datasets emitted by the
//! route workers can be asserted directly.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::util::config_reader::ConfigReader;
use platform_core::{
    logging::LogContextConfig, overrides, resources, trace, AppConfigReader, AppError,
    ComposableFunction, EventEnvelope, Platform, PostOffice,
};

fn setup_config() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let holding =
            std::env::temp_dir().join(format!("mercury-telemetry-test-{}", std::process::id()));
        overrides::set("transient.data.store", &holding.display().to_string());
        let _ = AppConfigReader::get_instance();
    });
}

/// Captures every telemetry dataset delivered to `distributed.tracing`.
struct TelemetryCapture {
    datasets: Arc<Mutex<Vec<serde_json::Value>>>,
}

#[async_trait]
impl ComposableFunction for TelemetryCapture {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if let Ok(dataset) = input.body_as::<serde_json::Value>() {
            self.datasets.lock().expect("capture mutex").push(dataset);
        }
        Ok(EventEnvelope::new())
    }
}

/// A hop in a traced call chain: records its correlation id, optionally
/// annotates the trace, and optionally calls the next hop via RPC — trace
/// propagation across hops is automatic (the task-local trace bracket).
/// Receives its (isolated, per-test) platform through the constructor.
struct Hop {
    platform: Platform,
    next: Option<&'static str>,
    seen_cid: Arc<Mutex<Option<String>>>,
    annotate: Option<(&'static str, &'static str)>,
}

#[async_trait]
impl ComposableFunction for Hop {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let po = PostOffice::new(&self.platform);
        *self.seen_cid.lock().expect("cid mutex") = po.my_correlation_id();
        if let Some((key, value)) = self.annotate {
            po.annotate_trace(key, value);
        }
        if let Some(next) = self.next {
            // no trace fields set here — propagation is automatic from the
            // task-local trace bracket
            let request = EventEnvelope::new().set_to(next).set_body("ping")?;
            let _ = po.request(request, Duration::from_secs(2)).await?;
        }
        Ok(EventEnvelope::new().set_body("done")?)
    }
}

fn capture_platform() -> (Platform, Arc<Mutex<Vec<serde_json::Value>>>) {
    let platform = Platform::new();
    let datasets = Arc::new(Mutex::new(Vec::new()));
    platform
        .register(
            "distributed.tracing",
            Arc::new(TelemetryCapture {
                datasets: datasets.clone(),
            }),
            1,
        )
        .unwrap();
    (platform, datasets)
}

async fn wait_for_datasets(
    datasets: &Arc<Mutex<Vec<serde_json::Value>>>,
    expected: usize,
) -> Vec<serde_json::Value> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    loop {
        let now = datasets.lock().expect("capture mutex").clone();
        if now.len() >= expected {
            return now;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "timed out waiting for {expected} telemetry datasets, have {}",
            now.len()
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

fn trace_of(dataset: &serde_json::Value) -> &serde_json::Map<String, serde_json::Value> {
    dataset["trace"].as_object().expect("trace object")
}

#[tokio::test]
async fn traced_request_produces_span_lineage_across_two_hops() {
    setup_config();
    let (platform, datasets) = capture_platform();
    let cid_a = Arc::new(Mutex::new(None));
    let cid_b = Arc::new(Mutex::new(None));
    platform
        .register(
            "v1.second.hop",
            Arc::new(Hop {
                platform: platform.clone(),
                next: None,
                seen_cid: cid_b.clone(),
                annotate: None,
            }),
            1,
        )
        .unwrap();
    platform
        .register(
            "v1.first.hop",
            Arc::new(Hop {
                platform: platform.clone(),
                next: Some("v1.second.hop"),
                seen_cid: cid_a.clone(),
                annotate: Some(("business.step", "checkout")),
            }),
            1,
        )
        .unwrap();
    let po = PostOffice::new(&platform);
    let trace_id = trace::new_trace_id();
    let request = EventEnvelope::new()
        .set_to("v1.first.hop")
        .set_trace(&trace_id, "GET /api/first")
        .set_correlation_id("order-42")
        .set_body("go")
        .unwrap();
    let response = po.request(request, Duration::from_secs(5)).await.unwrap();
    // the response carries the trace context back (applyTraceContext parity)
    assert_eq!(response.trace_id(), Some(trace_id.as_str()));
    assert!(response.span_id().is_some());
    // EXACTLY ONE record per span (Java WorkerHandler suppression): both hops
    // are RPC-served, so each span's single record is the caller-side RPC
    // record carrying round_trip — the test's request for v1.first.hop and
    // the first hop's nested request for v1.second.hop
    let _ = wait_for_datasets(&datasets, 2).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    let all = datasets.lock().expect("capture mutex").clone();
    assert_eq!(all.len(), 2, "one record per span, no worker duplicates");
    assert!(all.iter().all(|d| trace_of(d).contains_key("round_trip")));
    let first = all
        .iter()
        .find(|d| trace_of(d)["service"] == "v1.first.hop")
        .expect("first hop record");
    let second = all
        .iter()
        .find(|d| trace_of(d)["service"] == "v1.second.hop")
        .expect("second hop record");
    // same trace, OTel-shaped ids; span_id is each callee's OWN span,
    // adopted from the direct responder's reply (spanIdFromResponder)
    assert_eq!(trace_of(first)["id"], serde_json::json!(trace_id));
    assert_eq!(trace_of(second)["id"], serde_json::json!(trace_id));
    let first_span = trace_of(first)["span_id"].as_str().unwrap();
    assert_eq!(first_span.len(), 16);
    // lineage: the second hop's parent is the first hop's span
    assert_eq!(
        trace_of(second)["parent_span_id"].as_str().unwrap(),
        first_span
    );
    // spans stay unique across the trace (Java SpanPropagationTest invariant)
    assert_ne!(trace_of(second)["span_id"], trace_of(first)["span_id"]);
    // the second hop's caller is recorded; the test itself has no route
    assert_eq!(trace_of(second)["from"], serde_json::json!("v1.first.hop"));
    assert!(!trace_of(first).contains_key("from"));
    // timing + success + path present
    assert!(trace_of(first)["exec_time"].is_number());
    assert!(trace_of(first)["round_trip"].is_number());
    assert_eq!(trace_of(first)["success"], serde_json::json!(true));
    assert_eq!(trace_of(first)["path"], serde_json::json!("GET /api/first"));
    assert!(trace_of(first)["start"].as_str().unwrap().ends_with('Z'));
    // annotations ride the reply envelope and fold into the span's single
    // record (Java: applyTraceContext -> inbox recordTrace); the caller's
    // returned envelope no longer carries them
    assert_eq!(
        first["annotations"]["business.step"],
        serde_json::json!("checkout")
    );
    assert!(response.annotations().is_empty());
    // business correlation-id propagated to BOTH hops
    assert_eq!(cid_a.lock().unwrap().as_deref(), Some("order-42"));
    assert_eq!(cid_b.lock().unwrap().as_deref(), Some("order-42"));
}

#[tokio::test]
async fn untraced_request_emits_no_telemetry() {
    setup_config();
    let (platform, datasets) = capture_platform();
    let cid = Arc::new(Mutex::new(None));
    platform
        .register(
            "v1.untraced",
            Arc::new(Hop {
                platform: platform.clone(),
                next: None,
                seen_cid: cid.clone(),
                annotate: None,
            }),
            1,
        )
        .unwrap();
    let po = PostOffice::new(&platform);
    let request = EventEnvelope::new()
        .set_to("v1.untraced")
        .set_body("go")
        .unwrap();
    po.request(request, Duration::from_secs(2)).await.unwrap();
    tokio::time::sleep(Duration::from_millis(150)).await;
    assert!(
        datasets.lock().unwrap().is_empty(),
        "no trace = no telemetry"
    );
    // outside a trace, the business APIs are silent no-ops
    assert_eq!(*cid.lock().unwrap(), None);
}

#[tokio::test]
async fn failed_execution_is_reported_with_exception() {
    setup_config();
    struct Fails;
    #[async_trait]
    impl ComposableFunction for Fails {
        async fn handle_event(
            &self,
            _h: HashMap<String, String>,
            _i: EventEnvelope,
            _n: usize,
        ) -> Result<EventEnvelope, AppError> {
            Err(AppError::new(404, "not here"))
        }
    }
    let (platform, datasets) = capture_platform();
    platform
        .register("v1.fails.traced", Arc::new(Fails), 1)
        .unwrap();
    let po = PostOffice::new(&platform);
    let request = EventEnvelope::new()
        .set_to("v1.fails.traced")
        .set_trace(&trace::new_trace_id(), "GET /api/fails")
        .set_body("go")
        .unwrap();
    let response = po.request(request, Duration::from_secs(2)).await.unwrap();
    assert_eq!(response.status(), 404);
    let all = wait_for_datasets(&datasets, 1).await;
    let t = trace_of(&all[0]);
    assert_eq!(t["success"], serde_json::json!(false));
    assert_eq!(t["status"], serde_json::json!(404));
    assert_eq!(t["exception"], serde_json::json!("not here"));
}

#[tokio::test]
async fn update_context_rejects_reserved_keys() {
    setup_config();
    let po = PostOffice::new(&Platform::new());
    for reserved in trace::RESERVED_KEYS {
        assert!(po.update_context(reserved, "x").is_err());
    }
    // non-reserved key outside a trace: silent no-op, not an error
    assert!(po.update_context("user", "demo").is_ok());
}

#[test]
fn log_context_config_renders_tokens_constants_and_custom_keys() {
    setup_config();
    // build the template from an in-memory reader (the Java package-private path)
    let yaml = r#"
context:
  cid: $cid
  traceId: $traceId
  parentSpanId: $parentSpanId
  service: $service
  environment: '${PC_UNSET_ENV_VAR:dev}'
  hello: world
  bogus: $notAToken
"#;
    let dir = std::env::temp_dir().join(format!("pc-logctx-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("app-log-context.yaml");
    std::fs::write(&file, yaml).unwrap();
    let reader = ConfigReader::load(&format!("file:{}", file.display())).unwrap();
    let config = LogContextConfig::from_reader(&reader);
    assert!(config.is_enabled());
    let mut state = trace::TraceState::new("v1.demo", &trace::new_trace_id(), "GET /x", None, None);
    state.cid = Some("order-1".into());
    state
        .custom_log_keys
        .insert("user".into(), serde_json::json!("demo"));
    let rendered = config.render(&state, std::time::SystemTime::now());
    assert_eq!(rendered["cid"], serde_json::json!("order-1"));
    assert_eq!(rendered["service"], serde_json::json!("v1.demo"));
    assert_eq!(rendered["environment"], serde_json::json!("dev"));
    assert_eq!(rendered["hello"], serde_json::json!("world"));
    assert_eq!(rendered["user"], serde_json::json!("demo"));
    // absent token omitted, never null; invalid token skipped (advisory)
    assert!(!rendered.contains_key("parentSpanId"));
    assert!(!rendered.contains_key("bogus"));
    std::fs::remove_dir_all(&dir).ok();
}

// ---- increment 51: trace continuity (parity findings F3 / F7 / F8) ----

/// (trace id, trace path, span id) as observed on one delivered event.
type SeenTraces = Arc<Mutex<Vec<(Option<String>, Option<String>, Option<String>)>>>;

/// Records the trace identity each delivered event carries.
struct TraceRecorder {
    seen: SeenTraces,
}

#[async_trait]
impl ComposableFunction for TraceRecorder {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        self.seen.lock().expect("recorder mutex").push((
            input.trace_id().map(str::to_string),
            input.trace_path().map(str::to_string),
            input.span_id().map(str::to_string),
        ));
        EventEnvelope::new().set_body("ok")
    }
}

/// A zero-traced middle hop: calls the next route and returns its reply.
struct ZeroTracedRelay {
    platform: Platform,
    next: &'static str,
}

#[async_trait]
impl ComposableFunction for ZeroTracedRelay {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let po = PostOffice::new(&self.platform);
        let request = EventEnvelope::new().set_to(self.next).set_body("relay")?;
        po.request(request, Duration::from_secs(2)).await
    }
}

/// F3: a zero-traced route suppresses only its own telemetry — the trace
/// context still flows to nested calls and back on the reply (Java gates
/// startTracing/sendTracingInfo on the flag, never the propagation).
#[tokio::test]
async fn zero_traced_route_preserves_trace_continuity() {
    setup_config();
    let (platform, datasets) = capture_platform();
    let seen = Arc::new(Mutex::new(Vec::new()));
    platform
        .register(
            "v1.trace.recorder",
            Arc::new(TraceRecorder { seen: seen.clone() }),
            1,
        )
        .unwrap();
    platform
        .register_with_options(
            "v1.zero.relay",
            Arc::new(ZeroTracedRelay {
                platform: platform.clone(),
                next: "v1.trace.recorder",
            }),
            1,
            platform_core::FunctionOptions {
                zero_traced: true,
                ..Default::default()
            },
        )
        .unwrap();
    let po = PostOffice::new(&platform);
    let response = po
        .request(
            EventEnvelope::new()
                .set_to("v1.zero.relay")
                .set_trace("trace-through-zero", "GET /zero")
                .set_body("go")
                .unwrap(),
            Duration::from_secs(2),
        )
        .await
        .unwrap();
    // the reply from the zero-traced hop still carries the trace identity
    assert_eq!(response.trace_id(), Some("trace-through-zero"));
    assert_eq!(response.trace_path(), Some("GET /zero"));
    // the nested call saw the same trace id/path — with NO span from the
    // zero-traced hop (it owns no span, Java parity)
    let recorded = seen.lock().expect("recorder mutex").clone();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].0.as_deref(), Some("trace-through-zero"));
    assert_eq!(recorded[0].1.as_deref(), Some("GET /zero"));
    assert_eq!(recorded[0].2, None, "zero-traced hop must not mint a span");
    // exactly TWO telemetry datasets, both caller-side RPC records (one per
    // span — Java parity): the RELAY's nested request records the recorder's
    // span (the trace context flows through a zero-traced hop, so its
    // outbound events are traced and its inbox records like any other), and
    // the TEST's request records the relay. NO worker records: both
    // executions were RPC-served, and the zero-traced relay never emits its
    // own telemetry anyway.
    let _ = wait_for_datasets(&datasets, 2).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    let all = datasets.lock().expect("capture mutex").clone();
    assert_eq!(all.len(), 2, "one record per span, no late datasets");
    assert!(all.iter().all(|d| trace_of(d).contains_key("round_trip")));
    // the recorder's single record: its own span, adopted from the direct
    // responder's reply; NO parent — the zero-traced relay owns no span
    let recorder = all
        .iter()
        .find(|d| trace_of(d)["service"] == "v1.trace.recorder")
        .expect("the relay's RPC record about the recorder");
    assert!(trace_of(recorder)["span_id"].is_string());
    assert!(
        !trace_of(recorder).contains_key("parent_span_id"),
        "a zero-traced caller contributes no parent span"
    );
    assert_eq!(
        trace_of(recorder)["from"],
        serde_json::json!("v1.zero.relay")
    );
    // the relay's single record carries NO span_id — the relay owns no span,
    // so its reply carries none for the responder gate to adopt
    let relay = all
        .iter()
        .find(|d| trace_of(d)["service"] == "v1.zero.relay")
        .expect("the test caller's RPC record about the relay");
    assert!(
        !trace_of(relay).contains_key("span_id"),
        "a zero-traced callee contributes no span to the RPC record"
    );
}

/// A function that schedules a delayed send and returns immediately — the
/// timer fires long after the trace bracket is gone.
struct Scheduler {
    platform: Platform,
}

#[async_trait]
impl ComposableFunction for Scheduler {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let po = PostOffice::new(&self.platform);
        let event = EventEnvelope::new()
            .set_to("v1.trace.recorder.later")
            .set_body("later")?;
        po.send_later(event, Duration::from_millis(50));
        EventEnvelope::new().set_body("scheduled")
    }
}

/// F7: send_later captures the sender's trace context AT SCHEDULING TIME
/// (Java sendLater wraps the event in touch() before the timer).
#[tokio::test]
async fn scheduled_send_captures_context_at_schedule_time() {
    setup_config();
    let (platform, _datasets) = capture_platform();
    let seen = Arc::new(Mutex::new(Vec::new()));
    platform
        .register(
            "v1.trace.recorder.later",
            Arc::new(TraceRecorder { seen: seen.clone() }),
            1,
        )
        .unwrap();
    platform
        .register(
            "v1.scheduler",
            Arc::new(Scheduler {
                platform: platform.clone(),
            }),
            1,
        )
        .unwrap();
    let po = PostOffice::new(&platform);
    let _ = po
        .request(
            EventEnvelope::new()
                .set_to("v1.scheduler")
                .set_trace("trace-scheduled", "GET /later")
                .set_body("go")
                .unwrap(),
            Duration::from_secs(2),
        )
        .await
        .unwrap();
    // wait for the timer to fire and the recorder to run
    let deadline = tokio::time::Instant::now() + Duration::from_secs(3);
    while seen.lock().expect("recorder mutex").is_empty() {
        assert!(
            tokio::time::Instant::now() < deadline,
            "scheduled event never arrived"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    let recorded = seen.lock().expect("recorder mutex").clone();
    assert_eq!(recorded[0].0.as_deref(), Some("trace-scheduled"));
    assert_eq!(recorded[0].1.as_deref(), Some("GET /later"));
    assert!(recorded[0].2.is_some(), "scheduler's span rides the event");
}

/// Sends one event carrying an EXPLICIT trace identity from inside a bracket.
struct ExplicitSender {
    platform: Platform,
}

#[async_trait]
impl ComposableFunction for ExplicitSender {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let po = PostOffice::new(&self.platform);
        let request = EventEnvelope::new()
            .set_to("v1.trace.recorder.explicit")
            .set_trace("explicit-id", "EXPLICIT /path")
            .set_body("x")?;
        let _ = po.request(request, Duration::from_secs(2)).await?;
        EventEnvelope::new().set_body("sent")
    }
}

/// F8: an explicitly supplied trace identity survives the ambient bracket
/// (Java touch() fills trace id/path only when absent).
#[tokio::test]
async fn explicit_trace_identity_survives_ambient_bracket() {
    setup_config();
    let (platform, _datasets) = capture_platform();
    let seen = Arc::new(Mutex::new(Vec::new()));
    platform
        .register(
            "v1.trace.recorder.explicit",
            Arc::new(TraceRecorder { seen: seen.clone() }),
            1,
        )
        .unwrap();
    platform
        .register(
            "v1.explicit.sender",
            Arc::new(ExplicitSender {
                platform: platform.clone(),
            }),
            1,
        )
        .unwrap();
    let po = PostOffice::new(&platform);
    let _ = po
        .request(
            EventEnvelope::new()
                .set_to("v1.explicit.sender")
                .set_trace("ambient-id", "GET /ambient")
                .set_body("go")
                .unwrap(),
            Duration::from_secs(2),
        )
        .await
        .unwrap();
    let recorded = seen.lock().expect("recorder mutex").clone();
    assert_eq!(recorded.len(), 1);
    // the explicit identity won; the ambient bracket did not overwrite it
    assert_eq!(recorded[0].0.as_deref(), Some("explicit-id"));
    assert_eq!(recorded[0].1.as_deref(), Some("EXPLICIT /path"));
    // the sender's span still rides the event (Java: span is stamped
    // unconditionally so the receiver knows its parent)
    assert!(recorded[0].2.is_some());
}

// ---- RPC telemetry span lineage (Java PostOfficeTest.rpcTelemetryCarriesSpanLineage) ----

/// Regression twin of the Java fix (commit `04e5618f`): the RPC trace record
/// (the one carrying `round_trip`) must chain like a worker-emitted record —
/// `span_id` is the callee's own span (carried on the RPC reply) and
/// `parent_span_id` is the caller's span (carried on the outbound request).
/// A traced parent function makes a nested RPC to a leaf; the leaf's RPC
/// record must carry both fields, with `parent_span_id` equal to the parent
/// function's own span.
#[tokio::test]
async fn rpc_telemetry_carries_span_lineage() {
    setup_config();
    let (platform, datasets) = capture_platform();
    struct Leaf;
    #[async_trait]
    impl ComposableFunction for Leaf {
        async fn handle_event(
            &self,
            _h: HashMap<String, String>,
            _i: EventEnvelope,
            _n: usize,
        ) -> Result<EventEnvelope, AppError> {
            EventEnvelope::new().set_body("ok")
        }
    }
    platform
        .register("span.lineage.leaf", Arc::new(Leaf), 1)
        .unwrap();
    platform
        .register(
            "span.lineage.parent",
            Arc::new(Hop {
                platform: platform.clone(),
                next: Some("span.lineage.leaf"),
                seen_cid: Arc::new(Mutex::new(None)),
                annotate: None,
            }),
            1,
        )
        .unwrap();
    let po = PostOffice::new(&platform);
    let trace_id = trace::new_trace_id();
    let response = po
        .request(
            EventEnvelope::new()
                .set_to("span.lineage.parent")
                .set_trace(&trace_id, "GET /api/span/lineage")
                .set_body("start")
                .unwrap(),
            Duration::from_secs(8),
        )
        .await
        .unwrap();
    // the reply itself now reports the measured round trip (Java parity)
    assert!(response.round_trip().is_some());
    // exactly one record per span (worker records suppressed for delivered
    // RPCs): the parent's record by the test, the leaf's by the parent
    let _ = wait_for_datasets(&datasets, 2).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    let all = datasets.lock().expect("capture mutex").clone();
    assert_eq!(all.len(), 2, "one record per span");
    let leaf_rpc = all
        .iter()
        .find(|d| {
            trace_of(d)["service"] == "span.lineage.leaf" && trace_of(d).contains_key("round_trip")
        })
        .expect("the RPC trace record for the leaf must arrive");
    let parent_rpc = all
        .iter()
        .find(|d| {
            trace_of(d)["service"] == "span.lineage.parent"
                && trace_of(d).contains_key("round_trip")
        })
        .expect("the parent's RPC record must arrive");
    let leaf = trace_of(leaf_rpc);
    assert_eq!(leaf["id"], serde_json::json!(trace_id));
    assert!(
        leaf["span_id"].is_string(),
        "the RPC record carries the callee's own span"
    );
    assert!(
        leaf["parent_span_id"].is_string(),
        "the RPC record chains onto the caller's span"
    );
    assert_eq!(
        leaf["parent_span_id"],
        trace_of(parent_rpc)["span_id"],
        "the leaf's parent is the parent function's own span"
    );
    assert_ne!(
        leaf["span_id"],
        trace_of(parent_rpc)["span_id"],
        "spans stay unique across the trace"
    );
    assert_eq!(leaf["from"], serde_json::json!("span.lineage.parent"));
    assert!(leaf["round_trip"].is_number());
    assert!(leaf["exec_time"].is_number());
    assert_eq!(leaf["path"], serde_json::json!("GET /api/span/lineage"));
    assert_eq!(leaf["success"], serde_json::json!(true));
}

/// Regression twin of Java commit `140640d8` (`InboxBase.spanIdFromResponder`,
/// caught by the flow engine's SpanPropagationTest): a RELAYED reply — one
/// answered on behalf of the requested route by a different function — must
/// NOT donate its span id to the caller's `round_trip` record. That span
/// belongs to a function that reports its own record; adopting it would
/// misattribute and duplicate it. `parent_span_id` stays unconditional.
#[tokio::test]
async fn relayed_reply_does_not_donate_its_span_to_the_rpc_record() {
    setup_config();
    let (platform, datasets) = capture_platform();
    struct Responder;
    #[async_trait]
    impl ComposableFunction for Responder {
        async fn handle_event(
            &self,
            _h: HashMap<String, String>,
            _i: EventEnvelope,
            _n: usize,
        ) -> Result<EventEnvelope, AppError> {
            EventEnvelope::new().set_body("real answer")
        }
    }
    /// An event interceptor that relays the responder's reply verbatim to the
    /// original caller — the relayed reply's `from` names the responder, not
    /// the route the caller requested (the flow-engine reply shape).
    struct RelayInterceptor {
        platform: Platform,
    }
    #[async_trait]
    impl ComposableFunction for RelayInterceptor {
        async fn handle_event(
            &self,
            _h: HashMap<String, String>,
            input: EventEnvelope,
            _n: usize,
        ) -> Result<EventEnvelope, AppError> {
            let po = PostOffice::new(&self.platform);
            let nested = po
                .request(
                    EventEnvelope::new()
                        .set_to("relay.actual.responder")
                        .set_body("x")?,
                    Duration::from_secs(2),
                )
                .await?;
            if let (Some(reply_to), Some(cid)) = (input.reply_to(), input.correlation_id()) {
                // relay: keep the nested reply's `from` (the responder)
                let relayed = nested.set_to(reply_to).set_correlation_id(cid);
                po.send(relayed).await?;
            }
            EventEnvelope::new().set_body("ignored by the worker")
        }
    }
    platform
        .register("relay.actual.responder", Arc::new(Responder), 1)
        .unwrap();
    platform
        .register_with_options(
            "relay.entry",
            Arc::new(RelayInterceptor {
                platform: platform.clone(),
            }),
            1,
            platform_core::FunctionOptions {
                interceptor: true,
                ..Default::default()
            },
        )
        .unwrap();
    let po = PostOffice::new(&platform);
    let response = po
        .request(
            EventEnvelope::new()
                .set_to("relay.entry")
                .set_trace(&trace::new_trace_id(), "GET /relay")
                .set_body("go")
                .unwrap(),
            Duration::from_secs(5),
        )
        .await
        .unwrap();
    assert_eq!(response.body_as::<String>().unwrap(), "real answer");
    // two RPC records: the responder's (by the interceptor) and the relay
    // entry's (by the test); no worker records
    let _ = wait_for_datasets(&datasets, 2).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    let all = datasets.lock().expect("capture mutex").clone();
    assert_eq!(all.len(), 2, "one record per span");
    let responder = all
        .iter()
        .find(|d| trace_of(d)["service"] == "relay.actual.responder")
        .expect("the responder's record");
    let entry = all
        .iter()
        .find(|d| trace_of(d)["service"] == "relay.entry")
        .expect("the relay entry's record");
    // the responder's span reports exactly once — on its own record
    assert!(trace_of(responder)["span_id"].is_string());
    assert!(
        !trace_of(entry).contains_key("span_id"),
        "a relayed reply must not donate its span to the caller's record"
    );
}

/// The `inbox.*` route namespace belongs to APPLICATIONS (Eric's design
/// ruling): workflow apps commonly stage work for human operators on routes
/// like `inbox.approval`. The engine reserves exactly ONE reply-listener
/// route (`temporary.inbox`) and no prefix — so a user function on an
/// `inbox.*` route registers normally, serves send and RPC, and is TRACED
/// like any user function (no zero-tracing leakage from the old prefix rule).
#[tokio::test]
async fn inbox_namespace_belongs_to_applications() {
    setup_config();
    let (platform, datasets) = capture_platform();
    struct Approval;
    #[async_trait]
    impl ComposableFunction for Approval {
        async fn handle_event(
            &self,
            _h: HashMap<String, String>,
            _i: EventEnvelope,
            _n: usize,
        ) -> Result<EventEnvelope, AppError> {
            EventEnvelope::new().set_body("approved")
        }
    }
    platform
        .register("inbox.approval", Arc::new(Approval), 2)
        .unwrap();
    assert!(platform.has_route("inbox.approval"));
    let po = PostOffice::new(&platform);
    // fire-and-forget works
    po.send(
        EventEnvelope::new()
            .set_to("inbox.approval")
            .set_body("fyi")
            .unwrap(),
    )
    .await
    .expect("send to a user inbox.* route");
    // traced RPC works AND produces a trace record — the route is a normal
    // user function, not swallowed by any reserved-prefix zero-tracing
    let trace_id = trace::new_trace_id();
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("inbox.approval")
                .set_trace(&trace_id, "GET /api/approval")
                .set_body("go")
                .unwrap(),
            Duration::from_secs(5),
        )
        .await
        .expect("rpc to a user inbox.* route");
    assert_eq!(reply.body_as::<String>().unwrap(), "approved");
    let all = wait_for_datasets(&datasets, 1).await;
    let record = all
        .iter()
        .find(|d| trace_of(d)["service"] == "inbox.approval")
        .expect("a user inbox.* route must be traced like any function");
    assert_eq!(trace_of(record)["id"], serde_json::json!(trace_id));
    assert!(trace_of(record)["span_id"].is_string());
    assert!(trace_of(record).contains_key("round_trip"));
}
