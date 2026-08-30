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

//! Event over HTTP (increment 61) — Rust port of Java
//! `org.platformlambda.core.services.EventApiService` + the `EventEmitter`
//! event-over-http client. A serialized [`EventEnvelope`] (the **standard**
//! wire format, increment 59) is exchanged over `POST /api/event`, so a
//! function in one application instance can call a **public** function
//! (`is_private = false`) in another — the only cross-instance coupling, and
//! opt-in by design.
//!
//! The service is reached through REST automation (the `/api/event` entry
//! ships in the default `rest.yaml`, merged like the actuators). It decodes
//! the posted envelope, enforces the visibility boundary (403 for a private
//! target — a remote caller must never reach engine internals or an
//! unpublished function), and dispatches: async (`x-async: true`) is
//! drop-n-forget with a 202 ack; otherwise RPC up to `x-ttl` ms.

use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;

use async_trait::async_trait;
use rmpv::Value;

use crate::automation::http_client::{AsyncHttpRequest, ASYNC_HTTP_REQUEST};
use crate::envelope::EventEnvelope;
use crate::function::{AppError, ComposableFunction};
use crate::platform::Platform;
use crate::post_office::PostOffice;
use crate::util::app_config_reader::AppConfigReader;
use crate::util::config_reader::ConfigReader;
use crate::util::multi_level_map::ConfigValue;
use crate::util::w3c_trace;

/// Route of the event-over-http service (Java `EventApiService.EVENT_API_SERVICE`).
pub const EVENT_API_SERVICE: &str = "event.api.service";

/// Envelope header marking an event that already crossed an Event-over-HTTP
/// hop (Java `EventEmitter.X_EVENT_API`) — the recursion guard: the send and
/// request hooks never re-forward an event carrying it, so a declaratively
/// routed event crosses the wire exactly once. Visible to the receiving
/// function like any other envelope header.
pub const X_EVENT_API: &str = "x-event-api";

const OCTET_STREAM: &str = "application/octet-stream";
const TEXT_EVENT_STREAM: &str = "text/event-stream";
const STREAM_CALLER_REQUIRED: &str =
    "Streaming function requires a caller that accepts text/event-stream";
const X_TTL: &str = "x-ttl";
const X_ASYNC: &str = "x-async";
/// Java `EventEmitter.X_NO_STREAM`: instructs the receiving side to return a
/// small payload as bytes — part of the /api/event request's wire header set.
const X_NO_STREAM: &str = "x-small-payload-as-bytes";

/// Application key naming the declarative routing map
/// (Java `EventEmitter.EVENT_OVER_HTTP_YAML`).
const EVENT_OVER_HTTP_YAML: &str = "yaml.event.over.http";
const DEFAULT_EVENT_OVER_HTTP_YAML: &str = "classpath:/event-over-http.yaml";

/// Fixed forward timeout for the send-path (fire-and-forget / callback) hook
/// (Java `EventEmitter.ASYNC_EVENT_HTTP_TIMEOUT` = 60s).
const ASYNC_EVENT_HTTP_TIMEOUT: Duration = Duration::from_secs(60);

/// The `/api/event` service (Java `EventApiService`). Registered PRIVATE — it
/// is reached only through the REST boundary, never as a remote target.
pub struct EventApiService {
    platform: Platform,
}

impl EventApiService {
    pub fn new(platform: &Platform) -> Self {
        EventApiService {
            platform: platform.clone(),
        }
    }
}

#[async_trait]
impl ComposableFunction for EventApiService {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        // registered as an event interceptor (Java @EventInterceptor parity):
        // replies are sent manually to the edge's reply route with the edge
        // context id, so the streaming branch can withhold its auto-reply
        let Some(reply_to) = input.reply_to().map(str::to_string) else {
            return Ok(EventEnvelope::new());
        };
        let context_id = input.correlation_id().unwrap_or_default().to_string();
        let po = PostOffice::new(&self.platform);
        if let Some(response) = self.dispatch(&po, &reply_to, &context_id, input).await? {
            let _ = po
                .send(response.set_to(&reply_to).set_correlation_id(&context_id))
                .await;
        }
        Ok(EventEnvelope::new())
    }
}

