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

//! One consumer task per binding (Java `KafkaFlowConsumer`): poll a record,
//! decode it into the flow dataset (`header` + `metadata` + `body`), select
//! the target — the binding's one flow, or per record through its
//! second-level routing rules ([`crate::routing`]) — deliver it, and only
//! after it finishes commit the offset (**at-least-once,
//! commit-after-process**; a binding with `auto-commit: true` leaves the
//! commit to the client's own timer instead). On failure the record retries
//! per the policy, then parks on the binding's `dlq-topic` with a
//! **confirmed** write before committing, so a poison message is neither lost
//! nor reprocessed forever.
//!
//! **Subscription modes.** Group-managed `subscribe` by default; manual
//! `assign` of the one topic-partition when `partition` is set — bypassing
//! group rebalancing, so the operator owns the deployment model (one consumer
//! per partition, or each pod pinning a distinct partition via
//! `partition: ${POD_PARTITION}`); offsets still commit under the configured
//! group, and the pinned consumer resumes from the group's committed offset
//! (else `auto.offset.reset`). A `topic-pattern` binding subscribes with the
//! anchored regex: the client re-matches the cluster's topic list on every
//! metadata refresh (`topic.metadata.refresh.interval.ms`), so a new matching
//! topic joins without a restart, as the Java client's `subscribe(Pattern)`
//! does on its `metadata.max.age.ms`.
//!
//! **`serializer: 'json'`** tries to decode the record value before routing —
//! a JSON object or array becomes a map or list body (so `input.body.*` rules
//! match and the target receives the decoded value); anything else, malformed
//! text included, keeps the raw bytes and simply passes to the selected
//! target (best-effort by design: a target that cannot digest the bytes fails
//! normally into the retry/DLQ path).
//!
//! **`task://` targets** (Java `toTaskRequest`): the whole payload is the body
//! (raw bytes, or the decoded value), every record header is copied onto the
//! envelope headers, the business correlation-id rides the engine-managed
//! `my_cid` tag so the worker injects `my_correlation_id` at delivery, and the
//! deadline is the binding's `ttl` (default 30 s) — a bare function has no
//! flow ttl. There is no `metadata` map on this path; a function that needs
//! the record's envelope facts should be fronted by a flow.
//!
//! **Trace continuity.** The record's standard W3C `traceparent` always wins;
//! the effective traceparent-header override (per-binding, else the global
//! `kafka.traceparent.header`) is read only when the standard one is absent.
//! The flow chains onto the upstream span and the trace path is
//! `KAFKA /<topic>`. The business correlation-id comes from the effective
//! correlation-id header (per-binding `correlation.id.header`, else the global
//! `kafka.correlation.id.header`, default `cid`), with a fresh id minted when
//! absent; the effective trace-id header is the fallback trace-id source for
//! an upstream that sends no traceparent.
//!
//! **Threading** (the Java module runs one kernel thread per binding): the
//! loop is a tokio task using the client's async stream; the one blocking step
//! — the synchronous offset commit — runs under `block_in_place`, the
//! async-correct home for a short blocking round trip.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use platform_core::post_office::BUSINESS_CID_TAG;
use platform_core::{w3c_trace, AppConfigReader, AppError, EventEnvelope, Platform, PostOffice};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Headers, Message};
use rdkafka::TopicPartitionList;
use rmpv::Value;
use tokio::sync::watch;

use crate::adapter::KafkaConsumerBinding;
use crate::client_config::DEFAULT_TASK_TTL_MS;
use crate::headers::CORRELATION_ID;
use crate::publisher::KafkaRequestPublisher;
use crate::routing::RoutingTarget;

const ADAPTER_ROUTE: &str = "kafka.flow.adapter";
const FLOW_ID_HEADER: &str = "flow_id";
const DLQ_ERROR_HEADER: &str = "dlq.error";
const DLQ_ORIGIN_TOPIC_HEADER: &str = "dlq.origin.topic";
const INITIAL_FAILURE_BACKOFF_MS: u64 = 1000;
const MAX_FAILURE_BACKOFF_MS: u64 = 30000;

/// The shared retry/dead-letter shape (Java `RetryPolicy`): extra attempts
/// after the first failure, the pause between attempts, and the publisher
/// used to park exhausted messages (`None` = dead-letter writes are skipped
/// and the message is dropped with a logged DATA-LOSS `ERROR`).
#[derive(Clone)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub backoff_ms: u64,
    pub dead_letter_publisher: Option<Arc<KafkaRequestPublisher>>,
}

