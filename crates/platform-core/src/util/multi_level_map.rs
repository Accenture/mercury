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

//! Rust port of the Java `MultiLevelMap` (`org.platformlambda.core.util.MultiLevelMap`)
//! — a nested map/list tree addressed by composite dot-bracket paths such as
//! `a.b.c` or `x.y[0].z`.

use std::collections::BTreeMap;
use std::fmt;

/// The dynamic value tree — the Rust analog of Java's untyped `Object` config tree.
#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    List(Vec<ConfigValue>),
    Map(BTreeMap<String, ConfigValue>),
}

impl ConfigValue {
    pub fn is_null(&self) -> bool {
        matches!(self, ConfigValue::Null)
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            ConfigValue::Text(s) => Some(s),
            _ => None,
        }
    }

    /// String-enforced rendering, mirroring Java's `String.valueOf(value)` used by
    /// `getProperty`. Maps/lists render as JSON for readability.
    pub fn to_display_string(&self) -> String {
        match self {
            ConfigValue::Null => "null".to_string(),
            ConfigValue::Bool(b) => b.to_string(),
            ConfigValue::Int(i) => i.to_string(),
            ConfigValue::Float(f) => f.to_string(),
            ConfigValue::Text(s) => s.clone(),
            ConfigValue::List(_) | ConfigValue::Map(_) => self.to_json().to_string(),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            ConfigValue::Null => serde_json::Value::Null,
            ConfigValue::Bool(b) => serde_json::Value::Bool(*b),
            ConfigValue::Int(i) => serde_json::Value::from(*i),
            ConfigValue::Float(f) => serde_json::Value::from(*f),
            ConfigValue::Text(s) => serde_json::Value::String(s.clone()),
            ConfigValue::List(l) => {
                serde_json::Value::Array(l.iter().map(|v| v.to_json()).collect())
            }
            ConfigValue::Map(m) => {
                serde_json::Value::Object(m.iter().map(|(k, v)| (k.clone(), v.to_json())).collect())
            }
        }
    }

    pub fn from_json(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => ConfigValue::Null,
            serde_json::Value::Bool(b) => ConfigValue::Bool(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    ConfigValue::Int(i)
                } else {
                    ConfigValue::Float(n.as_f64().unwrap_or(0.0))
                }
            }
            serde_json::Value::String(s) => ConfigValue::Text(s.clone()),
            serde_json::Value::Array(a) => {
                ConfigValue::List(a.iter().map(ConfigValue::from_json).collect())
            }
            serde_json::Value::Object(o) => ConfigValue::Map(
                o.iter()
                    .map(|(k, v)| (k.clone(), ConfigValue::from_json(v)))
                    .collect(),
            ),
        }
    }

    /// YAML → ConfigValue, enforcing keys as text (the Java `enforceKeysAsText` behavior).
    pub fn from_yaml(value: &serde_yaml::Value) -> Self {
        match value {
            serde_yaml::Value::Null => ConfigValue::Null,
            serde_yaml::Value::Bool(b) => ConfigValue::Bool(*b),
            serde_yaml::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    ConfigValue::Int(i)
                } else {
                    ConfigValue::Float(n.as_f64().unwrap_or(0.0))
                }
            }
            serde_yaml::Value::String(s) => ConfigValue::Text(s.clone()),
            serde_yaml::Value::Sequence(seq) => {
                ConfigValue::List(seq.iter().map(ConfigValue::from_yaml).collect())
            }
            serde_yaml::Value::Mapping(m) => {
                let mut out = BTreeMap::new();
                for (k, v) in m {
                    let key = match k {
                        serde_yaml::Value::String(s) => s.clone(),
                        other => ConfigValue::from_yaml(other).to_display_string(),
                    };
                    out.insert(key, ConfigValue::from_yaml(v));
                }
                ConfigValue::Map(out)
            }
            serde_yaml::Value::Tagged(t) => ConfigValue::from_yaml(&t.value),
        }
    }
}

