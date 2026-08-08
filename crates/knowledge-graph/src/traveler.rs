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
const RUN_TIMEOUT: &str = "run_timeout";
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
        } else if headers.get("type").map(String::as_str) == Some(RUN_TIMEOUT) {
            handle_run_timeout(&po, &headers, &event).await;
        } else if event.reply_to().is_some() {
            execute_graph(platform, &po, &headers, &event).await;
        }
    }
    EventEnvelope::new().set_body("ignored")
}

/// Dry-run mirror of the deployed lane's flow timer (a FlowInstance schedules
/// one at construction): a one-shot watcher that turns a hung or overlong
/// traversal into the canonical failure terminal at the `model.ttl` deadline,
/// so the console — and the synchronous companion drain — always receives an
/// end-of-transmission line. Child calls are already deadline-bounded by
/// `get_effective_ttl` in both lanes; this covers what those cannot: total
/// run duration and a skill that never replies. `model.ttl` is immutable
/// during a run, so the deadline armed here is the deadline reported. The
/// slot token carries the owning run's correlation id, so a stale watcher
/// can never act on a newer run (Java `GraphTraveler.armRunWatcher`).
fn arm_run_watcher(po: &PostOffice, instance: &Arc<GraphInstance>, in_route: &str) {
    // the traveler is re-runnable in the same session - a previous run's
    // watcher may still be pending and must not abort the new run
    cancel_run_watcher(po, instance);
    let ttl = {
        let mut state = instance.state.lock().expect("graph state machine");
        common::get_model_ttl(&mut state)
    };
    let cid = instance.get_correlation_id();
    let timeout_event = EventEnvelope::new()
        .set_to(ROUTE)
        .set_header("type", RUN_TIMEOUT)
        .set_header("in", in_route)
        .set_correlation_id(&cid);
    let timer = po.send_later(
        timeout_event,
        std::time::Duration::from_millis(ttl.max(0) as u64),
    );
    instance.set_run_watcher(Some(format!("{cid}|{timer}")));
}

fn cancel_run_watcher(po: &PostOffice, instance: &Arc<GraphInstance>) {
    if let Some(token) = instance.get_run_watcher() {
        // atomic removal: two racing cancellers act at most once, on the
        // exact token read
        if instance.clear_run_watcher(&token) {
            if let Some(sep) = token.find('|') {
                po.cancel_future_event(&token[sep + 1..]);
            }
        }
    }
}

/// Exactly-one-terminal arbitration: every terminal path (success, error,
/// timeout) claims the run before emitting — the winner cancels the watcher
/// and emits its terminal, a loser stays silent, so a run racing its own
/// deadline can never emit both terminals (which would misclassify a
/// successful run as failed in the companion capture; Java
/// `GraphTraveler.claimTerminal`).
fn claim_terminal(po: &PostOffice, instance: &Arc<GraphInstance>) -> bool {
    if instance.claim_complete() {
        cancel_run_watcher(po, instance);
        true
    } else {
        false
    }
}

