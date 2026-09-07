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

//! Claims-registry engine pin (ADR-0023) — `extension-target-asymmetry`:
//! `graph.extension` validates a `flow://` target against the flow registry
//! BEFORE launching anything (a missing flow aborts the run — 400, "does not
//! exist" — and the node's `exception=` handler is NOT consulted), while a
//! GRAPH-id target is only discovered missing by the delegated
//! `graph-executor` flow at run time (404, "not found") — an ordinary
//! delegate failure that DOES route to the `exception=` handler with the
//! staged error context (`error.code` / `error.message`).
//!
//! This binary compiles ONLY `claims-graphs.yaml` (a process-local
//! `graph.model.automation` override), so `graphs.yaml` and its exact-count
//! assertion in compiler.rs are untouched.
//!
//! One `#[tokio::test]` wrapper on purpose (the global platform's workers
//! live on this test's runtime — the graph_runtime.rs pattern).

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use async_trait::async_trait;
use event_script::FlowExecutor;
use platform_core::{
    main_application, preload, trace, AppError, AutoStart, ComposableFunction, EntryPoint,
    EventEnvelope, Platform,
};

/// Every invocation of the exception-handler node lands here: the pin
/// asserts the handler is NEVER reached on the flow:// pre-flight failure
/// and reached EXACTLY ONCE (with the staged error context) on the
/// missing-graph delegate failure.
static PROBE_CALLS: Mutex<Vec<serde_json::Value>> = Mutex::new(Vec::new());

#[preload(route = "claims.error.probe", instances = 10)]
struct ClaimsErrorProbe;

#[async_trait]
impl ComposableFunction for ClaimsErrorProbe {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        PROBE_CALLS.lock().expect("probe log").push(body.clone());
        EventEnvelope::new().set_body(body)
    }
}

#[main_application]
struct ClaimsGraphTestApp;

#[async_trait]
impl EntryPoint for ClaimsGraphTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

// ---- helpers (graph_runtime.rs conventions) ----

async fn boot() -> Platform {
    platform_core::resources::prepend_resource_root("tests/resources");
    // this binary compiles ONLY the claims graphs — the process override keeps
    // graphs.yaml (and its exact-count assertion in compiler.rs) untouched
    platform_core::overrides::set("graph.model.automation", "classpath:/claims-graphs.yaml");
    AutoStart::main(vec![]).await.expect("lifecycle");
    Platform::get_instance()
}

/// The Java `runGraph` analog: POST /api/graph/{graph-id} through the
/// graph-executor flow.
async fn run_graph(platform: &Platform, graph_id: &str, body: serde_json::Value) -> EventEnvelope {
    let dataset = serde_json::json!({
        "body": body,
        "header": {},
        "path_parameter": {"graph_id": graph_id},
        "method": "POST",
    });
    FlowExecutor::request(
        platform,
        "graph-executor",
        event_script::conversions::from_json(&dataset),
        &format!("cid-{graph_id}"),
        Duration::from_secs(8),
        Some((&trace::new_trace_id(), &format!("TEST /graph/{graph_id}"))),
    )
    .await
    .unwrap_or_else(|e| panic!("graph {graph_id} failed: {} {}", e.status(), e.message()))
}

fn body_text(reply: &EventEnvelope) -> String {
    let body: serde_json::Value = reply
        .body_as()
        .unwrap_or_else(|_| serde_json::Value::String(format!("{:?}", reply.body())));
    serde_json::to_string(&body).unwrap_or_default()
}

// ---- the claims pin ----

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn claims_pins_run_end_to_end() {
    let platform = boot().await;
    extension_missing_flow_aborts_but_missing_graph_routes_to_handler(&platform).await;
}

/// Claim `extension-target-asymmetry` (extension.rs `build_forward`,
/// executor.rs `handle_skill_response`).
async fn extension_missing_flow_aborts_but_missing_graph_routes_to_handler(platform: &Platform) {
    // precondition: both claims graphs passed the CompileGraph gate
    assert!(knowledge_graph::graphs::graph_exists(
        "claims-ext-missing-flow"
    ));
    assert!(knowledge_graph::graphs::graph_exists(
        "claims-ext-missing-graph"
    ));

    // (a) extension=flow://no-such-flow: the flow registry is consulted
    // BEFORE any launch — the skill fails 400 with "does not exist" and the
    // run aborts; the node's exception= handler is NOT invoked
    let response = run_graph(platform, "claims-ext-missing-flow", serde_json::json!({})).await;
    assert_eq!(
        400,
        response.status(),
        "a missing flow:// target must abort the run: {:?}",
        response.body()
    );
    let text = body_text(&response);
    assert!(
        text.contains("does not exist"),
        "abort message must say the flow does not exist: {text}"
    );
    assert!(
        PROBE_CALLS.lock().expect("probe log").is_empty(),
        "the exception= handler must NOT be invoked on the flow:// pre-flight failure"
    );

    // (b) extension=<nonexistent graph id>: the delegate rides the
    // graph-executor flow and fails at run time with 404 "not found" — an
    // ordinary delegate failure that DOES route to the exception= handler,
    // which sees the staged error context
    let response = run_graph(platform, "claims-ext-missing-graph", serde_json::json!({})).await;
    assert_eq!(
        200,
        response.status(),
        "the handler must recover the run: {:?}",
        response.body()
    );
    let body: serde_json::Value = response.body_as().expect("json body");
    assert_eq!(
        body["code"], 404,
        "the handler must see error.code = 404: {body}"
    );
    let message = serde_json::to_string(&body["message"]).unwrap_or_default();
    assert!(
        message.contains("not found"),
        "the handler must see error.message containing 'not found': {body}"
    );
    let calls = PROBE_CALLS.lock().expect("probe log");
    assert_eq!(
        1,
        calls.len(),
        "the exception= handler must run exactly once (and only for the graph case)"
    );
    assert_eq!(calls[0]["code"], 404, "staged context reached the handler");
}
