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

//! The asynchronous `graph.extension` (RFC-0011 section 8.1; the twin of the Java
//! `AsyncExtensionTest`, mercury-composable #609): the node starts its target and the parent
//! graph answers before the target ends. The target's work is `v1.async.recorder`, which
//! records each run under the business correlation id the target inherited.
//!
//! Its own test binary: one platform for the eight cases, compiling only its own manifest
//! (`async-graphs.yaml`) and loading the recorder flow beside the engine's flows.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use event_script::FlowExecutor;
use platform_core::{
    main_application, preload, trace, AppError, AutoStart, ComposableFunction, EntryPoint,
    EventEnvelope, Platform,
};

const TIMEOUT: Duration = Duration::from_secs(8);
const ADVICE: &str = "advice";
const NOT_STARTED: &str = "not started";

/// The runs recorded for one business correlation id.
#[derive(Default, Clone)]
struct Runs {
    started: Vec<serde_json::Value>,
    ended: usize,
    active: usize,
    max_active: usize,
}

fn recorder() -> &'static Mutex<HashMap<String, Runs>> {
    static RUNS: OnceLock<Mutex<HashMap<String, Runs>>> = OnceLock::new();
    RUNS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn runs(cid: &str) -> Runs {
    recorder()
        .lock()
        .expect("recorder")
        .get(cid)
        .cloned()
        .unwrap_or_default()
}

