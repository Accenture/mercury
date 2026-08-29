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
//! the default endpoints (`/info`, `/info/routes`, `/env`, `/health`,
//! `/livenessprobe`).
//!
//! - **`/info`** — application identity (name, version, description), runtime,
//!   origin, start/current time, uptime.
//! - **`/info/routes`** — the app block plus the local routing table split by
//!   visibility (`routing.public` / `routing.private`, route → instance
//!   count; Java `handleInfoRoute`). Java's optional blocks (`journal`,
//!   `route_substitution`) and the mesh `network` table are omitted when
//!   empty — subsystems this port does not have, so the response is
//!   `{app, routing}` here.
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
//! Also deferred: XML responses. The Java per-route info cache is ported
//! (increment 71): the `type=info` lookup is cached 5 s per dependency via
//! `ManagedCache("health.info")` — see `check_services`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use async_trait::async_trait;

use crate::envelope::EventEnvelope;
use crate::function::{AppError, ComposableFunction};
use crate::platform::Platform;
use crate::post_office::PostOffice;
use crate::trace;
use crate::util::app_config_reader::AppConfigReader;
use crate::util::elapsed_time;
use crate::util::managed_cache::ManagedCache;

pub const INFO_ACTUATOR: &str = "info.actuator.service";
pub const ROUTES_ACTUATOR: &str = "routes.actuator.service";
pub const ENV_ACTUATOR: &str = "env.actuator.service";
pub const HEALTH_ACTUATOR: &str = "health.actuator.service";
pub const LIVENESS_ACTUATOR: &str = "liveness.actuator.service";

