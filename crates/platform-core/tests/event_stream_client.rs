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

//! Progressive SSE consumption in the HTTP client (raw mode): a request that
//! declares Accept: text/event-stream and carries a reply_to receives one
//! x-event-stream data envelope per upstream SSE event, then eof - the same
//! producer contract the HTTP edge consumes. Everything else keeps the
//! buffered single-shot behavior. (Java `EventStreamClientTest` twin.)

use std::collections::HashMap;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::{Arc, Mutex, Once, OnceLock};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use platform_core::automation::http_client::AsyncHttpClientService;
use platform_core::platform::FunctionOptions;
use platform_core::{
    automation, event_stream, overrides, resources, AppConfigReader, AppError, ComposableFunction,
    EventEnvelope, EventStreamWriter, Platform, PostOffice,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const REST_YAML: &str = r#"
rest:
  # SSE producer endpoint - the upstream for the raw-mapping and relay tests
  - service: "hello.event.stream"
    methods: ['GET']
    url: "/api/hello/stream"
    timeout: 15s
    stream: true

  # SSE-to-SSE relay: the function forwards its reply lane into
  # async.http.request, which consumes this app's own SSE endpoint progressively
  - service: "hello.event.relay"
    methods: ['GET']
    url: "/api/hello/relay"
    timeout: 15s
    stream: true

  # a plain JSON endpoint for the buffered-fallback test
  - service: "hello.plain.json"
    methods: ['GET']
    url: "/api/hello/json"
    timeout: 10s
"#;

/// Streams "Hello" and "token stream" 250 ms apart, then closes with metadata.
struct SseProducer {
    platform: Platform,
}

#[async_trait]
impl ComposableFunction for SseProducer {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let mut out = EventStreamWriter::from_request(&self.platform, &input)?;
        out.first(200, "text/event-stream");
        out.write("Hello").await?;
        tokio::time::sleep(Duration::from_millis(250)).await;
        out.write("token stream").await?;
        tokio::time::sleep(Duration::from_millis(250)).await;
        out.close_with(serde_json::json!({"segments": 2})).await?;
        Ok(EventEnvelope::new())
    }
}

/// Forwards its own reply lane and correlation id into async.http.request
/// aimed at this app's own SSE endpoint - the SSE-to-SSE relay composition.
struct SseRelayFixture {
    platform: Platform,
    edge_port: Arc<AtomicU16>,
}

#[async_trait]
impl ComposableFunction for SseRelayFixture {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let port = self.edge_port.load(Ordering::Relaxed);
        let upstream = automation::AsyncHttpRequest::new()
            .set_method("GET")
            .set_target_host(&format!("http://127.0.0.1:{port}"))
            .set_url("/api/hello/stream")
            // explicit Accept opts into progressive SSE consumption (D1)
            .set_header("accept", "text/event-stream")
            .set_timeout_seconds(10);
        let po = PostOffice::new(&self.platform);
        po.send(
            EventEnvelope::new()
                .set_to(automation::ASYNC_HTTP_REQUEST)
                .set_raw_body(upstream.to_value())
                .set_reply_to(input.reply_to().unwrap_or_default())
                .set_correlation_id(input.correlation_id().unwrap_or_default()),
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

/// Answers a plain JSON body (the buffered-fallback upstream).
struct PlainJson;

#[async_trait]
impl ComposableFunction for PlainJson {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new()
            .set_header("content-type", "application/json")
            .set_body(serde_json::json!({"hello": "world"}))
    }
}

/// Start a fresh server INSIDE this test's runtime (the rest_automation.rs
/// idiom): Platform::new() is an isolated registry and route workers belong to
/// the runtime that registers them, so each test registers its own fixtures on
/// its own platform and uses that handle throughout. async.http.request is an
/// essential service of the full application lifecycle (app_starter); these
/// tests bypass the lifecycle, so it registers here like rest_automation does.
async fn server() -> (u16, Platform) {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let rest_file =
            std::env::temp_dir().join(format!("rest-sse-client-{}.yaml", std::process::id()));
        std::fs::write(&rest_file, REST_YAML).unwrap();
        overrides::set(
            "yaml.rest.automation",
            &format!("file:{}", rest_file.display()),
        );
        overrides::set("rest.server.port", "0");
        let _ = AppConfigReader::get_instance();
    });
    let platform = Platform::new();
    let edge_port = Arc::new(AtomicU16::new(0));
    let options = FunctionOptions {
        zero_traced: false,
        interceptor: true,
        private: true,
    };
    platform
        .register_with_options(
            "hello.event.stream",
            Arc::new(SseProducer {
                platform: platform.clone(),
            }),
            10,
            options,
        )
        .expect("register producer");
    platform
        .register_with_options(
            "hello.event.relay",
            Arc::new(SseRelayFixture {
                platform: platform.clone(),
                edge_port: edge_port.clone(),
            }),
            10,
            options,
        )
        .expect("register relay");
    platform
        .register("hello.plain.json", Arc::new(PlainJson), 5)
        .expect("register json");
    platform
        .register_with_options(
            automation::ASYNC_HTTP_REQUEST,
            Arc::new(AsyncHttpClientService::new(&platform)),
            10,
            options,
        )
        .expect("register http client");
    let addr = automation::start_http_server(&platform)
        .await
        .expect("http server");
    edge_port.store(addr.port(), Ordering::Relaxed);
    (addr.port(), platform)
}

