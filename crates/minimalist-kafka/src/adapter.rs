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
//! `KafkaFlowAdapter` + `KafkaConsumerBinding`, the K2 surface): literal
//! `topic -> flow` bindings with an optional consumer `group` (default
//! `kafka-flow-adapter.<topic>`) and an optional per-binding `dlq-topic`.
//! Every value supports `${ENV_VAR:default}` substitution (the file is read
//! by `ConfigReader`). **A malformed entry fails startup fast and loud**
//! rather than being silently skipped — including a field of the port's later
//! increments (`topic-pattern`, `flows`, `partition`, `serializer`,
//! `schema.enabled`, `ttl`, `auto-commit`, `max-poll-records`, the
//! per-binding header overrides), which is rejected by name rather than
//! ignored.

use platform_core::{AppError, ConfigReader};

const DEFAULT_GROUP_PREFIX: &str = "kafka-flow-adapter";

/// Adapter fields of the port's later increments — present in the Java
/// module, deliberately not served yet. Naming them beats ignoring them.
const DEFERRED_FIELDS: [&str; 10] = [
    "topic-pattern",
    "flows",
    "partition",
    "serializer",
    "schema.enabled",
    "ttl",
    "auto-commit",
    "max-poll-records",
    "trace.id.header",
    "correlation.id.header",
];

/// One validated `consumer[]` binding (the K2 surface).
#[derive(Clone, Debug)]
pub struct KafkaConsumerBinding {
    pub topic: String,
    pub flow_id: String,
    pub group_id: String,
    pub dlq_topic: Option<String>,
}

/// Parse and validate the adapter configuration. Cross-reference checks (the
/// flow must be compiled) run against the live flow registry, so this is
/// called from the main-application hook — after `CompileFlows`.
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
        bindings.push(parse_binding(i, entry)?);
    }
    Ok(bindings)
}

fn parse_binding(i: usize, entry: &serde_json::Value) -> Result<KafkaConsumerBinding, AppError> {
    let serde_json::Value::Object(_) = entry else {
        return Err(bad_config(&format!(
            "consumer[{i}] must be a map with 'topic' and 'flow'"
        )));
    };
    for field in DEFERRED_FIELDS {
        if field_present(entry, field) {
            return Err(bad_config(&format!(
                "consumer[{i}] sets '{field}', which arrives in a later increment of this port \
                 (see draft-design-specs/minimalist-kafka-port.md §8)"
            )));
        }
    }
    let topic = text(entry, "topic")
        .ok_or_else(|| bad_config(&format!("consumer[{i}] is missing a 'topic'")))?;
    let flow_id = text(entry, "flow")
        .ok_or_else(|| bad_config(&format!("consumer[{i}] ({topic}) is missing a 'flow'")))?;
    let group_id =
        text(entry, "group").unwrap_or_else(|| format!("{DEFAULT_GROUP_PREFIX}.{topic}"));
    let dlq_topic = text(entry, "dlq-topic");
    if dlq_topic.as_deref() == Some(topic.as_str()) {
        return Err(bad_config(&format!(
            "consumer[{i}] ({topic}) uses its own source topic as 'dlq-topic' - an exhausted \
             message would loop forever"
        )));
    }
    // cross-reference check LAST (Java parity): everything above validates
    // this one entry's own shape; this depends on the compiled flow registry
    if event_script::flows::get_flow(&flow_id).is_none() {
        return Err(bad_config(&format!(
            "consumer[{i}] ({topic}) references unknown flow '{flow_id}'"
        )));
    }
    Ok(KafkaConsumerBinding {
        topic,
        flow_id,
        group_id,
        dlq_topic,
    })
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
                "binding '{}' declares dlq-topic '{dlq}' but {producer_enabled_key}=false - \
                 dead letters are published through this cluster's own producer; enable the \
                 producer or drop the dlq-topic",
                binding.topic
            )));
        }
    }
    Ok(())
}

