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

//! The agent demo graphs of the Playground, end to end and token-free - the Rust twin of the Java
//! example's `OrderAssistantTest`, `DiagramAssistantTest` and `AgentToolsGraphsTest`. The model
//! turns are scripted (`tests/common/agent_doubles.rs`), the tool routes and the Mermaid binding's
//! route resolve to stand-ins with the real tools' shapes and refusals, and the Redis of the
//! suspended runs is the RESP2 double of `tests/common/redis_double.rs`; the loop (`llm.run`), the
//! catalogs, the graphs' decisions, the suspension of a supervised write and the example's own
//! functions (the order book, `diagram.render`) are the real ones.
//!
//! - `order-assistant`: two reads, one write that waits for a supervisor through suspend and
//!   resume; a resume without decisions re-advises, a bad decision is refused by name and the
//!   transaction kept, a refusal reaches the model as the call's error, a second refund is the
//!   order book's 409; a watched run streams the loop's events before the result;
//! - `diagram-assistant`: the one tool reaches `diagram.render`, which calls the binding's route
//!   and stores the image out of band, so the model reports a reference;
//! - `site-notes`: a crawl of a documentation section, then a write of the notes that waits for a
//!   person under the node's default mode and runs when approved;
//! - `document-ingest`: a document of the synthetic corpus read, its facts extracted and merged
//!   into the knowledge graph by an ingest node that declares `mode: auto`, so no approval is asked;
//! - `document-query`: the schema read, one read-only query, an answer that cites the sources.
//!
//! One test function on purpose (the repo convention): the app boots ONCE per process, so the
//! scenarios run sequentially against the same server.
use std::path::PathBuf;
use std::time::Duration;

use async_trait::async_trait;
use platform_core::automation::AsyncHttpRequest;
use platform_core::{
    automation, main_application, overrides, AppError, AutoStart, EntryPoint, EventEnvelope,
    Platform, PostOffice,
};
use rmpv::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[path = "common/agent_doubles.rs"]
mod agent_doubles;
#[path = "common/redis_double.rs"]
mod redis_double;

use agent_doubles::{at, field, map, text_of, SECTION};

const TIMEOUT: Duration = Duration::from_secs(15);
const ORDER_ASSISTANT: &str = "order-assistant";
const DIAGRAM_ASSISTANT: &str = "diagram-assistant";
const SITE_NOTES: &str = "site-notes";
const DOCUMENT_INGEST: &str = "document-ingest";
const DOCUMENT_QUERY: &str = "document-query";
const ANSWER: &str = "answer";
const APPROVAL: &str = "approval";

/// The test's own entry point; naming the library's and the extensions' routes keeps their
/// annotation inventories (the demo tools, the agent loop, the Redis state store) linked into
/// this test binary.
#[main_application]
struct TestApp;

#[async_trait]
impl EntryPoint for TestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!(
            "under test: {} with {}, {}, {}, {}; state store {} / {}",
            minigraph_playground::AGENT_LOOP_ROUTE,
            minigraph_playground::order_book::ORDER_LOOKUP_ROUTE,
            minigraph_playground::order_book::SHIPMENT_STATUS_ROUTE,
            minigraph_playground::order_book::REFUND_ISSUE_ROUTE,
            minigraph_playground::diagram::DIAGRAM_RENDER_ROUTE,
            minigraph_state_redis::PERSIST_ROUTE,
            minigraph_state_redis::RETRIEVE_ROUTE
        );
        Ok(())
    }
}

struct Harness {
    po: PostOffice,
    target: String,
    port: u16,
    store: PathBuf,
}

impl Harness {
    /// `POST /api/graph/{graph}` through the HTTP edge, with the business correlation id.
    async fn run_graph(&self, graph: &str, cid: &str, body: Value) -> EventEnvelope {
        let request = AsyncHttpRequest::new()
            .set_method("POST")
            .set_target_host(&self.target)
            .set_url(&format!("/api/graph/{graph}"))
            .set_header("content-type", "application/json")
            .set_header("accept", "application/json")
            .set_header("x-correlation-id", cid)
            .set_body(body);
        self.po
            .request(
                EventEnvelope::new()
                    .set_to("async.http.request")
                    .set_raw_body(request.to_value()),
                TIMEOUT,
            )
            .await
            .expect("graph run reply")
    }

