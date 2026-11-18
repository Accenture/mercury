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

//! Scripted stand-ins for the routes the agent demo graphs reach - the twins of the Java example's
//! `MockLlmChat`, `MockAgentTools` and `MockDiagramRender`, so the graphs run in the test scope
//! without tokens, credentials, a network, a pack or a graph database.
//!
//! In production `llm.chat` resolves to the LLM helper app of a language pack, the tool routes to
//! the pack's `extensions/agent-tools` and `mermaid.render` to the pack's MCP binding, each through
//! declarative Event-over-HTTP; here the same route names resolve to these functions - the
//! route-decoupling contract is what makes an LLM node, a tool and an MCP-bound route testable by
//! configuration. The scripted model serves two contracts: a plain chat (the support-triage graph)
//! and a loop turn (a request with `tools`), following the loop contract of RFC-0011 section 3.2 -
//! `tool_use` with the calls, or `done` with the text, a conversation reference and a turn count
//! the next turn must echo, a 409 when it does not - and plays a careful assistant: it looks the
//! order up and checks the shipment, proposes a refund only for a lost shipment, drafts and
//! validates a diagram, crawls a section and writes the notes, extracts a document's facts into one
//! idempotent ingest, and answers a question from the graph with the sources.
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex, OnceLock};

use async_trait::async_trait;
use platform_core::{AppError, ComposableFunction, EventEnvelope, Platform};
use regex::Regex;
use rmpv::Value;

/// The documentation site the fake allowlist names.
pub const SITE: &str = "https://docs.example.test";
/// The section the site-notes scenarios crawl.
pub const SECTION: &str = "https://docs.example.test/guides/configuration/";
/// The model name the scripted turns report.
pub const MODEL: &str = "mock-llm";
/// The tool routes the stand-in serves, each registered under its own name.
pub const TOOL_ROUTES: [&str; 7] = [
    "web.crawl",
    "web.fetch",
    "file.write",
    "file.extract",
    "graph.ingest",
    "graph.schema",
    "graph.query",
];

const DIAGRAM_CODE: &str =
    "flowchart TD\n  A[Request] --> B{Review}\n  B -->|approve| C[Approved]\n  B -->|reject| D[Rejected]";
const CORPUS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/corpus");
const STATUS: &str = "status";
const RENDER_CALL: &str = "d1";
const LOOKUP_CALL: &str = "c1";
const SHIPMENT_CALL: &str = "c2";
const REFUND_CALL: &str = "c3";
const CRAWL_CALL: &str = "n1";
const WRITE_CALL: &str = "n2";
const EXTRACT_CALL: &str = "g1";
const INGEST_CALL: &str = "g2";
const SCHEMA_CALL: &str = "q1";
const QUERY_CALL: &str = "q2";

// ---- MsgPack value helpers ----

/// A MsgPack map from string keys.
pub fn map(entries: Vec<(&str, Value)>) -> Value {
    Value::Map(
        entries
            .into_iter()
            .map(|(key, value)| (Value::from(key), value))
            .collect(),
    )
}

/// One entry of a MsgPack map, by key.
pub fn field<'a>(body: &'a Value, key: &str) -> Option<&'a Value> {
    match body {
        Value::Map(entries) => entries
            .iter()
            .find(|(name, _)| name.as_str() == Some(key))
            .map(|(_, value)| value),
        _ => None,
    }
}

/// A nested value by path; a numeric segment indexes an array; a missing step is `Nil`.
pub fn at<'a>(value: &'a Value, path: &[&str]) -> &'a Value {
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

/// A value as text: a string's content, nothing for `Nil`, the display form otherwise.
pub fn text_of(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(text)) => text.as_str().unwrap_or_default().to_string(),
        Some(Value::Nil) | None => String::new(),
        Some(other) => format!("{other}"),
    }
}

fn string(body: &Value, key: &str) -> Option<String> {
    field(body, key).and_then(Value::as_str).map(str::to_string)
}

fn required_text(body: &Value, key: &str) -> Result<String, AppError> {
    match string(body, key) {
        Some(text) if !text.trim().is_empty() => Ok(text),
        _ => Err(AppError::new(400, format!("{key} is required"))),
    }
}

/// A number the way the Java twin prints it: an integral float keeps one decimal (`174.0`).
fn number_text(value: Option<&Value>) -> String {
    match value {
        Some(Value::F64(number)) if number.fract() == 0.0 => format!("{number:.1}"),
        Some(Value::F32(number)) if number.fract() == 0.0 => format!("{number:.1}"),
        other => text_of(other),
    }
}

