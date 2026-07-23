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
use std::sync::{Arc, Mutex, OnceLock};

use async_trait::async_trait;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::sync::oneshot;

use crate::envelope::EventEnvelope;
use crate::function::{AppError, ComposableFunction};
use crate::platform::Platform;
use crate::post_office::PostOffice;
use crate::trace;
use crate::util::app_config_reader::AppConfigReader;
use crate::util::config_reader::ConfigReader;
use crate::util::w3c_trace;

use super::routing::{AssignedRoute, RoutingTable};

/// Reserved read-only request header exposing the business correlation-id to
/// the target function (Java `HttpRouter.MY_CORRELATION_ID`).
pub const MY_CORRELATION_ID: &str = "my_correlation_id";

/// Route of the HTTP response-correlation service (Java
/// `AsyncHttpClient.ASYNC_HTTP_RESPONSE`).
pub const ASYNC_HTTP_RESPONSE: &str = "async.http.response";

/// Reserved `my_*` metadata headers that must never reach the HTTP wire
/// (Java `WorkerHandler.copyResponseHeaders` protected-metadata handling).
const PROTECTED_METADATA: [&str; 4] = [
    "my_route",
    "my_trace_id",
    "my_trace_path",
    MY_CORRELATION_ID,
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
        if let Err(e) =
            platform.register_private(ASYNC_HTTP_RESPONSE, Arc::new(AsyncHttpResponseService), 500)
        {
            if !platform.has_route(ASYNC_HTTP_RESPONSE) {
                return Err(e);
            }
        }
    }
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
    });
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
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    // websocket upgrade on a registered `/ws/{name}/{token}` path takes the
    // connection out of the HTTP request/response cycle (Java parity)
    if super::ws_server::is_ws_upgrade(&request) {
        return Ok(super::ws_server::handle_ws_upgrade(
            &state.platform,
            request,
            peer.ip().to_string(),
        ));
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
        return Ok(response
            .body(Full::new(Bytes::new()))
            .expect("static response"));
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
) -> Result<Response<Full<Bytes>>, AppError> {
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
    // caller's span as our parent; else the trace-id header; else generated
    let traceparent = headers
        .get(w3c_trace::TRACEPARENT)
        .and_then(|value| w3c_trace::parse(value));
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
    headers.insert(MY_CORRELATION_ID.to_string(), cid.clone());
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
    let body_value = match &parsed {
        ParsedBody::Value(value) => value.clone(),
        // a binary body can't ride the JSON shape — substituted as a
        // MsgPack binary in build_event (Java: byte[] on AsyncHttpRequest)
        ParsedBody::Form(_) | ParsedBody::Bytes(_) => serde_json::Value::Null,
    };
    let binary_body = match &parsed {
        ParsedBody::Bytes(bytes) => Some(bytes.as_slice()),
        _ => None,
    };
    let mut http_request = serde_json::json!({
        "method": method,
        "url": path,
        "ip": peer.ip().to_string(),
        // Java: setSecure(x-forwarded-proto == "https") — increment 56,
        // parity F14d (previously hardcoded false)
        "https": headers.get("x-forwarded-proto").map(String::as_str) == Some("https"),
        "host": headers.get("host").cloned().unwrap_or_default(),
        "headers": headers,
        "parameters": {"path": path_params, "query": query},
        "body": body_value,
        // Java AsyncHttpRequest.getTimeoutSeconds (the flow adapter derives
        // the flow TTL from it)
        "timeout": info.timeout.as_secs(),
    });
    // the raw query string rides as Java's top-level "query" key; cookies
    // appear only when present (Java toMap omits empty)
    if !query_text.is_empty() {
        http_request["query"] = serde_json::Value::String(query_text.clone());
    }
    if !cookies.is_empty() {
        http_request["cookies"] = serde_json::to_value(&cookies).unwrap_or_default();
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
            binary_body,
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
        if !verdict.headers().is_empty() {
            let session: serde_json::Map<String, serde_json::Value> = verdict
                .headers()
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            http_request["session"] = serde_json::Value::Object(session);
        }
    }
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
        binary_body,
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
    let result = match tokio::time::timeout(info.timeout, rx).await {
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
    };
    // map the response envelope back to HTTP (Java AsyncHttpResponse:
    // updateHeadersAndContentType + updateHeaders)
    let status = status_of(result.status());
    let is_head = method == "HEAD";
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
        .body(Full::new(payload))
        .map_err(|e| AppError::new(500, e.to_string()))
}

