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

//! Shared skill/executor behaviors — Rust port of the Java
//! `GraphLambdaFunction` base class as free functions. The data-mapping
//! mini-language is the event-script one (`get_lhs_or_constant`, constants,
//! `f:` plugins, `$.…` JSONPath) applied to the graph's own state machine.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

use event_script::conversions::{display, to_json_string};
use event_script::mapping::{get_constant_value, get_lhs_element, get_lhs_or_constant};
use event_script::mlm::MultiLevelMap;
use event_script::util::{split, str2long};
use platform_core::graph::{MiniGraph, SimpleNode};
use platform_core::{AppConfigReader, AppError};
use rmpv::Value;

use crate::model::{self, GraphInstance};

pub const MAP_TO: &str = "->";
pub const NEXT: &str = "next";
pub const SINK: &str = ".sink";
pub const EXECUTE: &str = "execute";
pub const SKILL: &str = "skill";
pub const NODE: &str = "node";
pub const IN: &str = "in";
pub const TYPE: &str = "type";
pub const STATUS: &str = "status";
pub const ERROR: &str = "error";
pub const HEADER: &str = "header";
pub const RESULT: &str = "result";
pub const TARGET: &str = "target";
pub const EXCEPTION: &str = "exception";
pub const OUTPUT_BODY_NAMESPACE: &str = "output.body";
pub const OUTPUT_HEADER_NAMESPACE: &str = "output.header";
pub const NODE_NAME: &str = "node ";
pub const NOT_FOUND: &str = " not found";

const MODEL_NAMESPACE: &str = "model.";
const OUTPUT_NAMESPACE: &str = "output.";
const OUTPUT_ARRAY: &str = "output[";
const PLUGIN_PREFIX: &str = "f:";
const RESULT_NAMESPACE: &str = "result.";

pub const MAPPING_TAG: &str = "mapping:";
pub const COMPUTE_TAG: &str = "compute:";
pub const EXECUTE_TAG: &str = "execute:";
pub const RESET_TAG: &str = "reset:";
pub const IF_TAG: &str = "if:";
pub const THEN_TAG: &str = "then:";
pub const ELSE_TAG: &str = "else:";
pub const NEXT_TAG: &str = "next:";
pub const DELAY_TAG: &str = "delay:";
pub const BEGIN: &str = "begin";
pub const END: &str = "end";

/// Node properties owned by the engine, never copied into the state machine
/// and never a valid data-mapping RHS (Java `RESERVED_PARAMETERS`).
pub const RESERVED_PARAMETERS: &[&str] = &[
    "skill",
    "mapping",
    "statement",
    "input",
    "output",
    "feature",
    "exception",
    "extension",
    "status",
    "error",
    "dictionary",
    "for_each",
    "concurrency",
    "purpose",
    "task",
];

pub fn invalid(message: impl Into<String>) -> AppError {
    AppError::new(400, message)
}

pub fn get_graph_instance(id: &str) -> Result<Arc<GraphInstance>, AppError> {
    model::get_instance(id).ok_or_else(|| invalid(format!("Graph instance {id} not started")))
}

pub fn get_node(node_name: &str, graph: &MiniGraph) -> Result<Arc<SimpleNode>, AppError> {
    graph
        .find_node_by_alias(node_name)?
        .ok_or_else(|| invalid(format!("{NODE_NAME}{node_name}{NOT_FOUND}")))
}

/// `graph.max.loop.interval` (ms, floor 100, default 1000).
pub fn loop_interval() -> i64 {
    static CACHE: AtomicI64 = AtomicI64::new(-1);
    let cached = CACHE.load(Ordering::Relaxed);
    if cached > 0 {
        return cached;
    }
    let config = AppConfigReader::get_instance();
    let value = str2long(&config.get_property_or("graph.max.loop.interval", "1000")).max(100);
    CACHE.store(value, Ordering::Relaxed);
    value
}

