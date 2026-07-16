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

//! Rust port of the Java `PostOffice` (`org.platformlambda.core.system.PostOffice`)
//! — the inter-function messaging client.
//!
//! Increment 2 covers the two core patterns: [`send`](PostOffice::send)
//! (fire-and-forget) and [`request`](PostOffice::request) (RPC). RPC works
//! exactly like the Java `AsyncInbox`: a **temporary inbox route** is registered,
//! the outbound envelope carries `reply_to = inbox` + a correlation id, and the
//! caller awaits the reply with a timeout (→ status **408** on expiry). The
//! inbox is always released afterwards. Fork-n-join, `send_later`, and tracing
//! propagation arrive in later increments.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::oneshot;

use crate::envelope::EventEnvelope;
use crate::function::{AppError, ComposableFunction};
use crate::platform::Platform;
use crate::trace;

/// Stamp the current trace context onto an outbound event (Java parity:
/// the sender's PostOffice propagates its trace and carries its span id so
/// the receiver knows its parent span; the business correlation-id follows
/// the request to the next touch point when the event has none of its own).
/// No-op outside a trace bracket.
fn apply_current_trace(mut event: EventEnvelope) -> EventEnvelope {
    let snapshot = trace::with_current(|state| {
        (
            state.route.clone(),
            state.trace_id.clone(),
            state.trace_path.clone(),
            state.span_id.clone(),
            state.cid.clone(),
        )
    });
    if let Some((route, trace_id, trace_path, span_id, cid)) = snapshot {
        event = event
            .set_trace(&trace_id, &trace_path)
            .set_span_id(&span_id);
        if event.from().is_none() {
            event = event.set_from(&route);
        }
        if event.correlation_id().is_none() {
            if let Some(cid) = cid {
                event = event.set_correlation_id(&cid);
            }
        }
    }
    event
}

/// The messaging client. Cheap to clone; holds a handle to the [`Platform`].
#[derive(Clone)]
pub struct PostOffice {
    platform: Platform,
}

impl PostOffice {
    pub fn new(platform: &Platform) -> Self {
        PostOffice {
            platform: platform.clone(),
        }
    }

    /// Fire-and-forget delivery to `event.to` (Java `po.send`).
    /// Errors: 400 when `to` is missing, 404 when the route is not registered.
    /// Awaits when the route's bounded manager mailbox is full — reactive
    /// back-pressure, not drops.
    ///
    /// When called from inside a traced function, the platform propagates the
    /// trace automatically: the outbound event carries the current trace
    /// id/path, this function's span id (the receiver's parent span), the
    /// sender route, and the business correlation-id when the event has none.
    pub async fn send(&self, event: EventEnvelope) -> Result<(), AppError> {
        let event = apply_current_trace(event);
        let Some(route) = event.to().map(str::to_string) else {
            return Err(AppError::new(400, "Missing routing path ('to')"));
        };
        self.platform.deliver(&route, event).await
    }

    // ---- trace-aware conveniences (Java PostOffice business APIs) ----

    /// The business correlation-id of the current traced request
    /// (Java `getMyCorrelationId`). `None` outside a trace or when the
    /// incoming event carried none.
    pub fn my_correlation_id(&self) -> Option<String> {
        trace::with_current(|state| state.cid.clone()).flatten()
    }

    /// The current trace id (Java `getTraceId` on the trace-aware PostOffice).
    pub fn my_trace_id(&self) -> Option<String> {
        trace::with_current(|state| state.trace_id.clone())
    }

    /// The current trace path.
    pub fn my_trace_path(&self) -> Option<String> {
        trace::with_current(|state| state.trace_path.clone())
    }

    /// Attach business context to the **distributed-trace dataset** that flows
    /// to the telemetry sink (Java `annotateTrace`). Silent no-op outside a
    /// trace.
    pub fn annotate_trace(&self, key: &str, value: impl serde::Serialize) -> &Self {
        if let Ok(value) = serde_json::to_value(value) {
            trace::with_current_mut(|state| {
                state.annotations.insert(key.to_string(), value);
            });
        }
        self
    }

    /// Attach business context to the **application log** stream only (Java
    /// `updateContext`) — appears in the `context` block of every subsequent
    /// structured log line of this request. A `null` value removes the key.
    /// The reserved keys (cid, traceId, tracePath, spanId, parentSpanId,
    /// service, utc) are rejected; outside a trace the call is a silent no-op.
    pub fn update_context(&self, key: &str, value: impl serde::Serialize) -> Result<(), AppError> {
        if crate::trace::RESERVED_KEYS.contains(&key) {
            return Err(AppError::new(
                400,
                format!("'{key}' is a reserved log context key"),
            ));
        }
        let value = serde_json::to_value(value)
            .map_err(|e| AppError::new(400, format!("unable to serialize context value: {e}")))?;
        trace::with_current_mut(|state| {
            if value.is_null() {
                state.custom_log_keys.remove(key);
            } else {
                state.custom_log_keys.insert(key.to_string(), value);
            }
        });
        Ok(())
    }

    /// RPC (Java `po.request(event, timeout)`): deliver the event and await the
    /// reply through a temporary inbox. Timeout → status **408**.
    pub async fn request(
        &self,
        event: EventEnvelope,
        timeout: Duration,
    ) -> Result<EventEnvelope, AppError> {
        // propagate the trace context first, so a business correlation-id
        // riding the current trace wins over a minted one
        let event = apply_current_trace(event);
        // temporary reply inbox — a valid (dotted, lowercase) dynamic route
        let inbox_route = format!("inbox.{}", uuid::Uuid::new_v4().simple());
        let (tx, rx) = oneshot::channel::<EventEnvelope>();
        self.platform
            .register(&inbox_route, Arc::new(OneshotInbox::new(tx)), 1)?;
        // correlation id: keep the caller's, or mint one (Java parity)
        let cid = event
            .correlation_id()
            .map(str::to_string)
            .unwrap_or_else(|| uuid::Uuid::new_v4().simple().to_string());
        let event = event.set_reply_to(&inbox_route).set_correlation_id(&cid);
        if let Err(e) = self.send(event).await {
            self.platform.release(&inbox_route);
            return Err(e);
        }
        let outcome = tokio::time::timeout(timeout, rx).await;
        self.platform.release(&inbox_route);
        match outcome {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => Err(AppError::new(500, "Reply channel closed unexpectedly")),
            Err(_) => Err(AppError::new(
                408,
                format!("Request timeout for {} ms", timeout.as_millis()),
            )),
        }
    }
}

/// The temporary inbox function: completes the caller's oneshot with the first
/// reply it receives (the Java `AsyncInbox` analog).
struct OneshotInbox {
    tx: Mutex<Option<oneshot::Sender<EventEnvelope>>>,
}

impl OneshotInbox {
    fn new(tx: oneshot::Sender<EventEnvelope>) -> Self {
        OneshotInbox {
            tx: Mutex::new(Some(tx)),
        }
    }
}

#[async_trait]
impl ComposableFunction for OneshotInbox {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if let Some(tx) = self.tx.lock().expect("inbox mutex poisoned").take() {
            // caller may have timed out already — dropping the reply is correct
            let _ = tx.send(input);
        }
        Ok(EventEnvelope::new())
    }
}
