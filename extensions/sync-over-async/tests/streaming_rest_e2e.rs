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

//! E2 of the streaming return route — single-process end-to-end, **no broker
//! anywhere** (Java `StreamingRestE2eTest` twin): both driving use cases run
//! behind real `stream: true` endpoints, through real RESP wire traffic
//! (against this suite's in-process double) and the real HTTP edge, and are
//! consumed progressively with the shipped SSE client — the full circle:
//!
//! ```text
//!   HTTP request -> facade interceptor (StreamBridge: begin_stream + EventStreamWriter)
//!     ... StreamResponder posts to Redis ... coordinator drains -> reply lane -> SSE out the edge
//!       -> async.http.request (Accept: text/event-stream + reply_to) -> collector envelopes
//! ```
//!
//! Scenarios: the chat case (ordered tokens + eof, exact order asserted
//! end-to-end), the notification case (cid announced to the UI, several
//! producers, backend-side close), and the two idle-expiry endings — the
//! final drain RECOVERING a fully-lost close (design D4's confirmed nuance),
//! and the in-band 408 when nothing was queued. The consumer-side close
//! (`close_stream`) is exercised by the idle-expiry path, which is also what
//! reclaims a disconnected client's stream.
//!
//! Each test boots an isolated pod — its own double, platform, coordinator,
//! and HTTP edge — because route workers belong to the runtime that registers
//! them (the platform-core per-test-runtime idiom).

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::automation::http_client::AsyncHttpClientService;
use platform_core::platform::FunctionOptions;
use platform_core::{
    automation, overrides, resources, AppConfigReader, AppError, ComposableFunction, EventEnvelope,
    Platform, PostOffice,
};
use redis_test_double::start_resp_double;
use sync_over_async::segment;
use sync_over_async::{
    RedisSettings, ReturnRouteCoordinator, ReturnRouteStore, StreamBridge, StreamResponder,
    StreamSegment, SyncOverAsyncConfig, CID,
};
use tokio::sync::Notify;

const REST_YAML: &str = r#"
rest:
  #
  # Streaming return route (E2): the facades are interceptor functions
  # addressed directly by stream:true endpoints (design D2 - no Event Script
  # composition on this path). The endpoint timeout is the edge idle allowance
  # for the FIRST event; the facade overrides the between-segment allowance
  # per request through StreamBridge (the writer's x-ttl head control).
  #
  - service: "chat.stream.facade"
    methods: ['POST']
    url: "/api/chat"
    timeout: 15s
    stream: true
  - service: "notify.stream.facade"
    methods: ['GET']
    url: "/api/notifications"
    timeout: 15s
    stream: true
"#;

const IDLE_HEADER: &str = "x-stream-idle-seconds";
const CID_EVENT: &str = "cid";
const MOCK_AI_BACKEND: &str = "mock.ai.backend";
const TOKENS: [&str; 5] = ["Streaming", " is", " composable", " by", " design"];
const EOT_METADATA: &str = "{\"tokens\":5}";

// ---------------------------------------------------------------------------
// fixtures: the two facades and the backend producer (Java mock package twins)
// ---------------------------------------------------------------------------

/// The "chat with an AI agent" facade (use case 2): an interceptor addressed
/// directly by the `stream: true` endpoint `POST /api/chat`. It opens the
/// streaming rendezvous through [`StreamBridge`] and fires the request leg —
/// here an event to the in-process backend; in production, any transport the
/// application likes.
struct ChatStreamFacade {
    platform: Platform,
    coordinator: Arc<ReturnRouteCoordinator>,
}

#[async_trait]
impl ComposableFunction for ChatStreamFacade {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let cid = match input.correlation_id() {
            Some(id) => id.to_string(),
            None => uuid::Uuid::new_v4().simple().to_string(),
        };
        StreamBridge::open(&self.coordinator, &self.platform, &input, &cid, 10).await?;
        // the request leg: hand the rendezvous cid to the backend service
        let po = PostOffice::new(&self.platform);
        po.send(
            EventEnvelope::new()
                .set_to(MOCK_AI_BACKEND)
                .set_header(CID, &cid),
        )
        .await?;
        Ok(EventEnvelope::new())
    }
}

