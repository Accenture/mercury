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

//! The OTLP wire format — a hand-written protobuf encoder for the one message
//! this crate sends, `ExportTraceServiceRequest`, and a reader for the one it
//! receives, `ExportTraceServiceResponse`.
//!
//! No `prost`, no generated code, no OpenTelemetry SDK: the OTLP v1 trace schema
//! is frozen and the forwarder needs eight message types with scalar fields.
//! Field numbers are the OTLP `trace.proto` / `common.proto` /
//! `resource.proto` / `trace_service.proto` definitions; the encoding rules are
//! <https://protobuf.dev/programming-guides/encoding/>. The Java module's own
//! test collector decodes the same bytes the same way.
//!
//! ```text
//! ExportTraceServiceRequest { repeated ResourceSpans resource_spans = 1; }
//! ResourceSpans   { Resource resource = 1; repeated ScopeSpans scope_spans = 2; }
//! Resource        { repeated KeyValue attributes = 1; }
//! ScopeSpans      { InstrumentationScope scope = 1; repeated Span spans = 2; }
//! InstrumentationScope { string name = 1; string version = 2; }
//! KeyValue        { string key = 1; AnyValue value = 2; }
//! AnyValue        { oneof value { string string_value = 1; bool bool_value = 2;
//!                                 int64 int_value = 3; double double_value = 4; } }
//! Span            { bytes trace_id = 1; bytes span_id = 2; bytes parent_span_id = 4;
//!                   string name = 5; SpanKind kind = 6; fixed64 start_time_unix_nano = 7;
//!                   fixed64 end_time_unix_nano = 8; repeated KeyValue attributes = 9;
//!                   Status status = 15; fixed32 flags = 16; }
//! Status          { string message = 2; StatusCode code = 3; }
//! ExportTraceServiceResponse { ExportTracePartialSuccess partial_success = 1; }
//! ExportTracePartialSuccess  { int64 rejected_spans = 1; string error_message = 2; }
//! ```

use crate::span::{AttributeValue, Span, StatusCode};

/// Protobuf wire types (the low 3 bits of a field tag).
pub const WIRE_VARINT: u8 = 0;
pub const WIRE_FIXED64: u8 = 1;
pub const WIRE_LEN: u8 = 2;
pub const WIRE_FIXED32: u8 = 5;

/// OTLP span `flags`: bits 0–7 are the W3C trace flags (`0x01` = sampled), bit
/// 8 says the is-remote bit is known, bit 9 is the is-remote bit itself. A span
/// the engine produced is sampled and local — the value the Java SDK writes.
pub const SPAN_FLAGS_SAMPLED_LOCAL: u32 = 0x0000_0101;

/// A minimal protobuf writer: field tags, varints, fixed-width scalars and
/// length-delimited payloads. Proto3 default values are omitted, except for
/// `oneof` members (which have explicit presence) — see the `*_always` methods.
#[derive(Default)]
pub struct ProtoWriter {
    buf: Vec<u8>,
}

