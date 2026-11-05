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

//! The way back to the caller of one graph run (RFC-0011 section 8.4; Java `ReturnChannel`):
//! progress events while the run goes on, then one terminal event, `result` or `error`. The entry
//! decides the transport, and the channel frames each event for it:
//!
//! - a REST request with `stream: true` - one server-sent event per event on the request's reply
//!   lane, the stream opened by the first event, so a run that sends none answers with its
//!   single-shot response;
//! - a WebSocket session - one text frame per event on the session's outbound route, the session
//!   closed after the terminal event;
//! - any other entry - one event to the route its `x-return-path` names, when the application
//!   declares it in `return.path.routes`.
//!
//! A node that carries `progress` gets a relay route of this channel, private to the application,
//! and the engine passes that route to the node's function (or to a synchronous subgraph) as
//! `x-return-path`. A relay names the sender's graph and node when the sender does not, so a
//! function such as `llm.run` only sends to the route it is given. The engine never waits on a
//! send, and a failing return route never fails the run. Progress events are sent untraced.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use event_script::conversions::display;
use platform_core::{EventEnvelope, EventStreamWriter, Platform, PostOffice};
use rmpv::Value;

use super::relay::ProgressRelay;

pub const X_RETURN_PATH: &str = "x-return-path";
pub const PROGRESS: &str = "progress";
/// The header of a drain request to a relay (Java `ReturnChannel.DRAIN`).
pub(crate) const DRAIN: &str = "x-drain";
/// The prefix of a relay route: `graph.progress.{flow instance id}.{n}`.
pub const RELAY_PREFIX: &str = "graph.progress.";
const RESULT: &str = "result";
const ERROR: &str = "error";
const TYPE: &str = "type";
const EVENT: &str = "event";
const SEQ: &str = "seq";
const TIME: &str = "time";
const CID: &str = "cid";
const GRAPH: &str = "graph";
const NODE: &str = "node";
const MESSAGE: &str = "message";
const STATUS: &str = "status";
const BODY: &str = "body";
const NODE_START: &str = "node.start";
const DRAIN_MS: u64 = 3000;
const TEXT_EVENT_STREAM: &str = "text/event-stream";
/// Divergence from Java (maintainer ruling): the Rust HTTP edge times a stream's idle allowance
/// exactly, where the Java edge checks every 10 seconds. A stream the engine opens therefore
/// declares its own allowance, the flow's TTL plus this margin, so the engine's own terminal
/// event - an `error` when the flow times out after the run streamed progress - and the `done`
/// event always reach the client before the edge times the stream out. The margin covers one
/// drain of the relays (`DRAIN_MS`) with room to spare.
const STREAM_IDLE_MARGIN_SECONDS: u64 = 5;

/// The relays of the runs that are still going: a synchronous subgraph returns its progress
/// through the parent's relay, which no application declares.
fn live_relays() -> &'static Mutex<HashSet<String>> {
    static LIVE: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    LIVE.get_or_init(|| Mutex::new(HashSet::new()))
}

/// The WebSocket sessions named for runs that have not started yet, by the flow's correlation id.
fn sockets() -> &'static Mutex<HashMap<String, String>> {
    static SOCKETS: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
    SOCKETS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Stream,
    Event,
    Socket,
}

/// The channel's mutable state, held under an async lock because a stream write is a send.
struct ChannelState {
    writer: Option<EventStreamWriter>,
    streaming: bool,
    closed: bool,
    unavailable: bool,
    /// node alias -> relay route, in the order the relays were registered
    relays: Vec<(String, String)>,
}

/// The way back to the caller of one graph run (Java `ReturnChannel`).
pub struct ReturnChannel {
    kind: Kind,
    target: String,
    instance_id: String,
    graph_id: String,
    stream_idle_seconds: u64,
    sequence: AtomicI64,
    business_cid: Mutex<Option<String>>,
    state: tokio::sync::Mutex<ChannelState>,
}

