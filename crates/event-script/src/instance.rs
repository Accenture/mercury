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

//! The per-transaction flow instance — Rust port of
//! `com.accenture.models.FlowInstance`: the state machine (`dataset` holding
//! `input` + `model`), task metrics, response bookkeeping, and the TTL
//! watcher (a scheduled event to the task executor; increment E-3's
//! `send_later`).
//!
//! Concurrency model: the task executor runs with **one instance** (Java
//! parity), so callbacks for a flow are naturally serialized; the dataset
//! mutex provides interior mutability, matching Java's per-instance
//! `modelSafety` lock. The `model.parent` / `model.root` shared-state aliases
//! arrive with sub-flows (increment E-7).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use rmpv::Value;

use crate::mlm::MultiLevelMap;
use crate::model::Flow;

/// Wall-clock + monotonic execution record for one task (Java `TaskMetrics`).
pub struct TaskMetrics {
    /// Task name (Java `service`).
    pub route: String,
    start: Instant,
    elapsed_ms: Mutex<Option<f32>>,
}

impl TaskMetrics {
    pub fn new(route: &str) -> Self {
        TaskMetrics {
            route: route.to_string(),
            start: Instant::now(),
            elapsed_ms: Mutex::new(None),
        }
    }

    pub fn complete(&self) {
        let mut elapsed = self.elapsed_ms.lock().expect("metrics");
        if elapsed.is_none() {
            *elapsed = Some(self.start.elapsed().as_secs_f32() * 1000.0);
        }
    }

    pub fn elapsed(&self) -> f32 {
        self.elapsed_ms
            .lock()
            .expect("metrics")
            .unwrap_or_else(|| self.start.elapsed().as_secs_f32() * 1000.0)
    }
}

/// One live flow transaction (Java `FlowInstance`).
pub struct FlowInstance {
    pub id: String,
    /// Reply-routing correlation id (returned to the calling party).
    pub internal_correlation_id: String,
    /// Business correlation id (exposed as `model.cid`).
    pub business_correlation_id: String,
    pub reply_to: Option<String>,
    pub template: Arc<Flow>,
    ttl_ms: u64,
    start: Instant,
    start_epoch_ms: u64,
    /// The state machine: `{input: …, model: {instance, cid, ttl, flow[, trace]}}`.
    /// The consolidated view for data mapping is built in this same tree, so
    /// `model.*` writes persist exactly like Java's shared-reference map.
    pub dataset: Mutex<MultiLevelMap>,
    /// uuid → metrics for pending/completed tasks.
    pub metrics: Mutex<std::collections::HashMap<String, Arc<TaskMetrics>>>,
    /// execution order (for the end-of-flow summary).
    pub tasks: Mutex<Vec<Arc<TaskMetrics>>>,
    responded: AtomicBool,
    running: AtomicBool,
    top_level_exception: AtomicBool,
    trace_id: Option<String>,
    trace_path: Option<String>,
    parent_span_id: Mutex<Option<String>>,
    /// End-of-flow advice routes + the error reference (Java `references`).
    end_flow_listeners: Mutex<Vec<String>>,
    error_reference: Mutex<Option<Value>>,
    /// The TTL watcher's timer id (cancelled on close).
    timeout_timer: Mutex<Option<String>>,
}