/// `graph.node.high.frequency` (hits within the loop interval, floor 2,
/// default 10).
pub fn high_frequency() -> i64 {
    static CACHE: AtomicI64 = AtomicI64::new(-1);
    let cached = CACHE.load(Ordering::Relaxed);
    if cached > 0 {
        return cached;
    }
    let config = AppConfigReader::get_instance();
    let value = str2long(&config.get_property_or("graph.node.high.frequency", "10")).max(2);
    CACHE.store(value, Ordering::Relaxed);
    value
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

/// Java `substituteVarIfAny`: replace `{key}` segments with values resolved
/// from the state machine (constants, `f:` plugins, `$.…` and composite keys
/// all work). Strings are single-quoted when the expression is a logical one
/// and the segment is not part of a dotted variable name; maps/lists render
/// as compact JSON; a missing value renders as `null`. Segments containing
/// newlines, tabs or `:` are left as-is (likely JSON, not a variable).
pub fn substitute_var_if_any(text: &str, state: &MultiLevelMap) -> Result<String, AppError> {
    let logical = has_boolean_operator(text) || (text.starts_with("$.") && text.contains('@'));
    let (Some(left), Some(right)) = (text.find('{'), text.rfind('}')) else {
        return Ok(text.to_string());
    };
    if right <= left {
        return Ok(text.to_string());
    }
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::new();
    let mut start = 0usize;
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == '{' {
            if let Some(close) = chars[i..].iter().position(|c| *c == '}') {
                let seg_start = i;
                let seg_end = i + close + 1; // exclusive, includes '}'
                                             // heading before the segment
                let heading: String = chars[start..seg_start].iter().collect();
                if !heading.is_empty() {
                    out.push_str(&heading);
                }
                let mut dot = heading.ends_with('.');
                if seg_end < chars.len() && chars[seg_end] == '.' {
                    dot = true;
                }
                let key: String = chars[seg_start + 1..seg_end - 1].iter().collect();
                if key.contains('\r')
                    || key.contains('\n')
                    || key.contains('\t')
                    || key.contains(':')
                {
                    // likely an embedded JSON object, not a variable
                    let verbatim: String = chars[seg_start..seg_end].iter().collect();
                    out.push_str(&verbatim);
                } else {
                    append_parameter(&mut out, &key, state, logical, dot)?;
                }
                start = seg_end;
                i = seg_end;
                continue;
            }
            break;
        }
        i += 1;
    }
    let tail: String = chars[start..].iter().collect();
    out.push_str(&tail);
    Ok(out)
}

fn append_parameter(
    out: &mut String,
    key: &str,
    state: &MultiLevelMap,
    logical: bool,
    dot: bool,
) -> Result<(), AppError> {
    let parameter = get_lhs_or_constant(key, state).map_err(invalid)?;
    match parameter {
        None | Some(Value::Nil) => out.push_str("null"),
        Some(Value::Integer(_))
        | Some(Value::F32(_))
        | Some(Value::F64(_))
        | Some(Value::Boolean(_)) => out.push_str(&display(&parameter.expect("checked"))),
        Some(v @ Value::Map(_)) | Some(v @ Value::Array(_)) => {
            out.push_str(&to_json_string(&v));
        }
        Some(other) => {
            let text = display(&other);
            // quote only in a boolean (logical) operation when the value is
            // not part of a dotted variable name
            if logical && !dot {
                out.push('\'');
                out.push_str(&text.replace('\'', "\\'"));
                out.push('\'');
            } else {
                out.push_str(&text);
            }
        }
    }
    Ok(())
}

