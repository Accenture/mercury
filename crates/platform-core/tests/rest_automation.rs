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
use platform_core::automation::http_client::AsyncHttpClientService;
use platform_core::platform::FunctionOptions;
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
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        let user = request["parameters"]["path"]["user"].clone();
        let q = request["parameters"]["query"]["q"].clone();
        // the read-only my_correlation_id key is INJECTED into the function's
        // input header copy at delivery (metadata contract) — it is no longer
        // a dataset header
        let cid_header = headers
            .get(automation::MY_CORRELATION_ID)
            .cloned()
            .map(serde_json::Value::String)
            .unwrap_or(serde_json::Value::Null);
        let body = request["body"].clone();
        EventEnvelope::new().set_body(serde_json::json!({
            "user": user,
            "q": q,
            "my_cid": cid_header,
            "echo_body": body,
            "method": request["method"],
            "flow_id": request["headers"]["x-flow-id"],
        }))
    }
}

/// The (trace id, trace path, correlation id) triple a probe observed.
type TraceContext = Arc<Mutex<Option<(Option<String>, Option<String>, Option<String>)>>>;

/// Records the trace context it executes under, for edge-trace assertions.
struct TraceProbe {
    seen: TraceContext,
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

/// Reports the wire *kind* of the request body it receives (map, list, text,
/// bytes, null) plus its content — proves the content-type dispatch of the
/// HTTP boundary end to end, including the MsgPack-binary body an unknown
/// content type produces (which a JSON view cannot represent).
struct BodyProbe;

#[async_trait]
impl ComposableFunction for BodyProbe {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let mut kind = "missing";
        let mut content = serde_json::Value::Null;
        let mut query = serde_json::Value::Null;
        if let rmpv::Value::Map(entries) = input.body() {
            for (key, value) in entries {
                match key.as_str() {
                    Some("body") => {
                        kind = match value {
                            rmpv::Value::Nil => "null",
                            rmpv::Value::String(_) => "text",
                            rmpv::Value::Binary(_) => "bytes",
                            rmpv::Value::Map(_) => "map",
                            rmpv::Value::Array(_) => "list",
                            _ => "other",
                        };
                        content = match value {
                            rmpv::Value::Binary(bytes) => serde_json::Value::String(
                                String::from_utf8_lossy(bytes).to_string(),
                            ),
                            other => serde_json::to_value(other).unwrap_or_default(),
                        };
                    }
                    Some("parameters") => {
                        let params: serde_json::Value =
                            serde_json::to_value(value).unwrap_or_default();
                        query = params["query"].clone();
                    }
                    _ => {}
                }
            }
        }
        EventEnvelope::new().set_body(serde_json::json!({
            "kind": kind,
            "content": content,
            "query": query,
        }))
    }
}

/// Increment 56: exposes the Java request-model keys — repeated query
/// values, the parsed cookies map, the raw query string, and the https flag.
struct RequestView;

#[async_trait]
impl ComposableFunction for RequestView {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        EventEnvelope::new().set_body(serde_json::json!({
            "q": request["parameters"]["query"]["q"],
            "single": request["parameters"]["query"]["single"],
            "cookie_first": request["cookies"]["first"],
            "cookie_second": request["cookies"]["second"],
            "cookie_header": request["headers"]["cookie"],
            "query": request["query"],
            "https": request["https"],
        }))
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

/// Increment 50: a function that decorates its response envelope — the
/// headers must survive the REST boundary exactly as in Java
/// `AsyncHttpResponse.updateHeaders` (redirect Location, repeated
/// Set-Cookie, content-type override; stream metadata withheld).
struct HeaderProbe;

#[async_trait]
impl ComposableFunction for HeaderProbe {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new()
            .set_status(302)
            .set_header("Location", "/somewhere/else")
            .set_header("Content-Type", "application/vnd.demo+json")
            .set_header("Set-Cookie", "first=1; Path=/|second=2; HttpOnly")
            .set_header("X-Custom", "custom-value")
            .set_header("x-stream-id", "stream.100.in")
            .set_header("x-ttl", "10")
            .set_body(serde_json::json!({"moved": true}))
    }
}

/// Reflects the raw HTTP headers the request arrived with, so a test can see
/// exactly what an upstream caller stamped on the wire.
struct TraceHeaderEcho;

#[async_trait]
impl ComposableFunction for TraceHeaderEcho {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        EventEnvelope::new().set_body(serde_json::json!({
            "headers": request["headers"],
        }))
    }
}

