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

//! `simple.kafka.notification` on the Schema Registry path (the Java
//! `SimpleKafkaNotification` subject-driven produce): a `subject` header
//! resolves to a pre-registered global schema id and type, the bytes body (a
//! JSON document) is framed in the Confluent wire format, and the routing
//! directives never reach Kafka as headers. Against the mock cluster and the
//! in-process registry double; one runtime, sequential scenarios (the
//! platform's HTTP client service belongs to the runtime that registers it).

#[path = "support/embedded_registry.rs"]
mod embedded_registry;

use std::collections::HashMap;
use std::sync::{Arc, Once};
use std::time::Duration;

use embedded_registry::EmbeddedRegistry;
use minimalist_kafka::schema::json_view;
use minimalist_kafka::{runtime, KafkaRequestPublisher, SchemaCodec, SimpleKafkaNotification};
use platform_core::automation::http_client::AsyncHttpClientService;
use platform_core::automation::ASYNC_HTTP_REQUEST;
use platform_core::platform::FunctionOptions;
use platform_core::{resources, AppConfigReader, ComposableFunction, EventEnvelope, Platform};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::{Headers, Message};
use rdkafka::mocking::MockCluster;
use rdkafka::producer::FutureProducer;

const JSON_SCHEMA: &str =
    r#"{"type":"object","properties":{"hello":{"type":"string"}},"additionalProperties":true}"#;
const AVRO_SCHEMA: &str = r#"{"type":"record","name":"Greeting","fields":[{"name":"hello","type":"string"},{"name":"count","type":"int","default":0}]}"#;

fn bootstrap_servers() -> String {
    static BOOTSTRAP: Once = Once::new();
    static mut SERVERS: Option<String> = None;
    #[allow(static_mut_refs)]
    unsafe {
        BOOTSTRAP.call_once(|| {
            let cluster = MockCluster::new(1).expect("mock cluster");
            for topic in ["t-schema-json", "t-schema-avro"] {
                cluster.create_topic(topic, 1, 1).expect("topic");
            }
            let servers = cluster.bootstrap_servers();
            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", &servers)
                .set("message.timeout.ms", "5000")
                .create()
                .expect("producer");
            runtime::set_publisher(Arc::new(KafkaRequestPublisher::new(producer)));
            resources::prepend_resource_root("tests/resources");
            let _ = AppConfigReader::get_instance();
            std::mem::forget(cluster);
            SERVERS = Some(servers);
        });
        SERVERS.clone().expect("servers")
    }
}

struct Received {
    payload: Vec<u8>,
    headers: HashMap<String, String>,
}

async fn consume_one(topic: &str) -> Received {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers())
        .set("group.id", format!("g-{}", uuid::Uuid::new_v4().simple()))
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .create()
        .expect("consumer");
    consumer.subscribe(&[topic]).expect("subscribe");
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
                String::from_utf8_lossy(header.value.unwrap_or_default()).to_string(),
            );
        }
    }
    Received {
        payload: message.payload().unwrap_or_default().to_vec(),
        headers,
    }
}

async fn call(
    headers: &[(&str, &str)],
    body: EventEnvelope,
) -> Result<EventEnvelope, platform_core::AppError> {
    let map: HashMap<String, String> = headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    SimpleKafkaNotification.handle_event(map, body, 1).await
}

