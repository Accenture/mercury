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

#[path = "support/embedded_registry.rs"]
mod embedded_registry;

use async_trait::async_trait;
use embedded_registry::EmbeddedRegistry;
// link the library crate: its inventory entries (the auto-start hook and the
// preloaded functions) register at link time - the one line a Rust
// application needs where the Java jar needs only the dependency
use minimalist_kafka as _;
use minimalist_kafka::SchemaCodec;
use platform_core::{
    preload, AppError, ComposableFunction, ConfigReader, EventEnvelope, Platform, PostOffice,
};
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

/// A `task://` routing target: records the whole payload it received as its
/// body, its input headers (the copied record headers plus the worker's
/// injected my_* keys) and the worker-visible trace and cid facts.
#[preload(route = "kafka.task.sink", instances = 5)]
#[derive(Default)]
struct KafkaTaskSink;

#[async_trait]
impl ComposableFunction for KafkaTaskSink {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let po = PostOffice::new(&Platform::get_instance());
        let mut record = serde_json::json!({
            "kind": "task",
            "body": bytes_tolerant_json(input.body()),
            "headers": headers,
        });
        record["observed_trace_id"] = po.my_trace_id().unwrap_or_default().into();
        record["observed_trace_path"] = po.my_trace_path().unwrap_or_default().into();
        record["observed_cid"] = po.my_correlation_id().unwrap_or_default().into();
        captured().lock().expect("captured").push(record);
        Ok(EventEnvelope::new())
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
            "k3-events.de",
            "k3-events.fr",
            "k3-routed",
            "k3-legacy",
            "k3-autocommit",
            "k5-schema-json",
            "k5-schema-avro",
            "k5-schema-routed",
            "k5-schema-dlq",
        ] {
            cluster.create_topic(topic, 1, 1).expect("topic");
        }
        // two partitions: the pinned binding reads exactly one of them
        cluster.create_topic("k3-pinned", 2, 1).expect("topic");
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
    produce_to(topic, None, payload.as_bytes(), headers).await;
}

/// A binary payload (a Confluent-framed record).
async fn produce_bytes(topic: &str, payload: &[u8], headers: &[(&str, &str)]) {
    produce_to(topic, None, payload, headers).await;
}