/// A traced function that calls the "/api/echo/headers" echo endpoint through
/// the "async.http.request" HTTP client and returns the echo's view of the
/// request it received. Because the echo reflects the raw HTTP headers, a
/// test can assert exactly what the async HTTP client stamped on the outgoing
/// call - e.g. that the W3C trace context travels under BOTH the standard
/// "traceparent" and a custom http.traceparent.header name.
struct EchoChainCaller {
    platform: Platform,
}

#[async_trait]
impl ComposableFunction for EchoChainCaller {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        let port = request["parameters"]["query"]["port"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let echo = automation::AsyncHttpRequest::new()
            .set_method("GET")
            .set_url("/api/echo/headers")
            .set_target_host(&format!("http://127.0.0.1:{port}"))
            .set_header("accept", "application/json");
        let po = PostOffice::new(&self.platform);
        let response = po
            .request(
                EventEnvelope::new()
                    .set_to(automation::ASYNC_HTTP_REQUEST)
                    .set_raw_body(echo.to_value()),
                Duration::from_secs(10),
            )
            .await?;
        Ok(EventEnvelope::new().set_raw_body(response.body().clone()))
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
  - service: "header.probe"
    methods: ['GET', 'HEAD']
    url: "/api/headers"
    timeout: 5s
    headers: header_1
  - service: "request.view"
    methods: ['GET']
    url: "/api/request/view"
    timeout: 5s
  - service: "plain.text"
    methods: ['GET']
    url: "/api/files/*"
    timeout: 5s
  - service: "plain.text"
    methods: ['GET']
    url: "/api/v*/ping"
    timeout: 5s
  - service: "plain.text"
    methods: ['GET']
    url: "/api/mid/*/end"
    timeout: 5s
  - service: "trace.probe"
    methods: ['GET']
    url: "/api/traced"
    timeout: 5s
    tracing: true
  # Per-endpoint traceparent header-name override: this endpoint reads the W3C trace context
  # from 'X-Endpoint-Trace', taking precedence over the global http.traceparent.header
  # (X-Trace-Context in this test suite) and the standard 'traceparent' (which remains a
  # fallback when absent).
  - service: "trace.probe"
    methods: ['GET']
    url: "/api/renamed/traceparent/probe"
    timeout: 5s
    tracing: true
    traceparent.header: "X-Endpoint-Trace"
  # A traced caller whose downstream target is the /api/echo/headers echo, so tests can
  # inspect the raw headers the async HTTP client stamped on the outgoing hop (e.g. the
  # custom traceparent name).
  - service: "echo.chain.caller"
    methods: ['GET']
    url: "/api/echo/chain"
    timeout: 15s
    tracing: true
  - service: "trace.header.echo"
    methods: ['GET']
    url: "/api/echo/headers"
    timeout: 5s
  - service: "plain.text"
    methods: ['GET']
    url: "/api/text"
    timeout: 5s
  - service: "body.probe"
    methods: ['POST']
    url: "/api/probe"
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
  - service: "http.echo"
    methods: ['GET']
    url: "/api/flow/demo"
    flow: 'demo-flow'
    timeout: 5s
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
    trace_seen: TraceContext,
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
        .register("body.probe", Arc::new(BodyProbe), 1)
        .unwrap();
    platform
        .register("slow.text", Arc::new(SlowText), 1)
        .unwrap();
    platform
        .register("api.key.check", Arc::new(ApiKeyCheck), 1)
        .unwrap();
    platform
        .register("header.probe", Arc::new(HeaderProbe), 1)
        .unwrap();
    platform
        .register("request.view", Arc::new(RequestView), 1)
        .unwrap();
    platform
        .register("trace.header.echo", Arc::new(TraceHeaderEcho), 1)
        .unwrap();
    platform
        .register(
            "echo.chain.caller",
            Arc::new(EchoChainCaller {
                platform: platform.clone(),
            }),
            1,
        )
        .unwrap();
    // the async HTTP client, for the echo-chain caller's outgoing hop
    platform
        .register_with_options(
            automation::ASYNC_HTTP_REQUEST,
            Arc::new(AsyncHttpClientService::new(&platform)),
            10,
            FunctionOptions {
                zero_traced: false,
                interceptor: true,
                private: true,
            },
        )
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

/// Raw HTTP exchange keeping the header block verbatim — repeated headers
/// (Set-Cookie) stay visible as separate lines, unlike the map in `http`.
async fn http_raw(port: u16, method: &str, path: &str) -> (u16, String, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let request =
        format!("{method} {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
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
        .expect("status code");
    (status, head.to_lowercase(), payload.to_string())
}

// ---- tests ----

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn function_response_headers_survive_the_rest_boundary() {
    let server = server().await;
    let (status, head, body) = http_raw(server.port, "GET", "/api/headers").await;
    // envelope status + Location = a working redirect (Java parity)
    assert_eq!(status, 302);
    assert!(head.contains("location: /somewhere/else"), "{head}");
    // function-set content type overrides the body-derived one
    assert!(
        head.contains("content-type: application/vnd.demo+json"),
        "{head}"
    );
    // custom header preserved
    assert!(head.contains("x-custom: custom-value"), "{head}");
    // "|"-separated cookies become one Set-Cookie line each
    assert!(head.contains("set-cookie: first=1; path=/"), "{head}");
    assert!(head.contains("set-cookie: second=2; httponly"), "{head}");
    // the rest.yaml response transform still applies alongside
    assert!(head.contains("x-served-by: mercury"), "{head}");
    // stream metadata is recognized and withheld from the wire (deferral)
    assert!(!head.contains("x-stream-id"), "{head}");
    assert!(!head.contains("x-ttl"), "{head}");
    // the body still rides normally on GET
    assert!(body.contains("moved"), "{body}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn head_response_carries_headers_but_no_body() {
    let server = server().await;
    let (status, head, body) = http_raw(server.port, "HEAD", "/api/headers").await;
    assert_eq!(status, 302);
    assert!(head.contains("location: /somewhere/else"), "{head}");
    // HEAD: the function's content-type override is skipped (Java isHead)
    assert!(
        !head.contains("content-type: application/vnd.demo+json"),
        "{head}"
    );
    assert!(body.is_empty(), "HEAD must not carry a body: {body:?}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_with_path_query_and_generated_cid() {
    let server = server().await;
    let (status, headers, body) = http(
        server.port,
        "GET",
        "/api/echo/eric?q=hello%20world",
        &[("Accept", "application/json")],
        "",
    )
    .await;
    assert_eq!(status, 200);
    // increment 56: the fallback content type is negotiated from Accept
    // (Java updateContentType), no longer derived from the body shape
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
async fn response_echoes_inbound_correlation_id() {
    // the edge echoes the request's business correlation-id on the HTTP
    // response so the caller can correlate without parsing the body
    // (Java RestEndpointTest.responseEchoesInboundCorrelationId twin)
    let server = server().await;
    let (status, headers, _) = http(
        server.port,
        "GET",
        "/api/echo/eric?q=hi",
        &[("X-Correlation-Id", "cid-echo-0101")],
        "",
    )
    .await;
    assert_eq!(status, 200);
    assert_eq!(headers["x-correlation-id"], "cid-echo-0101");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn response_carries_generated_correlation_id_when_absent() {
    // when the caller does not provide one, the edge generates a
    // correlation-id and still returns it on the response — the SAME value
    // the function saw as its injected my_correlation_id
    // (Java RestEndpointTest.responseCarriesGeneratedCorrelationIdWhenAbsent twin)
    let server = server().await;
    let (status, headers, body) = http(server.port, "GET", "/api/echo/eric?q=hi", &[], "").await;
    assert_eq!(status, 200);
    let echoed = headers
        .get("x-correlation-id")
        .expect("the generated correlation-id must be echoed on the response");
    assert!(!echoed.is_empty());
    // end-to-end identity: response header == the injected my_correlation_id
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["my_cid"], serde_json::json!(echoed.as_str()));
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

/// The request-body dispatch mirrors Java `HttpRouter.handlePayload` exactly:
/// no JSON sniffing outside `application/json`, no default content type, and
/// each content type delivers the same body KIND the Java engine delivers.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn body_dispatch_mirrors_java_content_type_rules() {
    let server = server().await;
    let probe = |headers: Vec<(&'static str, &'static str)>, body: &'static str| {
        let port = server.port;
        async move {
            let (status, _, text) = http(port, "POST", "/api/probe", &headers, body).await;
            assert_eq!(status, 200);
            serde_json::from_str::<serde_json::Value>(&text).unwrap()
        }
    };
    // application/json with a JSON object -> map
    let json = probe(
        vec![("Content-Type", "application/json")],
        r#"{"item":"book"}"#,
    )
    .await;
    assert_eq!(json["kind"], "map");
    assert_eq!(json["content"]["item"], "book");
    // plain text mislabeled application/json -> the raw text, not an error
    let mislabeled = probe(
        vec![("Content-Type", "application/json")],
        "import graph from tutorial-1",
    )
    .await;
    assert_eq!(mislabeled["kind"], "text");
    assert_eq!(mislabeled["content"], "import graph from tutorial-1");
    // an empty application/json body -> an empty map (Java parity)
    let empty = probe(vec![("Content-Type", "application/json")], "").await;
    assert_eq!(empty["kind"], "map");
    assert_eq!(empty["content"], serde_json::json!({}));
    // a JSON-looking body under text/plain stays text - no sniffing
    let unsniffed = probe(vec![("Content-Type", "text/plain")], r#"{"item":"book"}"#).await;
    assert_eq!(unsniffed["kind"], "text");
    assert_eq!(unsniffed["content"], r#"{"item":"book"}"#);
    // form fields decode into query parameters; the null body key is
    // STRIPPED on the bus hop (increment 58, the F2 normalization — Java's
    // always-serialize wire drops it too, and AsyncHttpRequest.getBody()
    // reads absent-as-null, so the two are indistinguishable in Java)
    let form = probe(
        vec![("Content-Type", "application/x-www-form-urlencoded")],
        "a=1&b=hello+world",
    )
    .await;
    assert_eq!(form["kind"], "missing");
    assert_eq!(form["query"]["a"], "1");
    assert_eq!(form["query"]["b"], "hello world");
    // no content type -> raw bytes (Java handleBinaryContent), never sniffed
    let bytes = probe(vec![], r#"{"item":"book"}"#).await;
    assert_eq!(bytes["kind"], "bytes");
    assert_eq!(bytes["content"], r#"{"item":"book"}"#);
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
        "/api/traced?flag=on",
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
    // Java appends the query string to the trace path (increment 56)
    assert_eq!(seen.1.as_deref(), Some("GET /api/traced?flag=on"));
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
                serde_json::json!("GET /api/traced?flag=on")
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
    // increment 56 (Java updateContentType): text/plain is the negotiated
    // fallback for an Accept that is not html/xml/json/any
    let (status, headers, body) = http(
        server.port,
        "GET",
        "/api/text",
        &[("Accept", "text/plain")],
        "",
    )
    .await;
    assert_eq!(status, 200);
    assert_eq!(headers["content-type"], "text/plain");
    assert_eq!(body, "plain response");
}

/// Increment 56 — the Java content negotiation (updateContentType +
/// handleMapContent): the fallback type comes from Accept, `*/*` negotiates
/// JSON even for a text body, text/html wraps a map body in an HTML shell,
/// and NO Accept means NO content-type header at all.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn response_content_negotiation_matches_java() {
    let server = server().await;
    // */* on a TEXT body: content-type application/json, body raw (Java quirk)
    let (_, headers, body) = http(server.port, "GET", "/api/text", &[("Accept", "*/*")], "").await;
    assert_eq!(headers["content-type"], "application/json");
    assert_eq!(body, "plain response");
    // text/html on a MAP body: HTML-wrapped JSON
    let (_, headers, body) = http(
        server.port,
        "GET",
        "/api/echo/html?q=1",
        &[("Accept", "text/html")],
        "",
    )
    .await;
    assert_eq!(headers["content-type"], "text/html");
    assert!(
        body.starts_with("<html><body><pre>\n") && body.ends_with("\n</pre></body></html>"),
        "{body}"
    );
    // NO Accept: no content-type header at all (Java's "?" marker)
    let (_, head, _) = http_raw(server.port, "GET", "/api/text").await;
    assert!(!head.contains("content-type"), "{head}");
}

/// Increment 56 (parity F14a/d) — the request model: repeated query values
/// become a list, cookies arrive as a parsed map (the raw header withheld),
/// the raw query string and https flag ride the Java keys, and the trace
/// path carries the query string.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn request_model_matches_java_keys() {
    let server = server().await;
    let (status, _, body) = http(
        server.port,
        "GET",
        "/api/request/view?q=a&q=b&single=1",
        &[
            ("Accept", "application/json"),
            ("Cookie", "first=alpha; second=beta"),
            ("X-Forwarded-Proto", "https"),
        ],
        "",
    )
    .await;
    assert_eq!(status, 200);
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    // repeated query parameter -> list; single stays a string (Java getAll)
    assert_eq!(json["q"], serde_json::json!(["a", "b"]));
    assert_eq!(json["single"], "1");
    // cookies parsed into a map; the raw cookie header withheld
    assert_eq!(json["cookie_first"], "alpha");
    assert_eq!(json["cookie_second"], "beta");
    assert_eq!(json["cookie_header"], serde_json::Value::Null);
    // Java top-level keys: raw query string + https from x-forwarded-proto
    assert_eq!(json["query"], "q=a&q=b&single=1");
    assert_eq!(json["https"], true);
}

/// Increment 56 (parity F14b) — the full Java wildcard grammar: mid-path `*`
/// (one segment), segment prefix `v*`, and the trailing `*` that no longer
/// matches an EMPTY remainder.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn wildcard_grammar_matches_java() {
    let server = server().await;
    let ok = |status: u16| assert_eq!(status, 200);
    // trailing * requires at least one remaining segment (Java matchRoute)
    let (status, _, _) = http(
        server.port,
        "GET",
        "/api/files/a/b",
        &[("Accept", "*/*")],
        "",
    )
    .await;
    ok(status);
    let (status, _, _) = http(server.port, "GET", "/api/files", &[("Accept", "*/*")], "").await;
    assert_eq!(status, 404, "trailing * must NOT match an empty remainder");
    // segment prefix wildcard v*
    let (status, _, _) = http(server.port, "GET", "/api/v2/ping", &[("Accept", "*/*")], "").await;
    ok(status);
    // mid-path bare * matches exactly one segment
    let (status, _, _) = http(
        server.port,
        "GET",
        "/api/mid/x/end",
        &[("Accept", "*/*")],
        "",
    )
    .await;
    ok(status);
    let (status, _, _) = http(server.port, "GET", "/api/mid/end", &[("Accept", "*/*")], "").await;
    assert_eq!(status, 404, "mid-path * consumes exactly one segment");
}

/// Increment 56 (parity F14c) — a known path under a wrong method is 405,
/// and OPTIONS without a CORS block is 405, never a bare 204.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn known_path_wrong_method_is_405() {
    let server = server().await;
    let (status, _, body) = http(server.port, "PUT", "/api/text", &[("Accept", "*/*")], "").await;
    assert_eq!(status, 405);
    assert!(body.contains("Method not allowed"), "{body}");
    // OPTIONS on a route WITH cors options: 204 + the options headers
    let (status, headers, _) = http(server.port, "OPTIONS", "/api/echo/x", &[], "").await;
    assert_eq!(status, 204);
    assert_eq!(headers["access-control-allow-origin"], "*");
    // OPTIONS on a route WITHOUT cors: 405 (Java handleOptionsMethod)
    let (status, _, _) = http(server.port, "OPTIONS", "/api/text", &[], "").await;
    assert_eq!(status, 405);
}

/// Increment E-3: a rest.yaml `flow:` binding injects the x-flow-id request
/// header the event-script flow adapter reads (Java parity).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn flow_binding_injects_x_flow_id_header() {
    let ts = server().await;
    let (status, _, body) = http(ts.port, "GET", "/api/flow/demo", &[], "").await;
    assert_eq!(status, 200);
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["flow_id"], "demo-flow");
    // endpoints without a flow binding carry no x-flow-id
    let (_, _, body) = http(ts.port, "GET", "/api/echo/alice", &[], "").await;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert!(json["flow_id"].is_null());
}

// ---- configurable traceparent header name (http.traceparent.header) ----
//
// The suite-wide config (tests/resources/application.properties) sets
// http.traceparent.header=X-Trace-Context, so EVERY test in this binary runs
// with the feature active - the untouched tests above prove it is additive.

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn custom_traceparent_header_name_carries_the_trace_context() {
    // http.traceparent.header=X-Trace-Context in this test suite: an intermediary that strips
    // the standard W3C "traceparent" can still deliver the trace context under the custom name
    let server = server().await;
    let w3c_trace_id = "1af92f3577b34da6a3ce929d0e0e4701";
    let value = format!("00-{w3c_trace_id}-00f067aa0ba902b7-01");
    let (status, _, _) = http(
        server.port,
        "GET",
        "/api/traced",
        &[("X-Trace-Context", value.as_str())],
        "",
    )
    .await;
    assert_eq!(status, 200);
    let seen = server
        .trace_seen
        .lock()
        .unwrap()
        .clone()
        .expect("probe ran");
    assert_eq!(
        seen.0.as_deref(),
        Some(w3c_trace_id),
        "the endpoint adopted the W3C trace context carried under the custom header name"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn custom_traceparent_name_wins_over_injected_standard_header() {
    // an intermediary (e.g. a service-mesh sidecar) may inject its OWN standard traceparent;
    // the peer's context under the deliberately configured custom name must not be overridden
    let server = server().await;
    let peer_trace_id = "2af92f3577b34da6a3ce929d0e0e4702";
    let injected_trace_id = "3af92f3577b34da6a3ce929d0e0e4703";
    let peer = format!("00-{peer_trace_id}-00f067aa0ba902b7-01");
    let injected = format!("00-{injected_trace_id}-00f067aa0ba902b8-01");
    let (status, _, _) = http(
        server.port,
        "GET",
        "/api/traced",
        &[
            ("X-Trace-Context", peer.as_str()),
            ("traceparent", injected.as_str()),
        ],
        "",
    )
    .await;
    assert_eq!(status, 200);
    let seen = server
        .trace_seen
        .lock()
        .unwrap()
        .clone()
        .expect("probe ran");
    assert_eq!(
        seen.0.as_deref(),
        Some(peer_trace_id),
        "a well-formed value under the custom traceparent name wins over the standard header"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standard_traceparent_remains_fallback_under_custom_name() {
    // a standards-compliant caller that only sends the standard header still propagates,
    // even though this application is configured with a custom traceparent name
    let server = server().await;
    let w3c_trace_id = "4af92f3577b34da6a3ce929d0e0e4704";
    let value = format!("00-{w3c_trace_id}-00f067aa0ba902b7-01");
    let (status, _, _) = http(
        server.port,
        "GET",
        "/api/traced",
        &[("traceparent", value.as_str())],
        "",
    )
    .await;
    assert_eq!(status, 200);
    let seen = server
        .trace_seen
        .lock()
        .unwrap()
        .clone()
        .expect("probe ran");
    assert_eq!(seen.0.as_deref(), Some(w3c_trace_id));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn per_endpoint_traceparent_override_beats_global_name() {
    // /api/renamed/traceparent/probe declares 'traceparent.header: X-Endpoint-Trace' in
    // rest.yaml, which replaces the global custom name (X-Trace-Context) for that endpoint
    let server = server().await;
    let endpoint_trace_id = "5af92f3577b34da6a3ce929d0e0e4705";
    let global_name_trace_id = "6af92f3577b34da6a3ce929d0e0e4706";
    let endpoint = format!("00-{endpoint_trace_id}-00f067aa0ba902b7-01");
    let global_name = format!("00-{global_name_trace_id}-00f067aa0ba902b8-01");
    let (status, _, _) = http(
        server.port,
        "GET",
        "/api/renamed/traceparent/probe",
        &[
            ("X-Endpoint-Trace", endpoint.as_str()),
            ("X-Trace-Context", global_name.as_str()),
        ],
        "",
    )
    .await;
    assert_eq!(status, 200);
    let seen = server
        .trace_seen
        .lock()
        .unwrap()
        .clone()
        .expect("probe ran");
    assert_eq!(
        seen.0.as_deref(),
        Some(endpoint_trace_id),
        "the per-endpoint traceparent.header override takes precedence over the global name"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn async_http_client_stamps_trace_context_under_both_names() {
    // outbound: with http.traceparent.header configured, the async HTTP client stamps the
    // SAME W3C value under both the standard "traceparent" and the custom name, so the
    // context survives an intermediary that strips the standard header while compliant
    // hops keep working
    let server = server().await;
    let w3c_trace_id = "7af92f3577b34da6a3ce929d0e0e4707";
    let value = format!("00-{w3c_trace_id}-00f067aa0ba902b7-01");
    let (status, _, body) = http(
        server.port,
        "GET",
        &format!("/api/echo/chain?port={}", server.port),
        &[
            ("traceparent", value.as_str()),
            ("Accept", "application/json"),
        ],
        "",
    )
    .await;
    assert_eq!(status, 200, "chain body: {body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("json body");
    let stamped = json["headers"]["traceparent"]
        .as_str()
        .expect("the traced caller stamped the standard traceparent on the outgoing hop");
    assert!(
        stamped.starts_with(&format!("00-{w3c_trace_id}-")),
        "the stamped traceparent continues the caller's trace: {stamped}"
    );
    assert_eq!(
        json["headers"]["x-trace-context"].as_str(),
        Some(stamped),
        "the same W3C value must be stamped under the custom traceparent header name"
    );
}