impl EventApiService {
    /// Decode, validate and dispatch one /api/event call. Returns the reply
    /// envelope for the single-shot modes, or None when the streaming relay
    /// rewired the inner request onto the edge's reply lane.
    async fn dispatch(
        &self,
        po: &PostOffice,
        reply_to: &str,
        context_id: &str,
        input: EventEnvelope,
    ) -> Result<Option<EventEnvelope>, AppError> {
        let request = AsyncHttpRequest::from_value(input.body());
        let timeout_ms = request
            .header(X_TTL)
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0)
            .max(1000);
        let is_async = request.header(X_ASYNC) == Some("true");
        let accepts_sse = request
            .header("accept")
            .is_some_and(|accept| accept.contains(TEXT_EVENT_STREAM));
        // on the streaming-capable path the edge dispatched through a reply
        // lane (envelope mode) and wraps every single-shot lane reply into the
        // classic wire itself - so errors ride RAW here, exactly once wrapped;
        // the other paths pack the classic wire as before
        let capable = accepts_sse && !is_async;
        let answer = |status: i32, message: &str| -> EventEnvelope {
            if capable {
                EventEnvelope::new()
                    .set_status(status)
                    .set_raw_body(Value::from(message))
            } else {
                reply(status, error_envelope(status, message))
            }
        };
        // the HTTP body is the serialized envelope; octet-stream arrives as a
        // MsgPack-binary body on the request map (Java parity)
        let Value::Binary(bytes) = request.body() else {
            return Ok(Some(if capable {
                answer(500, "Invalid event-over-http data format")
            } else {
                reply(500, b"Invalid event-over-http data format".to_vec())
            }));
        };
        // v1 accepts the standard wire format only (phase-2 decision); a
        // compact (all single-char keys) envelope is rejected clearly
        if is_compact_envelope(bytes) {
            return Ok(Some(answer(
                400,
                "compact format not supported - set event.over.http.format=standard on the sender",
            )));
        }
        let inner = match EventEnvelope::from_bytes(bytes) {
            Ok(envelope) => envelope,
            // the format is unknown when decode fails (Java falls back to a
            // compact error reply; we answer 400 with a plain message)
            Err(e) => return Ok(Some(answer(400, e.message()))),
        };
        // an inbound '@origin' suffix from a legacy/mesh-era peer is parsed
        // away — this port never generates one (Eric's ruling)
        let Some(to) = inner
            .to()
            .map(|to| crate::platform::bare_route(to).to_string())
        else {
            return Ok(Some(answer(400, "Missing routing path")));
        };
        // session info injected by an authentication service on this /api/event
        // entry rides to the target function as read-only headers (Java parity:
        // sessionInfo.forEach(request::setHeader))
        let mut inner = inner;
        for (key, value) in request.session() {
            inner = inner.set_header(key, value);
        }
        if !self.platform.has_route(&to) {
            return Ok(Some(answer(404, &format!("Route {to} not found"))));
        }
        if self.platform.is_private(&to) == Some(true) {
            return Ok(Some(answer(403, &format!("{to} is private"))));
        }
        if is_async {
            // drop-n-forget: deliver and acknowledge (Java 202 ack shape)
            po.send(inner).await?;
            let ack = EventEnvelope::new()
                .set_status(202)
                .set_body(serde_json::json!({
                    "type": "async",
                    "delivered": true,
                    "time": crate::trace::iso8601_utc_now(),
                }))?;
            Ok(Some(reply(200, ack.to_bytes()?)))
        } else if accepts_sse {
            // streaming-capable relay: the edge dispatched this call through a
            // dedicated reply lane (stream_dispatch, envelope mode) - rewire
            // the inner request onto that lane so the target streams straight
            // to it, with the edge context id as the correlation id (exactly
            // the rewrite the RPC inbox would make). A non-streaming target's
            // single reply takes the same lane and renders byte-identical to
            // the classic RPC response.
            let inner = inner.set_reply_to(reply_to).set_correlation_id(context_id);
            po.send(inner).await?;
            Ok(None)
        } else {
            // RPC: forward and mirror the target's envelope back (or 408).
            // A streaming reply cannot ride a single-shot response: answer
            // with an explicit refusal instead of a truncated first segment.
            match po.request(inner, Duration::from_millis(timeout_ms)).await {
                Ok(result) => {
                    if has_stream_marker(&result) {
                        Ok(Some(reply(
                            406,
                            error_envelope(406, STREAM_CALLER_REQUIRED),
                        )))
                    } else {
                        Ok(Some(reply(200, result.to_bytes()?)))
                    }
                }
                Err(e) => Ok(Some(reply(408, error_envelope(408, e.message())))),
            }
        }
    }
}

