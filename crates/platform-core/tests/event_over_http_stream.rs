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

//! Event-over-HTTP peer streaming (envelope mode): a send with reply_to that
//! declares the "accept: text/event-stream" event header relays a remote
//! streaming function's segments progressively to the caller's reply route.
//! The wire is the hybrid dialect - envelope frames (base64 MsgPack) for the
//! head, the terminals and non-text segments; raw SSE frames for text tokens.
//! A non-streaming target and every existing calling mode stay byte-identical.
//! (Java `EventOverHttpStreamTest` twin.)

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use base64::Engine as _;
use platform_core::automation::http_client::AsyncHttpClientService;
use platform_core::automation::{self, EventApiService};
use platform_core::platform::FunctionOptions;
use platform_core::{
    event_stream, overrides, resources, AppConfigReader, AppError, ComposableFunction,
    EventEnvelope, EventStreamWriter, Platform, PostOffice,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const STREAMING_TARGET: &str = "hello.stream.remote";
const SINGLE_SHOT_TARGET: &str = "hello.single.remote";
const TEXT_EVENT_STREAM: &str = "text/event-stream";

/// A PUBLIC streaming target reached through /api/event - modes are selected
/// by the "mode" event header (a plain function, not an edge endpoint).
struct RemoteStreamProducer {
    platform: Platform,
}

#[async_trait]
impl ComposableFunction for RemoteStreamProducer {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let mode = headers.get("mode").map(String::as_str).unwrap_or("tokens");
        let mut out = EventStreamWriter::from_request(&self.platform, &input)?;
        match mode {
            "tokens" => {
                out.first(200, TEXT_EVENT_STREAM);
                out.write("alpha").await?;
                tokio::time::sleep(Duration::from_millis(250)).await;
                out.write("beta").await?;
                tokio::time::sleep(Duration::from_millis(250)).await;
                out.close_with(serde_json::json!({"segments": 2})).await?;
            }
            "typed" => {
                // every escape-hatch trigger: a map body, text with a carriage
                // return, a user event name colliding with the reserved word,
                // a binary body - plus one plain token that rides a raw frame
                out.first(200, TEXT_EVENT_STREAM);
                out.write(serde_json::json!({"n": 1})).await?;
                out.write_named("crlf", "line1\r\nline2").await?;
                out.write_named("envelope", "reserved-name").await?;
                let po = PostOffice::new(&self.platform);
                po.send(
                    EventEnvelope::new()
                        .set_to(input.reply_to().unwrap_or_default())
                        .set_correlation_id(input.correlation_id().unwrap_or_default())
                        .set_header(event_stream::X_EVENT_STREAM, event_stream::DATA)
                        .set_raw_body(rmpv::Value::Binary(vec![1, 2, 3, 4])),
                )
                .await?;
                out.write("plain token").await?;
                out.close_with(serde_json::json!({"done": true})).await?;
            }
            "error-mid" => {
                out.first(200, TEXT_EVENT_STREAM);
                out.write("partial").await?;
                out.fail(&AppError::new(503, "backend on fire")).await?;
            }
            "error-first" => {
                out.fail(&AppError::new(503, "no backend")).await?;
            }
            "stall" => {
                // one-second declared idle allowance, then silence - the edge
                // renderer or the consuming client must fail the stream in-band
                out.first_with_ttl(200, TEXT_EVENT_STREAM, 1);
                out.write("one").await?;
            }
            other => {
                out.fail(&AppError::new(400, format!("unknown mode {other}")))
                    .await?;
            }
        }
        Ok(EventEnvelope::new())
    }
}

/// A PUBLIC single-shot echo (the byte-identical fallback pins).
struct SingleShotEcho;

#[async_trait]
impl ComposableFunction for SingleShotEcho {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new().set_body(serde_json::json!({
            "echo": format!("{}", input.body()),
        }))
    }
}