impl ProtoWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    /// Base-128 varint: little-endian groups of 7 bits, high bit = more follows.
    pub fn varint(&mut self, mut value: u64) {
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            if value == 0 {
                self.buf.push(byte);
                return;
            }
            self.buf.push(byte | 0x80);
        }
    }

    fn tag(&mut self, field: u32, wire: u8) {
        self.varint(((field as u64) << 3) | wire as u64);
    }

    /// `string` — omitted when empty (proto3 default).
    pub fn string(&mut self, field: u32, value: &str) {
        if !value.is_empty() {
            self.bytes(field, value.as_bytes());
        }
    }

    /// A `oneof` string member — always written, even when empty.
    pub fn string_always(&mut self, field: u32, value: &str) {
        self.tag(field, WIRE_LEN);
        self.varint(value.len() as u64);
        self.buf.extend_from_slice(value.as_bytes());
    }

    /// `bytes` — omitted when empty.
    pub fn bytes(&mut self, field: u32, value: &[u8]) {
        if !value.is_empty() {
            self.tag(field, WIRE_LEN);
            self.varint(value.len() as u64);
            self.buf.extend_from_slice(value);
        }
    }

    /// An embedded message — always written (an empty message is meaningful).
    pub fn message(&mut self, field: u32, body: &[u8]) {
        self.tag(field, WIRE_LEN);
        self.varint(body.len() as u64);
        self.buf.extend_from_slice(body);
    }

    /// `uint32` / `uint64` / enum — omitted when zero.
    pub fn uint(&mut self, field: u32, value: u64) {
        if value != 0 {
            self.tag(field, WIRE_VARINT);
            self.varint(value);
        }
    }

    /// A `oneof` `int64` member — always written (two's complement varint).
    pub fn int64_always(&mut self, field: u32, value: i64) {
        self.tag(field, WIRE_VARINT);
        self.varint(value as u64);
    }

    /// `fixed64` — omitted when zero.
    pub fn fixed64(&mut self, field: u32, value: u64) {
        if value != 0 {
            self.tag(field, WIRE_FIXED64);
            self.buf.extend_from_slice(&value.to_le_bytes());
        }
    }

    /// A `oneof` `double` member — always written (IEEE-754 bits, little-endian).
    pub fn double_always(&mut self, field: u32, value: f64) {
        self.tag(field, WIRE_FIXED64);
        self.buf.extend_from_slice(&value.to_bits().to_le_bytes());
    }

    /// `fixed32` — omitted when zero.
    pub fn fixed32(&mut self, field: u32, value: u32) {
        if value != 0 {
            self.tag(field, WIRE_FIXED32);
            self.buf.extend_from_slice(&value.to_le_bytes());
        }
    }
}

fn any_value(value: &AttributeValue) -> Vec<u8> {
    let mut w = ProtoWriter::new();
    match value {
        AttributeValue::Str(s) => w.string_always(1, s),
        AttributeValue::Int(i) => w.int64_always(3, *i),
        AttributeValue::Double(d) => w.double_always(4, *d),
    }
    w.into_bytes()
}

fn key_value(key: &str, value: &AttributeValue) -> Vec<u8> {
    let mut w = ProtoWriter::new();
    w.string(1, key);
    w.message(2, &any_value(value));
    w.into_bytes()
}

fn resource(service_name: &str) -> Vec<u8> {
    let mut w = ProtoWriter::new();
    w.message(
        1,
        &key_value(
            "service.name",
            &AttributeValue::Str(service_name.to_string()),
        ),
    );
    w.into_bytes()
}

fn instrumentation_scope(name: &str, version: &str) -> Vec<u8> {
    let mut w = ProtoWriter::new();
    w.string(1, name);
    w.string(2, version);
    w.into_bytes()
}

fn status(span: &Span) -> Option<Vec<u8>> {
    if span.status.code == StatusCode::Unset && span.status.message.is_empty() {
        return None;
    }
    let mut w = ProtoWriter::new();
    w.string(2, &span.status.message);
    w.uint(3, span.status.code as u64);
    Some(w.into_bytes())
}

/// The OTLP `Span` message for one mapped span.
pub fn span_message(span: &Span) -> Vec<u8> {
    let mut w = ProtoWriter::new();
    w.bytes(1, &span.trace_id);
    w.bytes(2, &span.span_id);
    if let Some(parent) = &span.parent_span_id {
        w.bytes(4, parent);
    }
    w.string(5, &span.name);
    w.uint(6, span.kind as u64);
    w.fixed64(7, span.start_unix_nano);
    w.fixed64(8, span.end_unix_nano);
    for (key, value) in &span.attributes {
        w.message(9, &key_value(key, value));
    }
    if let Some(status) = status(span) {
        w.message(15, &status);
    }
    w.fixed32(16, SPAN_FLAGS_SAMPLED_LOCAL);
    w.into_bytes()
}

