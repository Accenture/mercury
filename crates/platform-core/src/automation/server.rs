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

//! The HTTP protocol boundary — Rust port of the Java `HttpRouter` dispatch
//! (`org.platformlambda.automation.services.HttpRouter`), on **hyper**
//! (design D10: `rest.yaml` *is* the router, so no web framework).
//!
//! For each request: match the routing table → CORS preflight for `OPTIONS` →
//! apply request-header transforms → **ensure a business correlation-id**
//! (always, independent of tracing) → **start a trace** when the entry says
//! `tracing: true` (a valid W3C `traceparent` wins and contributes the
//! caller's span as our parent; else the trace-id header; else generated) →
//! optional authentication (an RPC; verdict headers become **session info**)
//! → build the `AsyncHttpRequest`-shaped event → **CALLBACK dispatch** to the
//! target service (Java `HttpRouter` parity: the event carries
//! `reply_to = async.http.response` and `cid` = the HTTP context id, so the
//! endpoint service's worker self-records its span — the first leg is a real
//! span record — and the response leg is itself a function span; the business
//! correlation-id rides the `my_correlation_id` envelope header) → the
//! [`AsyncHttpResponseService`] correlates the reply back to the waiting
//! connection → map the response envelope back to HTTP (status, body by type,
//! response-header transforms + CORS headers; the reserved `my_*` metadata is
//! stripped, Java `copyResponseHeaders` parity). Errors use the Java JSON
//! shape `{status, message, type: "error"}`.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex, OnceLock};
use std::task::{Context, Poll};
use std::time::Duration;

use async_trait::async_trait;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Frame};
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::sync::{mpsc, oneshot};

use crate::envelope::EventEnvelope;
use crate::event_stream;
use crate::function::{AppError, ComposableFunction};
use crate::platform::Platform;
use crate::post_office::PostOffice;
use crate::trace;
use crate::util::app_config_reader::AppConfigReader;
use crate::util::config_reader::ConfigReader;
use crate::util::w3c_trace;

use super::routing::{AssignedRoute, RouteInfo, RoutingTable};

/// Reserved read-only request header exposing the business correlation-id to
/// the target function (Java `HttpRouter.MY_CORRELATION_ID`).
pub const MY_CORRELATION_ID: &str = "my_correlation_id";

/// Route of the HTTP response-correlation service (Java
/// `AsyncHttpClient.ASYNC_HTTP_RESPONSE`).
pub const ASYNC_HTTP_RESPONSE: &str = "async.http.response";

/// Route-name base of the streaming reply-lane route pool (Java
/// `AsyncHttpClient.ASYNC_HTTP_RESPONSE_STREAM_POOL`). A streaming request
/// checks out one dedicated single-instance lane for its lifetime, so its
/// segments render in strict FIFO order while different requests stream
/// concurrently through their own lanes.
pub const ASYNC_HTTP_RESPONSE_STREAM_POOL: &str = "async.http.response.stream";

/// Shared by `async.http.response` and the streaming reply-lane pool
/// (one lane per instance — Java `AppStarter.RESPONSE_HANDLER_INSTANCES`).
const RESPONSE_HANDLER_INSTANCES: usize = 500;

/// Buffered segment events per in-flight stream (producer → renderer).
const STREAM_EVENT_BUFFER: usize = 64;
/// Buffered wire frames per in-flight stream (renderer → socket).
const STREAM_FRAME_BUFFER: usize = 64;

/// The response body type: complete payloads and progressive streams share
/// one boxed body so every handler path composes (Java: vert.x chunked writes).
type HttpBody = BoxBody<Bytes, std::convert::Infallible>;

/// A complete in-memory response body.
fn full(bytes: Bytes) -> HttpBody {
    BoxBody::new(Full::new(bytes))
}

/// Available streaming reply lanes — a LIFO stack (the "ready" signal pattern
/// of the reactive manager/worker design): checkout takes the most recently
/// released lane; release returns it. Filled once at server start.
fn lane_pool() -> &'static Mutex<Vec<String>> {
    static POOL: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    POOL.get_or_init(|| Mutex::new(Vec::new()))
}

/// Check out a dedicated ordered reply lane for one streaming request.
/// Returns None when the pool is exhausted.
pub fn checkout_lane() -> Option<String> {
    lane_pool().lock().expect("lane pool poisoned").pop()
}

/// Return a reply lane to the pool — called when the owning request ends,
/// and at startup to fill the pool.
pub fn release_lane(route: String) {
    lane_pool().lock().expect("lane pool poisoned").push(route);
}

/// The number of reply lanes currently available for checkout.
pub fn available_lanes() -> usize {
    lane_pool().lock().expect("lane pool poisoned").len()
}

/// In-flight streaming HTTP contexts — each entry forwards segment events
/// from the request's reply lane to its renderer task (Java: the
/// AsyncContextHolder + EventStreamState pair).
fn pending_streams() -> &'static Mutex<HashMap<String, mpsc::Sender<EventEnvelope>>> {
    static PENDING: OnceLock<Mutex<HashMap<String, mpsc::Sender<EventEnvelope>>>> = OnceLock::new();
    PENDING.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Remove a streaming context and return its lane to the pool. The map
/// removal is the exactly-once gate (Java `HttpRouter.closeContext`).
fn cleanup_stream(context_id: &str, lane: &str) {
    let removed = pending_streams()
        .lock()
        .expect("pending streams poisoned")
        .remove(context_id);
    if removed.is_some() {
        release_lane(lane.to_string());
    }
}

/// The streaming reply-lane service — one shared handler behind every
/// `async.http.response.stream.{n}` route (each registered with a single
/// instance, so per-request segment order is preserved end-to-end). It
/// forwards each event into the owning request's renderer; a missing context
/// (completed, timed out or disconnected) makes late segments no-op drops.
pub struct StreamLaneService;

#[async_trait]
impl ComposableFunction for StreamLaneService {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if let Some(context_id) = input.correlation_id().map(str::to_string) {
            let sender = pending_streams()
                .lock()
                .expect("pending streams poisoned")
                .get(&context_id)
                .cloned();
            if let Some(sender) = sender {
                // bounded back-pressure toward the renderer; a dropped
                // receiver (client gone) turns this into a no-op drop
                let _ = sender.send(input).await;
            }
        }
        Ok(EventEnvelope::new())
    }
}

/// A channel-backed streaming response body: the renderer task pushes wire
/// frames; hyper pulls them as the socket drains. Dropping the sender ends
/// the response body.
struct ChannelBody {
    rx: mpsc::Receiver<Frame<Bytes>>,
}

impl hyper::body::Body for ChannelBody {
    type Data = Bytes;
    type Error = std::convert::Infallible;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, Self::Error>>> {
        self.rx.poll_recv(cx).map(|frame| frame.map(Ok))
    }
}

/// SSE keep-alive comment interval in ms (`event.stream.keep.alive`,
/// default 30s; 0 disables — Java parity).
fn keep_alive_ms() -> u64 {
    static KEEP_ALIVE: OnceLock<u64> = OnceLock::new();
    *KEEP_ALIVE.get_or_init(|| {
        let config = AppConfigReader::get_instance();
        let text = config.get_property_or("event.stream.keep.alive", "30s");
        let trimmed = text.trim().to_lowercase();
        if trimmed == "0" || trimmed == "0s" || trimmed == "0ms" || trimmed == "0m" {
            0
        } else {
            super::routing::parse_timeout(Some(&trimmed)).as_millis() as u64
        }
    })
}

/// Reserved `my_*` metadata headers that must never reach the HTTP wire
/// (Java `WorkerHandler.copyResponseHeaders` protected-metadata handling).
const PROTECTED_METADATA: [&str; 5] = [
    "my_route",
    "my_trace_id",
    "my_trace_path",
    MY_CORRELATION_ID,
    "x-event-api",
];

/// Pending HTTP contexts awaiting their response envelope — keyed by the
/// per-request context id that rides the dispatched event's `cid`
/// (Java `HttpRouter` contexts + `AsyncContextHolder`).
fn pending_responses() -> &'static Mutex<HashMap<String, oneshot::Sender<EventEnvelope>>> {
    static PENDING: OnceLock<Mutex<HashMap<String, oneshot::Sender<EventEnvelope>>>> =
        OnceLock::new();
    PENDING.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The `async.http.response` service (Java `AsyncHttpResponse`) — the HTTP
/// response leg as a REAL registered function: a REST-automation dispatch is a
/// **callback** to the endpoint service, whose reply (or a flow's response)
/// arrives here carrying the HTTP context id as its correlation id, and this
/// service hands the envelope back to the waiting connection. Because it is
/// an ordinary traced worker, the response leg is a visible span that parents
/// onto the replying function's span — exactly the Java reference topology.
/// A missing context (the connection timed out) drops the reply silently.
pub struct AsyncHttpResponseService;

#[async_trait]
impl ComposableFunction for AsyncHttpResponseService {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if let Some(context_id) = input.correlation_id().map(str::to_string) {
            let sender = pending_responses()
                .lock()
                .expect("pending http contexts poisoned")
                .remove(&context_id);
            if let Some(sender) = sender {
                let _ = sender.send(input);
            }
        }
        Ok(EventEnvelope::new())
    }
}

