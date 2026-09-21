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

//! `kafka-flow-adapter.yaml` parsing and startup validation (Java
//! `KafkaFlowAdapter` + `KafkaConsumerBinding`). A `consumer:` binding names
//! its **source** — a literal `topic` or a regex `topic-pattern` (exactly one)
//! — and its **destination** — one `flow`, or a `flows` rule list for
//! second-level routing (exactly one; see [`crate::routing`]) — plus the
//! optional knobs: `group` (default `kafka-flow-adapter.<topic>`; required for
//! a pattern), `partition` pinning (literal topics only), `serializer: 'json'`
//! (best-effort JSON decode before routing), `ttl` for `task://` targets,
//! `dlq-topic`, `auto-commit`, `max-poll-records`, and the per-binding
//! header-name overrides (`correlation.id.header`, `trace.id.header`,
//! `traceparent.header`). Every value supports `${ENV_VAR:default}`
//! substitution (the file is read by `ConfigReader`).
//!
//! **A malformed entry fails startup fast and loud** rather than being
//! silently skipped — the Java guide's whole validation list: the
//! source/destination exclusivity rules, an invalid regex, a pattern combined
//! with `partition` or without an explicit `group`, a `dlq-topic` that equals
//! or matches its own source, a malformed routing rule, an unknown flow or
//! task route, an unsupported `serializer`, a non-positive `ttl`, `partition`
//! or `max-poll-records`. `schema.enabled` — the Schema Registry decode — is
//! the one field still deferred (port spec §9, Q2) and is rejected by name
//! rather than ignored.

use platform_core::{AppError, ConfigReader, Platform};
use regex::Regex;

use crate::routing::RoutingRuleSet;

const DEFAULT_GROUP_PREFIX: &str = "kafka-flow-adapter";
const SERIALIZER_JSON: &str = "json";

/// Adapter fields deferred to the post-K5 Schema Registry spec (port spec §9,
/// Q2) — present in the Java module, deliberately not served yet. Naming them
/// beats ignoring them.
const DEFERRED_FIELDS: [&str; 1] = ["schema.enabled"];

/// One validated `consumer[]` binding.
#[derive(Clone, Debug)]
pub struct KafkaConsumerBinding {
    /// Literal topic name, or the regex text when [`Self::pattern`] is true.
    pub topic_or_pattern: String,
    /// True for a `topic-pattern` binding (regex subscription).
    pub pattern: bool,
    /// The direct-routing flow id, or `None` when the binding uses
    /// second-level routing (`flows`).
    pub flow_id: Option<String>,
    /// The compiled `flows` rules, or `None` for a direct `flow` binding —
    /// exactly one of the two is set.
    pub routing: Option<RoutingRuleSet>,
    /// The consumer group id, used exactly as given.
    pub group_id: String,
    /// Set only when the binding pins a single partition (manual assignment;
    /// mutually exclusive with a pattern).
    pub partition: Option<i32>,
    /// `serializer: 'json'` — best-effort JSON decode of the record value
    /// before routing (a JSON object or array; anything else keeps the bytes).
    pub json_serializer: bool,
    /// The per-binding `ttl` in ms — the deadline for a `task://` target,
    /// which has no flow ttl of its own; `None` = the consumer's 30 s default.
    pub task_ttl_ms: Option<u64>,
    /// The binding's dead-letter topic, or `None` (exhausted messages are
    /// dropped with an ERROR log).
    pub dlq_topic: Option<String>,
    /// False (default) = manual commit after each successful delivery; true =
    /// Kafka-native auto-commit on the client's timer.
    pub auto_commit: bool,
    /// Explicit `max-poll-records`, mapped to this client's per-partition
    /// prefetch depth (see `client_config::apply_delivery_mode`).
    pub max_poll_records: Option<u32>,
    /// Per-binding override of the global `kafka.trace.id.header`.
    pub trace_id_header: Option<String>,
    /// Per-binding override of the global `kafka.correlation.id.header`.
    pub correlation_id_header: Option<String>,
    /// Per-binding override of the global `kafka.traceparent.header`.
    pub traceparent_header: Option<String>,
}

