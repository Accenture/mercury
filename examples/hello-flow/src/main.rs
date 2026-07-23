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

//! The event-script taste of mercury: the transaction lives in
//! `resources/flows/hello-flow.yml` — the functions below are the only
//! code, and none knows the others exist (route-name + envelope coupling
//! only). Linking the `event-script` crate self-registers the flow engine
//! through the annotation inventory; `rest.yaml` binds the endpoint to the
//! flow with `flow: 'hello-flow'`.
//!
//! This example is the structural parallel of the Java `composable-example`
//! (same port 8100, same demo endpoints). Besides the greeting flow, it
//! ships the two **Event-over-HTTP demo endpoints** — see `README.md` and
//! the Event over HTTP guide — whose callee is the `hello-world` example
//! (or, interchangeably, the Java `lambda-example`) on port 8085.
//!
//! ```bash
//! cargo run -p hello-flow
//! curl 'http://127.0.0.1:8100/api/hello/eric?lang=fr'
//! curl 'http://127.0.0.1:8100/api/hello/eric?lang=en'
//! ```

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use platform_core::automation::event_over_http_with_headers;
use platform_core::{
    main_application, preload, AppConfigReader, AppError, ComposableFunction, EntryPoint,
    EventEnvelope, Platform, PostOffice,
};

/// Decision task: French when `lang=fr`, English otherwise.
#[preload(route = "language.router")]
struct LanguageRouter;

#[async_trait]
impl ComposableFunction for LanguageRouter {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        EventEnvelope::new().set_body(body["lang"].as_str() == Some("fr"))
    }
}

/// Composes the greeting from whatever the flow mapped into its input.
#[preload(route = "greeting.composer", instances = 5)]
struct GreetingComposer;

#[async_trait]
impl ComposableFunction for GreetingComposer {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as()?;
        let user = body["user"].as_str().unwrap_or("world");
        let greeting = body["greeting"].as_str().unwrap_or("Hello");
        log::info!("composing greeting for {user}");
        EventEnvelope::new()
            .set_status(200)
            .set_body(serde_json::json!({
                "message": format!("{greeting}, {user}!"),
                "cid": body["cid"],
                "handled_by_instance": instance,
                "served_by": "hello-flow",
            }))
    }
}

/// The PROGRAMMATIC Event-over-HTTP demo task (the Rust twin of the Java
/// composable-example's `EventOverHttpRpc`): calls the peer's `hello.world`
/// function by passing the peer's Event API endpoint URL directly to the
/// request API. Because the target address is given programmatically,
/// `hello.world` does NOT appear in `event-over-http.yaml` — compare with
/// `hello.declarative` (an alias of the same peer function), which is
/// resolved through that configuration file instead. The two REST endpoints
/// `/api/event/http/programmatic` and `/api/event/http/declarative`
/// therefore hit the same peer function through the two different patterns.
///
/// The peer's `/api/event` endpoint is protected by the `event.api.auth`
/// demo — this task presents the shared token (resolved from the
/// DEMO_PEER_TOKEN environment variable via `demo.peer.token`) as a security
/// header on the HTTP request.
#[preload(route = "v1.event.over.http.rpc", instances = 10)]
struct EventOverHttpRpc;

#[async_trait]
impl ComposableFunction for EventOverHttpRpc {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let config = AppConfigReader::get_instance();
        let host = config.get_property_or("peer.demo.host", "127.0.0.1");
        let port = config.get_property_or("peer.demo.port", "8085");
        let endpoint = format!("http://{host}:{port}/api/event");
        // the peer's /api/event is protected by the event.api.auth demo —
        // present the shared token as a security header (never hard-coded;
        // "demo" is only the local-dev default inside ${DEMO_PEER_TOKEN:demo})
        let mut security_headers = HashMap::new();
        security_headers.insert(
            "authorization".to_string(),
            config.get_property_or("demo.peer.token", "demo"),
        );
        let mut request = EventEnvelope::new()
            .set_to("hello.world")
            .set_raw_body(input.body().clone());
        for (key, value) in &headers {
            request = request.set_header(key, value);
        }
        let po = PostOffice::new(&Platform::get_instance());
        let response = event_over_http_with_headers(
            &po,
            &endpoint,
            request,
            Duration::from_secs(10),
            true,
            &security_headers,
        )
        .await?;
        if response.status() != 200 {
            // surface the remote error to the flow's exception handler
            return Err(AppError::new(
                response.status(),
                response
                    .body_as::<String>()
                    .unwrap_or_else(|_| response.body().to_string()),
            ));
        }
        Ok(EventEnvelope::new().set_raw_body(response.body().clone()))
    }
}

/// A demo exception handler for the two Event-over-HTTP flows: formats the
/// flow engine's error dataset (`error.code`, `error.message`) as the HTTP
/// response (the Java composable-example's `v1.hello.exception` analog).
#[preload(route = "v1.hello.exception")]
struct HelloException;

#[async_trait]
impl ComposableFunction for HelloException {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        let status = body["status"].as_i64().unwrap_or(500);
        log::warn!(
            "demo exception handler: status={status}, message={}",
            body["message"]
        );
        EventEnvelope::new().set_body(serde_json::json!({
            "type": "error",
            "status": status,
            "message": body["message"],
        }))
    }
}

/// Health check honoring the actuator protocol (header `type=info`
/// describes the dependency; `type=health` reports live status) — here the
/// dependency IS the flow engine: healthy when flows are deployed.
#[preload(route = "flow.engine.health")]
struct FlowEngineHealth;

#[async_trait]
impl ComposableFunction for FlowEngineHealth {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        match headers.get("type").map(String::as_str) {
            Some("info") => EventEnvelope::new().set_body(serde_json::json!({
                "service": "event.script.engine",
                "href": "flow://hello-flow",
            })),
            Some("health") => {
                let flows = event_script::flows::get_all_flows();
                if flows.is_empty() {
                    Err(AppError::new(503, "no flows deployed"))
                } else {
                    EventEnvelope::new().set_body(format!(
                        "flow engine is running with {} flow{} deployed",
                        flows.len(),
                        if flows.len() == 1 { "" } else { "s" }
                    ))
                }
            }
            _ => Err(AppError::new(400, "unknown health request type")),
        }
    }
}

/// The main application: by the time it runs, the engine has compiled the
/// flows (CompileFlows at sequence 5) — announce what's deployed. Referencing
/// the event-script crate here also guarantees the linker keeps its
/// annotation inventory (the engine self-registers through it).
#[main_application]
struct HelloFlowApp;

#[async_trait]
impl EntryPoint for HelloFlowApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!(
            "Flows ready: {:?} — try: curl 'http://127.0.0.1:8100/api/hello/eric?lang=fr'",
            event_script::flows::get_all_flows()
        );
        Ok(())
    }
}

platform_core::auto_start_main!();
