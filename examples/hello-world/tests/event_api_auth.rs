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

//! End-to-end coverage of the Event-over-HTTP authentication demo — the Rust
//! twin of the Java lambda-example's `EventApiAuthTest`: `rest.yaml`
//! overrides the default `/api/event` endpoint with the `event.api.auth`
//! service, which validates the caller's `authorization` header against the
//! shared demo token (`demo.peer.token`, resolved from `DEMO_PEER_TOKEN`).
//!
//! One test function on purpose: the app boots ONCE per process (AutoStart is
//! a run-once lifecycle), so the accept / wrong-token / missing-token cases
//! run sequentially against the same server.

use std::collections::HashMap;
use std::time::Duration;

use platform_core::automation::{self, event_over_http, event_over_http_with_headers};
use platform_core::{overrides, AutoStart, EventEnvelope, Platform, PostOffice};

// The application under test is a BIN crate — include its source so the
// link-time inventory in this test binary carries the app's functions
// (the echo, hello.pojo and event.api.auth).
#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn event_api_auth_accepts_shared_token_and_rejects_others() {
    let holding = std::env::temp_dir().join(format!("hello-world-auth-{}", std::process::id()));
    overrides::set("transient.data.store", &holding.display().to_string());
    overrides::set("rest.server.port", "0");
    AutoStart::main(vec![]).await.expect("app lifecycle");
    let port = automation::server_address().expect("server started").port();
    let endpoint = format!("http://127.0.0.1:{port}/api/event");
    let po = PostOffice::new(&Platform::get_instance());

    // 1. ACCEPT: the shared demo token passes authentication, and the session
    // info injected by event.api.auth (user: demo) rides to the target
    // function as a read-only header — visible in the echo
    let mut security_headers = HashMap::new();
    security_headers.insert("authorization".to_string(), "demo".to_string());
    let request = EventEnvelope::new()
        .set_to("hello.world")
        .set_body(serde_json::json!({"hello": "auth"}))
        .expect("body");
    let response = event_over_http_with_headers(
        &po,
        &endpoint,
        request,
        Duration::from_secs(8),
        true,
        &security_headers,
    )
    .await
    .expect("authorized call");
    assert_eq!(response.status(), 200);
    let echo: serde_json::Value = response.body_as().expect("echo json");
    assert_eq!(echo["body"]["hello"], serde_json::json!("auth"));
    assert_eq!(
        echo["headers"]["user"],
        serde_json::json!("demo"),
        "session info injected by event.api.auth must reach the target"
    );

    // 2. WRONG TOKEN: rejected with HTTP-401 before reaching the target
    let mut wrong = HashMap::new();
    wrong.insert("authorization".to_string(), "let-me-in".to_string());
    let request = EventEnvelope::new()
        .set_to("hello.world")
        .set_body("x")
        .expect("body");
    let response = event_over_http_with_headers(
        &po,
        &endpoint,
        request,
        Duration::from_secs(8),
        true,
        &wrong,
    )
    .await
    .expect("transport ok");
    assert_eq!(response.status(), 401, "wrong token must be rejected");

    // 3. MISSING TOKEN: rejected with HTTP-401
    let request = EventEnvelope::new()
        .set_to("hello.world")
        .set_body("x")
        .expect("body");
    let response = event_over_http(&po, &endpoint, request, Duration::from_secs(8), true)
        .await
        .expect("transport ok");
    assert_eq!(response.status(), 401, "missing token must be rejected");
}
