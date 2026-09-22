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

//! The OTLP/HTTP export: one request per span through the platform's own
//! `async.http.request` client, with the Java exporter's retry policy and its
//! failure diagnostics (Java `OtelForwarderContext`).
//!
//! **Retry.** Telemetry delivery is at-least-once by design — duplicates are
//! tolerated, drops are what hurt — so a transport failure (connect refused,
//! TLS, a killed keep-alive, a timeout) and the retryable HTTP statuses
//! (408, 429, 502, 503, 504) are retried on the OpenTelemetry SDK's default
//! bounded backoff: 5 attempts, 1 s growing by 1.5× and capped at 5 s. Any
//! other status fails at once — a 401 will not get better by waiting.
//!
//! **Diagnostics.** A rejected export is actionable from the forwarder's own
//! warning line: the status leads, the backend's response body follows
//! (whitespace-collapsed, bounded), and the two rejections that actually
//! happen get a hint — a 404 is nearly always the signal path missing from the
//! endpoint, a 401 is the credential, a 403 a credential without the ingest
//! scope. Request headers are never rendered, so no credential can reach the
//! log.

use std::fmt;
use std::time::Duration;

use platform_core::automation::{AsyncHttpRequest, ASYNC_HTTP_REQUEST};
use platform_core::{AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::config::{ForwarderSettings, HeaderSupplier, ENDPOINT};
use crate::otlp;
use crate::span::Span;
use crate::INSTRUMENTATION_SCOPE;

/// HTTP statuses worth another attempt (the OpenTelemetry SDK's set, plus 408
/// — which is also how the platform client reports its own timeout).
pub const RETRYABLE_STATUSES: [i32; 5] = [408, 429, 502, 503, 504];

/// The waits between attempts: 5 attempts, 1 s × 1.5ⁿ (the SDK's
/// `RetryPolicy` defaults — initial 1 s, multiplier 1.5, max 5 s).
pub const DEFAULT_BACKOFF_MS: [u64; 4] = [1000, 1500, 2250, 3375];

const CONTENT_TYPE: &str = "application/x-protobuf";
const MAX_BODY_CHARS: usize = 256;

/// The OTLP/HTTP exporter for one endpoint.
pub struct Exporter {
    endpoint: String,
    target_host: String,
    path: String,
    query: Option<String>,
    timeout_ms: u64,
    headers: HeaderSupplier,
    backoff: Vec<Duration>,
    service_name: String,
    scope_version: String,
    compression: String,
}

/// Why an export gave up: the last attempt's diagnostic and how many attempts
/// were made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportFailure {
    pub attempts: u32,
    pub detail: String,
}

impl fmt::Display for ExportFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.attempts > 1 {
            write!(f, "{} (after {} attempts)", self.detail, self.attempts)
        } else {
            write!(f, "{}", self.detail)
        }
    }
}

impl std::error::Error for ExportFailure {}

struct AttemptError {
    retryable: bool,
    detail: String,
}

impl Exporter {
    /// Build the exporter — fails only for an endpoint that is not an http(s)
    /// URL, so a misconfiguration surfaces at start-up.
    pub fn from_settings(settings: ForwarderSettings) -> Result<Exporter, AppError> {
        let (target_host, path, query) = split_endpoint(&settings.endpoint)?;
        Ok(Exporter {
            endpoint: settings.endpoint,
            target_host,
            path,
            query,
            timeout_ms: settings.timeout_ms,
            headers: settings.headers,
            backoff: DEFAULT_BACKOFF_MS
                .iter()
                .map(|ms| Duration::from_millis(*ms))
                .collect(),
            service_name: settings.service_name,
            scope_version: settings.scope_version,
            compression: settings.compression,
        })
    }

