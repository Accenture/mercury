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

//! Progress events from a graph run (RFC-0011 section 8.4; the twin of the Java
//! `ProgressEventTest`, mercury-composable #611), end to end: the engine's `node.start` for a
//! node with `progress`, a function's own events through its node's relay, and one terminal
//! event - to a declared `x-return-path`, as server-sent events on a REST request with
//! `stream: true`, and as text frames on the WebSocket entry `/ws/graph-run/{graph_id}`.
//!
//! Its own test binary: one platform for the fourteen cases, compiling only its own manifest
//! (`progress-graphs.yaml`) and serving its own REST automation (`progress-rest.yaml`).
//! `v1.progress.sender` reports progress the way `llm.run` does, and
//! `test.progress.collector` is a caller's declared return route.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use event_script::FlowExecutor;
use futures_util::{SinkExt, StreamExt};
use platform_core::{
    main_application, overrides, preload, trace, AppError, AutoStart, ComposableFunction,
    EntryPoint, EventEnvelope, Platform, PostOffice,
};
use serde_json::{json, Value as Json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::tungstenite::Message;

const TIMEOUT: Duration = Duration::from_secs(8);
const PROGRESS: &str = "progress";
const RESULT: &str = "result";
const GRAPH_1: &str = "unit-test-progress-1";
const COLLECTOR: &str = "test.progress.collector";
const UNDECLARED: &str = "test.undeclared.collector";
const NONE: &str = "none";

// ---- the caller's return route (Java ProgressCollector) ----

/// The events each return route received, in order, per business correlation id.
fn collected() -> &'static Mutex<HashMap<String, Vec<Json>>> {
    static EVENTS: OnceLock<Mutex<HashMap<String, Vec<Json>>>> = OnceLock::new();
    EVENTS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn undeclared() -> &'static Mutex<Vec<Json>> {
    static EVENTS: OnceLock<Mutex<Vec<Json>>> = OnceLock::new();
    EVENTS.get_or_init(|| Mutex::new(Vec::new()))
}

fn events(cid: &str) -> Vec<Json> {
    collected()
        .lock()
        .expect("collector")
        .get(cid)
        .cloned()
        .unwrap_or_default()
}

/// Wait for a run's terminal event: the events received for the run, or what arrived before the
/// wait expired.
async fn await_terminal(cid: &str, timeout: Duration) -> Vec<Json> {
    let deadline = Instant::now() + timeout;
    loop {
        let received = events(cid);
        if received.iter().any(|e| e["type"] != PROGRESS) || Instant::now() >= deadline {
            return received;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

fn cid_of(event: &Json) -> String {
    match &event["cid"] {
        Json::String(cid) => cid.clone(),
        other => other.to_string(),
    }
}

/// A caller's return route (declared by `return.path.routes=test.progress.*`): it records each
/// event it receives, in order, per business correlation id. One instance, so the record keeps
/// the arrival order.
#[preload(route = "test.progress.collector")]
struct ProgressCollector;

#[async_trait]
impl ComposableFunction for ProgressCollector {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let event: Json = input.body_as()?;
        collected()
            .lock()
            .expect("collector")
            .entry(cid_of(&event))
            .or_default()
            .push(event);
        Ok(EventEnvelope::new())
    }
}

/// A route the application does not declare as a return path.
struct UndeclaredCollector;

#[async_trait]
impl ComposableFunction for UndeclaredCollector {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let event: Json = input.body_as()?;
        undeclared().lock().expect("undeclared").push(event);
        Ok(EventEnvelope::new())
    }
}

// ---- the function that reports progress (Java ProgressSender) ----

fn return_paths() -> &'static Mutex<HashMap<String, String>> {
    static PATHS: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
    PATHS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn return_path_seen(cid: &str) -> Option<String> {
    return_paths()
        .lock()
        .expect("return paths")
        .get(cid)
        .cloned()
}

/// A function that reports progress the way `llm.run` does: when its caller passes
/// `x-return-path`, it sends two progress events there, untraced, and names neither its graph nor
/// its node. It records the return path it was given, per business correlation id, so a test can
/// see that a node without `progress` gets none.
#[preload(route = "v1.progress.sender", instances = 10)]
struct ProgressSender;

#[async_trait]
impl ComposableFunction for ProgressSender {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let cid = headers
            .get("my_correlation_id")
            .cloned()
            .unwrap_or_else(|| "null".to_string());
        let return_path = headers.get("x-return-path").cloned();
        return_paths()
            .lock()
            .expect("return paths")
            .insert(cid, return_path.clone().unwrap_or_else(|| NONE.to_string()));
        let body: Json = input.body_as().unwrap_or(Json::Null);
        let mode = body["mode"].as_str().unwrap_or_default().to_string();
        let mut sent = 0;
        if let Some(return_path) = &return_path {
            // 'burst' sends many events and returns at once, so the run ends while the relay
            // still holds some
            let count = if mode == "burst" { 5000 } else { 2 };
            let platform = Platform::get_instance();
            let po = PostOffice::new(&platform);
            for i in 1..=count {
                let event = json!({
                    "type": "progress",
                    "event": "step",
                    "seq": i,
                    "time": "2026-11-04T00:00:00.000Z",
                    "message": format!("step {i}"),
                });
                po.send_untraced(EventEnvelope::new().set_to(return_path).set_body(event)?)
                    .await?;
                sent += 1;
            }
        }
        if mode == "fail" {
            return Err(AppError::new(422, "refused by the test"));
        }
        let item = match &body["item"] {
            Json::String(item) => item.clone(),
            other => other.to_string(),
        };
        let result = json!({"item": item, "sent": sent});
        // 'slow' outlives a short flow ttl, so the run ends on the flow's timeout after its
        // progress was sent
        if mode == "slow" {
            tokio::time::sleep(Duration::from_millis(4000)).await;
        }
        EventEnvelope::new().set_body(result)
    }
}