/// A dotted field may arrive nested (ConfigReader normalizes dotted YAML keys
/// into nested maps) or flat (an authored map) — accept both, like the Java
/// module's `nestedText`.
fn field_present(entry: &serde_json::Value, flat_key: &str) -> bool {
    if !entry[flat_key].is_null() {
        return true;
    }
    let mut node = entry;
    for segment in flat_key.split('.') {
        node = &node[segment];
        if node.is_null() {
            return false;
        }
    }
    true
}

fn text(entry: &serde_json::Value, key: &str) -> Option<String> {
    match &entry[key] {
        serde_json::Value::Null => None,
        serde_json::Value::String(s) if s.trim().is_empty() => None,
        serde_json::Value::String(s) => Some(s.clone()),
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

    /// The startup-validation table: every malformed shape fails loudly with
    /// the entry named (no flow registry needed for these — shape errors are
    /// checked before the cross-reference).
    #[test]
    fn malformed_entries_fail_startup_by_name() {
        let missing_topic = parse_bindings(&reader("consumer:\n  - flow: f1\n"));
        assert!(missing_topic
            .expect_err("no topic")
            .message()
            .contains("consumer[0] is missing a 'topic'"));

        let missing_flow = parse_bindings(&reader("consumer:\n  - topic: orders\n"));
        assert!(missing_flow
            .expect_err("no flow")
            .message()
            .contains("consumer[0] (orders) is missing a 'flow'"));

        let unknown_flow = parse_bindings(&reader(
            "consumer:\n  - topic: orders\n    flow: no-such-flow\n",
        ));
        assert!(unknown_flow
            .expect_err("unknown flow")
            .message()
            .contains("references unknown flow 'no-such-flow'"));

        let empty = parse_bindings(&reader("consumer: []\n"));
        assert!(empty.expect_err("empty").message().contains("no bindings"));

        let missing_list = parse_bindings(&reader("producer: {}\n"));
        assert!(missing_list
            .expect_err("no list")
            .message()
            .contains("missing the 'consumer' binding list"));
    }

    /// Fields of later increments are rejected by name — never silently
    /// ignored (both the flat and the ConfigReader-nested dotted form).
    #[test]
    fn deferred_fields_are_rejected_by_name() {
        for (yaml, field) in [
            (
                "consumer:\n  - topic: t\n    flow: f\n    topic-pattern: 'x.*'\n",
                "topic-pattern",
            ),
            (
                "consumer:\n  - topic: t\n    flow: f\n    auto-commit: true\n",
                "auto-commit",
            ),
            (
                "consumer:\n  - topic: t\n    flow: f\n    schema.enabled: true\n",
                "schema.enabled",
            ),
            (
                "consumer:\n  - topic: t\n    flow: f\n    serializer: json\n",
                "serializer",
            ),
            (
                "consumer:\n  - topic: t\n    flow: f\n    partition: 0\n",
                "partition",
            ),
        ] {
            let error = parse_bindings(&reader(yaml)).expect_err(field);
            assert!(
                error.message().contains(field) && error.message().contains("later increment"),
                "{field}: {}",
                error.message()
            );
        }
    }

    #[test]
    fn dlq_must_not_equal_the_source_topic() {
        let same = parse_bindings(&reader(
            "consumer:\n  - topic: orders\n    flow: f\n    dlq-topic: orders\n",
        ));
        assert!(same
            .expect_err("dlq == source")
            .message()
            .contains("uses its own source topic"));
    }

    #[test]
    fn dead_letter_without_producer_fails_deployment() {
        let bindings = vec![KafkaConsumerBinding {
            topic: "orders".into(),
            flow_id: "f".into(),
            group_id: "g".into(),
            dlq_topic: Some("orders-dlq".into()),
        }];
        let error = reject_dead_letter_without_producer(&bindings, "kafka.producer.enabled")
            .expect_err("guard fires");
        assert!(error.message().contains("dlq-topic 'orders-dlq'"));
        assert!(error.message().contains("kafka.producer.enabled=false"));

        let no_dlq = vec![KafkaConsumerBinding {
            topic: "orders".into(),
            flow_id: "f".into(),
            group_id: "g".into(),
            dlq_topic: None,
        }];
        reject_dead_letter_without_producer(&no_dlq, "kafka.producer.enabled")
            .expect("no dlq, no guard");
    }
}