/// The engine-to-engine composition: a streaming edge endpoint whose function
/// forwards its own reply lane and correlation id into a send to the
/// event-over-http MAPPED streaming function, opting in with the accept
/// event header - the remote segments re-render progressively out this edge.
struct RemoteRelayFixture {
    platform: Platform,
}

#[async_trait]
impl ComposableFunction for RemoteRelayFixture {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request = input.body_as::<serde_json::Value>().unwrap_or_default();
        let mode = request["parameters"]["query"]["mode"]
            .as_str()
            .unwrap_or("tokens")
            .to_string();
        let po = PostOffice::new(&self.platform);
        po.send(
            EventEnvelope::new()
                .set_to(STREAMING_TARGET)
                .set_reply_to(input.reply_to().unwrap_or_default())
                .set_correlation_id(input.correlation_id().unwrap_or_default())
                .set_header("accept", TEXT_EVENT_STREAM)
                .set_header("x-ttl", "10000")
                .set_header("mode", &mode),
        )
        .await?;
        Ok(EventEnvelope::new())
    }
}

/// Captures every envelope delivered to its route.
struct CaptureRoute {
    received: Arc<Mutex<Vec<EventEnvelope>>>,
}

#[async_trait]
impl ComposableFunction for CaptureRoute {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        self.received.lock().expect("capture mutex").push(input);
        Ok(EventEnvelope::new())
    }
}

/// The rest.yaml of the shared fixture: the /api/event entry plus the
/// streaming edge endpoint of the engine-to-engine composition.
const REST_YAML: &str = r#"
rest:
  - service: "event.api.service"
    methods: ['POST']
    url: "/api/event"
    timeout: 60s

  - service: "hello.remote.relay"
    methods: ['GET']
    url: "/api/hello/remote"
    timeout: 15s
    stream: true
"#;

/// The shared fixture: ONE platform + edge server + misbehaving-peer mock on a
/// dedicated thread with its own runtime, because the declarative
/// event-over-http registry is a process-wide one-shot (route workers would
/// otherwise die with the first test's runtime - the per-test-runtime lesson).
/// The event-over-http map is written at runtime with the REAL ports.
fn shared() -> (u16, Platform) {
    static SHARED: OnceLock<(u16, Platform)> = OnceLock::new();
    SHARED
        .get_or_init(|| {
            let (tx, rx) = std::sync::mpsc::channel::<(u16, Platform)>();
            std::thread::spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("fixture runtime");
                runtime.block_on(async move {
                    resources::prepend_resource_root("tests/resources");
                    let pid = std::process::id();
                    let rest_file =
                        std::env::temp_dir().join(format!("rest-eoh-stream-{pid}.yaml"));
                    std::fs::write(&rest_file, REST_YAML).expect("write rest.yaml");
                    overrides::set(
                        "yaml.rest.automation",
                        &format!("file:{}", rest_file.display()),
                    );
                    overrides::set("rest.server.port", "0");
                    let _ = AppConfigReader::get_instance();
                    let platform = Platform::new();
                    let interceptor = FunctionOptions {
                        zero_traced: false,
                        interceptor: true,
                        private: true,
                    };
                    let public_interceptor = FunctionOptions {
                        zero_traced: false,
                        interceptor: true,
                        private: false,
                    };
                    platform
                        .register_with_options(
                            automation::EVENT_API_SERVICE,
                            Arc::new(EventApiService::new(&platform)),
                            10,
                            interceptor,
                        )
                        .expect("register event api");
                    platform
                        .register_with_options(
                            automation::ASYNC_HTTP_REQUEST,
                            Arc::new(AsyncHttpClientService::new(&platform)),
                            10,
                            interceptor,
                        )
                        .expect("register http client");
                    platform
                        .register_with_options(
                            STREAMING_TARGET,
                            Arc::new(RemoteStreamProducer {
                                platform: platform.clone(),
                            }),
                            10,
                            public_interceptor,
                        )
                        .expect("register streaming target");
                    platform
                        .register(SINGLE_SHOT_TARGET, Arc::new(SingleShotEcho), 5)
                        .expect("register single-shot echo");
                    platform
                        .register_with_options(
                            "hello.remote.relay",
                            Arc::new(RemoteRelayFixture {
                                platform: platform.clone(),
                            }),
                            10,
                            interceptor,
                        )
                        .expect("register remote relay");
                    let addr = automation::start_http_server(&platform)
                        .await
                        .expect("http server");
                    let edge_port = addr.port();
                    let mock_port = start_misbehaving_peer().await;
                    let map_file = std::env::temp_dir().join(format!("eoh-map-{pid}.yaml"));
                    let map = format!(
                        r#"
event.http:
  - route: '{STREAMING_TARGET}'
    target: 'http://127.0.0.1:{edge_port}/api/event'
  - route: '{SINGLE_SHOT_TARGET}'
    target: 'http://127.0.0.1:{edge_port}/api/event'
  # an edge-level REST error on the relay path (POST to a GET-only endpoint)
  - route: 'mock.rest.error'
    target: 'http://127.0.0.1:{edge_port}/api/hello/remote'
  # conformance guards against a misbehaving peer
  - route: 'mock.sse.raw.first'
    target: 'http://127.0.0.1:{mock_port}/mock/raw-first'
  - route: 'mock.sse.no.terminal'
    target: 'http://127.0.0.1:{mock_port}/mock/no-terminal'
  - route: 'mock.sse.foreign'
    target: 'http://127.0.0.1:{mock_port}/mock/foreign-dialect'
"#
                    );
                    std::fs::write(&map_file, map).expect("write event-over-http map");
                    overrides::set(
                        "yaml.event.over.http",
                        &format!("file:{}", map_file.display()),
                    );
                    tx.send((edge_port, platform.clone())).expect("announce");
                    std::future::pending::<()>().await;
                });
            });
            rx.recv().expect("shared fixture")
        })
        .clone()
}

