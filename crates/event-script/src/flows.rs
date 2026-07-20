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

//! The process-wide flow registries — Rust port of
//! `com.accenture.models.Flows`: compiled flow templates (E-1) and live flow
//! instances (E-4).

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

use crate::instance::FlowInstance;
use crate::model::Flow;

fn registry() -> &'static RwLock<HashMap<String, Arc<Flow>>> {
    static ALL_FLOWS: OnceLock<RwLock<HashMap<String, Arc<Flow>>>> = OnceLock::new();
    ALL_FLOWS.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn get_flow(id: &str) -> Option<Arc<Flow>> {
    registry().read().expect("flow registry").get(id).cloned()
}

pub fn flow_exists(id: &str) -> bool {
    registry().read().expect("flow registry").contains_key(id)
}

pub fn add_flow(flow: Flow) {
    registry()
        .write()
        .expect("flow registry")
        .insert(flow.id.clone(), Arc::new(flow));
}

pub fn get_all_flows() -> Vec<String> {
    let mut ids: Vec<String> = registry()
        .read()
        .expect("flow registry")
        .keys()
        .cloned()
        .collect();
    ids.sort();
    ids
}

fn instances() -> &'static RwLock<HashMap<String, Arc<FlowInstance>>> {
    static INSTANCES: OnceLock<RwLock<HashMap<String, Arc<FlowInstance>>>> = OnceLock::new();
    INSTANCES.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn get_flow_instance(id: &str) -> Option<Arc<FlowInstance>> {
    instances()
        .read()
        .expect("instance registry")
        .get(id)
        .cloned()
}

pub fn add_flow_instance(instance: Arc<FlowInstance>) {
    instances()
        .write()
        .expect("instance registry")
        .insert(instance.id.clone(), instance);
}

pub fn close_flow_instance(id: &str) {
    instances().write().expect("instance registry").remove(id);
}

/// Resolve the ROOT ancestor of a flow family (Java `resolveParent`
/// recursion): follow parent links until an instance with no parent.
pub fn resolve_root_ancestor(parent_id: &str) -> Option<Arc<FlowInstance>> {
    let mut current = get_flow_instance(parent_id)?;
    while let Some(next_parent) = current.parent_id.clone() {
        match get_flow_instance(&next_parent) {
            Some(parent) => current = parent,
            None => break,
        }
    }
    Some(current)
}
