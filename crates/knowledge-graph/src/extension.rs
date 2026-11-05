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

//! The `graph.extension` skill — Rust port of `GraphExtension` (266 lines):
//! a node delegates to another deployed graph (a sub-graph, launched through
//! the `graph-executor` flow) or to an event-script flow (`flow://<id>`).
//! Input mapping stages the delegated body; `for_each` fans the delegation
//! out with clamped concurrency; the delegate's response lands in the node's
//! `result` (`[]`-appended per fork-join call) for the output mapping.
//!
//! `async=true` (RFC-0011 section 8.1, Java mercury-composable #609) starts the
//! target and continues at once: nothing comes back, the target inherits the
//! business correlation id and runs under its own TTL (or the node's `ttl`),
//! and only a failure to start - a target that does not exist, or an input the
//! target graph's root `schema` refuses - reaches the node's `exception` route.
//! With `for_each`, one run per element, at most `concurrency` active at once.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use event_script::conversions::display;
use event_script::mlm::MultiLevelMap;
use event_script::util::str2int;
use futures_util::StreamExt;
use platform_core::graph::SimpleNode;
use platform_core::post_office::BUSINESS_CID_TAG;
use platform_core::{AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::common::return_path_for;
use crate::common::{
    fill_fetcher_api_parameters, get_effective_ttl, get_entries, get_for_each_mapping,
    get_model_array_size, get_model_ttl, get_next_model_param_set, invalid, join_batch,
    perform_fetcher_output_mapping, ERROR, EXCEPTION, HEADER, ITERATION_INDEX_HEADER, NEXT,
    NODE_NAME, RESULT, SKILL, STATUS, TARGET,
};
use crate::model::GraphInstance;
use crate::progress::X_RETURN_PATH;

pub const ROUTE: &str = "graph.extension";
/// The node property that starts the target and returns at once (RFC-0011 section 8.1).
pub const ASYNC: &str = "async";
const FLOW_PROTOCOL: &str = "flow://";
const GRAPH_EXECUTOR_FLOW: &str = "graph-executor";
// an asynchronous run is waited on (to start the next element of a fan-out and to log its end)
// for its own TTL plus this margin; the target answers 408 itself when its TTL expires
const ASYNC_MARGIN_MS: i64 = 5000;
const DEFAULT_CONCURRENCY: i32 = 3;
const MAX_CONCURRENCY: i32 = 30;

/// Whether a `graph.extension` node starts its target and returns at once (`async=true`).
pub fn is_async(node: &SimpleNode) -> bool {
    node.get_property(ASYNC).map(|v| display(&v)).as_deref() == Some("true")
}

/// The skill entry point (Java `handleEvent`).
pub async fn handle(
    platform: &Platform,
    headers: HashMap<String, String>,
    _event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    if headers.get("type").map(String::as_str) != Some("execute") {
        return Err(invalid("Type must be EXECUTE"));
    }
    let po = PostOffice::new(platform);
    let node_name = headers.get("node").map(String::as_str).unwrap_or("none");
    po.annotate_trace("node", node_name);
    let in_id = headers.get("in").map(String::as_str).unwrap_or("none");
    let instance = crate::common::get_graph_instance(in_id)?;
    let node = crate::common::get_node(node_name, &instance.graph)?;
    let skill = node.get_property(SKILL).map(|v| display(&v));
    if skill.as_deref() != Some(ROUTE) {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have skill - {ROUTE}"
        )));
    }
    let extension = match node.get_property("extension") {
        Some(Value::String(id)) => id.as_str().unwrap_or_default().to_string(),
        _ => {
            return Err(invalid(format!(
                "{NODE_NAME}{node_name} does not have graph ID"
            )));
        }
    };
    let for_each = get_entries(node.get_property("for_each"));
    // reset results so execution is idempotent
    {
        let mut state = instance.state.lock().expect("graph state machine");
        state.remove_element(&format!("{node_name}.{RESULT}"));
        state.remove_element(&format!("{node_name}.{HEADER}"));
    }
    let asynchronous = is_async(&node);
    if for_each.is_empty() {
        return if asynchronous {
            start_extension(platform, &po, &node, &instance, &extension).await
        } else {
            call_extension(&po, &node, &instance, &extension).await
        };
    }
    // iterative delegation with an array of parameters (fork-join)
    let (size, timeout) = {
        let mut state = instance.state.lock().expect("graph state machine");
        let mappings = get_for_each_mapping(node_name, &for_each, &mut state)?;
        if mappings.is_empty() {
            return Err(invalid(format!(
                "{NODE_NAME}{node_name} - No data mapping resolved from 'for_each' entries. \
                 LHS must be a list."
            )));
        }
        let mapping = get_entries(node.get_property("input"));
        let size = get_model_array_size(&mappings);
        for i in 0..size {
            for (key, value) in get_next_model_param_set(&mappings, i) {
                state.set_element(&key, value).map_err(invalid)?;
            }
            for entry in &mapping {
                fill_fetcher_api_parameters(node_name, entry, &mut state, true)?;
            }
        }
        (size, get_effective_ttl(&mut state, &node)?)
    };
    if asynchronous {
        return start_extensions(platform, &po, &instance, &node, &extension, size).await;
    }
    call_extension_with_fork_join(&po, &instance, &node, &extension, size, timeout).await?;
    let state = instance.state.lock().expect("graph state machine");
    EventEnvelope::new().set_body(next_path(node_name, &node, &state))
}