fn bytes(text: &str) -> EventEnvelope {
    EventEnvelope::new().set_raw_body(rmpv::Value::Binary(text.as_bytes().to_vec()))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn subject_driven_publish_frames_the_document() {
    bootstrap_servers();
    let platform = Platform::get_instance();
    if !platform.has_route(ASYNC_HTTP_REQUEST) {
        platform
            .register_with_options(
                ASYNC_HTTP_REQUEST,
                Arc::new(AsyncHttpClientService::new(&platform)),
                10,
                FunctionOptions {
                    zero_traced: false,
                    interceptor: true,
                    private: true,
                },
            )
            .expect("register http client");
    }

    // --- without a registry the subject header is refused as a configuration error
    let refused = call(
        &[("topic", "t-schema-json"), ("subject", "t-schema-value")],
        bytes("{\"hello\":\"world\"}"),
    )
    .await
    .expect_err("refused");
    assert_eq!(500, refused.status());
    assert!(
        refused
            .message()
            .contains("'subject' header set but 'schema.registry.url' is not configured"),
        "{}",
        refused.message()
    );

    // --- the codec against the registry double (what the auto-start installs from config)
    let registry = EmbeddedRegistry::start().await;
    let codec = SchemaCodec::for_registry(
        AppConfigReader::get_instance(),
        Some(registry.base_url()),
        "schema.registry",
    )
    .expect("codec builds")
    .expect("registry url given");
    runtime::set_schema_codec(codec.clone());
    let json_id = registry.register("t-schema-value", "JSON", JSON_SCHEMA);
    let avro_id = registry.register("t-greeting-value", "AVRO", AVRO_SCHEMA);

    // --- 1. subject (version defaults to latest): the JSON document is framed with the id;
    // subject/version never travel as Kafka headers, other headers still do
    call(
        &[
            ("topic", "t-schema-json"),
            ("subject", "t-schema-value"),
            ("cid", "cid-schema-1"),
            ("x-extra", "kept"),
        ],
        bytes("{\"hello\":\"world\"}"),
    )
    .await
    .expect("published");
    let received = consume_one("t-schema-json").await;
    assert!(SchemaCodec::is_framed(&received.payload));
    assert_eq!(json_id, SchemaCodec::schema_id(&received.payload));
    assert_eq!(b"{\"hello\":\"world\"}", &received.payload[5..]);
    assert_eq!("cid-schema-1", received.headers["cid"]);
    assert_eq!("kept", received.headers["x-extra"]);
    assert!(
        !received.headers.contains_key("subject"),
        "an encoding directive, not a header"
    );
    assert!(!received.headers.contains_key("version"));

    // --- 2. a pinned version resolves the same id; Avro re-encodes the document as a datum
    call(
        &[
            ("topic", "t-schema-avro"),
            ("subject", "t-greeting-value"),
            ("version", "1"),
        ],
        bytes("{\"hello\":\"avro\"}"),
    )
    .await
    .expect("published");
    let received = consume_one("t-schema-avro").await;
    assert_eq!(avro_id, SchemaCodec::schema_id(&received.payload));
    assert_eq!(
        &[0x08, b'a', b'v', b'r', b'o', 0x00],
        &received.payload[5..],
        "the Avro datum: 'avro' + count default 0"
    );
    let decoded = json_view(
        &codec
            .decode("t-schema-avro", Some(&received.payload))
            .await
            .expect("decodes back"),
    );
    assert_eq!(serde_json::json!({"hello": "avro", "count": 0}), decoded);

    // --- 3. the caller-input errors, Java parity messages
    let not_bytes = call(
        &[("topic", "t-schema-json"), ("subject", "t-schema-value")],
        EventEnvelope::new()
            .set_body(serde_json::json!({"hello": "map"}))
            .expect("body"),
    )
    .await
    .expect_err("rejected");
    assert_eq!(400, not_bytes.status());
    assert!(not_bytes
        .message()
        .contains("body must be bytes (a JSON document) when 'subject' is set, got Map"));
    let not_json = call(
        &[("topic", "t-schema-json"), ("subject", "t-schema-value")],
        bytes("not a document"),
    )
    .await
    .expect_err("rejected");
    assert_eq!(400, not_json.status());
    assert!(not_json.message().contains("body is not a JSON document"));
    let unknown = call(
        &[("topic", "t-schema-json"), ("subject", "nope")],
        bytes("{}"),
    )
    .await
    .expect_err("rejected");
    assert_eq!(404, unknown.status());
    assert!(unknown
        .message()
        .contains("Unable to resolve subject 'nope'"));
    let bad_version = call(
        &[
            ("topic", "t-schema-json"),
            ("subject", "t-schema-value"),
            ("version", "v1"),
        ],
        bytes("{}"),
    )
    .await
    .expect_err("rejected");
    assert_eq!(400, bad_version.status());
    assert!(bad_version
        .message()
        .contains("'version' must be 'latest' or a positive integer"));
    let closed_shape = call(
        &[("topic", "t-schema-avro"), ("subject", "t-greeting-value")],
        bytes("{\"count\": 1}"),
    )
    .await
    .expect_err("rejected");
    assert_eq!(400, closed_shape.status());
    assert!(
        closed_shape
            .message()
            .contains("field 'hello' of Greeting is missing and has no default"),
        "{}",
        closed_shape.message()
    );
    runtime::clear_schema_codec();
}
