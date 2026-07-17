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

//! The core graph skills (Java `skills` package): `graph.data.mapper`,
//! `graph.math`, `graph.task`, `graph.join` and `graph.island`. Each skill
//! is invoked by the executor with headers `{type: execute, in: <flow
//! instance id>, node: <alias>}` and replies with the routing decision —
//! `next`, a node alias to jump to, or `.sink` to stop the branch.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use event_script::conversions::display;
use event_script::mapping::get_lhs_or_constant;
use event_script::mlm::MultiLevelMap;
use event_script::util::{split, str2int, str2long};
use platform_core::graph::SimpleNode;
use platform_core::{AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::common::{
    combine, count_execute_statements, get_else_statement, get_entries, get_first_word,
    get_for_each_mapping, get_graph_instance, get_if_statement, get_model_array_size,
    get_model_ttl, get_next_model_param_set, get_next_node, get_next_tag, get_node,
    get_then_statement, handle_data_mapping_entry, invalid, perform_fetcher_output_mapping,
    reset_nodes, split_blocks, substitute_var_if_any, COMPUTE_TAG, DELAY_TAG, ERROR, EXCEPTION,
    EXECUTE, HEADER, IF_TAG, IN, MAPPING_TAG, MAP_TO, NEXT, NODE, NODE_NAME, RESET_TAG, RESULT,
    SINK, SKILL, STATUS, TARGET, TYPE,
};
use crate::math::ExpressionEngine;
use crate::model::GraphInstance;

pub const DATA_MAPPER_ROUTE: &str = "graph.data.mapper";
pub const MATH_ROUTE: &str = "graph.math";
pub const TASK_ROUTE: &str = "graph.task";
pub const JOIN_ROUTE: &str = "graph.join";
pub const ISLAND_ROUTE: &str = "graph.island";

fn require_execute(headers: &HashMap<String, String>) -> Result<(), AppError> {
    if headers.get(TYPE).map(String::as_str) != Some(EXECUTE) {
        return Err(invalid("Type must be EXECUTE"));
    }
    Ok(())
}

fn skill_context<'a>(
    platform: &Platform,
    headers: &'a HashMap<String, String>,
    route: &str,
) -> Result<(Arc<GraphInstance>, Arc<SimpleNode>, &'a str), AppError> {
    let po = PostOffice::new(platform);
    let node_name = headers.get(NODE).map(String::as_str).unwrap_or("none");
    po.annotate_trace(NODE, node_name);
    let in_id = headers.get(IN).map(String::as_str).unwrap_or("none");
    let instance = get_graph_instance(in_id)?;
    let node = get_node(node_name, &instance.graph)?;
    let skill = node.get_property(SKILL).map(|v| display(&v));
    if skill.as_deref() != Some(route) {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have skill - {route}"
        )));
    }
    Ok((instance, node, node_name))
}

// ---- graph.data.mapper (Java GraphDataMapper) ----

pub async fn data_mapper(
    platform: &Platform,
    headers: HashMap<String, String>,
    _event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    require_execute(&headers)?;
    let (instance, node, node_name) = skill_context(platform, &headers, DATA_MAPPER_ROUTE)?;
    let mapping = get_entries(node.get_property("mapping"));
    if mapping.is_empty() {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have 'mapping' entries"
        )));
    }
    {
        let mut state = instance.state.lock().expect("graph state machine");
        for entry in &mapping {
            handle_data_mapping_entry(node_name, entry, &mut state, &instance.graph)?;
        }
    }
    EventEnvelope::new().set_body(NEXT)
}

// ---- graph.math (Java GraphMath) ----

fn engine() -> &'static ExpressionEngine {
    static ENGINE: std::sync::OnceLock<ExpressionEngine> = std::sync::OnceLock::new();
    ENGINE.get_or_init(ExpressionEngine::new)
}

