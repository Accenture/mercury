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

//! An in-process OTLP/HTTP collector double (the Java `MockOtlpCollector` /
//! `FlakyOtlpServer` twins): a tiny hyper server on a random loopback port that
//! captures every POST — transport headers plus the **decoded** OTLP protobuf
//! (resource, scope, spans) — and answers a scripted status sequence (then 200,
//! an empty and therefore valid `ExportTraceServiceResponse`). It can also KILL
//! the first N connections after reading the request, so the client sees EOF
//! where the status line should be — the transport failure the retry exists for.

#![allow(dead_code)]

use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use opentelemetry_forwarder::otlp::{
    ProtoReader, WIRE_FIXED32, WIRE_FIXED64, WIRE_LEN, WIRE_VARINT,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Notify;

/// One span as decoded from the wire.
#[derive(Clone, Debug, Default)]
pub struct DecodedSpan {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub name: String,
    pub kind: u64,
    pub start_unix_nano: u64,
    pub end_unix_nano: u64,
    pub status_code: u64,
    pub status_message: String,
    pub flags: u32,
    /// Attribute values rendered as text (`kind`-agnostic).
    pub attributes: Vec<(String, String)>,
}

impl DecodedSpan {
    pub fn attribute(&self, key: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// One captured request.
#[derive(Clone, Debug, Default)]
pub struct Captured {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body_len: usize,
    pub service_name: Option<String>,
    pub scope_name: Option<String>,
    pub scope_version: Option<String>,
    pub spans: Vec<DecodedSpan>,
}

impl Captured {
    /// A request header, case-insensitively.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

struct State {
    captured: Mutex<Vec<Captured>>,
    requests: AtomicUsize,
    connections: AtomicUsize,
    /// Scripted answers `(status, body)`, consumed in order; then 200.
    script: Mutex<VecDeque<(u16, String)>>,
    kill_first_connections: usize,
    notify: Notify,
}

pub struct MockCollector {
    base_url: String,
    state: Arc<State>,
}

impl MockCollector {
    /// A healthy collector: every request is captured and answered 200.
    pub async fn start() -> MockCollector {
        Self::start_with(Vec::new(), 0).await
    }

    /// A scripted collector: the first requests are answered from `script`
    /// (status + body), later ones 200; the first `kill_first_connections`
    /// connections are read and then closed without any response.
    pub async fn start_with(
        script: Vec<(u16, &str)>,
        kill_first_connections: usize,
    ) -> MockCollector {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind mock collector");
        let port = listener.local_addr().expect("local addr").port();
        let state = Arc::new(State {
            captured: Mutex::new(Vec::new()),
            requests: AtomicUsize::new(0),
            connections: AtomicUsize::new(0),
            script: Mutex::new(
                script
                    .into_iter()
                    .map(|(status, body)| (status, body.to_string()))
                    .collect(),
            ),
            kill_first_connections,
            notify: Notify::new(),
        });
        let served = state.clone();
        tokio::spawn(async move {
            loop {
                let Ok((stream, _)) = listener.accept().await else {
                    break;
                };
                let n = served.connections.fetch_add(1, Ordering::AcqRel) + 1;
                if n <= served.kill_first_connections {
                    tokio::spawn(drain_and_drop(stream));
                    continue;
                }
                let state = served.clone();
                tokio::spawn(async move {
                    let service = service_fn(move |request| handle(state.clone(), request));
                    let _ = http1::Builder::new()
                        .serve_connection(TokioIo::new(stream), service)
                        .await;
                });
            }
        });
        MockCollector {
            base_url: format!("http://127.0.0.1:{port}"),
            state,
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{path}", self.base_url)
    }

    /// Requests that reached the HTTP layer (killed connections do not count).
    pub fn requests(&self) -> usize {
        self.state.requests.load(Ordering::Acquire)
    }

    /// Connections accepted, killed ones included.
    pub fn connections(&self) -> usize {
        self.state.connections.load(Ordering::Acquire)
    }

    pub fn captured(&self) -> Vec<Captured> {
        self.state.captured.lock().expect("captured").clone()
    }

    pub fn clear(&self) {
        self.state.captured.lock().expect("captured").clear();
    }

    /// Wait until at least `count` requests have been captured.
    pub async fn wait_for(&self, count: usize, timeout: Duration) -> Vec<Captured> {
        let deadline = Instant::now() + timeout;
        loop {
            let captured = self.captured();
            if captured.len() >= count {
                return captured;
            }
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                panic!(
                    "expected {count} captured request(s) within {timeout:?}, got {}",
                    captured.len()
                );
            }
            let _ = tokio::time::timeout(remaining, self.state.notify.notified()).await;
        }
    }
}

/// Read one HTTP request fully, then close the socket without answering.
async fn drain_and_drop(mut stream: TcpStream) {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 4096];
    let mut header_end = None;
    while header_end.is_none() {
        let Ok(n) = stream.read(&mut chunk).await else {
            return;
        };
        if n == 0 {
            return;
        }
        buf.extend_from_slice(&chunk[..n]);
        header_end = buf.windows(4).position(|w| w == b"\r\n\r\n").map(|p| p + 4);
    }
    let header_end = header_end.unwrap_or(buf.len());
    let head = String::from_utf8_lossy(&buf[..header_end]).to_lowercase();
    let content_length: usize = head
        .lines()
        .find_map(|l| l.strip_prefix("content-length:"))
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(0);
    while buf.len() < header_end + content_length {
        let Ok(n) = stream.read(&mut chunk).await else {
            return;
        };
        if n == 0 {
            return;
        }
        buf.extend_from_slice(&chunk[..n]);
    }
    let _ = stream.shutdown().await;
    // dropped here: the client sees EOF where the status line should be
}

async fn handle(
    state: Arc<State>,
    request: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let mut captured = Captured {
        method: request.method().to_string(),
        path: request.uri().path().to_string(),
        ..Captured::default()
    };
    for (name, value) in request.headers() {
        captured
            .headers
            .push((name.to_string(), value.to_str().unwrap_or("").to_string()));
    }
    let body = request.into_body().collect().await?.to_bytes();
    captured.body_len = body.len();
    decode_request(&body, &mut captured);
    state.captured.lock().expect("captured").push(captured);
    state.requests.fetch_add(1, Ordering::AcqRel);
    state.notify.notify_one();
    let (status, body) = state
        .script
        .lock()
        .expect("script")
        .pop_front()
        .unwrap_or((200, String::new()));
    let content_type = if status == 200 {
        "application/x-protobuf"
    } else {
        "text/plain"
    };
    Ok(Response::builder()
        .status(StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        .header("content-type", content_type)
        .body(Full::new(Bytes::from(body)))
        .expect("response"))
}

// ---------------------------------------------------------------------------
// OTLP decoding (ExportTraceServiceRequest → ResourceSpans → ScopeSpans → Span)
// ---------------------------------------------------------------------------

fn decode_request(body: &[u8], captured: &mut Captured) {
    let mut r = ProtoReader::new(body);
    while r.has_more() {
        let Some((field, wire)) = r.read_tag() else {
            return;
        };
        if field == 1 && wire == WIRE_LEN {
            let Some(resource_spans) = r.read_bytes() else {
                return;
            };
            decode_resource_spans(resource_spans, captured);
        } else if r.skip(wire).is_none() {
            return;
        }
    }
}

fn decode_resource_spans(data: &[u8], captured: &mut Captured) {
    let mut r = ProtoReader::new(data);
    while r.has_more() {
        let Some((field, wire)) = r.read_tag() else {
            return;
        };
        match (field, wire) {
            (1, WIRE_LEN) => {
                let Some(resource) = r.read_bytes() else {
                    return;
                };
                for (key, value) in decode_key_values(resource, 1) {
                    if key == "service.name" {
                        captured.service_name = Some(value);
                    }
                }
            }
            (2, WIRE_LEN) => {
                let Some(scope_spans) = r.read_bytes() else {
                    return;
                };
                decode_scope_spans(scope_spans, captured);
            }
            _ => {
                if r.skip(wire).is_none() {
                    return;
                }
            }
        }
    }
}

fn decode_scope_spans(data: &[u8], captured: &mut Captured) {
    let mut r = ProtoReader::new(data);
    while r.has_more() {
        let Some((field, wire)) = r.read_tag() else {
            return;
        };
        match (field, wire) {
            (1, WIRE_LEN) => {
                let Some(scope) = r.read_bytes() else {
                    return;
                };
                let mut s = ProtoReader::new(scope);
                while s.has_more() {
                    let Some((f, w)) = s.read_tag() else {
                        return;
                    };
                    match (f, w) {
                        (1, WIRE_LEN) => captured.scope_name = s.read_string(),
                        (2, WIRE_LEN) => captured.scope_version = s.read_string(),
                        _ => {
                            if s.skip(w).is_none() {
                                return;
                            }
                        }
                    }
                }
            }
            (2, WIRE_LEN) => {
                let Some(span) = r.read_bytes() else {
                    return;
                };
                if let Some(span) = decode_span(span) {
                    captured.spans.push(span);
                }
            }
            _ => {
                if r.skip(wire).is_none() {
                    return;
                }
            }
        }
    }
}

fn decode_span(data: &[u8]) -> Option<DecodedSpan> {
    let mut r = ProtoReader::new(data);
    let mut span = DecodedSpan::default();
    while r.has_more() {
        let (field, wire) = r.read_tag()?;
        match (field, wire) {
            (1, WIRE_LEN) => span.trace_id = hex(r.read_bytes()?),
            (2, WIRE_LEN) => span.span_id = hex(r.read_bytes()?),
            (4, WIRE_LEN) => span.parent_span_id = Some(hex(r.read_bytes()?)),
            (5, WIRE_LEN) => span.name = r.read_string()?,
            (6, WIRE_VARINT) => span.kind = r.read_varint()?,
            (7, WIRE_FIXED64) => span.start_unix_nano = r.read_fixed64()?,
            (8, WIRE_FIXED64) => span.end_unix_nano = r.read_fixed64()?,
            (9, WIRE_LEN) => {
                let kv = r.read_bytes()?;
                span.attributes.extend(decode_key_values_one(kv));
            }
            (15, WIRE_LEN) => {
                let mut s = ProtoReader::new(r.read_bytes()?);
                while s.has_more() {
                    let (f, w) = s.read_tag()?;
                    match (f, w) {
                        (2, WIRE_LEN) => span.status_message = s.read_string()?,
                        (3, WIRE_VARINT) => span.status_code = s.read_varint()?,
                        _ => s.skip(w)?,
                    }
                }
            }
            (16, WIRE_FIXED32) => span.flags = r.read_fixed32()?,
            _ => r.skip(wire)?,
        }
    }
    Some(span)
}

/// `repeated KeyValue` at `field` of a message.
fn decode_key_values(data: &[u8], field_number: u32) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut r = ProtoReader::new(data);
    while r.has_more() {
        let Some((field, wire)) = r.read_tag() else {
            break;
        };
        if field == field_number && wire == WIRE_LEN {
            let Some(kv) = r.read_bytes() else {
                break;
            };
            out.extend(decode_key_values_one(kv));
        } else if r.skip(wire).is_none() {
            break;
        }
    }
    out
}

/// One `KeyValue { key = 1; AnyValue value = 2 }`, the scalar value as text.
fn decode_key_values_one(data: &[u8]) -> Option<(String, String)> {
    let mut r = ProtoReader::new(data);
    let mut key = String::new();
    let mut value = String::new();
    while r.has_more() {
        let (field, wire) = r.read_tag()?;
        match (field, wire) {
            (1, WIRE_LEN) => key = r.read_string()?,
            (2, WIRE_LEN) => {
                let mut v = ProtoReader::new(r.read_bytes()?);
                while v.has_more() {
                    let (f, w) = v.read_tag()?;
                    match (f, w) {
                        (1, WIRE_LEN) => value = v.read_string()?,
                        (2, WIRE_VARINT) => value = (v.read_varint()? != 0).to_string(),
                        (3, WIRE_VARINT) => value = (v.read_varint()? as i64).to_string(),
                        (4, WIRE_FIXED64) => value = f64::from_bits(v.read_fixed64()?).to_string(),
                        _ => v.skip(w)?,
                    }
                }
            }
            _ => r.skip(wire)?,
        }
    }
    Some((key, value))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
