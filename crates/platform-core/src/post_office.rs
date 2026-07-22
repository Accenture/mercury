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

use crate::automation::event_api;
use crate::envelope::EventEnvelope;
use crate::function::AppError;
use crate::platform::Platform;
use crate::trace;

/// Stamp the current trace context onto an outbound event — the mirror of
/// Java `PostOffice.touch()`: trace id and path are filled **only when the
/// event has none of its own** (an explicitly supplied trace identity always
/// wins — F8 parity fix, 2026-07-21); the span id is stamped unconditionally
/// so the receiver knows its parent span — except inside a zero-traced hop,
/// which owns no span (Java: no live TraceInfo there); `from` and the
/// business correlation-id follow the request when absent.
/// No-op outside a trace bracket.
fn apply_current_trace(mut event: EventEnvelope) -> EventEnvelope {
    let snapshot = trace::with_current(|state| {
        (
            state.route.clone(),
            state.trace_id.clone(),
            state.trace_path.clone(),
            state.span_id.clone(),
            state.cid.clone(),
            state.zero_traced,
        )
    });
    if let Some((route, trace_id, trace_path, span_id, cid, zero_traced)) = snapshot {
        // Java touch(): each trace field fills independently, if-absent
        let effective_id = event.trace_id().unwrap_or(&trace_id).to_string();
        let effective_path = event.trace_path().unwrap_or(&trace_path).to_string();
        event = event.set_trace(&effective_id, &effective_path);
        if !zero_traced {
            event = event.set_span_id(&span_id);
        }
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

/// Pending scheduled deliveries (Java `EventEmitter` future events): timer id
/// → abort handle. Entries remove themselves on firing.
fn scheduled_events(
) -> &'static std::sync::Mutex<std::collections::HashMap<String, tokio::task::AbortHandle>> {
    static TIMERS: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<String, tokio::task::AbortHandle>>,
    > = std::sync::OnceLock::new();
    TIMERS.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
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
    ///
    /// A route declared in `yaml.event.over.http` forwards transparently to
    /// the peer's `/api/event` instead of the local bus (Java
    /// `EventEmitter.send` declarative hook) — user code cannot tell a remote
    /// route from a local one. The `x-event-api` envelope header marks an
    /// event that already crossed the wire, so it is never re-forwarded.
    pub async fn send(&self, event: EventEnvelope) -> Result<(), AppError> {
        let event = apply_current_trace(event);
        let Some(route) = event.to().map(str::to_string) else {
            return Err(AppError::new(400, "Missing routing path ('to')"));
        };
        if event.header(event_api::X_EVENT_API).is_none() {
            if let Some(entry) = event_api::get_event_http_target(&route) {
                return event_api::send_with_event_http(&self.platform, event, &route, entry);
            }
        }
        self.platform.deliver(&route, event).await
    }

    /// Schedule a future one-time delivery (Java `po.sendLater(event, time)`):
    /// the event is sent after `delay`; the returned timer id cancels it via
    /// [`cancel_future_event`](Self::cancel_future_event). The timer rides an
    /// abortable tokio task (map-don't-mirror; increment E-3 — built for the
    /// event-script flow TTL watcher).
    pub fn send_later(&self, event: EventEnvelope, delay: std::time::Duration) -> String {
        // capture the sender/trace/correlation context NOW (Java sendLater
        // wraps the event in touch() before the timer) — the spawned timer
        // task does not inherit the task-local trace bracket (F7 parity fix)
        let event = apply_current_trace(event);
        let timer_id = uuid::Uuid::new_v4().simple().to_string();
        let platform = self.platform.clone();
        let id_for_task = timer_id.clone();
        let handle = tokio::spawn(async move {
            tokio::time::sleep(delay).await;
            scheduled_events()
                .lock()
                .expect("timer registry")
                .remove(&id_for_task);
            if let Some(route) = event.to().map(str::to_string) {
                // deliver through send() so a scheduled event honors the
                // declarative Event-over-HTTP hook exactly like a direct one
                // (the timer task has no trace bracket, so the trace context
                // captured above at schedule time is untouched)
                if let Err(e) = PostOffice::new(&platform).send(event).await {
                    log::warn!(
                        "Unable to deliver scheduled event to {route} - {}",
                        e.message()
                    );
                }
            }
        });
        scheduled_events()
            .lock()
            .expect("timer registry")
            .insert(timer_id.clone(), handle.abort_handle());
        timer_id
    }

    /// Cancel a scheduled delivery (Java `po.cancelFutureEvent(id)`).
    /// Returns whether the timer was still pending.
    pub fn cancel_future_event(&self, timer_id: &str) -> bool {
        match scheduled_events()
            .lock()
            .expect("timer registry")
            .remove(timer_id)
        {
            Some(handle) => {
                handle.abort();
                true
            }
            None => false,
        }
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
    ///
    /// A route declared in `yaml.event.over.http` forwards transparently as an
    /// Event-over-HTTP RPC and returns the peer's reply (Java
    /// `EventEmitter.asyncRequest`/`eRequest` declarative hook); the
    /// `x-event-api` recursion guard applies as in [`send`](Self::send).
    pub async fn request(
        &self,
        event: EventEnvelope,
        timeout: Duration,
    ) -> Result<EventEnvelope, AppError> {
        // propagate the trace context first, so a business correlation-id
        // riding the current trace wins over a minted one
        let event = apply_current_trace(event);
        if event.header(event_api::X_EVENT_API).is_none() {
            if let Some(entry) = event.to().and_then(event_api::get_event_http_target) {
                let forward = event.set_header(event_api::X_EVENT_API, "request");
                return event_api::event_over_http_with_headers(
                    self,
                    &entry.target,
                    forward,
                    timeout,
                    true,
                    &entry.headers,
                )
                .await;
            }
        }
        self.request_direct(event, timeout).await
    }

    /// The local inbox-based RPC without the declarative Event-over-HTTP hook
    /// — used by the framework's own internal calls (notably the HTTP client
    /// leg of an Event-over-HTTP forward, which must never consult the
    /// declarative registry itself).
    pub(crate) async fn request_direct(
        &self,
        event: EventEnvelope,
        timeout: Duration,
    ) -> Result<EventEnvelope, AppError> {
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