/// The worker-instance count for the actuator family (default 5 — a rule of
/// thumb, like every initial instance count): operations teams fine-tune it
/// via `worker.instances.actuator.services` in QA/Perf environments before
/// promoting to production. ONE family key covers all five actuator routes —
/// and it is the SAME key the Java engine carries: its actuators are one
/// aliased class whose primary route is `actuator.services` (that route
/// itself is unported here), so a single runbook line tunes both engines.
/// Numeric value wins, anything else falls back (env_instances semantics).
pub fn actuator_instances(config: &AppConfigReader) -> usize {
    config
        .get_property("worker.instances.actuator.services")
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(5)
}

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
    Routes,
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
                // Java ActuatorServices: explicit text/plain on the envelope
                if context.health_status.load(Ordering::SeqCst) {
                    Ok(EventEnvelope::new()
                        .set_header("content-type", "text/plain")
                        .set_body("OK")?)
                } else {
                    Ok(EventEnvelope::new()
                        .set_status(400)
                        .set_header("content-type", "text/plain")
                        .set_body("Unhealthy. Please check '/health' endpoint.")?)
                }
            }
            ActuatorKind::Routes => {
                // Java handleInfoRoute: the app block plus the local routing
                // table split by visibility, route → instance count. Java's
                // optional blocks — "journal" (journaling), "route_substitution"
                // and the mesh "network" table — are omitted when empty, and
                // none of those subsystems exist in this port, so the response
                // is exactly {app, routing}. BTreeMap keeps the output
                // deterministic (Java's HashMap ordering is arbitrary; JSON
                // object order is not contractual, but stable beats random).
                EventEnvelope::new()
                    .set_header("content-type", "application/json")
                    .set_body(serde_json::json!({
                        "app": context.app_block(),
                        "routing": local_routing(&context.platform),
                    }))
            }
            ActuatorKind::Info => {
                let now = std::time::SystemTime::now();
                let uptime = now.duration_since(context.start_time).unwrap_or_default();
                // Java ActuatorServices: explicit application/json envelope type
                EventEnvelope::new()
                    .set_header("content-type", "application/json")
                    .set_body(serde_json::json!({
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
                EventEnvelope::new()
                    .set_header("content-type", "application/json")
                    .set_body(serde_json::json!({
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
                    .set_header("content-type", "application/json")
                    .set_body(serde_json::Value::Object(result))?)
            }
        }
    }
}

/// The per-dependency info-lookup cache (Java parity:
/// `SimpleCache.createCache("health.info", 5000)` in `ActuatorServices` —
/// this port maps every Java `SimpleCache` site onto `ManagedCache`, per the
/// maintainer's one-cache-type ruling; `docs/design/managed-cache-port.md`).
/// Only the `type=info` lookup is cached — never the `/health` result: the
/// `type=health` probe re-runs on every call and `/livenessprobe` reads the
/// atomic health flag.
fn health_info_cache() -> &'static Arc<ManagedCache> {
    static CACHE: OnceLock<Arc<ManagedCache>> = OnceLock::new();
    CACHE.get_or_init(|| ManagedCache::create_cache("health.info", 5000))
}

/// Query each health-check function: header `type=info` (3 s, cached 5 s per
/// route — Java `isServiceUnhealthy`) merges its info map into the dependency
/// entry, then `type=health` (10 s) decides the status (non-200 = down).
/// Returns whether every service in the list is up.
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
        // info is advisory — merge whatever the service reports about itself.
        // Java parity: the lookup is cached under "info/{route}" and only a
        // map body is cached (a non-map response is re-requested every call)
        let cache = health_info_cache();
        let info_key = format!("info/{route}");
        if !cache.exists(&info_key) {
            let info_request = EventEnvelope::new()
                .set_to(route)
                .set_header("type", "info");
            if let Ok(info) = po.request(info_request, Duration::from_secs(3)).await {
                if let Ok(body @ serde_json::Value::Object(_)) = info.body_as::<serde_json::Value>()
                {
                    cache.put(&info_key, body);
                }
            }
        }
        if let Some(info) = cache.get_as::<serde_json::Value>(&info_key) {
            if let serde_json::Value::Object(map) = info.as_ref() {
                for (key, value) in map {
                    entry.insert(key.clone(), value.clone());
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

// `elapsed_time` moved to `crate::util` (increment 71): it is now shared by
// the `/info` uptime rendering here and the ManagedCache create log.\n\n/// The rendered local routing view, split by visibility with pool-style
/// route families compressed. The routing table changes infrequently, so the
/// rendered view is cached for 10 minutes to skip repeated computation under
/// actuator polling — an ad-hoc runtime registration may take up to the
/// window to appear, which is acceptable for the operator view (Java
/// ActuatorServices parity).
fn local_routing(platform: &crate::platform::Platform) -> serde_json::Value {
    use std::sync::OnceLock;
    static CACHE: OnceLock<std::sync::Arc<crate::util::managed_cache::ManagedCache>> =
        OnceLock::new();
    let cache = CACHE.get_or_init(|| {
        crate::util::managed_cache::ManagedCache::create_cache("local.routing.info", 10 * 60 * 1000)
    });
    if let Some(cached) = cache.get("local.routing") {
        if let Some(value) = cached.downcast_ref::<serde_json::Value>() {
            return value.clone();
        }
    }
    let mut public = std::collections::BTreeMap::new();
    let mut private = std::collections::BTreeMap::new();
    for route in platform.routes() {
        let instances = platform.instances(&route).unwrap_or(1);
        if platform.is_private(&route).unwrap_or(true) {
            private.insert(route, instances);
        } else {
            public.insert(route, instances);
        }
    }
    let value = serde_json::json!({
        "public": compress_route_families(public),
        "private": compress_route_families(private),
    });
    cache.put("local.routing", value.clone());
    value
}

/// Render pool-style route families compactly (Java `ActuatorServices.
/// compressRouteFamilies`): routes that differ only by a trailing numeric
/// suffix, with uniform instances and contiguous canonical numbering (no
/// leading zeros), collapse into one display entry — e.g. the 500 streaming
/// reply lanes render as `"async.http.response.stream.0 - 499": 1`.
/// Irregular families and singletons render individually with their names
/// preserved exactly. Display-only — the routing table itself is unchanged.
fn compress_route_families(
    routes: std::collections::BTreeMap<String, usize>,
) -> std::collections::BTreeMap<String, usize> {
    let mut result = std::collections::BTreeMap::new();
    let mut families: std::collections::BTreeMap<String, std::collections::BTreeMap<u64, usize>> =
        std::collections::BTreeMap::new();
    for (route, instances) in routes {
        let family = route.rfind('.').and_then(|dot| {
            let suffix = &route[dot + 1..];
            if !suffix.is_empty() && suffix.len() < 10 && suffix.bytes().all(|b| b.is_ascii_digit())
            {
                let n: u64 = suffix.parse().ok()?;
                // canonical digits only, so individual names are preserved exactly
                (suffix == n.to_string()).then(|| (route[..dot + 1].to_string(), n))
            } else {
                None
            }
        });
        match family {
            Some((base, n)) => {
                families.entry(base).or_default().insert(n, instances);
            }
            None => {
                result.insert(route, instances);
            }
        }
    }
    for (base, members) in families {
        let min = *members.keys().next().expect("non-empty family");
        let max = *members.keys().last().expect("non-empty family");
        let uniform = members
            .values()
            .collect::<std::collections::HashSet<_>>()
            .len()
            == 1;
        if members.len() > 1 && uniform && members.len() as u64 == max - min + 1 {
            let instances = *members.values().next().expect("non-empty family");
            result.insert(format!("{base}{min} - {max}"), instances);
        } else {
            for (n, instances) in members {
                result.insert(format!("{base}{n}"), instances);
            }
        }
    }
    result
}
