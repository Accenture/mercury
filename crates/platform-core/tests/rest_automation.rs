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

//! End-to-end tests for increment 6 — REST automation over a real HTTP
//! connection on an ephemeral port (raw HTTP/1.1 via TcpStream: no client
//! dependency). One server per test binary (the routing table and platform
//! are process-wide here), with per-concern routes.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::{
    automation, overrides, resources, AppConfigReader, AppError, ComposableFunction, EventEnvelope,
    Platform, PostOffice,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// ---- test fixtures ----

/// Echoes selected parts of the AsyncHttpRequest event back as JSON.
struct HttpEcho;

#[async_trait]
impl ComposableFunction for HttpEcho {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        let user = request["parameters"]["path"]["user"].clone();
        let q = request["parameters"]["query"]["q"].clone();
        let cid_header = request["headers"][automation::MY_CORRELATION_ID].clone();
        let body = request["body"].clone();
        EventEnvelope::new().set_body(serde_json::json!({
            "user": user,
            "q": q,
            "my_cid": cid_header,
            "echo_body": body,
            "method": request["method"],
        }))
    }
}

/// Records the trace context it executes under, for edge-trace assertions.
struct TraceProbe {
    seen: Arc<Mutex<Option<(Option<String>, Option<String>, Option<String>)>>>,
    platform: Platform,
}

#[async_trait]
impl ComposableFunction for TraceProbe {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let po = PostOffice::new(&self.platform);
        *self.seen.lock().expect("probe mutex") =
            Some((po.my_trace_id(), po.my_trace_path(), po.my_correlation_id()));
        EventEnvelope::new().set_body("traced")
    }
}

/// Returns plain text immediately (text/plain response mapping).
struct PlainText;

#[async_trait]
impl ComposableFunction for PlainText {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new().set_body("plain response")
    }
}

/// Returns plain text (text/plain mapping) after a configurable delay.
struct SlowText;

#[async_trait]
impl ComposableFunction for SlowText {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        tokio::time::sleep(Duration::from_millis(2500)).await;
        EventEnvelope::new().set_body("late")
    }
}

/// Authorizer: allows only when header x-api-key == "open-sesame".
struct ApiKeyCheck;

#[async_trait]
impl ComposableFunction for ApiKeyCheck {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        let allowed = request["headers"]["x-api-key"] == "open-sesame";
        EventEnvelope::new().set_body(allowed)
    }
}

const REST_YAML: &str = r#"
rest:
  - service: "http.echo"
    methods: ['GET', 'POST']
    url: "/api/echo/{user}"
    timeout: 5s
    cors: cors_1
    headers: header_1
  - service: "trace.probe"
    methods: ['GET']
    url: "/api/traced"
    timeout: 5s
    tracing: true
  - service: "plain.text"
    methods: ['GET']
    url: "/api/text"
    timeout: 5s
  - service: "slow.text"
    methods: ['GET']
    url: "/api/slow"
    timeout: 1s
  - service: "http.echo"
    methods: ['GET']
    url: "/api/secure"
    timeout: 5s
    authentication: "api.key.check"
cors:
  - id: cors_1
    options:
      - "Access-Control-Allow-Origin: *"
      - "Access-Control-Allow-Methods: GET, POST, OPTIONS"
    headers:
      - "Access-Control-Allow-Origin: *"
headers:
  - id: header_1
    response:
      add: ["x-served-by: mercury"]
"#;

struct TestServer {
    port: u16,
    platform: Platform,
    trace_seen: Arc<Mutex<Option<(Option<String>, Option<String>, Option<String>)>>>,
}