/// The address the first HTTP server bound to in this process (first-bind
/// wins; `start_http_server` still binds a fresh listener on every call).
/// Intended for a test or embedder that boots the app once on an ephemeral
/// port (`rest.server.port=0`) and needs the assigned port afterwards.
static SERVER_ADDR: OnceLock<SocketAddr> = OnceLock::new();

/// The address the HTTP server bound to, if one has started (see
/// [`SERVER_ADDR`]). With `rest.server.port=0` (ephemeral) this is how a
/// single-server app recovers the port the OS assigned at bind time.
pub fn server_address() -> Option<SocketAddr> {
    SERVER_ADDR.get().copied()
}

struct RouterState {
    table: RoutingTable,
    platform: Platform,
    trace_header: String,
    cid_header: String,
    /// Configurable traceparent header name (`http.traceparent.header`): an
    /// escape hatch for an intermediary (e.g. an API gateway) that strips the
    /// standard W3C `traceparent` header. When customized, the same W3C-format
    /// value travels under BOTH names on outbound calls. Inbound, the standard
    /// `traceparent` always wins; the custom name is read only when the
    /// standard header is absent or malformed — a well-formed standard
    /// traceparent means the caller already speaks W3C/OTel, so a proprietary
    /// header alongside it is residual and safely ignored.
    traceparent_header: String,
}

/// Start the REST automation server (Java: the Vert.x HTTP server started by
/// `AppStarter` when `rest.automation=true`). Reads `rest.yaml` from
/// `yaml.rest.automation` (default `classpath:/rest.yaml`) and binds
/// `rest.server.port` (default 8085; port 0 = ephemeral, for tests). Returns
/// the bound address; the accept loop runs as a background task.
pub async fn start_http_server(platform: &Platform) -> Result<SocketAddr, AppError> {
    let config = AppConfigReader::get_instance();
    // the response-correlation service is part of the HTTP boundary itself
    // (Java AppStarter registers AsyncHttpResponse with the server, private,
    // 500 instances); idempotent — tolerate a concurrent registration
    if !platform.has_route(ASYNC_HTTP_RESPONSE) {
        if let Err(e) = platform.register_private(
            ASYNC_HTTP_RESPONSE,
            Arc::new(AsyncHttpResponseService),
            RESPONSE_HANDLER_INSTANCES,
        ) {
            if !platform.has_route(ASYNC_HTTP_RESPONSE) {
                return Err(e);
            }
        }
    }
    // streaming responses use a route pool of dedicated single-instance reply
    // lanes: a streaming request checks out one lane for its lifetime (strict
    // FIFO for its segments) and returns it when its context closes; the pool
    // size matches the async.http.response instances, and an idle lane costs
    // only a little memory (Java AppStarter parity). Registration runs on
    // EVERY server start — the pool reload rebinds the lane workers to the
    // current runtime (the per-test-runtime idiom of this port) — but the
    // checkout POOL is filled exactly once per process: get_or_init blocks a
    // concurrent second server start until the fill completes, so the pool can
    // never be refilled or double-filled while requests are in flight
    let members = platform.register_route_pool(
        ASYNC_HTTP_RESPONSE_STREAM_POOL,
        Arc::new(StreamLaneService),
        RESPONSE_HANDLER_INSTANCES,
    )?;
    static POOL_FILLED: OnceLock<()> = OnceLock::new();
    POOL_FILLED.get_or_init(|| {
        for lane_route in members {
            release_lane(lane_route);
        }
    });
    let rest_yaml = config.get_property_or("yaml.rest.automation", "classpath:/rest.yaml");
    let reader = ConfigReader::load(&rest_yaml)
        .map_err(|e| AppError::new(500, format!("Unable to load {rest_yaml} - {e}")))?;
    let mut table = RoutingTable::load(&reader)?;
    merge_default_endpoints(&mut table)?;
    let table = table;
    for route in table.routes() {
        log::info!(
            "{} {} -> {}",
            route.methods.join(","),
            route.url,
            route.service
        );
    }
    let port: u16 = config
        .get_property_or("rest.server.port", "8085")
        .parse()
        .map_err(|_| AppError::new(500, "Invalid rest.server.port"))?;
    let state = Arc::new(RouterState {
        table,
        platform: platform.clone(),
        trace_header: config.get_property_or("http.trace.id.header", "X-Trace-Id"),
        cid_header: config.get_property_or("http.correlation.id.header", "X-Correlation-Id"),
        traceparent_header: config
            .get_property_or("http.traceparent.header", w3c_trace::TRACEPARENT),
    });
    // startup announcement of the resolved header names (Java HttpRouter
    // parity — same wording, presentation parity for side-by-side log review)
    log::info!("Correlation-id HTTP header is '{}'", state.cid_header);
    log::info!("Trace-id HTTP header is '{}'", state.trace_header);
    log::info!("Traceparent HTTP header is '{}'", state.traceparent_header);
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .map_err(|e| AppError::new(500, format!("Unable to bind port {port} - {e}")))?;
    let addr = listener
        .local_addr()
        .map_err(|e| AppError::new(500, e.to_string()))?;
    let _ = SERVER_ADDR.set(addr);
    log::info!("REST automation service started on port {}", addr.port());
    tokio::spawn(async move {
        loop {
            let Ok((stream, peer)) = listener.accept().await else {
                break;
            };
            let state = state.clone();
            tokio::spawn(async move {
                let io = TokioIo::new(stream);
                let service = service_fn(move |request| {
                    let state = state.clone();
                    async move { handle(state, request, peer).await }
                });
                if let Err(e) = hyper::server::conn::http1::Builder::new()
                    .serve_connection(io, service)
                    .with_upgrades()
                    .await
                {
                    log::debug!("HTTP connection ended - {e}");
                }
            });
        }
    });
    Ok(addr)
}

async fn handle(
    state: Arc<RouterState>,
    request: Request<hyper::body::Incoming>,
    peer: SocketAddr,
) -> Result<Response<HttpBody>, hyper::Error> {
    // websocket upgrade on a registered `/ws/{name}/{token}` path takes the
    // connection out of the HTTP request/response cycle (Java parity)
    if super::ws_server::is_ws_upgrade(&request) {
        return Ok(super::ws_server::handle_ws_upgrade(
            &state.platform,
            request,
            peer.ip().to_string(),
        )
        .map(BoxBody::new));
    }
    let method = request.method().as_str().to_uppercase();
    let path = request.uri().path().to_string();
    let query_text = request.uri().query().unwrap_or("").to_string();
    // header map (lowercase names — deterministic matching)
    let mut headers: HashMap<String, String> = HashMap::new();
    for (name, value) in request.headers() {
        if let Ok(value) = value.to_str() {
            headers.insert(name.as_str().to_lowercase(), value.to_string());
        }
    }
    let body_bytes = match request.into_body().collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => Bytes::new(),
    };
    let Some(assigned) = state.table.find(&method, &path) else {
        // Java HttpRequestHandler: a known path under a WRONG method is 405,
        // never 404 (increment 56, parity F14c — the getSimilarRoute marker)
        if state.table.path_matches_any_method(&path) {
            return Ok(error_response(405, "Method not allowed"));
        }
        // static HTML content from resources/public — including "/" →
        // index.html — served only when rest.yaml claims no route (a "/"
        // entry in rest.yaml always wins)
        if method == "GET" || method == "HEAD" {
            if let Some(response) =
                serve_static(&state, &path, &query_text, &headers, peer, method == "HEAD").await
            {
                return Ok(response);
            }
        }
        return Ok(error_response(404, "Resource not found"));
    };
    // CORS preflight (OPTIONS is auto-added per the grammar). Java
    // handleOptionsMethod: without a CORS block (or with empty options) the
    // answer is 405 "Method not allowed", never a bare 204 (increment 56,
    // parity F14c)
    if method == "OPTIONS" {
        let Some(cors) = assigned
            .info
            .cors
            .as_ref()
            .filter(|c| !c.options.is_empty())
        else {
            return Ok(error_response(405, "Method not allowed"));
        };
        let mut response = Response::builder().status(StatusCode::NO_CONTENT);
        for (name, value) in &cors.options {
            response = response.header(name, value);
        }
        return Ok(response.body(full(Bytes::new())).expect("static response"));
    }
    match process(
        &state, assigned, method, path, query_text, headers, body_bytes, peer,
    )
    .await
    {
        Ok(response) => Ok(response),
        Err(e) => Ok(error_response(e.status(), e.message())),
    }
}

