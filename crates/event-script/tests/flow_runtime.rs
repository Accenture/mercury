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

//! Increment E-4 end-to-end: real flows from the **canonical Java fixtures**
//! run through the full engine — `AutoStart::main` collects the engine
//! interceptors (compiler → manager → executor) AND the Java-parity task
//! functions below from the annotation inventory, exactly as a user
//! application would.
//!
//! One test function on purpose (the global platform's workers live on this
//! test's runtime); scenarios run sequentially inside it.

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use event_script::conversions::from_json;
use event_script::FlowExecutor;
use platform_core::{
    main_application, preload, trace, AppError, AutoStart, ComposableFunction, EntryPoint,
    EventEnvelope, Platform, PostOffice,
};
use rmpv::Value;

// ---- Java-parity task functions (ports of the Java test tasks) ----

/// Java `Greetings` (`greeting.test` ×10): echo user+greeting with status 201
/// and a demo header; the `exception` input triggers error scenarios.
#[preload(route = "greeting.test", instances = 10)]
struct Greetings;

#[async_trait]
impl ComposableFunction for Greetings {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        match body["exception"].as_str() {
            Some("409") => return Err(AppError::new(409, "Just a demo")),
            Some("timeout") => {
                // outlive the flow TTL so the watcher fires
                tokio::time::sleep(Duration::from_millis(2500)).await;
                return EventEnvelope::new().set_body("late");
            }
            _ => {}
        }
        let (Some(user), Some(greeting)) = (body["user"].as_str(), body["greeting"].as_str())
        else {
            return Err(AppError::new(400, "Missing user or greeting"));
        };
        let result = serde_json::json!({
            "user": user,
            "greeting": greeting,
            "message": format!("I got your greeting message - {greeting}"),
            "original": body,
            // the framework-injected read-only business correlation-id
            "my_cid": headers.get("my_correlation_id"),
        });
        EventEnvelope::new()
            .set_header("demo", "test-header")
            .set_status(201)
            .set_body(result)
    }
}

/// Java `HelloExceptionHandler` (`v1.hello.exception`): echoes the error map.
#[preload(route = "v1.hello.exception")]
struct HelloExceptionHandler;

#[async_trait]
impl ComposableFunction for HelloExceptionHandler {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Ok(EventEnvelope::new().set_raw_body(input.body().clone()))
    }
}

/// Java `SimpleDecision`: boolean from the `decision` input.
#[preload(route = "simple.decision")]
struct SimpleDecision;

#[async_trait]
impl ComposableFunction for SimpleDecision {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as()?;
        match body["decision"].as_str() {
            Some(text) => EventEnvelope::new().set_body(text == "true"),
            None => Err(AppError::new(400, "Missing decision")),
        }
    }
}

/// Java `NumericDecision`: integer from the `decision` input.
#[preload(route = "numeric.decision")]
struct NumericDecision;

#[async_trait]
impl ComposableFunction for NumericDecision {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as()?;
        let n = body["decision"]
            .as_str()
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0);
        EventEnvelope::new().set_body(n)
    }
}

/// Java `DecisionCase` (`decision.case`) — the loop workhorse: echoes its
/// input, optionally increments `n`, and raises the quit/jump/continue flags
/// when `n` hits the configured thresholds.
#[preload(route = "decision.case", instances = 2)]
struct DecisionCase;

#[async_trait]
impl ComposableFunction for DecisionCase {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let mut body: serde_json::Value = input.body_as()?;
        if let Some(reason) = body["exception"].as_str() {
            return Err(AppError::new(400, reason));
        }
        let as_int = |v: &serde_json::Value| -> i64 {
            v.as_i64()
                .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
                .unwrap_or(-1)
        };
        let mut n = as_int(&body["n"]);
        if !body["increment"].is_null() {
            n += 1;
            body["n"] = serde_json::json!(n);
        }
        if !body["continue"].is_null() && n == as_int(&body["continue"]) {
            body["continue"] = serde_json::json!(true);
        }
        if !body["break"].is_null() && n == as_int(&body["break"]) {
            body["quit"] = serde_json::json!(true);
        }
        if !body["jump"].is_null() && n == as_int(&body["jump"]) {
            body["jump"] = serde_json::json!(true);
        }
        Ok(EventEnvelope::new().set_raw_body(from_json(&body)))
    }
}