    /// `POST /api/stream/graph/{graph}` with `accept: text/event-stream`, read to the end: the
    /// status and the non-blank lines of the decoded body.
    async fn stream(&self, graph: &str, cid: &str, payload: &str) -> (u16, Vec<String>) {
        let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", self.port))
            .await
            .expect("connect");
        let request = format!(
            "POST /api/stream/graph/{graph} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\
             Content-Type: application/json\r\nAccept: text/event-stream\r\nX-Correlation-Id: {cid}\r\n\
             Content-Length: {}\r\n\r\n{payload}",
            payload.len()
        );
        stream.write_all(request.as_bytes()).await.expect("write");
        let mut raw = Vec::new();
        tokio::time::timeout(TIMEOUT, stream.read_to_end(&mut raw))
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
        let lines = String::from_utf8_lossy(&decoded)
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(str::to_string)
            .collect();
        (status, lines)
    }
}

fn decode_chunked(raw: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < raw.len() {
        let Some(nl) = raw[i..].windows(2).position(|w| w == b"\r\n") else {
            break;
        };
        let size_text = String::from_utf8_lossy(&raw[i..i + nl]);
        let Ok(size) = usize::from_str_radix(size_text.trim().split(';').next().unwrap_or(""), 16)
        else {
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

// ---- helpers ----

fn cid() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

fn body_of(response: &EventEnvelope) -> Value {
    assert!(
        matches!(response.body(), Value::Map(_)),
        "a map body: {:?}",
        response.body()
    );
    response.body().clone()
}

fn s(value: &Value) -> String {
    text_of(Some(value))
}

fn int(value: &Value) -> i64 {
    value
        .as_i64()
        .or_else(|| value.as_f64().map(|f| f as i64))
        .unwrap_or(-1)
}

/// One field of each entry of a list of maps (the call log's names, routes or statuses).
fn each(body: &Value, list: &str, key: &str) -> Vec<String> {
    match field(body, list) {
        Some(Value::Array(items)) => items.iter().map(|item| s(at(item, &[key]))).collect(),
        _ => Vec::new(),
    }
}

fn decision(id: &str, approved: bool, note: &str) -> Value {
    map(vec![
        ("id", Value::from(id)),
        ("approved", Value::Boolean(approved)),
        ("note", Value::from(note)),
    ])
}

fn decisions(turn: i64, decided: Vec<Value>) -> Value {
    map(vec![
        ("turn", Value::from(turn)),
        ("decisions", Value::Array(decided)),
    ])
}

fn question(text: &str) -> Value {
    map(vec![("question", Value::from(text))])
}

fn event_names(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| line.strip_prefix("event:"))
        .map(|name| name.trim().to_string())
        .collect()
}

fn data_events(lines: &[String]) -> Vec<serde_json::Value> {
    lines
        .iter()
        .filter_map(|line| line.strip_prefix("data:"))
        .filter_map(|text| serde_json::from_str::<serde_json::Value>(text.trim()).ok())
        .filter(|event| event.get("type").is_some())
        .collect()
}

fn js(event: &serde_json::Value, path: &[&str]) -> String {
    let mut current = event;
    for key in path {
        current = match current.get(key) {
            Some(value) => value,
            None => return String::new(),
        };
    }
    match current {
        serde_json::Value::String(text) => text.clone(),
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    }
}

// ---- the scenarios ----

async fn order_assistant(h: &Harness) {
    // a question is required
    let rejected = h
        .run_graph(
            ORDER_ASSISTANT,
            &cid(),
            map(vec![("customer", Value::from("anonymous"))]),
        )
        .await;
    assert_eq!(400, rejected.status(), "{:?}", rejected.body());
    let rejected = body_of(&rejected);
    assert_eq!("rejected", s(at(&rejected, &["type"])));
    assert!(s(at(&rejected, &["message"])).contains("question"));

    // a delivered order is answered without an approval
    let delivered_cid = cid();
    let response = h
        .run_graph(
            ORDER_ASSISTANT,
            &delivered_cid,
            question("Where is my order ORD-1001?"),
        )
        .await;
    assert_eq!(200, response.status(), "{:?}", response.body());
    let body = body_of(&response);
    assert_eq!(ANSWER, s(at(&body, &["type"])), "{body:?}");
    assert!(s(at(&body, &["message"])).contains("delivered"), "{body:?}");
    assert_eq!("fresh", s(at(&body, &["run"])));
    assert_eq!(delivered_cid, s(at(&body, &["cid"])));
    assert_eq!(
        2,
        int(at(&body, &["turn"])),
        "two model turns: the tool calls, then the answer"
    );
    assert_eq!(
        vec!["order_lookup", "shipment_status"],
        each(&body, "calls", "name")
    );
    assert_eq!(
        vec!["demo.order.lookup", "demo.shipment.status"],
        each(&body, "calls", "route")
    );
    assert_eq!(vec!["ok", "ok"], each(&body, "calls", "status"));
    assert_eq!("mock-llm", s(at(&body, &["model"])));
    assert!(!at(&body, &["usage", "input_tokens"]).is_nil(), "{body:?}");
    // the engine's graph context reached the model turns (RFC-0012 section 8)
    let context = agent_doubles::last_graph_context();
    assert_eq!(
        Some(&ORDER_ASSISTANT.to_string()),
        context.get("x-graph-id"),
        "{context:?}"
    );
    assert_eq!(
        Some(&"assist".to_string()),
        context.get("x-node"),
        "{context:?}"
    );

    // a lost shipment waits for the supervisor's refund approval
    let lost = cid();
    let advice = body_of(
        &h.run_graph(
            ORDER_ASSISTANT,
            &lost,
            question("My order ORD-1002 never arrived. Can you help?"),
        )
        .await,
    );
    assert_eq!(APPROVAL, s(at(&advice, &["type"])), "{advice:?}");
    assert_eq!("fresh", s(at(&advice, &["run"])));
    assert_eq!(
        2,
        int(at(&advice, &["turn"])),
        "the refund is asked for on the second model turn"
    );
    assert!(
        s(at(&advice, &["message"])).contains("supervisor"),
        "{advice:?}"
    );
    assert_eq!(
        vec!["issue_refund"],
        each(&advice, "calls", "name"),
        "{advice:?}"
    );
    assert_eq!(vec!["demo.refund.issue"], each(&advice, "calls", "route"));
    assert_eq!(Some(true), at(&advice, &["calls", "0", "wait"]).as_bool());
    assert_eq!(
        "ORD-1002",
        s(at(&advice, &["calls", "0", "input", "order_id"]))
    );
    assert_eq!(
        Some(174.0),
        at(&advice, &["calls", "0", "input", "amount"]).as_f64()
    );
    assert_eq!(
        vec!["order_lookup", "shipment_status"],
        each(&advice, "answered", "name"),
        "the advice also shows the calls the loop already answered"
    );
    assert_eq!(vec!["ok", "ok"], each(&advice, "answered", "status"));
    let call_id = s(at(&advice, &["calls", "0", "id"]));
    assert!(!call_id.is_empty());

    // a resume without decisions keeps waiting and re-advises
    let again = body_of(
        &h.run_graph(
            ORDER_ASSISTANT,
            &lost,
            map(vec![("note", Value::from("just checking"))]),
        )
        .await,
    );
    assert_eq!(APPROVAL, s(at(&again, &["type"])), "{again:?}");
    assert_eq!("resume", s(at(&again, &["run"])));
    assert_eq!(2, int(at(&again, &["turn"])));
    assert_eq!(
        call_id,
        s(at(&again, &["calls", "0", "id"])),
        "the same call still waits"
    );

    // decisions for another turn than the one that waits are not applied
    let stale = body_of(
        &h.run_graph(
            ORDER_ASSISTANT,
            &lost,
            decisions(1, vec![decision(&call_id, true, "late")]),
        )
        .await,
    );
    assert_eq!(APPROVAL, s(at(&stale, &["type"])), "{stale:?}");

    // a decision for a call that is not waiting is refused before any token is spent, and the
    // graph re-advises instead of losing the transaction
    let refused = body_of(
        &h.run_graph(
            ORDER_ASSISTANT,
            &lost,
            decisions(2, vec![decision("not-a-call", true, "typo")]),
        )
        .await,
    );
    assert_eq!(APPROVAL, s(at(&refused, &["type"])), "{refused:?}");
    assert!(
        s(at(&refused, &["error"])).contains("not a call waiting for approval"),
        "{refused:?}"
    );
    assert_eq!(
        call_id,
        s(at(&refused, &["calls", "0", "id"])),
        "the same call still waits"
    );

    // the supervisor approves: the loop runs the call it showed and the model answers
    let answer = body_of(
        &h.run_graph(
            ORDER_ASSISTANT,
            &lost,
            decisions(
                2,
                vec![decision(&call_id, true, "Checked with the carrier")],
            ),
        )
        .await,
    );
    assert_eq!(ANSWER, s(at(&answer, &["type"])), "{answer:?}");
    assert_eq!("resume", s(at(&answer, &["run"])));
    assert_eq!(3, int(at(&answer, &["turn"])));
    let text = s(at(&answer, &["message"]));
    assert!(
        text.contains("refund RF-") && text.contains("174.0 USD"),
        "{text}"
    );
    assert_eq!(vec!["issue_refund"], each(&answer, "calls", "name"));
    assert_eq!(vec!["ok"], each(&answer, "calls", "status"), "{answer:?}");

    // a second transaction for the same order: the write fails with 409 and the model reads the error
    let second = cid();
    let second_advice = body_of(
        &h.run_graph(
            ORDER_ASSISTANT,
            &second,
            question("Please refund ORD-1002, it is lost."),
        )
        .await,
    );
    assert_eq!(
        APPROVAL,
        s(at(&second_advice, &["type"])),
        "{second_advice:?}"
    );
    let second_id = s(at(&second_advice, &["calls", "0", "id"]));
    let declined = body_of(
        &h.run_graph(
            ORDER_ASSISTANT,
            &second,
            decisions(
                int(at(&second_advice, &["turn"])),
                vec![decision(&second_id, true, "approved")],
            ),
        )
        .await,
    );
    assert_eq!(ANSWER, s(at(&declined, &["type"])), "{declined:?}");
    let reason = s(at(&declined, &["message"]));
    assert!(
        reason.contains("could not issue") && reason.contains("already refunded"),
        "{reason}"
    );
    assert_eq!(
        Some(&"error".to_string()),
        each(&declined, "calls", "status").last(),
        "{declined:?}"
    );

    // a refused decision goes back to the model as the call's error
    let replacement = cid();
    let advice = body_of(
        &h.run_graph(
            ORDER_ASSISTANT,
            &replacement,
            question("Order ORD-1004 is lost, I want my money back."),
        )
        .await,
    );
    assert_eq!(APPROVAL, s(at(&advice, &["type"])), "{advice:?}");
    let refund_id = s(at(&advice, &["calls", "0", "id"]));
    let answer = body_of(
        &h.run_graph(
            ORDER_ASSISTANT,
            &replacement,
            decisions(
                int(at(&advice, &["turn"])),
                vec![decision(
                    &refund_id,
                    false,
                    "The customer prefers a replacement",
                )],
            ),
        )
        .await,
    );
    assert_eq!(ANSWER, s(at(&answer, &["type"])), "{answer:?}");
    let text = s(at(&answer, &["message"]));
    assert!(
        text.contains("could not issue") && text.contains("prefers a replacement"),
        "{text}"
    );
    assert_eq!(
        Some(&"refused".to_string()),
        each(&answer, "calls", "status").last(),
        "{answer:?}"
    );

    // a watched run streams its progress events, then the result
    let watched = cid();
    let (status, lines) = h
        .stream(
            ORDER_ASSISTANT,
            &watched,
            "{\"question\": \"Has order ORD-1003 shipped?\"}",
        )
        .await;
    assert_eq!(200, status, "{lines:?}");
    let names = event_names(&lines);
    assert!(names.len() >= 2, "{names:?}");
    assert_eq!("result", names[names.len() - 2], "{names:?}");
    assert_eq!("done", names[names.len() - 1], "{names:?}");
    let events = data_events(&lines);
    let progress: Vec<&serde_json::Value> = events
        .iter()
        .filter(|event| js(event, &["type"]) == "progress")
        .collect();
    assert_eq!(
        "Investigating the order",
        js(progress.first().expect("a progress event"), &["message"]),
        "the node's own progress text"
    );
    let loop_events: Vec<String> = progress
        .iter()
        .map(|event| js(event, &["event"]))
        .filter(|name| !name.is_empty())
        .collect();
    for expected in [
        "run.start",
        "turn.start",
        "tool.start",
        "tool.end",
        "turn.end",
        "run.end",
    ] {
        assert!(
            loop_events.iter().any(|name| name == expected),
            "{loop_events:?}"
        );
    }
    let tools: Vec<String> = progress
        .iter()
        .filter(|event| js(event, &["event"]) == "tool.start")
        .map(|event| js(event, &["data", "name"]))
        .collect();
    assert_eq!(vec!["order_lookup", "shipment_status"], tools);
    for event in &progress {
        assert_eq!(watched, js(event, &["cid"]), "{event}");
    }
    let result = events.last().expect("the result event");
    assert_eq!("result", js(result, &["type"]), "{result}");
    assert_eq!(ANSWER, js(result, &["body", "type"]), "{result}");
    assert!(
        js(result, &["body", "message"]).contains("in_transit"),
        "{result}"
    );
}

async fn diagram_assistant(h: &Harness) {
    // a description is required
    let rejected = h
        .run_graph(
            DIAGRAM_ASSISTANT,
            &cid(),
            map(vec![("topic", Value::from("nothing"))]),
        )
        .await;
    assert_eq!(400, rejected.status(), "{:?}", rejected.body());
    assert_eq!("rejected", s(at(&body_of(&rejected), &["type"])));

    // the model validates the diagram through the MCP-bound route before answering
    let response = h
        .run_graph(
            DIAGRAM_ASSISTANT,
            &cid(),
            question("Draw a flowchart of a refund approval: request, review, approve or reject."),
        )
        .await;
    assert_eq!(200, response.status(), "{:?}", response.body());
    let body = body_of(&response);
    assert_eq!(ANSWER, s(at(&body, &["type"])), "{body:?}");
    let text = s(at(&body, &["message"]));
    assert!(
        text.contains("```mermaid") && text.contains("flowchart TD"),
        "{text}"
    );
    assert_eq!(
        2,
        int(at(&body, &["turns"])),
        "the draft and the validated answer: {body:?}"
    );
    assert_eq!(
        vec!["render_diagram"],
        each(&body, "calls", "name"),
        "{body:?}"
    );
    assert_eq!(
        vec!["diagram.render"],
        each(&body, "calls", "route"),
        "the catalog maps the tool name to the bound route"
    );
    assert_eq!(vec!["ok"], each(&body, "calls", "status"));
    assert_eq!("mock-llm", s(at(&body, &["model"])));
    // the tool stored the image out of band: the model reports the reference, the file holds the SVG
    let reference = text
        .split("reference ")
        .nth(1)
        .map(|rest| rest.chars().take(32).collect::<String>())
        .expect("the answer names the stored image's reference");
    assert_eq!(32, reference.len(), "{text}");
    let file = h.store.join(format!("{reference}.svg"));
    let svg = std::fs::read_to_string(&file)
        .unwrap_or_else(|e| panic!("the rendered image is stored as {}: {e}", file.display()));
    assert!(svg.contains("<svg"), "the file holds the SVG");

    // the render function stores a valid diagram and nothing for an invalid one
    let valid =
        h.po.request(
            EventEnvelope::new()
                .set_to("diagram.render")
                .set_raw_body(map(vec![
                    ("diagramCode", Value::from("flowchart LR\n  A --> B")),
                    ("title", Value::from("two steps")),
                ])),
            TIMEOUT,
        )
        .await
        .expect("diagram.render replies");
    assert_eq!(200, valid.status(), "{:?}", valid.body());
    let stored = body_of(&valid);
    assert_eq!(Some(true), at(&stored, &["valid"]).as_bool());
    assert_eq!("two steps", s(at(&stored, &["title"])));
    assert!(
        at(&stored, &["rawSVG"]).is_nil(),
        "the image never rides the reply"
    );
    assert!(
        at(&stored, &["file"]).is_nil() && at(&stored, &["page"]).is_nil(),
        "no path in the result"
    );
    let reference = s(at(&stored, &["reference"]));
    let file = h.store.join(format!("{reference}.svg"));
    let bytes = std::fs::metadata(&file)
        .unwrap_or_else(|e| panic!("the rendered image is stored as {}: {e}", file.display()))
        .len();
    assert_eq!(
        bytes,
        at(&stored, &["bytes"]).as_u64().unwrap_or(0),
        "bytes is the SVG file's size"
    );
    let page = std::fs::read_to_string(h.store.join(format!("{reference}.html")))
        .expect("the page is stored");
    assert!(
        page.contains("<svg") && page.contains("two steps") && page.contains("flowchart LR"),
        "the page inlines the SVG and shows the title and the code"
    );
    let invalid =
        h.po.request(
            EventEnvelope::new()
                .set_to("diagram.render")
                .set_raw_body(map(vec![(
                    "diagramCode",
                    Value::from("this is not a diagram"),
                )])),
            TIMEOUT,
        )
        .await
        .expect("diagram.render replies");
    assert_eq!(200, invalid.status(), "{:?}", invalid.body());
    let refused = body_of(&invalid);
    assert_eq!(Some(false), at(&refused, &["valid"]).as_bool());
    assert!(
        !at(&refused, &["error"]).is_nil(),
        "the server's error reaches the model"
    );
    assert!(
        at(&refused, &["reference"]).is_nil(),
        "nothing is stored for an invalid diagram"
    );
    let missing =
        h.po.request(
            EventEnvelope::new()
                .set_to("diagram.render")
                .set_raw_body(map(vec![("title", Value::from("no code"))])),
            TIMEOUT,
        )
        .await;
    let missing_status = match missing {
        Ok(reply) => reply.status(),
        Err(error) => error.status(),
    };
    assert_eq!(400, missing_status);

    // a watched run shows the MCP tool call
    let (status, lines) = h
        .stream(
            DIAGRAM_ASSISTANT,
            &cid(),
            "{\"question\": \"A sequence of a login\"}",
        )
        .await;
    assert_eq!(200, status, "{lines:?}");
    let events = data_events(&lines);
    let tool_starts: Vec<&serde_json::Value> = events
        .iter()
        .filter(|event| js(event, &["event"]) == "tool.start")
        .collect();
    assert_eq!(1, tool_starts.len(), "{events:?}");
    assert_eq!("render_diagram", js(tool_starts[0], &["data", "name"]));
    assert_eq!("diagram.render", js(tool_starts[0], &["data", "route"]));
    assert_eq!(
        "Drawing the diagram",
        js(events.first().expect("an event"), &["message"])
    );
    assert_eq!("result", js(events.last().expect("an event"), &["type"]));
}

async fn site_notes(h: &Harness) {
    agent_doubles::reset_tools();
    // a section and a topic are required
    let rejected = h
        .run_graph(
            SITE_NOTES,
            &cid(),
            map(vec![("topic", Value::from("configuration"))]),
        )
        .await;
    assert_eq!(400, rejected.status(), "{:?}", rejected.body());
    assert_eq!("rejected", s(at(&body_of(&rejected), &["type"])));
    let second = h
        .run_graph(
            SITE_NOTES,
            &cid(),
            map(vec![("section", Value::from(SECTION))]),
        )
        .await;
    assert_eq!(400, second.status());

    // run 1: the crawl runs unasked (a read), the write of the notes waits for a person
    let notes_cid = cid();
    let request = map(vec![
        ("section", Value::from(SECTION)),
        ("topic", Value::from("Configuration keys")),
    ]);
    let advice = body_of(&h.run_graph(SITE_NOTES, &notes_cid, request).await);
    assert_eq!(APPROVAL, s(at(&advice, &["type"])), "{advice:?}");
    assert_eq!(
        2,
        int(at(&advice, &["turn"])),
        "the write is asked for on the second model turn"
    );
    assert_eq!(
        vec!["file_write"],
        each(&advice, "calls", "name"),
        "{advice:?}"
    );
    assert_eq!(vec!["file.write"], each(&advice, "calls", "route"));
    assert_eq!(Some(true), at(&advice, &["calls", "0", "wait"]).as_bool());
    assert_eq!(
        "notes/configuration-keys.md",
        s(at(&advice, &["calls", "0", "input", "path"]))
    );
    assert!(
        s(at(&advice, &["calls", "0", "input", "content"])).contains("Runtime overrides"),
        "the notes hold the crawled pages"
    );
    assert_eq!(vec!["web_crawl"], each(&advice, "answered", "name"));
    assert_eq!(vec!["ok"], each(&advice, "answered", "status"));
    assert!(
        agent_doubles::writes().lock().expect("writes").is_empty(),
        "nothing is written before the person decides"
    );
    let context = agent_doubles::last_graph_context();
    assert_eq!(
        Some(&SITE_NOTES.to_string()),
        context.get("x-graph-id"),
        "{context:?}"
    );
    assert_eq!(
        Some(&"research".to_string()),
        context.get("x-node"),
        "{context:?}"
    );
    let write_id = s(at(&advice, &["calls", "0", "id"]));

    // run 2: the person approves - the loop runs the write it showed, and the model answers with the path
    let answer = body_of(
        &h.run_graph(
            SITE_NOTES,
            &notes_cid,
            decisions(2, vec![decision(&write_id, true, "Notes are welcome")]),
        )
        .await,
    );
    assert_eq!(ANSWER, s(at(&answer, &["type"])), "{answer:?}");
    assert_eq!("resume", s(at(&answer, &["run"])));
    let text = s(at(&answer, &["message"]));
    assert!(
        text.contains("notes/configuration-keys.md") && text.contains("2 pages"),
        "{text}"
    );
    assert_eq!(vec!["file_write"], each(&answer, "calls", "name"));
    assert_eq!(vec!["ok"], each(&answer, "calls", "status"), "{answer:?}");
    assert!(agent_doubles::writes()
        .lock()
        .expect("writes")
        .get("notes/configuration-keys.md")
        .expect("the notes were written")
        .starts_with("# Notes on Configuration keys"));
    assert_eq!(
        vec!["web.crawl", "file.write"],
        agent_doubles::calls_made().lock().expect("calls").clone()
    );

    // a refused write goes back to the model
    agent_doubles::reset_tools();
    let refused_cid = cid();
    let advice = body_of(
        &h.run_graph(
            SITE_NOTES,
            &refused_cid,
            map(vec![
                ("section", Value::from(SECTION)),
                ("topic", Value::from("Overrides")),
            ]),
        )
        .await,
    );
    assert_eq!(APPROVAL, s(at(&advice, &["type"])), "{advice:?}");
    let call_id = s(at(&advice, &["calls", "0", "id"]));
    let answer = body_of(
        &h.run_graph(
            SITE_NOTES,
            &refused_cid,
            decisions(
                int(at(&advice, &["turn"])),
                vec![decision(&call_id, false, "Not in the shared workspace")],
            ),
        )
        .await,
    );
    assert_eq!(ANSWER, s(at(&answer, &["type"])), "{answer:?}");
    let text = s(at(&answer, &["message"]));
    assert!(
        text.contains("could not write") && text.contains("shared workspace"),
        "{text}"
    );
    assert_eq!(
        Some(&"refused".to_string()),
        each(&answer, "calls", "status").last(),
        "{answer:?}"
    );
    assert!(
        agent_doubles::writes().lock().expect("writes").is_empty(),
        "a refused write never ran"
    );

    // a section outside the allowlist is an error the model reads
    let answer = body_of(
        &h.run_graph(
            SITE_NOTES,
            &cid(),
            map(vec![
                ("section", Value::from("https://elsewhere.example/docs/")),
                ("topic", Value::from("Anything")),
            ]),
        )
        .await,
    );
    assert_eq!(ANSWER, s(at(&answer, &["type"])), "{answer:?}");
    let text = s(at(&answer, &["message"]));
    assert!(
        text.contains("could not read the section") && text.contains("not one of the sites"),
        "{text}"
    );
    assert_eq!(
        vec!["error"],
        each(&answer, "calls", "status"),
        "{answer:?}"
    );
}

async fn document_ingest(h: &Harness) {
    agent_doubles::reset_tools();
    // the write runs unasked (mode: auto) and loads the graph
    let answer = body_of(
        &h.run_graph(
            DOCUMENT_INGEST,
            &cid(),
            map(vec![("document", Value::from("supplier-agreement.md"))]),
        )
        .await,
    );
    assert_eq!(ANSWER, s(at(&answer, &["type"])), "{answer:?}");
    assert_eq!(3, int(at(&answer, &["turn"])), "extract, ingest, answer");
    let text = s(at(&answer, &["message"]));
    assert!(
        text.contains("Ingested 12 nodes and 11 relationships")
            && text.contains("2 parties, 4 obligations, 3 rules and 2 deadlines"),
        "{text}"
    );
    assert_eq!(
        vec!["file_extract", "graph_ingest"],
        each(&answer, "calls", "name")
    );
    assert_eq!(
        vec!["ok", "ok"],
        each(&answer, "calls", "status"),
        "{answer:?}"
    );
    assert_eq!(
        vec!["file.extract", "graph.ingest"],
        agent_doubles::calls_made().lock().expect("calls").clone(),
        "the write ran without an approval - mode auto"
    );
    {
        let nodes = agent_doubles::nodes().lock().expect("nodes");
        let document = nodes
            .get("document:supplier-agreement.md")
            .unwrap_or_else(|| panic!("{:?}", nodes.keys().collect::<Vec<_>>()));
        assert_eq!("Document", s(at(document, &["label"])));
        let obligation = nodes
            .get("obligation:supplier-agreement-obl-2")
            .expect("the second obligation");
        assert_eq!("Obligation", s(at(obligation, &["label"])));
        assert!(
            s(at(obligation, &["properties", "text"])).contains("30 days"),
            "{obligation:?}"
        );
        assert_eq!(
            "supplier-agreement.md",
            s(at(obligation, &["properties", "source"])),
            "every node carries the document as its source"
        );
        assert!(nodes.contains_key("party:northwind-office-supplies"));
        assert_eq!(
            11,
            agent_doubles::relationships()
                .lock()
                .expect("relationships")
                .len()
        );
    }
    let context = agent_doubles::last_graph_context();
    assert_eq!(
        Some(&DOCUMENT_INGEST.to_string()),
        context.get("x-graph-id"),
        "{context:?}"
    );
    assert_eq!(
        Some(&"ingest".to_string()),
        context.get("x-node"),
        "{context:?}"
    );
    // the same document again: the ingest is idempotent, the graph holds the same nodes
    let again = body_of(
        &h.run_graph(
            DOCUMENT_INGEST,
            &cid(),
            map(vec![("document", Value::from("supplier-agreement.md"))]),
        )
        .await,
    );
    assert_eq!(ANSWER, s(at(&again, &["type"])), "{again:?}");
    assert_eq!(12, agent_doubles::nodes().lock().expect("nodes").len());

    // a missing document is an error the model reads
    agent_doubles::reset_tools();
    let answer = body_of(
        &h.run_graph(
            DOCUMENT_INGEST,
            &cid(),
            map(vec![("document", Value::from("missing.md"))]),
        )
        .await,
    );
    assert_eq!(ANSWER, s(at(&answer, &["type"])), "{answer:?}");
    let text = s(at(&answer, &["message"]));
    assert!(
        text.contains("could not read missing.md") && text.contains("file not found"),
        "{text}"
    );
    assert!(agent_doubles::nodes().lock().expect("nodes").is_empty());
    let rejected = h
        .run_graph(
            DOCUMENT_INGEST,
            &cid(),
            map(vec![("path", Value::from("x.md"))]),
        )
        .await;
    assert_eq!(400, rejected.status(), "{:?}", rejected.body());
}

async fn document_query(h: &Harness) {
    // on an empty graph the answer says so
    agent_doubles::reset_tools();
    let answer = body_of(
        &h.run_graph(DOCUMENT_QUERY, &cid(), question("Which rules apply?"))
            .await,
    );
    assert_eq!(ANSWER, s(at(&answer, &["type"])), "{answer:?}");
    assert!(
        s(at(&answer, &["message"])).contains("holds nothing yet"),
        "{answer:?}"
    );
    assert_eq!(
        vec!["graph.schema"],
        agent_doubles::calls_made().lock().expect("calls").clone()
    );

    // two policies ingested, then the schema is read, one query runs and the sources are cited
    agent_doubles::reset_tools();
    for document in ["travel-policy.md", "data-retention-policy.md"] {
        let loaded = body_of(
            &h.run_graph(
                DOCUMENT_INGEST,
                &cid(),
                map(vec![("document", Value::from(document))]),
            )
            .await,
        );
        assert_eq!(ANSWER, s(at(&loaded, &["type"])), "{loaded:?}");
    }
    {
        let nodes = agent_doubles::nodes().lock().expect("nodes");
        let deadlines = nodes
            .values()
            .filter(|node| s(at(node, &["label"])) == "Deadline")
            .count();
        assert_eq!(4, deadlines, "{:?}", nodes.keys().collect::<Vec<_>>());
    }
    agent_doubles::calls_made().lock().expect("calls").clear();
    let answer = body_of(
        &h.run_graph(
            DOCUMENT_QUERY,
            &cid(),
            question("Which deadlines do the policies set?"),
        )
        .await,
    );
    assert_eq!(ANSWER, s(at(&answer, &["type"])), "{answer:?}");
    assert_eq!(3, int(at(&answer, &["turn"])), "schema, query, answer");
    let text = s(at(&answer, &["message"]));
    assert!(
        text.contains("From the knowledge graph:")
            && text.contains("retention audit")
            && text.contains("reimbursement caps"),
        "{text}"
    );
    assert_eq!(
        4,
        text.matches("\n- ").count(),
        "the four deadlines of the two policies: {text}"
    );
    assert!(
        text.contains("(source: travel-policy.md)")
            && text.contains("(source: data-retention-policy.md)"),
        "{text}"
    );
    assert_eq!(
        vec!["graph_schema", "graph_query"],
        each(&answer, "calls", "name")
    );
    assert_eq!(
        vec!["graph.schema", "graph.query"],
        agent_doubles::calls_made().lock().expect("calls").clone(),
        "reads only - no file is read"
    );
    let parties = body_of(
        &h.run_graph(DOCUMENT_QUERY, &cid(), question("Who are the parties?"))
            .await,
    );
    assert!(
        s(at(&parties, &["message"])).contains("Contoso Facilities"),
        "{parties:?}"
    );
    let rejected = h
        .run_graph(
            DOCUMENT_QUERY,
            &cid(),
            map(vec![("prompt", Value::from("x"))]),
        )
        .await;
    assert_eq!(400, rejected.status(), "{:?}", rejected.body());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn agent_demo_graphs() {
    test_support::run_at_exit(platform_core::util::elastic_queue::shutdown_cleanup);
    let redis_port = redis_double::start_resp_double().await;
    // the state-store connection is lazy and reads configuration at first use
    overrides::set("redis.host", "127.0.0.1");
    overrides::set("redis.port", &redis_port.to_string());
    overrides::set("rest.server.port", "0");
    // every route the graphs name is served locally here by the stand-ins: no peer map
    overrides::set("yaml.event.over.http", "classpath:/no-such-peer-map.yaml");
    let holding = test_support::temp_path("agent-graphs");
    overrides::set("transient.data.store", &holding.display().to_string());
    let store = test_support::temp_path("rendered-diagrams");
    overrides::set("diagram.store.folder", &store.display().to_string());
    AutoStart::main(vec![]).await.expect("app lifecycle");
    let platform = Platform::get_instance();
    agent_doubles::register_doubles(&platform);
    let graphs = knowledge_graph::graphs::get_all_graphs();
    for graph in [
        ORDER_ASSISTANT,
        DIAGRAM_ASSISTANT,
        SITE_NOTES,
        DOCUMENT_INGEST,
        DOCUMENT_QUERY,
    ] {
        assert!(
            graphs.iter().any(|g| g == graph),
            "{graph} compiles at boot - graphs: {graphs:?}"
        );
    }
    let port = automation::server_address().expect("server started").port();
    let harness = Harness {
        po: PostOffice::new(&platform),
        target: format!("http://127.0.0.1:{port}"),
        port,
        store,
    };
    order_assistant(&harness).await;
    diagram_assistant(&harness).await;
    site_notes(&harness).await;
    document_ingest(&harness).await;
    document_query(&harness).await;
    log::info!("the five agent demo graphs ran end to end, token-free");
}
