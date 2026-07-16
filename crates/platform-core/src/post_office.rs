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
//! Two core patterns: [`send`](PostOffice::send) (fire-and-forget) and
//! [`request`](PostOffice::request) (RPC). RPC works exactly like the Java
//! `AsyncInbox`: a **lightweight one-shot inbox** (see [`crate::inbox`]) —
//! one correlation-map entry, no route registration — receives the reply,
//! bypassing the ServiceQueue machinery entirely; the caller awaits with a
//! timeout (→ status **408** on expiry). Fork-n-join and `send_later` arrive
//! in later increments.

use std::time::Duration;

use crate::envelope::EventEnvelope;
use crate::function::AppError;
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
        // a lightweight one-shot inbox (Java AsyncInbox parity): one map entry,
        // no route registration — the reply bypasses the ServiceQueue machinery
        let (inbox_id, rx) = crate::inbox::open();
        // correlation id: keep the caller's, or mint one (Java parity)
        let cid = event
            .correlation_id()
            .map(str::to_string)
            .unwrap_or_else(|| uuid::Uuid::new_v4().simple().to_string());
        let event = event.set_reply_to(&inbox_id).set_correlation_id(&cid);
        if let Err(e) = self.send(event).await {
            crate::inbox::close(&inbox_id);
            return Err(e);
        }
        let outcome = tokio::time::timeout(timeout, rx).await;
        match outcome {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => Err(AppError::new(500, "Reply channel closed unexpectedly")),
            Err(_) => {
                crate::inbox::close(&inbox_id);
                Err(AppError::new(
                    408,
                    format!("Request timeout for {} ms", timeout.as_millis()),
                ))
            }
        }
    }
}
