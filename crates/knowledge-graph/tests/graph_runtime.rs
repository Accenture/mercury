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
//! (tutorials 1/2/4/7/8/9/13) and `GraphTaskTest` (unit-test-task-1..6),
//! running the real `graph-executor` flow through the flow engine. Tutorials
//! needing `graph.api.fetcher` / `graph.extension` join at K-5/K-6.
//! Rust-supplement graphs (listed in `graphs.yaml` like everything else —
//! deployed execution is compiled-or-404) cover the join barrier, loop
//! detection and the `graph.js` retirement.

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

/// Java `FileStateStore` (`v1.file.state.store`): the temp-file mock state
/// store for the suspend/resume engine tests — MsgPack wrapper
/// `{expires_at, data}` under /tmp/suspend-resume, DELETE-ON-READ
/// (consume-on-retrieve: at-most-once resume), expiry honored on read.
/// Like the Redis reference implementation, records are scoped by
/// graph + cid so the same business transaction may suspend independently
/// in a parent graph and in each subgraph.
#[preload(route = "v1.file.state.store", instances = 10)]
struct FileStateStore;

const STORE_DIR: &str = "/tmp/suspend-resume";

fn store_file(graph_id: &str, cid: &str) -> std::path::PathBuf {
    let safe: String = format!("{graph_id}:{cid}")
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    std::path::Path::new(STORE_DIR).join(safe)
}

fn pack_value(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    rmpv::encode::write_value(&mut out, value).expect("msgpack encode");
    out
}

fn unpack_value(bytes: &[u8]) -> Value {
    rmpv::decode::read_value(&mut &bytes[..]).unwrap_or(Value::Nil)
}

fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

#[async_trait]
impl ComposableFunction for FileStateStore {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        std::fs::create_dir_all(STORE_DIR)
            .map_err(|e| AppError::new(500, format!("Unable to create {STORE_DIR} - {e}")))?;
        let request = MultiLevelMap::from_value(input.body().clone());
        let cid = request
            .get_element("cid")
            .map(|v| event_script::conversions::display(&v))
            .unwrap_or_default();
        // fail fast like the Redis reference implementation - a missing graph id
        // would collapse every graph's records into one shared key space
        let graph_id = match request.get_element("graph") {
            Some(v) => {
                let text = event_script::conversions::display(&v);
                if text.trim().is_empty() {
                    return Err(AppError::new(400, "Missing graph"));
                }
                text
            }
            None => return Err(AppError::new(400, "Missing graph")),
        };
        let file = store_file(&graph_id, &cid);
        match headers.get("type").map(String::as_str) {
            Some("put") => {
                let ttl_seconds = match request.get_element("ttl") {
                    Some(Value::Integer(n)) => n.as_i64().unwrap_or(30),
                    _ => 30,
                };
                let wrapper = Value::Map(vec![
                    (
                        Value::from("expires_at"),
                        Value::from(now_millis() + ttl_seconds * 1000),
                    ),
                    (Value::from("data"), input.body().clone()),
                ]);
                std::fs::write(&file, pack_value(&wrapper))
                    .map_err(|e| AppError::new(500, e.to_string()))?;
                EventEnvelope::new().set_body(serde_json::json!({"stored": true}))
            }
            Some("get") => {
                if !file.exists() {
                    return Ok(EventEnvelope::new().set_raw_body(Value::Map(vec![])));
                }
                let bytes = std::fs::read(&file).map_err(|e| AppError::new(500, e.to_string()))?;
                std::fs::remove_file(&file).ok();
                let wrapper = MultiLevelMap::from_value(unpack_value(&bytes));
                let expiry = match wrapper.get_element("expires_at") {
                    Some(Value::Integer(n)) => n.as_i64().unwrap_or(0),
                    _ => 0,
                };
                if now_millis() > expiry {
                    return Ok(EventEnvelope::new().set_raw_body(Value::Map(vec![])));
                }
                Ok(EventEnvelope::new()
                    .set_raw_body(wrapper.get_element("data").unwrap_or(Value::Map(vec![]))))
            }
            _ => Err(AppError::new(400, "type must be put or get")),
        }
    }
}

/// Java `CountingStepTask` (`v1.counting.step`): counts executions per
/// step+cid and records the business correlation id each execution saw
/// (injected `my_correlation_id`) — the no-re-execution and business-cid
/// assertions read these registries.
#[preload(route = "v1.counting.step", instances = 10)]
struct CountingStepTask;

fn step_counters() -> &'static Mutex<HashMap<String, i64>> {
    static COUNTERS: std::sync::OnceLock<Mutex<HashMap<String, i64>>> = std::sync::OnceLock::new();
    COUNTERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn step_business_cids() -> &'static Mutex<HashMap<String, String>> {
    static CIDS: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();
    CIDS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn step_count(step: &str, cid: &str) -> i64 {
    *step_counters()
        .lock()
        .unwrap()
        .get(&format!("{step}:{cid}"))
        .unwrap_or(&0)
}

fn step_business_cid(step: &str, cid: &str) -> Option<String> {
    step_business_cids()
        .lock()
        .unwrap()
        .get(&format!("{step}:{cid}"))
        .cloned()
}

#[async_trait]
impl ComposableFunction for CountingStepTask {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body = MultiLevelMap::from_value(input.body().clone());
        let display = event_script::conversions::display;
        let step = body
            .get_element("step")
            .map(|v| display(&v))
            .unwrap_or_default();
        let cid = body
            .get_element("cid")
            .map(|v| display(&v))
            .unwrap_or_default();
        // the business correlation ID injected by the platform at delivery -
        // the suspend/resume tests assert it matches the caller's cid
        if let Some(my_cid) = headers.get("my_correlation_id") {
            step_business_cids()
                .lock()
                .unwrap()
                .insert(format!("{step}:{cid}"), my_cid.clone());
        }
        let count = {
            let mut counters = step_counters().lock().unwrap();
            let entry = counters.entry(format!("{step}:{cid}")).or_insert(0);
            *entry += 1;
            *entry
        };
        let mut result = serde_json::json!({"step": step, "count": count});
        if let Some(prior) = body.get_element("prior") {
            result["prior"] = serde_json::to_value(&prior).unwrap_or_default();
        }
        EventEnvelope::new().set_body(result)
    }
}

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

// `mock.mdm.profile` and `mock.account.details` are provided by the engine's
// dev-gated mocks (`knowledge_graph::mock`, each
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

/// Counts every provider call it receives — the cache-key regression probe
/// (increment 54, parity F6): two fetches whose DICTIONARY-declared inputs
/// match must produce exactly one call, regardless of fetcher-level staging.
static CACHE_PROBE_CALLS: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);

#[preload(route = "mock.cache.counter", instances = 10)]
struct MockCacheCounter;

#[async_trait]
impl ComposableFunction for MockCacheCounter {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let n = CACHE_PROBE_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        EventEnvelope::new().set_body(serde_json::json!({"count": n}))
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

/// Like [`run_graph`] but with an explicit business correlation id — the
/// suspend/resume tests drive several runs sharing ONE cid.
async fn run_graph_cid(
    platform: &Platform,
    graph_id: &str,
    cid: &str,
    body: serde_json::Value,
) -> EventEnvelope {
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
        cid,
        Duration::from_secs(8),
        Some((&trace::new_trace_id(), &format!("TEST /graph/{graph_id}"))),
    )
    .await
    .unwrap_or_else(|e| panic!("graph {graph_id} failed: {} {}", e.status(), e.message()))
}

fn body_map(reply: &EventEnvelope) -> MultiLevelMap {
    MultiLevelMap::from_value(reply.body().clone())
}

