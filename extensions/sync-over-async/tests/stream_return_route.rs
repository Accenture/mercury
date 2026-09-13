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

//! Experiment R1 — the Rust twin of the Java `StreamReturnRouteTest` (E1), one
//! scenario for one scenario.
//!
//! The UI pod opens a streaming rendezvous (`begin_stream`) and
//! [`StreamResponder`] producers — built from plain settings, no coordinator,
//! exactly as a backend service would — post segments cross-pod through Redis
//! alone. Covers both driving use cases: the sequential single producer whose
//! ordering must hold end to end (AI chat), and several uncoordinated producers
//! on one cid where either side may close the channel (event notification) —
//! plus the recovery paths (missed wake-up, final drain at idle expiry), the
//! orphan contract, capacity, and the unification (a one-shot response
//! completed by a stream producer's terminal post: the degenerate stream shown
//! to be degenerate).
//!
//! The Java suite runs against an embedded redis-server binary; this toolchain
//! has neither that nor a Docker daemon (deliberately — VDI-class machines
//! cannot virtualize), so every scenario runs against the shared in-process
//! **RESP2 test double**: real TCP, real protocol frames through the real
//! `redis` crate. Each test owns its own double on an ephemeral port, so the
//! suite is order-independent and parallel-safe.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::AppError;
use redis_test_double::start_resp_double;
use sync_over_async::{
    segment, RedisSettings, ReturnRouteCoordinator, ReturnRouteStore, SegmentSink, StreamResponder,
    StreamSegment, SyncOverAsyncConfig,
};

/// Matches the Java suite's config: small caps so capacity is testable.
fn test_config() -> SyncOverAsyncConfig {
    SyncOverAsyncConfig::new("svc-return", 90, 30, 100, 1800, 100)
}

fn settings(port: u16) -> RedisSettings {
    RedisSettings::new("127.0.0.1", port, "", false, 0, 5000)
}

fn new_cid() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

/// The UI pod: a started coordinator plus the double it talks to.
struct UiPod {
    coordinator: ReturnRouteCoordinator,
    port: u16,
    store: redis_test_double::SharedStore,
}

impl UiPod {
    async fn start() -> UiPod {
        UiPod::start_with(test_config(), "pod-UI").await
    }

    async fn start_with(config: SyncOverAsyncConfig, origin: &str) -> UiPod {
        let (port, store, _journal) = start_resp_double("7.4.1").await;
        let coordinator = ReturnRouteCoordinator::connect(&settings(port), origin, config)
            .await
            .expect("coordinator connects");
        coordinator.start().await.expect("subscriber starts");
        UiPod {
            coordinator,
            port,
            store,
        }
    }

    async fn responder(&self) -> StreamResponder {
        StreamResponder::connect(&settings(self.port))
            .await
            .expect("responder connects")
    }

    /// Wire-visible state, straight out of the double's key space.
    fn key_exists(&self, key: &str) -> bool {
        self.store
            .lock()
            .expect("raw store")
            .contains_key(key.as_bytes())
    }

    fn key_has_ttl(&self, key: &str) -> bool {
        self.store
            .lock()
            .expect("raw store")
            .get(key.as_bytes())
            .map(|entry| entry.expires_at.is_some())
            .unwrap_or(false)
    }

