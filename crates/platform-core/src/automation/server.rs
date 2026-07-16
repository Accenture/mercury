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
//! optional authentication → build the `AsyncHttpRequest`-shaped event → RPC
//! to the target function → map the response envelope back to HTTP (status,
//! body by type, response-header transforms + CORS headers). Errors use the
//! Java JSON shape `{status, message, type: "error"}`.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;

use crate::envelope::EventEnvelope;
use crate::function::AppError;
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
    let rest_yaml = config.get_property_or("yaml.rest.automation", "classpath:/rest.yaml");
    let reader = ConfigReader::load(&rest_yaml)
        .map_err(|e| AppError::new(500, format!("Unable to load {rest_yaml} - {e}")))?;
    let table = RoutingTable::load(&reader)?;
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
        return Ok(error_response(404, "Resource not found"));
    };
    // CORS preflight (OPTIONS is auto-added per the grammar)
    if method == "OPTIONS" {
        let mut response = Response::builder().status(StatusCode::NO_CONTENT);
        if let Some(cors) = &assigned.info.cors {
            for (name, value) in &cors.options {
                response = response.header(name, value);
            }
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
    // AsyncHttpRequest-shaped event body (Java parity keys)
    let mut query: HashMap<String, String> = HashMap::new();
    for pair in query_text.split('&').filter(|p| !p.is_empty()) {
        let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
        query.insert(url_decode(name), url_decode(value));
    }
    let path_params: HashMap<String, String> = assigned
        .path_params
        .iter()
        .map(|(k, v)| (k.clone(), url_decode(v)))
        .collect();
    let body_value = parse_body(&headers, &body_bytes);
    let http_request = serde_json::json!({
        "method": method,
        "url": path,
        "ip": peer.ip().to_string(),
        "https": false,
        "host": headers.get("host").cloned().unwrap_or_default(),
        "headers": headers,
        "parameters": {"path": path_params, "query": query},
        "body": body_value,
    });
    let po = PostOffice::new(&state.platform);
    let trace_path = format!("{method} {path}");
    // optional authentication before dispatch (simple route form)
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
                &verdict
                    .body_as::<String>()
                    .unwrap_or_else(|_| "Unauthorized".to_string()),
            ));
        }
        if !verdict.body_as::<bool>().unwrap_or(false) {
            return Err(AppError::new(401, "Unauthorized"));
        }
    }
    let event = build_event(
        &info.service,
        &http_request,
        &cid,
        &trace_id,
        &trace_path,
        &parent_span,
    )?;
    let result = po.request(event, info.timeout).await?;
    // map the response envelope back to HTTP
    let status =
        StatusCode::from_u16(result.status() as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let (content_type, payload) = match result.body() {
        rmpv::Value::Nil => (None, Bytes::new()),
        rmpv::Value::String(text) => (
            Some("text/plain"),
            Bytes::from(text.as_str().unwrap_or_default().to_string()),
        ),
        rmpv::Value::Binary(bytes) => {
            (Some("application/octet-stream"), Bytes::from(bytes.clone()))
        }
        _ => {
            let json = result.body_as::<serde_json::Value>().unwrap_or_default();
            (Some("application/json"), Bytes::from(json.to_string()))
        }
    };
    let mut response_headers: HashMap<String, String> = HashMap::new();
    if let Some(content_type) = content_type {
        response_headers.insert("content-type".to_string(), content_type.to_string());
    }
    if let Some(header_info) = &info.headers {
        header_info.response.apply(&mut response_headers);
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
    response
        .body(Full::new(payload))
        .map_err(|e| AppError::new(500, e.to_string()))
}

fn build_event(
    to: &str,
    http_request: &serde_json::Value,
    cid: &str,
    trace_id: &Option<String>,
    trace_path: &str,
    parent_span: &Option<String>,
) -> Result<EventEnvelope, AppError> {
    let mut event = EventEnvelope::new()
        .set_to(to)
        .set_from("http.request")
        .set_correlation_id(cid)
        .set_body(http_request)?;
    if let Some(trace_id) = trace_id {
        event = event.set_trace(trace_id, trace_path);
        if let Some(parent) = parent_span {
            // the caller's span (from traceparent) becomes our parent
            event = event.set_span_id(parent);
        }
    }
    Ok(event)
}

/// Parse the request body by content type: JSON object/array when declared
/// (or when it looks like JSON), else UTF-8 text; empty → null.
fn parse_body(headers: &HashMap<String, String>, bytes: &Bytes) -> serde_json::Value {
    if bytes.is_empty() {
        return serde_json::Value::Null;
    }
    let text = String::from_utf8_lossy(bytes).to_string();
    let declared_json = headers
        .get("content-type")
        .is_some_and(|ct| ct.contains("application/json"));
    if declared_json || text.trim_start().starts_with(['{', '[']) {
        serde_json::from_str(&text).unwrap_or(serde_json::Value::String(text))
    } else {
        serde_json::Value::String(text)
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

    #[test]
    fn body_parsing() {
        let json_headers =
            HashMap::from([("content-type".to_string(), "application/json".to_string())]);
        let value = parse_body(&json_headers, &Bytes::from(r#"{"a":1}"#));
        assert_eq!(value["a"], 1);
        let text = parse_body(&HashMap::new(), &Bytes::from("hello"));
        assert_eq!(text, serde_json::Value::String("hello".into()));
        assert_eq!(
            parse_body(&HashMap::new(), &Bytes::new()),
            serde_json::Value::Null
        );
    }
}