/// One `LHS -> RHS` data-mapping entry against the graph state machine
/// (Java `handleDataMappingEntry`).
pub fn handle_data_mapping_entry(
    node_name: &str,
    command: &str,
    state: &mut MultiLevelMap,
    graph: &MiniGraph,
) -> Result<(), AppError> {
    let Some(sep) = command.rfind(MAP_TO) else {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have '->' in '{command}'"
        )));
    };
    let lhs = substitute_var_if_any(command[..sep].trim(), state)?;
    let rhs = command[sep + MAP_TO.len()..].trim();
    let value = get_lhs_or_constant(&lhs, state).map_err(invalid)?;
    validate_rhs(node_name, rhs, graph)?;
    match value {
        Some(v) => {
            state.set_element(rhs, v).map_err(invalid)?;
        }
        None => {
            if rhs.ends_with(']') && rhs.contains('[') {
                state.set_element(rhs, Value::Nil).map_err(invalid)?;
            } else {
                state.remove_element(rhs);
            }
        }
    }
    Ok(())
}

fn validate_rhs(node_name: &str, rhs: &str, graph: &MiniGraph) -> Result<(), AppError> {
    if rhs.starts_with(OUTPUT_ARRAY)
        || rhs.starts_with(OUTPUT_NAMESPACE)
        || rhs.starts_with(MODEL_NAMESPACE)
    {
        return Ok(());
    }
    let parts = split(rhs, ".[]");
    if rhs.starts_with('.') || parts.len() < 2 {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} - invalid RHS ({rhs})"
        )));
    }
    if graph.find_node_by_alias(&parts[0])?.is_none() {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} - RHS node '{}' does not exist",
            parts[0]
        )));
    }
    if RESERVED_PARAMETERS.contains(&parts[1].as_str()) {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} - invalid RHS ({rhs}), '{}' is a reserved property",
            parts[1]
        )));
    }
    Ok(())
}

/// Copy node properties into the state machine: nodes with a skill get each
/// non-reserved property at `{node}.{key}`; nodes without a skill get the
/// whole property map at `{node}` (Java `initializeWithNodeProperties`).
pub fn initialize_with_node_properties(instance: &GraphInstance) -> Result<usize, AppError> {
    let mut state = instance.state.lock().expect("graph state machine");
    let nodes = instance.graph.get_nodes();
    for node in &nodes {
        let name = node.get_alias();
        let properties = node.get_properties();
        if properties.contains_key(SKILL) {
            for (key, value) in &properties {
                if !RESERVED_PARAMETERS.contains(&key.as_str()) {
                    state
                        .set_element(&format!("{name}.{key}"), value.clone())
                        .map_err(invalid)?;
                }
            }
        } else {
            state
                .set_element(name, properties_to_value(properties))
                .map_err(invalid)?;
        }
    }
    Ok(nodes.len())
}

fn properties_to_value(properties: HashMap<String, Value>) -> Value {
    let mut entries: Vec<(String, Value)> = properties.into_iter().collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    Value::Map(
        entries
            .into_iter()
            .map(|(k, v)| (Value::from(k.as_str()), v))
            .collect(),
    )
}

/// `RESET:` command — forget listed nodes COMPLETELY so they can run again
/// (Java `resetNodes`): the run-once guard, the completion mark, and the
/// node's state. Clearing `skill_run` matters for join barriers — a reset
/// (retrying) branch must not satisfy the barrier until it re-executes
/// successfully.
pub fn reset_nodes(command: &str, instance: &GraphInstance, state: &mut MultiLevelMap) {
    let names = split(command, "[],; ");
    {
        let mut seen = instance.node_seen.lock().expect("node seen");
        for name in &names {
            seen.remove(name);
        }
    }
    {
        let mut run = instance.skill_run.lock().expect("skill run");
        for name in &names {
            run.remove(name);
        }
    }
    for name in &names {
        if let Ok(Some(_)) = instance.graph.find_node_by_alias(name) {
            state.remove_element(name);
        }
    }
}

/// `model.ttl` with a 30s default and a 1s floor (Java `getModelTtl`).
pub fn get_model_ttl(state: &mut MultiLevelMap) -> i64 {
    if !state.exists("model") {
        let _ = state.set_element("model", Value::Map(vec![]));
    }
    let ttl = state
        .get_element("model.ttl")
        .map(|v| display(&v))
        .unwrap_or_else(|| "30000".to_string());
    str2long(&ttl).max(1000)
}

