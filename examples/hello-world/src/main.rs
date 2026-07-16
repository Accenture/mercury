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
use platform_core::{
    before_application, main_application, preload, trace, AppConfigReader, AppError,
    ComposableFunction, EntryPoint, EventEnvelope, Platform, PostOffice, TypedFunction,
};
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

/// Receives the AsyncHttpRequest event from the HTTP edge, then composes with
/// the typed greeting.demo function — the edge-started trace propagates
/// automatically, producing a two-span tree (greeting.api → greeting.demo).
#[preload(route = "greeting.api", instances = 5)]
struct GreetingApi;

#[async_trait]
impl ComposableFunction for GreetingApi {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        let user = request["parameters"]["path"]["user"]
            .as_str()
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
        EventEnvelope::new().set_body(serde_json::json!({
            "message": body.message,
            "handled_by_instance": body.handled_by_instance,
            "trace_id": po.my_trace_id(),
            "correlation_id": po.my_correlation_id(),
        }))
    }
}

// ---- the static-content request filter (increment 8) ----

/// A simple interceptor for static content (`static-content.filter`): inspects
/// the HTTP headers of matching requests and lets them through (status 200).
/// A real deployment would handle SSO here — inspect the session cookie and
/// return 302 + Location to the identity provider when absent.
#[preload(route = "http.request.filter", instances = 2)]
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
