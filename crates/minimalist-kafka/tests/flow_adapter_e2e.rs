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

//! The inbound flow adapter end to end (Java `KafkaFlowAdapterTest` core
//! scenarios): real records through the mock cluster, the REAL Event Script
//! engine, and the library's own auto-start — configuration only, exactly as
//! an application would ship it. One test function drives every scenario in
//! sequence: the adapter's consumer tasks live on this runtime, and the
//! lifecycle boots once (the Java `@BeforeAll` shape).

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use async_trait::async_trait;
// link the library crate: its inventory entries (the auto-start hook and the
// preloaded functions) register at link time - the one line a Rust
// application needs where the Java jar needs only the dependency
use minimalist_kafka as _;
use platform_core::{preload, AppError, ComposableFunction, EventEnvelope, Platform, PostOffice};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::{Header, Headers, Message, OwnedHeaders};
use rdkafka::mocking::MockCluster;
use rdkafka::producer::{FutureProducer, FutureRecord};

// ---------------------------------------------------------------------------
// sink fixtures (collected from this test binary by the annotation inventory)
// ---------------------------------------------------------------------------

fn captured() -> &'static Mutex<Vec<serde_json::Value>> {
    static CAPTURED: Mutex<Vec<serde_json::Value>> = Mutex::new(Vec::new());
    &CAPTURED
}

fn flaky_attempts() -> &'static AtomicU32 {
    static ATTEMPTS: AtomicU32 = AtomicU32::new(0);
    &ATTEMPTS
}

fn poison_attempts() -> &'static AtomicU32 {
    static ATTEMPTS: AtomicU32 = AtomicU32::new(0);
    &ATTEMPTS
}

/// Records every delivered dataset plus the worker-visible trace facts.
#[preload(route = "kafka.sink.task", instances = 5)]
#[derive(Default)]
struct KafkaSinkTask;

#[async_trait]
impl ComposableFunction for KafkaSinkTask {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        // the raw payload rides as BYTES (rmpv Binary) - a bytes-tolerant view
        // is needed because plain JSON has no byte type
        let mut dataset = bytes_tolerant_json(input.body());
        let po = PostOffice::new(&Platform::get_instance());
        dataset["observed_trace_id"] = po.my_trace_id().unwrap_or_default().into();
        dataset["observed_trace_path"] = po.my_trace_path().unwrap_or_default().into();
        dataset["observed_cid"] = po.my_correlation_id().unwrap_or_default().into();
        captured().lock().expect("captured").push(dataset);
        Ok(EventEnvelope::new())
    }
}

/// rmpv -> JSON with Binary rendered as a byte array (test-side view only).
fn bytes_tolerant_json(value: &rmpv::Value) -> serde_json::Value {
    match value {
        rmpv::Value::Nil => serde_json::Value::Null,
        rmpv::Value::Boolean(b) => (*b).into(),
        rmpv::Value::Integer(n) => n
            .as_i64()
            .map(serde_json::Value::from)
            .unwrap_or(serde_json::Value::Null),
        rmpv::Value::F32(f) => (*f as f64).into(),
        rmpv::Value::F64(f) => (*f).into(),
        rmpv::Value::String(s) => s.as_str().unwrap_or_default().into(),
        rmpv::Value::Binary(bytes) => {
            serde_json::Value::Array(bytes.iter().map(|b| (*b).into()).collect())
        }
        rmpv::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(bytes_tolerant_json).collect())
        }
        rmpv::Value::Map(entries) => serde_json::Value::Object(
            entries
                .iter()
                .map(|(k, v)| {
                    (
                        k.as_str().unwrap_or_default().to_string(),
                        bytes_tolerant_json(v),
                    )
                })
                .collect(),
        ),
        _ => serde_json::Value::Null,
    }
}

/// Fails the FIRST delivery of each message, succeeds on the retry.
#[preload(route = "kafka.flaky.task", instances = 5)]
#[derive(Default)]
struct KafkaFlakyTask;