impl FlowInstance {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        flow_id: &str,
        internal_correlation_id: &str,
        business_correlation_id: &str,
        reply_to: Option<String>,
        template: Arc<Flow>,
        ttl_ms: u64,
        trace: Option<(String, String)>,
    ) -> Self {
        let id = uuid::Uuid::new_v4().simple().to_string();
        let mut dataset = MultiLevelMap::new();
        // seed the read-only model metadata (guarded by the compiler's
        // reserved-key check and the executor's dynamic re-check)
        dataset
            .set_element("model.instance", Value::from(id.as_str()))
            .expect("seed");
        dataset
            .set_element("model.cid", Value::from(business_correlation_id))
            .expect("seed");
        dataset
            .set_element("model.ttl", Value::from(ttl_ms))
            .expect("seed");
        dataset
            .set_element("model.flow", Value::from(flow_id))
            .expect("seed");
        let (trace_id, trace_path) = match trace {
            Some((id_value, path)) => {
                dataset
                    .set_element("model.trace", Value::from(id_value.as_str()))
                    .expect("seed");
                (Some(id_value), Some(path))
            }
            None => (None, None),
        };
        FlowInstance {
            id,
            internal_correlation_id: internal_correlation_id.to_string(),
            business_correlation_id: business_correlation_id.to_string(),
            reply_to,
            template,
            ttl_ms,
            start: Instant::now(),
            start_epoch_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0),
            dataset: Mutex::new(dataset),
            metrics: Mutex::new(std::collections::HashMap::new()),
            tasks: Mutex::new(Vec::new()),
            responded: AtomicBool::new(false),
            running: AtomicBool::new(true),
            top_level_exception: AtomicBool::new(false),
            trace_id,
            trace_path,
            parent_span_id: Mutex::new(None),
            end_flow_listeners: Mutex::new(Vec::new()),
            error_reference: Mutex::new(None),
            timeout_timer: Mutex::new(None),
        }
    }

    pub fn ttl_ms(&self) -> u64 {
        self.ttl_ms
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }

    pub fn start_epoch_ms(&self) -> u64 {
        self.start_epoch_ms
    }

    pub fn trace_id(&self) -> Option<&str> {
        self.trace_id.as_deref()
    }

    pub fn trace_path(&self) -> Option<&str> {
        self.trace_path.as_deref()
    }

    pub fn parent_span_id(&self) -> Option<String> {
        self.parent_span_id.lock().expect("span").clone()
    }

    pub fn set_parent_span_id(&self, span_id: &str) {
        *self.parent_span_id.lock().expect("span") = Some(span_id.to_string());
    }

    pub fn is_not_responded(&self) -> bool {
        !self.responded.load(Ordering::SeqCst)
    }

    /// Atomically claim the right to respond (true = this caller responds).
    pub fn claim_response(&self) -> bool {
        !self.responded.swap(true, Ordering::SeqCst)
    }

    pub fn top_level_exception_happened(&self) -> bool {
        self.top_level_exception.load(Ordering::SeqCst)
    }

    pub fn set_exception_at_top_level(&self, state: bool) {
        self.top_level_exception.store(state, Ordering::SeqCst);
    }

    /// Register composable functions to be notified when the flow ends
    /// (Java `setEndFlowListeners`, unique routes).
    pub fn set_end_flow_listeners(&self, routes: &[&str]) {
        let mut listeners = self.end_flow_listeners.lock().expect("listeners");
        listeners.clear();
        for route in routes {
            if !listeners.iter().any(|r| r == route) {
                listeners.push(route.to_string());
            }
        }
    }

    pub fn end_flow_listeners(&self) -> Vec<String> {
        self.end_flow_listeners.lock().expect("listeners").clone()
    }

    pub fn set_error_reference(&self, error: Value) {
        *self.error_reference.lock().expect("error ref") = Some(error);
    }

    pub fn error_reference(&self) -> Option<Value> {
        self.error_reference.lock().expect("error ref").clone()
    }

    pub fn set_timeout_timer(&self, timer_id: String) {
        *self.timeout_timer.lock().expect("timer") = Some(timer_id);
    }

    pub fn take_timeout_timer(&self) -> Option<String> {
        self.timeout_timer.lock().expect("timer").take()
    }

    /// Mark the instance closed (idempotent); returns true on the first call.
    pub fn close(&self) -> bool {
        let was_running = self.running.swap(false, Ordering::SeqCst);
        if was_running {
            self.responded.store(true, Ordering::SeqCst);
        }
        was_running
    }
}
