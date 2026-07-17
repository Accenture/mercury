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

//! The Playground websocket handlers — Rust ports of `GraphUserInterface`
//! (`/ws/graph/{token}`, dispatching to the command service) and
//! `JsonPathHandler` (`/ws/json/{token}`, the JSON-Path playground). Both
//! are dev-gated: the `PlaygroundLoader` hook registers them only when
//! `app.env=dev` (the Java `@OptionalService` analog — this is why they use
//! programmatic `register_ws_service` instead of the static macro).

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use async_trait::async_trait;
use event_script::mlm::MultiLevelMap;
use platform_core::{AppError, ComposableFunction, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::commands;

const IN_SUFFIX: &str = ".in";

async fn say(po: &PostOffice, to: &str, message: impl Into<String>) {
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(to)
                .set_raw_body(Value::from(message.into().as_str())),
        )
        .await;
}

/// Java `GraphUserInterface` (`@WebSocketService("graph")`).
pub struct GraphUserInterface;

#[async_trait]
impl ComposableFunction for GraphUserInterface {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let platform = Platform::get_instance();
        let po = PostOffice::new(&platform);
        let get = |key: &str| headers.get(key).cloned().unwrap_or_default();
        match get("type").as_str() {
            "open" => {
                let route = get("route");
                let tx_path = get("tx_path");
                let _ = po
                    .send(
                        EventEnvelope::new()
                            .set_to(commands::ROUTE)
                            .set_raw_body(Value::Map(vec![
                                (Value::from("in"), Value::from(route.as_str())),
                                (Value::from("type"), Value::from("open")),
                            ])),
                    )
                    .await;
                log::info!(
                    "Started {route}, {tx_path}, ip={}, path={}, query={}, token={}",
                    get("ip"),
                    get("path"),
                    get("query"),
                    get("token")
                );
                // surface the public session id to the client
                let public_id = route
                    .strip_suffix(IN_SUFFIX)
                    .unwrap_or(&route)
                    .replace('.', "-");
                say(
                    &po,
                    &tx_path,
                    format!(
                        "session {public_id} started\nCompanion endpoint: /api/companion/{public_id}"
                    ),
                )
                .await;
            }
            "close" => {
                let route = get("route");
                log::info!("Stopped {route}, token={}", get("token"));
                let _ = po
                    .send(
                        EventEnvelope::new()
                            .set_to(commands::ROUTE)
                            .set_raw_body(Value::Map(vec![
                                (Value::from("in"), Value::from(route.as_str())),
                                (Value::from("type"), Value::from("close")),
                            ])),
                    )
                    .await;
            }
            "bytes" => {
                let route = get("route");
                let n = match input.body() {
                    Value::Binary(bytes) => bytes.len(),
                    _ => 0,
                };
                say(&po, &get("tx_path"), format!("received {n} bytes")).await;
                log::info!("{route} got {n} bytes");
            }
            "string" => {
                let message = match input.body() {
                    Value::String(text) => text.as_str().unwrap_or_default().trim().to_string(),
                    other => other.to_string(),
                };
                let _ = po
                    .send(
                        EventEnvelope::new()
                            .set_to(commands::ROUTE)
                            .set_raw_body(Value::Map(vec![
                                (Value::from("type"), Value::from("command")),
                                (Value::from("in"), Value::from(get("route").as_str())),
                                (Value::from("message"), Value::from(message.as_str())),
                                (Value::from("out"), Value::from(get("tx_path").as_str())),
                            ])),
                    )
                    .await;
            }
            _ => log::error!("Invalid event {headers:?}"),
        }
        EventEnvelope::new().set_body("ok")
    }
}

// ---- the JSON-Path playground (Java JsonPathHandler) ----

enum JsonSession {
    AwaitingText,
    Loaded(MultiLevelMap),
}

fn text_map() -> &'static Mutex<HashMap<String, JsonSession>> {
    static TEXT: OnceLock<Mutex<HashMap<String, JsonSession>>> = OnceLock::new();
    TEXT.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Java `JsonPathHandler` (`@WebSocketService("json")`).
pub struct JsonPathHandler;

#[async_trait]
impl ComposableFunction for JsonPathHandler {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let platform = Platform::get_instance();
        let po = PostOffice::new(&platform);
        let get = |key: &str| headers.get(key).cloned().unwrap_or_default();
        match get("type").as_str() {
            "open" => {
                log::info!(
                    "Started {}, {}, ip={}, path={}, query={}, token={}",
                    get("route"),
                    get("tx_path"),
                    get("ip"),
                    get("path"),
                    get("query"),
                    get("token")
                );
            }
            "close" => {
                let route = get("route");
                log::info!("Stopped {route}, token={}", get("token"));
                text_map().lock().expect("json sessions").remove(&route);
            }
            "bytes" => {
                let n = match input.body() {
                    Value::Binary(bytes) => bytes.len(),
                    _ => 0,
                };
                say(&po, &get("tx_path"), format!("received {n} bytes")).await;
            }
            "string" => {
                let message = match input.body() {
                    Value::String(text) => text.as_str().unwrap_or_default().trim().to_string(),
                    other => other.to_string(),
                };
                handle_text_message(&po, &get("route"), &get("tx_path"), &message).await;
            }
            _ => log::error!("Invalid event {headers:?}"),
        }
        EventEnvelope::new().set_body("ok")
    }
}

