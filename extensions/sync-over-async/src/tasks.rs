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

//! The three composable **facade tasks** of the sync-over-async pattern (Java
//! `SyncPrepareTask`, `SyncAwaitTask`, `SoaReplyTask`) — the generic boilerplate
//! every sync-over-async application needs, shipped with the extension so an
//! application only supplies its own backend (system-of-record) logic:
//!
//! ```text
//!   HTTP POST -> http.flow.adapter -> flow sync-to-async
//!     sync.prepare: begin(cid) [Redis] -> simple.kafka.notification -> request topic -> sync.await (parks)
//!       ... the backend's flow processes the request and publishes the reply ...
//!     response topic -> Kafka Flow Adapter -> flow soa-reply -> soa.reply: deliver(cid) [Redis]
//!       -> the return route wakes sync.await -> HTTP 200 + body
//! ```
//!
//! They are transport-neutral: nothing here names Kafka. The flow wires the
//! publish (`simple.kafka.notification` or any other transport) between
//! `sync.prepare` and `sync.await`, so a publish failure fails the flow before
//! anything waits (fail-fast), and the reply flow bound to the response topic
//! runs `soa.reply`.
//!
//! **Correlation-id sourcing.** The tasks read the module's own [`CID`] key
//! (`cid`), which the flow supplies from `model.cid` — the business correlation
//! id captured at the edge (REST automation reads `http.correlation.id.header`,
//! default `X-Correlation-Id`, and mints one when absent; an inbound flow adapter
//! seeds it from its configured wire header). The Kafka header that carries the
//! id between pods belongs to the transport (`kafka.correlation.id.header`,
//! default also `cid`), so these tasks never reference the transport's name.
//! Sized for user-facing concurrency (250 workers each): a parked `sync.await`
//! costs an async task, nothing more.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use platform_core::{preload, AppError, ComposableFunction, EventEnvelope};

use crate::coordinator::ReturnRouteCoordinator;
use crate::{runtime, CID};

/// First task of a facade flow: registers the return route and serializes the
/// request body into the outbound payload.
pub const SYNC_PREPARE_ROUTE: &str = "sync.prepare";
/// Last task of a facade flow: parks until the response arrives, or 408.
pub const SYNC_AWAIT_ROUTE: &str = "sync.await";
/// The reply-delivery task, bound to the response topic's flow.
pub const SOA_REPLY_ROUTE: &str = "soa.reply";

/// The per-request await budget header (milliseconds), read from the flow's
/// `input.header.x-sync-timeout -> header.x-sync-timeout` mapping.
const TIMEOUT_HEADER: &str = "x-sync-timeout";
const DEFAULT_TIMEOUT_MS: u64 = 10_000;

/// The running coordinator, or a clear 500 when the pod did not enable
/// sync-over-async (Java: a `NullPointerException` from `SyncRuntime.coordinator()`).
fn coordinator() -> Result<Arc<ReturnRouteCoordinator>, AppError> {
    runtime::coordinator().ok_or_else(|| {
        AppError::new(
            500,
            "sync-over-async is not enabled on this pod (sync.over.async.enabled) - the \
             return-route coordinator is not running",
        )
    })
}

/// **`sync.prepare`** — first task of a composable sync-over-async REST facade
/// flow. It registers the cross-pod return route in Redis (`begin`) and
/// serializes the request body into the outbound payload — JSON bytes, keyed
/// by the correlation-id that will track the round trip (returned as the `cid`
/// header for the flow to map onwards). It does **not** publish or wait: the
/// publish is the next task, so a publish failure fails the flow and is
/// rejected to the caller (fail-fast), and the blocking await is the last task,
/// so it runs after the request is on the wire.
///
/// Flow shape (the Java demo's `sync-to-async.yml`):
///
/// ```yaml
/// - input:
///     - 'model.cid -> header.cid'
///     - 'input.body -> *'
///   process: 'sync.prepare'
///   output:
///     - 'result -> model.payload'
/// ```
#[preload(route = "sync.prepare", instances = 250)]
#[derive(Default)]
pub struct SyncPrepare;

#[async_trait]
impl ComposableFunction for SyncPrepare {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        // model.cid is the business correlation-id captured at the edge; the
        // flow maps it here as header.cid - a fresh one only when nothing did
        let cid = headers
            .get(CID)
            .map(|c| c.trim())
            .filter(|c| !c.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| uuid::Uuid::new_v4().simple().to_string());
        // register the return route BEFORE publishing
        coordinator()?.begin(&cid).await?;
        let payload = request_document(&input)?;
        Ok(EventEnvelope::new()
            .set_header(CID, &cid)
            .set_raw_body(rmpv::Value::Binary(payload)))
    }
}

