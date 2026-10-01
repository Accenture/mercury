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

//! Null-transport policy for the wire serializers — the Rust mirror of Java's
//! `serializer.null.transport` (Java `SimpleMapper`/Gson for JSON and `MsgPack`

//! Runs the shared conformance vectors of the canonical MsgPack packager (RFC-0002; the Rust twin of the Java
//! `CanonicalPackagerVectorsTest`). `tests/resources/canonical-package-vectors.json` is byte-identical to the Java
//! engine's file, and its expected bytes come from an independent encoder written from the specification, so an
//! engine that matches the file matches every other engine that does: Java and Rust agree byte for byte because
//! each agrees with the file. A JSON number without a fraction or exponent is an integer; any other number is a
//! float64.

use std::collections::HashSet;
use std::str::FromStr;

use bigdecimal::{BigDecimal, Zero};
use platform_core::canonical_packager::{self as packager, Builder, PackagerError};
use rmpv::Value;
use serde_json::Value as Json;
use sha2::{Digest, Sha256};

const VECTORS: &str = include_str!("resources/canonical-package-vectors.json");

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len() / 2)
        .map(|i| u8::from_str_radix(&text[2 * i..2 * i + 2], 16).expect("hex"))
        .collect()
}

fn sha256(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

/// The canonical string of an exact decimal, as the Java packager writes a `BigDecimal`: plain notation, the
/// scale kept, a zero of any scale as "0".
fn decimal_text(text: &str) -> String {
    let d = BigDecimal::from_str(text).expect("a decimal");
    if d.is_zero() {
        return "0".to_string();
    }
    let d = if d.fractional_digit_count() < 0 {
        d.with_scale(0)
    } else {
        d
    };
    d.to_plain_string()
}

/// The `Value` a vector's JSON denotes.
fn value(json: &Json) -> Value {
    match json {
        Json::Null => Value::Nil,
        Json::Bool(b) => Value::from(*b),
        Json::String(s) => Value::from(s.as_str()),
        Json::Array(items) => Value::Array(items.iter().map(value).collect()),
        Json::Number(n) => match n.as_i64() {
            Some(i) => Value::from(i),
            None => Value::F64(n.as_f64().expect("a finite number")),
        },
        Json::Object(map) => {
            if map.len() == 1 {
                let (tag, arg) = map.iter().next().expect("one entry");
                match tag.as_str() {
                    "$bytes" => return Value::Binary(unhex(arg.as_str().expect("hex"))),
                    "$decimal" => {
                        return Value::from(decimal_text(arg.as_str().expect("a decimal")))
                    }
                    // an arbitrary-size integer travels as its digits
                    "$integer" => return Value::from(arg.as_str().expect("digits")),
                    "$float32" => {
                        return Value::F32(match arg.as_str() {
                            Some("NaN") => f32::NAN,
                            Some("Infinity") => f32::INFINITY,
                            Some(other) => panic!("unknown float {other}"),
                            None => arg.as_f64().expect("a float") as f32,
                        })
                    }
                    "$double" => {
                        return Value::F64(match arg.as_str().expect("a name") {
                            "NaN" => f64::NAN,
                            "Infinity" => f64::INFINITY,
                            other => panic!("unknown double {other}"),
                        })
                    }
                    "$unsupported" => return Value::Ext(1, vec![]),
                    _ => {}
                }
            }
            Value::Map(
                map.iter()
                    .map(|(k, v)| (Value::from(k.as_str()), value(v)))
                    .collect(),
            )
        }
    }
}

fn vectors() -> Json {
    let doc: Json = serde_json::from_str(VECTORS).expect("the vector file parses");
    assert_eq!(doc["format"], "mercury-canonical-package-vectors");
    doc
}

fn builder(v: &Json) -> Result<Builder, PackagerError> {
    let mut builder = Builder::new();
    if let Some(manifest) = v.get("manifest").and_then(Json::as_object) {
        for (k, f) in manifest {
            builder = builder.manifest(k, f.as_str().expect("text"))?;
        }
    }
    for pair in v["maps"].as_array().expect("maps") {
        let entry = pair.as_array().expect("a [name, map] pair");
        builder = builder.add(entry[0].as_str().expect("name"), value(&entry[1]))?;
    }
    Ok(builder)
}

fn every_package_holds(packages: &[Json]) {
    let mut ids = HashSet::new();
    for v in packages {
        let id = v["id"].as_str().expect("id");
        assert!(ids.insert(id.to_string()), "duplicate vector id {id}");
        let bytes = builder(v)
            .and_then(Builder::build)
            .unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(v["hex"].as_str().unwrap(), hex(&bytes), "{id}");
        assert_eq!(v["sha256"].as_str().unwrap(), sha256(&bytes), "{id} sha256");
        // an accepted package has exactly one byte form: the strict read re-encodes and compares
        let unpacked = packager::unpack(&bytes).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert!(unpacked
            .manifest
            .iter()
            .any(|(k, v)| k == "format" && v == "mercury-package"));
        let again = packager::encode(&Value::Map(vec![
            (
                Value::from("manifest"),
                Value::Map(
                    unpacked
                        .manifest
                        .iter()
                        .map(|(k, v)| (Value::from(k.as_str()), Value::from(v.as_str())))
                        .collect(),
                ),
            ),
            (
                Value::from("maps"),
                Value::Map(
                    unpacked
                        .maps
                        .iter()
                        .map(|(k, v)| (Value::from(k.as_str()), v.clone()))
                        .collect(),
                ),
            ),
        ]))
        .expect("re-encodes");
        assert_eq!(bytes, again, "{id} re-encodes to itself");
    }
}

#[test]
fn every_value_encodes_to_the_expected_bytes() {
    let doc = vectors();
    let all = doc["values"].as_array().expect("values");
    assert!(
        all.len() > 70,
        "the vector file looks truncated: {}",
        all.len()
    );
    let mut ids = HashSet::new();
    for v in all {
        let id = v["id"].as_str().expect("id");
        assert!(ids.insert(id.to_string()), "duplicate vector id {id}");
        let bytes = packager::encode(&value(&v["value"])).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(v["hex"].as_str().unwrap(), hex(&bytes), "{id}");
    }
}

#[test]
fn every_package_holds_in_the_rust_packager() {
    let doc = vectors();
    let packages = doc["packages"].as_array().expect("packages");
    assert!(packages.len() >= 6);
    every_package_holds(packages);
}

#[test]
fn the_generated_corpus_matches_byte_for_byte() {
    let doc = vectors();
    let corpus = doc["corpus"].as_array().expect("corpus");
    assert_eq!(60, corpus.len(), "the corpus looks truncated");
    every_package_holds(corpus);
}

fn pack_fragment(code: &str) -> &'static str {
    match code {
        "non-finite" => "non-finite number",
        "unsupported-type" => "Unsupported type",
        "duplicate-entry" => "Duplicate entry name",
        "reserved-field" => "written by the packager",
        other => panic!("unknown error code {other}"),
    }
}