pub async fn math(
    platform: &Platform,
    headers: HashMap<String, String>,
    _event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    require_execute(&headers)?;
    let (instance, node, node_name) = skill_context(platform, &headers, MATH_ROUTE)?;
    let for_each = get_entries(node.get_property("for_each"));
    let statements = get_entries(node.get_property("statement"));
    if statements.is_empty() {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have 'statement' entries"
        )));
    }
    let (result, delay_ms) = {
        let mut state = instance.state.lock().expect("graph state machine");
        let result = execute_math_node(node_name, &instance, &mut state, &for_each, &statements)?;
        state
            .set_element(
                &format!("{node_name}.decision"),
                Value::from(result.as_str()),
            )
            .map_err(invalid)?;
        let delay = match state.get_element(&format!("{node_name}.delay")) {
            Some(Value::Integer(n)) => n.as_i64(),
            _ => None,
        };
        (result, delay)
    };
    if let Some(ms) = delay_ms {
        tokio::time::sleep(Duration::from_millis(ms.max(0) as u64)).await;
    }
    EventEnvelope::new().set_body(result)
}

fn execute_math_node(
    node_name: &str,
    instance: &GraphInstance,
    state: &mut MultiLevelMap,
    for_each: &[String],
    statements: &[String],
) -> Result<String, AppError> {
    let execute = count_execute_statements(node_name, statements)?;
    let merged = if execute > 0 {
        combine(MATH_ROUTE, node_name, &instance.graph, statements)?
    } else {
        statements.to_vec()
    };
    if for_each.is_empty() {
        return execute_statements(node_name, &merged, instance, state);
    }
    let blocks = split_blocks(&merged);
    let result1 = execute_statements(node_name, &blocks[0], instance, state)?;
    if result1 != NEXT {
        return Ok(result1);
    }
    let result2 = execute_for_each(node_name, for_each, &blocks[1], instance, state)?;
    if result2 != NEXT {
        return Ok(result2);
    }
    execute_statements(node_name, &blocks[2], instance, state)
}

fn execute_for_each(
    node_name: &str,
    for_each: &[String],
    statements: &[String],
    instance: &GraphInstance,
    state: &mut MultiLevelMap,
) -> Result<String, AppError> {
    if statements.is_empty() {
        return Ok(NEXT.to_string());
    }
    let mappings = get_for_each_mapping(node_name, for_each, state)?;
    if mappings.is_empty() {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} - No data mapping resolved from 'for_each' entries. \
             LHS must be a list."
        )));
    }
    let size = get_model_array_size(&mappings);
    for i in 0..size {
        for (key, value) in get_next_model_param_set(&mappings, i) {
            state.set_element(&key, value).map_err(invalid)?;
        }
        let result = execute_statements(node_name, statements, instance, state)?;
        if result != NEXT {
            return Ok(result);
        }
    }
    Ok(NEXT.to_string())
}

fn execute_statements(
    node_name: &str,
    entries: &[String],
    instance: &GraphInstance,
    state: &mut MultiLevelMap,
) -> Result<String, AppError> {
    let mut jump: Option<String> = None;
    for entry in entries {
        let block = entry.trim();
        let colon = block.find(':').map(|c| c + 1).unwrap_or(0);
        let tag = block[..colon].to_lowercase();
        let command = block[colon..].trim();
        if tag == IF_TAG {
            // evaluate decisions from an if-then-else multiline statement
            let lines = split(block, "\n");
            if let Some(decision) = evaluate(instance, node_name, &lines, state)? {
                return Ok(decision);
            }
        }
        process_commands(&tag, command, node_name, instance, state)?;
        if let Some(next) = get_next_tag(&tag, command) {
            jump = Some(next);
        }
    }
    Ok(jump.unwrap_or_else(|| NEXT.to_string()))
}

fn process_commands(
    tag: &str,
    command: &str,
    node_name: &str,
    instance: &GraphInstance,
    state: &mut MultiLevelMap,
) -> Result<(), AppError> {
    // guarantee a single line for all commands except IF-THEN-ELSE
    let command = command.replace('\n', " ");
    if tag == COMPUTE_TAG {
        compute(&command, node_name, state)?;
    }
    if tag == MAPPING_TAG {
        handle_data_mapping_entry(node_name, &command, state, &instance.graph)?;
    }
    if tag == RESET_TAG {
        reset_nodes(&command, instance, state);
    }
    if tag == DELAY_TAG {
        let delay = str2long(&command);
        if delay >= 0 {
            state
                .set_element(&format!("{node_name}.delay"), Value::from(delay))
                .map_err(invalid)?;
        }
    }
    Ok(())
}

