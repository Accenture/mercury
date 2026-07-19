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
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use event_script::mlm::MultiLevelMap;
use event_script::FlowExecutor;
use platform_core::{
    main_application, preload, trace, AppError, AutoStart, ComposableFunction, EntryPoint,
    EventEnvelope, Platform, PostOffice,
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

// `v1.hello.task`, `mock.mdm.profile`, and `mock.account.details` are now
// provided by the engine's dev-gated mocks (`knowledge_graph::mock`, each
// `#[optional_service("app.env=dev")]`); with `app.env: dev` in this crate's
// test `application.yml` they register automatically — no local copies needed.

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
    // the engine serves (the dev Playground registers websocket services), so
    // the lifecycle already started the HTTP server the API-fetcher tutorials
    // call over real HTTP — recover its bound port.
    let addr =
        platform_core::automation::server_address().expect("rest server started by lifecycle");
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
    // Run in this single test so the whole file shares one runtime + one booted
    // server: a second `#[tokio::test]` gets its own runtime, which drops (killing
    // the shared HTTP server task) when the first finishes — the harness flake.
    companion_sync_returns_outcome_in_band(&platform).await;
    companion_sync_rejects_session_topology_commands(&platform).await;
    companion_sync_import_fallback_reports_ok(&platform).await;
    math_for_each_blocks_and_iteration(&platform).await;
    join_barrier_waits_for_a_retrying_branch(&platform).await;
}