#[allow(clippy::too_many_arguments)]
async fn process(
    state: &RouterState,
    assigned: AssignedRoute<'_>,
    method: String,
    path: String,
    query_text: String,
    mut headers: HashMap<String, String>,
    body_bytes: Bytes,
    peer: SocketAddr,
) -> Result<Response<HttpBody>, AppError> {
    let info = assigned.info;
    // request-header transforms
    if let Some(header_info) = &info.headers {
        header_info.request.apply(&mut headers);
    }
    // event-script flow binding: rest.yaml `flow:` becomes the x-flow-id
    // header the flow adapter reads (Java parity; increment E-3)
    if let Some(flow) = &info.flow {
        headers.insert("x-flow-id".to_string(), flow.clone());
    }
    // effective header names (per-entry impedance override > global > default)
    let trace_header = info
        .trace_id_header
        .as_deref()
        .unwrap_or(&state.trace_header)
        .to_lowercase();
    let cid_header = info
        .correlation_id_header
        .as_deref()
        .unwrap_or(&state.cid_header)
        .to_lowercase();
    // trace resolution: a valid W3C traceparent wins and contributes the
    // caller's span as our parent; else the trace-id header; else generated.
    // The standard "traceparent" header always wins; the custom name
    // (per-entry 'traceparent.header' in rest.yaml, else the global
    // http.traceparent.header) is read only when the standard header is
    // absent or malformed. Rationale: a well-formed standard traceparent
    // means the caller already speaks the W3C/OpenTelemetry standard - a
    // proprietary header alongside it is residual and safely ignored.
    let traceparent = headers
        .get(w3c_trace::TRACEPARENT)
        .and_then(|value| w3c_trace::parse(value))
        .or_else(|| {
            let traceparent_header = info
                .traceparent_header
                .as_deref()
                .unwrap_or(&state.traceparent_header)
                .to_lowercase();
            if traceparent_header == w3c_trace::TRACEPARENT {
                None
            } else {
                headers
                    .get(&traceparent_header)
                    .and_then(|value| w3c_trace::parse(value))
            }
        });
    let (trace_id, parent_span) = match &traceparent {
        Some((trace_id, parent)) => (Some(trace_id.clone()), Some(parent.clone())),
        None => (headers.get(&trace_header).cloned(), None),
    };
    let trace_id = if info.tracing {
        Some(trace_id.unwrap_or_else(trace::new_trace_id))
    } else {
        None
    };
    // a business correlation-id is ALWAYS ensured, independent of tracing;
    // legacy conflation (one shared header name) yields one id, not two
    let cid = headers.get(&cid_header).cloned().unwrap_or_else(|| {
        if cid_header == trace_header {
            trace_id
                .clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().simple().to_string())
        } else {
            uuid::Uuid::new_v4().simple().to_string()
        }
    });
    // stamp the resolved correlation-id onto the request dataset under the
    // configured header name (Java parity): the target function and the flow
    // engine see the SAME edge-resolved value even when the caller sent none
    headers.insert(cid_header.clone(), cid.clone());
    // the endpoint timeout is represented AS the x-ttl request header in
    // milliseconds — Java parity: HttpRouter calls req.setTimeoutSeconds(
    // route timeout) at ingress and AsyncHttpRequest stores/reads the TTL as
    // this header (one representation), so a flow's input.header view carries
    // the same key on both engines. A caller-sent x-ttl WINS — Java copies
    // the inbound headers after the stamp, which is how the Event-over-HTTP
    // client's own TTL rides through the /api/event endpoint.
    headers
        .entry("x-ttl".to_string())
        .or_insert_with(|| (info.timeout.as_secs().max(1) * 1000).to_string());
    // AsyncHttpRequest-shaped event body (Java parity keys).
    // Repeated query parameters keep EVERY value — one occurrence is a
    // string, more become a list (Java HttpRouter: params.getAll;
    // increment 56, parity F14a — previously last-wins)
    let mut query: HashMap<String, serde_json::Value> = HashMap::new();
    for pair in query_text.split('&').filter(|p| !p.is_empty()) {
        let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
        let (name, value) = (url_decode(name), url_decode(value));
        match query.get_mut(&name) {
            None => {
                query.insert(name, serde_json::Value::String(value));
            }
            Some(serde_json::Value::Array(values)) => {
                values.push(serde_json::Value::String(value));
            }
            Some(existing) => {
                let first = existing.clone();
                *existing = serde_json::Value::Array(vec![first, serde_json::Value::String(value)]);
            }
        }
    }
    let path_params: HashMap<String, String> = assigned
        .path_params
        .iter()
        .map(|(k, v)| (k.clone(), url_decode(v)))
        .collect();
    // the cookie header becomes a parsed cookies map and is WITHHELD from
    // the request headers (Java setRequestCookies; increment 56, parity
    // F14d — previously the raw header rode through and no map existed)
    let cookies: HashMap<String, String> = headers
        .remove("cookie")
        .map(|header| {
            header
                .split(';')
                .filter_map(|item| item.split_once('='))
                .map(|(name, value)| (name.trim().to_string(), value.trim().to_string()))
                .collect()
        })
        .unwrap_or_default();
    // the request's Accept header drives the response's fallback content
    // negotiation (Java AsyncContextHolder.accept), captured before the
    // headers map moves into the event body
    let accept = headers.get("accept").cloned();
    let parsed = parse_body(&headers, &body_bytes);
    // form fields become query parameters, on top of the URL's own
    // (Java handleTextContent's url-encode branch: setQueryParameter each —
    // single values, replacing)
    if let ParsedBody::Form(form) = &parsed {
        for (name, value) in form {
            query.insert(name.clone(), serde_json::Value::String(value.clone()));
        }
    }
    // ONE definition of the wire shape: the dataset is constructed through
    // AsyncHttpRequest's fluent API and rendered by its to_value() — the
    // same builder/parser pair a typed function deserializes through, so
    // server↔struct drift is impossible by construction (previously this
    // was a hand-assembled JSON literal, which is exactly how the server
    // came to emit keys from_value never parsed).
    let mut http_request = crate::automation::AsyncHttpRequest::new()
        .set_method(&method)
        .set_url(&path)
        .set_remote_ip(&peer.ip().to_string())
        // Java: setSecure(x-forwarded-proto == "https") — increment 56,
        // parity F14d (previously hardcoded false)
        .set_secure(headers.get("x-forwarded-proto").map(String::as_str) == Some("https"))
        .set_target_host(&headers.get("host").cloned().unwrap_or_default())
        // Java AsyncHttpRequest.getTimeoutSeconds (the flow adapter derives
        // the flow TTL from it)
        .set_route_timeout_seconds(info.timeout.as_secs());
    for (key, value) in &headers {
        http_request = http_request.set_header(key, value);
    }
    for (key, value) in &path_params {
        http_request = http_request.set_path_parameter(key, value);
    }
    for (key, value) in &query {
        http_request = match value {
            serde_json::Value::Array(values) => {
                let values: Vec<&str> = values
                    .iter()
                    .map(|v| v.as_str().unwrap_or_default())
                    .collect();
                http_request.set_query_parameter_values(key, &values)
            }
            serde_json::Value::String(value) => http_request.set_query_parameter(key, value),
            other => http_request.set_query_parameter(key, &other.to_string()),
        };
    }
    // the request body: a JSON-shaped payload rides as-is; a binary body
    // (unknown content type) rides as MsgPack binary (Java: byte[] on the
    // AsyncHttpRequest); a form body already became query parameters, so
    // the body key stays an explicit null — exactly the previous shape
    http_request = match &parsed {
        ParsedBody::Value(value) => http_request
            .set_body(rmpv::ext::to_value(value).map_err(|e| AppError::new(500, e.to_string()))?),
        ParsedBody::Bytes(bytes) => http_request.set_body(rmpv::Value::Binary(bytes.clone())),
        ParsedBody::Form(_) => http_request.set_body(rmpv::Value::Nil),
    };
    // the raw query string rides as Java's top-level "query" key; cookies
    // appear only when present (Java toMap omits empty)
    if !query_text.is_empty() {
        http_request = http_request.set_query_string(&query_text);
    }
    for (key, value) in &cookies {
        http_request = http_request.set_cookie(key, value);
    }
    let po = PostOffice::new(&state.platform);
    // Java appends the query string to the trace path (HttpRouter)
    let trace_path = if query_text.is_empty() {
        format!("{method} {path}")
    } else {
        format!("{method} {path}?{query_text}")
    };
    // optional authentication before dispatch (simple route form) — an RPC,
    // so the auth verdict reports as a round_trip record (Java parity)
    if let Some(auth_route) = &info.authentication {
        let auth_event = build_event(
            auth_route,
            &http_request,
            &cid,
            &trace_id,
            &trace_path,
            &parent_span,
        )?;
        let verdict = po.request(auth_event, info.timeout).await?;
        if verdict.has_error() {
            return Err(AppError::new(
                verdict.status(),
                verdict
                    .body_as::<String>()
                    .unwrap_or_else(|_| "Unauthorized".to_string()),
            ));
        }
        if !verdict.body_as::<bool>().unwrap_or(false) {
            return Err(AppError::new(401, "Unauthorized"));
        }
        // headers on the auth verdict become SESSION INFO that rides to the
        // target function as read-only headers (Java HttpRouter parity —
        // e.g. the event.api.auth demo injects `user: demo`)
        for (key, value) in verdict.headers() {
            http_request = http_request.set_session_info(key, value);
        }
    }
    let is_head = method == "HEAD";
    // a streaming-capable /api/event call (Accept: text/event-stream, not
    // drop-n-forget) dispatches through a dedicated reply lane rendering the
    // envelope-mode wire dialect - so a remote peer's streaming function can
    // answer the one POST progressively; plain RPC calls never consume a lane
    let envelope_stream = !is_head && is_event_api_stream(info, &http_request);
    // a streaming endpoint (rest.yaml `stream: true`) uses the multi-shot
    // reply route; HEAD requests never stream (Java parity)
    let result = if (info.stream_response && !is_head) || envelope_stream {
        match stream_dispatch(
            state,
            info,
            &http_request,
            &cid,
            &cid_header,
            &trace_id,
            &trace_path,
            &parent_span,
            accept.clone(),
            envelope_stream,
        )
        .await?
        {
            StreamOutcome::Streaming(response) => return Ok(response),
            StreamOutcome::SingleShot(envelope) => envelope,
        }
    } else {
        // CALLBACK dispatch (Java HttpRouter parity): the endpoint service is
        // invoked with reply_to = async.http.response and cid = the HTTP context
        // id — its worker self-records its span (no RPC suppression), and the
        // response leg is a visible function span. The business correlation-id
        // rides the my_correlation_id envelope header instead of the cid slot.
        let context_id = uuid::Uuid::new_v4().simple().to_string();
        let (tx, rx) = oneshot::channel();
        pending_responses()
            .lock()
            .expect("pending http contexts poisoned")
            .insert(context_id.clone(), tx);
        let event = build_event(
            &info.service,
            &http_request,
            &cid,
            &trace_id,
            &trace_path,
            &parent_span,
        )?
        .set_correlation_id(&context_id)
        .set_reply_to(ASYNC_HTTP_RESPONSE);
        if let Err(e) = po.send(event).await {
            pending_responses()
                .lock()
                .expect("pending http contexts poisoned")
                .remove(&context_id);
            return Err(e);
        }
        match tokio::time::timeout(info.timeout, rx).await {
            Ok(Ok(envelope)) => envelope,
            Ok(Err(_)) => {
                return Err(AppError::new(500, "Response channel closed unexpectedly"));
            }
            Err(_) => {
                pending_responses()
                    .lock()
                    .expect("pending http contexts poisoned")
                    .remove(&context_id);
                return Err(AppError::new(
                    408,
                    format!("Timeout for {} ms", info.timeout.as_millis()),
                ));
            }
        }
    };
    // map the response envelope back to HTTP (Java AsyncHttpResponse:
    // updateHeadersAndContentType + updateHeaders)
    let status = status_of(result.status());
    let mut content_type: Option<String> = None;
    let mut set_cookies: Vec<String> = Vec::new();
    let mut response_headers: HashMap<String, String> = HashMap::new();
    for (name, value) in result.headers() {
        let key = name.to_lowercase();
        // the reserved my_* metadata never reaches the HTTP wire (Java
        // WorkerHandler.copyResponseHeaders protected-metadata parity)
        if PROTECTED_METADATA.contains(&key.as_str()) {
            continue;
        }
        match key.as_str() {
            // the response-streaming contract (x-stream-id + x-ttl) is a
            // documented deferral in this port (D10) — recognized like Java
            // and withheld from the wire, never leaked as literal headers
            "x-stream-id" if value.starts_with("stream.") && value.contains(".in") => {}
            "x-ttl" => {}
            // a function-set content type overrides negotiation
            // (Java: response.putHeader directly, lowercased; skipped for HEAD)
            "content-type" => {
                if !is_head {
                    content_type = Some(value.to_lowercase());
                }
            }
            // repeated cookies ride one envelope header, "|"-separated
            // (Java SimpleHttpUtility.setCookies -> one header line each)
            "set-cookie" => {
                set_cookies.extend(value.split('|').map(|c| c.trim().to_string()));
            }
            _ => {
                response_headers.insert(key, value.clone());
            }
        }
    }
    // Without a function-set type, the fallback comes from the request's
    // Accept header (Java updateContentType — increment 56, the negotiation
    // sub-item queued at increment 50; previously derived from body shape),
    // and map/list bodies render per the negotiated type (handleMapContent).
    if content_type.is_none() && !is_head {
        content_type = accept_fallback_type(accept.as_deref(), result.body());
    }
    let payload = render_payload(result.body(), content_type.as_deref());
    // the rest.yaml response transform filters the merged header map (Java
    // filterHeaders); content-type and cookies bypass it, as in Java
    if let Some(header_info) = &info.headers {
        header_info.response.apply(&mut response_headers);
    }
    // echo the request's business correlation-id (inbound or edge-generated)
    // under the configured header name so the caller can correlate without
    // parsing the body; a function-set response header of the same name wins
    // (Java AsyncHttpResponse parity)
    response_headers.entry(cid_header.clone()).or_insert(cid);
    if let Some(content_type) = content_type {
        response_headers.insert("content-type".to_string(), content_type);
    }
    if let Some(cors) = &info.cors {
        for (name, value) in &cors.headers {
            response_headers.insert(name.to_lowercase(), value.clone());
        }
    }
    let mut response = Response::builder().status(status);
    for (name, value) in response_headers {
        response = response.header(name, value);
    }
    for cookie in set_cookies {
        if !cookie.is_empty() {
            response = response.header("set-cookie", cookie);
        }
    }
    // a HEAD response never carries a body (Java: isHeadMethod skips content)
    let payload = if is_head { Bytes::new() } else { payload };
    response
        .body(full(payload))
        .map_err(|e| AppError::new(500, e.to_string()))
}

