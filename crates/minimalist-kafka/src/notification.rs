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

//! `simple.kafka.notification` — a minimalist composable function that
//! publishes a Post Office event to a Kafka topic (Java
//! `SimpleKafkaNotification`). It reads the `topic` header (required) and the
//! optional `partition` header for routing, forwards every other event header
//! as a Kafka header (bytes), and uses the event body as the message body —
//! bytes verbatim, or a Map/List automatically serialized to JSON bytes (the
//! outbound symmetry of the flow adapter's inbound `serializer: 'json'`).
//! Publishing awaits the broker acknowledgement, so an RPC caller
//! (`po.request`) fails fast on a publishing failure while an async caller
//! (`po.send`) simply does not observe it — and failures are always logged.
//!
//! **Trace propagation.** Rather than forwarding the caller's (now-stale)
//! `traceparent`, it stamps a fresh W3C `traceparent` built from this
//! function's *own* current span, so the consuming side adopts this span as
//! the parent of the next hop — keeping the trace continuous across the Kafka
//! boundary.
//!
//! The `subject` header (the Confluent Schema Registry path) is reserved by
//! the deferred schema phase (port spec Q2) and refused with a clear message.

use std::collections::HashMap;
use std::sync::OnceLock;

use async_trait::async_trait;
use platform_core::{preload, w3c_trace, AppConfigReader, AppError, ComposableFunction};
use platform_core::{EventEnvelope, Platform, PostOffice};

use crate::headers::{
    CORRELATION_ID, MY_CORRELATION_ID, MY_ROUTE, MY_TRACE_ID, MY_TRACE_PATH, PARTITION, SUBJECT,
    TOPIC, VERSION,
};
use crate::runtime;

/// The route this function registers under.
pub const ROUTE: &str = "simple.kafka.notification";

/// Outbound header names, resolved lazily from configuration on first use
/// (never at registration time — the preload-before-bootstrap lesson).
struct HeaderNames {
    /// Outbound business correlation-id header (`kafka.correlation.id.header`,
    /// default `cid`).
    correlation_id: String,
    /// Optional legacy trace-id header (`kafka.trace.id.header`), stamped
    /// alongside the W3C traceparent when configured.
    trace_id: Option<String>,
    /// The traceparent header name (`kafka.traceparent.header`, default
    /// `traceparent`); when customized, the trace context is stamped under
    /// BOTH names.
    traceparent: String,
}

fn header_names() -> &'static HeaderNames {
    static NAMES: OnceLock<HeaderNames> = OnceLock::new();
    NAMES.get_or_init(|| {
        let config = AppConfigReader::get_instance();
        HeaderNames {
            correlation_id: config.get_property_or("kafka.correlation.id.header", CORRELATION_ID),
            trace_id: config.get_property("kafka.trace.id.header"),
            traceparent: config.get_property_or("kafka.traceparent.header", w3c_trace::TRACEPARENT),
        }
    })
}

/// The Kafka notification function — see the module documentation. Several
/// workers because publishing mostly awaits the broker acknowledgement; each
/// call is independent (the shared producer is thread-safe).
#[preload(route = "simple.kafka.notification", instances = 5)]
#[derive(Default)]
pub struct SimpleKafkaNotification;

