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

//! # event-script (Rust port)
//!
//! Rust port of mercury-composable's `event-script-engine` — layer 2,
//! **composable orchestration**: a YAML DSL describing an end-to-end
//! transaction as a *flow* that choreographs composable functions by route
//! name, executed over the platform-core event bus with a per-transaction
//! state machine. Canonical behavior spec: the Java project
//! (`system/event-script-engine`, v4.8.6) and its
//! `docs/guides/event-script/flow-grammar.md`; design:
//! `docs/design/event-script-port.md` (this repo).
//!
//! **Increment E-1 — flow model + compiler:** [`model`] (compiled
//! `Flow`/`Task`), [`flows`] (the template registry), [`compiler`]
//! (`yaml.flow.automation` discovery + the full grammar validation),
//! [`converter`] (legacy `:type` → `f:plugin(...)` rewriting), [`validator`]
//! (mapping-syntax + reserved-key checks) and [`plugins`].
//!
//! **Increment E-2 — data-mapping engine:** [`mlm`] (the runtime
//! `MultiLevelMap` over the state-machine tree — direct composite-key
//! traversal is the primary access path, `$.…` delegates to a JSONPath
//! engine for user-defined complex queries), [`mapping`] (LHS/RHS
//! resolution: constants, plugin calls, legacy `:type` commands,
//! `{model.key}` interpolation, `file()`/`classpath()` content),
//! [`conversions`] (type-conversion utilities) and executable bodies for
//! the core conversion/logical plugins (the remaining built-ins + the
//! `#[simple_plugin]` macro arrive with increment E-8). The runtime
//! (state machine, task executor, adapters) arrives with E-3+.
//!
//! Linking this crate makes the engine self-register through the
//! platform-core annotation inventory: [`CompileFlows`] runs as a
//! before-application hook at sequence 5 (essential services 0, plugins 3,
//! flows 5, user code ≥ 6 — the Java sequence contract).

pub mod adapter;
pub mod compiler;
pub mod conversions;
pub mod converter;
pub mod executor;
pub mod flows;
pub mod instance;
pub mod manager;
pub mod mapping;
pub mod mlm;
pub mod mock;
pub mod model;
pub mod plugins;
pub mod resilience;
pub mod util;
pub mod validator;

pub use executor::FlowExecutor;
pub use mock::EventScriptMock;
// re-exported so the #[simple_plugin] macro's generated code resolves
pub use event_script_macros::simple_plugin;
pub use platform_core::inventory;

use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::{
    before_application, preload, AppError, ComposableFunction, EntryPoint, EventEnvelope, Platform,
};

/// The flow compiler hook (Java `CompileFlows`, `@BeforeApplication(sequence=5)`).
#[before_application(sequence = 5)]
pub struct CompileFlows;

#[async_trait]
impl EntryPoint for CompileFlows {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        compiler::compile_flows();
        Ok(())
    }
}

/// The flow-engine entry point (Java `EventScriptManager`,
/// `@EventInterceptor @PreLoad(route = "event.script.manager")`).
#[preload(route = "event.script.manager")]
#[event_interceptor]
pub struct EventScriptManager;

#[async_trait]
impl ComposableFunction for EventScriptManager {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        manager::handle(&Platform::get_instance(), headers, input).await
    }
}

/// The task executor (Java `TaskExecutor`,
/// `@EventInterceptor @PreLoad(route = "task.executor")`).
#[preload(route = "task.executor")]
#[event_interceptor]
pub struct TaskExecutorService;

#[async_trait]
impl ComposableFunction for TaskExecutorService {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        executor::handle(&Platform::get_instance(), headers, input).await
    }
}

/// The generic exception-handler service (Java `SimpleExceptionHandler`,
/// `simple.exception.handler`): logs the error context and echoes
/// `{type: error, status, message}` — the shape flows map into their
/// responses.
#[preload(route = "simple.exception.handler", instances = 250)]
pub struct SimpleExceptionHandler;

#[async_trait]
impl ComposableFunction for SimpleExceptionHandler {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let view = crate::mlm::MultiLevelMap::from_value(input.body().clone());
        let (Some(status), Some(message)) =
            (view.get_element("status"), view.get_element("message"))
        else {
            return Ok(EventEnvelope::new().set_raw_body(rmpv::Value::Map(Vec::new())));
        };
        let task = view
            .get_element("task")
            .map(|v| crate::conversions::display(&v))
            .unwrap_or_else(|| "previous task".to_string());
        log::error!(
            "User defined exception handler received from {task}, rc={}, error={}",
            crate::conversions::display(&status),
            crate::conversions::display(&message)
        );
        Ok(EventEnvelope::new().set_raw_body(rmpv::Value::Map(vec![
            (rmpv::Value::from("type"), rmpv::Value::from("error")),
            (rmpv::Value::from("status"), status),
            (rmpv::Value::from("message"), message),
        ])))
    }
}

/// The plugin loader (Java `SimplePluginLoader`,
/// `@BeforeApplication(sequence = 3)` — before flows compile at sequence 5,
/// so `f:` names in mappings validate against user plugins too).
#[before_application(sequence = 3)]
pub struct SimplePluginLoader;

#[async_trait]
impl EntryPoint for SimplePluginLoader {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        let user = plugins::load_inventory_plugins();
        log::info!("Total {user} user plugin(s) registered");
        Ok(())
    }
}

/// The HTTP flow adapter (Java `HttpToFlow`,
/// `@EventInterceptor @PreLoad(route = "http.flow.adapter", instances = 200)`).
#[preload(route = "http.flow.adapter", instances = 200)]
#[event_interceptor]
pub struct HttpToFlow;

#[async_trait]
impl ComposableFunction for HttpToFlow {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        adapter::handle(&Platform::get_instance(), headers, input).await
    }
}

/// The resilience handler (Java `Resilience4Flow`,
/// `@EventInterceptor @PreLoad(route = "resilience.handler", instances = 500)`).
#[preload(route = "resilience.handler", instances = 500)]
#[event_interceptor]
pub struct Resilience4Flow;

#[async_trait]
impl ComposableFunction for Resilience4Flow {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        resilience::handle(&Platform::get_instance(), headers, input).await
    }
}