/// Outcome of a streaming dispatch: a committed progressive response, or the
/// first event turned out to be an ordinary single-shot reply.
/// (A short-lived by-value carrier - the size difference between variants is
/// one stack move per request, not worth a heap allocation.)
#[allow(clippy::large_enum_variant)]
enum StreamOutcome {
    Streaming(Response<HttpBody>),
    SingleShot(EventEnvelope),
}

/// The first event's stream marker: `Ok(Some(marker))` for a valid
/// `x-event-stream` value, `Ok(None)` when the header is absent (single-shot),
/// `Err(())` for a present-but-invalid value (drop the event, Java parity).
fn stream_marker(event: &EventEnvelope) -> Result<Option<&'static str>, ()> {
    for (name, value) in event.headers() {
        if name.eq_ignore_ascii_case(event_stream::X_EVENT_STREAM) {
            return match value.to_lowercase().as_str() {
                event_stream::DATA => Ok(Some(event_stream::DATA)),
                event_stream::EOF => Ok(Some(event_stream::EOF)),
                event_stream::EXCEPTION => Ok(Some(event_stream::EXCEPTION)),
                _ => Err(()),
            };
        }
    }
    Ok(None)
}

/// Error text from an exception event body (Java `EventStreamRenderer.errorMessage`).
fn stream_error_message(event: &EventEnvelope) -> String {
    match event.body() {
        rmpv::Value::Map(entries) => entries
            .iter()
            .find(|(key, _)| key.as_str() == Some("message"))
            .map(|(_, value)| stream_text(value))
            .unwrap_or_else(|| "Stream failed".to_string()),
        rmpv::Value::Nil => "Stream failed".to_string(),
        other => stream_text(other),
    }
}

/// The fallback content type for a streaming response from the request's
/// Accept header (Java `EventStreamRenderer.negotiateContentType`).
fn negotiate_stream_type(accept: Option<&str>) -> String {
    let Some(accept) = accept else {
        return "application/json".to_string();
    };
    if accept.contains("*/*") || accept.contains("application/json") {
        "application/json".to_string()
    } else if accept.contains("text/event-stream") {
        "text/event-stream".to_string()
    } else if accept.contains("text/html") {
        "text/html".to_string()
    } else if accept.contains("application/xml") {
        "application/xml".to_string()
    } else {
        "text/plain".to_string()
    }
}

/// A segment body as line-oriented text: strings ride as-is; binary as UTF-8;
/// structured bodies render as COMPACT one-line JSON — stream framing is
/// line-oriented on both engines (Java uses the compact Gson for frames).
fn stream_text(body: &rmpv::Value) -> String {
    match body {
        rmpv::Value::Nil => String::new(),
        rmpv::Value::String(text) => text.as_str().unwrap_or_default().to_string(),
        rmpv::Value::Binary(bytes) => String::from_utf8_lossy(bytes).to_string(),
        other => {
            let stripped = crate::serializer::strip_nulls(other);
            let json = serde_json::to_value(&stripped).unwrap_or_default();
            serde_json::to_string(&json).unwrap_or_default()
        }
    }
}

/// One SSE frame: optional `event:` line, one `data:` line per text line
/// (multi-line data splits per the SSE specification), then a blank line.
fn sse_frame(event_name: Option<&str>, text: &str) -> Bytes {
    let mut frame = String::new();
    if let Some(name) = event_name.filter(|n| !n.is_empty()) {
        frame.push_str("event: ");
        frame.push_str(name);
        frame.push('\n');
    }
    for line in text.split('\n') {
        frame.push_str("data: ");
        frame.push_str(line);
        frame.push('\n');
    }
    frame.push('\n');
    Bytes::from(frame)
}

/// One chunked-mode segment: strings and bytes append verbatim; structured
/// bodies stream as JSON Lines (one compact JSON object per line).
fn chunk_bytes(body: &rmpv::Value) -> Bytes {
    match body {
        rmpv::Value::Nil => Bytes::new(),
        rmpv::Value::String(text) => Bytes::from(text.as_str().unwrap_or_default().to_string()),
        rmpv::Value::Binary(bytes) => Bytes::from(bytes.clone()),
        other => {
            let mut line = stream_text(other);
            line.push('\n');
            Bytes::from(line)
        }
    }
}

/// The `x-event-name` companion header (SSE `event:` field), if any.
fn stream_event_name(event: &EventEnvelope) -> Option<&str> {
    event
        .headers()
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(event_stream::X_EVENT_NAME))
        .map(|(_, value)| value.as_str())
}

/// True when this request is a streaming-capable Event-over-HTTP call: the
/// /api/event service, invoked with `Accept: text/event-stream` and not
/// drop-n-forget. Such a call dispatches through a reply lane in envelope
/// mode - the EventApiService rewires the inner request onto the lane so a
/// streaming target's segments relay straight to the wire.
fn is_event_api_stream(info: &RouteInfo, request: &crate::automation::AsyncHttpRequest) -> bool {
    info.service == super::event_api::EVENT_API_SERVICE
        && request.header("x-async") != Some("true")
        && request
            .header("accept")
            .is_some_and(|accept| accept.contains("text/event-stream"))
}