fn compute(command: &str, node_name: &str, state: &mut MultiLevelMap) -> Result<(), AppError> {
    let Some(sep) = command.rfind(MAP_TO) else {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have '->' in '{command}'"
        )));
    };
    // lhs is the result key, rhs is the math expression
    let lhs = command[..sep].trim();
    let rhs = command[sep + MAP_TO.len()..].trim();
    if lhs.is_empty() || rhs.is_empty() {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} has invalid statement '{command}'"
        )));
    }
    let text = substitute_var_if_any(rhs, state)?;
    let result = if has_boolean_operator(&text) {
        Value::from(engine().eval_boolean(&text).map_err(math_error)?)
    } else {
        Value::from(engine().eval_number(&text).map_err(math_error)?)
    };
    state
        .set_element(&format!("{node_name}.result.{lhs}"), result)
        .map_err(invalid)
}

fn math_error(e: crate::math::MathError) -> AppError {
    invalid(e.message().to_string())
}

fn has_boolean_operator(text: &str) -> bool {
    text.contains("&&")
        || text.contains("||")
        || text.contains('!')
        || text.contains('>')
        || text.contains('<')
        || text.contains(">=")
        || text.contains("<=")
        || text.contains("==")
}

fn evaluate(
    instance: &GraphInstance,
    node_name: &str,
    lines: &[String],
    state: &mut MultiLevelMap,
) -> Result<Option<String>, AppError> {
    let if_statement = get_if_statement(lines);
    let then_full = get_then_statement(lines);
    let else_full = get_else_statement(lines);
    let then_statement = get_first_word(then_full.trim());
    let else_statement = get_first_word(else_full.trim());
    if if_statement.is_empty() || then_statement.is_empty() || else_statement.is_empty() {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have if:, then: or else:"
        )));
    }
    let text = substitute_var_if_any(&if_statement, state)?;
    let branch = if engine().eval_boolean(&text).map_err(math_error)? {
        then_statement
    } else {
        else_statement
    };
    get_next_node(&instance.graph, branch)
}

// ---- graph.task (Java GraphTask) ----

const WHOLE_BODY: &str = "*";

