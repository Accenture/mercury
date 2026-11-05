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

//! The Rust twin of the Java `LlmRunPlatformTest`: `llm.run` through the platform. The route registers
//! itself from the preload inventory, the model turn is a scripted `llm.chat` function, and the tools
//! are real routes, so the path `graph.task` uses is exercised - the request and response codec, a
//! route that fails, a route that times out, and a failure that reaches the caller with its status.
//!
//! One `#[tokio::test]` on purpose: the global platform's workers live on this test's runtime, and a
//! second test would get its own runtime (the graph_runtime.rs pattern).

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use agent_loop::{
    capped, failure, json_equals, Defaults, LoopGateway, PlatformGateway, LLM_CHAT, LLM_RUN,
    X_RETURN_PATH,
};
use async_trait::async_trait;
use platform_core::{
    main_application, trace, AppError, AutoStart, ComposableFunction, EntryPoint, EventEnvelope,
    Platform, PostOffice,
};
use rmpv::Value;
use serde_json::json;

const CREDIT_LIMIT: &str = "test.credit.limit";
const SLOW_LEDGER: &str = "test.slow.ledger";
const FAILING_LEDGER: &str = "test.failing.ledger";
const CONV: &str = "0123456789abcdef0123456789abcdef";
const WATCHER: &str = "test.progress.watcher";
const TIMEOUT: Duration = Duration::from_secs(15);

static REPLIES: Mutex<VecDeque<Value>> = Mutex::new(VecDeque::new());
static REQUESTS: Mutex<Vec<Value>> = Mutex::new(Vec::new());
static WATCHED: Mutex<Vec<EventEnvelope>> = Mutex::new(Vec::new());

#[main_application]
struct AgentLoopTestApp;

#[async_trait]
impl EntryPoint for AgentLoopTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        // llm.run registers itself through the preload inventory
        Ok(())
    }
}

// ---- the scripted routes ----

/// `llm.chat`: records each request and answers the next scripted reply; a reply carrying `_status`
/// fails with that status and `_message`.
struct ScriptedChat;

#[async_trait]
impl ComposableFunction for ScriptedChat {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        REQUESTS
            .lock()
            .expect("requests")
            .push(input.body().clone());
        let reply = REPLIES.lock().expect("replies").pop_front();
        let Some(reply) = reply else {
            return Err(AppError::new(500, "no scripted reply"));
        };
        if let Some(status) = field(&reply, "_status").and_then(Value::as_i64) {
            let message = field(&reply, "_message")
                .and_then(Value::as_str)
                .unwrap_or_default();
            return Err(AppError::new(status as i32, message));
        }
        Ok(EventEnvelope::new().set_raw_body(reply))
    }
}

struct CreditLimit;

#[async_trait]
impl ComposableFunction for CreditLimit {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let id = field(input.body(), "id").cloned().unwrap_or(Value::Nil);
        Ok(EventEnvelope::new().set_raw_body(Value::Map(vec![
            (Value::from("id"), id),
            (Value::from("limit"), Value::from(5000)),
            (Value::from("currency"), Value::from("EUR")),
        ])))
    }
}

struct FailingLedger;

#[async_trait]
impl ComposableFunction for FailingLedger {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Err(AppError::new(503, "ledger offline"))
    }
}

struct SlowLedger;

#[async_trait]
impl ComposableFunction for SlowLedger {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        tokio::time::sleep(Duration::from_millis(2500)).await;
        Ok(EventEnvelope::new().set_raw_body(v(json!({"balance": 1}))))
    }
}

struct Watcher;

#[async_trait]
impl ComposableFunction for Watcher {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        WATCHED.lock().expect("watched").push(input);
        Ok(EventEnvelope::new())
    }
}

// ---- helpers ----

fn v(json: serde_json::Value) -> Value {
    rmpv::ext::to_value(json).expect("a JSON value")
}

fn field<'a>(map: &'a Value, key: &str) -> Option<&'a Value> {
    match map {
        Value::Map(entries) => entries
            .iter()
            .find(|(name, _)| name.as_str() == Some(key))
            .map(|(_, value)| value),
        _ => None,
    }
}