#[async_trait]
impl ComposableFunction for KafkaFlakyTask {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let attempt = flaky_attempts().fetch_add(1, Ordering::AcqRel) + 1;
        if attempt == 1 {
            return Err(AppError::new(500, "flaky by design - first delivery fails"));
        }
        let mut dataset = bytes_tolerant_json(input.body());
        dataset["attempt"] = attempt.into();
        captured().lock().expect("captured").push(dataset);
        Ok(EventEnvelope::new())
    }
}

/// Always fails - the retry-exhaustion / dead-letter path.
#[preload(route = "kafka.poison.task", instances = 5)]
#[derive(Default)]
struct KafkaPoisonTask;

#[async_trait]
impl ComposableFunction for KafkaPoisonTask {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        poison_attempts().fetch_add(1, Ordering::AcqRel);
        Err(AppError::new(500, "poison by design"))
    }
}

// ---------------------------------------------------------------------------
// harness
// ---------------------------------------------------------------------------

/// Mock cluster + env + config, pinned BEFORE the configuration snapshot.
fn bootstrap_servers() -> &'static str {
    static BOOTSTRAP: OnceLock<String> = OnceLock::new();
    BOOTSTRAP.get_or_init(|| {
        let cluster = MockCluster::new(1).expect("mock cluster");
        for topic in [
            "k2-happy",
            "k2-retry",
            "k2-poison",
            "k2-poison-dlq",
            "k2-drop",
        ] {
            cluster.create_topic(topic, 1, 1).expect("topic");
        }
        let servers = cluster.bootstrap_servers();
        // the bundled templates read ${KAFKA_BOOTSTRAP_SERVERS:...} - config,
        // not code, reaches the mock
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", &servers);
        platform_core::resources::prepend_resource_root("tests/resources");
        let _ = platform_core::AppConfigReader::get_instance();
        std::mem::forget(cluster);
        servers
    })
}

async fn produce(topic: &str, payload: &str, headers: &[(&str, &str)]) {
    static PRODUCER: OnceLock<FutureProducer> = OnceLock::new();
    let producer = PRODUCER.get_or_init(|| {
        ClientConfig::new()
            .set("bootstrap.servers", bootstrap_servers())
            .set("message.timeout.ms", "5000")
            .create()
            .expect("test producer")
    });
    let mut kafka_headers = OwnedHeaders::new();
    for (key, value) in headers {
        kafka_headers = kafka_headers.insert(Header {
            key,
            value: Some(*value),
        });
    }
    producer
        .send(
            FutureRecord::to(topic)
                .key("k2")
                .payload(payload)
                .headers(kafka_headers),
            Duration::from_secs(5),
        )
        .await
        .expect("test record delivered");
}