fn unpack_fragments(code: &str) -> &'static [&'static str] {
    match code {
        "not-canonical" => &["Not canonical"],
        "trailing-bytes" => &["Unexpected bytes after the value"],
        "not-a-package" => &["A package is one map", "is not a map", "is not text"],
        "unsupported-format" => &["Not a package: the manifest format"],
        "unsupported-version" => &["Unsupported package format_version"],
        "too-deep" => &["Nesting deeper"],
        "malformed" => &[
            "Malformed MsgPack",
            "Duplicate key",
            "A key is not text",
            "Unsupported MsgPack type",
        ],
        other => panic!("unknown error code {other}"),
    }
}

#[test]
fn every_pack_rejection_fails_by_name() {
    let doc = vectors();
    let mut checked = 0;
    for v in doc["rejections"].as_array().expect("rejections") {
        if v["kind"] != "pack" {
            continue;
        }
        let id = v["id"].as_str().expect("id");
        let mut v = v.clone();
        if v.get("maps").is_none() {
            v["maps"] = Json::Array(vec![]);
        }
        let error = builder(&v)
            .and_then(Builder::build)
            .expect_err(&format!("{id} must fail"));
        let fragment = pack_fragment(v["error"].as_str().expect("code"));
        assert!(
            error.message().contains(fragment),
            "{id}: '{}'",
            error.message()
        );
        assert!(matches!(error, PackagerError::Invalid(_)), "{id}");
        checked += 1;
    }
    assert!(checked >= 8, "pack rejections checked: {checked}");
}

#[test]
fn every_unpack_rejection_fails_by_name() {
    let doc = vectors();
    let mut checked = 0;
    for v in doc["rejections"].as_array().expect("rejections") {
        if v["kind"] != "unpack" {
            continue;
        }
        let id = v["id"].as_str().expect("id");
        let bytes = unhex(v["hex"].as_str().expect("hex"));
        let code = v["error"].as_str().expect("code");
        let error = packager::unpack(&bytes).expect_err(&format!("{id} must fail"));
        assert!(
            unpack_fragments(code)
                .iter()
                .any(|f| error.message().contains(f)),
            "{id}: '{}' does not match {code}",
            error.message()
        );
        if v["strict_only"] == true {
            // the bytes decode: only the strict read rejects a form that is not canonical
            assert!(
                packager::unpack_with(&bytes, false).is_ok(),
                "{id} decodes when not strict"
            );
        }
        checked += 1;
    }
    assert!(checked >= 17, "unpack rejections checked: {checked}");
}