/// A list property rendered as strings (Java `getEntries`).
pub fn get_entries(entries: Option<Value>) -> Vec<String> {
    match entries {
        Some(Value::Array(items)) => items.iter().map(display).collect(),
        _ => Vec::new(),
    }
}

/// Resolve `for_each` entries: list-valued LHS become the iteration arrays
/// (all must agree on size), scalar values are set/removed directly
/// (Java `getForEachMapping`).
pub fn get_for_each_mapping(
    node_name: &str,
    for_each: &[String],
    state: &mut MultiLevelMap,
) -> Result<Vec<(String, Vec<Value>)>, AppError> {
    let mut size: i64 = -1;
    let mut mappings: Vec<(String, Vec<Value>)> = Vec::new();
    for entry in for_each {
        let Some(sep) = entry.rfind(MAP_TO) else {
            return Err(invalid(format!(
                "{NODE_NAME}{node_name} does not have '->' in '{entry}'"
            )));
        };
        let lhs = substitute_var_if_any(entry[..sep].trim(), state)?;
        let rhs = entry[sep + MAP_TO.len()..].trim();
        let parts = split(rhs, ".");
        if parts.len() < 2 || parts[0] != "model" {
            return Err(invalid(format!(
                "{NODE_NAME}{node_name} RHS of 'for_each' entry must use 'model.' namespace. \
                 Actual: {entry}"
            )));
        }
        let value = get_lhs_or_constant(&lhs, state).map_err(invalid)?;
        match value {
            Some(Value::Array(list)) => {
                if size == -1 {
                    size = list.len() as i64;
                } else if size != list.len() as i64 {
                    return Err(invalid(format!(
                        "{NODE_NAME}{node_name} LHS of 'for_each' contains inconsistent array sizes"
                    )));
                }
                mappings.push((rhs.to_string(), list));
            }
            Some(v) => {
                state.set_element(rhs, v).map_err(invalid)?;
            }
            None => state.remove_element(rhs),
        }
    }
    Ok(mappings)
}

pub fn get_model_array_size(mappings: &[(String, Vec<Value>)]) -> usize {
    mappings.first().map(|(_, list)| list.len()).unwrap_or(0)
}

pub fn get_next_model_param_set(
    mappings: &[(String, Vec<Value>)],
    index: usize,
) -> Vec<(String, Value)> {
    mappings
        .iter()
        .map(|(key, list)| (key.clone(), list[index].clone()))
        .collect()
}

/// Output data mapping for task/fetcher nodes: bare `result` selectors are
/// namespaced under the node, LHS/RHS namespaces are enforced
/// (Java `performFetcherOutputMapping` + `setFetcherOutputEntry`).
pub fn perform_fetcher_output_mapping(
    node_name: &str,
    state: &mut MultiLevelMap,
    mapping: &[String],
) -> Result<(), AppError> {
    for output in mapping {
        let text = output.trim();
        let Some(sep) = text.rfind(MAP_TO) else {
            return Err(invalid(format!(
                "{NODE_NAME}{node_name} - invalid output mapping: {text}"
            )));
        };
        let lhs = substitute_var_if_any(text[..sep].trim(), state)?;
        let rhs = text[sep + MAP_TO.len()..].trim();
        set_fetcher_output_entry(node_name, &lhs, rhs, state)?;
    }
    Ok(())
}

