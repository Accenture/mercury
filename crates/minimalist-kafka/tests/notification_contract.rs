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

//! The outbound contract (`simple.kafka.notification`), pinned end to end
//! against librdkafka's in-process mock cluster (design ruling Q5) — port spec
//! §3 items 6–8: routing headers, body shapes, header propagation, the
//! correlation-id fallback, and the fresh traceparent stamp.

use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;

use minimalist_kafka::{runtime, KafkaRequestPublisher, SimpleKafkaNotification};
use platform_core::{trace, w3c_trace, ComposableFunction, EventEnvelope, Platform, PostOffice};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::{Headers, Message};
use rdkafka::mocking::MockCluster;
use rdkafka::producer::FutureProducer;

/// One shared mock cluster + publisher for the whole file (the process-wide
/// runtime holds ONE publisher; tests isolate by topic). The cluster is
/// deliberately leaked — it must outlive every per-test tokio runtime.
fn bootstrap_servers() -> &'static str {
    static BOOTSTRAP: OnceLock<String> = OnceLock::new();
    BOOTSTRAP.get_or_init(|| {
        let cluster = MockCluster::new(1).expect("mock cluster");
        for topic in ["t-bytes", "t-json", "t-nil", "t-cid", "t-traced"] {
            cluster.create_topic(topic, 1, 1).expect("topic");
        }
        cluster.create_topic("t-partitioned", 3, 1).expect("topic");
        cluster.create_topic("t-spread", 3, 1).expect("topic");
        let servers = cluster.bootstrap_servers();
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &servers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("producer");
        runtime::set_publisher(std::sync::Arc::new(KafkaRequestPublisher::new(producer)));
        std::mem::forget(cluster);
        servers
    })
}

struct Received {
    partition: i32,
    payload: Option<Vec<u8>>,
    headers: HashMap<String, Vec<u8>>,
}

/// Consume `count` records from a topic with a fresh consumer group.
async fn consume(topic: &str, count: usize) -> Vec<Received> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers())
        .set("group.id", format!("g-{}", uuid::Uuid::new_v4().simple()))
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .create()
        .expect("consumer");
    consumer.subscribe(&[topic]).expect("subscribe");
    let mut received = Vec::new();
    for _ in 0..count {
        let message = tokio::time::timeout(Duration::from_secs(10), consumer.recv())
            .await
            .expect("record within deadline")
            .expect("record");
        let mut headers = HashMap::new();
        if let Some(borrowed) = message.headers() {
            for i in 0..borrowed.count() {
                let header = borrowed.get(i);
                headers.insert(
                    header.key.to_string(),
                    header.value.unwrap_or_default().to_vec(),
                );
            }
        }
        received.push(Received {
            partition: message.partition(),
            payload: message.payload().map(<[u8]>::to_vec),
            headers,
        });
    }
    received
}

async fn call(
    headers: &[(&str, &str)],
    body: EventEnvelope,
) -> Result<EventEnvelope, platform_core::AppError> {
    bootstrap_servers();
    let map: HashMap<String, String> = headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    SimpleKafkaNotification.handle_event(map, body, 1).await
}