/// Java `SequentialOne`: returns the pojo built from its inputs.
#[preload(route = "sequential.one")]
struct SequentialOne;

#[async_trait]
impl ComposableFunction for SequentialOne {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Ok(EventEnvelope::new().set_raw_body(input.body().clone()))
    }
}

/// Java `NoOp` (`no.op`): passthrough echo.
#[preload(route = "no.op", instances = 2)]
struct NoOp;

#[async_trait]
impl ComposableFunction for NoOp {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Ok(EventEnvelope::new().set_raw_body(input.body().clone()))
    }
}

/// Java `ParallelTask` (`parallel.task`): a shared counter detects when both
/// parallel branches have run; the second one reports decision=true (done).
#[preload(route = "parallel.task", instances = 2)]
struct ParallelTask;

#[async_trait]
impl ComposableFunction for ParallelTask {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        let done = n == 2;
        if done {
            // let the first branch finish its model writes (Java parity)
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
        let mut body: serde_json::Value = input.body_as()?;
        body["decision"] = serde_json::Value::Bool(done);
        EventEnvelope::new().set_body(body)
    }
}

/// Setup step of the parallel fixture (`begin.parallel.test`): echo.
#[preload(route = "begin.parallel.test")]
struct BeginParallel;

#[async_trait]
impl ComposableFunction for BeginParallel {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Ok(EventEnvelope::new().set_raw_body(input.body().clone()))
    }
}

/// Java `ExceptionSimulator` (`exception.simulator`): throws when the
/// `exception` event header is present, else echoes.
#[preload(route = "exception.simulator", instances = 2)]
struct ExceptionSimulator;

#[async_trait]
impl ComposableFunction for ExceptionSimulator {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if let Some(reason) = headers.get("exception") {
            return Err(AppError::new(
                400,
                format!("simulated exception - {reason}"),
            ));
        }
        Ok(EventEnvelope::new().set_raw_body(input.body().clone()))
    }
}

#[main_application]
struct FlowTestApp;

#[async_trait]
impl EntryPoint for FlowTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

// ---- helpers ----

fn http_dataset(path_user: Option<&str>, query: &[(&str, &str)]) -> Value {
    let query_map: serde_json::Map<String, serde_json::Value> = query
        .iter()
        .map(|(k, v)| (k.to_string(), serde_json::Value::String(v.to_string())))
        .collect();
    let mut dataset = serde_json::json!({
        "body": {},
        "header": {"accept": "application/json"},
        "query": query_map,
        "method": "GET",
    });
    if let Some(user) = path_user {
        dataset["path_parameter"] = serde_json::json!({"user": user});
    }
    from_json(&dataset)
}

async fn run_flow(platform: &Platform, flow_id: &str, dataset: Value, cid: &str) -> EventEnvelope {
    FlowExecutor::request(
        platform,
        flow_id,
        dataset,
        cid,
        Duration::from_secs(8),
        Some((&trace::new_trace_id(), &format!("FLOW /{flow_id}"))),
    )
    .await
    .unwrap_or_else(|e| panic!("flow {flow_id} failed: {} {}", e.status(), e.message()))
}

fn json_body(reply: &EventEnvelope) -> serde_json::Value {
    reply.body_as().expect("json body")
}