fn set_fetcher_output_entry(
    node_name: &str,
    lhs: &str,
    rhs: &str,
    state: &mut MultiLevelMap,
) -> Result<(), AppError> {
    let mut lhs = lhs.to_string();
    let mut value = get_constant_value(&lhs);
    if value.is_none() {
        if !lhs.starts_with(PLUGIN_PREFIX) {
            // reconstruct lhs with the node name as namespace
            if lhs == RESULT || lhs.starts_with(RESULT_NAMESPACE) || lhs.starts_with("result[") {
                lhs = format!("{node_name}.{lhs}");
            } else if let Some(rest) = lhs.strip_prefix("$.result") {
                lhs = format!("$.{node_name}.result{rest}");
            } else if !lhs.starts_with(&format!("{node_name}."))
                && !lhs.starts_with(MODEL_NAMESPACE)
                && !lhs.starts_with("$.model.")
            {
                return Err(invalid(format!(
                    "Invalid output data mapping in API fetcher {node_name} - LHS must start \
                     with 'model.', 'result.' namespace or '{node_name}.'"
                )));
            }
        }
        value = get_lhs_element(&lhs, state).map_err(invalid)?;
    }
    if let Some(v) = value {
        if !rhs.starts_with(MODEL_NAMESPACE) && !rhs.starts_with(OUTPUT_NAMESPACE) {
            return Err(invalid(format!(
                "Invalid output data mapping in data dictionary {node_name} - RHS must start \
                 with 'model.' or 'output.' namespace"
            )));
        }
        state.set_element(rhs, v).map_err(invalid)?;
    }
    Ok(())
}

/// Normalize an error payload into the `{target?, type, message}` map or a
/// pass-through of a map-shaped error (Java `getErrorMap`).
pub fn get_error_map(error: Option<Value>, target: Option<Value>) -> Value {
    let mut result: Vec<(Value, Value)> = Vec::new();
    if let Some(Value::String(_)) = &target {
        result.push((Value::from(TARGET), target.expect("checked")));
    }
    match error {
        Some(Value::Map(entries)) => {
            for (k, v) in entries {
                result.push((k, v));
            }
        }
        other => {
            result.push((Value::from(TYPE), Value::from(ERROR)));
            result.push((
                Value::from("message"),
                other.unwrap_or_else(|| Value::from("null")),
            ));
        }
    }
    Value::Map(result)
}

// ---- graph.math statement helpers (Java GraphLambdaFunction tail) ----

/// Validate statement keywords and count `EXECUTE:` lines
/// (Java `countExecuteStatements`).
pub fn count_execute_statements(node_name: &str, statements: &[String]) -> Result<usize, AppError> {
    let mut execute = 0;
    let mut js = 0;
    let mut error = 0;
    for entry in statements {
        let line = entry.trim().to_lowercase();
        if line.starts_with(IF_TAG)
            || line.starts_with(COMPUTE_TAG)
            || line.starts_with(RESET_TAG)
            || line.starts_with(DELAY_TAG)
        {
            js += 1;
        } else if line.starts_with(EXECUTE_TAG) {
            execute += 1;
            js += 1;
        } else if !line.starts_with(MAPPING_TAG)
            && !line.starts_with(NEXT_TAG)
            && line != BEGIN
            && line != END
        {
            error += 1;
        }
    }
    if js == 0 {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} must include 'IF:', 'COMPUTE:', 'EXECUTE:', 'RESET:' or \
             'DELAY:' statements"
        )));
    }
    if error > 0 {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} must use 'IF:', 'COMPUTE:', 'EXECUTE:', 'RESET:', \
             'MAPPING:', 'NEXT:', 'DELAY:', 'BEGIN' or 'END' keywords"
        )));
    }
    Ok(execute)
}

pub fn get_first_word(statement: &str) -> &str {
    match statement.find(' ') {
        Some(space) => &statement[..space],
        None => statement,
    }
}

fn collect_tagged(lines: &[String], own_tag: &str, stop_tags: &[&str]) -> String {
    let mut found = false;
    let mut out = String::new();
    for line in lines {
        let lc = line.to_lowercase().trim().to_string();
        if found {
            if stop_tags.iter().any(|t| lc.starts_with(t)) {
                break;
            }
            out.push_str(line);
            out.push(' ');
        }
        if lc.starts_with(own_tag) {
            out.push_str(&line[own_tag.len()..]);
            out.push(' ');
            found = true;
        }
    }
    out.trim().to_string()
}