/// True when the envelope carries the reserved x-event-stream marker.
fn has_stream_marker(event: &EventEnvelope) -> bool {
    event
        .headers()
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case(crate::event_stream::X_EVENT_STREAM))
}

/// Build the HTTP response envelope: the body is a serialized envelope carried
/// raw as `application/octet-stream` (Java `sendResponse`/`sendError`). The
/// outer status is 200 for a successful dispatch (the real result status rides
/// inside the serialized body) and the error code for a service-level error.
fn reply(http_status: i32, body: Vec<u8>) -> EventEnvelope {
    EventEnvelope::new()
        .set_status(http_status)
        .set_header("content-type", OCTET_STREAM)
        .set_raw_body(Value::Binary(body))
}

/// A serialized error envelope (status + message body) — the payload the
/// client deserializes and hands back to its caller.
fn error_envelope(status: i32, message: &str) -> Vec<u8> {
    EventEnvelope::new()
        .set_status(status)
        .set_raw_body(Value::from(message))
        .to_bytes()
        .unwrap_or_default()
}

/// A compact (legacy Java) envelope has ONLY single-character top-level keys;
/// the standard format's keys are all longer, so the namespaces are disjoint.
fn is_compact_envelope(bytes: &[u8]) -> bool {
    match rmp_serde::from_slice::<Value>(bytes) {
        Ok(Value::Map(entries)) if !entries.is_empty() => entries
            .iter()
            .all(|(k, _)| k.as_str().is_some_and(|s| s.chars().count() == 1)),
        _ => false,
    }
}

/// Event-over-http client (Java `EventEmitter.asyncRequest`/`eRequest` over an
/// endpoint): POST `event` to `{endpoint}` as a serialized standard envelope.
/// `rpc = true` awaits the target's reply up to `timeout`; `rpc = false` is
/// drop-n-forget (the 202 ack envelope is returned). Trace context propagates
/// via `x-trace-id` + W3C `traceparent` so cross-language traces chain.
pub async fn event_over_http(
    po: &PostOffice,
    endpoint: &str,
    event: EventEnvelope,
    timeout: Duration,
    rpc: bool,
) -> Result<EventEnvelope, AppError> {
    static NO_HEADERS: OnceLock<HashMap<String, String>> = OnceLock::new();
    event_over_http_with_headers(
        po,
        endpoint,
        event,
        timeout,
        rpc,
        NO_HEADERS.get_or_init(HashMap::new),
    )
    .await
}