/// The "event notification" facade (use case 1): the UI opens
/// `GET /api/notifications` as an SSE request, and this interceptor opens the
/// rendezvous and hands the session's cid back to the UI as the first SSE
/// event (`event: cid`) — the UI quotes it in its subsequent POSTs so backend
/// services know where to post. Long quiet stretches are normal on such a
/// channel, so the idle allowance is widened per request via the
/// `x-stream-idle-seconds` HTTP header (default 8s here; a real deployment
/// would use minutes).
struct NotificationStreamFacade {
    platform: Platform,
    coordinator: Arc<ReturnRouteCoordinator>,
}

#[async_trait]
impl ComposableFunction for NotificationStreamFacade {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let http = automation::AsyncHttpRequest::from_value(input.body());
        let idle_seconds = http
            .header(IDLE_HEADER)
            .and_then(|value| value.trim().parse::<u64>().ok())
            .unwrap_or(8);
        let cid = match input.correlation_id() {
            Some(id) => id.to_string(),
            None => uuid::Uuid::new_v4().simple().to_string(),
        };
        let writer = StreamBridge::open(
            &self.coordinator,
            &self.platform,
            &input,
            &cid,
            idle_seconds,
        )
        .await?;
        // announce the session's cid before any producer can know it - after
        // this, the serialized drain is the only writer
        writer.write_named(CID_EVENT, &cid).await?;
        Ok(EventEnvelope::new())
    }
}

/// Stand-in for a backend service talking point-to-point to an AI agent: on
/// request (a bare event carrying the rendezvous cid), it posts a fixed
/// sequence of token segments plus the end-of-transmission signal —
/// sequentially, over one connection, which per design D7 is all the ordering
/// contract there is. It talks to Redis only, through the [`StreamResponder`]
/// producer API — no coordinator, no broker.
struct MockAiBackend {
    responder: StreamResponder,
}

#[async_trait]
impl ComposableFunction for MockAiBackend {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let cid = headers.get(CID).cloned().unwrap_or_default();
        for token in TOKENS {
            if !self
                .responder
                .post(&cid, segment::DATA, None, Some(token))
                .await?
            {
                return Ok(EventEnvelope::new()); // orphan - stop producing
            }
        }
        // a closing post races its own consumption by construction (port spec
        // §3 item 8), so its answer is not read - the producer is done either way
        self.responder
            .post(&cid, segment::EOF, None, Some(EOT_METADATA))
            .await?;
        Ok(EventEnvelope::new())
    }
}

// ---------------------------------------------------------------------------
// plumbing: an isolated pod per test, and the SSE-consumer collector
// ---------------------------------------------------------------------------

/// One received reply envelope: the `x-event-stream` protocol headers plus
/// the body text.
#[derive(Clone)]
struct Frame {
    marker: Option<String>,
    name: Option<String>,
    body: String,
}

/// Collects the SSE consumer's relayed envelopes and signals on the terminal
/// one (Java `SseCollector` twin).
#[derive(Default)]
struct SseCollector {
    frames: Mutex<Vec<Frame>>,
    ended: Notify,
    is_ended: AtomicBool,
}

#[async_trait]
impl ComposableFunction for SseCollector {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let frame = Frame {
            marker: headers.get("x-event-stream").cloned(),
            name: headers.get("x-event-name").cloned(),
            body: input.body_as::<String>().unwrap_or_default(),
        };
        let terminal = matches!(
            frame.marker.as_deref(),
            Some(segment::EOF | segment::EXCEPTION)
        );
        self.frames.lock().expect("frames poisoned").push(frame);
        if terminal {
            self.is_ended.store(true, Ordering::Release);
            self.ended.notify_waiters();
        }
        Ok(EventEnvelope::new())
    }
}

impl SseCollector {
    fn frames(&self) -> Vec<Frame> {
        self.frames.lock().expect("frames poisoned").clone()
    }

    /// Bodies of unnamed data frames, in arrival order.
    fn data_bodies(&self) -> Vec<String> {
        self.frames()
            .into_iter()
            .filter(|f| f.marker.as_deref() == Some(segment::DATA) && f.name.is_none())
            .map(|f| f.body)
            .collect()
    }