/// Single delegation (Java `callExtension` + `retrieveFromExtension`).
async fn call_extension(
    po: &PostOffice,
    node: &Arc<SimpleNode>,
    instance: &Arc<GraphInstance>,
    extension: &str,
) -> Result<EventEnvelope, AppError> {
    let node_name = node.get_alias().to_string();
    let mapping = get_entries(node.get_property("input"));
    if mapping.is_empty() {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have 'input' entries"
        )));
    }
    let (parameters, ttl) = {
        let mut state = instance.state.lock().expect("graph state machine");
        for entry in &mapping {
            fill_fetcher_api_parameters(&node_name, entry, &mut state, false)?;
        }
        let ttl = get_effective_ttl(&mut state, node)?;
        let parameters = state
            .get_element(&format!("{node_name}.fetch"))
            .unwrap_or(Value::Map(vec![]));
        // clean up the working area
        state.remove_element(&format!("{node_name}.fetch"));
        state
            .set_element(&format!("{node_name}.{TARGET}"), Value::from(extension))
            .map_err(invalid)?;
        (parameters, ttl)
    };
    log::info!("Call extension {extension}, ttl={ttl}");
    po.annotate_trace("extension", extension);
    let business_cid = get_business_cid(instance);
    let return_path = return_path_for(instance, node).await;
    // a single delegation has no iteration index - its store key stays two-segment
    let forward = build_forward(
        &node_name,
        extension,
        parameters,
        Some(ttl),
        business_cid.as_deref(),
        None,
        return_path.as_deref(),
    )?;
    let response = call_flow(po, forward, ttl).await;
    let mut state = instance.state.lock().expect("graph state machine");
    state
        .set_element(
            &format!("{node_name}.{STATUS}"),
            Value::from(response.status),
        )
        .map_err(invalid)?;
    if !response.headers.is_empty() {
        state
            .set_element(
                &format!("{node_name}.{HEADER}"),
                headers_value(&response.headers),
            )
            .map_err(invalid)?;
    }
    let next = if response.status >= 400 {
        set_error(&node_name, node, &mut state, &response)?
    } else {
        state
            .set_element(&format!("{node_name}.{RESULT}"), response.body.clone())
            .map_err(invalid)?;
        let output_mapping = get_entries(node.get_property("output"));
        perform_fetcher_output_mapping(&node_name, &mut state, &output_mapping)?;
        NEXT.to_string()
    };
    drop(state);
    EventEnvelope::new().set_body(next)
}

