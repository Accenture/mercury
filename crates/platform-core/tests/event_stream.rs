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

//! End-to-end tests for HTTP response streaming: a callee streams events to
//! the caller's reply_to (a dedicated ordered reply lane checked out from the
//! pool for the request's lifetime) until end of transmission, and the edge
//! renders them progressively — SSE framing for text/event-stream,
//! chunked/NDJSON otherwise. The wire carries only standard HTTP.
//! (Java `EventStreamResponseTest` twin.)

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use platform_core::platform::FunctionOptions;
use platform_core::{
    automation, event_stream, overrides, resources, AppConfigReader, AppError, ComposableFunction,
    EventEnvelope, EventStreamWriter, Platform, PostOffice,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const REST_YAML: &str = r#"
rest:
  # Streaming response endpoint (multi-shot reply route). "stream: true"
  # checks out a dedicated ordered reply lane so token segments render in the
  # exact order the callee sent them. The endpoint's response header
  # transform must apply to the streamed head like a single-shot response.
  - service: "hello.event.stream"
    methods: ['GET']
    url: "/api/hello/stream"
    timeout: 15s
    stream: true
    headers: header_2

headers:
  - id: header_2
    response:
      add: ["x-stream-transform: applied"]
      drop: ['x-secret-header']
"#;

const PACE_MS: u64 = 250;

/// Test producer for HTTP response streaming — the callee side of the
/// multi-shot reply route. Modes are selected by the "mode" query parameter
/// (Java `MockEventStreamService` twin).
struct MockEventStream {
    platform: Platform,
}

#[async_trait]
impl ComposableFunction for MockEventStream {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        let mode = request["parameters"]["query"]["mode"]
            .as_str()
            .unwrap_or("sse")
            .to_string();
        let mut out = EventStreamWriter::from_request(&self.platform, &input)?;
        match mode.as_str() {
            "sse" => {
                out.first(200, "text/event-stream");
                out.write("Hello").await?;
                tokio::time::sleep(Duration::from_millis(PACE_MS)).await;
                out.write("token stream").await?;
                tokio::time::sleep(Duration::from_millis(PACE_MS)).await;
                out.close_with(serde_json::json!({"segments": 2})).await?;
            }
            "sse-named" => {
                out.first(200, "text/event-stream");
                out.write_named("tokens", serde_json::json!({"n": 1}))
                    .await?;
                out.write_named("tokens", serde_json::json!({"n": 2}))
                    .await?;
                out.close().await?;
            }
            "sse-multiline" => {
                out.first(200, "text/event-stream");
                out.write("line1\nline2").await?;
                out.close().await?;
            }
            "ndjson" => {
                // no first() - content type falls back to Accept negotiation
                for i in 1..=3 {
                    out.write(serde_json::json!({"seq": i})).await?;
                }
                out.close_with(serde_json::json!({"ignored": true})).await?;
            }
            "chunk" => {
                out.first(200, "text/plain");
                out.write("alpha").await?;
                out.write("beta").await?;
                out.close().await?;
            }
            "error" => {
                out.first(200, "text/event-stream");
                out.write("partial").await?;
                out.fail(&AppError::new(503, "backend on fire")).await?;
            }
            "error-first" => {
                out.fail(&AppError::new(503, "no backend")).await?;
            }
            "stall" => {
                // one-second idle allowance, then silence - the edge must
                // fail the stream in-band
                out.first_with_ttl(200, "text/event-stream", 1);
                out.write("one").await?;
            }
            "ping" => {
                // the keep-alive comments flow while the producer is quiet
                // after the head is committed
                out.first(200, "text/event-stream");
                out.write("early").await?;
                tokio::time::sleep(Duration::from_millis(2500)).await;
                out.write("late").await?;
                out.close().await?;
            }
            "empty-close" => {
                out.close_with(serde_json::json!({"done": true})).await?;
            }
            "slow-paced" => {
                // total duration exceeds the 1s idle allowance, but every gap
                // is within it: each arriving segment extends the stream's life
                out.first_with_ttl(200, "text/event-stream", 1);
                for i in 1..=3 {
                    out.write(format!("segment-{i}")).await?;
                    tokio::time::sleep(Duration::from_millis(700)).await;
                }
                out.close().await?;
            }
            "burst" => {
                // 50 unpaced 8KB segments - strict FIFO is guaranteed by the
                // request's dedicated reply lane, and the payload size forces
                // frame back-pressure so the drain path is exercised
                out.first(200, "text/plain");
                let padding = "x".repeat(8192);
                for i in 1..=50 {
                    out.write(format!("{i}|{padding}\n")).await?;
                }
                out.close().await?;
            }
            "headers" => {
                // raw first event carrying custom headers - the endpoint's
                // response header transform must add/drop exactly as for a
                // single-shot response; a stray x-stream-id is ignored
                let po = PostOffice::new(&self.platform);
                po.send(
                    EventEnvelope::new()
                        .set_to(input.reply_to().unwrap_or_default())
                        .set_correlation_id(input.correlation_id().unwrap_or_default())
                        .set_header(event_stream::X_EVENT_STREAM, event_stream::DATA)
                        .set_header("x-stream-id", "stream.fake.in")
                        .set_header("x-secret-header", "hide-me")
                        .set_header("x-custom-note", "visible")
                        .set_header("content-type", "text/event-stream")
                        .set_status(200)
                        .set_body("transformed")?,
                )
                .await?;
                out.close().await?;
            }
            "single-shot" => {
                // an unmarked reply on a streaming endpoint renders exactly
                // as an ordinary single-shot response
                let po = PostOffice::new(&self.platform);
                po.send(
                    EventEnvelope::new()
                        .set_to(input.reply_to().unwrap_or_default())
                        .set_correlation_id(input.correlation_id().unwrap_or_default())
                        .set_header("content-type", "text/plain")
                        .set_status(200)
                        .set_body("regular response")?,
                )
                .await?;
            }
            other => {
                out.fail(&AppError::new(400, format!("unknown mode {other}")))
                    .await?;
            }
        }
        Ok(EventEnvelope::new())
    }
}

