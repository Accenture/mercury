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

//! The shared hostile-header vectors (`tests/resources/msgpack-hostile-header-vectors.json`, byte-identical with the Java
//! engine's `system/platform-core/src/test/resources/` copy; Java twin `MsgPackHostileHeaderVectorsTest`): a MsgPack
//! header that promises more than the input holds - a str, bin or ext length or an array or map count beyond the
//! remaining bytes, the never-used byte 0xc1, nesting beyond the shared 64-level bound - is refused as a decoding error
//! by every decoder this engine exposes to bytes from outside the process: `serializer::from_msgpack` (the envelope and
//! the Event API's format check), `EventEnvelope::from_bytes` and `canonical_packager::decode`. Never a panic, a stack
//! overflow or an allocation sized by the header. The controls decode to exactly their JSON value, so a decoder that
//! refuses everything fails too.

use platform_core::canonical_packager;
use platform_core::serializer::from_msgpack;
use platform_core::EventEnvelope;
use rmpv::Value;
use serde_json::Value as Json;

const VECTORS: &str = include_str!("resources/msgpack-hostile-header-vectors.json");

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len() / 2)
        .map(|i| u8::from_str_radix(&text[2 * i..2 * i + 2], 16).expect("hex"))
        .collect()
}

fn vectors() -> Vec<Json> {
    let doc: Json = serde_json::from_str(VECTORS).expect("the vector file is JSON");
    assert_eq!(doc["format"], "mercury-msgpack-hostile-header-vectors");
    assert_eq!(doc["version"], "1");
    doc["vectors"].as_array().expect("vectors").clone()
}

/// The JSON form of a decoded value, for the controls: nil, booleans, integers, floats, text, arrays and maps with
/// text keys (a control never holds bytes or an extension value, which have no JSON form).
fn to_json(value: &Value) -> Json {
    match value {
        Value::Nil => Json::Null,
        Value::Boolean(b) => Json::Bool(*b),
        Value::Integer(n) => n
            .as_i64()
            .map(Json::from)
            .or_else(|| n.as_u64().map(Json::from))
            .expect("an integer"),
        Value::F32(f) => Json::from(f64::from(*f)),
        Value::F64(f) => Json::from(*f),
        Value::String(s) => Json::String(s.as_str().expect("utf-8 text").to_string()),
        Value::Array(items) => Json::Array(items.iter().map(to_json).collect()),
        Value::Map(entries) => Json::Object(
            entries
                .iter()
                .map(|(k, v)| (k.as_str().expect("a text key").to_string(), to_json(v)))
                .collect(),
        ),
        other => panic!("no JSON form for {other:?}"),
    }
}

#[test]
fn every_hostile_header_is_refused_at_the_header() {
    let mut checked = 0;
    for v in vectors() {
        if v["expect"] != "reject" {
            continue;
        }
        let id = v["id"].as_str().expect("id");
        let bytes = unhex(v["hex"].as_str().expect("hex"));
        // the envelope and Event API decoder: a decoding error, never a panic or an allocation sized by the header
        assert!(
            from_msgpack::<Value>(&bytes).is_err(),
            "{id}: serializer::from_msgpack accepted it"
        );
        assert!(
            EventEnvelope::from_bytes(&bytes).is_err(),
            "{id}: EventEnvelope::from_bytes accepted it"
        );
        // the canonical decoder
        assert!(
            canonical_packager::decode(&bytes).is_err(),
            "{id}: canonical_packager::decode accepted it"
        );
        checked += 1;
    }
    assert!(checked >= 20, "rejections checked: {checked}");
}

#[test]
fn every_control_decodes_to_its_value() {
    let mut checked = 0;
    for v in vectors() {
        if v["expect"] != "accept" {
            continue;
        }
        let id = v["id"].as_str().expect("id");
        let bytes = unhex(v["hex"].as_str().expect("hex"));
        let expected = &v["value"];
        let decoded: Value = from_msgpack(&bytes).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(
            &to_json(&decoded),
            expected,
            "{id} through serializer::from_msgpack"
        );
        let canonical = canonical_packager::decode(&bytes).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(
            &to_json(&canonical),
            expected,
            "{id} through canonical_packager::decode"
        );
        checked += 1;
    }
    assert!(checked >= 8, "controls checked: {checked}");
}
