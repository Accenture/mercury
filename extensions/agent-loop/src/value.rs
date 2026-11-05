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

//! Rust-only helpers with no Java class of their own: what the Java agent loop gets for free from
//! `Map<String, Object>` and `String`, written once over `rmpv::Value`.
//!
//! - **Map access.** A Java `map.get(key)` answers null both for a missing key and for a key that
//!   holds null, so [`get`] treats a `Nil` value as absent; `map.containsKey(key)` is
//!   [`contains_key`], and the value a Java `put(key, map.get(key))` copies is [`get_raw`]. A
//!   MessagePack map may repeat a key, which a Java map built by successive puts resolves to the
//!   last value, so a lookup takes the last match.
//! - **Java string rules.** `String.trim()` removes every character up to U+0020 ([`java_trim`]),
//!   `String.isBlank()` uses `Character.isWhitespace` ([`java_is_blank`]), and a `TreeSet<String>`
//!   orders by UTF-16 code units ([`java_cmp`]) - each differs from the Rust default at the edges, and
//!   the shared vectors pin messages that name the first problem in key order.

use std::cmp::Ordering;

use rmpv::Value;

/// The entries of a map, or none for any other value.
pub(crate) fn entries(map: &Value) -> &[(Value, Value)] {
    match map {
        Value::Map(entries) => entries,
        _ => &[],
    }
}

/// Java `map.get(key) != null`: the value at `key`, `None` when the key is missing or holds nil.
pub(crate) fn get<'a>(map: &'a Value, key: &str) -> Option<&'a Value> {
    get_raw(map, key).filter(|value| !value.is_nil())
}

/// The value at `key` as stored, nil included; `None` only when the key is missing.
pub(crate) fn get_raw<'a>(map: &'a Value, key: &str) -> Option<&'a Value> {
    entries(map)
        .iter()
        .rev()
        .find(|(name, _)| name.as_str() == Some(key))
        .map(|(_, value)| value)
}

/// Java `map.containsKey(key)`.
pub(crate) fn contains_key(map: &Value, key: &str) -> bool {
    get_raw(map, key).is_some()
}

/// Java `map.get(key)` as a value to copy: nil when the key is missing.
pub(crate) fn copy(map: &Value, key: &str) -> Value {
    get_raw(map, key).cloned().unwrap_or(Value::Nil)
}

/// The text of a string value; `None` for any other value.
pub(crate) fn text(value: Option<&Value>) -> Option<&str> {
    value.and_then(Value::as_str)
}

/// A map key as text: a string key as it is, any other key as its compact JSON.
pub(crate) fn key_text(key: &Value) -> String {
    match key.as_str() {
        Some(text) => text.to_string(),
        None => crate::json::compact(key),
    }
}

/// The distinct keys of a map in Java `TreeSet<String>` order.
pub(crate) fn sorted_keys(map: &Value) -> Vec<String> {
    let mut keys: Vec<String> = entries(map).iter().map(|(k, _)| key_text(k)).collect();
    keys.sort_by(|a, b| java_cmp(a, b));
    keys.dedup();
    keys
}

/// Java `String.valueOf(object)` for the values a message quotes: null, text, a boolean or an integer
/// as Java prints them; anything else as its compact JSON.
pub(crate) fn value_of(value: Option<&Value>) -> String {
    match value {
        None | Some(Value::Nil) => "null".to_string(),
        Some(Value::String(s)) => s.as_str().unwrap_or_default().to_string(),
        Some(Value::Boolean(b)) => b.to_string(),
        Some(Value::Integer(n)) => n.to_string(),
        Some(other) => crate::json::compact(other),
    }
}

/// "a, b and c", as the Java messages join a list.
pub(crate) fn and_list(items: &[&str]) -> String {
    match items.split_last() {
        Some((last, rest)) if !rest.is_empty() => format!("{} and {last}", rest.join(", ")),
        Some((last, _)) => (*last).to_string(),
        None => String::new(),
    }
}

/// Build an ordered map from its entries.
pub(crate) fn map_of(pairs: Vec<(&str, Value)>) -> Value {
    Value::Map(
        pairs
            .into_iter()
            .map(|(key, value)| (Value::from(key), value))
            .collect(),
    )
}

/// Java `LinkedHashMap.put`: replace the value in place, or append the entry. Of a repeated key it
/// replaces the last entry, the one [`get`] reads.
pub(crate) fn put(map: &mut Vec<(Value, Value)>, key: &str, value: Value) {
    match map
        .iter_mut()
        .rev()
        .find(|(name, _)| name.as_str() == Some(key))
    {
        Some(entry) => entry.1 = value,
        None => map.push((Value::from(key), value)),
    }
}

/// Java `String.trim()`: every leading and trailing character up to U+0020.
pub(crate) fn java_trim(text: &str) -> &str {
    text.trim_matches(|c: char| c <= ' ')
}

/// Java `Character.isWhitespace`: the Unicode space, line and paragraph separators except the
/// non-breaking ones, the ASCII controls tab to carriage return, and the four separators U+001C to
/// U+001F. Rust's `char::is_whitespace` adds NEL and the non-breaking spaces, and leaves out the four.
fn java_whitespace(c: char) -> bool {
    matches!(c, '\u{1c}'..='\u{1f}')
        || (c.is_whitespace() && !matches!(c, '\u{85}' | '\u{a0}' | '\u{2007}' | '\u{202f}'))
}

/// Java `String.isBlank()`.
pub(crate) fn java_is_blank(text: &str) -> bool {
    text.chars().all(java_whitespace)
}

/// Java `String.compareTo`: UTF-16 code unit order, as a `TreeSet<String>` keeps it.
pub(crate) fn java_cmp(a: &str, b: &str) -> Ordering {
    a.encode_utf16().cmp(b.encode_utf16())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn java_string_rules() {
        assert_eq!(java_trim("\u{1} a \t"), "a");
        assert_eq!(java_trim("\u{a0}a"), "\u{a0}a");
        assert!(java_is_blank(" \t\u{1c}\u{2028}"));
        assert!(!java_is_blank("\u{a0}"));
        assert!(!java_is_blank("\u{85}"));
        assert_eq!(java_cmp("\u{ffff}", "\u{1f600}"), Ordering::Greater);
        assert_eq!("\u{ffff}".cmp("\u{1f600}"), Ordering::Less);
    }

    #[test]
    fn map_access_treats_nil_as_absent() {
        let map = map_of(vec![
            ("a", Value::Nil),
            ("b", Value::from(1)),
            ("b", Value::from(2)),
        ]);
        assert!(get(&map, "a").is_none());
        assert!(contains_key(&map, "a"));
        assert_eq!(get(&map, "b"), Some(&Value::from(2)));
        assert_eq!(copy(&map, "z"), Value::Nil);
    }

    #[test]
    fn put_replaces_the_entry_get_reads() {
        let Value::Map(mut entries) = map_of(vec![("b", Value::from(1)), ("b", Value::from(2))])
        else {
            unreachable!()
        };
        put(&mut entries, "b", Value::from(3));
        assert_eq!(get(&Value::Map(entries), "b"), Some(&Value::from(3)));
    }
}