/// A raw mock SSE upstream with hand-rolled chunked framing, so the tests can
/// produce silence, an abrupt disconnect (no terminal chunk), keep-alive
/// comments, an unpaced burst, and multi-field frames. It runs on its OWN
/// thread and runtime - a task spawned on a test's runtime dies with that
/// test (the per-test-runtime idiom), which would drop sockets mid-sleep.
fn mock_sse_upstream() -> u16 {
    static PORT: OnceLock<u16> = OnceLock::new();
    *PORT.get_or_init(|| {
        let (tx, rx) = std::sync::mpsc::channel::<u16>();
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("mock runtime");
            runtime.block_on(mock_accept_loop(tx));
        });
        rx.recv().expect("mock upstream port")
    })
}

async fn mock_accept_loop(ready: std::sync::mpsc::Sender<u16>) {
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .expect("mock upstream bind");
    let port = listener.local_addr().expect("mock addr").port();
    ready.send(port).expect("announce mock port");
    {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                break;
            };
            tokio::spawn(async move {
                let mut buf = vec![0u8; 4096];
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
                let head = "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\ntransfer-encoding: chunked\r\n\r\n";
                if socket.write_all(head.as_bytes()).await.is_err() {
                    return;
                }
                let chunk = |text: &str| format!("{:x}\r\n{}\r\n", text.len(), text);
                match path.as_str() {
                    "/sse/silent" => {
                        // one event, then silence - no pings, no terminal chunk
                        let _ = socket.write_all(chunk("data: one\n\n").as_bytes()).await;
                        tokio::time::sleep(Duration::from_secs(30)).await;
                    }
                    "/sse/abort" => {
                        // one event, then the connection dies mid-stream
                        // (no terminal 0-chunk: an incomplete chunked body)
                        let _ = socket
                            .write_all(chunk("data: partial\n\n").as_bytes())
                            .await;
                        tokio::time::sleep(Duration::from_millis(200)).await;
                        drop(socket);
                    }
                    "/sse/comments" => {
                        // quiet for ~2.5s but alive: keep-alive comments every
                        // 300ms, then a final event and a clean end
                        let _ = socket.write_all(chunk("data: early\n\n").as_bytes()).await;
                        for _ in 0..8 {
                            tokio::time::sleep(Duration::from_millis(300)).await;
                            if socket
                                .write_all(chunk(": ping\n\n").as_bytes())
                                .await
                                .is_err()
                            {
                                return;
                            }
                        }
                        let _ = socket.write_all(chunk("data: late\n\n").as_bytes()).await;
                        let _ = socket.write_all(b"0\r\n\r\n").await;
                    }
                    "/sse/burst" => {
                        for i in 1..=50 {
                            let frame = format!("data: item-{i}\n\n");
                            if socket.write_all(chunk(&frame).as_bytes()).await.is_err() {
                                return;
                            }
                        }
                        let _ = socket.write_all(b"0\r\n\r\n").await;
                    }
                    "/sse/multifield" => {
                        let frame =
                            "event: tokens\nid: 7\nretry: 1000\ndata: line1\ndata: line2\n\n";
                        let _ = socket.write_all(chunk(frame).as_bytes()).await;
                        let _ = socket.write_all(b"0\r\n\r\n").await;
                    }
                    _ => {
                        let _ = socket.write_all(b"0\r\n\r\n").await;
                    }
                }
            });
        }
    }
}

