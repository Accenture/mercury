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

//! The WebSocket server — Rust port of the Java platform-core websocket
//! layer (`@WebSocketService` + `WsRequestHandler` + `WsServerTransmitter` +
//! `WsEnvelope`), riding the REST automation server's HTTP upgrade path
//! (hyper upgrade + tokio-tungstenite, design K6a).
//!
//! Protocol (Java parity): a service registered under a name listens at
//! `/ws/{name}/{token}`. Each connection becomes a private route pair —
//! `{session}.in` (an instance of the service function) and `{session}.out`
//! (the transmitter) where `session` is `ws.{random}.{seq}`. The service
//! receives lifecycle events on the `.in` route:
//!
//! - `type: open` with `route`, `tx_path`, `ip`, `path`, `query`, `token`
//! - `type: string` / `type: bytes` with the message body, `route`, `tx_path`
//! - `type: close` with `close_code`, `close_reason`, `token` (the reply-to
//!   housekeeper releases the route pair afterwards)
//!
//! Replies go by sending to the `tx_path`: a string body becomes a text
//! frame, bytes become a binary frame, a map/list is rendered as JSON text
//! (segmented above 62 KB, Java parity); `type: close` with optional
//! `status`/`message` headers closes the socket. Idle connections are closed
//! by the housekeeping sweep (`websocket.idle.timeout`, default 60s).
//!
//! Rust has no runtime annotation scanning, so services register explicitly
//! through [`register_ws_service`] (typically in a `#[before_application]`
//! hook) instead of Java's `@WebSocketService` classpath scan.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock};

use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response, StatusCode};
use rmpv::Value;
use tokio_tungstenite::tungstenite::handshake::derive_accept_key;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::{CloseFrame, Role};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use crate::envelope::EventEnvelope;
use crate::function::{AppError, ComposableFunction};
use crate::platform::{FunctionOptions, Platform};
use crate::post_office::PostOffice;
use crate::util::app_config_reader::AppConfigReader;

const HOUSEKEEPER: &str = "ws.server.housekeeper";
const BUFFER_SIZE: usize = 62 * 1024;

/// A registered websocket service: one function instantiated per connection.
type WsFactory = Arc<dyn Fn() -> Arc<dyn ComposableFunction> + Send + Sync>;

fn services() -> &'static RwLock<HashMap<String, WsFactory>> {
    static SERVICES: OnceLock<RwLock<HashMap<String, WsFactory>>> = OnceLock::new();
    SERVICES.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Register a websocket service under `/ws/{name}` (the Java
/// `@WebSocketService(name)` analog; prefer the declarative
/// `#[websocket_service]` macro). The factory creates one function object
/// per connection.
pub fn register_ws_service(
    name: &str,
    factory: impl Fn() -> Arc<dyn ComposableFunction> + Send + Sync + 'static,
) {
    register_ws_service_with_namespace("ws", name, factory);
}

/// Namespace-aware registration (Java `@WebSocketService(value, namespace)`).
pub fn register_ws_service_with_namespace(
    namespace: &str,
    name: &str,
    factory: impl Fn() -> Arc<dyn ComposableFunction> + Send + Sync + 'static,
) {
    services()
        .write()
        .expect("ws service registry poisoned")
        .insert(format!("/{namespace}/{name}"), Arc::new(factory));
    log::info!("Websocket service /{namespace}/{name} loaded");
}

/// True when any websocket service is registered.
pub fn has_ws_services() -> bool {
    !services()
        .read()
        .expect("ws service registry poisoned")
        .is_empty()
}

struct LiveConnection {
    last_access: Arc<AtomicI64>,
    sender: tokio::sync::mpsc::Sender<TxCommand>,
}

fn connections() -> &'static Mutex<HashMap<String, LiveConnection>> {
    static CONNECTIONS: OnceLock<Mutex<HashMap<String, LiveConnection>>> = OnceLock::new();
    CONNECTIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Match a request path against the registered services
/// (Java `findPath`: `/ws/{name}/{token}` — token required).
fn find_service(path: &str) -> Option<(String, WsFactory)> {
    let registry = services().read().expect("ws service registry poisoned");
    for (prefix, factory) in registry.iter() {
        let with_slash = format!("{prefix}/");
        if path.starts_with(&with_slash) && path != with_slash {
            return Some((prefix.clone(), factory.clone()));
        }
    }
    None
}

/// True when the request asks for a websocket upgrade on a registered path.
pub(crate) fn is_ws_upgrade(request: &Request<hyper::body::Incoming>) -> bool {
    if request.method() != hyper::Method::GET {
        return false;
    }
    let headers = request.headers();
    let connection_upgrade = headers
        .get("connection")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_lowercase().contains("upgrade"))
        .unwrap_or(false);
    let websocket = headers
        .get("upgrade")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.eq_ignore_ascii_case("websocket"))
        .unwrap_or(false);
    connection_upgrade && websocket && find_service(request.uri().path()).is_some()
}