/// [`event_over_http`] with additional per-call HTTP headers — the carrier of
/// the per-target security headers (e.g. `authorization`) declared in
/// `yaml.event.over.http` (Java `EventEmitter.asyncRequest(event, timeout,
/// headers, endpoint, rpc)`).
pub async fn event_over_http_with_headers(
    po: &PostOffice,
    endpoint: &str,
    event: EventEnvelope,
    timeout: Duration,
    rpc: bool,
    security_headers: &HashMap<String, String>,
) -> Result<EventEnvelope, AppError> {
    let (host, path) = split_endpoint(endpoint)?;
    // stamp the calling function's trace context onto the WIRE envelope
    // (fill-if-absent trace id/path; the caller's span unconditionally) —
    // Java parity: the trace-aware po.request(..., endpoint, rpc) touches the
    // event before serialization, so the remote function parents onto the
    // caller's span. The declarative hook already applied this in request();
    // a second application is idempotent. A PROGRAMMATIC caller passing a
    // fresh envelope gets the same lineage automatically.
    let event = crate::post_office::apply_current_trace(event);
    let trace_id = event.trace_id().map(str::to_string);
    let span_id = event.span_id().map(str::to_string);
    let payload = event.to_bytes()?;
    let mut http = AsyncHttpRequest::new()
        .set_method("POST")
        .set_url(&path)
        .set_target_host(&host)
        .set_header("content-type", OCTET_STREAM)
        // Java EventEmitter wire parity: both engines' /api/event requests
        // carry the same header set (x-small-payload-as-bytes + accept)
        .set_header(X_NO_STREAM, "true")
        .set_header("accept", "*/*")
        .set_header(X_TTL, &timeout.as_millis().max(1000).to_string())
        // a client-side instruction, consumed by the async HTTP client and
        // never sent to the peer (the x-event-format precedent in Java):
        // this is the engine's own Event-over-HTTP transport leg, so the
        // client must not stamp the business correlation-id header on it —
        // the business cid rides INSIDE the envelope (my_cid tag; Java's
        // EventEmitter leg carries no ambient business context)
        .set_header(X_EVENT_API, "true")
        .set_body(Value::Binary(payload));
    if !rpc {
        http = http.set_header(X_ASYNC, "true");
    }
    // per-target security headers ride the HTTP call; the framework headers
    // above stay authoritative on a name clash (first match wins on read)
    for (key, value) in security_headers {
        http = http.set_header(key, value);
    }
    http = stamp_trace_headers(http, trace_id.as_deref(), span_id.as_deref());
    let http_event = EventEnvelope::new()
        .set_to(ASYNC_HTTP_REQUEST)
        .set_raw_body(http.to_value());
    // the local wait gets a small grace over the remote TTL (Java parity:
    // EventEmitter's inner deadline is timeout + 100ms) so a peer that spends
    // its whole TTL still replies in-band — its 408 envelope must win the
    // race against the local abort, never lose it. This internal RPC bypasses
    // the declarative hook (request_direct) — the HTTP client leg of a
    // forward must never consult the registry itself.
    let response = po
        .request_direct(http_event, timeout + Duration::from_millis(100))
        .await?;
    // the response body is the serialized reply envelope (octet-stream →
    // binary); a non-envelope response — e.g. an authentication-layer 401 in
    // the REST error JSON shape, produced before the Event API service ever
    // ran — is returned as-is with its HTTP status (Java parity:
    // EventEmitter.handleFutureResponse)
    match response.body() {
        Value::Binary(bytes) => EventEnvelope::from_bytes(bytes),
        _ => Ok(response),
    }
}

/// Trace propagation of an Event-over-HTTP call (Java sets both headers so the
/// receiver chains onto this span as its parent): `x-trace-id` plus the W3C
/// `traceparent`. When a custom traceparent header name is configured
/// (`http.traceparent.header`), the same value is stamped under that name too,
/// so the trace context survives an intermediary that strips the standard header.
fn stamp_trace_headers(
    mut http: AsyncHttpRequest,
    trace_id: Option<&str>,
    span_id: Option<&str>,
) -> AsyncHttpRequest {
    if let Some(trace_id) = trace_id {
        http = http.set_header("x-trace-id", trace_id);
        if let Some(span_id) = span_id {
            if let Some(traceparent) = w3c_trace::format(trace_id, span_id) {
                http = http.set_header(w3c_trace::TRACEPARENT, &traceparent);
                let custom_traceparent = AppConfigReader::get_instance()
                    .get_property_or("http.traceparent.header", w3c_trace::TRACEPARENT);
                if !custom_traceparent.eq_ignore_ascii_case(w3c_trace::TRACEPARENT) {
                    http = http.set_header(&custom_traceparent, &traceparent);
                }
            }
        }
    }
    http
}