async fn produce_to(topic: &str, partition: Option<i32>, payload: &[u8], headers: &[(&str, &str)]) {
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
    let mut record = FutureRecord::to(topic)
        .key("k2")
        .payload(payload)
        .headers(kafka_headers);
    if let Some(partition) = partition {
        record = record.partition(partition);
    }
    producer
        .send(record, Duration::from_secs(5))
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

/// The first captured record whose `observed_cid` is the given one (the K3
/// scenarios tag every record with a distinct business cid).
async fn await_captured_cid(cid: &str) -> serde_json::Value {
    for _ in 0..600 {
        {
            let captured = captured().lock().expect("captured");
            if let Some(found) = captured.iter().find(|d| d["observed_cid"] == cid) {
                return found.clone();
            }
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    panic!("no captured record with observed_cid {cid}");
}

fn captured_has_cid(cid: &str) -> bool {
    captured()
        .lock()
        .expect("captured")
        .iter()
        .any(|d| d["observed_cid"] == cid)
}

fn payload_text(dataset: &serde_json::Value) -> String {
    let bytes: Vec<u8> = dataset["body"]
        .as_array()
        .expect("byte body")
        .iter()
        .map(|n| n.as_u64().unwrap_or(0) as u8)
        .collect();
    String::from_utf8_lossy(&bytes).to_string()
}

// ---------------------------------------------------------------------------
// the end-to-end scenarios (one runtime, one lifecycle - sequential)
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn inbound_adapter_end_to_end() {
    // the in-process Schema Registry double, reached through the same
    // ${SCHEMA_REGISTRY_URL} substitution a deployment would use - pinned
    // before the configuration snapshot, like the mock cluster
    let registry = EmbeddedRegistry::start().await;
    std::env::set_var("SCHEMA_REGISTRY_URL", registry.base_url());
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

    // =======================================================================
    // K3 - inbound completions
    // =======================================================================

    // --- scenario 5: a topic-pattern binding routes the concrete matched topic;
    // metadata carries the record's ACTUAL topic, not the binding's regex
    produce(
        "k3-events.de",
        "{\"region\":\"de\"}",
        &[("cid", "k3-pattern-cid")],
    )
    .await;
    let pattern = await_captured_cid("k3-pattern-cid").await;
    assert_eq!("k3-events.de", pattern["metadata"]["topic"]);
    assert_eq!("KAFKA /k3-events.de", pattern["observed_trace_path"]);

    // --- scenario 6: second-level routing - an exact header rule selects the
    // flow, and serializer 'json' delivers a decoded map body
    produce(
        "k3-routed",
        "{\"hello\":\"routed\"}",
        &[
            ("type", "order"),
            ("cid", "k3-order-cid"),
            (
                "traceparent",
                "00-33333333333333333333333333333333-4444444444444444-01",
            ),
        ],
    )
    .await;
    let routed = await_captured_cid("k3-order-cid").await;
    assert_eq!(
        "order", routed["type"],
        "the routing key the rule matched on"
    );
    assert_eq!(
        "routed", routed["body"]["hello"],
        "serializer 'json' delivered a decoded map to the selected flow"
    );
    assert_eq!(
        "33333333333333333333333333333333",
        routed["observed_trace_id"]
    );

    // --- scenario 7: the wildcard rule
    produce(
        "k3-routed",
        "{\"hello\":\"bulk\"}",
        &[("type", "bulk-7"), ("cid", "k3-bulk-cid")],
    )
    .await;
    assert_eq!("bulk-7", await_captured_cid("k3-bulk-cid").await["type"]);

    // --- scenario 8: an input.body rule routes to a task:// target - the whole
    // decoded payload is the body, record headers are copied verbatim, the
    // business cid rides the my_cid tag and the trace stays continuous
    produce(
        "k3-routed",
        "{\"event\":{\"kind\":\"refund\"},\"amount\":10}",
        &[
            ("cid", "k3-refund-cid"),
            (
                "traceparent",
                "00-55555555555555555555555555555555-6666666666666666-01",
            ),
        ],
    )
    .await;
    let refund = await_captured_cid("k3-refund-cid").await;
    assert_eq!(
        "task", refund["kind"],
        "the task target received it, not a flow"
    );
    assert_eq!("refund", refund["body"]["event"]["kind"]);
    assert_eq!(10, refund["body"]["amount"]);
    assert_eq!(
        "k3-refund-cid", refund["headers"]["cid"],
        "inbound record headers are copied verbatim to the task"
    );
    assert_eq!(
        "55555555555555555555555555555555",
        refund["observed_trace_id"]
    );
    assert_eq!("KAFKA /k3-routed", refund["observed_trace_path"]);

    // --- scenario 9: a top-level JSON array decodes to a list, addressable by a
    // bracket rule and delivered whole
    produce(
        "k3-routed",
        "[{\"type\":\"batch-order\"},{\"type\":\"noise\"}]",
        &[("cid", "k3-batch-cid")],
    )
    .await;
    let batch = await_captured_cid("k3-batch-cid").await;
    assert_eq!("task", batch["kind"]);
    assert_eq!("batch-order", batch["body"][0]["type"]);

    // --- scenario 10: a non-JSON record keeps its raw bytes, every body rule is
    // a non-match, and the default target receives the bytes unchanged
    produce("k3-routed", "not-json-payload", &[("cid", "k3-raw-cid")]).await;
    let raw = await_captured_cid("k3-raw-cid").await;
    assert_eq!(
        "task", raw["kind"],
        "the default rule caught the unmatched record"
    );
    assert_eq!("not-json-payload", payload_text(&raw));

    // --- scenario 11: per-binding header-name overrides (impedance matching
    // for a legacy upstream)
    produce(
        "k3-legacy",
        "legacy",
        &[
            ("X-Correlation-ID", "k3-legacy-cid"),
            ("X-Legacy-Trace", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        ],
    )
    .await;
    let legacy = await_captured_cid("k3-legacy-cid").await;
    assert_eq!(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", legacy["observed_trace_id"],
        "the binding's trace-id header is the fallback trace source"
    );
    produce(
        "k3-legacy",
        "legacy-context",
        &[
            ("X-Correlation-ID", "k3-legacy-tp"),
            (
                "X-Trace-Context",
                "00-bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb-cccccccccccccccc-01",
            ),
        ],
    )
    .await;
    assert_eq!(
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        await_captured_cid("k3-legacy-tp").await["observed_trace_id"],
        "the binding's traceparent header carries the W3C context when the standard one is absent"
    );
    produce(
        "k3-legacy",
        "standard-wins",
        &[
            ("X-Correlation-ID", "k3-legacy-std"),
            (
                "traceparent",
                "00-dddddddddddddddddddddddddddddddd-eeeeeeeeeeeeeeee-01",
            ),
            (
                "X-Trace-Context",
                "00-ffffffffffffffffffffffffffffffff-1111111111111111-01",
            ),
        ],
    )
    .await;
    assert_eq!(
        "dddddddddddddddddddddddddddddddd",
        await_captured_cid("k3-legacy-std").await["observed_trace_id"],
        "the standard traceparent always wins over the custom name"
    );

    // --- scenario 12: partition pinning - the binding reads exactly partition 1
    produce_to("k3-pinned", Some(0), b"p0", &[("cid", "k3-pinned-p0")]).await;
    produce_to("k3-pinned", Some(1), b"p1", &[("cid", "k3-pinned-p1")]).await;
    let pinned = await_captured_cid("k3-pinned-p1").await;
    assert_eq!(1, pinned["metadata"]["partition"]);
    tokio::time::sleep(Duration::from_millis(1500)).await;
    assert!(
        !captured_has_cid("k3-pinned-p0"),
        "a record on the unpinned partition never reaches the pinned binding"
    );

    // --- scenario 13: an auto-commit binding delivers like any other (the
    // commit timing is the client's; the overlay is pinned at unit level)
    produce("k3-autocommit", "clicks", &[("cid", "k3-auto-cid")]).await;
    assert_eq!(
        "k3-autocommit",
        await_captured_cid("k3-auto-cid").await["metadata"]["topic"]
    );

    // --- scenario 14: startup validation against the LIVE registries - a
    // task:// target must be a registered route, a flow:// target a compiled flow
    let unknown_task = ConfigReader::from_yaml_text(
        "consumer:\n  - topic: t\n    flows:\n      - 'default -> task://no.such.route'\n",
    )
    .expect("yaml");
    let error = minimalist_kafka::adapter::parse_bindings(&unknown_task).expect_err("rejected");
    assert!(
        error
            .message()
            .contains("references unknown task route 'no.such.route'"),
        "{}",
        error.message()
    );
    let valid = ConfigReader::from_yaml_text(
        "consumer:\n  - topic: t\n    flows:\n      - 'input.header.type(x) -> flow://kafka-ingest'\n      - 'default -> task://kafka.task.sink'\n",
    )
    .expect("yaml");
    assert_eq!(
        1,
        minimalist_kafka::adapter::parse_bindings(&valid)
            .expect("registered targets accepted")
            .len()
    );

    // --- scenario 16: schema.enabled decodes a Confluent JSON Schema frame by
    // its embedded id - the flow receives a map, not bytes
    let json_id = registry.register(
        "k5-schema-json-value",
        "JSON",
        r#"{"type":"object","additionalProperties":true}"#,
    );
    produce_bytes(
        "k5-schema-json",
        &SchemaCodec::frame(json_id, br#"{"hello":"schema","n":1}"#),
        &[("cid", "k5-json-cid")],
    )
    .await;
    let dataset = await_captured_cid("k5-json-cid").await;
    assert_eq!(
        serde_json::json!({"hello": "schema", "n": 1}),
        dataset["body"],
        "the decoded document is the body"
    );
    assert_eq!("k5-schema-json", dataset["metadata"]["topic"]);

    // --- scenario 17: the nested `schema: enabled:` spelling, an Avro frame -
    // the binary datum a stock Confluent Avro serializer writes, hand-computed
    // ('avro-schema' = len 11 -> zigzag 0x16; count 3 -> 0x06) - decodes to a
    // map through the registered writer schema
    let avro_id = registry.register(
        "k5-schema-avro-value",
        "AVRO",
        r#"{"type":"record","name":"Greeting","fields":[{"name":"hello","type":"string"},{"name":"count","type":"int","default":0}]}"#,
    );
    let mut datum = vec![0x16];
    datum.extend_from_slice(b"avro-schema");
    datum.push(0x06);
    produce_bytes(
        "k5-schema-avro",
        &SchemaCodec::frame(avro_id, &datum),
        &[("cid", "k5-avro-cid")],
    )
    .await;
    assert_eq!(
        serde_json::json!({"hello": "avro-schema", "count": 3}),
        await_captured_cid("k5-avro-cid").await["body"]
    );

    // --- scenario 18: a poison frame is dead-lettered AT ONCE - no retry, no
    // flow attempt - with the raw record preserved and the cause named; an
    // unresolvable id is poison too
    let captured_before = captured().lock().expect("captured").len();
    let dlq_consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers())
        .set("group.id", "k5-dlq-observer")
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .create()
        .expect("dlq consumer");
    dlq_consumer
        .subscribe(&["k5-schema-dlq"])
        .expect("subscribe");
    produce("k5-schema-json", "not-framed", &[("cid", "k5-poison-cid")]).await;
    let parked = tokio::time::timeout(Duration::from_secs(15), dlq_consumer.recv())
        .await
        .expect("dead letter within deadline")
        .expect("dead letter");
    assert_eq!(
        "not-framed",
        String::from_utf8_lossy(parked.payload().unwrap_or_default()),
        "the raw record is preserved"
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
    assert_eq!("k5-schema-json", parked_headers["dlq.origin.topic"]);
    assert_eq!("k5-poison-cid", parked_headers["cid"]);
    assert!(
        parked_headers["dlq.error"].contains("not Confluent schema-framed"),
        "the dead letter names the decode failure: {}",
        parked_headers["dlq.error"]
    );
    produce_bytes(
        "k5-schema-json",
        &SchemaCodec::frame(987_654, br#"{"hello":"orphan"}"#),
        &[("cid", "k5-orphan-cid")],
    )
    .await;
    let parked = tokio::time::timeout(Duration::from_secs(15), dlq_consumer.recv())
        .await
        .expect("dead letter within deadline")
        .expect("dead letter");
    let orphan_error = parked
        .headers()
        .and_then(|h| h.iter().find(|h| h.key == "dlq.error"))
        .map(|h| String::from_utf8_lossy(h.value.unwrap_or_default()).to_string())
        .unwrap_or_default();
    assert!(
        orphan_error.contains("Unable to resolve schema id 987654"),
        "{orphan_error}"
    );
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert_eq!(
        captured_before,
        captured().lock().expect("captured").len(),
        "a poison frame never reaches the flow (no retry attempt)"
    );

    // --- scenario 19: second-level routing over a DECODED schema body - the
    // rule reads the document, the flow target and the task target both
    // receive the decoded map
    produce_bytes(
        "k5-schema-routed",
        &SchemaCodec::frame(json_id, br#"{"kind":"order","id":7}"#),
        &[("cid", "k5-routed-order")],
    )
    .await;
    let routed = await_captured_cid("k5-routed-order").await;
    assert_eq!("order", routed["body"]["kind"]);
    assert_eq!(7, routed["body"]["id"]);
    assert_eq!("KAFKA /k5-schema-routed", routed["observed_trace_path"]);
    produce_bytes(
        "k5-schema-routed",
        &SchemaCodec::frame(json_id, br#"{"kind":"other"}"#),
        &[("cid", "k5-routed-default")],
    )
    .await;
    let defaulted = await_captured_cid("k5-routed-default").await;
    assert_eq!(
        "task", defaulted["kind"],
        "the default rule's task:// target"
    );
    assert_eq!("other", defaulted["body"]["kind"]);

    // --- scenario 15: the stop is what a SIGTERM/Ctrl-C triggers through the
    // shutdown hook - every binding consumer finishes and reports stopped
    // within the grace, so the process can go on shutting down with nothing
    // abandoned mid-flow (the consumers leave their groups as they drop)
    let still_running = tokio::task::spawn_blocking(minimalist_kafka::runtime::stop_flow_consumers)
        .await
        .expect("stop runs");
    assert_eq!(0, still_running, "every consumer stopped within the grace");
}
