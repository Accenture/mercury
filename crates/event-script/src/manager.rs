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

//! The flow-engine entry point — Rust port of
//! `com.accenture.automation.EventScriptManager` (`event.script.manager`,
//! an event interceptor): a launch event carries the `flow_id` header, the
//! input dataset as body, the reply-routing correlation id on the envelope,
//! and optionally the business `correlation_id` header. The manager creates
//! the [`FlowInstance`], schedules its TTL watcher, and fires the first task
//! through the task executor.

use std::collections::HashMap;
use std::sync::Arc;

use platform_core::{AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::executor;
use crate::flows;
use crate::instance::FlowInstance;

pub const SERVICE_NAME: &str = "event.script.manager";
/// Launch-event header carrying the business correlation id (distinct from
/// the reply-routing internal correlation id on the envelope).
pub const BUSINESS_CORRELATION_ID: &str = "correlation_id";

/// Handle one launch event (the interceptor body). Errors are routed back to
/// the caller when a reply address exists (Java parity).
pub async fn handle(
    platform: &Platform,
    headers: HashMap<String, String>,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    if let Err(e) = process_request(platform, &headers, &event).await {
        log::error!("Unable to process request - {}", e.message());
        if let (Some(reply_to), Some(cid)) = (event.reply_to(), event.correlation_id()) {
            let po = PostOffice::new(platform);
            let error = EventEnvelope::new()
                .set_to(reply_to)
                .set_correlation_id(cid)
                .set_status(e.status())
                .set_raw_body(Value::from(e.message()));
            let _ = po.send(error).await;
        }
    }
    EventEnvelope::new().set_body("ok")
}

async fn process_request(
    platform: &Platform,
    headers: &HashMap<String, String>,
    event: &EventEnvelope,
) -> Result<(), AppError> {
    let flow_id = headers
        .get("flow_id")
        .filter(|id| !id.is_empty())
        .ok_or_else(|| AppError::new(400, "Missing flow_id"))?;
    let template = flows::get_flow(flow_id)
        .ok_or_else(|| AppError::new(400, format!("Flow {flow_id} not found")))?;
    // the input dataset is a map payload; a deep copy is inherent — rmpv
    // values are owned trees (design E5d)
    let payload = match event.body() {
        Value::Map(entries) => Value::Map(entries.clone()),
        _ => Value::Map(Vec::new()),
    };
    // a caller-provided ttl (ms) overrides the template's
    let ttl_ms = match crate::mlm::MultiLevelMap::from_value(payload.clone()).get_element("ttl") {
        Some(Value::Integer(n)) => n.as_u64().unwrap_or(template.ttl),
        _ => template.ttl,
    };
    let internal_correlation_id = event
        .correlation_id()
        .ok_or_else(|| AppError::new(400, format!("Missing correlation ID for {flow_id}")))?;
    // business correlation id: the header, falling back to the internal id
    let business_cid = headers
        .get(BUSINESS_CORRELATION_ID)
        .cloned()
        .unwrap_or_else(|| internal_correlation_id.to_string());
    let trace = match (event.trace_id(), event.trace_path()) {
        (Some(id), Some(path)) => Some((id.to_string(), path.to_string())),
        _ => None,
    };
    // a sub-flow inherits the ROOT ancestor's shared state tree
    let parent = headers.get("parent").and_then(|parent_id| {
        flows::resolve_root_ancestor(parent_id).map(|ancestor| {
            log::info!(
                "{}:{{new}} extends {}:{}",
                flow_id,
                ancestor.template.id,
                ancestor.id
            );
            (ancestor.id.clone(), ancestor.shared.clone())
        })
    });
    let instance = Arc::new(FlowInstance::new(
        flow_id,
        internal_correlation_id,
        &business_cid,
        event.reply_to().map(str::to_string),
        template.clone(),
        ttl_ms,
        trace,
        parent,
    ));
    // the triggering function's span becomes the flow's parent span (OTel lineage)
    if let Some(span_id) = event.span_id() {
        instance.set_parent_span_id(span_id);
    }
    // seed the input dataset into the state machine
    instance
        .dataset
        .lock()
        .expect("dataset")
        .set_element("input", payload)
        .map_err(|e| AppError::new(500, e))?;
    // TTL watcher: a scheduled timeout event to the task executor (E-3
    // send_later); body = end-of-flow listener list placeholder (Java parity)
    let po = PostOffice::new(platform);
    let timeout_event = EventEnvelope::new()
        .set_to(executor::SERVICE_NAME)
        .set_correlation_id(&instance.id)
        .set_header("timeout", "true")
        .set_raw_body(Value::Array(Vec::new()));
    let timer_id = po.send_later(timeout_event, std::time::Duration::from_millis(ttl_ms));
    instance.set_timeout_timer(timer_id);
    flows::add_flow_instance(instance.clone());
    // fire the first task; the flow instance id is the correlation id during
    // flow execution
    let first_task = EventEnvelope::new()
        .set_from(SERVICE_NAME)
        .set_to(executor::SERVICE_NAME)
        .set_correlation_id(&instance.id)
        .set_header("first_task", &template.first_task);
    po.send(first_task).await
}
