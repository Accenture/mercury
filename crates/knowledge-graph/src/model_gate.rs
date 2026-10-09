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

//! Rust port of `com.accenture.minigraph.common.GraphModelGate` — the static
//! checks of the deployment gate, as one function that the compiler
//! ([`crate::compiler`]), the graph packager and the deployment of packaged
//! graph sets share, so "passes the gate" means the same thing in every place
//! (RFC-0005).
//!
//! Every check reads the model alone — its structure, the root node's
//! 'purpose', an 'end' node, the data mapping syntax and the rules of
//! [`crate::model_validator`] — and none consults the functions of the target
//! application, so the gate runs as well when a graph set is packed as when an
//! application starts.

use event_script::converter;
use event_script::mlm::MultiLevelMap;
use platform_core::graph::MiniGraph;
use rmpv::Value;

use crate::model_validator;

const INPUT: &str = "input";
const MAPPING_PROPERTIES: &[&str] = &["mapping", INPUT, "output", "for_each"];
const MAP_TO: &str = "->";

/// Validate a graph model the way the deployment gate does. The deprecated
/// "simple type matching" syntax (`model.someKey:type`) in the `mapping`,
/// `input`, `output` and `for_each` properties is converted in place to the
/// equivalent simple plugin syntax (`f:type(model.someKey)`), so the model a
/// caller registers afterward is the converted one.
///
/// Returns the imported graph, or the reason the model is rejected.
pub fn validate(graph_id: &str, model: &mut Value) -> Result<MiniGraph, String> {
    convert_data_mapping_entries(graph_id, model)?;
    // structural validation - a malformed graph is rejected
    let graph = MiniGraph::new();
    graph
        .import_graph(model)
        .map_err(|e| e.message().to_string())?;
    // discovery contract: every deployable graph documents itself - the root
    // node's 'purpose' is what `list graphs` shows as living documentation
    if !has_root_purpose(model) {
        return Err("root node must define a non-empty 'purpose' property".to_string());
    }
    // every run must be able to complete - the graph executor trusts this at runtime
    if graph.get_end_node().is_none() {
        return Err("graph must have an 'end' node".to_string());
    }
    model_validator::validate(&graph)?;
    // a mismatch between the declared contract and the model's data surface warns
    // (RFC-0007): a declared path the model never reads, or a path the model reads
    // that the declaration lacks (Java `GraphModelGate.validate`)
    if let Some(json) = event_script::conversions::to_json(model) {
        for issue in crate::contract::GraphContract::derive(graph_id, &json, &|_| None).issues() {
            log::warn!("Graph {graph_id} - {issue}");
        }
    }
    Ok(graph)
}

/// The file-name rule for a graph id: letters, digits, `_` and `-` only, so an
/// id is always a safe file name.
pub fn is_valid_graph_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

/// The name the root node declares for its graph, as `export graph as` writes
/// it: the root node's `name` property, trimmed; empty when there is none.
pub fn declared_root_name(model: &Value) -> String {
    let mm = MultiLevelMap::from_value(model.clone());
    let Some(Value::Array(nodes)) = mm.get_element("nodes") else {
        return String::new();
    };
    for i in 0..nodes.len() {
        if mm.get_element(&format!("nodes[{i}].alias")) == Some(Value::from("root")) {
            return match mm.get_element(&format!("nodes[{i}].properties.name")) {
                None | Some(Value::Nil) => String::new(),
                Some(name) => event_script::conversions::display(&name).trim().to_string(),
            };
        }
    }
    String::new()
}

/// A graph holds no null property: a map entry whose value is null is filtered
/// out, at every depth, as the serializers do by default and as the Java
/// configuration reader does when an application loads a deployed graph.
/// Every other value is kept as it is - an empty string (`"key": ""`) is a
/// value - and a list keeps its elements in place (Java
/// `GraphModelGate.withoutNullProperties`).
///
/// This is the serializers' strip without their switch: a graph drops its
/// null properties whatever `serializer.null.transport` says, because that
/// switch governs what the event transport keeps, not what a graph holds.
pub fn without_null_properties(model: &Value) -> Value {
    platform_core::serializer::strip_nulls_always(model)
}

/// A deployed graph the way the Java engine reads it: its configuration reader
/// normalizes a graph by flattening it to composite keys and rebuilding it
/// (`Utility.getFlatMap`, then `MultiLevelMap.setElement`), and only a value
/// that carries something gets a key. So a null, an empty map and an empty
/// list - and a map or list left empty by them - disappear from a map, while
/// inside a list such an element keeps its place as null when an element with
/// a value follows it and is dropped at the end. An empty string is a value.
/// This engine's configuration reader keeps those values (and already splits a
/// dotted key into nested maps the same way), so the graph read applies this
/// after it, and both engines deploy the same model; the shared vectors in
/// `tests/resources/graph-read-normalization-vectors.json` pin it.
pub fn normalize_graph(model: &Value) -> Value {
    with_value(model).unwrap_or_else(|| Value::Map(Vec::new()))
}