    /// Poll until the route key is gone - the drain's terminal cleanup has
    /// fully completed.
    async fn await_route_gone(&self, cid: &str) {
        let key = ReturnRouteStore::route_key(cid);
        for _ in 0..500 {
            if !self.key_exists(&key) {
                return;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        panic!("route key for {cid} was not cleaned up in time");
    }
}

/// Test sink: collects every delivered segment in forward order and signals on
/// the terminal one.
#[derive(Default)]
struct CollectingSink {
    segments: Mutex<Vec<StreamSegment>>,
    closed: tokio::sync::Notify,
    terminal: AtomicBool,
}

#[async_trait]
impl SegmentSink for CollectingSink {
    async fn accept(&self, segment: StreamSegment) -> Result<(), AppError> {
        let terminal = segment.is_terminal();
        self.segments.lock().expect("segments").push(segment);
        if terminal {
            self.terminal.store(true, Ordering::Release);
            self.closed.notify_waiters();
        }
        Ok(())
    }
}

impl CollectingSink {
    fn bodies(&self) -> Vec<Option<String>> {
        self.segments
            .lock()
            .expect("segments")
            .iter()
            .map(|s| s.body().map(str::to_string))
            .collect()
    }

    fn names(&self) -> Vec<Option<String>> {
        self.segments
            .lock()
            .expect("segments")
            .iter()
            .map(|s| s.name().map(str::to_string))
            .collect()
    }

    fn len(&self) -> usize {
        self.segments.lock().expect("segments").len()
    }

    fn last_type(&self) -> Option<String> {
        self.segments
            .lock()
            .expect("segments")
            .last()
            .map(|s| s.segment_type().to_string())
    }

    /// Wait for the terminal segment; false on timeout.
    async fn await_closed(&self, duration: Duration) -> bool {
        let closed = self.closed.notified();
        tokio::pin!(closed);
        if self.terminal.load(Ordering::Acquire) {
            return true;
        }
        tokio::time::timeout(duration, closed).await.is_ok()
    }

    /// Poll until the sink has received its first segment (expectations with no
    /// terminal to wait on).
    async fn await_first_segment(&self) {
        for _ in 0..500 {
            if self.len() > 0 {
                return;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        panic!("expected a first segment, got none in time");
    }
}

/// A consumer that is already gone - every delivery fails.
struct FailingSink;

#[async_trait]
impl SegmentSink for FailingSink {
    async fn accept(&self, _segment: StreamSegment) -> Result<(), AppError> {
        Err(AppError::new(500, "simulated: the HTTP edge is gone"))
    }
}

fn text(value: &str) -> Option<String> {
    Some(value.to_string())
}

/// Post a **terminal** segment without asserting liveness.
///
/// A closing post races its own consumption by construction: the segment is
/// appended store-first, and an in-flight drain (woken by an earlier post) may
/// pop it, deliver it, and close the rendezvous — deleting the route — before
/// the producer's own route check runs. `post` then answers `false` although
/// the segment *was* delivered. That answer is still correct under the
/// contract ("false = stop producing", and a producer has nothing to send
/// after a terminal segment), so the scenarios below assert on what actually
/// matters: the sink saw the terminal segment, and the rendezvous closed.
/// Liveness is asserted on DATA posts, where a `false` would mean lost tokens.
async fn post_terminal(
    responder: &StreamResponder,
    cid: &str,
    name: Option<&str>,
    body: Option<&str>,
) {
    responder
        .post(cid, segment::EOF, name, body)
        .await
        .expect("post succeeds");
}

// ------------------------------------------------------------------
// Use case: chat with an AI agent - one sequential producer, strict order
// ------------------------------------------------------------------

#[tokio::test]
async fn sequential_producer_delivers_in_exact_order() {
    let pod = UiPod::start().await;
    let cid = new_cid();
    let sink = Arc::new(CollectingSink::default());
    pod.coordinator
        .begin_stream(&cid, sink.clone())
        .await
        .expect("stream opens");
    let tokens = [
        "The",
        " quick",
        " brown",
        " fox",
        " jumps",
        " over",
        " the",
        " lazy dog",
    ];
    let responder = pod.responder().await;
    for token in tokens {
        assert!(
            responder
                .post(&cid, segment::DATA, None, Some(token))
                .await
                .expect("post succeeds"),
            "rendezvous is live"
        );
    }
    post_terminal(&responder, &cid, None, Some("{\"total\":8}")).await;

    assert!(
        sink.await_closed(Duration::from_secs(5)).await,
        "terminal segment delivered"
    );
    // no sequence number anywhere: posting discipline -> list order ->
    // serialized drain == exact order
    let expected: Vec<Option<String>> = tokens.iter().map(|t| text(t)).collect();
    assert_eq!(expected, sink.bodies()[..tokens.len()].to_vec());
    assert_eq!(Some(segment::EOF.to_string()), sink.last_type());
    assert_eq!(text("{\"total\":8}"), sink.bodies()[tokens.len()]);
    assert_eq!(
        tokens.len() + 1,
        sink.len(),
        "nothing lost, nothing duplicated"
    );

    // terminal cleanup: entry closed, keys deleted eagerly
    pod.await_route_gone(&cid).await;
    assert_eq!(0, pod.coordinator.active_streams());
    assert!(
        !pod.key_exists(&ReturnRouteStore::queue_key(&cid)),
        "queue deleted with the rendezvous"
    );
}

// ------------------------------------------------------------------
// Use case: event notification - several producers, unordered, anyone may close
// ------------------------------------------------------------------

#[tokio::test]
async fn concurrent_producers_interleave_without_loss_or_duplication() {
    const PER_PRODUCER: usize = 25;
    let pod = UiPod::start().await;
    let cid = new_cid();
    let sink = Arc::new(CollectingSink::default());
    pod.coordinator
        .begin_stream(&cid, sink.clone())
        .await
        .expect("stream opens");
    let producers = ["svc-A", "svc-B", "svc-C"];
    let mut posting = Vec::new();
    for producer in producers {
        // each producer owns its responder = its own connection, as separate
        // pods would
        let responder = pod.responder().await;
        let cid = cid.clone();
        posting.push(tokio::spawn(async move {
            for i in 1..=PER_PRODUCER {
                assert!(
                    responder
                        .post(
                            &cid,
                            segment::DATA,
                            Some(producer),
                            Some(&format!("{producer}-{i}"))
                        )
                        .await
                        .expect("post succeeds"),
                    "rendezvous stays live while producing"
                );
            }
        }));
    }
    for task in posting {
        task.await.expect("producer finished");
    }
    let closer = pod.responder().await;
    post_terminal(&closer, &cid, None, None).await;

    assert!(
        sink.await_closed(Duration::from_secs(5)).await,
        "terminal segment delivered"
    );
    assert_eq!(
        producers.len() * PER_PRODUCER + 1,
        sink.len(),
        "no loss, no duplication"
    );
    // the global interleave is arbitrary by design, but each producer's own
    // subsequence must keep its posting order (Redis per-connection command
    // ordering + serialized drains)
    let names = sink.names();
    let bodies = sink.bodies();
    for producer in producers {
        let subsequence: Vec<Option<String>> = names
            .iter()
            .zip(bodies.iter())
            .filter(|(name, _)| name.as_deref() == Some(producer))
            .map(|(_, body)| body.clone())
            .collect();
        let expected: Vec<Option<String>> = (1..=PER_PRODUCER)
            .map(|i| text(&format!("{producer}-{i}")))
            .collect();
        assert_eq!(
            expected, subsequence,
            "per-producer order preserved for {producer}"
        );
    }
    pod.await_route_gone(&cid).await;
    assert_eq!(0, pod.coordinator.active_streams());
}

#[tokio::test]
async fn any_producer_may_close_the_channel() {
    let pod = UiPod::start().await;
    let cid = new_cid();
    let sink = Arc::new(CollectingSink::default());
    pod.coordinator
        .begin_stream(&cid, sink.clone())
        .await
        .expect("stream opens");
    let notifier = pod.responder().await;
    let other = pod.responder().await;
    assert!(notifier
        .post(
            &cid,
            segment::DATA,
            Some("orders"),
            Some("order 42 shipped")
        )
        .await
        .expect("post succeeds"));
    assert!(notifier
        .post(
            &cid,
            segment::DATA,
            Some("orders"),
            Some("order 43 shipped")
        )
        .await
        .expect("post succeeds"));
    // a NON-originating producer posts the terminal entry - "the end signal is
    // also an event"
    post_terminal(&other, &cid, None, None).await;
    assert!(
        sink.await_closed(Duration::from_secs(5)).await,
        "close from another producer completes the stream"
    );
    pod.await_route_gone(&cid).await;
    // the still-active producer learns the rendezvous is over from its next post
    assert!(
        !notifier
            .post(
                &cid,
                segment::DATA,
                Some("orders"),
                Some("order 44 shipped")
            )
            .await
            .expect("post succeeds"),
        "route deletion stops the remaining producers"
    );
    assert_eq!(0, pod.coordinator.active_streams());
}

// ------------------------------------------------------------------
// Orphan and consumer-side close contracts
// ------------------------------------------------------------------

#[tokio::test]
async fn post_without_rendezvous_is_orphan() {
    let pod = UiPod::start().await;
    let cid = new_cid(); // nobody ever called begin_stream for it
    let responder = pod.responder().await;
    assert!(!responder
        .post(&cid, segment::DATA, None, Some("nobody is listening"))
        .await
        .expect("post succeeds"));
    // store-first is deliberate: the segment was appended before the route
    // check, and simply ages out
    let queue = ReturnRouteStore::queue_key(&cid);
    assert!(
        pod.key_exists(&queue),
        "orphan segment queued under its TTL"
    );
    assert!(
        pod.key_has_ttl(&queue),
        "orphan remnant carries a TTL from birth"
    );
}

#[tokio::test]
async fn consumer_side_close_stops_producers() {
    let pod = UiPod::start().await;
    let cid = new_cid();
    let sink = Arc::new(CollectingSink::default());
    pod.coordinator
        .begin_stream(&cid, sink.clone())
        .await
        .expect("stream opens");
    let responder = pod.responder().await;
    assert!(responder
        .post(&cid, segment::DATA, None, Some("before the client left"))
        .await
        .expect("post succeeds"));
    sink.await_first_segment().await;
    // client disconnect (or edge idle expiry): the facade closes from the
    // consumer side
    pod.coordinator.close_stream(&cid).await;
    assert_eq!(0, pod.coordinator.active_streams());
    pod.await_route_gone(&cid).await;
    assert!(
        !responder
            .post(&cid, segment::DATA, None, Some("after the client left"))
            .await
            .expect("post succeeds"),
        "producers stop on their next post"
    );
}

// ------------------------------------------------------------------
// Recovery cornerstones: missed wake-up, final drain at idle expiry
// ------------------------------------------------------------------

#[tokio::test]
async fn missed_wake_up_is_healed_by_the_next_drain() {
    let pod = UiPod::start().await;
    let cid = new_cid();
    let sink = Arc::new(CollectingSink::default());
    pod.coordinator
        .begin_stream(&cid, sink.clone())
        .await
        .expect("stream opens");
    let responder = pod.responder().await;
    // simulate a lost notification: a segment is stored (store-first!) but its
    // wake-up never arrives
    let silent = StreamSegment::of(segment::DATA, None, Some("the silent one")).expect("valid");
    responder
        .store()
        .append_segment(&cid, &silent.to_json(), 60)
        .await
        .expect("stored without a wake-up");
    assert_eq!(
        0,
        sink.len(),
        "no wake-up, no drain - the segment waits in the queue"
    );

    // the NEXT post's wake-up drains everything queued, in order - the dropped
    // signal costs latency only
    post_terminal(&responder, &cid, None, None).await;
    assert!(sink.await_closed(Duration::from_secs(5)).await);
    assert_eq!(
        vec![text("the silent one"), None],
        sink.bodies(),
        "healed drain delivered both, in order"
    );
}

#[tokio::test]
async fn final_drain_at_idle_expiry_recovers_a_dropped_close() {
    let pod = UiPod::start().await;
    let cid = new_cid();
    let sink = Arc::new(CollectingSink::default());
    pod.coordinator
        .begin_stream(&cid, sink.clone())
        .await
        .expect("stream opens");
    // every notification is lost, including the final one - the worst case the
    // final-drain nuance exists for
    let responder = pod.responder().await;
    let store = responder.store();
    let data = StreamSegment::of(segment::DATA, None, Some("last tokens")).expect("valid");
    let eof = StreamSegment::of(segment::EOF, None, Some("{\"done\":true}")).expect("valid");
    store
        .append_segment(&cid, &data.to_json(), 60)
        .await
        .expect("stored");
    store
        .append_segment(&cid, &eof.to_json(), 60)
        .await
        .expect("stored");

    // at edge idle expiry the facade does ONE last-chance drain before failing
    // the render in-band
    assert!(
        pod.coordinator.final_drain(&cid).await,
        "the single final drain completes the stream"
    );
    assert_eq!(
        vec![text("last tokens"), text("{\"done\":true}")],
        sink.bodies()
    );
    assert_eq!(
        0,
        pod.coordinator.active_streams(),
        "stream closed by its recovered terminal segment"
    );
    pod.await_route_gone(&cid).await;
}

#[tokio::test]
async fn final_drain_on_a_quiet_stream_finds_nothing_and_keeps_it_open() {
    let pod = UiPod::start().await;
    let cid = new_cid();
    let sink = Arc::new(CollectingSink::default());
    pod.coordinator
        .begin_stream(&cid, sink)
        .await
        .expect("stream opens");
    assert!(
        !pod.coordinator.final_drain(&cid).await,
        "nothing queued, stream not completed"
    );
    assert_eq!(
        1,
        pod.coordinator.active_streams(),
        "a quiet stream stays open - closing is the caller's decision"
    );
    pod.coordinator.close_stream(&cid).await;
    assert_eq!(0, pod.coordinator.active_streams());
}

// ------------------------------------------------------------------
// Robustness: duplicate wake-ups, capacity, sink failure
// ------------------------------------------------------------------

#[tokio::test]
async fn duplicate_wake_up_delivers_nothing_twice() {
    let pod = UiPod::start().await;
    let cid = new_cid();
    let sink = Arc::new(CollectingSink::default());
    pod.coordinator
        .begin_stream(&cid, sink.clone())
        .await
        .expect("stream opens");
    let responder = pod.responder().await;
    assert!(responder
        .post(&cid, segment::DATA, None, Some("once"))
        .await
        .expect("post succeeds"));
    sink.await_first_segment().await;
    // duplicate/spurious wake-ups pop nothing: destructive reads need no
    // consumer bookkeeping
    let channel = pod.coordinator.return_channel().to_string();
    responder
        .store()
        .publish(&channel, &cid)
        .await
        .expect("spurious wake-up");
    responder
        .store()
        .publish(&channel, &cid)
        .await
        .expect("spurious wake-up");
    // give the spurious drains time to (not) deliver
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_eq!(vec![text("once")], sink.bodies(), "no duplicate delivery");
    post_terminal(&responder, &cid, None, None).await;
    assert!(sink.await_closed(Duration::from_secs(5)).await);
}

#[tokio::test]
async fn stream_capacity_is_bounded_and_released_on_close() {
    let tiny_cap = SyncOverAsyncConfig::new("svc-return", 90, 30, 100, 1800, 1);
    let pod = UiPod::start_with(tiny_cap, "pod-CAP").await;
    pod.coordinator
        .begin_stream("cap-1", Arc::new(CollectingSink::default()))
        .await
        .expect("first stream fits");
    let rejected = pod
        .coordinator
        .begin_stream("cap-2", Arc::new(CollectingSink::default()))
        .await
        .expect_err("begin_stream rejects deterministically at capacity");
    assert_eq!(503, rejected.status());
    pod.coordinator.close_stream("cap-1").await;
    // slot released on close
    pod.coordinator
        .begin_stream("cap-3", Arc::new(CollectingSink::default()))
        .await
        .expect("the freed slot is usable");
    assert_eq!(1, pod.coordinator.active_streams());
}

#[tokio::test]
async fn sink_failure_closes_the_stream() {
    let pod = UiPod::start().await;
    let cid = new_cid();
    pod.coordinator
        .begin_stream(&cid, Arc::new(FailingSink))
        .await
        .expect("stream opens");
    let responder = pod.responder().await;
    assert!(responder
        .post(&cid, segment::DATA, None, Some("undeliverable"))
        .await
        .expect("post succeeds"));
    // the failed drain closes the stream and deletes the keys
    pod.await_route_gone(&cid).await;
    assert_eq!(
        0,
        pod.coordinator.active_streams(),
        "a broken consumer cannot leak its stream entry"
    );
    assert!(
        !responder
            .post(&cid, segment::DATA, None, Some("more"))
            .await
            .expect("post succeeds"),
        "producers stop (orphan)"
    );
}

// ------------------------------------------------------------------
// One mechanism - the one-shot response is the degenerate stream
// ------------------------------------------------------------------

#[tokio::test]
async fn stream_producer_can_complete_a_one_shot_request() {
    // begin() registers an ordinary one-shot request; a StreamResponder's
    // terminal post completes it - deliver() and a closing post are the same
    // act on the same store
    let pod = UiPod::start().await;
    let cid = new_cid();
    pod.coordinator
        .begin(&cid)
        .await
        .expect("request registered");
    let responder = pod.responder().await;
    post_terminal(&responder, &cid, None, Some("{\"result\":\"accepted\"}")).await;
    assert_eq!(
        "{\"result\":\"accepted\"}",
        pod.coordinator
            .await_response(&cid, 5000)
            .await
            .expect("response collected")
    );
    assert_eq!(0, pod.coordinator.pending_count());
}

#[tokio::test]
async fn one_shot_and_stream_share_the_return_channel() {
    // one subscription serves both patterns: a wake-up is checked against
    // streams first, then requests
    let pod = UiPod::start().await;
    let one_shot_cid = new_cid();
    let stream_cid = new_cid();
    let sink = Arc::new(CollectingSink::default());
    pod.coordinator
        .begin(&one_shot_cid)
        .await
        .expect("request registered");
    pod.coordinator
        .begin_stream(&stream_cid, sink.clone())
        .await
        .expect("stream opens");
    let responder = pod.responder().await;
    assert!(responder
        .post(&stream_cid, segment::DATA, None, Some("progress 50%"))
        .await
        .expect("post succeeds"));
    post_terminal(
        &responder,
        &one_shot_cid,
        None,
        Some("{\"status\":\"200\"}"),
    )
    .await;
    post_terminal(&responder, &stream_cid, None, None).await;

    assert_eq!(
        "{\"status\":\"200\"}",
        pod.coordinator
            .await_response(&one_shot_cid, 5000)
            .await
            .expect("response collected")
    );
    assert!(sink.await_closed(Duration::from_secs(5)).await);
    assert_eq!(vec![text("progress 50%"), None], sink.bodies());
    assert_eq!(0, pod.coordinator.pending_count());
    pod.await_route_gone(&stream_cid).await;
    assert_eq!(0, pod.coordinator.active_streams());
}
