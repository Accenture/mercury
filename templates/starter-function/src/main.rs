// Scaffolded from Mercury's starter templates (https://github.com/Accenture/mercury, Apache-2.0)

//! Layer 1 starter — one composable function exposed over HTTP by REST
//! automation. Copy this template out of the Mercury repository to begin a
//! new project (see README.md and AGENTS.md).

use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::automation::AsyncHttpRequest;
use platform_core::{main_application, preload, AppError, EntryPoint, TypedFunction};

/// The starter's single composable function. It is addressed only by its
/// route name ("v1.greeting") - rest.yaml maps /api/greeting to it, and the
/// same function could join an Event Script flow or a knowledge graph
/// without a code change. The typed AsyncHttpRequest input carries the HTTP
/// request from the REST edge (query, path, headers, body).
#[preload(route = "v1.greeting", instances = 10, typed)]
struct Greeting;

#[async_trait]
impl TypedFunction<AsyncHttpRequest, serde_json::Value> for Greeting {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        request: AsyncHttpRequest,
        instance: usize,
    ) -> Result<serde_json::Value, AppError> {
        let name = request
            .query_parameter("name")
            .or_else(|| {
                request
                    .body_as::<serde_json::Value>()
                    .ok()
                    .and_then(|body| body["name"].as_str().map(str::to_string))
            })
            .unwrap_or_default();
        let name = name.trim();
        if name.is_empty() {
            return Err(AppError::new(
                400,
                "Missing 'name' - send ?name=... or a JSON body {\"name\": \"...\"}",
            ));
        }
        Ok(serde_json::json!({
            "greeting": format!("Hello, {name}"),
            "served_by": "v1.greeting",
            "instance": instance,
        }))
    }
}

#[main_application]
struct StarterApp;

#[async_trait]
impl EntryPoint for StarterApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        // one-time application setup goes here; functions are already registered
        log::info!("Started");
        Ok(())
    }
}

platform_core::auto_start_main!();