/// The idle allowance of a streaming-capable Event-over-HTTP call: the POST's
/// x-ttl header in milliseconds (the caller's declaration), floor one second -
/// the same reading the EventApiService applies (Java parity).
fn event_api_idle(request: &crate::automation::AsyncHttpRequest) -> Duration {
    let ttl_ms = request
        .header("x-ttl")
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(0)
        .max(1000);
    Duration::from_millis(ttl_ms)
}

/// Dispatch to a streaming endpoint: check out a dedicated ordered reply
/// lane, send the request with `reply_to` = that lane, and turn the event
/// sequence into a progressive HTTP response. The first event decides the
/// shape: unmarked = ordinary single-shot; `exception` before the head = a
/// normal HTTP error; `data`/`eof` commit the head and start the renderer.
/// In envelope mode (the Event-over-HTTP streaming relay) the wire is the
/// hybrid dialect and a pre-head exception still rides the stream, so the
/// caller always receives the exact envelope.
#[allow(clippy::too_many_arguments)]
async fn stream_dispatch(
    state: &RouterState,
    info: &RouteInfo,
    http_request: &crate::automation::AsyncHttpRequest,
    cid: &str,
    cid_header: &str,
    trace_id: &Option<String>,
    trace_path: &str,
    parent_span: &Option<String>,
    accept: Option<String>,
    envelope_mode: bool,
) -> Result<StreamOutcome, AppError> {
    // a streaming endpoint borrows a dedicated ordered reply lane for the
    // lifetime of the request - an empty pool means full streaming capacity
    let Some(lane) = checkout_lane() else {
        return Err(AppError::new(503, "Streaming response pool exhausted"));
    };
    let po = PostOffice::new(&state.platform);
    let context_id = uuid::Uuid::new_v4().simple().to_string();
    let (tx, mut rx) = mpsc::channel::<EventEnvelope>(STREAM_EVENT_BUFFER);
    pending_streams()
        .lock()
        .expect("pending streams poisoned")
        .insert(context_id.clone(), tx);
    let event = build_event(
        &info.service,
        http_request,
        cid,
        trace_id,
        trace_path,
        parent_span,
    )?
    .set_correlation_id(&context_id)
    .set_reply_to(&lane);
    if let Err(e) = po.send(event).await {
        cleanup_stream(&context_id, &lane);
        return Err(e);
    }
    // the idle allowance: the endpoint timeout, or the caller-declared x-ttl
    // for a streaming-capable Event-over-HTTP call
    let base_idle = if envelope_mode {
        event_api_idle(http_request)
    } else {
        info.timeout
    };
    // await the first event within the idle allowance
    let (first, marker) = loop {
        match tokio::time::timeout(base_idle, rx.recv()).await {
            Ok(Some(envelope)) => match stream_marker(&envelope) {
                Ok(Some(marker)) => break (envelope, Some(marker)),
                Ok(None) => break (envelope, None),
                Err(()) => {
                    // present-but-invalid marker: drop the event (Java parity)
                    log::warn!(
                        "Dropping event for {context_id} - invalid {} signal",
                        event_stream::X_EVENT_STREAM
                    );
                }
            },
            Ok(None) => {
                cleanup_stream(&context_id, &lane);
                return Err(AppError::new(500, "Response channel closed unexpectedly"));
            }
            Err(_) => {
                cleanup_stream(&context_id, &lane);
                return Err(AppError::new(
                    408,
                    format!("Timeout for {} ms", base_idle.as_millis()),
                ));
            }
        }
    };
    let Some(marker) = marker else {
        // the endpoint answered single-shot - render exactly as before; in
        // envelope mode the reply is wrapped into the classic Event-over-HTTP
        // wire (the whole envelope as a serialized octet-stream body), so a
        // non-streaming target stays byte-identical to the RPC path
        cleanup_stream(&context_id, &lane);
        let reply = if envelope_mode {
            wire_single_shot(first)?
        } else {
            first
        };
        return Ok(StreamOutcome::SingleShot(reply));
    };
    if marker == event_stream::EXCEPTION && !envelope_mode {
        // failure before the head is committed - render a normal HTTP error
        // (in envelope mode a pre-head failure still rides the stream, so the
        // caller receives the exact error envelope)
        cleanup_stream(&context_id, &lane);
        let status = if first.status() >= 400 {
            first.status()
        } else {
            500
        };
        return Err(AppError::new(status, stream_error_message(&first)));
    }
    // ---- the first data/eof event commits the HTTP head ----
    if first
        .headers()
        .keys()
        .any(|k| k.eq_ignore_ascii_case("x-stream-id"))
    {
        // mutual exclusivity rule: x-event-stream wins over a stray x-stream-id
        log::warn!("Ignoring x-stream-id on a streaming response for {context_id}");
    }
    let mut response_headers: HashMap<String, String> = HashMap::new();
    let mut set_cookies: Vec<String> = Vec::new();
    let mut content_type: Option<String> = None;
    let mut idle_override: Option<Duration> = None;
    for (name, value) in first.headers() {
        let key = name.to_lowercase();
        match key.as_str() {
            // reserved envelope headers - never on the wire
            event_stream::X_EVENT_STREAM | event_stream::X_EVENT_NAME | "x-stream-id" => {}
            // idle-allowance override in seconds (producer head control)
            "x-ttl" => {
                if let Ok(seconds) = value.trim().parse::<u64>() {
                    if seconds > 0 {
                        idle_override = Some(Duration::from_secs(seconds));
                    }
                }
            }
            // in envelope mode the target's own headers stay inside the
            // envelope frames; only endpoint-level headers reach the wire
            _ if envelope_mode => {}
            "content-type" => content_type = Some(value.to_lowercase()),
            "set-cookie" => {
                set_cookies.extend(value.split('|').map(|c| c.trim().to_string()));
            }
            _ => {
                response_headers.insert(key, value.clone());
            }
        }
    }
    // the rest.yaml response transform applies to the streamed head exactly
    // as it does to a single-shot response (single-shot parity)
    if let Some(header_info) = &info.headers {
        header_info.response.apply(&mut response_headers);
    }
    // echo the business correlation-id; a function-set header of the same name wins
    response_headers
        .entry(cid_header.to_string())
        .or_insert_with(|| cid.to_string());
    if let Some(cors) = &info.cors {
        for (name, value) in &cors.headers {
            response_headers.insert(name.to_lowercase(), value.clone());
        }
    }
    // envelope mode is always SSE on the wire; raw mode negotiates
    let content_type = if envelope_mode {
        "text/event-stream".to_string()
    } else {
        content_type.unwrap_or_else(|| negotiate_stream_type(accept.as_deref()))
    };
    let sse = content_type.starts_with("text/event-stream");
    if sse {
        // default for SSE - an explicit event header or transform add wins
        response_headers
            .entry("cache-control".to_string())
            .or_insert_with(|| "no-cache".to_string());
    }
    let idle = idle_override.unwrap_or(base_idle);
    let mut builder = Response::builder().status(status_of(first.status()));
    for (name, value) in &response_headers {
        builder = builder.header(name, value);
    }
    for cookie in set_cookies {
        if !cookie.is_empty() {
            builder = builder.header("set-cookie", cookie);
        }
    }
    builder = builder.header("content-type", &content_type);
    let (body_tx, body_rx) = mpsc::channel::<Frame<Bytes>>(STREAM_FRAME_BUFFER);
    let response = builder
        .body(BoxBody::new(ChannelBody { rx: body_rx }))
        .map_err(|e| AppError::new(500, e.to_string()))?;
    tokio::spawn(render_stream(
        rx,
        body_tx,
        sse,
        idle,
        context_id,
        lane,
        first,
        marker,
        envelope_mode,
    ));
    Ok(StreamOutcome::Streaming(response))
}

/// What the renderer observed while waiting for the next segment event.
/// (A short-lived by-value carrier on the per-segment path - boxing the
/// envelope would trade one stack move for a heap allocation per segment.)
#[allow(clippy::large_enum_variant)]
enum Waited {
    Event(EventEnvelope),
    Idle,
    Closed,
}

/// Wait for the next event within the idle allowance, emitting SSE keep-alive
/// comments while the producer is quiet (best-effort; pings never extend the
/// idle allowance).
async fn next_stream_event(
    rx: &mut mpsc::Receiver<EventEnvelope>,
    body_tx: &mpsc::Sender<Frame<Bytes>>,
    sse: bool,
    idle: Duration,
) -> Waited {
    let ping_every = keep_alive_ms();
    let idle_deadline = tokio::time::sleep(idle);
    tokio::pin!(idle_deadline);
    loop {
        if sse && ping_every > 0 {
            let ping = tokio::time::sleep(Duration::from_millis(ping_every));
            tokio::pin!(ping);
            tokio::select! {
                received = rx.recv() => {
                    return match received {
                        Some(event) => Waited::Event(event),
                        None => Waited::Closed,
                    };
                }
                _ = &mut idle_deadline => return Waited::Idle,
                _ = &mut ping => {
                    let _ = body_tx.try_send(Frame::data(Bytes::from_static(b": ping\n\n")));
                }
            }
        } else {
            tokio::select! {
                received = rx.recv() => {
                    return match received {
                        Some(event) => Waited::Event(event),
                        None => Waited::Closed,
                    };
                }
                _ = &mut idle_deadline => return Waited::Idle,
            }
        }
    }
}

