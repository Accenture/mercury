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

//! The dev-only Playground walker — Rust port of `GraphTraveler`
//! (`graph.traveler`, zero-tracing interceptor, dev-gated). The `run`
//! command launches it against a session's live graph instance: it walks
//! like the executor but narrates every step to the console (`Walk to X`,
//! `Executed X with skill Y in T ms`) and is idempotent — the operator may
//! run it repeatedly (node bookkeeping and output reset per run).

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use event_script::conversions::display;
use platform_core::graph::SimpleNode;
use platform_core::{AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::common::{self, get_error_map, invalid, EXCEPTION, SINK, SKILL};
use crate::executor::RETIRED_JS_MESSAGE;
use crate::model::GraphInstance;
use crate::session;

pub const ROUTE: &str = "graph.traveler";
const NEXT: &str = "next";
const MAX_BUFFER_SIZE: usize = 62 * 1024;

/// The interceptor body (Java `handleEvent`).
pub async fn handle(
    platform: &Platform,
    headers: HashMap<String, String>,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let po = PostOffice::new(platform);
    if let Some(cid) = event.correlation_id() {
        if cid.contains('@') {
            handle_skill_response(platform, &po, &event).await;
        } else if event.reply_to().is_some() {
            execute_graph(platform, &po, &headers, &event).await;
        }
    }
    EventEnvelope::new().set_body("ignored")
}

async fn execute_graph(
    platform: &Platform,
    po: &PostOffice,
    headers: &HashMap<String, String>,
    event: &EventEnvelope,
) {
    let reply_to = event.reply_to().unwrap_or_default().to_string();
    let cid = event.correlation_id().unwrap_or_default().to_string();
    let outcome = begin(platform, po, headers, &reply_to, &cid).await;
    if let Err(e) = outcome {
        let _ = po
            .send(
                EventEnvelope::new()
                    .set_to(&reply_to)
                    .set_status(e.status())
                    .set_raw_body(Value::from(e.message()))
                    .set_correlation_id(&cid),
            )
            .await;
        // Uniform end-of-transmission even when the traversal fails before it
        // starts (no graph instance yet, missing root/end) — no `GraphInstance`
        // exists here, so emit the terminal line directly to the reply route.
        let _ = po
            .send(
                EventEnvelope::new()
                    .set_to(&reply_to)
                    .set_status(400)
                    .set_raw_body(Value::from("Graph traversal aborted"))
                    .set_correlation_id(&cid),
            )
            .await;
    }
}

async fn begin(
    platform: &Platform,
    po: &PostOffice,
    headers: &HashMap<String, String>,
    reply_to: &str,
    cid: &str,
) -> Result<(), AppError> {
    let in_route = headers
        .get("in")
        .ok_or_else(|| invalid("Missing instance ID in header"))?;
    let instance = common::get_graph_instance(in_route)?;
    instance.set_flow_instance_id(in_route);
    instance.set_correlation_id(cid);
    instance.set_reply_to(reply_to);
    instance.node_seen.lock().expect("node seen").clear();
    instance.skill_run.lock().expect("skill run").clear();
    instance
        .complete
        .store(false, std::sync::atomic::Ordering::SeqCst);
    instance.reset_start_time();
    // clean output for idempotent behavior — the traveler may run repeatedly
    {
        let mut state = instance.state.lock().expect("graph state machine");
        state
            .set_element("output", Value::Map(vec![]))
            .map_err(invalid)?;
    }
    let root = instance
        .graph
        .get_root_node()
        .ok_or_else(|| invalid("Root node does not exist"))?;
    instance
        .graph
        .get_end_node()
        .ok_or_else(|| invalid("End node does not exist"))?;
    walk(platform, po, &instance, root).await
}

async fn handle_skill_response(platform: &Platform, po: &PostOffice, response: &EventEnvelope) {
    let composite = response.correlation_id().unwrap_or_default();
    let Some(at) = composite.find('@') else {
        return;
    };
    let in_route = &composite[..at];
    let node_name = &composite[at + 1..];
    let Some(instance) = crate::model::get_instance(in_route) else {
        return;
    };
    let target = {
        let state = instance.state.lock().expect("graph state machine");
        state.get_element(&format!("{node_name}.target"))
    };
    if response.has_error() {
        if target.is_some() {
            let mut state = instance.state.lock().expect("graph state machine");
            let error_map = get_error_map(state.get_element("output.body"), target);
            let _ = state.set_element("output.body", error_map);
        }
        handle_error_response(po, &instance, response).await;
        return;
    }
    let Ok(Some(node)) = instance.graph.find_node_by_alias(node_name) else {
        return;
    };
    check_frequency(po, &instance, node_name).await;
    // advise the operator that the node has been executed
    let skill = node
        .get_property(SKILL)
        .map(|v| display(&v))
        .unwrap_or_default();
    let spent = response.exec_time().unwrap_or(0.0);
    let reply_to = instance.get_reply_to();
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(&reply_to)
                .set_raw_body(Value::from(format!(
                    "Executed {node_name} with skill {skill} in {spent} ms"
                ))),
        )
        .await;
    let (process_status, result_error) = {
        let state = instance.state.lock().expect("graph state machine");
        (
            state.get_element(&format!("{node_name}.status")),
            state.get_element(&format!("{node_name}.error")),
        )
    };
    // mark the skill complete only when it did NOT fail (status + error set,
    // e.g. an exception-routed fetcher): a join barrier counts skill_run, so
    // a failed branch must not satisfy the barrier while it retries.
    // GraphExecutor (deployed graphs) keeps identical semantics.
    if !matches!(
        (&process_status, &result_error),
        (Some(Value::Integer(_)), Some(_))
    ) {
        instance
            .skill_run
            .lock()
            .expect("skill run")
            .insert(node_name.to_string(), true);
    }
    let error_handler = node.get_property(EXCEPTION);
    if let (Some(Value::Integer(rc)), Some(error), None) =
        (&process_status, &result_error, &error_handler)
    {
        let error_map = get_error_map(Some(error.clone()), target);
        let _ = po
            .send(
                EventEnvelope::new()
                    .set_to(&reply_to)
                    .set_correlation_id(&instance.get_correlation_id())
                    .set_raw_body(error_map)
                    .set_status(rc.as_i64().unwrap_or(500) as i32),
            )
            .await;
        emit_aborted(po, &instance).await;
    } else if !instance.is_complete() {
        let next = display(response.body());
        decide_next(platform, po, &instance, node, &next).await;
    }
}

