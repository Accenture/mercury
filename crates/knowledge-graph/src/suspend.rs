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

//! Workflow suspension for the Active Knowledge Graph — Rust port of the
//! Java `GraphSuspend` / `GraphResume` skills (shared base
//! `GraphStateSkill`). A long-running business process with human
//! checkpoints is a **sequence of short graph runs**: at a suspension point
//! the run persists `{model + traversal bookkeeping}` to an external state
//! store keyed by the business correlation ID (`model.cid`) with a
//! designer-chosen TTL, then completes normally; a later request with the
//! same cid restores the state and continues **past** the checkpoint without
//! re-executing it. Both skills are supersets of `graph.task` — the node's
//! `task` property names a pluggable store function with a fixed put/get
//! contract and **zero node data mapping**. Node types (Suspend / Resume /
//! Suspensible) are visual convention; the skill defines behavior.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use event_script::conversions::display;
use event_script::mlm::MultiLevelMap;
use event_script::util::str2long;
use platform_core::graph::SimpleNode;
use platform_core::{AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::common::{
    get_graph_instance, get_model_ttl, get_node, invalid, ERROR, EXCEPTION, EXECUTE, HEADER, IN,
    NEXT, NODE, NODE_NAME, RESULT, SKILL, STATUS, TARGET, TYPE,
};
use crate::model::GraphInstance;

pub const SUSPEND_ROUTE: &str = "graph.suspend";
pub const RESUME_ROUTE: &str = "graph.resume";
/// The reserved node alias (like root/end): traversal jumps to the
/// suspension point BY NAME, so there is exactly one per graph.
pub const SUSPEND_ALIAS: &str = "suspend";
/// The walker directive a successful resume returns: `resume:<alias>`.
pub const RESUME_PREFIX: &str = "resume:";

const FROM: &str = "from";
const TTL: &str = "ttl";
const CID: &str = "cid";
const MODEL: &str = "model";
const MODEL_CID: &str = "model.cid";
/// The engine-managed run flag (`"resume" | "fresh"`): graph.resume is its
/// only writer. Reserved flow metadata — the flow compiler rejects any data
/// mapping that overwrites it (reading it as a source stays legal).
const MODEL_RUN: &str = "model.run";
const FRESH: &str = "fresh";
const RESUMED: &str = "resume";
const SEEN: &str = "seen";
const RUN: &str = "run";
const PUT: &str = "put";
const GET: &str = "get";
const SUSPENDED: &str = "suspended";
const TASK: &str = "task";
const OUTPUT_BODY: &str = "output.body";
const INTERNAL_SERVER_ERROR: i32 = 500;

/// Per-run engine metadata never crosses a suspension in either direction:
/// graph.suspend excludes these keys from the persistence envelope and
/// graph.resume strips them from a restored record — the resumed run's own
/// values are authoritative (`run` is the fresh/resume flag set by
/// graph.resume; embalming it would let a later resume read a stale
/// condition, and the store is pluggable so a record is external input).
pub const NON_PERSISTED_MODEL_KEYS: [&str; 9] = [
    "cid", "instance", "flow", "ttl", "trace", "parent", "root", "none", "run",
];

/// The shared context ladder (Java `GraphStateSkill.getContext`): validate
/// EXECUTE, resolve instance/node, check the skill route, require a valid
/// `task` store route, reset the node's result keys, stamp the target.
struct SkillContext {
    instance: Arc<GraphInstance>,
    node: Arc<SimpleNode>,
    route: String,
}

fn get_context(
    platform: &Platform,
    headers: &HashMap<String, String>,
    skill_route: &str,
) -> Result<(PostOffice, SkillContext), AppError> {
    if headers.get(TYPE).map(String::as_str) != Some(EXECUTE) {
        return Err(invalid("Type must be EXECUTE"));
    }
    let po = PostOffice::new(platform);
    let node_name = headers.get(NODE).map(String::as_str).unwrap_or("none");
    po.annotate_trace(NODE, node_name);
    let in_id = headers.get(IN).map(String::as_str).unwrap_or("none");
    let instance = get_graph_instance(in_id)?;
    let node = get_node(node_name, &instance.graph)?;
    let skill = node.get_property(SKILL).map(|v| display(&v));
    if skill.as_deref() != Some(skill_route) {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have skill - {skill_route}"
        )));
    }
    let route = match node.get_property(TASK) {
        Some(v) => {
            let value = display(&v);
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        }
        None => None,
    };
    let Some(route) = route else {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have a 'task' route"
        )));
    };
    if !platform.has_route(&route) {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} - task '{route}' does not exist"
        )));
    }
    // reset result to ensure execution is idempotent, then stamp the target
    {
        let mut state = instance.state.lock().expect("graph state machine");
        for suffix in [RESULT, HEADER, STATUS, ERROR] {
            state.remove_element(&format!("{node_name}.{suffix}"));
        }
        state
            .set_element(
                &format!("{node_name}.{TARGET}"),
                Value::from(route.as_str()),
            )
            .map_err(invalid)?;
    }
    Ok((
        po,
        SkillContext {
            instance,
            node,
            route,
        },
    ))
}

