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

//! Rust port of the Java `Telemetry` service
//! (`org.platformlambda.core.services.Telemetry`) — the built-in
//! **`distributed.tracing`** function, registered by the lifecycle's
//! essential-services phase (the Java `EssentialServiceLoader`).
//!
//! When a traced function finishes, its route worker sends the
//! performance-metrics dataset here: `{ trace: { id, span_id, parent_span_id,
//! service, path, from, origin, start, exec_time, success, status,
//! exception? }, annotations: {...} }`. By default the dataset is **logged**
//! (the real-time telemetry stream); two reserved extension routes forward it
//! elsewhere:
//!
//! - **`distributed.trace.forwarder`** — register a function here (e.g. an
//!   OpenTelemetry OTLP exporter) and every dataset is forwarded to it;
//! - **`transaction.journal.recorder`** — receives request/response journals
//!   when journaling is enabled (payloads may contain PII/PHI/PCI — handle per
//!   your organization's security policy).
//!
//! Reserved for system use — do not call this route from application code.

use std::collections::HashMap;

use async_trait::async_trait;

use crate::envelope::EventEnvelope;
use crate::function::{AppError, ComposableFunction};
use crate::platform::Platform;

pub const DISTRIBUTED_TRACING: &str = "distributed.tracing";
pub const DISTRIBUTED_TRACE_FORWARDER: &str = "distributed.trace.forwarder";
pub const TRANSACTION_JOURNAL_RECORDER: &str = "transaction.journal.recorder";

/// Routes that must never appear as a traced service (they are the telemetry
/// plumbing itself — Java `ZERO_TRACING_FILTER`).
pub const ZERO_TRACING_FILTER: [&str; 3] = [
    DISTRIBUTED_TRACING,
    DISTRIBUTED_TRACE_FORWARDER,
    TRANSACTION_JOURNAL_RECORDER,
];

/// The built-in telemetry sink. Holds a handle to its own platform so it can
/// forward datasets to the optional extension routes.
pub struct Telemetry {
    platform: Platform,
}

impl Telemetry {
    pub fn new(platform: &Platform) -> Self {
        Telemetry {
            platform: platform.clone(),
        }
    }
}

#[async_trait]
impl ComposableFunction for Telemetry {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let Ok(payload) = input.body_as::<serde_json::Value>() else {
            return Ok(EventEnvelope::new());
        };
        let Some(payload) = payload.as_object() else {
            return Ok(EventEnvelope::new());
        };
        let mut metrics = match payload.get("trace").and_then(|t| t.as_object()) {
            Some(m) if !m.is_empty() => m.clone(),
            _ => return Ok(EventEnvelope::new()),
        };
        // filter the telemetry plumbing itself; trim any "@origin" suffix
        let Some(service) = permitted_route(metrics.get("service")) else {
            return Ok(EventEnvelope::new());
        };
        metrics.insert("service".to_string(), serde_json::Value::String(service));
        if let Some(from) = metrics.get("from").and_then(|f| f.as_str()) {
            let trimmed = trim_origin(from).to_string();
            metrics.insert("from".to_string(), serde_json::Value::String(trimmed));
        }
        let annotations = payload
            .get("annotations")
            .and_then(|a| a.as_object())
            .cloned()
            .unwrap_or_default();
        let mut dataset = serde_json::Map::new();
        dataset.insert("trace".to_string(), serde_json::Value::Object(metrics));
        if !annotations.is_empty() {
            dataset.insert(
                "annotations".to_string(),
                serde_json::Value::Object(annotations),
            );
        }
        let dataset = serde_json::Value::Object(dataset);
        // the default sink: the real-time telemetry log stream
        log::info!("{dataset}");
        // optional forwarders (must be registered in the same application)
        if self.platform.has_route(DISTRIBUTED_TRACE_FORWARDER) {
            let event = EventEnvelope::new()
                .set_to(DISTRIBUTED_TRACE_FORWARDER)
                .set_body(&dataset)?;
            let _ = self
                .platform
                .deliver(DISTRIBUTED_TRACE_FORWARDER, event)
                .await;
        }
        if payload.contains_key("journal") && self.platform.has_route(TRANSACTION_JOURNAL_RECORDER)
        {
            let mut forward = dataset;
            if let (Some(map), Some(journal)) = (forward.as_object_mut(), payload.get("journal")) {
                map.insert("journal".to_string(), journal.clone());
            }
            let event = EventEnvelope::new()
                .set_to(TRANSACTION_JOURNAL_RECORDER)
                .set_body(&forward)?;
            let _ = self
                .platform
                .deliver(TRANSACTION_JOURNAL_RECORDER, event)
                .await;
        }
        Ok(EventEnvelope::new())
    }
}

/// Extract the service route, dropping the telemetry plumbing routes and
/// trimming any `@origin` suffix (Java `getPermittedRoute`).
fn permitted_route(service: Option<&serde_json::Value>) -> Option<String> {
    let route = service?.as_str()?;
    let name = trim_origin(route);
    if ZERO_TRACING_FILTER.contains(&name) {
        None
    } else {
        Some(name.to_string())
    }
}

fn trim_origin(route: &str) -> &str {
    match route.find('@') {
        Some(at) => &route[..at],
        None => route,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plumbing_routes_are_filtered() {
        let v = |s: &str| serde_json::Value::String(s.to_string());
        assert_eq!(
            permitted_route(Some(&v("v1.hello"))),
            Some("v1.hello".into())
        );
        assert_eq!(
            permitted_route(Some(&v("v1.hello@abc123"))),
            Some("v1.hello".into())
        );
        assert_eq!(permitted_route(Some(&v("distributed.tracing"))), None);
        assert_eq!(
            permitted_route(Some(&v("distributed.trace.forwarder@x"))),
            None
        );
        assert_eq!(permitted_route(None), None);
    }
}
