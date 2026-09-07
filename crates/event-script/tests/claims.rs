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

//! Claims-registry engine pins (ADR-0023): each scenario pins exactly what a
//! registered documentation claim states — no more.
//!
//! * `decision-invalid-value` — a NON-boolean decision value coerces via
//!   `str2int(...).max(1)`: a non-numeric string (even the string "false")
//!   yields -1, clamped to 1, and SILENTLY routes to the FIRST branch with no
//!   error; a missing/null decision aborts the flow with status 500 and a
//!   message containing 'invalid decision' (executor `handle_decision`).
//! * `flows-location-default` — a flows manifest that OMITS `location` loads
//!   its flow files from the compiler default `classpath:/flows/`.
//!
//! This binary compiles ONLY `claims-flows.yaml` (a process-local
//! `yaml.flow.automation` override), so the fixture manifests and their
//! exact-set assertions in compiler.rs / flow_runtime.rs are untouched.
//!
//! One `#[tokio::test]` wrapper on purpose (the global platform's workers
//! live on this test's runtime — the flow_runtime.rs pattern); the per-claim
//! scenarios run sequentially inside it.

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use event_script::conversions::from_json;
use event_script::{flows, FlowExecutor};
use platform_core::{
    main_application, preload, trace, AppError, AutoStart, ComposableFunction, EntryPoint,
    EventEnvelope, Platform,
};

/// Echoes its input body VERBATIM. As the decision task's process, the
/// caller-supplied `decision` value reaches the engine's decision router
/// unconverted — the Java-parity `SimpleDecision` fixture converts to a real
/// boolean first, so it can never exercise the string-coercion path.
#[preload(route = "claims.decision.echo", instances = 10)]
struct ClaimsDecisionEcho;

#[async_trait]
impl ComposableFunction for ClaimsDecisionEcho {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Ok(EventEnvelope::new().set_raw_body(input.body().clone()))
    }
}

#[main_application]
struct ClaimsTestApp;

#[async_trait]
impl EntryPoint for ClaimsTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

// ---- helpers ----

/// One-time process setup (the same `Once` pattern as flow_runtime.rs):
/// pin the resource root and the claims-only manifest BEFORE the first
/// config touch freezes the snapshot.
fn setup_config() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        platform_core::resources::prepend_resource_root("tests/resources");
        // this binary loads ONLY the claims manifest — the process override
        // keeps the fixture manifests (and their exact-set assertions in
        // other test binaries) out of this registry and vice versa
        platform_core::overrides::set("yaml.flow.automation", "classpath:/claims-flows.yaml");
        let holding =
            std::env::temp_dir().join(format!("mercury-claims-test-{}", std::process::id()));
        platform_core::overrides::set("transient.data.store", &holding.display().to_string());
        let _ = platform_core::AppConfigReader::get_instance();
    });
}

async fn run_flow(
    platform: &Platform,
    flow_id: &str,
    dataset: serde_json::Value,
    cid: &str,
) -> EventEnvelope {
    FlowExecutor::request(
        platform,
        flow_id,
        from_json(&dataset),
        cid,
        Duration::from_secs(8),
        Some((&trace::new_trace_id(), &format!("FLOW /{flow_id}"))),
    )
    .await
    .unwrap_or_else(|e| panic!("flow {flow_id} failed: {} {}", e.status(), e.message()))
}

fn decision_dataset(decision: Option<&str>) -> serde_json::Value {
    let mut query = serde_json::Map::new();
    if let Some(d) = decision {
        query.insert("decision".to_string(), serde_json::Value::String(d.into()));
    }
    serde_json::json!({
        "body": {},
        "header": {"accept": "application/json"},
        "query": query,
        "method": "GET",
    })
}

fn json_body(reply: &EventEnvelope) -> serde_json::Value {
    reply.body_as().expect("json body")
}

// ---- the claims pins ----

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn claims_pins_run_end_to_end() {
    setup_config();
    AutoStart::main(vec![]).await.expect("lifecycle");
    let platform = Platform::get_instance();
    non_numeric_decision_silently_routes_to_branch_one(&platform).await;
    manifest_without_location_loads_flows_from_default_classpath(&platform).await;
}

/// Claim `decision-invalid-value`: engine `handle_decision` coerces any
/// non-boolean decision with `str2int(display(value)).max(1)`.
async fn non_numeric_decision_silently_routes_to_branch_one(platform: &Platform) {
    // (a) a non-numeric string: str2int("pizza") = -1, .max(1) = 1 — the
    // FIRST branch runs, silently, with no error
    let reply = run_flow(
        platform,
        "claims-decision-coercion",
        decision_dataset(Some("pizza")),
        "claims-cid-1",
    )
    .await;
    assert_eq!(
        reply.status(),
        200,
        "a non-numeric decision must not abort the flow: {:?}",
        reply.body()
    );
    assert_eq!(
        json_body(&reply)["from"],
        "one",
        "non-numeric decision must coerce to the FIRST branch"
    );

    // the STRING "false" is not the boolean false on this path — it is
    // non-numeric too, so it also routes to branch one (not branch two)
    let reply = run_flow(
        platform,
        "claims-decision-coercion",
        decision_dataset(Some("false")),
        "claims-cid-2",
    )
    .await;
    assert_eq!(reply.status(), 200, "string 'false' must not abort");
    assert_eq!(
        json_body(&reply)["from"],
        "one",
        "the string \"false\" coerces to 1 (first branch), not to branch two"
    );

    // sanity: a genuine numeric string still selects the 1-based branch
    let reply = run_flow(
        platform,
        "claims-decision-coercion",
        decision_dataset(Some("2")),
        "claims-cid-3",
    )
    .await;
    assert_eq!(json_body(&reply)["from"], "two", "numeric routing intact");

    // (b) a MISSING decision (no query parameter -> no 'decision' element in
    // the dataset) aborts the flow: status 500, message 'invalid decision'
    let reply = run_flow(
        platform,
        "claims-decision-coercion",
        decision_dataset(None),
        "claims-cid-4",
    )
    .await;
    assert_eq!(reply.status(), 500, "missing decision must abort the flow");
    let body = json_body(&reply);
    assert_eq!(body["type"], "error");
    assert!(
        body["message"]
            .as_str()
            .unwrap_or("")
            .contains("invalid decision"),
        "abort message must name the invalid decision: {body}"
    );
}

/// Claim `flows-location-default`: `claims-flows.yaml` omits `location`, and
/// its listed flow files exist ONLY under tests/resources/flows/ — the folder
/// the compiler default `classpath:/flows/` resolves to.
async fn manifest_without_location_loads_flows_from_default_classpath(platform: &Platform) {
    assert!(
        flows::flow_exists("claims-default-location"),
        "a manifest without 'location' must load its flows from classpath:/flows/"
    );
    assert!(
        flows::get_all_flows().contains(&"claims-default-location".to_string()),
        "the default-located flow must be in the registry listing"
    );
    // and the flow executes end to end
    let reply = run_flow(
        platform,
        "claims-default-location",
        serde_json::json!({"body": {}}),
        "claims-cid-loc",
    )
    .await;
    assert_eq!(reply.status(), 200, "flow body: {:?}", reply.body());
    assert_eq!(json_body(&reply)["value"], "default-location");
}