pub fn get_if_statement(lines: &[String]) -> String {
    collect_tagged(lines, IF_TAG, &[THEN_TAG, ELSE_TAG])
}

pub fn get_then_statement(lines: &[String]) -> String {
    collect_tagged(lines, THEN_TAG, &[IF_TAG, ELSE_TAG])
}

pub fn get_else_statement(lines: &[String]) -> String {
    collect_tagged(lines, ELSE_TAG, &[IF_TAG, THEN_TAG])
}

/// Resolve a decision branch: `next` means continue, anything else must be
/// an existing node to jump to (Java `getNext(MiniGraph, String)`).
pub fn get_next_node(graph: &MiniGraph, statement: &str) -> Result<Option<String>, AppError> {
    if !statement.eq_ignore_ascii_case(NEXT) {
        get_node(statement, graph)?;
        return Ok(Some(statement.to_string()));
    }
    Ok(None)
}

/// The `NEXT:` jump target, if the tag is `next:` (Java `getNext(tag, command)`).
pub fn get_next_tag(tag: &str, command: &str) -> Option<String> {
    if tag == NEXT_TAG {
        let names = split(command, " ,");
        if !names.is_empty() {
            return Some(names[0].clone());
        }
    }
    None
}

/// Inline `EXECUTE:` statement merge (Java `combine` + `mergeStatements`).
/// Note the Java behavior: merged lines are lowercased.
pub fn combine(
    route: &str,
    node_name: &str,
    graph: &MiniGraph,
    entries: &[String],
) -> Result<Vec<String>, AppError> {
    let mut merged: Vec<String> = Vec::new();
    for entry in entries {
        let line = entry.trim().to_lowercase();
        if let Some(another) = line.strip_prefix(EXECUTE_TAG) {
            merge_statements(route, node_name, another.trim(), graph, &mut merged)?;
        } else {
            merged.push(line);
        }
    }
    Ok(merged)
}

fn merge_statements(
    route: &str,
    node_name: &str,
    another_node: &str,
    graph: &MiniGraph,
    merged: &mut Vec<String>,
) -> Result<(), AppError> {
    let Some(that) = graph.find_node_by_alias(another_node)? else {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} is referring to non-existing node '{another_node}'"
        )));
    };
    let that_skill = that.get_property(SKILL).map(|v| display(&v));
    if that_skill.as_deref() != Some(route) {
        return Err(invalid(format!(
            "{NODE_NAME}{another_node} does not have skill - {route}"
        )));
    }
    let other_statements = get_entries(that.get_property("statement"));
    if count_execute_statements(another_node, &other_statements)? > 0 {
        return Err(invalid(format!(
            "{NODE_NAME}{another_node} contains nested EXECUTE statements"
        )));
    }
    merged.extend(other_statements);
    Ok(())
}

/// Split statements into pre/each/post blocks around BEGIN/END
/// (Java `splitBlocks` + `distributeBlocks`).
pub fn split_blocks(statements: &[String]) -> [Vec<String>; 3] {
    let has_begin = statements
        .iter()
        .any(|entry| entry.trim().eq_ignore_ascii_case(BEGIN));
    let mut blocks: [Vec<String>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    if !has_begin {
        // no BEGIN: the whole block runs inside the for-each
        blocks[1] = statements.to_vec();
        return blocks;
    }
    let mut n = 0usize;
    for entry in statements {
        let text = entry.trim();
        match n {
            0 => {
                if text.eq_ignore_ascii_case(BEGIN) {
                    n += 1;
                } else {
                    blocks[0].push(text.to_string());
                }
            }
            1 => {
                if text.eq_ignore_ascii_case(END) {
                    n += 1;
                } else {
                    blocks[1].push(text.to_string());
                }
            }
            _ => blocks[2].push(text.to_string()),
        }
    }
    blocks
}

// ---- API-fetcher helpers (Java GraphLambdaFunction fetcher tail) ----

/// Stage one fetcher `input` entry: a `model.*` RHS writes through; anything
/// else lands in the fetcher's staging area — `{node}.fetch.{rhs}` for a
/// single call, `{node}.each.{rhs}[]` per for-each iteration
/// (Java `fillFetcherApiParameters`).
pub fn fill_fetcher_api_parameters(
    node_name: &str,
    command: &str,
    state: &mut MultiLevelMap,
    is_array: bool,
) -> Result<(), AppError> {
    let Some(sep) = command.rfind(MAP_TO) else {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have '->' in '{command}'"
        )));
    };
    let lhs = substitute_var_if_any(command[..sep].trim(), state)?;
    let rhs = command[sep + MAP_TO.len()..].trim();
    let target = if rhs.starts_with(MODEL_NAMESPACE) {
        rhs.to_string()
    } else if is_array {
        format!("{node_name}.each.{rhs}[]")
    } else {
        format!("{node_name}.fetch.{rhs}")
    };
    let value = get_lhs_or_constant(&lhs, state).map_err(invalid)?;
    match value {
        Some(v) => state.set_element(&target, v).map_err(invalid)?,
        None => {
            if target.ends_with(']') && target.contains('[') {
                state.set_element(&target, Value::Nil).map_err(invalid)?;
            } else {
                state.remove_element(&target);
            }
        }
    }
    Ok(())
}

