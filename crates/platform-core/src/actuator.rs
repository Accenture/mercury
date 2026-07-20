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

//! Actuator endpoints — Rust port of the Java `ActuatorServices`
//! (`org.platformlambda.core.services.ActuatorServices`), registered by the
//! lifecycle's essential-services phase and exposed over REST automation via
//! the default endpoints (`/info`, `/env`, `/health`, `/livenessprobe`).
//!
//! - **`/info`** — application identity (name, version, description), runtime,
//!   origin, start/current time, uptime.
//! - **`/env`** — selected environment variables (`show.env.variables`) and
//!   selected base-configuration parameters (`show.application.properties`) —
//!   opt-in lists, so secrets are never dumped wholesale (Java parity).
//! - **`/health`** — runs the health-check functions listed in
//!   `mandatory.health.dependencies` / `optional.health.dependencies`
//!   (comma-separated routes): each is called with header `type=info` then
//!   `type=health`; a non-200 health status marks the dependency down. All
//!   mandatory up → `UP` (HTTP 200); any mandatory down → `DOWN` (HTTP 400,
//!   Java parity). The outcome feeds the liveness state.
//! - **`/livenessprobe`** — `OK` (text) while the last health outcome is good,
//!   else HTTP 400 `Unhealthy. Please check '/health' endpoint.`
//!
//! Deferred (maintainer-approved): `/info/lib` — Java lists JAR dependencies
//! from the archive manifest; a Rust binary has no runtime dependency
//! manifest (a build-script–embedded cargo metadata could provide it later).
//! Also deferred: `/info/routes`, XML responses, and the Java per-route info
//! cache.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;

use crate::envelope::EventEnvelope;
use crate::function::{AppError, ComposableFunction};
use crate::platform::Platform;
use crate::post_office::PostOffice;
use crate::trace;
use crate::util::app_config_reader::AppConfigReader;

pub const INFO_ACTUATOR: &str = "info.actuator.service";
pub const ENV_ACTUATOR: &str = "env.actuator.service";
pub const HEALTH_ACTUATOR: &str = "health.actuator.service";
pub const LIVENESS_ACTUATOR: &str = "liveness.actuator.service";

const SHOW_ENV: &str = "show.env.variables";
const SHOW_PROPERTIES: &str = "show.application.properties";
const REQUIRED_SERVICES: &str = "mandatory.health.dependencies";
const OPTIONAL_SERVICES: &str = "optional.health.dependencies";

/// Which actuator a registered instance serves (Java switches on the invoked
/// route via the `my_route` header; the Rust port parameterizes at
/// registration instead).
#[derive(Clone, Copy)]
pub enum ActuatorKind {
    Info,
    Env,
    Health,
    Liveness,
}

/// State shared by all four actuator registrations: the liveness flag follows
/// the most recent health outcome (Java `healthStatus`), and the app identity
/// is resolved once.
pub struct ActuatorContext {
    platform: Platform,
    health_status: AtomicBool,
    start_time: std::time::SystemTime,
    required: Vec<String>,
    optional: Vec<String>,
    description: String,
    app_version: String,
}

impl ActuatorContext {
    pub fn new(platform: &Platform) -> Arc<Self> {
        let config = AppConfigReader::get_instance();
        let split = |key: &str| -> Vec<String> {
            config
                .get_property_or(key, "")
                .split([',', ' '])
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect()
        };
        let required = split(REQUIRED_SERVICES);
        let optional = split(OPTIONAL_SERVICES);
        if !required.is_empty() {
            log::info!("Mandatory service dependencies - {required:?}");
        }
        if !optional.is_empty() {
            log::info!("Optional services dependencies - {optional:?}");
        }
        Arc::new(ActuatorContext {
            platform: platform.clone(),
            health_status: AtomicBool::new(true),
            start_time: std::time::SystemTime::now(),
            required,
            optional,
            description: config.get_property_or("info.app.description", &Platform::name()),
            // an application may declare its own version; the platform-core
            // version is the fallback (Java reads the app version from the
            // build metadata, which a Rust library cannot see at runtime)
            app_version: config.get_property_or("info.app.version", env!("CARGO_PKG_VERSION")),
        })
    }

    fn app_block(&self) -> serde_json::Value {
        serde_json::json!({
            "name": Platform::name(),
            "version": self.app_version,
            "description": self.description,
        })
    }
}

/// One actuator endpoint (register with the shared [`ActuatorContext`]).
pub struct ActuatorServices {
    kind: ActuatorKind,
    context: Arc<ActuatorContext>,
}

impl ActuatorServices {
    pub fn new(kind: ActuatorKind, context: Arc<ActuatorContext>) -> Self {
        ActuatorServices { kind, context }
    }
}