fn at<'a>(value: &'a Value, path: &[&str]) -> &'a Value {
    const NIL: &Value = &Value::Nil;
    path.iter()
        .fold(value, |current, key| match key.parse::<usize>() {
            Ok(index) => match current {
                Value::Array(items) => items.get(index).unwrap_or(NIL),
                _ => NIL,
            },
            Err(_) => field(current, key).unwrap_or(NIL),
        })
}

fn reset() {
    REPLIES.lock().expect("replies").clear();
    REQUESTS.lock().expect("requests").clear();
    WATCHED.lock().expect("watched").clear();
}

fn script(replies: Vec<Value>) {
    REPLIES.lock().expect("replies").extend(replies);
}

fn requests() -> Vec<Value> {
    REQUESTS.lock().expect("requests").clone()
}

fn row(route: &str) -> Value {
    v(json!({
        "name": "lookup",
        "description": "A tool for the test.",
        "input_schema": {"type": "object", "properties": {"id": {"type": "string"}}, "required": ["id"]},
        "route": route
    }))
}

fn input(route: &str, settings: &[(&str, &str)]) -> Value {
    let mut map = vec![
        (Value::from("tools"), Value::Array(vec![row(route)])),
        (
            Value::from("prompt"),
            Value::from("May C-1 order 4200 EUR?"),
        ),
    ];
    for (key, value) in settings {
        map.push((Value::from(*key), Value::from(*value)));
    }
    Value::Map(map)
}

fn tool_use() -> Value {
    v(json!({
        "status": "tool_use",
        "calls": [{"id": "c1", "name": "lookup", "input": {"id": "C-1"}}],
        "conversation": CONV, "turn": 1, "model": "claude-opus-5-5",
        "usage": {"input_tokens": 900, "output_tokens": 60}
    }))
}

fn done() -> Value {
    v(json!({
        "status": "done", "text": "Yes.", "conversation": CONV, "turn": 2, "model": "claude-opus-5-5",
        "usage": {"input_tokens": 100, "output_tokens": 20, "cache_read_input_tokens": 800}
    }))
}

async fn run(po: &PostOffice, input: Value, return_path: Option<&str>) -> EventEnvelope {
    let mut event = EventEnvelope::new()
        .set_to(LLM_RUN)
        .set_raw_body(input)
        .set_trace(&trace::new_trace_id(), "TEST /agent-loop");
    if let Some(route) = return_path {
        event = event.set_header(X_RETURN_PATH, route);
    }
    po.request(event, TIMEOUT).await.expect("llm.run replies")
}

fn body_text(response: &EventEnvelope) -> String {
    response.body().as_str().unwrap_or_default().to_string()
}

async fn boot() -> Platform {
    platform_core::resources::prepend_resource_root("tests/resources");
    AutoStart::main(vec![]).await.expect("lifecycle");
    let platform = Platform::get_instance();
    assert!(
        platform.has_route(LLM_RUN),
        "llm.run must register itself from the preload inventory"
    );
    platform
        .register_private(LLM_CHAT, Arc::new(ScriptedChat), 1)
        .expect("llm.chat");
    platform
        .register_private(CREDIT_LIMIT, Arc::new(CreditLimit), 5)
        .expect("credit limit");
    platform
        .register_private(FAILING_LEDGER, Arc::new(FailingLedger), 1)
        .expect("failing ledger");
    platform
        .register_private(WATCHER, Arc::new(Watcher), 1)
        .expect("watcher");
    platform
        .register_private(SLOW_LEDGER, Arc::new(SlowLedger), 1)
        .expect("slow ledger");
    platform
}

// ---- the scenarios ----

