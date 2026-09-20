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

//! **The distributed-cache worked example** — Rust twin of the Java
//! `examples/distributed-cache-example`: the same profile GET/POST/DELETE CRUD
//! three times, one route family per layer, all backed by ONE shared Redis
//! cache (`v1.cache.redis`, the `mercury-distributed-cache` crate):
//!
//! - **Layer 1 (Platform Core / PostOffice)** — `/api/l1/profile/{profile_id}`:
//!   a single function (`v1.profile.l1`) calls the cache with the PostOffice
//!   RPC API in code. The orchestration IS the function body.
//! - **Layer 2 (Event Script)** — `/api/l2/profile/{profile_id}`: ONE flow
//!   (`resources/flows/l2-profile.yml`, byte-identical to the Java example's)
//!   composes the cache task with the encode/decode helpers declaratively.
//! - **Layer 3 (Knowledge Graph)** — `/api/graph/profile-cache`: ONE graph
//!   (`resources/graph/profile-cache.json`, byte-identical) drives the same
//!   route from `graph.task` nodes; the action rides in the payload, and the
//!   standard graph endpoint serves it — a Layer 3 app needs no endpoint of
//!   its own.
//!
//! **The shared value is a plain MsgPack-packed map.** A profile written
//! through one layer reads back through the other two — and through the Java
//! example, on either side: the key is the raw profile id under the app
//! namespace `cache-demo:`, and the value is the profile map packed with
//! standard MsgPack (no envelope wrapper, no type tags), which is exactly what
//! the Java `MsgPack.packMapOrList` writes and reads. That makes this example
//! the cross-engine interop harness of the cache.
//!
//! Run a Redis first (the Java repository's `helpers/redis-standalone`, or any
//! server), then `cargo run -p distributed-cache-example`; see README.md.

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use platform_core::automation::AsyncHttpRequest;
use platform_core::{
    main_application, preload, AppError, ComposableFunction, EntryPoint, EventEnvelope, Platform,
    PostOffice,
};
use rmpv::Value;

const CACHE: &str = distributed_cache::CACHE_ROUTE;
const ACTION: &str = "action";
const KEY: &str = "key";
const LAYER: &str = "layer";
const TIMEOUT: Duration = Duration::from_secs(5);

/// Pack a profile map to the opaque bytes the cache stores — standard MsgPack,
/// the Java `MsgPack.packMapOrList` form: no envelope wrapper, no type tags,
/// so the same bytes are read by the Java example and by every layer here.
fn pack(profile: &Value) -> Result<Vec<u8>, AppError> {
    let mut out = Vec::new();
    rmpv::encode::write_value(&mut out, profile)
        .map_err(|e| AppError::new(500, format!("Unable to pack the profile - {e}")))?;
    Ok(out)
}

/// The inverse of [`pack`] (Java `MsgPack.unpackMapOrList`).
fn unpack(bytes: &[u8]) -> Result<Value, AppError> {
    rmpv::decode::read_value(&mut &bytes[..])
        .map_err(|e| AppError::new(500, format!("Unable to unpack the profile - {e}")))
}

/// A cache reply body as the stored bytes, or `None` on a miss (a null body).
fn cached_bytes(reply: &EventEnvelope) -> Option<&[u8]> {
    match reply.body() {
        Value::Binary(bytes) if !bytes.is_empty() => Some(bytes),
        _ => None,
    }
}

/// A cache reply, or the cache's failure as this function's own error. An
/// `Err` from `v1.cache.redis` arrives as a reply with the error status and
/// the message as its string body (the Java engine shapes it the same way),
/// so it must never be read as "nothing cached": a cache outage is a 5xx,
/// not a miss — found in the live Java ⇄ Rust interop drive (2026-09-19),
/// where Layer 1 answered 404 *Profile not found* while Redis was down.
fn checked(reply: EventEnvelope) -> Result<EventEnvelope, AppError> {
    if reply.has_error() {
        let message = reply
            .body()
            .as_str()
            .filter(|text| !text.is_empty())
            .unwrap_or("Cache request failed")
            .to_string();
        return Err(AppError::new(reply.status(), message));
    }
    Ok(reply)
}