/// Java `getRequiredCorrelationId`: the business correlation ID is the
/// retrieval key AND the resume capability — a state skill cannot run
/// without it.
fn get_required_correlation_id(
    instance: &GraphInstance,
    node_name: &str,
) -> Result<String, AppError> {
    let value = {
        let state = instance.state.lock().expect("graph state machine");
        state.get_element(MODEL_CID)
    };
    if let Some(Value::String(text)) = value {
        let cid = text.as_str().unwrap_or_default().trim().to_string();
        if !cid.is_empty() {
            return Ok(cid);
        }
    }
    Err(invalid(format!(
        "{NODE_NAME}{node_name} requires model.cid - supply a business correlation ID \
         (e.g. X-Correlation-Id header) or set model.cid"
    )))
}

/// Parse and validate a checkpoint ttl — the single implementation shared by
/// this skill and the CompileGraph gate's static check (Java
/// `GraphSuspend.getValidTtlSeconds`).
///
/// Duration syntax 20s/5m/2h/2d or plain seconds, computed in 64-bit math:
/// a narrower computation wraps for absurd values (e.g. a huge day count),
/// which could pass a naive `< 1` guard and silently expire the record far
/// earlier than modeled. There is deliberately NO default — "a workflow step
/// can be suspended for a minute or a few days"; only the designer knows.
pub fn get_valid_ttl_seconds(ttl: Option<&Value>, node_alias: &str) -> Result<i64, AppError> {
    let text = ttl.map(display).unwrap_or_default();
    let text = text.trim();
    if text.is_empty() {
        return Err(invalid(format!(
            "{NODE_NAME}{node_alias} does not have a 'ttl' property"
        )));
    }
    let (digits, multiplier) = match text.chars().last() {
        Some('s') => (&text[..text.len() - 1], 1i64),
        Some('m') => (&text[..text.len() - 1], 60),
        Some('h') => (&text[..text.len() - 1], 3600),
        Some('d') => (&text[..text.len() - 1], 86400),
        _ => (text, 1),
    };
    let seconds = str2long(digits.trim()).saturating_mul(multiplier);
    if seconds < 1 || seconds > i32::MAX as i64 {
        return Err(invalid(format!(
            "{NODE_NAME}{node_alias} - invalid ttl '{text}'"
        )));
    }
    Ok(seconds)
}

/// Java `GraphStateSkill.setError`: stage the failure onto the state machine
/// and route to the node's exception handler when one exists, else stage the
/// error as the run's output and continue to completion.
fn set_error(
    state: &mut MultiLevelMap,
    node: &Arc<SimpleNode>,
    status: i32,
    body: Value,
    response_headers: &HashMap<String, String>,
) -> String {
    let node_name = node.get_alias();
    let _ = state.set_element(&format!("{node_name}.{ERROR}"), body.clone());
    match node.get_property(EXCEPTION) {
        None => {
            let _ = state.set_element(OUTPUT_BODY, body);
            let headers: Vec<(Value, Value)> = response_headers
                .iter()
                .map(|(k, v)| (Value::from(k.as_str()), Value::from(v.as_str())))
                .collect();
            let _ = state.set_element(&format!("output.{HEADER}"), Value::Map(headers));
            let _ = state.set_element(&format!("output.{STATUS}"), Value::from(status));
            NEXT.to_string()
        }
        Some(handler) => display(&handler),
    }
}

