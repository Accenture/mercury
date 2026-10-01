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

//! The packager's own rules (RFC-0002; the Rust twin of the Java `CanonicalPackagerTest`): the builder's
//! validation, key handling, the ordered read, the strict read and the bounds. The byte-level contract is pinned by
//! `canonical_packager_vectors.rs`.

use platform_core::canonical_packager::{self as packager, Builder, PackagerError};
use rmpv::Value;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn map(entries: Vec<(&str, Value)>) -> Value {
    Value::Map(
        entries
            .into_iter()
            .map(|(k, v)| (Value::from(k), v))
            .collect(),
    )
}

fn keys(value: &Value) -> Vec<String> {
    match value {
        Value::Map(entries) => entries
            .iter()
            .map(|(k, _)| k.as_str().expect("text key").to_string())
            .collect(),
        other => panic!("not a map: {other:?}"),
    }
}

#[test]
fn the_same_content_gives_the_same_bytes_whatever_the_insertion_order() {
    let forward = map(vec![
        ("b", Value::from(1)),
        ("a", map(vec![("y", Value::from(2)), ("x", Value::from(1))])),
    ]);
    let backward = map(vec![
        ("a", map(vec![("x", Value::from(1)), ("y", Value::from(2))])),
        ("b", Value::from(1i64)),
    ]);
    assert_eq!(
        packager::encode(&forward).unwrap(),
        packager::encode(&backward).unwrap()
    );
    let first = Builder::new()
        .add("b.json", forward.clone())
        .unwrap()
        .add("a.json", backward.clone())
        .unwrap()
        .build()
        .unwrap();
    let second = Builder::new()
        .add("a.json", backward)
        .unwrap()
        .add("b.json", forward)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(first, second);
}

#[test]
fn keys_sort_by_utf8_bytes_not_utf16() {
    // U+1F600 is the surrogate pair D83D DE00 in UTF-16 (sorts before U+FF5E) but F0 9F 98 80 in UTF-8
    let value = map(vec![
        ("\u{1F600}", Value::from(1)),
        ("\u{FF5E}", Value::from(2)),
    ]);
    let decoded = packager::decode(&packager::encode(&value).unwrap()).unwrap();
    assert_eq!(vec!["\u{FF5E}", "\u{1F600}"], keys(&decoded));
}

#[test]
fn a_non_text_key_becomes_text_and_a_collision_or_a_null_key_is_an_error() {
    let numbered = Value::Map(vec![
        (Value::from(10), Value::from("ten")),
        (Value::from(2), Value::from("two")),
    ]);
    let decoded = packager::decode(&packager::encode(&numbered).unwrap()).unwrap();
    assert_eq!(vec!["10", "2"], keys(&decoded), "text order, not numeric");
    let collision = Value::Map(vec![
        (Value::from(1), Value::from("a")),
        (Value::from("1"), Value::from("b")),
    ]);
    let e = packager::encode(&collision).unwrap_err();
    assert!(e.message().contains("Two keys become '1'"), "{e}");
    let null_key = Value::Map(vec![(Value::Nil, Value::from("x"))]);
    assert!(packager::encode(&null_key)
        .unwrap_err()
        .message()
        .contains("A null key"));
    let float_key = Value::Map(vec![(Value::F64(1.5), Value::from("x"))]);
    assert!(packager::encode(&float_key).is_err());
}

#[test]
fn a_null_value_is_kept_as_nil() {
    let value = map(vec![("a", Value::Nil)]);
    assert_eq!("81a161c0", hex(&packager::encode(&value).unwrap()));
}

#[test]
fn small_integers_use_the_smallest_encoding_and_only_signed_64_bit_is_canonical() {
    assert_eq!("05", hex(&packager::encode(&Value::from(5)).unwrap()));
    assert_eq!("cc80", hex(&packager::encode(&Value::from(128)).unwrap()));
    assert_eq!(
        "cf7fffffffffffffff",
        hex(&packager::encode(&Value::from(i64::MAX)).unwrap())
    );
    // above 2^63-1 the Java engine writes the digits as text, so the caller must too
    let e = packager::encode(&Value::from(u64::MAX)).unwrap_err();
    assert!(e.message().contains("write it as text"), "{e}");
}

