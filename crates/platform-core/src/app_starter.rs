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

//! Rust port of the Java application lifecycle — `AutoStart` + `AppStarter`
//! (`org.platformlambda.core.system`) and the `EntryPoint` contract
//! (`org.platformlambda.core.models.EntryPoint`).
//!
//! Startup order is the Java sequence exactly:
//!
//! 1. **Essential services** (the Java `EssentialServiceLoader`, sequence 0 —
//!    reserved for the framework): here, the elastic store's housekeeping
//!    (holding-area liveness marker, keep-alive refresh, expired-store scan).
//! 2. **Before-application hooks** (`@BeforeApplication`), ordered by
//!    `sequence` (1–999, lower first; user code conventionally uses 3–999) —
//!    validation/compilation work before anything is registered or served
//!    (e.g. event-script's `CompileFlows` in Java). A failure **aborts
//!    startup** (Java parity: fatal).
//! 3. **Preload** (`@PreLoad`): composable functions are registered and bound
//!    to their routes — callable via `PostOffice` from this point on.
//! 4. The HTTP server would start here when `rest.automation=true` — REST
//!    automation is a later increment; the flag currently logs a notice.
//! 5. **Main applications** (`@MainApplication`), ordered by `sequence`.
//!    A missing main application is an error (Java parity).
//!
//! Java discovers all five phases by classpath annotation scanning; Rust has
//! no runtime scanning (design D6), so [`AppStarter`] is an explicit
//! **builder** — a `#[preload]`-style proc-macro can add the ergonomic layer
//! in a later increment. Java's `AutoStart` global run-once guard is not
//! ported: the builder is consumed by [`run`](AppStarter::run), and the
//! framework phases behind it are idempotent (deliberate divergence,
//! test-friendly).

use std::sync::Arc;

use async_trait::async_trait;

use crate::function::{AppError, ComposableFunction};
use crate::platform::{FunctionOptions, Platform};
use crate::util::app_config_reader::AppConfigReader;
use crate::util::elastic_queue;

/// Highest allowed hook sequence (Java `MAX_SEQ`); larger values clamp to it.
const MAX_SEQ: u32 = 999;

/// The application entry-point contract (Java `EntryPoint`): implemented by
/// both before-application hooks and main applications.
#[async_trait]
pub trait EntryPoint: Send + Sync {
    async fn start(&self, args: &[String]) -> Result<(), AppError>;
}

/// The lifecycle builder (Java `AutoStart`/`AppStarter`, explicit-registration
/// form). Build the application declaratively, then [`run`](Self::run) it:
///
/// ```ignore
/// AppStarter::new()
///     .before_application(5, Arc::new(CompileCheck))
///     .preload("greeting.demo", TypedAdapter::arc(Greetings), 10)
///     .main_application(1, Arc::new(MainApp))
///     .run(std::env::args().collect())
///     .await?;
/// ```
#[derive(Default)]
pub struct AppStarter {
    before: Vec<(u32, Arc<dyn EntryPoint>)>,
    preloads: Vec<(String, Arc<dyn ComposableFunction>, usize, FunctionOptions)>,
    mains: Vec<(u32, Arc<dyn EntryPoint>)>,
}

