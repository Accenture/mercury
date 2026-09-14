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

//! Kafka health check for the platform's `/health` endpoint (Java
//! `kafka.health`). Add [`KAFKA_HEALTH_ROUTE`] to
//! `mandatory.health.dependencies` (or the optional list) and `/health`
//! includes the cluster status.
//!
//! The probe is a single Kafka **Metadata** request — the most lightweight
//! cluster round trip the client offers: it joins no consumer group, commits
//! no offsets, and needs no admin privileges (brokers FILTER the topic list
//! by Describe grants rather than rejecting the request, so a locked-down
//! principal still proves connectivity, TLS/SASL authentication, and a served
//! API request — the reported topic count may be 0). The probe follows
//! whichever client the deployment configured: the consumer template
//! normally, or the producer template's connection/security surface on a
//! produce-only leg (see `client_config::health_probe_config`).
//!
//! The probe's configuration is resolved **lazily** — when the probe client
//! is built, and again whenever a failed probe forces a rebuild — never at
//! registration time (the vault pattern: a template interpolating a
//! late-published credential must not be frozen with it missing). A template
//! the client cannot even be **built** from reports a passing
//! `Waiting for Kafka connection` status; only a real round trip that fails —
//! client built, cluster unreachable — fails `/health` with the `{text, code}`
//! 503 shape. During start-up a **placeholder healthy** status is returned
//! while the client warms up in the background
//! (`kafka.health.startup.grace`, default `30s`; `kafka.health.timeout`,
//! default `5s`, bounds the round trip).
//!
//! **Threading:** the Java module pins this function to kernel threads
//! because the JVM client does blocking I/O in `synchronized` sections; this
//! engine's client (librdkafka) runs its own native threads, and the blocking
//! metadata call is offloaded to the tokio blocking pool — the async-correct
//! mapping of the same concern.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex as StdMutex, OnceLock};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use platform_core::{preload, AppConfigReader, AppError, ComposableFunction, EventEnvelope};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::util::Timeout;

/// The health-check route name.
pub const KAFKA_HEALTH_ROUTE: &str = "kafka.health";

const SERVICE_NAME: &str = "kafka";
const TIMEOUT_KEY: &str = "kafka.health.timeout";
const GRACE_KEY: &str = "kafka.health.startup.grace";
const PLACEHOLDER: &str = "Kafka client is starting up";
const WAITING: &str = "Waiting for Kafka connection";
const REACHABLE: &str = "Kafka cluster is reachable";

/// The Kafka health check function (see the module documentation). Several
/// workers because `/health` is polled concurrently; every probe serializes
/// on the internal lock.
#[preload(route = "kafka.health", instances = 5)]
#[derive(Default)]
pub struct KafkaHealthCheck;

struct HealthInner {
    /// Re-invoked whenever the probe client is (re)built — never resolved at
    /// registration time.
    probe_config: Box<dyn Fn() -> Result<ClientConfig, AppError> + Send + Sync>,
    timeout: Duration,
    grace_deadline: Instant,
    ready: AtomicBool,
    warming_up: AtomicBool,
    /// The cached probe consumer; probes serialize on this lock and a failed
    /// probe drops the client so the next one re-resolves the template.
    probe: tokio::sync::Mutex<Option<BaseConsumer>>,
    /// The bootstrap servers of the current probe config — the `href` source.
    current: StdMutex<Option<String>>,
}

struct ProbeFailure {
    waiting: bool,
    cause: String,
}

fn inner() -> &'static Arc<HealthInner> {
    static INNER: OnceLock<Arc<HealthInner>> = OnceLock::new();
    INNER.get_or_init(|| {
        let config = AppConfigReader::get_instance();
        let timeout = duration_seconds(&config.get_property_or(TIMEOUT_KEY, "5s")).unwrap_or(5);
        let grace = duration_seconds(&config.get_property_or(GRACE_KEY, "30s")).unwrap_or(30);
        Arc::new(HealthInner::new(
            crate::client_config::health_probe_config,
            Duration::from_secs(timeout),
            Duration::from_secs(grace),
        ))
    })
}

