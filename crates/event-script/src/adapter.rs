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

//! The HTTP flow adapter — Rust port of `com.accenture.adapters.HttpToFlow`
//! (`http.flow.adapter`, an event interceptor): REST automation routes a
//! `flow:`-bound endpoint here with the `x-flow-id` header injected
//! (increment E-3); the adapter reshapes the AsyncHttpRequest into the flow
//! input dataset and launches the flow, preserving the HTTP edge's reply
//! address so the flow's response completes the HTTP request.
//!
//! Divergence (doc'd): cookies, upload streams, file names and session info
//! are not part of the Rust HTTP edge yet (platform-core §7 deferrals), so
//! those dataset keys are absent — mappings referencing them resolve null,
//! which the mapping engine already tolerates.

use std::collections::HashMap;

use platform_core::{AppConfigReader, AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::mlm::MultiLevelMap;

pub const ROUTE: &str = "http.flow.adapter";

/// The interceptor body: convert the HTTP context to the flow input dataset
/// and launch the flow. Errors route back to the HTTP edge (Java parity:
/// `{type: error, status, message}` with the same status).
pub async fn handle(
    platform: &Platform,
    _headers: HashMap<String, String>,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    if let Err(e) = process_request(platform, &event).await {
        if let (Some(reply_to), Some(cid)) = (event.reply_to(), event.correlation_id()) {
            let mut error = MultiLevelMap::new();
            let _ = error.set_element("type", Value::from("error"));
            let _ = error.set_element("status", Value::from(e.status() as i64));
            let _ = error.set_element("message", Value::from(e.message()));
            let po = PostOffice::new(platform);
            let _ = po
                .send(
                    EventEnvelope::new()
                        .set_to(reply_to)
                        .set_correlation_id(cid)
                        .set_status(e.status())
                        .set_raw_body(error.to_value()),
                )
                .await;
        }
    }
    EventEnvelope::new().set_body("ok")
}

async fn process_request(platform: &Platform, event: &EventEnvelope) -> Result<(), AppError> {
    let request = MultiLevelMap::from_value(event.body().clone());
    let flow_id = request
        .get_element("headers.x-flow-id")
        .map(|v| crate::conversions::display(&v))
        .ok_or_else(|| AppError::new(400, "Missing x-flow-id in HTTP request headers"))?;
    // convert the HTTP context to the flow "input" dataset (Java key set)
    let mut dataset = MultiLevelMap::new();
    let seconds = request
        .get_element("timeout")
        .map(|v| crate::util::str2long(&crate::conversions::display(&v)))
        .filter(|n| *n > 0)
        .unwrap_or(30);
    set(&mut dataset, "ttl", Value::from(seconds * 1000))?;
    for (from, to) in [
        ("headers", "header"),
        ("body", "body"),
        ("parameters.path", "path_parameter"),
        ("method", "method"),
        ("url", "uri"),
        ("parameters.query", "query"),
        ("ip", "ip"),
    ] {
        if let Some(value) = request.get_element(from) {
            set(&mut dataset, to, value)?;
        }
    }
    // the business correlation id: the edge-ensured header (configurable),
    // falling back to a fresh uuid (Java parity)
    let cid_header = AppConfigReader::get_instance()
        .get_property_or("http.correlation.id.header", "X-Correlation-Id")
        .to_lowercase();
    let business_cid = request
        .get_element(&format!("headers.{cid_header}"))
        .map(|v| crate::conversions::display(&v))
        .unwrap_or_else(|| uuid::Uuid::new_v4().simple().to_string());
    let reply_to = event
        .reply_to()
        .ok_or_else(|| AppError::new(400, "Missing reply address"))?;
    let internal_cid = event
        .correlation_id()
        .ok_or_else(|| AppError::new(400, "Missing correlation ID"))?;
    // launch the flow, preserving the HTTP edge's reply routing
    let mut launch = EventEnvelope::new()
        .set_to(crate::manager::SERVICE_NAME)
        .set_reply_to(reply_to)
        .set_correlation_id(internal_cid)
        .set_header("flow_id", &flow_id)
        .set_header(crate::manager::BUSINESS_CORRELATION_ID, &business_cid)
        .set_raw_body(dataset.to_value());
    if let (Some(trace_id), Some(path)) = (event.trace_id(), event.trace_path()) {
        launch = launch.set_trace(trace_id, path);
    }
    if let Some(span) = event.span_id() {
        launch = launch.set_span_id(span);
    }
    let po = PostOffice::new(platform);
    po.send(launch).await
}

fn set(dataset: &mut MultiLevelMap, key: &str, value: Value) -> Result<(), AppError> {
    dataset
        .set_element(key, value)
        .map_err(|e| AppError::new(500, e))
}