/// A misbehaving-peer mock: it violates the envelope dialect in the exact ways
/// the consuming client must catch. It lives on the fixture runtime, which
/// never ends.
async fn start_misbehaving_peer() -> u16 {
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .expect("mock peer bind");
    let port = listener.local_addr().expect("mock addr").port();
    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                break;
            };
            tokio::spawn(async move {
                let mut buf = vec![0u8; 8192];
                let mut head = Vec::new();
                loop {
                    let Ok(n) = socket.read(&mut buf).await else {
                        return;
                    };
                    if n == 0 {
                        return;
                    }
                    head.extend_from_slice(&buf[..n]);
                    if head.windows(4).any(|w| w == b"\r\n\r\n") {
                        break;
                    }
                }
                let request = String::from_utf8_lossy(&head);
                let path = request.split_whitespace().nth(1).unwrap_or("/").to_string();
                // the peer may still be sending the POST body - it is ignored
                let head = "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\ntransfer-encoding: chunked\r\n\r\n";
                if socket.write_all(head.as_bytes()).await.is_err() {
                    return;
                }
                let chunk = |text: &str| format!("{:x}\r\n{}\r\n", text.len(), text);
                match path.as_str() {
                    "/mock/raw-first" => {
                        // the dialect guarantees an envelope frame first
                        let _ = socket.write_all(chunk("data: hello\n\n").as_bytes()).await;
                        let _ = socket.write_all(b"0\r\n\r\n").await;
                    }
                    "/mock/no-terminal" => {
                        // a clean end without a decoded terminal is a truncation
                        let _ = socket.write_all(chunk(&mock_head_frame()).as_bytes()).await;
                        let _ = socket.write_all(b"0\r\n\r\n").await;
                    }
                    "/mock/foreign-dialect" => {
                        // a conforming foreign peer: envelope head, raw token,
                        // envelope eof - plus a trailing frame that must be
                        // discarded after the terminal
                        let frames = format!(
                            "{}data: mock-token\n\n{}data: trailing-noise\n\n",
                            mock_head_frame(),
                            mock_eof_frame()
                        );
                        let _ = socket.write_all(chunk(&frames).as_bytes()).await;
                        let _ = socket.write_all(b"0\r\n\r\n").await;
                    }
                    _ => {
                        let _ = socket.write_all(b"0\r\n\r\n").await;
                    }
                }
            });
        }
    });
    port
}