fn build_event(
    to: &str,
    http_request: &serde_json::Value,
    binary_body: Option<&[u8]>,
    cid: &str,
    trace_id: &Option<String>,
    trace_path: &str,
    parent_span: &Option<String>,
) -> Result<EventEnvelope, AppError> {
    let mut event = EventEnvelope::new()
        .set_to(to)
        .set_from("http.request")
        .set_correlation_id(cid)
        // the business correlation-id channel (Java parity): the header
        // survives when the dispatch overwrites cid with the HTTP context id,
        // and the worker's trace bracket prefers it for my_correlation_id()
        .set_header(MY_CORRELATION_ID, cid)
        .set_body(http_request)?;
    // a binary body (unknown content type) rides as MsgPack binary — the
    // JSON-shaped request map can't carry bytes (Java: byte[] on the
    // AsyncHttpRequest), so it is substituted after the map conversion
    if let Some(bytes) = binary_body {
        let mut root = event.body().clone();
        if let rmpv::Value::Map(entries) = &mut root {
            for (key, value) in entries.iter_mut() {
                if key.as_str() == Some("body") {
                    *value = rmpv::Value::Binary(bytes.to_vec());
                    break;
                }
            }
        }
        event = event.set_raw_body(root);
    }
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
/// `/info/lib` and `/info/routes` are deferred (see the actuator module doc).
const DEFAULT_REST_YAML: &str = r#"
rest:
  - service: "event.api.service"
    methods: ['POST']
    url: "/api/event"
    timeout: 60s
    tracing: true
  - service: "info.actuator.service"
    methods: ['GET']
    url: "/info"
    timeout: 10s
  - service: "env.actuator.service"
    methods: ['GET']
    url: "/env"
    timeout: 10s
  - service: "health.actuator.service"
    methods: ['GET']
    url: "/health"
    timeout: 30s
  - service: "liveness.actuator.service"
    methods: ['GET']
    url: "/livenessprobe"
    timeout: 10s
"#;

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
) -> Option<Response<Full<Bytes>>> {
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
                            return response.body(Full::new(payload)).ok();
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
                .body(Full::new(Bytes::new()))
                .ok();
        }
        response = response.header("ETag", etag);
    }
    let payload = if head_only {
        Bytes::new()
    } else {
        Bytes::from(bytes)
    };
    response.body(Full::new(payload)).ok()
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
    let mut query: HashMap<String, String> = HashMap::new();
    for pair in query_text.split('&').filter(|p| !p.is_empty()) {
        let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
        query.insert(url_decode(name), url_decode(value));
    }
    let request = serde_json::json!({
        "method": "GET",
        "url": path,
        "ip": peer.ip().to_string(),
        "https": false,
        "host": headers.get("host").cloned().unwrap_or_default(),
        "headers": headers,
        "parameters": {"path": {}, "query": query},
        "body": serde_json::Value::Null,
    });
    let event = EventEnvelope::new()
        .set_to(&filter.service)
        .set_body(&request)?;
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
            if content_type.is_some_and(|t| t.starts_with("text/html"))
                && matches!(body, rmpv::Value::Map(_) | rmpv::Value::Array(_))
            {
                Bytes::from(format!("<html><body><pre>\n{json}\n</pre></body></html>"))
            } else {
                Bytes::from(json.to_string())
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
            (Some("application/json"), Bytes::from(json.to_string()))
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
fn error_response(status: i32, message: &str) -> Response<Full<Bytes>> {
    let body = serde_json::json!({"status": status, "message": message, "type": "error"});
    Response::builder()
        .status(StatusCode::from_u16(status as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(body.to_string())))
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
