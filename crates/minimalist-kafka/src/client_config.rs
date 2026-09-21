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

//! Kafka client configuration from external **template** files (Java
//! `KafkaClientConfig`), so the wide variety of enterprise installations
//! (on-prem / cloud / SaaS / Confluent; SASL, OAuth2, mTLS) is handled by
//! configuration rather than code. Templates are read via `ConfigReader`
//! (which applies `${ENV_VAR:default}` substitution) from the application's
//! `resources/` by default — `classpath:/kafka-producer.yml` /
//! `kafka-consumer.yml` (`.properties` works too) — falling back to the
//! library's compiled-in defaults, and overridable with the
//! `kafka.producer.properties` / `kafka.consumer.properties` application
//! settings (one location normally; a comma-separated list is a deliberate
//! fallback chain).
//!
//! Keys are **librdkafka parameter names** — identical to the Java client's
//! for connection and security basics. Known JVM-only keys (serializers,
//! `partitioner.class`, `sasl.jaas.config`, callback handler classes) are
//! dropped with a log line naming each one, so a field template ports with a
//! glance at the log rather than a client-build failure; a genuinely unknown
//! key still fails loudly when the client is built. Nothing needs pinning
//! here: this client is byte-native (the Java module pins String/byte[]
//! serializers — that wire contract is librdkafka's only mode). The outbound
//! partitioner **defaults** to `murmur2_random` — Java-producer-compatible
//! murmur2 for keyed records, uniform random for keyless ones: the same
//! semantics as the Java module's `SimpleRandomPartitioner` default, and the
//! same key→partition mapping across engines. A template that sets
//! `partitioner` wins.
//!
//! The consumer side also carries the per-binding overlay the flow adapter
//! applies on top of the template ([`apply_delivery_mode`],
//! [`apply_poll_interval`]) and the `group.protocol=auto` resolution
//! ([`resolve_group_protocol`]).

use std::sync::atomic::{AtomicBool, Ordering};

use platform_core::{AppConfigReader, AppError, ConfigReader};
use rdkafka::config::ClientConfig;

use crate::adapter::KafkaConsumerBinding;

/// Opt-out flag for the producer (default true) — see [`client_enabled`].
pub const PRODUCER_ENABLED: &str = "kafka.producer.enabled";
/// Opt-out flag for the consumer (default true) — see [`client_enabled`].
pub const CONSUMER_ENABLED: &str = "kafka.consumer.enabled";
/// The consumer template's rebalance-protocol key (`classic` | `consumer` | `auto`).
pub const GROUP_PROTOCOL: &str = "group.protocol";
/// The `task://` deadline when a binding sets no `ttl` (Java
/// `KafkaFlowConsumer.DEFAULT_TASK_TTL_MS`).
pub const DEFAULT_TASK_TTL_MS: u64 = 30_000;

const ENABLE_AUTO_COMMIT: &str = "enable.auto.commit";
const QUEUED_MIN_MESSAGES: &str = "queued.min.messages";
const MAX_POLL_INTERVAL_MS: &str = "max.poll.interval.ms";
/// The client's own `max.poll.interval.ms` default (identical to the Java client's).
const KAFKA_DEFAULT_MAX_POLL_INTERVAL_MS: u64 = 300_000;
/// The property's upper bound in this client (24 h) — an absurd envelope saturates here.
const CLIENT_MAX_POLL_INTERVAL_MS: u64 = 86_400_000;
const POLL_INTERVAL_HEADROOM_MS: u64 = 10_000;
/// Client-side tuning that cannot be combined with `group.protocol=consumer`.
const CONSUMER_PROTOCOL_CONFLICTS: [&str; 3] = [
    "session.timeout.ms",
    "heartbeat.interval.ms",
    "partition.assignment.strategy",
];
static AUTO_PROTOCOL_WARNED: AtomicBool = AtomicBool::new(false);

const PRODUCER_LOCATION: &str = "kafka.producer.properties";
const CONSUMER_LOCATION: &str = "kafka.consumer.properties";
const DEFAULT_PRODUCER: &str =
    "classpath:/kafka-producer.yml, classpath:/kafka-producer.properties";
const DEFAULT_CONSUMER: &str =
    "classpath:/kafka-consumer.yml, classpath:/kafka-consumer.properties";

