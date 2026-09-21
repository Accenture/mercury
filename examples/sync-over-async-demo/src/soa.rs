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

//! The request/reply roles — **synchronous REST over an asynchronous Kafka
//! backend** (the Java demo's `facade` / `backend` profiles): a caller makes one
//! synchronous HTTP request, the request travels over Kafka to a separate
//! backend pod, the reply comes back over Kafka, and the Redis return route
//! delivers it to the exact facade pod that is waiting.
//!
//! Only the functions here are application-specific: the backend business logic
//! (`system.of.record` and its schema-typed variants) and the HTTP status policy
//! (`sync.error.handler`). Everything else is reused from the extension —
//! `sync.prepare`, `sync.await`, `soa.reply`, the Redis coordinator — and from
//! minimalist-kafka — `simple.kafka.notification`, the flow adapter, the Schema
//! Registry codec. None of these functions publishes: the flow YAML does.
//!
//! Each function derives its trace context from the read-only `my_*` headers the
//! engine injects, so every one is directly unit-testable (the Java
//! `SoaTaskTest`, one to one).

use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::trace::iso8601_utc_now;
use platform_core::{preload, AppError, ComposableFunction, EventEnvelope, TypedFunction};
use rmpv::Value;
use sync_over_async::{runtime, tasks, CID};

/// The engine injects the function's own trace id under this read-only header.
const MY_TRACE_ID: &str = "my_trace_id";
const SERVICE_UNAVAILABLE: i64 = 503;

// ---------------------------------------------------------------------------
// the backend: system.of.record (raw bytes), .json and .avro (decoded maps)
// ---------------------------------------------------------------------------

/// The backend business logic on the raw-bytes path — the task of the
/// `system-of-record` flow, reached (on the **backend** pod) when the Kafka flow
/// adapter routes a message from `soa.request`. It echoes the request with a
/// little processing metadata and returns the result; the flow then publishes
/// it to `soa.response` via `simple.kafka.notification`. The correlation-id is
/// carried through so the facade can match the reply to the awaiting request,
/// and the `traceId` is echoed to show the trace stayed continuous across the
/// Kafka hops.
#[preload(route = "system.of.record", instances = 50)]
#[derive(Default)]
pub struct SystemOfRecord;

#[async_trait]
impl ComposableFunction for SystemOfRecord {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request = match input.body() {
            Value::Binary(bytes) => serde_json::from_slice(bytes)
                .map_err(|e| AppError::new(400, format!("request is not a JSON document - {e}")))?,
            _ => input.body_as()?,
        };
        process(request, &headers)
    }
}

/// JSON Schema variant of [`SystemOfRecord`], the task of the
/// `system-of-record-json` flow (bound to `json-topic-1`). Because that binding
/// sets `schema.enabled`, the adapter decodes the Confluent-framed value and
/// hands this task the request as a **map** — the only difference from the raw
/// path. The flow then re-encodes the reply against the `sync-demo-json` subject
/// and publishes it to `json-topic-2`.
#[preload(route = "system.of.record.json", instances = 50, typed)]
#[derive(Default)]
pub struct SystemOfRecordJson;

#[async_trait]
impl TypedFunction<HashMap<String, serde_json::Value>, EventEnvelope> for SystemOfRecordJson {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: HashMap<String, serde_json::Value>,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        process(
            serde_json::Value::Object(input.into_iter().collect()),
            &headers,
        )
    }
}

/// Avro variant of [`SystemOfRecord`], the task of the `system-of-record-avro`
/// flow (bound to `avro-topic-1`). Unlike the JSON Schema path — whose
/// `additionalProperties: true` schema tolerates the open, nested response the
/// shared logic builds — an **Avro record is closed-shape**: every field of
/// `SyncDemoMessage` (`action`, `status`, `processedBy`, `traceId`) is declared,
/// and only those. So this task builds a **flat** reply matching the record
/// exactly; the flow then re-encodes it against the `sync-demo-avro` subject and
/// publishes it to `avro-topic-2`.
#[preload(route = "system.of.record.avro", instances = 50, typed)]
#[derive(Default)]
pub struct SystemOfRecordAvro;

#[async_trait]
impl TypedFunction<HashMap<String, serde_json::Value>, EventEnvelope> for SystemOfRecordAvro {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: HashMap<String, serde_json::Value>,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let cid = headers.get(CID).cloned();
        log::info!(
            "Processing Avro request (cid={}): {}",
            cid.as_deref().unwrap_or("null"),
            serde_json::Value::Object(input.clone().into_iter().collect())
        );
        // a flat reply matching the Avro SyncDemoMessage record exactly (no extra/nested fields)
        let response = serde_json::json!({
            "action": input.get("action").cloned().unwrap_or_else(|| serde_json::Value::String(String::new())),
            "status": "processed",
            "processedBy": "system-of-record",
            "traceId": headers.get(MY_TRACE_ID).cloned().unwrap_or_default(),
        });
        reply(response, cid)
    }
}

