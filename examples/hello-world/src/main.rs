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

//! The Rust analog of mercury-composable's README "greeting.demo" taste —
//! a bootable application proving the platform-core foundation end-to-end:
//!
//! configuration (`application.yml`, `${ENV:default}` substitution) →
//! lifecycle (before-application hook → preload → REST automation → main) →
//! event bus (a typed composable function invoked by route name over
//! `PostOffice` RPC) → distributed tracing (OpenTelemetry-compatible spans
//! logged in real time) → application log context (JSON log lines carry
//! cid / trace / span ids + business key-values) → actuators + static
//! content with etag/304 and a request filter.
//!
//! **Increment 10:** the whole application is declared with annotations —
//! `#[preload]`, `#[before_application]`, `#[main_application]` (the Java
//! `@PreLoad` / `@BeforeApplication` / `@MainApplication` analogs) — and
//! started by the one-line `auto_start_main!()` (Java `AutoStart.main(args)`).
//!
//! Run it (`-Dkey=value` after `--` is the JVM `-D` runtime-override analog —
//! it beats any configuration file value):
//! ```bash
//! cargo run -p hello-world                            # pretty JSON (from application.yml)
//! cargo run -p hello-world -- -Dlog.format=compact    # single-line jsonl
//! cargo run -p hello-world -- -Dlog.format=text       # plain console, no context block
//! GREETING_USER=eric cargo run -p hello-world
//! ```
//!
//! Watch for two structured log records: the greeting function's own log
//! entry (with the `context` block joining it to the trace), then the
//! telemetry dataset from `distributed.tracing` (the span: same trace id,
//! `span_id`, `parent_span_id`, timing, annotations).

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use platform_core::automation::AsyncHttpRequest;
use platform_core::{
    before_application, main_application, preload, trace, AppConfigReader, AppError,
    ComposableFunction, EntryPoint, EventEnvelope, EventStreamWriter, Platform, PostOffice,
    TypedFunction,
};
use rmpv::Value;
use serde::{Deserialize, Serialize};

// ---- the composable function (Java: @PreLoad(route = "greeting.demo", instances = 10)) ----

#[derive(Serialize, Deserialize)]
struct GreetingRequest {
    user: String,
}

#[derive(Serialize, Deserialize)]
struct GreetingResponse {
    message: String,
    handled_by_instance: usize,
}

/// `env_instances` (Java `envInstances`) lets `greeting.instances` in
/// `application.yml` — or a `-Dgreeting.instances=` override — set the worker
/// pool size; the literal `instances` is the fallback.
#[preload(
    route = "greeting.demo",
    instances = 10,
    env_instances = "greeting.instances",
    typed
)]
struct Greetings;

#[async_trait]
impl TypedFunction<GreetingRequest, GreetingResponse> for Greetings {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: GreetingRequest,
        instance: usize,
    ) -> Result<GreetingResponse, AppError> {
        let po = PostOffice::new(&Platform::get_instance());
        // business context for the APPLICATION LOG stream (context block)
        po.update_context("user", &input.user)?;
        // business context for the DISTRIBUTED-TRACE dataset (span annotation)
        po.annotate_trace("greeting.for", &input.user);
        // this log line carries the context block: cid, trace/span ids,
        // service, environment, and the "user" key added above
        log::info!("processing greeting request");
        Ok(GreetingResponse {
            message: format!("Welcome, {}", input.user),
            handled_by_instance: instance,
        })
    }
}

// ---- the HTTP-facing function (REST automation: /api/greeting/{user}) ----

/// Receives the HTTP request from the REST edge as a TYPED
/// `AsyncHttpRequest` (Java `TypedLambdaFunction<AsyncHttpRequest, Object>`
/// parity — here without any engine special case: `AsyncHttpRequest`
/// deserializes from the request dataset, so the ordinary typed adapter
/// carries it), then composes with the typed greeting.demo function — the
/// edge-started trace propagates automatically, producing a two-span tree
/// (greeting.api → greeting.demo).
#[preload(route = "greeting.api", instances = 5, typed)]
struct GreetingApi;