#[async_trait]
impl ComposableFunction for ActuatorServices {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let context = &self.context;
        match self.kind {
            ActuatorKind::Liveness => {
                if context.health_status.load(Ordering::SeqCst) {
                    EventEnvelope::new().set_body("OK")
                } else {
                    Ok(EventEnvelope::new()
                        .set_status(400)
                        .set_body("Unhealthy. Please check '/health' endpoint.")?)
                }
            }
            ActuatorKind::Info => {
                let now = std::time::SystemTime::now();
                let uptime = now.duration_since(context.start_time).unwrap_or_default();
                EventEnvelope::new().set_body(serde_json::json!({
                    "app": context.app_block(),
                    "runtime": {
                        "language": "rust",
                        "platform_core": env!("CARGO_PKG_VERSION"),
                    },
                    "origin": Platform::origin(),
                    "time": {
                        "start": trace::iso8601_utc(context.start_time),
                        "current": trace::iso8601_utc(now),
                    },
                    "up_time": elapsed_time(uptime),
                }))
            }
            ActuatorKind::Env => {
                let config = AppConfigReader::get_instance();
                let list = |key: &str| -> Vec<String> {
                    config
                        .get_property_or(key, "")
                        .split([',', ' '])
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(str::to_string)
                        .collect()
                };
                let mut environment = serde_json::Map::new();
                for name in list(SHOW_ENV) {
                    let value = std::env::var(&name).unwrap_or_default();
                    environment.insert(name, serde_json::Value::String(value));
                }
                let mut properties = serde_json::Map::new();
                for name in list(SHOW_PROPERTIES) {
                    let value = config.get_property(&name).unwrap_or_default();
                    properties.insert(name, serde_json::Value::String(value));
                }
                EventEnvelope::new().set_body(serde_json::json!({
                    "app": context.app_block(),
                    "env": {
                        "environment": environment,
                        "properties": properties,
                    },
                }))
            }
            ActuatorKind::Health => {
                let po = PostOffice::new(&context.platform);
                let mut dependency: Vec<serde_json::Value> = Vec::new();
                // optional services never affect the overall status
                check_services(&po, &context.optional, false, &mut dependency).await;
                let up = check_services(&po, &context.required, true, &mut dependency).await;
                context.health_status.store(up, Ordering::SeqCst);
                let mut result = serde_json::Map::new();
                if dependency.is_empty() {
                    result.insert(
                        "message".into(),
                        serde_json::Value::String(
                            "Did you forget to define mandatory.health.dependencies or optional.health.dependencies"
                                .to_string(),
                        ),
                    );
                }
                result.insert("dependency".into(), serde_json::Value::Array(dependency));
                result.insert(
                    "status".into(),
                    serde_json::Value::String(if up { "UP" } else { "DOWN" }.to_string()),
                );
                result.insert(
                    "origin".into(),
                    serde_json::Value::String(Platform::origin().to_string()),
                );
                result.insert("name".into(), serde_json::Value::String(Platform::name()));
                Ok(EventEnvelope::new()
                    .set_status(if up { 200 } else { 400 }) // Java parity
                    .set_body(serde_json::Value::Object(result))?)
            }
        }
    }
}

/// Query each health-check function: header `type=info` (3 s) merges its info
/// map into the dependency entry, then `type=health` (10 s) decides the
/// status (non-200 = down). Returns whether every service in the list is up.
async fn check_services(
    po: &PostOffice,
    services: &[String],
    required: bool,
    dependency: &mut Vec<serde_json::Value>,
) -> bool {
    let mut all_up = true;
    for route in services {
        let mut entry = serde_json::Map::new();
        entry.insert("route".into(), serde_json::Value::String(route.clone()));
        entry.insert("required".into(), serde_json::Value::Bool(required));
        // info is advisory — merge whatever the service reports about itself
        let info_request = EventEnvelope::new()
            .set_to(route)
            .set_header("type", "info");
        if let Ok(info) = po.request(info_request, Duration::from_secs(3)).await {
            if let Ok(serde_json::Value::Object(map)) = info.body_as::<serde_json::Value>() {
                for (key, value) in map {
                    entry.insert(key, value);
                }
            }
        }
        // health decides the status
        let health_request = EventEnvelope::new()
            .set_to(route)
            .set_header("type", "health");
        match po.request(health_request, Duration::from_secs(10)).await {
            Ok(response) => {
                entry.insert(
                    "status_code".into(),
                    serde_json::Value::from(response.status()),
                );
                if let Ok(message) = response.body_as::<serde_json::Value>() {
                    if message.is_string() || message.is_object() {
                        entry.insert("message".into(), message);
                    }
                }
                if response.has_error() {
                    all_up = false;
                }
            }
            Err(e) => {
                all_up = false;
                entry.insert("status_code".into(), serde_json::Value::from(e.status()));
                entry.insert(
                    "message".into(),
                    serde_json::Value::String(format!("Please check - {}", e.message())),
                );
            }
        }
        dependency.push(serde_json::Value::Object(entry));
    }
    all_up
}

/// Human-readable elapsed time (the Java `util.elapsedTime` analog).
fn elapsed_time(duration: Duration) -> String {
    let total = duration.as_secs();
    let (days, hours, minutes, seconds) = (
        total / 86_400,
        (total % 86_400) / 3600,
        (total % 3600) / 60,
        total % 60,
    );
    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{days} day{}", if days == 1 { "" } else { "s" }));
    }
    if hours > 0 {
        parts.push(format!("{hours} hour{}", if hours == 1 { "" } else { "s" }));
    }
    if minutes > 0 {
        parts.push(format!(
            "{minutes} minute{}",
            if minutes == 1 { "" } else { "s" }
        ));
    }
    parts.push(format!(
        "{seconds} second{}",
        if seconds == 1 { "" } else { "s" }
    ));
    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elapsed_time_formats() {
        assert_eq!(elapsed_time(Duration::from_secs(0)), "0 seconds");
        assert_eq!(elapsed_time(Duration::from_secs(1)), "1 second");
        assert_eq!(elapsed_time(Duration::from_secs(61)), "1 minute 1 second");
        assert_eq!(
            elapsed_time(Duration::from_secs(90_061)),
            "1 day 1 hour 1 minute 1 second"
        );
    }
}