/// Fork-join delegation over the staged parameter arrays
/// (Java `callExtensionWithForkJoin` + `runConcurrentRequests` + `doForkJoin`).
async fn call_extension_with_fork_join(
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    node: &Arc<SimpleNode>,
    extension: &str,
    size: usize,
    timeout: i64,
) -> Result<(), AppError> {
    let node_name = node.get_alias().to_string();
    let concurrency = concurrency_of(node);
    po.annotate_trace("extension", extension);
    let business_cid = get_business_cid(instance);
    let return_path = return_path_for(instance, node).await;
    let forwards = {
        let mut state = instance.state.lock().expect("graph state machine");
        let api_params = state
            .get_element(&format!("{node_name}.each"))
            .unwrap_or(Value::Map(vec![]));
        let Value::Map(param_arrays) = &api_params else {
            return Err(invalid(format!(
                "{NODE_NAME}{node_name} - no staged for_each parameters"
            )));
        };
        let mut names: Vec<String> = param_arrays
            .iter()
            .filter_map(|(k, _)| k.as_str().map(str::to_string))
            .collect();
        names.sort();
        po.annotate_trace("for_each", format!("{names:?}"));
        let mut forwards = Vec::with_capacity(size);
        for i in 0..size {
            let mut parameters: Vec<(Value, Value)> = Vec::new();
            for (key, values) in param_arrays {
                if let Value::Array(items) = values {
                    if let Some(value) = items.get(i) {
                        parameters.push((key.clone(), value.clone()));
                    }
                }
            }
            // the iteration's position rides with the invocation, so N
            // iterations of ONE suspending subgraph - which all inherit this
            // parent's business cid by design - no longer collide on one
            // suspend/resume record
            forwards.push(build_forward(
                &node_name,
                extension,
                Value::Map(parameters),
                Some(timeout),
                business_cid.as_deref(),
                Some(i),
                return_path.as_deref(),
            )?);
        }
        state
            .set_element(&format!("{node_name}.{TARGET}"), Value::from(extension))
            .map_err(invalid)?;
        forwards
    };
    for batch in forwards.chunks(concurrency) {
        log::info!(
            "Call extension {extension}, parallel={}, ttl={timeout}",
            batch.len()
        );
        let responses = run_flow_batch(batch.to_vec(), timeout).await;
        let mut state = instance.state.lock().expect("graph state machine");
        for response in &responses {
            state
                .set_element(
                    &format!("{node_name}.{STATUS}"),
                    Value::from(response.status),
                )
                .map_err(invalid)?;
            if !response.headers.is_empty() {
                state
                    .set_element(
                        &format!("{node_name}.{HEADER}[]"),
                        headers_value(&response.headers),
                    )
                    .map_err(invalid)?;
            }
            if response.status >= 400 {
                set_error(&node_name, node, &mut state, response)?;
            } else {
                state
                    .set_element(&format!("{node_name}.{RESULT}[]"), response.body.clone())
                    .map_err(invalid)?;
            }
        }
    }
    let mut state = instance.state.lock().expect("graph state machine");
    let output_mapping = get_entries(node.get_property("output"));
    perform_fetcher_output_mapping(&node_name, &mut state, &output_mapping)?;
    // clear the temporary dataset
    state.remove_element(&format!("{node_name}.fetch"));
    state.remove_element(&format!("{node_name}.each"));
    Ok(())
}

/// A fan-out's concurrency: the node's `concurrency`, default 3, clamped to 1-30 (Java parity).
fn concurrency_of(node: &SimpleNode) -> usize {
    let given = str2int(&display(
        &node.get_property("concurrency").unwrap_or(Value::Nil),
    ));
    if given < 0 {
        DEFAULT_CONCURRENCY
    } else {
        given
    }
    .clamp(1, MAX_CONCURRENCY) as usize
}

/// Start the target with the node's input mapping and return at once (Java `startExtension`).
/// Nothing comes back: the target runs under its own TTL, and only a failure to start reaches
/// this node's exception route.
async fn start_extension(
    platform: &Platform,
    po: &PostOffice,
    node: &Arc<SimpleNode>,
    instance: &Arc<GraphInstance>,
    extension: &str,
) -> Result<EventEnvelope, AppError> {
    let node_name = node.get_alias().to_string();
    let mapping = get_entries(node.get_property("input"));
    if mapping.is_empty() {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have 'input' entries"
        )));
    }
    let parameters = {
        let mut state = instance.state.lock().expect("graph state machine");
        for entry in &mapping {
            fill_fetcher_api_parameters(&node_name, entry, &mut state, false)?;
        }
        let parameters = state
            .get_element(&format!("{node_name}.fetch"))
            .unwrap_or(Value::Map(vec![]));
        state.remove_element(&format!("{node_name}.fetch"));
        parameters
    };
    start_runs(
        platform,
        po,
        instance,
        node,
        extension,
        vec![parameters],
        false,
    )
    .await
}