#[async_trait]
impl TypedFunction<AsyncHttpRequest, serde_json::Value> for GreetingApi {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        request: AsyncHttpRequest,
        _instance: usize,
    ) -> Result<serde_json::Value, AppError> {
        let user = request
            .path_parameter("user")
            .unwrap_or("world")
            .to_string();
        let po = PostOffice::new(&Platform::get_instance());
        po.update_context("user", &user)?;
        log::info!("HTTP request for {user}");
        // no trace fields set — propagation from the edge-started trace is automatic
        let reply = po
            .request(
                EventEnvelope::new()
                    .set_to("greeting.demo")
                    .set_body(GreetingRequest { user })?,
                Duration::from_secs(5),
            )
            .await?;
        let body: GreetingResponse = reply.body_as()?;
        Ok(serde_json::json!({
            "message": body.message,
            "handled_by_instance": body.handled_by_instance,
            "trace_id": po.my_trace_id(),
            "correlation_id": po.my_correlation_id(),
        }))
    }
}

// ---- the public echo function (Java lambda-example: hello.world) ----

/// Mirrors the Java lambda-example's `hello.world` echo: replies with the
/// request body and headers plus the worker instance and this application's
/// origin id. Declared `is_private = false` — the deliberate opt-out that
/// publishes the route to remote callers via Event over HTTP
/// (`POST /api/event`); every other function here keeps the private default.
///
/// The function registers TWO route names (a comma-separated alias list,
/// Java `@PreLoad` parity) because it is the standing Event-over-HTTP
/// interop target of the demo pair: the Rust `hello-flow` example (or the
/// Java `composable-example`) calls `hello.world` through the PROGRAMMATIC
/// pattern (the caller passes this app's `/api/event` URL to the request
/// API) and `hello.declarative` through the DECLARATIVE pattern (the route
/// is resolved via the caller's `event-over-http.yaml`). Both examples run
/// on port 8085 with the same route names, so the Java and Rust callees are
/// drop-in replacements for each other — that interchangeability is the
/// point of the demo.
///
/// The body is reflected as the raw MsgPack value, never through a JSON
/// detour — JSON has no byte type, so converting would silently drop binary
/// values from the echo (found by the cross-language interop matrix).
///
/// An optional integer body key `sleep_ms` delays the reply by that many
/// milliseconds — the cross-language interop matrix uses it to exercise the
/// RPC-timeout (408) path (e.g. `sleep_ms: 3000` against `x-ttl: 1500`).
#[preload(
    route = "hello.world, hello.declarative",
    instances = 10,
    is_private = false
)]
struct HelloWorld;

#[async_trait]
impl ComposableFunction for HelloWorld {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body = input.body().clone();
        if let Some(ms) = sleep_ms(&body) {
            log::info!("echo #{instance} sleeping {ms} ms before replying");
            tokio::time::sleep(Duration::from_millis(ms)).await;
        }
        log::info!("echo #{instance} got a request");
        // forward an event to hello.pojo so the span-id of this function is
        // seen propagated to hello.pojo (Java lambda-example parity)
        let po = PostOffice::new(&Platform::get_instance());
        po.send(
            EventEnvelope::new()
                .set_to("hello.pojo")
                .set_header("id", "1"),
        )
        .await?;
        // Note that the '_headers' input = input.headers() plus injected read-only metadata
        // (my_route, my_trace_id, my_trace_path and my_correlation_id).
        // Since we want to echo back the original request headers, we use input.headers().
        let echo_headers = Value::Map(
            input
                .headers()
                .iter()
                .map(|(k, v)| (Value::from(k.as_str()), Value::from(v.as_str())))
                .collect(),
        );
        Ok(EventEnvelope::new().set_raw_body(Value::Map(vec![
            (Value::from("body"), body),
            (Value::from("headers"), echo_headers),
            (Value::from("instance"), Value::from(instance as u64)),
            (Value::from("origin"), Value::from(Platform::origin())),
        ])))
    }
}

