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
//! The four checks read the model alone, so they live in
//! [`crate::model_gate`], which the graph packager shares: a graph set is
//! refused at pack time for the reasons this gate would reject it at startup.
//!
//! CompileGraph is the deployment gate: set `graph.model.automation` to a
//! YAML manifest — or, since 4.12.19, a comma-separated list of manifests, each
//! with its own `location`, the later manifest winning a duplicate graph id —
//! listing the graph ids to compile at startup (mirroring
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

use platform_core::{AppConfigReader, ConfigReader, ConfigValue};
use rmpv::Value;

use crate::graphs;
use crate::model_gate;

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
    let manifests = config.get_property_or("graph.model.automation", "");
    if manifests.trim().is_empty() {
        log::warn!(
            "No graph manifest configured (graph.model.automation) - \
             no deployed graph models will be executable"
        );
        return graphs::get_all_graphs();
    }
    // Since 4.12.19 the property may name several manifests, comma-separated (the
    // yaml.flow.automation convention): the one bundled in the artifact and, for rapid
    // prototyping, an external one - each carries its own 'location'. Manifests compile
    // in the order listed, and a manifest that cannot be loaded is skipped with a warning
    // so the others still compile (Java: util.split(manifests, ", ")).
    for manifest in manifests.split([',', ' ']).filter(|s| !s.is_empty()) {
        compile_manifest(manifest);
    }
    let all = graphs::get_all_graphs();
    log::info!("Graph models compiled: {}", all.len());
    all
}

fn compile_manifest(manifest: &str) {
    match ConfigReader::load(manifest) {
        Ok(reader) => {
            log::info!("Loading graph manifest {manifest}");
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
            graphs::add_deployed_location(&deploy_location);
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
}

fn compile_one_graph(deploy_location: &str, graph_id: &str) {
    // later manifest wins: when a later manifest lists a graph id again, that manifest owns
    // the id - its copy replaces the earlier one, and if the new copy is rejected the id is
    // not executable (404) rather than silently served from the copy the operator meant to
    // replace (a curl test would otherwise pass against the old behavior)
    if let Some(previous) = graphs::graph_location(graph_id).filter(|p| p != deploy_location) {
        log::warn!("Graph {graph_id} from {deploy_location} replaces the copy from {previous}");
        graphs::remove_graph(graph_id);
    }
    match load_and_validate(deploy_location, graph_id) {
        Ok(model) => {
            graphs::add_graph(graph_id, model, deploy_location);
            log::info!("Compiled graph {graph_id}");
        }
        // a rejected graph is simply not registered: deployed execution is served
        // exclusively from the compiled registry, so requests to it answer 404
        Err(e) => log::error!("Rejected graph {graph_id} - {e}"),
    }
}

/// Load a graph JSON as an rmpv value with `${...}` references resolved and
/// normalized the way the Java engine reads it — the raw form the startup
/// compiler shares with the playground's temp workspace import (Java uses
/// `ConfigReader` in both places, whose normalization drops nulls, empty maps
/// and empty lists; this reader keeps them, so [`model_gate::normalize_graph`]
/// is applied here).
pub(crate) fn load_raw_graph(deploy_location: &str, graph_id: &str) -> Result<Value, String> {
    // pass the loader error through untouched (Java parity): a missing model
    // file logs the FULL normalized path — "Rejected graph g1 -
    // classpath:/graph/g1.json not found" — so the operator sees which
    // location was searched
    let reader = ConfigReader::load(&normalized_path(deploy_location, graph_id))
        .map_err(|e| e.to_string())?;
    let json = ConfigValue::Map(reader.get_map().clone().into_map()).to_json();
    // a graph holds no null property ("key": null) and no empty map or list, while "key": "" is a value
    Ok(model_gate::normalize_graph(
        &event_script::conversions::from_json(&json),
    ))
}

fn load_and_validate(deploy_location: &str, graph_id: &str) -> Result<Value, String> {
    // the ConfigReader load resolves ${...} references against the app
    // config, exactly like the Java loader
    let mut model = load_raw_graph(deploy_location, graph_id)?;
    // the gate's static checks, shared with the graph packager - converts
    // deprecated mapping syntax in place
    model_gate::validate(graph_id, &mut model)?;
    Ok(model)
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
    use event_script::mlm::MultiLevelMap;

    /// A deployed graph holding `"key": null` compiles, as on the Java engine,
    /// whose configuration reader drops a null-valued key when it normalizes the
    /// graph; `"key": ""` stays a value.
    #[test]
    fn a_deployed_null_property_is_filtered_out() {
        let dir =
            std::env::temp_dir().join(format!("compiler-null-property-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a folder");
        std::fs::write(
            dir.join("nulls.json"),
            r#"{"nodes": [
              {"alias": "root", "types": ["Root"],
               "properties": {"purpose": "null properties", "name": "nulls", "note": null, "empty": ""}},
              {"alias": "end", "types": ["End"], "properties": {}}],
             "connections": [{"source": "root", "target": "end",
                              "relations": [{"type": "done", "properties": {"x": null}}]}]}"#,
        )
        .expect("a graph file");
        let model = load_and_validate(&format!("file:{}", dir.display()), "nulls");
        let _ = std::fs::remove_dir_all(&dir);
        let mm =
            MultiLevelMap::from_value(model.expect("a null property is filtered out, not refused"));
        assert_eq!(None, mm.get_element("nodes[0].properties.note"));
        assert_eq!(
            Some(Value::from("")),
            mm.get_element("nodes[0].properties.empty")
        );
        assert_eq!(
            None,
            mm.get_element("connections[0].relations[0].properties.x")
        );
    }

    /// The graph read reproduces the Java configuration reader's normalization -
    /// nulls, empty maps and empty lists carry no value; inside a list such an
    /// element keeps its place as null unless nothing with a value follows - with
    /// the vector file the Java engine's `GraphSetTest` reads too (byte-identical).
    #[test]
    fn the_graph_read_follows_the_shared_normalization_vectors() {
        let vectors: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/resources/graph-read-normalization-vectors.json"
            ))
            .expect("the vector file"),
        )
        .expect("JSON");
        let dir = std::env::temp_dir().join(format!("graph-read-vectors-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a folder");
        let location = format!("file:{}", dir.display());
        let cases = vectors["cases"].as_array().expect("cases");
        assert!(!cases.is_empty());
        for case in cases {
            let name = case["name"].as_str().expect("a name");
            std::fs::write(dir.join(format!("{name}.json")), case["input"].to_string())
                .expect("written");
            let read = load_raw_graph(&location, name).expect("read");
            let expected = event_script::conversions::from_json(&case["expected"]);
            assert_eq!(
                platform_core::canonical_packager::encode(&expected).expect("encodes"),
                platform_core::canonical_packager::encode(&read).expect("encodes"),
                "{name}: {}",
                event_script::conversions::to_json(&read).expect("JSON")
            );
            if case["gate"] == "accepted" {
                let mut model = read;
                assert!(model_gate::validate(name, &mut model).is_ok(), "{name}");
            }
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