    /// Data frames carrying this SSE event name, in arrival order.
    fn named(&self, event_name: &str) -> Vec<Frame> {
        self.frames()
            .into_iter()
            .filter(|f| {
                f.marker.as_deref() == Some(segment::DATA) && f.name.as_deref() == Some(event_name)
            })
            .collect()
    }

    fn ended_with_eof(&self) -> bool {
        self.frames()
            .iter()
            .any(|f| f.marker.as_deref() == Some(segment::EOF))
    }

    async fn await_ended(&self, duration: Duration) -> bool {
        let deadline = tokio::time::Instant::now() + duration;
        loop {
            let ended = self.ended.notified();
            tokio::pin!(ended);
            if self.is_ended.load(Ordering::Acquire) {
                return true;
            }
            let now = tokio::time::Instant::now();
            if now >= deadline {
                return false;
            }
            let _ = tokio::time::timeout(deadline - now, &mut ended).await;
        }
    }

    /// Poll for the notification facade's cid announcement (the first SSE
    /// event of the channel).
    async fn await_announced_cid(&self) -> String {
        for _ in 0..500 {
            if let Some(first) = self.named(CID_EVENT).first() {
                return first.body.clone();
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        panic!("the notification facade did not announce the session cid in time");
    }
}

/// One isolated UI pod: its own double, platform, coordinator, and HTTP edge.
struct UiPod {
    platform: Platform,
    coordinator: Arc<ReturnRouteCoordinator>,
    settings: RedisSettings,
    host: String,
}

impl UiPod {
    async fn boot() -> UiPod {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            resources::prepend_resource_root("tests/resources");
            let rest_file =
                std::env::temp_dir().join(format!("rest-soa-e2e-{}.yaml", std::process::id()));
            std::fs::write(&rest_file, REST_YAML).expect("write rest.yaml");
            overrides::set(
                "yaml.rest.automation",
                &format!("file:{}", rest_file.display()),
            );
            overrides::set("rest.server.port", "0");
            let _ = AppConfigReader::get_instance();
        });
        let (redis_port, _store, _journal) = start_resp_double("7.4.0").await;
        let settings = RedisSettings::new("127.0.0.1", redis_port, "", false, 0, 5000);
        let origin = format!("ui-pod-{}", uuid::Uuid::new_v4().simple());
        let coordinator = Arc::new(
            ReturnRouteCoordinator::connect(&settings, &origin, SyncOverAsyncConfig::default())
                .await
                .expect("coordinator connects"),
        );
        coordinator.start().await.expect("subscriber starts");
        let platform = Platform::new();
        let interceptor = FunctionOptions {
            zero_traced: false,
            interceptor: true,
            private: true,
        };
        platform
            .register_with_options(
                "chat.stream.facade",
                Arc::new(ChatStreamFacade {
                    platform: platform.clone(),
                    coordinator: coordinator.clone(),
                }),
                10,
                interceptor,
            )
            .expect("register chat facade");
        platform
            .register_with_options(
                "notify.stream.facade",
                Arc::new(NotificationStreamFacade {
                    platform: platform.clone(),
                    coordinator: coordinator.clone(),
                }),
                10,
                interceptor,
            )
            .expect("register notify facade");
        platform
            .register(
                MOCK_AI_BACKEND,
                Arc::new(MockAiBackend {
                    responder: StreamResponder::connect(&settings)
                        .await
                        .expect("responder connects"),
                }),
                10,
            )
            .expect("register backend");
        platform
            .register_with_options(
                automation::ASYNC_HTTP_REQUEST,
                Arc::new(AsyncHttpClientService::new(&platform)),
                10,
                interceptor,
            )
            .expect("register http client");
        let addr = automation::start_http_server(&platform)
            .await
            .expect("http server");
        UiPod {
            platform,
            coordinator,
            settings,
            host: format!("http://127.0.0.1:{}", addr.port()),
        }
    }

