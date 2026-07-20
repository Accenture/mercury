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

//! Rust port of the Java `W3cTrace` (`org.platformlambda.core.util.W3cTrace`) —
//! helper for W3C Trace Context `traceparent` propagation across HTTP
//! boundaries (used by REST automation / HTTP client in later increments).
//!
//! Format: `version-trace-id-parent-id-trace-flags`, e.g.
//! `00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01`.
//! The trace-id is 32 hex and the parent-id — the caller's span — is 16 hex;
//! these match the auto-generated trace/span ID formats, so they propagate
//! directly.

pub const TRACEPARENT: &str = "traceparent";
const VERSION: &str = "00";
const SAMPLED: &str = "01";
const ZERO_TRACE: &str = "00000000000000000000000000000000";
const ZERO_SPAN: &str = "0000000000000000";

/// Build a W3C `traceparent` value from a trace ID and the current span ID
/// (which becomes the downstream's parent span). `None` if either ID is not
/// W3C-compatible.
pub fn format(trace_id: &str, span_id: &str) -> Option<String> {
    if valid_trace_id(trace_id) && valid_span_id(span_id) {
        Some(format!("{VERSION}-{trace_id}-{span_id}-{SAMPLED}"))
    } else {
        None
    }
}

/// Parse a W3C `traceparent` value into `(trace_id, parent_span_id)`.
/// `None` when the value is invalid. The trace-id and parent-id are always the
/// 2nd and 3rd fields across all versions.
pub fn parse(traceparent: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = traceparent.trim().split('-').collect();
    if parts.len() >= 4 && valid_trace_id(parts[1]) && valid_span_id(parts[2]) {
        Some((parts[1].to_string(), parts[2].to_string()))
    } else {
        None
    }
}

fn valid_trace_id(id: &str) -> bool {
    id.len() == 32 && is_lower_hex(id) && id != ZERO_TRACE
}

fn valid_span_id(id: &str) -> bool {
    id.len() == 16 && is_lower_hex(id) && id != ZERO_SPAN
}

fn is_lower_hex(s: &str) -> bool {
    s.bytes()
        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRACE: &str = "4bf92f3577b34da6a3ce929d0e0e4736";
    const SPAN: &str = "00f067aa0ba902b7";

    #[test]
    fn format_and_parse_round_trip() {
        let header = format(TRACE, SPAN).unwrap();
        assert_eq!(header, format!("00-{TRACE}-{SPAN}-01"));
        assert_eq!(parse(&header), Some((TRACE.to_string(), SPAN.to_string())));
    }

    #[test]
    fn invalid_values_are_rejected() {
        assert_eq!(format("short", SPAN), None);
        assert_eq!(format(ZERO_TRACE, SPAN), None);
        assert_eq!(format(TRACE, ZERO_SPAN), None);
        assert_eq!(format(&TRACE.to_uppercase(), SPAN), None);
        assert_eq!(parse("garbage"), None);
        assert_eq!(parse(&format!("00-{ZERO_TRACE}-{SPAN}-01")), None);
        // future versions still parse (2nd/3rd fields)
        assert!(parse(&format!("01-{TRACE}-{SPAN}-01-extra")).is_some());
    }
}