async fn one_tool_then_the_answer(po: &PostOffice) {
    reset();
    script(vec![tool_use(), done()]);
    let response = run(po, input(CREDIT_LIMIT, &[]), None).await;
    assert_eq!(response.status(), 200, "{}", body_text(&response));
    let envelope = response.body();
    assert_eq!(at(envelope, &["status"]).as_str(), Some("done"));
    assert_eq!(at(envelope, &["text"]).as_str(), Some("Yes."));
    assert_eq!(at(envelope, &["conversation"]).as_str(), Some(CONV));
    assert_eq!(at(envelope, &["turn"]).as_i64(), Some(2));
    assert_eq!(at(envelope, &["turns"]).as_i64(), Some(2));
    assert_eq!(
        at(envelope, &["usage", "input_tokens"]).as_i64(),
        Some(1000)
    );
    assert_eq!(
        at(envelope, &["usage", "cache_read_input_tokens"]).as_i64(),
        Some(800)
    );
    assert!(json_equals(
        at(envelope, &["calls"]),
        &v(json!([{"id": "c1", "name": "lookup", "route": CREDIT_LIMIT, "status": "ok"}]))
    ));
    let requests = requests();
    assert_eq!(requests.len(), 2);
    let first = &requests[0];
    assert_eq!(
        at(first, &["prompt"]).as_str(),
        Some("May C-1 order 4200 EUR?")
    );
    assert!(
        field(at(first, &["tools", "0"]), "route").is_none(),
        "the model never sees a route"
    );
    let timeout = at(first, &["params", "timeout_ms"]).as_i64().unwrap_or(0);
    assert!(timeout > 20000 && timeout <= 24000, "timeout_ms {timeout}");
    let second = &requests[1];
    assert!(field(second, "prompt").is_none());
    assert_eq!(at(second, &["turn"]).as_i64(), Some(1));
    assert_eq!(at(second, &["results", "0", "id"]).as_str(), Some("c1"));
    assert_eq!(
        at(second, &["results", "0", "output", "limit"]).as_i64(),
        Some(5000)
    );
}

