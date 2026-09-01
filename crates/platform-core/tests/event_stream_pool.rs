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

//! Reply-lane pool behavior for HTTP response streaming: checkout/release
//! balance, rotation reuse, and deterministic HTTP-503 back-pressure when the
//! pool is exhausted. These tests manipulate the process-wide pool, so they
//! live in their own test binary and serialize among themselves through a
//! shared guard.

use std::collections::HashMap;
use std::sync::{Arc, Once, OnceLock};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::platform::FunctionOptions;
use platform_core::{
    automation, overrides, resources, AppConfigReader, AppError, ComposableFunction, EventEnvelope,
    EventStreamWriter, Platform,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const REST_YAML: &str = r#"
rest:
  - service: "hello.pool.stream"
    methods: ['GET']
    url: "/api/pool/stream"
    timeout: 15s
    stream: true
"#;

/// Streams two text segments and closes.
struct ChunkStream {
    platform: Platform,
}

#[async_trait]
impl ComposableFunction for ChunkStream {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let mut out = EventStreamWriter::from_request(&self.platform, &input)?;
        out.first(200, "text/plain");
        out.write("alpha").await?;
        out.write("beta").await?;
        out.close().await?;
        Ok(EventEnvelope::new())
    }
}

/// The pool is process-wide state - tests in this binary run one at a time.
fn pool_guard() -> &'static tokio::sync::Mutex<()> {
    static GUARD: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| tokio::sync::Mutex::new(()))
}

async fn server() -> u16 {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let rest_file = std::env::temp_dir().join(format!("rest-pool-{}.yaml", std::process::id()));
        std::fs::write(&rest_file, REST_YAML).unwrap();
        overrides::set(
            "yaml.rest.automation",
            &format!("file:{}", rest_file.display()),
        );
        overrides::set("rest.server.port", "0");
        let _ = AppConfigReader::get_instance();
    });
    let platform = Platform::new();
    if !platform.has_route("hello.pool.stream") {
        let _ = platform.register_with_options(
            "hello.pool.stream",
            Arc::new(ChunkStream {
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

async fn http_get(port: u16, path: &str) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: localhost\r\nAccept: text/plain\r\nConnection: close\r\n\r\n"
    );
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut raw = Vec::new();
    tokio::time::timeout(Duration::from_secs(20), stream.read_to_end(&mut raw))
        .await
        .expect("read timed out")
        .expect("read");
    let text = String::from_utf8_lossy(&raw).to_string();
    let status: u16 = text
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap_or(0);
    (status, text)
}

/// Wait briefly for in-flight releases to settle back into the pool.
async fn settle_to(expected: usize) -> usize {
    for _ in 0..50 {
        if automation::available_lanes() >= expected {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    automation::available_lanes()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pool_exhaustion_rejects_with_http_503_and_recovers() {
    let port = server().await;
    let _guard = pool_guard().lock().await;
    // drain the reply-lane pool: a streaming endpoint without an available
    // lane is rejected immediately with HTTP-503 (deterministic back-pressure)
    let mut drained = Vec::new();
    while let Some(lane) = automation::checkout_lane() {
        drained.push(lane);
    }
    assert!(!drained.is_empty(), "the pool should have lanes to drain");
    let (status, text) = http_get(port, "/api/pool/stream").await;
    for lane in drained {
        automation::release_lane(lane);
    }
    assert_eq!(status, 503);
    assert!(text.contains("Streaming response pool exhausted"), "{text}");
    // capacity restored - the same endpoint streams normally again
    let (status, text) = http_get(port, "/api/pool/stream").await;
    assert_eq!(status, 200);
    assert!(text.contains("alpha"), "{text}");
    assert!(text.contains("beta"), "{text}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_completed_stream_returns_its_lane_to_the_pool() {
    let port = server().await;
    let _guard = pool_guard().lock().await;
    let before = automation::available_lanes();
    assert!(before > 0, "lanes should be available before the request");
    let (status, text) = http_get(port, "/api/pool/stream").await;
    assert_eq!(status, 200);
    assert!(text.contains("alpha"), "{text}");
    // the lane is released when the stream completes; allow a brief settle
    assert_eq!(
        settle_to(before).await,
        before,
        "checkout/release must balance"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn lane_checkout_rotates_through_the_pool() {
    let _port = server().await;
    let _guard = pool_guard().lock().await;
    // the pool is a rotating FIFO queue: a released lane rejoins at the tail,
    // so consecutive requests take successive lanes (round-robin) and a
    // just-released lane gets the longest possible rest before reuse
    let first = automation::checkout_lane().expect("a lane");
    automation::release_lane(first.clone());
    let second = automation::checkout_lane().expect("another lane");
    assert_ne!(
        first, second,
        "a released lane must go to the tail, not be reused immediately"
    );
    automation::release_lane(second);
}
