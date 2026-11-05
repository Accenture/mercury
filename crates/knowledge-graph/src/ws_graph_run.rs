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

//! A watched graph run over a WebSocket session (RFC-0011 section 8.2) — the Rust port of the
//! Java `GraphRunSocket` (`/ws/graph-run/{graph_id}`) and `GraphRunReply`
//! (`graph.run.socket.reply`). The session's first text frame is the run's input, a JSON object
//! with an optional `body` and `header` (an `X-Correlation-Id` header names the business
//! correlation id, as on a REST request). The run sends its progress events, then its result, as
//! text frames to the session, and the session closes after the result. One session runs one
//! graph; a later frame is ignored.
//!
//! Off by default (`graph.websocket.enabled=true` turns it on): a WebSocket path is outside the
//! REST automation, so its authentication does not apply here. An application that turns this
//! entry on puts it behind a gateway that authenticates the upgrade request.
//! `graph.websocket.ttl` (default 30m) is each run's TTL.
//!
//! The Java handler keeps its sessions in a map keyed by the session's route; the Rust websocket
//! server creates one function object per connection, so the session lives in the object.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use async_trait::async_trait;
use event_script::conversions::{display, from_json};
use platform_core::{
    preload, websocket_service, AppConfigReader, AppError, ComposableFunction, EventEnvelope,
    Platform, PostOffice,
};
use rmpv::Value;

use crate::progress::return_channel::close_session;
use crate::progress::ReturnChannel;

/// The switch of the WebSocket entry (Java `GraphRunSocket.ENABLED`).
pub const ENABLED: &str = "graph.websocket.enabled";
/// The TTL of a run started from the WebSocket entry (Java `GraphRunSocket.TTL_CONFIG`).
pub const TTL_CONFIG: &str = "graph.websocket.ttl";
/// Where the flow of a run started from a WebSocket session replies (Java `GraphRunReply.ROUTE`).
pub const REPLY_ROUTE: &str = "graph.run.socket.reply";
const GRAPH_EXECUTOR: &str = "graph-executor";
const X_CORRELATION_ID: &str = "x-correlation-id";

#[derive(Clone)]
struct Session {
    tx_path: String,
    graph_id: String,
}

/// Java `GraphRunSocket` (`@OptionalService("graph.websocket.enabled")
/// @WebSocketService("graph-run")`) — one object per connection.
#[websocket_service("graph-run")]
#[optional_service("graph.websocket.enabled")]
#[derive(Default)]
pub struct GraphRunSocket {
    session: Mutex<Option<Session>>,
    started: AtomicBool,
}

impl GraphRunSocket {
    fn session(&self) -> Option<Session> {
        self.session.lock().expect("graph run session").clone()
    }
}

#[async_trait]
impl ComposableFunction for GraphRunSocket {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let get = |key: &str| headers.get(key).cloned().unwrap_or_default();
        let route = get("route");
        match get("type").as_str() {
            "open" => {
                let graph_id = get("token");
                *self.session.lock().expect("graph run session") = Some(Session {
                    tx_path: get("tx_path"),
                    graph_id: graph_id.clone(),
                });
                log::info!("Session {route} opened for graph {graph_id}");
            }
            "string" => match self.session() {
                Some(session)
                    if self
                        .started
                        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                        .is_ok() =>
                {
                    let text = match input.body() {
                        Value::String(text) => text.as_str().unwrap_or_default().to_string(),
                        other => display(other),
                    };
                    start(&session, &text).await;
                }
                _ => log::warn!("Session {route} ignores a frame - one session runs one graph"),
            },
            "bytes" => {
                if let Some(session) = self.session() {
                    refuse(
                        &session.tx_path,
                        400,
                        "The run's input is a JSON text frame",
                    )
                    .await;
                }
            }
            "close" => {
                *self.session.lock().expect("graph run session") = None;
            }
            other => log::warn!("Session {route} ignores an event of type {other}"),
        }
        EventEnvelope::new().set_body("ok")
    }
}

