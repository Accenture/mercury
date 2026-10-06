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
//! for the event bus, both reading the same config).
//!
//! ## Rationale
//!
//! A PoJo rarely has every field initialized, so serializing it emits a crowd of
//! `null` fields that are pure noise; dropping them makes the JSON/MsgPack output
//! much cleaner. That is why omission is the **default**. The opposite need is
//! real too: some applications must distinguish "key present with a null value"
//! from "key absent" — for them, `serializer.null.transport=true` keeps the nulls
//! on the wire.
//!
//! ## Behavior (the invariants — must match Java exactly)
//!
//! - Config `false` (**default**): `Nil` **map** key-values are dropped — like
//!   Gson's default field omission and Java `MsgPack.packMap`'s
//!   `if (supportNulls || value != null)`. Config `true`: nulls are transported.
//! - **Map key-values only.** The parameter affects nothing else.
//! - **Array elements are always kept — including `Nil`.** Dropping a null array
//!   element would shift the following elements and break array ordering, so
//!   arrays are never filtered (Gson and Java `packList` keep null slots too).
//! - **An empty collection is not a null.** A map value of `[]` or `{}` is a
//!   real, present value and is kept regardless of the config; only `Nil` is
//!   dropped.
//!
//! Apply [`strip_nulls`] at every wire boundary that emits a payload: JSON HTTP
//! responses, WebSocket text frames, outbound HTTP request bodies, and the
//! MsgPack envelope encoder.

use std::sync::OnceLock;

use rmpv::Value;

use crate::util::app_config_reader::AppConfigReader;

/// `serializer.null.transport` (default `false`), read once and cached — mirrors
/// Java reading it at `SimpleMapper` / `MsgPack` construction (a process-wide
/// singleton decision, not a per-call toggle).
pub fn null_transport() -> bool {
    static CACHE: OnceLock<bool> = OnceLock::new();
    *CACHE.get_or_init(|| {
        AppConfigReader::get_instance()
            .get_property_or("serializer.null.transport", "false")
            .eq_ignore_ascii_case("true")
    })
}

/// Drop `Nil` map values recursively unless null transport is enabled — or unless
/// there is nothing to drop. Returns a plain clone when
/// `serializer.null.transport=true` **or** [`has_nil_map_entry`] is false, so the
/// recursive rebuild only runs when a strippable `Nil` is actually present.
pub fn strip_nulls(value: &Value) -> Value {
    if null_transport() || !has_nil_map_entry(value) {
        value.clone()
    } else {
        strip_nulls_always(value)
    }
}

/// Read-only, allocation-free predicate: does `value` hold a `Nil` at any **map**
/// key-value position (recursively, through nested maps and arrays)? This is
/// exactly "would [`strip_nulls_always`] change anything" — a `Nil` *array*
/// element is preserved, so it does **not** count. Callers use it to skip the
/// clone/strip entirely when there is nothing to remove (the common case).
pub fn has_nil_map_entry(value: &Value) -> bool {
    match value {
        Value::Map(entries) => entries
            .iter()
            .any(|(_, v)| matches!(v, Value::Nil) || has_nil_map_entry(v)),
        Value::Array(items) => items.iter().any(has_nil_map_entry),
        _ => false,
    }
}