/// The effective inbound header names for one binding: the per-binding
/// override, else the global `kafka.*.header` setting, else the convention.
#[derive(Clone, Debug)]
struct InboundNames {
    correlation_id: String,
    trace_id: Option<String>,
    traceparent: String,
}

impl InboundNames {
    fn for_binding(binding: &KafkaConsumerBinding) -> Self {
        let config = AppConfigReader::get_instance();
        InboundNames {
            correlation_id: binding.correlation_id_header.clone().unwrap_or_else(|| {
                config.get_property_or("kafka.correlation.id.header", CORRELATION_ID)
            }),
            trace_id: binding
                .trace_id_header
                .clone()
                .or_else(|| config.get_property("kafka.trace.id.header")),
            traceparent: binding.traceparent_header.clone().unwrap_or_else(|| {
                config.get_property_or("kafka.traceparent.header", w3c_trace::TRACEPARENT)
            }),
        }
    }
}

/// A running binding consumer; dropping the handle does not stop it — call
/// [`KafkaFlowConsumer::close`].
pub struct KafkaFlowConsumer {
    shutdown: watch::Sender<bool>,
    binding_label: String,
}

impl KafkaFlowConsumer {
    /// Start the poll loop for one binding on the given consumer (already
    /// carrying the binding's `group.id` and its delivery-mode overlay).
    pub fn start(
        platform: Platform,
        consumer: StreamConsumer,
        binding: KafkaConsumerBinding,
        retry_policy: RetryPolicy,
        dlq_timeout: Duration,
    ) -> Result<KafkaFlowConsumer, AppError> {
        subscribe_or_assign(&consumer, &binding)?;
        log_binding(&binding);
        let (shutdown, shutdown_rx) = watch::channel(false);
        let handle = KafkaFlowConsumer {
            shutdown,
            binding_label: binding.label(),
        };
        tokio::spawn(poll_loop(
            platform,
            consumer,
            binding,
            retry_policy,
            dlq_timeout,
            shutdown_rx,
        ));
        Ok(handle)
    }

    /// Stop the poll loop after its in-flight record completes (Java parity:
    /// the running flag is honored per iteration, so a message is never
    /// abandoned mid-flow).
    pub fn close(&self) {
        let _ = self.shutdown.send(true);
        log::info!("Kafka flow consumer for {} stopping", self.binding_label);
    }
}

/// Group-managed `subscribe` by default; manual `assign` of the single pinned
/// topic-partition when a `partition` was configured (the client translates
/// the unset offset to the group's stored offset); the anchored regex
/// `subscribe` for a `topic-pattern` binding.
fn subscribe_or_assign(
    consumer: &StreamConsumer,
    binding: &KafkaConsumerBinding,
) -> Result<(), AppError> {
    let result = if let Some(partition) = binding.partition {
        let mut assignment = TopicPartitionList::new();
        assignment.add_partition(&binding.topic_or_pattern, partition);
        consumer.assign(&assignment)
    } else if binding.pattern {
        consumer.subscribe(&[binding.subscription_regex().as_str()])
    } else {
        consumer.subscribe(&[binding.topic_or_pattern.as_str()])
    };
    result.map_err(|e| {
        AppError::new(
            500,
            format!("Unable to subscribe to {} - {e}", binding.label()),
        )
    })
}

/// One-line summary of a resolved binding (Java `logBinding`).
fn log_binding(binding: &KafkaConsumerBinding) {
    let destination = match &binding.routing {
        Some(rules) => format!("second-level routing ({} rules + default)", rules.size()),
        None => format!("flow '{}'", binding.flow_id.as_deref().unwrap_or_default()),
    };
    let mut extras = String::new();
    if let Some(partition) = binding.partition {
        extras.push_str(&format!(", pinned to partition {partition}"));
    }
    if binding.json_serializer {
        extras.push_str(", serializer 'json'");
    }
    if let Some(ttl) = binding.task_ttl_ms {
        extras.push_str(&format!(", task ttl {}s", ttl / 1000));
    }
    if let Some(dlq) = &binding.dlq_topic {
        extras.push_str(&format!(", dlq-topic '{dlq}'"));
    }
    if binding.auto_commit {
        extras.push_str(", auto-commit on");
    }
    if let Some(header) = &binding.trace_id_header {
        extras.push_str(&format!(", trace-id header '{header}'"));
    }
    if let Some(header) = &binding.correlation_id_header {
        extras.push_str(&format!(", correlation-id header '{header}'"));
    }
    if let Some(header) = &binding.traceparent_header {
        extras.push_str(&format!(", traceparent header '{header}'"));
    }
    log::info!(
        "Kafka flow adapter binding: {} -> {destination} (consumer group '{}'{extras})",
        binding.label(),
        binding.group_id
    );
}