// ---- the E2E scenarios ----

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn flows_run_end_to_end_like_java() {
    platform_core::resources::prepend_resource_root("tests/resources");
    let holding = std::env::temp_dir().join(format!("mercury-flow-test-{}", std::process::id()));
    platform_core::overrides::set("transient.data.store", &holding.display().to_string());
    // the one-liner lifecycle collects the engine (compiler/manager/executor)
    // and every task function above from the annotation inventory
    AutoStart::main(vec![]).await.expect("lifecycle");
    let platform = Platform::get_instance();

    // --- greetings: end task with type conversions and the exception handler wired
    let reply = run_flow(
        &platform,
        "greetings",
        http_dataset(Some("12345"), &[]),
        "biz-cid-1",
    )
    .await;
    assert_eq!(
        reply.status(),
        201,
        "'status -> output.status' maps the function's 201"
    );
    let body = json_body(&reply);
    assert_eq!(body["user"], "12345");
    assert_eq!(body["greeting"], "hello world");
    assert_eq!(body["message"], "I got your greeting message - hello world");
    // the function saw the read-only business correlation-id header
    assert_eq!(body["my_cid"], "biz-cid-1");
    // 'model.cid -> output.body.cid' exposes the business correlation-id
    assert_eq!(body["cid"], "biz-cid-1");
    // 'map(direction=right, ...) -> model.map' then 'model.map -> output.body.map2'
    assert_eq!(body["map2"]["direction"], "right");
    // 'model.bool -> !model.bool -> output.body.positive' (double negation)
    assert_eq!(body["positive"], true);
    // 'header.demo -> output.header.x-demo' copies the function's result
    // header; note 'header -> output.header' REPLACED the earlier
    // content-type mapping with the whole result-header map (Java parity)
    assert_eq!(
        reply.headers().get("x-demo").map(String::as_str),
        Some("test-header")
    );
    assert_eq!(
        reply.headers().get("demo").map(String::as_str),
        Some("test-header")
    );

    // --- decision: boolean branching
    let reply = run_flow(
        &platform,
        "decision-test",
        http_dataset(None, &[("decision", "true")]),
        "biz-cid-2",
    )
    .await;
    assert_eq!(json_body(&reply)["from"], "one");
    let reply = run_flow(
        &platform,
        "decision-test",
        http_dataset(None, &[("decision", "false")]),
        "biz-cid-3",
    )
    .await;
    assert_eq!(json_body(&reply)["from"], "two");

    // --- numeric decision: 1-indexed multi-way branching
    let reply = run_flow(
        &platform,
        "numeric-decision-test",
        http_dataset(None, &[("decision", "3")]),
        "biz-cid-4",
    )
    .await;
    assert_eq!(json_body(&reply)["from"], "three");
    // out-of-range decision aborts the flow with 500
    let reply = run_flow(
        &platform,
        "numeric-decision-test",
        http_dataset(None, &[("decision", "4")]),
        "biz-cid-5",
    )
    .await;
    assert_eq!(reply.status(), 500);
    let body = json_body(&reply);
    assert_eq!(body["type"], "error");
    assert!(body["message"]
        .as_str()
        .unwrap_or("")
        .contains("invalid decision"));

    // --- sequential chaining through a no-op with the '*' wildcard body
    let reply = run_flow(
        &platform,
        "sequential-test",
        http_dataset(Some("alice"), &[("seq", "5")]),
        "biz-cid-6",
    )
    .await;
    let body = json_body(&reply);
    assert_eq!(body["pojo"]["user"], "alice");
    assert_eq!(body["pojo"]["sequence"], "5");
    assert_eq!(body["integer"], 12345);
    assert_eq!(body["double"], 12.345);

    // --- response task answers early; the end task's output is NOT delivered
    let reply = run_flow(
        &platform,
        "response-test",
        http_dataset(Some("bob"), &[("seq", "1")]),
        "biz-cid-7",
    )
    .await;
    let body = json_body(&reply);
    assert_eq!(body["user"], "bob", "the response task's body wins");
    assert_eq!(
        reply.headers().get("content-type").map(String::as_str),
        Some("application/json"),
        "the end task's text/plain override must never reach the caller"
    );

    // --- exception routing: task error → the flow's exception handler
    let reply = run_flow(
        &platform,
        "greetings",
        http_dataset(Some("12345"), &[("ex", "409")]),
        "biz-cid-8",
    )
    .await;
    assert_eq!(reply.status(), 409);
    let body = json_body(&reply);
    assert_eq!(
        body["status"], 409,
        "'error.code -> status' reaches the handler"
    );
    assert_eq!(body["message"], "Just a demo");

    // --- TTL: the watcher aborts a stuck flow with 408
    let mut dataset = http_dataset(Some("12345"), &[("ex", "timeout")]);
    if let Value::Map(entries) = &mut dataset {
        entries.push((Value::from("ttl"), Value::from(500)));
    }
    let reply = run_flow(&platform, "timeout-test", dataset, "biz-cid-9").await;
    assert_eq!(reply.status(), 408);
    let body = json_body(&reply);
    assert_eq!(body["type"], "error");
    assert_eq!(body["message"], "Flow timeout for 500 ms");

    // --- dynamic reserved-key rejection at runtime (Java FlowTests parity):
    // 'model.{model.pointer}' resolves to model.none only at runtime; the
    // executor re-checks and aborts
    let reply = run_flow(
        &platform,
        "dynamic-reserved-key",
        from_json(&serde_json::json!({"body": {"hello": "world"}, "header": {}})),
        "biz-cid-10",
    )
    .await;
    let body = json_body(&reply);
    let message = body["message"].as_str().unwrap_or("");
    assert!(
        message.contains("reserved state-machine key"),
        "unexpected error: {body}"
    );
    assert!(
        message.contains("model.none"),
        "the error should name the key: {body}"
    );

    // --- fire-and-forget launch works (no reply expected)
    FlowExecutor::launch(
        &platform,
        "greetings",
        http_dataset(Some("fire-and-forget"), &[]),
        "biz-cid-11",
        None,
    )
    .await
    .expect("launch");
    // give the flow a beat to complete (nothing to assert — no reply address)
    tokio::time::sleep(Duration::from_millis(300)).await;

    // --- concurrent flows: the engine runs directly on the event core
    // (reserved-route optimization), so orchestration never queues behind
    // itself — 20 simultaneous transactions all complete correctly
    let mut handles = Vec::new();
    for i in 0..20 {
        let platform = platform.clone();
        handles.push(tokio::spawn(async move {
            let reply = FlowExecutor::request(
                &platform,
                "greetings",
                http_dataset(Some(&format!("user-{i}")), &[]),
                &format!("concurrent-cid-{i}"),
                Duration::from_secs(8),
                None,
            )
            .await
            .expect("concurrent flow");
            let body: serde_json::Value = reply.body_as().expect("json");
            assert_eq!(body["user"], format!("user-{i}"));
            assert_eq!(body["cid"], format!("concurrent-cid-{i}"));
        }));
    }
    for handle in handles {
        handle.await.expect("concurrent flow task");
    }

    // --- fork/join: both branches run, the join barrier fires once
    let reply = run_flow(
        &platform,
        "fork-n-join-test",
        http_dataset(Some("carol"), &[("seq", "9")]),
        "biz-cid-12",
    )
    .await;
    let body = json_body(&reply);
    assert_eq!(
        body["user"], "carol",
        "the '*' pojo mapping seeds the join input"
    );
    assert_eq!(body["key1"], "hello-world-one");
    assert_eq!(body["key2"], "hello-world-two");

    // --- fork/join: a failing branch (no handler in the flow) aborts
    let reply = run_flow(
        &platform,
        "fork-n-join-test",
        http_dataset(Some("dave"), &[("exception", "boom")]),
        "biz-cid-13",
    )
    .await;
    assert_eq!(reply.status(), 400);
    assert!(json_body(&reply)["message"]
        .as_str()
        .unwrap_or("")
        .contains("simulated exception"));

    // --- parallel: both branches run concurrently; the second to finish
    // (decision=true) routes to the response task with both model keys set
    let reply = run_flow(
        &platform,
        "parallel-test",
        http_dataset(None, &[]),
        "biz-cid-14",
    )
    .await;
    let body = json_body(&reply);
    assert_eq!(body["key1"], "hello-world-one");
    assert_eq!(body["key2"], "hello-world-two");

    // --- dynamic fork: the single branch replicates per list element with
    // .ITEM/.INDEX; concurrent [] appends into the state machine stay safe
    let reply = run_flow(
        &platform,
        "dynamic-fork-test",
        http_dataset(Some("erin"), &[]),
        "biz-cid-15",
    )
    .await;
    let body = json_body(&reply);
    assert_eq!(body["user"], "erin");
    let mut serialized: Vec<String> = body["serialized"]
        .as_array()
        .expect("serialized list")
        .iter()
        .map(|v| v.as_str().unwrap_or("").to_string())
        .collect();
    serialized.sort();
    assert_eq!(
        serialized,
        vec!["one", "three", "two"],
        "every ITEM processed exactly once"
    );
    let mut indexes: Vec<i64> = body["indexes"]
        .as_array()
        .expect("index list")
        .iter()
        .map(|v| v.as_i64().unwrap_or(-1))
        .collect();
    indexes.sort();
    assert_eq!(indexes, vec![0, 1, 2], "every INDEX seen exactly once");

    // --- pipeline without a loop: three steps run once, then the exit task
    let reply = run_flow(
        &platform,
        "pipeline-test",
        http_dataset(Some("frank"), &[("seq", "1")]),
        "biz-cid-16",
    )
    .await;
    assert_eq!(reply.status(), 200);
    let body = json_body(&reply);
    assert_eq!(
        body["data"]["user"], "frank",
        "the pojo travelled the pipeline"
    );

    // --- for loop: 3 iterations × 3 steps, file append/read/delete included
    let reply = run_flow(
        &platform,
        "for-loop-test",
        http_dataset(Some("gina"), &[("seq", "2")]),
        "biz-cid-17",
    )
    .await;
    let body = json_body(&reply);
    assert_eq!(body["n"], 3, "the loop counter finished at model.iteration");
    assert_eq!(
        body["content"], "one,two,three,one,two,three,one,two,three,",
        "each iteration appended its steps to the file"
    );
    assert!(
        !std::path::Path::new("/tmp/for-loop-test.txt").exists(),
        "the end task's null mapping deletes the file"
    );

    // --- for loop with a break condition (query break=1 → quit at n==1)
    let reply = run_flow(
        &platform,
        "for-loop-break",
        http_dataset(Some("hank"), &[("break", "1")]),
        "biz-cid-18",
    )
    .await;
    let body = json_body(&reply);
    assert_eq!(body["n"], 1, "the loop broke on the quit flag at n==1");

    // --- while loop: echo.three flips model.running via boolean(3=false)
    let reply = run_flow(
        &platform,
        "while-loop",
        http_dataset(Some("iris"), &[("seq", "3")]),
        "biz-cid-19",
    )
    .await;
    let body = json_body(&reply);
    assert_eq!(
        body["n"], 3,
        "the while loop ran until model.running turned false"
    );

    // --- pipeline step failure routes to the step's own exception handler
    let reply = run_flow(
        &platform,
        "pipeline-exception",
        http_dataset(Some("jack"), &[("seq", "4")]),
        "biz-cid-20",
    )
    .await;
    assert_eq!(reply.status(), 400);
    assert_eq!(json_body(&reply)["message"], "just a test");

    // sanity: the RPC inbox is still healthy after all scenarios
    let po = PostOffice::new(&platform);
    let ping = po
        .request(
            EventEnvelope::new()
                .set_to("no.op")
                .set_body("ping")
                .unwrap(),
            Duration::from_secs(2),
        )
        .await
        .expect("direct rpc");
    assert_eq!(ping.body_as::<String>().unwrap(), "ping");
}