fn regex(cell: &'static OnceLock<Regex>, pattern: &str) -> &'static Regex {
    cell.get_or_init(|| Regex::new(pattern).expect("a valid pattern"))
}

fn slug(text: &str) -> String {
    static NON_ALNUM: OnceLock<Regex> = OnceLock::new();
    let lowered = text.to_lowercase();
    let dashed = regex(&NON_ALNUM, "[^a-z0-9]+").replace_all(&lowered, "-");
    dashed.trim_matches('-').to_string()
}

// ---- the scripted model: llm.chat ----

#[derive(Clone, Copy, Default, PartialEq)]
enum Kind {
    #[default]
    Order,
    Diagram,
    Notes,
    Ingest,
    Query,
}

#[derive(Default)]
struct Counts {
    parties: usize,
    obligations: usize,
    rules: usize,
    deadlines: usize,
}

/// What the scripted assistant remembers of one conversation: the record's turn count and the facts.
#[derive(Default)]
struct Conversation {
    turn: i64,
    kind: Kind,
    section: Option<String>,
    topic: Option<String>,
    document: Option<String>,
    pages: Vec<Value>,
    crawl_error: Option<String>,
    written: Option<Value>,
    write_error: Option<String>,
    extracted: Option<String>,
    extract_error: Option<String>,
    ingested: Option<Value>,
    ingest_error: Option<String>,
    counts: Counts,
    schema: Option<Value>,
    schema_error: Option<String>,
    rows: Option<Vec<Value>>,
    query_error: Option<String>,
    render: Option<Value>,
    render_error: Option<String>,
    order_id: Option<String>,
    order: Option<Value>,
    order_error: Option<String>,
    shipment: Option<Value>,
    shipment_error: Option<String>,
    refund: Option<Value>,
    refund_error: Option<String>,
}

fn conversations() -> &'static Mutex<HashMap<String, Conversation>> {
    static CONVERSATIONS: OnceLock<Mutex<HashMap<String, Conversation>>> = OnceLock::new();
    CONVERSATIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn graph_context() -> &'static Mutex<HashMap<String, String>> {
    static CONTEXT: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
    CONTEXT.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The graph context the most recent loop turn carried: `x-graph-id` and `x-node`, which the
/// engine's task node sends and the loop copies onto each turn (RFC-0012 section 8).
pub fn last_graph_context() -> HashMap<String, String> {
    graph_context().lock().expect("graph context").clone()
}

/// The scripted `llm.chat`.
pub struct ScriptedModel;

#[async_trait]
impl ComposableFunction for ScriptedModel {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body = input.body();
        if field(body, "tools").is_some() {
            let mut context = HashMap::new();
            for name in ["x-graph-id", "x-node"] {
                if let Some(value) = headers.get(name) {
                    context.insert(name.to_string(), value.clone());
                }
            }
            *graph_context().lock().expect("graph context") = context;
            return loop_turn(body).map(|reply| EventEnvelope::new().set_raw_body(reply));
        }
        // a plain chat: the support-triage graph's verdict, derived from the prompt
        let prompt = text_of(field(body, "prompt"));
        let label = if prompt.contains("crash") {
            "bug"
        } else if prompt.contains("wish") || prompt.contains("feature") {
            "feature"
        } else {
            "question"
        };
        Ok(EventEnvelope::new().set_raw_body(map(vec![
            (
                "data",
                map(vec![
                    ("label", Value::from(label)),
                    (
                        "reason",
                        Value::from("mock verdict for the token-free test"),
                    ),
                ]),
            ),
            (
                "usage",
                map(vec![
                    ("input_tokens", Value::from(42)),
                    ("output_tokens", Value::from(7)),
                ]),
            ),
            ("model", Value::from(MODEL)),
            ("stop_reason", Value::from("end_turn")),
        ])))
    }
}

fn loop_turn(body: &Value) -> Result<Value, AppError> {
    let mut conversations = conversations().lock().expect("conversations");
    let reference = match string(body, "conversation") {
        Some(reference) => {
            if !conversations.contains_key(&reference) {
                return Err(AppError::new(
                    404,
                    format!("Conversation not found: {reference}"),
                ));
            }
            reference
        }
        None => {
            let reference = uuid::Uuid::new_v4().simple().to_string();
            conversations.insert(reference.clone(), Conversation::default());
            reference
        }
    };
    let conversation = conversations
        .get_mut(&reference)
        .expect("the conversation exists");
    let expected = field(body, "turn").and_then(Value::as_i64).unwrap_or(0);
    if expected != conversation.turn {
        return Err(AppError::new(
            409,
            format!(
                "The record holds turn {}, not {expected}",
                conversation.turn
            ),
        ));
    }
    conversation.turn += 1;
    if let Some(Value::Array(results)) = field(body, "results") {
        remember(conversation, results);
    }
    let mut reply: Vec<(&str, Value)> = Vec::new();
    if conversation.turn == 1 {
        let tools = field(body, "tools");
        conversation.kind = if has_tool(tools, "web_crawl") {
            Kind::Notes
        } else if has_tool(tools, "graph_ingest") {
            Kind::Ingest
        } else if has_tool(tools, "graph_query") {
            Kind::Query
        } else if has_tool(tools, "render_diagram") {
            Kind::Diagram
        } else {
            Kind::Order
        };
        first_turn(conversation, &text_of(field(body, "prompt")), &mut reply);
    } else {
        match conversation.kind {
            Kind::Diagram => later_diagram_turn(conversation, &mut reply),
            Kind::Notes => later_notes_turn(conversation, &mut reply),
            Kind::Ingest => later_ingest_turn(conversation, &mut reply),
            Kind::Query => later_query_turn(conversation, &mut reply),
            Kind::Order => later_turn(conversation, &mut reply),
        }
    }
    reply.push(("conversation", Value::from(reference.as_str())));
    reply.push(("turn", Value::from(conversation.turn)));
    reply.push(("model", Value::from(MODEL)));
    reply.push((
        "usage",
        map(vec![
            ("input_tokens", Value::from(500)),
            ("output_tokens", Value::from(40)),
            ("cache_creation_input_tokens", Value::from(0)),
            ("cache_read_input_tokens", Value::from(0)),
        ]),
    ));
    Ok(map(reply))
}