async fn poll_loop(
    platform: Platform,
    consumer: StreamConsumer,
    binding: KafkaConsumerBinding,
    retry_policy: RetryPolicy,
    dlq_timeout: Duration,
    mut shutdown: watch::Receiver<bool>,
) {
    let names = InboundNames::for_binding(&binding);
    let label = binding.label();
    let mut consecutive_failures = 0u32;
    loop {
        if *shutdown.borrow() {
            break;
        }
        let received = tokio::select! {
            _ = shutdown.changed() => break,
            received = consumer.recv() => received,
        };
        match received {
            Ok(message) => {
                let delivered = deliver_record(
                    &platform,
                    &binding,
                    &names,
                    &retry_policy,
                    dlq_timeout,
                    &message,
                )
                .await;
                if delivered && !binding.auto_commit {
                    // commit only after the target finished -> at-least-once.
                    // (auto-commit mode leaves the offset to the client's own
                    // timer, regardless of processing outcome - the documented
                    // throughput-for-redelivery trade.) The synchronous commit
                    // is one short blocking round trip; block_in_place keeps it
                    // off the async reactor correctly.
                    let commit = tokio::task::block_in_place(|| {
                        consumer.commit_message(&message, CommitMode::Sync)
                    });
                    if let Err(e) = commit {
                        // routine after a group rebalance: the uncommitted
                        // record redelivers to whichever consumer now owns the
                        // partition (at-least-once holds; flows are idempotent
                        // by contract) and the next poll rejoins the group
                        log::warn!(
                            "Offset commit for {label} failed; records will redeliver - {e}"
                        );
                    }
                }
                consecutive_failures = 0;
            }
            Err(error) => {
                // stay alive (the alternative is a binding that is dead until
                // the pod restarts) but pause with escalating backoff so a
                // persistent error cannot hot-loop
                consecutive_failures += 1;
                let pause = MAX_FAILURE_BACKOFF_MS
                    .min(INITIAL_FAILURE_BACKOFF_MS << (consecutive_failures - 1).min(5));
                log::error!(
                    "Kafka flow consumer for {label} caught an error (failure #{consecutive_failures}); \
                     pausing {pause} ms before it continues - {error}"
                );
                tokio::select! {
                    _ = shutdown.changed() => break,
                    _ = tokio::time::sleep(Duration::from_millis(pause)) => {}
                }
            }
        }
    }
    log::info!("Kafka flow consumer for {label} stopped");
}