impl KafkaConsumerBinding {
    /// A literal-topic binding routed to one flow with every default — the
    /// seed for tests and programmatic callers.
    pub fn direct(topic: &str, flow_id: &str) -> Self {
        KafkaConsumerBinding {
            topic_or_pattern: topic.to_string(),
            pattern: false,
            flow_id: Some(flow_id.to_string()),
            routing: None,
            group_id: format!("{DEFAULT_GROUP_PREFIX}.{topic}"),
            partition: None,
            json_serializer: false,
            task_ttl_ms: None,
            dlq_topic: None,
            auto_commit: false,
            max_poll_records: None,
            trace_id_header: None,
            correlation_id_header: None,
            traceparent_header: None,
        }
    }

    /// Display label: `topic 'x'` or `topic-pattern 'x'`.
    pub fn label(&self) -> String {
        if self.pattern {
            format!("topic-pattern '{}'", self.topic_or_pattern)
        } else {
            format!("topic '{}'", self.topic_or_pattern)
        }
    }

    /// The client's subscription form of a `topic-pattern`: `^`-prefixed (the
    /// client's regex marker) and anchored to the whole topic name, so the
    /// match is full-string like the Java module's `subscribe(Pattern)`.
    pub fn subscription_regex(&self) -> String {
        anchored(&self.topic_or_pattern)
    }
}

/// Full-string anchoring shared by the startup validation, the dlq check and
/// the subscription: a group keeps a top-level alternation inside the anchors.
/// A plain capturing group, deliberately — the client's regex engine rejects
/// the `(?:` non-capturing form in a subscription (probed against librdkafka
/// 2.12), and a capture is harmless here.
fn anchored(pattern: &str) -> String {
    format!("^({pattern})$")
}

/// Parse and validate the adapter configuration. Cross-reference checks (a
/// flow must be compiled, a task route registered) run against the live
/// registries, so this is called from the main-application hook — after
/// `CompileFlows` and function preload.
pub fn parse_bindings(reader: &ConfigReader) -> Result<Vec<KafkaConsumerBinding>, AppError> {
    let consumer = reader
        .get("consumer")
        .ok_or_else(|| bad_config("missing the 'consumer' binding list"))?
        .to_json();
    let serde_json::Value::Array(entries) = consumer else {
        return Err(bad_config("'consumer' must be a list of bindings"));
    };
    if entries.is_empty() {
        return Err(bad_config("'consumer' has no bindings"));
    }
    let mut bindings = Vec::with_capacity(entries.len());
    for (i, entry) in entries.iter().enumerate() {
        let binding = parse_binding_shape(i, entry)?;
        cross_reference(i, &binding)?;
        bindings.push(binding);
    }
    Ok(bindings)
}