/// Split `http://host:port/api/event` into (`http://host:port`, `/api/event`).
fn split_endpoint(endpoint: &str) -> Result<(String, String), AppError> {
    let scheme_end = endpoint
        .find("://")
        .map(|i| i + 3)
        .ok_or_else(|| AppError::new(400, format!("Invalid endpoint {endpoint}")))?;
    match endpoint[scheme_end..].find('/') {
        Some(offset) => {
            let split = scheme_end + offset;
            Ok((endpoint[..split].to_string(), endpoint[split..].to_string()))
        }
        None => Ok((endpoint.to_string(), "/api/event".to_string())),
    }
}

// ---- declarative Event over HTTP (Java `yaml.event.over.http`) ----

/// One declarative routing entry: the peer's `/api/event` URL plus optional
/// per-target security headers (Java `EventEmitter.eventHttpTargets` +
/// `eventHttpHeaders`, folded into one struct).
#[derive(Debug)]
pub struct EventHttpTarget {
    /// The peer endpoint, e.g. `http://127.0.0.1:8085/api/event`.
    pub target: String,
    /// HTTP headers added to every forwarded call (e.g. `authorization`).
    pub headers: HashMap<String, String>,
}

/// The route → target map, loaded once on first use (Java loads it in the
/// `EventEmitter` constructor; the Rust port has no such singleton, so the
/// registry initializes lazily from the same configuration).
fn event_http_registry() -> &'static HashMap<String, EventHttpTarget> {
    static REGISTRY: OnceLock<HashMap<String, EventHttpTarget>> = OnceLock::new();
    REGISTRY.get_or_init(load_event_http_routes)
}

/// Declarative Event-over-HTTP lookup (Java `EventEmitter.getEventHttpTarget`
/// and `getEventHttpHeaders`): the configured target for a route, with any
/// `@instance` suffix stripped for the lookup. `None` = the route is local.
pub fn get_event_http_target(route: &str) -> Option<&'static EventHttpTarget> {
    let base = match route.find('@') {
        Some(at) => &route[..at],
        None => route,
    };
    event_http_registry().get(base)
}

/// Load the `event.http[]` entries from the file named by
/// `yaml.event.over.http` (default `classpath:/event-over-http.yaml`).
/// An absent file simply disables the feature; `${...}` references in the
/// values (environment variables, base configuration keys) resolve at load
/// time (Java `EventEmitter.loadHttpRoutes`).
fn load_event_http_routes() -> HashMap<String, EventHttpTarget> {
    let mut targets = HashMap::new();
    let explicit = AppConfigReader::get_instance().get_property(EVENT_OVER_HTTP_YAML);
    let path = explicit
        .clone()
        .unwrap_or_else(|| DEFAULT_EVENT_OVER_HTTP_YAML.to_string());
    let reader = match ConfigReader::load(&path) {
        Ok(reader) => reader,
        Err(e) => {
            // only an explicitly configured file is worth an error — the
            // default location is optional by design
            if explicit.is_some() {
                log::error!("Unable to load event-over-http config - {e}");
            }
            return targets;
        }
    };
    let Some(ConfigValue::List(entries)) = reader.get("event.http") else {
        log::error!(
            "Invalid config {path} - the event.http section should be a list of route and target"
        );
        return targets;
    };
    for i in 0..entries.len() {
        let route = reader
            .get_property(&format!("event.http[{i}].route"))
            .unwrap_or_default();
        let target = reader
            .get_property(&format!("event.http[{i}].target"))
            .unwrap_or_default();
        if route.is_empty() || target.is_empty() {
            continue;
        }
        if crate::platform::validate_route(&route).is_err() {
            log::error!("Invalid Event over HTTP config entry - check route {route}");
            continue;
        }
        if split_endpoint(&target).is_err() {
            log::error!("Invalid Event over HTTP config entry - check target {target}");
            continue;
        }
        let mut headers = HashMap::new();
        if let Some(ConfigValue::Map(map)) = reader.get(&format!("event.http[{i}].headers")) {
            for key in map.keys() {
                if let Some(value) = reader.get_property(&format!("event.http[{i}].headers.{key}"))
                {
                    headers.insert(key.clone(), value);
                }
            }
        }
        log::info!(
            "Event-over-HTTP {route} -> {target} with {} header{}",
            headers.len(),
            if headers.len() == 1 { "" } else { "s" }
        );
        targets.insert(route, EventHttpTarget { target, headers });
    }
    log::info!(
        "Total {} event-over-http target{} configured",
        targets.len(),
        if targets.len() == 1 { "" } else { "s" }
    );
    targets
}