fn text(value: &[u8]) -> String {
    String::from_utf8_lossy(value).to_string()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn bytes_pass_verbatim_with_forwarded_headers_and_explicit_partition() {
    call(
        &[
            ("topic", "t-partitioned"),
            ("partition", "2"),
            ("cid", "explicit-cid"),
            ("event", "order-created"),
        ],
        EventEnvelope::new().set_raw_body(rmpv::Value::Binary(b"payload-bytes".to_vec())),
    )
    .await
    .expect("publish succeeds");
    let records = consume("t-partitioned", 1).await;
    let record = &records[0];
    assert_eq!(2, record.partition, "explicit partition header wins");
    assert_eq!(Some(b"payload-bytes".to_vec()), record.payload);
    assert_eq!(
        "order-created",
        text(&record.headers["event"]),
        "other headers forwarded"
    );
    assert_eq!(
        "explicit-cid",
        text(&record.headers["cid"]),
        "explicit cid wins"
    );
    assert!(
        !record.headers.contains_key("topic"),
        "routing directives never ride as headers"
    );
    assert!(!record.headers.contains_key("partition"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn map_and_list_bodies_serialize_to_json_bytes() {
    call(
        &[("topic", "t-json")],
        EventEnvelope::new()
            .set_body(serde_json::json!({"kind": "map", "n": 7}))
            .expect("map body"),
    )
    .await
    .expect("map publish");
    call(
        &[("topic", "t-json")],
        EventEnvelope::new()
            .set_body(serde_json::json!(["a", "b"]))
            .expect("list body"),
    )
    .await
    .expect("list publish");
    let records = consume("t-json", 2).await;
    let first: serde_json::Value =
        serde_json::from_slice(records[0].payload.as_deref().unwrap()).expect("json");
    assert_eq!(serde_json::json!({"kind": "map", "n": 7}), first);
    let second: serde_json::Value =
        serde_json::from_slice(records[1].payload.as_deref().unwrap()).expect("json");
    assert_eq!(serde_json::json!(["a", "b"]), second);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn nil_body_publishes_a_tombstone() {
    call(&[("topic", "t-nil")], EventEnvelope::new())
        .await
        .expect("tombstone publish");
    let records = consume("t-nil", 1).await;
    assert_eq!(None, records[0].payload, "null body = Kafka tombstone");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn caller_input_errors_surface_with_java_parity_messages() {
    let missing_topic = call(&[], EventEnvelope::new()).await.expect_err("no topic");
    assert_eq!(400, missing_topic.status());
    assert_eq!("Missing 'topic' header", missing_topic.message());

    let string_body = call(
        &[("topic", "t-bytes")],
        EventEnvelope::new().set_body("plain text").expect("string"),
    )
    .await
    .expect_err("string body rejected");
    assert_eq!(400, string_body.status());
    assert!(
        string_body
            .message()
            .contains("body must be bytes, Map or List, got String"),
        "got: {}",
        string_body.message()
    );

    let subject = call(
        &[("topic", "t-bytes"), ("subject", "orders-value")],
        EventEnvelope::new().set_raw_body(rmpv::Value::Binary(b"{}".to_vec())),
    )
    .await
    .expect_err("schema path deferred");
    assert_eq!(501, subject.status());
    assert!(subject
        .message()
        .contains("Schema Registry support is deferred"));

    let bad_partition = call(
        &[("topic", "t-bytes"), ("partition", "not-a-number")],
        EventEnvelope::new().set_raw_body(rmpv::Value::Binary(b"x".to_vec())),
    )
    .await
    .expect_err("bad partition");
    assert_eq!(400, bad_partition.status());
}

/// The correlation-id auto-stamp: when the flow maps no value under the
/// configured header, the publisher stamps the flow's own business
/// correlation id (carried as the my_correlation_id reserved header).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn correlation_id_falls_back_to_the_flow_cid() {
    call(
        &[("topic", "t-cid"), ("my_correlation_id", "flow-cid-42")],
        EventEnvelope::new().set_raw_body(rmpv::Value::Binary(b"x".to_vec())),
    )
    .await
    .expect("publish");
    let records = consume("t-cid", 1).await;
    assert_eq!("flow-cid-42", text(&records[0].headers["cid"]));
    assert!(
        !records[0].headers.contains_key("my_correlation_id"),
        "reserved framework headers never ride to Kafka"
    );
}

/// The registered-function path: a traced RPC through the platform proves the
/// worker wiring end to end — the auto-registered route answers the request
/// (the publish was acknowledged), the record carries a FRESH traceparent
/// built from this hop's own span, and the trace stays continuous across the
/// Kafka boundary.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn traced_publish_stamps_a_fresh_traceparent() {
    bootstrap_servers();
    let platform = Platform::new();
    platform
        .register(
            "kafka.notify.test",
            std::sync::Arc::new(SimpleKafkaNotification),
            2,
        )
        .expect("register");
    let po = PostOffice::new(&platform);
    let trace_id = trace::new_trace_id();
    po.request(
        EventEnvelope::new()
            .set_to("kafka.notify.test")
            .set_header("topic", "t-traced")
            .set_trace(&trace_id, "PUBLISH /kafka")
            .set_correlation_id("biz-cid-7")
            .set_raw_body(rmpv::Value::Binary(b"traced".to_vec())),
        Duration::from_secs(10),
    )
    .await
    .expect("publish acknowledged through the worker");
    let records = consume("t-traced", 1).await;
    let record = &records[0];
    let traceparent = text(&record.headers[w3c_trace::TRACEPARENT]);
    assert!(
        traceparent.starts_with(&format!("00-{trace_id}-")),
        "fresh traceparent carries the live trace id: {traceparent}"
    );
    assert_eq!(
        "biz-cid-7",
        text(&record.headers["cid"]),
        "flow cid auto-stamped"
    );
}

/// Keyless records spread across partitions (murmur2_random default) — pinned
/// loosely: over 30 records on 3 partitions, at least two partitions receive
/// data (a sticky partitioner would put all 30 on one).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn keyless_records_spread_across_partitions() {
    bootstrap_servers();
    // a dedicated producer built the library way (partitioner defaulted)
    let producer: FutureProducer = {
        let mut config = ClientConfig::new();
        config.set("bootstrap.servers", bootstrap_servers());
        config.set("message.timeout.ms", "5000");
        config.set("partitioner", "murmur2_random");
        config.create().expect("producer")
    };
    let publisher = KafkaRequestPublisher::new(producer);
    for i in 0..30 {
        publisher
            .publish(
                "t-spread",
                None,
                HashMap::new(),
                Some(format!("n-{i}").into_bytes()),
            )
            .await
            .expect("publish");
    }
    let records = consume("t-spread", 30).await;
    let partitions: std::collections::HashSet<i32> = records.iter().map(|r| r.partition).collect();
    assert!(
        partitions.len() >= 2,
        "keyless records spread across partitions, got {partitions:?}"
    );
}