#[main_application]
struct ProgressEventTestApp;

#[async_trait]
impl EntryPoint for ProgressEventTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

/// Boot one platform for the binary and return the REST server's port.
async fn boot() -> (Platform, u16) {
    platform_core::resources::prepend_resource_root("tests/resources");
    overrides::set("graph.model.automation", "classpath:/progress-graphs.yaml");
    overrides::set("yaml.rest.automation", "classpath:/progress-rest.yaml");
    // the routes a caller may name as its x-return-path: a prefix, and one route
    overrides::set(
        "return.path.routes",
        "test.progress.*, test.collector.exact",
    );
    // the WebSocket entry for a watched graph run, off by default
    overrides::set("graph.websocket.enabled", "true");
    overrides::set("graph.websocket.ttl", "20s");
    // the test configuration's port belongs to the graph_runtime binary
    overrides::set("rest.server.port", "0");
    AutoStart::main(vec![]).await.expect("lifecycle");
    // the manifest compiled (and the reference links the engine's functions into this binary)
    for id in [
        GRAPH_1,
        "unit-test-progress-2",
        "unit-test-progress-3",
        "unit-test-progress-4",
    ] {
        assert!(
            knowledge_graph::graphs::graph_exists(id),
            "{id} must compile"
        );
    }
    for id in ["unit-test-progress-err1", "unit-test-progress-err2"] {
        assert!(
            !knowledge_graph::graphs::graph_exists(id),
            "{id} must fail the gate"
        );
    }
    let platform = Platform::get_instance();
    platform
        .register_private(UNDECLARED, Arc::new(UndeclaredCollector), 1)
        .expect("undeclared collector");
    let port = platform_core::automation::server_address()
        .expect("rest server started by lifecycle")
        .port();
    (platform, port)
}

/// Run a graph from an event (the Java `runFromEvent`): the graph-executor flow, the caller's
/// return route in the run's `x-return-path` header, under a business correlation id.
async fn run_from_event(
    platform: &Platform,
    graph_id: &str,
    cid: &str,
    return_path: &str,
    body: Json,
    ttl_ms: u64,
) -> (i32, Json) {
    let dataset = json!({
        "body": body,
        "header": {"x-return-path": return_path},
        "path_parameter": {"graph_id": graph_id},
        "ttl": ttl_ms,
    });
    match FlowExecutor::request(
        platform,
        "graph-executor",
        event_script::conversions::from_json(&dataset),
        cid,
        TIMEOUT,
        Some((&trace::new_trace_id(), "TEST /progress")),
    )
    .await
    {
        Ok(reply) => (reply.status(), reply.body_as().unwrap_or(Json::Null)),
        Err(e) => (e.status(), json!(e.message())),
    }
}

fn new_cid(name: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("progress-{name}-{n}")
}