/// The library's compiled-in default templates — the Rust analog of a
/// library-jar classpath resource (the application's `resources/` shadows
/// them by using the default locations above).
const EMBEDDED_PRODUCER: &str = include_str!("../resources/kafka-producer.yml");
const EMBEDDED_CONSUMER: &str = include_str!("../resources/kafka-consumer.yml");

/// JVM-only template keys this engine's client has no analog for — dropped
/// with a log line naming each, so a Java-side field template ports with a
/// glance at the log instead of a client-build failure.
const JVM_ONLY_KEYS: [&str; 8] = [
    "key.serializer",
    "value.serializer",
    "key.deserializer",
    "value.deserializer",
    "partitioner.class",
    "sasl.jaas.config",
    "sasl.login.callback.handler.class",
    "sasl.client.callback.handler.class",
];

/// Whether a Kafka client is enabled — the flag is a **veto, not a trigger**:
/// the default is enabled, and only the literal `false` switches a client
/// off. Its purpose is the one-way leg of a bridge, where the cluster grants
/// credentials for a producer OR a consumer but not both.
pub fn client_enabled(key: &str) -> bool {
    !AppConfigReader::get_instance()
        .get_property_or(key, "true")
        .trim()
        .eq_ignore_ascii_case("false")
}

/// Whether the producer is enabled (`kafka.producer.enabled`, default true).
pub fn producer_enabled() -> bool {
    client_enabled(PRODUCER_ENABLED)
}

/// Whether the consumer is enabled (`kafka.consumer.enabled`, default true).
pub fn consumer_enabled() -> bool {
    client_enabled(CONSUMER_ENABLED)
}

/// Producer client config from the template, with the partitioner defaulted
/// to `murmur2_random` (a template that sets `partitioner` wins).
pub fn producer_client_config() -> Result<ClientConfig, AppError> {
    let locations = AppConfigReader::get_instance().get_property_or(PRODUCER_LOCATION, "");
    let mut config = from_template(&locations, DEFAULT_PRODUCER, EMBEDDED_PRODUCER)?;
    if config.get("partitioner").is_none() {
        config.set("partitioner", "murmur2_random");
    }
    Ok(config)
}

/// Base consumer client config from the template. The per-binding
/// delivery-mode overlay (`group.id`, auto-commit, poll batching) is applied
/// by the flow adapter when it builds each binding's consumer.
pub fn consumer_client_config() -> Result<ClientConfig, AppError> {
    let locations = AppConfigReader::get_instance().get_property_or(CONSUMER_LOCATION, "");
    from_template(&locations, DEFAULT_CONSUMER, EMBEDDED_CONSUMER)
}

/// Client config for the `kafka.health` Metadata probe: the consumer template
/// normally, or the PRODUCER template on a produce-only leg (consumer
/// disabled, producer enabled), reduced to its connection and security
/// parameters — a one-way bridge leg holds credentials for one client only,
/// yet is healthy only when its cluster is reachable. The probe joins no
/// consumer group, so group-related keys (including a `group.protocol` the
/// adapter resolves per binding) are dropped either way.
pub fn health_probe_config() -> Result<ClientConfig, AppError> {
    let config = if consumer_enabled() || !producer_enabled() {
        let mut consumer = consumer_client_config()?;
        consumer.remove("group.protocol");
        consumer
    } else {
        let producer = producer_client_config()?;
        // positive filter: only the connection/security surface crosses over
        // (the Java module filters the producer template to consumer-valid
        // keys; the probe needs nothing beyond how to reach the cluster)
        let mut probe = ClientConfig::new();
        for (key, value) in producer.config_map() {
            if key == "bootstrap.servers"
                || key == "client.id"
                || key.starts_with("security.")
                || key.starts_with("sasl.")
                || key.starts_with("ssl.")
            {
                probe.set(key, value);
            }
        }
        probe
    };
    Ok(config)
}