/// Start one run of the target per element of the `for_each` list and return at once (Java
/// `startExtensions`). The runs go in the background, at most `concurrency` of them active.
async fn start_extensions(
    platform: &Platform,
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    node: &Arc<SimpleNode>,
    extension: &str,
    size: usize,
) -> Result<EventEnvelope, AppError> {
    let node_name = node.get_alias().to_string();
    let elements = {
        let mut state = instance.state.lock().expect("graph state machine");
        let staged = state
            .get_element(&format!("{node_name}.each"))
            .unwrap_or(Value::Map(vec![]));
        let Value::Map(param_arrays) = &staged else {
            return Err(invalid(format!(
                "{NODE_NAME}{node_name} - no staged for_each parameters"
            )));
        };
        let elements: Vec<Value> = (0..size)
            .map(|i| {
                Value::Map(
                    param_arrays
                        .iter()
                        .filter_map(|(key, values)| match values {
                            Value::Array(items) => items.get(i).map(|v| (key.clone(), v.clone())),
                            _ => None,
                        })
                        .collect(),
                )
            })
            .collect();
        state.remove_element(&format!("{node_name}.fetch"));
        state.remove_element(&format!("{node_name}.each"));
        elements
    };
    start_runs(platform, po, instance, node, extension, elements, true).await
}

/// Check, then start, the runs of an asynchronous node (Java `startRuns`).
async fn start_runs(
    platform: &Platform,
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    node: &Arc<SimpleNode>,
    extension: &str,
    elements: Vec<Value>,
    indexed: bool,
) -> Result<EventEnvelope, AppError> {
    let node_name = node.get_alias().to_string();
    {
        let mut state = instance.state.lock().expect("graph state machine");
        state
            .set_element(&format!("{node_name}.{TARGET}"), Value::from(extension))
            .map_err(invalid)?;
    }
    po.annotate_trace("extension", extension);
    po.annotate_trace(ASYNC, elements.len().to_string());
    let Some(target_ttl) = target_ttl(extension) else {
        return start_failure(
            &node_name,
            node,
            instance,
            404,
            Value::from(format!(
                "{NODE_NAME}{node_name} - {extension} does not exist"
            )),
        );
    };
    // the node's own ttl, when it declares one, is the target's TTL; otherwise the target's own applies
    let (node_ttl, model_ttl) = {
        let mut state = instance.state.lock().expect("graph state machine");
        let node_ttl = match node.get_property("ttl") {
            Some(_) => Some(get_effective_ttl(&mut state, node)?),
            None => None,
        };
        (node_ttl, get_model_ttl(&mut state))
    };
    let business_cid = get_business_cid(instance);
    let mut runs = Vec::with_capacity(elements.len());
    for (i, parameters) in elements.into_iter().enumerate() {
        // an asynchronous target reports to nobody: it never inherits this run's return path
        runs.push(build_forward(
            &node_name,
            extension,
            parameters,
            node_ttl,
            business_cid.as_deref(),
            indexed.then_some(i),
            None,
        )?);
    }
    if let Some((status, reason)) =
        refused_input(platform, po, extension, &runs, indexed, model_ttl).await
    {
        return start_failure(&node_name, node, instance, status, reason);
    }
    // a run started after this node returned is outside the worker's trace bracket (a tokio
    // task-local), so each run carries this node's trace, its span as the parent and the business
    // correlation id explicitly - what the bracket gives a synchronous delegation
    let trace = po.my_trace_id().zip(po.my_trace_path());
    let span = po.my_span_id();
    let runs: Vec<EventEnvelope> = runs
        .into_iter()
        .map(|mut run| {
            if let Some((trace_id, trace_path)) = &trace {
                run = run.set_trace(trace_id, trace_path);
            }
            if let Some(span) = &span {
                run = run.set_span_id(span);
            }
            if let Some(cid) = &business_cid {
                run = run.add_tag(BUSINESS_CID_TAG, cid);
            }
            run
        })
        .collect();
    let ttl = node_ttl.unwrap_or(target_ttl);
    let concurrency = if indexed { concurrency_of(node) } else { 1 };
    log::info!(
        "Start extension {extension}, async, runs={}, concurrency={concurrency}, ttl={ttl}",
        runs.len()
    );
    start_in_background(
        extension.to_string(),
        runs,
        concurrency,
        ttl + ASYNC_MARGIN_MS,
    );
    EventEnvelope::new().set_body(NEXT)
}

