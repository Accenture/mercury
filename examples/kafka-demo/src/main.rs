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

//! **kafka-demo** — the minimalist-kafka worked example (Rust twin of the
//! Java `examples/kafka-demo`). There is no application code to start here:
//! the Kafka building blocks autoload. Linking `mercury-minimalist-kafka`
//! registers `simple.kafka.notification` and `kafka.health` and runs its
//! auto-start, which builds the producer and starts the **Kafka flow
//! adapter** from `resources/kafka-flow-adapter.yaml`:
//!
//! - `demo.inbound` → `kafka-demo-flow` (**direct routing** — every message
//!   goes to one flow): `demo.processor` wraps the message with processing
//!   metadata and `simple.kafka.notification` publishes the result to
//!   `demo.outbound`;
//! - `demo.orders` → a `flows` rule list (**second-level routing** — the
//!   target is picked per record): `flow://demo-order-flow`,
//!   `task://demo.refund.processor`, or the mandatory `default`
//!   `flow://demo-catch-all-flow`.
//!
//! The functions below are the only code; none knows the others exist, and
//! nothing here publishes — the flow YAML does (`event-script-over-code`).
//! Each function derives its trace context from the read-only `my_*` headers
//! the engine injects, so every one is directly unit-testable (see the tests).
//!
//! ```bash
//! cargo run -p kafka-demo                                   # the demo
//! cargo run -p kafka-demo -- -Dapp.profiles.active=interop  # the Java-interop relay legs
//! ```
//!
//! The runbook — the local `kafka-standalone` broker, the Node helper
//! programs, what to expect on each terminal — is in this example's README.

use std::collections::HashMap;

use async_trait::async_trait;
use minimalist_kafka::headers::CORRELATION_ID;
use platform_core::automation::MY_CORRELATION_ID;
use platform_core::trace::iso8601_utc_now;
use platform_core::{
    main_application, preload, w3c_trace, AppError, ComposableFunction, EntryPoint, EventEnvelope,
    TypedFunction,
};
use rmpv::Value;

/// The engine injects the function's own trace id under this read-only input
/// header (alongside `my_route`, `my_trace_path` and `my_correlation_id`).
const MY_TRACE_ID: &str = "my_trace_id";
/// The injected trace path names the hop that started the trace segment — for
/// a record routed by the flow adapter, `KAFKA /<topic>` with the record's
/// ACTUAL topic, which is how a function reused by several flows (the interop
/// relay flows reuse `demo.processor`) tells where a message came from.
const MY_TRACE_PATH: &str = "my_trace_path";

/// Renders an optional header value the way the Java demo logs a nullable.
fn or_none(value: Option<&String>) -> &str {
    value.map(String::as_str).unwrap_or("null")
}

// ---------------------------------------------------------------------------
// kafka-demo-flow (direct routing)
// ---------------------------------------------------------------------------

/// The single task of `kafka-demo-flow`, reached when the Kafka flow adapter
/// routes a message from `demo.inbound`. A self-contained function: it reads
/// the inbound text, wraps it with a little processing metadata (who
/// processed it, when, and the trace id) and returns that as JSON bytes. The
/// flow then publishes the result to `demo.outbound` via
/// `simple.kafka.notification` — this function does not publish itself (that
/// is orchestration, expressed in the flow YAML).
///
/// The echoed `traceId` lets you confirm the trace stayed continuous across
/// the Kafka hop, which the telemetry log also shows as the end-to-end span
/// path. The incoming span is the upstream span carried in the record's
/// `traceparent` header, which the flow maps onto this task's input. The log
/// line names the hop from the injected trace path (`KAFKA /<topic>`) rather
/// than a fixed topic, because the interop relay flows reuse this function.
#[preload(route = "demo.processor", instances = 10)]
#[derive(Default)]
struct DemoProcessor;