    /// Replace the waits between attempts (`attempts = waits.len() + 1`).
    pub fn with_backoff(mut self, waits: Vec<Duration>) -> Self {
        self.backoff = waits;
        self
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    pub fn scope_version(&self) -> &str {
        &self.scope_version
    }

    pub fn compression(&self) -> &str {
        &self.compression
    }

    /// The names of the request headers that resolve right now (values are
    /// never exposed — this feeds the start-up line).
    pub fn header_names(&self) -> Vec<String> {
        (self.headers)().into_iter().map(|(k, _)| k).collect()
    }

    /// The OTLP request body for one span.
    pub fn encode(&self, span: &Span) -> Vec<u8> {
        otlp::encode_export_request(
            &self.service_name,
            INSTRUMENTATION_SCOPE,
            &self.scope_version,
            span,
        )
    }

    /// Export one span, retrying transient failures on the backoff schedule.
    pub async fn export(&self, span: &Span) -> Result<(), ExportFailure> {
        let body = self.encode(span);
        let mut attempt: u32 = 0;
        loop {
            attempt += 1;
            match self.attempt(&body).await {
                Ok(()) => return Ok(()),
                Err(e) if e.retryable && (attempt as usize) <= self.backoff.len() => {
                    log::debug!(
                        "OTLP export attempt {attempt} for span {} failed ({}) - retrying",
                        span.span_id_hex(),
                        e.detail
                    );
                    tokio::time::sleep(self.backoff[attempt as usize - 1]).await;
                }
                Err(e) => {
                    return Err(ExportFailure {
                        attempts: attempt,
                        detail: e.detail,
                    })
                }
            }
        }
    }

    async fn attempt(&self, body: &[u8]) -> Result<(), AttemptError> {
        let platform = Platform::get_instance();
        let po = PostOffice::new(&platform);
        let mut request = AsyncHttpRequest::new()
            .set_method("POST")
            .set_target_host(&self.target_host)
            .set_url(&self.path)
            .set_header("content-type", CONTENT_TYPE)
            .set_header("accept", CONTENT_TYPE)
            .set_body(Value::Binary(body.to_vec()))
            .set_timeout_seconds(self.timeout_ms.div_ceil(1000).max(1));
        if let Some(query) = &self.query {
            request = request.set_query_string(query);
        }
        // headers are resolved per export, never baked in: a credential published
        // after start-up takes effect without a restart
        for (name, value) in (self.headers)() {
            request = request.set_header(&name, &value);
        }
        let response = po
            .request(
                EventEnvelope::new()
                    .set_to(ASYNC_HTTP_REQUEST)
                    .set_raw_body(request.to_value()),
                Duration::from_millis(self.timeout_ms + 3000),
            )
            .await
            .map_err(|e| AttemptError {
                retryable: true,
                detail: e.message().to_string(),
            })?;
        classify(&response)
    }
}

/// Decide what an HTTP answer means for the export.
fn classify(response: &EventEnvelope) -> Result<(), AttemptError> {
    let status = response.status();
    if (200..300).contains(&status) {
        if let Some(partial) = otlp::partial_success(body_bytes(response.body())) {
            log::warn!(
                "OTLP backend accepted the request but rejected {} span(s) - {}",
                partial.rejected_spans,
                partial.error_message
            );
        }
        return Ok(());
    }
    let body = body_text(response.body());
    if RETRYABLE_STATUSES.contains(&status) {
        return Err(AttemptError {
            retryable: true,
            detail: describe_http_failure(status, &body),
        });
    }
    // No response headers at all means this is not an HTTP answer but the
    // platform client's own rendering of a transport failure (connection
    // refused, TLS handshake, a connection closed mid-exchange) - retried like
    // the Java exporter retries every IOException.
    if status >= 500 && response.headers().is_empty() {
        return Err(AttemptError {
            retryable: true,
            detail: body,
        });
    }
    Err(AttemptError {
        retryable: false,
        detail: describe_http_failure(status, &body),
    })
}

/// Render an HTTP rejection so it is actionable from one log line: the status,
/// the backend's own explanation (bounded), and a hint for the usual causes.
pub fn describe_http_failure(status: i32, body: &str) -> String {
    let mut text = format!("HTTP {status}");
    let body = collapse(body);
    if !body.is_empty() {
        text.push_str(" - ");
        text.push_str(&body);
    }
    if let Some(hint) = hint_for(status) {
        text.push_str(" | ");
        text.push_str(hint);
    }
    text
}

fn hint_for(status: i32) -> Option<&'static str> {
    match status {
        404 => Some(
            "check otel.exporter.otlp.endpoint includes the signal path (e.g. .../v1/traces), \
             not just the vendor base URL",
        ),
        401 => Some(
            "the backend rejected the credential itself - check otel.exporter.otlp.headers (the \
             header name and any auth scheme must match what the backend expects)",
        ),
        // 403 means the credential was ACCEPTED but lacks a scope; the response body names it
        403 => Some(
            "the credential was accepted but lacks permission - grant the trace-ingest scope on \
             the token (the response body above names it)",
        ),
        413 => Some("the backend rejected the payload as too large"),
        429 => Some("the backend is rate-limiting; the exporter retries with backoff"),
        _ => None,
    }
}

