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

//! Redis health check for the platform's `/health` endpoint — Rust port of
//! the Java `soa.redis.health` function.
//!
//! Add [`REDIS_HEALTH_ROUTE`] to `mandatory.health.dependencies` (or
//! `optional.health.dependencies`) and `/health` will include the Redis
//! server status. The function follows the standard health contract:
//! `type=info` describes the dependency; `type=health` returns a status map
//! when the server is reachable and a 503 response carrying a key-value map
//! (`text` + `code`) when it is not — the status code is what the health
//! aggregation (and Kubernetes) detects; the map is for the DevOps reader.
//!
//! The probe is a single Redis **PING** on a dedicated connection built from
//! the module's discrete `redis.*` startup parameters — the lightest round
//! trip the protocol offers, and one successful call proves connectivity,
//! TLS, and authentication in a single request. The `minigraph-state-redis`
//! extension reads the same `redis.*` parameters, so one `soa.redis.health`
//! covers a deployment using either or both modules.
//!
//! The route carries the `soa.` prefix deliberately: the plain `redis.health`
//! name is reserved for the health check of the planned generic Redis
//! distributed-cache module, so both features can coexist against the same
//! Redis server.
//!
//! **The probe's client configuration is resolved lazily** — when the probe
//! connection is built, and again whenever a failed probe forces a rebuild —
//! never at construction (the Java preload-before-bootstrap lesson: a
//! credential bootstrap that publishes secrets after startup registration
//! must not be frozen out). Until a usable configuration lands, an
//! authentication rejection (`NOAUTH` / `WRONGPASS` / `ERR Client sent AUTH`)
//! or an unbuildable configuration is reported as a **passing**
//! `Waiting for Redis connection` status rather than a failure: failing
//! `/health` would invite the container orchestrator to restart the pod, and
//! a restart cannot produce the missing credential. Only a real connectivity
//! failure — connection refused or a timed-out round trip — fails `/health`
//! with HTTP 503.
//!
//! During application start-up the function returns a **placeholder healthy**
//! status and warms up the connection in the background, so `/health` does
//! not fail (or block) while the client and the rest of the start-up sequence
//! are still coming up. After the first successful probe — or once the grace
//! period (`redis.health.startup.grace`, default `30s`) has elapsed — every
//! check is a live probe. `redis.health.timeout` (default `5s`) bounds the
//! probe's connect and command round trips.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex as StdMutex};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use platform_core::{AppConfigReader, AppError, ComposableFunction, EventEnvelope, Platform};
use redis::aio::MultiplexedConnection;

use crate::connection::RedisSettings;

/// The health-check route name. The `soa.` prefix is normative: the plain
/// `redis.health` route is reserved for the planned generic Redis
/// distributed-cache module's check.
pub const REDIS_HEALTH_ROUTE: &str = "soa.redis.health";

const SERVICE_NAME: &str = "redis";
const TIMEOUT_KEY: &str = "redis.health.timeout";
const GRACE_KEY: &str = "redis.health.startup.grace";
const PLACEHOLDER: &str = "Redis client is starting up";
const WAITING: &str = "Waiting for Redis connection";
const REACHABLE: &str = "Redis is reachable";

/// Redis health check function (see the module documentation). Register it
/// with [`RedisHealthCheck::register`] or hold it and call it directly.
#[derive(Clone)]
pub struct RedisHealthCheck {
    inner: Arc<HealthInner>,
}

struct HealthInner {
    /// Re-invoked whenever the probe connection is (re)built — never resolved
    /// at construction, so late-published credentials are picked up.
    settings: Box<dyn Fn() -> RedisSettings + Send + Sync>,
    timeout: Duration,
    grace_deadline: Instant,
    ready: AtomicBool,
    warming_up: AtomicBool,
    /// The cached probe connection; probes serialize on this lock and a
    /// failed probe drops the connection so the next one re-resolves.
    probe: tokio::sync::Mutex<Option<MultiplexedConnection>>,
    /// What the current probe client was built from — the `href` source,
    /// replaced on every rebuild.
    current: StdMutex<Option<String>>,
}

/// One probe failure, already classified against the waiting-vs-outage
/// boundary (see [`waiting_on_config`]).
struct ProbeFailure {
    waiting: bool,
    cause: String,
}

impl RedisHealthCheck {
    /// Read `redis.health.timeout` / `redis.health.startup.grace` from
    /// application configuration; the probe parameters themselves come from
    /// the `redis.*` family, re-resolved on every rebuild.
    pub fn from_config() -> Self {
        let config = AppConfigReader::get_instance();
        let timeout = duration_seconds(&config.get_property_or(TIMEOUT_KEY, "5s")).unwrap_or(5);
        let grace = duration_seconds(&config.get_property_or(GRACE_KEY, "30s")).unwrap_or(30);
        RedisHealthCheck::new(
            RedisSettings::from_config,
            Duration::from_secs(timeout),
            Duration::from_secs(grace),
        )
    }