/// Java `GraphStateSkill.recordFailure`: an invalid or corrupted store
/// record fails the node so the walker's error handling (or the node's
/// exception handler) takes over — always an internal server error because
/// the record is engine-managed state.
fn record_failure(state: &mut MultiLevelMap, node: &Arc<SimpleNode>, message: &str) -> String {
    let _ = state.set_element(
        &format!("{}.{STATUS}", node.get_alias()),
        Value::from(INTERNAL_SERVER_ERROR),
    );
    set_error(
        state,
        node,
        INTERNAL_SERVER_ERROR,
        Value::from(message),
        &HashMap::new(),
    )
}

/// The normalized store response.
struct StoreResponse {
    status: i32,
    headers: HashMap<String, String>,
    body: Value,
}

async fn call_store(po: &PostOffice, request: EventEnvelope, timeout_ms: i64) -> StoreResponse {
    match po
        .request(request, Duration::from_millis(timeout_ms.max(0) as u64))
        .await
    {
        Ok(response) => StoreResponse {
            status: response.status(),
            headers: response.headers().clone(),
            body: response.body().clone(),
        },
        Err(e) => StoreResponse {
            status: e.status(),
            headers: HashMap::new(),
            body: Value::from(e.message()),
        },
    }
}

// ---- graph.suspend (Java GraphSuspend) ----

pub async fn suspend(
    platform: &Platform,
    headers: HashMap<String, String>,
    _event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let (po, ctx) = get_context(platform, &headers, SUSPEND_ROUTE)?;
    let node_name = ctx.node.get_alias().to_string();
    if node_name != SUSPEND_ALIAS {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} - a node with skill {SUSPEND_ROUTE} must be named '{SUSPEND_ALIAS}'"
        )));
    }
    let cid = get_required_correlation_id(&ctx.instance, &node_name)?;
    let from = headers.get(FROM).map(String::as_str).unwrap_or("").trim();
    if from.is_empty() {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} - suspension point unknown; the '{SUSPEND_ALIAS}' node \
             must be reached from another node"
        )));
    }
    let ttl_seconds = get_valid_ttl_seconds(ctx.node.get_property(TTL).as_ref(), &node_name)?;
    warn_if_branches_in_flight(&ctx.instance, from);
    let (timeout, envelope) = {
        let mut state = ctx.instance.state.lock().expect("graph state machine");
        (
            get_model_ttl(&mut state),
            persistence_envelope(&ctx.instance, &state, &cid, from, ttl_seconds),
        )
    };
    let request = EventEnvelope::new()
        .set_to(&ctx.route)
        .set_correlation_id(&uuid_simple())
        .set_header(TYPE, PUT)
        .set_raw_body(envelope);
    log::debug!(
        "Suspend at '{from}' for cid {cid}, store={}, ttl={ttl_seconds}s",
        ctx.route
    );
    po.annotate_trace(TASK, &ctx.route);
    po.annotate_trace(CID, &cid);
    // the request is issued within the skill's own task-scoped trace context,
    // so the store call's span chains onto THIS skill's span (the observable
    // topology Java achieves by issuing the request on the worker thread)
    let response = call_store(&po, request, timeout).await;
    let next = {
        let mut state = ctx.instance.state.lock().expect("graph state machine");
        state
            .set_element(
                &format!("{node_name}.{STATUS}"),
                Value::from(response.status),
            )
            .map_err(invalid)?;
        if response.status >= 400 {
            set_error(
                &mut state,
                &ctx.node,
                response.status,
                response.body,
                &response.headers,
            )
        } else {
            // a meaningful default reply for the caller of the suspended run,
            // unless the graph staged its own output before suspension
            if state.get_element(OUTPUT_BODY).is_none() {
                let _ = state.set_element(
                    OUTPUT_BODY,
                    Value::Map(vec![
                        (Value::from(TYPE), Value::from(SUSPENDED)),
                        (Value::from(CID), Value::from(cid.as_str())),
                    ]),
                );
            }
            NEXT.to_string()
        }
    };
    EventEnvelope::new().set_body(next)
}

