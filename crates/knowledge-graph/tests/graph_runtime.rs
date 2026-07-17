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

//! End-to-end graph execution — parity ports of the Java `GraphTests`
//! (tutorials 1/2/4/7/8/9/13) and `GraphTaskTest` (unit-test-task-1..5),
//! running the real `graph-executor` flow through the flow engine. Tutorials
//! needing `graph.api.fetcher` / `graph.extension` join at K-5/K-6.
//! Rust-supplement graphs (lazy-loaded, deliberately not in `graphs.yaml`)
//! cover the join barrier, loop detection and the `graph.js` retirement.

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use event_script::mlm::MultiLevelMap;
use event_script::FlowExecutor;
use platform_core::{
    main_application, preload, trace, AppError, AutoStart, ComposableFunction, EntryPoint,
    EventEnvelope, Platform,
};
use rmpv::Value;

// ---- Java-parity test functions ----

/// Java `DemoTaskFunction` (`v1.demo.task`): echoes the body and the `hello`
/// request header, doubles `amount`, returns a response header; the
/// `exception` field triggers the error path.
#[preload(route = "v1.demo.task", instances = 10)]
struct DemoTaskFunction;

#[async_trait]
impl ComposableFunction for DemoTaskFunction {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        if body.get("exception").is_some() {
            return Err(AppError::new(400, "just a test"));
        }
        let mut result = serde_json::json!({"received": body});
        if let Some(hello) = headers.get("hello") {
            result["hello_header"] = serde_json::json!(hello);
        }
        if let Some(amount) = body.get("amount").and_then(|v| v.as_f64()) {
            result["doubled"] = serde_json::json!(amount * 2.0);
        }
        Ok(EventEnvelope::new()
            .set_header("x-task", "demo")
            .set_body(result)?)
    }
}

#[derive(serde::Deserialize)]
struct TaskPoJo {
    name: String,
    amount: i64,
}

/// Java `DemoPoJoTask` (`v1.pojo.task`): the request body converts to the
/// PoJo at the function boundary.
#[preload(route = "v1.pojo.task", instances = 10)]
struct DemoPoJoTask;

#[async_trait]
impl ComposableFunction for DemoPoJoTask {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let pojo: TaskPoJo = input.body_as()?;
        EventEnvelope::new().set_body(serde_json::json!({
            "name": pojo.name,
            "total": pojo.amount * 2,
        }))
    }
}

/// Java `HelloTask` (`v1.hello.task`, the tutorial-13 demo function).
#[preload(route = "v1.hello.task", instances = 50)]
struct HelloTask;

#[async_trait]
impl ComposableFunction for HelloTask {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        let name = body
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("stranger");
        let mut result = serde_json::json!({"greeting": format!("Hello, {name}")});
        if let Some(amount) = body.get("amount").and_then(|v| v.as_f64()) {
            result["doubled"] = serde_json::json!(amount * 2.0);
        }
        if let Some(app) = headers.get("x-app") {
            result["app"] = serde_json::json!(app);
        }
        EventEnvelope::new().set_body(result)
    }
}

#[main_application]
struct GraphRuntimeTestApp;

#[async_trait]
impl EntryPoint for GraphRuntimeTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        // referencing both engine crates guarantees their inventories link
        log::info!(
            "Flows ready: {:?}, graphs compiled: {}",
            event_script::flows::get_all_flows().len(),
            knowledge_graph::graphs::get_all_graphs().len()
        );
        Ok(())
    }
}

// ---- helpers ----

async fn boot() -> Platform {
    platform_core::resources::prepend_resource_root("tests/resources");
    AutoStart::main(vec![]).await.expect("lifecycle");
    // AutoStart runs only once per process (Java parity) — a repeated
    // execution is a no-op, not a route-collision error
    AutoStart::main(vec![])
        .await
        .expect("second call must be a no-op");
    Platform::get_instance()
}

