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

//! The flow-runtime `MultiLevelMap` — composite dot-bracket access over the
//! state-machine tree (`rmpv::Value`, the bus currency).
//!
//! **Access strategy (maintainer decision, 2026-07-16):** direct composite-key
//! traversal (`a.b[0].c`) is the PRIMARY data-mapping tool — lightweight, no
//! conversion, no query engine. A `$`-prefixed path is a **user-defined
//! complex query** delegated to a real JSONPath engine (`serde_json_path`,
//! RFC 9535) over an on-demand JSON view of the tree — exactly how the Java
//! `MultiLevelMap` delegates `$` paths to the Jayway JsonPath library.
//!
//! Path semantics mirror the platform-core (config) `MultiLevelMap` and the
//! Java original: `get` of a missing/invalid path or explicit null is `None`;
//! `set` creates intermediate maps and pads lists with nulls; `key[]` appends
//! (the empty index normalizes to the list's current length); `remove` drops
//! map keys and nulls list slots (indices never shift).

use rmpv::Value;

use crate::conversions::{from_json, to_json};

enum Segment {
    Key(String),
    Index(usize),
}

fn parse_path(path: &str) -> Result<Vec<Segment>, String> {
    if path.trim().is_empty() {
        return Err("composite path must not be empty".to_string());
    }
    let mut segments = Vec::new();
    for part in path.split('.') {
        if part.is_empty() {
            return Err(format!("invalid composite path '{path}': empty segment"));
        }
        let bracket = part.find('[');
        let (name, rest) = match bracket {
            Some(i) => (&part[..i], &part[i..]),
            None => (part, ""),
        };
        if name.is_empty() {
            return Err(format!(
                "invalid composite path '{path}': segment '{part}' has no key"
            ));
        }
        if name.contains(']') {
            return Err(format!(
                "invalid composite path '{path}': unmatched ']' in '{part}'"
            ));
        }
        segments.push(Segment::Key(name.to_string()));
        let mut cursor = rest;
        while !cursor.is_empty() {
            if !cursor.starts_with('[') {
                return Err(format!(
                    "invalid composite path '{path}': unexpected '{cursor}'"
                ));
            }
            let close = cursor
                .find(']')
                .ok_or_else(|| format!("invalid composite path '{path}': missing ']'"))?;
            let digits = &cursor[1..close];
            if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
                return Err(format!(
                    "invalid composite path '{path}': index must be digits, got '[{digits}]'"
                ));
            }
            segments.push(Segment::Index(digits.parse().map_err(|e| format!("{e}"))?));
            cursor = &cursor[close + 1..];
        }
    }
    Ok(segments)
}

fn map_get<'a>(entries: &'a [(Value, Value)], key: &str) -> Option<&'a Value> {
    entries
        .iter()
        .find(|(k, _)| k.as_str() == Some(key))
        .map(|(_, v)| v)
}

fn map_get_mut<'a>(entries: &'a mut [(Value, Value)], key: &str) -> Option<&'a mut Value> {
    entries
        .iter_mut()
        .find(|(k, _)| k.as_str() == Some(key))
        .map(|(_, v)| v)
}

/// The per-transaction state machine view (Java `MultiLevelMap` over the
/// flow-instance dataset). Holds a map-rooted `rmpv::Value`.
#[derive(Debug, Clone, Default)]
pub struct MultiLevelMap {
    root: Vec<(Value, Value)>,
}