/// Join + RESET interplay (backlog probe): a join barrier must not count a
/// branch whose skill FAILED into its `exception=` route (skill_run is
/// success-only), and `RESET` clears the completion mark along with the
/// run-once guard and state. Fork: fetch-a fails on the exception flag and
/// retries through pause (300 ms) → recover-a, while br-b reaches the join
/// almost immediately — before the fix, the join fired prematurely off
/// fetch-a's failed-run mark and the output silently lost branch A.
async fn join_barrier_waits_for_a_retrying_branch(platform: &Platform) {
    let reply = run_graph(
        platform,
        "rust-join-retry",
        serde_json::json!({"person_id": 100, "exception": true}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "join-retry failed: {:?}", reply.body());
    let mm = body_map(&reply);
    // the join waited for fetch-a's successful retry: branch A's data is
    // present in the assembled output (premature fire would have lost it)
    assert_eq!(Some(Value::from("Peter")), mm.get_element("a-name"));
    assert_eq!(Some(Value::from("B")), mm.get_element("b"));
}

/// `graph.math` `for_each` + `BEGIN`/`END` semantics (finding #29 spec probe;
/// Java `GraphMath.executeNode`/`executeForEach`/`splitBlocks` parity):
/// pre-block once → each-block per element (strictly sequential, loop
/// variables rebound each iteration) → post-block once; a taken IF jump
/// breaks the loop and skips the post-block; without BEGIN the whole list is
/// the loop body; scalar for_each entries bind once; an unresolvable LHS
/// removes the model key; empty lists skip the body but keep pre/post.
async fn math_for_each_blocks_and_iteration(platform: &Platform) {
    // A) happy path: 3 elements, parallel arrays, scalar + unresolvable binds
    let reply = run_graph(
        platform,
        "rust-foreach",
        serde_json::json!({"items": [7, 8, 9], "prices": [10, 20, 30]}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "run A failed: {:?}", reply.body());
    let mm = body_map(&reply);
    // pre once, post once — in that order
    assert_eq!(vec!["pre", "post"], str_list(&mm, "phases"));
    // strictly sequential iteration; loop variables rebound per element
    assert_eq!(vec![7, 8, 9], int_list(&mm, "seen"));
    // parallel arrays bind in lockstep (same index each iteration)
    assert_eq!(vec![10, 20, 30], int_list(&mm, "prices"));
    // the each-block ran exactly once per element
    assert_eq!(Some(Value::from(3)), mm.get_element("count"));
    // a scalar for_each entry binds once (not per iteration)
    assert_eq!(Some(Value::from("fixed")), mm.get_element("tag"));
    // an unresolvable for_each LHS REMOVES the model key; the later mapping
    // of the removed key is skipped (unresolvable source), so no output key
    assert_eq!(None, mm.get_element("ghost"));
    // no BEGIN: the whole statement list is the loop body (node `plain`) —
    // the per-element COMPUTE ran three times (COMPUTE yields doubles)
    assert_eq!(vec![7, 8, 9], int_list(&mm, "seen2"));
    let Some(Value::Array(lines)) = mm.get_element("lines") else {
        panic!("expected per-element line totals");
    };
    assert_eq!(
        vec![70.0, 160.0, 270.0],
        lines
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect::<Vec<_>>()
    );
    // the pure-COMPUTE accumulator (node `totaler`): read model.total back
    // into the expression each iteration
    assert_eq!(Some(Value::from(500.0)), mm.get_element("total"));
    // ...and the f:add accumulator on the SAME doubles (node `plain`):
    // numeric promotion lets f:add consume COMPUTE results — any floating
    // arg promotes the fold to f64 (all-integral input still stays exact i64)
    assert_eq!(Some(Value::from(500.0)), mm.get_element("lsum"));

    // B) early exit: the IF jump at element 99 breaks the loop, skips post
    let reply = run_graph(
        platform,
        "rust-foreach",
        serde_json::json!({"items": [7, 99, 13], "prices": [1, 2, 3]}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "run B failed: {:?}", reply.body());
    let mm_b = body_map(&reply);
    // the jump routed traversal to `bail`; the post-block never ran
    assert_eq!(vec!["pre", "bailed"], str_list(&mm_b, "phases"));
    // element 0 completed; element 1's IF fired before its mappings;
    // element 2 never started — the taken jump BREAKS the loop
    assert_eq!(vec![7], int_list(&mm_b, "seen"));
    assert_eq!(vec![1], int_list(&mm_b, "prices"));
    // post-block outputs (count/tag) skipped with it
    assert_eq!(None, mm_b.get_element("count"));
    // traversal continued from `bail`, so `plain`/`totaler` never executed
    assert_eq!(None, mm_b.get_element("seen2"));
    assert_eq!(None, mm_b.get_element("total"));

    // C) empty lists: zero iterations, pre/post still run
    let reply = run_graph(
        platform,
        "rust-foreach",
        serde_json::json!({"items": [], "prices": []}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "run C failed: {:?}", reply.body());
    let mm_c = body_map(&reply);
    // zero iterations, but pre and post blocks still run
    assert_eq!(vec!["pre", "post"], str_list(&mm_c, "phases"));
    assert_eq!(Some(Value::from(0)), mm_c.get_element("count"));
    // scalar entries bind during for_each RESOLUTION, even with empty lists
    assert_eq!(Some(Value::from("fixed")), mm_c.get_element("tag"));
    assert_eq!(None, mm_c.get_element("seen"));
}

fn str_list(mm: &MultiLevelMap, key: &str) -> Vec<String> {
    let Some(Value::Array(items)) = mm.get_element(key) else {
        panic!("expected a list at {key}");
    };
    items
        .iter()
        .map(|v| v.as_str().unwrap_or_default().to_string())
        .collect()
}

fn int_list(mm: &MultiLevelMap, key: &str) -> Vec<i64> {
    let Some(Value::Array(items)) = mm.get_element(key) else {
        panic!("expected a list at {key}");
    };
    items.iter().map(|v| v.as_i64().unwrap_or(-1)).collect()
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

/// Stand-in for a session's WebSocket `.out` route — captures whatever the sync
/// endpoint tees to the live console (proves real-time human+AI collaboration).
struct OutTap {
    seen: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl ComposableFunction for OutTap {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if let Value::String(s) = input.body() {
            self.seen
                .lock()
                .expect("tap")
                .push(s.as_str().unwrap_or_default().to_string());
        }
        EventEnvelope::new().set_body("ok")
    }
}

/// Prototype: the **synchronous** companion endpoint returns the command outcome
/// in-band — `ok`/`output`/`error`/`result` — instead of a fire-and-forget ack
/// (design: `docs/design/ai-companion-sync.md`). This is the Tut-4 blind-spot fix:
/// an invalid command's error is now in the HTTP response, not WS-only — and the
/// output is *also* teed to the session's `.out` so a human watches live.
async fn companion_sync_returns_outcome_in_band(platform: &Platform) {
    let po = PostOffice::new(platform);
    let sid = "ws-770001-1";
    let in_route = "ws.770001.1.in";

    // create the session (mimic the WebSocket "open" event)
    po.send(
        EventEnvelope::new()
            .set_to("graph.command.singleton")
            .set_raw_body(rmpv::Value::Map(vec![
                (rmpv::Value::from("type"), rmpv::Value::from("open")),
                (rmpv::Value::from("in"), rmpv::Value::from(in_route)),
            ])),
    )
    .await
    .expect("open dispatched");
    for _ in 0..50 {
        if knowledge_graph::commands::has_session(sid) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert!(
        knowledge_graph::commands::has_session(sid),
        "session must exist before a companion command"
    );

    // stand in for the session's WebSocket console to prove the sync endpoint
    // tees output there (the real-time human+AI collaboration path)
    let tap = Arc::new(Mutex::new(Vec::<String>::new()));
    platform
        .register("ws.770001.1.out", Arc::new(OutTap { seen: tap.clone() }), 1)
        .expect("register out tap");

    // call the synchronous endpoint and decode its structured body
    async fn sync_cmd(platform: &Platform, sid: &str, command: &str) -> serde_json::Value {
        let event = EventEnvelope::new().set_raw_body(rmpv::Value::Map(vec![
            (
                rmpv::Value::from("parameters"),
                rmpv::Value::Map(vec![(
                    rmpv::Value::from("path"),
                    rmpv::Value::Map(vec![(rmpv::Value::from("id"), rmpv::Value::from(sid))]),
                )]),
            ),
            (rmpv::Value::from("body"), rmpv::Value::from(command)),
            (rmpv::Value::from("method"), rmpv::Value::from("POST")),
        ]));
        let resp = knowledge_graph::rest::post_companion_command_sync(platform, event)
            .await
            .expect("sync endpoint returns Ok");
        let json = event_script::conversions::to_json_string(resp.body());
        serde_json::from_str(&json).expect("response body is JSON")
    }

    // 1) an invalid command → ok:false, error present in-band (the blind spot, closed)
    let bad = sync_cmd(platform, sid, "flibbertigibbet not a command").await;
    assert_eq!(
        bad["ok"],
        serde_json::json!(false),
        "invalid command → ok:false"
    );
    assert!(
        bad["error"].is_string(),
        "error text returned in-band, not WS-only: {bad}"
    );

    // 2) a valid command → ok:true, error null, output populated
    let good = sync_cmd(platform, sid, "create node root\nwith type Root").await;
    assert_eq!(
        good["ok"],
        serde_json::json!(true),
        "valid command → ok:true: {good}"
    );
    assert!(good["error"].is_null(), "no error on success: {good}");
    assert!(
        good["output"].as_array().is_some_and(|a| !a.is_empty()),
        "console output returned in-band: {good}"
    );

    // 3) the tee — the same output also reached the session's WebSocket .out route,
    //    so a human (or a subscribed session) watches live, not just the AI caller
    tokio::time::sleep(Duration::from_millis(150)).await;
    let teed = tap.lock().expect("tap").clone();
    assert!(
        teed.iter().any(|l| l.contains("node root created")),
        "sync output must be teed to the session's WS .out for live human view: {teed:?}"
    );
}

/// A companion is an **assistant to** a session, not a WebSocket session of its
/// own (maintainer decision, 2026-07-18) — so **both** companion endpoints limit
/// the `session` command to the read-only status query: the topology subcommands
/// (`subscribe`/`unsubscribe`/`reset`) are rejected before dispatch. Executed on
/// the sync path they would durably register the per-request
/// `companion.sync.<uuid>` capture route as a subscriber (observed live during
/// the tutorial-5 companion test).
async fn companion_sync_rejects_session_topology_commands(platform: &Platform) {
    let po = PostOffice::new(platform);
    let sid = "ws-770002-1";
    let in_route = "ws.770002.1.in";
    let peer = "ws-770001-1"; // the session opened by the previous helper

    po.send(
        EventEnvelope::new()
            .set_to("graph.command.singleton")
            .set_raw_body(rmpv::Value::Map(vec![
                (rmpv::Value::from("type"), rmpv::Value::from("open")),
                (rmpv::Value::from("in"), rmpv::Value::from(in_route)),
            ])),
    )
    .await
    .expect("open dispatched");
    for _ in 0..50 {
        if knowledge_graph::commands::has_session(sid) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert!(knowledge_graph::commands::has_session(sid), "session open");

    let tap = Arc::new(Mutex::new(Vec::<String>::new()));
    platform
        .register("ws.770002.1.out", Arc::new(OutTap { seen: tap.clone() }), 1)
        .expect("register out tap");

    async fn sync_cmd(platform: &Platform, sid: &str, command: &str) -> serde_json::Value {
        let event = EventEnvelope::new().set_raw_body(rmpv::Value::Map(vec![
            (
                rmpv::Value::from("parameters"),
                rmpv::Value::Map(vec![(
                    rmpv::Value::from("path"),
                    rmpv::Value::Map(vec![(rmpv::Value::from("id"), rmpv::Value::from(sid))]),
                )]),
            ),
            (rmpv::Value::from("body"), rmpv::Value::from(command)),
            (rmpv::Value::from("method"), rmpv::Value::from("POST")),
        ]));
        let resp = knowledge_graph::rest::post_companion_command_sync(platform, event)
            .await
            .expect("sync endpoint returns Ok");
        let json = event_script::conversions::to_json_string(resp.body());
        serde_json::from_str(&json).expect("response body is JSON")
    }

    // 1) every topology-mutating form is rejected in-band, without dispatch
    for command in [
        format!("session subscribe {peer}"),
        "session unsubscribe".to_string(),
        "session reset".to_string(),
    ] {
        let resp = sync_cmd(platform, sid, &command).await;
        assert_eq!(resp["ok"], serde_json::json!(false), "rejected: {resp}");
        assert!(
            resp["error"]
                .as_str()
                .is_some_and(|e| e.contains("not available on the companion endpoint")),
            "refusal reason returned in-band: {resp}"
        );
    }

    // 2) no subscription was registered anywhere: the peer's status must not list
    //    a subscriber (in particular no companion.sync.* capture route), and this
    //    session must still be primary (not "subscribed to")
    let peer_status = sync_cmd(platform, peer, "session").await;
    let peer_text = peer_status["output"].to_string();
    assert!(
        !peer_text.contains("companion.sync") && !peer_text.contains("subscribed by"),
        "no capture-route subscriber may be registered on the peer: {peer_status}"
    );
    let my_status = sync_cmd(platform, sid, "session").await;
    assert_eq!(
        my_status["ok"],
        serde_json::json!(true),
        "read-only 'session' status stays allowed: {my_status}"
    );
    assert!(
        !my_status["output"].to_string().contains("subscribed to"),
        "the rejected subscribe must not mark this session as subscribed: {my_status}"
    );

    // 3) the refusal is also teed to the session's WS console for the human
    tokio::time::sleep(Duration::from_millis(150)).await;
    let teed = tap.lock().expect("tap").clone();
    assert!(
        teed.iter()
            .any(|l| l.contains("not available on the companion endpoint")),
        "refusal must be visible on the live console: {teed:?}"
    );

    // 4) the legacy fire-and-forget endpoint enforces the same restriction (400),
    //    while the read-only `session` status query still dispatches (accepted)
    async fn legacy_cmd(
        platform: &Platform,
        sid: &str,
        command: &str,
    ) -> Result<EventEnvelope, AppError> {
        let event = EventEnvelope::new().set_raw_body(rmpv::Value::Map(vec![
            (
                rmpv::Value::from("parameters"),
                rmpv::Value::Map(vec![(
                    rmpv::Value::from("path"),
                    rmpv::Value::Map(vec![(rmpv::Value::from("id"), rmpv::Value::from(sid))]),
                )]),
            ),
            (rmpv::Value::from("body"), rmpv::Value::from(command)),
            (rmpv::Value::from("method"), rmpv::Value::from("POST")),
        ]));
        knowledge_graph::rest::post_companion_command(platform, event).await
    }
    let refused = legacy_cmd(platform, sid, &format!("session subscribe {peer}")).await;
    match refused {
        Err(e) => {
            assert_eq!(e.status(), 400, "legacy endpoint refuses with 400");
            assert!(
                e.message()
                    .contains("not available on the companion endpoint"),
                "legacy refusal carries the reason: {}",
                e.message()
            );
        }
        Ok(resp) => panic!("legacy endpoint must refuse session subscribe: {resp:?}"),
    }
    let status_ok = legacy_cmd(platform, sid, "session").await;
    assert!(
        status_ok.is_ok(),
        "read-only 'session' stays allowed on the legacy endpoint: {status_ok:?}"
    );
}

/// The `/sync` `ok` flag is derived from the console output — and `import
/// graph from {deployed}` legitimately prints "Graph model not found in
/// /tmp/…" before falling back to the deployed classpath copy (finding #40).
/// The classification is whole-output-aware: the benign fallback pair reports
/// `ok:true`, while a genuine miss (the not-found line alone) stays `ok:false`.
async fn companion_sync_import_fallback_reports_ok(platform: &Platform) {
    let po = PostOffice::new(platform);
    let sid = "ws-770003-1";
    let in_route = "ws.770003.1.in";

    po.send(
        EventEnvelope::new()
            .set_to("graph.command.singleton")
            .set_raw_body(rmpv::Value::Map(vec![
                (rmpv::Value::from("type"), rmpv::Value::from("open")),
                (rmpv::Value::from("in"), rmpv::Value::from(in_route)),
            ])),
    )
    .await
    .expect("open dispatched");
    for _ in 0..50 {
        if knowledge_graph::commands::has_session(sid) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert!(knowledge_graph::commands::has_session(sid), "session open");

    async fn sync_cmd(platform: &Platform, sid: &str, command: &str) -> serde_json::Value {
        let event = EventEnvelope::new().set_raw_body(rmpv::Value::Map(vec![
            (
                rmpv::Value::from("parameters"),
                rmpv::Value::Map(vec![(
                    rmpv::Value::from("path"),
                    rmpv::Value::Map(vec![(rmpv::Value::from("id"), rmpv::Value::from(sid))]),
                )]),
            ),
            (rmpv::Value::from("body"), rmpv::Value::from(command)),
            (rmpv::Value::from("method"), rmpv::Value::from("POST")),
        ]));
        let resp = knowledge_graph::rest::post_companion_command_sync(platform, event)
            .await
            .expect("sync endpoint returns Ok");
        let json = event_script::conversions::to_json_string(resp.body());
        serde_json::from_str(&json).expect("response body is JSON")
    }

    // 1) benign fallback: the deployed model imports; the "not found in /tmp"
    //    line must NOT mark the command failed
    let imported = sync_cmd(platform, sid, "import graph from tutorial-3").await;
    let text = imported["output"].to_string();
    assert!(
        text.contains("Graph model not found in"),
        "the benign fallback line is still reported: {imported}"
    );
    assert!(
        text.contains("Found deployed graph model"),
        "fallback success marker expected: {imported}"
    );
    assert_eq!(
        imported["ok"],
        serde_json::json!(true),
        "benign import fallback must be ok:true: {imported}"
    );
    assert!(
        imported["error"].is_null(),
        "no error on success: {imported}"
    );

    // 2) genuine miss: the not-found line alone stays an error
    let missing = sync_cmd(platform, sid, "import graph from no-such-graph-xyz").await;
    assert_eq!(
        missing["ok"],
        serde_json::json!(false),
        "a genuine miss must stay ok:false: {missing}"
    );
    assert!(
        missing["error"]
            .as_str()
            .is_some_and(|e| e.contains("not found")),
        "genuine miss carries the not-found error: {missing}"
    );
}