async fn handle_run_timeout(
    po: &PostOffice,
    headers: &HashMap<String, String>,
    event: &EventEnvelope,
) {
    let Some(in_route) = headers.get("in") else {
        return;
    };
    let Some(instance) = crate::model::get_instance(in_route) else {
        return;
    };
    let event_cid = event.correlation_id().unwrap_or_default();
    let Some(token) = instance.get_run_watcher() else {
        return;
    };
    // the watcher may act only for the run that armed it: the slot token
    // carries the owning run's correlation id and the atomic removal is the
    // claim - a stale watcher (a newer run owns the slot, or a canceller
    // already won) stays silent
    if !token.starts_with(&format!("{event_cid}|"))
        || !instance.clear_run_watcher(&token)
        || !claim_terminal(po, &instance)
    {
        return;
    }
    let ttl = {
        let mut state = instance.state.lock().expect("graph state machine");
        common::get_model_ttl(&mut state)
    };
    let out = instance.get_reply_to();
    // best-effort sends: the reply route may be a released companion capture
    // route - the run is already marked complete, so bookkeeping stays
    // consistent either way
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(&out)
                .set_correlation_id(event_cid)
                .set_status(408)
                .set_raw_body(Value::from(format!(
                    "Graph traversal timed out after {ttl} ms"
                ))),
        )
        .await;
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(&out)
                .set_correlation_id(event_cid)
                .set_raw_body(Value::from("Graph traversal aborted"))
                .set_status(400),
        )
        .await;
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
        // a live instance may exist when the failure happened after arming
        // the run watcher (a walk-time error) - claim the terminal so the
        // watcher cannot fire a second one later
        if let Some(in_route) = headers.get("in") {
            if let Some(instance) = crate::model::get_instance(in_route) {
                claim_terminal(po, &instance);
            }
        }
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
    // disarm a previous run's watcher BEFORE the reset so it cannot observe
    // the half-reset state and abort the new run
    cancel_run_watcher(po, &instance);
    instance.set_flow_instance_id(in_route);
    instance.set_correlation_id(cid);
    instance.set_reply_to(reply_to);
    instance.node_seen.lock().expect("node seen").clear();
    instance.skill_run.lock().expect("skill run").clear();
    // loop-detection counts must not bleed across playground runs
    instance.hits.lock().expect("visit counters").clear();
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
    arm_run_watcher(po, &instance, in_route);
    walk(platform, po, &instance, root, None).await
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
    // a late reply after the run reached a terminal (completed, aborted or
    // timed out) is dropped before any console send - the reply route may
    // already be a released companion capture route
    if instance.is_complete() {
        return;
    }
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
                // {spent:?} — Java Float.toString always keeps the decimal
                // point ("3.0 ms"), the repo's float-parity rule
                .set_raw_body(Value::from(format!(
                    "Executed {node_name} with skill {skill} in {spent:?} ms"
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
        if claim_terminal(po, &instance) {
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
        }
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
        if let Some(alias) = next.strip_prefix(crate::suspend::RESUME_PREFIX) {
            resume_traversal(platform, po, instance, alias).await;
        } else if next == NEXT {
            if has_suspend_edge(instance, &node) {
                walk_to_suspend_node(platform, po, instance, &node).await;
            } else {
                let _ = walk_next(platform, po, instance, &node, false).await;
            }
        } else {
            match instance.graph.find_node_by_alias(next) {
                Ok(Some(next_node)) => {
                    let _ = walk(
                        platform,
                        po,
                        instance,
                        next_node,
                        Some(node.get_alias().to_string()),
                    )
                    .await;
                }
                _ => {
                    send_error(po, instance, &format!("Next node '{next}' does not exist")).await;
                }
            }
        }
    }
}

/// A node with a drawn edge to the reserved 'suspend' node is a suspension point
/// (edge mode): on a normal 'next' completion the walker redirects to the checkpoint,
/// and a resumed run continues along the node's other forward links. The retired
/// 'suspend=true' property is ignored - the drawn edge is the declaration.
/// A decision reaches the checkpoint by jumping instead (jump mode) and is
/// re-executed on resume; see is_jump_mode_checkpoint.
fn has_suspend_edge(instance: &Arc<GraphInstance>, node: &Arc<SimpleNode>) -> bool {
    instance
        .graph
        .get_forward_links(node.get_alias())
        .map(|links| {
            links
                .iter()
                .any(|n| n.get_alias() == crate::suspend::SUSPEND_ALIAS)
        })
        .unwrap_or(false)
}

/// A suspension point with NO drawn edge to the suspend node reached the checkpoint
/// by an IF-THEN-ELSE jump (jump mode) - by construction only a routing skill can
/// jump, so the node is a decision and a resumed run RE-EXECUTES it against the new
/// request input instead of continuing past it. An edge-mode suspension point
/// (drawn edge present) is never re-executed - the resumed run continues along its
/// other forward links, exactly the pre-rationalization behavior.
fn is_jump_mode_checkpoint(instance: &Arc<GraphInstance>, alias: &str) -> bool {
    match instance.graph.find_node_by_alias(alias) {
        Ok(Some(node)) => !has_suspend_edge(instance, &node),
        _ => false,
    }
}

/// The dry-run lane keeps the FULL suspend-contract guards: playground
/// drafts never pass the CompileGraph gate, so the traveler is that lane's
/// only enforcement (Java GraphTraveler parity).
async fn walk_to_suspend_node(
    platform: &Platform,
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    node: &Arc<SimpleNode>,
) {
    let skill = node.get_property(SKILL).map(|v| display(&v));
    if matches!(skill.as_deref(), Some("graph.math") | Some("graph.js")) {
        let skill_name = skill.unwrap_or_default();
        send_error(
            po,
            instance,
            &format!(
                "Node '{}' has a drawn edge to the 'suspend' node but uses routing skill {skill_name} - \
                 a decision reaches the checkpoint by jumping: return 'suspend' from its IF-THEN-ELSE \
                 and draw edges to 'suspend' only from working nodes",
                node.get_alias()
            ),
        )
        .await;
        return;
    }
    match instance
        .graph
        .find_node_by_alias(crate::suspend::SUSPEND_ALIAS)
    {
        Ok(Some(suspend_node)) => {
            let suspend_skill = suspend_node.get_property(SKILL).map(|v| display(&v));
            if suspend_skill.as_deref() != Some(crate::suspend::SUSPEND_ROUTE) {
                send_error(
                    po,
                    instance,
                    &format!(
                        "The '{}' node must use skill {}",
                        crate::suspend::SUSPEND_ALIAS,
                        crate::suspend::SUSPEND_ROUTE
                    ),
                )
                .await;
            } else {
                let _ = walk(
                    platform,
                    po,
                    instance,
                    suspend_node,
                    Some(node.get_alias().to_string()),
                )
                .await;
            }
        }
        _ => {
            send_error(
                po,
                instance,
                &format!(
                    "Node '{}' is a suspension point but the graph has no '{}' node",
                    node.get_alias(),
                    crate::suspend::SUSPEND_ALIAS
                ),
            )
            .await;
        }
    }
}

async fn resume_traversal(
    platform: &Platform,
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    alias: &str,
) {
    match instance.graph.find_node_by_alias(alias) {
        Ok(Some(resumed)) => {
            if is_jump_mode_checkpoint(instance, alias) {
                // the decision jumped to the checkpoint: re-execute it against the
                // new request input - its forward links are outcome alternatives,
                // not branches (clear the marks restored from the suspension record
                // so the walk dispatches)
                instance.node_seen.lock().expect("node seen").remove(alias);
                instance.skill_run.lock().expect("skill run").remove(alias);
                let _ = walk(platform, po, instance, resumed, None).await;
            } else {
                // the suspension point (drawn checkpoint edge) already ran before
                // suspension - do not re-execute it; continue along its other
                // forward links
                instance
                    .node_seen
                    .lock()
                    .expect("node seen")
                    .insert(alias.to_string(), true);
                instance
                    .skill_run
                    .lock()
                    .expect("skill run")
                    .insert(alias.to_string(), true);
                let _ = walk_next(platform, po, instance, &resumed, true).await;
            }
        }
        _ => {
            send_error(
                po,
                instance,
                &format!("Resumed node '{alias}' does not exist"),
            )
            .await;
        }
    }
}

fn walk<'a>(
    platform: &'a Platform,
    po: &'a PostOffice,
    instance: &'a Arc<GraphInstance>,
    node: Arc<SimpleNode>,
    from: Option<String>,
) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'a>> {
    Box::pin(async move {
        if instance.is_complete() {
            return Ok(());
        }
        let node_name = node.get_alias().to_string();
        let skill = node.get_property(SKILL).map(|v| display(&v));
        // atomic mark-and-test under ONE lock acquisition (executor mirror)
        let is_join = skill.as_deref() == Some(crate::skills::JOIN_ROUTE);
        // Java putIfAbsent parity (executor mirror): never overwrite a
        // join's `false` barrier flag with a transient `true`
        let seen = {
            let mut marks = instance.node_seen.lock().expect("node seen");
            let seen = marks.contains_key(&node_name);
            if !seen {
                marks.insert(node_name.clone(), true);
            }
            seen
        };
        if is_join || !seen {
            let _ = po
                .send(
                    EventEnvelope::new()
                        .set_to(&instance.get_reply_to())
                        .set_raw_body(Value::from(format!("Walk to {node_name}"))),
                )
                .await;
            walk_to(platform, po, skill, instance, node, from).await?;
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
    from: Option<String>,
) -> Result<(), AppError> {
    let is_end = instance
        .graph
        .get_end_node()
        .map(|end| end.get_id() == node.get_id())
        .unwrap_or(false);
    match skill {
        Some(skill) => execute_skill(platform, po, &skill, instance, &node, &from).await,
        None if is_end => {
            execution_complete(po, instance).await;
            Ok(())
        }
        None if has_suspend_edge(instance, &node) => {
            walk_to_suspend_node(platform, po, instance, &node).await;
            Ok(())
        }
        None => walk_next(platform, po, instance, &node, false).await,
    }
}

async fn execution_complete(po: &PostOffice, instance: &Arc<GraphInstance>) {
    if !claim_terminal(po, instance) {
        return;
    }
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
    from: &Option<String>,
) -> Result<(), AppError> {
    if skill == "graph.js" {
        send_error(po, instance, RETIRED_JS_MESSAGE).await;
        return Ok(());
    }
    if platform.has_route(skill) {
        let in_route = instance.get_flow_instance_id();
        let node_name = node.get_alias();
        let composite = format!("{in_route}@{node_name}");
        let mut event = EventEnvelope::new()
            .set_to(skill)
            .set_header("in", &in_route)
            .set_header("type", "execute")
            .set_header("node", node_name)
            .set_reply_to(ROUTE)
            .set_correlation_id(&composite);
        if let Some(from) = from {
            event = event.set_header("from", from);
        }
        // interceptor walker: stamp the business correlation-id from the
        // graph's own model.cid (executor mirror)
        let business_cid = {
            let state = instance.state.lock().expect("graph state machine");
            state.get_element("model.cid")
        };
        if let Some(Value::String(text)) = business_cid {
            // tag the TRIMMED value (Java-exact <= U+0020 trim): the business
            // cid is the store key, so both engines normalize identically —
            // my_correlation_id / log-context stay one value fleet-wide
            let cid = crate::common::java_trim(text.as_str().unwrap_or_default());
            if !cid.trim().is_empty() {
                event = event.add_tag(platform_core::post_office::BUSINESS_CID_TAG, cid);
            }
        }
        po.send(event).await
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
    after_resume: bool,
) -> Result<(), AppError> {
    if instance.is_complete() {
        return Ok(());
    }
    let nodes = instance.graph.get_forward_links(node.get_alias())?;
    let mut dead_end = true;
    for next in nodes {
        // a resumed traversal continues along the normal path, never back
        // into suspension
        if after_resume && next.get_alias() == crate::suspend::SUSPEND_ALIAS {
            continue;
        }
        dead_end = false;
        walk(
            platform,
            po,
            instance,
            next,
            Some(node.get_alias().to_string()),
        )
        .await?;
    }
    if after_resume && dead_end {
        send_error(
            po,
            instance,
            &format!(
                "Resumed node '{}' has no forward path to continue",
                node.get_alias()
            ),
        )
        .await;
    }
    Ok(())
}

async fn handle_error_response(
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    response: &EventEnvelope,
) {
    if !claim_terminal(po, instance) {
        return;
    }
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
/// [`execution_complete`]. Emits the single end-of-transmission line the
/// synchronous companion endpoint drains on, so **every** `run` finishes
/// with either `Graph traversal completed in N ms` or
/// `Graph traversal aborted` — a deterministic signal, never a timeout.
/// Callers own the terminal via [`claim_terminal`] before emitting.
async fn emit_aborted(po: &PostOffice, instance: &Arc<GraphInstance>) {
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
    if !claim_terminal(po, instance) {
        return;
    }
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