/// The TTL a target runs under when the node declares none: a flow's own, and for a deployed
/// graph the TTL of the flow that runs graphs. None when the target does not exist.
fn target_ttl(extension: &str) -> Option<i64> {
    if let Some(flow_id) = extension.strip_prefix(FLOW_PROTOCOL) {
        return event_script::flows::get_flow(flow_id).map(|flow| flow.ttl as i64);
    }
    if !crate::graphs::graph_exists(extension) {
        return None;
    }
    event_script::flows::get_flow(GRAPH_EXECUTOR_FLOW).map(|flow| flow.ttl as i64)
}

/// A map value's entry by key.
fn entry<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    match value {
        Value::Map(entries) => entries
            .iter()
            .find(|(k, _)| k.as_str() == Some(key))
            .map(|(_, v)| v),
        _ => None,
    }
}

/// The `schema` of a deployed graph's root node, when it declares one as a map.
fn root_schema(graph_id: &str) -> Option<Value> {
    let model = crate::graphs::get_graph(graph_id)?;
    let Some(Value::Array(nodes)) = entry(&model, "nodes") else {
        return None;
    };
    let root = nodes
        .iter()
        .find(|node| entry(node, "alias").and_then(Value::as_str) == Some("root"))?;
    match entry(root, "properties").and_then(|p| entry(p, crate::common::SCHEMA)) {
        Some(schema @ Value::Map(_)) => Some(schema.clone()),
        _ => None,
    }
}

/// A deployed graph with a contract validates its input at its root. An asynchronous run cannot
/// tell this node, so the input of every run is validated here before any run starts, the way the
/// target's root will validate it: a refused input is a failure to start (Java `refusedInput`).
async fn refused_input(
    platform: &Platform,
    po: &PostOffice,
    extension: &str,
    runs: &[EventEnvelope],
    indexed: bool,
    timeout_ms: i64,
) -> Option<(i32, Value)> {
    if extension.starts_with(FLOW_PROTOCOL) {
        return None;
    }
    let schema = root_schema(extension)?;
    let route = crate::common::schema_validator_route();
    if !platform.has_route(route)
        && platform_core::automation::event_api::get_event_http_target(route).is_none()
    {
        return Some((
            500,
            Value::from(format!("Schema validator '{route}' does not exist")),
        ));
    }
    for (i, run) in runs.iter().enumerate() {
        let dataset = run.body();
        let body = entry(dataset, "body")
            .cloned()
            .unwrap_or(Value::Map(vec![]));
        let header = entry(dataset, "header")
            .cloned()
            .unwrap_or(Value::Map(vec![]));
        let request = EventEnvelope::new()
            .set_to(route)
            .set_correlation_id(&uuid::Uuid::new_v4().simple().to_string())
            .set_raw_body(Value::Map(vec![
                (Value::from("body"), body),
                (Value::from("header"), header),
                (Value::from(crate::common::SCHEMA), schema.clone()),
            ]));
        let refusal = match po
            .request(request, Duration::from_millis(timeout_ms.max(0) as u64))
            .await
        {
            Ok(response) if response.has_error() => {
                Some((response.status(), response.body().clone()))
            }
            Ok(_) => None,
            Err(e) => Some((e.status(), Value::from(e.message()))),
        };
        if let Some((status, reason)) = refusal {
            let reason = match (&reason, indexed) {
                (Value::String(text), true) => Value::from(format!(
                    "for_each element {i} - {}",
                    text.as_str().unwrap_or_default()
                )),
                _ => reason,
            };
            return Some((status, reason));
        }
    }
    None
}

/// Stage a failure to start as the node's error: its status, then the error and its route
/// (Java `startFailure`).
fn start_failure(
    node_name: &str,
    node: &Arc<SimpleNode>,
    instance: &Arc<GraphInstance>,
    status: i32,
    message: Value,
) -> Result<EventEnvelope, AppError> {
    let mut state = instance.state.lock().expect("graph state machine");
    state
        .set_element(&format!("{node_name}.{STATUS}"), Value::from(status))
        .map_err(invalid)?;
    let response = FlowResponse {
        status,
        headers: HashMap::new(),
        body: message,
    };
    let next = set_error(node_name, node, &mut state, &response)?;
    drop(state);
    EventEnvelope::new().set_body(next)
}

