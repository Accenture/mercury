// Scaffolded from Mercury's starter templates (https://github.com/Accenture/mercury, Apache-2.0)

//! Layer 2 starter — an HTTP endpoint launches an Event Script flow that
//! sequences two decoupled functions with declarative data mapping.
//! Orchestration lives in resources/flows/greeting-flow.yml, never in code.
//! Copy this template out of the Mercury repository to begin a new project
//! (see README.md and AGENTS.md).

use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::{
    main_application, preload, AppError, ComposableFunction, EntryPoint, EventEnvelope,
};

/// First task of greeting-flow: reject a request without a name. The flow's
/// input data mapping delivers 'input.body.name' as the "name" key; business
/// logic stays here, never in the flow YAML.
#[preload(route = "v1.validate.request", instances = 10)]
struct ValidateRequest;

#[async_trait]
impl ComposableFunction for ValidateRequest {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as()?;
        let name = body["name"].as_str().unwrap_or("").trim().to_string();
        if name.is_empty() {
            return Err(AppError::new(
                400,
                "Missing 'name' - send a JSON body {\"name\": \"...\"}",
            ));
        }
        EventEnvelope::new().set_body(serde_json::json!({ "name": name }))
    }
}

/// Second task of greeting-flow: compose the response. It knows nothing about
/// the validator - the flow's state machine ('model.name') carries data
/// between the decoupled tasks.
#[preload(route = "v1.make.greeting", instances = 10)]
struct MakeGreeting;

#[async_trait]
impl ComposableFunction for MakeGreeting {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as()?;
        let name = body["name"].as_str().unwrap_or("");
        EventEnvelope::new().set_body(serde_json::json!({
            "greeting": format!("Hello, {name}"),
            "served_by": "v1.make.greeting",
        }))
    }
}

#[main_application]
struct StarterApp;

#[async_trait]
impl EntryPoint for StarterApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        // one-time application setup goes here; functions and flows are already
        // registered (referencing the flow registry also anchors the event-script
        // crate into the link-time inventory)
        log::info!(
            "starter-flow ready: {} flow(s) compiled",
            event_script::flows::get_all_flows().len()
        );
        Ok(())
    }
}

platform_core::auto_start_main!();
