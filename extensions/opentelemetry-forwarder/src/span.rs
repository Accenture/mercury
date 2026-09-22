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

//! The telemetry dataset → OpenTelemetry span mapping (Java
//! `TraceMetricsSpanData`).
//!
//! The engine has already produced the W3C-compatible trace id, span id and
//! parent span id during execution, so the span built here carries those
//! **exact** ids — an OpenTelemetry `Tracer` would mint new ones and break the
//! lineage. A dataset whose ids are not W3C-valid (32 / 16 lowercase hex, not
//! all zeros) is skipped rather than exported with forged ids.
//!
//! | Dataset metric | Span |
//! |----------------|------|
//! | `id` | trace id |
//! | `span_id` | span id |
//! | `parent_span_id` | parent span id (root span when absent) |
//! | `service` (route name) | span name (`path`, then `task`, when absent) |
//! | `start` + `exec_time` | start / end timestamps |
//! | `success` / `status` / `exception` | status OK, or ERROR with a description |
//! | `service` = `http.request` (the edge's round-trip record) | kind SERVER (every function execution is INTERNAL) |
//! | `path`, `from`, `origin`, `status`, `exec_time_ms`, `round_trip_ms`, `exception` | attributes (same names) |
//! | `service` | the `route` attribute |
//! | `annotations` entries | `annotation.<key>` attributes |

use std::time::{SystemTime, UNIX_EPOCH};

use rmpv::Value;

const NANOS_PER_MILLI: f64 = 1_000_000.0;
const HTTP_REQUEST: &str = "http.request";

/// OTLP `SpanKind` — only the two values the engine's datasets produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanKind {
    Internal = 1,
    Server = 2,
}

/// OTLP `StatusCode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Unset = 0,
    Ok = 1,
    Error = 2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpanStatus {
    pub code: StatusCode,
    /// The error description (empty for OK).
    pub message: String,
}

/// An OTLP `AnyValue` — the three scalar shapes the mapping emits.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    Str(String),
    Int(i64),
    Double(f64),
}

/// One completed span, ready for the OTLP encoder.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub trace_id: [u8; 16],
    pub span_id: [u8; 8],
    pub parent_span_id: Option<[u8; 8]>,
    pub name: String,
    pub kind: SpanKind,
    pub start_unix_nano: u64,
    pub end_unix_nano: u64,
    pub attributes: Vec<(String, AttributeValue)>,
    pub status: SpanStatus,
}

impl Span {
    /// Map one telemetry dataset (`{ trace: {...}, annotations: {...} }`) to a
    /// span — `None` when the dataset has no `trace` block or its trace / span
    /// id is not W3C-valid.
    pub fn from_dataset(dataset: &Value) -> Option<Span> {
        let entries = map_entries(dataset)?;
        let trace = field(entries, "trace").and_then(map_entries)?;
        let trace_id = hex_id::<16>(field(trace, "id"))?;
        let span_id = hex_id::<8>(field(trace, "span_id"))?;
        let parent_span_id = hex_id::<8>(field(trace, "parent_span_id"));
        let annotations = field(entries, "annotations")
            .and_then(map_entries)
            .unwrap_or(&[]);

        let start_unix_nano = field(trace, "start")
            .and_then(display)
            .and_then(|s| parse_iso8601_nanos(&s))
            .unwrap_or_else(now_nanos);
        let exec_ms = to_f64(field(trace, "exec_time"));
        let end_unix_nano = start_unix_nano + (exec_ms * NANOS_PER_MILLI) as u64;

        let status = if to_bool(field(trace, "success")) {
            SpanStatus {
                code: StatusCode::Ok,
                message: String::new(),
            }
        } else {
            let message = field(trace, "exception")
                .and_then(display)
                .unwrap_or_else(|| {
                    format!(
                        "status={}",
                        field(trace, "status")
                            .and_then(display)
                            .unwrap_or_else(|| "null".to_string())
                    )
                });
            SpanStatus {
                code: StatusCode::Error,
                message,
            }
        };

        let service = field(trace, "service").and_then(display);
        let path = field(trace, "path").and_then(display);
        let name = service
            .clone()
            .or_else(|| path.clone())
            .unwrap_or_else(|| "task".to_string());
        // the edge's round-trip record (service "http.request", emitted by REST
        // automation when the response completes) is the SERVER span; every
        // function execution - including the first one, whose `from` is
        // http.request - is an INTERNAL hop under it
        let kind = match service.as_deref() {
            Some(HTTP_REQUEST) => SpanKind::Server,
            _ => SpanKind::Internal,
        };

        let mut attributes: Vec<(String, AttributeValue)> = Vec::new();
        let mut put_str = |key: &str, value: Option<String>| {
            if let Some(value) = value {
                attributes.push((key.to_string(), AttributeValue::Str(value)));
            }
        };
        put_str("route", service);
        put_str("from", field(trace, "from").and_then(display));
        put_str("origin", field(trace, "origin").and_then(display));
        put_str("path", path);
        if let Some(status) = field(trace, "status").filter(|v| !v.is_nil()) {
            attributes.push(("status".to_string(), AttributeValue::Int(to_i64(status))));
        }
        attributes.push(("exec_time_ms".to_string(), AttributeValue::Double(exec_ms)));
        if let Some(round_trip) = field(trace, "round_trip").filter(|v| !v.is_nil()) {
            attributes.push((
                "round_trip_ms".to_string(),
                AttributeValue::Double(to_f64(Some(round_trip))),
            ));
        }
        if let Some(exception) = field(trace, "exception").and_then(display) {
            attributes.push(("exception".to_string(), AttributeValue::Str(exception)));
        }
        for (key, value) in annotations {
            if let (Some(key), Some(value)) = (key.as_str(), display(value)) {
                attributes.push((format!("annotation.{key}"), AttributeValue::Str(value)));
            }
        }

        Some(Span {
            trace_id,
            span_id,
            parent_span_id,
            name,
            kind,
            start_unix_nano,
            end_unix_nano,
            attributes,
            status,
        })
    }

