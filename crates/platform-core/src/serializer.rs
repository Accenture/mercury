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
//! When the config is `false` (the **default**, matching Gson's default field
//! omission), `Nil` **map** values are dropped from JSON and MsgPack output —
//! exactly like Gson omitting null object fields and Java `MsgPack.packMap`'s
//! `if (supportNulls || value != null)`. When `true`, nulls are transported.
//! **Array** elements are always preserved (Gson and Java `packList` keep null
//! slots), so only map entries are affected.
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

/// Drop `Nil` map values recursively unless null transport is enabled. A no-op
/// clone when `serializer.null.transport=true`.
pub fn strip_nulls(value: &Value) -> Value {
    if null_transport() {
        value.clone()
    } else {
        strip_nulls_always(value)
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
}
