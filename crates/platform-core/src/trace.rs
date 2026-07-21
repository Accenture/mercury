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

//! Distributed-trace context — Rust port of the Java `TraceInfo` +
//! `LogContext`/`LogContextManager` design (`org.platformlambda.core`).
//!
//! Trace and span IDs follow the **W3C Trace Context / OpenTelemetry** format:
//! a 32-hex trace ID and a 16-hex span ID. Each traced function execution gets
//! its **own span**; the caller's span travels on the envelope's `span_id`
//! field and becomes the callee's `parent_span_id` — producing a causal span
//! tree without any coupling between functions.
//!
//! Java threads the context through a per-worker registry keyed by thread id —
//! deliberately avoiding the ThreadLocal / MDC pattern (an anti-pattern on a
//! virtual-thread runtime). The Rust analog of that per-task anchor is a
//! **tokio `task_local!`**: the route worker scopes the state around the
//! function invocation, so `PostOffice` (propagation, annotations, log
//! context) and the JSON logger (context block) read it from within the same
//! task, and it is torn down when the function returns. Work `tokio::spawn`ed
//! *from inside* a function does not inherit the context — the same boundary
//! Java has for `Mono`/`Flux` completions after the worker returns.

use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;

/// Reserved log-context token names (Java `LogContext.RESERVED_KEYS`): these
/// resolve live per log line and cannot be overridden via
/// `PostOffice::update_context`.
pub const RESERVED_KEYS: [&str; 7] = [
    "cid",
    "traceId",
    "tracePath",
    "spanId",
    "parentSpanId",
    "service",
    "utc",
];

/// Per-execution trace state (Java `TraceInfo` + `LogContext`, combined —
/// one task-local anchor holds both the trace identity and the log context).
#[derive(Clone, Debug)]
pub struct TraceState {
    pub route: String,
    pub trace_id: String,
    pub trace_path: String,
    /// This execution's own span (16-hex, minted at creation — Java parity).
    pub span_id: String,
    /// The caller's span, taken from the incoming envelope's `span_id`.
    pub parent_span_id: Option<String>,
    /// The business correlation id captured from the incoming event —
    /// a separate concern from the trace id.
    pub cid: Option<String>,
    /// ISO-8601 UTC start time (Java `TraceInfo.startTime`).
    pub start_time: String,
    /// Business annotations for the distributed-trace dataset
    /// (`PostOffice::annotate_trace`) — flows to the telemetry sink.
    pub annotations: HashMap<String, serde_json::Value>,
    /// Developer-supplied log-context key-values
    /// (`PostOffice::update_context`) — flows to the application log only.
    pub custom_log_keys: HashMap<String, serde_json::Value>,
    /// True on a zero-traced route: the trace CONTEXT still flows (replies and
    /// nested calls keep the trace id/path — Java propagates them from the
    /// incoming event unconditionally), but this hop emits no telemetry and
    /// mints no span into the chain (Java never calls `startTracing`, so
    /// `touch()` finds no TraceInfo and stamps no span id).
    pub zero_traced: bool,
}

impl TraceState {
    pub fn new(
        route: &str,
        trace_id: &str,
        trace_path: &str,
        parent_span_id: Option<&str>,
        cid: Option<&str>,
    ) -> Self {
        TraceState {
            route: route.to_string(),
            trace_id: trace_id.to_string(),
            trace_path: trace_path.to_string(),
            span_id: new_span_id(),
            parent_span_id: parent_span_id.map(str::to_string),
            cid: cid.map(str::to_string),
            start_time: iso8601_utc_now(),
            annotations: HashMap::new(),
            custom_log_keys: HashMap::new(),
            zero_traced: false,
        }
    }

    /// Resolve a reserved log-context token to its live value
    /// (Java `LogContext.token`). `None` means the key is omitted from the
    /// output (never rendered as null).
    pub fn token(&self, token: &str, log_time: std::time::SystemTime) -> Option<serde_json::Value> {
        match token {
            "cid" => self.cid.clone().map(serde_json::Value::String),
            "traceId" => Some(serde_json::Value::String(self.trace_id.clone())),
            "tracePath" => Some(serde_json::Value::String(self.trace_path.clone())),
            "spanId" => Some(serde_json::Value::String(self.span_id.clone())),
            "parentSpanId" => self.parent_span_id.clone().map(serde_json::Value::String),
            "service" => Some(serde_json::Value::String(self.route.clone())),
            "utc" => Some(serde_json::Value::String(iso8601_utc(log_time))),
            _ => None,
        }
    }
}

