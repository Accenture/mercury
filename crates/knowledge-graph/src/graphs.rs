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

//! The process-wide registry of compiled graph models — Rust port of
//! `com.accenture.minigraph.models.CompiledGraphs`, mirroring the role the
//! `Flows` registry plays in event-script: a model registered here has been
//! structurally validated (via `MiniGraph::import_graph`) and had its
//! data-mapping entries converted from the deprecated "simple type matching"
//! syntax to "simple plugin" syntax. The graph executor serves deployed graph
//! execution EXCLUSIVELY from this registry: a deployed graph model is
//! executable only when it is listed in the graph manifest
//! (`graph.model.automation`) AND passed the CompileGraph quality gate. A
//! graph id that is not here answers HTTP-404 as if the model does not exist
//! — the CompileFlows precedent, where an invalid flow never becomes
//! executable. There is no lazy loading of deployed models. (The playground's
//! dry-run workspace is a separate surface and is not affected.)
//!
//! Since 4.12.19 the manifest property may name several manifests (comma-separated,
//! the `yaml.flow.automation` convention), each carrying its own `location`. The
//! registry therefore keeps the deployed locations in manifest order and, per graph,
//! the location its compiled copy came from. When two manifests list the same graph
//! id the LATER manifest owns the id: its copy replaces the earlier one, and if that
//! copy is rejected the id is not executable — the operator's latest intent wins, the
//! way a later configuration source overrides an earlier one.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

use rmpv::Value;

const DEFAULT_DEPLOY_DIR: &str = "classpath:/graph";

fn registry() -> &'static RwLock<HashMap<String, Arc<Value>>> {
    static COMPILED_GRAPHS: OnceLock<RwLock<HashMap<String, Arc<Value>>>> = OnceLock::new();
    COMPILED_GRAPHS.get_or_init(|| RwLock::new(HashMap::new()))
}

/// graph id -> the deployed location its compiled copy came from
fn locations() -> &'static RwLock<HashMap<String, String>> {
    static GRAPH_LOCATIONS: OnceLock<RwLock<HashMap<String, String>>> = OnceLock::new();
    GRAPH_LOCATIONS.get_or_init(|| RwLock::new(HashMap::new()))
}

/// every manifest's deployed location, in manifest order
fn deployed_locations_slot() -> &'static RwLock<Vec<String>> {
    static DEPLOYED_LOCATIONS: OnceLock<RwLock<Vec<String>>> = OnceLock::new();
    DEPLOYED_LOCATIONS.get_or_init(|| RwLock::new(Vec::new()))
}

/// A compiled graph model, or `None` when the graph is not compiled (not listed or rejected).
pub fn get_graph(graph_id: &str) -> Option<Arc<Value>> {
    registry()
        .read()
        .expect("graph registry poisoned")
        .get(graph_id)
        .cloned()
}

/// Whether a graph is compiled and therefore executable.
pub fn graph_exists(graph_id: &str) -> bool {
    registry()
        .read()
        .expect("graph registry poisoned")
        .contains_key(graph_id)
}

/// Register a compiled graph model and the deployed location it was compiled from.
pub fn add_graph(graph_id: &str, model: Value, location: &str) {
    registry()
        .write()
        .expect("graph registry poisoned")
        .insert(graph_id.to_string(), Arc::new(model));
    locations()
        .write()
        .expect("graph locations poisoned")
        .insert(graph_id.to_string(), location.to_string());
}

/// Drop a compiled graph model — a later manifest takes ownership of the id.
pub fn remove_graph(graph_id: &str) -> Option<Arc<Value>> {
    locations()
        .write()
        .expect("graph locations poisoned")
        .remove(graph_id);
    registry()
        .write()
        .expect("graph registry poisoned")
        .remove(graph_id)
}

/// The deployed location a compiled graph came from (`None` when it is not compiled).
pub fn graph_location(graph_id: &str) -> Option<String> {
    locations()
        .read()
        .expect("graph locations poisoned")
        .get(graph_id)
        .cloned()
}

/// The ids of all compiled graphs.
pub fn get_all_graphs() -> Vec<String> {
    registry()
        .read()
        .expect("graph registry poisoned")
        .keys()
        .cloned()
        .collect()
}

/// Replace the deployed locations with a single one (the single-manifest form).
pub fn set_deployed_location(location: &str) {
    let mut slot = deployed_locations_slot()
        .write()
        .expect("deployed locations poisoned");
    slot.clear();
    slot.push(location.to_string());
}

/// Append a manifest's deployed location (manifest order; a repeated location is kept once).
pub fn add_deployed_location(location: &str) {
    let mut slot = deployed_locations_slot()
        .write()
        .expect("deployed locations poisoned");
    if !slot.iter().any(|l| l == location) {
        slot.push(location.to_string());
    }
}

/// The primary deployed location — the first manifest's folder (the bundled one in the
/// common case), or the default when no manifest is configured.
pub fn deployed_location() -> String {
    deployed_locations_slot()
        .read()
        .expect("deployed locations poisoned")
        .first()
        .cloned()
        .unwrap_or_else(|| DEFAULT_DEPLOY_DIR.to_string())
}

/// Every deployed location in manifest order (the default alone when none is configured).
pub fn deployed_locations() -> Vec<String> {
    let slot = deployed_locations_slot()
        .read()
        .expect("deployed locations poisoned");
    if slot.is_empty() {
        vec![DEFAULT_DEPLOY_DIR.to_string()]
    } else {
        slot.clone()
    }
}