    /// Open a streaming HTTP call through the shipped SSE consumer, relaying
    /// to a fresh collector.
    async fn open_sse(
        &self,
        collector_route: &str,
        request: automation::AsyncHttpRequest,
    ) -> Arc<SseCollector> {
        let collector = Arc::new(SseCollector::default());
        self.platform
            .register_with_options(
                collector_route,
                collector.clone(),
                1,
                FunctionOptions {
                    zero_traced: false,
                    interceptor: true,
                    private: true,
                },
            )
            .expect("register collector");
        let po = PostOffice::new(&self.platform);
        po.send(
            EventEnvelope::new()
                .set_to(automation::ASYNC_HTTP_REQUEST)
                .set_raw_body(request.to_value())
                .set_reply_to(collector_route)
                .set_correlation_id(&uuid::Uuid::new_v4().simple().to_string()),
        )
        .await
        .expect("dispatch sse request");
        collector
    }

    fn sse_request(&self, method: &str, url: &str) -> automation::AsyncHttpRequest {
        automation::AsyncHttpRequest::new()
            .set_method(method)
            .set_target_host(&self.host)
            .set_url(url)
            .set_header("accept", "text/event-stream")
            .set_timeout_seconds(20) // SSE consumption: the idle allowance between reads
    }

    async fn store(&self) -> ReturnRouteStore {
        ReturnRouteStore::connect(&self.settings)
            .await
            .expect("store connects")
    }