async fn handle_text_message(po: &PostOffice, route: &str, tx_path: &str, message: &str) {
    match message {
        "help" => {
            say(
                po,
                tx_path,
                "- load: load XML/JSON string as JSON object,\n\
                 - upload: upload XML/JSON string,\n\
                 - unload: clear stored JSON object,\n\
                 - any simple data retrieval or JSON-Path command,\n\
                 - help: this command\n",
            )
            .await;
        }
        "load" => {
            text_map()
                .lock()
                .expect("json sessions")
                .insert(route.to_string(), JsonSession::AwaitingText);
        }
        "upload" => {
            text_map()
                .lock()
                .expect("json sessions")
                .insert(route.to_string(), JsonSession::AwaitingText);
            let id = route.replace('.', "-");
            say(
                po,
                tx_path,
                format!("Please upload XML/JSON text to /api/json/content/{id}\n"),
            )
            .await;
        }
        "unload" => {
            text_map().lock().expect("json sessions").remove(route);
            say(po, tx_path, "JSON unloaded\n").await;
        }
        _ => {
            if message.starts_with('{')
                && message.ends_with('}')
                && handle_handshake(po, tx_path, message).await
            {
                return;
            }
            let state = {
                let sessions = text_map().lock().expect("json sessions");
                match sessions.get(route) {
                    Some(JsonSession::AwaitingText) => Some(None),
                    Some(JsonSession::Loaded(mm)) => Some(Some(mm.to_value())),
                    None => None,
                }
            };
            match state {
                Some(None) => {
                    if let Some(error) = render_json(po, route, tx_path, message).await {
                        text_map().lock().expect("json sessions").remove(route);
                        say(po, tx_path, format!("Invalid JSON/XML - {error}")).await;
                    }
                }
                Some(Some(loaded)) => {
                    let mm = MultiLevelMap::from_value(loaded);
                    say(
                        po,
                        tx_path,
                        format!(
                            "{} command: {message}",
                            if message.starts_with('$') {
                                "JSON-Path"
                            } else {
                                "Simple retrieval"
                            }
                        ),
                    )
                    .await;
                    let result = mm.get_element(message).unwrap_or(Value::Nil);
                    say(
                        po,
                        tx_path,
                        format!("{}\n", event_script::conversions::to_json_string(&result)),
                    )
                    .await;
                }
                None => {
                    say(
                        po,
                        tx_path,
                        format!("JSON/XML not loaded - you entered: {message}\n"),
                    )
                    .await;
                }
            }
        }
    }
}

/// Parse and store the uploaded JSON (Java `renderXmlOrJson`; the XML branch
/// is a documented deferral — the XML parser is not ported).
async fn render_json(po: &PostOffice, route: &str, tx_path: &str, message: &str) -> Option<String> {
    let parsed: Result<serde_json::Value, _> = if (message.starts_with('{')
        && message.ends_with('}'))
        || (message.starts_with('[') && message.ends_with(']'))
    {
        serde_json::from_str(message)
    } else if message.starts_with('<') && message.ends_with('>') {
        return Some("XML parsing is not supported by this port - please use JSON".to_string());
    } else {
        return Some("Payload cannot be parsed as XML or JSON".to_string());
    };
    match parsed {
        Ok(json) => {
            let value = Value::Map(vec![(
                Value::from("response"),
                event_script::conversions::from_json(&json),
            )]);
            let mm = MultiLevelMap::from_value(value.clone());
            text_map()
                .lock()
                .expect("json sessions")
                .insert(route.to_string(), JsonSession::Loaded(mm));
            say(
                po,
                tx_path,
                format!("{}\n", event_script::conversions::to_json_string(&value)),
            )
            .await;
            say(
                po,
                tx_path,
                "JSON rendered. Please enter simple retrieval or JSON-Path commands.",
            )
            .await;
            None
        }
        Err(e) => Some(e.to_string()),
    }
}

async fn handle_handshake(po: &PostOffice, tx_path: &str, message: &str) -> bool {
    let Ok(parsed) = serde_json::from_str::<serde_json::Value>(message) else {
        return false;
    };
    match parsed.get("type").and_then(|v| v.as_str()) {
        Some("ping") => {
            let _ = po
                .send(
                    EventEnvelope::new()
                        .set_to(tx_path)
                        .set_raw_body(Value::Map(vec![(Value::from("type"), Value::from("pong"))])),
                )
                .await;
            true
        }
        Some("welcome") => {
            say(po, tx_path, "Welcome to JSON-Path Playground").await;
            true
        }
        _ => false,
    }
}

/// Java `JsonPathHandler.uploadContent` (the `/api/json/content/{id}` REST hop).
pub async fn upload_content(platform: &Platform, id: &str, content: &str) -> bool {
    let route = id.replace('-', ".");
    let loaded = {
        let sessions = text_map().lock().expect("json sessions");
        sessions.contains_key(&route)
    };
    if !loaded {
        return false;
    }
    let tx_path = match route.rfind('.') {
        Some(dot) => format!("{}.out", &route[..dot]),
        None => route.clone(),
    };
    let po = PostOffice::new(platform);
    if let Some(error) = render_json(&po, &route, &tx_path, content).await {
        say(&po, &tx_path, error).await;
    }
    true
}