impl MultiLevelMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Wrap an existing map value (non-map roots become an empty tree).
    pub fn from_value(value: Value) -> Self {
        match value {
            Value::Map(entries) => Self { root: entries },
            _ => Self::default(),
        }
    }

    /// The whole tree as a map value.
    pub fn to_value(&self) -> Value {
        Value::Map(self.root.clone())
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    /// Retrieve by composite path — or by JSONPath when the path starts with
    /// `$` (Jayway parity: no match → `None`; one match → the value; several
    /// matches → an array of them). Explicit null resolves to `None`.
    pub fn get_element(&self, composite_path: &str) -> Option<Value> {
        if composite_path.starts_with('$') {
            return self.json_path_query(composite_path);
        }
        let segments = parse_path(composite_path).ok()?;
        let mut current: Option<&Value> = None;
        for seg in &segments {
            current = match (current, seg) {
                (None, Segment::Key(k)) => map_get(&self.root, k),
                (Some(Value::Map(m)), Segment::Key(k)) => map_get(m, k),
                (Some(Value::Array(l)), Segment::Index(i)) => l.get(*i),
                _ => return None,
            };
            current?;
        }
        match current {
            Some(Value::Nil) | None => None,
            Some(v) => Some(v.clone()),
        }
    }

    /// The `$.…` user-defined complex query (design E3 / maintainer decision):
    /// evaluated by `serde_json_path` over an on-demand JSON view.
    fn json_path_query(&self, path: &str) -> Option<Value> {
        let json = to_json(&self.to_value())?;
        let query = match serde_json_path::JsonPath::parse(path) {
            Ok(query) => query,
            // Jayway (the Java engine) tolerates hyphens in dot-notation
            // member names (e.g. `$.fetcher-ext.result`); RFC 9535 does not.
            // Rewrite such segments to bracket notation and retry.
            Err(_) => serde_json_path::JsonPath::parse(&bracketize_lenient_segments(path)).ok()?,
        };
        let nodes = query.query(&json).all();
        match nodes.len() {
            0 => None,
            1 => Some(from_json(nodes[0])),
            _ => Some(Value::Array(nodes.into_iter().map(from_json).collect())),
        }
    }

    /// True when the path resolves to a non-null value (Java `exists`).
    pub fn exists(&self, composite_path: &str) -> bool {
        self.get_element(composite_path).is_some()
    }

    /// True when the key is present even if its value is null (Java `keyExists`).
    pub fn key_exists(&self, composite_path: &str) -> bool {
        let Ok(segments) = parse_path(composite_path) else {
            return false;
        };
        let mut current: Option<&Value> = None;
        for seg in &segments {
            current = match (current, seg) {
                (None, Segment::Key(k)) => map_get(&self.root, k),
                (Some(Value::Map(m)), Segment::Key(k)) => map_get(m, k),
                (Some(Value::Array(l)), Segment::Index(i)) => l.get(*i),
                _ => return false,
            };
            if current.is_none() {
                return false;
            }
        }
        true
    }

    /// Set by composite path, creating intermediate maps/lists (lists pad
    /// with nulls up to the index). `key[]` appends: the empty index
    /// normalizes to the target list's current length (Java `appendIndex`).
    pub fn set_element(&mut self, composite_path: &str, value: Value) -> Result<(), String> {
        let normalized = if composite_path.contains("[]") {
            self.append_index(composite_path)
        } else {
            composite_path.to_string()
        };
        let segments = parse_path(&normalized)?;
        let mut root_holder = Value::Map(std::mem::take(&mut self.root));
        set_in(&mut root_holder, &segments, value);
        if let Value::Map(m) = root_holder {
            self.root = m;
        }
        Ok(())
    }

    /// Java `appendIndex`: replace the first `[]` with the current length of
    /// the list at that prefix (0 when absent), then RECURSE until no `[]`
    /// remains — nested appends like `model.rows[].items[]` resolve fully
    /// (increment 57, parity F17; previously only the first marker expanded
    /// and the leftover `[]` failed the mapping).
    fn append_index(&self, composite_path: &str) -> String {
        let Some(empty) = composite_path.find("[]") else {
            return composite_path.to_string();
        };
        let prefix = &composite_path[..empty];
        let len = match self.get_element(prefix) {
            Some(Value::Array(list)) => list.len(),
            _ => 0,
        };
        self.append_index(&format!("{prefix}[{len}]{}", &composite_path[empty + 2..]))
    }

    /// Remove by composite path: map keys are removed; list slots become null
    /// (sibling indices never shift — Java parity).
    pub fn remove_element(&mut self, composite_path: &str) {
        let Ok(segments) = parse_path(composite_path) else {
            return;
        };
        let mut root_holder = Value::Map(std::mem::take(&mut self.root));
        remove_in(&mut root_holder, &segments);
        if let Value::Map(m) = root_holder {
            self.root = m;
        }
    }
}