/// Everything that validates one entry's OWN shape — no registry needed.
/// Cross-references run last, in [`cross_reference`] (Java parity: shape
/// checks first, external wiring after).
fn parse_binding_shape(
    i: usize,
    entry: &serde_json::Value,
) -> Result<KafkaConsumerBinding, AppError> {
    let serde_json::Value::Object(_) = entry else {
        return Err(bad_config(&format!(
            "consumer[{i}] must be a map with 'topic' and 'flow'"
        )));
    };
    for field in DEFERRED_FIELDS {
        if nested_text(entry, field).is_some() {
            return Err(bad_config(&format!(
                "consumer[{i}] sets '{field}', which belongs to the Schema Registry phase of this \
                 port - deferred to its own spec (see draft-design-specs/minimalist-kafka-port.md §9, Q2)"
            )));
        }
    }
    // --- the source: exactly one of topic / topic-pattern
    let topic = text(entry, "topic");
    let topic_pattern = text(entry, "topic-pattern");
    let (source, pattern) = match (&topic, &topic_pattern) {
        (None, None) => {
            return Err(bad_config(&format!(
                "consumer[{i}] is missing a 'topic' or 'topic-pattern'"
            )))
        }
        (Some(_), Some(_)) => {
            return Err(bad_config(&format!(
                "consumer[{i}] cannot set both 'topic' and 'topic-pattern'"
            )))
        }
        (Some(topic), None) => (topic.clone(), false),
        (None, Some(topic_pattern)) => (topic_pattern.clone(), true),
    };
    let compiled_pattern = if pattern {
        Some(Regex::new(&anchored(&source)).map_err(|e| {
            bad_config(&format!(
                "consumer[{i}] (topic-pattern '{source}') is not a valid regex: {e}"
            ))
        })?)
    } else {
        None
    };
    let label = if pattern {
        format!("topic-pattern '{source}'")
    } else {
        format!("topic '{source}'")
    };
    // --- the destination: exactly one of flow / flows
    let flow_id = text(entry, "flow");
    let routing = resolve_routing(i, &label, entry, flow_id.as_deref())?;
    // --- the optional knobs
    let json_serializer = match text(entry, "serializer") {
        None => false,
        Some(serializer) => {
            if !serializer.eq_ignore_ascii_case(SERIALIZER_JSON) {
                return Err(bad_config(&format!(
                    "consumer[{i}] ({label}) unsupported 'serializer' '{other}' - only 'json' is \
                     supported",
                    other = serializer
                )));
            }
            true
        }
    };
    let task_ttl_ms = match text(entry, "ttl") {
        None => None,
        Some(ttl) => Some(parse_duration_ms(&ttl).ok_or_else(|| {
            bad_config(&format!(
                "consumer[{i}] ({label}) 'ttl' must be a positive duration (e.g. '30s'), got '{ttl}'"
            ))
        })?),
    };
    let partition = match text(entry, "partition") {
        None => None,
        Some(value) => Some(
            parse_partition(&value)
                .map_err(|m| bad_config(&format!("consumer[{i}] ({label}) {m}")))?,
        ),
    };
    if pattern && partition.is_some() {
        return Err(bad_config(&format!(
            "consumer[{i}] ({label}) cannot combine 'topic-pattern' with 'partition' - manual \
             partition assignment requires a literal 'topic'"
        )));
    }
    let group_id = match text(entry, "group") {
        Some(group) => group,
        None => {
            if pattern {
                return Err(bad_config(&format!(
                    "consumer[{i}] ({label}) 'topic-pattern' requires an explicit 'group' - no \
                     sensible default exists for a pattern-based binding"
                )));
            }
            format!("{DEFAULT_GROUP_PREFIX}.{source}")
        }
    };
    let dlq_topic = text(entry, "dlq-topic");
    if let Some(dlq) = &dlq_topic {
        if !pattern && *dlq == source {
            return Err(bad_config(&format!(
                "consumer[{i}] ({label}) uses its own source topic as 'dlq-topic' - an exhausted \
                 message would loop forever"
            )));
        }
        if let Some(regex) = &compiled_pattern {
            if regex.is_match(dlq) {
                return Err(bad_config(&format!(
                    "consumer[{i}] ({label}) 'dlq-topic' ('{dlq}') must not match 'topic-pattern' - \
                     it would re-consume its own dead letters"
                )));
            }
        }
    }
    let auto_commit = text(entry, "auto-commit").is_some_and(|v| v.eq_ignore_ascii_case("true"));
    let max_poll_records = match text(entry, "max-poll-records") {
        None => None,
        Some(value) => Some(
            parse_max_poll_records(&value)
                .map_err(|m| bad_config(&format!("consumer[{i}] ({label}) {m}")))?,
        ),
    };
    Ok(KafkaConsumerBinding {
        topic_or_pattern: source,
        pattern,
        flow_id,
        routing,
        group_id,
        partition,
        json_serializer,
        task_ttl_ms,
        dlq_topic,
        auto_commit,
        max_poll_records,
        trace_id_header: nested_text(entry, "trace.id.header"),
        correlation_id_header: nested_text(entry, "correlation.id.header"),
        traceparent_header: nested_text(entry, "traceparent.header"),
    })
}

/// Fail fast when a binding names a flow that was never compiled or a task
/// route that is not in the platform registry — CompileFlows and function
/// preload both run before this adapter starts, so target existence is
/// checkable here rather than failing every message at runtime.
fn cross_reference(i: usize, binding: &KafkaConsumerBinding) -> Result<(), AppError> {
    let label = binding.label();
    match &binding.routing {
        Some(rules) => {
            for target in rules.all_targets() {
                if target.task {
                    // the flow engine is a registered route, but addressing it as a
                    // bare task would bypass the flow-launch contract (no flow_id)
                    if target.destination == event_script::manager::SERVICE_NAME {
                        return Err(bad_config(&format!(
                            "consumer[{i}] ({label}) 'task://{}' is not allowed - use \
                             'flow://<flow-id>' to dispatch to the flow engine",
                            event_script::manager::SERVICE_NAME
                        )));
                    }
                    if !Platform::get_instance().has_route(&target.destination) {
                        return Err(bad_config(&format!(
                            "consumer[{i}] ({label}) references unknown task route '{}'",
                            target.destination
                        )));
                    }
                } else if event_script::flows::get_flow(&target.destination).is_none() {
                    return Err(bad_config(&format!(
                        "consumer[{i}] ({label}) references unknown flow '{}'",
                        target.destination
                    )));
                }
            }
        }
        None => {
            let flow_id = binding.flow_id.as_deref().unwrap_or_default();
            if event_script::flows::get_flow(flow_id).is_none() {
                return Err(bad_config(&format!(
                    "consumer[{i}] ({label}) references unknown flow '{flow_id}'"
                )));
            }
        }
    }
    Ok(())
}