/// The send-path forward (Java `EventEmitter.sendWithEventHttp`): an event
/// whose route is declaratively mapped crosses to the peer instead of the
/// local bus. With a `reply_to` it is a **callback**: the reply address is
/// withheld from the wire, the forward runs as RPC, and the peer's response
/// is delivered to the original `reply_to` locally (restoring `from`, trace,
/// and the business correlation-id). Without one it is **async**: forwarded
/// drop-n-forget, expecting the peer's 202 ack. Both run detached — like
/// Java, `send` returns as soon as the forward is scheduled.
pub(crate) fn send_with_event_http(
    platform: &Platform,
    event: EventEnvelope,
    to: &str,
    entry: &'static EventHttpTarget,
) -> Result<(), AppError> {
    let callback = event.reply_to().map(str::to_string);
    if let Some(callback) = &callback {
        if accepts_event_stream_header(&event) {
            // the event-level opt-in for progressive streaming over
            // Event-over-HTTP: "accept: text/event-stream"
            return relay_event_stream(platform, event, to, entry, callback.clone());
        }
    }
    let event_api_type = if callback.is_some() {
        "callback"
    } else {
        "async"
    };
    let trace_id = event.trace_id().map(str::to_string);
    let trace_path = event.trace_path().map(str::to_string);
    let cid = event.correlation_id().map(str::to_string);
    let forward = event
        .clear_reply_to()
        .set_header(X_EVENT_API, event_api_type);
    let platform = platform.clone();
    let to = to.to_string();
    tokio::spawn(async move {
        let po = PostOffice::new(&platform);
        let outcome = event_over_http_with_headers(
            &po,
            &entry.target,
            forward,
            ASYNC_EVENT_HTTP_TIMEOUT,
            callback.is_some(),
            &entry.headers,
        )
        .await;
        match outcome {
            Ok(reply) => {
                if let Some(callback) = callback {
                    // deliver the peer's response to the original reply_to
                    // locally, restoring sender, trace, and correlation-id
                    let mut response = reply.set_to(&callback).clear_reply_to().set_from(&to);
                    if let (Some(id), Some(path)) = (&trace_id, &trace_path) {
                        response = response.set_trace(id, path);
                    }
                    if let Some(cid) = &cid {
                        response = response.set_correlation_id(cid);
                    }
                    if let Err(e) = po.send(response).await {
                        log::error!(
                            "Error in sending callback event {to} from {} to {callback} - {}",
                            entry.target,
                            e.message()
                        );
                    }
                } else if reply.status() != 202 {
                    log::error!(
                        "Error in sending async event {to} to {} - status={}, error={}",
                        entry.target,
                        reply.status(),
                        reply
                            .body_as::<String>()
                            .unwrap_or_else(|_| format!("{}", reply.body()))
                    );
                }
            }
            Err(e) => {
                log::error!(
                    "Error in sending event {to} to {} - {}",
                    entry.target,
                    e.message()
                );
            }
        }
    });
    Ok(())
}