/// The runs of an asynchronous node after it returned (Java `BackgroundRuns`): a sliding window
/// that keeps at most `concurrency` runs active and starts the next as an earlier one ends. Each
/// run's end is logged with its status, the only record of an outcome nobody waits for.
fn start_in_background(
    extension: String,
    runs: Vec<EventEnvelope>,
    concurrency: usize,
    wait_ms: i64,
) {
    let total = runs.len();
    let wait = Duration::from_millis(wait_ms.max(0) as u64);
    tokio::spawn(async move {
        let platform = Platform::get_instance();
        let po = PostOffice::new(&platform);
        let (po, extension) = (&po, &extension);
        futures_util::stream::iter(runs.into_iter().enumerate())
            .for_each_concurrent(concurrency, |(i, run)| async move {
                let begin = Instant::now();
                let outcome = po.request(run, wait).await;
                let elapsed = begin.elapsed().as_millis();
                let position = i + 1;
                match outcome {
                    Err(e) if e.status() == 408 => log::warn!(
                        "Extension {extension} run {position} of {total} not completed in {elapsed} ms - {}",
                        e.message()
                    ),
                    Err(e) => log::warn!(
                        "Extension {extension} run {position} of {total} ended with status {} in {elapsed} ms - {}",
                        e.status(),
                        e.message()
                    ),
                    Ok(response) if response.has_error() => log::warn!(
                        "Extension {extension} run {position} of {total} ended with status {} in {elapsed} ms - {}",
                        response.status(),
                        display(response.body())
                    ),
                    Ok(response) => log::info!(
                        "Extension {extension} run {position} of {total} ended with status {} in {elapsed} ms",
                        response.status()
                    ),
                }
            })
            .await;
    });
}

/// The caller's business correlation-id (model.cid), trimmed — `None` when
/// absent or blank. Both engines trim identically so the store key space
/// stays one key per business transaction per graph.
fn get_business_cid(instance: &Arc<GraphInstance>) -> Option<String> {
    let state = instance.state.lock().expect("graph state machine");
    match state.get_element("model.cid") {
        Some(Value::String(s)) => {
            let trimmed = s.as_str().unwrap_or_default().trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        }
        _ => None,
    }
}

/// Build the flow-launch envelope: `flow://<id>` targets that flow directly;
/// a graph id rides the `graph-executor` flow with `path_parameter.graph_id`
/// (Java parity — the sub-graph runs as its own flow instance). The child
/// flow or graph inherits the caller's business correlation-id (model.cid),
/// exactly like an Event Script sub-flow launch — so a subgraph that suspends
/// can be resumed under the same business transaction id (its store record is
/// keyed by graph + cid, never by a per-call random id).
///
/// A `for_each` iteration additionally carries its position as the
/// `x-iteration-index` HEADER of a graph invocation (Java `GraphExtension`):
/// it must not go in the body, which is the author's own input-mapping
/// contract, and the header map is the one channel a parent controls that
/// reaches the executor WITHOUT touching the application-owned
/// `graph-executor` flow file. The subgraph's suspend/resume key includes it,
/// so N iterations of one subgraph no longer collide on one record. A
/// `flow://` target gets no index (Java parity): the flow hop carries no graph
/// iteration identity, which is why parent → `for_each` → flow → suspending
/// graph is a declared non-goal rather than a supported shape.
///
/// `return_path` is the relay a synchronous subgraph returns its progress
/// events through (RFC-0011 section 8.4): a graph target gets it as its
/// `x-return-path` header and names itself in its events, which reach this
/// run's caller through this node's relay; a `flow://` target gets none.
#[allow(clippy::too_many_arguments)]
fn build_forward(
    node_name: &str,
    extension: &str,
    parameters: Value,
    ttl: Option<i64>,
    business_cid: Option<&str>,
    iteration_index: Option<usize>,
    return_path: Option<&str>,
) -> Result<EventEnvelope, AppError> {
    let body = match parameters {
        v @ Value::Map(_) => v,
        _ => Value::Map(vec![]),
    };
    let mut header_entries: Vec<(Value, Value)> = Vec::new();
    if !extension.starts_with(FLOW_PROTOCOL) {
        if let Some(i) = iteration_index {
            header_entries.push((
                Value::from(ITERATION_INDEX_HEADER),
                Value::from(i.to_string()),
            ));
        }
        if let Some(return_path) = return_path {
            header_entries.push((Value::from(X_RETURN_PATH), Value::from(return_path)));
        }
    }
    let header = Value::Map(header_entries);
    let mut dataset: Vec<(Value, Value)> =
        vec![(Value::from("body"), body), (Value::from("header"), header)];
    // no ttl: the target runs under its own (an asynchronous run without a node ttl)
    if let Some(ttl) = ttl {
        dataset.push((Value::from("ttl"), Value::from(ttl)));
    }
    let flow_id = if let Some(flow_id) = extension.strip_prefix(FLOW_PROTOCOL) {
        if event_script::flows::get_flow(flow_id).is_none() {
            return Err(invalid(format!(
                "{NODE_NAME}{node_name} -  {extension} does not exist"
            )));
        }
        flow_id.to_string()
    } else {
        dataset.push((
            Value::from("path_parameter"),
            Value::Map(vec![(Value::from("graph_id"), Value::from(extension))]),
        ));
        GRAPH_EXECUTOR_FLOW.to_string()
    };
    let mut forward = EventEnvelope::new()
        .set_to(event_script::manager::SERVICE_NAME)
        .set_header("flow_id", &flow_id)
        .set_correlation_id(&uuid::Uuid::new_v4().simple().to_string())
        .set_raw_body(Value::Map(dataset));
    if let Some(cid) = business_cid {
        forward = forward.set_header(event_script::manager::BUSINESS_CORRELATION_ID, cid);
    }
    Ok(forward)
}