    /// Reuse/test seam. `settings` is invoked when the probe connection is
    /// built — and again on every rebuild after a failure — so values
    /// published later in the start-up sequence (e.g. a vault-fetched
    /// `redis.password`) are resolved on the next probe instead of being
    /// frozen at construction. When the real values are not yet known, return
    /// the best-known ones — an unusable result is handled by the waiting
    /// semantics.
    pub fn new(
        settings: impl Fn() -> RedisSettings + Send + Sync + 'static,
        timeout: Duration,
        grace: Duration,
    ) -> Self {
        RedisHealthCheck {
            inner: Arc::new(HealthInner {
                settings: Box::new(settings),
                timeout,
                grace_deadline: Instant::now() + grace,
                ready: AtomicBool::new(false),
                warming_up: AtomicBool::new(false),
                probe: tokio::sync::Mutex::new(None),
                current: StdMutex::new(None),
            }),
        }
    }

    /// Register this function at [`REDIS_HEALTH_ROUTE`] with the platform —
    /// several workers, because `/health` is polled concurrently (operations
    /// tooling plus the container platform's probes); the probe connection
    /// itself stays serialized on its own lock.
    pub fn register(self, platform: &Platform) -> Result<(), AppError> {
        platform.register(REDIS_HEALTH_ROUTE, Arc::new(self), 5)
    }
}

#[async_trait]
impl ComposableFunction for RedisHealthCheck {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        match headers.get("type").map(String::as_str) {
            Some("info") => EventEnvelope::new().set_body(serde_json::json!({
                "service": SERVICE_NAME,
                "href": self.inner.href(),
            })),
            Some("health") => {
                if !self.inner.ready.load(Ordering::Acquire)
                    && Instant::now() < self.inner.grace_deadline
                {
                    // let the Redis connection and the application start-up
                    // sequence complete first: warm up in the background and
                    // report a placeholder healthy status meanwhile
                    self.inner.warm_up();
                    return EventEnvelope::new()
                        .set_body(serde_json::json!({"status": PLACEHOLDER}));
                }
                self.inner.probe_response().await
            }
            _ => Err(AppError::new(400, "type must be info or health")),
        }
    }
}

impl HealthInner {
    fn warm_up(self: &Arc<Self>) {
        if self
            .warming_up
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            let inner = self.clone();
            tokio::spawn(async move {
                let _ = inner.probe_response().await;
                if inner.ready.load(Ordering::Acquire) {
                    log::info!("{SERVICE_NAME} health check is ready");
                } else {
                    // the client configuration is still incomplete - allow
                    // another warm-up attempt
                    inner.warming_up.store(false, Ordering::Release);
                }
            });
        }
    }

    async fn probe_response(&self) -> Result<EventEnvelope, AppError> {
        let mut guard = self.probe.lock().await;
        match self.probe_locked(&mut guard).await {
            Ok(()) => {
                self.ready.store(true, Ordering::Release);
                EventEnvelope::new().set_body(serde_json::json!({
                    "status": REACHABLE,
                    "href": self.href(),
                }))
            }
            Err(failure) => {
                // drop the connection: the next probe re-resolves the
                // configuration, so the check heals itself once real values land
                *guard = None;
                if failure.waiting {
                    // not yet usable (unbuildable values, or credentials the
                    // server rejects): a PASSING waiting status - failing
                    // /health would invite a pod restart, and a restart cannot
                    // produce the credential
                    log::warn!(
                        "{SERVICE_NAME} health check waiting for a usable client configuration - {}",
                        failure.cause
                    );
                    EventEnvelope::new().set_body(serde_json::json!({"status": WAITING}))
                } else {
                    // a genuine outage: the 503 status is what the health
                    // aggregation (and Kubernetes) detects; the key-value body
                    // keeps the code visible to the DevOps reader too
                    EventEnvelope::new()
                        .set_status(503)
                        .set_body(serde_json::json!({
                            "text": format!("Redis is not reachable - {}", failure.cause),
                            "code": 503,
                        }))
                }
            }
        }
    }

    async fn probe_locked(
        &self,
        connection: &mut Option<MultiplexedConnection>,
    ) -> Result<(), ProbeFailure> {
        if connection.is_none() {
            // re-resolved, not cached from the constructor: a credential
            // published by a later application bootstrap is not visible while
            // this function is constructed at startup registration time
            let settings = (self.settings)();
            *self.current.lock().expect("health href poisoned") =
                Some(format!("{}:{}", settings.host(), settings.port()));
            let client = settings.client().map_err(|e| ProbeFailure {
                // the client cannot even be built from the resolved values -
                // the unresolved-placeholder signature, never an outage
                waiting: true,
                cause: e.message().to_string(),
            })?;
            let fresh =
                tokio::time::timeout(self.timeout, client.get_multiplexed_async_connection())
                    .await
                    .map_err(|_| self.timed_out("connect"))?
                    .map_err(classify)?;
            *connection = Some(fresh);
        }
        let live = connection.as_mut().expect("probe connection just built");
        tokio::time::timeout(self.timeout, redis::cmd("PING").query_async::<()>(live))
            .await
            .map_err(|_| self.timed_out("ping"))?
            .map_err(classify)?;
        Ok(())
    }

    fn timed_out(&self, phase: &str) -> ProbeFailure {
        ProbeFailure {
            waiting: false,
            cause: format!("{phase} timed out after {} ms", self.timeout.as_millis()),
        }
    }

    /// The dependency's href — the configured host:port. Resolved on demand
    /// for an info call that arrives before the first probe, and overwritten
    /// by every probe rebuild's own fresh resolve.
    fn href(&self) -> String {
        let mut current = self.current.lock().expect("health href poisoned");
        current
            .get_or_insert_with(|| {
                let settings = (self.settings)();
                format!("{}:{}", settings.host(), settings.port())
            })
            .clone()
    }
}