/// Invoke async.http.request with Accept: text/event-stream and a capture
/// route as reply_to; return the captured envelopes up to and including the
/// first terminal marker.
async fn consume(
    platform: &Platform,
    target_host: &str,
    url: &str,
    timeout_seconds: u64,
    expected: usize,
) -> Vec<EventEnvelope> {
    static SEQ: OnceLock<Mutex<u32>> = OnceLock::new();
    let route = {
        let mut seq = SEQ.get_or_init(|| Mutex::new(0)).lock().expect("seq");
        *seq += 1;
        format!("capture.sse.client.{}", *seq)
    };
    let received = Arc::new(Mutex::new(Vec::new()));
    let _ = platform.register_with_options(
        &route,
        Arc::new(CaptureRoute {
            received: received.clone(),
        }),
        1,
        FunctionOptions {
            zero_traced: false,
            interceptor: true,
            private: true,
        },
    );
    let request = automation::AsyncHttpRequest::new()
        .set_method("GET")
        .set_target_host(target_host)
        .set_url(url)
        .set_header("accept", "text/event-stream")
        .set_timeout_seconds(timeout_seconds);
    let po = PostOffice::new(platform);
    po.send(
        EventEnvelope::new()
            .set_to(automation::ASYNC_HTTP_REQUEST)
            .set_raw_body(request.to_value())
            .set_reply_to(&route)
            .set_correlation_id("cid-sse-client"),
    )
    .await
    .expect("dispatch");
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        {
            let events = received.lock().expect("capture mutex");
            let terminal = events.iter().any(|e| {
                matches!(
                    e.header(event_stream::X_EVENT_STREAM),
                    Some(event_stream::EOF) | Some(event_stream::EXCEPTION)
                )
            });
            if events.len() >= expected || terminal {
                return events.clone();
            }
        }
        assert!(Instant::now() < deadline, "timed out waiting for segments");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

fn marker(event: &EventEnvelope) -> Option<&str> {
    event.header(event_stream::X_EVENT_STREAM)
}

fn body_text(event: &EventEnvelope) -> String {
    match event.body() {
        rmpv::Value::String(text) => text.as_str().unwrap_or_default().to_string(),
        other => other.to_string(),
    }
}

// ---- tests ----

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn raw_sse_events_map_to_data_envelopes_with_terminal_eof() {
    let (port, platform) = server().await;
    let events = consume(
        &platform,
        &format!("http://127.0.0.1:{port}"),
        "/api/hello/stream",
        10,
        4,
    )
    .await;
    assert_eq!(events.len(), 4, "3 data envelopes + eof");
    let first = &events[0];
    assert_eq!(marker(first), Some(event_stream::DATA));
    // head control rides the first envelope: upstream status + SSE content type
    assert_eq!(first.status(), 200);
    assert_eq!(first.header("content-type"), Some("text/event-stream"));
    assert_eq!(body_text(first), "Hello");
    assert_eq!(body_text(&events[1]), "token stream");
    // the upstream's terminal SSE frame arrives as a NAMED data event - the
    // client does not interpret payloads (D3); its own eof marks the real end
    let done = &events[2];
    assert_eq!(marker(done), Some(event_stream::DATA));
    assert_eq!(done.header(event_stream::X_EVENT_NAME), Some("done"));
    assert_eq!(body_text(done), "{\"segments\":2}");
    assert_eq!(marker(&events[3]), Some(event_stream::EOF));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn multi_field_frames_map_per_sse_specification() {
    let (_port, platform) = server().await;
    let upstream = mock_sse_upstream();
    let events = consume(
        &platform,
        &format!("http://127.0.0.1:{upstream}"),
        "/sse/multifield",
        5,
        2,
    )
    .await;
    let data = &events[0];
    assert_eq!(marker(data), Some(event_stream::DATA));
    assert_eq!(data.header(event_stream::X_EVENT_NAME), Some("tokens"));
    assert_eq!(
        body_text(data),
        "line1\nline2",
        "multi-line data joins with newline"
    );
    assert_eq!(marker(&events[1]), Some(event_stream::EOF));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn burst_events_arrive_in_strict_fifo_order() {
    let (_port, platform) = server().await;
    let upstream = mock_sse_upstream();
    let events = consume(
        &platform,
        &format!("http://127.0.0.1:{upstream}"),
        "/sse/burst",
        10,
        51,
    )
    .await;
    assert_eq!(events.len(), 51, "50 data envelopes + eof");
    for i in 1..=50 {
        assert_eq!(
            body_text(&events[i - 1]),
            format!("item-{i}"),
            "strict FIFO"
        );
    }
    assert_eq!(marker(&events[50]), Some(event_stream::EOF));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn idle_stall_fails_in_band_with_timeout_408() {
    let (_port, platform) = server().await;
    let upstream = mock_sse_upstream();
    let events = consume(
        &platform,
        &format!("http://127.0.0.1:{upstream}"),
        "/sse/silent",
        2,
        3,
    )
    .await;
    assert_eq!(body_text(&events[0]), "one");
    let error = &events[1];
    assert_eq!(marker(error), Some(event_stream::EXCEPTION));
    let body: serde_json::Value = error.body_as().expect("error body");
    assert_eq!(error.status(), 408, "unexpected error: {body}");
    assert_eq!(body["message"], "Timeout for 2 seconds");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn keep_alive_comments_reset_the_idle_allowance() {
    // quiet for ~2.5s with 300ms comments under a 2s idle allowance:
    // the comments prove liveness, so the stream must complete
    let (_port, platform) = server().await;
    let upstream = mock_sse_upstream();
    let events = consume(
        &platform,
        &format!("http://127.0.0.1:{upstream}"),
        "/sse/comments",
        2,
        3,
    )
    .await;
    assert_eq!(events.len(), 3);
    assert_eq!(body_text(&events[0]), "early");
    assert_eq!(body_text(&events[1]), "late");
    assert_eq!(marker(&events[2]), Some(event_stream::EOF));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mid_stream_disconnect_fails_in_band() {
    let (_port, platform) = server().await;
    let upstream = mock_sse_upstream();
    let events = consume(
        &platform,
        &format!("http://127.0.0.1:{upstream}"),
        "/sse/abort",
        10,
        3,
    )
    .await;
    assert_eq!(body_text(&events[0]), "partial");
    let error = &events[1];
    assert_eq!(marker(error), Some(event_stream::EXCEPTION));
    assert_eq!(error.status(), 500);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn non_sse_upstream_falls_back_to_buffered_single_shot() {
    // Accept opted in, but the upstream answers JSON - one unmarked reply
    let (port, platform) = server().await;
    let events = consume(
        &platform,
        &format!("http://127.0.0.1:{port}"),
        "/api/hello/json",
        10,
        1,
    )
    .await;
    assert_eq!(events.len(), 1);
    let reply = &events[0];
    assert_eq!(
        marker(reply),
        None,
        "a buffered reply carries no stream marker"
    );
    let body: serde_json::Value = reply.body_as().expect("json body");
    assert_eq!(body["hello"], "world");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn without_accept_the_sse_response_buffers_as_before() {
    // backward-compat pin: an RPC without Accept: text/event-stream receives
    // the whole SSE payload buffered as one text body (today's behavior)
    let (port, platform) = server().await;
    let request = automation::AsyncHttpRequest::new()
        .set_method("GET")
        .set_target_host(&format!("http://127.0.0.1:{port}"))
        .set_url("/api/hello/stream")
        .set_timeout_seconds(10);
    let po = PostOffice::new(&platform);
    let response = po
        .request(
            EventEnvelope::new()
                .set_to(automation::ASYNC_HTTP_REQUEST)
                .set_raw_body(request.to_value()),
            Duration::from_secs(15),
        )
        .await
        .expect("rpc");
    assert_eq!(response.status(), 200);
    let text = body_text(&response);
    assert!(text.contains("data: Hello"), "{text}");
    assert!(text.contains("event: done"), "{text}");
}

/// Decode as much of an HTTP/1.1 chunked body as has arrived.
fn decode_chunked(raw: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0;
    while let Some(nl) = raw[i..].windows(2).position(|w| w == b"\r\n") {
        let size_text = String::from_utf8_lossy(&raw[i..i + nl]);
        let size_text = size_text.split(';').next().unwrap_or("").trim().to_string();
        let Ok(size) = usize::from_str_radix(&size_text, 16) else {
            break;
        };
        if size == 0 {
            break;
        }
        let start = i + nl + 2;
        if raw.len() < start + size + 2 {
            break;
        }
        out.extend_from_slice(&raw[start..start + size]);
        i = start + size + 2;
    }
    out
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn self_relay_streams_progressively_out_the_edge() {
    // the flagship composition: /api/hello/relay forwards its reply lane into
    // async.http.request aimed at this app's own SSE endpoint - upstream
    // frames re-render at the edge, followed by the relay's own terminal
    let (port, _platform) = server().await;
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let request = "GET /api/hello/relay HTTP/1.1\r\nHost: localhost\r\nAccept: text/event-stream\r\nConnection: close\r\n\r\n";
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut raw: Vec<u8> = Vec::new();
    let mut buf = [0u8; 8192];
    let mut lines: Vec<(String, Instant)> = Vec::new();
    let mut seen = 0usize;
    let mut head_text = String::new();
    loop {
        let n = tokio::time::timeout(Duration::from_secs(20), stream.read(&mut buf))
            .await
            .expect("read timed out")
            .unwrap_or(0);
        if n == 0 {
            break;
        }
        raw.extend_from_slice(&buf[..n]);
        let now = Instant::now();
        let Some(pos) = raw.windows(4).position(|w| w == b"\r\n\r\n") else {
            continue;
        };
        if head_text.is_empty() {
            head_text = String::from_utf8_lossy(&raw[..pos]).to_lowercase();
        }
        let decoded = decode_chunked(&raw[pos + 4..]);
        let text = String::from_utf8_lossy(&decoded).to_string();
        if let Some(last) = text.rfind('\n') {
            let complete: Vec<&str> = text[..last].split('\n').collect();
            for line in complete.iter().skip(seen) {
                lines.push((line.to_string(), now));
            }
            seen = complete.len();
        }
    }
    assert!(head_text.contains(" 200 "), "{head_text}");
    assert!(
        head_text.contains("content-type: text/event-stream"),
        "{head_text}"
    );
    let body: Vec<String> = lines.iter().map(|(l, _)| l.clone()).collect();
    let hello = body.iter().position(|l| l == "data: Hello").expect("hello");
    let tokens = body
        .iter()
        .position(|l| l == "data: token stream")
        .expect("token stream");
    assert!(hello < tokens, "ordered relay: {body:?}");
    let metadata = body
        .iter()
        .position(|l| l == "data: {\"segments\":2}")
        .expect("upstream metadata");
    // the upstream's done event re-renders as a named frame; the relay's own
    // eof renders one final terminal event
    let done_count = body.iter().filter(|l| *l == "event: done").count();
    assert_eq!(done_count, 2, "upstream done + relay terminal: {body:?}");
    // progressive end to end: the upstream paces segments 250ms apart
    let elapsed = lines[metadata].1.duration_since(lines[hello].1);
    assert!(
        elapsed >= Duration::from_millis(300),
        "progressive relay expected, elapsed {elapsed:?}"
    );
}
