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

//! Per-execution graph state (Java `GraphInstance` + `Visits`) and the
//! process-wide instance registry (the `graphInstances` map on the Java
//! `GraphLambdaFunction` base class). One `GraphInstance` lives for one
//! traversal of one graph model, keyed by the flow instance id; the
//! housekeeper removes it when the wrapping flow ends.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock};

use event_script::mlm::MultiLevelMap;
use platform_core::graph::MiniGraph;

/// Loop-detection counters for one node (Java `Visits`).
#[derive(Clone, Copy, Default)]
pub struct Visits {
    pub last_visit: i64,
    pub hits: i64,
}

#[derive(Default)]
struct Metadata {
    correlation_id: Option<String>,
    flow_instance_id: Option<String>,
    reply_to: Option<String>,
}

/// One live graph traversal (Java `GraphInstance`).
///
/// The state machine is a single rmpv `MultiLevelMap` shared by the executor
/// and every skill; guards are scoped to synchronous sections and never held
/// across awaits (the layer-2 locking discipline).
pub struct GraphInstance {
    pub graph_id: String,
    pub graph: MiniGraph,
    pub state: Mutex<MultiLevelMap>,
    pub hits: Mutex<HashMap<String, Visits>>,
    /// Java semantics: a key present with `false` still counts as "seen" for
    /// join bookkeeping, so this is a map, not a set.
    pub node_seen: Mutex<HashMap<String, bool>>,
    pub skill_run: Mutex<HashMap<String, bool>>,
    pub complete: AtomicBool,
    metadata: Mutex<Metadata>,
    start_time_ms: std::sync::atomic::AtomicI64,
}

impl GraphInstance {
    pub fn new(graph_id: &str) -> Self {
        GraphInstance {
            graph_id: graph_id.to_string(),
            graph: MiniGraph::new(),
            state: Mutex::new(MultiLevelMap::new()),
            hits: Mutex::new(HashMap::new()),
            node_seen: Mutex::new(HashMap::new()),
            skill_run: Mutex::new(HashMap::new()),
            complete: AtomicBool::new(false),
            metadata: Mutex::new(Metadata::default()),
            start_time_ms: std::sync::atomic::AtomicI64::new(now_epoch_ms()),
        }
    }

    pub fn reset_start_time(&self) {
        self.start_time_ms.store(now_epoch_ms(), Ordering::SeqCst);
    }

    pub fn start_time_ms(&self) -> i64 {
        self.start_time_ms.load(Ordering::SeqCst)
    }

    pub fn is_complete(&self) -> bool {
        self.complete.load(Ordering::SeqCst)
    }

    pub fn set_complete(&self) {
        self.complete.store(true, Ordering::SeqCst);
    }

    pub fn get_correlation_id(&self) -> String {
        self.metadata
            .lock()
            .expect("graph metadata")
            .correlation_id
            .clone()
            .unwrap_or_else(|| "none".to_string())
    }

    pub fn set_correlation_id(&self, correlation_id: &str) {
        self.metadata.lock().expect("graph metadata").correlation_id =
            Some(correlation_id.to_string());
    }

    pub fn get_flow_instance_id(&self) -> String {
        self.metadata
            .lock()
            .expect("graph metadata")
            .flow_instance_id
            .clone()
            .unwrap_or_else(|| "none".to_string())
    }

    pub fn set_flow_instance_id(&self, instance_id: &str) {
        self.metadata
            .lock()
            .expect("graph metadata")
            .flow_instance_id = Some(instance_id.to_string());
    }

    pub fn get_reply_to(&self) -> String {
        self.metadata
            .lock()
            .expect("graph metadata")
            .reply_to
            .clone()
            .unwrap_or_else(|| "none".to_string())
    }

    pub fn set_reply_to(&self, reply_to: &str) {
        self.metadata.lock().expect("graph metadata").reply_to = Some(reply_to.to_string());
    }
}

fn now_epoch_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn registry() -> &'static RwLock<HashMap<String, Arc<GraphInstance>>> {
    static INSTANCES: OnceLock<RwLock<HashMap<String, Arc<GraphInstance>>>> = OnceLock::new();
    INSTANCES.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn get_instance(id: &str) -> Option<Arc<GraphInstance>> {
    registry()
        .read()
        .expect("graph instances poisoned")
        .get(id)
        .cloned()
}

pub fn add_instance(id: &str, instance: Arc<GraphInstance>) {
    registry()
        .write()
        .expect("graph instances poisoned")
        .insert(id.to_string(), instance);
}

pub fn remove_instance(id: &str) -> Option<Arc<GraphInstance>> {
    registry()
        .write()
        .expect("graph instances poisoned")
        .remove(id)
}