// ---------------------------------------------------------------------------
// Layer 1 — the whole CRUD in one function, driving the cache in code
// ---------------------------------------------------------------------------

/// **Layer 1 (Platform Core / PostOffice).** Bound directly to
/// `/api/l1/profile/{profile_id}` in `rest.yaml` (no flow, no graph): GET
/// reads the value (a miss is HTTP 404), POST packs the JSON body and stores it
/// (201), DELETE evicts the key and reports whether anything was removed. The
/// cache RPC inherits this request's trace automatically — the trace bracket
/// is task-local, so the span chain stays continuous across the
/// L1 → `v1.cache.redis` hop — and `update_context` tags the app-context log
/// with the layer (Java `ProfileCacheL1`).
#[preload(route = "v1.profile.l1", instances = 20)]
struct ProfileCacheL1;

#[async_trait]
impl ComposableFunction for ProfileCacheL1 {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request = AsyncHttpRequest::from_value(input.body());
        let method = request.method().to_ascii_uppercase();
        let id = request
            .path_parameter("profile_id")
            .map(str::trim)
            .filter(|id| !id.is_empty())
            .ok_or_else(|| AppError::new(400, "Missing profile_id"))?
            .to_string();
        let po = PostOffice::new(&Platform::get_instance());
        // app-context logging: tag this request's log with its layer
        let _ = po.update_context(LAYER, "1");
        match method.as_str() {
            "GET" => {
                let reply = checked(
                    po.request(
                        EventEnvelope::new()
                            .set_to(CACHE)
                            .set_header(ACTION, "GET")
                            .set_header(KEY, &id),
                        TIMEOUT,
                    )
                    .await?,
                )?;
                match cached_bytes(&reply) {
                    Some(bytes) => Ok(EventEnvelope::new().set_raw_body(unpack(bytes)?)),
                    None => Err(AppError::new(404, "Profile not found")),
                }
            }
            "POST" => {
                // the value must be a JSON object: a map is the only shape the
                // shared MsgPack profile format carries
                let profile = request.body();
                if !matches!(profile, Value::Map(_)) {
                    return Err(AppError::new(400, "Profile must be a JSON object"));
                }
                checked(
                    po.request(
                        EventEnvelope::new()
                            .set_to(CACHE)
                            .set_header(ACTION, "PUT")
                            .set_header(KEY, &id)
                            .set_raw_body(Value::Binary(pack(profile)?)),
                        TIMEOUT,
                    )
                    .await?,
                )?;
                EventEnvelope::new()
                    .set_status(201)
                    .set_body(serde_json::json!({"id": id, "layer": 1, "status": "stored"}))
            }
            "DELETE" => {
                let reply = checked(
                    po.request(
                        EventEnvelope::new()
                            .set_to(CACHE)
                            .set_header(ACTION, "DELETE")
                            .set_header(KEY, &id),
                        TIMEOUT,
                    )
                    .await?,
                )?;
                let removed = reply.body().as_i64().unwrap_or(0);
                EventEnvelope::new()
                    .set_body(serde_json::json!({"id": id, "layer": 1, "deleted": removed > 0}))
            }
            other => Err(AppError::new(405, format!("Method not allowed: {other}"))),
        }
    }
}

// ---------------------------------------------------------------------------
// Layer 2 / Layer 3 helpers — tiny composable functions the flow and the
// graph wire declaratively
// ---------------------------------------------------------------------------

/// Serialise a profile map into the opaque bytes the cache stores (Java
/// `ProfileEncoder`): the Layer 2 flow and the Layer 3 graph compose this with
/// the `v1.cache.redis` PUT; Layer 1 does the same inline.
#[preload(route = "v1.profile.encode", instances = 10)]
struct ProfileEncoder;

