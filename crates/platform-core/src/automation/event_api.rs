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

//! Event over HTTP (increment 61) — Rust port of Java
//! `org.platformlambda.core.services.EventApiService` + the `EventEmitter`
//! event-over-http client. A serialized [`EventEnvelope`] (the **standard**
//! wire format, increment 59) is exchanged over `POST /api/event`, so a
//! function in one application instance can call a **public** function
//! (`is_private = false`) in another — the only cross-instance coupling, and
//! opt-in by design.
//!
//! The service is reached through REST automation (the `/api/event` entry
//! ships in the default `rest.yaml`, merged like the actuators). It decodes
//! the posted envelope, enforces the visibility boundary (403 for a private
//! target — a remote caller must never reach engine internals or an
//! unpublished function), and dispatches: async (`x-async: true`) is
//! drop-n-forget with a 202 ack; otherwise RPC up to `x-ttl` ms.

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use rmpv::Value;

use crate::automation::http_client::{AsyncHttpRequest, ASYNC_HTTP_REQUEST};
use crate::envelope::EventEnvelope;
use crate::function::{AppError, ComposableFunction};
use crate::platform::Platform;
use crate::post_office::PostOffice;
use crate::util::w3c_trace;

/// Route of the event-over-http service (Java `EventApiService.EVENT_API_SERVICE`).
pub const EVENT_API_SERVICE: &str = "event.api.service";

const OCTET_STREAM: &str = "application/octet-stream";
const X_TTL: &str = "x-ttl";
const X_ASYNC: &str = "x-async";

/// The `/api/event` service (Java `EventApiService`). Registered PRIVATE — it
/// is reached only through the REST boundary, never as a remote target.
pub struct EventApiService {
    platform: Platform,
}

impl EventApiService {
    pub fn new(platform: &Platform) -> Self {
        EventApiService {
            platform: platform.clone(),
        }
    }
}

#[async_trait]
impl ComposableFunction for EventApiService {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request = AsyncHttpRequest::from_value(input.body());
        let timeout_ms = request
            .header(X_TTL)
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0)
            .max(1000);
        let is_async = request.header(X_ASYNC) == Some("true");
        // the HTTP body is the serialized envelope; octet-stream arrives as a
        // MsgPack-binary body on the request map (Java parity)
        let Value::Binary(bytes) = request.body() else {
            return Ok(reply(500, b"Invalid event-over-http data format".to_vec()));
        };
        // v1 accepts the standard wire format only (phase-2 decision); a
        // compact (all single-char keys) envelope is rejected clearly
        if is_compact_envelope(bytes) {
            return Ok(reply(
                400,
                error_envelope(
                    400,
                    "compact format not supported - set event.over.http.format=standard on the sender",
                ),
            ));
        }
        let inner = match EventEnvelope::from_bytes(bytes) {
            Ok(envelope) => envelope,
            // the format is unknown when decode fails (Java falls back to a
            // compact error reply; we answer 400 with a plain message)
            Err(e) => return Ok(reply(400, error_envelope(400, e.message()))),
        };
        let Some(to) = inner.to().map(str::to_string) else {
            return Ok(reply(400, error_envelope(400, "Missing routing path")));
        };
        if !self.platform.has_route(&to) {
            return Ok(reply(
                404,
                error_envelope(404, &format!("Route {to} not found")),
            ));
        }
        if self.platform.is_private(&to) == Some(true) {
            return Ok(reply(403, error_envelope(403, &format!("{to} is private"))));
        }
        let po = PostOffice::new(&self.platform);
        if is_async {
            // drop-n-forget: deliver and acknowledge (Java 202 ack shape)
            po.send(inner).await?;
            let ack = EventEnvelope::new()
                .set_status(202)
                .set_body(serde_json::json!({
                    "type": "async",
                    "delivered": true,
                    "time": crate::trace::iso8601_utc_now(),
                }))?;
            Ok(reply(200, ack.to_bytes()?))
        } else {
            // RPC: forward and mirror the target's envelope back (or 408)
            match po.request(inner, Duration::from_millis(timeout_ms)).await {
                Ok(result) => Ok(reply(200, result.to_bytes()?)),
                Err(e) => Ok(reply(408, error_envelope(408, e.message()))),
            }
        }
    }
}

