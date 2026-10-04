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
    /// `$`, in Jayway's result shape (see `json_path_query`). Explicit null
    /// resolves to `None`.
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
    /// evaluated by `serde_json_path` over an on-demand JSON view, with the
    /// result shaped the way Jayway shapes it for the Java engine - by the kind
    /// of path, not by the number of matches. A definite path (child names and
    /// single indexes only) yields the value, or `None` when it is absent. An
    /// indefinite path (a filter, a wildcard, a descendant segment, a slice or
    /// a union) always yields a list, one match being a one-element list and
    /// none an empty list - unless a member name before its first indefinite
    /// step is missing, which Jayway reports as not found (`None`).
    fn json_path_query(&self, path: &str) -> Option<Value> {
        let json = to_json(&self.to_value())?;
        let (query, parsed) = match serde_json_path::JsonPath::parse(path) {
            Ok(query) => (query, path.to_string()),
            // Jayway (the Java engine) tolerates hyphens in dot-notation
            // member names (e.g. `$.fetcher-ext.result`); RFC 9535 does not.
            // Rewrite such segments to bracket notation and retry.
            Err(_) => {
                let lenient = bracketize_lenient_segments(path);
                (serde_json_path::JsonPath::parse(&lenient).ok()?, lenient)
            }
        };
        let nodes = query.query(&json).all();
        match path_shape(&parsed) {
            Some(PathShape::Indefinite(prefix)) => {
                if misses_a_member_name(&json, &prefix) {
                    None
                } else {
                    Some(Value::Array(nodes.into_iter().map(from_json).collect()))
                }
            }
            // a definite path selects at most one node; a path the scanner does
            // not recognize keeps the count rule
            Some(PathShape::Definite) | None => match nodes.len() {
                0 => None,
                1 => Some(from_json(nodes[0])),
                _ => Some(Value::Array(nodes.into_iter().map(from_json).collect())),
            },
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

/// One step of a JSONPath's definite prefix: a member name or an array index.
#[derive(Debug, PartialEq)]
enum PathStep {
    Name(String),
    Index(i64),
}

/// How Jayway shapes a path's result. A path is definite when every segment is a
/// child member name or a single index (RFC 9535's singular query); otherwise it
/// is indefinite, carrying the steps before its first indefinite segment.
#[derive(Debug, PartialEq)]
enum PathShape {
    Definite,
    Indefinite(Vec<PathStep>),
}

/// Classify a path that `serde_json_path` has already parsed. `None` when a
/// construct is not recognized, so the caller keeps the count rule.
fn path_shape(path: &str) -> Option<PathShape> {
    let chars: Vec<char> = path.trim().chars().collect();
    if chars.first() != Some(&'$') {
        return None;
    }
    let mut steps = Vec::new();
    let mut i = 1usize;
    while i < chars.len() {
        match chars[i] {
            c if c.is_whitespace() => i += 1,
            '.' => {
                // a descendant segment or the wildcard shorthand
                if matches!(chars.get(i + 1), Some('.') | Some('*')) {
                    return Some(PathShape::Indefinite(steps));
                }
                let start = i + 1;
                let mut end = start;
                while end < chars.len()
                    && chars[end] != '.'
                    && chars[end] != '['
                    && !chars[end].is_whitespace()
                {
                    end += 1;
                }
                if end == start {
                    return None;
                }
                steps.push(PathStep::Name(chars[start..end].iter().collect()));
                i = end;
            }
            '[' => {
                let (selectors, next) = bracketed_selectors(&chars, i + 1)?;
                // a union of selectors is indefinite, even of names or indexes
                if selectors.len() != 1 {
                    return Some(PathShape::Indefinite(steps));
                }
                let selector = selectors[0].as_str();
                if let Some(name) = string_literal(selector) {
                    steps.push(PathStep::Name(name));
                } else if let Ok(index) = selector.parse::<i64>() {
                    steps.push(PathStep::Index(index));
                } else if selector == "*" || selector.starts_with('?') || selector.contains(':') {
                    return Some(PathShape::Indefinite(steps));
                } else {
                    return None;
                }
                i = next;
            }
            _ => return None,
        }
    }
    Some(PathShape::Definite)
}

/// The selectors of one bracketed segment, split at its top-level commas
/// (outside quotes and a filter's own brackets), and the position after the
/// closing bracket.
fn bracketed_selectors(chars: &[char], start: usize) -> Option<(Vec<String>, usize)> {
    let mut selectors = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;
    let mut quote: Option<char> = None;
    let mut i = start;
    while i < chars.len() {
        let c = chars[i];
        if let Some(q) = quote {
            current.push(c);
            if c == '\\' {
                if let Some(&escaped) = chars.get(i + 1) {
                    current.push(escaped);
                    i += 1;
                }
            } else if c == q {
                quote = None;
            }
        } else {
            match c {
                '\'' | '"' => {
                    quote = Some(c);
                    current.push(c);
                }
                '(' | '[' => {
                    depth += 1;
                    current.push(c);
                }
                ')' => {
                    depth = depth.saturating_sub(1);
                    current.push(c);
                }
                ']' if depth == 0 => {
                    selectors.push(current.trim().to_string());
                    return Some((selectors, i + 1));
                }
                ']' => {
                    depth -= 1;
                    current.push(c);
                }
                ',' if depth == 0 => {
                    selectors.push(current.trim().to_string());
                    current.clear();
                }
                _ => current.push(c),
            }
        }
        i += 1;
    }
    None
}

/// Decode an RFC 9535 string literal, single- or double-quoted.
fn string_literal(selector: &str) -> Option<String> {
    let quote = selector.chars().next()?;
    if (quote != '\'' && quote != '"') || selector.len() < 2 || !selector.ends_with(quote) {
        return None;
    }
    let inner: Vec<char> = selector[1..selector.len() - 1].chars().collect();
    let mut out = String::new();
    let mut i = 0usize;
    while i < inner.len() {
        if inner[i] != '\\' {
            out.push(inner[i]);
            i += 1;
            continue;
        }
        match inner.get(i + 1)? {
            'b' => out.push('\u{8}'),
            'f' => out.push('\u{c}'),
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            'u' => {
                let high = hex4(&inner, i + 2)?;
                if (0xD800..0xDC00).contains(&high) {
                    // a surrogate pair: 😀
                    if inner.get(i + 6) != Some(&'\\') || inner.get(i + 7) != Some(&'u') {
                        return None;
                    }
                    let low = hex4(&inner, i + 8)?;
                    let code = 0x10000 + ((high - 0xD800) << 10) + (low.checked_sub(0xDC00)?);
                    out.push(char::from_u32(code)?);
                    i += 12;
                } else {
                    out.push(char::from_u32(high)?);
                    i += 6;
                }
                continue;
            }
            other => out.push(*other),
        }
        i += 2;
    }
    Some(out)
}

fn hex4(chars: &[char], start: usize) -> Option<u32> {
    let text: String = chars.get(start..start + 4)?.iter().collect();
    u32::from_str_radix(&text, 16).ok()
}

/// Jayway reports a path as not found ("Missing property in path") when a member
/// name before the first indefinite step is absent, or is applied to a value that
/// is not an object. A missing index there is not an error: the list is empty.
fn misses_a_member_name(root: &serde_json::Value, prefix: &[PathStep]) -> bool {
    let mut current = Some(root);
    for step in prefix {
        let Some(node) = current else {
            return false;
        };
        current = match step {
            PathStep::Name(name) => match node.as_object().and_then(|map| map.get(name)) {
                Some(child) => Some(child),
                None => return true,
            },
            PathStep::Index(index) => node.as_array().and_then(|list| {
                let at = if *index < 0 {
                    list.len() as i64 + index
                } else {
                    *index
                };
                usize::try_from(at).ok().and_then(|at| list.get(at))
            }),
        };
    }
    false
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
        // a definite path (names and single indexes) → the value
        assert_eq!(
            m.get_element("$.shop.items[1].price"),
            Some(Value::from(25))
        );
        // an indefinite path → a list
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
        // a definite path that is absent → not found
        assert_eq!(m.get_element("$.shop.missing"), None);
    }

    /// The Jayway shapes the Java engine returns (probed against Jayway 3.0.0):
    /// the kind of path decides, never the number of matches.
    #[test]
    fn json_path_result_shape_follows_jayway() {
        let mut m = MultiLevelMap::new();
        m.set_element("shop.items[0].price", Value::from(10))
            .unwrap();
        m.set_element("shop.items[1].price", Value::from(25))
            .unwrap();
        m.set_element("shop.items[2].price", Value::from(5))
            .unwrap();
        m.set_element("shop.single[0]", Value::from("only"))
            .unwrap();
        let list = |values: Vec<Value>| Some(Value::Array(values));
        // one match of an indefinite path is a one-element list, none an empty list
        assert_eq!(
            m.get_element("$.shop.items[?(@.price > 20)].price"),
            list(vec![Value::from(25)])
        );
        assert_eq!(
            m.get_element("$.shop.items[?(@.price > 100)].price"),
            list(vec![])
        );
        assert_eq!(
            m.get_element("$.shop.single[*]"),
            list(vec![Value::from("only")])
        );
        assert_eq!(
            m.get_element("$.shop.items[0:1].price"),
            list(vec![Value::from(10)])
        );
        assert_eq!(
            m.get_element("$.shop.items[0,1].price"),
            list(vec![Value::from(10), Value::from(25)])
        );
        assert_eq!(
            m.get_element("$..price"),
            list(vec![Value::from(10), Value::from(25), Value::from(5)])
        );
        // after the first indefinite step a missing member only empties the list
        assert_eq!(m.get_element("$.shop.items[*].missing"), list(vec![]));
        assert_eq!(m.get_element("$..missing"), list(vec![]));
        // a missing index before it is not an error either
        assert_eq!(m.get_element("$.shop.items[9][*]"), list(vec![]));
        // ... but a missing member name before it is not found, as is a name
        // applied to a value that is not an object
        assert_eq!(m.get_element("$.shop.missing[*]"), None);
        assert_eq!(m.get_element("$.missing..price"), None);
        assert_eq!(m.get_element("$.shop.items[1].missing[*]"), None);
        assert_eq!(m.get_element("$.shop.single.x[*]"), None);
        // definite paths, bracketed and negative indexes included
        assert_eq!(
            m.get_element("$['shop']['items'][-1].price"),
            Some(Value::from(5))
        );
        assert_eq!(m.get_element("$.shop.items[7].price"), None);
    }

    #[test]
    fn path_shape_reads_brackets_quotes_and_filters() {
        use PathShape::{Definite, Indefinite};
        use PathStep::{Index, Name};
        assert_eq!(path_shape("$.a.b[0]"), Some(Definite));
        assert_eq!(path_shape("$['a.b'][\"c]d\"][-2]"), Some(Definite));
        assert_eq!(
            path_shape("$.a['x,y'][?(@['e,f'] == 'u,]v')].z"),
            Some(Indefinite(vec![Name("a".into()), Name("x,y".into())]))
        );
        assert_eq!(
            path_shape("$.a[1][*]"),
            Some(Indefinite(vec![Name("a".into()), Index(1)]))
        );
        assert_eq!(path_shape("$..a"), Some(Indefinite(vec![])));
        assert_eq!(
            path_shape("$.a.*"),
            Some(Indefinite(vec![Name("a".into())]))
        );
        assert_eq!(
            path_shape("$.a[1:3]"),
            Some(Indefinite(vec![Name("a".into())]))
        );
        assert_eq!(
            path_shape("$.a['b','c']"),
            Some(Indefinite(vec![Name("a".into())]))
        );
        // escapes in a quoted name, a surrogate pair included
        assert_eq!(
            path_shape("$['it\\'s']['\\u00e9\\uD83D\\uDE00'][*]"),
            Some(Indefinite(vec![
                Name("it's".into()),
                Name("\u{e9}\u{1F600}".into())
            ]))
        );
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