/// The persistence envelope (headers `type=put`): `{cid, node, ttl, model,
/// seen, run}` — model = the model namespace MINUS the reserved keys.
fn persistence_envelope(
    instance: &GraphInstance,
    state: &MultiLevelMap,
    cid: &str,
    from: &str,
    ttl_seconds: i64,
) -> Value {
    let model_copy = match state.get_element(MODEL) {
        Some(Value::Map(entries)) => Value::Map(
            entries
                .into_iter()
                .filter(|(k, _)| {
                    !NON_PERSISTED_MODEL_KEYS.contains(&k.as_str().unwrap_or_default())
                })
                .collect(),
        ),
        _ => Value::Map(vec![]),
    };
    Value::Map(vec![
        (Value::from(CID), Value::from(cid)),
        (Value::from(NODE), Value::from(from)),
        (Value::from(TTL), Value::from(ttl_seconds)),
        (Value::from(MODEL), model_copy),
        (
            Value::from(SEEN),
            marks_to_value(&instance.node_seen.lock().expect("node seen")),
        ),
        (
            Value::from(RUN),
            marks_to_value(&instance.skill_run.lock().expect("skill run")),
        ),
    ])
}

fn marks_to_value(marks: &HashMap<String, bool>) -> Value {
    Value::Map(
        marks
            .iter()
            .map(|(k, v)| (Value::from(k.as_str()), Value::from(*v)))
            .collect(),
    )
}

/// Best-effort guard (Java `warnIfBranchesInFlight`): a suspension point
/// should be the sole active branch — a node dispatched but not completed at
/// suspension time cannot be persisted (its callback will be orphaned when
/// this run completes).
fn warn_if_branches_in_flight(instance: &GraphInstance, from: &str) {
    let seen: Vec<String> = instance
        .node_seen
        .lock()
        .expect("node seen")
        .keys()
        .cloned()
        .collect();
    let run = instance.skill_run.lock().expect("skill run");
    for name in seen {
        if !run.contains_key(&name) && name != SUSPEND_ALIAS && name != from {
            if let Ok(Some(other)) = instance.graph.find_node_by_alias(&name) {
                let skill = other.get_property(SKILL).map(|v| display(&v));
                if let Some(skill) = skill {
                    if skill != crate::skills::JOIN_ROUTE {
                        log::warn!(
                            "Suspension while node '{name}' may still be in flight - a suspension \
                             point should be the sole active branch in {}",
                            instance.graph_id
                        );
                    }
                }
            }
        }
    }
}

// ---- graph.resume (Java GraphResume) ----

pub async fn resume(
    platform: &Platform,
    headers: HashMap<String, String>,
    _event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let (po, ctx) = get_context(platform, &headers, RESUME_ROUTE)?;
    let node_name = ctx.node.get_alias().to_string();
    let cid = get_required_correlation_id(&ctx.instance, &node_name)?;
    let timeout = {
        let mut state = ctx.instance.state.lock().expect("graph state machine");
        get_model_ttl(&mut state)
    };
    let request = EventEnvelope::new()
        .set_to(&ctx.route)
        .set_correlation_id(&uuid_simple())
        .set_header(TYPE, GET)
        .set_raw_body(Value::Map(vec![(
            Value::from(CID),
            Value::from(cid.as_str()),
        )]));
    po.annotate_trace(TASK, &ctx.route);
    po.annotate_trace(CID, &cid);
    // issued within the skill's trace context — the store call's span chains
    // onto this skill's span (Java issues eagerly on the worker thread)
    let response = call_store(&po, request, timeout).await;
    let next = {
        let mut state = ctx.instance.state.lock().expect("graph state machine");
        state
            .set_element(
                &format!("{node_name}.{STATUS}"),
                Value::from(response.status),
            )
            .map_err(invalid)?;
        if response.status >= 400 {
            set_error(
                &mut state,
                &ctx.node,
                response.status,
                response.body,
                &response.headers,
            )
        } else if matches!(&response.body, Value::Map(entries) if !entries.is_empty()) {
            restore_and_jump(&ctx.instance, &mut state, &ctx.node, &cid, &response.body)
        } else {
            // no suspension record: a fresh transaction is the normal case —
            // the run flag lets the graph's own logic react to the condition
            // (absent and expired records are indistinguishable BY DESIGN)
            let _ = state.set_element(MODEL_RUN, Value::from(FRESH));
            log::debug!("No suspension record for cid {cid} - fresh start");
            NEXT.to_string()
        }
    };
    EventEnvelope::new().set_body(next)
}