/// Start a fresh server **inside this test's runtime** — a `#[tokio::test]`
/// gets its own runtime, so a server shared across tests would die with the
/// first test's runtime (its spawned accept-loop and route workers belong to
/// it). Port 0 keeps parallel servers from colliding.
async fn server() -> TestServer {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let holding =
            std::env::temp_dir().join(format!("mercury-rest-test-{}", std::process::id()));
        overrides::set("transient.data.store", &holding.display().to_string());
        // ephemeral port + a dedicated rest.yaml written to a temp file
        let rest_file = std::env::temp_dir().join(format!("rest-{}.yaml", std::process::id()));
        std::fs::write(&rest_file, REST_YAML).unwrap();
        overrides::set(
            "yaml.rest.automation",
            &format!("file:{}", rest_file.display()),
        );
        overrides::set("rest.server.port", "0");
        let _ = AppConfigReader::get_instance();
    });
    let platform = Platform::new();
    let trace_seen = Arc::new(Mutex::new(None));
    platform
        .register("http.echo", Arc::new(HttpEcho), 2)
        .unwrap();
    platform
        .register(
            "trace.probe",
            Arc::new(TraceProbe {
                seen: trace_seen.clone(),
                platform: platform.clone(),
            }),
            1,
        )
        .unwrap();
    platform
        .register("plain.text", Arc::new(PlainText), 1)
        .unwrap();
    platform
        .register("slow.text", Arc::new(SlowText), 1)
        .unwrap();
    platform
        .register("api.key.check", Arc::new(ApiKeyCheck), 1)
        .unwrap();
    let addr = automation::start_http_server(&platform).await.unwrap();
    TestServer {
        port: addr.port(),
        platform,
        trace_seen,
    }
}

/// Raw HTTP/1.1 request over TcpStream; returns (status, headers, body).
async fn http(
    port: u16,
    method: &str,
    path: &str,
    extra_headers: &[(&str, &str)],
    body: &str,
) -> (u16, HashMap<String, String>, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let mut request = format!("{method} {path} HTTP/1.1\r\nHost: localhost\r\n");
    for (name, value) in extra_headers {
        request.push_str(&format!("{name}: {value}\r\n"));
    }
    request.push_str(&format!(
        "Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    ));
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).await.expect("read");
    let text = String::from_utf8_lossy(&raw).to_string();
    let (head, payload) = text.split_once("\r\n\r\n").unwrap_or((text.as_str(), ""));
    let mut lines = head.lines();
    let status: u16 = lines
        .next()
        .and_then(|status_line| status_line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap_or_else(|| panic!("status code missing in: {text:?}"));
    let mut headers = HashMap::new();
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            headers.insert(name.trim().to_lowercase(), value.trim().to_string());
        }
    }
    (status, headers, payload.to_string())
}