/// The request body as a JSON document: a map or list is rendered, bytes pass
/// through (already a document), an absent body is the empty object.
fn request_document(input: &EventEnvelope) -> Result<Vec<u8>, AppError> {
    match input.body() {
        rmpv::Value::Nil => Ok(b"{}".to_vec()),
        rmpv::Value::Binary(bytes) => Ok(bytes.clone()),
        rmpv::Value::String(text) => Ok(text.as_str().unwrap_or_default().as_bytes().to_vec()),
        _ => {
            let json: serde_json::Value = input.body_as().map_err(|e| {
                AppError::new(
                    400,
                    format!("Unable to serialize the request - {}", e.message()),
                )
            })?;
            serde_json::to_vec(&json)
                .map_err(|e| AppError::new(400, format!("Unable to serialize the request - {e}")))
        }
    }
}

/// **`sync.await`** — last task of a composable sync-over-async REST facade
/// flow. It parks until the asynchronous backend's response arrives via the
/// response topic and the Redis return route, then returns the response body
/// (HTTP 200). On timeout it fails with **408**, which the flow's exception
/// handler maps to an HTTP status.
///
/// The correlation-id was allocated and registered by `sync.prepare`; this
/// task awaits by correlation-id (the flow maps `model.cid -> cid` into its
/// body) since the coordinator holds the pending entry rather than the flow
/// passing a handle between tasks. The per-request budget is the
/// `x-sync-timeout` header in milliseconds (default 10 000; a non-numeric value
/// falls back to the default).
///
/// ```yaml
/// - input:
///     - 'model.cid -> cid'
///     - 'input.header.x-sync-timeout -> header.x-sync-timeout'
///   process: 'sync.await'
///   output:
///     - 'result -> output.body'
/// ```
#[preload(route = "sync.await", instances = 250)]
#[derive(Default)]
pub struct SyncAwait;

#[async_trait]
impl ComposableFunction for SyncAwait {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let cid = body_text(&input, CID).ok_or_else(|| {
            AppError::new(
                400,
                format!("Missing '{CID}' - map model.cid -> cid into sync.await"),
            )
        })?;
        let timeout_ms = headers
            .get(TIMEOUT_HEADER)
            .and_then(|v| v.trim().parse::<u64>().ok())
            .unwrap_or(DEFAULT_TIMEOUT_MS);
        match coordinator()?.await_response(&cid, timeout_ms).await {
            Ok(response_json) => {
                let value: serde_json::Value =
                    serde_json::from_str(&response_json).map_err(|e| {
                        AppError::new(
                            500,
                            format!("The response for {cid} is not a JSON document - {e}"),
                        )
                    })?;
                EventEnvelope::new().set_body(value)
            }
            Err(e) if e.status() == 408 => Err(AppError::new(
                408,
                format!("Timed out awaiting response for {cid}"),
            )),
            Err(e) => Err(e),
        }
    }
}

/// **`soa.reply`** — the reusable reply-delivery task. Wire it as the task of
/// a flow bound (via the Kafka flow adapter) to the response topic: it hands
/// the asynchronous response to the return-route coordinator, which completes
/// the awaiting REST request — cross-pod via the Redis return route, so the pod
/// that consumed the reply need not be the one that originated the request.
///
/// The reply flow's input mapping supplies the correlation-id under the
/// module's own `cid` key (`model.cid -> header.cid`; the flow adapter seeds
/// `model.cid` from its configured wire header), so this task never reads the
/// transport's header name. The response payload is the body — bytes (the raw
/// path) or, on a `schema.enabled` binding, the decoded map, rendered to JSON.
/// Returns `{cid, delivered}`; `delivered: false` marks an orphan (the route
/// expired, or an unknown correlation-id).
///
/// ```yaml
/// - input:
///     - 'input.body -> *'
///     - 'model.cid -> header.cid'
///   process: 'soa.reply'
///   output:
///     - 'result -> output.body'
/// ```
#[preload(route = "soa.reply", instances = 250)]
#[derive(Default)]
pub struct SoaReply;

#[async_trait]
impl ComposableFunction for SoaReply {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let cid = headers
            .get(CID)
            .map(|c| c.trim())
            .filter(|c| !c.is_empty())
            .ok_or_else(|| {
                AppError::new(
                    400,
                    format!("Missing '{CID}' header - map model.cid -> header.cid into soa.reply"),
                )
            })?;
        deliver(cid, &input).await
    }
}