/// Start a fresh server inside this test's runtime (the rest_automation.rs
/// pattern): port 0 keeps parallel servers from colliding; platform state and
/// the reply-lane pool are process-wide.
async fn server() -> u16 {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let rest_file =
            std::env::temp_dir().join(format!("rest-stream-{}.yaml", std::process::id()));
        std::fs::write(&rest_file, REST_YAML).unwrap();
        overrides::set(
            "yaml.rest.automation",
            &format!("file:{}", rest_file.display()),
        );
        overrides::set("rest.server.port", "0");
        // test configuration: fast keep-alive so quiet streams show pings
        overrides::set("event.stream.keep.alive", "1s");
        let _ = AppConfigReader::get_instance();
    });
    let platform = Platform::new();
    if !platform.has_route("hello.event.stream") {
        let _ = platform.register_with_options(
            "hello.event.stream",
            Arc::new(MockEventStream {
                platform: platform.clone(),
            }),
            10,
            FunctionOptions {
                zero_traced: false,
                interceptor: true,
                private: true,
            },
        );
    }
    let addr = automation::start_http_server(&platform)
        .await
        .expect("http server");
    addr.port()
}

/// One received line with its arrival time.
struct TimedLine {
    line: String,
    at: Instant,
}

/// Decode as much of an HTTP/1.1 chunked body as has arrived (incomplete
/// trailing chunks are ignored until more bytes arrive).
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

/// Issue a GET and read the response progressively, timestamping every line
/// as it arrives (the Java BodyHandlers.ofLines + nanoTime pattern). Returns
/// (status, lowercased head block, timed lines).
async fn stream_request(port: u16, path: &str, accept: &str) -> (u16, String, Vec<TimedLine>) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: localhost\r\nAccept: {accept}\r\nConnection: close\r\n\r\n"
    );
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut raw: Vec<u8> = Vec::new();
    let mut buf = [0u8; 8192];
    let mut lines: Vec<TimedLine> = Vec::new();
    let mut seen = 0usize;
    let mut head_text = String::new();
    let mut decoded_text = String::new();
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
        let body_raw = &raw[pos + 4..];
        let decoded = if head_text.contains("transfer-encoding: chunked") {
            decode_chunked(body_raw)
        } else {
            body_raw.to_vec()
        };
        decoded_text = String::from_utf8_lossy(&decoded).to_string();
        // record newly completed lines with this read's arrival time
        let complete: Vec<&str> = match decoded_text.rfind('\n') {
            Some(last) => decoded_text[..last].split('\n').collect(),
            None => Vec::new(),
        };
        for line in complete.iter().skip(seen) {
            lines.push(TimedLine {
                line: line.to_string(),
                at: now,
            });
        }
        seen = complete.len();
    }
    // flush a trailing line without a newline (chunked text mode)
    let tail_start = decoded_text.rfind('\n').map(|last| last + 1).unwrap_or(0);
    if tail_start < decoded_text.len() {
        lines.push(TimedLine {
            line: decoded_text[tail_start..].to_string(),
            at: Instant::now(),
        });
    }
    let status: u16 = head_text
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap_or(0);
    (status, head_text, lines)
}