/// Deliver one record to its target — the binding's flow, or the rule-selected
/// flow or task — blocking until it finishes. Returns whether the offset may
/// be committed: the target completed, or the message was durably
/// dead-lettered (or dropped-with-ERROR to protect partition liveness).
async fn deliver_record(
    platform: &Platform,
    binding: &KafkaConsumerBinding,
    names: &InboundNames,
    retry_policy: &RetryPolicy,
    dlq_timeout: Duration,
    message: &BorrowedMessage<'_>,
) -> bool {
    let headers = record_headers(message);
    // serializer: 'json' - best-effort decode BEFORE routing, so input.body
    // rules see the decoded value and the target receives it
    let decoded = if binding.json_serializer {
        best_effort_json(message.payload())
    } else {
        None
    };
    let target = match &binding.routing {
        Some(rules) => rules.select(&headers, decoded.as_ref()).clone(),
        None => RoutingTarget::flow(binding.flow_id.as_deref().unwrap_or_default()),
    };
    // trace-id precedence: W3C traceparent > configured trace-id header
    // (legacy upstream) > fresh UUID; the target chains onto the upstream span
    let trace = parse_inbound_traceparent(&headers, names);
    let trace_id = trace.as_ref().map(|(id, _)| id.clone()).unwrap_or_else(|| {
        names
            .trace_id
            .as_ref()
            .and_then(|header| headers.get(header).cloned())
            .unwrap_or_else(new_id)
    });
    let trace_path = format!("KAFKA /{}", message.topic());
    let business_cid = resolve_business_cid(&headers, names, &trace_id);
    let body = match &decoded {
        Some(json) => json_to_value(json),
        None => raw_body(message),
    };
    let ttl_ms = if target.task {
        binding.task_ttl_ms.unwrap_or(DEFAULT_TASK_TTL_MS)
    } else {
        event_script::flows::get_flow(&target.destination)
            .map(|flow| flow.ttl)
            .unwrap_or(DEFAULT_TASK_TTL_MS)
    };
    let dataset = if target.task {
        None
    } else {
        Some(to_dataset(message, &headers, body.clone()))
    };
    let label = target.label();
    let mut attempt = 0u32;
    loop {
        // a fresh envelope per attempt (its own event id) - the same content
        let request = match &dataset {
            Some(dataset) => flow_request(&target.destination, dataset.clone(), &business_cid),
            None => task_request(&target.destination, body.clone(), &headers, &business_cid),
        };
        let request = request
            .set_from(ADAPTER_ROUTE)
            .set_trace(&trace_id, &trace_path);
        let request = match &trace {
            Some((_, span)) => request.set_span_id(span), // chain onto the upstream span
            None => request,
        };
        let po = PostOffice::new(platform);
        let outcome = po
            .request(request, Duration::from_millis(ttl_ms))
            .await
            .and_then(|response| {
                if response.status() < 400 {
                    Ok(())
                } else {
                    Err(AppError::new(
                        response.status(),
                        format!("{label} returned status {}", response.status()),
                    ))
                }
            });
        let cause = match outcome {
            Ok(()) => return true, // the target finished normally -> commit
            Err(e) => e,
        };
        if attempt >= retry_policy.max_retries {
            log::warn!(
                "{label} failed for a '{}' message after {} attempt(s); routing to {:?} - {}",
                message.topic(),
                attempt + 1,
                binding.dlq_topic,
                cause.message()
            );
            return write_to_dead_letter(binding, retry_policy, dlq_timeout, message, &cause).await;
        }
        attempt += 1;
        log::warn!(
            "{label} failed for a '{}' message (attempt {attempt}/{}); retrying - {}",
            message.topic(),
            retry_policy.max_retries,
            cause.message()
        );
        if retry_policy.backoff_ms > 0 {
            tokio::time::sleep(Duration::from_millis(retry_policy.backoff_ms)).await;
        }
    }
}

/// The flow-engine request: the per-record flow id, the business
/// correlation-id (seeded into `model.cid` by the engine) and the dataset.
fn flow_request(flow_id: &str, dataset: Value, business_cid: &str) -> EventEnvelope {
    EventEnvelope::new()
        .set_to(event_script::manager::SERVICE_NAME)
        .set_header(FLOW_ID_HEADER, flow_id)
        .set_header(event_script::manager::BUSINESS_CORRELATION_ID, business_cid)
        .set_correlation_id(business_cid)
        .set_raw_body(dataset)
}

/// A direct function invocation for a `task://` target (Java `toTaskRequest`):
/// every inbound record header copied onto the envelope headers, the whole
/// payload as the body, and the business correlation-id on the engine-managed
/// `my_cid` tag — never an envelope header — so the worker injects
/// `my_correlation_id` at delivery.
fn task_request(
    route: &str,
    body: Value,
    headers: &HashMap<String, String>,
    business_cid: &str,
) -> EventEnvelope {
    let mut request = EventEnvelope::new()
        .set_to(route)
        .set_raw_body(body)
        .add_tag(BUSINESS_CID_TAG, business_cid);
    for (key, value) in headers {
        request = request.set_header(key, value);
    }
    request
}

