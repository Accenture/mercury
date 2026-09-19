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

//! Reusable Redis health-check logic for the platform's `/health` endpoint —
//! Rust port of the Java `RedisHealthProbe`: the shared PING probe behind
//! each module's own health route. It registers no route by itself (a
//! foundation function would auto-register in every consumer); a module binds
//! it with [`RedisHealthProbe::register`] under its own route and config
//! prefix — `soa.redis.health` for sync-over-async, `redis.health` for the
//! distributed cache — over this one probe.
//!
//! The function follows the standard health contract: `type=info` describes
//! the dependency; `type=health` returns a status map when the server is
//! reachable and a 503 response carrying a key-value map (`text` + `code`)
//! when it is not — the status code is what the health aggregation (and
//! Kubernetes) detects; the map is for the DevOps reader.
//!
//! The probe is a single Redis **PING** on a dedicated connection built from
//! the module's [`RedisConfig`] — the lightest round trip the protocol offers,
//! and one successful call proves connectivity, TLS, and authentication in a
//! single request.
//!
//! **The probe's configuration is resolved lazily** — when the probe
//! connection is built, and again whenever a failed probe forces a rebuild —
//! never at construction: a credential bootstrap that publishes secrets after
//! start-up registration must not be frozen out. Until a usable configuration
//! lands, an authentication rejection (`NOAUTH` / `WRONGPASS` /
//! `ERR Client sent AUTH`) or an unbuildable configuration is reported as a
//! **passing** `Waiting for Redis connection` status rather than a failure:
//! failing `/health` would invite the container orchestrator to restart the
//! pod, and a restart cannot produce the missing credential. Only a real
//! connectivity failure — connection refused or a timed-out round trip —
//! fails `/health` with HTTP 503.
//!
//! During application start-up the function returns a **placeholder healthy**
//! status and warms up the connection in the background, so `/health` does
//! not fail (or block) while the client and the rest of the start-up sequence
//! are still coming up. After the first successful probe — or once the grace
//! period has elapsed — every check is a live probe.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex as StdMutex};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use platform_core::{AppConfigReader, AppError, ComposableFunction, EventEnvelope, Platform};

use crate::backend::{ConnectError, RedisBackend};
use crate::config::RedisConfig;

const SERVICE_NAME: &str = "redis";
const PLACEHOLDER: &str = "Redis client is starting up";
const WAITING: &str = "Waiting for Redis connection";
const REACHABLE: &str = "Redis is reachable";

/// The reusable Redis health probe (see the module documentation). Bind it to
/// a route with [`register`](Self::register), or hold it and call it directly.
#[derive(Clone)]
pub struct RedisHealthProbe {
    inner: Arc<ProbeInner>,
}