/// The optional integer `sleep_ms` key of a map body.
fn sleep_ms(body: &Value) -> Option<u64> {
    let Value::Map(entries) = body else {
        return None;
    };
    entries
        .iter()
        .find(|(k, _)| k.as_str() == Some("sleep_ms"))
        .and_then(|(_, v)| v.as_u64())
}

// ---- the PoJo demo function (Java lambda-example: hello.pojo) ----

/// Mirrors the Java lambda-example's `hello.pojo`: returns a place-holder
/// object for `id = 1` (the echo forwards here fire-and-forget so the span
/// propagation from `hello.world` is visible in the trace), 404 otherwise.
#[preload(route = "hello.pojo", instances = 10, is_private = false)]
struct HelloPoJo;

#[async_trait]
impl ComposableFunction for HelloPoJo {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        _input: EventEnvelope,
        instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        match headers.get("id").map(String::as_str) {
            Some("1") => {
                log::info!("PoJo delivered by instance #{instance}");
                EventEnvelope::new().set_body(serde_json::json!({
                    "id": 1,
                    "name": "Simple PoJo class",
                    "address": "100 World Blvd, Planet Earth",
                    "instance": instance,
                    "origin": Platform::origin(),
                }))
            }
            Some(_) => Err(AppError::new(404, "Not found. Try id = 1")),
            None => Err(AppError::new(400, "Missing parameter 'id'")),
        }
    }
}

// ---- the Event-over-HTTP authentication demo (Java lambda-example: event.api.auth) ----

/// Demo authentication service for the Event-over-HTTP endpoint.
///
/// The `rest.yaml` entry for `POST /api/event` declares
/// `authentication: 'event.api.auth'`, so every incoming Event API request is
/// delivered here first. This demo compares the caller's `authorization`
/// header against a shared token that both peers resolve from the environment
/// (`demo.peer.token: ${DEMO_PEER_TOKEN:demo}` in application.yml) — never
/// hard-code a real credential in source or configuration files.
///
/// Returning an envelope with a boolean body tells the REST automation engine
/// to continue (`true`) or reject with HTTP-401 (`false`). Additional headers
/// on the envelope become **session info** that rides to the target function
/// as read-only headers — the `user` header in this demo. Replace this
/// function with your own OAuth 2.0 bearer-token validation for production:
/// a real deployment verifies the bearer token against an OAuth2 security
/// authority — an I/O-bound call, hence the higher rule-of-thumb instance
/// count (30) and the `worker.instances.event.api.auth` ops knob (the same
/// key the Java demo uses), so operations teams can fine-tune concurrency in
/// QA/Perf environments before promoting to production.
#[preload(
    route = "event.api.auth",
    instances = 30,
    env_instances = "worker.instances.event.api.auth"
)]
struct EventApiAuth;

#[async_trait]
impl ComposableFunction for EventApiAuth {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request = platform_core::automation::AsyncHttpRequest::from_value(input.body());
        let expected = AppConfigReader::get_instance().get_property_or("demo.peer.token", "demo");
        let authorized = request.header("authorization") == Some(expected.as_str());
        log::info!(
            "Event API authorization {} {} = {}",
            request.method(),
            request.url(),
            if authorized { "PASS" } else { "FAIL" }
        );
        EventEnvelope::new()
            .set_header("user", "demo")
            .set_body(authorized)
    }
}

// ---- the static-content request filter (increment 8) ----

/// A simple interceptor for static content (`static-content.filter`): inspects
/// the HTTP headers of matching requests and lets them through (status 200).
/// A real deployment would handle SSO here — inspect the session cookie and
/// return 302 + Location to the identity provider when absent. The instance
/// count (20) is a rule of thumb sized for a static-content front door;
/// operations teams tune it via `worker.instances.http.request.filter` in
/// QA/Perf environments before promoting to production.
#[preload(
    route = "http.request.filter",
    instances = 20,
    env_instances = "worker.instances.http.request.filter"
)]
struct HttpRequestFilter;