/// Resolve the binding's routing: exactly one of `flow` (direct routing) or
/// `flows` (second-level routing rules) must be set. Returns the compiled rule
/// set for `flows`, or `None` for direct `flow` routing.
fn resolve_routing(
    i: usize,
    label: &str,
    entry: &serde_json::Value,
    flow_id: Option<&str>,
) -> Result<Option<RoutingRuleSet>, AppError> {
    let flows = &entry["flows"];
    match (flow_id, flows.is_null()) {
        (Some(_), false) => Err(bad_config(&format!(
            "consumer[{i}] ({label}) cannot set both 'flow' and 'flows' - use 'flow' for direct \
             routing or 'flows' for second-level routing"
        ))),
        (None, true) => Err(bad_config(&format!(
            "consumer[{i}] ({label}) is missing a 'flow' or 'flows'"
        ))),
        (Some(_), true) => Ok(None),
        (None, false) => {
            let items = match flows {
                serde_json::Value::Array(items) if !items.is_empty() => items,
                _ => {
                    return Err(bad_config(&format!(
                        "consumer[{i}] ({label}) 'flows' must be a non-empty list of routing rules"
                    )))
                }
            };
            let mut rules = Vec::with_capacity(items.len());
            for item in items {
                match value_text(item) {
                    Some(rule) => rules.push(rule),
                    None => {
                        return Err(bad_config(&format!(
                            "consumer[{i}] ({label}) 'flows' contains an empty routing rule"
                        )))
                    }
                }
            }
            RoutingRuleSet::compile(&rules)
                .map(Some)
                .map_err(|m| bad_config(&format!("consumer[{i}] ({label}) {m}")))
        }
    }
}

/// The dead-letter startup guard (Java
/// `KafkaFlowAdapter.rejectDeadLetterWithoutProducer`): dead letters are
/// published through this cluster's own producer, so a binding that declares
/// `dlq-topic` while the producer is switched off **fails the deployment at
/// startup**, naming both settings — without the guard an exhausted message
/// would be dropped with a DATA LOSS log and its offset committed.
pub fn reject_dead_letter_without_producer(
    bindings: &[KafkaConsumerBinding],
    producer_enabled_key: &str,
) -> Result<(), AppError> {
    for binding in bindings {
        if let Some(dlq) = &binding.dlq_topic {
            return Err(bad_config(&format!(
                "binding {} declares dlq-topic '{dlq}' but {producer_enabled_key}=false - dead \
                 letters are published through this cluster's own producer; enable the producer or \
                 drop the dlq-topic",
                binding.label()
            )));
        }
    }
    Ok(())
}

/// A `ttl` duration in milliseconds: a positive integer with an `s`/`m`/`h`/`d`
/// suffix (case-insensitive), or bare seconds. Zero, negative, fractional,
/// unknown-suffix and overflowing inputs are `None` — the Java module's
/// long-math twin: an absurd duration is rejected, never silently wrapped.
fn parse_duration_ms(text: &str) -> Option<u64> {
    let text = text.trim();
    let last = text.chars().last()?;
    let (number, multiplier) = if last.is_ascii_digit() {
        (text, 1u64)
    } else {
        let multiplier = match last.to_ascii_lowercase() {
            's' => 1,
            'm' => 60,
            'h' => 3600,
            'd' => 86_400,
            _ => return None,
        };
        (text[..text.len() - last.len_utf8()].trim(), multiplier)
    };
    if number.is_empty() || !number.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let seconds = number.parse::<u64>().ok()?.checked_mul(multiplier)?;
    if seconds == 0 {
        return None;
    }
    seconds.checked_mul(1000)
}