fn has_tool(tools: Option<&Value>, name: &str) -> bool {
    match tools {
        Some(Value::Array(rows)) => rows
            .iter()
            .any(|row| field(row, "name").and_then(Value::as_str) == Some(name)),
        _ => false,
    }
}

/// `Section: ...`, `Topic: ...` or `Document: ...` in the prompt the graph composed.
fn prompt_field(prompt: &str, name: &str) -> Option<String> {
    static FIELD: OnceLock<Regex> = OnceLock::new();
    regex(&FIELD, r"(Section|Topic|Document):\s*([^;\n]+)")
        .captures_iter(prompt)
        .find(|capture| &capture[1] == name)
        .map(|capture| capture[2].trim().to_string())
}

fn call(id: &str, name: &str, arguments: Value) -> Value {
    map(vec![
        ("id", Value::from(id)),
        ("name", Value::from(name)),
        ("input", arguments),
    ])
}

fn calls(reply: &mut Vec<(&'static str, Value)>, calls: Vec<Value>) {
    reply.push((STATUS, Value::from("tool_use")));
    reply.push(("calls", Value::Array(calls)));
    reply.push(("stop_reason", Value::from("tool_use")));
}

fn done(reply: &mut Vec<(&'static str, Value)>, text: String) {
    reply.push((STATUS, Value::from("done")));
    reply.push(("text", Value::from(text)));
    reply.push(("stop_reason", Value::from("end_turn")));
}

fn first_turn(
    conversation: &mut Conversation,
    prompt: &str,
    reply: &mut Vec<(&'static str, Value)>,
) {
    match conversation.kind {
        Kind::Notes => {
            // the research assistant reads the section first - a bounded crawl under its path
            conversation.section = prompt_field(prompt, "Section");
            conversation.topic = prompt_field(prompt, "Topic");
            let (Some(section), Some(_)) = (&conversation.section, &conversation.topic) else {
                done(
                    reply,
                    "Please name the documentation section and the topic.".to_string(),
                );
                return;
            };
            static HOST: OnceLock<Regex> = OnceLock::new();
            let include = regex(&HOST, r"^https?://[^/]+").replace(section, "");
            let include = if include.is_empty() {
                "/".to_string()
            } else {
                include.to_string()
            };
            calls(
                reply,
                vec![call(
                    CRAWL_CALL,
                    "web_crawl",
                    map(vec![
                        ("url", Value::from(section.as_str())),
                        ("max_pages", Value::from(5)),
                        ("include", Value::from(include)),
                    ]),
                )],
            );
        }
        Kind::Ingest => {
            conversation.document = prompt_field(prompt, "Document");
            let Some(document) = &conversation.document else {
                done(reply, "Please name the document to ingest.".to_string());
                return;
            };
            calls(
                reply,
                vec![call(
                    EXTRACT_CALL,
                    "file_extract",
                    map(vec![("path", Value::from(document.as_str()))]),
                )],
            );
        }
        Kind::Query => {
            conversation.topic = Some(prompt.to_string());
            calls(reply, vec![call(SCHEMA_CALL, "graph_schema", map(vec![]))]);
        }
        Kind::Diagram => {
            // the diagram assistant drafts the code and validates it before answering
            let title: String = prompt.chars().take(40).collect();
            calls(
                reply,
                vec![call(
                    RENDER_CALL,
                    "render_diagram",
                    map(vec![
                        ("diagramCode", Value::from(DIAGRAM_CODE)),
                        ("title", Value::from(title)),
                    ]),
                )],
            );
        }
        Kind::Order => {
            static ORDER_ID: OnceLock<Regex> = OnceLock::new();
            match regex(&ORDER_ID, r"ORD-\d+").find(prompt) {
                Some(found) => {
                    let id = found.as_str().to_string();
                    let arguments = || map(vec![("order_id", Value::from(id.as_str()))]);
                    conversation.order_id = Some(id.clone());
                    calls(
                        reply,
                        vec![
                            call(LOOKUP_CALL, "order_lookup", arguments()),
                            call(SHIPMENT_CALL, "shipment_status", arguments()),
                        ],
                    );
                }
                None => done(
                    reply,
                    "Please give me your order number. It starts with ORD-.".to_string(),
                ),
            }
        }
    }
}

fn later_turn(conversation: &mut Conversation, reply: &mut Vec<(&'static str, Value)>) {
    let id = conversation.order_id.clone().unwrap_or_default();
    if let Some(error) = &conversation.order_error {
        done(reply, format!("I could not find order {id}: {error}."));
    } else if let Some(error) = &conversation.shipment_error {
        done(
            reply,
            format!("I found order {id} but not its shipment: {error}."),
        );
    } else if let Some(refund) = &conversation.refund {
        done(
            reply,
            format!(
                "I have issued refund {} of {} {} for order {id} because the carrier lost the shipment. \
                 It reaches your statement within five business days.",
                text_of(field(refund, "refund_id")),
                number_text(field(refund, "amount")),
                text_of(field(refund, "currency"))
            ),
        );
    } else if let Some(error) = &conversation.refund_error {
        done(
            reply,
            format!("I could not issue the refund for order {id}: {error}."),
        );
    } else {
        let shipment = conversation.shipment.clone().unwrap_or(Value::Nil);
        let status = text_of(field(&shipment, STATUS));
        if status == "lost" {
            let total = conversation
                .order
                .as_ref()
                .and_then(|order| field(order, "total").cloned())
                .unwrap_or(Value::Nil);
            calls(
                reply,
                vec![call(
                    REFUND_CALL,
                    "issue_refund",
                    map(vec![
                        ("order_id", Value::from(id.as_str())),
                        ("amount", total),
                        (
                            "reason",
                            Value::from(format!("The carrier lost the shipment of order {id}")),
                        ),
                    ]),
                )],
            );
        } else {
            done(
                reply,
                format!(
                    "Your order {id} is {status}. {}",
                    text_of(field(&shipment, "note"))
                ),
            );
        }
    }
}

/// The notes assistant: the crawl's pages become a notes file, written under the topic's slug; the write waits.
fn later_notes_turn(conversation: &mut Conversation, reply: &mut Vec<(&'static str, Value)>) {
    let section = conversation.section.clone().unwrap_or_default();
    let topic = conversation.topic.clone().unwrap_or_default();
    if let Some(error) = &conversation.crawl_error {
        done(
            reply,
            format!("I could not read the section {section}: {error}."),
        );
    } else if let Some(written) = &conversation.written {
        done(
            reply,
            format!(
                "I wrote the notes on {topic} to {} ({} bytes) after reading {} pages of {section}. \
                 They cover the configuration keys and the runtime overrides.",
                text_of(field(written, "path")),
                text_of(field(written, "bytes")),
                conversation.pages.len()
            ),
        );
    } else if let Some(error) = &conversation.write_error {
        done(
            reply,
            format!(
                "I could not write the notes: {error}. In short, the section covers the configuration keys \
                 and the -D overrides."
            ),
        );
    } else {
        let mut notes = format!("# Notes on {topic}\n");
        for page in &conversation.pages {
            notes.push_str(&format!(
                "\n## {}\n\n{}\n\nSource: {}\n",
                text_of(field(page, "title")),
                text_of(field(page, "text")),
                text_of(field(page, "url"))
            ));
        }
        calls(
            reply,
            vec![call(
                WRITE_CALL,
                "file_write",
                map(vec![
                    ("path", Value::from(format!("notes/{}.md", slug(&topic)))),
                    ("content", Value::from(notes)),
                ]),
            )],
        );
    }
}

/// The analyst: the document's facts, extracted by the corpus's shape, become one idempotent ingest.
fn later_ingest_turn(conversation: &mut Conversation, reply: &mut Vec<(&'static str, Value)>) {
    let document = conversation.document.clone().unwrap_or_default();
    if let Some(error) = &conversation.extract_error {
        done(reply, format!("I could not read {document}: {error}."));
    } else if let Some(ingested) = &conversation.ingested {
        let counts = &conversation.counts;
        done(
            reply,
            format!(
                "Ingested {} nodes and {} relationships from {document}: {} parties, {} obligations, {} rules \
                 and {} deadlines, each with the document as its source.",
                text_of(field(ingested, "nodes")),
                text_of(field(ingested, "relationships")),
                counts.parties,
                counts.obligations,
                counts.rules,
                counts.deadlines
            ),
        );
    } else if let Some(error) = &conversation.ingest_error {
        done(reply, format!("I could not ingest {document}: {error}."));
    } else {
        let ingest = extraction(conversation);
        calls(reply, vec![call(INGEST_CALL, "graph_ingest", ingest)]);
    }
}

/// The answerer: the schema first, then one read-only query for the label the question names, then
/// the answer with sources.
fn later_query_turn(conversation: &mut Conversation, reply: &mut Vec<(&'static str, Value)>) {
    if let Some(error) = &conversation.schema_error {
        done(
            reply,
            format!("I could not read the graph's schema: {error}."),
        );
    } else if let Some(rows) = &conversation.rows {
        if rows.is_empty() {
            done(
                reply,
                "I found nothing about that in the knowledge graph.".to_string(),
            );
            return;
        }
        let mut answer = String::from("From the knowledge graph:");
        for row in rows {
            let node = at(row, &["0"]);
            let properties = at(node, &["properties"]);
            if matches!(properties, Value::Map(_)) {
                // a fact has a text; a party a name; a document a title
                let fact = ["text", "name", "title"]
                    .iter()
                    .map(|key| field(properties, key))
                    .find(|value| matches!(value, Some(value) if !value.is_nil()))
                    .flatten();
                answer.push_str(&format!(
                    "\n- {} (source: {})",
                    text_of(fact),
                    text_of(field(properties, "source"))
                ));
            }
        }
        done(reply, answer);
    } else if let Some(error) = &conversation.query_error {
        done(reply, format!("The query failed: {error}."));
    } else if matches!(
        conversation.schema.as_ref().and_then(|schema| field(schema, "labels")),
        Some(Value::Array(labels)) if labels.is_empty()
    ) {
        done(
            reply,
            "The knowledge graph holds nothing yet. Ingest a document first.".to_string(),
        );
    } else {
        let question = conversation
            .topic
            .clone()
            .unwrap_or_default()
            .to_lowercase();
        let label = if question.contains("deadline") {
            "Deadline"
        } else if question.contains("rule") {
            "Rule"
        } else if question.contains("part") {
            "Party"
        } else {
            "Obligation"
        };
        calls(
            reply,
            vec![call(
                QUERY_CALL,
                "graph_query",
                map(vec![
                    (
                        "query",
                        Value::from(format!("MATCH (n:{label}) RETURN n ORDER BY n.key")),
                    ),
                    ("parameters", map(vec![])),
                ]),
            )],
        );
    }
}

fn node(key: &str, label: &str, properties: Value) -> Value {
    map(vec![
        ("key", Value::from(key)),
        ("label", Value::from(label)),
        ("properties", properties),
    ])
}

fn relationship(from: &str, to: &str, kind: &str) -> Value {
    map(vec![
        ("from", Value::from(from)),
        ("to", Value::from(to)),
        ("type", Value::from(kind)),
    ])
}

/// The corpus's shape as an ingest: the document, its parties, and one node per obligation, rule and deadline.
fn extraction(conversation: &mut Conversation) -> Value {
    static HEADING: OnceLock<Regex> = OnceLock::new();
    static PARTIES: OnceLock<Regex> = OnceLock::new();
    static PARTY: OnceLock<Regex> = OnceLock::new();
    static FACT: OnceLock<Regex> = OnceLock::new();
    static EXTENSION: OnceLock<Regex> = OnceLock::new();
    let text = conversation.extracted.clone().unwrap_or_default();
    let path = conversation.document.clone().unwrap_or_default();
    let document_key = format!("document:{path}");
    let title = regex(&HEADING, r"(?m)^# (.+)$")
        .captures(&text)
        .map(|capture| capture[1].trim().to_string())
        .unwrap_or_else(|| path.clone());
    let mut nodes = vec![node(
        &document_key,
        "Document",
        map(vec![
            ("title", Value::from(title)),
            ("path", Value::from(path.as_str())),
        ]),
    )];
    let mut relationships = Vec::new();
    let mut counts = Counts::default();
    if let Some(line) = regex(&PARTIES, r"\*\*Parties:\*\*\s*(.+)").captures(&text) {
        let parties = line[1].replace(" and ", ", ");
        for party in regex(&PARTY, r"([^,]+?)\s*\(([^)]+)\)").captures_iter(&parties) {
            let name = party[1].trim().to_string();
            let key = format!("party:{}", slug(&name));
            nodes.push(node(
                &key,
                "Party",
                map(vec![
                    ("name", Value::from(name)),
                    ("role", Value::from(party[2].trim())),
                ]),
            ));
            relationships.push(relationship(&key, &document_key, "PARTY_TO"));
            counts.parties += 1;
        }
    }
    let stem = slug(&regex(&EXTENSION, r"\.[a-z]+$").replace(&path, ""));
    for fact in
        regex(&FACT, r"(?m)^- (OBL-\d+|RULE-\d+|\d{4}-\d{2}-\d{2}): (.+)$").captures_iter(&text)
    {
        let id = &fact[1];
        let label = if id.starts_with("OBL") {
            "Obligation"
        } else if id.starts_with("RULE") {
            "Rule"
        } else {
            "Deadline"
        };
        let key = format!("{}:{stem}-{}", label.to_lowercase(), slug(id));
        let mut properties = vec![("text", Value::from(fact[2].trim()))];
        if label == "Deadline" {
            properties.push(("date", Value::from(id)));
        } else {
            properties.push(("ref", Value::from(id)));
        }
        nodes.push(node(&key, label, map(properties)));
        relationships.push(relationship(&key, &document_key, "STATED_IN"));
        match label {
            "Obligation" => counts.obligations += 1,
            "Rule" => counts.rules += 1,
            _ => counts.deadlines += 1,
        }
    }
    conversation.counts = counts;
    map(vec![
        ("nodes", Value::Array(nodes)),
        ("relationships", Value::Array(relationships)),
        ("source", Value::from(path)),
    ])
}

fn later_diagram_turn(conversation: &mut Conversation, reply: &mut Vec<(&'static str, Value)>) {
    if let Some(error) = &conversation.render_error {
        done(reply, format!("I could not validate the diagram: {error}."));
    } else if let Some(render) = conversation
        .render
        .as_ref()
        .filter(|render| field(render, "valid").and_then(Value::as_bool) == Some(true))
    {
        done(
            reply,
            format!(
                "Here is the validated diagram:\n```mermaid\n{}\n```\nIt shows the steps from the request to \
                 the decision. The rendered image is stored under the reference {}.",
                text_of(field(render, "diagramCode")),
                text_of(field(render, "reference"))
            ),
        );
    } else {
        done(
            reply,
            "The renderer reported the diagram invalid and I could not fix it.".to_string(),
        );
    }
}

fn remember(conversation: &mut Conversation, results: &[Value]) {
    for result in results {
        let id = text_of(field(result, "id"));
        let output = field(result, "output")
            .filter(|value| matches!(value, Value::Map(_)))
            .cloned();
        let error = field(result, "error")
            .filter(|value| !value.is_nil())
            .map(|value| text_of(Some(value)));
        match id.as_str() {
            RENDER_CALL => {
                conversation.render = output;
                conversation.render_error = error;
            }
            LOOKUP_CALL => {
                conversation.order = output;
                conversation.order_error = error;
            }
            SHIPMENT_CALL => {
                conversation.shipment = output;
                conversation.shipment_error = error;
            }
            REFUND_CALL => {
                conversation.refund = output;
                conversation.refund_error = error;
            }
            CRAWL_CALL => {
                conversation.pages = match output.as_ref().and_then(|out| field(out, "pages")) {
                    Some(Value::Array(pages)) => pages.clone(),
                    _ => Vec::new(),
                };
                conversation.crawl_error = error;
            }
            WRITE_CALL => {
                conversation.written = output;
                conversation.write_error = error;
            }
            EXTRACT_CALL => {
                conversation.extracted = output.as_ref().map(|out| text_of(field(out, "text")));
                conversation.extract_error = error;
            }
            INGEST_CALL => {
                conversation.ingested = output;
                conversation.ingest_error = error;
            }
            SCHEMA_CALL => {
                conversation.schema = output;
                conversation.schema_error = error;
            }
            QUERY_CALL => {
                conversation.rows =
                    Some(match output.as_ref().and_then(|out| field(out, "rows")) {
                        Some(Value::Array(rows)) => rows.clone(),
                        _ => Vec::new(),
                    });
                conversation.query_error = error;
            }
            other => panic!("Unexpected call id {other}"),
        }
    }
}

// ---- the stand-in for the built-in agent tools' routes ----

/// One of the tool routes, registered under its own name: the stand-in keeps the tools' result
/// shapes and refusals - the allowlist, the workspace, a second `create` as a 409, an in-memory graph.
pub struct AgentToolsDouble {
    pub route: &'static str,
}

/// The writes `file.write` was asked for: workspace path to content.
pub fn writes() -> &'static Mutex<HashMap<String, String>> {
    static WRITES: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
    WRITES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The in-memory graph: node key to `{label, properties}`.
pub fn nodes() -> &'static Mutex<BTreeMap<String, Value>> {
    static NODES: OnceLock<Mutex<BTreeMap<String, Value>>> = OnceLock::new();
    NODES.get_or_init(|| Mutex::new(BTreeMap::new()))
}

/// The in-memory graph's relationships.
pub fn relationships() -> &'static Mutex<Vec<Value>> {
    static RELATIONSHIPS: OnceLock<Mutex<Vec<Value>>> = OnceLock::new();
    RELATIONSHIPS.get_or_init(|| Mutex::new(Vec::new()))
}

/// Every call, as route names in order.
pub fn calls_made() -> &'static Mutex<Vec<String>> {
    static CALLS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    CALLS.get_or_init(|| Mutex::new(Vec::new()))
}

/// Forget every write, node and relationship - a scenario's clean slate.
pub fn reset_tools() {
    writes().lock().expect("writes").clear();
    nodes().lock().expect("nodes").clear();
    relationships().lock().expect("relationships").clear();
    calls_made().lock().expect("calls").clear();
}

#[async_trait]
impl ComposableFunction for AgentToolsDouble {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        calls_made()
            .lock()
            .expect("calls")
            .push(self.route.to_string());
        let body = input.body();
        let result = match self.route {
            "web.crawl" => crawl(body)?,
            "web.fetch" => fetch(body)?,
            "file.write" => write(body)?,
            "file.extract" => extract(body)?,
            "graph.ingest" => ingest(body)?,
            "graph.schema" => schema(),
            "graph.query" => query(body)?,
            other => return Err(AppError::new(404, format!("Route {other} not found"))),
        };
        Ok(EventEnvelope::new().set_raw_body(result))
    }
}

fn host_of(url: &str) -> Option<&str> {
    let rest = url.split("://").nth(1)?;
    let host = rest.split('/').next()?;
    let host = host.split('@').next_back()?;
    Some(host.split(':').next().unwrap_or(host))
}

fn allowed_url(input: &Value) -> Result<String, AppError> {
    let url = required_text(input, "url")?;
    let url = url.trim().to_string();
    let allowed = host_of(SITE).expect("the site has a host");
    match host_of(&url) {
        Some(host) if host == allowed => Ok(url),
        other => Err(AppError::new(
            400,
            format!(
                "{} is not one of the sites this agent may reach (tools.web.allow)",
                other.unwrap_or(url.as_str())
            ),
        )),
    }
}

fn page(url: &str, title: &str, text: &str) -> Value {
    map(vec![
        ("url", Value::from(url)),
        ("title", Value::from(title)),
        ("text", Value::from(text)),
        ("bytes", Value::from((text.len() * 3) as u64)),
    ])
}

fn crawl(input: &Value) -> Result<Value, AppError> {
    let url = allowed_url(input)?;
    let pages = vec![
        page(
            &url,
            "Configuration",
            "The configuration keys of the platform: application.name names the application, \
             rest.server.port the port, and log.format the log presentation (text, json or compact). \
             A value may carry ${ENV_VAR:default} references, resolved at load.",
        ),
        page(
            &format!("{url}overrides/"),
            "Runtime overrides",
            "Any key can be overridden at run time with -Dkey=value, and a -D value is substituted like \
             a file value, so a credential travels as -Dkey=${SECRET}.",
        ),
    ];
    let visited = pages.len() as u64;
    Ok(map(vec![
        ("pages", Value::Array(pages)),
        ("visited", Value::from(visited)),
        ("truncated", Value::Boolean(false)),
    ]))
}

fn fetch(input: &Value) -> Result<Value, AppError> {
    let url = allowed_url(input)?;
    Ok(map(vec![
        ("url", Value::from(url)),
        ("status", Value::from(200)),
        ("content_type", Value::from("text/html")),
        ("title", Value::from("Configuration")),
        (
            "text",
            Value::from("The configuration keys of the platform, one page."),
        ),
        ("bytes", Value::from(512)),
        ("truncated", Value::Boolean(false)),
    ]))
}

fn workspace_path(input: &Value) -> Result<String, AppError> {
    let path = required_text(input, "path")?;
    if path.starts_with('/') || path.contains("..") {
        return Err(AppError::new(400, "path is outside the workspace"));
    }
    Ok(path)
}

fn write(input: &Value) -> Result<Value, AppError> {
    let path = workspace_path(input)?;
    let content = required_text(input, "content")?;
    let mode = string(input, "mode").unwrap_or_else(|| "create".to_string());
    let mut writes = writes().lock().expect("writes");
    if mode == "create" && writes.contains_key(&path) {
        return Err(AppError::new(
            409,
            format!("{path} exists - mode overwrite replaces it, append extends it"),
        ));
    }
    let bytes = content.len() as u64;
    match writes.get_mut(&path) {
        Some(existing) if mode == "append" => existing.push_str(&content),
        _ => {
            writes.insert(path.clone(), content);
        }
    }
    Ok(map(vec![
        ("path", Value::from(path)),
        ("bytes", Value::from(bytes)),
        ("mode", Value::from(mode)),
    ]))
}

fn extract(input: &Value) -> Result<Value, AppError> {
    let path = workspace_path(input)?;
    let file = std::path::Path::new(CORPUS).join(&path);
    if !file.is_file() {
        return Err(AppError::new(404, format!("file not found: {path}")));
    }
    let content = std::fs::read_to_string(&file)
        .map_err(|e| AppError::new(400, format!("{path} could not be read ({e})")))?;
    Ok(map(vec![
        ("path", Value::from(path)),
        ("type", Value::from("text")),
        ("pages", Value::from(1)),
        (
            "text",
            Value::from(format!("--- page 1 ---\n{}", content.trim())),
        ),
        ("truncated", Value::Boolean(false)),
    ]))
}

fn array<'a>(body: &'a Value, key: &str) -> &'a [Value] {
    match field(body, key) {
        Some(Value::Array(items)) => items,
        _ => &[],
    }
}

fn ingest(input: &Value) -> Result<Value, AppError> {
    static IDENTIFIER: OnceLock<Regex> = OnceLock::new();
    let nodes_given = array(input, "nodes");
    let relationships_given = array(input, "relationships");
    if nodes_given.is_empty() && relationships_given.is_empty() {
        return Err(AppError::new(
            400,
            "nothing to ingest - give nodes, relationships or both",
        ));
    }
    let source = field(input, "source")
        .filter(|value| !value.is_nil())
        .cloned();
    let mut graph = nodes().lock().expect("nodes");
    for given in nodes_given {
        let label = text_of(field(given, "label"));
        if !regex(&IDENTIFIER, r"^[A-Za-z][A-Za-z0-9_]{0,63}$").is_match(&label) {
            return Err(AppError::new(
                400,
                "label must be an identifier - letters, digits and underscore, starting with a letter",
            ));
        }
        let key = text_of(field(given, "key"));
        let mut properties: Vec<(Value, Value)> = match field(given, "properties") {
            Some(Value::Map(entries)) => entries.clone(),
            _ => Vec::new(),
        };
        properties.push((Value::from("key"), Value::from(key.as_str())));
        if let Some(source) = &source {
            properties.push((Value::from("source"), source.clone()));
        }
        graph.insert(
            key,
            map(vec![
                ("label", Value::from(label)),
                ("properties", Value::Map(properties)),
            ]),
        );
    }
    let mut stored_relationships = relationships().lock().expect("relationships");
    for given in relationships_given {
        for end in ["from", "to"] {
            let key = text_of(field(given, end));
            if !graph.contains_key(&key) {
                return Err(AppError::new(
                    400,
                    format!("relationship endpoints not found in the graph or in this call: {key}"),
                ));
            }
        }
        let mut stored = match given {
            Value::Map(entries) => entries.clone(),
            _ => Vec::new(),
        };
        if let Some(source) = &source {
            stored.push((Value::from("source"), source.clone()));
        }
        stored_relationships.push(Value::Map(stored));
    }
    Ok(map(vec![
        ("nodes", Value::from(nodes_given.len() as u64)),
        (
            "relationships",
            Value::from(relationships_given.len() as u64),
        ),
    ]))
}

fn label_of(node: &Value) -> String {
    text_of(field(node, "label"))
}

fn schema() -> Value {
    let graph = nodes().lock().expect("nodes");
    let mut labels: BTreeMap<String, (u64, Vec<String>)> = BTreeMap::new();
    for stored in graph.values() {
        let entry = labels.entry(label_of(stored)).or_default();
        entry.0 += 1;
        if let Some(Value::Map(properties)) = field(stored, "properties") {
            for (name, _) in properties {
                let name = text_of(Some(name));
                if !entry.1.contains(&name) {
                    entry.1.push(name);
                }
            }
        }
    }
    let mut kinds: BTreeMap<String, (u64, Vec<String>, Vec<String>)> = BTreeMap::new();
    for relationship in relationships().lock().expect("relationships").iter() {
        let entry = kinds
            .entry(text_of(field(relationship, "type")))
            .or_default();
        entry.0 += 1;
        for (end, list) in [("from", 0usize), ("to", 1usize)] {
            let key = text_of(field(relationship, end));
            if let Some(node) = graph.get(&key) {
                let label = label_of(node);
                let labels = if list == 0 {
                    &mut entry.1
                } else {
                    &mut entry.2
                };
                if !labels.contains(&label) {
                    labels.push(label);
                }
            }
        }
    }
    let labels = labels
        .into_iter()
        .map(|(label, (count, names))| {
            map(vec![
                ("label", Value::from(label)),
                ("count", Value::from(count)),
                (
                    "properties",
                    Value::Array(
                        names
                            .into_iter()
                            .map(|name| {
                                map(vec![
                                    ("name", Value::from(name)),
                                    ("types", Value::Array(vec![Value::from("String")])),
                                ])
                            })
                            .collect(),
                    ),
                ),
            ])
        })
        .collect();
    let kinds = kinds
        .into_iter()
        .map(|(kind, (count, from, to))| {
            map(vec![
                ("type", Value::from(kind)),
                ("count", Value::from(count)),
                (
                    "from",
                    Value::Array(from.into_iter().map(Value::from).collect()),
                ),
                (
                    "to",
                    Value::Array(to.into_iter().map(Value::from).collect()),
                ),
                ("properties", Value::Array(Vec::new())),
            ])
        })
        .collect();
    map(vec![
        ("labels", Value::Array(labels)),
        ("relationship_types", Value::Array(kinds)),
    ])
}

fn query(input: &Value) -> Result<Value, AppError> {
    static WRITE_CLAUSE: OnceLock<Regex> = OnceLock::new();
    static LABEL_IN_QUERY: OnceLock<Regex> = OnceLock::new();
    let query = required_text(input, "query")?;
    if let Some(clause) = regex(
        &WRITE_CLAUSE,
        r"(?i)\b(CREATE|MERGE|SET|DELETE|REMOVE|DROP|LOAD\s+CSV)\b",
    )
    .captures(&query)
    {
        return Err(AppError::new(
            400,
            format!(
                "query carries {}, a write clause - graph_query is read-only; graph_write or graph_ingest writes",
                clause[1].to_uppercase()
            ),
        ));
    }
    let mut rows = Vec::new();
    if let Some(label) = regex(&LABEL_IN_QUERY, ":`?([A-Za-z][A-Za-z0-9_]*)`?").captures(&query) {
        let wanted = &label[1];
        for (key, node) in nodes().lock().expect("nodes").iter() {
            if label_of(node) == wanted {
                rows.push(Value::Array(vec![map(vec![
                    ("id", Value::from(format!("4:test:{key}"))),
                    ("labels", Value::Array(vec![Value::from(wanted)])),
                    (
                        "properties",
                        field(node, "properties").cloned().unwrap_or(Value::Nil),
                    ),
                ])]));
            }
        }
    }
    Ok(map(vec![
        ("columns", Value::Array(vec![Value::from("n")])),
        ("rows", Value::Array(rows)),
        ("truncated", Value::Boolean(false)),
    ]))
}

// ---- the stand-in for mermaid.render ----

/// The route `mermaid.render` of a pack's MCP binding: the shape the real server answers - the
/// title, the code, `valid`, and the rendered image when valid, or an error when the code does not
/// start like a Mermaid diagram.
pub struct MermaidDouble;

#[async_trait]
impl ComposableFunction for MermaidDouble {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        const HEADERS: [&str; 5] = [
            "flowchart",
            "graph",
            "sequenceDiagram",
            "classDiagram",
            "stateDiagram",
        ];
        let body = input.body();
        let code = required_text(body, "diagramCode")?;
        let valid = HEADERS
            .iter()
            .any(|header| code.trim_start().starts_with(header));
        let mut result = vec![
            (
                "title",
                field(body, "title")
                    .filter(|value| !value.is_nil())
                    .cloned()
                    .unwrap_or_else(|| Value::from("")),
            ),
            ("diagramCode", Value::from(code.as_str())),
            ("valid", Value::Boolean(valid)),
        ];
        if valid {
            result.push((
                "rawSVG",
                Value::from(
                    "<svg xmlns=\"http://www.w3.org/2000/svg\"><text>mock render</text></svg>",
                ),
            ));
        } else {
            result.push((
                "error",
                Value::from("Parse error on line 1: the diagram must start with its type"),
            ));
        }
        Ok(EventEnvelope::new().set_raw_body(map(result)))
    }
}

/// Register every stand-in on the platform: the scripted model, the seven tool routes and the
/// Mermaid binding's route - private functions, reached locally like the pack's would be over
/// Event-over-HTTP.
pub fn register_doubles(platform: &Platform) {
    platform
        .register_private("llm.chat", Arc::new(ScriptedModel), 10)
        .expect("llm.chat");
    for route in TOOL_ROUTES {
        platform
            .register_private(route, Arc::new(AgentToolsDouble { route }), 10)
            .expect(route);
    }
    platform
        .register_private("mermaid.render", Arc::new(MermaidDouble), 10)
        .expect("mermaid.render");
}