impl ReturnChannel {
    fn new(
        kind: Kind,
        target: &str,
        request_id: Option<&str>,
        instance_id: &str,
        graph_id: &str,
        stream_idle_seconds: u64,
    ) -> Arc<Self> {
        let writer = match kind {
            Kind::Stream => {
                EventStreamWriter::new(&Platform::get_instance(), target, request_id).ok()
            }
            _ => None,
        };
        Arc::new(ReturnChannel {
            kind,
            target: target.to_string(),
            instance_id: instance_id.to_string(),
            graph_id: graph_id.to_string(),
            stream_idle_seconds,
            sequence: AtomicI64::new(0),
            business_cid: Mutex::new(None),
            state: tokio::sync::Mutex::new(ChannelState {
                writer,
                streaming: false,
                closed: false,
                unavailable: false,
                relays: Vec::new(),
            }),
        })
    }

    /// A REST request's reply lane (`stream: true`): `request_id` is the request's correlation
    /// id, which the lane matches its response context by, and `flow_ttl_ms` the run's flow TTL,
    /// from which the stream's idle allowance is derived (see `STREAM_IDLE_MARGIN_SECONDS`).
    pub fn stream(
        lane: &str,
        request_id: &str,
        instance_id: &str,
        graph_id: &str,
        flow_ttl_ms: u64,
    ) -> Arc<Self> {
        let idle = flow_ttl_ms.div_ceil(1000) + STREAM_IDLE_MARGIN_SECONDS;
        Self::new(
            Kind::Stream,
            lane,
            Some(request_id),
            instance_id,
            graph_id,
            idle,
        )
    }

    /// A route the caller named and the application declares, or the relay route of a parent run.
    pub fn event(route: &str, instance_id: &str, graph_id: &str) -> Arc<Self> {
        Self::new(Kind::Event, route, None, instance_id, graph_id, 0)
    }

    /// A WebSocket session's outbound route.
    pub fn socket(tx_path: &str, instance_id: &str, graph_id: &str) -> Arc<Self> {
        Self::new(Kind::Socket, tx_path, None, instance_id, graph_id, 0)
    }

    /// The WebSocket entry names its session for the run it starts, before the run exists.
    pub fn watch_from_socket(flow_correlation_id: &str, tx_path: &str) {
        sockets()
            .lock()
            .expect("socket registry")
            .insert(flow_correlation_id.to_string(), tx_path.to_string());
    }

    /// Take the session the WebSocket entry named for a run: None for a run no session watches.
    pub fn take_socket(flow_correlation_id: &str) -> Option<String> {
        sockets()
            .lock()
            .expect("socket registry")
            .remove(flow_correlation_id)
    }

    /// Whether a route is the relay of a run that is still going: a synchronous subgraph returns
    /// its progress through the parent's relay, which no application declares.
    pub fn is_live_relay(route: &str) -> bool {
        live_relays().lock().expect("live relays").contains(route)
    }

    /// The business correlation id the events carry as `cid`.
    pub fn set_cid(&self, cid: &str) {
        *self.business_cid.lock().expect("business cid") = Some(cid.to_string());
    }

    fn cid(&self) -> Option<String> {
        self.business_cid.lock().expect("business cid").clone()
    }

    /// The engine's event for a node that carries a `progress` property.
    pub async fn node_start(&self, node: &str, message: &str) {
        let mut state = self.state.lock().await;
        if state.closed {
            return;
        }
        let mut event = self.engine_event(PROGRESS);
        event.push(entry(EVENT, Value::from(NODE_START)));
        event.push(entry(NODE, Value::from(node)));
        event.push(entry(MESSAGE, Value::from(message)));
        self.deliver(&mut state, Value::Map(event)).await;
    }

    /// A sender's progress event, through the relay of the node that called it. Only a progress
    /// event passes: a synchronous subgraph's own terminal event is not the end of this run.
    pub(crate) async fn relay(&self, node: &str, event: &Value) {
        let Value::Map(entries) = event else {
            return;
        };
        let is_progress = entries.iter().any(|(k, v)| {
            k.as_str() == Some(TYPE)
                && matches!(v, Value::String(s) if s.as_str() == Some(PROGRESS))
        });
        if !is_progress {
            return;
        }
        let mut copy: Vec<(Value, Value)> = entries
            .iter()
            .map(|(k, v)| (Value::from(display(k)), v.clone()))
            .collect();
        put_if_absent(&mut copy, GRAPH, Value::from(self.graph_id.as_str()));
        put_if_absent(&mut copy, NODE, Value::from(node));
        if let Some(cid) = self.cid() {
            put_if_absent(&mut copy, CID, Value::from(cid));
        }
        let mut state = self.state.lock().await;
        if state.closed {
            return;
        }
        self.deliver(&mut state, Value::Map(copy)).await;
    }

