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

//! The Active Knowledge Graph engine (layer 3) — Rust port of the Java
//! `minigraph-playground-engine`. A property graph whose nodes carry
//! executable skills: traversing the graph IS running the application.
//!
//! Builds on [`platform_core::graph::MiniGraph`](../platform_core/graph)
//! (layer 1) and exposes deployed graphs through event-script flows
//! (layer 2). This crate starts with the self-contained [`math`] expression
//! engine (increment K-2); the compiler, runtime and skills follow.
//!
//! Deliberately absent: `graph.js` — the Java engine embeds a GraalVM
//! JavaScript interpreter for lack of an alternative; an interpreter running
//! arbitrary user-supplied code is an attack surface this port retires
//! (maintainer decision, 2026-07-17). `graph.math` (typed, bounded) and
//! `graph.task` (reviewed, compiled functions) cover the use cases.

pub mod commands;
pub mod common;
pub mod compiler;
pub mod executor;
pub mod extension;
pub mod features;
pub mod fetcher;
pub mod graphs;
pub mod math;
pub mod mock;
pub mod model;
pub mod rest;
pub mod services;
pub mod session;
pub mod skills;
pub mod traveler;
pub mod ws_ui;

// the annotation layer (Java @FetchFeature analog)
pub use knowledge_graph_macros::fetch_feature;
// re-exported so the macro's generated `submit!` resolves without the user
// adding `inventory` as a direct dependency
pub use platform_core::inventory;

use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::{
    before_application, preload, AppError, ComposableFunction, EntryPoint, EventEnvelope, Platform,
};

/// The engine's bundled resources (graphs, flows, help, mock data, the
/// webapp bundle) — the Rust analog of a jar's classpath resources. Appended
/// (not prepended), so the application's own `resources/` always wins.
/// Runs early (sequence 1) so both the flow compiler (5) and the graph
/// compiler (6) can see `classpath:` resources that travel with this crate.
#[before_application(sequence = 1)]
pub struct GraphResources;

#[async_trait]
impl EntryPoint for GraphResources {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        platform_core::resources::append_resource_root(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources"
        ));
        // built-in API-fetcher features (Java ships them in the engine jar)
        features::register_builtins();
        // declarative #[fetch_feature] entries (the PlaygroundLoader scan analog)
        features::load_declared_features();
        Ok(())
    }
}

/// Java `GraphApiFetcher` (`graph.api.fetcher`) — dictionary/provider HTTP
/// fetch through the platform-core async HTTP client.
#[preload(route = "graph.api.fetcher", instances = 300)]
pub struct GraphApiFetcher;

#[async_trait]
impl ComposableFunction for GraphApiFetcher {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        fetcher::handle(&Platform::get_instance(), headers, input).await
    }
}

/// Java `GraphExtension` (`graph.extension`) — delegate to a sub-graph or a
/// `flow://` event-script flow.
#[preload(route = "graph.extension", instances = 300)]
pub struct GraphExtension;

#[async_trait]
impl ComposableFunction for GraphExtension {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        extension::handle(&Platform::get_instance(), headers, input).await
    }
}

/// The graph-model quality gate (Java `CompileGraph`,
/// `@BeforeApplication(sequence=6)`).
#[before_application(sequence = 6)]
pub struct CompileGraph;

#[async_trait]
impl EntryPoint for CompileGraph {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        compiler::compile_graphs();
        Ok(())
    }
}

/// The graph runtime (Java `GraphExecutor` — `@ZeroTracing @EventInterceptor
/// @PreLoad(route = "graph.executor", instances = 300)`).
#[preload(route = "graph.executor", instances = 300)]
#[zero_tracing]
#[event_interceptor]
pub struct GraphExecutorService;

#[async_trait]
impl ComposableFunction for GraphExecutorService {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        executor::handle(&Platform::get_instance(), headers, input).await
    }
}

/// Java `GraphDataMapper` (`graph.data.mapper`).
#[preload(route = "graph.data.mapper", instances = 300)]
pub struct GraphDataMapper;

#[async_trait]
impl ComposableFunction for GraphDataMapper {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        skills::data_mapper(&Platform::get_instance(), headers, input).await
    }
}

/// Java `GraphMath` (`graph.math`) — the retired `graph.js`'s sanctioned
/// replacement for inline compute/branching.
#[preload(route = "graph.math", instances = 300)]
pub struct GraphMath;

#[async_trait]
impl ComposableFunction for GraphMath {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        skills::math(&Platform::get_instance(), headers, input).await
    }
}

/// Java `GraphTask` (`graph.task`) — invoke a composable function by route.
#[preload(route = "graph.task", instances = 300)]
pub struct GraphTask;

#[async_trait]
impl ComposableFunction for GraphTask {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        skills::task(&Platform::get_instance(), headers, input).await
    }
}

/// Java `GraphJoin` (`graph.join`) — the fork-join barrier node.
#[preload(route = "graph.join", instances = 300)]
pub struct GraphJoin;

#[async_trait]
impl ComposableFunction for GraphJoin {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        skills::join(&Platform::get_instance(), headers, input).await
    }
}

/// Java `GraphIsland` (`graph.island`) — a terminal no-op branch.
#[preload(route = "graph.island", instances = 200)]
pub struct GraphIsland;

#[async_trait]
impl ComposableFunction for GraphIsland {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        skills::island(&Platform::get_instance(), headers, input).await
    }
}

/// Java `GraphHousekeeper` (`graph.housekeeper`, zero-tracing).
#[preload(route = "graph.housekeeper", instances = 20)]
#[zero_tracing]
pub struct GraphHousekeeper;

#[async_trait]
impl ComposableFunction for GraphHousekeeper {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        services::housekeeper(headers, input).await
    }
}