fn set_in(current: &mut Value, segments: &[Segment], value: Value) {
    match segments {
        [] => {}
        [last] => match (current, last) {
            (Value::Map(entries), Segment::Key(k)) => {
                if let Some(slot) = map_get_mut(entries, k) {
                    *slot = value;
                } else {
                    entries.push((Value::from(k.as_str()), value));
                }
            }
            (Value::Array(list), Segment::Index(i)) => {
                if list.len() <= *i {
                    list.resize(*i + 1, Value::Nil);
                }
                list[*i] = value;
            }
            _ => {}
        },
        [head, rest @ ..] => {
            let child_should_be_list = matches!(rest[0], Segment::Index(_));
            let container_fits = |v: &Value| {
                matches!(
                    (v, child_should_be_list),
                    (Value::Map(_), false) | (Value::Array(_), true)
                )
            };
            let fresh = || {
                if child_should_be_list {
                    Value::Array(Vec::new())
                } else {
                    Value::Map(Vec::new())
                }
            };
            let child = match (current, head) {
                (Value::Map(entries), Segment::Key(k)) => {
                    match map_get_mut(entries, k) {
                        Some(slot) => {
                            if !container_fits(slot) {
                                *slot = fresh();
                            }
                        }
                        None => entries.push((Value::from(k.as_str()), fresh())),
                    }
                    map_get_mut(entries, k)
                }
                (Value::Array(list), Segment::Index(i)) => {
                    if list.len() <= *i {
                        list.resize(*i + 1, Value::Nil);
                    }
                    if !container_fits(&list[*i]) {
                        list[*i] = fresh();
                    }
                    list.get_mut(*i)
                }
                _ => None,
            };
            if let Some(child) = child {
                set_in(child, rest, value);
            }
        }
    }
}

fn remove_in(current: &mut Value, segments: &[Segment]) {
    match segments {
        [] => {}
        [last] => match (current, last) {
            (Value::Map(entries), Segment::Key(k)) => {
                entries.retain(|(key, _)| key.as_str() != Some(k.as_str()));
            }
            (Value::Array(list), Segment::Index(i)) => {
                if let Some(slot) = list.get_mut(*i) {
                    *slot = Value::Nil;
                }
            }
            _ => {}
        },
        [head, rest @ ..] => {
            let child = match (current, head) {
                (Value::Map(entries), Segment::Key(k)) => map_get_mut(entries, k),
                (Value::Array(list), Segment::Index(i)) => list.get_mut(*i),
                _ => None,
            };
            if let Some(child) = child {
                remove_in(child, rest);
            }
        }
    }
}