#[async_trait]
impl ComposableFunction for HttpRequestFilter {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        log::info!(
            "[filter] {} from {} (user-agent: {})",
            request["url"].as_str().unwrap_or("?"),
            request["ip"].as_str().unwrap_or("?"),
            request["headers"]["user-agent"].as_str().unwrap_or("-"),
        );
        // 200 = continue serving; the header below rides onto the HTTP response
        EventEnvelope::new()
            .set_header("x-filter", "inspected")
            .set_body(serde_json::Value::Null)
    }
}

// ---- progressive result set rendering (HTTP response streaming) ----

/// This demo function serves an HTTP endpoint with progressive result set
/// rendering (HTTP response streaming). The endpoint is declared with
/// `stream: true` in rest.yaml, and the function streams a sequence of test
/// messages slowly so that you can watch them render one by one as
/// Server-Sent Events.
///
/// A streaming producer is an interceptor - it receives the raw event
/// envelope (including the caller's reply_to address) and streams segments
/// through the `EventStreamWriter` until it declares end of transmission.
///
/// Optional query parameters: "delay" in milliseconds between messages
/// (default 1000, bounded 50 - 5000) and "count" for the number of messages
/// (default 10, bounded 1 - 100).
///
/// Try it with the companion script: `node scripts/sse-client.mjs`
/// or with `curl -N -H 'accept: text/event-stream'` against
/// `http://127.0.0.1:8085/api/hello/sse`
// public so a peer application (an engine or a python/node function host) can
// consume this stream through /api/event - the cross-runtime streaming demo,
// the hello.declarative precedent
#[preload(route = "hello.sse", instances = 10, interceptor, is_private = false)]
struct HelloSse;

#[async_trait]
impl ComposableFunction for HelloSse {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        let query = &request["parameters"]["query"];
        let delay = query["delay"]
            .as_str()
            .and_then(|v| v.parse::<u64>().ok())
            .map_or(1000, |v| v.clamp(50, 5000));
        let count = query["count"]
            .as_str()
            .and_then(|v| v.parse::<u32>().ok())
            .map_or(10, |v| v.clamp(1, 100));
        let mut out = EventStreamWriter::from_request(&Platform::get_instance(), &input)?;
        out.first(200, "text/event-stream");
        out.write("The following messages are rendered slowly to demonstrate the SSE feature:")
            .await?;
        for i in 1..=count {
            tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
            out.write(format!("test message {i}")).await?;
        }
        out.close_with("end of SSE page.").await?;
        Ok(EventEnvelope::new())
    }
}

// ---- a health-check function (increment 7: /health lists it as mandatory) ----

/// Honors the actuator health protocol: header `type=info` describes the
/// dependency; `type=health` reports its live status.
#[preload(route = "demo.health")]
struct DemoHealth;

#[async_trait]
impl ComposableFunction for DemoHealth {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        match headers.get("type").map(String::as_str) {
            Some("info") => EventEnvelope::new().set_body(serde_json::json!({
                "service": "demo.store",
                "href": "memory://demo",
            })),
            Some("health") => EventEnvelope::new().set_body("demo store is running"),
            _ => Err(AppError::new(400, "unknown health request type")),
        }
    }
}

// ---- a before-application hook (Java: @BeforeApplication(sequence = 5), like CompileFlows) ----

#[before_application(sequence = 5)]
struct PreflightCheck;

#[async_trait]
impl EntryPoint for PreflightCheck {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        // validation work that must precede registration — here, prove the
        // configuration is sane before anything is bound
        let config = AppConfigReader::get_instance();
        if !config.exists("greeting.user") {
            return Err(AppError::new(
                500,
                "greeting.user missing from application.yml",
            ));
        }
        // through the structured logger: renders as a pretty-JSON record with
        // the trace-independent context keys (environment, hello, timestamp)
        log::info!("[before-application] configuration validated");
        Ok(())
    }
}

// ---- the engine-to-wrapper streaming relay (/api/hello/remote) ----