    pub fn trace_id_hex(&self) -> String {
        hex(&self.trace_id)
    }

    pub fn span_id_hex(&self) -> String {
        hex(&self.span_id)
    }

    pub fn parent_span_id_hex(&self) -> Option<String> {
        self.parent_span_id.as_ref().map(|id| hex(id))
    }

    /// The value of a string attribute.
    pub fn attribute_str(&self, key: &str) -> Option<&str> {
        self.attributes.iter().find_map(|(k, v)| match v {
            AttributeValue::Str(s) if k == key => Some(s.as_str()),
            _ => None,
        })
    }

    /// The value of a numeric attribute (int or double) as f64.
    pub fn attribute_number(&self, key: &str) -> Option<f64> {
        self.attributes.iter().find_map(|(k, v)| match v {
            AttributeValue::Int(i) if k == key => Some(*i as f64),
            AttributeValue::Double(d) if k == key => Some(*d),
            _ => None,
        })
    }
}

// ---------------------------------------------------------------------------
// dataset access (the body arrives as a MessagePack value)
// ---------------------------------------------------------------------------

fn map_entries(value: &Value) -> Option<&[(Value, Value)]> {
    match value {
        Value::Map(entries) => Some(entries.as_slice()),
        _ => None,
    }
}

fn field<'a>(entries: &'a [(Value, Value)], key: &str) -> Option<&'a Value> {
    entries
        .iter()
        .find(|(k, _)| k.as_str() == Some(key))
        .map(|(_, v)| v)
}

/// Java `String.valueOf(value)`: text for scalars, JSON for structures, `None`
/// for a missing or nil value.
fn display(value: &Value) -> Option<String> {
    match value {
        Value::Nil => None,
        Value::String(s) => Some(s.as_str().unwrap_or_default().to_string()),
        Value::Boolean(b) => Some(b.to_string()),
        Value::Integer(i) => Some(i.to_string()),
        Value::F32(f) => Some(f.to_string()),
        Value::F64(f) => Some(f.to_string()),
        Value::Binary(bytes) => Some(String::from_utf8_lossy(bytes).to_string()),
        other => Some(
            rmpv::ext::from_value::<serde_json::Value>(other.clone())
                .map(|json| json.to_string())
                .unwrap_or_else(|_| other.to_string()),
        ),
    }
}

/// Java `toDouble`: parse via the canonical string form so a float-precision
/// `0.007` stays `0.007` instead of widening to `0.007000000216066837`.
fn to_f64(value: Option<&Value>) -> f64 {
    match value {
        None | Some(Value::Nil) => 0.0,
        Some(Value::F64(f)) => *f,
        Some(Value::F32(f)) => f.to_string().parse().unwrap_or(0.0),
        Some(Value::Integer(i)) => i.as_f64().unwrap_or(0.0),
        Some(Value::String(s)) => s
            .as_str()
            .and_then(|t| t.trim().parse().ok())
            .unwrap_or(0.0),
        Some(_) => 0.0,
    }
}