impl AppStarter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a before-application hook (Java `@BeforeApplication`).
    /// `sequence` orders execution (lower first, clamped to 999; 0 is reserved
    /// for the framework's essential services — user code uses 1–999,
    /// conventionally 3–999).
    pub fn before_application(mut self, sequence: u32, hook: Arc<dyn EntryPoint>) -> Self {
        self.before.push((sequence.min(MAX_SEQ), hook));
        self
    }

    /// Register a composable function at startup (Java `@PreLoad`).
    pub fn preload(
        self,
        route: &str,
        function: Arc<dyn ComposableFunction>,
        instances: usize,
    ) -> Self {
        self.preload_with_options(route, function, instances, FunctionOptions::default())
    }

    /// Register a composable function with explicit options (Java `@PreLoad`
    /// combined with `@ZeroTracing` and/or `@EventInterceptor`).
    pub fn preload_with_options(
        mut self,
        route: &str,
        function: Arc<dyn ComposableFunction>,
        instances: usize,
        options: FunctionOptions,
    ) -> Self {
        self.preloads
            .push((route.to_string(), function, instances, options));
        self
    }

    /// Register a main application (Java `@MainApplication`); `sequence`
    /// orders execution when there is more than one (lower first).
    pub fn main_application(mut self, sequence: u32, entry: Arc<dyn EntryPoint>) -> Self {
        self.mains.push((sequence.min(MAX_SEQ), entry));
        self
    }

    /// Run the lifecycle (Java `AutoStart.main` → `AppStarter.main`).
    /// Must be called within a Tokio runtime.
    pub async fn run(self, args: Vec<String>) -> Result<(), AppError> {
        // -Dkey=value runtime arguments (the JVM -D analog) become process
        // overrides — idempotent, in case logging::init already loaded them
        crate::util::overrides::load_runtime_args();
        let config = AppConfigReader::get_instance();
        log::info!(
            "Starting application {} (platform-core v{})",
            Platform::name(),
            env!("CARGO_PKG_VERSION")
        );
        // 1. essential services (sequence 0, framework-reserved — the Java
        //    EssentialServiceLoader): elastic-store housekeeping + the
        //    distributed-tracing telemetry sink (idempotent across runs)
        elastic_queue::start_housekeeping();
        let platform = Platform::get_instance();
        if !platform.has_route(crate::telemetry::DISTRIBUTED_TRACING) {
            if let Err(e) = platform.register(
                crate::telemetry::DISTRIBUTED_TRACING,
                std::sync::Arc::new(crate::telemetry::Telemetry::new(&platform)),
                1,
            ) {
                // a concurrent lifecycle run may have won the race — benign;
                // anything else is a real startup failure
                if !platform.has_route(crate::telemetry::DISTRIBUTED_TRACING) {
                    return Err(e);
                }
            }
        }
        // actuator endpoints (Java EssentialServiceLoader parity): /info /env
        // /health /livenessprobe, exposed via REST automation's default routes
        if !platform.has_route(crate::actuator::INFO_ACTUATOR) {
            use crate::actuator::{ActuatorContext, ActuatorKind, ActuatorServices};
            let context = ActuatorContext::new(&platform);
            let actuators = [
                (crate::actuator::INFO_ACTUATOR, ActuatorKind::Info),
                (crate::actuator::ENV_ACTUATOR, ActuatorKind::Env),
                (crate::actuator::HEALTH_ACTUATOR, ActuatorKind::Health),
                (crate::actuator::LIVENESS_ACTUATOR, ActuatorKind::Liveness),
            ];
            for (route, kind) in actuators {
                if let Err(e) = platform.register(
                    route,
                    Arc::new(ActuatorServices::new(kind, context.clone())),
                    1,
                ) {
                    if !platform.has_route(route) {
                        return Err(e);
                    }
                }
            }
        }
        // the no-op echo function (Java NoOpFunction, a platform built-in
        // for event scripts): echoes headers and body; instance count is
        // overridable via worker.instances.no.op (Java envInstances parity)
        if !platform.has_route("no.op") {
            let config = AppConfigReader::get_instance();
            let no_op_instances = config
                .get_property("worker.instances.no.op")
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(500);
            if let Err(e) = platform.register(
                "no.op",
                Arc::new(crate::function::NoOpFunction),
                no_op_instances,
            ) {
                if !platform.has_route("no.op") {
                    return Err(e);
                }
            }
        }
        // the Async HTTP client (Java EssentialServiceLoader parity);
        // tolerate a concurrent registration like the actuators above
        if !platform.has_route(crate::automation::ASYNC_HTTP_REQUEST) {
            if let Err(e) = platform.register_with_options(
                crate::automation::ASYNC_HTTP_REQUEST,
                Arc::new(crate::automation::http_client::AsyncHttpClientService),
                500,
                FunctionOptions {
                    zero_traced: false,
                    interceptor: true,
                },
            ) {
                if !platform.has_route(crate::automation::ASYNC_HTTP_REQUEST) {
                    return Err(e);
                }
            }
        }
        // 2. before-application hooks, ordered by sequence (stable sort keeps
        //    registration order within a sequence — Java scan order analog)
        let mut before = self.before;
        before.sort_by_key(|(sequence, _)| *sequence);
        for (sequence, hook) in before {
            hook.start(&args).await.map_err(|e| {
                AppError::new(
                    e.status(),
                    format!(
                        "BeforeApplication (sequence {sequence}) failed: {}",
                        e.message()
                    ),
                )
            })?;
        }
        // 3. preload: bind composable functions to their routes
        for (route, function, instances, options) in self.preloads {
            platform.register_with_options(&route, function, instances, options)?;
            log::info!(
                "{route} with {instances} instance{} started",
                if instances == 1 { "" } else { "s" }
            );
        }
        // 4. REST automation: the HTTP protocol boundary. Websocket services
        //    from the `#[websocket_service]` inventory register their URL
        //    paths first (Java `prepareWebsocketServices`); the server starts
        //    when REST automation is enabled OR any websocket service exists
        //    (Java `startHttpServerIfAny` parity).
        for entry in inventory::iter::<crate::registry::WsServiceEntry> {
            if validate_ws_service_name(entry.name) && validate_ws_service_name(entry.namespace) {
                crate::automation::ws_server::register_ws_service_with_namespace(
                    entry.namespace,
                    entry.name,
                    entry.factory,
                );
            } else {
                log::error!(
                    "Unable to load websocket service /{}/{} - not a valid service name",
                    entry.namespace,
                    entry.name
                );
            }
        }
        let serve_http = config.get_property_or("rest.automation", "false") == "true"
            || crate::automation::ws_server::has_ws_services();
        if serve_http {
            crate::automation::start_http_server(&platform).await?;
        }
        // 5. main applications, ordered by sequence
        if self.mains.is_empty() {
            // Java: "Missing MainApplication ... Did you forget to annotate your main module?"
            return Err(AppError::new(
                400,
                "Missing main application - did you forget to add one with main_application()?",
            ));
        }
        let mut mains = self.mains;
        mains.sort_by_key(|(sequence, _)| *sequence);
        for (sequence, entry) in mains {
            entry.start(&args).await.map_err(|e| {
                AppError::new(
                    e.status(),
                    format!(
                        "MainApplication (sequence {sequence}) failed: {}",
                        e.message()
                    ),
                )
            })?;
        }
        Ok(())
    }
}

