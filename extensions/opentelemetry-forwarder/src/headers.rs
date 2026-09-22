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

//! `otel.exporter.otlp.headers` parsing (Java `OtelForwarderContext.parseHeaders`).

/// Parse an OpenTelemetry `OTEL_EXPORTER_OTLP_HEADERS`-style value:
/// comma-separated pairs, each split on the FIRST `=` *or* `:`, whichever
/// appears earlier.
///
/// `=` is the OpenTelemetry environment-variable convention; `:` is literal
/// HTTP header syntax, which is what an operator naturally writes and what a
/// backend's own documentation shows (`Authorization: Api-Token <token>`).
/// Accepting both means a deployment can compose the header from a
/// vendor-specific prefix and a bare secret without the two having to agree on
/// a separator. Splitting on the first occurrence only means the VALUE may
/// contain either character — a base64 token ending in `=`, or a URL with
/// `https://` — while the NAME may contain neither, so the earlier separator
/// always delimits the name.
///
/// `None`, a blank value and the literal `null` (an unset credential variable)
/// yield no headers. A value cannot itself contain a comma: the list is split
/// on `,` first. A repeated name keeps its first position with the last value
/// (the Java `LinkedHashMap.put` behaviour).
pub fn parse_headers(raw: Option<&str>) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let Some(raw) = raw else {
        return out;
    };
    if raw.trim().is_empty() || raw.trim() == "null" {
        return out;
    }
    for pair in raw.split(',') {
        let Some(sep) = first_separator(pair) else {
            continue;
        };
        let key = pair[..sep].trim();
        let value = pair[sep + 1..].trim();
        if key.is_empty() {
            continue;
        }
        match out.iter_mut().find(|(k, _)| k == key) {
            Some(existing) => existing.1 = value.to_string(),
            None => out.push((key.to_string(), value.to_string())),
        }
    }
    out
}

/// Index of the first `=` or `:` in the pair, or `None` when it holds neither
/// (or the separator is the first character, so the name would be empty).
fn first_separator(pair: &str) -> Option<usize> {
    let sep = match (pair.find('='), pair.find(':')) {
        (Some(eq), Some(colon)) => eq.min(colon),
        (Some(eq), None) => eq,
        (None, Some(colon)) => colon,
        (None, None) => return None,
    };
    (sep > 0).then_some(sep)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pairs(raw: &str) -> Vec<(String, String)> {
        parse_headers(Some(raw))
    }

    #[test]
    fn equals_and_colon_forms_split_on_the_first_separator() {
        assert_eq!(
            pairs("Authorization=Api-Token abc=="),
            vec![("Authorization".into(), "Api-Token abc==".into())]
        );
        assert_eq!(
            pairs("Authorization: Api-Token abc"),
            vec![("Authorization".into(), "Api-Token abc".into())]
        );
        // a URL-shaped value keeps its own colon: the '=' comes first
        assert_eq!(
            pairs("X-Target=https://example.test/v1"),
            vec![("X-Target".into(), "https://example.test/v1".into())]
        );
        // the earlier of the two separators delimits the name
        assert_eq!(
            pairs("X-SF-Token:abc=def"),
            vec![("X-SF-Token".into(), "abc=def".into())]
        );
    }

    #[test]
    fn lists_keep_order_and_last_value_wins_for_a_repeated_name() {
        assert_eq!(
            pairs("a=1, b: 2 ,a=3"),
            vec![("a".into(), "3".into()), ("b".into(), "2".into())]
        );
    }

    #[test]
    fn unset_blank_null_and_malformed_yield_no_headers() {
        assert!(parse_headers(None).is_empty());
        assert!(pairs("").is_empty());
        assert!(pairs("   ").is_empty());
        assert!(pairs("null").is_empty());
        assert!(pairs("no-separator").is_empty());
        assert!(pairs("=value-without-name").is_empty());
    }
}