/// A minimal polyglot peer: an /api/event endpoint that decodes the relayed
/// event envelope and answers with a reply envelope - the same wire contract
/// a python or node.js function host speaks.
async fn start_stub_peer(port: u16) {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port))
        .await
        .expect("stub peer port");
    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                break;
            };
            tokio::spawn(async move {
                let mut buf: Vec<u8> = Vec::new();
                let mut tmp = [0u8; 4096];
                let header_end = loop {
                    match socket.read(&mut tmp).await {
                        Ok(0) | Err(_) => return,
                        Ok(n) => buf.extend_from_slice(&tmp[..n]),
                    }
                    if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
                        break pos + 4;
                    }
                };
                let head_text = String::from_utf8_lossy(&buf[..header_end]).to_lowercase();
                let content_length: usize = head_text
                    .lines()
                    .find_map(|line| line.strip_prefix("content-length:"))
                    .and_then(|v| v.trim().parse().ok())
                    .unwrap_or(0);
                while buf.len() < header_end + content_length {
                    match socket.read(&mut tmp).await {
                        Ok(0) | Err(_) => return,
                        Ok(n) => buf.extend_from_slice(&tmp[..n]),
                    }
                }
                let request =
                    EventEnvelope::from_bytes(&buf[header_end..header_end + content_length])
                        .expect("relayed envelope");
                let reply_body = Value::Map(vec![
                    ("language".into(), "stub".into()),
                    ("route".into(), request.to().unwrap_or("").into()),
                    ("echo".into(), request.body().clone()),
                ]);
                let payload = EventEnvelope::new()
                    .set_body(reply_body)
                    .expect("reply body")
                    .to_bytes()
                    .expect("reply bytes");
                let head = format!(
                    "HTTP/1.1 200 OK\r\ncontent-type: application/octet-stream\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                    payload.len()
                );
                let _ = socket.write_all(head.as_bytes()).await;
                let _ = socket.write_all(&payload).await;
                let _ = socket.shutdown().await;
            });
        }
    });
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_runtime_end_to_end() {
    let platform = boot().await;
    graphs_run_end_to_end_like_java(&platform).await;
    graph_task_matches_java_semantics(&platform).await;
    join_loop_retirement_and_health(&platform).await;
    api_fetcher_matches_java_semantics(&platform).await;
    fetcher_cache_key_uses_dictionary_declared_inputs_only(&platform).await;
    graph_extension_matches_java_semantics(&platform).await;
    activated_hello_graphs_match_java_semantics(&platform).await;
    suspend_resume_matches_java_semantics(&platform).await;
    same_cid_suspends_independently_per_graph(&platform).await;
    orchestrator_parent_drives_suspending_subgraph_path(&platform).await;
    generic_exception_context_serves_every_node(&platform).await;
    statement_commands_resolve_dynamic_variables(&platform).await;
    successful_retry_resolves_the_error_context(&platform).await;
    suspend_resume_x_run_over_the_real_http_stack(&platform).await;
    suspend_resume_store_calls_chain_to_their_skill_spans(&platform).await;
    rejected_deployed_graph_is_not_executable(&platform).await;
    // Run in this single test so the whole file shares one runtime + one booted
    // server: a second `#[tokio::test]` gets its own runtime, which drops (killing
    // the shared HTTP server task) when the first finishes — the harness flake.
    companion_sync_returns_outcome_in_band(&platform).await;
    companion_sync_rejects_session_topology_commands(&platform).await;
    companion_sync_import_fallback_reports_ok(&platform).await;
    companion_sync_contract_gaps_closed(&platform).await;
    companion_sync_pre_run_check_rejects_broken_suspend_contract(&platform).await;
    companion_sync_instantiate_creates_model_cid(&platform).await;
    companion_sync_inspect_error_shows_context(&platform).await;
    companion_sync_inspect_error_reports_recovery(&platform).await;
    companion_dry_run_resumes_across_instantiations(&platform).await;
    companion_unnamed_draft_resumes_across_instantiations(&platform).await;
    math_for_each_blocks_and_iteration(&platform).await;
    join_barrier_waits_for_a_retrying_branch(&platform).await;
    chained_join_counts_only_a_fired_upstream_join(&platform).await;
}