/// The per-binding delivery-mode overlay (Java `applyDeliveryMode`) — the one
/// place that decides the commit contract, so the base template cannot
/// contradict it: `enable.auto.commit` exactly as the binding says (manual
/// commit-after-process by default; the client's own timer under
/// `auto-commit: true`), and an explicit `max-poll-records` mapped to this
/// client's per-partition prefetch depth (`queued.min.messages`).
///
/// **Delta from the Java module (port spec §7 item 5):** librdkafka has no
/// `max.poll.records` — per-record delivery IS the poll batch of one — so the
/// mode defaults (1 manual / 500 auto-commit) are not applied: manual mode's
/// batch-of-one is inherent, and auto-commit mode keeps the client's own
/// prefetch defaults. Only an explicit value is mapped, and the mapping is
/// stated in the startup log.
pub fn apply_delivery_mode(config: &mut ClientConfig, binding: &KafkaConsumerBinding) {
    config.set(
        ENABLE_AUTO_COMMIT,
        if binding.auto_commit { "true" } else { "false" },
    );
    if let Some(records) = binding.max_poll_records {
        config.set(QUEUED_MIN_MESSAGES, records.to_string());
        log::info!(
            "Binding {} maps max-poll-records={records} to {QUEUED_MIN_MESSAGES} (this client's \
             per-partition prefetch depth; per-record delivery is the poll batch of one)",
            binding.label()
        );
    }
}

/// Guard against poll-task eviction (Java `applyPollInterval`). Message
/// processing happens between two polls — a target's own `ttl` is its
/// deadline — so the worst case is the binding's full retry envelope:
/// `(max_retries + 1) x` the slowest reachable target ttl `+ max_retries x
/// backoff`, plus headroom (one record per poll here: per-record delivery).
/// If that exceeds the client's `max.poll.interval.ms` (default 5 minutes),
/// the group coordinator evicts the consumer mid-processing and the following
/// commit fails. The interval is derived from the envelope, never lowered
/// below the client default; an explicit value in the consumer template is an
/// operator decision and is respected as-is, with a `WARN` when the computed
/// envelope exceeds it. Raising the interval is low-risk: crash liveness is
/// detected by heartbeats — this setting only bounds time between polls.
pub fn apply_poll_interval(
    config: &mut ClientConfig,
    binding: &KafkaConsumerBinding,
    max_retries: u32,
    backoff_ms: u64,
) {
    let retries = u64::from(max_retries);
    let envelope = (retries + 1)
        .saturating_mul(max_target_ttl_ms(binding))
        .saturating_add(retries.saturating_mul(backoff_ms))
        .saturating_add(POLL_INTERVAL_HEADROOM_MS)
        .min(CLIENT_MAX_POLL_INTERVAL_MS);
    if let Some(explicit) = config.get(MAX_POLL_INTERVAL_MS) {
        if let Ok(configured) = explicit.trim().parse::<u64>() {
            if envelope > configured {
                log::warn!(
                    "Binding {} sets {MAX_POLL_INTERVAL_MS}={configured} but its worst-case retry \
                     envelope is {envelope} ms - a slow-failing message may get this consumer \
                     evicted from the group mid-processing",
                    binding.label()
                );
            }
        }
        return;
    }
    let derived = envelope.max(KAFKA_DEFAULT_MAX_POLL_INTERVAL_MS);
    config.set(MAX_POLL_INTERVAL_MS, derived.to_string());
    if derived > KAFKA_DEFAULT_MAX_POLL_INTERVAL_MS {
        log::info!(
            "Binding {} derives {MAX_POLL_INTERVAL_MS}={derived} from its worst-case retry envelope \
             (slowest target ttl x retries + backoff + headroom)",
            binding.label()
        );
    }
}

/// The slowest reachable target deadline for a binding: flow targets use the
/// compiled flow's own ttl (every target was validated to exist at startup);
/// `task://` targets use the binding's task ttl.
fn max_target_ttl_ms(binding: &KafkaConsumerBinding) -> u64 {
    let task_ttl = binding.task_ttl_ms.unwrap_or(DEFAULT_TASK_TTL_MS);
    match &binding.routing {
        None => flow_ttl_ms(binding.flow_id.as_deref().unwrap_or_default(), task_ttl),
        Some(rules) => rules
            .all_targets()
            .iter()
            .map(|target| {
                if target.task {
                    task_ttl
                } else {
                    flow_ttl_ms(&target.destination, task_ttl)
                }
            })
            .max()
            .unwrap_or(task_ttl),
    }
}

/// A compiled flow's ttl, or the fallback for a flow not in the registry
/// (defensive; validated earlier).
fn flow_ttl_ms(flow_id: &str, fallback: u64) -> u64 {
    event_script::flows::get_flow(flow_id)
        .map(|flow| flow.ttl)
        .unwrap_or(fallback)
}