struct FlowResponse {
    status: i32,
    headers: HashMap<String, String>,
    body: Value,
}

async fn call_flow(po: &PostOffice, forward: EventEnvelope, ttl: i64) -> FlowResponse {
    match po
        .request(forward, Duration::from_millis(ttl.max(0) as u64))
        .await
    {
        Ok(response) => FlowResponse {
            status: response.status(),
            headers: response.headers().clone(),
            body: response.body().clone(),
        },
        Err(e) => FlowResponse {
            status: e.status(),
            headers: HashMap::new(),
            body: Value::from(e.message()),
        },
    }
}

/// One fork-join batch of flow launches, responses in request order — awaited
/// on the calling task so each child flow inherits this worker's trace and
/// business correlation-id exactly as a single delegation does (see
/// `common::join_batch` for why this must not spawn).
async fn run_flow_batch(forwards: Vec<EventEnvelope>, ttl: i64) -> Vec<FlowResponse> {
    let platform = Platform::get_instance();
    let po = PostOffice::new(&platform);
    join_batch(
        forwards
            .into_iter()
            .map(|forward| call_flow(&po, forward, ttl))
            .collect(),
    )
    .await
}

fn set_error(
    node_name: &str,
    node: &Arc<SimpleNode>,
    state: &mut MultiLevelMap,
    response: &FlowResponse,
) -> Result<String, AppError> {
    state
        .set_element(&format!("{node_name}.{ERROR}"), response.body.clone())
        .map_err(invalid)?;
    match node.get_property(EXCEPTION) {
        None => {
            state
                .set_element("output.body", response.body.clone())
                .map_err(invalid)?;
            state
                .set_element("output.header", headers_value(&response.headers))
                .map_err(invalid)?;
            state
                .set_element("output.status", Value::from(response.status))
                .map_err(invalid)?;
            Ok(NEXT.to_string())
        }
        Some(handler) => Ok(display(&handler)),
    }
}

fn next_path(node_name: &str, node: &Arc<SimpleNode>, state: &MultiLevelMap) -> String {
    let process_status = state.get_element(&format!("{node_name}.{STATUS}"));
    let result_error = state.get_element(&format!("{node_name}.{ERROR}"));
    let error_handler = node.get_property(EXCEPTION);
    if let (Some(Value::Integer(_)), Some(_), Some(handler)) =
        (&process_status, &result_error, &error_handler)
    {
        display(handler)
    } else {
        NEXT.to_string()
    }
}

fn headers_value(headers: &HashMap<String, String>) -> Value {
    let mut entries: Vec<(String, String)> = headers
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    Value::Map(
        entries
            .into_iter()
            .map(|(k, v)| (Value::from(k.as_str()), Value::from(v.as_str())))
            .collect(),
    )
}
