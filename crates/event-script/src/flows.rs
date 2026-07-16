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

//! The process-wide flow-template registry — Rust port of the template half of
//! `com.accenture.models.Flows`. The live flow-*instance* registry arrives
//! with the core runtime (design increment E-4).

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

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