/// Push one wire frame with back-pressure, bounded by the idle allowance —
/// a client that stops reading beyond it gets truncated (the missing
/// terminal event is the in-band truncation signal). Returns false when the
/// stream can no longer be written (client gone or too slow).
async fn push_frame(
    body_tx: &mpsc::Sender<Frame<Bytes>>,
    idle: Duration,
    context_id: &str,
    bytes: Bytes,
) -> bool {
    if bytes.is_empty() {
        return true;
    }
    match tokio::time::timeout(idle, body_tx.send(Frame::data(bytes))).await {
        Ok(Ok(())) => true,
        Ok(Err(_)) => {
            log::debug!("Client disconnected from event stream {context_id}");
            false
        }
        Err(_) => {
            log::error!("Closing event stream for {context_id} - client too slow");
            false
        }
    }
}

/// The per-request renderer: consumes segment events from the reply lane and
/// writes SSE or chunked frames until end of transmission, an in-band error,
/// an idle timeout, or a gone/too-slow client. Always returns the lane to
/// the pool at the end (Java: closeContext, the termination funnel).
/// In envelope mode the wire is the hybrid dialect: envelope frames wherever
/// envelope semantics matter (the first event, the terminals, non-text
/// segments), raw SSE frames for plain text - and no cosmetic done/error
/// frames, because the decoded terminal envelope is the signal.
#[allow(clippy::too_many_arguments)]
async fn render_stream(
    mut rx: mpsc::Receiver<EventEnvelope>,
    body_tx: mpsc::Sender<Frame<Bytes>>,
    sse: bool,
    idle: Duration,
    context_id: String,
    lane: String,
    first: EventEnvelope,
    first_marker: &'static str,
    envelope_mode: bool,
) {
    let mut pending = Some((first, first_marker));
    let mut first_frame = true;
    loop {
        let (event, marker) = match pending.take() {
            Some(next) => next,
            None => match next_stream_event(&mut rx, &body_tx, sse, idle).await {
                Waited::Event(event) => match stream_marker(&event) {
                    Ok(Some(marker)) => (event, marker),
                    Ok(None) | Err(()) => {
                        log::warn!(
                            "Dropping event for {context_id} - invalid {} signal",
                            event_stream::X_EVENT_STREAM
                        );
                        continue;
                    }
                },
                Waited::Idle => {
                    // fail the stream in-band (Java housekeeper parity)
                    if envelope_mode {
                        let frame = idle_timeout_envelope_frame(idle);
                        let _ = push_frame(&body_tx, idle, &context_id, frame).await;
                    } else if sse {
                        let error = serde_json::json!({
                            "status": 408,
                            "message": format!("Timeout for {} seconds", idle.as_secs()),
                            "type": "error",
                        });
                        let frame = sse_frame(Some("error"), &error.to_string());
                        let _ = push_frame(&body_tx, idle, &context_id, frame).await;
                    }
                    break;
                }
                Waited::Closed => break,
            },
        };
        match marker {
            event_stream::DATA => {
                let bytes = if envelope_mode {
                    envelope_mode_data_frame(&event, first_frame)
                } else if sse {
                    if matches!(event.body(), rmpv::Value::Nil) {
                        Bytes::new()
                    } else {
                        sse_frame(stream_event_name(&event), &stream_text(event.body()))
                    }
                } else {
                    chunk_bytes(event.body())
                };
                first_frame = false;
                if !push_frame(&body_tx, idle, &context_id, bytes).await {
                    break;
                }
            }
            event_stream::EOF => {
                if envelope_mode {
                    let frame = envelope_wire_frame(&event);
                    let _ = push_frame(&body_tx, idle, &context_id, frame).await;
                } else if sse {
                    let text = if matches!(event.body(), rmpv::Value::Nil) {
                        "{}".to_string()
                    } else {
                        stream_text(event.body())
                    };
                    let frame = sse_frame(Some("done"), &text);
                    let _ = push_frame(&body_tx, idle, &context_id, frame).await;
                }
                break;
            }
            _ => {
                // in-band failure after the head is committed: envelope mode
                // frames the exact envelope; SSE renders an error event;
                // chunked mode truncates (Java parity)
                if envelope_mode {
                    let frame = envelope_wire_frame(&event);
                    let _ = push_frame(&body_tx, idle, &context_id, frame).await;
                } else if sse {
                    let status = if event.status() >= 400 {
                        event.status()
                    } else {
                        500
                    };
                    let error = serde_json::json!({
                        "status": status,
                        "message": stream_error_message(&event),
                        "type": "error",
                    });
                    let frame = sse_frame(Some("error"), &error.to_string());
                    let _ = push_frame(&body_tx, idle, &context_id, frame).await;
                }
                break;
            }
        }
    }
    cleanup_stream(&context_id, &lane);
}

/// One envelope-mode data frame: the first event always rides an envelope
/// frame (it carries the head control), a losslessly raw-able text segment
/// rides a raw SSE frame, a bare no-op segment carries nothing, and anything
/// else takes the envelope-frame escape hatch.
fn envelope_mode_data_frame(event: &EventEnvelope, first_frame: bool) -> Bytes {
    if first_frame || !raw_streamable(event) {
        envelope_wire_frame(event)
    } else if matches!(event.body(), rmpv::Value::Nil) {
        Bytes::new()
    } else {
        sse_frame(stream_event_name(event), &stream_text(event.body()))
    }
}

/// A data segment may ride a raw SSE frame only when the frame carries it
/// losslessly: a 200 status, no custom envelope headers, a user event name
/// clear of the reserved word, and a Nil-or-text body without a carriage
/// return (SSE normalizes line endings). Everything else takes the
/// envelope-frame escape hatch.
fn raw_streamable(event: &EventEnvelope) -> bool {
    if event.status() != 200 {
        return false;
    }
    for (name, value) in event.headers() {
        let key = name.to_lowercase();
        let reserved = key == event_stream::X_EVENT_STREAM
            || key == event_stream::X_EVENT_NAME
            || key == "x-ttl";
        if !reserved || (key == event_stream::X_EVENT_NAME && value == event_stream::ENVELOPE) {
            return false;
        }
    }
    match event.body() {
        rmpv::Value::Nil => true,
        rmpv::Value::String(text) => !text.as_str().unwrap_or_default().contains('\r'),
        _ => false,
    }
}

/// The classic Event-over-HTTP single-shot wire: the whole reply envelope as
/// a serialized byte body with an octet-stream content type and outer status
/// 200 (the real status rides inside - Java sendResponse parity).
fn wire_single_shot(result: EventEnvelope) -> Result<EventEnvelope, AppError> {
    let bytes = result.clear_to().clear_reply_to().to_bytes()?;
    Ok(EventEnvelope::new()
        .set_status(200)
        .set_header("content-type", "application/octet-stream")
        .set_raw_body(rmpv::Value::Binary(bytes)))
}

/// One envelope-mode wire frame: the envelope serialized verbatim - with the
/// server-internal addressing cleared, because the consuming relay rewrites
/// addressing to the original caller - as base64 under the reserved SSE event
/// name "envelope".
fn envelope_wire_frame(event: &EventEnvelope) -> Bytes {
    use base64::Engine as _;
    let wire = event.clone().clear_to().clear_reply_to();
    match wire.to_bytes() {
        Ok(bytes) => sse_frame(
            Some(event_stream::ENVELOPE),
            &base64::engine::general_purpose::STANDARD.encode(bytes),
        ),
        Err(_) => Bytes::new(),
    }
}

/// The in-band idle-timeout terminal of an envelope-mode stream: an exception
/// envelope with the standard error key-values, framed for the wire
/// (Java housekeeper-abort parity).
fn idle_timeout_envelope_frame(idle: Duration) -> Bytes {
    let message = format!("Timeout for {} seconds", idle.as_secs());
    let error = EventEnvelope::new()
        .set_header(event_stream::X_EVENT_STREAM, event_stream::EXCEPTION)
        .set_status(408)
        .set_body(serde_json::json!({"type": "error", "status": 408, "message": message}));
    match error {
        Ok(envelope) => envelope_wire_frame(&envelope),
        Err(_) => Bytes::new(),
    }
}

fn build_event(
    to: &str,
    http_request: &crate::automation::AsyncHttpRequest,
    cid: &str,
    trace_id: &Option<String>,
    trace_path: &str,
    parent_span: &Option<String>,
) -> Result<EventEnvelope, AppError> {
    let mut event = EventEnvelope::new()
        .set_to(to)
        .set_from("http.request")
        .set_correlation_id(cid)
        // the business correlation-id rides the engine-managed envelope tag
        // (never a header): it survives when the dispatch overwrites cid with
        // the HTTP context id, and the worker injects my_correlation_id into
        // the target function's input copy at delivery (Java parity)
        .add_tag(crate::post_office::BUSINESS_CID_TAG, cid)
        // the struct's to_value() IS the wire shape (single source of truth)
        // — a binary body rides natively as MsgPack binary (Java: byte[] on
        // the AsyncHttpRequest)
        .set_raw_body(http_request.to_value());
    if let Some(trace_id) = trace_id {
        event = event.set_trace(trace_id, trace_path);
        if let Some(parent) = parent_span {
            // the caller's span (from traceparent) becomes our parent
            event = event.set_span_id(parent);
        }
    }
    Ok(event)
}