// ---- tests ----

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_with_path_query_and_generated_cid() {
    let server = server().await;
    let (status, headers, body) = http(
        server.port,
        "GET",
        "/api/echo/eric?q=hello%20world",
        &[],
        "",
    )
    .await;
    assert_eq!(status, 200);
    assert_eq!(headers["content-type"], "application/json");
    // response header transform + CORS headers applied
    assert_eq!(headers["x-served-by"], "mercury");
    assert_eq!(headers["access-control-allow-origin"], "*");
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["user"], "eric");
    assert_eq!(json["q"], "hello world"); // url-decoded
    assert_eq!(json["method"], "GET");
    // a business correlation-id was generated at the edge and exposed
    assert!(json["my_cid"].as_str().is_some_and(|cid| !cid.is_empty()));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn post_json_body_and_supplied_cid() {
    let server = server().await;
    let (status, _, body) = http(
        server.port,
        "POST",
        "/api/echo/poster",
        &[
            ("Content-Type", "application/json"),
            ("X-Correlation-Id", "order-777"),
        ],
        r#"{"item":"book","qty":2}"#,
    )
    .await;
    assert_eq!(status, 200);
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["echo_body"]["item"], "book");
    assert_eq!(json["echo_body"]["qty"], 2);
    // the caller's business correlation-id is preserved, not regenerated
    assert_eq!(json["my_cid"], "order-777");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unknown_path_is_java_shaped_404() {
    let server = server().await;
    let (status, headers, body) = http(server.port, "GET", "/api/nowhere", &[], "").await;
    assert_eq!(status, 404);
    assert_eq!(headers["content-type"], "application/json");
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["status"], 404);
    assert_eq!(json["type"], "error");
    assert_eq!(json["message"], "Resource not found");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cors_preflight_returns_options_headers() {
    let server = server().await;
    let (status, headers, _) = http(server.port, "OPTIONS", "/api/echo/any", &[], "").await;
    assert_eq!(status, 204);
    assert_eq!(headers["access-control-allow-origin"], "*");
    assert!(headers["access-control-allow-methods"].contains("POST"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn slow_function_times_out_as_408() {
    let server = server().await;
    let (status, _, body) = http(server.port, "GET", "/api/slow", &[], "").await;
    assert_eq!(status, 408, "slow body: {body}");
    assert!(body.contains("error"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn authentication_gate_401_and_pass() {
    let server = server().await;
    let (status, _, body) = http(server.port, "GET", "/api/secure", &[], "").await;
    assert_eq!(status, 401, "missing api key must be rejected: {body}");
    let (status, _, body) = http(
        server.port,
        "GET",
        "/api/secure",
        &[("X-Api-Key", "open-sesame")],
        "",
    )
    .await;
    assert_eq!(status, 200, "secure body: {body}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn edge_starts_trace_and_traceparent_parent_is_adopted() {
    let server = server().await;
    // telemetry capture on the same platform
    let datasets: Arc<Mutex<Vec<serde_json::Value>>> = Arc::new(Mutex::new(Vec::new()));
    struct Capture(Arc<Mutex<Vec<serde_json::Value>>>);
    #[async_trait]
    impl ComposableFunction for Capture {
        async fn handle_event(
            &self,
            _h: HashMap<String, String>,
            input: EventEnvelope,
            _n: usize,
        ) -> Result<EventEnvelope, AppError> {
            if let Ok(dataset) = input.body_as::<serde_json::Value>() {
                self.0.lock().expect("capture mutex").push(dataset);
            }
            Ok(EventEnvelope::new())
        }
    }
    server
        .platform
        .register(
            "distributed.tracing",
            Arc::new(Capture(datasets.clone())),
            1,
        )
        .unwrap();
    // a W3C traceparent: its trace id wins; its parent-id becomes our parent span
    let upstream_trace = "4bf92f3577b34da6a3ce929d0e0e4736";
    let upstream_span = "00f067aa0ba902b7";
    let traceparent = format!("00-{upstream_trace}-{upstream_span}-01");
    let (status, _, _) = http(
        server.port,
        "GET",
        "/api/traced",
        &[
            ("traceparent", traceparent.as_str()),
            ("X-Correlation-Id", "biz-9"),
        ],
        "",
    )
    .await;
    assert_eq!(status, 200);
    // the probe executed inside the edge-started trace
    let seen = server
        .trace_seen
        .lock()
        .unwrap()
        .clone()
        .expect("probe ran");
    assert_eq!(seen.0.as_deref(), Some(upstream_trace));
    assert_eq!(seen.1.as_deref(), Some("GET /api/traced"));
    assert_eq!(seen.2.as_deref(), Some("biz-9"));
    // and the telemetry span adopted the caller's span as parent
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    loop {
        let all = datasets.lock().unwrap().clone();
        if let Some(dataset) = all.iter().find(|d| d["trace"]["service"] == "trace.probe") {
            assert_eq!(dataset["trace"]["id"], serde_json::json!(upstream_trace));
            assert_eq!(
                dataset["trace"]["parent_span_id"],
                serde_json::json!(upstream_span)
            );
            assert_eq!(
                dataset["trace"]["path"],
                serde_json::json!("GET /api/traced")
            );
            assert_eq!(dataset["trace"]["from"], serde_json::json!("http.request"));
            break;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "telemetry span not seen"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn text_response_maps_to_text_plain() {
    let server = server().await;
    let (status, headers, body) = http(server.port, "GET", "/api/text", &[], "").await;
    assert_eq!(status, 200);
    assert_eq!(headers["content-type"], "text/plain");
    assert_eq!(body, "plain response");
}