/// Java `toLong`: a number's integer value, a parseable string, else 0.
fn to_i64(value: &Value) -> i64 {
    match value {
        Value::Integer(i) => i.as_i64().unwrap_or(0),
        Value::F64(f) => *f as i64,
        Value::F32(f) => *f as i64,
        Value::String(s) => s.as_str().and_then(|t| t.trim().parse().ok()).unwrap_or(0),
        _ => 0,
    }
}

/// Java `toBool`: a boolean, or the text `true` (case-insensitive); a missing
/// value means success.
fn to_bool(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Nil) => true,
        Some(Value::Boolean(b)) => *b,
        Some(Value::String(s)) => s.as_str().is_some_and(|t| t.eq_ignore_ascii_case("true")),
        Some(_) => false,
    }
}

/// A W3C id: exactly `2 * N` lowercase hex digits, not all zeros.
fn hex_id<const N: usize>(value: Option<&Value>) -> Option<[u8; N]> {
    let text = display(value?)?;
    let bytes = text.as_bytes();
    if bytes.len() != 2 * N
        || !bytes
            .iter()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(b))
    {
        return None;
    }
    let mut out = [0u8; N];
    for (i, chunk) in bytes.chunks(2).enumerate() {
        out[i] = u8::from_str_radix(std::str::from_utf8(chunk).ok()?, 16).ok()?;
    }
    if out.iter().all(|b| *b == 0) {
        return None;
    }
    Some(out)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn now_nanos() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

/// Parse an ISO-8601 UTC instant (`YYYY-MM-DDTHH:MM:SS[.fraction]Z`, the shape
/// the engine writes) to nanoseconds since the Unix epoch — no date crate
/// (days-from-civil per Howard Hinnant, the inverse of the engine's formatter).
pub fn parse_iso8601_nanos(text: &str) -> Option<u64> {
    let b = text.trim().as_bytes();
    if b.len() < 20 {
        return None;
    }
    let num = |from: usize, to: usize| -> Option<i64> {
        let slice = std::str::from_utf8(&b[from..to]).ok()?;
        if !slice.bytes().all(|c| c.is_ascii_digit()) {
            return None;
        }
        slice.parse().ok()
    };
    if b[4] != b'-'
        || b[7] != b'-'
        || !matches!(b[10], b'T' | b't')
        || b[13] != b':'
        || b[16] != b':'
    {
        return None;
    }
    let (year, month, day) = (num(0, 4)?, num(5, 7)?, num(8, 10)?);
    let (hour, minute, second) = (num(11, 13)?, num(14, 16)?, num(17, 19)?);
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || hour > 23
        || minute > 59
        || second > 60
    {
        return None;
    }
    // optional fraction, then the mandatory Z
    let mut pos = 19;
    let mut nanos: u64 = 0;
    if b[pos] == b'.' {
        pos += 1;
        let start = pos;
        while pos < b.len() && b[pos].is_ascii_digit() {
            pos += 1;
        }
        let digits = &b[start..pos];
        if digits.is_empty() || digits.len() > 9 {
            return None;
        }
        for (i, d) in digits.iter().enumerate() {
            nanos += ((d - b'0') as u64) * 10u64.pow(8 - i as u32);
        }
    }
    if pos + 1 != b.len() || !matches!(b[pos], b'Z' | b'z') {
        return None;
    }
    let days = days_from_civil(year, month, day);
    let secs = days * 86_400 + hour * 3600 + minute * 60 + second;
    if secs < 0 {
        return None;
    }
    Some((secs as u64) * 1_000_000_000 + nanos)
}