/// Build the HTTP response envelope: the body is a serialized envelope carried
/// raw as `application/octet-stream` (Java `sendResponse`/`sendError`). The
/// outer status is 200 for a successful dispatch (the real result status rides
/// inside the serialized body) and the error code for a service-level error.
fn reply(http_status: i32, body: Vec<u8>) -> EventEnvelope {
    EventEnvelope::new()
        .set_status(http_status)
        .set_header("content-type", OCTET_STREAM)
        .set_raw_body(Value::Binary(body))
}

/// A serialized error envelope (status + message body) — the payload the
/// client deserializes and hands back to its caller.
fn error_envelope(status: i32, message: &str) -> Vec<u8> {
    EventEnvelope::new()
        .set_status(status)
        .set_raw_body(Value::from(message))
        .to_bytes()
        .unwrap_or_default()
}

/// A compact (legacy Java) envelope has ONLY single-character top-level keys;
/// the standard format's keys are all longer, so the namespaces are disjoint.
fn is_compact_envelope(bytes: &[u8]) -> bool {
    match rmp_serde::from_slice::<Value>(bytes) {
        Ok(Value::Map(entries)) if !entries.is_empty() => entries
            .iter()
            .all(|(k, _)| k.as_str().is_some_and(|s| s.chars().count() == 1)),
        _ => false,
    }
}

/// Event-over-http client (Java `EventEmitter.asyncRequest`/`eRequest` over an
/// endpoint): POST `event` to `{endpoint}` as a serialized standard envelope.
/// `rpc = true` awaits the target's reply up to `timeout`; `rpc = false` is
/// drop-n-forget (the 202 ack envelope is returned). Trace context propagates
/// via `x-trace-id` + W3C `traceparent` so cross-language traces chain.
pub async fn event_over_http(
    po: &PostOffice,
    endpoint: &str,
    event: EventEnvelope,
    timeout: Duration,
    rpc: bool,
) -> Result<EventEnvelope, AppError> {
    let (host, path) = split_endpoint(endpoint)?;
    let trace_id = event.trace_id().map(str::to_string);
    let span_id = event.span_id().map(str::to_string);
    let payload = event.to_bytes()?;
    let mut http = AsyncHttpRequest::new()
        .set_method("POST")
        .set_url(&path)
        .set_target_host(&host)
        .set_header("content-type", OCTET_STREAM)
        .set_header(X_TTL, &timeout.as_millis().max(1000).to_string())
        .set_body(Value::Binary(payload));
    if !rpc {
        http = http.set_header(X_ASYNC, "true");
    }
    // trace propagation (Java sets both headers so the receiver chains onto
    // this span as its parent)
    if let Some(trace_id) = &trace_id {
        http = http.set_header("x-trace-id", trace_id);
        if let Some(span_id) = &span_id {
            if let Some(traceparent) = w3c_trace::format(trace_id, span_id) {
                http = http.set_header(w3c_trace::TRACEPARENT, &traceparent);
            }
        }
    }
    let http_event = EventEnvelope::new()
        .set_to(ASYNC_HTTP_REQUEST)
        .set_raw_body(http.to_value());
    // the local wait gets a small grace over the remote TTL (Java parity:
    // EventEmitter's inner deadline is timeout + 100ms) so a peer that spends
    // its whole TTL still replies in-band — its 408 envelope must win the
    // race against the local abort, never lose it
    let response = po
        .request(http_event, timeout + Duration::from_millis(100))
        .await?;
    // the response body is the serialized reply envelope (octet-stream →
    // binary); anything else is a transport-level failure
    match response.body() {
        Value::Binary(bytes) => EventEnvelope::from_bytes(bytes),
        other => Err(AppError::new(
            500,
            format!("Invalid event-over-http response: {other:?}"),
        )),
    }
}

/// Split `http://host:port/api/event` into (`http://host:port`, `/api/event`).
fn split_endpoint(endpoint: &str) -> Result<(String, String), AppError> {
    let scheme_end = endpoint
        .find("://")
        .map(|i| i + 3)
        .ok_or_else(|| AppError::new(400, format!("Invalid endpoint {endpoint}")))?;
    match endpoint[scheme_end..].find('/') {
        Some(offset) => {
            let split = scheme_end + offset;
            Ok((endpoint[..split].to_string(), endpoint[split..].to_string()))
        }
        None => Ok((endpoint.to_string(), "/api/event".to_string())),
    }
}
