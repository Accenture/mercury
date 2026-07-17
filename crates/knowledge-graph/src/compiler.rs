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
//!
//! Opt-in (Java parity): set `graph.model.automation` to a YAML file listing
//! the graph ids to compile at startup. Graph ids not listed continue to be
//! loaded lazily by the graph executor, so this is purely additive. Ad-hoc
//! graphs created interactively through the dev playground are out of scope.

use event_script::converter;
use event_script::mlm::MultiLevelMap;
use platform_core::graph::MiniGraph;
use platform_core::{AppConfigReader, ConfigReader, ConfigValue};
use rmpv::Value;

use crate::graphs;

const MAPPING_PROPERTIES: &[&str] = &["mapping", "input", "output", "for_each"];
const MAP_TO: &str = "->";

/// Compile and register every graph model listed by `graph.model.automation`.
/// Returns the ids of all graphs in the registry (Java logs the same count).
pub fn compile_graphs() -> Vec<String> {
    let config = AppConfigReader::get_instance();
    let manifest = config.get_property_or("graph.model.automation", "");
    if manifest.trim().is_empty() {
        log::info!(
            "No graph manifest configured (graph.model.automation) - skipping graph compilation"
        );
        return graphs::get_all_graphs();
    }
    let deploy_location = config.get_property_or("location.graph.deployed", "classpath:/graph");
    match ConfigReader::load(&manifest) {
        Ok(reader) => {
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
        Err(e) => log::error!("Skip invalid graph {graph_id} - {e}"),
    }
}

fn load_and_validate(deploy_location: &str, graph_id: &str) -> Result<Value, String> {
    // the ConfigReader load resolves ${...} references against the app
    // config, exactly like the Java loader
    let reader = ConfigReader::load(&normalized_path(deploy_location, graph_id))
        .map_err(|e| e.to_string())?;
    let json = ConfigValue::Map(reader.get_map().clone().into_map()).to_json();
    let mut model = event_script::conversions::from_json(&json);
    convert_data_mapping_entries(graph_id, &mut model);
    // structural validation - a malformed graph is skipped with an error log
    MiniGraph::new()
        .import_graph(&model)
        .map_err(|e| e.message().to_string())?;
    Ok(model)
}

fn convert_data_mapping_entries(graph_id: &str, model: &mut Value) {
    let mut mm = MultiLevelMap::from_value(model.clone());
    let node_count = match mm.get_element("nodes") {
        Some(Value::Array(nodes)) => nodes.len(),
        _ => return,
    };
    for i in 0..node_count {
        for key in MAPPING_PROPERTIES {
            let path = format!("nodes[{i}].properties.{key}");
            if let Some(Value::Array(entries)) = mm.get_element(&path) {
                let converted = convert_entries(graph_id, i, key, &entries);
                if mm.set_element(&path, Value::Array(converted)).is_err() {
                    log::error!("Unable to update {path} in graph {graph_id}");
                }
            }
        }
    }
    *model = mm.to_value();
}

fn convert_entries(
    graph_id: &str,
    node_index: usize,
    property: &str,
    entries: &[Value],
) -> Vec<Value> {
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
        } else {
            log::error!(
                "Invalid data mapping in graph {graph_id} node[{node_index}].{property} - \
                 missing '{MAP_TO}' in '{line}'"
            );
            converted.push(Value::from(line));
        }
    }
    converted
}

/// Java `getNormalizedPath`: rejoin the folder on single slashes, keep the
/// scheme prefix, append `<graph-id>.json`.
fn normalized_path(folder: &str, graph_id: &str) -> String {
    let parts: Vec<&str> = folder.split('/').filter(|p| !p.is_empty()).collect();
    format!("{}/{graph_id}.json", parts.join("/"))
}