/// Perform the websocket handshake and hand the connection to a session
/// task. Returns the `101 Switching Protocols` response.
pub(crate) fn handle_ws_upgrade(
    platform: &Platform,
    mut request: Request<hyper::body::Incoming>,
    peer_ip: String,
) -> Response<Full<Bytes>> {
    let path = request.uri().path().to_string();
    let query = request.uri().query().unwrap_or("").to_string();
    let Some((service_path, factory)) = find_service(&path) else {
        return simple_status(StatusCode::NOT_FOUND, "Not found");
    };
    let Some(key) = request
        .headers()
        .get("sec-websocket-key")
        .and_then(|v| v.to_str().ok())
        .map(str::to_string)
    else {
        return simple_status(StatusCode::BAD_REQUEST, "Missing Sec-WebSocket-Key");
    };
    let accept = derive_accept_key(key.as_bytes());
    let platform = platform.clone();
    tokio::spawn(async move {
        match hyper::upgrade::on(&mut request).await {
            Ok(upgraded) => {
                let io = hyper_util::rt::TokioIo::new(upgraded);
                let stream = WebSocketStream::from_raw_socket(io, Role::Server, None).await;
                run_session(
                    platform,
                    stream,
                    factory,
                    service_path,
                    path,
                    query,
                    peer_ip,
                )
                .await;
            }
            Err(e) => log::warn!("Websocket upgrade failed - {e}"),
        }
    });
    Response::builder()
        .status(StatusCode::SWITCHING_PROTOCOLS)
        .header("upgrade", "websocket")
        .header("connection", "Upgrade")
        .header("sec-websocket-accept", accept)
        .body(Full::new(Bytes::new()))
        .unwrap_or_else(|_| simple_status(StatusCode::INTERNAL_SERVER_ERROR, "handshake error"))
}

fn simple_status(status: StatusCode, message: &str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .body(Full::new(Bytes::from(message.to_string())))
        .expect("static response")
}

/// Commands accepted by the transmitter route.
enum TxCommand {
    Text(String),
    Bytes(Vec<u8>),
    Close { code: u16, message: String },
}

/// The `{session}.out` function (Java `WsServerTransmitter`).
struct WsTransmitter {
    sender: tokio::sync::mpsc::Sender<TxCommand>,
}