/// One `ExportTraceServiceRequest` carrying one span under one resource and
/// one instrumentation scope — the request body of an OTLP/HTTP export.
pub fn encode_export_request(
    service_name: &str,
    scope_name: &str,
    scope_version: &str,
    span: &Span,
) -> Vec<u8> {
    let mut scope_spans = ProtoWriter::new();
    scope_spans.message(1, &instrumentation_scope(scope_name, scope_version));
    scope_spans.message(2, &span_message(span));
    let mut resource_spans = ProtoWriter::new();
    resource_spans.message(1, &resource(service_name));
    resource_spans.message(2, &scope_spans.into_bytes());
    let mut request = ProtoWriter::new();
    request.message(1, &resource_spans.into_bytes());
    request.into_bytes()
}

// ---------------------------------------------------------------------------
// reading
// ---------------------------------------------------------------------------

/// A minimal, bounds-checked protobuf reader — enough to walk the OTLP message
/// tree (the response's `partial_success`, and the test collectors' decoding of
/// what this crate sent). Every read returns `None` past the end instead of
/// panicking, so a malformed body is reported, never fatal.
pub struct ProtoReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ProtoReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        ProtoReader { buf, pos: 0 }
    }

    pub fn has_more(&self) -> bool {
        self.pos < self.buf.len()
    }

    /// The next field tag as `(field_number, wire_type)`.
    pub fn read_tag(&mut self) -> Option<(u32, u8)> {
        let tag = self.read_varint()?;
        Some(((tag >> 3) as u32, (tag & 0x7) as u8))
    }

    pub fn read_varint(&mut self) -> Option<u64> {
        let mut result: u64 = 0;
        let mut shift = 0u32;
        loop {
            let byte = *self.buf.get(self.pos)?;
            self.pos += 1;
            if shift > 63 {
                return None;
            }
            result |= ((byte & 0x7f) as u64) << shift;
            if byte & 0x80 == 0 {
                return Some(result);
            }
            shift += 7;
        }
    }

    pub fn read_fixed64(&mut self) -> Option<u64> {
        let bytes: [u8; 8] = self.buf.get(self.pos..self.pos + 8)?.try_into().ok()?;
        self.pos += 8;
        Some(u64::from_le_bytes(bytes))
    }

    pub fn read_fixed32(&mut self) -> Option<u32> {
        let bytes: [u8; 4] = self.buf.get(self.pos..self.pos + 4)?.try_into().ok()?;
        self.pos += 4;
        Some(u32::from_le_bytes(bytes))
    }

    /// A length-delimited chunk: bytes, a string, or an embedded message.
    pub fn read_bytes(&mut self) -> Option<&'a [u8]> {
        let len = self.read_varint()? as usize;
        let out = self.buf.get(self.pos..self.pos + len)?;
        self.pos += len;
        Some(out)
    }

    pub fn read_string(&mut self) -> Option<String> {
        self.read_bytes()
            .map(|b| String::from_utf8_lossy(b).to_string())
    }

    /// Advance past a field whose value is not needed, honouring its wire type.
    pub fn skip(&mut self, wire: u8) -> Option<()> {
        match wire {
            WIRE_VARINT => self.read_varint().map(|_| ()),
            WIRE_FIXED64 => self.read_fixed64().map(|_| ()),
            WIRE_LEN => self.read_bytes().map(|_| ()),
            WIRE_FIXED32 => self.read_fixed32().map(|_| ()),
            _ => None,
        }
    }
}

/// The `partial_success` block of an `ExportTraceServiceResponse`, when the
/// body carries one with content — a backend that accepted the request but
/// rejected spans says so here.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PartialSuccess {
    pub rejected_spans: i64,
    pub error_message: String,
}