impl fmt::Display for ConfigValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_display_string())
    }
}

/// One parsed element of a composite path: a map key or a list index.
#[derive(Clone, Debug, PartialEq)]
enum Segment {
    Key(String),
    Index(usize),
}

/// Parse a composite dot-bracket path (`a.b[0][1].c`) into segments.
/// Mirrors Java's `validateCompositePathSyntax` — invalid syntax is an error.
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
        // zero or more [n] suffixes
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

/// A nested map/list tree addressed by composite dot-bracket paths.
///
/// Port of Java's `MultiLevelMap`; `flat_map()` is the analog of
/// `Utility.getFlatMap` — nested maps flatten to dotted keys, list elements to
/// `key[i]` (recursively).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MultiLevelMap {
    root: BTreeMap<String, ConfigValue>,
}

impl MultiLevelMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_map(map: BTreeMap<String, ConfigValue>) -> Self {
        Self { root: map }
    }

    /// Replace the whole tree (Java `reload`).
    pub fn reload(&mut self, map: BTreeMap<String, ConfigValue>) {
        self.root = map;
    }

    /// The raw underlying map (Java `getMap`).
    pub fn get_map(&self) -> &BTreeMap<String, ConfigValue> {
        &self.root
    }

    pub fn into_map(self) -> BTreeMap<String, ConfigValue> {
        self.root
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    /// Retrieve an element by composite path. `None` when the path is missing,
    /// syntactically invalid, or the value is explicit null (Java returns `null`
    /// in all three cases).
    pub fn get_element(&self, composite_path: &str) -> Option<&ConfigValue> {
        let segments = parse_path(composite_path).ok()?;
        let mut current: Option<&ConfigValue> = None;
        for seg in &segments {
            current = match (current, seg) {
                (None, Segment::Key(k)) => self.root.get(k),
                (Some(ConfigValue::Map(m)), Segment::Key(k)) => m.get(k),
                (Some(ConfigValue::List(l)), Segment::Index(i)) => l.get(*i),
                _ => return None,
            };
            current?;
        }
        match current {
            Some(ConfigValue::Null) | None => None,
            some => some,
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
        let mut current: Option<&ConfigValue> = None;
        for seg in &segments {
            current = match (current, seg) {
                (None, Segment::Key(k)) => self.root.get(k),
                (Some(ConfigValue::Map(m)), Segment::Key(k)) => m.get(k),
                (Some(ConfigValue::List(l)), Segment::Index(i)) => l.get(*i),
                _ => return false,
            };
            if current.is_none() {
                return false;
            }
        }
        true
    }

    /// Set an element by composite path, creating intermediate maps/lists as
    /// needed (lists are padded with nulls up to the index — Java behavior).
    ///
    /// # Panics
    /// Panics on invalid path syntax — the analog of Java's unchecked
    /// `IllegalArgumentException` from `validateCompositePathSyntax`.
    pub fn set_element(&mut self, composite_path: &str, value: ConfigValue) -> &mut Self {
        self.try_set_element(composite_path, value)
            .unwrap_or_else(|e| panic!("set_element: {e}"));
        self
    }

    /// Fallible variant of [`set_element`](Self::set_element) — used by file
    /// loaders, where an invalid key should surface as an error rather than a
    /// panic (Java's checked path: the exception propagates out of `load`).
    pub fn try_set_element(
        &mut self,
        composite_path: &str,
        value: ConfigValue,
    ) -> Result<(), String> {
        let segments = parse_path(composite_path)?;
        // The root is a map, so the first segment is always a Key (guaranteed by the parser).
        let mut root_holder = ConfigValue::Map(std::mem::take(&mut self.root));
        set_in(&mut root_holder, &segments, value);
        if let ConfigValue::Map(m) = root_holder {
            self.root = m;
        }
        Ok(())
    }

    /// Remove an element by composite path. Map keys are removed; list slots are
    /// set to null (mirrors the Java semantics of not shifting sibling indices).
    pub fn remove_element(&mut self, composite_path: &str) -> &mut Self {
        let Ok(segments) = parse_path(composite_path) else {
            return self;
        };
        let mut root_holder = ConfigValue::Map(std::mem::take(&mut self.root));
        remove_in(&mut root_holder, &segments);
        if let ConfigValue::Map(m) = root_holder {
            self.root = m;
        }
        self
    }

    /// Flatten the tree into composite keys (the `Utility.getFlatMap` analog).
    /// Only leaf values appear; nested maps become dotted keys and list elements
    /// become `key[i]` (recursively, including maps inside lists).
    pub fn flat_map(&self) -> BTreeMap<String, ConfigValue> {
        let mut target = BTreeMap::new();
        for (k, v) in &self.root {
            flatten(k, v, &mut target);
        }
        target
    }

    /// Rebuild a normalized tree from any flat map of composite keys.
    pub fn from_flat_map(flat: &BTreeMap<String, ConfigValue>) -> Self {
        let mut mlm = MultiLevelMap::new();
        // BTreeMap iterates in sorted key order — the Java code sorts keys explicitly.
        for (k, v) in flat {
            mlm.set_element(k, v.clone());
        }
        mlm
    }
}

fn flatten(prefix: &str, value: &ConfigValue, target: &mut BTreeMap<String, ConfigValue>) {
    match value {
        ConfigValue::Map(m) => {
            if m.is_empty() {
                target.insert(prefix.to_string(), value.clone());
            } else {
                for (k, v) in m {
                    flatten(&format!("{prefix}.{k}"), v, target);
                }
            }
        }
        ConfigValue::List(l) => {
            if l.is_empty() {
                target.insert(prefix.to_string(), value.clone());
            } else {
                for (i, v) in l.iter().enumerate() {
                    flatten(&format!("{prefix}[{i}]"), v, target);
                }
            }
        }
        leaf => {
            target.insert(prefix.to_string(), leaf.clone());
        }
    }
}

fn set_in(current: &mut ConfigValue, segments: &[Segment], value: ConfigValue) {
    let (seg, rest) = segments.split_first().expect("segments must not be empty");
    match seg {
        Segment::Key(k) => {
            // ensure current is a map (replace a conflicting intermediate — Java behavior)
            if !matches!(current, ConfigValue::Map(_)) {
                *current = ConfigValue::Map(BTreeMap::new());
            }
            let ConfigValue::Map(m) = current else {
                unreachable!()
            };
            if rest.is_empty() {
                m.insert(k.clone(), value);
            } else {
                let placeholder = match rest[0] {
                    Segment::Index(_) => ConfigValue::List(Vec::new()),
                    Segment::Key(_) => ConfigValue::Map(BTreeMap::new()),
                };
                let entry = m.entry(k.clone()).or_insert(placeholder);
                set_in(entry, rest, value);
            }
        }
        Segment::Index(i) => {
            if !matches!(current, ConfigValue::List(_)) {
                *current = ConfigValue::List(Vec::new());
            }
            let ConfigValue::List(l) = current else {
                unreachable!()
            };
            while l.len() <= *i {
                l.push(ConfigValue::Null); // pad with nulls — Java behavior
            }
            if rest.is_empty() {
                l[*i] = value;
            } else {
                let needs_replacement = !matches!(
                    (&l[*i], &rest[0]),
                    (ConfigValue::Map(_), Segment::Key(_))
                        | (ConfigValue::List(_), Segment::Index(_))
                );
                if needs_replacement {
                    l[*i] = match rest[0] {
                        Segment::Index(_) => ConfigValue::List(Vec::new()),
                        Segment::Key(_) => ConfigValue::Map(BTreeMap::new()),
                    };
                }
                set_in(&mut l[*i], rest, value);
            }
        }
    }
}

fn remove_in(current: &mut ConfigValue, segments: &[Segment]) {
    let (seg, rest) = segments.split_first().expect("segments must not be empty");
    if rest.is_empty() {
        match (current, seg) {
            (ConfigValue::Map(m), Segment::Key(k)) => {
                m.remove(k);
            }
            (ConfigValue::List(l), Segment::Index(i)) if *i < l.len() => {
                l[*i] = ConfigValue::Null;
            }
            _ => {}
        }
    } else {
        match (current, seg) {
            (ConfigValue::Map(m), Segment::Key(k)) => {
                if let Some(next) = m.get_mut(k) {
                    remove_in(next, rest);
                }
            }
            (ConfigValue::List(l), Segment::Index(i)) => {
                if let Some(next) = l.get_mut(*i) {
                    remove_in(next, rest);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composite_set_and_get() {
        let mut m = MultiLevelMap::new();
        m.set_element("a.b.c", ConfigValue::Int(42));
        assert_eq!(m.get_element("a.b.c"), Some(&ConfigValue::Int(42)));
        assert_eq!(m.get_element("a.b.missing"), None);
        assert_eq!(m.get_element("a"), m.root.get("a"));
    }

    #[test]
    fn list_index_set_and_get_with_padding() {
        let mut m = MultiLevelMap::new();
        m.set_element("x.y[2].z", ConfigValue::Text("deep".into()));
        // indices 0 and 1 padded with null
        assert!(!m.exists("x.y[0]"));
        assert!(m.key_exists("x.y[0]"));
        assert_eq!(
            m.get_element("x.y[2].z"),
            Some(&ConfigValue::Text("deep".into()))
        );
    }

    #[test]
    fn multi_dimension_index() {
        let mut m = MultiLevelMap::new();
        m.set_element("grid[1][2]", ConfigValue::Bool(true));
        assert_eq!(m.get_element("grid[1][2]"), Some(&ConfigValue::Bool(true)));
        assert_eq!(m.get_element("grid[0]"), None); // padded null
    }

    #[test]
    fn exists_vs_key_exists() {
        let mut m = MultiLevelMap::new();
        m.set_element("present.null", ConfigValue::Null);
        m.set_element("present.value", ConfigValue::Int(1));
        assert!(!m.exists("present.null"));
        assert!(m.key_exists("present.null"));
        assert!(m.exists("present.value"));
        assert!(!m.key_exists("absent.key"));
    }

    #[test]
    fn remove_element() {
        let mut m = MultiLevelMap::new();
        m.set_element("a.b", ConfigValue::Int(1));
        m.set_element("a.c", ConfigValue::Int(2));
        m.remove_element("a.b");
        assert!(!m.key_exists("a.b"));
        assert!(m.exists("a.c"));
    }

    #[test]
    fn flat_map_round_trip() {
        let mut m = MultiLevelMap::new();
        m.set_element("app.name", ConfigValue::Text("mercury".into()));
        m.set_element("server.port", ConfigValue::Int(8085));
        m.set_element("deep.list[0].name", ConfigValue::Text("first".into()));
        m.set_element("deep.list[1].name", ConfigValue::Text("second".into()));
        let flat = m.flat_map();
        assert_eq!(
            flat.get("app.name"),
            Some(&ConfigValue::Text("mercury".into()))
        );
        assert_eq!(
            flat.get("deep.list[1].name"),
            Some(&ConfigValue::Text("second".into()))
        );
        let rebuilt = MultiLevelMap::from_flat_map(&flat);
        assert_eq!(rebuilt, m);
    }

    #[test]
    #[should_panic(expected = "invalid composite path")]
    fn invalid_path_panics_on_set() {
        let mut m = MultiLevelMap::new();
        m.set_element("a..b", ConfigValue::Null);
    }

    #[test]
    fn invalid_path_is_none_on_get() {
        let m = MultiLevelMap::new();
        assert_eq!(m.get_element("a..b"), None);
        assert_eq!(m.get_element("a[x]"), None);
        assert_eq!(m.get_element(""), None);
    }
}