/// A node's progress events.
fn from(events: &[Json], node: &str) -> Vec<Json> {
    events
        .iter()
        .filter(|e| e["node"] == node && e["type"] == PROGRESS)
        .cloned()
        .collect()
}

fn types(events: &[Json]) -> Vec<String> {
    events
        .iter()
        .map(|e| e["type"].as_str().unwrap_or_default().to_string())
        .collect()
}

// ---- HTTP and WebSocket helpers ----

/// Decode an HTTP/1.1 chunked body.
fn decode_chunked(raw: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0;
    while let Some(nl) = raw[i..].windows(2).position(|w| w == b"\r\n") {
        let size_text = String::from_utf8_lossy(&raw[i..i + nl]);
        let size_text = size_text.split(';').next().unwrap_or("").trim().to_string();
        let Ok(size) = usize::from_str_radix(&size_text, 16) else {
            break;
        };
        if size == 0 {
            break;
        }
        let start = i + nl + 2;
        if raw.len() < start + size + 2 {
            break;
        }
        out.extend_from_slice(&raw[start..start + size]);
        i = start + size + 2;
    }
    out
}

/// POST a JSON body and read the whole response (`Connection: close`): the status, the
/// lowercased head and the decoded body.
async fn post(port: u16, path: &str, body: &Json, extra: &[(&str, &str)]) -> (u16, String, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let payload = body.to_string();
    let mut request = format!(
        "POST {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\
         Content-Type: application/json\r\nContent-Length: {}\r\n",
        payload.len()
    );
    for (name, value) in extra {
        request.push_str(&format!("{name}: {value}\r\n"));
    }
    request.push_str("\r\n");
    request.push_str(&payload);
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut raw = Vec::new();
    tokio::time::timeout(Duration::from_secs(20), stream.read_to_end(&mut raw))
        .await
        .expect("response within the timeout")
        .expect("read");
    let pos = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .expect("a response head");
    let head = String::from_utf8_lossy(&raw[..pos]).to_lowercase();
    let body_raw = &raw[pos + 4..];
    let decoded = if head.contains("transfer-encoding: chunked") {
        decode_chunked(body_raw)
    } else {
        body_raw.to_vec()
    };
    let status = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap_or(0);
    (status, head, String::from_utf8_lossy(&decoded).to_string())
}

/// POST to a `stream: true` endpoint: the status, the head and the non-blank lines.
async fn stream(port: u16, path: &str, body: &Json) -> (u16, String, Vec<String>) {
    let (status, head, text) = post(port, path, body, &[("Accept", "text/event-stream")]).await;
    let lines = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect();
    (status, head, lines)
}

fn event_names(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|l| l.strip_prefix("event:"))
        .map(|name| name.trim().to_string())
        .collect()
}

fn header_value(head: &str, name: &str) -> String {
    head.lines()
        .find_map(|line| line.strip_prefix(&format!("{name}:")))
        .map(|value| value.trim().to_string())
        .unwrap_or_default()
}

fn json(text: &str) -> Json {
    serde_json::from_str(text).unwrap_or_else(|e| panic!("not JSON ({e}): {text}"))
}

/// Open a session on the WebSocket entry, send one text frame, and collect the session's text
/// frames until it closes; the bool says whether it closed within the timeout.
async fn session(port: u16, graph_id: &str, frame: &str) -> (Vec<String>, bool) {
    let url = format!("ws://127.0.0.1:{port}/ws/graph-run/{graph_id}");
    let (mut ws, _) = tokio::time::timeout(TIMEOUT, tokio_tungstenite::connect_async(url))
        .await
        .expect("connect within the timeout")
        .expect("websocket connects");
    ws.send(Message::Text(frame.to_string()))
        .await
        .expect("frame sent");
    let mut frames = Vec::new();
    let deadline = Instant::now() + TIMEOUT;
    loop {
        let left = deadline.saturating_duration_since(Instant::now());
        match tokio::time::timeout(left, ws.next()).await {
            Ok(Some(Ok(Message::Text(text)))) => frames.push(text),
            Ok(Some(Ok(Message::Close(_)))) => {
                // the client answers the close: read on until the connection ends, so the
                // closing handshake completes
                while let Ok(Some(Ok(_))) =
                    tokio::time::timeout(Duration::from_secs(1), ws.next()).await
                {}
                return (frames, true);
            }
            Ok(None) => return (frames, true),
            Ok(Some(Ok(_))) => {}
            Ok(Some(Err(_))) => return (frames, true),
            Err(_) => return (frames, false),
        }
    }
}