/// Resolve a provider-input LHS: constant, then the state machine, then the
/// dictionary's API-input staging area (Java `getValueBestEffort`).
fn get_value_best_effort(
    node_name: &str,
    dd_name: &str,
    lhs: &str,
    state: &MultiLevelMap,
) -> Result<Option<Value>, AppError> {
    if let Some(constant) = get_constant_value(lhs) {
        return Ok(Some(constant));
    }
    if let Some(value) = get_lhs_element(lhs, state).map_err(invalid)? {
        return Ok(Some(value));
    }
    get_lhs_element(&format!("{node_name}.dd.{dd_name}.{lhs}"), state).map_err(invalid)
}

/// Build the HTTP request parts from a provider's `input` mapping
/// (Java `mapHttpInput` + `mapHttpParams`): `path_parameter.*`, `query.*`
/// and `header.*` namespaces, `body.*` fields, or the whole body.
pub fn map_http_input(
    mut request: platform_core::automation::AsyncHttpRequest,
    node_name: &str,
    dd_name: &str,
    state: &MultiLevelMap,
    mapping: &[String],
) -> Result<platform_core::automation::AsyncHttpRequest, AppError> {
    let mut body = MultiLevelMap::new();
    let mut whole_body: Option<Value> = None;
    let mut has_fields = false;
    for entry in mapping {
        let Some(sep) = entry.rfind(MAP_TO) else {
            continue;
        };
        let lhs = entry[..sep].trim();
        let rhs = entry[sep + MAP_TO.len()..].trim();
        let Some(value) = get_value_best_effort(node_name, dd_name, lhs, state)? else {
            continue;
        };
        if let Some(key) = rhs.strip_prefix("path_parameter.") {
            request = request.set_path_parameter(key.trim(), &display(&value));
        } else if let Some(key) = rhs.strip_prefix("query.") {
            request = request.set_query_parameter(key.trim(), &display(&value));
        } else if let Some(key) = rhs.strip_prefix("header.") {
            request = request.set_header(key.trim(), &display(&value));
        } else if let Some(key) = rhs.strip_prefix("body.") {
            let key = key.trim();
            if key.is_empty() {
                whole_body = Some(value);
            } else {
                body.set_element(key, value).map_err(invalid)?;
                has_fields = true;
            }
        } else if rhs == "body" {
            whole_body = Some(value);
        } else {
            body.set_element(rhs, value).map_err(invalid)?;
            has_fields = true;
        }
    }
    // a whole-body mapping wins over individual fields; Java always sets a
    // body (an empty map when nothing mapped)
    let _ = has_fields;
    Ok(match whole_body {
        Some(whole) => request.set_body(whole),
        None => request.set_body(body.to_value()),
    })
}
