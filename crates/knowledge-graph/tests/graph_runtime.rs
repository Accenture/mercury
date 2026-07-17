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

/// Java `MdmProfile` mock (`mock.mdm.profile`): serves profile JSON from
/// classpath fixtures; person id from the POST body or the GET path.
#[preload(route = "mock.mdm.profile", instances = 50)]
struct MdmProfile;

#[async_trait]
impl ComposableFunction for MdmProfile {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        if request["headers"]["x-exception"] == "true" {
            return Err(AppError::new(401, "simulated exception"));
        }
        let person_id = if request["method"] == "POST" {
            request["body"]["person_id"]
                .as_i64()
                .map(|n| n.to_string())
                .or_else(|| request["body"]["person_id"].as_str().map(str::to_string))
        } else {
            request["parameters"]["path"]["id"]
                .as_str()
                .map(str::to_string)
        };
        let Some(person_id) = person_id else {
            return Err(AppError::new(400, "Missing person id"));
        };
        serve_mock_json(
            &format!("profile-{person_id}"),
            &format!("Profile {person_id} not found"),
        )
    }
}

/// Java `AccountDetails` mock (`mock.account.details`).
#[preload(route = "mock.account.details", instances = 50)]
struct AccountDetails;

#[async_trait]
impl ComposableFunction for AccountDetails {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        log::info!(
            "Got request parameters {} and Authorization='{}'",
            request["body"],
            request["headers"]["authorization"]
                .as_str()
                .unwrap_or_default()
        );
        let account_id = request["body"]["account_id"].as_str().map(str::to_string);
        let (Some(account_id), true) = (account_id, request["method"] == "POST") else {
            return Err(AppError::new(400, "Missing account id"));
        };
        serve_mock_json(
            &format!("account-{account_id}"),
            &format!("Account {account_id} not found"),
        )
    }
}

fn serve_mock_json(name: &str, not_found: &str) -> Result<EventEnvelope, AppError> {
    match platform_core::ConfigReader::load(&format!("classpath:/mock/{name}.json")) {
        Ok(reader) => {
            let json =
                platform_core::ConfigValue::Map(reader.get_map().clone().into_map()).to_json();
            Ok(EventEnvelope::new().set_raw_body(event_script::conversions::from_json(&json)))
        }
        Err(_) => Err(AppError::new(400, not_found)),
    }
}

/// Java `DemoAuth` (`@FetchFeature("demo-auth")`), declared with the Rust
/// macro: a before-feature adding a demo bearer token — the field-installation
/// pattern for OAuth 2.0 access-token insertion. The token is the fetcher's
/// node name so the test can validate it on the wire (Java parity).
#[knowledge_graph::fetch_feature("demo-auth")]
struct DemoAuth;

impl knowledge_graph::features::FeatureRunner for DemoAuth {
    fn run_before(&self) -> bool {
        true
    }

    fn execute(
        &self,
        request: Option<&mut platform_core::automation::AsyncHttpRequest>,
        _response: Option<&knowledge_graph::features::HttpResponseView>,
        _state: &mut MultiLevelMap,
        node_name: &str,
    ) {
        if let Some(request) = request {
            *request = request
                .clone()
                .set_header("Authorization", &format!("Bearer {node_name}"));
        }
    }
}

/// Echoes the Authorization header it received on the wire.
#[preload(route = "mock.echo.auth", instances = 10)]
struct MockEchoAuth;

#[async_trait]
impl ComposableFunction for MockEchoAuth {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        EventEnvelope::new().set_body(serde_json::json!({
            "auth": request["headers"]["authorization"],
        }))
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
    let platform = Platform::get_instance();
    // the API-fetcher tutorials call the mock endpoints over real HTTP
    let addr = platform_core::automation::start_http_server(&platform)
        .await
        .expect("rest server");
    assert_eq!(8090, addr.port(), "rest.server.port from test config");
    platform
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
    api_fetcher_matches_java_semantics(&platform).await;
    graph_extension_matches_java_semantics(&platform).await;
    activated_hello_graphs_match_java_semantics(&platform).await;
}