// ---- the cases ----

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_watched_graph_run_sends_progress_then_one_terminal_event() {
    test_support::run_at_exit(platform_core::util::elastic_queue::shutdown_cleanup);
    let (platform, port) = boot().await;
    an_event_entry_gets_progress_then_the_result(&platform).await;
    a_synchronous_subgraph_reports_through_its_parent(&platform).await;
    every_event_a_sender_sent_arrives_before_the_terminal_event(&platform).await;
    a_failed_run_ends_with_an_error_event(&platform).await;
    a_run_that_times_out_ends_with_an_error_event(&platform).await;
    a_stream_that_times_out_ends_with_an_error_event(port).await;
    a_node_without_progress_sends_nothing_and_passes_no_return_path(&platform).await;
    an_undeclared_return_path_is_ignored(&platform).await;
    a_plain_rest_request_ignores_an_inbound_return_path_header(port).await;
    a_streaming_rest_request_gets_server_sent_events(port).await;
    a_streaming_request_that_sends_no_progress_answers_single_shot(port).await;
    a_websocket_session_gets_progress_then_the_result_and_closes(port).await;
    a_websocket_session_for_a_graph_that_is_not_deployed_ends_with_an_error(port).await;
    a_websocket_frame_that_is_not_an_object_starts_nothing(port).await;
}

async fn an_event_entry_gets_progress_then_the_result(platform: &Platform) {
    let cid = new_cid("event");
    let (status, body) = run_from_event(
        platform,
        GRAPH_1,
        &cid,
        COLLECTOR,
        json!({"item": "x"}),
        20000,
    )
    .await;
    assert_eq!(200, status, "{body}");
    let events = await_terminal(&cid, TIMEOUT).await;
    assert_eq!(5, events.len(), "{events:?}");
    // the engine's events: node.start for each node with 'progress', numbered by the engine
    let root = &from(&events, "root")[0];
    assert_eq!("node.start", root["event"]);
    assert_eq!("Started", root["message"]);
    assert_eq!(1, root["seq"]);
    assert_eq!(GRAPH_1, root["graph"]);
    assert_eq!(cid.as_str(), root["cid"]);
    let time = root["time"].as_str().unwrap_or_default();
    let pattern = regex::Regex::new(r"^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d\.\d{3}Z$").expect("regex");
    assert!(pattern.is_match(time), "{time}");
    let work = from(&events, "work");
    assert_eq!(3, work.len(), "{work:?}");
    assert_eq!("node.start", work[0]["event"]);
    assert_eq!("Working on it", work[0]["message"]);
    assert_eq!(2, work[0]["seq"]);
    // the function's events, through the node's relay: its own numbering, its graph and node
    // named for it
    let own = &work[1..3];
    assert_eq!(
        vec!["step", "step"],
        own.iter()
            .map(|e| e["event"].as_str().unwrap_or_default())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![json!(1), json!(2)],
        own.iter().map(|e| e["seq"].clone()).collect::<Vec<_>>()
    );
    for event in own {
        assert_eq!(GRAPH_1, event["graph"]);
        assert_eq!(cid.as_str(), event["cid"]);
    }
    assert!(
        return_path_seen(&cid)
            .unwrap_or_default()
            .starts_with("graph.progress."),
        "the function gets its node's relay, never the caller's route"
    );
    // the terminal event is last: the relay is drained before it is sent
    let terminal = events.last().expect("terminal");
    assert_eq!(RESULT, terminal["type"]);
    assert_eq!(3, terminal["seq"]);
    assert_eq!(200, terminal["status"]);
    assert_eq!("x", terminal["body"]["item"]);
}