async fn await_captured(count: usize) -> Vec<serde_json::Value> {
    for _ in 0..600 {
        {
            let captured = captured().lock().expect("captured");
            if captured.len() >= count {
                return captured.clone();
            }
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    panic!(
        "expected {count} captured dataset(s), got {}",
        captured().lock().expect("captured").len()
    );
}

// ---------------------------------------------------------------------------
// the end-to-end scenarios (one runtime, one lifecycle - sequential)
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn inbound_adapter_end_to_end() {
    bootstrap_servers();
    platform_core::AutoStart::main(vec![])
        .await
        .expect("lifecycle");

    // --- scenario 1: the happy path pins the dataset, trace and cid contracts
    produce(
        "k2-happy",
        "{\"order\":42}",
        &[
            ("cid", "biz-cid-9"),
            (
                "traceparent",
                "00-11111111111111111111111111111111-2222222222222222-01",
            ),
            ("event", "order-created"),
        ],
    )
    .await;
    let captured_now = await_captured(1).await;
    let dataset = &captured_now[0];
    assert_eq!(
        "order-created", dataset["record_header"]["event"],
        "record headers ride into input.header (mapped to a plain body key - \
         a bare 'header' target addresses the event-header namespace)"
    );
    assert_eq!(
        "k2-happy", dataset["metadata"]["topic"],
        "the record's ACTUAL topic"
    );
    assert_eq!(0, dataset["metadata"]["partition"]);
    assert!(dataset["metadata"]["offset"].is_number());
    assert!(dataset["metadata"]["timestamp"].is_number());
    assert_eq!(
        "k2", dataset["metadata"]["key"],
        "the record key is metadata"
    );
    // the raw byte[] body rides as bytes; through the JSON view it is an array
    // of numbers - decode and compare
    let body_bytes: Vec<u8> = dataset["body"]
        .as_array()
        .expect("byte body")
        .iter()
        .map(|n| n.as_u64().unwrap_or(0) as u8)
        .collect();
    assert_eq!("{\"order\":42}", String::from_utf8_lossy(&body_bytes));
    assert_eq!(
        "11111111111111111111111111111111", dataset["observed_trace_id"],
        "the W3C traceparent's trace-id carries into the flow"
    );
    assert_eq!("KAFKA /k2-happy", dataset["observed_trace_path"]);
    assert_eq!(
        "biz-cid-9", dataset["observed_cid"],
        "the cid header is the business cid"
    );

    // --- scenario 2: at-least-once retry - first delivery fails, the retry lands
    produce("k2-retry", "retry-me", &[]).await;
    let captured_now = await_captured(2).await;
    assert_eq!(
        2,
        captured_now[1]["attempt"].as_u64().unwrap_or(0),
        "the flow succeeded on the second attempt"
    );
    assert_eq!(
        2,
        flaky_attempts().load(Ordering::Acquire),
        "exactly one retry"
    );

    // --- scenario 3: retry exhaustion parks the record on the dlq-topic with
    // the origin facts, original headers and body preserved
    produce("k2-poison", "poison-payload", &[("cid", "poison-cid")]).await;
    let dlq_consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers())
        .set("group.id", "k2-dlq-observer")
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .create()
        .expect("dlq consumer");
    dlq_consumer
        .subscribe(&["k2-poison-dlq"])
        .expect("subscribe");
    let parked = tokio::time::timeout(Duration::from_secs(15), dlq_consumer.recv())
        .await
        .expect("dead letter within deadline")
        .expect("dead letter");
    assert_eq!(
        "poison-payload",
        String::from_utf8_lossy(parked.payload().unwrap_or_default())
    );
    let mut parked_headers = HashMap::new();
    if let Some(borrowed) = parked.headers() {
        for i in 0..borrowed.count() {
            let header = borrowed.get(i);
            parked_headers.insert(
                header.key.to_string(),
                String::from_utf8_lossy(header.value.unwrap_or_default()).to_string(),
            );
        }
    }
    assert_eq!("k2-poison", parked_headers["dlq.origin.topic"]);
    assert!(
        parked_headers["dlq.error"].contains("status 500"),
        "the dead letter names the failure: {}",
        parked_headers["dlq.error"]
    );
    assert_eq!(
        "poison-cid", parked_headers["cid"],
        "original headers preserved"
    );
    // max.retries=2 -> 3 attempts total before parking
    assert_eq!(3, poison_attempts().load(Ordering::Acquire));

    // --- scenario 4: no dlq-topic - the poison record is dropped loudly and
    // the partition stays LIVE (the next message still processes)
    produce("k2-drop", "dropped-poison", &[]).await;
    // the drop path consumed 3 more attempts; then prove liveness with a
    // fresh happy record
    for _ in 0..600 {
        if poison_attempts().load(Ordering::Acquire) >= 6 {
            break;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    assert_eq!(
        6,
        poison_attempts().load(Ordering::Acquire),
        "drop path ran its retries"
    );
    produce("k2-happy", "{\"order\":43}", &[("cid", "after-drop")]).await;
    let captured_now = await_captured(3).await;
    assert_eq!(
        "after-drop", captured_now[2]["observed_cid"],
        "the binding without a dlq stays live after dropping a poison message"
    );
}