/// The event-level opt-in for progressive streaming over Event-over-HTTP:
/// the outbound event declares the header `accept: text/event-stream`.
fn accepts_event_stream_header(event: &EventEnvelope) -> bool {
    event.headers().iter().any(|(name, value)| {
        name.eq_ignore_ascii_case("accept") && value.contains(TEXT_EVENT_STREAM)
    })
}

/// Streaming-capable Event-over-HTTP relay (callback mode with the accept
/// opt-in, Java `EventEmitter.relayEventStream`): the POST advertises
/// `Accept: text/event-stream` and the caller's reply route and correlation id
/// pass through to the HTTP client, which consumes the peer's SSE response
/// progressively (the envelope-mode wire dialect) and forwards each decoded
/// event to the callback. A peer that answers single-shot - a non-streaming
/// target, or an older engine - falls back to the classic callback delivery.
/// The event's `x-ttl` header (milliseconds, default 60 seconds) is the idle
/// allowance between stream events on both hops.
fn relay_event_stream(
    platform: &Platform,
    event: EventEnvelope,
    to: &str,
    entry: &'static EventHttpTarget,
    callback: String,
) -> Result<(), AppError> {
    let ttl_ms = event
        .headers()
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(X_TTL))
        .and_then(|(_, value)| value.trim().parse::<u64>().ok())
        .map_or(ASYNC_EVENT_HTTP_TIMEOUT.as_millis() as u64, |v| v.max(1000));
    // the wire envelope carries the caller's trace lineage (fill-if-absent id
    // and path, the caller's span) so the remote function parents onto it
    let forward = crate::post_office::apply_current_trace(
        event
            .clear_reply_to()
            .set_header(X_EVENT_API, crate::automation::http_client::STREAM_RELAY),
    );
    let cid = forward.correlation_id().map(str::to_string);
    let trace_id = forward.trace_id().map(str::to_string);
    let trace_path = forward.trace_path().map(str::to_string);
    let span_id = forward.span_id().map(str::to_string);
    let platform = platform.clone();
    let to = to.to_string();
    tokio::spawn(async move {
        let (host, path) = match split_endpoint(&entry.target) {
            Ok(parts) => parts,
            Err(e) => {
                log::error!(
                    "Unable to relay event stream {to} to {} - {}",
                    entry.target,
                    e.message()
                );
                return;
            }
        };
        let payload = match forward.to_bytes() {
            Ok(bytes) => bytes,
            Err(e) => {
                log::error!(
                    "Unable to relay event stream {to} to {} - {}",
                    entry.target,
                    e.message()
                );
                return;
            }
        };
        let mut http = AsyncHttpRequest::new()
            .set_method("POST")
            .set_url(&path)
            .set_target_host(&host)
            .set_header("content-type", OCTET_STREAM)
            .set_header(X_NO_STREAM, "true")
            .set_header("accept", TEXT_EVENT_STREAM)
            .set_header(X_TTL, &ttl_ms.to_string())
            // the engine's own transport leg: the client must not stamp the
            // business correlation-id header on it (event_over_http parity)
            .set_header(X_EVENT_API, "true")
            .set_body(Value::Binary(payload));
        for (key, value) in &entry.headers {
            http = http.set_header(key, value);
        }
        http = stamp_trace_headers(http, trace_id.as_deref(), span_id.as_deref());
        let mut http_event = EventEnvelope::new()
            .set_to(ASYNC_HTTP_REQUEST)
            .set_raw_body(http.to_value())
            .set_reply_to(&callback)
            .set_from(&to)
            .set_header(X_EVENT_API, crate::automation::http_client::STREAM_RELAY);
        if let Some(cid) = &cid {
            http_event = http_event.set_correlation_id(cid);
        }
        if let (Some(id), Some(path)) = (&trace_id, &trace_path) {
            http_event = http_event.set_trace(id, path);
        }
        let po = PostOffice::new(&platform);
        if let Err(e) = po.send(http_event).await {
            log::error!(
                "Unable to relay event stream {to} to {} - {}",
                entry.target,
                e.message()
            );
        }
    });
    Ok(())
}
