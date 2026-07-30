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

//! Rust port of `com.accenture.minigraph.start.CompileGraph` — the quality
//! gate for graph models, mirroring what the flow compiler does for event
//! flows:
//!
//! 1. **Structural validation** — every node/connection is imported once via
//!    `MiniGraph::import_graph`, catching missing/duplicate aliases, invalid
//!    types and dangling connections at startup.
//! 2. **Syntax conversion** — the deprecated "simple type matching" syntax
//!    (`model.someKey:type`) in `mapping`, `input`, `output` and `for_each`
//!    node properties is converted once to the equivalent "simple plugin"
//!    syntax (`f:type(model.someKey)`), instead of on every node execution.
//!    A `mapping`/`output`/`for_each` entry without `->` rejects the graph
//!    (it is guaranteed to fail at runtime); an `input` entry without `->`
//!    is skill vocabulary (e.g. the fetcher's dictionary parameter names)
//!    and passes through.
//! 3. **Discovery contract and completeness** — every deployable graph must
//!    document itself (the root node needs a non-empty 'purpose' property —
//!    what "list graphs" shows as living documentation) and must have an
//!    'end' node so every run can complete.
//! 4. **Suspend/resume contract** — the static half of the workflow-suspension
//!    rules ([`crate::model_validator`]); the runtime guards remain the
//!    enforcement floor for the playground dry-run surface.
//!
//! CompileGraph is the deployment gate: set `graph.model.automation` to a
//! YAML file listing the graph ids to compile at startup (mirroring
//! `yaml.flow.automation` for event flows). Like flows.yaml, the manifest
//! carries the location of its own models in an optional `location` entry
//! (file:/ or classpath:/, default `classpath:/graph`) — there is no separate
//! application.properties key. A deployed graph model is executable ONLY when
//! it is listed in the manifest and passes this gate — a graph that fails, or
//! is not listed, answers HTTP-404 as if it does not exist. This is the
//! CompileFlows precedent: an invalid flow never becomes executable, and
//! there is no lazy loading of unvalidated models. Ad-hoc graphs created
//! interactively through the dev playground are intentionally out of scope
//! since they are not known ahead of time (the playground dry-run runs from
//! its own temp workspace).

use event_script::converter;
use event_script::mlm::MultiLevelMap;
use platform_core::graph::MiniGraph;
use platform_core::{AppConfigReader, ConfigReader, ConfigValue};
use rmpv::Value;

use crate::graphs;
use crate::model_validator;

const INPUT: &str = "input";
const MAPPING_PROPERTIES: &[&str] = &["mapping", INPUT, "output", "for_each"];
const MAP_TO: &str = "->";
const LOCATION: &str = "location";
const DEFAULT_DEPLOY_DIR: &str = "classpath:/graph";
const FILE_PREFIX: &str = "file:/";
const CLASSPATH_PREFIX: &str = "classpath:/";

/// Compile and register every graph model listed by `graph.model.automation`.
/// Returns the ids of all graphs in the registry (Java logs the same count).
pub fn compile_graphs() -> Vec<String> {
    let config = AppConfigReader::get_instance();
    if !config
        .get_property_or("location.graph.deployed", "")
        .trim()
        .is_empty()
    {
        log::warn!(
            "location.graph.deployed is obsolete - \
             set 'location' in the graph manifest (graph.model.automation) instead"
        );
    }
    let manifest = config.get_property_or("graph.model.automation", "");
    if manifest.trim().is_empty() {
        log::warn!(
            "No graph manifest configured (graph.model.automation) - \
             no deployed graph models will be executable"
        );
        return graphs::get_all_graphs();
    }
    match ConfigReader::load(&manifest) {
        Ok(reader) => {
            // like flows.yaml, the manifest carries the location of its own models
            let mut deploy_location = reader
                .get_property(LOCATION)
                .unwrap_or_else(|| DEFAULT_DEPLOY_DIR.to_string());
            if !deploy_location.starts_with(FILE_PREFIX)
                && !deploy_location.starts_with(CLASSPATH_PREFIX)
            {
                log::warn!(
                    "Graph manifest 'location' must start with file:/ or classpath:/. \
                     Fallback to {DEFAULT_DEPLOY_DIR}"
                );
                deploy_location = DEFAULT_DEPLOY_DIR.to_string();
            }
            graphs::set_deployed_location(&deploy_location);
            log::info!("Deployed graph model folder - {deploy_location}");
            if let Some(ConfigValue::List(list)) = reader.get("graphs") {
                for i in 0..list.len() {
                    if let Some(graph_id) = reader.get_property(&format!("graphs[{i}]")) {
                        compile_one_graph(&deploy_location, &graph_id);
                    }
                }
            }
        }
        Err(e) => log::warn!("Unable to load graph manifest {manifest} - {e}"),
    }
    let all = graphs::get_all_graphs();
    log::info!("Graph models compiled: {}", all.len());
    all
}

fn compile_one_graph(deploy_location: &str, graph_id: &str) {
    match load_and_validate(deploy_location, graph_id) {
        Ok(model) => {
            graphs::add_graph(graph_id, model);
            log::info!("Compiled graph {graph_id}");
        }
        // a rejected graph is simply not registered: deployed execution is served
        // exclusively from the compiled registry, so requests to it answer 404
        Err(e) => log::error!("Rejected graph {graph_id} - {e}"),
    }
}

/// Load a graph JSON as an rmpv value with `${...}` references resolved —
/// the raw form the startup compiler shares with the playground's temp
/// workspace import (Java uses `ConfigReader` in both places).
pub(crate) fn load_raw_graph(deploy_location: &str, graph_id: &str) -> Result<Value, String> {
    let reader = ConfigReader::load(&normalized_path(deploy_location, graph_id)).map_err(|e| {
        if matches!(e, platform_core::ConfigError::NotFound(_)) {
            format!("{graph_id} not found")
        } else {
            e.to_string()
        }
    })?;
    let json = ConfigValue::Map(reader.get_map().clone().into_map()).to_json();
    Ok(event_script::conversions::from_json(&json))
}

fn load_and_validate(deploy_location: &str, graph_id: &str) -> Result<Value, String> {
    // the ConfigReader load resolves ${...} references against the app
    // config, exactly like the Java loader
    let mut model = load_raw_graph(deploy_location, graph_id)?;
    convert_data_mapping_entries(graph_id, &mut model)?;
    // structural validation - a malformed graph is rejected with an error log
    let graph = MiniGraph::new();
    graph
        .import_graph(&model)
        .map_err(|e| e.message().to_string())?;
    // discovery contract: every deployable graph documents itself - the root
    // node's 'purpose' is what `list graphs` shows as living documentation
    if !has_root_purpose(&model) {
        return Err("root node must define a non-empty 'purpose' property".to_string());
    }
    // every run must be able to complete - the graph executor trusts this at runtime
    if graph.get_end_node().is_none() {
        return Err("graph must have an 'end' node".to_string());
    }
    model_validator::validate_suspend_resume(&graph)?;
    Ok(model)
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
            // (this module is a quality gate - a compiled graph must be runnable)
            return Err(format!(
                "node [{node_index}].{property} - missing '{MAP_TO}' in '{line}'"
            ));
        }
    }
    Ok(converted)
}

/// Java `getNormalizedPath`: rejoin the folder on single slashes, keep the
/// scheme prefix, append `<graph-id>.json`.
fn normalized_path(folder: &str, graph_id: &str) -> String {
    let parts: Vec<&str> = folder.split('/').filter(|p| !p.is_empty()).collect();
    format!("{}/{graph_id}.json", parts.join("/"))
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
}