#[async_trait]
impl ComposableFunction for DemoProcessor {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let cid = headers.get(CORRELATION_ID).cloned();
        let received = match input.body() {
            Value::Binary(bytes) => String::from_utf8_lossy(bytes).to_string(),
            Value::String(text) => text.as_str().unwrap_or_default().to_string(),
            Value::Nil => String::new(),
            other => other.to_string(),
        };
        let trace_id = headers.get(MY_TRACE_ID).cloned();
        let incoming_span = headers
            .get(w3c_trace::TRACEPARENT)
            .and_then(|traceparent| w3c_trace::parse(traceparent))
            .map(|(_, span)| span);
        log::info!(
            "Received from {} (cid={}, traceId={}, incoming span={}): {received}",
            headers
                .get(MY_TRACE_PATH)
                .map(String::as_str)
                .unwrap_or("demo.inbound"),
            or_none(cid.as_ref()),
            or_none(trace_id.as_ref()),
            or_none(incoming_span.as_ref())
        );
        let response = serde_json::json!({
            "received": received,
            "processedBy": "kafka-demo",
            // ISO-8601 UTC with milliseconds - the same rendering the Java
            // demo's Instant gets from its mapper
            "processedAt": iso8601_utc_now(),
            "traceId": trace_id,   // continuous across the Kafka hop
        });
        let payload = serde_json::to_vec(&response)
            .map_err(|e| AppError::new(500, format!("unable to render the response - {e}")))?;
        let mut result = EventEnvelope::new().set_raw_body(Value::Binary(payload));
        if let Some(cid) = cid {
            result = result.set_header(CORRELATION_ID, &cid);
        }
        Ok(result)
    }
}

// ---------------------------------------------------------------------------
// second-level routing targets
// ---------------------------------------------------------------------------

/// The task of `demo-order-flow` — the SPECIFIC FLOW of the second-level
/// routing demo, selected when a `demo.orders` record's `type` header matches
/// the `order` (exact) or `order-*` (wildcard) routing rule.
///
/// The binding's `serializer: 'json'` decoded the record before routing, so
/// the input is a map — a record that is NOT a JSON object cannot be
/// deserialized into it and the task fails, which is the demo's failure path
/// (retries, then the dead-letter topic). The function returns a map too: the
/// flow maps it straight into `simple.kafka.notification`, which
/// auto-serializes a map body to JSON bytes on a non-schema topic (the
/// outbound symmetry of `serializer: 'json'`).
///
/// `type` is the routing key the rule matched on (mapped by the flow from the
/// raw record header), echoed in the response so the outbound message shows
/// which rule fired.
#[preload(route = "demo.order.processor", instances = 10, typed)]
#[derive(Default)]
struct OrderProcessor;

#[async_trait]
impl TypedFunction<HashMap<String, serde_json::Value>, serde_json::Value> for OrderProcessor {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: HashMap<String, serde_json::Value>,
        _instance: usize,
    ) -> Result<serde_json::Value, AppError> {
        let kind = headers.get("type").cloned().unwrap_or_default();
        let trace_id = headers.get(MY_TRACE_ID).cloned();
        log::info!(
            "Order event routed by rule type({kind}) (cid={}, traceId={}): {}",
            or_none(headers.get(MY_CORRELATION_ID)),
            or_none(trace_id.as_ref()),
            serde_json::Value::Object(input.clone().into_iter().collect())
        );
        Ok(serde_json::json!({
            "order": input,
            "routedBy": format!("input.header.type({kind})"),
            "processedBy": "demo-order-flow",
            "processedAt": iso8601_utc_now(),
            "traceId": trace_id,   // continuous across the Kafka hop
        }))
    }
}

/// The SPECIFIC TASK of the second-level routing demo — the
/// `task://demo.refund.processor` target, selected when the
/// `input.body.event.kind(refund)` rule matches a `demo.orders` record's
/// payload (a body rule needs the map that `serializer: 'json'` decoded).
///
/// A `task://` target invokes this function DIRECTLY — no flow, no data
/// mapping. The adapter copies all inbound record headers verbatim onto the
/// input headers, passes the whole decoded payload as the body, and carries
/// the business correlation-id so the engine injects it as
/// `my_correlation_id`, with full trace continuity. Returning normally
/// (status below 400) lets the adapter commit the offset; failing follows the
/// same bounded-retry then dead-letter path as a flow failure.
///
/// Use a task for processing simple enough that a flow is overweight —
/// record, count, notify. This demo task just acknowledges the refund in the
/// application log (watch the app's terminal); anything needing orchestration
/// — like publishing onward — belongs in a `flow://` target instead.
#[preload(route = "demo.refund.processor", instances = 10, typed)]
#[derive(Default)]
struct RefundProcessor;