#[test]
fn floats_are_float64_and_finite_only() {
    assert_eq!(
        "cb3ff8000000000000",
        hex(&packager::encode(&Value::F64(1.5)).unwrap())
    );
    assert!(packager::encode(&Value::F64(f64::NAN)).is_err());
    assert!(packager::encode(&Value::F64(f64::NEG_INFINITY)).is_err());
    // the integer 1 and the float 1.0 are different content
    assert_ne!(
        hex(&packager::encode(&Value::from(1)).unwrap()),
        hex(&packager::encode(&Value::F64(1.0)).unwrap())
    );
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len() / 2)
        .map(|i| u8::from_str_radix(&text[2 * i..2 * i + 2], 16).unwrap())
        .collect()
}

#[test]
fn an_f32_is_widened_through_its_shortest_decimal_text() {
    // 0.1f32 is the float64 0.1, not the exact value of the f32 (0.10000000149011612)
    assert_eq!(
        hex(&packager::encode(&Value::F64(0.1)).unwrap()),
        hex(&packager::encode(&Value::F32(0.1)).unwrap())
    );
    assert_eq!(
        "cb3fb999999999999a",
        hex(&packager::encode(&Value::F32(0.1)).unwrap())
    );
    assert_eq!(
        "cb3ff8000000000000",
        hex(&packager::encode(&Value::F32(1.5)).unwrap())
    );
    assert_eq!(
        hex(&packager::encode(&Value::F64(1.1)).unwrap()),
        hex(&packager::encode(&Value::F32(1.1)).unwrap())
    );
    // inside a map, and the non-finite forms are still refused
    assert_eq!(
        hex(&packager::encode(&map(vec![("x", Value::F64(2.5))])).unwrap()),
        hex(&packager::encode(&map(vec![("x", Value::F32(2.5))])).unwrap())
    );
    let nan = packager::encode(&Value::F32(f32::NAN)).unwrap_err();
    assert!(nan.message().contains("non-finite"), "{nan}");
    assert!(packager::encode(&Value::F32(f32::INFINITY)).is_err());
    // an f32 on the wire is valid MsgPack but not the canonical form of its content: the strict read refuses it
    let on_the_wire = unhex(
        "82a86d616e696665737482a6666f726d6174af6d6572637572792d7061636b616765\
         ae666f726d61745f76657273696f6ea131a46d61707381a66e2e6a736f6e81a178ca3fc00000",
    );
    assert!(packager::unpack(&on_the_wire).is_err());
    let lenient = packager::unpack_with(&on_the_wire, false).unwrap();
    assert_eq!(Some(1.5), lenient.maps[0].1.as_map().unwrap()[0].1.as_f64());
}

#[test]
fn an_unsupported_type_is_rejected_naming_its_path() {
    let value = map(vec![(
        "nodes",
        Value::Array(vec![map(vec![("when", Value::Ext(1, vec![0]))])]),
    )]);
    let e = packager::encode(&value).unwrap_err();
    assert!(e.message().contains("Unsupported type ext"), "{e}");
    assert!(e.message().contains("$.nodes[0].when"), "{e}");
}

#[test]
fn the_builder_refuses_reserved_duplicate_and_empty_names() {
    assert!(Builder::new().manifest("format", "x").is_err());
    assert!(Builder::new().manifest("format_version", "9").is_err());
    assert!(Builder::new().manifest("", "x").is_err());
    let b = Builder::new().manifest("graph_id", "quote").unwrap();
    assert!(b.manifest("graph_id", "again").is_err());
    let b = Builder::new().add("a.json", map(vec![])).unwrap();
    assert!(b.add("a.json", map(vec![])).is_err());
    assert!(Builder::new().add("", map(vec![])).is_err());
    assert!(Builder::new().add("n.json", Value::from(1)).is_err());
}