async fn a_synchronous_subgraph_reports_through_its_parent(platform: &Platform) {
    let cid = new_cid("subgraph");
    let (status, body) = run_from_event(
        platform,
        "unit-test-progress-4",
        &cid,
        COLLECTOR,
        json!({"item": "p"}),
        20000,
    )
    .await;
    assert_eq!(200, status, "{body}");
    let events = await_terminal(&cid, TIMEOUT).await;
    let parent: Vec<Json> = events
        .iter()
        .filter(|e| e["graph"] == "unit-test-progress-4")
        .cloned()
        .collect();
    let child: Vec<Json> = events
        .iter()
        .filter(|e| e["graph"] == GRAPH_1)
        .cloned()
        .collect();
    assert_eq!(
        vec![PROGRESS, RESULT],
        types(&parent),
        "the parent's node.start, then its terminal event: {events:?}"
    );
    assert_eq!("Delegating", parent[0]["message"]);
    assert_eq!(
        RESULT,
        parent[parent.len() - 1]["type"],
        "the subgraph's own result is not the run's end"
    );
    assert_eq!(
        vec!["root", "work", "work", "work"],
        child
            .iter()
            .map(|e| e["node"].as_str().unwrap_or_default())
            .collect::<Vec<_>>(),
        "the subgraph names itself: {child:?}"
    );
    assert!(child.iter().all(|e| e["type"] == PROGRESS));
    assert_eq!(6, events.len(), "{events:?}");
    assert_eq!(RESULT, events[events.len() - 1]["type"]);
}

async fn every_event_a_sender_sent_arrives_before_the_terminal_event(platform: &Platform) {
    let cid = new_cid("burst");
    let (status, body) = run_from_event(
        platform,
        GRAPH_1,
        &cid,
        COLLECTOR,
        json!({"item": "b", "mode": "burst"}),
        20000,
    )
    .await;
    assert_eq!(200, status, "{body}");
    let events = await_terminal(&cid, TIMEOUT).await;
    let own = events.iter().filter(|e| e["event"] == "step").count();
    assert_eq!(5000, own, "the relay is drained before the terminal event");
    assert_eq!(RESULT, events[events.len() - 1]["type"]);
}

async fn a_failed_run_ends_with_an_error_event(platform: &Platform) {
    let cid = new_cid("failed");
    let (status, body) = run_from_event(
        platform,
        "unit-test-progress-3",
        &cid,
        COLLECTOR,
        json!({"item": "y"}),
        20000,
    )
    .await;
    assert_eq!(422, status, "{body}");
    let events = await_terminal(&cid, TIMEOUT).await;
    let terminal = events.last().expect("terminal");
    assert_eq!("error", terminal["type"], "{events:?}");
    assert_eq!(422, terminal["status"]);
    assert_eq!("refused by the test", terminal["message"]);
}

async fn a_run_that_times_out_ends_with_an_error_event(platform: &Platform) {
    let cid = new_cid("timeout");
    let (status, body) = run_from_event(
        platform,
        GRAPH_1,
        &cid,
        COLLECTOR,
        json!({"item": "w", "mode": "slow"}),
        1500,
    )
    .await;
    assert_eq!(408, status, "{body}");
    let events = await_terminal(&cid, TIMEOUT).await;
    assert_eq!(
        4,
        from(&events, "root").len() + from(&events, "work").len(),
        "the progress sent before: {events:?}"
    );
    let terminal = events.last().expect("terminal");
    assert_eq!(
        "error", terminal["type"],
        "the flow's end ends the channel: {events:?}"
    );
    assert_eq!(408, terminal["status"]);
}

async fn a_stream_that_times_out_ends_with_an_error_event(port: u16) {
    let (status, _, lines) = stream(
        port,
        &format!("/api/stream/short/{GRAPH_1}"),
        &json!({"item": "v", "mode": "slow"}),
    )
    .await;
    assert_eq!(
        200, status,
        "the head was committed by the first progress event"
    );
    // the flow's own single-shot 408 cannot write to the started stream; the flow's end ends it
    // in-band
    assert_eq!(
        vec![PROGRESS, PROGRESS, PROGRESS, PROGRESS, "error", "done"],
        event_names(&lines),
        "{lines:?}"
    );
    let error = lines
        .iter()
        .find(|l| l.starts_with("data:") && l.contains("\"type\":\"error\""))
        .unwrap_or_else(|| panic!("an error event: {lines:?}"));
    assert_eq!(408, json(error["data:".len()..].trim())["status"]);
}

async fn a_node_without_progress_sends_nothing_and_passes_no_return_path(platform: &Platform) {
    let cid = new_cid("no-progress");
    let (status, body) = run_from_event(
        platform,
        "unit-test-progress-2",
        &cid,
        COLLECTOR,
        json!({"item": "z"}),
        20000,
    )
    .await;
    assert_eq!(200, status, "{body}");
    let events = await_terminal(&cid, TIMEOUT).await;
    assert_eq!(vec![RESULT], types(&events), "only the terminal event");
    assert_eq!(
        Some(NONE.to_string()),
        return_path_seen(&cid),
        "a node without 'progress' never sees the header"
    );
}

