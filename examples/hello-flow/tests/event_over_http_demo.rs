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

//! End-to-end coverage of the two Event-over-HTTP demo endpoints — the Rust
//! twin of the Java composable-example's `EventOverHttpDemoTest`. The test
//! points `peer.demo.host`/`peer.demo.port` back at this test server, so both
//! patterns make a REAL HTTP hop through `/api/event` and land on the public
//! echo functions registered below — the same wiring as the live demo against
//! the `hello-world` example (or the Java lambda-example), but self-contained
//! in one process. The loopback is legitimate: the declarative map is
//! consulted before local discovery, and the inbound `/api/event` delivery
//! carries the `x-event-api` recursion guard, so the hop still crosses the
//! wire exactly once.
//!
//! One test function on purpose: the app boots ONCE per process (AutoStart is
//! a run-once lifecycle) and the declarative registry is a process-wide
//! one-shot load, so both endpoints are exercised in a single sequential run.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use platform_core::{
    automation, overrides, AppError, AutoStart, ComposableFunction, EventEnvelope, Platform,
};
use rmpv::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// The application under test is a BIN crate (examples are standalone
// applications, not libraries), so its annotated items are not linkable as a
// dependency — include its source so the link-time inventory in this test
// binary carries the app's functions, flows adapter and main application.
#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

/// Stand-in for the hello-world example's public echo (`hello.world` and its
/// alias `hello.declarative`) — body, headers, instance, origin.
struct Echo;

#[async_trait]
impl ComposableFunction for Echo {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let echo_headers = Value::Map(
            headers
                .iter()
                .map(|(k, v)| (Value::from(k.as_str()), Value::from(v.as_str())))
                .collect(),
        );
        Ok(EventEnvelope::new().set_raw_body(Value::Map(vec![
            (Value::from("body"), input.body().clone()),
            (Value::from("headers"), echo_headers),
            (Value::from("instance"), Value::from(instance as u64)),
            (Value::from("origin"), Value::from(Platform::origin())),
        ])))
    }
}

/// Minimal raw HTTP/1.1 POST (no client dependency).
async fn http_post(port: u16, path: &str, body: &str) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let request = format!(
        "POST {path} HTTP/1.1\r\nHost: localhost\r\ncontent-type: application/json\r\n\
         accept: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).await.expect("read");
    let text = String::from_utf8_lossy(&raw).to_string();
    let (head, payload) = text.split_once("\r\n\r\n").unwrap_or((text.as_str(), ""));
    let status: u16 = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap_or_else(|| panic!("status code missing in: {text:?}"));
    (status, payload.to_string())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn both_event_over_http_demo_endpoints_round_trip() {
    let holding = std::env::temp_dir().join(format!("hello-flow-demo-{}", std::process::id()));
    overrides::set("transient.data.store", &holding.display().to_string());
    // an ephemeral port; the peer address is set to the SAME server below
    overrides::set("rest.server.port", "0");
    // boot the whole application: flow compiler, preloaded functions, REST
    AutoStart::main(vec![]).await.expect("app lifecycle");
    let port = automation::server_address().expect("server started").port();
    // loop the demo peer back to this very server — ${peer.demo.port} in
    // event-over-http.yaml resolves when the declarative registry first
    // loads, which happens on the first mapped PostOffice call below
    overrides::set("peer.demo.host", "127.0.0.1");
    overrides::set("peer.demo.port", &port.to_string());
    // the public echo functions the live demo finds in the hello-world app
    let platform = Platform::get_instance();
    platform.register("hello.world", Arc::new(Echo), 5).unwrap();
    platform
        .register("hello.declarative", Arc::new(Echo), 5)
        .unwrap();

    // PROGRAMMATIC pattern: flow task passes the peer URL to the request API
    let (status, body) = http_post(
        port,
        "/api/event/http/programmatic",
        "{\"hello\":\"world\"}",
    )
    .await;
    assert_eq!(status, 200, "programmatic demo body: {body}");
    let response: serde_json::Value = serde_json::from_str(&body).expect("json");
    assert_eq!(response["body"]["hello"], serde_json::json!("world"));
    assert_eq!(response["origin"], serde_json::json!(Platform::origin()));

    // DECLARATIVE pattern: flow task names the foreign route, resolved
    // through event-over-http.yaml — zero code
    let (status, body) =
        http_post(port, "/api/event/http/declarative", "{\"hello\":\"world\"}").await;
    assert_eq!(status, 200, "declarative demo body: {body}");
    let response: serde_json::Value = serde_json::from_str(&body).expect("json");
    assert_eq!(response["body"]["hello"], serde_json::json!("world"));
    assert_eq!(response["origin"], serde_json::json!(Platform::origin()));
    // the engine-internal x-event-api relay guard never reaches a user
    // function's view (metadata contract) — even though the declarative hop
    // rode it on the wire envelope
    assert_eq!(response["headers"]["x-event-api"], serde_json::Value::Null);
    // the read-only my_* metadata is INJECTED into the function's input
    // header copy at delivery — never transported in the event: the injected
    // my_correlation_id equals the edge-resolved business correlation-id
    assert_eq!(
        response["headers"]["my_route"],
        serde_json::json!("hello.declarative")
    );
    assert!(response["headers"]["my_trace_id"].is_string());
    assert!(response["headers"]["my_correlation_id"].is_string());
    assert_eq!(
        response["headers"]["my_correlation_id"], response["headers"]["x-correlation-id"],
        "the injected business cid must match the edge-resolved dataset header"
    );
}