#[async_trait]
impl TypedFunction<HashMap<String, serde_json::Value>, serde_json::Value> for RefundProcessor {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: HashMap<String, serde_json::Value>,
        _instance: usize,
    ) -> Result<serde_json::Value, AppError> {
        let cid = headers.get(MY_CORRELATION_ID).cloned();
        log::info!(
            "Refund routed by rule input.body.event.kind(refund) (cid={}, traceId={}): {}",
            or_none(cid.as_ref()),
            or_none(headers.get(MY_TRACE_ID)),
            serde_json::Value::Object(input.clone().into_iter().collect())
        );
        Ok(serde_json::json!({
            "status": "refund recorded",
            "refund": input,
            "cid": cid,
        }))
    }
}

/// The task of `demo-catch-all-flow` — the DEFAULT FLOW of the second-level
/// routing demo, reached when NO routing rule matches a `demo.orders` record.
///
/// The body can be either shape here: a map or list (a JSON record that
/// matched no rule) or the raw bytes (a record `serializer: 'json'` could not
/// parse — best-effort by design, the raw bytes simply pass through), so this
/// function stays untyped and handles both. That is exactly the pattern a
/// production default handler should follow.
#[preload(route = "demo.catch.all", instances = 10)]
#[derive(Default)]
struct CatchAllProcessor;

#[async_trait]
impl ComposableFunction for CatchAllProcessor {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        // Raw bytes arrive when the best-effort JSON deserializer could not
        // parse the record, so they are shown as text. A map or list is a
        // well-formed JSON record that matched no routing rule.
        let (received, shape) = match input.body() {
            Value::Binary(bytes) => (
                serde_json::Value::String(String::from_utf8_lossy(bytes).to_string()),
                "raw bytes (not a JSON object/array)",
            ),
            map @ Value::Map(_) => (as_json(map), "map (JSON object, no rule matched)"),
            list @ Value::Array(_) => (as_json(list), "list (JSON array, no rule matched)"),
            other => (serde_json::Value::String(other.to_string()), "other"),
        };
        let trace_id = headers.get(MY_TRACE_ID).cloned();
        log::info!(
            "Unmatched record caught by the default rule (cid={}, traceId={}): {shape}",
            or_none(headers.get(MY_CORRELATION_ID)),
            or_none(trace_id.as_ref())
        );
        EventEnvelope::new().set_body(serde_json::json!({
            "received": received,
            "shape": shape,
            "processedBy": "demo-catch-all-flow",
            "routedBy": "default",
            "traceId": trace_id,
        }))
    }
}

/// A decoded map or list body as JSON (the rendering the outbound message
/// carries).
fn as_json(value: &Value) -> serde_json::Value {
    rmpv::ext::from_value(value.clone()).unwrap_or(serde_json::Value::Null)
}

// ---------------------------------------------------------------------------
// the application
// ---------------------------------------------------------------------------

/// Entry point. By the time it runs the engine has compiled the flows
/// (CompileFlows at sequence 5); the Kafka auto-start (sequence 20) follows
/// and announces each binding. Referencing the event-script crate here also
/// guarantees the linker keeps its annotation inventory.
#[main_application]
struct KafkaDemoApp;

#[async_trait]
impl EntryPoint for KafkaDemoApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!(
            "kafka-demo started - flows {:?}; the flow adapter binds them next",
            event_script::flows::get_all_flows()
        );
        Ok(())
    }
}

platform_core::auto_start_main!();

#[cfg(test)]
mod tests {
    //! Each function derives its context from the reserved `my_*` input
    //! headers a flow (or the adapter's task dispatch) would carry, so the
    //! four are directly testable without a broker — the Java demo's tests,
    //! one to one.

    use super::*;

