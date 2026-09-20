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

//! A cache failure surfaces as the failure on every layer — never as a miss.
//! Found in the live Java ⇄ Rust interop drive (2026-09-19): with Redis down,
//! Layer 1 answered 404 *Profile not found*, because it read the error reply's
//! string body as "no bytes" (and a POST would have acknowledged `stored`).
//! Layers 2 and 3 were already safe — the flow and graph engines check a
//! task's status for the author. Here the real `v1.cache.redis` is switched
//! off (`redis.cache.enabled=false`) and a stub that fails fast takes its
//! route, so the assertion depends neither on a Redis outage nor on which of
//! two timeouts fires first.

use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::{
    automation, overrides, preload, AppError, AutoStart, ComposableFunction, EventEnvelope,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// the application's functions and entry point ride into this test binary
#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

const OUTAGE: &str = "Redis unavailable - connection refused";

/// Stands in for `v1.cache.redis` while the real function is switched off:
/// every action fails the way the cache fails when Redis refuses connections.
#[preload(route = "v1.cache.redis", instances = 1)]
struct FailingCache;

#[async_trait]
impl ComposableFunction for FailingCache {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Err(AppError::new(503, OUTAGE))
    }
}

/// Minimal raw HTTP/1.1 call (no client dependency): status + body text.
async fn http(port: u16, method: &str, path: &str, body: Option<&str>) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let payload = body.unwrap_or("");
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: localhost\r\ncontent-type: application/json\r\n\
         accept: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{payload}",
        payload.len()
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

fn json(payload: &str) -> serde_json::Value {
    serde_json::from_str(payload).unwrap_or_else(|e| panic!("not json ({e}): {payload}"))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_cache_failure_is_never_a_miss() {
    // the real cache function (and its health check) stay unregistered; the
    // stub above owns the route
    overrides::set("redis.cache.enabled", "false");
    overrides::set("rest.server.port", "0");
    AutoStart::main(vec![]).await.expect("app lifecycle");
    let port = automation::server_address().expect("server started").port();

    // Layer 1 drives the cache in code, so it must check the reply itself:
    // GET is not a miss, POST is not "stored", DELETE is not "nothing removed"
    let profile = r#"{"name": "Alice", "email": "alice@example.com"}"#;
    for (method, body) in [("GET", None), ("POST", Some(profile)), ("DELETE", None)] {
        let (status, text) = http(port, method, "/api/l1/profile/alice", body).await;
        assert_eq!(503, status, "Layer 1 {method} surfaces the failure: {text}");
        let reply = json(&text);
        assert_eq!(OUTAGE, reply["message"], "Layer 1 {method}: {text}");
        assert_eq!("error", reply["type"], "Layer 1 {method}: {text}");
    }

    // Layers 2 and 3: the flow and the graph propagate the task's failure
    // through the exception handler — the engine checks the status for the
    // author, which is why they never had the gap
    let (status, text) = http(port, "GET", "/api/l2/profile/alice", None).await;
    assert_eq!(503, status, "Layer 2 surfaces the failure: {text}");
    assert_eq!(OUTAGE, json(&text)["message"], "Layer 2: {text}");
    let (status, text) = http(
        port,
        "POST",
        "/api/graph/profile-cache",
        Some(r#"{"action": "get", "id": "alice"}"#),
    )
    .await;
    assert_eq!(503, status, "Layer 3 surfaces the failure: {text}");
    assert_eq!(OUTAGE, json(&text)["message"], "Layer 3: {text}");
}