async fn progress_goes_to_the_callers_return_path(po: &PostOffice) {
    reset();
    script(vec![tool_use(), done()]);
    let response = run(po, input(CREDIT_LIMIT, &[]), Some(WATCHER)).await;
    assert_eq!(response.status(), 200, "{}", body_text(&response));
    let deadline = Instant::now() + Duration::from_secs(5);
    while WATCHED.lock().expect("watched").len() < 8 && Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    let watched = WATCHED.lock().expect("watched").clone();
    let mut names = Vec::new();
    for (i, event) in watched.iter().enumerate() {
        let body = event.body();
        names.push(
            at(body, &["event"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
        );
        assert_eq!(at(body, &["type"]).as_str(), Some("progress"));
        assert_eq!(at(body, &["seq"]).as_i64(), Some(i as i64 + 1));
        assert!(event.trace_id().is_none(), "a progress event adds no span");
    }
    assert_eq!(
        names,
        [
            "run.start",
            "turn.start",
            "turn.end",
            "tool.start",
            "tool.end",
            "turn.start",
            "turn.end",
            "run.end"
        ],
        "events so far"
    );
    let end = at(watched.last().expect("run.end").body(), &["data"]);
    assert_eq!(at(end, &["status"]).as_str(), Some("done"));
}

async fn without_a_return_path_nothing_is_sent(po: &PostOffice) {
    reset();
    script(vec![done()]);
    let response = run(po, input(CREDIT_LIMIT, &[]), None).await;
    assert_eq!(response.status(), 200, "{}", body_text(&response));
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(
        WATCHED.lock().expect("watched").is_empty(),
        "a caller that names no return route gets no events"
    );
}

async fn a_route_that_fails_answers_with_its_status(po: &PostOffice) {
    reset();
    script(vec![tool_use(), done()]);
    let response = run(po, input(FAILING_LEDGER, &[]), None).await;
    assert_eq!(response.status(), 200, "{}", body_text(&response));
    let requests = requests();
    assert_eq!(
        at(&requests[1], &["results", "0", "error"]).as_str(),
        Some("error 503: ledger offline")
    );
    assert_eq!(
        at(response.body(), &["calls", "0", "status"]).as_str(),
        Some("error")
    );
}

async fn a_slow_route_times_out_and_the_deadline_stops_the_run(po: &PostOffice) {
    reset();
    script(vec![tool_use()]);
    let response = run(po, input(SLOW_LEDGER, &[("timeout", "3s")]), None).await;
    assert_eq!(response.status(), 200, "{}", body_text(&response));
    let envelope = response.body();
    assert_eq!(at(envelope, &["status"]).as_str(), Some("cap"));
    assert_eq!(at(envelope, &["cap"]).as_str(), Some("deadline"));
    assert_eq!(
        at(envelope, &["calls", "0", "status"]).as_str(),
        Some("error")
    );
    assert_eq!(
        requests().len(),
        1,
        "no model turn starts after the deadline"
    );
}

async fn a_preflight_failure_reaches_the_caller_with_its_status(po: &PostOffice) {
    reset();
    let response = run(po, input("test.nobody.home", &[]), None).await;
    assert_eq!(response.status(), 400);
    assert_eq!(
        body_text(&response),
        "tools[0].route 'test.nobody.home' is not reachable - no local function, Event-over-HTTP \
         target or mesh route serves it"
    );
    assert!(requests().is_empty(), "no token is spent on a bad catalog");
}

async fn a_failed_turn_reaches_the_caller_with_its_status(po: &PostOffice) {
    reset();
    script(vec![v(json!({
        "_status": 429,
        "_message": "LLM provider rate limit - 429 rate_limit_error: slow down"
    }))]);
    let response = run(po, input(CREDIT_LIMIT, &[]), None).await;
    assert_eq!(response.status(), 429);
    assert_eq!(
        body_text(&response),
        "llm.chat failed - LLM provider rate limit - 429 rate_limit_error: slow down"
    );
}

fn configured_defaults_reach_the_loop() {
    let defaults = Defaults::from_config();
    assert_eq!(defaults.result_limit, "32768");
    assert_eq!(defaults.max_turns, Defaults::built_in().max_turns);
    assert_eq!(defaults.timeout, "25s");
}

fn the_gateway_finds_local_routes_only(platform: &Platform) {
    let gateway = PlatformGateway::new(platform);
    assert!(gateway.reachable(CREDIT_LIMIT));
    assert!(gateway.reachable(LLM_CHAT));
    assert!(!gateway.reachable("test.nobody.home"));
    assert!(!gateway.reachable("not a route"));
}

fn a_request_that_cannot_be_sent_becomes_an_error_response() {
    assert_eq!(
        failure(&AppError::new(404, "Route x not found")).status(),
        404
    );
    assert_eq!(failure(&AppError::new(408, "late")).status(), 408);
    assert_eq!(failure(&AppError::new(418, "teapot")).status(), 418);
    let other = failure(&AppError::new(500, "broken"));
    assert_eq!(other.status(), 500);
    assert_eq!(other.body().as_str(), Some("broken"));
}

fn a_cut_result_keeps_whole_characters() {
    let cut = capped(&Value::from("😀😀😀"), 2);
    assert_eq!(
        cut.as_str(),
        Some("😀😀\n[the result was cut at 2 of 3 characters]")
    );
    assert_eq!(capped(&Value::Nil, 2).as_str(), Some(""));
    let structured = v(json!([{"k": "v"}]));
    assert!(!capped(&structured, 100).is_nil());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn llm_run_through_the_platform() {
    test_support::run_at_exit(platform_core::util::elastic_queue::shutdown_cleanup);
    let platform = boot().await;
    let po = PostOffice::new(&platform);
    one_tool_then_the_answer(&po).await;
    progress_goes_to_the_callers_return_path(&po).await;
    without_a_return_path_nothing_is_sent(&po).await;
    a_route_that_fails_answers_with_its_status(&po).await;
    a_slow_route_times_out_and_the_deadline_stops_the_run(&po).await;
    a_preflight_failure_reaches_the_caller_with_its_status(&po).await;
    a_failed_turn_reaches_the_caller_with_its_status(&po).await;
    configured_defaults_reach_the_loop();
    the_gateway_finds_local_routes_only(&platform);
    a_request_that_cannot_be_sent_becomes_an_error_response();
    a_cut_result_keeps_whole_characters();
}
