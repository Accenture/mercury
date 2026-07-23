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

//! Increment 61 — Event over HTTP end to end: a serialized envelope crosses
//! `POST /api/event` over a real HTTP connection, exercising the visibility
//! boundary (403 private), RPC + async dispatch, 404, and the compact-format
//! rejection. The `event_over_http` client is round-tripped against the same
//! server so the two halves prove each other (a local stand-in for the
//! cross-language interop pairing).

use std::collections::HashMap;
use std::sync::{Arc, Once};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::automation::http_client::AsyncHttpClientService;
use platform_core::automation::{self, event_over_http, AsyncHttpRequest, EventApiService};
use platform_core::{
    overrides, resources, AppConfigReader, AppError, ComposableFunction, EventEnvelope,
    FunctionOptions, Platform, PostOffice,
};
use rmpv::Value;

/// A public target that spends more than the caller's TTL before replying —
/// the remote peer's in-band 408 must reach the caller (see
/// `remote_timeout_arrives_in_band`).
struct SleepyEcho;

#[async_trait]
impl ComposableFunction for SleepyEcho {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        tokio::time::sleep(Duration::from_millis(3000)).await;
        EventEnvelope::new().set_body("too late")
    }
}

/// A public target: echoes its body and reports the trace id it ran under.
struct RemoteEcho;

#[async_trait]
impl ComposableFunction for RemoteEcho {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let po = PostOffice::new(&Platform::get_instance());
        EventEnvelope::new()
            .set_status(201)
            .set_header("x-demo", "remote")
            .set_body(serde_json::json!({
                "echo": serde_json::to_value(input.body()).unwrap_or_default(),
                "seen_trace": po.my_trace_id(),
            }))
    }
}

async fn server() -> (u16, Platform) {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let holding = std::env::temp_dir().join(format!("mercury-eoh-test-{}", std::process::id()));
        overrides::set("transient.data.store", &holding.display().to_string());
        overrides::set("rest.server.port", "0");
        // a minimal rest.yaml; /api/event arrives via the default-endpoint merge
        overrides::set("yaml.rest.automation", "classpath:/event-rest.yaml");
        let _ = AppConfigReader::get_instance();
    });
    let platform = Platform::new();
    // the event service + a PUBLIC target and a PRIVATE one
    platform
        .register_private(
            automation::EVENT_API_SERVICE,
            Arc::new(EventApiService::new(&platform)),
            10,
        )
        .unwrap();
    platform
        .register("v1.remote.echo", Arc::new(RemoteEcho), 4)
        .unwrap();
    platform
        .register_private("v1.secret", Arc::new(RemoteEcho), 1)
        .unwrap();
    platform
        .register("v1.sleepy.echo", Arc::new(SleepyEcho), 2)
        .unwrap();
    // the event-over-http CLIENT posts through async.http.request — the
    // lifecycle registers it normally; this test boots the server directly
    platform
        .register_with_options(
            automation::ASYNC_HTTP_REQUEST,
            Arc::new(AsyncHttpClientService),
            10,
            FunctionOptions {
                interceptor: true,
                private: true,
                ..FunctionOptions::default()
            },
        )
        .unwrap();
    let addr = automation::start_http_server(&platform).await.unwrap();
    (addr.port(), platform)
}

