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
}