#[async_trait]
impl ComposableFunction for ProfileEncoder {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if !matches!(input.body(), Value::Map(_)) {
            return Err(AppError::new(400, "Profile must be a JSON object"));
        }
        Ok(EventEnvelope::new().set_raw_body(Value::Binary(pack(input.body())?)))
    }
}

/// Restore a profile map from the cache's opaque bytes — the inverse of
/// [`ProfileEncoder`] (Java `ProfileDecoder`). The input is the value from a
/// `v1.cache.redis` GET: the stored bytes, or null on a miss, which raises
/// HTTP 404 *Profile not found* for the flow's / graph's exception handler to
/// render.
#[preload(route = "v1.profile.decode", instances = 10)]
struct ProfileDecoder;

#[async_trait]
impl ComposableFunction for ProfileDecoder {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        match cached_bytes(&input) {
            Some(bytes) => Ok(EventEnvelope::new().set_raw_body(unpack(bytes)?)),
            None => Err(AppError::new(404, "Profile not found")),
        }
    }
}

/// **Layer 2 — the method-to-action mapper** (Java `MethodActionMapper`): turns
/// the HTTP method into a cache action so the single `l2-profile` flow routes
/// GET / POST / DELETE the way the Layer 3 graph routes on its payload
/// `action`. Returns the two values the `execution: decision` task consumes:
/// `action` (get / save / delete, for the flow state and the trace) and
/// `decision` (the 1-based branch: GET→1, POST→2, DELETE→3).
#[preload(route = "v1.http.method.action", instances = 10)]
struct MethodActionMapper;

#[async_trait]
impl ComposableFunction for MethodActionMapper {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        let method = body["method"].as_str().unwrap_or("").to_ascii_uppercase();
        let (action, decision) = match method.as_str() {
            "GET" => ("get", 1),
            "POST" => ("save", 2),
            "DELETE" => ("delete", 3),
            other => return Err(AppError::new(405, format!("Method not allowed: {other}"))),
        };
        let po = PostOffice::new(&Platform::get_instance());
        // app-context logging: this request's layer and the derived action
        let _ = po.update_context(LAYER, "2");
        let _ = po.update_context(ACTION, action);
        EventEnvelope::new().set_body(serde_json::json!({"action": action, "decision": decision}))
    }
}

/// Exception handler for the Layer 2 flow and the Layer 3 graph (Java
/// `ProfileExceptionHandler`): the flow / graph maps `error.code`,
/// `error.message` and `error.stack` into this function's input; it renders
/// the standard error body (`type=error`, `status`, `message`) which the
/// caller receives with the failing HTTP status — a cache miss surfaced by
/// `v1.profile.decode` as HTTP 404 *Profile not found* is rendered here.
#[preload(route = "v1.profile.exception", instances = 10)]
struct ProfileExceptionHandler;

#[async_trait]
impl ComposableFunction for ProfileExceptionHandler {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let po = PostOffice::new(&Platform::get_instance());
        let _ = po.update_context(LAYER, "flow");
        let body: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        match (body.get("status"), body.get("message")) {
            (Some(status), Some(message)) => {
                log::info!("Profile flow exception - status={status} message={message}");
                EventEnvelope::new().set_body(serde_json::json!({
                    "type": "error",
                    "status": status,
                    "message": message,
                }))
            }
            _ => EventEnvelope::new().set_body(serde_json::json!({})),
        }
    }
}

// ---------------------------------------------------------------------------
// entry point
// ---------------------------------------------------------------------------

/// Entry point (Java `MainApp`): the three layers are wired in `rest.yaml`,
/// `flows.yaml` and `graphs.yaml`; nothing to start here beyond the log line.
#[main_application]
struct MainApp;

#[async_trait]
impl EntryPoint for MainApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!(
            "distributed-cache-example started - profile CRUD over {CACHE} at Layers 1, 2 and 3 ({} graph(s) compiled)",
            knowledge_graph::graphs::get_all_graphs().len()
        );
        Ok(())
    }
}

platform_core::auto_start_main!();
