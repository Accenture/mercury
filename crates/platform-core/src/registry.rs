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

//! Link-time registration inventory — the Rust analog of Java's classpath
//! annotation scanning (design D6's ergonomic layer). The attribute macros
//! (`#[preload]`, `#[before_application]`, `#[main_application]`) submit
//! entries here at link time; [`AutoStart`](crate::app_starter::AutoStart)
//! collects them at startup, exactly as Java's `AppStarter` scans for its
//! annotations.

use std::sync::Arc;

use crate::function::ComposableFunction;

/// A `#[preload]`-annotated composable function (Java `@PreLoad`).
pub struct PreloadEntry {
    pub route: &'static str,
    pub instances: usize,
    /// Optional configuration key for the instance count (Java `envInstances`);
    /// `instances` is the fallback.
    pub env_instances: Option<&'static str>,
    /// Java `@OptionalService`: a configuration condition (e.g. `"app.env=dev"`)
    /// that must hold at startup for this route to register; `None` = always
    /// register. Evaluated by [`crate::util::feature::is_required`].
    pub optional_service: Option<&'static str>,
    /// Java `@ZeroTracing`: this route's executions are excluded from
    /// distributed-trace recording.
    pub zero_tracing: bool,
    /// Java `@EventInterceptor`: manual replies, no auto-reply on success.
    pub interceptor: bool,
    pub factory: fn() -> Arc<dyn ComposableFunction>,
}

/// A `#[websocket_service]`-annotated websocket handler
/// (Java `@WebSocketService(value, namespace)`).
pub struct WsServiceEntry {
    /// The service name — the URL becomes `/{namespace}/{name}/{token}`.
    pub name: &'static str,
    /// Java `namespace()` default: "ws".
    pub namespace: &'static str,
    /// One function object per websocket connection.
    pub factory: fn() -> Arc<dyn ComposableFunction>,
}

/// A `#[before_application]`-annotated entry point (Java `@BeforeApplication`).
pub struct BeforeAppEntry {
    pub sequence: u32,
    pub factory: fn() -> Arc<dyn crate::app_starter::EntryPoint>,
}

/// A `#[main_application]`-annotated entry point (Java `@MainApplication`).
pub struct MainAppEntry {
    pub sequence: u32,
    pub factory: fn() -> Arc<dyn crate::app_starter::EntryPoint>,
}

inventory::collect!(PreloadEntry);
inventory::collect!(WsServiceEntry);
inventory::collect!(BeforeAppEntry);
inventory::collect!(MainAppEntry);