#[test]
fn unpack_returns_ordered_maps_with_the_manifest_first() {
    let bytes = Builder::new()
        .manifest("graph_id", "quote")
        .unwrap()
        .manifest("version", "1.0.0")
        .unwrap()
        .add(
            "quote.json",
            map(vec![("z", Value::from(1)), ("a", Value::from(2))]),
        )
        .unwrap()
        .add("quote-fees.json", map(vec![("m", Value::from(1))]))
        .unwrap()
        .build()
        .unwrap();
    let unpacked = packager::unpack(&bytes).unwrap();
    let manifest_keys: Vec<&str> = unpacked.manifest.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(
        vec!["format", "format_version", "graph_id", "version"],
        manifest_keys
    );
    let names: Vec<&str> = unpacked.maps.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(vec!["quote-fees.json", "quote.json"], names);
    assert_eq!(vec!["a", "z"], keys(&unpacked.maps[1].1));
    // manifest sorts before maps, so a reader can read the metadata before decoding any map
    assert_eq!(
        vec!["manifest", "maps"],
        keys(&packager::decode(&bytes).unwrap())
    );
}

#[test]
fn a_strict_read_rejects_what_is_not_canonical_but_a_non_strict_read_decodes_it() {
    // the same content with a wider integer than needed: valid MsgPack, not the canonical bytes
    let canonical = Builder::new()
        .add("g.json", map(vec![("n", Value::from(1))]))
        .unwrap()
        .build()
        .unwrap();
    let wide = hex(&canonical).replace("a16e01", "a16ecd0001");
    assert_ne!(hex(&canonical), wide);
    let bytes: Vec<u8> = (0..wide.len() / 2)
        .map(|i| u8::from_str_radix(&wide[2 * i..2 * i + 2], 16).unwrap())
        .collect();
    let e = packager::unpack(&bytes).unwrap_err();
    assert!(e.message().starts_with("Not canonical"), "{e}");
    assert!(matches!(e, PackagerError::Malformed(_)));
    let lenient = packager::unpack_with(&bytes, false).unwrap();
    assert_eq!(Some(1), lenient.maps[0].1.as_map().unwrap()[0].1.as_i64());
}

#[test]
fn a_round_trip_keeps_types_and_order() {
    let content = map(vec![
        ("n", Value::from(7)),
        ("f", Value::F64(2.5)),
        ("s", Value::from("text")),
        ("b", Value::Binary(vec![1, 2, 3])),
        ("flag", Value::from(true)),
        ("none", Value::Nil),
        (
            "list",
            Value::Array(vec![Value::from(1), map(vec![("k", Value::from("v"))])]),
        ),
    ]);
    let bytes = Builder::new()
        .add("c.json", content)
        .unwrap()
        .build()
        .unwrap();
    let back = &packager::unpack(&bytes).unwrap().maps[0].1;
    assert_eq!(vec!["b", "f", "flag", "list", "n", "none", "s"], keys(back));
    let entries = back.as_map().unwrap();
    let get = |k: &str| {
        &entries
            .iter()
            .find(|(key, _)| key.as_str() == Some(k))
            .unwrap()
            .1
    };
    assert_eq!(Some(7), get("n").as_i64());
    assert_eq!(Some(2.5), get("f").as_f64());
    assert_eq!(Some("text"), get("s").as_str());
    assert_eq!(&Value::Binary(vec![1, 2, 3]), get("b"));
    assert_eq!(Some(true), get("flag").as_bool());
    assert!(get("none").is_nil());
}

#[test]
fn nesting_beyond_the_bound_is_refused_on_both_sides() {
    let mut deep = Value::from("x");
    for _ in 0..70 {
        deep = Value::Array(vec![deep]);
    }
    assert!(packager::encode(&deep).is_err());
    let mut bytes = vec![0x91u8; 70];
    bytes.push(0xc0);
    let e = packager::decode(&bytes).unwrap_err();
    assert!(e.message().contains("Nesting deeper"), "{e}");
}

#[test]
fn the_example_on_the_canonical_package_format_page_is_the_canonical_package() {
    // docs/guides/canonical-package-format.md: no caller fields, one map only.json holding {"a": 1}
    let bytes = Builder::new()
        .add("only.json", map(vec![("a", Value::from(1))]))
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        "82a86d616e696665737482a6666f726d6174af6d6572637572792d7061636b616765ae666f726d61745f76657273696f6ea131a46d61707381a96f6e6c792e6a736f6e81a16101",
        hex(&bytes)
    );
    use sha2::{Digest, Sha256};
    assert_eq!(
        "56ebc3ba08b5181e1afa0fdcbb499f2e82c3958490d9da31d295c0553c926de7",
        hex(&Sha256::digest(&bytes))
    );
}