/// The shared backend logic: echo the request with processing metadata. Returns
/// the reply as JSON bytes carrying the correlation-id header — ready for
/// `simple.kafka.notification`, which publishes it raw or re-encodes it with a
/// schema id (the raw / JSON Schema paths share this contract).
fn process(
    request: serde_json::Value,
    headers: &HashMap<String, String>,
) -> Result<EventEnvelope, AppError> {
    let cid = headers.get(CID).cloned();
    log::info!(
        "Processing request (cid={}): {request}",
        cid.as_deref().unwrap_or("null")
    );
    let response = serde_json::json!({
        "status": "processed",
        "processedBy": "system-of-record",
        // ISO-8601 UTC with milliseconds - the rendering the Java demo's Instant gets
        "processedAt": iso8601_utc_now(),
        "traceId": headers.get(MY_TRACE_ID),   // continuous across the Kafka hops
        "request": request,
    });
    reply(response, cid)
}

fn reply(response: serde_json::Value, cid: Option<String>) -> Result<EventEnvelope, AppError> {
    let payload = serde_json::to_vec(&response)
        .map_err(|e| AppError::new(500, format!("unable to render the reply - {e}")))?;
    let mut result = EventEnvelope::new().set_raw_body(Value::Binary(payload));
    if let Some(cid) = cid {
        result = result.set_header(CID, &cid);
    }
    Ok(result)
}

// ---------------------------------------------------------------------------
// the facade: the schema-typed reply variants and the status policy
// ---------------------------------------------------------------------------

/// JSON Schema variant of the extension's `soa.reply` task, for the
/// `soa-reply-json` flow on the facade pod (bound to `json-topic-2`). The
/// adapter decoded the Confluent-framed reply to a map; the delivery is
/// schema-type-agnostic — the map is rendered to JSON text and handed to the
/// return-route coordinator, which completes the awaiting REST request
/// (cross-pod via the Redis return route), so `sync.await` returns that body.
#[preload(route = "soa.reply.json", instances = 250)]
#[derive(Default)]
pub struct SoaReplyJson;

#[async_trait]
impl ComposableFunction for SoaReplyJson {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        deliver_decoded(&headers, &input).await
    }
}

/// Avro variant of the extension's `soa.reply` task, for the `soa-reply-avro`
/// flow on the facade pod (bound to `avro-topic-2`); identical in behaviour to
/// [`SoaReplyJson`] once the adapter has decoded the frame.
#[preload(route = "soa.reply.avro", instances = 250)]
#[derive(Default)]
pub struct SoaReplyAvro;

#[async_trait]
impl ComposableFunction for SoaReplyAvro {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        deliver_decoded(&headers, &input).await
    }
}

async fn deliver_decoded(
    headers: &HashMap<String, String>,
    input: &EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let cid = headers
        .get(CID)
        .ok_or_else(|| AppError::new(400, format!("Missing '{CID}' header")))?;
    tasks::deliver(cid, input).await
}

/// Exception handler for the `sync-to-async` flows (referenced by the flow's
/// `exception:` tag). The flow maps `error.code -> status` and
/// `error.message -> message` into this task; the returned `status` becomes the
/// HTTP status (`result.status -> output.status`).
///
/// Status policy lives here (the HTTP facade), not in the generic building
/// blocks: a 408 from `sync.await` (the backend did not reply in time) passes
/// through, while any 5xx — a Kafka publish failure or a Redis registration
/// failure, i.e. the async backend is unreachable — is re-mapped to **503
/// Service Unavailable** so the caller can retry or take an alternate path. The
/// pending entry registered by `sync.prepare` is cancelled when the publish step
/// fails (the fail-fast path).
#[preload(route = "sync.error.handler", instances = 10, typed)]
#[derive(Default)]
pub struct SyncErrorHandler;

#[async_trait]
impl TypedFunction<HashMap<String, serde_json::Value>, serde_json::Value> for SyncErrorHandler {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: HashMap<String, serde_json::Value>,
        _instance: usize,
    ) -> Result<serde_json::Value, AppError> {
        Ok(error_policy(&input))
    }
}