fn plain(lines: &[TimedLine]) -> Vec<String> {
    lines.iter().map(|l| l.line.clone()).collect()
}

// ---- tests ----

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn sse_segments_arrive_progressively_with_terminal_done_event() {
    let port = server().await;
    let (status, head, lines) =
        stream_request(port, "/api/hello/stream?mode=sse", "text/event-stream").await;
    assert_eq!(status, 200);
    assert!(head.contains("content-type: text/event-stream"), "{head}");
    assert!(head.contains("cache-control: no-cache"), "{head}");
    assert!(
        !head.contains("content-length"),
        "a stream has no content length: {head}"
    );
    let body = plain(&lines);
    let first = body.iter().position(|l| l == "data: Hello");
    let second = body.iter().position(|l| l == "data: token stream");
    let done = body.iter().position(|l| l == "event: done");
    let (first, second, done) = (
        first.expect("first segment"),
        second.expect("second segment"),
        done.expect("done event"),
    );
    assert!(
        first < second && second < done,
        "ordered SSE frames: {body:?}"
    );
    assert!(
        body.contains(&"data: {\"segments\":2}".to_string()),
        "eof body rides the done event: {body:?}"
    );
    // the producer paces segments 250 ms apart - a buffered response would
    // arrive all at once
    let elapsed = lines[done].at.duration_since(lines[first].at);
    assert!(
        elapsed >= Duration::from_millis(300),
        "progressive delivery expected, elapsed {elapsed:?}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn named_segments_become_typed_sse_events() {
    let port = server().await;
    let (_, _, lines) = stream_request(
        port,
        "/api/hello/stream?mode=sse-named",
        "text/event-stream",
    )
    .await;
    let body = plain(&lines);
    let name = body
        .iter()
        .position(|l| l == "event: tokens")
        .expect("typed event");
    assert_eq!(
        body[name + 1],
        "data: {\"n\":1}",
        "typed event framing: {body:?}"
    );
    assert!(body.contains(&"event: done".to_string()), "{body:?}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn multi_line_segment_splits_into_successive_data_lines() {
    let port = server().await;
    let (_, _, lines) = stream_request(
        port,
        "/api/hello/stream?mode=sse-multiline",
        "text/event-stream",
    )
    .await;
    let body = plain(&lines);
    let first = body
        .iter()
        .position(|l| l == "data: line1")
        .expect("first line");
    assert_eq!(
        body[first + 1],
        "data: line2",
        "SSE multi-line framing: {body:?}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn structured_segments_stream_as_json_lines_in_chunked_mode() {
    let port = server().await;
    let (status, head, lines) = stream_request(port, "/api/hello/stream?mode=ndjson", "*/*").await;
    assert_eq!(status, 200);
    assert!(head.contains("content-type: application/json"), "{head}");
    let body: Vec<String> = plain(&lines)
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(body.len(), 3, "one JSON object per line: {body:?}");
    for (i, line) in body.iter().enumerate() {
        let map: serde_json::Value = serde_json::from_str(line).expect("json line");
        assert_eq!(map["seq"], (i + 1) as i64);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn text_segments_append_in_chunked_mode() {
    let port = server().await;
    let (status, head, lines) =
        stream_request(port, "/api/hello/stream?mode=chunk", "text/plain").await;
    assert_eq!(status, 200);
    assert!(head.contains("content-type: text/plain"), "{head}");
    assert!(!head.contains("content-length"), "{head}");
    let text: String = plain(&lines).join("");
    assert_eq!(text, "alphabeta");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mid_stream_failure_arrives_as_in_band_error_event() {
    let port = server().await;
    let (status, _, lines) =
        stream_request(port, "/api/hello/stream?mode=error", "text/event-stream").await;
    // the head is committed by the first segment, so the status stays 200
    assert_eq!(status, 200);
    let body = plain(&lines);
    assert!(body.contains(&"data: partial".to_string()), "{body:?}");
    let error = body
        .iter()
        .position(|l| l == "event: error")
        .expect("in-band error event");
    assert!(body[error + 1].contains("backend on fire"), "{body:?}");
    assert!(body[error + 1].contains("503"), "{body:?}");
    assert!(
        !body.contains(&"event: done".to_string()),
        "a failed stream has no done event"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn failure_before_first_segment_is_a_normal_http_error() {
    let port = server().await;
    let (status, _, lines) = stream_request(
        port,
        "/api/hello/stream?mode=error-first",
        "application/json",
    )
    .await;
    assert_eq!(status, 503);
    let text = plain(&lines).join("");
    assert!(text.contains("no backend"), "{text}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn eof_only_stream_renders_terminal_event_immediately() {
    let port = server().await;
    let (status, head, lines) = stream_request(
        port,
        "/api/hello/stream?mode=empty-close",
        "text/event-stream",
    )
    .await;
    assert_eq!(status, 200);
    assert!(head.contains("content-type: text/event-stream"), "{head}");
    let body = plain(&lines);
    let done = body
        .iter()
        .position(|l| l == "event: done")
        .expect("done event");
    assert!(body[done + 1].contains("\"done\":true"), "{body:?}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn keep_alive_comments_flow_while_the_producer_is_quiet() {
    // test configuration sets event.stream.keep.alive=1s; the producer is
    // quiet for 2.5s after its first segment
    let port = server().await;
    let (_, _, lines) =
        stream_request(port, "/api/hello/stream?mode=ping", "text/event-stream").await;
    let body = plain(&lines);
    assert!(
        body.contains(&": ping".to_string()),
        "keep-alive comment expected: {body:?}"
    );
    assert!(body.contains(&"data: late".to_string()), "{body:?}");
    assert!(body.contains(&"event: done".to_string()), "{body:?}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn stalled_producer_times_out_in_band() {
    // the producer declares a one-second idle allowance (x-ttl) then goes
    // silent; the edge must fail the stream in-band
    let started = Instant::now();
    let port = server().await;
    let (_, _, lines) =
        stream_request(port, "/api/hello/stream?mode=stall", "text/event-stream").await;
    let body = plain(&lines);
    assert!(body.contains(&"data: one".to_string()), "{body:?}");
    let error = body
        .iter()
        .position(|l| l == "event: error")
        .expect("in-band timeout expected");
    assert!(
        body[error + 1].contains("Timeout for 1 seconds"),
        "{body:?}"
    );
    assert!(
        started.elapsed() < Duration::from_secs(10),
        "the idle allowance should fail the stream quickly, took {:?}",
        started.elapsed()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn arriving_segments_extend_the_idle_allowance() {
    // 3 segments 700ms apart under a 1s idle ttl: total 2.1s > ttl, but every
    // gap is within it - the per-segment arrival must keep the stream alive
    let port = server().await;
    let (_, _, lines) = stream_request(
        port,
        "/api/hello/stream?mode=slow-paced",
        "text/event-stream",
    )
    .await;
    let body = plain(&lines);
    assert!(body.contains(&"data: segment-3".to_string()), "{body:?}");
    assert!(
        body.contains(&"event: done".to_string()),
        "the paced stream must complete: {body:?}"
    );
    assert!(!body.contains(&"event: error".to_string()), "{body:?}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn burst_segments_render_in_strict_fifo_order() {
    // 50 unpaced segments - the request's dedicated reply lane (a
    // single-instance route) must preserve exact FIFO order
    let port = server().await;
    let (status, _, lines) =
        stream_request(port, "/api/hello/stream?mode=burst", "text/plain").await;
    assert_eq!(status, 200);
    let body: Vec<String> = plain(&lines)
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(body.len(), 50, "all segments delivered");
    for (i, line) in body.iter().enumerate() {
        let sequence = line.split('|').next().unwrap_or("");
        assert_eq!(sequence, (i + 1).to_string(), "strict segment order");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_streams_render_independently_and_in_order() {
    // four parallel 50-segment bursts: each request checks out its own
    // dedicated reply lane, so segments stay in strict FIFO while the
    // requests stream concurrently
    let port = server().await;
    let mut tasks = Vec::new();
    for _ in 0..4 {
        tasks.push(tokio::spawn(async move {
            let (_, _, lines) =
                stream_request(port, "/api/hello/stream?mode=burst", "text/plain").await;
            plain(&lines)
                .into_iter()
                .filter(|l| !l.is_empty())
                .collect::<Vec<String>>()
        }));
    }
    for task in tasks {
        let body = task.await.expect("burst task");
        assert_eq!(body.len(), 50, "all segments delivered");
        for (i, line) in body.iter().enumerate() {
            let sequence = line.split('|').next().unwrap_or("");
            assert_eq!(sequence, (i + 1).to_string(), "strict per-request order");
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn response_header_transform_applies_to_the_streamed_head() {
    // the endpoint declares "headers: header_2" (add x-stream-transform, drop
    // x-secret-header) - the streamed head must honor it like a single-shot
    // response; the stray x-stream-id is ignored (marker precedence)
    let port = server().await;
    let (status, head, lines) =
        stream_request(port, "/api/hello/stream?mode=headers", "text/event-stream").await;
    assert_eq!(status, 200);
    assert!(
        head.contains("x-stream-transform: applied"),
        "add directive: {head}"
    );
    assert!(!head.contains("x-secret-header"), "drop directive: {head}");
    assert!(
        head.contains("x-custom-note: visible"),
        "passthrough: {head}"
    );
    assert!(
        !head.contains("x-stream-id"),
        "reserved header never on the wire: {head}"
    );
    let body = plain(&lines);
    assert!(body.contains(&"data: transformed".to_string()), "{body:?}");
    assert!(body.contains(&"event: done".to_string()), "{body:?}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unmarked_reply_on_a_streaming_endpoint_renders_single_shot() {
    let port = server().await;
    let (status, head, lines) =
        stream_request(port, "/api/hello/stream?mode=single-shot", "text/plain").await;
    assert_eq!(status, 200);
    assert!(head.contains("content-type: text/plain"), "{head}");
    assert_eq!(plain(&lines).join(""), "regular response");
}

// ---- producer contract (Java EventStreamWriterTest twin) ----

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

async fn captured_count(received: &Arc<Mutex<Vec<EventEnvelope>>>, at_least: usize) {
    for _ in 0..100 {
        if received.lock().expect("capture mutex").len() >= at_least {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("expected {at_least} captured segments");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn writer_speaks_the_multi_shot_reply_route_protocol() {
    let platform = Platform::new();
    let received = Arc::new(Mutex::new(Vec::new()));
    let route = "capture.stream.protocol";
    let _ = platform.register_with_options(
        route,
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
    let mut out = EventStreamWriter::new(&platform, route, Some("cid-100")).expect("writer");
    out.first_with_ttl(200, "text/event-stream", 30);
    assert!(!out.is_closed());
    out.write("Hello").await.expect("write");
    out.write_named("tokens", serde_json::json!({"n": 2}))
        .await
        .expect("write");
    out.close_with(serde_json::json!({"usage": 42}))
        .await
        .expect("close");
    assert!(out.is_closed());
    out.write("late segment must be dropped")
        .await
        .expect("drop");
    out.fail(&AppError::new(500, "fail after close is a no-op"))
        .await
        .expect("no-op");
    captured_count(&received, 3).await;
    let events = received.lock().expect("capture mutex");
    assert_eq!(events.len(), 3, "segments after close must be dropped");
    let first = &events[0];
    assert_eq!(first.correlation_id(), Some("cid-100"));
    assert_eq!(
        first.header(event_stream::X_EVENT_STREAM),
        Some(event_stream::DATA)
    );
    assert_eq!(first.header("content-type"), Some("text/event-stream"));
    assert_eq!(first.header("x-ttl"), Some("30"));
    assert_eq!(first.status(), 200);
    let second = &events[1];
    assert_eq!(second.header(event_stream::X_EVENT_NAME), Some("tokens"));
    assert_eq!(
        second.header("content-type"),
        None,
        "head control rides the first event only"
    );
    let eof = &events[2];
    assert_eq!(
        eof.header(event_stream::X_EVENT_STREAM),
        Some(event_stream::EOF)
    );
    let metadata: serde_json::Value = eof.body_as().expect("eof body");
    assert_eq!(metadata["usage"], 42);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn fail_maps_exceptions_to_the_in_band_error_contract() {
    let platform = Platform::new();
    let received = Arc::new(Mutex::new(Vec::new()));
    let route = "capture.stream.failure";
    let _ = platform.register_with_options(
        route,
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
    let mut out = EventStreamWriter::new(&platform, route, Some("cid-200")).expect("writer");
    out.fail(&AppError::new(429, "slow down"))
        .await
        .expect("fail");
    assert!(out.is_closed());
    captured_count(&received, 1).await;
    let events = received.lock().expect("capture mutex");
    let error = &events[0];
    assert_eq!(
        error.header(event_stream::X_EVENT_STREAM),
        Some(event_stream::EXCEPTION)
    );
    assert_eq!(error.status(), 429);
    let body: serde_json::Value = error.body_as().expect("error body");
    assert_eq!(body["status"], 429);
    assert_eq!(body["message"], "slow down");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn writer_requires_a_reply_route() {
    let platform = Platform::new();
    assert!(EventStreamWriter::new(&platform, "", Some("cid")).is_err());
    let request_without_reply_to = EventEnvelope::new();
    assert!(EventStreamWriter::from_request(&platform, &request_without_reply_to).is_err());
}