    /// Poll until the route key is gone — the rendezvous is fully closed.
    async fn await_route_gone(&self, cid: &str) {
        let store = self.store().await;
        for _ in 0..500 {
            if store.get_route(cid).await.expect("route lookup").is_none() {
                return;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        panic!("route key for {cid} was not cleaned up in time");
    }
}

/// Post a **terminal** segment without asserting liveness: a closing post
/// races its own consumption by construction (port spec §3 item 8) — an
/// in-flight drain woken by an earlier post may pop it, deliver it, and close
/// the rendezvous before the producer's own route check runs, so `post` may
/// answer `false` although the segment WAS delivered. The scenarios assert on
/// what actually matters: the collector saw the terminal event, and the
/// rendezvous closed. Liveness is asserted on DATA posts only.
async fn post_terminal(responder: &StreamResponder, cid: &str) {
    responder
        .post(cid, segment::EOF, None, None)
        .await
        .expect("post succeeds");
}

// ---------------------------------------------------------------------------
// use case: chat with an AI agent - strict order, single sequential producer
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn chat_tokens_render_progressively_in_exact_order() {
    let pod = UiPod::boot().await;
    let collector = pod
        .open_sse("chat.collector", pod.sse_request("POST", "/api/chat"))
        .await;

    assert!(
        collector.await_ended(Duration::from_secs(15)).await,
        "the stream should end with the terminal event"
    );
    // no sequence number anywhere in the pipeline, yet order holds end-to-end:
    // producer connection order -> list order -> serialized drain -> ordered
    // reply lane -> SSE -> consumer envelopes
    assert_eq!(
        TOKENS.to_vec(),
        collector.data_bodies(),
        "token order preserved end-to-end"
    );
    let done = collector.named("done");
    assert_eq!(
        1,
        done.len(),
        "close metadata renders as the terminal SSE done event"
    );
    assert_eq!(
        EOT_METADATA, done[0].body,
        "eof metadata rides the done event"
    );
    assert!(
        collector.ended_with_eof(),
        "a clean upstream end relays as eof"
    );
    assert!(
        collector.frames().len() >= TOKENS.len() + 2,
        "segments arrived multi-shot (progressive), not as one buffered response"
    );
    assert_eq!(
        0,
        pod.coordinator.active_streams(),
        "rendezvous closed by its terminal segment"
    );
}

// ---------------------------------------------------------------------------
// use case: event notification - several producers, backend-side close
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn notification_channel_serves_several_producers_and_backend_close() {
    let pod = UiPod::boot().await;
    let collector = pod
        .open_sse(
            "notify.collector",
            pod.sse_request("GET", "/api/notifications"),
        )
        .await;
    let cid = collector.await_announced_cid().await;

    // two backend services, each with its own responder (its own connection) -
    // as separate pods would be
    let orders = StreamResponder::connect(&pod.settings)
        .await
        .expect("orders responder");
    let billing = StreamResponder::connect(&pod.settings)
        .await
        .expect("billing responder");
    assert!(orders
        .post(
            &cid,
            segment::DATA,
            Some("orders"),
            Some("order 42 shipped")
        )
        .await
        .expect("post succeeds"));
    assert!(billing
        .post(
            &cid,
            segment::DATA,
            Some("billing"),
            Some("invoice 7 ready")
        )
        .await
        .expect("post succeeds"));
    // a backend service - not the UI - ends the channel: any producer may post
    // the terminal (liveness deliberately not asserted - see post_terminal)
    post_terminal(&billing, &cid).await;

    assert!(
        collector.await_ended(Duration::from_secs(15)).await,
        "backend-side close ends the SSE render"
    );
    assert_eq!("order 42 shipped", collector.named("orders")[0].body);
    assert_eq!("invoice 7 ready", collector.named("billing")[0].body);
    assert!(collector.ended_with_eof());

    pod.await_route_gone(&cid).await;
    assert!(
        !orders
            .post(
                &cid,
                segment::DATA,
                Some("orders"),
                Some("order 43 shipped")
            )
            .await
            .expect("post succeeds"),
        "the other producer stops on its next post (orphan)"
    );
    assert_eq!(0, pod.coordinator.active_streams());
}

// ---------------------------------------------------------------------------
// idle expiry, ending 1: the single final drain RECOVERS a fully-lost close
// (design D4's nuance)
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn idle_expiry_final_drain_recovers_a_lost_close() {
    let pod = UiPod::boot().await;
    let collector = pod
        .open_sse(
            "recover.collector",
            pod.sse_request("GET", "/api/notifications")
                .set_header(IDLE_HEADER, "2"),
        )
        .await;
    let cid = collector.await_announced_cid().await;

    // worst case: segments are stored (store-first!) but EVERY wake-up is lost
    // - no publish at all
    let direct = pod.store().await;
    direct
        .append_segment(
            &cid,
            &StreamSegment::of(segment::DATA, Some("orders"), Some("the lost one"))
                .expect("segment")
                .to_json(),
            60,
        )
        .await
        .expect("append data");
    direct
        .append_segment(
            &cid,
            &StreamSegment::of(segment::EOF, None, Some("{\"recovered\":true}"))
                .expect("segment")
                .to_json(),
            60,
        )
        .await
        .expect("append eof");

    // nothing wakes the pod; at idle expiry the watchdog performs ONE final
    // drain - and the render completes successfully despite the lost
    // notifications
    assert!(
        collector.await_ended(Duration::from_secs(15)).await,
        "final drain completes the render"
    );
    assert_eq!(
        "the lost one",
        collector.named("orders")[0].body,
        "queued segment recovered"
    );
    let done = collector.named("done");
    assert_eq!(
        1,
        done.len(),
        "the recovered eof closes the stream normally"
    );
    assert_eq!("{\"recovered\":true}", done[0].body);
    assert!(collector.ended_with_eof());
    pod.await_route_gone(&cid).await;
    assert_eq!(0, pod.coordinator.active_streams());
}

// ---------------------------------------------------------------------------
// idle expiry, ending 2: nothing queued - fail in-band (408) and close the
// rendezvous
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn idle_expiry_with_nothing_queued_fails_in_band_and_stops_producers() {
    let pod = UiPod::boot().await;
    let collector = pod
        .open_sse(
            "idle.collector",
            pod.sse_request("GET", "/api/notifications")
                .set_header(IDLE_HEADER, "2"),
        )
        .await;
    let cid = collector.await_announced_cid().await;

    // no producer ever posts; the watchdog's final drain finds nothing and
    // fails the render in-band
    assert!(
        collector.await_ended(Duration::from_secs(15)).await,
        "idle expiry ends the SSE render"
    );
    let error = collector.named("error");
    assert_eq!(
        1,
        error.len(),
        "the in-band failure renders as the SSE error event"
    );
    assert!(
        error[0].body.contains("408") && error[0].body.contains("Stream idle timeout"),
        "the error event carries the 408 timeout, got: {}",
        error[0].body
    );

    // the consumer-side close deleted the route, so a late producer stops
    // immediately
    pod.await_route_gone(&cid).await;
    let late = StreamResponder::connect(&pod.settings)
        .await
        .expect("late responder");
    assert!(
        !late
            .post(&cid, segment::DATA, Some("orders"), Some("too late"))
            .await
            .expect("post succeeds"),
        "producers learn the rendezvous is over from their next post"
    );
    assert_eq!(0, pod.coordinator.active_streams());
}