#[async_trait]
impl ComposableFunction for WsTransmitter {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if headers.get("type").map(String::as_str) == Some("close") {
            let code = headers
                .get("status")
                .and_then(|v| v.parse::<u16>().ok())
                .unwrap_or(1000);
            let message = headers
                .get("message")
                .cloned()
                .unwrap_or_else(|| "bye".to_string());
            let _ = self.sender.send(TxCommand::Close { code, message }).await;
            return EventEnvelope::new().set_body(true);
        }
        let delivered = match input.body() {
            Value::String(text) => {
                let text = text.as_str().unwrap_or_default().to_string();
                self.sender.send(TxCommand::Text(text)).await.is_ok()
            }
            Value::Binary(bytes) => self
                .sender
                .send(TxCommand::Bytes(bytes.clone()))
                .await
                .is_ok(),
            value @ (Value::Map(_) | Value::Array(_)) => {
                // maps and lists render as JSON text, segmented above 62 KB;
                // Nil map entries are omitted unless serializer.null.transport=true
                let text = serde_json::to_value(crate::serializer::strip_nulls(value))
                    .map(|v| v.to_string())
                    .unwrap_or_default();
                let mut ok = true;
                let bytes = text.as_bytes();
                let mut start = 0;
                while start < bytes.len() {
                    let end = (start + BUFFER_SIZE).min(bytes.len());
                    let segment = String::from_utf8_lossy(&bytes[start..end]).to_string();
                    ok &= self.sender.send(TxCommand::Text(segment)).await.is_ok();
                    start = end;
                }
                if bytes.is_empty() {
                    ok &= self
                        .sender
                        .send(TxCommand::Text(String::new()))
                        .await
                        .is_ok();
                }
                ok
            }
            _ => false,
        };
        EventEnvelope::new().set_body(delivered)
    }
}

/// The housekeeper (Java `WsHousekeeper`): releases a closed session's
/// route pair after the service consumed the close signal.
struct WsHousekeeper {
    platform: Platform,
}

#[async_trait]
impl ComposableFunction for WsHousekeeper {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if let Some(session) = input.correlation_id() {
            self.platform.release(&format!("{session}.in"));
            self.platform.release(&format!("{session}.out"));
        }
        EventEnvelope::new().set_body("done")
    }
}

fn ensure_housekeeper(platform: &Platform) {
    if !platform.has_route(HOUSEKEEPER) {
        let _ = platform.register_with_options(
            HOUSEKEEPER,
            Arc::new(WsHousekeeper {
                platform: platform.clone(),
            }),
            1,
            FunctionOptions {
                zero_traced: true,
                // Java WsRequestHandler: registerPrivate
                private: true,
                interceptor: true,
            },
        );
    }
    // start the idle sweep once
    static SWEEP: OnceLock<()> = OnceLock::new();
    SWEEP.get_or_init(|| {
        let config = AppConfigReader::get_instance();
        let timeout_s = config
            .get_property_or("websocket.idle.timeout", "60")
            .parse::<i64>()
            .unwrap_or(60)
            .max(10);
        log::info!("Websocket server idle expiry {timeout_s} seconds");
        tokio::spawn(async move {
            let expiry_ms = timeout_s * 1000;
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                let now = now_ms();
                let expired: Vec<(String, tokio::sync::mpsc::Sender<TxCommand>)> = {
                    let live = connections().lock().expect("ws connections poisoned");
                    live.iter()
                        .filter(|(_, c)| now - c.last_access.load(Ordering::Relaxed) > expiry_ms)
                        .map(|(session, c)| (session.clone(), c.sender.clone()))
                        .collect()
                };
                for (session, sender) in expired {
                    log::warn!("Websocket {session} expired");
                    let _ = sender
                        .send(TxCommand::Close {
                            code: 1003,
                            message: format!("Idle for {timeout_s} seconds"),
                        })
                        .await;
                }
            }
        });
    });
}

