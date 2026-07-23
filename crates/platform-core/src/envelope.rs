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

//! Rust port of the Java `EventEnvelope`
//! (`org.platformlambda.core.models.EventEnvelope`) — the immutable message
//! container between composable functions.
//!
//! Three parts, as in Java: **metadata** (routing, correlation, tracing, status,
//! timing), **headers** (`String → String`), and a dynamic **body**
//! (`rmpv::Value` — the analog of Java's untyped `Object` payload).
//!
//! Wire format: **idiomatic serde MsgPack** (design D4) — deliberately *not*
//! byte-compatible with Java's compact flag-keyed encoding, since cross-JVM
//! interop is out of scope. Later fields (`tags`, `annotations`, `span_id`,
//! serialized exceptions) arrive with the increments that need them.

use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::function::AppError;

/// The immutable message container between functions. Build with the fluent
/// setters (`EventEnvelope::new().set_to("v1.echo").set_body(...)`).
/// Wire format (increment 59): the **standard event envelope wire format** —
/// one MsgPack map with these descriptive string keys, shared verbatim with
/// the Java engine for Event over HTTP (normative spec:
/// `docs/guides/event-envelope-wire-format.md` in the Java repo; golden
/// vectors under `tests/resources/envelope-vectors/`). Encoders emit `id` and
/// `headers` always and other fields only when set; decoders treat absent and
/// nil identically and ignore unknown keys (Java may add `tags`,
/// `annotations`, `stack`, `obj_type`, `exception`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventEnvelope {
    id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reply_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    trace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    trace_path: Option<String>,
    /// The sender's OTel span id, carried so the receiver knows its own
    /// parent span (Java parity — the `s` flag on the wire).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    span_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    status: Option<i32>,
    headers: HashMap<String, String>,
    /// Java omits an unset body on the wire, so absent decodes as `Nil`.
    #[serde(default = "nil_value", skip_serializing_if = "is_nil")]
    body: rmpv::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    exec_time: Option<f32>,
    /// RPC round-trip milliseconds (Java `roundTrip`) — carried for the wire
    /// format; stamped by callers that measure it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    round_trip: Option<f32>,
}

fn nil_value() -> rmpv::Value {
    rmpv::Value::Nil
}

fn is_nil(value: &rmpv::Value) -> bool {
    matches!(value, rmpv::Value::Nil)
}

impl Default for EventEnvelope {
    fn default() -> Self {
        EventEnvelope {
            id: uuid::Uuid::new_v4().simple().to_string(),
            to: None,
            from: None,
            reply_to: None,
            cid: None,
            trace_id: None,
            trace_path: None,
            span_id: None,
            status: None,
            headers: HashMap::new(),
            body: rmpv::Value::Nil,
            exec_time: None,
            round_trip: None,
        }
    }
}

impl EventEnvelope {
    pub fn new() -> Self {
        Self::default()
    }

    // ---- fluent builders (Java setX chaining) ----

    pub fn set_to(mut self, route: &str) -> Self {
        self.to = Some(route.to_string());
        self
    }

    pub fn set_from(mut self, route: &str) -> Self {
        self.from = Some(route.to_string());
        self
    }

    pub fn set_reply_to(mut self, route: &str) -> Self {
        self.reply_to = Some(route.to_string());
        self
    }

    /// Remove the reply-to address (Java `setReplyTo(null)`) — used when an
    /// event is re-targeted, e.g. the declarative Event-over-HTTP forward
    /// nulls the local callback route before the envelope crosses the wire.
    pub fn clear_reply_to(mut self) -> Self {
        self.reply_to = None;
        self
    }

    pub fn set_correlation_id(mut self, cid: &str) -> Self {
        self.cid = Some(cid.to_string());
        self
    }

    pub fn set_trace(mut self, trace_id: &str, trace_path: &str) -> Self {
        self.trace_id = Some(trace_id.to_string());
        self.trace_path = Some(trace_path.to_string());
        self
    }

    /// Carry a span id on the envelope — the sender's span, which the receiver
    /// adopts as its `parent_span_id` (OTel lineage).
    pub fn set_span_id(mut self, span_id: &str) -> Self {
        self.span_id = Some(span_id.to_string());
        self
    }

    pub fn set_status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    pub fn set_header(mut self, key: &str, value: &str) -> Self {
        // Java setHeader guarantees CR/LF never enter a header value
        // (header-injection guard); same filter here
        let value: String = value.chars().filter(|c| *c != '\r' && *c != '\n').collect();
        self.headers.insert(key.to_string(), value);
        self
    }

    /// Serialize any `Serialize` value into the dynamic body
    /// (the analog of Java's `setBody(Object)`).
    pub fn set_body<T: Serialize>(mut self, value: T) -> Result<Self, AppError> {
        self.body = rmpv::ext::to_value(value)
            .map_err(|e| AppError::new(500, format!("unable to serialize body: {e}")))?;
        Ok(self)
    }

    /// Set the body from an already-dynamic value.
    pub fn set_raw_body(mut self, value: rmpv::Value) -> Self {
        self.body = value;
        self
    }