    fn headers(route: &str, extra: &[(&str, &str)]) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("my_route".to_string(), route.to_string());
        headers.insert(MY_TRACE_ID.to_string(), "trace-1234".to_string());
        headers.insert(
            "my_trace_path".to_string(),
            "KAFKA /demo.orders".to_string(),
        );
        for (key, value) in extra {
            headers.insert(key.to_string(), value.to_string());
        }
        headers
    }

    fn map(json: serde_json::Value) -> HashMap<String, serde_json::Value> {
        json.as_object()
            .expect("object")
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    #[tokio::test]
    async fn demo_processor_wraps_the_inbound_text_with_processing_metadata() {
        let result = DemoProcessor
            .handle_event(
                headers("demo.processor", &[("cid", "order-001")]),
                EventEnvelope::new().set_raw_body(Value::Binary(b"hello kafka".to_vec())),
                1,
            )
            .await
            .expect("processed");
        assert_eq!(Some("order-001"), result.header("cid"));
        let Value::Binary(payload) = result.body() else {
            panic!("the result body is the JSON bytes the flow publishes verbatim");
        };
        let response: serde_json::Value = serde_json::from_slice(payload).expect("json");
        assert_eq!("hello kafka", response["received"]);
        assert_eq!("kafka-demo", response["processedBy"]);
        assert_eq!("trace-1234", response["traceId"]);
        assert!(response["processedAt"]
            .as_str()
            .is_some_and(|t| t.ends_with('Z')));
    }

    #[tokio::test]
    async fn demo_processor_without_correlation_id_stamps_no_cid_header() {
        let result = DemoProcessor
            .handle_event(
                headers("demo.processor", &[]),
                EventEnvelope::new().set_raw_body(Value::Binary(b"plain".to_vec())),
                1,
            )
            .await
            .expect("processed");
        assert!(result.header("cid").is_none());
    }

    #[tokio::test]
    async fn order_processor_wraps_the_order_with_routing_and_processing_metadata() {
        let order = map(serde_json::json!({"item": "keyboard", "qty": 1}));
        let response = OrderProcessor
            .handle_event(
                headers("demo.order.processor", &[("type", "order")]),
                order.clone(),
                1,
            )
            .await
            .expect("processed");
        assert_eq!(
            serde_json::json!({"item": "keyboard", "qty": 1}),
            response["order"]
        );
        assert_eq!("input.header.type(order)", response["routedBy"]);
        assert_eq!("demo-order-flow", response["processedBy"]);
        assert_eq!("trace-1234", response["traceId"]);
        assert!(response["processedAt"].is_string());
    }

    #[tokio::test]
    async fn order_processor_echoes_the_wildcard_matched_routing_key() {
        let response = OrderProcessor
            .handle_event(
                headers("demo.order.processor", &[("type", "order-42")]),
                map(serde_json::json!({"item": "mouse"})),
                1,
            )
            .await
            .expect("processed");
        assert_eq!("input.header.type(order-42)", response["routedBy"]);
    }

    #[tokio::test]
    async fn refund_processor_acknowledges_the_refund_with_the_business_cid() {
        let refund = map(serde_json::json!({"event": {"kind": "refund"}, "orderId": "abc123"}));
        let ack = RefundProcessor
            .handle_event(
                headers(
                    "demo.refund.processor",
                    &[(MY_CORRELATION_ID, "refund-001")],
                ),
                refund,
                1,
            )
            .await
            .expect("acknowledged");
        assert_eq!("refund recorded", ack["status"]);
        assert_eq!("abc123", ack["refund"]["orderId"]);
        assert_eq!("refund-001", ack["cid"]);
    }

    #[tokio::test]
    async fn catch_all_shows_raw_bytes_as_text() {
        let response: serde_json::Value = CatchAllProcessor
            .handle_event(
                headers("demo.catch.all", &[]),
                EventEnvelope::new().set_raw_body(Value::Binary(b"not-json".to_vec())),
                1,
            )
            .await
            .expect("processed")
            .body_as()
            .expect("json body");
        assert_eq!("not-json", response["received"]);
        assert_eq!("raw bytes (not a JSON object/array)", response["shape"]);
        assert_eq!("default", response["routedBy"]);
        assert_eq!("demo-catch-all-flow", response["processedBy"]);
    }

    #[tokio::test]
    async fn catch_all_passes_unmatched_json_shapes_through() {
        let object: serde_json::Value = CatchAllProcessor
            .handle_event(
                headers("demo.catch.all", &[]),
                EventEnvelope::new()
                    .set_body(serde_json::json!({"hello": "world"}))
                    .expect("body"),
                1,
            )
            .await
            .expect("processed")
            .body_as()
            .expect("json body");
        assert_eq!(serde_json::json!({"hello": "world"}), object["received"]);
        assert_eq!("map (JSON object, no rule matched)", object["shape"]);

        let list: serde_json::Value = CatchAllProcessor
            .handle_event(
                headers("demo.catch.all", &[]),
                EventEnvelope::new()
                    .set_body(serde_json::json!([{"n": 1}]))
                    .expect("body"),
                1,
            )
            .await
            .expect("processed")
            .body_as()
            .expect("json body");
        assert_eq!(serde_json::json!([{"n": 1}]), list["received"]);
        assert_eq!("list (JSON array, no rule matched)", list["shape"]);
    }
}