/// Hand a response body to the coordinator for `cid` (shared by `soa.reply`
/// and an application's schema-typed variants of it).
pub async fn deliver(cid: &str, input: &EventEnvelope) -> Result<EventEnvelope, AppError> {
    let payload = response_text(input)?;
    let delivered = coordinator()?.deliver(cid, &payload).await?;
    EventEnvelope::new().set_body(serde_json::json!({ CID: cid, "delivered": delivered }))
}

/// The response as the JSON text the awaiting side parses: bytes and text pass
/// through, a decoded map or list is rendered.
fn response_text(input: &EventEnvelope) -> Result<String, AppError> {
    match input.body() {
        rmpv::Value::Binary(bytes) => Ok(String::from_utf8_lossy(bytes).to_string()),
        rmpv::Value::String(text) => Ok(text.as_str().unwrap_or_default().to_string()),
        rmpv::Value::Nil => Ok("null".to_string()),
        _ => {
            let json: serde_json::Value = input.body_as().map_err(|e| {
                AppError::new(
                    400,
                    format!("Unable to render the response - {}", e.message()),
                )
            })?;
            Ok(json.to_string())
        }
    }
}

/// A top-level text value of a map body (the flow's `model.cid -> cid`).
fn body_text(input: &EventEnvelope, key: &str) -> Option<String> {
    let rmpv::Value::Map(entries) = input.body() else {
        return None;
    };
    entries
        .iter()
        .find(|(k, _)| k.as_str() == Some(key))
        .and_then(|(_, v)| match v {
            rmpv::Value::String(s) => s.as_str().map(str::to_string),
            rmpv::Value::Nil => None,
            other => Some(other.to_string()),
        })
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_documents_render_maps_and_pass_bytes_through() {
        let map = EventEnvelope::new()
            .set_body(serde_json::json!({"action": "create"}))
            .expect("body");
        assert_eq!(
            b"{\"action\":\"create\"}".to_vec(),
            request_document(&map).unwrap()
        );
        let bytes = EventEnvelope::new().set_raw_body(rmpv::Value::Binary(b"{\"x\":1}".to_vec()));
        assert_eq!(b"{\"x\":1}".to_vec(), request_document(&bytes).unwrap());
        assert_eq!(
            b"{}".to_vec(),
            request_document(&EventEnvelope::new()).unwrap()
        );
    }

    #[test]
    fn response_text_and_body_cid() {
        let decoded = EventEnvelope::new()
            .set_body(serde_json::json!({"cid": "c-1", "status": "processed"}))
            .expect("body");
        assert_eq!(
            "{\"cid\":\"c-1\",\"status\":\"processed\"}",
            response_text(&decoded).unwrap()
        );
        assert_eq!(Some("c-1".to_string()), body_text(&decoded, CID));
        assert_eq!(None, body_text(&decoded, "missing"));
        let raw = EventEnvelope::new().set_raw_body(rmpv::Value::Binary(b"{\"a\":1}".to_vec()));
        assert_eq!("{\"a\":1}", response_text(&raw).unwrap());
        assert_eq!(None, body_text(&raw, CID), "a bytes body has no cid key");
    }

    /// Without a coordinator every task fails with the configuration message,
    /// never a panic (the Java analog is a NullPointerException).
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn tasks_name_the_missing_coordinator() {
        runtime::shutdown();
        let mut headers = HashMap::new();
        headers.insert(CID.to_string(), "c-9".to_string());
        let prepare = SyncPrepare
            .handle_event(headers.clone(), EventEnvelope::new(), 1)
            .await
            .expect_err("no coordinator");
        assert_eq!(500, prepare.status());
        assert!(prepare.message().contains("sync.over.async.enabled"));
        let reply = SoaReply
            .handle_event(
                headers,
                EventEnvelope::new().set_raw_body(rmpv::Value::Binary(b"{}".to_vec())),
                1,
            )
            .await
            .expect_err("no coordinator");
        assert_eq!(500, reply.status());
        let await_task = SyncAwait
            .handle_event(
                HashMap::new(),
                EventEnvelope::new()
                    .set_body(serde_json::json!({"cid": "c-9"}))
                    .expect("body"),
                1,
            )
            .await
            .expect_err("no coordinator");
        assert_eq!(500, await_task.status());
        let missing_cid = SyncAwait
            .handle_event(HashMap::new(), EventEnvelope::new(), 1)
            .await
            .expect_err("no cid");
        assert_eq!(400, missing_cid.status());
    }
}
