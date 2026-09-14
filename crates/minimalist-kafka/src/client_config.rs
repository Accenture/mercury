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

use platform_core::{AppConfigReader, AppError, ConfigReader};
use rdkafka::config::ClientConfig;

/// Opt-out flag for the producer (default true) — see [`client_enabled`].
pub const PRODUCER_ENABLED: &str = "kafka.producer.enabled";
/// Opt-out flag for the consumer (default true) — see [`client_enabled`].
pub const CONSUMER_ENABLED: &str = "kafka.consumer.enabled";

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
}