fn collapse(body: &str) -> String {
    let collapsed = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.chars().count() > MAX_BODY_CHARS {
        let cut: String = collapsed.chars().take(MAX_BODY_CHARS).collect();
        format!("{cut}...")
    } else {
        collapsed
    }
}

fn body_bytes(body: &Value) -> &[u8] {
    match body {
        Value::Binary(bytes) => bytes,
        _ => &[],
    }
}

fn body_text(body: &Value) -> String {
    match body {
        Value::Nil => String::new(),
        Value::Binary(bytes) => String::from_utf8_lossy(bytes).to_string(),
        Value::String(s) => s.as_str().unwrap_or_default().to_string(),
        Value::Map(_) | Value::Array(_) => rmpv::ext::from_value::<serde_json::Value>(body.clone())
            .map(|json| json.to_string())
            .unwrap_or_default(),
        other => other.to_string(),
    }
}

/// Split the endpoint URL into the platform client's `target_host`
/// (`scheme://host[:port]`), the path and the optional query string.
pub fn split_endpoint(url: &str) -> Result<(String, String, Option<String>), AppError> {
    let invalid = || {
        AppError::new(
            400,
            format!(
                "{ENDPOINT}='{url}' must be an http(s) URL including the signal path, \
                 e.g. http://localhost:4318/v1/traces"
            ),
        )
    };
    let trimmed = url.trim();
    let scheme_end = trimmed.find("://").ok_or_else(invalid)?;
    let scheme = &trimmed[..scheme_end];
    if !(scheme.eq_ignore_ascii_case("http") || scheme.eq_ignore_ascii_case("https")) {
        return Err(invalid());
    }
    let rest = &trimmed[scheme_end + 3..];
    let (authority, path_and_query) = match rest.find('/') {
        Some(slash) => (&rest[..slash], &rest[slash..]),
        None => (rest, "/"),
    };
    if authority.is_empty() || authority.contains('?') || authority.contains('#') {
        return Err(invalid());
    }
    let (path, query) = match path_and_query.find('?') {
        Some(q) => (
            &path_and_query[..q],
            Some(path_and_query[q + 1..].to_string()).filter(|s| !s.is_empty()),
        ),
        None => (path_and_query, None),
    };
    Ok((
        format!("{}://{authority}", scheme.to_ascii_lowercase()),
        path.to_string(),
        query,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoints_split_into_host_path_and_query() {
        assert_eq!(
            split_endpoint("http://localhost:4318/v1/traces").unwrap(),
            ("http://localhost:4318".into(), "/v1/traces".into(), None)
        );
        assert_eq!(
            split_endpoint("HTTPS://tenant.live.dynatrace.com/api/v2/otlp/v1/traces?x=1").unwrap(),
            (
                "https://tenant.live.dynatrace.com".into(),
                "/api/v2/otlp/v1/traces".into(),
                Some("x=1".into())
            )
        );
        assert_eq!(
            split_endpoint("http://collector").unwrap(),
            ("http://collector".into(), "/".into(), None)
        );
        for bad in [
            "localhost:4318/v1/traces",
            "ftp://x/y",
            "http:///v1/traces",
            "",
        ] {
            let err = split_endpoint(bad).expect_err(bad);
            assert_eq!(err.status(), 400);
            assert!(
                err.message().contains("otel.exporter.otlp.endpoint"),
                "{}",
                err.message()
            );
        }
    }

    #[test]
    fn a_404_names_the_most_likely_cause() {
        let text = describe_http_failure(404, "");
        assert!(text.starts_with("HTTP 404"), "{text}");
        assert!(text.contains("signal path"), "{text}");
        assert!(text.contains("/v1/traces"), "{text}");
    }

    #[test]
    fn a_401_points_at_the_credential_rather_than_the_endpoint() {
        let text = describe_http_failure(401, "Token Authentication failed");
        assert!(
            text.starts_with("HTTP 401 - Token Authentication failed"),
            "{text}"
        );
        assert!(text.contains("otel.exporter.otlp.headers"), "{text}");
        assert!(!text.contains("signal path"), "{text}");
    }

    #[test]
    fn a_403_points_at_the_permission_rather_than_the_credential() {
        let text = describe_http_failure(
            403,
            "User is missing required permission: openpipeline:traces:ingest",
        );
        assert!(text.starts_with("HTTP 403"), "{text}");
        assert!(text.contains("openpipeline:traces:ingest"), "{text}");
        assert!(text.contains("lacks permission"), "{text}");
        assert!(
            !text.contains("the header name and any auth scheme"),
            "{text}"
        );
    }

    #[test]
    fn the_response_body_is_included_but_bounded() {
        let text = describe_http_failure(400, "span 0 rejected:\n   invalid trace id");
        assert_eq!(text, "HTTP 400 - span 0 rejected: invalid trace id");
        let flood = describe_http_failure(400, &"x".repeat(5000));
        assert!(flood.len() < 600, "{}", flood.len());
        assert!(flood.ends_with("..."), "{flood}");
    }

    #[test]
    fn classification_separates_retryable_from_fatal() {
        let ok = EventEnvelope::new().set_status(200);
        assert!(classify(&ok).is_ok());
        let busy = EventEnvelope::new()
            .set_status(503)
            .set_header("content-type", "text/plain")
            .set_raw_body(Value::from("busy"));
        let e = classify(&busy).err().unwrap();
        assert!(e.retryable);
        assert_eq!(e.detail, "HTTP 503 - busy");
        // the platform client's own transport failure: a 500 with no response headers
        let refused = EventEnvelope::new()
            .set_status(500)
            .set_raw_body(Value::from(
                "Unable to connect to 127.0.0.1:1 - Connection refused",
            ));
        let e = classify(&refused).err().unwrap();
        assert!(e.retryable);
        assert!(e.detail.starts_with("Unable to connect"), "{}", e.detail);
        // a real HTTP 500 from the server is final
        let server_error = EventEnvelope::new()
            .set_status(500)
            .set_header("content-type", "text/plain")
            .set_raw_body(Value::from("internal"));
        let e = classify(&server_error).err().unwrap();
        assert!(!e.retryable);
        assert_eq!(e.detail, "HTTP 500 - internal");
        let unauthorized = EventEnvelope::new()
            .set_status(401)
            .set_header("content-type", "text/plain")
            .set_raw_body(Value::Binary(b"Token Authentication failed".to_vec()));
        let e = classify(&unauthorized).err().unwrap();
        assert!(!e.retryable);
        assert!(
            e.detail
                .starts_with("HTTP 401 - Token Authentication failed |"),
            "{}",
            e.detail
        );
    }

    #[test]
    fn failures_display_their_attempt_count_only_when_retried() {
        let once = ExportFailure {
            attempts: 1,
            detail: "HTTP 401".into(),
        };
        assert_eq!(once.to_string(), "HTTP 401");
        let five = ExportFailure {
            attempts: 5,
            detail: "HTTP 503 - busy".into(),
        };
        assert_eq!(five.to_string(), "HTTP 503 - busy (after 5 attempts)");
    }

    #[test]
    fn the_default_backoff_is_the_sdk_policy() {
        let exporter = Exporter::from_settings(ForwarderSettings::defaults()).unwrap();
        assert_eq!(
            exporter.backoff,
            [1000u64, 1500, 2250, 3375].map(Duration::from_millis)
        );
        assert_eq!(exporter.endpoint(), "http://localhost:4318/v1/traces");
        assert_eq!(exporter.target_host, "http://localhost:4318");
        assert_eq!(exporter.path, "/v1/traces");
        assert!(exporter.header_names().is_empty());
    }
}