async fn check_frequency(po: &PostOffice, instance: &Arc<GraphInstance>, node_name: &str) {
    let now = session::now_ms();
    let (total, last) = {
        let mut hits = instance.hits.lock().expect("visit counters");
        let entry = hits.entry(node_name.to_string()).or_default();
        if now - entry.last_visit > common::loop_interval() {
            entry.last_visit = now;
            entry.hits = 0;
        }
        entry.hits += 1;
        (entry.hits, entry.last_visit)
    };
    if total > common::high_frequency() {
        log::error!(
            "Looping detected - {total} hits in {} ms for {node_name} in {}",
            now - last,
            instance.graph_id
        );
        let response = EventEnvelope::new()
            .set_raw_body(Value::from(format!(
                "Node {node_name} executed too frequently"
            )))
            .set_status(400);
        handle_error_response(po, instance, &response).await;
    }
}

async fn decide_next(
    platform: &Platform,
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    node: Arc<SimpleNode>,
    next: &str,
) {
    let is_end = instance
        .graph
        .get_end_node()
        .map(|end| end.get_id() == node.get_id())
        .unwrap_or(false);
    if is_end {
        execution_complete(po, instance).await;
    } else if next != SINK {
        if next == NEXT {
            let _ = walk_next(platform, po, instance, &node).await;
        } else {
            match instance.graph.find_node_by_alias(next) {
                Ok(Some(next_node)) => {
                    let _ = walk(platform, po, instance, next_node).await;
                }
                _ => {
                    send_error(po, instance, &format!("Next node '{next}' does not exist")).await;
                }
            }
        }
    }
}

fn walk<'a>(
    platform: &'a Platform,
    po: &'a PostOffice,
    instance: &'a Arc<GraphInstance>,
    node: Arc<SimpleNode>,
) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'a>> {
    Box::pin(async move {
        if instance.is_complete() {
            return Ok(());
        }
        let node_name = node.get_alias().to_string();
        let skill = node.get_property(SKILL).map(|v| display(&v));
        let seen = skill.as_deref() != Some(crate::skills::JOIN_ROUTE)
            && instance
                .node_seen
                .lock()
                .expect("node seen")
                .contains_key(&node_name);
        if !seen {
            instance
                .node_seen
                .lock()
                .expect("node seen")
                .insert(node_name.clone(), true);
            let _ = po
                .send(
                    EventEnvelope::new()
                        .set_to(&instance.get_reply_to())
                        .set_raw_body(Value::from(format!("Walk to {node_name}"))),
                )
                .await;
            walk_to(platform, po, skill, instance, node).await?;
        }
        Ok(())
    })
}