struct ProbeInner {
    /// Re-invoked whenever the probe connection is (re)built — never resolved
    /// at construction, so late-published credentials are picked up.
    settings: Box<dyn Fn() -> RedisConfig + Send + Sync>,
    timeout: Duration,
    grace_deadline: Instant,
    ready: AtomicBool,
    warming_up: AtomicBool,
    /// The cached probe connection; probes serialize on this lock and a
    /// failed probe drops the connection so the next one re-resolves.
    probe: tokio::sync::Mutex<Option<RedisBackend>>,
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

impl RedisHealthProbe {
    /// `settings` is invoked when the probe connection is built — and again on
    /// every rebuild after a failure — so values published later in the
    /// start-up sequence (e.g. a vault-fetched password) are resolved on the
    /// next probe instead of being frozen at construction. When the real
    /// values are not yet known, return the best-known ones — an unusable
    /// result is handled by the waiting semantics. `timeout` bounds connect
    /// and command round trips; `grace` is the start-up placeholder window
    /// (`Duration::ZERO` = probe immediately).
    pub fn new(
        settings: impl Fn() -> RedisConfig + Send + Sync + 'static,
        timeout: Duration,
        grace: Duration,
    ) -> Self {
        RedisHealthProbe {
            inner: Arc::new(ProbeInner {
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

    /// Register this probe under `route` with the platform — several workers,
    /// because `/health` is polled concurrently (operations tooling plus the
    /// container platform's probes); the probe connection itself stays
    /// serialized on its own lock.
    pub fn register(
        self,
        platform: &Platform,
        route: &str,
        instances: usize,
    ) -> Result<(), AppError> {
        platform.register(route, Arc::new(self), instances)
    }
}

#[async_trait]
impl ComposableFunction for RedisHealthProbe {
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

impl ProbeInner {
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

    async fn probe_locked(&self, backend: &mut Option<RedisBackend>) -> Result<(), ProbeFailure> {
        if backend.is_none() {
            // re-resolved, not cached from the constructor: a credential
            // published by a later application bootstrap is not visible while
            // this function is constructed at start-up registration time. The
            // probe bounds connect and command with its own timeout, and the
            // backend selects standalone or cluster per the config's two keys.
            let settings = (self.settings)().with_timeout(self.timeout.as_millis() as u64);
            *self.current.lock().expect("health href poisoned") = Some(settings.endpoint());
            let fresh = RedisBackend::connect_once(&settings)
                .await
                .map_err(classify)?;
            *backend = Some(fresh);
        }
        let live = backend.as_ref().expect("probe connection just built");
        live.ping().await.map_err(classify)
    }

    /// The dependency's href — the configured host:port. Resolved on demand
    /// for an info call that arrives before the first probe, and overwritten
    /// by every probe rebuild's own fresh resolve.
    fn href(&self) -> String {
        let mut current = self.current.lock().expect("health href poisoned");
        current
            .get_or_insert_with(|| (self.settings)().endpoint())
            .clone()
    }
}

/// Classify one connect/probe failure against the waiting-vs-outage boundary
/// (Java `RedisHealthProbe.waitingOnConfig`): an unbuildable configuration and
/// a rejected credential are "waiting"; a refused connection or a timeout is
/// an outage. (The redis crate folds a rejected connect-time `AUTH` into
/// `ErrorKind::AuthenticationFailed` and drops the server text, so the kind is
/// checked as well as the message.)
fn classify(error: ConnectError) -> ProbeFailure {
    match error {
        ConnectError::Unbuildable(cause) => ProbeFailure {
            waiting: true,
            cause,
        },
        ConnectError::Redis(error) => {
            let cause = error.to_string();
            ProbeFailure {
                waiting: error.kind() == redis::ErrorKind::AuthenticationFailed
                    || waiting_on_config(&cause),
                cause,
            }
        }
        ConnectError::TimedOut(_) => ProbeFailure {
            waiting: false,
            cause: error.to_string(),
        },
    }
}

/// The waiting-vs-outage boundary on the server's text: `NOAUTH` /
/// `WRONGPASS` (credentials not yet landed), or a password presented to a
/// server that wants none (`ERR Client sent AUTH`) — signatures a pod restart
/// cannot fix. Everything else is a genuine outage.
fn waiting_on_config(cause: &str) -> bool {
    cause.contains("NOAUTH")
        || cause.contains("WRONGPASS")
        || cause.contains("ERR Client sent AUTH")
}

/// Parse a duration expression to whole seconds — the Java
/// `Utility.getDurationInSeconds` surface these modules need: a bare number is
/// seconds; `s`/`m`/`h`/`d` suffixes scale. `None` for anything else.
pub fn duration_seconds(text: &str) -> Option<u64> {
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

/// Resolve a duration configuration key with a built-in default, preferring
/// the module's own key and falling back to the legacy un-prefixed one when it
/// is absent — the same backward-compatible policy [`RedisConfig`] applies to
/// the connection keys (Java `RedisHealthProbe.resolveDurationMs`). A module
/// calls this to build the `timeout` / `grace` arguments of its probe:
/// `resolve_duration("soa.redis.health.timeout", "redis.health.timeout", "5s")`.
pub fn resolve_duration(key: &str, legacy_key: &str, default: &str) -> Duration {
    let config = AppConfigReader::get_instance();
    let text = config
        .get_property(key)
        .or_else(|| config.get_property(legacy_key))
        .unwrap_or_else(|| default.to_string());
    let seconds = duration_seconds(&text)
        .or_else(|| duration_seconds(default))
        .unwrap_or(0);
    Duration::from_secs(seconds)
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
        // an unbuildable configuration is waiting; a timeout is an outage
        assert!(classify(ConnectError::Unbuildable("Invalid Redis address".into())).waiting);
        assert!(!classify(ConnectError::TimedOut(Duration::from_secs(2))).waiting);
    }

    /// Java `resolveDurationMs`: the module key first, the legacy key next,
    /// then the built-in default — and a garbage value degrades to the default.
    #[test]
    fn resolve_duration_prefers_the_module_key_then_the_legacy_key() {
        use platform_core::overrides;
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = LOCK.lock().unwrap_or_else(|p| p.into_inner());
        overrides::clear("test.health.timeout");
        overrides::clear("legacy.health.timeout");
        assert_eq!(
            Duration::from_secs(5),
            resolve_duration("test.health.timeout", "legacy.health.timeout", "5s")
        );
        overrides::set("legacy.health.timeout", "2m");
        assert_eq!(
            Duration::from_secs(120),
            resolve_duration("test.health.timeout", "legacy.health.timeout", "5s")
        );
        overrides::set("test.health.timeout", "7s");
        assert_eq!(
            Duration::from_secs(7),
            resolve_duration("test.health.timeout", "legacy.health.timeout", "5s")
        );
        overrides::set("test.health.timeout", "soon");
        assert_eq!(
            Duration::from_secs(5),
            resolve_duration("test.health.timeout", "legacy.health.timeout", "5s"),
            "an unparseable value degrades to the default"
        );
        overrides::clear("test.health.timeout");
        overrides::clear("legacy.health.timeout");
    }
}