#[async_trait]
impl ComposableFunction for SimpleKafkaNotification {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let Some(topic) = headers.get(TOPIC) else {
            return Err(AppError::new(400, format!("Missing '{TOPIC}' header")));
        };
        if headers.get(SUBJECT).is_some_and(|s| !s.trim().is_empty()) {
            // the Confluent wire-format path is a deliberate deferral, not an
            // accident - refuse loudly instead of publishing a surprising shape
            return Err(AppError::new(
                501,
                format!(
                    "'{SUBJECT}' header set, but Schema Registry support is deferred in this port \
                     (minimalist-kafka port spec, ruling Q2)"
                ),
            ));
        }
        let partition = parse_partition(headers.get(PARTITION).map(String::as_str))?;
        let names = header_names();
        let mut kafka_headers: HashMap<String, Vec<u8>> = HashMap::new();
        for (key, value) in &headers {
            if propagatable(key, names) {
                kafka_headers.insert(key.clone(), value.clone().into_bytes());
            }
        }
        // the business correlation-id under the configured header; an
        // explicitly mapped value wins over the flow's correlation-id
        // (model.cid, carried as the my_correlation_id reserved header)
        if let Some(cid) = headers
            .get(&names.correlation_id)
            .or_else(|| headers.get(MY_CORRELATION_ID))
        {
            kafka_headers.insert(names.correlation_id.clone(), cid.clone().into_bytes());
        }
        // a FRESH traceparent from this hop's own span - the consuming side
        // adopts it as the parent of the next hop
        let po = PostOffice::new(&Platform::get_instance());
        if let (Some(trace_id), Some(span_id)) = (po.my_trace_id(), po.my_span_id()) {
            if let Some(traceparent) = w3c_trace::format(&trace_id, &span_id) {
                kafka_headers.insert(
                    w3c_trace::TRACEPARENT.to_string(),
                    traceparent.clone().into_bytes(),
                );
                if !names
                    .traceparent
                    .eq_ignore_ascii_case(w3c_trace::TRACEPARENT)
                {
                    // a customized name is stamped ALONGSIDE the standard one
                    kafka_headers.insert(names.traceparent.clone(), traceparent.into_bytes());
                }
            }
        }
        // the optional legacy trace-id header; an explicitly mapped value wins
        if let Some(trace_header) = &names.trace_id {
            if let Some(trace_id) = headers
                .get(trace_header)
                .cloned()
                .or_else(|| headers.get(MY_TRACE_ID).cloned())
            {
                kafka_headers.insert(trace_header.clone(), trace_id.into_bytes());
            }
        }
        let payload = to_bytes(&input)?;
        // checked last: every caller-input error above is the caller's own
        // mistake and must surface ahead of this environment condition
        let Some(publisher) = runtime::publisher() else {
            return Err(AppError::new(
                500,
                format!(
                    "Kafka producer is disabled ({}=false); cannot publish",
                    crate::client_config::PRODUCER_ENABLED
                ),
            ));
        };
        publisher
            .publish(topic, partition, kafka_headers, payload)
            .await?;
        Ok(EventEnvelope::new())
    }
}

/// The body contract (Java `SimpleKafkaNotification.toBytes`): bytes pass
/// through verbatim (the minimalist default), a Map or List is automatically
/// serialized to JSON bytes, `null` stays `null` (a Kafka tombstone), and
/// anything else is rejected loudly rather than published in a surprising
/// shape.
fn to_bytes(input: &EventEnvelope) -> Result<Option<Vec<u8>>, AppError> {
    match input.body() {
        rmpv::Value::Nil => Ok(None),
        rmpv::Value::Binary(bytes) => Ok(Some(bytes.clone())),
        body @ (rmpv::Value::Map(_) | rmpv::Value::Array(_)) => {
            let json: serde_json::Value = input.body_as().map_err(|e| {
                AppError::new(400, format!("Unable to serialize body - {}", e.message()))
            })?;
            let _ = body;
            serde_json::to_vec(&json)
                .map(Some)
                .map_err(|e| AppError::new(400, format!("Unable to serialize body - {e}")))
        }
        other => Err(AppError::new(
            400,
            format!("body must be bytes, Map or List, got {}", type_name(other)),
        )),
    }
}

fn type_name(value: &rmpv::Value) -> &'static str {
    match value {
        rmpv::Value::String(_) => "String",
        rmpv::Value::Integer(_) => "Integer",
        rmpv::Value::F32(_) | rmpv::Value::F64(_) => "Float",
        rmpv::Value::Boolean(_) => "Boolean",
        _ => "an unsupported type",
    }
}

/// Whether an event header is forwarded verbatim as a Kafka header — excludes
/// routing/encoding directives, the traceparent under the standard or
/// configured name (replaced with this hop's own span), the correlation-id
/// and configured trace-id headers (stamped explicitly from resolved values),
/// and the framework's read-only reserved headers.
fn propagatable(key: &str, names: &HeaderNames) -> bool {
    key != TOPIC
        && key != PARTITION
        && key != SUBJECT
        && key != VERSION
        && key != w3c_trace::TRACEPARENT
        && key != names.traceparent
        && key != names.correlation_id
        && names.trace_id.as_deref() != Some(key)
        && key != MY_ROUTE
        && key != MY_TRACE_ID
        && key != MY_TRACE_PATH
        && key != MY_CORRELATION_ID
}

fn parse_partition(value: Option<&str>) -> Result<Option<i32>, AppError> {
    match value {
        None => Ok(None),
        Some(text) => text
            .trim()
            .parse::<i32>()
            .map(Some)
            .map_err(|_| AppError::new(400, format!("Invalid '{PARTITION}' header: {text}"))),
    }
}