pub fn partial_success(body: &[u8]) -> Option<PartialSuccess> {
    let mut r = ProtoReader::new(body);
    let mut result: Option<PartialSuccess> = None;
    while r.has_more() {
        let (field, wire) = r.read_tag()?;
        if field == 1 && wire == WIRE_LEN {
            let mut inner = ProtoReader::new(r.read_bytes()?);
            let mut partial = PartialSuccess::default();
            while inner.has_more() {
                let (f, w) = inner.read_tag()?;
                match (f, w) {
                    (1, WIRE_VARINT) => partial.rejected_spans = inner.read_varint()? as i64,
                    (2, WIRE_LEN) => partial.error_message = inner.read_string()?,
                    _ => inner.skip(w)?,
                }
            }
            result = Some(partial);
        } else {
            r.skip(wire)?;
        }
    }
    result.filter(|p| p.rejected_spans != 0 || !p.error_message.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::{SpanKind, SpanStatus};

    fn sample() -> Span {
        Span {
            trace_id: [
                0x4b, 0xf9, 0x2f, 0x35, 0x77, 0xb3, 0x4d, 0xa6, 0xa3, 0xce, 0x92, 0x9d, 0x0e, 0x0e,
                0x47, 0x36,
            ],
            span_id: [0x00, 0xf0, 0x67, 0xaa, 0x0b, 0xa9, 0x02, 0xb7],
            parent_span_id: Some([0xa3, 0xce, 0x92, 0x9d, 0x0e, 0x0e, 0x47, 0x36]),
            name: "hello.world".into(),
            kind: SpanKind::Server,
            start_unix_nano: 1_782_295_200_000_000_000,
            end_unix_nano: 1_782_295_200_012_500_000,
            attributes: vec![
                ("route".into(), AttributeValue::Str("hello.world".into())),
                ("status".into(), AttributeValue::Int(200)),
                ("exec_time_ms".into(), AttributeValue::Double(12.5)),
            ],
            status: SpanStatus {
                code: StatusCode::Ok,
                message: String::new(),
            },
        }
    }

    #[test]
    fn varints_and_tags_follow_the_wire_format() {
        let mut w = ProtoWriter::new();
        w.varint(1);
        w.varint(300);
        w.varint(u64::MAX);
        assert_eq!(
            w.into_bytes(),
            [1, 0xac, 0x02, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]
        );
        // KeyValue { key = "k", value = AnyValue { string_value = "v" } }
        assert_eq!(
            key_value("k", &AttributeValue::Str("v".into())),
            [0x0a, 0x01, b'k', 0x12, 0x03, 0x0a, 0x01, b'v']
        );
        // a oneof member is written even when it holds the default value
        assert_eq!(any_value(&AttributeValue::Str(String::new())), [0x0a, 0x00]);
        assert_eq!(any_value(&AttributeValue::Int(0)), [0x18, 0x00]);
        assert_eq!(any_value(&AttributeValue::Int(-1)), {
            let mut v = vec![0x18];
            v.extend_from_slice(&[0xff; 9]);
            v.push(0x01);
            v
        });
        assert_eq!(
            any_value(&AttributeValue::Double(1.0)),
            [0x21, 0, 0, 0, 0, 0, 0, 0xf0, 0x3f]
        );
    }

    #[test]
    fn a_span_encodes_its_ids_as_raw_bytes_and_round_trips_through_the_reader() {
        let bytes = span_message(&sample());
        // first field: trace_id, 16 bytes
        assert_eq!(&bytes[..2], &[0x0a, 0x10]);
        let mut r = ProtoReader::new(&bytes);
        let mut seen = Vec::new();
        while r.has_more() {
            let (field, wire) = r.read_tag().unwrap();
            match (field, wire) {
                (1, WIRE_LEN) => assert_eq!(r.read_bytes().unwrap(), &sample().trace_id),
                (2, WIRE_LEN) => assert_eq!(r.read_bytes().unwrap(), &sample().span_id),
                (4, WIRE_LEN) => assert_eq!(
                    r.read_bytes().unwrap(),
                    &[0xa3, 0xce, 0x92, 0x9d, 0x0e, 0x0e, 0x47, 0x36]
                ),
                (5, WIRE_LEN) => assert_eq!(r.read_string().unwrap(), "hello.world"),
                (6, WIRE_VARINT) => assert_eq!(r.read_varint().unwrap(), 2),
                (7, WIRE_FIXED64) => {
                    assert_eq!(r.read_fixed64().unwrap(), 1_782_295_200_000_000_000)
                }
                (8, WIRE_FIXED64) => {
                    assert_eq!(r.read_fixed64().unwrap(), 1_782_295_200_012_500_000)
                }
                (9, WIRE_LEN) => {
                    r.read_bytes().unwrap();
                }
                (15, WIRE_LEN) => assert_eq!(
                    r.read_bytes().unwrap(),
                    &[0x18, 0x01],
                    "Status {{ code = OK }}"
                ),
                (16, WIRE_FIXED32) => {
                    assert_eq!(r.read_fixed32().unwrap(), SPAN_FLAGS_SAMPLED_LOCAL)
                }
                other => panic!("unexpected field {other:?}"),
            }
            seen.push(field);
        }
        assert_eq!(seen, [1, 2, 4, 5, 6, 7, 8, 9, 9, 9, 15, 16]);
    }

    #[test]
    fn the_export_request_nests_resource_scope_and_span() {
        let bytes = encode_export_request("mercury-otel-demo", "scope", "1.2.3", &sample());
        let mut r = ProtoReader::new(&bytes);
        let (field, wire) = r.read_tag().unwrap();
        assert_eq!(
            (field, wire),
            (1, WIRE_LEN),
            "ExportTraceServiceRequest.resource_spans"
        );
        let resource_spans = r.read_bytes().unwrap();
        assert!(!r.has_more(), "exactly one ResourceSpans");
        let mut rs = ProtoReader::new(resource_spans);
        assert_eq!(rs.read_tag().unwrap(), (1, WIRE_LEN), "Resource");
        let resource = rs.read_bytes().unwrap();
        assert_eq!(resource, {
            let mut w = ProtoWriter::new();
            w.message(
                1,
                &key_value(
                    "service.name",
                    &AttributeValue::Str("mercury-otel-demo".into()),
                ),
            );
            w.into_bytes()
        });
        assert_eq!(rs.read_tag().unwrap(), (2, WIRE_LEN), "ScopeSpans");
        let mut ss = ProtoReader::new(rs.read_bytes().unwrap());
        assert_eq!(ss.read_tag().unwrap(), (1, WIRE_LEN));
        assert_eq!(
            ss.read_bytes().unwrap(),
            &instrumentation_scope("scope", "1.2.3")[..]
        );
        assert_eq!(ss.read_tag().unwrap(), (2, WIRE_LEN));
        assert_eq!(ss.read_bytes().unwrap(), &span_message(&sample())[..]);
        assert!(!ss.has_more());
    }

    #[test]
    fn a_root_span_and_an_unset_status_omit_their_fields() {
        let mut span = sample();
        span.parent_span_id = None;
        span.status = SpanStatus {
            code: StatusCode::Unset,
            message: String::new(),
        };
        let bytes = span_message(&span);
        let mut r = ProtoReader::new(&bytes);
        let mut fields = Vec::new();
        while r.has_more() {
            let (field, wire) = r.read_tag().unwrap();
            r.skip(wire).unwrap();
            fields.push(field);
        }
        assert!(!fields.contains(&4), "no parent_span_id for a root span");
        assert!(!fields.contains(&15), "no Status when unset");
    }

    #[test]
    fn partial_success_is_read_and_an_empty_response_is_none() {
        assert_eq!(partial_success(&[]), None);
        // ExportTraceServiceResponse { partial_success { rejected_spans = 1, error_message = "bad id" } }
        let mut inner = ProtoWriter::new();
        inner.uint(1, 1);
        inner.string(2, "bad id");
        let mut outer = ProtoWriter::new();
        outer.message(1, &inner.into_bytes());
        assert_eq!(
            partial_success(&outer.into_bytes()),
            Some(PartialSuccess {
                rejected_spans: 1,
                error_message: "bad id".into()
            })
        );
        // an all-default block (nothing rejected) reads as no partial success
        let mut empty = ProtoWriter::new();
        empty.message(1, &[]);
        assert_eq!(partial_success(&empty.into_bytes()), None);
        // truncated input is None, not a panic
        assert_eq!(partial_success(&[0x0a, 0x10, 0x08]), None);
    }
}