    // ---- getters ----

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn to(&self) -> Option<&str> {
        self.to.as_deref()
    }

    pub fn from(&self) -> Option<&str> {
        self.from.as_deref()
    }

    pub fn reply_to(&self) -> Option<&str> {
        self.reply_to.as_deref()
    }

    pub fn correlation_id(&self) -> Option<&str> {
        self.cid.as_deref()
    }

    pub fn trace_id(&self) -> Option<&str> {
        self.trace_id.as_deref()
    }

    pub fn trace_path(&self) -> Option<&str> {
        self.trace_path.as_deref()
    }

    /// The sender's span id (the receiver's parent span).
    pub fn span_id(&self) -> Option<&str> {
        self.span_id.as_deref()
    }

    /// HTTP-style status; unset means 200 (Java `getStatus`).
    pub fn status(&self) -> i32 {
        self.status.unwrap_or(200)
    }

    /// An error condition is a status code >= 400 (Java `hasError`).
    pub fn has_error(&self) -> bool {
        self.status() >= 400
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn header(&self, key: &str) -> Option<&str> {
        // Java getHeader falls back to a case-insensitive scan when the
        // exact key is absent
        if let Some(value) = self.headers.get(key) {
            return Some(value.as_str());
        }
        self.headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(key))
            .map(|(_, value)| value.as_str())
    }

    pub fn body(&self) -> &rmpv::Value {
        &self.body
    }

    /// Deserialize the dynamic body into a concrete type
    /// (the analog of Java's `getBody(Class)`).
    pub fn body_as<T: DeserializeOwned>(&self) -> Result<T, AppError> {
        rmpv::ext::from_value(self.body.clone())
            .map_err(|e| AppError::new(500, format!("unable to deserialize body: {e}")))
    }

    /// Function execution time in milliseconds, when stamped by a worker.
    pub fn exec_time(&self) -> Option<f32> {
        self.exec_time
    }

    /// RPC round-trip milliseconds (Java `getRoundTrip`), when measured.
    pub fn round_trip(&self) -> Option<f32> {
        self.round_trip
    }

    /// Stamp the RPC round-trip time (Java parity: the requester measures
    /// the full request/response cycle).
    pub fn set_round_trip(mut self, ms: f32) -> Self {
        self.round_trip = Some(ms);
        self
    }

    // ---- crate-internal mutators (worker bookkeeping) ----

    pub(crate) fn set_body_internal(&mut self, body: rmpv::Value) {
        self.body = body;
    }

    pub(crate) fn set_cid_internal(&mut self, cid: Option<String>) {
        self.cid = cid;
    }

    pub(crate) fn set_from_internal(&mut self, from: &str) {
        self.from = Some(from.to_string());
    }

    pub(crate) fn set_to_internal(&mut self, to: &str) {
        self.to = Some(to.to_string());
    }

    pub(crate) fn set_exec_time_internal(&mut self, ms: f32) {
        self.exec_time = Some(ms);
    }

    pub(crate) fn set_trace_internal(&mut self, trace_id: &str, trace_path: &str) {
        self.trace_id = Some(trace_id.to_string());
        self.trace_path = Some(trace_path.to_string());
    }

    pub(crate) fn set_span_id_internal(&mut self, span_id: &str) {
        self.span_id = Some(span_id.to_string());
    }

    pub(crate) fn clear_span_id_internal(&mut self) {
        self.span_id = None;
    }

    // ---- wire format ----

    /// Encode the envelope as MsgPack bytes (idiomatic serde — design D4).
    ///
    /// The body's `Nil` map entries are omitted unless `serializer.null.transport`
    /// is `true` — the Rust mirror of Java `MsgPack.packMap`'s null-skip. Since
    /// increment 58 (the F2 decision) the same strip also runs explicitly on the
    /// in-memory fast path (`platform::normalize_null_transport`), so delivery
    /// semantics are deterministic on every hop — here it is normally a no-op.
    /// The clone + strip runs **only** when the body actually carries a
    /// strippable `Nil` (`has_nil_map_entry`); otherwise — a scalar body, a
    /// structured body with no nulls, or transport on — `self` encodes directly
    /// with no extra allocation, so the common case pays only a read-only scan.
    pub fn to_bytes(&self) -> Result<Vec<u8>, AppError> {
        if crate::serializer::null_transport() || !crate::serializer::has_nil_map_entry(&self.body)
        {
            return rmp_serde::to_vec_named(self)
                .map_err(|e| AppError::new(500, format!("unable to encode envelope: {e}")));
        }
        let mut stripped = self.clone();
        stripped.body = crate::serializer::strip_nulls_always(&self.body);
        rmp_serde::to_vec_named(&stripped)
            .map_err(|e| AppError::new(500, format!("unable to encode envelope: {e}")))
    }

    /// Decode an envelope from MsgPack bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, AppError> {
        rmp_serde::from_slice(bytes)
            .map_err(|e| AppError::new(500, format!("unable to decode envelope: {e}")))
    }
}