impl HealthInner {
    fn new(
        probe_config: impl Fn() -> Result<ClientConfig, AppError> + Send + Sync + 'static,
        timeout: Duration,
        grace: Duration,
    ) -> Self {
        HealthInner {
            probe_config: Box::new(probe_config),
            timeout,
            grace_deadline: Instant::now() + grace,
            ready: AtomicBool::new(false),
            warming_up: AtomicBool::new(false),
            probe: tokio::sync::Mutex::new(None),
            current: StdMutex::new(None),
        }
    }

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
                    // the configuration is still incomplete - allow another attempt
                    inner.warming_up.store(false, Ordering::Release);
                }
            });
        }
    }

    async fn probe_response(&self) -> Result<EventEnvelope, AppError> {
        let mut guard = self.probe.lock().await;
        match self.probe_locked(&mut guard).await {
            Ok(topics) => {
                self.ready.store(true, Ordering::Release);
                EventEnvelope::new().set_body(serde_json::json!({
                    "status": REACHABLE,
                    "topics": topics,
                    "href": self.href(),
                }))
            }
            Err(failure) => {
                // drop the client: the next probe re-resolves the template,
                // so the check heals itself once the real values land
                *guard = None;
                if failure.waiting {
                    log::warn!(
                        "{SERVICE_NAME} health check waiting for a usable client configuration - {}",
                        failure.cause
                    );
                    EventEnvelope::new().set_body(serde_json::json!({"status": WAITING}))
                } else {
                    EventEnvelope::new()
                        .set_status(503)
                        .set_body(serde_json::json!({
                            "text": format!("Kafka cluster is not reachable - {}", failure.cause),
                            "code": 503,
                        }))
                }
            }
        }
    }

    async fn probe_locked(
        &self,
        consumer: &mut Option<BaseConsumer>,
    ) -> Result<usize, ProbeFailure> {
        if consumer.is_none() {
            // re-resolved, not cached from registration: a credential
            // published by a later bootstrap must reach the next rebuild
            let config = (self.probe_config)().map_err(|e| ProbeFailure {
                waiting: true,
                cause: e.message().to_string(),
            })?;
            *self.current.lock().expect("health href poisoned") =
                config.get("bootstrap.servers").map(str::to_string);
            let fresh: BaseConsumer = config.create().map_err(|e| ProbeFailure {
                // the client cannot even be built from the resolved values -
                // the not-yet-usable-configuration signature, never an outage
                waiting: true,
                cause: e.to_string(),
            })?;
            *consumer = Some(fresh);
        }
        let client = consumer.take().expect("probe client just built");
        let timeout = self.timeout;
        // the metadata call blocks its thread (librdkafka round trip) - the
        // blocking pool is the async-correct home for it
        let (result, client) = tokio::task::spawn_blocking(move || {
            let result = client.fetch_metadata(None, Timeout::After(timeout));
            (result, client)
        })
        .await
        .map_err(|e| ProbeFailure {
            waiting: false,
            cause: format!("probe task failed - {e}"),
        })?;
        match result {
            Ok(metadata) => {
                *consumer = Some(client);
                Ok(metadata.topics().len())
            }
            Err(error) => Err(ProbeFailure {
                waiting: false,
                cause: error.to_string(),
            }),
        }
    }

    /// The dependency's href — the configured bootstrap servers.
    fn href(&self) -> String {
        let mut current = self.current.lock().expect("health href poisoned");
        current
            .get_or_insert_with(|| {
                (self.probe_config)()
                    .ok()
                    .and_then(|config| config.get("bootstrap.servers").map(str::to_string))
                    .unwrap_or_else(|| "unresolved".to_string())
            })
            .clone()
    }

    async fn handle(self: &Arc<Self>, kind: Option<&str>) -> Result<EventEnvelope, AppError> {
        match kind {
            Some("info") => EventEnvelope::new().set_body(serde_json::json!({
                "service": SERVICE_NAME,
                "href": self.href(),
            })),
            Some("health") => {
                if !self.ready.load(Ordering::Acquire) && Instant::now() < self.grace_deadline {
                    // let the client and the start-up sequence come up first
                    self.warm_up();
                    return EventEnvelope::new()
                        .set_body(serde_json::json!({"status": PLACEHOLDER}));
                }
                self.probe_response().await
            }
            _ => Err(AppError::new(400, "type must be info or health")),
        }
    }
}

#[async_trait]
impl ComposableFunction for KafkaHealthCheck {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        inner()
            .handle(headers.get("type").map(String::as_str))
            .await
    }
}

/// Test seam: the same machinery with an explicit probe-config supplier,
/// timeout and grace — the process-wide instance reads application
/// configuration instead.
pub struct KafkaHealthProbe {
    inner: Arc<HealthInner>,
}

impl KafkaHealthProbe {
    pub fn new(
        probe_config: impl Fn() -> Result<ClientConfig, AppError> + Send + Sync + 'static,
        timeout: Duration,
        grace: Duration,
    ) -> Self {
        KafkaHealthProbe {
            inner: Arc::new(HealthInner::new(probe_config, timeout, grace)),
        }
    }

    /// Serve one `type=info` / `type=health` call (the function contract).
    pub async fn handle(&self, kind: &str) -> Result<EventEnvelope, AppError> {
        self.inner.handle(Some(kind)).await
    }
}

/// Parse a duration expression to whole seconds — bare number = seconds;
/// `s`/`m`/`h`/`d` suffixes scale (the Java `Utility.getDurationInSeconds`
/// surface this module needs).
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
        assert_eq!(Some(1800), duration_seconds("30m"));
        assert_eq!(Some(45), duration_seconds("45"));
        assert_eq!(None, duration_seconds("soon"));
    }
}