fn envelope_frame(event: EventEnvelope) -> String {
    let bytes = event.to_bytes().expect("serialize mock envelope");
    format!(
        "event: envelope\ndata: {}\n\n",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    )
}

fn mock_head_frame() -> String {
    envelope_frame(
        EventEnvelope::new()
            .set_header(event_stream::X_EVENT_STREAM, event_stream::DATA)
            .set_header("content-type", TEXT_EVENT_STREAM)
            .set_status(200)
            .set_body("mock-head")
            .expect("mock head"),
    )
}

fn mock_eof_frame() -> String {
    envelope_frame(
        EventEnvelope::new()
            .set_header(event_stream::X_EVENT_STREAM, event_stream::EOF)
            .set_body(serde_json::json!({"done": true}))
            .expect("mock eof"),
    )
}

/// One global test guard: the pool-drain test must not race sibling tests.
async fn suite_guard() -> tokio::sync::MutexGuard<'static, ()> {
    static GUARD: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    GUARD
        .get_or_init(|| tokio::sync::Mutex::new(()))
        .lock()
        .await
}

fn next_seq() -> usize {
    static SEQ: AtomicUsize = AtomicUsize::new(0);
    SEQ.fetch_add(1, Ordering::Relaxed) + 1
}

fn marker(event: &EventEnvelope) -> Option<String> {
    event
        .headers()
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(event_stream::X_EVENT_STREAM))
        .map(|(_, value)| value.to_lowercase())
}

fn header<'a>(event: &'a EventEnvelope, name: &str) -> Option<&'a str> {
    event
        .headers()
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(name))
        .map(|(_, value)| value.as_str())
}

fn is_terminal(event: &EventEnvelope) -> bool {
    matches!(
        marker(event).as_deref(),
        Some(event_stream::EOF) | Some(event_stream::EXCEPTION)
    )
}

