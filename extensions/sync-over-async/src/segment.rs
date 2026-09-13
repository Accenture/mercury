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

//! One entry of a `queue:{cid}` rendezvous queue — Rust port of the Java
//! `StreamSegment`, and the **normative wire envelope** both engines share.
//!
//! Serialized as compact JSON, e.g. `{"type":"data","name":"tokens","body":"..."}`;
//! `name` (an optional SSE event name) and `body` are omitted when absent, so
//! a Rust-written entry is byte-identical to the Java form for the same values.
//!
//! The `eof` and `exception` types are [terminal](StreamSegment::is_terminal):
//! "the end signal is also an event", stored like any other segment, so it can
//! never outrun the data it follows. A one-shot response is the degenerate
//! case — a queue whose first entry is terminal.
//!
//! There is deliberately no sequence number: ordering, where required, is the
//! producer's posting discipline (post sequentially over one connection), and
//! reads are destructive pops, so neither side keeps an index.

use platform_core::AppError;
use serde::Serialize;
use serde_json::Value;

/// A progressive segment of the stream.
pub const DATA: &str = "data";
/// End of transmission (optional body = trailing metadata).
pub const EOF: &str = "eof";
/// In-band failure.
pub const EXCEPTION: &str = "exception";

const TYPE_FIELD: &str = "type";
const NAME_FIELD: &str = "name";
const BODY_FIELD: &str = "body";

/// One queued segment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StreamSegment {
    #[serde(rename = "type")]
    segment_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
}

impl StreamSegment {
    /// Validating factory used by the producer paths (Java `StreamSegment.of`).
    ///
    /// Returns HTTP-400 when `segment_type` is not `data`, `eof` or `exception`.
    pub fn of(
        segment_type: &str,
        name: Option<&str>,
        body: Option<&str>,
    ) -> Result<Self, AppError> {
        if !matches!(segment_type, DATA | EOF | EXCEPTION) {
            return Err(AppError::new(
                400,
                format!(
                    "Segment type must be one of [{DATA}, {EOF}, {EXCEPTION}], not {segment_type}"
                ),
            ));
        }
        Ok(StreamSegment {
            segment_type: segment_type.to_string(),
            name: name.map(str::to_string),
            body: body.map(str::to_string),
        })
    }

    pub fn segment_type(&self) -> &str {
        &self.segment_type
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    /// True for the `eof` and `exception` entries that complete a rendezvous.
    pub fn is_terminal(&self) -> bool {
        self.segment_type == EOF || self.segment_type == EXCEPTION
    }

    /// Serialize to the compact JSON wire form (absent fields omitted).
    pub fn to_json(&self) -> String {
        // the derive above cannot fail: three plain string fields
        serde_json::to_string(self)
            .unwrap_or_else(|_| format!("{{\"{TYPE_FIELD}\":\"{EXCEPTION}\"}}"))
    }

    /// Parse a stored queue entry.
    ///
    /// Tolerant on the value side the way the Java reader is: a `name` or
    /// `body` that arrives as a number or boolean is rendered as text rather
    /// than rejected, so a peer engine's encoder cannot break a render.
    /// Returns HTTP-400 when the text is not a segment (not JSON, or an
    /// unknown type).
    pub fn from_json(json: &str) -> Result<Self, AppError> {
        let value: Value = serde_json::from_str(json)
            .map_err(|_| AppError::new(400, format!("Not a stream segment: {json}")))?;
        let Some(fields) = value.as_object() else {
            return Err(AppError::new(400, format!("Not a stream segment: {json}")));
        };
        let segment_type = match fields.get(TYPE_FIELD).and_then(Value::as_str) {
            Some(found) if matches!(found, DATA | EOF | EXCEPTION) => found,
            other => {
                return Err(AppError::new(
                    400,
                    format!(
                        "Segment type must be one of [{DATA}, {EOF}, {EXCEPTION}], not {}",
                        other.unwrap_or("<missing>")
                    ),
                ))
            }
        };
        Ok(StreamSegment {
            segment_type: segment_type.to_string(),
            name: fields.get(NAME_FIELD).and_then(as_text),
            body: fields.get(BODY_FIELD).and_then(as_text),
        })
    }
}

/// Render a JSON value as segment text; `null` is absent.
fn as_text(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(text) => Some(text.clone()),
        other => Some(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_json_omits_absent_fields() {
        let segment = StreamSegment::of(DATA, None, Some("hello")).expect("valid");
        assert_eq!(r#"{"type":"data","body":"hello"}"#, segment.to_json());
        let named = StreamSegment::of(DATA, Some("orders"), Some("42")).expect("valid");
        assert_eq!(
            r#"{"type":"data","name":"orders","body":"42"}"#,
            named.to_json()
        );
        let bare = StreamSegment::of(EOF, None, None).expect("valid");
        assert_eq!(r#"{"type":"eof"}"#, bare.to_json());
    }

    #[test]
    fn round_trips_through_the_wire_form() {
        for segment in [
            StreamSegment::of(DATA, Some("tokens"), Some("The")).expect("valid"),
            StreamSegment::of(EOF, None, Some("{\"total\":8}")).expect("valid"),
            StreamSegment::of(EXCEPTION, None, None).expect("valid"),
        ] {
            assert_eq!(
                segment,
                StreamSegment::from_json(&segment.to_json()).expect("parses")
            );
        }
    }

    #[test]
    fn terminal_types_complete_a_rendezvous() {
        assert!(!StreamSegment::of(DATA, None, None)
            .expect("valid")
            .is_terminal());
        assert!(StreamSegment::of(EOF, None, None)
            .expect("valid")
            .is_terminal());
        assert!(StreamSegment::of(EXCEPTION, None, None)
            .expect("valid")
            .is_terminal());
    }

    #[test]
    fn rejects_an_unknown_type() {
        assert_eq!(
            400,
            StreamSegment::of("progress", None, None)
                .expect_err("rejected")
                .status()
        );
        assert_eq!(
            400,
            StreamSegment::from_json(r#"{"type":"progress"}"#)
                .expect_err("rejected")
                .status()
        );
        assert_eq!(
            400,
            StreamSegment::from_json("not json")
                .expect_err("rejected")
                .status()
        );
        assert_eq!(
            400,
            StreamSegment::from_json("[1,2]")
                .expect_err("rejected")
                .status()
        );
    }

    #[test]
    fn a_non_string_value_is_rendered_rather_than_rejected() {
        // a peer engine's encoder must never be able to break a render
        let segment =
            StreamSegment::from_json(r#"{"type":"data","name":7,"body":true}"#).expect("tolerated");
        assert_eq!(Some("7"), segment.name());
        assert_eq!(Some("true"), segment.body());
        let null_body =
            StreamSegment::from_json(r#"{"type":"eof","body":null}"#).expect("tolerated");
        assert_eq!(None, null_body.body());
    }
}
