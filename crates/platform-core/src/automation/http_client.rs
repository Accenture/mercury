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

//! The Async HTTP client (`async.http.request`) — Rust port of
//! `org.platformlambda.automation.http.AsyncHttpClient`, closing the
//! long-standing design §7 deferral. An event interceptor: the request is an
//! [`AsyncHttpRequest`] map on the envelope body; the reply carries the HTTP
//! status, response headers and a body decoded by content type (JSON object/
//! array, text, or raw bytes). Outbound calls propagate the distributed
//! trace (`X-Trace-Id` + W3C `traceparent`) and the business correlation-id,
//! exactly like the REST automation edge expects on ingress.
//!
//! Deliberate deferrals from the Java original (documented, not silent):
//! object streams (up/download) and multipart file upload wait for the
//! platform-core streams port; XML request/response bodies pass through as
//! text (the `SimpleXmlParser`/writer pair is not ported).
//!
//! `https` targets are supported (increment 48): certificates verify against
//! the OS trust store (the JDK-truststore analog), and `trust_all_cert` on
//! the request skips chain validation for self-signed endpoints — the same
//! escape hatch as Java's `InsecureTrustManagerFactory` path.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use rmpv::Value;
use tokio_rustls::rustls;
use tokio_rustls::rustls::pki_types::ServerName;
use tokio_rustls::TlsConnector;

use crate::envelope::EventEnvelope;
use crate::function::AppError;
use crate::platform::Platform;
use crate::post_office::PostOffice;
use crate::util::app_config_reader::AppConfigReader;
use crate::util::w3c_trace;

pub const ASYNC_HTTP_REQUEST: &str = "async.http.request";
const USER_AGENT_NAME: &str = "async-http-client";
const DEFAULT_TTL_SECONDS: u64 = 30;
/// Headers that may interfere with the underlying HTTP client (Java parity).
const HEADERS_TO_IGNORE: &[&str] = &[
    "content-length",
    "user-agent",
    "x-stream-id",
    "content-encoding",
    "transfer-encoding",
    "host",
    "connection",
    "upgrade-insecure-requests",
    "accept-encoding",
    "sec-fetch-mode",
    "sec-fetch-site",
    "sec-fetch-user",
    // engine-internal client instruction (the Event-over-HTTP transport-leg
    // marker) — consumed by this client, never sent to the peer
    "x-event-api",
];

/// The HTTP request contract (Java `AsyncHttpRequest`): a map-backed model
/// with `method`, `url`, `host`, `headers`, `body`, `parameters.query`,
/// `parameters.path`, `cookies`, `session` and the server-side dataset keys
/// (`ip`, `https`, `timeout`, raw `query`). Programmatic callers build it
/// with the fluent setters; declarative callers (flow data mapping) build
/// the same map shape directly; a **typed function**
/// (`TypedFunction<AsyncHttpRequest, O>` + `#[preload(..., typed)]`) receives
/// it deserialized from the REST edge's request dataset — see the
/// `Deserialize` impl below.
#[derive(Clone, Debug)]
pub struct AsyncHttpRequest {
    method: Option<String>,
    url: Option<String>,
    target_host: Option<String>,
    headers: Vec<(String, String)>,
    // Option distinguishes "never set" (key omitted) from "set to null"
    // (the REST edge emits an explicit null body for byte/form payloads)
    body: Option<Value>,
    query_parameters: Vec<(String, Value)>,
    path_parameters: Vec<(String, String)>,
    cookies: Vec<(String, String)>,
    session: Vec<(String, String)>,
    // Option: emitted only when explicitly set — a service-target dataset
    // carries `host` (the Host header) WITHOUT a trust flag, while a relay
    // caller that sets the flag keeps it on the wire
    trust_all_cert: Option<bool>,
    // server-side dataset keys (the REST edge emits them into the function's
    // body; see automation::server process())
    ip: Option<String>,
    https: Option<bool>,
    timeout: Option<u64>,
    query_string: Option<String>,
}

/// Typed-function support (the maintainer-agreed design, 2026-07-26): the
/// two serde traits are thin delegates onto the existing map-shape parser
/// and builder, so `#[preload(..., typed)]` +
/// `TypedFunction<AsyncHttpRequest, O>` flows through the ordinary
/// `TypedAdapter` (`body_as::<I>()`) with ZERO worker special-casing — the
/// knowledge lives on the type, not in the engine (Java, by contrast,
/// special-cases `AsyncHttpRequest.class` inside `WorkerHandler.getMapBody`).
/// This is the template rule for the Python/Node ports: their request
/// classes must be constructible from the request map so typed signatures
/// just work.
impl<'de> serde::Deserialize<'de> for AsyncHttpRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Ok(AsyncHttpRequest::from_value(&value))
    }
}

impl serde::Serialize for AsyncHttpRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_value().serialize(serializer)
    }
}