fn restore_and_jump(
    instance: &GraphInstance,
    state: &mut MultiLevelMap,
    node: &Arc<SimpleNode>,
    cid: &str,
    received: &Value,
) -> String {
    let record = MultiLevelMap::from_value(received.clone());
    let suspended_at = match record.get_element(NODE) {
        Some(Value::String(text)) => {
            let value = text.as_str().unwrap_or_default().trim().to_string();
            if value.is_empty() {
                None
            } else {
                Some(value)
            }
        }
        _ => None,
    };
    let Some(suspended_at) = suspended_at else {
        return record_failure(state, node, "Corrupted suspension record - missing 'node'");
    };
    if !matches!(
        instance.graph.find_node_by_alias(&suspended_at),
        Ok(Some(_))
    ) {
        return record_failure(
            state,
            node,
            &format!(
                "Suspension record refers to unknown node '{suspended_at}' - the graph model \
                 may have changed"
            ),
        );
    }
    if let Some(Value::Map(persisted)) = record.get_element(MODEL) {
        // persisted keys are authoritative for the workflow, but never the
        // per-run reserved keys: graph.suspend does not persist them, and the
        // store is pluggable — a record from a foreign writer or an older
        // build must not override the current run's identity (model.cid is a
        // capability)
        for (key, value) in persisted {
            let name = key.as_str().unwrap_or_default();
            if !name.is_empty() && !NON_PERSISTED_MODEL_KEYS.contains(&name) {
                let _ = state.set_element(&format!("{MODEL}.{name}"), value);
            }
        }
    }
    // set AFTER the merge so a record written by an older build can never
    // resurrect a stale run flag
    let _ = state.set_element(MODEL_RUN, Value::from(RESUMED));
    restore_marks(record.get_element(SEEN), &instance.node_seen);
    restore_marks(record.get_element(RUN), &instance.skill_run);
    log::debug!("Resume at '{suspended_at}' for cid {cid}");
    format!("{RESUME_PREFIX}{suspended_at}")
}

/// Restore TRUTHY bookkeeping marks only, and EXCLUDE the `suspend` alias —
/// the suspend node's marks are per-run mechanics: restoring them would
/// block re-suspension at a later checkpoint in the resumed run.
fn restore_marks(marks: Option<Value>, target: &std::sync::Mutex<HashMap<String, bool>>) {
    if let Some(Value::Map(entries)) = marks {
        let mut guard = target.lock().expect("bookkeeping marks");
        for (key, value) in entries {
            let name = key.as_str().unwrap_or_default();
            let truthy = matches!(&value, Value::Boolean(true))
                || display(&value).eq_ignore_ascii_case("true");
            if !name.is_empty() && name != SUSPEND_ALIAS && truthy {
                guard.insert(name.to_string(), true);
            }
        }
    }
}

fn uuid_simple() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shared ttl parser (skill + compile gate): duration syntax, plain
    /// seconds, and the 64-bit overflow guard — Java's int computation
    /// wrapped for absurd day counts and could pass a naive `< 1` check.
    #[test]
    fn ttl_parser_matches_java_semantics() {
        let ok = |text: &str| get_valid_ttl_seconds(Some(&Value::from(text)), "suspend").unwrap();
        assert_eq!(20, ok("20s"));
        assert_eq!(300, ok("5m"));
        assert_eq!(7200, ok("2h"));
        assert_eq!(172800, ok("2d"));
        assert_eq!(90, ok("90"));
        assert_eq!(3600, ok("1h"));
        let err = |ttl: Option<&Value>| {
            get_valid_ttl_seconds(ttl, "suspend")
                .expect_err("must reject")
                .message()
                .to_string()
        };
        // missing / blank
        assert_eq!("node suspend does not have a 'ttl' property", err(None));
        assert_eq!(
            "node suspend does not have a 'ttl' property",
            err(Some(&Value::from("  ")))
        );
        // zero, negative, garbage
        assert!(err(Some(&Value::from("0"))).contains("invalid ttl"));
        assert!(err(Some(&Value::from("-5m"))).contains("invalid ttl"));
        assert!(err(Some(&Value::from("abc"))).contains("invalid ttl"));
        // the overflow guard: an absurd day count must reject, not wrap
        assert!(err(Some(&Value::from("99999999999d"))).contains("invalid ttl"));
        assert!(err(Some(&Value::from("9999999999"))).contains("invalid ttl"));
    }
}
