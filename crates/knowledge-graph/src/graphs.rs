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
//!
//! The registry is one immutable snapshot behind a lock (RFC-0017 in the Java repository's
//! register; Java `CompiledGraphs` since the same work): a read clones the current `Arc` and
//! looks the id up; a write builds the next snapshot and publishes it once, so a graph set
//! deployed, replaced or removed at run time appears or disappears as a unit and a run never
//! sees half of it. A refresh hook, installed by the hot-deployment loader when it is on, is
//! consulted on every lookup with the id and the location of the compiled copy (`None` for an
//! unknown id); the hook may reconcile the registry before the lookup is answered.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

use rmpv::Value;

const DEFAULT_DEPLOY_DIR: &str = "classpath:/graph";

/// The graph set a deployed graph came from (ADR-0027 in the Java repository's ledger; Java
/// `CompiledGraphs.DeployedSet`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployedSet {
    /// The set name, which is the package's file name without `.pack`.
    pub name: String,
    /// The set's `version` manifest field, empty when the set has none.
    pub version: String,
}

/// A hook consulted on every lookup of a graph by id, before the lookup is answered, with the
/// id and the deployed location of the compiled copy (`None` when the id is unknown).
pub type Refresh = fn(&str, Option<&str>);

#[derive(Clone)]
struct Entry {
    model: Arc<Value>,
    location: String,
    set: Option<DeployedSet>,
}

#[derive(Clone, Default)]
struct Snapshot {
    graphs: HashMap<String, Entry>,
    locations: Vec<String>,
}

fn current() -> &'static RwLock<Arc<Snapshot>> {
    static CURRENT: OnceLock<RwLock<Arc<Snapshot>>> = OnceLock::new();
    CURRENT.get_or_init(|| RwLock::new(Arc::new(Snapshot::default())))
}

fn refresh_slot() -> &'static RwLock<Option<Refresh>> {
    static REFRESH: OnceLock<RwLock<Option<Refresh>>> = OnceLock::new();
    REFRESH.get_or_init(|| RwLock::new(None))
}

fn snapshot() -> Arc<Snapshot> {
    current().read().expect("graph registry poisoned").clone()
}

/// Build the next snapshot from the current one and publish it once; writers serialize on the
/// lock, readers never wait for it.
fn update(change: impl FnOnce(&mut Snapshot)) {
    let mut slot = current().write().expect("graph registry poisoned");
    let mut next = (**slot).clone();
    change(&mut next);
    *slot = Arc::new(next);
}

fn lookup(graph_id: &str) -> Option<Entry> {
    let entry = snapshot().graphs.get(graph_id).cloned();
    let refresh = *refresh_slot().read().expect("graph refresh poisoned");
    if let Some(refresh) = refresh {
        refresh(graph_id, entry.as_ref().map(|e| e.location.as_str()));
        return snapshot().graphs.get(graph_id).cloned();
    }
    entry
}

/// A compiled graph model, or `None` when the graph is not compiled (not listed or rejected).
pub fn get_graph(graph_id: &str) -> Option<Arc<Value>> {
    lookup(graph_id).map(|entry| entry.model)
}

/// Whether a graph is compiled and therefore executable.
pub fn graph_exists(graph_id: &str) -> bool {
    lookup(graph_id).is_some()
}

/// Register a compiled graph model that a manifest lists.
pub fn add_graph(graph_id: &str, model: Value, location: &str) {
    let entry = Entry {
        model: Arc::new(model),
        location: location.to_string(),
        set: None,
    };
    update(|next| {
        next.graphs.insert(graph_id.to_string(), entry);
    });
}

/// Register a compiled graph model that came from a graph set.
pub fn add_set_graph(graph_id: &str, model: Value, location: &str, set: DeployedSet) {
    let entry = Entry {
        model: Arc::new(model),
        location: location.to_string(),
        set: Some(set),
    };
    update(|next| {
        next.graphs.insert(graph_id.to_string(), entry);
    });
}

/// Replace graphs in one swap (RFC-0017): the ids to remove leave and the models given register,
/// and a reader sees either the registry before or the registry after — never a mixture. A
/// hot-deployed set is deployed, replaced (its previous ids removed, its new graphs added) or
/// undeployed (no models) this way.
pub fn replace_graphs(
    remove_ids: &[String],
    models: Vec<(String, Value)>,
    location: &str,
    set: Option<DeployedSet>,
) {
    update(|next| {
        for id in remove_ids {
            next.graphs.remove(id);
        }
        for (id, model) in models {
            next.graphs.insert(
                id,
                Entry {
                    model: Arc::new(model),
                    location: location.to_string(),
                    set: set.clone(),
                },
            );
        }
    });
}

/// The graph set a compiled graph came from, or `None` when a manifest lists the graph or the
/// graph is not compiled.
pub fn graph_set(graph_id: &str) -> Option<DeployedSet> {
    snapshot().graphs.get(graph_id).and_then(|e| e.set.clone())
}

/// The graphs a set registered from a location holds in the registry, in no particular order.
pub fn graphs_of_set(set_name: &str, location: &str) -> Vec<String> {
    snapshot()
        .graphs
        .iter()
        .filter(|(_, entry)| {
            entry.location == location && entry.set.as_ref().is_some_and(|set| set.name == set_name)
        })
        .map(|(id, _)| id.clone())
        .collect()
}

/// Drop a compiled graph model — a later manifest takes ownership of the id.
pub fn remove_graph(graph_id: &str) {
    if snapshot().graphs.contains_key(graph_id) {
        update(|next| {
            next.graphs.remove(graph_id);
        });
    }
}

/// The deployed location a compiled graph came from, or `None` when the graph is not compiled.
pub fn graph_location(graph_id: &str) -> Option<String> {
    snapshot().graphs.get(graph_id).map(|e| e.location.clone())
}

/// The ids of every compiled graph.
pub fn get_all_graphs() -> Vec<String> {
    snapshot().graphs.keys().cloned().collect()
}

/// Append a manifest's deployed location (manifest order; a repeated location is kept once).
pub fn add_deployed_location(location: &str) {
    if snapshot().locations.iter().any(|l| l == location) {
        return;
    }
    update(|next| {
        if !next.locations.iter().any(|l| l == location) {
            next.locations.push(location.to_string());
        }
    });
}

/// The primary deployed location — the first manifest's folder (the bundled one in the common
/// case), or the default when no manifest is configured.
pub fn deployed_location() -> String {
    snapshot()
        .locations
        .first()
        .cloned()
        .unwrap_or_else(|| DEFAULT_DEPLOY_DIR.to_string())
}

/// Every deployed location in manifest order (the default alone when no manifest is configured).
pub fn deployed_locations() -> Vec<String> {
    let locations = &snapshot().locations;
    if locations.is_empty() {
        vec![DEFAULT_DEPLOY_DIR.to_string()]
    } else {
        locations.clone()
    }
}

/// Install the refresh hook, or remove it with `None`. Reserved for the hot-deployment loader.
pub fn set_refresh(refresh: Option<Refresh>) {
    *refresh_slot().write().expect("graph refresh poisoned") = refresh;
}