/// Java `GraphExecutionTest.testGraphExecutionMath/Js` + `GraphTests.tutorial113`
/// — the graphs that carried `graph.js`, activated by the maintainer-directed
/// swap to `graph.math` (2026-07-17). The former JS variant now renders
/// numbers as doubles (math-engine semantics); `rust-js-retired` remains the
/// single `graph.js` case proving the retirement error.
async fn activated_hello_graphs_match_java_semantics(platform: &Platform) {
    let platform = platform.clone();

    // --- hello (MATH variant): fetch + math + join + extension composite
    let reply = run_graph(
        &platform,
        "hello",
        serde_json::json!({"person_id": 100}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "hello failed: {:?}", reply.body());
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Peter")), mm.get_element("name"));
    assert_eq!(
        Some(Value::from("100 World Blvd")),
        mm.get_element("address")
    );
    assert_eq!(Some(Value::from(558.0)), mm.get_element("sum"));
    assert_eq!(Some(Value::from(50000.0)), mm.get_element("multiply"));
    let Some(Value::Array(accounts)) = mm.get_element("accounts") else {
        panic!("expected accounts, got {:?}", reply.body());
    };
    assert_eq!(5, accounts.len());

    // --- helloworld (CONVERT variant): deprecated-syntax conversion + joins
    let reply = run_graph(
        &platform,
        "helloworld",
        serde_json::json!({"person_id": 100}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "helloworld failed: {:?}", reply.body());
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Peter")), mm.get_element("name"));
    assert_eq!(
        Some(Value::from("100 World Blvd")),
        mm.get_element("address")
    );

    // --- hellojs (the former JS variant, now math semantics — doubles)
    let reply = run_graph(
        &platform,
        "hellojs",
        serde_json::json!({"person_id": 100}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "hellojs failed: {:?}", reply.body());
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Peter")), mm.get_element("name"));
    assert_eq!(
        Some(Value::from("100 World Blvd")),
        mm.get_element("address")
    );
    assert_eq!(Some(Value::from(558.0)), mm.get_element("sum"));
    assert_eq!(Some(Value::from(50000.0)), mm.get_element("multiply"));
    let Some(Value::Array(accounts)) = mm.get_element("accounts") else {
        panic!("expected accounts, got {:?}", reply.body());
    };
    assert_eq!(5, accounts.len());
    let Some(Value::Array(details)) = mm.get_element("account_details") else {
        panic!("expected account details, got {:?}", reply.body());
    };
    assert_eq!(5, details.len());

    // --- rust-auth: the declarative #[fetch_feature] (OAuth-bearer pattern) —
    // the before-feature injects the bearer token into the provider request
    let reply = run_graph(
        &platform,
        "rust-auth",
        serde_json::json!({"person_id": 7}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "rust-auth failed: {:?}", reply.body());
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Bearer fetcher")), mm.get_element("auth"));

    // --- tutorial 113: the retry pattern (error-handler + clear-exception)
    let reply = run_graph(
        &platform,
        "tutorial-113",
        serde_json::json!({"person_id": 100, "exception": true}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(
        200,
        reply.status(),
        "tutorial-113 failed: {:?}",
        reply.body()
    );
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Peter")), mm.get_element("name"));
    assert_eq!(
        Some(Value::from("100 World Blvd")),
        mm.get_element("address")
    );
}

/// Java `GraphTests` tutorials 10/11 + `GraphExecutionTest` helloworld2:
/// graph.extension delegating to a sub-graph and to a `flow://` flow.
async fn graph_extension_matches_java_semantics(platform: &Platform) {
    let platform = platform.clone();

    // --- tutorial 10: extension -> the tutorial-3 sub-graph
    let reply = run_graph(
        &platform,
        "tutorial-10",
        serde_json::json!({"person_id": 100}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(
        200,
        reply.status(),
        "tutorial-10 failed: {:?}",
        reply.body()
    );
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Peter")), mm.get_element("name"));
    assert_eq!(
        Some(Value::from("100 World Blvd")),
        mm.get_element("address")
    );

    // --- tutorial 11: extension -> flow://flow-11 (echo flow)
    let reply = run_graph(
        &platform,
        "tutorial-11",
        serde_json::json!({"hello": "world", "message": "this is a good day"}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(
        200,
        reply.status(),
        "tutorial-11 failed: {:?}",
        reply.body()
    );
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("world")), mm.get_element("hello"));
    assert_eq!(
        Some(Value::from("this is a good day")),
        mm.get_element("message")
    );

    // --- helloworld2 (GraphExecutionTest, MATH variant): fetcher ->
    // for-each extension over the helloext sub-graph -> math -> end
    let reply = run_graph(
        &platform,
        "helloworld2",
        serde_json::json!({"person_id": 100}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(
        200,
        reply.status(),
        "helloworld2 failed: {:?}",
        reply.body()
    );
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Peter")), mm.get_element("name"));
    assert_eq!(
        Some(Value::from("100 World Blvd")),
        mm.get_element("address")
    );
    // graph.math renders numbers as doubles (Java parity)
    assert_eq!(Some(Value::from(558.0)), mm.get_element("sum"));
    assert_eq!(Some(Value::from(50000.0)), mm.get_element("multiply"));
    let Some(Value::Array(accounts)) = mm.get_element("accounts") else {
        panic!("expected an accounts list, got {:?}", reply.body());
    };
    let mut ids: Vec<String> = accounts
        .iter()
        .map(event_script::conversions::display)
        .collect();
    ids.sort();
    assert_eq!(vec!["a101", "b202", "c303", "d400", "e500"], ids);
    let Some(Value::Array(details)) = mm.get_element("account_details") else {
        panic!("expected account details, got {:?}", reply.body());
    };
    assert_eq!(5, details.len());
    // the fetcher's output header mapping surfaces as a response header
    assert_eq!(
        Some("world"),
        reply.headers().get("x-hello").map(String::as_str)
    );
}

/// Java `GraphTests` fetcher tutorials + `GraphExecutionTest.unitTest1HappyPath`:
/// dictionary/provider fetch over real HTTP against the mock endpoints.
async fn api_fetcher_matches_java_semantics(platform: &Platform) {
    let platform = platform.clone();

    // --- tutorial 3: two dictionaries share one provider (cache exercised)
    let reply = run_graph(
        &platform,
        "tutorial-3",
        serde_json::json!({"person_id": 100}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "tutorial-3 failed: {:?}", reply.body());
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Peter")), mm.get_element("name"));
    assert_eq!(
        Some(Value::from("100 World Blvd")),
        mm.get_element("address")
    );

    // --- tutorial 3 negative: a missing profile aborts with the mock's error
    let reply = run_graph(
        &platform,
        "tutorial-3",
        serde_json::json!({"person_id": 10}),
        serde_json::json!({}),
    )
    .await;
    let mm = body_map(&reply);
    assert_eq!(
        Some(Value::from("Profile 10 not found")),
        mm.get_element("message")
    );
    assert_eq!(Some(Value::from(400)), mm.get_element("status"));
    assert_eq!(Some(Value::from("error")), mm.get_element("type"));

    // --- tutorial 5: fork-join branches fetch two profiles, join merges them
    let reply = run_graph(
        &platform,
        "tutorial-5",
        serde_json::json!({"person1": 100, "person2": 200}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "tutorial-5 failed: {:?}", reply.body());
    let mm = body_map(&reply);
    let Some(Value::Array(profiles)) = mm.get_element("profile") else {
        panic!("expected a profile list, got {:?}", reply.body());
    };
    assert_eq!(2, profiles.len());
    let mut names: Vec<String> = profiles
        .iter()
        .map(|p| {
            event_script::conversions::display(
                &MultiLevelMap::from_value(p.clone())
                    .get_element("name")
                    .expect("name"),
            )
        })
        .collect();
    names.sort();
    assert_eq!(vec!["Mary".to_string(), "Peter".to_string()], names);

    // --- tutorial 6: for_each fork-join over the account list
    let reply = run_graph(
        &platform,
        "tutorial-6",
        serde_json::json!({"person_id": 100}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "tutorial-6 failed: {:?}", reply.body());
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Peter")), mm.get_element("name"));
    assert_eq!(
        Some(Value::from("100 World Blvd")),
        mm.get_element("address")
    );
    let Some(Value::Array(accounts)) = mm.get_element("accounts") else {
        panic!("expected an account list, got {:?}", reply.body());
    };
    assert_eq!(5, accounts.len());
    for account in &accounts {
        let account = MultiLevelMap::from_value(account.clone());
        let id = event_script::conversions::display(&account.get_element("id").expect("id"));
        let balance = account.get_element("balance").and_then(|v| v.as_f64());
        match id.as_str() {
            "a101" => {
                assert_eq!(Some(Value::from("Saving")), account.get_element("type"));
                assert_eq!(Some(25032.13), balance);
            }
            "b202" => assert_eq!(Some(6020.68), balance),
            "c303" => assert_eq!(Some(120000.0), balance),
            "d400" => assert_eq!(Some(6000.0), balance),
            "e500" => assert_eq!(Some(8200.0), balance),
            other => panic!("unexpected account id {other}"),
        }
    }

    // --- tutorial 12: mapper + math + fetcher + island combined
    let reply = run_graph(
        &platform,
        "tutorial-12",
        serde_json::json!({"person_id": 100, "exception": true}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(
        200,
        reply.status(),
        "tutorial-12 failed: {:?}",
        reply.body()
    );
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Peter")), mm.get_element("name"));
    assert_eq!(
        Some(Value::from("100 World Blvd")),
        mm.get_element("address")
    );

    // --- tutorial 114: loop detection with a fetcher in the cycle
    let reply = run_graph(
        &platform,
        "tutorial-114",
        serde_json::json!({"person_id": 100, "exception": true}),
        serde_json::json!({}),
    )
    .await;
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from(400)), mm.get_element("status"));
    assert_eq!(
        Some(Value::from("Node fetcher executed too frequently")),
        mm.get_element("message")
    );

    // --- unit-test-1 (GraphExecutionTest happy path)
    let reply = run_graph(
        &platform,
        "unit-test-1",
        serde_json::json!({"person_id": 100}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(
        200,
        reply.status(),
        "unit-test-1 failed: {:?}",
        reply.body()
    );
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("Peter")), mm.get_element("name"));
    assert_eq!(
        Some(Value::from("100 World Blvd")),
        mm.get_element("address")
    );
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