/// Findings #62/#63 (HTTPS drive pre-flight): the `/sync` contract gaps.
/// #62 — a synchronous companion RPC is a deliberate request: the 1-second
/// identical-command dedup guard (a WS double-submit protection) must NOT
/// silently swallow a repeat; the guard stays intact for the WS path.
/// #63 — a malformed command answered with a `Syntax: …` usage hint did
/// nothing: the envelope must say `ok:false` with the hint as the error.
async fn companion_sync_contract_gaps_closed(platform: &Platform) {
    let po = PostOffice::new(platform);
    let sid = "ws-770004-1";
    let in_route = "ws.770004.1.in";
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
    assert!(knowledge_graph::commands::has_session(sid), "session ready");

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

    // #62 — the same command twice, back-to-back (well inside the 1s window):
    // both must execute and both envelopes must carry the echo + output
    let first = sync_cmd(platform, sid, "list nodes").await;
    let second = sync_cmd(platform, sid, "list nodes").await;
    for (label, envelope) in [("first", &first), ("second", &second)] {
        assert_eq!(
            envelope["ok"],
            serde_json::json!(true),
            "{label} repeat must execute: {envelope}"
        );
        assert!(
            envelope["output"].as_array().is_some_and(|a| a
                .iter()
                .any(|l| l.as_str().is_some_and(|s| s.contains("list nodes")))),
            "{label} envelope must carry the echo (not silently dropped): {envelope}"
        );
    }

    // #63 — a malformed command answered with the usage hint is a failed
    // command: ok:false, the hint in-band as the error
    let bad = sync_cmd(platform, sid, "connect a to b with type x").await;
    assert_eq!(
        bad["ok"],
        serde_json::json!(false),
        "usage response must classify as failure: {bad}"
    );
    assert!(
        bad["error"]
            .as_str()
            .is_some_and(|e| e.starts_with("Syntax:")),
        "the usage hint must be the in-band error: {bad}"
    );

    // the WS-path guard is untouched: two identical NON-direct commands within
    // the window — the second is dropped, so the console sees exactly one echo
    let sid2 = "ws-770005-1";
    let in2 = "ws.770005.1.in";
    po.send(
        EventEnvelope::new()
            .set_to("graph.command.singleton")
            .set_raw_body(rmpv::Value::Map(vec![
                (rmpv::Value::from("type"), rmpv::Value::from("open")),
                (rmpv::Value::from("in"), rmpv::Value::from(in2)),
            ])),
    )
    .await
    .expect("open dispatched");
    for _ in 0..50 {
        if knowledge_graph::commands::has_session(sid2) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    let tap = Arc::new(Mutex::new(Vec::<String>::new()));
    platform
        .register("ws.770005.1.out", Arc::new(OutTap { seen: tap.clone() }), 1)
        .expect("register out tap");
    for _ in 0..2 {
        po.send(
            EventEnvelope::new()
                .set_to("graph.command.singleton")
                .set_raw_body(rmpv::Value::Map(vec![
                    (rmpv::Value::from("type"), rmpv::Value::from("command")),
                    (rmpv::Value::from("in"), rmpv::Value::from(in2)),
                    (
                        rmpv::Value::from("out"),
                        rmpv::Value::from("ws.770005.1.out"),
                    ),
                    (
                        rmpv::Value::from("message"),
                        rmpv::Value::from("list nodes"),
                    ),
                ])),
        )
        .await
        .expect("ws command dispatched");
    }
    tokio::time::sleep(Duration::from_millis(300)).await;
    let echoes = tap
        .lock()
        .expect("tap")
        .iter()
        .filter(|l| l.contains("list nodes"))
        .count();
    assert_eq!(
        1, echoes,
        "WS double-submit guard must still drop the duplicate (saw {echoes} echoes)"
    );
}

/// Chained joins: an upstream join that evaluated and SANK is still marked in
/// skill_run (its skill ran fine), so a downstream join must judge it by the
/// OUTCOME it recorded, not the run mark. Topology: slow-x (200 ms) and
/// fast-y feed j-one; j-one chains into j-two alongside pace-z (100 ms).
/// fast-y makes j-one evaluate-and-sink at ~1 ms; pace-z reaches j-two at
/// ~100 ms — before the fix, j-two counted the sunk j-one off its run mark
/// and fired prematurely, losing branch X from the output.
async fn chained_join_counts_only_a_fired_upstream_join(platform: &Platform) {
    let reply = run_graph(
        platform,
        "rust-join-chain",
        serde_json::json!({}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "join-chain failed: {:?}", reply.body());
    let mm = body_map(&reply);
    // j-two waited for j-one to actually FIRE: all three branches present
    assert_eq!(Some(Value::from("X")), mm.get_element("x"));
    assert_eq!(Some(Value::from("Y")), mm.get_element("y"));
    assert_eq!(Some(Value::from("Z")), mm.get_element("z"));
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

/// Increment 54 (parity F6): the provider cache key is built EXCLUSIVELY from
/// dictionary-declared inputs (Java `makeRegularHttpCall` reads the
/// `{node}.dd.{alias}.*` namespace). Two sequential fetchers stage a
/// DIFFERENT undeclared fetcher-level parameter (`extra` = alpha/beta) while
/// their dictionaries declare the same `person_id` — Java reuses the cached
/// response; the pre-fix Rust keyed on the whole staged map and re-fired the
/// provider call.
async fn fetcher_cache_key_uses_dictionary_declared_inputs_only(platform: &Platform) {
    let before = CACHE_PROBE_CALLS.load(std::sync::atomic::Ordering::SeqCst);
    let reply = run_graph(
        platform,
        "rust-cache-key",
        serde_json::json!({"person_id": 100}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(
        200,
        reply.status(),
        "rust-cache-key failed: {:?}",
        reply.body()
    );
    let after = CACHE_PROBE_CALLS.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(
        1,
        after - before,
        "equivalent dictionary requests must share ONE provider call (Java parity)"
    );
    // both fetches surfaced the SAME cached response
    let mm = body_map(&reply);
    let count1 = mm.get_element("count1");
    let count2 = mm.get_element("count2");
    assert!(count1.is_some(), "count1 missing: {:?}", reply.body());
    assert_eq!(
        count1, count2,
        "the second fetch must reuse the first fetch's cached response"
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

    // --- the api.fetcher stamps x-ttl from its effective deadline (Java
    // DeadlineEnforcementTest twins): the mock MDM endpoint echoes the header
    // exactly as observed on the wire. Node ttl 7s -> "7000".
    let reply = run_graph(
        &platform,
        "unit-test-ttl-wire",
        serde_json::json!({"person_id": 100}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "ttl-wire failed: {:?}", reply.body());
    let mm = body_map(&reply);
    assert_eq!(
        Some(Value::from("7000")),
        mm.get_element("observed_ttl"),
        "the api.fetcher must stamp x-ttl from the node ttl"
    );
    // without a node ttl the fetcher stamps the propagated model.ttl. The
    // Java twin pins "30000" (its rest.yaml HTTP entry); this harness drives
    // the graph-executor flow directly, so the propagated value is the flow
    // template's own ttl (60s) - same mechanism, harness-visible number.
    let reply = run_graph(
        &platform,
        "unit-test-ttl-wire-default",
        serde_json::json!({"person_id": 100}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(
        200,
        reply.status(),
        "ttl-wire-default failed: {:?}",
        reply.body()
    );
    let mm = body_map(&reply);
    assert_eq!(
        Some(Value::from("60000")),
        mm.get_element("observed_ttl"),
        "the api.fetcher must stamp the propagated model.ttl when no node ttl is declared"
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

    // --- tutorial 13: graph.task invoking the AsyncHttpClient - the input
    // mapping stages 'model.person_id' and resolves it as a dynamic variable
    // in the url; success proves CompileGraph resolved ${rest.server.port:8080}
    // when the deployed model was loaded
    let reply = run_graph(
        &platform,
        "tutorial-13",
        serde_json::json!({"person_id": 100}),
        serde_json::json!({}),
    )
    .await;
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("100")), mm.get_element("profile.id"));
    assert_eq!(Some(Value::from("Peter")), mm.get_element("profile.name"));
    assert_eq!(
        Some(Value::from("100 World Blvd")),
        mm.get_element("profile.address")
    );
    // 'text(5000) -> headers.x-ttl' sets the HTTP timeout and rides the wire
    // as the X-TTL request header - the mock echoes what it observed
    assert_eq!(Some(Value::from("5000")), mm.get_element("observed_ttl"));
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
    // 'text(alpha) -> model.token' stages a model variable (Event Script
    // parity) and the next entry references it as a dynamic variable into a
    // composite body path
    assert_eq!(
        Some(Value::from("Bearer alpha")),
        mm.get_element("received.nested.auth")
    );
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

    // --- unit-test-task-7: a foreign route reached through the declarative
    // event-over-http map (yaml.event.over.http) - the way python/node.js
    // polyglot functions join a knowledge graph. The stub peer speaks the
    // standard envelope wire format on /api/event.
    start_stub_peer(8391).await; // matches stub.peer.port in application.yml
    let reply = run_graph(
        &platform,
        "unit-test-task-7",
        serde_json::json!({"text": "polyglot"}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(
        200,
        reply.status(),
        "foreign route failed: {}",
        event_script::conversions::to_json_string(reply.body())
    );
    let mm = body_map(&reply);
    assert_eq!(Some(Value::from("stub")), mm.get_element("language"));
    assert_eq!(
        Some(Value::from("polyglot.stub.function")),
        mm.get_element("route")
    );
    assert_eq!(Some(Value::from("polyglot")), mm.get_element("echo.text"));

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

    // --- tutorial-13 negative: the mock mdm service throws for an unknown
    // person id and the graph returns the HTTP error as its output
    let reply = run_graph(
        &platform,
        "tutorial-13",
        serde_json::json!({"person_id": 999}),
        serde_json::json!({}),
    )
    .await;
    assert_ne!(200, reply.status());
    let text = event_script::conversions::to_json_string(reply.body());
    assert!(
        text.contains("Profile 999 not found"),
        "unexpected error response: {text}"
    );

    // --- unit-test-task-6: an input mapping must not overwrite engine-managed
    // model metadata - the deployment gate rejects the graph so it answers 404
    // as if nonexistent (compiled-or-404)
    let reply = run_graph(
        &platform,
        "unit-test-task-6",
        serde_json::json!({"hello": "world"}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(404, reply.status());
    let text = event_script::conversions::to_json_string(reply.body());
    assert!(
        text.contains("not found"),
        "unexpected error response: {text}"
    );
}

async fn join_loop_retirement_and_health(platform: &Platform) {
    let platform = platform.clone();

    // --- rust-join: the join barrier waits for both branches
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

/// The Java `GraphSuspendResumeTest` twin: workflow suspension end to end
/// against the temp-file mock store — persistence-envelope shape, resume
/// without re-execution, multi-checkpoint, join-across-suspension,
/// fresh-vs-expired as application logic, and the forged-record
/// reserved-key strip.
async fn suspend_resume_matches_java_semantics(platform: &Platform) {
    // --- suspend-1: suspend at step-1, resume continues without re-execution
    let cid = "wf-suspend-basic-001";
    let first = run_graph_cid(platform, "unit-test-suspend-1", cid, serde_json::json!({})).await;
    assert_eq!(200, first.status(), "run 1: {:?}", first.body());
    let suspended = body_map(&first);
    assert_eq!(
        Some(Value::from("suspended")),
        suspended.get_element("type")
    );
    assert_eq!(Some(Value::from(cid)), suspended.get_element("cid"));
    assert_eq!(1, step_count("one", cid));
    assert_eq!(0, step_count("two", cid));
    // the business correlation ID propagates through the walker's internal
    // events into every skill and task - not the engine's callback IDs
    assert_eq!(Some(cid.to_string()), step_business_cid("one", cid));
    // the persisted record has the documented envelope shape and no reserved
    // model keys
    let record = MultiLevelMap::from_value(unpack_value(
        &std::fs::read(store_file("unit-test-suspend-1", cid)).expect("suspension record file"),
    ));
    assert_eq!(Some(Value::from("step-1")), record.get_element("data.node"));
    assert_eq!(Some(Value::from(cid)), record.get_element("data.cid"));
    // cid + graph form the retrieval key: the record carries the graph that suspended
    assert_eq!(
        Some(Value::from("unit-test-suspend-1")),
        record.get_element("data.graph")
    );
    assert_eq!(
        Some(Value::from(1)),
        record.get_element("data.model.step1_count")
    );
    for reserved in ["cid", "instance", "flow", "trace", "run"] {
        assert_eq!(
            None,
            record.get_element(&format!("data.model.{reserved}")),
            "reserved model key '{reserved}' must not persist"
        );
    }
    assert_eq!(
        Some(Value::from(true)),
        record.get_element("data.run.step-1")
    );
    // run 2 with the same correlation ID: resume continues past the checkpoint
    let second = run_graph_cid(platform, "unit-test-suspend-1", cid, serde_json::json!({})).await;
    assert_eq!(200, second.status(), "run 2: {:?}", second.body());
    let completed = body_map(&second);
    assert_eq!(Some(Value::from("two")), completed.get_element("step"));
    assert_eq!(
        Some(Value::from(1)),
        completed.get_element("prior"),
        "restored model.step1_count must reach step-2"
    );
    assert_eq!(
        1,
        step_count("one", cid),
        "the suspension point must not re-execute"
    );
    assert_eq!(1, step_count("two", cid));
    assert_eq!(
        Some("resume"),
        second.header("x-run"),
        "graph.resume must flag the resumed condition"
    );
    assert!(
        !store_file("unit-test-suspend-1", cid).exists(),
        "the record must be consumed on resume"
    );

    // --- suspend-2: multiple checkpoints, three runs, one cid
    let cid = "wf-suspend-multi-002";
    let r1 = run_graph_cid(platform, "unit-test-suspend-2", cid, serde_json::json!({})).await;
    assert_eq!(
        Some(Value::from("suspended")),
        body_map(&r1).get_element("type")
    );
    let r2 = run_graph_cid(platform, "unit-test-suspend-2", cid, serde_json::json!({})).await;
    assert_eq!(
        Some(Value::from("suspended")),
        body_map(&r2).get_element("type")
    );
    let r3 = run_graph_cid(platform, "unit-test-suspend-2", cid, serde_json::json!({})).await;
    assert_eq!(200, r3.status(), "run 3: {:?}", r3.body());
    let completed = body_map(&r3);
    assert_eq!(Some(Value::from("c")), completed.get_element("step"));
    assert_eq!(
        Some(Value::from(1)),
        completed.get_element("prior"),
        "model.b_count must survive the second suspension"
    );
    for step in ["a", "b", "c"] {
        assert_eq!(
            1,
            step_count(step, cid),
            "step {step} must run exactly once"
        );
    }

    // --- suspend-3: a join barrier is still satisfied after resume
    let cid = "wf-suspend-join-003";
    let r1 = run_graph_cid(platform, "unit-test-suspend-3", cid, serde_json::json!({})).await;
    assert_eq!(200, r1.status(), "join run 1: {:?}", r1.body());
    assert_eq!(
        Some(Value::from("suspended")),
        body_map(&r1).get_element("type")
    );
    // without the restored bookkeeping, the join would never see the
    // pre-suspension branch and the run would time out
    let r2 = run_graph_cid(platform, "unit-test-suspend-3", cid, serde_json::json!({})).await;
    assert_eq!(200, r2.status(), "join run 2: {:?}", r2.body());
    let completed = body_map(&r2);
    assert_eq!(Some(Value::from("final")), completed.get_element("step"));
    assert_eq!(Some(Value::from(1)), completed.get_element("prior"));
    assert_eq!(
        1,
        step_count("gamma", cid),
        "gamma must not re-execute after resume"
    );

    // --- suspend-4: fresh-vs-expired is APPLICATION logic on the resume path
    // (absent and expired records are indistinguishable to the engine: the
    // graph's own gate rejects an invalid fresh request with a declaratively
    // staged 404, and the reply carries run=fresh so the caller knows why)
    let cid = "wf-suspend-fresh-004";
    let response = run_graph_cid(platform, "unit-test-suspend-4", cid, serde_json::json!({})).await;
    assert_eq!(404, response.status(), "gate: {:?}", response.body());
    let body = body_map(&response);
    assert_eq!(Some(Value::from("no-record")), body.get_element("reason"));
    assert_eq!(
        Some(Value::from("fresh")),
        body.get_element("run"),
        "graph.resume must flag the fresh condition"
    );
    assert_eq!(Some(Value::from(404)), body.get_element("status"));
    assert_eq!(
        0,
        step_count("x", cid),
        "the gate must not run the normal path"
    );
    // a valid fresh request passes the same gate (the probe reads
    // input.body.start via the null-safe '=' prefix)
    let accepted = run_graph_cid(
        platform,
        "unit-test-suspend-4",
        cid,
        serde_json::json!({"start": true}),
    )
    .await;
    assert_eq!(200, accepted.status(), "accepted: {:?}", accepted.body());

    // --- suspend-5: an expired record falls back to a fresh run
    let cid = "wf-suspend-expire-005";
    let r1 = run_graph_cid(platform, "unit-test-suspend-5", cid, serde_json::json!({})).await;
    assert_eq!(
        Some(Value::from("suspended")),
        body_map(&r1).get_element("type")
    );
    assert_eq!(1, step_count("expiry", cid));
    tokio::time::sleep(Duration::from_millis(1200)).await;
    // the 1s record has expired: the resume falls back to a fresh run and
    // suspends again
    let r2 = run_graph_cid(platform, "unit-test-suspend-5", cid, serde_json::json!({})).await;
    assert_eq!(
        Some(Value::from("suspended")),
        body_map(&r2).get_element("type")
    );
    assert_eq!(
        2,
        step_count("expiry", cid),
        "an expired record means a fresh run"
    );

    // --- forged record: the store is pluggable, so a record is EXTERNAL
    // input - reserved keys injected by a hostile writer must never reach
    // the state machine (model.cid is a capability)
    let cid = "wf-suspend-forge-006";
    let first = run_graph_cid(platform, "unit-test-suspend-1", cid, serde_json::json!({})).await;
    assert_eq!(200, first.status());
    let file = store_file("unit-test-suspend-1", cid);
    let mut record = MultiLevelMap::from_value(unpack_value(
        &std::fs::read(&file).expect("record to forge"),
    ));
    // the forged keys are inserted LITERALLY into the record's model map —
    // "cid.x" / "ttl[0]" must arrive as single literal keys (a set_element
    // path write would nest them), because the composite-path shape is
    // exactly the bypass vector: a path-interpreting merge would descend
    // into and REPLACE the real model.cid / model.ttl despite the
    // literal-name reserved-key filter
    {
        let mut wrapper = record.to_value();
        let model = get_map_mut(&mut wrapper, &["data", "model"]).expect("record model map");
        for (key, value) in [
            ("cid", "stolen-cid"),
            ("instance", "bogus"),
            ("flow", "bogus"),
            ("run", "resume"),
            ("cid.x", "path-bypass"),
            ("ttl[0]", "path-bypass"),
        ] {
            model.push((Value::from(key), Value::from(value)));
        }
        record = MultiLevelMap::from_value(wrapper);
    }
    std::fs::write(&file, pack_value(&record.to_value())).expect("write forged record");
    // resume with the REAL correlation ID: the workflow continues, and none
    // of the forged reserved keys reach the state machine — neither the
    // flat names nor the composite-path shapes (the merge is a literal
    // key-level putAll, Java parity)
    let second = run_graph_cid(platform, "unit-test-suspend-1", cid, serde_json::json!({})).await;
    assert_eq!(200, second.status(), "forged resume: {:?}", second.body());
    let completed = body_map(&second);
    assert_eq!(Some(Value::from("two")), completed.get_element("step"));
    assert_eq!(
        Some(cid.to_string()),
        step_business_cid("two", cid),
        "the current run's identity must survive a forged record"
    );
    // the step counter is keyed by the cid DELIVERED through the
    // `model.cid -> cid` input mapping: a composite-path forgery that
    // replaced model.cid with a fresh map would break this key
    assert_eq!(
        1,
        step_count("two", cid),
        "model.cid must survive a composite-path forgery (cid.x / ttl[0])"
    );

    // --- suspend-6: JUMP MODE - a decision jumps to the island-anchored
    // suspend node and is RE-EXECUTED against the new input on every resume
    let cid = "wf-suspend-jump-007";
    // run 1: no decision - the gate jumps to suspend and stages its own waiting reply
    let r1 = run_graph_cid(
        platform,
        "unit-test-suspend-6",
        cid,
        serde_json::json!({"noise": true}),
    )
    .await;
    assert_eq!(200, r1.status(), "jump run 1: {:?}", r1.body());
    let waiting1 = body_map(&r1);
    assert_eq!(
        Some(Value::from("waiting")),
        waiting1.get_element("stage"),
        "the decision stages the caller's reply"
    );
    assert_eq!(Some(Value::from("fresh")), waiting1.get_element("run"));
    // the persisted suspension point is the DECISION that jumped
    let record = MultiLevelMap::from_value(unpack_value(
        &std::fs::read(store_file("unit-test-suspend-6", cid)).expect("jump record"),
    ));
    assert_eq!(Some(Value::from("gate")), record.get_element("data.node"));
    // run 2: still no decision - the gate re-executes and re-suspends (before
    // jump-mode re-execution this dead-ended: a node marked seen never
    // re-dispatches, and the persisted seen marks include the gate)
    let r2 = run_graph_cid(
        platform,
        "unit-test-suspend-6",
        cid,
        serde_json::json!({"noise": true}),
    )
    .await;
    assert_eq!(200, r2.status(), "jump run 2: {:?}", r2.body());
    let waiting2 = body_map(&r2);
    assert_eq!(Some(Value::from("waiting")), waiting2.get_element("stage"));
    assert_eq!(
        Some(Value::from("resume")),
        waiting2.get_element("run"),
        "the second wait is a resumed run"
    );
    let record2 = MultiLevelMap::from_value(unpack_value(
        &std::fs::read(store_file("unit-test-suspend-6", cid)).expect("re-suspension record"),
    ));
    assert_eq!(
        Some(Value::from("gate")),
        record2.get_element("data.node"),
        "re-suspension re-persists"
    );
    assert_eq!(
        0,
        step_count("go-step", cid),
        "the continuing path must not run yet"
    );
    // run 3: the decision arrives - the re-executed gate routes onward
    let r3 = run_graph_cid(
        platform,
        "unit-test-suspend-6",
        cid,
        serde_json::json!({"decision": "go"}),
    )
    .await;
    assert_eq!(200, r3.status(), "jump run 3: {:?}", r3.body());
    assert_eq!(
        Some(Value::from("go-step")),
        body_map(&r3).get_element("step")
    );
    assert_eq!(1, step_count("go-step", cid));
    assert!(
        !store_file("unit-test-suspend-6", cid).exists(),
        "the record must be consumed on the final resume"
    );

    // --- compat: the retired 'suspend=true' property with NO drawn edge
    // deploys (gate WARN) and never suspends
    let cid = "wf-suspend-compat-008";
    let response = run_graph_cid(
        platform,
        "unit-test-suspend-compat-1",
        cid,
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, response.status(), "compat: {:?}", response.body());
    assert_eq!(
        Some(Value::from("finished")),
        body_map(&response).get_element("stage")
    );
    assert_eq!(1, step_count("compat-step", cid));
    assert!(
        !store_file("unit-test-suspend-compat-1", cid).exists(),
        "the retired property must not suspend"
    );
}

/// The field scenario (Java `sameCorrelationIdSuspendsIndependentlyPerGraph`):
/// one business transaction suspends in more than one graph - each record is
/// scoped by graph + cid, so a shared business correlation ID never collides
/// across domains or subgraphs.
async fn same_cid_suspends_independently_per_graph(platform: &Platform) {
    let cid = "wf-suspend-isolation-009";
    let r1 = run_graph_cid(platform, "unit-test-suspend-1", cid, serde_json::json!({})).await;
    assert_eq!(
        Some(Value::from("suspended")),
        body_map(&r1).get_element("type")
    );
    let r2 = run_graph_cid(platform, "unit-test-suspend-5", cid, serde_json::json!({})).await;
    assert_eq!(
        Some(Value::from("suspended")),
        body_map(&r2).get_element("type")
    );
    let record1 = MultiLevelMap::from_value(unpack_value(
        &std::fs::read(store_file("unit-test-suspend-1", cid)).expect("suspend-1 record"),
    ));
    assert_eq!(
        Some(Value::from("unit-test-suspend-1")),
        record1.get_element("data.graph")
    );
    let record5 = MultiLevelMap::from_value(unpack_value(
        &std::fs::read(store_file("unit-test-suspend-5", cid)).expect("suspend-5 record"),
    ));
    assert_eq!(
        Some(Value::from("unit-test-suspend-5")),
        record5.get_element("data.graph")
    );
    // resuming one graph consumes only its own record
    let done = run_graph_cid(platform, "unit-test-suspend-1", cid, serde_json::json!({})).await;
    assert_eq!(200, done.status(), "resume: {:?}", done.body());
    assert_eq!(
        Some(Value::from("two")),
        body_map(&done).get_element("step")
    );
    assert!(!store_file("unit-test-suspend-1", cid).exists());
    assert!(
        store_file("unit-test-suspend-5", cid).exists(),
        "another graph's record for the same cid must survive"
    );
}

/// The orchestrator pattern (Java `orchestratorParentDrivesSuspendingSubgraphPath`):
/// the parent delegates a processing path to a subgraph via graph.extension;
/// the subgraph inherits the business correlation ID, suspends at its own
/// checkpoint under graph:{subgraph}:{cid}, and the parent routes on the
/// suspended reply. Re-invoking the parent with the same cid resumes the
/// subgraph past its checkpoint.
async fn orchestrator_parent_drives_suspending_subgraph_path(platform: &Platform) {
    let cid = "wf-orchestrator-010";
    // run 1: the subgraph reaches its checkpoint - the parent reports 'waiting'
    let first = run_graph_cid(
        platform,
        "unit-test-orchestrator",
        cid,
        serde_json::json!({"item": "widget-7"}),
    )
    .await;
    assert_eq!(200, first.status(), "run 1: {:?}", first.body());
    let waiting = body_map(&first);
    assert_eq!(Some(Value::from("waiting")), waiting.get_element("stage"));
    assert_eq!(Some(Value::from("suspended")), waiting.get_element("type"));
    assert_eq!(
        Some(Value::from(cid)),
        waiting.get_element("cid"),
        "the subgraph must suspend under the business cid, not a per-call random id"
    );
    assert_eq!(1, step_count("sub", cid));
    // the parent's business correlation ID propagated into the subgraph's task
    assert_eq!(Some(cid.to_string()), step_business_cid("sub", cid));
    // the record is scoped by the SUBGRAPH's id and only that record exists
    let stored = MultiLevelMap::from_value(unpack_value(
        &std::fs::read(store_file("unit-test-sub-suspend", cid)).expect("subgraph record"),
    ));
    assert_eq!(
        Some(Value::from("unit-test-sub-suspend")),
        stored.get_element("data.graph")
    );
    assert_eq!(
        Some(Value::from("prepare")),
        stored.get_element("data.node")
    );
    assert_eq!(
        Some(Value::from("widget-7")),
        stored.get_element("data.model.item")
    );
    assert!(
        !store_file("unit-test-orchestrator", cid).exists(),
        "the parent did not suspend - only the subgraph's record exists"
    );
    // run 2 with the same cid: the subgraph resumes past its checkpoint
    let second = run_graph_cid(
        platform,
        "unit-test-orchestrator",
        cid,
        serde_json::json!({"item": "ignored-on-resume"}),
    )
    .await;
    assert_eq!(200, second.status(), "run 2: {:?}", second.body());
    let done = body_map(&second);
    assert_eq!(Some(Value::from("done")), done.get_element("stage"));
    assert_eq!(Some(Value::from("resume")), done.get_element("run"));
    assert_eq!(Some(Value::from(cid)), done.get_element("cid_check"));
    assert_eq!(
        Some(Value::from("widget-7")),
        done.get_element("item"),
        "the restored model must carry the original item"
    );
    assert_eq!(Some(Value::from(1)), done.get_element("result.prior"));
    assert_eq!(
        1,
        step_count("sub", cid),
        "the checkpoint step must not re-execute"
    );
    assert_eq!(1, step_count("deliver", cid));
    assert!(
        !store_file("unit-test-sub-suspend", cid).exists(),
        "the record must be consumed on resume"
    );
}

/// Java `GraphErrorContextTest`: when a failed node routes to its exception=
/// handler, the walker stages error.source/code/message (and error.stack when
/// the failure carries one - this engine has no native stack-trace transport,
/// a documented port divergence), so ONE island-anchored handler serves every
/// node without naming any failing node in its data mapping. Also pins the
/// reserved 'error' alias rejection (compiled-or-404).
async fn generic_exception_context_serves_every_node(platform: &Platform) {
    // a failing composable function (v1.demo.task throws 400 "just a test")
    let cid = "wf-error-context-011";
    let response = run_graph_cid(
        platform,
        "unit-test-error-context",
        cid,
        serde_json::json!({"mode": "task"}),
    )
    .await;
    assert_eq!(200, response.status(), "task mode: {:?}", response.body());
    let body = body_map(&response);
    assert_eq!(Some(Value::from("handled")), body.get_element("stage"));
    // the handler reads the generic context - it never names the failing node
    assert_eq!(Some(Value::from("fail-task")), body.get_element("source"));
    assert_eq!(Some(Value::from(400)), body.get_element("code"));
    let message =
        event_script::conversions::display(&body.get_element("message").unwrap_or(Value::Nil));
    assert!(
        message.contains("just a test"),
        "unexpected message: {message}"
    );
    // an exception handler node connects onward like any node
    assert_eq!(Some(Value::from(1)), body.get_element("relay_count"));
    assert_eq!(1, step_count("relay", cid));

    // the SAME handler serves a failing HTTP call (the mock's x-exception -> 401)
    let cid2 = "wf-error-context-012";
    let response = run_graph_cid(
        platform,
        "unit-test-error-context",
        cid2,
        serde_json::json!({"mode": "http"}),
    )
    .await;
    assert_eq!(200, response.status(), "http mode: {:?}", response.body());
    let body = body_map(&response);
    assert_eq!(Some(Value::from("handled")), body.get_element("stage"));
    assert_eq!(
        Some(Value::from("fetch-profile")),
        body.get_element("source")
    );
    assert_eq!(Some(Value::from(401)), body.get_element("code"));
    let message =
        event_script::conversions::display(&body.get_element("message").unwrap_or(Value::Nil));
    assert!(
        message.contains("simulated exception"),
        "unexpected message: {message}"
    );
    assert_eq!(1, step_count("relay", cid2));

    // a node aliased 'error' shadows the exception-context namespace - the
    // graph model rejects the reserved alias, so the gate rejection is
    // inherited and the endpoint answers 404 as if nonexistent
    let alias_cid = uuid::Uuid::new_v4().simple().to_string();
    let rejected = run_graph_cid(
        platform,
        "unit-test-error-alias",
        &alias_cid,
        serde_json::json!({}),
    )
    .await;
    assert_eq!(404, rejected.status(), "alias: {:?}", rejected.body());
    let text = event_script::conversions::display(rejected.body());
    assert!(text.contains("not found"), "unexpected: {text}");
}

/// The dry-run twin of the generic exception context (Java
/// `CompanionSyncTest.dryRunStagesErrorContextAndInspectErrorShowsIt`): the
/// traveler stages error.source/code/message when a failed node routes to its
/// exception= handler, and 'inspect error' shows the staged context - the
/// 'error' namespace is a first-class state-machine citizen like 'model', so
/// the inspect command needs no special case.
async fn companion_sync_inspect_error_shows_context(platform: &Platform) {
    let po = PostOffice::new(platform);
    let sid = "ws-770013-1";
    let in_route = "ws.770013.1.in";

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

    // the model must come from the deployed classpath copy, not a stale export
    let temp = std::path::Path::new("/tmp/graph/unit-test-error-context.json");
    if temp.exists() {
        std::fs::remove_file(temp).expect("stale temp copy removed");
    }
    let imported = sync_cmd(platform, sid, "import graph from unit-test-error-context").await;
    assert_eq!(
        imported["ok"],
        serde_json::json!(true),
        "import must succeed: {imported}"
    );
    let instantiated = sync_cmd(
        platform,
        sid,
        "instantiate graph\ntext(task) -> input.body.mode\ntext(dry-err-1) -> model.cid",
    )
    .await;
    assert_eq!(
        instantiated["ok"],
        serde_json::json!(true),
        "instantiate -> ok:true: {instantiated}"
    );
    let ran = sync_cmd(platform, sid, "run").await;
    assert_eq!(ran["ok"], serde_json::json!(true), "dry-run: {ran}");
    let result = &ran["result"][0];
    assert_eq!(
        "handled",
        result["output"]["body"]["stage"].as_str().unwrap_or(""),
        "dry-run output: {ran}"
    );
    assert_eq!(
        "fail-task",
        result["output"]["body"]["source"].as_str().unwrap_or(""),
        "dry-run output: {ran}"
    );
    // 'inspect error' returns the staged context - same mechanics as 'inspect model'
    let inspected = sync_cmd(platform, sid, "inspect error").await;
    assert_eq!(
        inspected["ok"],
        serde_json::json!(true),
        "inspect error: {inspected}"
    );
    let context = &inspected["result"][0];
    assert_eq!("error", context["inspect"].as_str().unwrap_or(""));
    assert_eq!(
        "fail-task",
        context["outcome"]["source"].as_str().unwrap_or(""),
        "context: {inspected}"
    );
    assert_eq!(
        400,
        context["outcome"]["code"].as_i64().unwrap_or(0),
        "context: {inspected}"
    );
    assert_eq!(
        "just a test",
        context["outcome"]["message"].as_str().unwrap_or(""),
        "context: {inspected}"
    );
}

/// Java `DynamicStatementTargetTest`: every statement command resolves
/// {dynamic variables} - RESET:/NEXT: are pinned end-to-end by tutorial-12's
/// generic error handler (RESET:/NEXT: {error.source}); this pins the
/// remaining positions - a THEN: jump target and a DELAY: value from the model.
async fn statement_commands_resolve_dynamic_variables(platform: &Platform) {
    // the gate jumps to 'THEN: {model.hop}' with 'DELAY: {model.backoff}'
    let reply = run_graph(
        platform,
        "unit-test-dynamic-jump",
        serde_json::json!({"hop": "stage-b"}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "dynamic: {:?}", reply.body());
    let body = body_map(&reply);
    assert_eq!(
        Some(Value::from("dynamic")),
        body.get_element("route_taken")
    );
    assert_eq!(
        Some(Value::from(15)),
        body.get_element("applied_delay"),
        "DELAY: {{model.backoff}} must resolve to the staged value"
    );
    // an unmatched condition takes the literal alternative
    let reply = run_graph(
        platform,
        "unit-test-dynamic-jump",
        serde_json::json!({"hop": "nowhere"}),
        serde_json::json!({}),
    )
    .await;
    assert_eq!(200, reply.status(), "static: {:?}", reply.body());
    assert_eq!(
        Some(Value::from("static")),
        body_map(&reply).get_element("route_taken")
    );
}

/// Java `GraphErrorContextTest.successfulRetryResolvesTheErrorContext`: the
/// generic one-shot handler disarms the simulated exception and retries via
/// RESET:/NEXT: {error.source}; when the retried source succeeds, the walker
/// resolves the virtual 'error' node - code=200, source kept, details gone.
async fn successful_retry_resolves_the_error_context(platform: &Platform) {
    let cid = "wf-error-recovery-013";
    let reply = run_graph_cid(
        platform,
        "unit-test-error-recovery",
        cid,
        serde_json::json!({"start": true}),
    )
    .await;
    assert_eq!(200, reply.status(), "recovery: {:?}", reply.body());
    let body = body_map(&reply);
    assert_eq!(
        Some(Value::from("Peter")),
        body.get_element("name"),
        "the retry must deliver the real result"
    );
    assert_eq!(Some(Value::from(200)), body.get_element("recovered_code"));
    assert_eq!(
        Some(Value::from("work")),
        body.get_element("recovered_source")
    );
    assert_eq!(
        None,
        body.get_element("stale_message"),
        "the failure message must be removed on recovery"
    );
}

/// Java `CompanionSyncTest.successfulRetryResolvesErrorContextInDryRun`:
/// tutorial-12's generic handler recovers the fetcher in the dry-run lane and
/// 'inspect error' reports the RECOVERY, not the stale failure.
async fn companion_sync_inspect_error_reports_recovery(platform: &Platform) {
    let po = PostOffice::new(platform);
    let sid = "ws-770014-1";
    let in_route = "ws.770014.1.in";

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

    let temp = std::path::Path::new("/tmp/graph/tutorial-12.json");
    if temp.exists() {
        std::fs::remove_file(temp).expect("stale temp copy removed");
    }
    let imported = sync_cmd(platform, sid, "import graph from tutorial-12").await;
    assert_eq!(
        imported["ok"],
        serde_json::json!(true),
        "import: {imported}"
    );
    let instantiated = sync_cmd(
        platform,
        sid,
        "instantiate graph\nint(100) -> input.body.person_id\nboolean(true) -> input.body.exception",
    )
    .await;
    assert_eq!(
        instantiated["ok"],
        serde_json::json!(true),
        "instantiate: {instantiated}"
    );
    let ran = sync_cmd(platform, sid, "run").await;
    assert_eq!(ran["ok"], serde_json::json!(true), "dry-run: {ran}");
    assert_eq!(
        "Peter",
        ran["result"][0]["output"]["body"]["name"]
            .as_str()
            .unwrap_or(""),
        "the retried fetcher must deliver the profile: {ran}"
    );
    // the virtual 'error' node reports the RECOVERY, not the stale failure
    let inspected = sync_cmd(platform, sid, "inspect error").await;
    assert_eq!(
        inspected["ok"],
        serde_json::json!(true),
        "inspect error: {inspected}"
    );
    let outcome = &inspected["result"][0]["outcome"];
    assert_eq!(200, outcome["code"].as_i64().unwrap_or(0), "{inspected}");
    assert_eq!(
        "fetcher",
        outcome["source"].as_str().unwrap_or(""),
        "{inspected}"
    );
    assert!(
        outcome["message"].is_null(),
        "the failure message must be removed on recovery: {inspected}"
    );
}

/// Java `CompanionSyncTest.dryRunResumesAcrossInstantiations` - the dry-run
/// twin of the field regression that surfaced on tutorial-14: the store
/// contract scopes records by graph + cid, so the dry-run instance must
/// present the model's stable identity (root name); an ephemeral
/// per-instantiation handle writes the suspension under a key no later
/// instantiation can ever read, and every resume silently restarts fresh.
async fn companion_dry_run_resumes_across_instantiations(platform: &Platform) {
    let po = PostOffice::new(platform);
    let sid = "ws-770015-1";
    let in_route = "ws.770015.1.in";

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

    // the model must import from the deployed classpath copy, and a leftover
    // store record would fake a resume (consume-on-retrieve)
    let temp = std::path::Path::new("/tmp/graph/unit-test-suspend-1.json");
    if temp.exists() {
        std::fs::remove_file(temp).expect("stale temp copy removed");
    }
    let cid = "dry-run-scope-1";
    let record = store_file("unit-test-suspend-1", cid);
    if record.exists() {
        std::fs::remove_file(&record).expect("stale store record removed");
    }
    let imported = sync_cmd(platform, sid, "import graph from unit-test-suspend-1").await;
    assert_eq!(
        imported["ok"],
        serde_json::json!(true),
        "import: {imported}"
    );

    // run 1: fresh transaction suspends at the checkpoint, persisted under graph + cid
    let instantiated = sync_cmd(
        platform,
        sid,
        &format!("instantiate graph\ntext({cid}) -> model.cid"),
    )
    .await;
    assert_eq!(
        instantiated["ok"],
        serde_json::json!(true),
        "instantiate: {instantiated}"
    );
    let first = sync_cmd(platform, sid, "run").await;
    assert_eq!(first["ok"], serde_json::json!(true), "run 1: {first}");
    assert_eq!(
        1,
        step_count("one", cid),
        "step-1 executes on the fresh run"
    );
    assert_eq!(
        0,
        step_count("two", cid),
        "the suspension stops before step-2"
    );
    // the KEY pin: the record must be scoped by the model's stable name, so a
    // later instantiation (or the production executor) can find it
    assert!(
        record.exists(),
        "the suspension must persist under the model's stable identity, \
         not an ephemeral per-instantiation handle"
    );

    // run 2: a NEW instantiation with the same business cid resumes past the checkpoint
    let again = sync_cmd(
        platform,
        sid,
        &format!("instantiate graph\ntext({cid}) -> model.cid"),
    )
    .await;
    assert_eq!(
        again["ok"],
        serde_json::json!(true),
        "re-instantiate: {again}"
    );
    let second = sync_cmd(platform, sid, "run").await;
    assert_eq!(second["ok"], serde_json::json!(true), "run 2: {second}");
    assert_eq!(
        1,
        step_count("one", cid),
        "the checkpoint must not re-execute"
    );
    assert_eq!(
        1,
        step_count("two", cid),
        "the continuation must run on resume"
    );
    assert!(
        !record.exists(),
        "the record is consumed on resume (at-most-once)"
    );
}

/// Java `CompanionSyncTest.unnamedDraftResumesAcrossInstantiations`: the nameless
/// twin - a draft sketched in the playground has no root `name` yet and must still
/// suspend and resume, scoped under the stable constant `untitled`. This is the
/// branch the regression lived in: the identity only has to be STABLE across
/// instantiations, so anything per-instantiation writes a suspension no later run
/// can read and silently restarts fresh instead of resuming.
async fn companion_unnamed_draft_resumes_across_instantiations(platform: &Platform) {
    let po = PostOffice::new(platform);
    let sid = "ws-770016-1";
    let in_route = "ws.770016.1.in";

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

    let cid = "dry-run-untitled-1";
    // consume-on-retrieve makes a leftover record indistinguishable from this test's own
    let record = store_file("untitled", cid);
    if record.exists() {
        std::fs::remove_file(&record).expect("stale store record removed");
    }
    // sketch a suspend/resume draft with NO name on the root - the edge-mode shape:
    // a drawn edge into the suspend node, plus the mandatory continuation edge
    for command in [
        "create node root\nwith type Root",
        "create node end\nwith type End",
        "create node resume\nwith type Resume\nwith properties\nskill=graph.resume\n\
         task=v1.file.state.store",
        "create node step-1\nwith type Suspensible\nwith properties\nskill=graph.task\n\
         task=v1.counting.step\ninput[]=text(u-one) -> step\ninput[]=model.cid -> cid\n\
         output[]=result.count -> model.step1_count",
        "create node suspend\nwith type Suspend\nwith properties\nskill=graph.suspend\n\
         task=v1.file.state.store\nttl=30s",
        "create node step-2\nwith type Task\nwith properties\nskill=graph.task\n\
         task=v1.counting.step\ninput[]=text(u-two) -> step\ninput[]=model.cid -> cid\n\
         input[]=model.step1_count -> prior\noutput[]=result -> output.body",
        "connect root to resume with test",
        "connect resume to step-1 with test",
        "connect step-1 to suspend with checkpoint",
        "connect step-1 to step-2 with approved",
        "connect suspend to end with test",
        "connect step-2 to end with test",
    ] {
        sync_cmd(platform, sid, command).await;
    }

    // run 1: the nameless draft suspends at the checkpoint
    let instantiated = sync_cmd(
        platform,
        sid,
        &format!("instantiate graph\ntext({cid}) -> model.cid"),
    )
    .await;
    assert_eq!(
        instantiated["ok"],
        serde_json::json!(true),
        "a nameless draft must instantiate: {instantiated}"
    );
    let first = sync_cmd(platform, sid, "run").await;
    assert_eq!(first["ok"], serde_json::json!(true), "run 1: {first}");
    assert_eq!(
        1,
        step_count("u-one", cid),
        "step-1 executes on the fresh run"
    );
    assert_eq!(
        0,
        step_count("u-two", cid),
        "the suspension stops before step-2"
    );
    assert!(
        record.exists(),
        "a nameless draft must persist under the stable 'untitled' scope, not a \
         per-instantiation handle"
    );

    // run 2: a NEW instantiation with the same business cid resumes past the checkpoint
    let again = sync_cmd(
        platform,
        sid,
        &format!("instantiate graph\ntext({cid}) -> model.cid"),
    )
    .await;
    assert_eq!(
        again["ok"],
        serde_json::json!(true),
        "re-instantiate: {again}"
    );
    let second = sync_cmd(platform, sid, "run").await;
    assert_eq!(second["ok"], serde_json::json!(true), "run 2: {second}");
    assert_eq!(
        1,
        step_count("u-one", cid),
        "the checkpoint must not re-execute"
    );
    assert_eq!(
        1,
        step_count("u-two", cid),
        "the continuation must run on resume"
    );
    assert!(
        !record.exists(),
        "the record is consumed on resume (at-most-once)"
    );
}

/// Find a mutable reference to a nested map's entry list by literal keys.
fn get_map_mut<'a>(value: &'a mut Value, path: &[&str]) -> Option<&'a mut Vec<(Value, Value)>> {
    let mut current = value;
    for key in path {
        let Value::Map(entries) = current else {
            return None;
        };
        current = &mut entries
            .iter_mut()
            .find(|(k, _)| k.as_str() == Some(*key))?
            .1;
    }
    match current {
        Value::Map(entries) => Some(entries),
        _ => None,
    }
}

/// Java parity (`GraphSuspendResumeTest.rejectedDeployedGraphIsNotExecutable`):
/// every suspend-err graph failed the CompileGraph quality gate, so a request
/// answers 404 as if the model does not exist - deployed execution is served
/// exclusively from the compiled registry (CompileFlows parity). Notably err6
/// (suspend node without an outgoing connection) would otherwise persist the
/// record and stall the run until the HTTP timeout; the runtime guards remain
/// the enforcement floor for the playground dry-run surface only.
async fn rejected_deployed_graph_is_not_executable(platform: &Platform) {
    for id in [
        "unit-test-suspend-err1",
        "unit-test-suspend-err2",
        "unit-test-suspend-err3",
        "unit-test-suspend-err4",
        "unit-test-suspend-err5",
        "unit-test-suspend-err6",
        "unit-test-suspend-err7",
        "unit-test-no-end",
    ] {
        let cid = uuid::Uuid::new_v4().simple().to_string();
        let response = run_graph_cid(platform, id, &cid, serde_json::json!({})).await;
        assert_eq!(
            404,
            response.status(),
            "{id} must be rejected as not found: {:?}",
            response.body()
        );
        let text = event_script::conversions::display(response.body());
        assert!(
            text.contains("not found"),
            "unexpected error response for {id}: {text}"
        );
    }
}

/// Java parity (`CompanionSyncTest.preRunCheckRejectsBrokenSuspendContract`):
/// the playground's `run` command reuses the deployment gate's whole-graph
/// rules just before dispatching the traveler: draft authoring allows partial
/// models, but a runnable graph must honor the suspend/resume contract - here
/// a suspend node without a ttl is rejected pre-run with the same message
/// CompileGraph would log at deployment time, and the uniform terminal line
/// keeps the sync drain deterministic.
async fn companion_sync_pre_run_check_rejects_broken_suspend_contract(platform: &Platform) {
    let po = PostOffice::new(platform);
    let sid = "ws-770011-3";
    let in_route = "ws.770011.3.in";

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

    // the root carries a name so the STABLE-IDENTITY instantiate guard passes and the
    // run reaches the pre-run check this test pins
    sync_cmd(
        platform,
        sid,
        "create node root\nwith type Root\nwith properties\nname=unit-test-prerun-check",
    )
    .await;
    sync_cmd(platform, sid, "create node end\nwith type End").await;
    sync_cmd(
        platform,
        sid,
        "create node suspend\nwith type Suspend\nwith properties\nskill=graph.suspend\ntask=v1.file.state.store",
    )
    .await;
    sync_cmd(platform, sid, "connect root to suspend with then").await;
    sync_cmd(platform, sid, "connect suspend to end with then").await;
    let instantiated = sync_cmd(platform, sid, "instantiate graph").await;
    assert_eq!(
        serde_json::json!(true),
        instantiated["ok"],
        "instantiate must succeed: {instantiated}"
    );
    let run = sync_cmd(platform, sid, "run").await;
    assert_eq!(
        serde_json::json!(false),
        run["ok"],
        "run must be rejected pre-run: {run}"
    );
    let output: Vec<String> = run["output"]
        .as_array()
        .expect("output list")
        .iter()
        .map(|v| v.as_str().unwrap_or_default().to_string())
        .collect();
    assert!(
        output
            .iter()
            .any(|l| l.contains("Unable to run - node suspend does not have a 'ttl'")),
        "the gate's rule message must reach the author: {output:?}"
    );
    assert!(
        output.iter().any(|l| l == "Graph traversal aborted"),
        "pre-run rejection must still emit the uniform terminal: {output:?}"
    );
}

/// Java parity (`GraphSuspendResumeTest` drives every scenario through the
/// real HTTP stack): the `model.run -> output.header.x-run` mapping must be
/// visible on the HTTP RESPONSE headers, not just the flow-reply envelope —
/// the engine-twin above deliberately drives `FlowExecutor` directly (the
/// file's convention), so this scenario crosses the HTTP boundary once.
async fn suspend_resume_x_run_over_the_real_http_stack(platform: &Platform) {
    let po = PostOffice::new(platform);
    let port = platform_core::automation::server_address()
        .expect("rest server started by lifecycle")
        .port();
    let target = format!("http://127.0.0.1:{port}");
    let cid = "wf-suspend-http-007";
    async fn http_run(po: &PostOffice, target: &str, cid: &str) -> EventEnvelope {
        let request = platform_core::automation::AsyncHttpRequest::new()
            .set_method("POST")
            .set_target_host(target)
            .set_url("/api/graph/unit-test-suspend-1")
            .set_header("content-type", "application/json")
            .set_header("accept", "application/json")
            .set_header("x-correlation-id", cid)
            .set_body(Value::Map(vec![]));
        po.request(
            EventEnvelope::new()
                .set_to("async.http.request")
                .set_raw_body(request.to_value()),
            Duration::from_secs(8),
        )
        .await
        .expect("http graph run reply")
    }
    let first = http_run(&po, &target, cid).await;
    assert_eq!(200, first.status(), "http run 1: {:?}", first.body());
    let second = http_run(&po, &target, cid).await;
    assert_eq!(200, second.status(), "http run 2: {:?}", second.body());
    assert_eq!(
        Some("resume"),
        second.header("x-run"),
        "the x-run header must reach the HTTP response (Java parity)"
    );
}

/// Java parity (`GraphSpanPropagationTest.suspendResumeStoreCallsChainToTheirSkillSpans`):
/// the pluggable store function's span chains onto the suspend/resume SKILL
/// span (the observable topology the ratified task-scoped-trace asymmetry is
/// conditioned on), and a completed resumed run has NO graph.suspend span
/// (no-re-execution is visible in trace topology).
async fn suspend_resume_store_calls_chain_to_their_skill_spans(platform: &Platform) {
    #[derive(Clone)]
    struct SpanCapture {
        records: Arc<Mutex<Vec<serde_json::Value>>>,
    }
    #[async_trait]
    impl ComposableFunction for SpanCapture {
        async fn handle_event(
            &self,
            _headers: HashMap<String, String>,
            input: EventEnvelope,
            _instance: usize,
        ) -> Result<EventEnvelope, AppError> {
            let record: serde_json::Value = input.body_as()?;
            self.records.lock().expect("spans").push(record);
            EventEnvelope::new().set_body("ok")
        }
    }
    let records = Arc::new(Mutex::new(Vec::<serde_json::Value>::new()));
    platform
        .register(
            "distributed.trace.forwarder",
            Arc::new(SpanCapture {
                records: records.clone(),
            }),
            1,
        )
        .expect("register span capture");

    #[derive(Debug, Clone)]
    struct Span {
        service: String,
        span_id: String,
        parent: String,
        from: String,
    }
    async fn run_and_collect(
        platform: &Platform,
        records: &Arc<Mutex<Vec<serde_json::Value>>>,
        cid: &str,
    ) -> Vec<Span> {
        let trace_id = trace::new_trace_id();
        let dataset = serde_json::json!({
            "body": {},
            "header": {},
            "path_parameter": {"graph_id": "unit-test-suspend-1"},
            "method": "POST",
        });
        let reply = FlowExecutor::request(
            platform,
            "graph-executor",
            event_script::conversions::from_json(&dataset),
            cid,
            Duration::from_secs(8),
            Some((&trace_id, "TEST /graph/span")),
        )
        .await
        .expect("span run reply");
        assert_eq!(200, reply.status(), "span run: {:?}", reply.body());
        // telemetry is emitted asynchronously after the reply — poll until
        // this trace's store span arrived (bounded)
        for _ in 0..50 {
            let spans: Vec<Span> = records
                .lock()
                .expect("spans")
                .iter()
                .filter_map(|r| {
                    let t = r.get("trace")?;
                    if t.get("id")?.as_str()? != trace_id {
                        return None;
                    }
                    Some(Span {
                        service: t.get("service")?.as_str()?.to_string(),
                        span_id: t.get("span_id")?.as_str().unwrap_or_default().to_string(),
                        parent: t
                            .get("parent_span_id")
                            .and_then(|p| p.as_str())
                            .unwrap_or_default()
                            .to_string(),
                        from: t
                            .get("from")
                            .and_then(|f| f.as_str())
                            .unwrap_or_default()
                            .to_string(),
                    })
                })
                .collect();
            if spans.iter().any(|s| s.service == "v1.file.state.store")
                && spans.iter().any(|s| s.service == "task.executor")
            {
                return spans;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        panic!("span records for trace {trace_id} did not arrive");
    }
    fn find<'a>(spans: &'a [Span], service: &str, from: &str) -> &'a Span {
        spans
            .iter()
            .find(|s| s.service == service && s.from == from)
            .unwrap_or_else(|| panic!("span {service} from {from} not found: {spans:?}"))
    }

    let cid = "wf-suspend-span-008";
    // run 1: fresh transaction - retrieve (miss), then suspend + persist
    let run1 = run_and_collect(platform, &records, cid).await;
    let resume1 = find(&run1, "graph.resume", "graph.executor");
    let suspend = find(&run1, "graph.suspend", "graph.executor");
    let retrieve1 = find(&run1, "v1.file.state.store", "graph.resume");
    let persist = find(&run1, "v1.file.state.store", "graph.suspend");
    assert_eq!(
        resume1.span_id, retrieve1.parent,
        "the retrieve call must chain to graph.resume"
    );
    assert_eq!(
        suspend.span_id, persist.parent,
        "the persist call must chain to graph.suspend"
    );
    // run 2: restore and continue - the retrieve chains to the new resume
    // span and the suspension point is not re-executed, so no suspend span
    let run2 = run_and_collect(platform, &records, cid).await;
    let resume2 = find(&run2, "graph.resume", "graph.executor");
    let retrieve2 = find(&run2, "v1.file.state.store", "graph.resume");
    assert_eq!(
        resume2.span_id, retrieve2.parent,
        "the restore call must chain to graph.resume"
    );
    assert!(
        run2.iter().all(|s| s.service != "graph.suspend"),
        "a resumed run that completes must not suspend again: {run2:?}"
    );
}

/// Java parity (`CompanionSyncTest`, commit f2107126): the `instantiate
/// graph` command is the dry-run's edge — like the REST edge, it guarantees
/// a business correlation ID, auto-created with a reminder when the initial
/// data mapping did not supply one, and honored silently when it did.
async fn companion_sync_instantiate_creates_model_cid(platform: &Platform) {
    let po = PostOffice::new(platform);
    let sid = "ws-770012-4";
    let in_route = "ws.770012.4.in";
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
    fn output_lines(reply: &serde_json::Value) -> Vec<String> {
        reply["output"]
            .as_array()
            .map(|a| {
                a.iter()
                    .map(|v| v.as_str().unwrap_or_default().to_string())
                    .collect()
            })
            .unwrap_or_default()
    }

    sync_cmd(platform, sid, "create node root\nwith type Root").await;
    sync_cmd(platform, sid, "create node end\nwith type End").await;
    sync_cmd(
        platform,
        sid,
        "create node mapper\nwith type mapper\nwith properties\nskill=graph.data.mapper\nmapping[]=input.body.id -> output.body",
    )
    .await;
    sync_cmd(platform, sid, "connect root to mapper with first").await;
    sync_cmd(platform, sid, "connect mapper to end with second").await;
    // no model.cid mapping: the edge auto-creates one with a reminder
    let instantiated = sync_cmd(
        platform,
        sid,
        "instantiate graph\ntext(hello world) -> input.body.id",
    )
    .await;
    assert_eq!(
        serde_json::json!(true),
        instantiated["ok"],
        "instantiate -> ok:true: {instantiated}"
    );
    let init_output = output_lines(&instantiated);
    assert!(
        init_output.iter().any(|l| l
            .starts_with("No business correlation ID given - this dry-run created model.cid = ")),
        "the dry-run edge must auto-create model.cid with a reminder: {init_output:?}"
    );
    // an explicitly mapped model.cid is honored without the reminder
    let with_cid = sync_cmd(
        platform,
        sid,
        "instantiate graph\ntext(hello world) -> input.body.id\ntext(dry-run-77) -> model.cid",
    )
    .await;
    assert_eq!(
        serde_json::json!(true),
        with_cid["ok"],
        "instantiate with cid -> ok:true: {with_cid}"
    );
    let with_cid_output = output_lines(&with_cid);
    assert!(
        with_cid_output
            .iter()
            .all(|l| !l.contains("created model.cid")),
        "a supplied model.cid must be honored without the reminder: {with_cid_output:?}"
    );
}
