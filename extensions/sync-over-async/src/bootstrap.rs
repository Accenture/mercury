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

//! Autoloads the cross-pod return-route engine at startup (Java
//! `SyncOverAsyncAutoStart`, a `@MainApplication` gated by
//! `@OptionalService("sync.over.async.enabled")`): after every composable
//! function is registered, when `sync.over.async.enabled=true` the hook reads
//! the `soa.redis.*` connection parameters and the `sync.*` tunables from
//! configuration, connects the [`ReturnRouteCoordinator`], starts its
//! subscriber and publishes it through [`runtime`], so the facade tasks
//! (`sync.prepare` / `sync.await` / `soa.reply`) and any `StreamBridge` facade
//! find it. It also registers the `soa.redis.health` probe on every pod (the
//! Java `@PreLoad SoaRedisHealthCheck`), unless the application registered it
//! already.
//!
//! An application that builds its own coordinator (a test, a custom topology)
//! is respected: a coordinator already installed in the runtime is left alone.
//! Collected from this library by the application's `auto_start_main!()` —
//! the Java classpath-scan parity — with the same one-line linker caveat as
//! every Rust library of composable functions: an application that references
//! no symbol of this crate must still link it (`use mercury_sync_over_async as
//! _;`), or the inventory entries are dropped as an unused dependency.
//!
//! Sequence 15 keeps it after a typical application entry point (default 10)
//! and before the Kafka flow adapter's start (20), so a reply consumed the
//! moment its binding starts already finds the coordinator.

use async_trait::async_trait;
use platform_core::{main_application, AppConfigReader, AppError, EntryPoint, Platform};

use crate::config::SyncOverAsyncConfig;
use crate::connection::RedisSettings;
use crate::coordinator::ReturnRouteCoordinator;
use crate::health::{RedisHealthCheck, REDIS_HEALTH_ROUTE};
use crate::runtime;

/// The feature switch (Java parity): only the literal `true` starts the coordinator.
pub const ENABLED_KEY: &str = "sync.over.async.enabled";

/// The library's startup hook — see the module documentation.
#[main_application(sequence = 15)]
#[derive(Default)]
pub struct SyncOverAsyncAutoStart;

#[async_trait]
impl EntryPoint for SyncOverAsyncAutoStart {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        let platform = Platform::get_instance();
        if !platform.has_route(REDIS_HEALTH_ROUTE) {
            RedisHealthCheck::from_config().register(&platform)?;
        }
        let config = AppConfigReader::get_instance();
        if !config
            .get_property_or(ENABLED_KEY, "false")
            .trim()
            .eq_ignore_ascii_case("true")
        {
            log::info!("{ENABLED_KEY} is not true; no return-route coordinator on this pod");
            return Ok(());
        }
        if runtime::coordinator().is_some() {
            log::info!("Return-route coordinator already installed by the application");
            return Ok(());
        }
        let settings = RedisSettings::from_config();
        let coordinator = runtime::init(
            &settings,
            Platform::origin(),
            SyncOverAsyncConfig::from_config(),
        )
        .await?;
        log::info!(
            "Return-route coordinator started for pod {} (redis {}:{}, ssl={}, channel {})",
            Platform::origin(),
            settings.host(),
            settings.port(),
            settings.ssl(),
            coordinator.return_channel()
        );
        // the subscriber connection is released with the process (the Java
        // SyncRuntime.shutdown() analog runs from the platform's hook)
        platform.on_shutdown(runtime::shutdown);
        Ok(())
    }
}

// keep the type in scope for the docs link above
#[allow(dead_code)]
fn _coordinator_type(_: &ReturnRouteCoordinator) {}