tokio::task_local! {
    static TRACE_STATE: RefCell<Option<TraceState>>;
}

/// Run a future inside a trace scope (the worker's trace bracket). Returns the
/// future's output together with the final state — annotations and custom keys
/// added during execution included. A `None` state runs unscoped (non-traced).
pub(crate) async fn run_scoped<F>(
    state: Option<TraceState>,
    future: F,
) -> (F::Output, Option<TraceState>)
where
    F: Future,
{
    if state.is_none() {
        return (future.await, None);
    }
    TRACE_STATE
        .scope(RefCell::new(state), async {
            let output = future.await;
            let state = TRACE_STATE.with(|cell| cell.borrow_mut().take());
            (output, state)
        })
        .await
}

/// Read the current trace state, if this task runs inside a trace bracket.
pub fn with_current<T>(reader: impl FnOnce(&TraceState) -> T) -> Option<T> {
    TRACE_STATE
        .try_with(|cell| cell.borrow().as_ref().map(reader))
        .ok()
        .flatten()
}

/// Mutate the current trace state (annotations / custom log keys), if any.
/// Returns false when there is no active trace (the caller no-ops — Java parity).
pub(crate) fn with_current_mut(mutator: impl FnOnce(&mut TraceState)) -> bool {
    TRACE_STATE
        .try_with(|cell| {
            let mut guard = cell.borrow_mut();
            match guard.as_mut() {
                Some(state) => {
                    mutator(state);
                    true
                }
                None => false,
            }
        })
        .unwrap_or(false)
}

/// Mint a new 32-hex W3C/OpenTelemetry-compatible trace ID.
pub fn new_trace_id() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

/// Mint a new 16-hex W3C/OpenTelemetry-compatible span ID
/// (Java: `String.format("%016x", UUID.randomUUID().getLeastSignificantBits())`).
pub fn new_span_id() -> String {
    format!("{:016x}", uuid::Uuid::new_v4().as_u128() as u64)
}

/// ISO-8601 UTC timestamp with milliseconds for the current time.
pub fn iso8601_utc_now() -> String {
    iso8601_utc(std::time::SystemTime::now())
}

/// ISO-8601 UTC timestamp with milliseconds (no external date dependency —
/// civil-from-days per Howard Hinnant's algorithm).
pub fn iso8601_utc(time: std::time::SystemTime) -> String {
    let duration = time
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs() as i64;
    let millis = duration.subsec_millis();
    let days = secs.div_euclid(86_400);
    let secs_of_day = secs.rem_euclid(86_400);
    let (hh, mm, ss) = (
        secs_of_day / 3600,
        (secs_of_day % 3600) / 60,
        secs_of_day % 60,
    );
    // civil_from_days
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    format!("{year:04}-{month:02}-{day:02}T{hh:02}:{mm:02}:{ss:02}.{millis:03}Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_and_span_ids_are_w3c_shaped() {
        let trace = new_trace_id();
        let span = new_span_id();
        assert_eq!(trace.len(), 32);
        assert!(trace
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()));
        assert_eq!(span.len(), 16);
        assert!(span
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()));
        assert_ne!(new_trace_id(), trace);
    }

    #[test]
    fn iso8601_formats_known_instant() {
        let t = std::time::UNIX_EPOCH + std::time::Duration::from_millis(1_752_620_400_123);
        // 2025-07-15T23:00:00.123Z
        assert_eq!(iso8601_utc(t), "2025-07-15T23:00:00.123Z");
        let epoch = std::time::UNIX_EPOCH;
        assert_eq!(iso8601_utc(epoch), "1970-01-01T00:00:00.000Z");
    }

    #[test]
    fn tokens_resolve_and_absent_keys_are_none() {
        let mut state = TraceState::new("v1.demo", "t".repeat(32).as_str(), "GET /x", None, None);
        state.cid = Some("cid-1".into());
        let now = std::time::SystemTime::now();
        assert_eq!(
            state.token("service", now),
            Some(serde_json::Value::String("v1.demo".into()))
        );
        assert_eq!(
            state.token("cid", now),
            Some(serde_json::Value::String("cid-1".into()))
        );
        assert_eq!(state.token("parentSpanId", now), None); // omitted, not null
        assert_eq!(state.token("unknown", now), None);
        assert!(state.token("utc", now).is_some());
    }
}