/// One live websocket session (Java `WsRequestHandler.handle`).
async fn run_session<S>(
    platform: Platform,
    stream: WebSocketStream<S>,
    factory: WsFactory,
    service_path: String,
    uri_path: String,
    query: String,
    peer_ip: String,
) where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
{
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    ensure_housekeeper(&platform);
    let token = uri_path.rsplit('/').next().unwrap_or_default().to_string();
    let random: u32 = 100000 + (uuid::Uuid::new_v4().as_u128() % 900000) as u32;
    let n = COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    let session = format!("ws.{random}.{n}");
    let rx_path = format!("{session}.in");
    let tx_path = format!("{session}.out");
    let (tx_sender, mut tx_receiver) = tokio::sync::mpsc::channel::<TxCommand>(64);
    let last_access = Arc::new(AtomicI64::new(now_ms()));
    // the service function and the transmitter become a private route pair
    if platform.register(&rx_path, factory(), 1).is_err() {
        log::error!("Unable to register websocket session {session}");
        return;
    }
    let _ = platform.register_with_options(
        &tx_path,
        Arc::new(WsTransmitter {
            sender: tx_sender.clone(),
        }),
        1,
        FunctionOptions {
            zero_traced: true,
            // Java WsRequestHandler: registerPrivate
            private: true,
            interceptor: false,
        },
    );
    connections()
        .lock()
        .expect("ws connections poisoned")
        .insert(
            session.clone(),
            LiveConnection {
                last_access: last_access.clone(),
                sender: tx_sender.clone(),
            },
        );
    log::info!("Session {session} connected");
    let po = PostOffice::new(&platform);
    let open = EventEnvelope::new()
        .set_to(&rx_path)
        .set_header("type", "open")
        .set_header("route", &rx_path)
        .set_header("tx_path", &tx_path)
        .set_header("ip", &peer_ip)
        .set_header("path", &service_path)
        .set_header("query", &query)
        .set_header("token", &token);
    let _ = po.send(open).await;
    let (mut write, mut read) = stream.split();
    // writer task: consume transmitter commands
    let writer = tokio::spawn(async move {
        let mut close_frame: Option<(u16, String)> = None;
        while let Some(command) = tx_receiver.recv().await {
            match command {
                TxCommand::Text(text) => {
                    if write.send(Message::Text(text)).await.is_err() {
                        break;
                    }
                }
                TxCommand::Bytes(bytes) => {
                    if write.send(Message::Binary(bytes)).await.is_err() {
                        break;
                    }
                }
                TxCommand::Close { code, message } => {
                    close_frame = Some((code, message));
                    break;
                }
            }
        }
        let (code, message) = close_frame.unwrap_or((1000, "bye".to_string()));
        let _ = write
            .send(Message::Close(Some(CloseFrame {
                code: CloseCode::from(code),
                reason: message.into(),
            })))
            .await;
    });
    // reader loop: forward frames to the service function
    let mut close_code: u16 = 1000;
    let mut close_reason = "ok".to_string();
    while let Some(frame) = read.next().await {
        last_access.store(now_ms(), Ordering::Relaxed);
        match frame {
            Ok(Message::Text(text)) => {
                let event = EventEnvelope::new()
                    .set_to(&rx_path)
                    .set_header("type", "string")
                    .set_header("route", &rx_path)
                    .set_header("tx_path", &tx_path)
                    .set_raw_body(Value::from(text.as_str()));
                let _ = po.send(event).await;
            }
            Ok(Message::Binary(bytes)) => {
                let event = EventEnvelope::new()
                    .set_to(&rx_path)
                    .set_header("type", "bytes")
                    .set_header("route", &rx_path)
                    .set_header("tx_path", &tx_path)
                    .set_raw_body(Value::from(bytes.to_vec()));
                let _ = po.send(event).await;
            }
            Ok(Message::Close(frame)) => {
                if let Some(frame) = frame {
                    close_code = frame.code.into();
                    close_reason = frame.reason.to_string();
                }
                break;
            }
            Ok(_) => {} // ping/pong handled by tungstenite
            Err(e) => {
                log::warn!("Session {session} exception - {e}");
                break;
            }
        }
    }
    log::info!("Session {session} closed ({close_code}, {close_reason})");
    connections()
        .lock()
        .expect("ws connections poisoned")
        .remove(&session);
    // close signal to the service, then the housekeeper releases the routes
    let close_signal = EventEnvelope::new()
        .set_to(&rx_path)
        .set_header("route", &rx_path)
        .set_header("token", &token)
        .set_header("close_code", &close_code.to_string())
        .set_header("close_reason", &close_reason)
        .set_header("type", "close")
        .set_reply_to(HOUSEKEEPER)
        .set_correlation_id(&session);
    let _ = po.send(close_signal).await;
    let _ = writer.await;
}