/// POST raw bytes to `/api/event` and return (http_status, response body bytes).
async fn post_event(port: u16, body: &[u8], extra: &[(&str, &str)]) -> (u16, Vec<u8>) {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let mut head = format!(
        "POST /api/event HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n",
        body.len()
    );
    for (name, value) in extra {
        head.push_str(&format!("{name}: {value}\r\n"));
    }
    head.push_str("Connection: close\r\n\r\n");
    let mut request = head.into_bytes();
    request.extend_from_slice(body);
    stream.write_all(&request).await.expect("write");
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).await.expect("read");
    // split head/body on the CRLFCRLF
    let sep = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .expect("http header terminator");
    let head = String::from_utf8_lossy(&raw[..sep]).to_string();
    let status: u16 = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|c| c.parse().ok())
        .expect("status");
    (status, raw[sep + 4..].to_vec())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn event_over_http_service_and_client() {
    let (port, platform) = server().await;

    // --- RPC to a public target: the reply envelope carries the target's
    // status, headers, and body (the serialized envelope rides the HTTP body)
    let request = EventEnvelope::new()
        .set_to("v1.remote.echo")
        .set_trace("trace-eoh", "POST /api/event")
        .set_span_id("00f067aa0ba902b7")
        .set_body(serde_json::json!({"ping": 1}))
        .unwrap();
    let (http_status, body) =
        post_event(port, &request.to_bytes().unwrap(), &[("x-ttl", "5000")]).await;
    assert_eq!(http_status, 200);
    let result = EventEnvelope::from_bytes(&body).expect("decode reply envelope");
    assert_eq!(result.status(), 201);
    assert_eq!(result.header("x-demo"), Some("remote"));
    let payload: serde_json::Value = result.body_as().unwrap();
    assert_eq!(payload["echo"]["ping"], 1);
    // the trace crossed the boundary and the target ran under it
    assert_eq!(payload["seen_trace"], "trace-eoh");

    // --- a private target is rejected 403 (the encapsulation boundary)
    let secret = EventEnvelope::new()
        .set_to("v1.secret")
        .set_body("x")
        .unwrap();
    let (status, body) = post_event(port, &secret.to_bytes().unwrap(), &[]).await;
    assert_eq!(status, 403);
    let err = EventEnvelope::from_bytes(&body).unwrap();
    assert_eq!(err.status(), 403);
    assert!(err
        .body_as::<String>()
        .unwrap()
        .contains("v1.secret is private"));

    // --- an unknown route is 404
    let missing = EventEnvelope::new()
        .set_to("v1.nope")
        .set_body("x")
        .unwrap();
    let (status, _) = post_event(port, &missing.to_bytes().unwrap(), &[]).await;
    assert_eq!(status, 404);

    // --- async (drop-n-forget) → HTTP 200 with a 202 ack envelope
    let async_req = EventEnvelope::new()
        .set_to("v1.remote.echo")
        .set_body("fire")
        .unwrap();
    let (status, body) =
        post_event(port, &async_req.to_bytes().unwrap(), &[("x-async", "true")]).await;
    assert_eq!(status, 200);
    let ack = EventEnvelope::from_bytes(&body).unwrap();
    assert_eq!(ack.status(), 202);
    let ack_body: serde_json::Value = ack.body_as().unwrap();
    assert_eq!(ack_body["type"], "async");
    assert_eq!(ack_body["delivered"], true);

    // --- a compact (legacy) envelope is rejected 400 with a clear message
    let compact = Value::Map(vec![
        (Value::from("0"), Value::from("e1")),
        (Value::from("T"), Value::from("v1.remote.echo")),
    ]);
    let compact_bytes = rmp_serde::to_vec_named(&compact).unwrap();
    let (status, body) = post_event(port, &compact_bytes, &[]).await;
    assert_eq!(status, 400);
    assert!(EventEnvelope::from_bytes(&body)
        .unwrap()
        .body_as::<String>()
        .unwrap()
        .contains("compact format not supported"));

    // --- the client half round-trips against the same server (a local
    // stand-in for cross-language interop): RPC + async
    let po = PostOffice::new(&platform);
    let endpoint = format!("http://127.0.0.1:{port}/api/event");
    let via_client = event_over_http(
        &po,
        &endpoint,
        EventEnvelope::new()
            .set_to("v1.remote.echo")
            .set_body(serde_json::json!({"ping": 2}))
            .unwrap(),
        Duration::from_secs(5),
        true,
    )
    .await
    .expect("client RPC");
    assert_eq!(via_client.status(), 201);
    let client_payload: serde_json::Value = via_client.body_as().unwrap();
    assert_eq!(client_payload["echo"]["ping"], 2);

    let via_client_async = event_over_http(
        &po,
        &endpoint,
        EventEnvelope::new()
            .set_to("v1.remote.echo")
            .set_body("fire")
            .unwrap(),
        Duration::from_secs(5),
        false,
    )
    .await
    .expect("client async");
    assert_eq!(via_client_async.status(), 202);

    // the private-target 403 surfaces through the client too
    let client_403 = event_over_http(
        &po,
        &endpoint,
        EventEnvelope::new()
            .set_to("v1.secret")
            .set_body("x")
            .unwrap(),
        Duration::from_secs(5),
        true,
    )
    .await
    .expect("client 403 is a normal envelope, not a transport error");
    assert_eq!(client_403.status(), 403);
}

/// A peer that spends more than the caller's TTL replies with its own in-band
/// 408 envelope — and the caller must RECEIVE it, not abort locally first.
/// Guards two deadline bugs (the Rust twin of Java's
/// `EventHttpTest.remoteTimeoutArrivesInBand`): the wire-level read timeout
/// must round a fractional-second x-ttl UP with grace
/// (`AsyncHttpRequest::timeout_seconds` + the response-timeout site), and the
/// client's local RPC wait must exceed the remote TTL (`event_over_http`).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn remote_timeout_arrives_in_band() {
    let (port, platform) = server().await;
    let po = PostOffice::new(&platform);
    let endpoint = format!("http://127.0.0.1:{port}/api/event");
    let reply = event_over_http(
        &po,
        &endpoint,
        EventEnvelope::new()
            .set_to("v1.sleepy.echo")
            .set_body("x")
            .unwrap(),
        Duration::from_millis(1500),
        true,
    )
    .await
    .expect("the remote in-band 408 must arrive as a normal envelope, not a local client error");
    assert_eq!(reply.status(), 408);
    let message: String = reply.body_as().unwrap();
    assert!(
        message.contains("1500"),
        "the 408 message should carry the remote ttl: {message}"
    );
}

