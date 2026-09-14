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

//! The `kafka.health` contract against the in-process mock cluster: the info
//! shape, the live Metadata probe, the `{text, code}` 503 on a genuine
//! outage, the start-up placeholder with background warm-up, and the
//! waiting-vs-outage boundary (a client that cannot be BUILT is a passing
//! "waiting", never a restart-inviting failure — the vault pattern).

use std::time::Duration;

use minimalist_kafka::KafkaHealthProbe;
use platform_core::{AppError, EventEnvelope};
use rdkafka::config::ClientConfig;
use rdkafka::mocking::MockCluster;

fn reachable_config(servers: &str) -> ClientConfig {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", servers);
    config
}

fn body(event: &EventEnvelope) -> serde_json::Value {
    event.body_as().expect("json body")
}

fn status_of(event: &EventEnvelope) -> String {
    body(event)["status"]
        .as_str()
        .unwrap_or_default()
        .to_string()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn info_and_live_probe_report_the_cluster() {
    let cluster = MockCluster::new(1).expect("mock cluster");
    cluster.create_topic("observed", 1, 1).expect("topic");
    let servers = cluster.bootstrap_servers();
    let probe = KafkaHealthProbe::new(
        move || Ok(reachable_config(&servers)),
        Duration::from_secs(5),
        Duration::ZERO, // no grace - probe live immediately
    );
    let info = probe.handle("info").await.expect("info");
    assert_eq!("kafka", body(&info)["service"]);
    assert_eq!(cluster.bootstrap_servers(), body(&info)["href"]);

    let healthy = probe.handle("health").await.expect("health");
    assert!(!healthy.has_error());
    let report = body(&healthy);
    assert_eq!("Kafka cluster is reachable", report["status"]);
    assert!(
        report["topics"].as_u64().unwrap_or(0) >= 1,
        "topic count reported"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn outage_fails_health_with_text_and_code() {
    // bind-and-drop: a port nobody answers metadata on
    let dead_port = {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
        listener.local_addr().expect("addr").port()
    };
    let probe = KafkaHealthProbe::new(
        move || Ok(reachable_config(&format!("127.0.0.1:{dead_port}"))),
        Duration::from_secs(2),
        Duration::ZERO,
    );
    let reply = probe.handle("health").await.expect("health reply");
    assert_eq!(503, reply.status());
    let down = body(&reply);
    assert_eq!(503, down["code"]);
    assert!(
        down["text"]
            .as_str()
            .unwrap_or_default()
            .starts_with("Kafka cluster is not reachable - "),
        "got: {}",
        down["text"]
    );
}

/// A template the client cannot even be built from (or that cannot be
/// loaded) is a passing WAITING status — a pod restart cannot produce the
/// missing credential.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unusable_configuration_is_waiting_not_outage() {
    let unbuildable = KafkaHealthProbe::new(
        || {
            let mut config = ClientConfig::new();
            config.set("bootstrap.servers", "127.0.0.1:1");
            config.set("definitely.not.a.librdkafka.property", "x");
            Ok(config)
        },
        Duration::from_secs(2),
        Duration::ZERO,
    );
    let reply = unbuildable.handle("health").await.expect("health reply");
    assert!(!reply.has_error(), "waiting must never fail /health");
    assert_eq!("Waiting for Kafka connection", status_of(&reply));

    let unresolvable = KafkaHealthProbe::new(
        || {
            Err(AppError::new(
                500,
                "No Kafka client config found at any of: /vault/render.yml",
            ))
        },
        Duration::from_secs(2),
        Duration::ZERO,
    );
    let reply = unresolvable.handle("health").await.expect("health reply");
    assert!(!reply.has_error());
    assert_eq!("Waiting for Kafka connection", status_of(&reply));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn startup_grace_reports_placeholder_then_warms_up() {
    let cluster = MockCluster::new(1).expect("mock cluster");
    let servers = cluster.bootstrap_servers();
    let probe = KafkaHealthProbe::new(
        move || Ok(reachable_config(&servers)),
        Duration::from_secs(5),
        Duration::from_secs(30), // generous grace - the placeholder window
    );
    let first = probe.handle("health").await.expect("health");
    assert!(!first.has_error());
    assert_eq!("Kafka client is starting up", status_of(&first));
    // the background warm-up goes live without waiting out the grace period
    for _ in 0..500 {
        let reply = probe.handle("health").await.expect("health");
        if status_of(&reply) == "Kafka cluster is reachable" {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("warm-up never went live");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unknown_type_is_a_client_error() {
    let probe = KafkaHealthProbe::new(
        || Ok(ClientConfig::new()),
        Duration::from_secs(2),
        Duration::ZERO,
    );
    let error = probe.handle("bogus").await.expect_err("refused");
    assert_eq!(400, error.status());
}