/// The engine-to-wrapper streaming composition: this endpoint's function
/// forwards its own reply lane and correlation id into a send to the
/// event-over-http mapped "hello.tokens" function - the python/node demo apps'
/// streaming function - and opts in with the "accept: text/event-stream" event
/// header. The remote segments relay through the peer's /api/event in envelope
/// mode and re-render progressively out this application's HTTP edge, with no
/// imperative streaming code in between.
///
/// The routing map ships in resources/event-over-http.yaml (the python demo
/// defaults to port 8086; point at another peer with -Dpeer.demo.host /
/// -Dpeer.demo.port). Start a wrapper demo app, then:
/// `curl -N -H 'accept: text/event-stream'` against
/// `http://127.0.0.1:8085/api/hello/remote?delay=300&count=3`
#[preload(route = "hello.remote.relay", instances = 10, interceptor)]
struct HelloRemoteRelay;

#[async_trait]
impl ComposableFunction for HelloRemoteRelay {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        const REMOTE_ROUTE: &str = "hello.tokens";
        let platform = Platform::get_instance();
        if platform_core::automation::get_event_http_target(REMOTE_ROUTE).is_none() {
            // teaching failure: the demo depends on the declarative routing map
            let mut out = EventStreamWriter::from_request(&platform, &input)?;
            out.fail(&AppError::new(
                503,
                "Remote streaming demo is not configured - check event-over-http.yaml and start a wrapper demo app (see README)",
            ))
            .await?;
            return Ok(EventEnvelope::new());
        }
        let request: serde_json::Value = input.body_as()?;
        let query = &request["parameters"]["query"];
        let mut forward = EventEnvelope::new()
            .set_to(REMOTE_ROUTE)
            .set_reply_to(input.reply_to().unwrap_or_default())
            .set_correlation_id(input.correlation_id().unwrap_or_default())
            // the event-level opt-in for progressive streaming over Event-over-HTTP
            .set_header("accept", "text/event-stream")
            // idle allowance between stream events on both hops (ms)
            .set_header("x-ttl", "30000");
        if let Some(delay) = query["delay"].as_str() {
            forward = forward.set_header("delay", delay);
        }
        if let Some(count) = query["count"].as_str() {
            forward = forward.set_header("count", count);
        }
        PostOffice::new(&platform).send(forward).await?;
        Ok(EventEnvelope::new())
    }
}

// ---- the main application (Java: @MainApplication implementing EntryPoint) ----

#[main_application]
struct MainApp;

#[async_trait]
impl EntryPoint for MainApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        let config = AppConfigReader::get_instance();
        let user = config.get_property_or("greeting.user", "world");
        let po = PostOffice::new(&Platform::get_instance());
        // start a TRACED request: a W3C/OTel-compatible trace id + a request
        // path (in Java, REST automation does this when tracing: true), plus
        // a BUSINESS correlation-id — a separate concern from the trace
        let request = EventEnvelope::new()
            .set_to("greeting.demo")
            .set_trace(&trace::new_trace_id(), "GET /api/greeting")
            .set_correlation_id("order-12345")
            .set_body(GreetingRequest { user })?;
        let response = po.request(request, Duration::from_secs(5)).await?;
        let body: GreetingResponse = response.body_as()?;
        // through the structured logger: a pretty-JSON record with a context
        // section (main runs outside a traced worker, so only the
        // trace-independent keys render; the message carries the trace id)
        log::info!(
            "[main] {} (worker #{}, {:.2} ms round trip, trace {})",
            body.message,
            body.handled_by_instance,
            response.exec_time().unwrap_or(0.0),
            response.trace_id().unwrap_or("-")
        );
        if config.get_property_or("rest.automation", "false") == "true" {
            let port = config.get_property_or("rest.server.port", "8085");
            log::info!("Try: curl http://127.0.0.1:{port}/api/greeting/eric");
        }
        Ok(())
    }
}

// the whole startup — Java `AutoStart.main(args)`: runtime, `-D` overrides,
// structured logging, annotation collection, lifecycle, serve until Ctrl-C
platform_core::auto_start_main!();