/// The service must not be reachable AS a remote target (it is private) — a
/// defense-in-depth check on the visibility gate.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn event_service_itself_is_private() {
    let (port, _platform) = server().await;
    let recursive = EventEnvelope::new()
        .set_to(automation::EVENT_API_SERVICE)
        .set_body("x")
        .unwrap();
    let (status, _) = post_event(port, &recursive.to_bytes().unwrap(), &[]).await;
    assert_eq!(
        status, 403,
        "the event service must reject itself as a target"
    );
    // silence the unused import when only this test builds
    let _ = AsyncHttpRequest::new();
}

/// A capture target for the metadata-transport contract: reports what it saw
/// in its input header COPY versus the envelope view.
struct MetadataCapture {
    seen: Arc<std::sync::Mutex<Option<serde_json::Value>>>,
}

#[async_trait]
impl ComposableFunction for MetadataCapture {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        *self.seen.lock().expect("capture") = Some(serde_json::json!({
            "injected_cid": headers.get("my_correlation_id"),
            "injected_route": headers.get("my_route"),
            "envelope_cid_header": input.header("my_correlation_id"),
            "envelope_event_api_header": input.header("x-event-api"),
            "delivered_event_api": headers.get("x-event-api"),
            // engine tags are scrubbed from the function's envelope view
            "visible_tags": input.tag(platform_core::post_office::BUSINESS_CID_TAG),
        }));
        EventEnvelope::new().set_body("captured")
    }
}

/// Regression twin of Java `EventHttpTest.metadataIsNeverTransportedInTheEvent`:
/// the my_* metadata keys are injected into the function's input header COPY
/// at delivery — they must never exist as envelope headers. The business
/// correlation-id crosses the Event-over-HTTP wire as an engine-managed tag.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn metadata_is_never_transported_in_the_event() {
    let (port, platform) = server().await;
    let seen = Arc::new(std::sync::Mutex::new(None));
    platform
        .register(
            "metadata.transport.capture",
            Arc::new(MetadataCapture { seen: seen.clone() }),
            1,
        )
        .unwrap();
    // a TRACED caller function with a business correlation-id makes the
    // remote loopback hop — the wire + relay path is exercised end to end
    struct Caller {
        platform: Platform,
        port: u16,
    }
    #[async_trait]
    impl ComposableFunction for Caller {
        async fn handle_event(
            &self,
            _headers: HashMap<String, String>,
            _input: EventEnvelope,
            _instance: usize,
        ) -> Result<EventEnvelope, AppError> {
            let po = PostOffice::new(&self.platform);
            event_over_http(
                &po,
                &format!("http://127.0.0.1:{}/api/event", self.port),
                EventEnvelope::new()
                    .set_to("metadata.transport.capture")
                    .set_body("ping")?,
                Duration::from_secs(5),
                true,
            )
            .await
        }
    }
    platform
        .register_private(
            "metadata.transport.caller",
            Arc::new(Caller {
                platform: platform.clone(),
                port,
            }),
            1,
        )
        .unwrap();
    let po = PostOffice::new(&platform);
    let correlation_id = format!("corr-{}", uuid_like());
    let _ = po
        .request(
            EventEnvelope::new()
                .set_to("metadata.transport.caller")
                .set_trace(
                    &platform_core::trace::new_trace_id(),
                    "TEST /metadata/transport",
                )
                .set_correlation_id(&correlation_id)
                .set_body("start")
                .unwrap(),
            Duration::from_secs(8),
        )
        .await
        .expect("remote hop");
    let seen = seen.lock().expect("capture").clone().expect("captured");
    // injected metadata reaches the function's header copy — the business
    // cid arrived INTACT across the wire (tag transport)
    assert_eq!(seen["injected_cid"], serde_json::json!(correlation_id));
    assert_eq!(
        seen["injected_route"],
        serde_json::json!("metadata.transport.capture")
    );
    // but the envelope itself never carries metadata or engine-internal keys
    assert_eq!(
        seen["envelope_cid_header"],
        serde_json::Value::Null,
        "my_correlation_id must not be an envelope header"
    );
    assert_eq!(
        seen["delivered_event_api"],
        serde_json::Value::Null,
        "x-event-api must not reach a user function"
    );
    assert_eq!(seen["envelope_event_api_header"], serde_json::Value::Null);
    // the tag channel is engine-managed — not even visible to the function's
    // envelope view (the injected value arriving intact proves it carried)
    assert_eq!(seen["visible_tags"], serde_json::Value::Null);
}

/// A cheap unique suffix without importing uuid in the test crate.
fn uuid_like() -> String {
    format!(
        "{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}
