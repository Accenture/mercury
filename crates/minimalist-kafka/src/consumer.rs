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
//! decode it into the flow dataset (`header` + `metadata` + `body`), launch
//! the bound Event Script flow, and — only after the flow finishes — commit
//! the offset (**at-least-once, commit-after-process**). On flow failure the
//! record retries per the policy, then parks on the binding's `dlq-topic`
//! with a **confirmed** write before committing, so a poison message is
//! neither lost nor reprocessed forever.
//!
//! Trace continuity: the record's W3C `traceparent` always wins (the
//! configured legacy trace-id header is a fallback), the flow chains onto the
//! upstream span, and the trace path is `KAFKA /<topic>`. The business
//! correlation-id comes from the configured header (`cid` by default), with a
//! fresh id minted when absent.
//!
//! Threading (the Java module runs one kernel thread per binding): the loop
//! is a tokio task using the client's async stream; the one blocking step —
//! the synchronous offset commit — runs under `block_in_place`, the
//! async-correct home for a short blocking round trip.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use platform_core::{w3c_trace, AppConfigReader, AppError, EventEnvelope, Platform, PostOffice};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Headers, Message};
use rmpv::Value;
use tokio::sync::watch;

use crate::adapter::KafkaConsumerBinding;
use crate::headers::CORRELATION_ID;
use crate::publisher::KafkaRequestPublisher;

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

/// Inbound header names (globals; per-binding overrides arrive with K3).
struct InboundNames {
    correlation_id: String,
    trace_id: Option<String>,
    traceparent: String,
}

fn inbound_names() -> &'static InboundNames {
    static NAMES: OnceLock<InboundNames> = OnceLock::new();
    NAMES.get_or_init(|| {
        let config = AppConfigReader::get_instance();
        InboundNames {
            correlation_id: config.get_property_or("kafka.correlation.id.header", CORRELATION_ID),
            trace_id: config.get_property("kafka.trace.id.header"),
            traceparent: config.get_property_or("kafka.traceparent.header", w3c_trace::TRACEPARENT),
        }
    })
}

/// A running binding consumer; dropping the handle does not stop it — call
/// [`KafkaFlowConsumer::close`].
pub struct KafkaFlowConsumer {
    shutdown: watch::Sender<bool>,
    binding_topic: String,
}

impl KafkaFlowConsumer {
    /// Start the poll loop for one binding on the given consumer (already
    /// carrying the binding's `group.id` and the pinned delivery-mode
    /// overlay).
    pub fn start(
        platform: Platform,
        consumer: StreamConsumer,
        binding: KafkaConsumerBinding,
        retry_policy: RetryPolicy,
        dlq_timeout: Duration,
    ) -> Result<KafkaFlowConsumer, AppError> {
        consumer.subscribe(&[&binding.topic]).map_err(|e| {
            AppError::new(
                500,
                format!("Unable to subscribe to '{}' - {e}", binding.topic),
            )
        })?;
        let (shutdown, shutdown_rx) = watch::channel(false);
        let handle = KafkaFlowConsumer {
            shutdown,
            binding_topic: binding.topic.clone(),
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
        log::info!("Kafka flow consumer for {} stopping", self.binding_topic);
    }
}

async fn poll_loop(
    platform: Platform,
    consumer: StreamConsumer,
    binding: KafkaConsumerBinding,
    retry_policy: RetryPolicy,
    dlq_timeout: Duration,
    mut shutdown: watch::Receiver<bool>,
) {
    log::info!(
        "Kafka flow consumer started - topic '{}' -> flow '{}' (group {})",
        binding.topic,
        binding.flow_id,
        binding.group_id
    );
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
                if route_to_flow(&platform, &binding, &retry_policy, dlq_timeout, &message).await {
                    // commit only after the flow finished -> at-least-once.
                    // The synchronous commit is one short blocking round trip;
                    // block_in_place keeps it off the async reactor correctly.
                    let commit = tokio::task::block_in_place(|| {
                        consumer.commit_message(&message, CommitMode::Sync)
                    });
                    if let Err(e) = commit {
                        // routine after a group rebalance: the uncommitted
                        // record redelivers to whichever consumer now owns the
                        // partition (at-least-once holds; flows are idempotent
                        // by contract) and the next poll rejoins the group
                        log::warn!(
                            "Offset commit for {} failed; records will redeliver - {e}",
                            binding.topic
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
                    "Kafka flow consumer for {} caught an error (failure #{consecutive_failures}); \
                     pausing {pause} ms before it continues - {error}",
                    binding.topic
                );
                tokio::select! {
                    _ = shutdown.changed() => break,
                    _ = tokio::time::sleep(Duration::from_millis(pause)) => {}
                }
            }
        }
    }
    log::info!("Kafka flow consumer for {} stopped", binding.topic);
}

