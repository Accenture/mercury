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
//!
//! The producer half of the same contract: after the consumers, the shutdown hook flushes the shared
//! producer within the same grace — a record a caller enqueued is delivered before the process
//! exits — and forgets the handle; a broker that cannot take the records in time is reported as an
//! undelivered count, never waited on past the grace (Java's producer close waits without bound).
//! The flush waits through the client's linger (`linger.ms`, 5 ms by default) rather than cutting
//! it short: the `rdkafka` crate's flush calls librdkafka's `rd_kafka_flush` with a zero timeout in
//! a poll loop, so the "linger ignored while flushing" flag is never seen by the broker thread — a
//! 5 s linger measured a 4.98 s flush. Accepted: any sane linger is far inside the grace, and the
//! alternative is this crate's first `unsafe` FFI call.
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use minimalist_kafka::{
    runtime, KafkaConsumerBinding, KafkaFlowConsumer, KafkaRequestPublisher, RetryPolicy,
};
use platform_core::Platform;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::mocking::MockCluster;
use rdkafka::producer::{DefaultProducerContext, FutureProducer};

/// Far below the mock cluster's 30 s consumer session timeout (and the 45 s broker default): a
/// member that merely vanished would keep its partitions for the whole session.
const WELL_INSIDE_SESSION_TIMEOUT: Duration = Duration::from_secs(15);

/// One mock heartbeat interval (3 s) plus margin: the time a freshly assigned member needs to
/// acknowledge its assignment before it is a settled member of the group.
const SETTLED_MEMBER: Duration = Duration::from_secs(4);

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
    // ... and the leaver has acknowledged its own assignment on its next heartbeat (3 s on the mock).
    // Measured: a member closed within that first heartbeat lost its leave in about one run in seven
    // (the survivor then waited the mock's full 30 s session for the partition); a member closed
    // mid-reconciliation is not the contract under test, a settled member is.
    tokio::time::sleep(SETTLED_MEMBER).await;

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

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn close_publisher_delivers_the_lingering_records_then_forgets_the_handle() {
    // --- a record held back by the client's linger is delivered by the flush ---
    let topic = "shutdown-flush-topic";
    let cluster = MockCluster::new(1).expect("mock cluster");
    cluster.create_topic(topic, 1, 1).expect("topic");
    let mut config = ClientConfig::new();
    config
        .set("bootstrap.servers", cluster.bootstrap_servers())
        // a deliberately long linger (the client's default is 5 ms): the record sits in
        // the client's queue when the close begins, and the flush waits it out - the
        // safe rdkafka flush cannot shorten the linger (see the module doc)
        .set("queue.buffering.max.ms", "1500")
        .set("message.timeout.ms", "30000");
    let producer: FutureProducer = config.create().expect("producer");
    let publisher = Arc::new(KafkaRequestPublisher::new(producer));
    runtime::set_publisher(publisher.clone());
    let caller = publisher.clone();
    let publish = tokio::spawn(async move {
        caller
            .publish(topic, None, HashMap::new(), Some(b"lingering".to_vec()))
            .await
    });
    wait_for(|| publisher.in_flight_count() >= 1, Duration::from_secs(5)).await;
    assert!(
        publisher.in_flight_count() >= 1,
        "the record should be waiting in the client's queue (the linger holds it)"
    );
    let started = Instant::now();
    let undelivered = tokio::task::spawn_blocking(runtime::close_publisher)
        .await
        .expect("close runs");
    assert_eq!(
        0, undelivered,
        "the flush must deliver the lingering record"
    );
    let elapsed = started.elapsed();
    // the caller's own future resolves with the delivery report the flush served
    let delivered = tokio::time::timeout(Duration::from_secs(1), publish)
        .await
        .expect("the caller's publish must complete once flushed")
        .expect("publish task");
    assert!(
        delivered.is_ok(),
        "the flushed record is acknowledged: {delivered:?}"
    );
    assert!(
        elapsed < Duration::from_secs(5),
        "the flush waited out the linger and returned in {elapsed:?} - well inside the 10 s grace"
    );
    eprintln!("the lingering record was delivered {elapsed:?} after the close began");
    assert!(
        runtime::publisher().is_none(),
        "close_publisher forgets the handle - a late caller sees 'not started'"
    );
    // a second call finds nothing to do (the platform hook and a test teardown may both run it)
    assert_eq!(0, runtime::close_publisher());
    drop(publisher);

    // --- a broker that cannot take the records is not waited on past the grace ---
    let mut unreachable = ClientConfig::new();
    unreachable
        .set("bootstrap.servers", "127.0.0.1:1")
        .set("message.timeout.ms", "60000");
    let producer: FutureProducer = unreachable.create().expect("producer");
    let publisher = Arc::new(KafkaRequestPublisher::new(producer));
    runtime::set_publisher(publisher.clone());
    let caller = publisher.clone();
    let stuck = tokio::spawn(async move {
        caller
            .publish(topic, None, HashMap::new(), Some(b"stranded".to_vec()))
            .await
    });
    wait_for(|| publisher.in_flight_count() >= 1, Duration::from_secs(5)).await;
    let started = Instant::now();
    let undelivered =
        tokio::task::spawn_blocking(|| runtime::close_publisher_within(Duration::from_millis(500)))
            .await
            .expect("close runs");
    assert_eq!(
        1, undelivered,
        "the grace ended with the stranded record still undelivered - reported, not waited for"
    );
    assert!(
        started.elapsed() < Duration::from_secs(3),
        "the close honours its grace against a dead broker ({:?})",
        started.elapsed()
    );
    assert!(runtime::publisher().is_none());
    stuck.abort();
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