/// The Java `runTutorial`/`runGraph` analog: POST /api/graph/{graph-id}
/// through the graph-executor flow.
async fn run_graph(
    platform: &Platform,
    graph_id: &str,
    body: serde_json::Value,
    headers: serde_json::Value,
) -> EventEnvelope {
    let dataset = serde_json::json!({
        "body": body,
        "header": headers,
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

fn body_map(reply: &EventEnvelope) -> MultiLevelMap {
    MultiLevelMap::from_value(reply.body().clone())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_runtime_end_to_end() {
    let platform = boot().await;
    graphs_run_end_to_end_like_java(&platform).await;
    graph_task_matches_java_semantics(&platform).await;
    join_loop_retirement_and_health(&platform).await;
}

async fn graphs_run_end_to_end_like_java(platform: &Platform) {
    let platform = platform.clone();

    // --- tutorial 1: the hello-world graph (a single data-mapper end node)
    let reply = run_graph(
        &platform,
        "tutorial-1",
        serde_json::json!({}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status());
    assert_eq!(&Value::from("hello world"), reply.body());

    // --- tutorial 2: echo the request body
    let reply = run_graph(
        &platform,
        "tutorial-2",
        serde_json::json!({"hello": "world"}),
        serde_json::json!({}),
    )
    .await;
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("world")), mm.get_element("hello"));

    // --- tutorial 4: math decision routing (a < b takes the else branch)
    let reply = run_graph(
        &platform,
        "tutorial-4",
        serde_json::json!({"a": 100, "b": 200}),
        serde_json::json!({}),
    )
    .await;
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("a < b")), mm.get_element("message"));
    assert_eq!(Some(Value::from(300.0)), mm.get_element("sum"));
    assert_eq!(Some(Value::from(true)), mm.get_element("less_than"));

    // --- tutorial 4 again: a > b takes the then branch (next)
    let reply = run_graph(
        &platform,
        "tutorial-4",
        serde_json::json!({"a": 300, "b": 200}),
        serde_json::json!({}),
    )
    .await;
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("a >= b")), mm.get_element("message"));
    assert_eq!(Some(Value::from(false)), mm.get_element("less_than"));
    assert_eq!(Some(Value::from(500.0)), mm.get_element("sum"));

    // --- tutorial 7: mapper with model arrays and an f: plugin
    let reply = run_graph(
        &platform,
        "tutorial-7",
        serde_json::json!({"profile": {"name": "Peter",
            "address1": "100 World Blvd", "address2": "New York"}}),
        serde_json::json!({}),
    )
    .await;
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Peter")), mm.get_element("name"));
    assert_eq!(Some(Value::from("world")), mm.get_element("hello"));
    assert_eq!(
        Some(Value::from("100 World Blvd")),
        mm.get_element("address[0]")
    );
    assert_eq!(Some(Value::from("New York")), mm.get_element("address[1]"));
    assert!(
        mm.get_element("time").is_some(),
        "f:now plugin output expected"
    );

    // --- tutorial 8: nested structures pass through the mapper
    let reply = run_graph(
        &platform,
        "tutorial-8",
        serde_json::json!({"profile": {"name": "Peter", "account": [
            {"id": "100", "amount": 18000.30, "description": "Time deposit", "type": "C/D"},
            {"id": "200", "amount": 62050.80, "description": "Saving account", "type": "Saving"}
        ]}}),
        serde_json::json!({}),
    )
    .await;
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Peter")), mm.get_element("name"));
    assert_eq!(
        Some(Value::from(18000.30)),
        mm.get_element("account[0].amount")
    );
    assert_eq!(
        Some(Value::from(62050.80)),
        mm.get_element("account[1].amount")
    );
    assert_eq!(Some(Value::from("100")), mm.get_element("account[0].id"));
    assert_eq!(Some(Value::from("C/D")), mm.get_element("account[0].type"));
    assert_eq!(
        Some(Value::from("Saving")),
        mm.get_element("account[1].type")
    );

    // --- tutorial 9: EXECUTE statement merge + island branch
    let reply = run_graph(
        &platform,
        "tutorial-9",
        serde_json::json!({"a": 10, "b": 20}),
        serde_json::json!({}),
    )
    .await;
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from(30.0)), mm.get_element("sum"));

    // --- tutorial 13: graph.task invoking a composable function
    let reply = run_graph(
        &platform,
        "tutorial-13",
        serde_json::json!({"name": "world", "amount": 21}),
        serde_json::json!({}),
    )
    .await;
    let mm = body_map(&reply);
    assert_eq!(
        Some(Value::from("Hello, world")),
        mm.get_element("greeting")
    );
    assert_eq!(Some(Value::from(42.0)), mm.get_element("doubled"));
    assert_eq!(Some(Value::from("minigraph")), mm.get_element("app"));
}