/// Outcome of the request-body dispatch (Java `HttpRouter.handlePayload`).
enum ParsedBody {
    /// JSON map/list, text, or null — representable in the JSON-shaped event.
    Value(serde_json::Value),
    /// `application/x-www-form-urlencoded` — fields become query parameters.
    Form(HashMap<String, String>),
    /// Unknown content type (Java `handleBinaryContent`) — raw bytes.
    Bytes(Vec<u8>),
}

/// The content-type without any `;charset=...` suffix (Java
/// `CustomContentTypeResolver.getContentType`; the optional
/// `custom.content.types` mapping feature is deferred). Like Java, the
/// value is matched case-sensitively — only the header name is normalized.
fn base_content_type(headers: &HashMap<String, String>) -> Option<String> {
    headers
        .get("content-type")
        .map(|ct| ct.split(';').next().unwrap_or(ct).trim().to_string())
}

/// Parse the request body by declared content type — the Java `HttpRouter`
/// dispatch (`handlePayload` + its per-type handlers), mirrored exactly:
///
/// - `application/json`: empty → `{}`; a body wrapped in matching JSON
///   brackets is parsed (a parse failure falls back to the raw text);
///   anything else stays the raw text. There is **no** JSON sniffing under
///   other content types.
/// - `application/xml`: raw text (the XML-to-map parse is deferred with the
///   rest of the XML surface, exactly like the HTTP client's response side).
/// - `application/x-www-form-urlencoded` (exact match): fields decode into
///   query parameters; the body stays null.
/// - `text/html` / `text/plain`: raw text.
/// - anything else — including a missing content type: raw bytes (Java
///   `handleBinaryContent`; its no-content-length streaming variant is the
///   existing response-streaming deferral — hyper hands us the aggregated
///   body, matching Java's fixed-length path). An empty payload stays null.
fn parse_body(headers: &HashMap<String, String>, bytes: &Bytes) -> ParsedBody {
    let content_type = base_content_type(headers);
    let ct = content_type.as_deref().unwrap_or("?");
    if ct.starts_with("application/json") {
        let text = String::from_utf8_lossy(bytes).to_string();
        let trimmed = text.trim();
        let parsed = if trimmed.is_empty() {
            Some(serde_json::Value::Object(serde_json::Map::new()))
        } else if (trimmed.starts_with('{') && trimmed.ends_with('}'))
            || (trimmed.starts_with('[') && trimmed.ends_with(']'))
        {
            serde_json::from_str(&text).ok()
        } else {
            None
        };
        ParsedBody::Value(parsed.unwrap_or(serde_json::Value::String(text)))
    } else if ct == "application/x-www-form-urlencoded" {
        let text = String::from_utf8_lossy(bytes);
        let mut form = HashMap::new();
        for pair in text.split('&').filter(|p| !p.is_empty()) {
            let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
            form.insert(url_decode(name), url_decode(value));
        }
        ParsedBody::Form(form)
    } else if ct.starts_with("application/xml")
        || ct.starts_with("text/html")
        || ct.starts_with("text/plain")
    {
        ParsedBody::Value(serde_json::Value::String(
            String::from_utf8_lossy(bytes).to_string(),
        ))
    } else if bytes.is_empty() {
        ParsedBody::Value(serde_json::Value::Null)
    } else {
        ParsedBody::Bytes(bytes.to_vec())
    }
}