/// Classify one Redis error against the waiting-vs-outage boundary.
fn classify(error: redis::RedisError) -> ProbeFailure {
    let cause = error.to_string();
    ProbeFailure {
        waiting: error.kind() == redis::ErrorKind::AuthenticationFailed
            || waiting_on_config(&cause),
        cause,
    }
}

/// The waiting-vs-outage boundary. True when the failure is a configuration
/// that is not yet usable: the server rejected the credentials (`NOAUTH` /
/// `WRONGPASS`), or a password was presented to a server that wants none
/// (`ERR Client sent AUTH`) — the signatures of a credential that has not
/// landed yet, which a pod restart cannot fix. (The redis crate folds a
/// rejected connect-time `AUTH` into `ErrorKind::AuthenticationFailed` and
/// drops the server text, so [`classify`] checks the kind as well.)
/// Everything else (connection refused, timeout) is a genuine outage and
/// fails `/health`.
fn waiting_on_config(cause: &str) -> bool {
    cause.contains("NOAUTH")
        || cause.contains("WRONGPASS")
        || cause.contains("ERR Client sent AUTH")
}

/// Parse a duration expression to whole seconds — the Java
/// `Utility.getDurationInSeconds` surface this module needs: a bare number is
/// seconds; `s`/`m`/`h`/`d` suffixes scale.
fn duration_seconds(text: &str) -> Option<u64> {
    let trimmed = text.trim().to_lowercase();
    let (digits, scale) = match trimmed.strip_suffix(&['s', 'm', 'h', 'd'][..]) {
        Some(prefix) => {
            let unit = trimmed.as_bytes()[trimmed.len() - 1];
            let scale = match unit {
                b'm' => 60,
                b'h' => 3600,
                b'd' => 86400,
                _ => 1,
            };
            (prefix.trim().to_string(), scale)
        }
        None => (trimmed, 1),
    };
    digits.parse::<u64>().ok().map(|n| n * scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_expressions_parse_to_seconds() {
        assert_eq!(Some(5), duration_seconds("5s"));
        assert_eq!(Some(30), duration_seconds(" 30s "));
        assert_eq!(Some(120), duration_seconds("2m"));
        assert_eq!(Some(3600), duration_seconds("1h"));
        assert_eq!(Some(86400), duration_seconds("1d"));
        assert_eq!(Some(45), duration_seconds("45"));
        assert_eq!(None, duration_seconds("soon"));
        assert_eq!(None, duration_seconds(""));
    }

    /// The waiting-vs-outage boundary pins the three server-side signatures of
    /// a credential that has not landed yet; plain outages stay failures.
    #[test]
    fn waiting_classification_matches_the_java_boundary() {
        assert!(waiting_on_config("NOAUTH Authentication required."));
        assert!(waiting_on_config(
            "WRONGPASS invalid username-password pair or user is disabled."
        ));
        assert!(waiting_on_config(
            "An error was signalled by the server: ERR Client sent AUTH, but no password is set"
        ));
        assert!(!waiting_on_config("Connection refused (os error 61)"));
        assert!(!waiting_on_config("ping timed out after 5000 ms"));
    }
}