/// Route one record into the bound flow, blocking until it finishes. Returns
/// whether the offset may be committed: the flow completed, or the message
/// was durably dead-lettered (or dropped-with-ERROR to protect partition
/// liveness).
async fn route_to_flow(
    platform: &Platform,
    binding: &KafkaConsumerBinding,
    retry_policy: &RetryPolicy,
    dlq_timeout: Duration,
    message: &BorrowedMessage<'_>,
) -> bool {
    let names = inbound_names();
    let headers = record_headers(message);
    // trace-id precedence: W3C traceparent > configured trace-id header
    // (legacy upstream) > fresh UUID; the flow chains onto the upstream span
    let trace = headers
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
        });
    let trace_id = trace.as_ref().map(|(id, _)| id.clone()).unwrap_or_else(|| {
        names
            .trace_id
            .as_ref()
            .and_then(|header| headers.get(header).cloned())
            .unwrap_or_else(new_id)
    });
    let trace_path = format!("KAFKA /{}", message.topic());
    let business_cid = resolve_business_cid(&headers, names, &trace_id);
    let dataset = to_dataset(message, &headers);
    let ttl_ms = event_script::flows::get_flow(&binding.flow_id)
        .map(|flow| flow.ttl)
        .unwrap_or(30_000);
    let mut attempt = 0u32;
    loop {
        let forward = EventEnvelope::new()
            .set_to(event_script::manager::SERVICE_NAME)
            .set_header(FLOW_ID_HEADER, &binding.flow_id)
            .set_header(
                event_script::manager::BUSINESS_CORRELATION_ID,
                &business_cid,
            )
            .set_correlation_id(&business_cid)
            .set_from(ADAPTER_ROUTE)
            .set_trace(&trace_id, &trace_path)
            .set_raw_body(dataset.clone());
        let forward = match &trace {
            Some((_, span)) => forward.set_span_id(span), // chain onto the upstream span
            None => forward,
        };
        let po = PostOffice::new(platform);
        let outcome = po
            .request(forward, Duration::from_millis(ttl_ms))
            .await
            .and_then(|response| {
                if response.status() < 400 {
                    Ok(())
                } else {
                    Err(AppError::new(
                        response.status(),
                        format!(
                            "flow '{}' returned status {}",
                            binding.flow_id,
                            response.status()
                        ),
                    ))
                }
            });
        let cause = match outcome {
            Ok(()) => return true, // the flow finished normally -> commit
            Err(e) => e,
        };
        if attempt >= retry_policy.max_retries {
            log::warn!(
                "flow '{}' failed for a '{}' message after {} attempt(s); routing to {:?} - {}",
                binding.flow_id,
                message.topic(),
                attempt + 1,
                binding.dlq_topic,
                cause.message()
            );
            return write_to_dead_letter(binding, retry_policy, dlq_timeout, message, &cause).await;
        }
        attempt += 1;
        log::warn!(
            "flow '{}' failed for a '{}' message (attempt {attempt}/{}); retrying - {}",
            binding.flow_id,
            message.topic(),
            retry_policy.max_retries,
            cause.message()
        );
        if retry_policy.backoff_ms > 0 {
            tokio::time::sleep(Duration::from_millis(retry_policy.backoff_ms)).await;
        }
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
/// (the raw payload bytes; a tombstone is null).
fn to_dataset(message: &BorrowedMessage<'_>, headers: &HashMap<String, String>) -> Value {
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
    let body = match message.payload() {
        Some(bytes) => Value::Binary(bytes.to_vec()),
        None => Value::Nil,
    };
    Value::Map(vec![
        (Value::from("header"), Value::Map(header_entries)),
        (Value::from("metadata"), Value::Map(metadata)),
        (Value::from("body"), body),
    ])
}

fn new_id() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}