/// Days since 1970-01-01 for a proleptic Gregorian date.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRACE_ID: &str = "4bf92f3577b34da6a3ce929d0e0e4736";
    const SPAN_ID: &str = "00f067aa0ba902b7";
    const PARENT_SPAN_ID: &str = "a3ce929d0e0e4736";

    fn dataset(success: bool) -> serde_json::Value {
        let mut trace = serde_json::json!({
            "id": TRACE_ID,
            "span_id": SPAN_ID,
            "parent_span_id": PARENT_SPAN_ID,
            "service": "hello.world",
            "path": "/api/hello",
            "from": "http.request",
            "origin": "node-1",
            "start": "2026-06-24T10:00:00Z",
            "exec_time": 12.5,
            "status": 200,
            "success": success,
        });
        if !success {
            trace["exception"] = serde_json::json!("boom");
        }
        serde_json::json!({ "trace": trace, "annotations": { "user": "alice" } })
    }

    fn map(dataset: &serde_json::Value) -> Option<Span> {
        Span::from_dataset(&rmpv::ext::to_value(dataset).expect("dataset"))
    }

    #[test]
    fn maps_ids_timing_and_attributes() {
        let span = map(&dataset(true)).expect("a valid dataset maps");
        assert_eq!(span.trace_id_hex(), TRACE_ID);
        assert_eq!(span.span_id_hex(), SPAN_ID);
        assert_eq!(span.parent_span_id_hex().as_deref(), Some(PARENT_SPAN_ID));
        assert_eq!(span.name, "hello.world");
        // a function execution is an INTERNAL hop, even the first one (from=http.request)
        assert_eq!(span.kind, SpanKind::Internal);
        assert_eq!(span.start_unix_nano, 1_782_295_200 * 1_000_000_000);
        assert_eq!(span.end_unix_nano - span.start_unix_nano, 12_500_000);
        assert_eq!(span.status.code, StatusCode::Ok);
        assert_eq!(span.attribute_str("path"), Some("/api/hello"));
        assert_eq!(span.attribute_str("route"), Some("hello.world"));
        assert_eq!(span.attribute_number("status"), Some(200.0));
        assert_eq!(span.attribute_number("exec_time_ms"), Some(12.5));
        assert_eq!(span.attribute_str("annotation.user"), Some("alice"));
    }

    #[test]
    fn failed_trace_becomes_error_status() {
        let span = map(&dataset(false)).expect("maps");
        assert_eq!(span.status.code, StatusCode::Error);
        assert_eq!(span.status.message, "boom");
        assert_eq!(span.attribute_str("exception"), Some("boom"));
    }

    #[test]
    fn invalid_or_missing_ids_yield_none() {
        let mut bad = dataset(true);
        bad["trace"]["id"] = serde_json::json!("not-a-valid-trace-id");
        assert!(map(&bad).is_none());
        let mut upper = dataset(true);
        upper["trace"]["id"] = serde_json::json!(TRACE_ID.to_uppercase());
        assert!(map(&upper).is_none(), "W3C ids are lowercase hex");
        let mut no_span = dataset(true);
        no_span["trace"].as_object_mut().unwrap().remove("span_id");
        assert!(map(&no_span).is_none());
        assert!(map(&serde_json::json!({ "annotations": {} })).is_none());
        let mut zero = dataset(true);
        zero["trace"]["id"] = serde_json::json!("00000000000000000000000000000000");
        assert!(map(&zero).is_none(), "all-zero ids are rejected");
    }

    #[test]
    fn root_span_has_no_parent_and_ignores_an_invalid_one() {
        let mut ds = dataset(true);
        ds["trace"]
            .as_object_mut()
            .unwrap()
            .remove("parent_span_id");
        assert!(map(&ds).unwrap().parent_span_id.is_none());
        ds["trace"]["parent_span_id"] = serde_json::json!("0000000000000000");
        assert!(map(&ds).unwrap().parent_span_id.is_none());
    }

    #[test]
    fn float_exec_time_keeps_three_decimal_precision() {
        // the engine sends exec_time as float milliseconds; an F32 must not surface widening noise
        let mut value = rmpv::ext::to_value(dataset(true)).unwrap();
        if let Value::Map(entries) = &mut value {
            if let Some((_, Value::Map(trace))) = entries
                .iter_mut()
                .find(|(k, _)| k.as_str() == Some("trace"))
            {
                for (k, v) in trace.iter_mut() {
                    if k.as_str() == Some("exec_time") {
                        *v = Value::F32(0.007);
                    }
                }
            }
        }
        let span = Span::from_dataset(&value).unwrap();
        assert_eq!(span.attribute_number("exec_time_ms"), Some(0.007));
        assert_eq!(span.end_unix_nano - span.start_unix_nano, 7_000);
    }

    #[test]
    fn handles_string_typed_and_optional_metrics() {
        let mut ds = dataset(true);
        ds["trace"]["exec_time"] = serde_json::json!("2.5");
        ds["trace"]["status"] = serde_json::json!("200");
        ds["trace"]["success"] = serde_json::json!("true");
        ds["trace"]["round_trip"] = serde_json::json!(3.75);
        let span = map(&ds).unwrap();
        assert_eq!(span.end_unix_nano - span.start_unix_nano, 2_500_000);
        assert_eq!(span.attribute_number("status"), Some(200.0));
        assert_eq!(span.attribute_number("round_trip_ms"), Some(3.75));
        assert_eq!(span.status.code, StatusCode::Ok);
    }

    #[test]
    fn invalid_numbers_fall_back_to_defaults() {
        let mut ds = dataset(true);
        ds["trace"]["exec_time"] = serde_json::json!("not-a-number");
        ds["trace"]["status"] = serde_json::json!("xyz");
        let span = map(&ds).unwrap();
        assert_eq!(span.end_unix_nano, span.start_unix_nano);
        assert_eq!(span.attribute_number("status"), Some(0.0));
    }

    #[test]
    fn error_without_exception_uses_the_status_description() {
        let mut ds = dataset(true);
        ds["trace"]["success"] = serde_json::json!(false);
        ds["trace"]["status"] = serde_json::json!(500);
        let span = map(&ds).unwrap();
        assert_eq!(span.status.code, StatusCode::Error);
        assert_eq!(span.status.message, "status=500");
    }

    #[test]
    fn edge_round_trip_record_is_the_server_span() {
        // REST automation emits one record per traced request with service
        // "http.request" - the round trip from receipt to the completed response;
        // it is the SERVER span and the first function's parent
        let mut ds = dataset(true);
        ds["trace"]["service"] = serde_json::json!("http.request");
        ds["trace"]["path"] = serde_json::json!("GET /api/hello");
        ds["trace"].as_object_mut().unwrap().remove("from");
        ds["trace"]["exec_time"] = serde_json::json!(2016.0);
        let span = map(&ds).expect("the edge record maps");
        assert_eq!(span.name, "http.request");
        assert_eq!(span.kind, SpanKind::Server);
        assert_eq!(span.end_unix_nano - span.start_unix_nano, 2_016_000_000);
        assert_eq!(span.attribute_str("path"), Some("GET /api/hello"));
        assert!(span.attribute_str("from").is_none());
    }

    #[test]
    fn span_name_falls_back_to_path_then_task_and_kind_to_internal() {
        let mut ds = dataset(true);
        ds["trace"].as_object_mut().unwrap().remove("service");
        ds["trace"]["from"] = serde_json::json!("some.route");
        let span = map(&ds).unwrap();
        assert_eq!(span.name, "/api/hello");
        assert_eq!(span.kind, SpanKind::Internal);
        assert!(span.attribute_str("route").is_none());
        ds["trace"].as_object_mut().unwrap().remove("path");
        assert_eq!(map(&ds).unwrap().name, "task");
    }

    #[test]
    fn a_missing_start_uses_now() {
        let mut ds = dataset(true);
        ds["trace"]["start"] = serde_json::json!("yesterday");
        let before = now_nanos();
        let span = map(&ds).unwrap();
        assert!(span.start_unix_nano >= before);
    }

    #[test]
    fn iso8601_parsing_matches_the_engine_formatter() {
        assert_eq!(
            parse_iso8601_nanos("1970-01-02T00:00:00Z"),
            Some(86_400 * 1_000_000_000)
        );
        assert_eq!(
            parse_iso8601_nanos("2000-03-01T00:00:00Z"),
            Some(951_868_800 * 1_000_000_000)
        );
        assert_eq!(
            parse_iso8601_nanos("2026-09-16T19:22:37.656Z"),
            Some(1_789_586_557 * 1_000_000_000 + 656_000_000)
        );
        assert_eq!(
            parse_iso8601_nanos("2026-06-24T10:00:00.5Z"),
            Some(1_782_295_200 * 1_000_000_000 + 500_000_000)
        );
        // the engine's own formatter round-trips
        let now = SystemTime::now();
        let millis = now.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let text = platform_core::trace::iso8601_utc(now);
        assert_eq!(parse_iso8601_nanos(&text), Some(millis * 1_000_000));
        for bad in [
            "",
            "2026-06-24",
            "2026-06-24T10:00:00",
            "2026-13-01T00:00:00Z",
            "2026-06-24 10:00:00Z",
            "2026-06-24T10:00:00.Z",
        ] {
            assert_eq!(parse_iso8601_nanos(bad), None, "{bad}");
        }
    }
}