    async fn deliver(&self, state: &mut ChannelState, event: Value) {
        if self.kind == Kind::Stream {
            let name = event_type(&event);
            let idle = self.stream_idle_seconds;
            let streaming = state.streaming;
            if let Some(writer) = state.writer.as_mut() {
                if !streaming {
                    writer.first_with_ttl(200, TEXT_EVENT_STREAM, idle);
                }
                if let Err(e) = writer.write_named(&name, event).await {
                    log::debug!(
                        "Progress of graph {} not streamed - {}",
                        self.graph_id,
                        e.message()
                    );
                }
            }
            state.streaming = true;
        } else {
            self.send(state, event).await;
        }
    }

    /// The run's terminal event, from the response the run returns: `error` for a status of 400
    /// or more, otherwise `result` (a suspension's advice included). A stream the run never opened
    /// sends nothing, so the single-shot response answers the request.
    pub async fn finish(&self, response: &EventEnvelope) {
        self.drain_relays().await;
        self.finish_after_drain(response).await;
    }

    async fn finish_after_drain(&self, response: &EventEnvelope) {
        let mut state = self.state.lock().await;
        if state.closed {
            return;
        }
        state.closed = true;
        let status = response.status();
        let mut terminal = self.engine_event(if status >= 400 { ERROR } else { RESULT });
        terminal.push(entry(STATUS, Value::from(status)));
        if status >= 400 {
            terminal.push(entry(MESSAGE, Value::from(message_of(response.body()))));
        } else {
            terminal.push(entry(BODY, response.body().clone()));
        }
        let terminal = Value::Map(terminal);
        match self.kind {
            Kind::Stream => {
                if state.streaming {
                    let name = event_type(&terminal);
                    if let Some(writer) = state.writer.as_mut() {
                        let _ = writer.write_named(&name, terminal).await;
                        let _ = writer.close().await;
                    }
                }
            }
            Kind::Event => self.send(&mut state, terminal).await,
            Kind::Socket => {
                self.send(&mut state, terminal).await;
                close_session(&self.target).await;
            }
        }
    }

    /// A sender's last events may still wait in its relay when the run ends. Each relay is one
    /// instance, so its answer to a drain request means every event queued before the request
    /// has been delivered: the terminal event follows them. Called without the channel's lock,
    /// which the relay needs to deliver.
    async fn drain_relays(&self) {
        let routes: Vec<String> = {
            let state = self.state.lock().await;
            if state.closed || state.relays.is_empty() {
                return;
            }
            state
                .relays
                .iter()
                .map(|(_, route)| route.clone())
                .collect()
        };
        // each drain request runs as its own task, outside the caller's trace bracket (a tokio
        // task-local), so it is sent untraced, as the Java engine's EventEmitter.eRequest sends
        // it: a traced RPC would record a span for every drain
        let pending = routes.into_iter().map(|route| {
            tokio::spawn(async move {
                let platform = Platform::get_instance();
                let request = EventEnvelope::new()
                    .set_to(&route)
                    .set_header(DRAIN, "true");
                if let Err(e) = PostOffice::new(&platform)
                    .request(request, Duration::from_millis(DRAIN_MS))
                    .await
                {
                    log::debug!("Relay {route} not drained - {}", e.message());
                }
            })
        });
        for outcome in futures_util::future::join_all(pending).await {
            if let Err(e) = outcome {
                log::debug!("Relay not drained - {e}");
            }
        }
    }

    /// A run that ended without a terminal event (a flow timeout, for example) still ends its
    /// channel, with the error its flow reported: `advice` is the end-of-flow advice, carrying
    /// the flow's error when it was aborted.
    pub async fn abandon(&self, advice: &Value) {
        let status = map_get(advice, STATUS).and_then(|v| match v {
            Value::Integer(n) => n.as_i64(),
            _ => None,
        });
        let response = match status {
            Some(status) if status >= 400 => EventEnvelope::new()
                .set_status(status as i32)
                .set_raw_body(Value::from(display(
                    &map_get(advice, MESSAGE).cloned().unwrap_or(Value::Nil),
                ))),
            _ => EventEnvelope::new()
                .set_status(500)
                .set_raw_body(Value::from("The run ended without a result")),
        };
        self.finish(&response).await;
    }