async fn an_undeclared_return_path_is_ignored(platform: &Platform) {
    let cid = new_cid("undeclared");
    let (status, body) = run_from_event(
        platform,
        GRAPH_1,
        &cid,
        UNDECLARED,
        json!({"item": "u"}),
        20000,
    )
    .await;
    assert_eq!(200, status, "the run continues without progress: {body}");
    assert_eq!(Some(NONE.to_string()), return_path_seen(&cid));
    assert!(undeclared()
        .lock()
        .expect("undeclared")
        .iter()
        .all(|e| cid_of(e) != cid));
}

async fn a_plain_rest_request_ignores_an_inbound_return_path_header(port: u16) {
    let cid = new_cid("rest");
    let (status, _, body) = post(
        port,
        &format!("/api/graph/{GRAPH_1}"),
        &json!({"item": "r"}),
        &[("X-Correlation-Id", &cid), ("X-Return-Path", COLLECTOR)],
    )
    .await;
    assert_eq!(200, status, "{body}");
    assert_eq!(
        Some(NONE.to_string()),
        return_path_seen(&cid),
        "the REST automation sets a REST entry's return route"
    );
    assert!(events(&cid).is_empty());
}

async fn a_streaming_rest_request_gets_server_sent_events(port: u16) {
    let (status, head, lines) = stream(
        port,
        &format!("/api/stream/graph/{GRAPH_1}"),
        &json!({"item": "s"}),
    )
    .await;
    assert_eq!(200, status);
    assert_eq!("text/event-stream", header_value(&head, "content-type"));
    assert_eq!(
        vec![PROGRESS, PROGRESS, PROGRESS, PROGRESS, RESULT, "done"],
        event_names(&lines),
        "{lines:?}"
    );
    let data: Vec<Json> = lines
        .iter()
        .filter(|l| l.starts_with("data:") && l.contains("\"type\""))
        .map(|l| json(l["data:".len()..].trim()))
        .collect();
    assert_eq!("Started", data[0]["message"]);
    assert_eq!("s", data[4]["body"]["item"]);
}

async fn a_streaming_request_that_sends_no_progress_answers_single_shot(port: u16) {
    let (status, head, body) = post(
        port,
        "/api/stream/graph/unit-test-progress-2",
        &json!({"item": "t"}),
        &[("Accept", "text/event-stream")],
    )
    .await;
    assert_eq!(200, status, "{body}");
    assert_ne!(
        "text/event-stream",
        header_value(&head, "content-type"),
        "a run that sends no progress never opens the stream"
    );
    assert!(!body.contains("event:"), "{body}");
    assert_eq!(
        "t",
        json(&body)["item"],
        "the single-shot response carries the result"
    );
}

async fn a_websocket_session_gets_progress_then_the_result_and_closes(port: u16) {
    let cid = new_cid("socket");
    let (frames, closed) = session(
        port,
        GRAPH_1,
        &format!(r#"{{"header": {{"X-Correlation-Id": "{cid}"}}, "body": {{"item": "q"}}}}"#),
    )
    .await;
    assert!(closed, "the session closes after the result");
    let frames: Vec<Json> = frames.iter().map(|f| json(f)).collect();
    assert_eq!(
        vec![PROGRESS, PROGRESS, PROGRESS, PROGRESS, RESULT],
        types(&frames),
        "{frames:?}"
    );
    for frame in &frames {
        assert_eq!(
            cid.as_str(),
            frame["cid"],
            "the session's X-Correlation-Id is the run's cid"
        );
    }
    assert_eq!("q", frames[frames.len() - 1]["body"]["item"]);
}

async fn a_websocket_session_for_a_graph_that_is_not_deployed_ends_with_an_error(port: u16) {
    let (frames, closed) = session(port, "no-such-graph", r#"{"body": {}}"#).await;
    assert!(closed);
    let terminal = json(frames.last().expect("a frame"));
    assert_eq!("error", terminal["type"]);
    assert_eq!(404, terminal["status"]);
}

async fn a_websocket_frame_that_is_not_an_object_starts_nothing(port: u16) {
    let (frames, closed) = session(port, GRAPH_1, "[1, 2]").await;
    assert!(closed);
    let refusal = json(frames.last().expect("a frame"));
    assert_eq!(400, refusal["status"]);
    assert_eq!(
        "The run's input is a JSON object with an optional body and header",
        refusal["message"]
    );
}
