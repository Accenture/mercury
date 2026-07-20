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
//! `resources/flows/hello-flow.yml` — the two functions below are the only
//! code, and neither knows the other exists (route-name + envelope coupling
//! only). Linking the `event-script` crate self-registers the flow engine
//! through the annotation inventory; `rest.yaml` binds the endpoint to the
//! flow with `flow: 'hello-flow'`.
//!
//! ```bash
//! cargo run -p hello-flow
//! curl 'http://127.0.0.1:8086/api/hello/eric?lang=fr'
//! curl 'http://127.0.0.1:8086/api/hello/eric?lang=en'
//! ```

use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::{
    main_application, preload, AppError, ComposableFunction, EntryPoint, EventEnvelope,
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
            "Flows ready: {:?} — try: curl 'http://127.0.0.1:8086/api/hello/eric?lang=fr'",
            event_script::flows::get_all_flows()
        );
        Ok(())
    }
}

platform_core::auto_start_main!();