/// Resolve a template `group.protocol=auto` in place (Java
/// `GroupProtocolResolver`); `classic` and `consumer` pass through verbatim —
/// this client supports the KIP-848 `consumer` protocol natively.
///
/// **Delta from the Java module (port spec §7 item 7):** the Java resolver
/// probes the cluster's finalized `group.version` feature through the Admin
/// client's `describeFeatures`. librdkafka exposes no feature probe, and every
/// side-effect-free alternative was rejected (a broker config is not the
/// feature flag; a trial join churns a real group), so on this engine `auto`
/// resolves to `classic` — every broker's safe answer — with a `WARN` that
/// names the explicit setting for a KIP-848 cluster. The conflict guard is
/// kept: a template that also sets client-side tuning the consumer protocol
/// does not allow resolves to `classic` naming the keys, as the Java module
/// does.
pub fn resolve_group_protocol(config: &mut ClientConfig) {
    let auto = config
        .get(GROUP_PROTOCOL)
        .is_some_and(|value| value.trim().eq_ignore_ascii_case("auto"));
    if !auto {
        return;
    }
    let conflicts: Vec<&str> = CONSUMER_PROTOCOL_CONFLICTS
        .iter()
        .copied()
        .filter(|key| config.get(key).is_some())
        .collect();
    config.set(GROUP_PROTOCOL, "classic");
    if !conflicts.is_empty() {
        log::warn!(
            "{GROUP_PROTOCOL}=auto resolved to classic - {conflicts:?} cannot be used with the \
             consumer rebalance protocol; remove the setting(s) before selecting \
             {GROUP_PROTOCOL}=consumer"
        );
    } else if !AUTO_PROTOCOL_WARNED.swap(true, Ordering::AcqRel) {
        log::warn!(
            "{GROUP_PROTOCOL}=auto resolved to classic - this client (librdkafka) exposes no cluster \
             feature probe, so 'auto' cannot detect a KIP-848 cluster the way the Java module does; \
             on an Apache Kafka 4.0+ cluster with group.version >= 1, set {GROUP_PROTOCOL}=consumer \
             explicitly"
        );
    } else {
        log::debug!("{GROUP_PROTOCOL}=auto resolved to classic (stated once per process)");
    }
}

/// Load a template into a [`ClientConfig`]: the configured locations when the
/// application set the key (exactly those, loudly failing when none exists —
/// a misdirected override must not silently fall back), or the default
/// classpath chain with the compiled-in template as the final fallback.
fn from_template(
    configured: &str,
    default_locations: &str,
    embedded: &str,
) -> Result<ClientConfig, AppError> {
    let reader = if configured.trim().is_empty() {
        match load_first(default_locations) {
            Some(reader) => reader,
            None => ConfigReader::from_yaml_text(embedded)
                .map_err(|e| AppError::new(500, format!("Bundled Kafka template invalid - {e}")))?,
        }
    } else {
        load_first(configured).ok_or_else(|| {
            AppError::new(
                500,
                format!("No Kafka client config found at any of: {configured}"),
            )
        })?
    };
    let mut config = ClientConfig::new();
    for (key, value) in reader.get_composite_key_values() {
        if JVM_ONLY_KEYS.contains(&key.as_str()) {
            log::info!("Ignoring JVM-only Kafka template key '{key}' (librdkafka client)");
            continue;
        }
        config.set(key, value.to_display_string());
    }
    Ok(config)
}