/// The optional `partition` for pinning: a non-negative integer.
fn parse_partition(text: &str) -> Result<i32, String> {
    let partition = text
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("'partition' must be an integer, got '{text}'"))?;
    if partition < 0 {
        return Err(format!("'partition' must be >= 0, got {partition}"));
    }
    Ok(partition)
}

/// The optional `max-poll-records`: a positive integer.
fn parse_max_poll_records(text: &str) -> Result<u32, String> {
    let records = text
        .trim()
        .parse::<i64>()
        .map_err(|_| format!("'max-poll-records' must be an integer, got '{text}'"))?;
    if records <= 0 {
        return Err(format!("'max-poll-records' must be > 0, got {records}"));
    }
    u32::try_from(records).map_err(|_| format!("'max-poll-records' is out of range, got {records}"))
}

/// A dotted field may arrive nested (ConfigReader normalizes dotted YAML keys
/// into nested maps — `trace.id.header` becomes `{trace: {id: {header}}}`) or
/// flat (an authored map) — accept both, like the Java module's `nestedText`.
fn nested_text(entry: &serde_json::Value, flat_key: &str) -> Option<String> {
    if let Some(direct) = text(entry, flat_key) {
        return Some(direct);
    }
    let mut node = entry;
    for segment in flat_key.split('.') {
        node = &node[segment];
        if node.is_null() {
            return None;
        }
    }
    value_text(node)
}

fn text(entry: &serde_json::Value, key: &str) -> Option<String> {
    value_text(&entry[key])
}

/// The trimmed text of a value, or `None` when absent or blank (Java `text`).
fn value_text(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Null => None,
        serde_json::Value::String(s) if s.trim().is_empty() => None,
        serde_json::Value::String(s) => Some(s.trim().to_string()),
        other => Some(other.to_string()),
    }
}