impl Default for AsyncHttpRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl AsyncHttpRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        AsyncHttpRequest {
            method: None,
            url: None,
            target_host: None,
            headers: Vec::new(),
            body: None,
            query_parameters: Vec::new(),
            path_parameters: Vec::new(),
            cookies: Vec::new(),
            session: Vec::new(),
            trust_all_cert: None,
            ip: None,
            https: None,
            timeout: None,
            query_string: None,
        }
    }

    /// Parse the map shape (the envelope body of an `async.http.request`
    /// event).
    pub fn from_value(value: &Value) -> Self {
        let mut request = AsyncHttpRequest::new();
        let Value::Map(entries) = value else {
            return request;
        };
        let get = |key: &str| -> Option<&Value> {
            entries
                .iter()
                .find(|(k, _)| k.as_str() == Some(key))
                .map(|(_, v)| v)
        };
        request.method = get("method").and_then(|v| v.as_str()).map(str::to_string);
        request.url = get("url").and_then(|v| v.as_str()).map(str::to_string);
        request.target_host = get("host").and_then(|v| v.as_str()).map(str::to_string);
        request.trust_all_cert = get("trust_all_cert").and_then(|v| v.as_bool());
        request.body = get("body").cloned();
        // server-side dataset keys (REST edge → service-target function)
        request.ip = get("ip").and_then(|v| v.as_str()).map(str::to_string);
        request.https = get("https").and_then(|v| v.as_bool());
        request.timeout = get("timeout").and_then(|v| v.as_u64());
        request.query_string = get("query").and_then(|v| v.as_str()).map(str::to_string);
        if let Some(Value::Map(headers)) = get("headers") {
            for (k, v) in headers {
                if let Some(key) = k.as_str() {
                    request.headers.push((key.to_string(), display_text(v)));
                }
            }
        }
        if let Some(Value::Map(cookies)) = get("cookies") {
            for (k, v) in cookies {
                if let Some(key) = k.as_str() {
                    request.cookies.push((key.to_string(), display_text(v)));
                }
            }
        }
        if let Some(Value::Map(session)) = get("session") {
            for (k, v) in session {
                if let Some(key) = k.as_str() {
                    request.session.push((key.to_string(), display_text(v)));
                }
            }
        }
        if let Some(Value::Map(parameters)) = get("parameters") {
            for (k, v) in parameters {
                match (k.as_str(), v) {
                    (Some("query"), Value::Map(query)) => {
                        for (qk, qv) in query {
                            if let Some(key) = qk.as_str() {
                                request.query_parameters.push((key.to_string(), qv.clone()));
                            }
                        }
                    }
                    (Some("path"), Value::Map(path)) => {
                        for (pk, pv) in path {
                            if let Some(key) = pk.as_str() {
                                request
                                    .path_parameters
                                    .push((key.to_string(), display_text(pv)));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        request
    }

    /// Render the Java `toMap()` shape for the envelope body.
    pub fn to_value(&self) -> Value {
        let mut map: Vec<(Value, Value)> = Vec::new();
        if let Some(method) = &self.method {
            map.push((Value::from("method"), Value::from(method.as_str())));
        }
        if let Some(url) = &self.url {
            map.push((Value::from("url"), Value::from(url.as_str())));
        }
        if let Some(host) = &self.target_host {
            map.push((Value::from("host"), Value::from(host.as_str())));
        }
        if let Some(trust_all_cert) = self.trust_all_cert {
            // Java toMap: trust_all_cert travels with the relay target
            map.push((Value::from("trust_all_cert"), Value::from(trust_all_cert)));
        }
        if !self.headers.is_empty() {
            map.push((Value::from("headers"), string_pairs(&self.headers)));
        }
        if let Some(body) = &self.body {
            // an explicit null body stays on the wire (the REST edge emits
            // body: null for byte/form payloads; an UNSET body is omitted)
            map.push((Value::from("body"), body.clone()));
        }
        // server-side dataset keys, emitted when set (round-trip integrity:
        // to_value emits exactly what from_value parses)
        if let Some(ip) = &self.ip {
            map.push((Value::from("ip"), Value::from(ip.as_str())));
        }
        if let Some(https) = self.https {
            map.push((Value::from("https"), Value::from(https)));
        }
        if let Some(timeout) = self.timeout {
            map.push((Value::from("timeout"), Value::from(timeout)));
        }
        if let Some(query) = &self.query_string {
            map.push((Value::from("query"), Value::from(query.as_str())));
        }
        if !self.cookies.is_empty() {
            map.push((Value::from("cookies"), string_pairs(&self.cookies)));
        }
        if !self.session.is_empty() {
            map.push((Value::from("session"), string_pairs(&self.session)));
        }
        // "parameters" is always present with both sub-maps — the REST
        // edge's dataset shape (a paramless request still carries the empty
        // maps, exactly like today's server emission)
        let query: Vec<(Value, Value)> = self
            .query_parameters
            .iter()
            .map(|(k, v)| (Value::from(k.as_str()), v.clone()))
            .collect();
        let path: Vec<(Value, Value)> = self
            .path_parameters
            .iter()
            .map(|(k, v)| (Value::from(k.as_str()), Value::from(v.as_str())))
            .collect();
        map.push((
            Value::from("parameters"),
            Value::Map(vec![
                (Value::from("query"), Value::Map(query)),
                (Value::from("path"), Value::Map(path)),
            ]),
        ));
        Value::Map(map)
    }

    pub fn set_method(mut self, method: &str) -> Self {
        self.method = Some(method.to_string());
        self
    }

    pub fn set_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn set_target_host(mut self, host: &str) -> Self {
        self.target_host = Some(host.to_string());
        self
    }

    /// Java `setTrustAllCert`: skip certificate-chain validation for an
    /// `https` target (self-signed endpoints). Ignored for plain `http`.
    pub fn set_trust_all_cert(mut self, trust_all_cert: bool) -> Self {
        self.trust_all_cert = Some(trust_all_cert);
        self
    }

    /// Set (or replace, case-insensitively) a request header.
    pub fn set_header(mut self, key: &str, value: &str) -> Self {
        self.headers.retain(|(k, _)| !k.eq_ignore_ascii_case(key));
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    pub fn set_body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// Set (or replace — Java `setQueryParameter` put semantics) a
    /// single-value query parameter.
    pub fn set_query_parameter(mut self, key: &str, value: &str) -> Self {
        self.query_parameters.retain(|(k, _)| k != key);
        self.query_parameters
            .push((key.to_string(), Value::from(value)));
        self
    }

    /// Set a REPEATED query parameter (the list shape the REST edge produces
    /// for `?q=a&q=b` — Java `setQueryParameter` with a List value).
    pub fn set_query_parameter_values(mut self, key: &str, values: &[&str]) -> Self {
        self.query_parameters.retain(|(k, _)| k != key);
        self.query_parameters.push((
            key.to_string(),
            Value::Array(values.iter().map(|v| Value::from(*v)).collect()),
        ));
        self
    }

    pub fn set_path_parameter(mut self, key: &str, value: &str) -> Self {
        self.path_parameters
            .push((key.to_string(), value.to_string()));
        self
    }

    /// Set (or replace) a cookie (Java `setCookie`).
    pub fn set_cookie(mut self, key: &str, value: &str) -> Self {
        self.cookies.retain(|(k, _)| k != key);
        self.cookies.push((key.to_string(), value.to_string()));
        self
    }

    /// Set (or replace) a session-info entry (Java `setSessionInfo`) — on the
    /// server side these arrive from the authentication service's verdict.
    pub fn set_session_info(mut self, key: &str, value: &str) -> Self {
        self.session.retain(|(k, _)| k != key);
        self.session.push((key.to_string(), value.to_string()));
        self
    }

    /// The caller's IP address (Java `setRemoteIp`) — the REST edge stamps it
    /// on the request dataset.
    pub fn set_remote_ip(mut self, ip: &str) -> Self {
        self.ip = Some(ip.to_string());
        self
    }

    /// Whether the original request arrived over HTTPS (Java `setSecure`).
    pub fn set_secure(mut self, https: bool) -> Self {
        self.https = Some(https);
        self
    }

    /// The raw query string (Java `setQueryString`).
    pub fn set_query_string(mut self, query: &str) -> Self {
        self.query_string = Some(query.to_string());
        self
    }

    /// The ROUTE timeout in seconds — the REST edge's `timeout` dataset key
    /// (rest.yaml endpoint timeout). Distinct from [`set_timeout_seconds`],
    /// which writes the caller's TTL as the `x-ttl` header (milliseconds,
    /// Java `setTimeoutSeconds`); on read, `timeout_seconds()` prefers a
    /// caller-sent x-ttl and falls back to this field — the ingress
    /// precedence.
    pub fn set_route_timeout_seconds(mut self, seconds: u64) -> Self {
        self.timeout = Some(seconds);
        self
    }

    /// Java `setTimeoutSeconds`: the TTL travels as the `x-ttl` header (ms).
    pub fn set_timeout_seconds(self, timeout_seconds: u64) -> Self {
        let ms = timeout_seconds.max(1) * 1000;
        self.set_header("x-ttl", &ms.to_string())
    }

    pub fn method(&self) -> &str {
        self.method.as_deref().unwrap_or("GET")
    }

    /// The request URI path (Java `AsyncHttpRequest.getUrl`).
    pub fn url(&self) -> &str {
        self.url.as_deref().unwrap_or("/")
    }

    pub fn target_host(&self) -> Option<&str> {
        self.target_host.as_deref()
    }

    pub fn trust_all_cert(&self) -> bool {
        self.trust_all_cert.unwrap_or(false)
    }

    /// All request headers in insertion order.
    pub fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    /// Session info injected by an authentication service (Java
    /// `AsyncHttpRequest.getSessionInfo`): headers returned on the auth
    /// verdict, riding to the target function as read-only headers.
    pub fn session(&self) -> &[(String, String)] {
        &self.session
    }

    pub fn header(&self, key: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v.as_str())
    }

    pub fn body(&self) -> &Value {
        static NIL: Value = Value::Nil;
        self.body.as_ref().unwrap_or(&NIL)
    }

    /// Java `AsyncHttpRequest.getTimeoutSeconds()`: the x-ttl header is in
    /// milliseconds and a fractional second must round UP, never down
    /// (1,500 ms is a 2-second budget, not 1) — otherwise the wire-level
    /// read timeout fires before a peer that spends its whole TTL replies.
    pub fn timeout_seconds(&self) -> u64 {
        self.header("x-ttl")
            .and_then(|v| v.parse::<u64>().ok())
            .map(|ms| ms.max(1).div_ceil(1000))
            // the REST edge also carries the route timeout as the dataset's
            // "timeout" key (seconds); a caller-sent x-ttl wins, matching
            // the ingress precedence
            .or(self.timeout)
            .unwrap_or(DEFAULT_TTL_SECONDS)
    }

    /// Deserialize the request body into a typed value (Java
    /// `AsyncHttpRequest.getBody(Class<T>)`).
    pub fn body_as<T: serde::de::DeserializeOwned>(&self) -> Result<T, AppError> {
        rmpv::ext::from_value(self.body().clone())
            .map_err(|e| AppError::new(500, format!("unable to deserialize body: {e}")))
    }

    /// The caller's IP address (Java `AsyncHttpRequest.getRemoteIp`) — set
    /// by the REST edge on the request dataset.
    pub fn remote_ip(&self) -> Option<&str> {
        self.ip.as_deref()
    }

    /// Whether the original request arrived over HTTPS (Java
    /// `AsyncHttpRequest.isSecure`; the REST edge derives it from
    /// `x-forwarded-proto`).
    pub fn is_secure(&self) -> bool {
        self.https.unwrap_or(false)
    }

    /// The raw query string (Java `AsyncHttpRequest.getQueryString`).
    pub fn query_string(&self) -> Option<&str> {
        self.query_string.as_deref()
    }

    /// One `{path}` parameter by name (Java `getPathParameter`).
    pub fn path_parameter(&self, key: &str) -> Option<&str> {
        self.path_parameters
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    /// All `{path}` parameters (Java `getPathParameters`).
    pub fn path_parameters(&self) -> &[(String, String)] {
        &self.path_parameters
    }

    /// One query parameter as text (Java `getQueryParameter`): a repeated
    /// parameter (list value) yields its FIRST occurrence.
    pub fn query_parameter(&self, key: &str) -> Option<String> {
        self.query_parameters
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| match v {
                Value::Array(values) => values.first().map(display_text).unwrap_or_default(),
                other => display_text(other),
            })
    }

    /// Every value of a query parameter (Java `getQueryParameters`): one
    /// occurrence is a one-element list, repeats keep every value.
    pub fn query_parameters(&self, key: &str) -> Vec<String> {
        self.query_parameters
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| match v {
                Value::Array(values) => values.iter().map(display_text).collect(),
                other => vec![display_text(other)],
            })
            .unwrap_or_default()
    }

    /// One cookie by name (Java `getCookie`).
    pub fn cookie(&self, key: &str) -> Option<&str> {
        self.cookies
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    /// All cookies as parsed by the REST edge (Java `getCookies`).
    pub fn cookies(&self) -> &[(String, String)] {
        &self.cookies
    }

    /// One session-info entry by name (Java `getSessionInfo(String)`).
    pub fn session_info(&self, key: &str) -> Option<&str> {
        self.session
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    /// Java `getFinalizedUrl`: substitute `{path}` parameters, merge query
    /// parameters into the query string, and keep any `#hash` suffix.
    pub fn finalized_url(&self) -> String {
        let uri = self.url.as_deref().unwrap_or("/");
        let (without_hash, hash) = match uri.rfind('#') {
            Some(mark) => (&uri[..mark], Some(&uri[mark + 1..])),
            None => (uri, None),
        };
        let (mut raw_uri, mut query_string) = match without_hash.rfind('?') {
            Some(mark) => (
                without_hash[..mark].to_string(),
                Some(without_hash[mark + 1..].to_string()),
            ),
            None => (without_hash.to_string(), None),
        };
        let qs = self.query_parameters_to_string();
        if let Some(qs) = qs {
            query_string = Some(match query_string {
                Some(existing) => format!("{existing}&{qs}"),
                None => qs,
            });
        }
        for (key, value) in &self.path_parameters {
            let token = format!("{{{key}}}");
            if raw_uri.contains(&token) {
                raw_uri = raw_uri.replace(&token, value);
            }
        }
        let mut out = raw_uri;
        if let Some(qs) = query_string {
            out.push('?');
            out.push_str(&qs);
        }
        if let Some(hash) = hash {
            out.push('#');
            out.push_str(hash);
        }
        // minimal encoding (Java getEncodedUri): spaces in the path
        out.replace(' ', "%20")
    }

    /// Java `queryParametersToString`: string values and lists of strings
    /// only (other types are skipped, Java parity).
    fn query_parameters_to_string(&self) -> Option<String> {
        if self.query_parameters.is_empty() {
            return None;
        }
        let mut parts: Vec<String> = Vec::new();
        for (key, value) in &self.query_parameters {
            match value {
                Value::String(_) => parts.push(format!("{key}={}", display_text(value))),
                Value::Array(items) => {
                    for item in items {
                        if matches!(item, Value::String(_)) {
                            parts.push(format!("{key}={}", display_text(item)));
                        }
                    }
                }
                _ => {}
            }
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join("&"))
        }
    }
}

fn string_pairs(pairs: &[(String, String)]) -> Value {
    Value::Map(
        pairs
            .iter()
            .map(|(k, v)| (Value::from(k.as_str()), Value::from(v.as_str())))
            .collect(),
    )
}

fn display_text(value: &Value) -> String {
    match value {
        Value::String(s) => s.as_str().unwrap_or_default().to_string(),
        Value::Nil => String::new(),
        other => other.to_string(),
    }
}

/// The interceptor body: process the request and reply manually with the
/// decoded HTTP response (or an error envelope).
pub(crate) async fn handle(
    platform: &Platform,
    _headers: HashMap<String, String>,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let po = PostOffice::new(platform);
    let Some(reply_to) = event.reply_to().map(str::to_string) else {
        // fire-and-forget: errors are logged, successes discarded (Java parity)
        if let Err(e) = process_request(&po, &_headers, &event).await {
            log::error!("Unhandled exception (no reply-to) - {}", e.message());
        }
        return EventEnvelope::new().set_body("ignored");
    };
    let cid = event.correlation_id().unwrap_or_default().to_string();
    let response = match process_request(&po, &_headers, &event).await {
        Ok(response) => response,
        Err(e) => EventEnvelope::new()
            .set_status(e.status())
            .set_raw_body(Value::from(e.message())),
    };
    let _ = po
        .send(response.set_to(&reply_to).set_correlation_id(&cid))
        .await;
    EventEnvelope::new().set_body("ignored")
}

async fn process_request(
    po: &PostOffice,
    invocation_headers: &HashMap<String, String>,
    event: &EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let request = AsyncHttpRequest::from_value(event.body());
    let (secure, host, port) = validate_url(&request)?;
    let uri = request.finalized_url();
    po.annotate_trace(
        "destination",
        format!(
            "{}{}",
            request.target_host().unwrap_or_default(),
            raw_url(&uri)
        ),
    );
    let method = request.method().to_string();
    if !matches!(
        method.as_str(),
        "GET" | "HEAD" | "PUT" | "POST" | "PATCH" | "DELETE" | "OPTIONS"
    ) {
        return Err(AppError::new(405, "Method not allowed"));
    }
    // connect with the configured timeout (Java http.client.connection.timeout)
    let connect_ms = connect_timeout_ms();
    let stream = tokio::time::timeout(
        Duration::from_millis(connect_ms),
        tokio::net::TcpStream::connect((host.as_str(), port)),
    )
    .await
    .map_err(|_| AppError::new(408, format!("Connection timeout for {host}:{port}")))?
    .map_err(|e| AppError::new(500, format!("Unable to connect to {host}:{port} - {e}")))?;
    // http and https produce different stream types but the same SendRequest
    let mut sender = if secure {
        let connector = tls_connector(request.trust_all_cert())?;
        let server_name = ServerName::try_from(host.clone())
            .map_err(|e| AppError::new(400, format!("Invalid TLS server name {host} - {e}")))?;
        let tls_stream = connector
            .connect(server_name, stream)
            .await
            .map_err(|e| AppError::new(500, format!("TLS handshake failed for {host} - {e}")))?;
        let io = hyper_util::rt::TokioIo::new(tls_stream);
        let (sender, connection) = hyper::client::conn::http1::handshake(io)
            .await
            .map_err(|e| AppError::new(500, format!("HTTP handshake failed - {e}")))?;
        tokio::spawn(async move {
            let _ = connection.await;
        });
        sender
    } else {
        let io = hyper_util::rt::TokioIo::new(stream);
        let (sender, connection) = hyper::client::conn::http1::handshake(io)
            .await
            .map_err(|e| AppError::new(500, format!("HTTP handshake failed - {e}")))?;
        tokio::spawn(async move {
            let _ = connection.await;
        });
        sender
    };
    let mut builder = hyper::Request::builder()
        .method(method.as_str())
        .uri(if uri.is_empty() { "/" } else { &uri });
    builder = apply_headers(
        builder,
        invocation_headers,
        &request,
        event,
        &host,
        port,
        secure,
    );
    let body_bytes = request_body_bytes(&request, &method)?;
    let http_request = builder
        .body(Full::new(Bytes::from(body_bytes)))
        .map_err(|e| AppError::new(400, format!("Invalid HTTP request - {e}")))?;
    // per-request response timeout (the x-ttl header, default 30s) with one
    // extra second of wire-level grace so a peer that spends its whole TTL
    // and replies AT the deadline is still readable; the caller's own RPC
    // timeout, not this read timeout, governs the user-visible deadline
    // (Java parity: AsyncHttpClient responseTimeout = getTimeoutSeconds() + 1)
    let ttl = Duration::from_secs(request.timeout_seconds() + 1);
    let http_response = tokio::time::timeout(ttl, sender.send_request(http_request))
        .await
        .map_err(|_| AppError::new(408, format!("Timeout for {} ms", ttl.as_millis())))?
        .map_err(|e| AppError::new(500, format!("HTTP request failed - {e}")))?;
    let mut response = EventEnvelope::new().set_status(http_response.status().as_u16() as i32);
    let mut content_type: Option<String> = None;
    let mut has_content_length = false;
    for (name, value) in http_response.headers() {
        let key = name.as_str();
        let text = value.to_str().unwrap_or_default();
        if key.eq_ignore_ascii_case("content-type") {
            content_type = Some(text.to_lowercase());
        }
        if key.eq_ignore_ascii_case("content-length") {
            has_content_length = true;
        }
        response = response.set_header(key, text);
    }
    let bytes = http_response
        .into_body()
        .collect()
        .await
        .map_err(|e| AppError::new(500, format!("Unable to read HTTP response - {e}")))?
        .to_bytes();
    if !has_content_length {
        response = response.set_header("x-content-length", &bytes.len().to_string());
    }
    Ok(response.set_raw_body(decode_response_body(&bytes, content_type.as_deref())))
}

fn raw_url(uri: &str) -> &str {
    match uri.rfind('?') {
        Some(mark) => &uri[..mark],
        None => uri,
    }
}

fn connect_timeout_ms() -> u64 {
    let config = AppConfigReader::get_instance();
    config
        .get_property_or("http.client.connection.timeout", "5000")
        .parse::<u64>()
        .unwrap_or(5000)
        .max(2000)
}

/// Java `validateUrl`: the target host must be `http(s)://host[:port]` with
/// no URI path. Default ports follow the scheme (80 / 443).
fn validate_url(request: &AsyncHttpRequest) -> Result<(bool, String, u16), AppError> {
    let Some(target) = request.target_host() else {
        return Err(AppError::new(
            400,
            "Missing target host. e.g. https://hostname",
        ));
    };
    let (secure, rest) = if let Some(rest) = target.strip_prefix("http://") {
        (false, rest)
    } else if let Some(rest) = target.strip_prefix("https://") {
        (true, rest)
    } else {
        return Err(AppError::new(400, "Protocol must be http or https"));
    };
    let authority = rest.trim_end_matches('/');
    if authority.contains('/') {
        return Err(AppError::new(400, "Target host must not contain URI path"));
    }
    let default_port = if secure { 443 } else { 80 };
    let (host, port) = match authority.rsplit_once(':') {
        Some((h, p)) => (
            h.to_string(),
            p.parse::<u16>()
                .map_err(|_| AppError::new(400, "Invalid port number in target host"))?,
        ),
        None => (authority.to_string(), default_port),
    };
    if host.trim().is_empty() {
        return Err(AppError::new(
            400,
            "Unable to resolve target host as domain or IP address",
        ));
    }
    Ok((secure, host, port))
}

/// TLS client config, built once per verification mode. The strict config
/// trusts the OS certificate store (the closest analog of the JDK's default
/// truststore that the Java client uses); the trust-all config skips chain
/// validation only — TLS signatures are still verified — mirroring Java's
/// `InsecureTrustManagerFactory` escape hatch for self-signed endpoints.
fn tls_connector(trust_all_cert: bool) -> Result<TlsConnector, AppError> {
    static STRICT: OnceLock<Result<Arc<rustls::ClientConfig>, String>> = OnceLock::new();
    static TRUST_ALL: OnceLock<Arc<rustls::ClientConfig>> = OnceLock::new();
    let config = if trust_all_cert {
        TRUST_ALL
            .get_or_init(|| {
                let config = rustls::ClientConfig::builder()
                    .dangerous()
                    .with_custom_certificate_verifier(Arc::new(TrustAllVerifier))
                    .with_no_client_auth();
                Arc::new(config)
            })
            .clone()
    } else {
        STRICT
            .get_or_init(|| {
                let loaded = rustls_native_certs::load_native_certs();
                let mut roots = rustls::RootCertStore::empty();
                for cert in loaded.certs {
                    // tolerate individual unparsable certs (OS stores carry
                    // legacy entries); fail only if nothing loads at all
                    let _ = roots.add(cert);
                }
                if roots.is_empty() {
                    return Err(format!(
                        "No usable certificates in the OS trust store - {:?}",
                        loaded.errors
                    ));
                }
                Ok(Arc::new(
                    rustls::ClientConfig::builder()
                        .with_root_certificates(roots)
                        .with_no_client_auth(),
                ))
            })
            .clone()
            .map_err(|e| AppError::new(500, e))?
    };
    Ok(TlsConnector::from(config))
}

/// Certificate verifier that accepts any server certificate (chain
/// validation skipped; handshake signatures still verified) — the rustls
/// mirror of Java's `InsecureTrustManagerFactory`.
#[derive(Debug)]
struct TrustAllVerifier;

impl rustls::client::danger::ServerCertVerifier for TrustAllVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &rustls::pki_types::CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &rustls::crypto::ring::default_provider().signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &rustls::pki_types::CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &rustls::crypto::ring::default_provider().signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        rustls::crypto::ring::default_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_headers(
    mut builder: hyper::http::request::Builder,
    invocation_headers: &HashMap<String, String>,
    request: &AsyncHttpRequest,
    event: &EventEnvelope,
    host: &str,
    port: u16,
    secure: bool,
) -> hyper::http::request::Builder {
    // omit the scheme's default port from the host header (80 / 443)
    let default_port = if secure { 443 } else { 80 };
    let authority = if port == default_port {
        host.to_string()
    } else {
        format!("{host}:{port}")
    };
    builder = builder.header("host", authority);
    builder = builder.header("user-agent", USER_AGENT_NAME);
    // request headers (session info becomes headers, Java parity)
    let mut merged: Vec<(String, String)> = request.headers.clone();
    merged.extend(request.session.iter().cloned());
    for (key, value) in &merged {
        if permitted_http_header(key) {
            // trim optional whitespace (RFC 7230 OWS) — netty strips it on
            // write, hyper strictly rejects it, so trimming keeps parity
            // (e.g. a token loaded from a file with a trailing newline)
            builder = builder.header(key.as_str(), value.trim());
        }
    }
    // best practice (maintainer ruling): send a default 'Accept: */*' when the
    // caller gives none — the Java engine's reactor-netty client does this
    // implicitly, and the REST edge negotiates the response content-type from
    // Accept, so the default keeps JSON decoding identical on both engines
    if !merged.iter().any(|(k, _)| k.eq_ignore_ascii_case("accept")) {
        builder = builder.header("accept", "*/*");
    }
    // distributed trace propagation: this route is untraced by default
    // (skip.rpc.tracing, Java parity), so the trace rides the ENVELOPE and
    // the injected invocation headers, not the ambient trace state — exactly
    // like Java's PostOffice.trackable(headers).
    // The engine's own stamps use INSERT semantics (Java `http.set`): a
    // same-named header forwarded from the request object above is replaced,
    // never duplicated (append-vs-insert wire hygiene — the Event-over-HTTP
    // leg pre-sets the same trace headers on its request).
    let config = AppConfigReader::get_instance();
    if let Some(trace_id) = event.trace_id() {
        let trace_header = config.get_property_or("http.trace.id.header", "X-Trace-Id");
        stamp_header(&mut builder, &trace_header, trace_id);
        if let Some(traceparent) = w3c_trace::format(trace_id, event.span_id().unwrap_or_default())
        {
            stamp_header(&mut builder, w3c_trace::TRACEPARENT, &traceparent);
            // when a custom traceparent header name is configured
            // (http.traceparent.header), stamp the same value under that name
            // too, so the W3C trace context survives an intermediary that
            // strips the standard header
            let custom_traceparent =
                config.get_property_or("http.traceparent.header", w3c_trace::TRACEPARENT);
            if !custom_traceparent.eq_ignore_ascii_case(w3c_trace::TRACEPARENT) {
                stamp_header(&mut builder, &custom_traceparent, &traceparent);
            }
        }
    }
    // propagate the business correlation-id (unless the caller set it).
    // The engine's own Event-over-HTTP transport leg (the x-event-api client
    // instruction) is exempt: the business cid rides INSIDE the envelope
    // (my_cid tag) and the HTTP-level header is absent on that path (Java
    // parity — its EventEmitter leg carries no ambient business context).
    if request.header(super::event_api::X_EVENT_API).is_none() {
        if let Some(business_cid) = invocation_headers.get(crate::automation::MY_CORRELATION_ID) {
            let cid_header =
                config.get_property_or("http.correlation.id.header", "X-Correlation-Id");
            if request.header(&cid_header).is_none() {
                stamp_header(&mut builder, &cid_header, business_cid.as_str());
            }
        }
    }
    // cookies
    if !request.cookies.is_empty() {
        let cookie = request
            .cookies
            .iter()
            .map(|(k, v)| format!("{k}={}", url_encode(v)))
            .collect::<Vec<_>>()
            .join("; ");
        builder = builder.header("cookie", cookie.as_str());
    }
    builder
}

fn permitted_http_header(header: &str) -> bool {
    !HEADERS_TO_IGNORE
        .iter()
        .any(|ignored| header.eq_ignore_ascii_case(ignored))
}

/// Insert-or-replace an engine-stamped header on the outgoing request (Java
/// `http.set` semantics): a same-named header already forwarded from the
/// request object is replaced, never duplicated. An invalid name/value is
/// dropped silently — the same outcome `Builder::header` would produce as a
/// deferred build error, but without failing the whole request.
fn stamp_header(builder: &mut hyper::http::request::Builder, name: &str, value: &str) {
    if let Some(headers) = builder.headers_mut() {
        if let (Ok(name), Ok(value)) = (
            hyper::header::HeaderName::from_bytes(name.as_bytes()),
            hyper::header::HeaderValue::from_str(value),
        ) {
            headers.insert(name, value);
        }
    }
}

fn url_encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'*' => {
                out.push(byte as char)
            }
            b' ' => out.push('+'),
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}

fn request_body_bytes(request: &AsyncHttpRequest, method: &str) -> Result<Vec<u8>, AppError> {
    if !matches!(method, "POST" | "PUT" | "PATCH") {
        return Ok(Vec::new());
    }
    match request.body() {
        Value::Nil => Ok(Vec::new()),
        Value::Binary(bytes) => Ok(bytes.clone()),
        Value::String(text) => Ok(text.as_str().unwrap_or_default().as_bytes().to_vec()),
        value @ (Value::Map(_) | Value::Array(_)) => {
            // maps and lists serialize as JSON (the Java XML writer path is a
            // documented deferral); Nil map entries are omitted unless
            // serializer.null.transport=true (Java Gson parity)
            let json = serde_json::to_value(crate::serializer::strip_nulls(value))
                .map_err(|e| AppError::new(400, format!("Invalid HTTP request body - {e}")))?;
            serde_json::to_vec(&json)
                .map_err(|e| AppError::new(400, format!("Invalid HTTP request body - {e}")))
        }
        _ => Err(AppError::new(400, "Invalid HTTP request body")),
    }
}

/// Decode the response body by content type (Java `sendFixedLengthResponse`):
/// JSON objects/arrays become maps/lists, text stays text, XML passes
/// through as raw text (parser deferral), anything else is bytes.
fn decode_response_body(bytes: &[u8], content_type: Option<&str>) -> Value {
    let Some(content_type) = content_type else {
        return Value::from(bytes.to_vec());
    };
    if content_type.starts_with("application/json") {
        let text = String::from_utf8_lossy(bytes).trim().to_string();
        if text.is_empty() {
            return Value::Map(vec![]);
        }
        if (text.starts_with('{') && text.ends_with('}'))
            || (text.starts_with('[') && text.ends_with(']'))
        {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Ok(value) = rmpv::ext::to_value(&json) {
                    return value;
                }
            }
        }
        return Value::from(text);
    }
    if content_type.starts_with("text/")
        || content_type.starts_with("application/javascript")
        || content_type.starts_with("application/xml")
    {
        return Value::from(String::from_utf8_lossy(bytes).to_string());
    }
    Value::from(bytes.to_vec())
}

/// The registered service wrapper (Java `AsyncHttpClient` implements
/// `TypedLambdaFunction`; registered by the app starter at 500 instances).
/// Holds the platform it is registered on so its manual replies route to
/// that platform's `temporary.inbox` reply listener.
pub struct AsyncHttpClientService {
    platform: Platform,
}

impl AsyncHttpClientService {
    pub fn new(platform: &Platform) -> Self {
        AsyncHttpClientService {
            platform: platform.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::function::ComposableFunction for AsyncHttpClientService {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        // reply through the platform this service was registered on — its
        // manual reply must reach THAT platform's temporary.inbox listener
        // (the global instance may live on another runtime in tests)
        handle(&self.platform, headers, input).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The server-side request dataset, FLUENT-BUILT (the maintainer's
    /// requirement: the test setup creates a new AsyncHttpRequest and sets
    /// every value through the fluent API — the test doubles as living
    /// documentation of the builder surface, exercising every setter).
    fn server_dataset_request() -> AsyncHttpRequest {
        AsyncHttpRequest::new()
            .set_method("POST")
            .set_url("/api/typed/alice")
            .set_remote_ip("127.0.0.1")
            .set_secure(true)
            .set_target_host("localhost:8085")
            .set_header("accept", "application/json")
            .set_header("x-api-key", "open-sesame")
            .set_header("x-ttl", "5000")
            .set_path_parameter("user", "alice")
            .set_query_parameter_values("q", &["a", "b"])
            .set_query_parameter("single", "1")
            .set_query_string("q=a&q=b&single=1")
            .set_route_timeout_seconds(30)
            .set_cookie("first", "alpha")
            .set_session_info("user_id", "u-1")
            .set_body(Value::Map(vec![(
                Value::from("note"),
                Value::from("hello"),
            )]))
    }

    /// Round-trip integrity of the typed contract: every fluent-set field is
    /// visible through its accessor, survives to_value → from_value, and the
    /// serde impls delegate to exactly that pair.
    ///
    /// Division of labor: this fluent-built round-trip proves INTERNAL
    /// consistency (builder ↔ accessors ↔ map shape ↔ serde). The REAL
    /// server-shape contract is pinned by the end-to-end test
    /// `typed_async_http_request_function_serves_a_real_request` in
    /// tests/rest_automation.rs (a typed function behind /api/typed/{user}
    /// over the live automation server) — do not "simplify" that e2e away
    /// in favor of this unit test.
    #[test]
    fn server_dataset_round_trips_through_the_typed_contract() {
        let request = server_dataset_request();
        // every field is visible through the typed accessors
        assert_eq!(request.method(), "POST");
        assert_eq!(request.url(), "/api/typed/alice");
        assert_eq!(request.remote_ip(), Some("127.0.0.1"));
        assert!(request.is_secure());
        assert_eq!(request.target_host(), Some("localhost:8085"));
        assert_eq!(request.path_parameter("user"), Some("alice"));
        assert_eq!(request.query_parameter("single").as_deref(), Some("1"));
        assert_eq!(request.query_parameter("q").as_deref(), Some("a"));
        assert_eq!(request.query_parameters("q"), vec!["a", "b"]);
        assert_eq!(request.query_parameters("single"), vec!["1"]);
        assert_eq!(request.query_string(), Some("q=a&q=b&single=1"));
        assert_eq!(
            request.header("Accept"),
            Some("application/json"),
            "case-insensitive"
        );
        assert_eq!(request.cookie("first"), Some("alpha"));
        assert_eq!(request.session_info("user_id"), Some("u-1"));
        // caller-sent x-ttl (5000 ms -> 5 s) wins over the route timeout (30)
        assert_eq!(request.timeout_seconds(), 5);
        #[derive(serde::Deserialize)]
        struct Note {
            note: String,
        }
        assert_eq!(request.body_as::<Note>().expect("typed body").note, "hello");
        // to_value emits what from_value parses: a second pass is identical
        let round = AsyncHttpRequest::from_value(&request.to_value());
        assert_eq!(round.to_value(), request.to_value());
        // the serde impls are thin delegates onto the same pair
        let deserialized: AsyncHttpRequest =
            rmpv::ext::from_value(request.to_value()).expect("serde deserialize");
        assert_eq!(deserialized.to_value(), request.to_value());
        let serialized = rmpv::ext::to_value(&request).expect("serde serialize");
        assert_eq!(
            AsyncHttpRequest::from_value(&serialized).to_value(),
            request.to_value()
        );
    }

    /// The route timeout (`set_route_timeout_seconds` / the REST edge's
    /// "timeout" dataset key) is the fallback when no x-ttl header rides
    /// the request.
    #[test]
    fn route_timeout_is_the_fallback_without_x_ttl() {
        let request = AsyncHttpRequest::new()
            .set_method("GET")
            .set_url("/x")
            .set_route_timeout_seconds(30);
        assert_eq!(request.timeout_seconds(), 30);
        // and it round-trips through the map shape
        assert_eq!(
            AsyncHttpRequest::from_value(&request.to_value()).timeout_seconds(),
            30
        );
    }
}