/// Start the session's run: the graph-executor flow, with the session named for the run before
/// it exists, so the run picks its channel up by the flow's correlation id.
async fn start(session: &Session, text: &str) {
    let frame: serde_json::Value = match serde_json::from_str(text) {
        Ok(frame) => frame,
        Err(e) => {
            refuse(
                &session.tx_path,
                400,
                &format!("The run's input is not JSON - {e}"),
            )
            .await;
            return;
        }
    };
    let Some(frame) = frame.as_object() else {
        refuse(
            &session.tx_path,
            400,
            "The run's input is a JSON object with an optional body and header",
        )
        .await;
        return;
    };
    let object_or_empty = |key: &str| {
        frame
            .get(key)
            .filter(|v| v.is_object())
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}))
    };
    let header = object_or_empty("header");
    let body = object_or_empty("body");
    let business_cid = business_cid(&header);
    let flow_cid = uuid::Uuid::new_v4().simple().to_string();
    let ttl = AppConfigReader::get_instance().get_property_or(TTL_CONFIG, "30m");
    let ttl_ms = event_script::util::duration_in_seconds(&ttl).saturating_mul(1000);
    let dataset = serde_json::json!({
        "body": body,
        "header": header,
        "path_parameter": {"graph_id": session.graph_id},
        "ttl": ttl_ms,
    });
    // the run picks its channel up by the flow's correlation id; the flow's own reply goes to the
    // reply route
    ReturnChannel::watch_from_socket(&flow_cid, &session.tx_path);
    let launch = EventEnvelope::new()
        .set_to(event_script::manager::SERVICE_NAME)
        .set_header("flow_id", GRAPH_EXECUTOR)
        .set_header(
            event_script::manager::BUSINESS_CORRELATION_ID,
            &business_cid,
        )
        .set_reply_to(REPLY_ROUTE)
        .set_correlation_id(&flow_cid)
        .set_trace(
            &platform_core::trace::new_trace_id(),
            &format!("WS /ws/graph-run/{}", session.graph_id),
        )
        .set_raw_body(from_json(&dataset));
    let platform = Platform::get_instance();
    match PostOffice::new(&platform).send(launch).await {
        Ok(()) => log::info!(
            "Session started graph {}, cid {business_cid}",
            session.graph_id
        ),
        Err(e) => {
            ReturnChannel::take_socket(&flow_cid);
            refuse(
                &session.tx_path,
                500,
                &format!("The run did not start - {}", e.message()),
            )
            .await;
        }
    }
}

/// The session's `X-Correlation-Id`, matched without regard to case, or a new one.
fn business_cid(header: &serde_json::Value) -> String {
    if let Some(entries) = header.as_object() {
        for (key, value) in entries {
            if !key.eq_ignore_ascii_case(X_CORRELATION_ID) || value.is_null() {
                continue;
            }
            let text = match value {
                serde_json::Value::String(text) => text.clone(),
                other => other.to_string(),
            };
            if !text.trim().is_empty() {
                return crate::common::java_trim(&text).to_string();
            }
        }
    }
    uuid::Uuid::new_v4().simple().to_string()
}

/// End a session whose run cannot start, with an error event in the shape of a run's terminal
/// event.
async fn refuse(tx_path: &str, status: i32, message: &str) {
    let platform = Platform::get_instance();
    let error = EventEnvelope::new()
        .set_to(tx_path)
        .set_raw_body(Value::Map(vec![
            (Value::from("type"), Value::from("error")),
            (Value::from("status"), Value::from(status)),
            (Value::from("message"), Value::from(message)),
        ]));
    let _ = PostOffice::new(&platform).send_untraced(error).await;
    close_session(tx_path).await;
}

/// Java `GraphRunReply` (`graph.run.socket.reply`): where the flow of a run started from a
/// WebSocket session replies. The run's channel has already sent the result to the session and
/// closed it. Therefore, the reply matters only for a run whose graph never started - a flow that
/// failed before the graph executor ran - which still ends its session with the error.
#[preload(route = "graph.run.socket.reply", instances = 20)]
#[zero_tracing]
#[optional_service("graph.websocket.enabled")]
pub struct GraphRunReply;

#[async_trait]
impl ComposableFunction for GraphRunReply {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if let Some(tx_path) = input.correlation_id().and_then(ReturnChannel::take_socket) {
            let status = if input.status() >= 400 {
                input.status()
            } else {
                500
            };
            refuse(
                &tx_path,
                status,
                &format!(
                    "The run ended before the graph started - {}",
                    display(input.body())
                ),
            )
            .await;
        }
        Ok(EventEnvelope::new())
    }
}