/// The status policy, separated so it is directly testable.
fn error_policy(input: &HashMap<String, serde_json::Value>) -> serde_json::Value {
    if let Some(cid) = input.get(CID).and_then(|c| match c {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Null => None,
        other => Some(other.to_string()),
    }) {
        if let Some(coordinator) = runtime::coordinator() {
            coordinator.abort(&cid);
        }
    }
    let mut status = input
        .get("status")
        .and_then(|s| {
            s.as_i64()
                .or_else(|| s.as_str().and_then(|t| t.trim().parse().ok()))
        })
        .unwrap_or(500);
    if status >= 500 {
        status = SERVICE_UNAVAILABLE; // backend unreachable (publish/registration failure) -> retriable 503
    }
    serde_json::json!({
        "type": "error",
        "status": status,
        "message": input
            .get("message")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("Internal error"),
    })
}

#[cfg(test)]
mod tests {
    //! The system-of-record tasks derive their trace context from the input
    //! headers and do not publish (the flows do), so all three variants are
    //! directly testable. The soa.reply variants hand replies to the Redis
    //! return-route coordinator and are exercised by the multi-terminal run.

    use super::*;

    fn flow_headers(cid: Option<&str>) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("my_route".to_string(), "system.of.record".to_string());
        headers.insert(MY_TRACE_ID.to_string(), "trace-9999".to_string());
        headers.insert(
            "my_trace_path".to_string(),
            "KAFKA /soa.request".to_string(),
        );
        if let Some(cid) = cid {
            headers.insert(CID.to_string(), cid.to_string());
        }
        headers
    }

    fn body_json(result: &EventEnvelope) -> serde_json::Value {
        let Value::Binary(bytes) = result.body() else {
            panic!("the reply is JSON bytes")
        };
        serde_json::from_slice(bytes).expect("json")
    }

    fn map(json: serde_json::Value) -> HashMap<String, serde_json::Value> {
        json.as_object()
            .expect("object")
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    #[tokio::test]
    async fn byte_variant_echoes_the_request_with_metadata() {
        let request = EventEnvelope::new().set_raw_body(Value::Binary(
            br#"{"action":"create","order":"A-100"}"#.to_vec(),
        ));
        let result = SystemOfRecord
            .handle_event(flow_headers(Some("cid-1")), request, 1)
            .await
            .expect("reply");
        assert_eq!(Some("cid-1"), result.header(CID));
        let response = body_json(&result);
        assert_eq!("processed", response["status"]);
        assert_eq!("system-of-record", response["processedBy"]);
        assert_eq!("trace-9999", response["traceId"]);
        assert_eq!("create", response["request"]["action"]);
        assert!(response["processedAt"]
            .as_str()
            .unwrap_or_default()
            .ends_with('Z'));
    }

    #[tokio::test]
    async fn json_schema_variant_shares_the_same_logic() {
        let result = SystemOfRecordJson
            .handle_event(
                flow_headers(Some("cid-2")),
                map(serde_json::json!({"action": "update", "order": "B-200"})),
                1,
            )
            .await
            .expect("reply");
        assert_eq!(Some("cid-2"), result.header(CID));
        let response = body_json(&result);
        assert_eq!("processed", response["status"]);
        assert_eq!("update", response["request"]["action"]);
    }

    #[tokio::test]
    async fn avro_variant_builds_the_flat_closed_shape_reply() {
        let result = SystemOfRecordAvro
            .handle_event(
                flow_headers(None),
                map(serde_json::json!({"action": "create"})),
                1,
            )
            .await
            .expect("reply");
        assert_eq!(None, result.header(CID));
        // a flat reply matching the Avro SyncDemoMessage record exactly
        assert_eq!(
            serde_json::json!({"action": "create", "status": "processed",
                "processedBy": "system-of-record", "traceId": "trace-9999"}),
            body_json(&result)
        );
    }

    #[test]
    fn error_handler_status_policy() {
        // a 408 timeout from sync.await passes through
        let timeout = error_policy(&map(
            serde_json::json!({"status": 408, "message": "Timeout for 5000 ms"}),
        ));
        assert_eq!(408, timeout["status"]);
        assert_eq!("Timeout for 5000 ms", timeout["message"]);
        assert_eq!("error", timeout["type"]);
        // any 5xx (backend unreachable) is re-mapped to a retriable 503
        let backend_down = error_policy(&map(
            serde_json::json!({"status": 500, "message": "Kafka publish failed"}),
        ));
        assert_eq!(503, backend_down["status"]);
        // defaults apply when the flow maps nothing
        let defaults = error_policy(&HashMap::new());
        assert_eq!(503, defaults["status"]);
        assert_eq!("Internal error", defaults["message"]);
    }
}