/// Rewrite dot-notation segments that are not valid RFC 9535 shorthand
/// names (currently: names containing hyphens) into bracket notation —
/// `$.fetcher-ext.result` becomes `$['fetcher-ext'].result` (Jayway parity).
fn bracketize_lenient_segments(path: &str) -> String {
    let chars: Vec<char> = path.chars().collect();
    let mut out = String::with_capacity(path.len() + 8);
    let mut in_brackets = 0usize;
    let mut in_quote: Option<char> = None;
    let mut i = 0usize;
    while i < chars.len() {
        let c = chars[i];
        if let Some(quote) = in_quote {
            out.push(c);
            if c == quote {
                in_quote = None;
            }
            i += 1;
            continue;
        }
        match c {
            '\'' | '"' => {
                in_quote = Some(c);
                out.push(c);
                i += 1;
            }
            '[' => {
                in_brackets += 1;
                out.push(c);
                i += 1;
            }
            ']' => {
                in_brackets = in_brackets.saturating_sub(1);
                out.push(c);
                i += 1;
            }
            '.' if in_brackets == 0 => {
                let descendant = chars.get(i + 1) == Some(&'.');
                let start = if descendant { i + 2 } else { i + 1 };
                let mut end = start;
                while end < chars.len()
                    && (chars[end].is_alphanumeric() || chars[end] == '_' || chars[end] == '-')
                {
                    end += 1;
                }
                let name: String = chars[start..end].iter().collect();
                if name.contains('-') {
                    if descendant {
                        out.push_str("..");
                    }
                    out.push_str("['");
                    out.push_str(&name);
                    out.push_str("']");
                } else {
                    out.push('.');
                    if descendant {
                        out.push('.');
                    }
                    out.push_str(&name);
                }
                i = end;
            }
            other => {
                out.push(other);
                i += 1;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> MultiLevelMap {
        let mut m = MultiLevelMap::new();
        m.set_element("a.b.c", Value::from("hello")).unwrap();
        m.set_element("a.list[2]", Value::from(3)).unwrap();
        m.set_element("a.list[0]", Value::from(1)).unwrap();
        m
    }

    #[test]
    fn composite_get_set_with_padding() {
        let m = sample();
        assert_eq!(m.get_element("a.b.c"), Some(Value::from("hello")));
        assert_eq!(m.get_element("a.list[0]"), Some(Value::from(1)));
        // padded slot is explicit null → get is None but the key exists
        assert_eq!(m.get_element("a.list[1]"), None);
        assert!(m.key_exists("a.list[1]"));
        assert_eq!(m.get_element("a.list[2]"), Some(Value::from(3)));
        assert!(!m.exists("a.nope"));
        assert!(!m.key_exists("a.nope"));
    }

    #[test]
    fn empty_index_appends() {
        let mut m = MultiLevelMap::new();
        m.set_element("x.items[]", Value::from("first")).unwrap();
        m.set_element("x.items[]", Value::from("second")).unwrap();
        assert_eq!(m.get_element("x.items[0]"), Some(Value::from("first")));
        assert_eq!(m.get_element("x.items[1]"), Some(Value::from("second")));
    }

    #[test]
    fn remove_keeps_list_indices_stable() {
        let mut m = sample();
        m.remove_element("a.list[0]");
        assert_eq!(m.get_element("a.list[0]"), None);
        assert_eq!(m.get_element("a.list[2]"), Some(Value::from(3)));
        m.remove_element("a.b.c");
        assert!(!m.key_exists("a.b.c"));
    }

    #[test]
    fn json_path_is_the_complex_query_escape_hatch() {
        let mut m = MultiLevelMap::new();
        m.set_element("shop.items[0].price", Value::from(10))
            .unwrap();
        m.set_element("shop.items[1].price", Value::from(25))
            .unwrap();
        m.set_element("shop.items[2].price", Value::from(5))
            .unwrap();
        // single match → scalar
        assert_eq!(
            m.get_element("$.shop.items[1].price"),
            Some(Value::from(25))
        );
        // multi-match query → array
        assert_eq!(
            m.get_element("$.shop.items[*].price"),
            Some(Value::Array(vec![
                Value::from(10),
                Value::from(25),
                Value::from(5)
            ]))
        );
        // filter expression (the "user-defined complex query" case)
        assert_eq!(
            m.get_element("$.shop.items[?(@.price > 8)].price"),
            Some(Value::Array(vec![Value::from(10), Value::from(25)]))
        );
        // no match
        assert_eq!(m.get_element("$.shop.missing"), None);
    }

    #[test]
    fn jsonpath_tolerates_hyphenated_member_names_like_jayway() {
        let mut m = MultiLevelMap::new();
        m.set_element("fetcher-ext.result[0].account_details", Value::from("a"))
            .unwrap();
        m.set_element("fetcher-ext.result[1].account_details", Value::from("b"))
            .unwrap();
        assert_eq!(
            m.get_element("$.fetcher-ext.result[*].account_details"),
            Some(Value::Array(vec![Value::from("a"), Value::from("b")]))
        );
    }

    /// Increment 57 (parity F17): nested `[]` markers all expand — Java
    /// appendIndex recurses until none remain.
    #[test]
    fn nested_append_markers_expand_recursively() {
        let mut m = MultiLevelMap::new();
        m.set_element("model.rows[].items[]", Value::from("a"))
            .unwrap();
        m.set_element("model.rows[0].items[]", Value::from("b"))
            .unwrap();
        m.set_element("model.rows[].items[]", Value::from("c"))
            .unwrap();
        assert_eq!(
            m.get_element("model.rows[0].items[0]"),
            Some(Value::from("a"))
        );
        assert_eq!(
            m.get_element("model.rows[0].items[1]"),
            Some(Value::from("b"))
        );
        assert_eq!(
            m.get_element("model.rows[1].items[0]"),
            Some(Value::from("c"))
        );
    }
}