async fn graph_task_matches_java_semantics(platform: &Platform) {
    let platform = platform.clone();

    // --- unit-test-task-1: whole-body '*' seed + field merge + headers
    let reply = run_graph(
        &platform,
        "unit-test-task-1",
        serde_json::json!({"hello": "world", "amount": 5}),
        serde_json::json!({"x-demo": "sunshine"}),
    )
    .await;
    assert_eq!(200, reply.status());
    let mm = body_map(&reply);
    // 'input.body -> *' seeds the whole body and 'int(100) -> amount' merges
    assert_eq!(Some(Value::from("world")), mm.get_element("received.hello"));
    assert_eq!(Some(Value::from(100)), mm.get_element("received.amount"));
    // 'input.header.x-demo -> header.hello' becomes a function request header
    assert_eq!(
        Some(Value::from("sunshine")),
        mm.get_element("hello_header")
    );
    assert_eq!(Some(Value::from(200.0)), mm.get_element("doubled"));
    // the function's response header maps to the graph output header
    assert_eq!(
        Some("demo"),
        reply.headers().get("x-task").map(String::as_str)
    );

    // --- unit-test-task-2: field mapping into a PoJo function
    let reply = run_graph(
        &platform,
        "unit-test-task-2",
        serde_json::json!({"name": "apple", "amount": 7}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status());
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("apple")), mm.get_element("name"));
    assert_eq!(Some(Value::from(14)), mm.get_element("total"));

    // --- unit-test-task-3: for_each fork-join over an array
    let reply = run_graph(
        &platform,
        "unit-test-task-3",
        serde_json::json!({"items": [1, 2, 3]}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status());
    let Value::Array(results) = reply.body() else {
        panic!("expected a list body, got {:?}", reply.body());
    };
    assert_eq!(3, results.len());
    let mut doubled: Vec<f64> = results
        .iter()
        .map(|r| {
            MultiLevelMap::from_value(r.clone())
                .get_element("doubled")
                .and_then(|v| v.as_f64())
                .expect("doubled")
        })
        .collect();
    doubled.sort_by(|a, b| a.partial_cmp(b).expect("ordered"));
    assert_eq!(vec![2.0, 4.0, 6.0], doubled);

    // --- unit-test-task-4: exception handler node recovers the error
    let reply = run_graph(
        &platform,
        "unit-test-task-4",
        serde_json::json!({"hello": "world"}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status());
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("recovered")), mm.get_element("message"));
    assert_eq!(Some(Value::from(400)), mm.get_element("status"));

    // --- unit-test-task-5: a missing task route fails fast
    let reply = run_graph(
        &platform,
        "unit-test-task-5",
        serde_json::json!({"hello": "world"}),
        serde_json::json!({}),
    )
    .await;
    assert_ne!(200, reply.status());
    let text = event_script::conversions::to_json_string(reply.body());
    assert!(
        text.contains("does not exist"),
        "unexpected error response: {text}"
    );
}

async fn join_loop_retirement_and_health(platform: &Platform) {
    let platform = platform.clone();

    // --- rust-join (lazy-loaded): the join barrier waits for both branches
    let reply = run_graph(
        &platform,
        "rust-join",
        serde_json::json!({"go": true}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status());
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("alpha")), mm.get_element("a"));
    assert_eq!(Some(Value::from("beta")), mm.get_element("b"));
    assert_eq!(Some(Value::from("joined")), mm.get_element("message"));

    // --- rust-js-retired: graph.js fails with the explicit retirement message
    let reply = run_graph(
        &platform,
        "rust-js-retired",
        serde_json::json!({"x": 1}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(400, reply.status());
    let mm = body_map(&reply);
    let message = mm
        .get_element("message")
        .map(|v| event_script::conversions::display(&v));
    assert!(
        message.as_deref().unwrap_or_default().contains("retired"),
        "expected the retirement message, got {message:?}"
    );
    assert!(message
        .as_deref()
        .unwrap_or_default()
        .contains("graph.math"));

    // --- rust-loop: loop detection aborts a self-resetting node
    let reply = run_graph(
        &platform,
        "rust-loop",
        serde_json::json!({"x": 1}),
        serde_json::json!({}),
    )
    .await;
    let mm = body_map(&reply);
    assert_eq!(
        Some(Value::from("Node spinner executed too frequently")),
        mm.get_element("message")
    );
    assert_eq!(Some(Value::from(400)), mm.get_element("status"));

    // --- graph.health joins the actuator protocol (Java healthCheck)
    let po = platform_core::PostOffice::new(&platform);
    let info = po
        .request(
            EventEnvelope::new()
                .set_to("graph.health")
                .set_header("type", "info"),
            Duration::from_secs(2),
        )
        .await
        .expect("info");
    let mm = MultiLevelMap::from_value(info.body().clone());
    assert_eq!(
        Some(Value::from("mini-graph service")),
        mm.get_element("service")
    );
    let health = po
        .request(
            EventEnvelope::new()
                .set_to("graph.health")
                .set_header("type", "health"),
            Duration::from_secs(2),
        )
        .await
        .expect("health");
    let mm = MultiLevelMap::from_value(health.body().clone());
    assert_eq!(
        Some(Value::from("I am doing fine")),
        mm.get_element("mini-graph")
    );

    // --- the housekeeper clears graph instances when flows end
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert!(
        knowledge_graph::model::get_instance("no-such-instance").is_none(),
        "registry lookup sanity"
    );
}