/// The one-line application entry point (Java `AutoStart.main(args)`): loads
/// the `-D` runtime overrides, installs the logger, **collects every
/// annotated item from the link-time inventory** (`#[preload]`,
/// `#[before_application]`, `#[main_application]` — the classpath-scanning
/// analog), and runs the lifecycle. A standalone `fn main()` uses
/// [`AutoStart::run`], which then keeps serving until Ctrl-C; an embedder
/// (tests, an existing async context) calls [`AutoStart::main`], which returns
/// once the app is booted (the HTTP/websocket accept loop runs in the
/// background).
pub struct AutoStart;

impl AutoStart {
    /// Build a Tokio runtime, boot the application, then — when serving (REST
    /// automation on, or any websocket service registered) — stay alive until
    /// Ctrl-C (Java: the JVM stays up on non-daemon threads). This is the whole
    /// `fn main()` body; the `auto_start_main!` macro wraps exactly this plus
    /// the app's resource root.
    pub fn run() -> Result<(), AppError> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| AppError::new(500, format!("Unable to start runtime: {e}")))?;
        runtime.block_on(async {
            Self::main(std::env::args().collect()).await?;
            // HTTP/websocket serving: stay alive until Ctrl-C. This blocking
            // wait lives here (the standalone-process entry point), NOT in
            // `main`, so an embedder that awaits `main` gets control back once
            // the app is booted instead of hanging on the signal.
            let config = AppConfigReader::get_instance();
            if config.get_property_or("rest.automation", "false") == "true"
                || crate::automation::ws_server::has_ws_services()
            {
                log::info!("Application running - press Ctrl-C to stop");
                let _ = tokio::signal::ctrl_c().await;
            } else {
                // give fire-and-forget telemetry a beat to be logged before exit
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
            crate::util::elastic_queue::shutdown_cleanup();
            Ok(())
        })
    }

    /// The async lifecycle (must run within a Tokio runtime): run every
    /// before-application hook, bind the preloads, start the HTTP/websocket
    /// server when serving — then **return** (the accept loop keeps running as
    /// a background task). Booting the engine hands control back to the caller;
    /// a standalone process uses [`AutoStart::run`] to serve until Ctrl-C.
    ///
    /// Runs only once per process (Java parity: `AutoStart.started` is an
    /// `AtomicBoolean`) — repeated execution is a no-op.
    pub async fn main(args: Vec<String>) -> Result<(), AppError> {
        static STARTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if STARTED.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return Ok(());
        }
        crate::util::overrides::load_runtime_args();
        crate::logging::init();
        let mut starter = AppStarter::new();
        for entry in inventory::iter::<crate::registry::BeforeAppEntry> {
            starter = starter.before_application(entry.sequence, (entry.factory)());
        }
        let config = AppConfigReader::get_instance();
        for entry in inventory::iter::<crate::registry::PreloadEntry> {
            // Java @OptionalService: skip a conditionally-registered route when
            // its configuration condition does not hold (Java `Feature`).
            if !crate::util::feature::is_required(entry.optional_service, config) {
                log::info!(
                    "Skip optional {} (condition: {})",
                    entry.route,
                    entry.optional_service.unwrap_or_default()
                );
                continue;
            }
            // Java envInstances: the instance count may come from configuration
            let instances = entry
                .env_instances
                .and_then(|key| config.get_property(key))
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(entry.instances);
            starter = starter.preload_with_options(
                entry.route,
                (entry.factory)(),
                instances,
                FunctionOptions {
                    zero_traced: entry.zero_tracing,
                    interceptor: entry.interceptor,
                },
            );
        }
        for entry in inventory::iter::<crate::registry::MainAppEntry> {
            starter = starter.main_application(entry.sequence, (entry.factory)());
        }
        starter.run(args).await?;
        // The app is booted; the HTTP/websocket accept loop (if serving) runs
        // as a background task. Return control to the caller — `AutoStart::run`
        // is what blocks a standalone process alive until Ctrl-C.
        Ok(())
    }
}

/// Java `validServiceName` for websocket paths: lowercase letters, digits,
/// '.', '-', '_'.
fn validate_ws_service_name(name: &str) -> bool {
    !name.is_empty()
        && name.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'-' | b'_')
        })
}
