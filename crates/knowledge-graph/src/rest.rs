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

//! The Playground REST endpoints (Java `rest` package, all dev-gated):
//! home/workbench pages, the AI-companion command hop, uploads into live
//! sessions, draft-graph description, live-graph download and state-machine
//! inspection. Registered declaratively in [`crate`] via `#[preload]` +
//! `#[optional_service("app.env=dev")]` (the Java `@RestEndpoint`/`@PreLoad`
//! + `@OptionalService` analog), so they register only when `app.env=dev`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use event_script::conversions::{display, from_json, to_json_string};
use platform_core::{
    AppConfigReader, AppError, ComposableFunction, EventEnvelope, Platform, PostOffice,
};
use rmpv::Value;

use crate::commands;

fn invalid(message: impl Into<String>) -> AppError {
    AppError::new(400, message)
}

fn request_view(event: &EventEnvelope) -> (HashMap<String, String>, Value, String) {
    let mut path_parameters: HashMap<String, String> = HashMap::new();
    let mut body = Value::Nil;
    let mut method = String::new();
    if let Value::Map(entries) = event.body() {
        for (key, value) in entries {
            match key.as_str() {
                Some("parameters") => {
                    if let Value::Map(parameters) = value {
                        for (k, v) in parameters {
                            if k.as_str() == Some("path") {
                                if let Value::Map(path) = v {
                                    for (pk, pv) in path {
                                        if let Some(name) = pk.as_str() {
                                            path_parameters.insert(name.to_string(), display(pv));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some("body") => body = value.clone(),
                Some("method") => method = display(value),
                _ => {}
            }
        }
    }
    (path_parameters, body, method)
}

/// Java `GetIndexHtml` (`get.index.html`): the home page — `/public` when
/// dev, `/template` otherwise.
pub async fn get_index_html(_event: EventEnvelope) -> Result<EventEnvelope, AppError> {
    let config = AppConfigReader::get_instance();
    let location = if config.get_property_or("app.env", "dev") == "dev" {
        "/public"
    } else {
        "/template"
    };
    let resource = format!("{location}/index.html");
    let resolved = platform_core::resources::resolve_classpath(&resource)
        .ok_or_else(|| AppError::new(404, format!("{resource} not found")))?;
    let content = std::fs::read_to_string(resolved).map_err(|e| invalid(e.to_string()))?;
    Ok(EventEnvelope::new()
        .set_header("Content-Type", "text/html; charset=utf-8")
        .set_raw_body(Value::from(content.as_str())))
}

/// Java `GetWsHtml` (`get.ws.html`): the raw websocket workbench pages.
pub async fn get_ws_html(event: EventEnvelope) -> Result<EventEnvelope, AppError> {
    let (path_parameters, _, _) = request_view(&event);
    let id = path_parameters.get("id").cloned().unwrap_or_default();
    if id != "graph" && id != "json" {
        return Err(invalid("Path parameter must be graph or json"));
    }
    let resolved = platform_core::resources::resolve_classpath(&format!("/template/ws-{id}.html"))
        .ok_or_else(|| invalid("Template not found"))?;
    let config = AppConfigReader::get_instance();
    let Some(port) = config.get_property("rest.server.port") else {
        return Err(invalid(
            "Missing rest.server.port in application configuration",
        ));
    };
    let url = format!("http://127.0.0.1:{port}/ws/{id}/playground");
    let html = std::fs::read_to_string(resolved)
        .map_err(|e| invalid(e.to_string()))?
        .replace("$WS_URL", &url);
    Ok(EventEnvelope::new()
        .set_header("Content-Type", "text/html; charset=utf-8")
        .set_raw_body(Value::from(html.as_str())))
}

/// Java `PostCompanionCommand` (`post.companion.command`): the AI-companion
/// hop — dispatches a text command to the singleton command handler.
pub async fn post_companion_command(
    platform: &Platform,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let (path_parameters, body, _) = request_view(&event);
    let Some(id) = path_parameters.get("id") else {
        return Err(invalid("Missing path parameter: id"));
    };
    let command = match &body {
        Value::String(text) => text.as_str().unwrap_or_default().trim().to_string(),
        _ => String::new(),
    };
    if command.is_empty() {
        return Err(invalid("Body must be a non-empty text/plain command"));
    }
    if !commands::has_session(id) {
        return Err(AppError::new(404, format!("No active session for id {id}")));
    }
    let route = id.replace('-', ".");
    let in_route = format!("{route}.in");
    let out_route = format!("{route}.out");
    let po = PostOffice::new(platform);
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(commands::SINGLETON_COMMAND_HANDLER)
                .set_raw_body(Value::Map(vec![
                    (Value::from("type"), Value::from("command")),
                    (Value::from("in"), Value::from(in_route.as_str())),
                    (Value::from("out"), Value::from(out_route.as_str())),
                    (Value::from("message"), Value::from(command.as_str())),
                ])),
        )
        .await;
    Ok(EventEnvelope::new()
        .set_header("Content-Type", "application/json")
        .set_raw_body(Value::Map(vec![
            (Value::from("type"), Value::from("companion")),
            (Value::from("status"), Value::from("accepted")),
            (Value::from("id"), Value::from(id.as_str())),
            (
                Value::from("message"),
                Value::from(
                    "Command dispatched to graph.command.service. Output streams to the \
                     WebSocket console for this session.",
                ),
            ),
        ])))
}

/// Sentinel appended after a synchronous command so the capture route knows the
/// command's (FIFO) output is fully drained. Not part of the returned output.
const SYNC_SENTINEL: &str = "__companion_sync_done__";

/// A private, per-call **capture route** used by [`post_companion_command_sync`]:
/// the command pipeline's `say()` output is directed here and buffered so the
/// outcome can be returned in the HTTP response. It **also tees** each line to the
/// session's real WebSocket `.out` route (fire-and-forget) so a watching human —
/// and, via the command service's subscriber fan-out, any subscribed sessions —
/// see the same output live. This is what makes the synchronous endpoint a
/// **real-time human+AI collaboration** surface rather than an AI-only side
/// channel. The internal sentinel is never teed.
struct CaptureSink {
    buffer: Arc<Mutex<Vec<Value>>>,
    tee_to: String,
}

#[async_trait]
impl ComposableFunction for CaptureSink {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body = input.body().clone();
        if !self.tee_to.is_empty() && body.as_str() != Some(SYNC_SENTINEL) {
            let po = PostOffice::new(&Platform::get_instance());
            let _ = po
                .send(
                    EventEnvelope::new()
                        .set_to(self.tee_to.as_str())
                        .set_raw_body(body.clone()),
                )
                .await;
        }
        self.buffer
            .lock()
            .expect("capture buffer poisoned")
            .push(body);
        EventEnvelope::new().set_body("ok")
    }
}

fn is_error_line(line: &str) -> bool {
    line.starts_with("ERROR:")
        || line.contains("aborted")
        || line.contains("does not have")
        || line.starts_with("Invalid")
        || line.contains("not found")
        || line.contains("Please try 'help'")
}

/// **Synchronous** AI-companion command (design: `docs/design/ai-companion-sync.md`).
/// Additive sibling of [`post_companion_command`]: dispatches the same command but
/// returns the command's **outcome in-band** — `ok`, the console `output` lines,
/// the first `error` (if any), and any structured `result` (e.g. a run's
/// `output.body`) — instead of a fire-and-forget `{status:"accepted"}`. The output
/// is **also teed to the session's WebSocket console** (via [`CaptureSink`]) so a
/// watching human — and any subscribed sessions — see it live: real-time human+AI
/// collaboration on one graph. The legacy fire-and-forget endpoint is unchanged.
pub async fn post_companion_command_sync(
    platform: &Platform,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let (path_parameters, body, _) = request_view(&event);
    let Some(id) = path_parameters.get("id") else {
        return Err(invalid("Missing path parameter: id"));
    };
    let command = match &body {
        Value::String(text) => text.as_str().unwrap_or_default().trim().to_string(),
        _ => String::new(),
    };
    if command.is_empty() {
        return Err(invalid("Body must be a non-empty text/plain command"));
    }
    if !commands::has_session(id) {
        return Err(AppError::new(404, format!("No active session for id {id}")));
    }

    let route = id.replace('-', ".");
    let in_route = format!("{route}.in");
    let out_route = format!("{route}.out");
    let capture_route = format!("companion.sync.{}", uuid::Uuid::new_v4().simple());
    let buffer = Arc::new(Mutex::new(Vec::<Value>::new()));
    platform.register(
        &capture_route,
        Arc::new(CaptureSink {
            buffer: buffer.clone(),
            // tee to the session's real WebSocket console so humans watch live
            tee_to: out_route,
        }),
        1,
    )?;

    let po = PostOffice::new(platform);
    // RPC the command handler with the capture route as `out`; `handle()` awaits
    // all `say()` calls before replying, so this resolves once the command is done.
    let _ = po
        .request(
            EventEnvelope::new()
                .set_to(commands::SINGLETON_COMMAND_HANDLER)
                .set_raw_body(Value::Map(vec![
                    (Value::from("type"), Value::from("command")),
                    (Value::from("in"), Value::from(in_route.as_str())),
                    (Value::from("out"), Value::from(capture_route.as_str())),
                    (Value::from("message"), Value::from(command.as_str())),
                ])),
            Duration::from_secs(30),
        )
        .await;

    // `say()` is fire-and-forget; the sentinel (enqueued after, FIFO) marks the
    // buffer fully drained — deterministic, no arbitrary sleep.
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(capture_route.as_str())
                .set_raw_body(Value::from(SYNC_SENTINEL)),
        )
        .await;
    for _ in 0..250 {
        let seen = {
            let g = buffer.lock().expect("capture buffer poisoned");
            g.iter().any(|v| v.as_str() == Some(SYNC_SENTINEL))
        };
        if seen {
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    platform.release(&capture_route);

    // Build the structured outcome.
    let items = buffer.lock().expect("capture buffer poisoned").clone();
    let mut output: Vec<Value> = Vec::new();
    let mut result: Vec<Value> = Vec::new();
    let mut error: Option<String> = None;
    for v in items {
        match &v {
            Value::String(s) => {
                let line = s.as_str().unwrap_or_default().to_string();
                if line == SYNC_SENTINEL {
                    continue;
                }
                if error.is_none() && is_error_line(&line) {
                    error = Some(line.clone());
                }
                output.push(Value::from(line.as_str()));
            }
            _ => result.push(v),
        }
    }
    let ok = error.is_none();

    Ok(EventEnvelope::new()
        .set_header("Content-Type", "application/json")
        .set_raw_body(Value::Map(vec![
            (Value::from("ok"), Value::Boolean(ok)),
            (Value::from("id"), Value::from(id.as_str())),
            (Value::from("command"), Value::from(command.as_str())),
            (Value::from("output"), Value::Array(output)),
            (
                Value::from("error"),
                error.as_deref().map(Value::from).unwrap_or(Value::Nil),
            ),
            (
                Value::from("result"),
                if result.is_empty() {
                    Value::Nil
                } else {
                    Value::Array(result)
                },
            ),
        ])))
}

/// Java `UploadMockContent` (`upload.mock.content`): mock data into a live
/// graph instance's `input.body`.
pub async fn upload_mock_content(
    platform: &Platform,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let (path_parameters, body, _) = request_view(&event);
    let Some(id) = path_parameters.get("id") else {
        return Err(invalid("Missing path parameter: id"));
    };
    if !matches!(body, Value::Map(_) | Value::Array(_)) {
        return Err(invalid(
            "Input is not a valid JSON payload that represents a Map or List",
        ));
    }
    if commands::upload_content(platform, id, body).await {
        Ok(upload_ok())
    } else {
        Err(invalid(format!("Session {id} is expired or invalid")))
    }
}

/// Java `UploadJsonContent` (`upload.json.content`): JSON text into a
/// JSON-Path playground session.
pub async fn upload_json_content(
    platform: &Platform,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let (path_parameters, body, _) = request_view(&event);
    let Some(id) = path_parameters.get("id") else {
        return Err(invalid("Missing path parameter: id"));
    };
    if !matches!(body, Value::Map(_) | Value::Array(_)) {
        return Err(invalid(
            "Input is not a valid JSON/XML text that represents a Map or List",
        ));
    }
    let text = to_json_string(&body);
    if crate::ws_ui::upload_content(platform, id, &text).await {
        Ok(upload_ok())
    } else {
        Err(invalid(format!("Session {id} is expired or invalid")))
    }
}

fn upload_ok() -> EventEnvelope {
    EventEnvelope::new()
        .set_header("Content-Type", "application/json")
        .set_raw_body(Value::Map(vec![
            (Value::from("message"), Value::from("Content uploaded")),
            (Value::from("type"), Value::from("upload")),
        ]))
}

/// Java `DescribeGraph` (`show.graph.model`): read a draft graph from the
/// Playground temp folder.
pub async fn show_graph_model(event: EventEnvelope) -> Result<EventEnvelope, AppError> {
    let (path_parameters, _, _) = request_view(&event);
    let Some(filename) = path_parameters.get("graph_id") else {
        return Err(invalid("Missing path parameter 'graph_id'"));
    };
    let file = commands::temp_dir().join(format!("{filename}.json"));
    if !file.exists() {
        return Err(invalid(format!("Draft graph '{filename}' does not exist")));
    }
    let text = std::fs::read_to_string(&file).map_err(|e| invalid(e.to_string()))?;
    let parsed: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| invalid(e.to_string()))?;
    Ok(EventEnvelope::new()
        .set_header("Content-Type", "application/json")
        .set_raw_body(from_json(&parsed)))
}

/// Java `GetLiveGraph` (`get.live.graph`): export a live session's draft.
pub async fn get_live_graph(event: EventEnvelope) -> Result<EventEnvelope, AppError> {
    let (path_parameters, _, _) = request_view(&event);
    let Some(id) = path_parameters.get("id") else {
        return Err(invalid("Missing path parameter: id"));
    };
    match commands::download_graph(id) {
        Some(graph) => Ok(EventEnvelope::new()
            .set_header("Content-Type", "application/json")
            .set_raw_body(graph)),
        None => Err(AppError::new(404, format!("No active session for id {id}"))),
    }
}

/// Java `InspectStateMachine` (`inspect.state.machine`): read a key from a
/// live instance's state machine.
pub async fn inspect_state_machine(event: EventEnvelope) -> Result<EventEnvelope, AppError> {
    let (path_parameters, _, _) = request_view(&event);
    let (Some(id), Some(key)) = (path_parameters.get("id"), path_parameters.get("key")) else {
        return Err(invalid("Missing path parameter: id or key"));
    };
    // Wrap the resolved value in `{inspect, outcome}` — the same envelope the
    // `inspect {key}` console command emits (Java/Rust `GraphCommandService`).
    // Because it is always a Map, a scalar / primitive / Map / List all
    // serialize as clean JSON (no bare-scalar content-type ambiguity), and the
    // shape matches the command exactly. The composite key resolves through the
    // MultiLevelMap in `download_content`; 404 only when it resolves to nothing.
    match commands::download_content(id, key) {
        Some(outcome) => Ok(EventEnvelope::new()
            .set_header("Content-Type", "application/json")
            .set_raw_body(Value::Map(vec![
                (Value::from("inspect"), Value::from(key.as_str())),
                (Value::from("outcome"), outcome),
            ]))),
        None => Err(AppError::new(404, "Not found")),
    }
}