/// Best-effort JSON decode for a `serializer: 'json'` binding: a JSON object
/// or array decodes; a scalar or malformed text is `None` (the raw bytes stay
/// the body). The shape sniff + parse fallback is the same idiom the platform
/// uses for JSON HTTP content.
fn best_effort_json(payload: Option<&[u8]>) -> Option<serde_json::Value> {
    let text = std::str::from_utf8(payload?).ok()?.trim();
    let shaped = (text.starts_with('{') && text.ends_with('}'))
        || (text.starts_with('[') && text.ends_with(']'));
    if !shaped {
        return None;
    }
    serde_json::from_str::<serde_json::Value>(text)
        .ok()
        .filter(|value| value.is_object() || value.is_array())
}

/// A decoded JSON value as the dynamic body (maps and lists nest; numbers keep
/// their width).
fn json_to_value(json: &serde_json::Value) -> Value {
    rmpv::ext::to_value(json).unwrap_or(Value::Nil)
}

/// The raw payload bytes as the body; a tombstone is null.
fn raw_body(message: &BorrowedMessage<'_>) -> Value {
    match message.payload() {
        Some(bytes) => Value::Binary(bytes.to_vec()),
        None => Value::Nil,
    }
}

/// Park an un-processable message on the binding's `dlq-topic` with a
/// **confirmed** write, preserving its headers + body and adding the origin
/// facts. When no DLQ is configured — or the write fails — the message is
/// dropped with a loud DATA-LOSS `ERROR` and the offset still commits:
/// refusing would redeliver the same poison message forever (a
/// self-sustaining recovery storm), so partition liveness deliberately wins.
async fn write_to_dead_letter(
    binding: &KafkaConsumerBinding,
    retry_policy: &RetryPolicy,
    dlq_timeout: Duration,
    message: &BorrowedMessage<'_>,
    cause: &AppError,
) -> bool {
    let Some(dlq_topic) = &binding.dlq_topic else {
        log::error!(
            "DATA LOSS: no dlq-topic configured; dropping '{}' offset {} after flow failure \
             (set dlq-topic to retain failed messages) - {}",
            message.topic(),
            message.offset(),
            cause.message()
        );
        return true;
    };
    let Some(publisher) = &retry_policy.dead_letter_publisher else {
        log::error!(
            "DATA LOSS: no dead-letter publisher; dropping '{}' offset {} after flow failure",
            message.topic(),
            message.offset()
        );
        return true;
    };
    let mut dead_letter_headers: HashMap<String, Vec<u8>> = HashMap::new();
    if let Some(borrowed) = message.headers() {
        for i in 0..borrowed.count() {
            let header = borrowed.get(i);
            dead_letter_headers.insert(
                header.key.to_string(),
                header.value.unwrap_or_default().to_vec(),
            );
        }
    }
    dead_letter_headers.insert(
        DLQ_ORIGIN_TOPIC_HEADER.to_string(),
        message.topic().as_bytes().to_vec(),
    );
    dead_letter_headers.insert(
        DLQ_ERROR_HEADER.to_string(),
        cause.message().as_bytes().to_vec(),
    );
    let write = tokio::time::timeout(
        dlq_timeout,
        publisher.publish(
            dlq_topic,
            None,
            dead_letter_headers,
            message.payload().map(<[u8]>::to_vec),
        ),
    )
    .await;
    match write {
        Ok(Ok(())) => true,
        Ok(Err(e)) => {
            data_loss(dlq_topic, message, e.message());
            true
        }
        Err(_) => {
            data_loss(dlq_topic, message, "confirm-write timed out");
            true
        }
    }
}

fn data_loss(dlq_topic: &str, message: &BorrowedMessage<'_>, reason: &str) {
    // the last line of defense failed - drop loudly rather than block the
    // partition retrying forever (ensure the DLQ topic exists)
    log::error!(
        "DATA LOSS: dead-letter write to {dlq_topic} failed; dropping '{}' offset {} to avoid a \
         redelivery storm - {reason}",
        message.topic(),
        message.offset()
    );
}

/// The record's Kafka headers as UTF-8 text.
fn record_headers(message: &BorrowedMessage<'_>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    if let Some(borrowed) = message.headers() {
        for i in 0..borrowed.count() {
            let header = borrowed.get(i);
            headers.insert(
                header.key.to_string(),
                String::from_utf8_lossy(header.value.unwrap_or_default()).to_string(),
            );
        }
    }
    headers
}