/// Try each comma-separated location in order, returning the first that
/// loads (Java `loadFirst`).
fn load_first(locations: &str) -> Option<ConfigReader> {
    locations
        .split(',')
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .find_map(|path| ConfigReader::load(path).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The compiled-in templates parse, enumerate flat librdkafka keys, and
    /// carry the connection default.
    #[test]
    fn embedded_templates_parse_to_flat_keys() {
        for embedded in [EMBEDDED_PRODUCER, EMBEDDED_CONSUMER] {
            let reader = ConfigReader::from_yaml_text(embedded).expect("template parses");
            let keys = reader.get_composite_key_values();
            assert!(
                keys.contains_key("bootstrap.servers"),
                "flat dotted key survives: {keys:?}"
            );
        }
    }

    #[test]
    fn jvm_only_keys_are_dropped_and_partitioner_defaults() {
        let reader = ConfigReader::from_yaml_text(
            "bootstrap.servers: '127.0.0.1:9'\nkey.serializer: org.apache.kafka.X\nsasl.jaas.config: secret\nacks: all\n",
        )
        .expect("parses");
        let mut config = ClientConfig::new();
        for (key, value) in reader.get_composite_key_values() {
            if JVM_ONLY_KEYS.contains(&key.as_str()) {
                continue;
            }
            config.set(key, value.to_display_string());
        }
        assert_eq!(
            Some(&"all".to_string()),
            config.get("acks").map(str::to_string).as_ref()
        );
        assert!(config.get("key.serializer").is_none());
        assert!(config.get("sasl.jaas.config").is_none());
    }

    /// The delivery-mode overlay pins the commit contract; only an EXPLICIT
    /// max-poll-records maps to the prefetch depth.
    #[test]
    fn delivery_mode_overlay_pins_the_commit_contract() {
        let manual = KafkaConsumerBinding::direct("orders", "f");
        let mut config = ClientConfig::new();
        apply_delivery_mode(&mut config, &manual);
        assert_eq!(Some("false"), config.get(ENABLE_AUTO_COMMIT));
        assert!(
            config.get(QUEUED_MIN_MESSAGES).is_none(),
            "no mode default is applied"
        );

        let mut auto = KafkaConsumerBinding::direct("clicks", "f");
        auto.auto_commit = true;
        auto.max_poll_records = Some(50);
        let mut config = ClientConfig::new();
        apply_delivery_mode(&mut config, &auto);
        assert_eq!(Some("true"), config.get(ENABLE_AUTO_COMMIT));
        assert_eq!(Some("50"), config.get(QUEUED_MIN_MESSAGES));
    }

    /// (retries + 1) x slowest target ttl + retries x backoff + headroom, one
    /// record per poll — never below the client default, saturating at its
    /// upper bound, and an explicit template value is respected.
    #[test]
    fn poll_interval_is_derived_from_the_retry_envelope() {
        // no compiled flow in a unit test -> the 30 s task-ttl fallback:
        // (3 + 1) x 30000 + 3 x 500 + 10000 = 131500 < the 300000 default
        let small = KafkaConsumerBinding::direct("orders", "f");
        let mut config = ClientConfig::new();
        apply_poll_interval(&mut config, &small, 3, 500);
        assert_eq!(Some("300000"), config.get(MAX_POLL_INTERVAL_MS));

        let mut slow = KafkaConsumerBinding::direct("slow", "f");
        slow.task_ttl_ms = Some(600_000);
        let mut config = ClientConfig::new();
        apply_poll_interval(&mut config, &slow, 3, 500);
        assert_eq!(Some("2411500"), config.get(MAX_POLL_INTERVAL_MS));

        let mut absurd = KafkaConsumerBinding::direct("absurd", "f");
        absurd.task_ttl_ms = Some(u64::MAX / 2);
        let mut config = ClientConfig::new();
        apply_poll_interval(&mut config, &absurd, 3, 500);
        assert_eq!(Some("86400000"), config.get(MAX_POLL_INTERVAL_MS));

        let mut config = ClientConfig::new();
        config.set(MAX_POLL_INTERVAL_MS, "1000");
        apply_poll_interval(&mut config, &slow, 3, 500);
        assert_eq!(
            Some("1000"),
            config.get(MAX_POLL_INTERVAL_MS),
            "an operator's explicit interval is respected (with a WARN)"
        );
    }

    /// `auto` resolves to classic on this engine (with or without the conflict
    /// guard); explicit values and an absent key pass through untouched.
    #[test]
    fn group_protocol_auto_resolves_to_classic() {
        let mut config = ClientConfig::new();
        config.set(GROUP_PROTOCOL, " Auto ");
        resolve_group_protocol(&mut config);
        assert_eq!(Some("classic"), config.get(GROUP_PROTOCOL));

        let mut conflicting = ClientConfig::new();
        conflicting.set(GROUP_PROTOCOL, "auto");
        conflicting.set("session.timeout.ms", "45000");
        resolve_group_protocol(&mut conflicting);
        assert_eq!(Some("classic"), conflicting.get(GROUP_PROTOCOL));
        assert_eq!(
            Some("45000"),
            conflicting.get("session.timeout.ms"),
            "the operator's tuning is respected, not discarded"
        );

        for explicit in ["consumer", "classic"] {
            let mut config = ClientConfig::new();
            config.set(GROUP_PROTOCOL, explicit);
            resolve_group_protocol(&mut config);
            assert_eq!(Some(explicit), config.get(GROUP_PROTOCOL));
        }
        let mut absent = ClientConfig::new();
        resolve_group_protocol(&mut absent);
        assert!(absent.get(GROUP_PROTOCOL).is_none());
    }
}
