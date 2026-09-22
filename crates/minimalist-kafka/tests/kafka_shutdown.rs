//! The shutdown contract of the Kafka flow adapter, proven against a broker (librdkafka's mock
//! cluster) rather than inferred from the client's documentation — the twin of the Java module's
//! `KafkaShutdownTest`.
//!
//! Closing a flow consumer makes its member **leave** the consumer group at once: the coordinator
//! reassigns the member's partitions to the survivors immediately, instead of holding them until the
//! member's session expires (45 seconds by default under the KIP-848 consumer protocol), which is how
//! a rolling restart used to park every partition the old pod held. The bootstrap registers
//! `runtime::stop_flow_consumers` on `Platform::on_shutdown` so a `SIGTERM` takes exactly this path;
//! the registration itself belongs to the entry point and is verified live (the certification drives
//! of 2026-09-22), not here.
//!
//! The mock broker serves no `DescribeGroups`, so membership is observed through its consequence: two
//! members share a two-partition topic, and when the module's consumer closes, the survivor must hold
//! both partitions well inside the session timeout (30 s on the mock cluster). The mock coordinator
//! computes the assignments itself under the consumer protocol and hands the leaver's partition over
//! on the survivor's next heartbeat (3 s). Its classic coordinator rebalances slowly after a leave — a
//! mock artifact; a real broker records the explicit `LeaveGroup` within milliseconds under both
//! protocols — so the test pins the consumer protocol, which is what the bundled template's `auto`
//! resolves to on a KIP-848 cluster.
use std::sync::Arc;
use std::time::{Duration, Instant};

use minimalist_kafka::{runtime, KafkaConsumerBinding, KafkaFlowConsumer, RetryPolicy};
use platform_core::Platform;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::mocking::MockCluster;
use rdkafka::producer::DefaultProducerContext;

/// Far below the mock cluster's 30 s consumer session timeout (and the 45 s broker default): a
/// member that merely vanished would keep its partitions for the whole session.
const WELL_INSIDE_SESSION_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn closing_a_flow_consumer_leaves_the_group_at_once() {
    let topic = "shutdown-test-topic";
    let group = "shutdown-test-group";
    let (cluster, config) = mock_cluster(topic, 2, group);
    // the survivor: a plain consumer in the same group, serviced by a receive loop so its
    // rebalances are processed
    let survivor: Arc<StreamConsumer> = Arc::new(config.create().expect("survivor consumer"));
    survivor.subscribe(&[topic]).expect("subscribe");
    let servicing = survivor.clone();
    tokio::spawn(async move {
        loop {
            let _ = servicing.recv().await;
        }
    });
    // the member under test: the module's own flow consumer, the code a SIGTERM stops
    let member = flow_member(&config, topic, group);
    // the group settles: the coordinator gives each member one of the two partitions
    wait_for(|| assigned(&survivor) == 1, Duration::from_secs(45)).await;
    assert_eq!(
        1,
        assigned(&survivor),
        "the two members should share the two partitions before the close"
    );

    // the property under test: close() = LeaveGroup, observed by the coordinator at once - the
    // survivor receives the leaver's partition on its next heartbeat, not after the session timeout
    let started = Instant::now();
    member.close();
    wait_for(|| member.is_stopped(), Duration::from_secs(15)).await;
    assert!(
        member.is_stopped(),
        "the poll loop should exit once told to stop"
    );
    wait_for(|| assigned(&survivor) == 2, WELL_INSIDE_SESSION_TIMEOUT).await;
    let elapsed = started.elapsed();
    assert_eq!(
        2,
        assigned(&survivor),
        "closing the consumer must make the member LEAVE the group so its partition moves to the \
         survivor, not linger until its session expires (survivor's partitions after {elapsed:?})"
    );
    assert!(
        elapsed < WELL_INSIDE_SESSION_TIMEOUT,
        "the leave was observed in {elapsed:?} - a session-timeout fence would have taken 30-45 s"
    );
    eprintln!("the survivor held the leaver's partition {elapsed:?} after the close");
    // the mock broker outlives the test: the survivor closes against it when the runtime stops
    std::mem::forget(cluster);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn stop_flow_consumers_stops_every_registered_consumer_and_is_idempotent() {
    let topic = "shutdown-hook-topic";
    let group = "shutdown-hook-group";
    let (cluster, config) = mock_cluster(topic, 1, group);
    // the shutdown hook's own routine, on the consumers the bootstrap would have registered
    runtime::set_flow_consumers(vec![flow_member(&config, topic, group)]);
    let still_running = tokio::task::spawn_blocking(runtime::stop_flow_consumers)
        .await
        .expect("stop runs");
    assert_eq!(
        0, still_running,
        "the registered consumer must finish within the grace"
    );
    // a second call is a no-op (the platform hook and a test teardown may both run it)
    let again = tokio::task::spawn_blocking(runtime::stop_flow_consumers)
        .await
        .expect("stop runs again");
    assert_eq!(0, again, "a second stop finds nothing running");
    // and nothing-started is fine too
    runtime::set_flow_consumers(Vec::new());
    assert_eq!(0, runtime::stop_flow_consumers());
    std::mem::forget(cluster);
}

/// A one-broker mock cluster with the topic, and a client config for the group under the KIP-848
/// consumer protocol (see the module doc for why the classic protocol is not pinned here).
fn mock_cluster(
    topic: &str,
    partitions: i32,
    group: &str,
) -> (MockCluster<'static, DefaultProducerContext>, ClientConfig) {
    let cluster = MockCluster::new(1).expect("mock cluster");
    cluster.create_topic(topic, partitions, 1).expect("topic");
    let mut config = ClientConfig::new();
    config
        .set("bootstrap.servers", cluster.bootstrap_servers())
        .set("group.id", group)
        .set("group.protocol", "consumer")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest");
    (cluster, config)
}

/// The module's flow consumer for the topic, in the group - no record is ever produced to it, so the
/// binding's flow is never reached.
fn flow_member(config: &ClientConfig, topic: &str, group: &str) -> KafkaFlowConsumer {
    let mut binding = KafkaConsumerBinding::direct(topic, "shutdown-test-flow");
    binding.group_id = group.to_string();
    let policy = RetryPolicy {
        max_retries: 1,
        backoff_ms: 10,
        dead_letter_publisher: None,
    };
    KafkaFlowConsumer::start(
        Platform::get_instance(),
        config.clone(),
        binding,
        policy,
        Duration::from_secs(5),
        false,
        None,
    )
    .expect("flow consumer")
}

fn assigned(consumer: &StreamConsumer) -> usize {
    consumer.assignment().map(|list| list.count()).unwrap_or(0)
}

async fn wait_for(mut condition: impl FnMut() -> bool, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    while !condition() && Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