/// The inbound W3C trace context. The standard `traceparent` header always
/// wins; the effective custom name is read only when the standard header is
/// absent or malformed — a well-formed standard traceparent means the
/// upstream already speaks the W3C standard, so a proprietary header alongside
/// it is residual and safely ignored.
fn parse_inbound_traceparent(
    headers: &HashMap<String, String>,
    names: &InboundNames,
) -> Option<(String, String)> {
    headers
        .get(w3c_trace::TRACEPARENT)
        .and_then(|value| w3c_trace::parse(value))
        .or_else(|| {
            if names
                .traceparent
                .eq_ignore_ascii_case(w3c_trace::TRACEPARENT)
            {
                None
            } else {
                headers
                    .get(&names.traceparent)
                    .and_then(|value| w3c_trace::parse(value))
            }
        })
}

/// The upstream business correlation-id from the effective header; a fresh
/// one when absent — unless the trace-id and correlation-id share ONE header
/// name (legacy conflation), where the resolved trace id is authoritative so
/// the hop stays self-consistent.
fn resolve_business_cid(
    headers: &HashMap<String, String>,
    names: &InboundNames,
    trace_id: &str,
) -> String {
    match headers.get(&names.correlation_id) {
        Some(cid) => cid.clone(),
        None => {
            if names
                .trace_id
                .as_ref()
                .is_some_and(|t| t.eq_ignore_ascii_case(&names.correlation_id))
            {
                trace_id.to_string()
            } else {
                new_id()
            }
        }
    }
}

/// Decode a record into the flow dataset — `header` (the record's Kafka
/// headers), `metadata` (the record's OWN envelope facts: actual topic,
/// partition, offset, timestamp epoch-ms, and key when present), and `body`
/// (the raw payload bytes, or the decoded value under `serializer: 'json'`; a
/// tombstone is null).
fn to_dataset(
    message: &BorrowedMessage<'_>,
    headers: &HashMap<String, String>,
    body: Value,
) -> Value {
    let header_entries: Vec<(Value, Value)> = headers
        .iter()
        .map(|(k, v)| (Value::from(k.as_str()), Value::from(v.as_str())))
        .collect();
    let mut metadata: Vec<(Value, Value)> = vec![
        (Value::from("topic"), Value::from(message.topic())),
        (Value::from("partition"), Value::from(message.partition())),
        (Value::from("offset"), Value::from(message.offset())),
        (
            Value::from("timestamp"),
            Value::from(message.timestamp().to_millis().unwrap_or(0)),
        ),
    ];
    if let Some(key) = message.key() {
        metadata.push((
            Value::from("key"),
            Value::from(String::from_utf8_lossy(key).to_string()),
        ));
    }
    Value::Map(vec![
        (Value::from("header"), Value::Map(header_entries)),
        (Value::from("metadata"), Value::Map(metadata)),
        (Value::from("body"), body),
    ])
}

fn new_id() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A JSON object or array decodes; anything else keeps the raw bytes.
    #[test]
    fn best_effort_json_parses_objects_and_arrays_and_keeps_bytes_otherwise() {
        assert!(best_effort_json(Some(br#" {"a":1} "#)).is_some_and(|v| v.is_object()));
        assert!(best_effort_json(Some(br#"[{"type":"x"}]"#)).is_some_and(|v| v.is_array()));
        assert!(best_effort_json(Some(b"not-json")).is_none());
        assert!(
            best_effort_json(Some(b"{broken")).is_none(),
            "malformed keeps the bytes"
        );
        assert!(best_effort_json(Some(b"\"scalar\"")).is_none());
        assert!(best_effort_json(Some(&[0xff, 0xfe])).is_none(), "not UTF-8");
        assert!(best_effort_json(None).is_none(), "a tombstone has no body");
    }

    #[test]
    fn decoded_json_becomes_a_nested_dynamic_body() {
        let json = serde_json::json!({"event": {"kind": "refund"}, "amount": 10, "tags": ["a"]});
        let value = json_to_value(&json);
        let Value::Map(entries) = value else {
            panic!("a JSON object is a map body");
        };
        let event = entries
            .iter()
            .find(|(k, _)| k.as_str() == Some("event"))
            .map(|(_, v)| v.clone())
            .expect("event");
        assert!(matches!(event, Value::Map(_)));
        let amount = entries
            .iter()
            .find(|(k, _)| k.as_str() == Some("amount"))
            .map(|(_, v)| v.clone())
            .expect("amount");
        assert_eq!(Some(10), amount.as_i64());
    }
}