/// Send to an event-over-http mapped route with the streaming opt-in and a
/// capture route as reply_to; return the captured envelopes up to and
/// including the first terminal marker - or the first envelope for
/// single-shot pins.
async fn send_streaming(
    platform: &Platform,
    route: &str,
    mode: Option<&str>,
    ttl_ms: Option<&str>,
    expected: usize,
) -> Vec<EventEnvelope> {
    let received = Arc::new(Mutex::new(Vec::new()));
    let seq = next_seq();
    let capture = format!("capture.eoh.stream.{seq}");
    platform
        .register_with_options(
            &capture,
            Arc::new(CaptureRoute {
                received: received.clone(),
            }),
            1,
            FunctionOptions {
                zero_traced: false,
                interceptor: true,
                private: true,
            },
        )
        .expect("register capture");
    let mut event = EventEnvelope::new()
        .set_to(route)
        .set_reply_to(&capture)
        .set_correlation_id(&format!("cid-eoh-{seq}"))
        .set_header("accept", TEXT_EVENT_STREAM);
    if let Some(mode) = mode {
        event = event.set_header("mode", mode);
    }
    if let Some(ttl) = ttl_ms {
        event = event.set_header("x-ttl", ttl);
    }
    let po = PostOffice::new(platform);
    po.send(event).await.expect("send");
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        {
            let events = received.lock().expect("capture mutex");
            let complete = events.len() >= expected || events.iter().any(is_terminal);
            if complete {
                let mut trimmed = Vec::new();
                for event in events.iter() {
                    trimmed.push(event.clone());
                    if is_terminal(event) {
                        break;
                    }
                }
                return trimmed;
            }
        }
        assert!(
            Instant::now() < deadline,
            "expected {expected} events from {route}"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

#[tokio::test]
async fn streaming_target_relays_progressively_to_callback() {
    let _guard = suite_guard().await;
    let (_, platform) = shared();
    let events = send_streaming(&platform, STREAMING_TARGET, Some("tokens"), None, 3).await;
    assert_eq!(3, events.len(), "2 data envelopes + eof");
    // the decoded head is the target's first envelope, verbatim
    let head = &events[0];
    assert_eq!(Some(event_stream::DATA.to_string()), marker(head));
    assert_eq!(200, head.status());
    assert_eq!(Some(TEXT_EVENT_STREAM), header(head, "content-type"));
    assert_eq!("alpha", head.body_as::<String>().expect("text body"));
    assert!(
        head.correlation_id()
            .unwrap_or_default()
            .starts_with("cid-eoh-"),
        "original correlation id restored"
    );
    // the second token rode a raw frame and was synthesized back
    let token = &events[1];
    assert_eq!(Some(event_stream::DATA.to_string()), marker(token));
    assert_eq!("beta", token.body_as::<String>().expect("text body"));
    // eof carries the trailing metadata with its exact map type
    let eof = &events[2];
    assert_eq!(Some(event_stream::EOF.to_string()), marker(eof));
    let metadata = eof.body_as::<serde_json::Value>().expect("eof metadata");
    assert_eq!(2, metadata["segments"].as_i64().unwrap_or_default());
}

#[tokio::test]
async fn typed_segments_round_trip_exactly() {
    let _guard = suite_guard().await;
    let (_, platform) = shared();
    let events = send_streaming(&platform, STREAMING_TARGET, Some("typed"), None, 6).await;
    assert_eq!(6, events.len(), "5 data envelopes + eof");
    let map_segment = events[0].body_as::<serde_json::Value>().expect("map body");
    assert_eq!(1, map_segment["n"].as_i64().unwrap_or_default());
    let crlf = &events[1];
    assert_eq!(Some("crlf"), header(crlf, event_stream::X_EVENT_NAME));
    assert_eq!(
        "line1\r\nline2",
        crlf.body_as::<String>().expect("text body"),
        "carriage return preserved"
    );
    let reserved = &events[2];
    assert_eq!(
        Some("envelope"),
        header(reserved, event_stream::X_EVENT_NAME),
        "a user event name colliding with the reserved word survives"
    );
    assert_eq!(
        "reserved-name",
        reserved.body_as::<String>().expect("text body")
    );
    let bytes = &events[3];
    assert_eq!(
        &rmpv::Value::Binary(vec![1, 2, 3, 4]),
        bytes.body(),
        "binary body preserved"
    );
    assert_eq!(
        "plain token",
        events[4].body_as::<String>().expect("text body")
    );
    let eof = &events[5];
    assert_eq!(Some(event_stream::EOF.to_string()), marker(eof));
    let metadata = eof.body_as::<serde_json::Value>().expect("eof metadata");
    assert_eq!(Some(true), metadata["done"].as_bool());
}

#[tokio::test]
async fn single_shot_target_over_capable_path_is_classic() {
    let _guard = suite_guard().await;
    let (_, platform) = shared();
    // classic callback (no accept header) as the baseline
    let received = Arc::new(Mutex::new(Vec::new()));
    let capture = format!("capture.eoh.single.{}", next_seq());
    platform
        .register_with_options(
            &capture,
            Arc::new(CaptureRoute {
                received: received.clone(),
            }),
            1,
            FunctionOptions {
                zero_traced: false,
                interceptor: true,
                private: true,
            },
        )
        .expect("register capture");
    let po = PostOffice::new(&platform);
    po.send(
        EventEnvelope::new()
            .set_to(SINGLE_SHOT_TARGET)
            .set_reply_to(&capture)
            .set_correlation_id("cid-classic")
            .set_body("ping-1")
            .expect("body"),
    )
    .await
    .expect("classic send");
    let classic = wait_for_one(&received, 0).await;
    // streaming-capable call to the same target
    po.send(
        EventEnvelope::new()
            .set_to(SINGLE_SHOT_TARGET)
            .set_reply_to(&capture)
            .set_correlation_id("cid-capable")
            .set_header("accept", TEXT_EVENT_STREAM)
            .set_body("ping-1")
            .expect("body"),
    )
    .await
    .expect("capable send");
    let capable = wait_for_one(&received, 1).await;
    assert!(
        marker(&capable).is_none(),
        "a single-shot reply carries no stream marker"
    );
    assert_eq!(classic.status(), capable.status());
    assert_eq!(classic.body(), capable.body());
    assert_eq!(Some("cid-capable"), capable.correlation_id());
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert_eq!(
        2,
        received.lock().expect("capture mutex").len(),
        "single-shot means one reply per call"
    );
}

async fn wait_for_one(received: &Arc<Mutex<Vec<EventEnvelope>>>, index: usize) -> EventEnvelope {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        {
            let events = received.lock().expect("capture mutex");
            if events.len() > index {
                return events[index].clone();
            }
        }
        assert!(Instant::now() < deadline, "expected reply {index}");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

#[tokio::test]
async fn streaming_target_without_accept_is_refused_406() {
    let _guard = suite_guard().await;
    let (_, platform) = shared();
    // classic callback mode (no accept opt-in) against a streaming function:
    // the peer answers with an explicit refusal instead of a truncated reply
    let received = Arc::new(Mutex::new(Vec::new()));
    let capture = format!("capture.eoh.refuse.{}", next_seq());
    platform
        .register_with_options(
            &capture,
            Arc::new(CaptureRoute {
                received: received.clone(),
            }),
            1,
            FunctionOptions {
                zero_traced: false,
                interceptor: true,
                private: true,
            },
        )
        .expect("register capture");
    let po = PostOffice::new(&platform);
    po.send(
        EventEnvelope::new()
            .set_to(STREAMING_TARGET)
            .set_reply_to(&capture)
            .set_correlation_id("cid-refuse")
            .set_header("mode", "tokens"),
    )
    .await
    .expect("send");
    let reply = wait_for_one(&received, 0).await;
    assert_eq!(406, reply.status());
    assert_eq!(
        "Streaming function requires a caller that accepts text/event-stream",
        reply.body_as::<String>().expect("text body")
    );
}

#[tokio::test]
async fn streaming_target_via_rpc_is_refused_406() {
    let _guard = suite_guard().await;
    let (edge_port, platform) = shared();
    // the RPC path never streams (a request completes once)
    let po = PostOffice::new(&platform);
    let endpoint = format!("http://127.0.0.1:{edge_port}/api/event");
    let response = automation::event_over_http(
        &po,
        &endpoint,
        EventEnvelope::new()
            .set_to(STREAMING_TARGET)
            .set_header("mode", "tokens"),
        Duration::from_secs(15),
        true,
    )
    .await
    .expect("rpc");
    assert_eq!(406, response.status());
    assert_eq!(
        "Streaming function requires a caller that accepts text/event-stream",
        response.body_as::<String>().expect("text body")
    );
}

#[tokio::test]
async fn mid_stream_failure_propagates_exact_status() {
    let _guard = suite_guard().await;
    let (_, platform) = shared();
    let events = send_streaming(&platform, STREAMING_TARGET, Some("error-mid"), None, 2).await;
    assert_eq!("partial", events[0].body_as::<String>().expect("text body"));
    let error = &events[1];
    assert_eq!(Some(event_stream::EXCEPTION.to_string()), marker(error));
    assert_eq!(503, error.status());
    // the standard error key-values: '{"type": "error", "status": n, "message": text}'
    let body = error.body_as::<serde_json::Value>().expect("error body");
    assert_eq!(Some("error"), body["type"].as_str());
    assert_eq!(503, body["status"].as_i64().unwrap_or_default());
    assert_eq!(Some("backend on fire"), body["message"].as_str());
}

#[tokio::test]
async fn failure_before_first_segment_arrives_as_exception() {
    let _guard = suite_guard().await;
    let (_, platform) = shared();
    // a pre-head failure still rides the stream (SSE-uniform) - the caller
    // receives the exact error envelope
    let events = send_streaming(&platform, STREAMING_TARGET, Some("error-first"), None, 1).await;
    let error = &events[0];
    assert_eq!(Some(event_stream::EXCEPTION.to_string()), marker(error));
    assert_eq!(503, error.status());
    let body = error.body_as::<serde_json::Value>().expect("error body");
    assert_eq!(Some("no backend"), body["message"].as_str());
}

#[tokio::test]
async fn idle_stall_fails_in_band_408() {
    let _guard = suite_guard().await;
    let (_, platform) = shared();
    // the target declares a one-second idle allowance and goes silent - the
    // edge renderer or the client's own idle timer must fail it in-band;
    // both produce the pinned 408 wording
    let started = Instant::now();
    let events = send_streaming(&platform, STREAMING_TARGET, Some("stall"), Some("5000"), 2).await;
    let elapsed = started.elapsed();
    assert_eq!("one", events[0].body_as::<String>().expect("text body"));
    let error = &events[1];
    assert_eq!(Some(event_stream::EXCEPTION.to_string()), marker(error));
    assert_eq!(408, error.status());
    let body = error.body_as::<serde_json::Value>().expect("error body");
    assert_eq!(Some("error"), body["type"].as_str());
    let message = body["message"].as_str().unwrap_or_default();
    assert!(message.starts_with("Timeout for "), "{message}");
    assert!(
        elapsed < Duration::from_secs(15),
        "in-band timeout expected, took {elapsed:?}"
    );
}

#[tokio::test]
async fn server_pool_exhaustion_answers_503() {
    let _guard = suite_guard().await;
    let (_, platform) = shared();
    // with no reply lane available, a streaming-capable call is refused
    // single-shot with the pinned message - and recovers after release
    let mut drained = Vec::new();
    while let Some(lane) = automation::checkout_lane() {
        drained.push(lane);
    }
    assert!(!drained.is_empty(), "the pool should have lanes to drain");
    let events = send_streaming(&platform, STREAMING_TARGET, Some("tokens"), None, 1).await;
    let reply = &events[0];
    assert_eq!(503, reply.status());
    assert_eq!(
        "Streaming response pool exhausted",
        reply.body_as::<String>().expect("text body")
    );
    for lane in drained {
        automation::release_lane(lane);
    }
    // capacity restored - the same call streams normally again
    let events = send_streaming(&platform, STREAMING_TARGET, Some("tokens"), None, 3).await;
    assert_eq!(Some(event_stream::EOF.to_string()), marker(&events[2]));
}

#[tokio::test]
async fn rest_level_error_unwraps_to_callback() {
    let _guard = suite_guard().await;
    let (_, platform) = shared();
    // the relay POSTs to a GET-only endpoint: the edge answers a REST error
    // (JSON, not a serialized envelope) - the client unwraps it classically
    let events = send_streaming(&platform, "mock.rest.error", None, None, 1).await;
    let reply = &events[0];
    assert!(
        marker(reply).is_none(),
        "an edge error is a single-shot reply"
    );
    assert_eq!(405, reply.status());
}

#[tokio::test]
async fn raw_first_frame_from_foreign_server_is_rejected() {
    let _guard = suite_guard().await;
    let (_, platform) = shared();
    let events = send_streaming(&platform, "mock.sse.raw.first", None, None, 1).await;
    let error = &events[0];
    assert_eq!(Some(event_stream::EXCEPTION.to_string()), marker(error));
    assert_eq!(500, error.status());
    let body = error.body_as::<serde_json::Value>().expect("error body");
    assert_eq!(
        Some("Invalid event stream - missing envelope head"),
        body["message"].as_str()
    );
}

#[tokio::test]
async fn transport_end_without_terminal_is_truncation() {
    let _guard = suite_guard().await;
    let (_, platform) = shared();
    let events = send_streaming(&platform, "mock.sse.no.terminal", None, None, 2).await;
    assert_eq!(
        "mock-head",
        events[0].body_as::<String>().expect("text body")
    );
    let error = &events[1];
    assert_eq!(Some(event_stream::EXCEPTION.to_string()), marker(error));
    assert_eq!(500, error.status());
    let body = error.body_as::<serde_json::Value>().expect("error body");
    assert_eq!(
        Some("Event stream ended without eof"),
        body["message"].as_str()
    );
}

#[tokio::test]
async fn foreign_dialect_peer_works_and_trailing_frames_drop() {
    let _guard = suite_guard().await;
    let (_, platform) = shared();
    let events = send_streaming(&platform, "mock.sse.foreign", None, None, 3).await;
    assert_eq!(3, events.len(), "head + raw token + eof");
    assert_eq!(
        "mock-head",
        events[0].body_as::<String>().expect("text body")
    );
    assert_eq!(Some(event_stream::DATA.to_string()), marker(&events[1]));
    assert_eq!(
        "mock-token",
        events[1].body_as::<String>().expect("text body")
    );
    assert_eq!(Some(event_stream::EOF.to_string()), marker(&events[2]));
    // frames after the decoded terminal are discarded
    tokio::time::sleep(Duration::from_millis(300)).await;
}

#[tokio::test]
async fn remote_stream_renders_progressively_out_the_edge() {
    let _guard = suite_guard().await;
    let (edge_port, _) = shared();
    // the engine-to-engine composition: a streaming edge endpoint forwards its
    // reply lane into a send to the event-over-http mapped streaming function -
    // segments relay through /api/event and re-render progressively here
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", edge_port))
        .await
        .expect("connect edge");
    let request = format!(
        "GET /api/hello/remote HTTP/1.1\r\nHost: 127.0.0.1:{edge_port}\r\nAccept: text/event-stream\r\nConnection: close\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .await
        .expect("send request");
    let started = Instant::now();
    let mut received = Vec::new();
    let mut arrivals: Vec<(usize, Duration)> = Vec::new();
    let mut buf = vec![0u8; 4096];
    loop {
        match tokio::time::timeout(Duration::from_secs(15), stream.read(&mut buf)).await {
            Ok(Ok(0)) | Err(_) => break,
            Ok(Ok(n)) => {
                arrivals.push((received.len(), started.elapsed()));
                received.extend_from_slice(&buf[..n]);
            }
            Ok(Err(_)) => break,
        }
    }
    let text = String::from_utf8_lossy(&received).to_string();
    assert!(text.contains("200 OK"), "{text}");
    assert!(text.contains("text/event-stream"), "{text}");
    let alpha = text.find("data: alpha").expect("alpha frame");
    let beta = text.find("data: beta").expect("beta frame");
    let done = text.find("event: done").expect("terminal frame");
    assert!(alpha < beta && beta < done, "ordered relay: {text}");
    // the remote eof's trailing metadata is the terminal frame's data
    assert!(text.contains("data: {\"segments\":2}"), "{text}");
    // progressive end to end: the remote target paces segments 250 ms apart -
    // the read that carried beta must be measurably later than alpha's
    let offset_of = |needle: usize| {
        arrivals
            .iter()
            .rev()
            .find(|(start, _)| *start <= needle)
            .map(|(_, at)| *at)
            .unwrap_or_default()
    };
    let gap = offset_of(beta).saturating_sub(offset_of(alpha));
    assert!(
        gap >= Duration::from_millis(150),
        "progressive relay expected, gap {gap:?}: {text}"
    );
}