/// The unconditional strip — recursively removes `Nil` entries from maps and
/// preserves array elements. Callers that have already checked
/// [`null_transport`] (e.g. the MsgPack hot path) use this directly.
pub fn strip_nulls_always(value: &Value) -> Value {
    match value {
        Value::Map(entries) => Value::Map(
            entries
                .iter()
                .filter(|(_, v)| !matches!(v, Value::Nil))
                .map(|(k, v)| (k.clone(), strip_nulls_always(v)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.iter().map(strip_nulls_always).collect()),
        other => other.clone(),
    }
}

/// The deepest nesting of maps and lists a MsgPack payload may carry, the outermost container being level 1
/// (Java `MsgPack.MAX_DEPTH`). The decoder recurses once per level, and in Rust a stack overflow aborts the
/// whole process, so a payload nested deeper is refused as a decoding error. rmp-serde's own default (1,024) is
/// no guard: a debug build's 2 MiB thread stack overflows at about 550 levels.
pub const MAX_DEPTH: usize = 64;

/// The bytes of an empty map, which empty input decodes as.
const EMPTY_MAP: &[u8] = &[0x80];

/// Decode MsgPack bytes with their nesting bounded at [`MAX_DEPTH`] and exactly one value read: `rmp_serde::from_slice`
/// with a depth limit, then the proof that nothing follows the value.
/// Bytes that arrive from outside the process are decoded here (the envelope, the Event API's format check).
///
/// Empty input decodes as an empty map - so an empty byte array is an empty envelope, never a decoding error - as the
/// Java engine's `MsgPack.unpack` reads it; the Java engine is the reference implementation (Eric, 2026-10-06). The
/// canonical package decoder keeps refusing empty input, as the Java one does: a package is never empty.
///
/// The input holds exactly one value: bytes after the top-level container, well-formed or not, are refused as
/// `Unexpected bytes after the value at offset N` - the Java engine's rule (`MsgPack.unpackMapOrList`) and the one the
/// canonical package decoder always applied - so two different byte strings never decode to the same value.
pub fn from_msgpack<'a, T: serde::Deserialize<'a>>(bytes: &'a [u8]) -> Result<T, String> {
    let bytes = if bytes.is_empty() { EMPTY_MAP } else { bytes };
    let mut de = rmp_serde::Deserializer::from_read_ref(bytes);
    // rmp-serde refuses the container that brings its counter to zero, so MAX_DEPTH + 1 accepts MAX_DEPTH levels
    de.set_max_depth(MAX_DEPTH + 1);
    let value = T::deserialize(&mut de).map_err(|e| match e {
        rmp_serde::decode::Error::DepthLimitExceeded => {
            format!("Nesting deeper than {MAX_DEPTH} levels")
        }
        other => other.to_string(),
    })?;
    // The zero-copy reader keeps its cursor to itself, so the end of the input is proven by reading on: the marker of
    // a second value must be missing. A probe for a number reads that marker and at most eight data bytes, and only a
    // missing marker fails as InvalidMarkerRead with UnexpectedEof - a marker that is present fails later, or not at
    // all - so any other outcome means bytes follow the value. The offset is computed for the error only.
    match <u8 as serde::Deserialize>::deserialize(&mut de) {
        Err(rmp_serde::decode::Error::InvalidMarkerRead(ref e))
            if e.kind() == std::io::ErrorKind::UnexpectedEof =>
        {
            Ok(value)
        }
        _ => Err(format!(
            "Unexpected bytes after the value at offset {}",
            value_end(bytes)
        )),
    }
}

/// The offset at which the first value in `bytes` ends, read over a cursor so the position is known. Used only to
/// name the offset in the error for bytes after the value, once the first value has decoded, so every length it
/// declares lies within the input.
fn value_end(bytes: &[u8]) -> usize {
    let mut de = rmp_serde::Deserializer::new(std::io::Cursor::new(bytes));
    de.set_max_depth(MAX_DEPTH + 1);
    let _ = <serde::de::IgnoredAny as serde::Deserialize>::deserialize(&mut de);
    de.position() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmpv::Value;

    fn s(text: &str) -> Value {
        Value::from(text)
    }

    #[test]
    fn drops_nil_map_entries() {
        let input = Value::Map(vec![
            (s("ok"), Value::Boolean(true)),
            (s("error"), Value::Nil),
            (s("name"), s("Peter")),
        ]);
        let out = strip_nulls_always(&input);
        let Value::Map(entries) = out else {
            panic!("expected map");
        };
        let keys: Vec<&str> = entries.iter().filter_map(|(k, _)| k.as_str()).collect();
        assert_eq!(keys, vec!["ok", "name"], "the Nil `error` entry is dropped");
    }

    #[test]
    fn recurses_into_nested_maps() {
        let input = Value::Map(vec![(
            s("output"),
            Value::Map(vec![(s("body"), s("hello")), (s("meta"), Value::Nil)]),
        )]);
        let Value::Map(entries) = strip_nulls_always(&input) else {
            panic!("map");
        };
        let Value::Map(inner) = &entries[0].1 else {
            panic!("nested map");
        };
        let keys: Vec<&str> = inner.iter().filter_map(|(k, _)| k.as_str()).collect();
        assert_eq!(keys, vec!["body"], "nested Nil dropped, non-null kept");
    }

    #[test]
    fn preserves_array_elements_including_null() {
        // Gson / packList keep null slots; only object fields are omitted.
        let input = Value::Array(vec![s("a"), Value::Nil, s("b")]);
        let Value::Array(items) = strip_nulls_always(&input) else {
            panic!("array");
        };
        assert_eq!(
            items.len(),
            3,
            "array length (with the null slot) is preserved"
        );
        assert!(matches!(items[1], Value::Nil));
    }

    #[test]
    fn strips_maps_inside_arrays() {
        let input = Value::Array(vec![Value::Map(vec![
            (s("keep"), s("x")),
            (s("drop"), Value::Nil),
        ])]);
        let Value::Array(items) = strip_nulls_always(&input) else {
            panic!("array");
        };
        let Value::Map(inner) = &items[0] else {
            panic!("map in array");
        };
        assert_eq!(
            inner.len(),
            1,
            "Nil entry in an array's map element is dropped"
        );
    }

    #[test]
    fn scalars_pass_through() {
        assert_eq!(strip_nulls_always(&s("x")), s("x"));
        assert_eq!(strip_nulls_always(&Value::Nil), Value::Nil);
    }

    #[test]
    fn empty_collections_are_not_null() {
        // An empty array or map value is a present value, not a null — kept.
        let input = Value::Map(vec![
            (s("empty_list"), Value::Array(vec![])),
            (s("empty_map"), Value::Map(vec![])),
            (s("gone"), Value::Nil),
        ]);
        let Value::Map(entries) = strip_nulls_always(&input) else {
            panic!("map");
        };
        let keys: Vec<&str> = entries.iter().filter_map(|(k, _)| k.as_str()).collect();
        assert_eq!(
            keys,
            vec!["empty_list", "empty_map"],
            "empty [] and {{}} are kept; only the Nil entry is dropped"
        );
    }

    #[test]
    fn has_nil_map_entry_matches_what_strip_removes() {
        // strippable: a Nil map value, at any depth reachable via maps/arrays
        assert!(has_nil_map_entry(&Value::Map(vec![(s("a"), Value::Nil)])));
        assert!(has_nil_map_entry(&Value::Map(vec![(
            s("a"),
            Value::Map(vec![(s("b"), Value::Nil)]),
        )])));
        assert!(has_nil_map_entry(&Value::Array(vec![Value::Map(vec![(
            s("k"),
            Value::Nil,
        )])])));
        // NOT strippable: no Nil map value present
        assert!(!has_nil_map_entry(&Value::Map(vec![(s("a"), s("x"))])));
        // NOT strippable: a Nil *array element* is preserved, so it doesn't count
        assert!(!has_nil_map_entry(&Value::Array(vec![Value::Nil, s("x")])));
        // NOT strippable: empty collections / scalars
        assert!(!has_nil_map_entry(&Value::Array(vec![])));
        assert!(!has_nil_map_entry(&Value::Map(vec![])));
        assert!(!has_nil_map_entry(&Value::Nil));
        assert!(!has_nil_map_entry(&s("x")));
    }

    #[test]
    fn array_ordering_preserved_with_interior_null() {
        // Dropping a null element would shift the rest and corrupt ordering.
        let input = Value::Array(vec![
            Value::from(0),
            Value::Nil,
            Value::from(2),
            Value::Nil,
            Value::from(4),
        ]);
        let Value::Array(items) = strip_nulls_always(&input) else {
            panic!("array");
        };
        assert_eq!(items.len(), 5, "no element dropped");
        assert_eq!(items[0], Value::from(0));
        assert!(matches!(items[1], Value::Nil));
        assert_eq!(items[2], Value::from(2));
        assert!(matches!(items[3], Value::Nil));
        assert_eq!(items[4], Value::from(4));
    }

    // each 0x91 opens an array of one element; the innermost element is nil (0xc0)
    fn nested_arrays(depth: usize) -> Vec<u8> {
        let mut bytes = vec![0x91u8; depth];
        bytes.push(0xc0);
        bytes
    }

    // each level is a map of one entry, the key "a" (0x81 0xa1 0x61); the innermost value is nil
    fn nested_maps(depth: usize) -> Vec<u8> {
        let mut bytes = [0x81u8, 0xa1, 0x61].repeat(depth);
        bytes.push(0xc0);
        bytes
    }

    // maps and arrays alternate, starting with a map
    fn nested_mix(depth: usize) -> Vec<u8> {
        let mut bytes = Vec::new();
        for i in 0..depth {
            bytes.extend_from_slice(if i % 2 == 0 {
                &[0x81, 0xa1, 0x61]
            } else {
                &[0x91]
            });
        }
        bytes.push(0xc0);
        bytes
    }

    #[test]
    fn nesting_up_to_the_limit_decodes() {
        // 64 nested arrays, the outermost being level 1, decode (Java `MsgPackTest`, same payloads)
        let mut value: Value = from_msgpack(&nested_arrays(MAX_DEPTH)).expect("64 levels decode");
        let mut depth = 0;
        while let Value::Array(items) = value {
            depth += 1;
            value = items.into_iter().next().unwrap_or(Value::Nil);
        }
        assert_eq!(MAX_DEPTH, depth);
        assert!(from_msgpack::<Value>(&nested_maps(MAX_DEPTH)).is_ok());
        assert!(from_msgpack::<Value>(&nested_mix(MAX_DEPTH)).is_ok());
    }

    #[test]
    fn nesting_beyond_the_limit_is_refused() {
        // one level more is refused by name; maps count like arrays, and so does a mix of both
        assert_eq!(
            Err("Nesting deeper than 64 levels".to_string()),
            from_msgpack::<Value>(&nested_arrays(MAX_DEPTH + 1))
        );
        assert!(from_msgpack::<Value>(&nested_maps(MAX_DEPTH + 1)).is_err());
        assert!(from_msgpack::<Value>(&nested_mix(MAX_DEPTH + 1)).is_err());
    }

    #[test]
    fn a_deeply_nested_payload_is_refused_before_the_stack_runs_out() {
        // 100,000 nested arrays, about 100 KB: rmp-serde's default stops at 1,024, but a debug build's thread
        // stack overflowed first, at about 550 levels, and a stack overflow aborts the process
        assert!(from_msgpack::<Value>(&nested_arrays(100_000)).is_err());
    }

    #[test]
    fn empty_input_is_an_empty_map_as_in_java() {
        // the Java engine's MsgPack.unpack returns an empty map for empty input; this engine follows the reference
        assert_eq!(Ok(Value::Map(vec![])), from_msgpack::<Value>(&[]));
    }

    #[test]
    fn bytes_after_the_value_are_refused() {
        // the input holds exactly one value, as on Java (MsgPack.unpackMapOrList): whatever follows the top-level
        // container is refused by name, well-formed or not; the truncated shapes prove the probe tells a present
        // marker from a missing one, since their own end of input comes after the marker
        let trailing: [&[u8]; 6] = [
            &[0x80, 0xc1],                   // the format byte the specification never uses
            &[0x80, 0xc0],                   // a well-formed nil
            &[0x80, 0x05],                   // a positive fixint, which the probe reads whole
            &[0x80, 0x91],                   // an array header whose element is missing
            &[0x80, 0xcc],                   // a uint 8 header whose byte is missing
            &[0x80, 0xa3, 0x61, 0x62, 0x63], // a complete string
        ];
        for bytes in trailing {
            assert_eq!(
                Err("Unexpected bytes after the value at offset 1".to_string()),
                from_msgpack::<Value>(bytes),
                "{bytes:02x?}"
            );
        }
        // the offset names where the value ended
        assert_eq!(
            Err("Unexpected bytes after the value at offset 4".to_string()),
            from_msgpack::<Value>(&[0x81, 0xa1, 0x61, 0x01, 0xc0])
        );
        // the value alone decodes: the rule refuses what follows the value, not the value
        assert_eq!(Ok(Value::Map(vec![])), from_msgpack::<Value>(&[0x80]));
    }
}