/// `None` when the value carries nothing - the Java flattening gives it no key.
fn with_value(value: &Value) -> Option<Value> {
    match value {
        Value::Nil => None,
        Value::Map(entries) => {
            let kept: Vec<(Value, Value)> = entries
                .iter()
                .filter_map(|(k, v)| with_value(v).map(|v| (k.clone(), v)))
                .collect();
            (!kept.is_empty()).then_some(Value::Map(kept))
        }
        Value::Array(items) => {
            let mut kept: Vec<Value> = items
                .iter()
                .map(|v| with_value(v).unwrap_or(Value::Nil))
                .collect();
            while matches!(kept.last(), Some(Value::Nil)) {
                kept.pop();
            }
            (!kept.is_empty()).then_some(Value::Array(kept))
        }
        other => Some(other.clone()),
    }
}

fn has_root_purpose(model: &Value) -> bool {
    let mm = MultiLevelMap::from_value(model.clone());
    let Some(Value::Array(nodes)) = mm.get_element("nodes") else {
        return false;
    };
    for i in 0..nodes.len() {
        if mm.get_element(&format!("nodes[{i}].alias")) == Some(Value::from("root")) {
            return matches!(
                mm.get_element(&format!("nodes[{i}].properties.purpose")),
                Some(Value::String(text)) if !text.as_str().unwrap_or_default().trim().is_empty()
            );
        }
    }
    false
}

fn convert_data_mapping_entries(graph_id: &str, model: &mut Value) -> Result<(), String> {
    let mut mm = MultiLevelMap::from_value(model.clone());
    let node_count = match mm.get_element("nodes") {
        Some(Value::Array(nodes)) => nodes.len(),
        _ => return Ok(()),
    };
    for i in 0..node_count {
        for key in MAPPING_PROPERTIES {
            let path = format!("nodes[{i}].properties.{key}");
            if let Some(Value::Array(entries)) = mm.get_element(&path) {
                let converted = convert_entries(graph_id, i, key, &entries)?;
                if mm.set_element(&path, Value::Array(converted)).is_err() {
                    log::error!("Unable to update {path} in graph {graph_id}");
                }
            }
        }
    }
    *model = mm.to_value();
    Ok(())
}

fn convert_entries(
    graph_id: &str,
    node_index: usize,
    property: &str,
    entries: &[Value],
) -> Result<Vec<Value>, String> {
    let mut converted = Vec::with_capacity(entries.len());
    for entry in entries {
        let line = event_script::conversions::display(entry);
        if line.contains(MAP_TO) {
            let converted_line = converter::convert(&line);
            if converted_line != line {
                log::warn!(
                    "Deprecated syntax in graph {graph_id} node[{node_index}].{property} - \
                     '{line}' converted to '{converted_line}'"
                );
            }
            converted.push(Value::from(converted_line));
        } else if property == INPUT {
            // an 'input' entry without '->' is skill vocabulary, not a data mapping -
            // e.g. the fetcher's dictionary parameter names and feature flags
            converted.push(Value::from(line));
        } else {
            // a mapping/for_each/output entry is always a data mapping: a line
            // without '->' is guaranteed to fail at runtime, so reject the graph
            // (the gate's promise is that a compiled graph is runnable)
            return Err(format!(
                "node [{node_index}].{property} - missing '{MAP_TO}' in '{line}'"
            ));
        }
    }
    Ok(converted)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Property-aware mapping-entry rejection (the fetcher-vocabulary nuance):
    /// a bare `input` entry is skill vocabulary (e.g. dictionary parameter
    /// names) and passes; the same shape in mapping/for_each/output is a
    /// guaranteed runtime failure, so the gate rejects the graph.
    #[test]
    fn bare_input_entries_are_vocabulary_not_mappings() {
        let entries = vec![Value::from("payload"), Value::from("dictionary")];
        let passed = convert_entries("g", 0, "input", &entries).expect("input passes");
        assert_eq!(entries, passed);
        for property in ["mapping", "for_each", "output"] {
            let err = convert_entries("g", 3, property, &entries)
                .expect_err("a bare data-mapping entry must reject the graph");
            assert_eq!(
                format!("node [3].{property} - missing '->' in 'payload'"),
                err
            );
        }
    }

    #[test]
    fn a_null_property_is_filtered_out_and_an_empty_string_kept() {
        let model = Value::Map(vec![
            (Value::from("note"), Value::Nil),
            (Value::from("empty"), Value::from("")),
            (
                Value::from("nested"),
                Value::Array(vec![
                    Value::Map(vec![
                        (Value::from("x"), Value::Nil),
                        (Value::from("y"), Value::from(1)),
                    ]),
                    Value::Nil,
                ]),
            ),
        ]);
        let expected = Value::Map(vec![
            (Value::from("empty"), Value::from("")),
            (
                Value::from("nested"),
                Value::Array(vec![
                    Value::Map(vec![(Value::from("y"), Value::from(1))]),
                    Value::Nil,
                ]),
            ),
        ]);
        assert_eq!(expected, without_null_properties(&model));
    }

    #[test]
    fn the_graph_id_rule_is_the_file_name_rule() {
        assert!(is_valid_graph_id("tutorial-1"));
        assert!(is_valid_graph_id("A_b-9"));
        for bad in ["", "a.b", "../a", "a b", "é"] {
            assert!(!is_valid_graph_id(bad), "{bad}");
        }
    }
}
