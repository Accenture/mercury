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

//! Increment 59 — conformance against the **standard event envelope wire
//! format** golden vectors, shared verbatim with the Java engine
//! (`tests/resources/envelope-vectors/vectors.json`; normative spec:
//! `docs/guides/event-envelope-wire-format.md` in the Java repo). Per vector:
//! decode → compare each expected field semantically → re-encode with our
//! encoder → decode → compare again (round trip). Conformance is semantic —
//! map ordering is free, absent == nil, unknown keys ignored.

use base64::Engine;
use platform_core::EventEnvelope;

fn vectors() -> serde_json::Value {
    let text = std::fs::read_to_string("tests/resources/envelope-vectors/vectors.json")
        .expect("vectors fixture");
    serde_json::from_str(&text).expect("vectors json")
}

/// Compare one decoded envelope against a vector's `expect` map. Fields the
/// Rust envelope does not model (`tags`, `annotations`, `stack`, `obj_type`,
/// `exception` — Java extensions, spec-legal to ignore) are skipped.
fn assert_matches(name: &str, envelope: &EventEnvelope, expect: &serde_json::Value) {
    let expect = expect.as_object().expect("expect object");
    for (key, value) in expect {
        match key.as_str() {
            "id" => assert_eq!(envelope.id(), value.as_str().unwrap(), "{name}: id"),
            "to" => assert_eq!(envelope.to(), value.as_str(), "{name}: to"),
            "from" => assert_eq!(envelope.from(), value.as_str(), "{name}: from"),
            "reply_to" => assert_eq!(envelope.reply_to(), value.as_str(), "{name}: reply_to"),
            "cid" => assert_eq!(envelope.correlation_id(), value.as_str(), "{name}: cid"),
            "trace_id" => assert_eq!(envelope.trace_id(), value.as_str(), "{name}: trace_id"),
            "trace_path" => {
                assert_eq!(envelope.trace_path(), value.as_str(), "{name}: trace_path")
            }
            "span_id" => assert_eq!(envelope.span_id(), value.as_str(), "{name}: span_id"),
            "status" => assert_eq!(
                i64::from(envelope.status()),
                value.as_i64().unwrap(),
                "{name}: status"
            ),
            "exec_time" => assert_eq!(
                envelope.exec_time().map(f64::from),
                value.as_f64(),
                "{name}: exec_time"
            ),
            "round_trip" => assert_eq!(
                envelope.round_trip().map(f64::from),
                value.as_f64(),
                "{name}: round_trip"
            ),
            "headers" => {
                let expected: std::collections::HashMap<String, String> =
                    serde_json::from_value(value.clone()).expect("headers map");
                assert_eq!(envelope.headers(), &expected, "{name}: headers");
            }
            "body" => {
                let actual =
                    serde_json::to_value(envelope.body()).expect("body as comparable json");
                assert_eq!(&actual, value, "{name}: body");
            }
            // Java extension fields — not modeled here, ignored per the spec
            "tags" | "annotations" | "stack" | "obj_type" | "exception" => {}
            other => panic!("{name}: vector expects unknown field '{other}'"),
        }
    }
}

#[test]
fn standard_vectors_decode_and_round_trip() {
    let doc = vectors();
    let all = doc["vectors"].as_array().expect("vector list");
    let mut standard = 0;
    for vector in all {
        let name = vector["name"].as_str().expect("name");
        if vector["format"] != "standard" {
            // compact (legacy Java) decode is not implemented — v1 accepts
            // the standard format only, per the phase-2 handoff decision
            continue;
        }
        standard += 1;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(vector["base64"].as_str().expect("base64"))
            .expect("valid base64");
        // 1. decode the Java-produced bytes and compare semantically
        let decoded = EventEnvelope::from_bytes(&bytes)
            .unwrap_or_else(|e| panic!("{name}: decode failed - {e}"));
        assert_matches(name, &decoded, &vector["expect"]);
        // 2. re-encode with OUR encoder, decode again, compare again
        let re_encoded = decoded.to_bytes().unwrap();
        let round_tripped = EventEnvelope::from_bytes(&re_encoded)
            .unwrap_or_else(|e| panic!("{name}: round-trip decode failed - {e}"));
        assert_matches(name, &round_tripped, &vector["expect"]);
    }
    assert_eq!(standard, 5, "all five standard vectors exercised");
}

/// The spec's decoder rules beyond the vectors: absent body decodes as Nil
/// (Java omits an unset body), and our encoder omits unset optional fields
/// while always emitting `id` + `headers`.
#[test]
fn absent_body_decodes_as_nil_and_unset_fields_are_omitted() {
    let minimal = EventEnvelope::new();
    let bytes = minimal.to_bytes().unwrap();
    // the encoded form is one MsgPack map holding ONLY id + headers
    let raw: rmpv::Value = rmp_serde::from_slice(&bytes).unwrap();
    let rmpv::Value::Map(entries) = raw else {
        panic!("envelope must encode as a map");
    };
    let keys: Vec<&str> = entries.iter().filter_map(|(k, _)| k.as_str()).collect();
    assert_eq!(
        keys.len(),
        2,
        "only id + headers when nothing is set: {keys:?}"
    );
    assert!(
        keys.contains(&"id") && keys.contains(&"headers"),
        "{keys:?}"
    );
    // and the decoder reads the absent body back as Nil
    let decoded = EventEnvelope::from_bytes(&bytes).unwrap();
    assert!(matches!(decoded.body(), rmpv::Value::Nil));
}