fn bad_config(message: &str) -> AppError {
    AppError::new(400, format!("kafka-flow-adapter: {message}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reader(yaml: &str) -> ConfigReader {
        ConfigReader::from_yaml_text(yaml).expect("yaml parses")
    }

    /// The full path (shape + cross-reference) for one entry — every negative
    /// case fails before the registry is consulted.
    fn error_of(yaml: &str) -> String {
        parse_bindings(&reader(yaml))
            .expect_err("rejected")
            .message()
            .to_string()
    }

    /// Shape only — the positive cases, which need no compiled flow registry.
    fn shape_of(yaml: &str) -> KafkaConsumerBinding {
        let entries = reader(yaml)
            .get("consumer")
            .expect("consumer list")
            .to_json();
        parse_binding_shape(0, &entries[0]).expect("shape accepted")
    }

    #[test]
    fn malformed_entries_fail_startup_by_name() {
        assert!(error_of("consumer:\n  - flow: f1\n")
            .contains("consumer[0] is missing a 'topic' or 'topic-pattern'"));
        assert!(error_of("consumer:\n  - topic: orders\n")
            .contains("consumer[0] (topic 'orders') is missing a 'flow' or 'flows'"));
        assert!(
            error_of("consumer:\n  - topic: orders\n    flow: no-such-flow\n")
                .contains("references unknown flow 'no-such-flow'")
        );
        assert!(error_of("consumer: []\n").contains("no bindings"));
        assert!(error_of("producer: {}\n").contains("missing the 'consumer' binding list"));
        assert!(error_of("consumer:\n  - just-a-string\n").contains("must be a map"));
    }

    #[test]
    fn schema_decode_is_deferred_by_name() {
        for yaml in [
            "consumer:\n  - topic: t\n    flow: f\n    schema.enabled: true\n",
            "consumer:\n  - topic: t\n    flow: f\n    schema:\n      enabled: true\n",
        ] {
            let error = error_of(yaml);
            assert!(
                error.contains("schema.enabled") && error.contains("Schema Registry"),
                "{error}"
            );
        }
    }

    #[test]
    fn source_selector_rules() {
        assert!(
            error_of("consumer:\n  - topic: t\n    topic-pattern: 'x.*'\n    flow: f\n")
                .contains("cannot set both 'topic' and 'topic-pattern'")
        );
        assert!(
            error_of("consumer:\n  - topic-pattern: '(unclosed'\n    flow: f\n    group: g\n")
                .contains("is not a valid regex")
        );
        assert!(
            error_of("consumer:\n  - topic-pattern: 'events\\.[a-z]{2}'\n    flow: f\n    group: g\n    partition: 0\n")
                .contains("cannot combine 'topic-pattern' with 'partition'")
        );
        assert!(
            error_of("consumer:\n  - topic-pattern: 'events\\.[a-z]{2}'\n    flow: f\n")
                .contains("requires an explicit 'group'")
        );
        let pattern = shape_of("consumer:\n  - topic-pattern: 'events\\.[a-z]{2}'\n    flow: f\n    group: 'region-group'\n");
        assert!(pattern.pattern);
        assert_eq!("region-group", pattern.group_id);
        assert_eq!("^(events\\.[a-z]{2})$", pattern.subscription_regex());
        assert_eq!("topic-pattern 'events\\.[a-z]{2}'", pattern.label());
    }

    #[test]
    fn destination_rules() {
        assert!(error_of(
            "consumer:\n  - topic: t\n    flow: f\n    flows:\n      - 'default -> flow://f'\n"
        )
        .contains("cannot set both 'flow' and 'flows'"));
        assert!(error_of("consumer:\n  - topic: t\n    flows: []\n")
            .contains("'flows' must be a non-empty list"));
        assert!(
            error_of("consumer:\n  - topic: t\n    flows: 'default -> flow://f'\n")
                .contains("'flows' must be a non-empty list")
        );
        assert!(
            error_of("consumer:\n  - topic: t\n    flows:\n      - ''\n")
                .contains("contains an empty routing rule")
        );
        let malformed = error_of(
            "consumer:\n  - topic: t\n    flows:\n      - 'input.header.type(order) flow://f'\n",
        );
        assert!(
            malformed.contains("consumer[0] (topic 't') routing rule"),
            "the rule error carries the binding prefix: {malformed}"
        );
        assert!(
            error_of("consumer:\n  - topic: t\n    flows:\n      - 'default -> task://event.script.manager'\n")
                .contains("'task://event.script.manager' is not allowed")
        );
        assert!(error_of(
            "consumer:\n  - topic: t\n    flows:\n      - 'default -> flow://no-such-flow'\n"
        )
        .contains("references unknown flow 'no-such-flow'"));
        let routed = shape_of(
            "consumer:\n  - topic: mixed\n    serializer: 'json'\n    ttl: '10s'\n    flows:\n      - 'input.header.type(order) -> flow://order'\n      - 'default -> task://sink'\n",
        );
        assert!(routed.flow_id.is_none());
        assert_eq!(1, routed.routing.as_ref().expect("rules").size());
        assert!(routed.json_serializer);
        assert_eq!(Some(10_000), routed.task_ttl_ms);
    }

    #[test]
    fn serializer_accepts_only_json() {
        assert!(
            shape_of("consumer:\n  - topic: t\n    flow: f\n    serializer: JSON\n")
                .json_serializer
        );
        assert!(!shape_of("consumer:\n  - topic: t\n    flow: f\n").json_serializer);
        assert!(
            error_of("consumer:\n  - topic: t\n    flow: f\n    serializer: avro\n")
                .contains("unsupported 'serializer' 'avro' - only 'json' is supported")
        );
    }

    #[test]
    fn task_ttl_accepts_duration_syntax_and_rejects_the_rest() {
        assert_eq!(Some(30_000), parse_duration_ms("30s"));
        assert_eq!(Some(300_000), parse_duration_ms("5m"));
        assert_eq!(Some(3_600_000), parse_duration_ms("1H"));
        assert_eq!(Some(172_800_000), parse_duration_ms("2d"));
        assert_eq!(Some(45_000), parse_duration_ms("45"), "no suffix = seconds");
        for bad in [
            "0s",
            "-1",
            "abc",
            "10x",
            "1.5s",
            "",
            "s",
            "99999999999999999999d",
        ] {
            assert_eq!(None, parse_duration_ms(bad), "{bad}");
        }
        assert!(
            error_of("consumer:\n  - topic: t\n    flow: f\n    ttl: '0s'\n")
                .contains("'ttl' must be a positive duration")
        );
    }

    #[test]
    fn partition_and_max_poll_records_are_bounded_integers() {
        assert_eq!(
            Some(3),
            shape_of("consumer:\n  - topic: t\n    flow: f\n    partition: 3\n").partition
        );
        // (`partition: '${POD_PARTITION:1}'` substitutes through ConfigReader
        // only when an application base config exists - proven by the e2e's
        // `${K2_HAPPY_GROUP:...}` group, not reproducible in a bare unit test)
        assert!(
            error_of("consumer:\n  - topic: t\n    flow: f\n    partition: x\n")
                .contains("'partition' must be an integer, got 'x'")
        );
        assert!(
            error_of("consumer:\n  - topic: t\n    flow: f\n    partition: -1\n")
                .contains("'partition' must be >= 0, got -1")
        );
        assert_eq!(
            Some(500),
            shape_of("consumer:\n  - topic: t\n    flow: f\n    max-poll-records: 500\n")
                .max_poll_records
        );
        assert!(
            error_of("consumer:\n  - topic: t\n    flow: f\n    max-poll-records: 0\n")
                .contains("'max-poll-records' must be > 0, got 0")
        );
        assert!(
            error_of("consumer:\n  - topic: t\n    flow: f\n    max-poll-records: many\n")
                .contains("'max-poll-records' must be an integer, got 'many'")
        );
    }

    #[test]
    fn group_and_delivery_mode_flags() {
        let defaults = shape_of("consumer:\n  - topic: orders\n    flow: f\n");
        assert_eq!("kafka-flow-adapter.orders", defaults.group_id);
        assert!(!defaults.auto_commit);
        let explicit = shape_of(
            "consumer:\n  - topic: orders\n    flow: f\n    group: 'sales-group'\n    auto-commit: true\n",
        );
        assert_eq!(
            "sales-group", explicit.group_id,
            "the group is used exactly as given"
        );
        assert!(explicit.auto_commit);
        assert!(
            !shape_of("consumer:\n  - topic: t\n    flow: f\n    auto-commit: 'yes'\n").auto_commit
        );
    }

    #[test]
    fn header_overrides_read_nested_and_flat_forms() {
        // ConfigReader normalizes dotted YAML keys into nested maps
        let nested = shape_of(
            "consumer:\n  - topic: legacy\n    flow: f\n    trace.id.header: 'X-Legacy-Trace'\n    correlation.id.header: 'X-Correlation-ID'\n    traceparent.header: 'X-Trace-Context'\n",
        );
        assert_eq!(Some("X-Legacy-Trace".to_string()), nested.trace_id_header);
        assert_eq!(
            Some("X-Correlation-ID".to_string()),
            nested.correlation_id_header
        );
        assert_eq!(
            Some("X-Trace-Context".to_string()),
            nested.traceparent_header
        );
        // a programmatically authored map keeps the flat key
        let flat = serde_json::json!({"topic": "t", "flow": "f", "correlation.id.header": "X-Cid"});
        assert_eq!(
            Some("X-Cid".to_string()),
            parse_binding_shape(0, &flat)
                .expect("shape")
                .correlation_id_header
        );
        assert!(shape_of("consumer:\n  - topic: t\n    flow: f\n")
            .trace_id_header
            .is_none());
    }

    #[test]
    fn dlq_must_not_equal_or_match_the_source() {
        assert!(
            error_of("consumer:\n  - topic: orders\n    flow: f\n    dlq-topic: orders\n")
                .contains("uses its own source topic")
        );
        assert!(
            error_of("consumer:\n  - topic-pattern: 'events\\.[a-z]{2}'\n    flow: f\n    group: g\n    dlq-topic: 'events.dl'\n")
                .contains("must not match 'topic-pattern'")
        );
        let ok = shape_of(
            "consumer:\n  - topic-pattern: 'events\\.[a-z]{2}'\n    flow: f\n    group: g\n    dlq-topic: 'events-dlq'\n",
        );
        assert_eq!(Some("events-dlq".to_string()), ok.dlq_topic);
    }

    #[test]
    fn dead_letter_without_producer_fails_deployment() {
        let mut with_dlq = KafkaConsumerBinding::direct("orders", "f");
        with_dlq.dlq_topic = Some("orders-dlq".into());
        let error = reject_dead_letter_without_producer(&[with_dlq], "kafka.producer.enabled")
            .expect_err("guard fires");
        assert!(error.message().contains("dlq-topic 'orders-dlq'"));
        assert!(error.message().contains("kafka.producer.enabled=false"));

        reject_dead_letter_without_producer(
            &[KafkaConsumerBinding::direct("orders", "f")],
            "kafka.producer.enabled",
        )
        .expect("no dlq, no guard");
    }
}
