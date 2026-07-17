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

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use event_script::conversions::display;
use event_script::mlm::MultiLevelMap;
use event_script::util::str2int;
use platform_core::graph::SimpleNode;
use platform_core::{AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::common::{
    fill_fetcher_api_parameters, get_entries, get_for_each_mapping, get_model_array_size,
    get_model_ttl, get_next_model_param_set, invalid, perform_fetcher_output_mapping, ERROR,
    EXCEPTION, HEADER, NEXT, NODE_NAME, RESULT, SKILL, STATUS, TARGET,
};
use crate::model::GraphInstance;

pub const ROUTE: &str = "graph.extension";
const FLOW_PROTOCOL: &str = "flow://";
const GRAPH_EXECUTOR_FLOW: &str = "graph-executor";

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
    if for_each.is_empty() {
        return call_extension(&po, &node, &instance, &extension).await;
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
        (size, get_model_ttl(&mut state))
    };
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
        let ttl = get_model_ttl(&mut state);
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
    let forward = build_forward(&node_name, extension, parameters, ttl)?;
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
    let given = str2int(&display(
        &node.get_property("concurrency").unwrap_or(Value::Nil),
    ));
    let concurrency = if given < 0 { 3 } else { given }.clamp(1, 30) as usize;
    po.annotate_trace("extension", extension);
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
            forwards.push(build_forward(
                &node_name,
                extension,
                Value::Map(parameters),
                timeout,
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

/// Build the flow-launch envelope: `flow://<id>` targets that flow directly;
/// a graph id rides the `graph-executor` flow with `path_parameter.graph_id`
/// (Java parity — the sub-graph runs as its own flow instance).
fn build_forward(
    node_name: &str,
    extension: &str,
    parameters: Value,
    ttl: i64,
) -> Result<EventEnvelope, AppError> {
    let body = match parameters {
        v @ Value::Map(_) => v,
        _ => Value::Map(vec![]),
    };
    let mut dataset: Vec<(Value, Value)> = vec![
        (Value::from("body"), body),
        (Value::from("header"), Value::Map(vec![])),
        (Value::from("ttl"), Value::from(ttl)),
    ];
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
    Ok(EventEnvelope::new()
        .set_to(event_script::manager::SERVICE_NAME)
        .set_header("flow_id", &flow_id)
        .set_correlation_id(&uuid::Uuid::new_v4().simple().to_string())
        .set_raw_body(Value::Map(dataset)))
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

/// One fork-join batch of flow launches, responses in request order.
async fn run_flow_batch(forwards: Vec<EventEnvelope>, ttl: i64) -> Vec<FlowResponse> {
    let mut handles = Vec::with_capacity(forwards.len());
    for forward in forwards {
        handles.push(tokio::spawn(async move {
            let platform = Platform::get_instance();
            let po = PostOffice::new(&platform);
            call_flow(&po, forward, ttl).await
        }));
    }
    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        results.push(handle.await.unwrap_or_else(|_| FlowResponse {
            status: 500,
            headers: HashMap::new(),
            body: Value::from("Extension join failure"),
        }));
    }
    results
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