    /// The relay route a node's function sends its progress events to, registered when the node
    /// first needs it; None when the run has ended.
    pub async fn relay_for(self: &Arc<Self>, node: &str) -> Option<String> {
        let mut state = self.state.lock().await;
        if state.closed {
            return None;
        }
        if let Some((_, route)) = state.relays.iter().find(|(alias, _)| alias == node) {
            return Some(route.clone());
        }
        let route = format!(
            "{RELAY_PREFIX}{}.{}",
            self.instance_id,
            state.relays.len() + 1
        );
        let relay = ProgressRelay::new(self.clone(), node);
        // one instance, so a sender's events keep their order through it and a drain request is
        // answered only after every event queued before it; private to the application
        if let Err(e) = Platform::get_instance().register_private(&route, Arc::new(relay), 1) {
            log::warn!(
                "Unable to register progress relay {route} for node {node} - {}",
                e.message()
            );
            return None;
        }
        live_relays()
            .lock()
            .expect("live relays")
            .insert(route.clone());
        state.relays.push((node.to_string(), route.clone()));
        Some(route)
    }

    /// Release the run's relay routes. A sender's event after this is not delivered.
    pub async fn close(&self) {
        let routes = {
            let mut state = self.state.lock().await;
            state.closed = true;
            std::mem::take(&mut state.relays)
        };
        let platform = Platform::get_instance();
        for (_, route) in routes {
            live_relays().lock().expect("live relays").remove(&route);
            platform.release(&route);
        }
    }

    fn engine_event(&self, kind: &str) -> Vec<(Value, Value)> {
        let seq = self.sequence.fetch_add(1, Ordering::SeqCst) + 1;
        let mut event = vec![
            entry(TYPE, Value::from(kind)),
            entry(SEQ, Value::from(seq)),
            entry(TIME, Value::from(platform_core::trace::iso8601_utc_now())),
        ];
        if let Some(cid) = self.cid() {
            event.push(entry(CID, Value::from(cid)));
        }
        event.push(entry(GRAPH, Value::from(self.graph_id.as_str())));
        event
    }

    async fn send(&self, state: &mut ChannelState, event: Value) {
        let platform = Platform::get_instance();
        let po = PostOffice::new(&platform);
        let outcome = po
            .send_untraced(
                EventEnvelope::new()
                    .set_to(&self.target)
                    .set_raw_body(event),
            )
            .await;
        if let Err(e) = outcome {
            if !state.unavailable {
                state.unavailable = true;
                log::warn!(
                    "Return route {} of graph {} is not available - {}",
                    self.target,
                    self.graph_id,
                    e.message()
                );
            }
        }
    }
}

/// Close a WebSocket session after its last frame (Java `Utility.closeConnection(txPath, 1000,
/// "done")`): the transmitter route is one instance, so the close follows the frames sent before.
pub(crate) async fn close_session(tx_path: &str) {
    let platform = Platform::get_instance();
    let po = PostOffice::new(&platform);
    let close = EventEnvelope::new()
        .set_to(tx_path)
        .set_header(TYPE, "close")
        .set_header(STATUS, "1000")
        .set_header(MESSAGE, "done");
    let _ = po.send_untraced(close).await;
}

fn entry(key: &str, value: Value) -> (Value, Value) {
    (Value::from(key), value)
}

fn map_get<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    match value {
        Value::Map(entries) => entries
            .iter()
            .find(|(k, _)| k.as_str() == Some(key))
            .map(|(_, v)| v),
        _ => None,
    }
}

/// Java `Map.putIfAbsent`: set the key when it is absent or holds null.
fn put_if_absent(entries: &mut Vec<(Value, Value)>, key: &str, value: Value) {
    match entries.iter_mut().find(|(k, _)| k.as_str() == Some(key)) {
        Some((_, existing)) if existing.is_nil() => *existing = value,
        Some(_) => {}
        None => entries.push(entry(key, value)),
    }
}

fn event_type(event: &Value) -> String {
    map_get(event, TYPE).map(display).unwrap_or_default()
}

/// An error event's message: the `message` of a map body, otherwise the body as text.
fn message_of(body: &Value) -> String {
    match map_get(body, MESSAGE) {
        Some(message) if !message.is_nil() => display(message),
        _ => display(body),
    }
}