/// Java `GraphExceptionHandler` (`graph.exception.handler`).
#[preload(route = "graph.exception.handler")]
pub struct GraphExceptionHandler;

#[async_trait]
impl ComposableFunction for GraphExceptionHandler {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        services::exception_handler(headers, input).await
    }
}

// --- The MiniGraph Playground (the Java `@OptionalService("app.env=dev")`
// services + the `@WebSocketService` handlers in `ws_ui`). Each Playground
// service registers only when `app.env=dev` (via `#[optional_service]`); in any
// other environment none of it exists — production graphs run only through
// `POST /api/graph/{graph-id}`. The home page (`get.index.html`) is registered
// regardless (it serves `/template` outside dev), matching Java's non-optional
// `GetIndexHtml`.

/// Dev-only: start the Playground's temp-graph housekeeping sweep. A
/// `#[before_application]` gated by `#[optional_service]`. This hook plus the
/// dev-gated services below (each `#[optional_service("app.env=dev")]`)
/// declaratively replace the former programmatic Playground registration —
/// mirroring how Java registers these via `@PreLoad`/`@WebSocketService` +
/// `@OptionalService` rather than through a loader.
#[before_application(sequence = 8)]
#[optional_service("app.env=dev")]
struct PlaygroundHousekeeping;

#[async_trait]
impl EntryPoint for PlaygroundHousekeeping {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        commands::start_housekeeping();
        log::info!("Playground loaded (app.env=dev)");
        Ok(())
    }
}

/// Java `GetIndexHtml` (`get.index.html`) — the home page; registered in **all**
/// environments (serves `/template` outside dev).
#[preload(route = "get.index.html", instances = 10)]
pub struct GetIndexHtml;

#[async_trait]
impl ComposableFunction for GetIndexHtml {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        rest::get_index_html(input).await
    }
}

/// Java `GraphCommandService` (`graph.command.service`) — the Playground command grammar.
#[preload(route = "graph.command.service", instances = 50)]
#[optional_service("app.env=dev")]
pub struct GraphCommandServiceFn;

#[async_trait]
impl ComposableFunction for GraphCommandServiceFn {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        commands::handle(&Platform::get_instance(), HashMap::new(), input).await
    }
}

/// The singleton command handler (orderly AI-companion requests).
#[preload(route = "graph.command.singleton", instances = 1)]
#[optional_service("app.env=dev")]
pub struct GraphCommandSingleton;

#[async_trait]
impl ComposableFunction for GraphCommandSingleton {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        commands::handle(&Platform::get_instance(), HashMap::new(), input).await
    }
}

/// Java `GraphTraveler` (`graph.traveler`) — the dev-only graph walker
/// (zero-tracing event interceptor).
#[preload(route = "graph.traveler", instances = 300)]
#[zero_tracing]
#[event_interceptor]
#[optional_service("app.env=dev")]
pub struct GraphTravelerService;

#[async_trait]
impl ComposableFunction for GraphTravelerService {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        traveler::handle(&Platform::get_instance(), headers, input).await
    }
}

/// Java `GetWsHtml` (`get.ws.html`) — the raw websocket workbench pages.
#[preload(route = "get.ws.html", instances = 10)]
#[optional_service("app.env=dev")]
pub struct GetWsHtml;

#[async_trait]
impl ComposableFunction for GetWsHtml {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        rest::get_ws_html(input).await
    }
}

/// Java `PostCompanionCommand` (`post.companion.command`) — the AI-companion hop.
#[preload(route = "post.companion.command", instances = 10)]
#[optional_service("app.env=dev")]
pub struct PostCompanionCommand;

#[async_trait]
impl ComposableFunction for PostCompanionCommand {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        rest::post_companion_command(&Platform::get_instance(), input).await
    }
}

/// Java `UploadJsonContent` (`upload.json.content`).
#[preload(route = "upload.json.content", instances = 10)]
#[optional_service("app.env=dev")]
pub struct UploadJsonContent;

#[async_trait]
impl ComposableFunction for UploadJsonContent {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        rest::upload_json_content(&Platform::get_instance(), input).await
    }
}

/// Java `UploadMockContent` (`upload.mock.content`).
#[preload(route = "upload.mock.content", instances = 10)]
#[optional_service("app.env=dev")]
pub struct UploadMockContent;

#[async_trait]
impl ComposableFunction for UploadMockContent {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        rest::upload_mock_content(&Platform::get_instance(), input).await
    }
}

/// Java `DescribeGraph` (`show.graph.model`).
#[preload(route = "show.graph.model", instances = 20)]
#[optional_service("app.env=dev")]
pub struct ShowGraphModel;

#[async_trait]
impl ComposableFunction for ShowGraphModel {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        rest::show_graph_model(input).await
    }
}

/// Java `GetLiveGraph` (`get.live.graph`).
#[preload(route = "get.live.graph", instances = 10)]
#[optional_service("app.env=dev")]
pub struct GetLiveGraph;

#[async_trait]
impl ComposableFunction for GetLiveGraph {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        rest::get_live_graph(input).await
    }
}

/// Java `InspectStateMachine` (`inspect.state.machine`).
#[preload(route = "inspect.state.machine", instances = 10)]
#[optional_service("app.env=dev")]
pub struct InspectStateMachine;

#[async_trait]
impl ComposableFunction for InspectStateMachine {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        rest::inspect_state_machine(input).await
    }
}

/// Java `GraphHealth` (`graph.health`) — the template health service.
#[preload(route = "graph.health", instances = 10)]
pub struct GraphHealth;

#[async_trait]
impl ComposableFunction for GraphHealth {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        services::health(headers, input).await
    }
}
