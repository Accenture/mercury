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
//! syntax to "simple plugin" syntax. The graph executor consults this
//! registry first and falls back to lazy per-request loading for any graph
//! id not declared in the manifest.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

use rmpv::Value;

fn registry() -> &'static RwLock<HashMap<String, Arc<Value>>> {
    static COMPILED_GRAPHS: OnceLock<RwLock<HashMap<String, Arc<Value>>>> = OnceLock::new();
    COMPILED_GRAPHS.get_or_init(|| RwLock::new(HashMap::new()))
}

/// The compiled graph model, or `None` if not compiled at startup.
pub fn get_graph(graph_id: &str) -> Option<Arc<Value>> {
    registry()
        .read()
        .expect("graph registry poisoned")
        .get(graph_id)
        .cloned()
}

/// True if the graph model was compiled at startup.
pub fn graph_exists(graph_id: &str) -> bool {
    registry()
        .read()
        .expect("graph registry poisoned")
        .contains_key(graph_id)
}

/// Register a validated and converted graph model (system use).
pub fn add_graph(graph_id: &str, model: Value) {
    registry()
        .write()
        .expect("graph registry poisoned")
        .insert(graph_id.to_string(), Arc::new(model));
}

/// All compiled graph ids.
pub fn get_all_graphs() -> Vec<String> {
    registry()
        .read()
        .expect("graph registry poisoned")
        .keys()
        .cloned()
        .collect()
}