/// Wait until the runs of a cid meet a condition; false when the wait expired first.
async fn wait_for(cid: &str, timeout: Duration, condition: impl Fn(&Runs) -> bool) -> bool {
    let deadline = Instant::now() + timeout;
    loop {
        if condition(&runs(cid)) {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

/// The work an asynchronous graph.extension starts: what each run received, when it ended, and
/// how many runs were active at once. A run takes `delay` milliseconds, so a test can see that
/// the caller was answered before the work ended.
#[preload(route = "v1.async.recorder", instances = 20)]
struct AsyncRecorder;

#[async_trait]
impl ComposableFunction for AsyncRecorder {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let cid = headers
            .get("my_correlation_id")
            .cloned()
            .unwrap_or_default();
        let body: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        {
            let mut all = recorder().lock().expect("recorder");
            let runs = all.entry(cid.clone()).or_default();
            runs.started.push(body.clone());
            runs.active += 1;
            runs.max_active = runs.max_active.max(runs.active);
        }
        let delay = body["delay"].as_u64().unwrap_or(0);
        tokio::time::sleep(Duration::from_millis(delay)).await;
        {
            let mut all = recorder().lock().expect("recorder");
            let runs = all.entry(cid).or_default();
            runs.active -= 1;
            runs.ended += 1;
        }
        EventEnvelope::new().set_body(serde_json::json!({"item": body["item"].to_string()}))
    }
}

#[main_application]
struct AsyncExtensionTestApp;

#[async_trait]
impl EntryPoint for AsyncExtensionTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

async fn boot() -> Platform {
    platform_core::resources::prepend_resource_root("tests/resources");
    platform_core::overrides::set("graph.model.automation", "classpath:/async-graphs.yaml");
    platform_core::overrides::set(
        "yaml.flow.automation",
        "classpath:/flows.yaml, classpath:/async-flows.yaml",
    );
    // the test configuration's port belongs to the graph_runtime binary
    platform_core::overrides::set("rest.server.port", "0");
    AutoStart::main(vec![]).await.expect("lifecycle");
    // the manifest compiled (and the reference links the engine's functions into this binary)
    for id in [
        "unit-test-async-sub",
        "unit-test-async-1",
        "unit-test-async-2",
    ] {
        assert!(
            knowledge_graph::graphs::graph_exists(id),
            "{id} must compile"
        );
    }
    Platform::get_instance()
}

/// The Java `runGraph` analog: POST /api/graph/{graph-id} through the graph-executor flow,
/// under a business correlation id.
async fn run_graph(
    platform: &Platform,
    graph_id: &str,
    cid: &str,
    body: serde_json::Value,
) -> (i32, serde_json::Value) {
    let dataset = serde_json::json!({
        "body": body,
        "header": {},
        "path_parameter": {"graph_id": graph_id},
        "method": "POST",
    });
    match FlowExecutor::request(
        platform,
        "graph-executor",
        event_script::conversions::from_json(&dataset),
        cid,
        TIMEOUT,
        Some((&trace::new_trace_id(), &format!("TEST /graph/{graph_id}"))),
    )
    .await
    {
        Ok(reply) => (
            reply.status(),
            reply.body_as().unwrap_or(serde_json::Value::Null),
        ),
        Err(e) => (e.status(), serde_json::json!(e.message())),
    }
}

fn new_cid(name: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("async-{name}-{n}")
}

fn absent(value: &serde_json::Value) -> bool {
    value.is_null()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn an_asynchronous_extension_starts_its_target_and_continues() {
    test_support::run_at_exit(platform_core::util::elastic_queue::shutdown_cleanup);
    let platform = boot().await;
    the_parent_answers_before_its_target_ends(&platform).await;
    a_fan_out_starts_one_run_per_element_within_its_concurrency(&platform).await;
    a_flow_target_runs_under_the_node_ttl(&platform).await;
    a_target_without_a_node_ttl_runs_under_its_own(&platform).await;
    a_graph_that_is_not_deployed_is_a_failure_to_start(&platform).await;
    a_flow_that_does_not_exist_is_a_failure_to_start(&platform).await;
    an_input_the_target_refuses_is_a_failure_to_start(&platform).await;
    one_refused_element_starts_none_of_the_fan_out(&platform).await;
}

async fn the_parent_answers_before_its_target_ends(platform: &Platform) {
    let cid = new_cid("single");
    let (status, body) = run_graph(
        platform,
        "unit-test-async-1",
        &cid,
        serde_json::json!({"item": "a", "delay": 1500}),
    )
    .await;
    assert_eq!(202, status, "{body}");
    assert_eq!(ADVICE, body["type"]);
    assert_eq!(
        0,
        runs(&cid).ended,
        "the caller is answered before the target ends"
    );
    assert!(
        wait_for(&cid, TIMEOUT, |r| r.ended >= 1).await,
        "the target runs to its end after the parent answered"
    );
    let recorded = runs(&cid);
    assert_eq!(1, recorded.started.len());
    let run = &recorded.started[0];
    assert_eq!("a", run["item"]);
    assert!(
        absent(&run["index"]),
        "a single delegation has no for_each position: {run}"
    );
}

async fn a_fan_out_starts_one_run_per_element_within_its_concurrency(platform: &Platform) {
    let cid = new_cid("fan-out");
    let items = ["a", "b", "c", "d", "e"];
    let (status, body) = run_graph(
        platform,
        "unit-test-async-2",
        &cid,
        serde_json::json!({"items": items, "delay": 400}),
    )
    .await;
    assert_eq!(202, status, "{body}");
    assert_eq!(
        0,
        runs(&cid).ended,
        "the caller is answered before the runs end"
    );
    assert!(
        wait_for(&cid, TIMEOUT, |r| r.ended >= items.len()).await,
        "every element runs"
    );
    let recorded = runs(&cid);
    assert_eq!(
        2, recorded.max_active,
        "concurrency bounds the active runs, and the window stays full"
    );
    let mut seen: Vec<String> = recorded
        .started
        .iter()
        .map(|run| run["item"].as_str().unwrap_or_default().to_string())
        .collect();
    seen.sort();
    assert_eq!(items.to_vec(), seen);
    let mut positions: Vec<String> = recorded
        .started
        .iter()
        .map(|run| match &run["index"] {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        })
        .collect();
    positions.sort();
    assert_eq!(
        vec!["0", "1", "2", "3", "4"],
        positions,
        "each run carries its position in the list"
    );
}

async fn a_flow_target_runs_under_the_node_ttl(platform: &Platform) {
    let cid = new_cid("flow-node-ttl");
    let (status, body) = run_graph(
        platform,
        "unit-test-async-5",
        &cid,
        serde_json::json!({"item": "f", "delay": 300}),
    )
    .await;
    assert_eq!(202, status, "{body}");
    assert!(wait_for(&cid, TIMEOUT, |r| r.ended >= 1).await);
    let recorded = runs(&cid);
    assert_eq!("f", recorded.started[0]["item"]);
    assert_eq!(
        20000,
        recorded.started[0]["ttl"].as_i64().unwrap_or_default(),
        "the node's ttl is the target's own, not the parent's"
    );
}

async fn a_target_without_a_node_ttl_runs_under_its_own(platform: &Platform) {
    let cid = new_cid("flow-own-ttl");
    let (status, body) = run_graph(
        platform,
        "unit-test-async-6",
        &cid,
        serde_json::json!({"item": "g", "delay": 0}),
    )
    .await;
    assert_eq!(202, status, "{body}");
    assert!(wait_for(&cid, TIMEOUT, |r| r.ended >= 1).await);
    assert_eq!(
        45000,
        runs(&cid).started[0]["ttl"].as_i64().unwrap_or_default(),
        "the flow's own ttl, not the 30 seconds the parent runs under"
    );
}

async fn a_graph_that_is_not_deployed_is_a_failure_to_start(platform: &Platform) {
    let (_, handled) = run_graph(
        platform,
        "unit-test-async-3",
        &new_cid("missing-graph"),
        serde_json::json!({"item": "a"}),
    )
    .await;
    assert_eq!(NOT_STARTED, handled["stage"], "{handled}");
    assert_eq!("start", handled["source"]);
    assert_eq!(404, handled["code"]);
    assert_eq!(
        "node start - unit-test-async-missing does not exist",
        handled["message"]
    );
}

async fn a_flow_that_does_not_exist_is_a_failure_to_start(platform: &Platform) {
    let (_, handled) = run_graph(
        platform,
        "unit-test-async-4",
        &new_cid("missing-flow"),
        serde_json::json!({"item": "a"}),
    )
    .await;
    assert_eq!(NOT_STARTED, handled["stage"], "{handled}");
    assert_eq!(404, handled["code"]);
    assert_eq!(
        "node start - flow://no-such-flow does not exist",
        handled["message"]
    );
}

async fn an_input_the_target_refuses_is_a_failure_to_start(platform: &Platform) {
    let cid = new_cid("refused");
    let (_, handled) = run_graph(
        platform,
        "unit-test-async-1",
        &cid,
        serde_json::json!({"item": 42}),
    )
    .await;
    assert_eq!(NOT_STARTED, handled["stage"], "{handled}");
    assert_eq!(400, handled["code"]);
    let message = handled["message"].as_str().unwrap_or_default();
    assert!(
        message.contains("item"),
        "the validator names the field: {message}"
    );
    assert!(
        !wait_for(&cid, Duration::from_secs(1), |r| !r.started.is_empty()).await,
        "a refused input starts nothing"
    );
}

async fn one_refused_element_starts_none_of_the_fan_out(platform: &Platform) {
    let cid = new_cid("refused-element");
    let (_, handled) = run_graph(
        platform,
        "unit-test-async-2",
        &cid,
        serde_json::json!({"items": ["a", 7], "delay": 0}),
    )
    .await;
    assert_eq!(400, handled["code"], "{handled}");
    let message = handled["message"].as_str().unwrap_or_default();
    assert!(message.starts_with("for_each element 1 - "), "{message}");
    assert!(
        !wait_for(&cid, Duration::from_secs(1), |r| !r.started.is_empty()).await,
        "no element starts when one is refused"
    );
}