/// Minimal percent-decoding (+ `+` → space) for path/query values.
fn url_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok();
                match hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                    Some(byte) => {
                        out.push(byte);
                        i += 3;
                    }
                    None => {
                        out.push(bytes[i]);
                        i += 1;
                    }
                }
            }
            other => {
                out.push(other);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

/// The built-in default endpoints (Java `default-rest.yaml`): added only when
/// `rest.yaml` does not already claim the URL — user entries always win.
/// Shipped as a real resource file embedded at compile time (the
/// default-log-context.yaml pattern), so it is discoverable where a Java
/// developer expects it and byte-diffable against the Java repo's copy.
/// `/info/lib` is the one deferred Java default (see the actuator module doc).
const DEFAULT_REST_YAML: &str = include_str!("../../resources/default-rest.yaml");

fn merge_default_endpoints(table: &mut RoutingTable) -> Result<(), AppError> {
    let defaults = RoutingTable::from_yaml_text(DEFAULT_REST_YAML)?;
    for route in defaults.routes() {
        if !table.has_url(&route.url) {
            table.add_route(route.clone());
        }
    }
    Ok(())
}

/// Serve static HTML content from the `resources/public` folder with the
/// full Java static-content behavior:
///
/// 1. **path resolution** (Java `getStaticFile`): `/` and trailing-`/` paths
///    resolve to `index.html`; an extensionless filename assumes `.html`;
///    parent traversal is rejected;
/// 2. **optional request filter** (`static-content.filter`): a composable
///    function inspects matching requests (e.g. SSO redirection for a UI
///    bundle) — its response **headers are always copied** onto the HTTP
///    response; status 200 continues to serve, any other status (or a
///    redirect) passes the filter's response through;
/// 3. **no-cache pages** (`static-content.no-cache-pages`, default `/` and
///    `/index.html`): `Cache-Control: no-cache, no-store` + `Pragma` +
///    `Expires` instead of caching — entry pages must always revalidate;
/// 4. **etag protocol** for everything else: a quoted SHA-256 content hash;
///    a matching `If-None-Match` (comma-list aware) → **HTTP 304** with an
///    empty body.
async fn serve_static(
    state: &RouterState,
    path: &str,
    query_text: &str,
    headers: &HashMap<String, String>,
    peer: SocketAddr,
    head_only: bool,
) -> Option<Response<HttpBody>> {
    let (bytes, filename) = resolve_static_file(path)?;
    let static_content = state.table.static_content();
    let no_cache = super::routing::matched_element(&static_content.no_cache_pages, path);
    // the optional request filter (Java handleFilter)
    let mut filter_headers: Vec<(String, String)> = Vec::new();
    if let Some(filter) = &static_content.filter {
        let applies = super::routing::matched_element(&filter.path_list, path)
            && !super::routing::matched_element(&filter.exclusion_list, path);
        if applies {
            if state.platform.has_route(&filter.service) {
                match run_static_filter(state, filter, path, query_text, headers, peer).await {
                    Ok(filtered) => {
                        // the filter may set HTTP response headers (Java parity)
                        for (name, value) in filtered.headers() {
                            filter_headers.push((name.clone(), value.clone()));
                        }
                        if filtered.status() != 200 {
                            // redirect / rejection: pass the filter's response through
                            let (content_type, payload) = envelope_payload(&filtered);
                            let mut response =
                                Response::builder().status(status_of(filtered.status()));
                            let mut has_content_type = false;
                            for (name, value) in &filter_headers {
                                has_content_type |= name.eq_ignore_ascii_case("content-type");
                                response = response.header(name, value);
                            }
                            if let (Some(content_type), false) = (content_type, has_content_type) {
                                response = response.header("content-type", content_type);
                            }
                            return response.body(full(payload)).ok();
                        }
                    }
                    Err(e) => {
                        // resilient divergence from Java (which leaves the request
                        // to time out): log and serve the static file anyway
                        log::error!(
                            "Unable to filter static content HTTP-GET {} - {}",
                            filter.service,
                            e.message()
                        );
                    }
                }
            } else {
                log::warn!(
                    "Static content filter {} ignored because it does not exist",
                    filter.service
                );
            }
        }
    }
    // serve the file: no-cache headers or the etag protocol
    let mime = mime_for(
        std::path::Path::new(&filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or(""),
    );
    let mut response = Response::builder().status(StatusCode::OK);
    for (name, value) in &filter_headers {
        response = response.header(name, value);
    }
    response = response.header("content-type", mime);
    if no_cache {
        response = response
            .header("Cache-Control", "no-cache, no-store")
            .header("Pragma", "no-cache")
            .header("Expires", "Thu, 01 Jan 1970 00:00:00 GMT");
    } else {
        use sha2::Digest;
        let etag = format!("\"{:x}\"", sha2::Sha256::digest(&bytes));
        // If-None-Match may carry a comma-separated list (Java EtagFile.sameTag)
        let matched = headers
            .get("if-none-match")
            .is_some_and(|inm| inm.split(',').any(|tag| tag.trim() == etag));
        if matched {
            return Response::builder()
                .status(StatusCode::NOT_MODIFIED)
                .header("content-length", "0")
                .body(full(Bytes::new()))
                .ok();
        }
        response = response.header("ETag", etag);
    }
    let payload = if head_only {
        Bytes::new()
    } else {
        Bytes::from(bytes)
    };
    response.body(full(payload)).ok()
}

/// Resolve a request path to a file under `resources/public`
/// (Java `getStaticFile` rules).
fn resolve_static_file(path: &str) -> Option<(Vec<u8>, String)> {
    if path.contains("..") {
        return None; // traversal guard
    }
    let rel = path.trim_start_matches('/');
    let relative = if rel.is_empty() || path.ends_with('/') {
        format!("{rel}/index.html")
            .trim_start_matches('/')
            .to_string()
    } else {
        let filename = rel.rsplit('/').next().unwrap_or(rel);
        if filename.contains('.') {
            rel.to_string()
        } else {
            format!("{rel}.html") // assume .html for extensionless paths
        }
    };
    let file = crate::util::resources::resolve_classpath(&format!("public/{relative}"))?;
    let bytes = std::fs::read(&file).ok()?;
    let filename = relative.rsplit('/').next().unwrap_or(&relative).to_string();
    Some((bytes, filename))
}

/// Invoke the static-content filter with an AsyncHttpRequest-shaped event
/// (no body, no path parameters — Java `createHttpRequest`).
async fn run_static_filter(
    state: &RouterState,
    filter: &super::routing::SimpleHttpFilter,
    path: &str,
    query_text: &str,
    headers: &HashMap<String, String>,
    peer: SocketAddr,
) -> Result<EventEnvelope, AppError> {
    // the same single source of truth as the main dispatch: the filter's
    // request dataset is constructed through AsyncHttpRequest and rendered
    // by its to_value()
    let mut request = crate::automation::AsyncHttpRequest::new()
        .set_method("GET")
        .set_url(path)
        .set_remote_ip(&peer.ip().to_string())
        .set_secure(false)
        .set_target_host(&headers.get("host").cloned().unwrap_or_default())
        .set_body(rmpv::Value::Nil);
    for (key, value) in headers {
        request = request.set_header(key, value);
    }
    for pair in query_text.split('&').filter(|p| !p.is_empty()) {
        let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
        request = request.set_query_parameter(&url_decode(name), &url_decode(value));
    }
    let event = EventEnvelope::new()
        .set_to(&filter.service)
        .set_raw_body(request.to_value());
    let po = PostOffice::new(&state.platform);
    // Java FILTER_TIMEOUT = 10 seconds
    po.request(event, std::time::Duration::from_secs(10)).await
}

/// Map an envelope body to HTTP payload + content type (shared by the normal
/// dispatch and the filter pass-through).
/// The fallback response content type from the request's Accept header —
/// Java `AsyncHttpResponse.updateContentType` (increment 56): html → html,
/// json or `*/*` → json, no Accept → NO content-type header at all; anything
/// else → text/plain. Java's `application/xml` branch renders XML, which this
/// port defers (D10) — an xml Accept negotiates JSON instead, never claiming
/// xml on the wire.
fn accept_fallback_type(accept: Option<&str>, _body: &rmpv::Value) -> Option<String> {
    let accept = accept?;
    if accept.contains("text/html") {
        Some("text/html".to_string())
    } else if accept.contains("application/json")
        || accept.contains("*/*")
        || accept.contains("application/xml")
    {
        Some("application/json".to_string())
    } else {
        Some("text/plain".to_string())
    }
}

/// Render the response body per the effective content type — Java
/// `AsyncHttpResponse.handleContent`: strings and bytes ride raw regardless
/// of the negotiated type; map/list bodies render as JSON, wrapped in
/// `<html><body><pre>` when the effective type is text/html
/// (`handleMapContent`/`handleArrayContent`).
fn render_payload(body: &rmpv::Value, content_type: Option<&str>) -> Bytes {
    match body {
        rmpv::Value::Nil => Bytes::new(),
        rmpv::Value::String(text) => Bytes::from(text.as_str().unwrap_or_default().to_string()),
        rmpv::Value::Binary(bytes) => Bytes::from(bytes.clone()),
        _ => {
            // Omit Nil map entries unless serializer.null.transport=true (Java Gson parity).
            let stripped = crate::serializer::strip_nulls(body);
            let json = serde_json::to_value(&stripped).unwrap_or_default();
            // PRETTY-printed (presentation parity, 2026-07-26): Java renders
            // map/list bodies through SimpleMapper's default mapper, a
            // pretty-printing Gson (2-space indent) — interop drives showed
            // Java echoes multi-line and Rust echoes single-line. serde_json's
            // pretty writer matches the Gson shape. The HTML shell wraps the
            // same pretty text (Java AsyncHttpResponse HTML_START + text).
            let text = serde_json::to_string_pretty(&json).unwrap_or_default();
            if content_type.is_some_and(|t| t.starts_with("text/html"))
                && matches!(body, rmpv::Value::Map(_) | rmpv::Value::Array(_))
            {
                Bytes::from(format!("<html><body><pre>\n{text}\n</pre></body></html>"))
            } else {
                Bytes::from(text)
            }
        }
    }
}

fn envelope_payload(result: &EventEnvelope) -> (Option<&'static str>, Bytes) {
    match result.body() {
        rmpv::Value::Nil => (None, Bytes::new()),
        rmpv::Value::String(text) => (
            Some("text/plain"),
            Bytes::from(text.as_str().unwrap_or_default().to_string()),
        ),
        rmpv::Value::Binary(bytes) => {
            (Some("application/octet-stream"), Bytes::from(bytes.clone()))
        }
        _ => {
            // Omit Nil map entries unless serializer.null.transport=true (Java Gson parity).
            let body = crate::serializer::strip_nulls(result.body());
            let json = serde_json::to_value(&body).unwrap_or_default();
            // pretty-printed like render_payload (Java SimpleMapper parity)
            (
                Some("application/json"),
                Bytes::from(serde_json::to_string_pretty(&json).unwrap_or_default()),
            )
        }
    }
}

fn status_of(code: i32) -> StatusCode {
    StatusCode::from_u16(code as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

/// Minimal content-type resolution by extension (the Java `MimeTypeResolver`
/// analog; `mime-types.yml` customization is deferred).
fn mime_for(extension: &str) -> &'static str {
    match extension.to_ascii_lowercase().as_str() {
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" | "mjs" => "text/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "txt" => "text/plain",
        "pdf" => "application/pdf",
        "woff2" => "font/woff2",
        "xml" => "application/xml",
        _ => "application/octet-stream",
    }
}

/// The Java error shape: `{"status": n, "message": "...", "type": "error"}`.
fn error_response(status: i32, message: &str) -> Response<HttpBody> {
    let body = serde_json::json!({"status": status, "message": message, "type": "error"});
    Response::builder()
        .status(StatusCode::from_u16(status as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        .header("content-type", "application/json")
        .body(full(Bytes::from(body.to_string())))
        .expect("static response")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_decoding() {
        assert_eq!(url_decode("hello%20world"), "hello world");
        assert_eq!(url_decode("a+b"), "a b");
        assert_eq!(url_decode("plain"), "plain");
        assert_eq!(url_decode("bad%zz"), "bad%zz");
    }

    fn headers_of(content_type: &str) -> HashMap<String, String> {
        HashMap::from([("content-type".to_string(), content_type.to_string())])
    }

    fn value_of(parsed: ParsedBody) -> serde_json::Value {
        match parsed {
            ParsedBody::Value(value) => value,
            ParsedBody::Form(_) => panic!("expected a value, got form fields"),
            ParsedBody::Bytes(_) => panic!("expected a value, got bytes"),
        }
    }

    /// The dispatch mirrors Java `HttpRouter.handlePayload` exactly — see the
    /// `parse_body` doc for the per-content-type rules being asserted here.
    #[test]
    fn body_parsing() {
        // application/json: bracket-wrapped bodies parse; charset suffix ignored
        let json = headers_of("application/json; charset=utf-8");
        let value = value_of(parse_body(&json, &Bytes::from(r#"{"a":1}"#)));
        assert_eq!(value["a"], 1);
        // a non-JSON body under application/json stays the raw text (no error)
        let text = value_of(parse_body(&json, &Bytes::from("import graph from x")));
        assert_eq!(
            text,
            serde_json::Value::String("import graph from x".into())
        );
        // malformed JSON falls back to the raw text
        let bad = value_of(parse_body(&json, &Bytes::from("{broken")));
        assert_eq!(bad, serde_json::Value::String("{broken".into()));
        // an empty application/json body is an empty map
        let empty = value_of(parse_body(&json, &Bytes::new()));
        assert_eq!(empty, serde_json::json!({}));
        // no JSON sniffing under text/plain: a JSON-looking body stays text
        let plain = headers_of("text/plain");
        let unsniffed = value_of(parse_body(&plain, &Bytes::from(r#"{"a":1}"#)));
        assert_eq!(unsniffed, serde_json::Value::String(r#"{"a":1}"#.into()));
        // XML rides as raw text (parser deferral, like the client's response side)
        let xml = value_of(parse_body(
            &headers_of("application/xml"),
            &Bytes::from("<a>1</a>"),
        ));
        assert_eq!(xml, serde_json::Value::String("<a>1</a>".into()));
        // form fields decode into query parameters, not the body
        let form = parse_body(
            &headers_of("application/x-www-form-urlencoded"),
            &Bytes::from("a=1&b=hello+world"),
        );
        match form {
            ParsedBody::Form(fields) => {
                assert_eq!(fields["a"], "1");
                assert_eq!(fields["b"], "hello world");
            }
            _ => panic!("expected form fields"),
        }
        // unknown or missing content type: bytes (Java handleBinaryContent)
        match parse_body(&HashMap::new(), &Bytes::from("hello")) {
            ParsedBody::Bytes(bytes) => assert_eq!(bytes, b"hello"),
            _ => panic!("expected bytes for a missing content type"),
        }
        // ...and an empty unknown-type payload leaves the body null
        assert_eq!(
            value_of(parse_body(&HashMap::new(), &Bytes::new())),
            serde_json::Value::Null
        );
    }
}