async fn walk_to(
    platform: &Platform,
    po: &PostOffice,
    skill: Option<String>,
    instance: &Arc<GraphInstance>,
    node: Arc<SimpleNode>,
) -> Result<(), AppError> {
    let is_end = instance
        .graph
        .get_end_node()
        .map(|end| end.get_id() == node.get_id())
        .unwrap_or(false);
    match skill {
        Some(skill) => execute_skill(platform, po, &skill, instance, &node).await,
        None if is_end => {
            execution_complete(po, instance).await;
            Ok(())
        }
        None => walk_next(platform, po, instance, &node).await,
    }
}

async fn execution_complete(po: &PostOffice, instance: &Arc<GraphInstance>) {
    let in_route = instance.get_flow_instance_id();
    let out = instance.get_reply_to();
    let value = {
        let state = instance.state.lock().expect("graph state machine");
        state.get_element("output").unwrap_or(Value::Map(vec![]))
    };
    if matches!(value, Value::Map(_) | Value::Array(_)) {
        let text = event_script::conversions::to_json_string(&value);
        if text.len() > MAX_BUFFER_SIZE {
            let name = session::temp_graph_name(&in_route);
            let _ = po
                .send(
                    EventEnvelope::new()
                        .set_to(&out)
                        .set_raw_body(Value::from(format!(
                            "Large payload ({}) -> GET /api/inspect/{name}/output",
                            text.len()
                        ))),
                )
                .await;
        } else {
            let _ = po
                .send(
                    EventEnvelope::new()
                        .set_to(&out)
                        .set_raw_body(Value::Map(vec![(Value::from("output"), value)])),
                )
                .await;
        }
    } else {
        let _ = po
            .send(
                EventEnvelope::new()
                    .set_to(&out)
                    .set_raw_body(Value::Map(vec![(Value::from("output"), value)])),
            )
            .await;
    }
    instance.set_complete();
    let elapsed = session::now_ms() - instance.start_time_ms();
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(&out)
                .set_raw_body(Value::from(format!(
                    "Graph traversal completed in {elapsed} ms"
                ))),
        )
        .await;
}

async fn execute_skill(
    platform: &Platform,
    po: &PostOffice,
    skill: &str,
    instance: &Arc<GraphInstance>,
    node: &Arc<SimpleNode>,
) -> Result<(), AppError> {
    if skill == "graph.js" {
        send_error(po, instance, RETIRED_JS_MESSAGE).await;
        return Ok(());
    }
    if platform.has_route(skill) {
        let in_route = instance.get_flow_instance_id();
        let node_name = node.get_alias();
        let composite = format!("{in_route}@{node_name}");
        po.send(
            EventEnvelope::new()
                .set_to(skill)
                .set_header("in", &in_route)
                .set_header("type", "execute")
                .set_header("node", node_name)
                .set_reply_to(ROUTE)
                .set_correlation_id(&composite),
        )
        .await
    } else {
        send_error(po, instance, &format!("Skill {skill} does not exist")).await;
        Ok(())
    }
}

async fn walk_next(
    platform: &Platform,
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    node: &Arc<SimpleNode>,
) -> Result<(), AppError> {
    if instance.is_complete() {
        return Ok(());
    }
    let nodes = instance.graph.get_forward_links(node.get_alias())?;
    for next in nodes {
        walk(platform, po, instance, next).await?;
    }
    Ok(())
}

async fn handle_error_response(
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    response: &EventEnvelope,
) {
    let out = instance.get_reply_to();
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(&out)
                .set_correlation_id(&instance.get_correlation_id())
                .set_raw_body(response.body().clone())
                .set_status(response.status()),
        )
        .await;
    emit_aborted(po, instance).await;
}

/// Canonical failure terminal — the mirror of the success terminal in
/// [`execution_complete`]. Marks the traversal complete and emits the single
/// end-of-transmission line the synchronous companion endpoint drains on, so
/// **every** `run` finishes with either `Graph traversal completed in N ms` or
/// `Graph traversal aborted` — a deterministic signal, never a timeout.
async fn emit_aborted(po: &PostOffice, instance: &Arc<GraphInstance>) {
    instance.set_complete();
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(&instance.get_reply_to())
                .set_correlation_id(&instance.get_correlation_id())
                .set_raw_body(Value::from("Graph traversal aborted"))
                .set_status(400),
        )
        .await;
}

/// Emit a specific failure reason and then the canonical [`emit_aborted`]
/// terminal, so the human/companion sees *why* and any watcher (the sync
/// endpoint included) still gets the uniform end-of-transmission line last.
async fn send_error(po: &PostOffice, instance: &Arc<GraphInstance>, message: &str) {
    instance.set_complete();
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(&instance.get_reply_to())
                .set_correlation_id(&instance.get_correlation_id())
                .set_raw_body(Value::from(message))
                .set_status(400),
        )
        .await;
    emit_aborted(po, instance).await;
}