pub async fn task(
    platform: &Platform,
    headers: HashMap<String, String>,
    _event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    require_execute(&headers)?;
    let po = PostOffice::new(platform);
    let (instance, node, node_name) = skill_context(platform, &headers, TASK_ROUTE)?;
    let node_name = node_name.to_string();
    let route = match node.get_property("task") {
        Some(v) => {
            let value = display(&v);
            if value.trim().is_empty() {
                None
            } else {
                Some(value.trim().to_string())
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
    let mapping = get_entries(node.get_property("input"));
    let for_each = get_entries(node.get_property("for_each"));
    // reset results so execution is idempotent, then resolve ttl
    let ttl = {
        let mut state = instance.state.lock().expect("graph state machine");
        for suffix in [RESULT, HEADER, STATUS, ERROR] {
            state.remove_element(&format!("{node_name}.{suffix}"));
        }
        get_model_ttl(&mut state)
    };
    if for_each.is_empty() {
        let request = {
            let mut state = instance.state.lock().expect("graph state machine");
            state
                .set_element(
                    &format!("{node_name}.{TARGET}"),
                    Value::from(route.as_str()),
                )
                .map_err(invalid)?;
            build_task_request(&node_name, &route, &mapping, &mut state)?
        };
        log::info!("Call task {route}, ttl={ttl}");
        po.annotate_trace("task", &route);
        let response = call_task(&po, request, ttl).await;
        let next = {
            let mut state = instance.state.lock().expect("graph state machine");
            store_single_response(&node_name, &node, &mut state, &response)?
        };
        return EventEnvelope::new().set_body(next);
    }
    // iterative task requests with an array of parameters (fork-join)
    let (requests, concurrency) = {
        let mut state = instance.state.lock().expect("graph state machine");
        let mappings = get_for_each_mapping(&node_name, &for_each, &mut state)?;
        if mappings.is_empty() {
            return Err(invalid(format!(
                "{NODE_NAME}{node_name} - No data mapping resolved from 'for_each' entries. \
                 LHS must be a list."
            )));
        }
        let given = str2int(&display(
            &node.get_property("concurrency").unwrap_or(Value::Nil),
        ));
        let concurrency = if given < 0 { 3 } else { given }.clamp(1, 30) as usize;
        let size = get_model_array_size(&mappings);
        let mut requests = Vec::with_capacity(size);
        for i in 0..size {
            for (key, value) in get_next_model_param_set(&mappings, i) {
                state.set_element(&key, value).map_err(invalid)?;
            }
            requests.push(build_task_request(
                &node_name, &route, &mapping, &mut state,
            )?);
        }
        log::info!(
            "Call task {route}, for each {:?}, parallel={concurrency}, ttl={ttl}",
            mappings.iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>()
        );
        po.annotate_trace("task", &route);
        state
            .set_element(
                &format!("{node_name}.{TARGET}"),
                Value::from(route.as_str()),
            )
            .map_err(invalid)?;
        (requests, concurrency)
    };
    for batch in requests.chunks(concurrency) {
        let responses = run_batch(batch.to_vec(), ttl).await;
        let mut state = instance.state.lock().expect("graph state machine");
        for response in &responses {
            store_fork_join_response(&node_name, &node, &mut state, response)?;
        }
    }
    let next = {
        let mut state = instance.state.lock().expect("graph state machine");
        let output_mapping = get_entries(node.get_property("output"));
        perform_fetcher_output_mapping(&node_name, &mut state, &output_mapping)?;
        next_path(&node_name, &node, &state)
    };
    EventEnvelope::new().set_body(next)
}

/// One fork-join batch: requests run concurrently on spawned tasks and the
/// responses come back in request order (Java `po.request(batch, timeout)`).
async fn run_batch(requests: Vec<EventEnvelope>, ttl: i64) -> Vec<TaskResponse> {
    let mut handles = Vec::with_capacity(requests.len());
    for request in requests {
        handles.push(tokio::spawn(async move {
            let platform = Platform::get_instance();
            let po = PostOffice::new(&platform);
            call_task(&po, request, ttl).await
        }));
    }
    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        results.push(handle.await.unwrap_or_else(|_| TaskResponse {
            status: 500,
            headers: HashMap::new(),
            body: Value::from("Task join failure"),
        }));
    }
    results
}

/// The normalized task response: status, headers, body, error flag.
struct TaskResponse {
    status: i32,
    headers: HashMap<String, String>,
    body: Value,
}

impl TaskResponse {
    fn has_error(&self) -> bool {
        self.status >= 400
    }
}

async fn call_task(po: &PostOffice, request: EventEnvelope, ttl: i64) -> TaskResponse {
    match po
        .request(request, Duration::from_millis(ttl.max(0) as u64))
        .await
    {
        Ok(response) => TaskResponse {
            status: response.status(),
            headers: response.headers().clone(),
            body: response.body().clone(),
        },
        Err(e) => TaskResponse {
            status: e.status(),
            headers: HashMap::new(),
            body: Value::from(e.message()),
        },
    }
}

fn build_task_request(
    node_name: &str,
    route: &str,
    mapping: &[String],
    state: &mut MultiLevelMap,
) -> Result<EventEnvelope, AppError> {
    let mut request = EventEnvelope::new()
        .set_to(route)
        .set_correlation_id(&uuid_simple());
    let mut body = Value::Map(vec![]);
    let mut request_headers: Vec<(String, String)> = Vec::new();
    for entry in mapping {
        let Some(sep) = entry.rfind(MAP_TO) else {
            return Err(invalid(format!(
                "{NODE_NAME}{node_name} does not have '->' in '{entry}'"
            )));
        };
        let lhs = substitute_var_if_any(entry[..sep].trim(), state)?;
        let rhs = entry[sep + MAP_TO.len()..].trim();
        let value = get_lhs_or_constant(&lhs, state).map_err(invalid)?;
        body = stage_task_parameter(node_name, &mut request_headers, rhs, value, body)?;
    }
    for (key, value) in request_headers {
        request = request.set_header(&key, &value);
    }
    Ok(request.set_raw_body(body))
}

fn stage_task_parameter(
    node_name: &str,
    request_headers: &mut Vec<(String, String)>,
    rhs: &str,
    value: Option<Value>,
    body: Value,
) -> Result<Value, AppError> {
    if rhs == WHOLE_BODY {
        // the whole-body target maps the LHS value as the entire request
        // body; a clone is a deep copy, so later entries merge into it
        // without touching the graph's state machine
        return Ok(value.unwrap_or(Value::Nil));
    }
    if let Some(key) = rhs.strip_prefix("header.") {
        let key = key.trim();
        if let (Some(v), false) = (&value, key.is_empty()) {
            request_headers.push((key.to_string(), display(v)));
        }
        return Ok(body);
    }
    if matches!(body, Value::Map(_)) {
        let mut map = MultiLevelMap::from_value(body);
        match value {
            Some(v) => map.set_element(rhs, v).map_err(invalid)?,
            None => map.remove_element(rhs),
        }
        return Ok(map.to_value());
    }
    Err(invalid(format!(
        "{NODE_NAME}{node_name} - cannot map '{rhs}' because '*' was mapped with a non-map value"
    )))
}

fn store_single_response(
    node_name: &str,
    node: &Arc<SimpleNode>,
    state: &mut MultiLevelMap,
    response: &TaskResponse,
) -> Result<String, AppError> {
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
                headers_to_value(&response.headers),
            )
            .map_err(invalid)?;
    }
    if response.has_error() {
        return set_error(node_name, node, state, response);
    }
    state
        .set_element(&format!("{node_name}.{RESULT}"), response.body.clone())
        .map_err(invalid)?;
    let output_mapping = get_entries(node.get_property("output"));
    perform_fetcher_output_mapping(node_name, state, &output_mapping)?;
    Ok(NEXT.to_string())
}

fn store_fork_join_response(
    node_name: &str,
    node: &Arc<SimpleNode>,
    state: &mut MultiLevelMap,
    response: &TaskResponse,
) -> Result<(), AppError> {
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
                headers_to_value(&response.headers),
            )
            .map_err(invalid)?;
    }
    if response.has_error() {
        set_error(node_name, node, state, response)?;
    } else {
        state
            .set_element(&format!("{node_name}.{RESULT}[]"), response.body.clone())
            .map_err(invalid)?;
    }
    Ok(())
}

fn set_error(
    node_name: &str,
    node: &Arc<SimpleNode>,
    state: &mut MultiLevelMap,
    response: &TaskResponse,
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
                .set_element("output.header", headers_to_value(&response.headers))
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

fn headers_to_value(headers: &HashMap<String, String>) -> Value {
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

fn uuid_simple() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

// ---- graph.join (Java GraphJoin) ----

pub async fn join(
    platform: &Platform,
    headers: HashMap<String, String>,
    _event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let (instance, node, node_name) = skill_context(platform, &headers, JOIN_ROUTE)?;
    let connected = instance.graph.get_backward_links(node.get_alias())?;
    let mut count = 0;
    for from in &connected {
        if node_completed(from, &instance) {
            count += 1;
        }
    }
    // a successful join means all upstream nodes have been seen
    let joined = count == connected.len();
    instance
        .node_seen
        .lock()
        .expect("node seen")
        .insert(node_name.to_string(), joined);
    EventEnvelope::new().set_body(if joined { NEXT } else { SINK })
}

fn node_completed(predecessor: &Arc<SimpleNode>, instance: &GraphInstance) -> bool {
    let name = predecessor.get_alias();
    if predecessor.get_property(SKILL).is_some() {
        instance
            .skill_run
            .lock()
            .expect("skill run")
            .contains_key(name)
    } else {
        instance
            .node_seen
            .lock()
            .expect("node seen")
            .contains_key(name)
    }
}

// ---- graph.island (Java GraphIsland) ----

pub async fn island(
    platform: &Platform,
    headers: HashMap<String, String>,
    _event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let po = PostOffice::new(platform);
    let node_name = headers.get(NODE).map(String::as_str).unwrap_or("none");
    po.annotate_trace(NODE, node_name);
    EventEnvelope::new().set_body(SINK)
}
