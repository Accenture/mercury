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

//! Rust port of the Java `ConfigReader` (`org.platformlambda.core.util.ConfigReader`).
//!
//! Loads `.yml` / `.yaml` (interchangeable), `.json`, and `.properties` files from
//! `classpath:/` (the resource roots — see [`crate::util::resources`]) or `file:/`
//! paths, and resolves `${...}` references with the exact Java precedence:
//!
//! 1. the process **override registry** (the `System.getProperty` analog) wins for
//!    any key lookup;
//! 2. inside `${...}`: **environment variable** → **base-config key reference**
//!    (recursive, with loop detection) → the **`:default`** fallback.
//!
//! Base-config references resolve against the [`AppConfigReader`] singleton once
//! it is initialized (or against the reader itself for the base config). Before
//! that, `${...}` values are returned raw — mirroring Java's `baseConfig == null`
//! behavior.
//!
//! [`AppConfigReader`]: crate::util::app_config_reader::AppConfigReader

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::util::app_config_reader;
use crate::util::multi_level_map::{ConfigValue, MultiLevelMap};
use crate::util::overrides;
use crate::util::resources;

const CLASSPATH: &str = "classpath:";
const FILEPATH: &str = "file:";
const REF_BEGIN: &str = "${";

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// The configuration file was not found (Java `IllegalArgumentException("<path> not found")`).
    #[error("{0} not found")]
    NotFound(String),
    /// Invalid path or content (Java `IllegalArgumentException`).
    #[error("{0}")]
    Invalid(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// A configuration reader over a [`MultiLevelMap`], with `${...}` substitution.
#[derive(Debug, Default)]
pub struct ConfigReader {
    map: MultiLevelMap,
    /// True for the AppConfigReader's inner reader — base refs then resolve
    /// against `self` (Java: `isBaseConfig()`).
    is_base: bool,
    resolved: bool,
    flat_cache: OnceLock<BTreeMap<String, ConfigValue>>,
}

impl ConfigReader {
    /// Load a configuration file and resolve references (Java `new ConfigReader(path)`).
    pub fn load(path: &str) -> Result<Self, ConfigError> {
        let mut reader = Self::load_raw(path)?;
        reader.resolve_references();
        Ok(reader)
    }

    /// Load a configuration file **without** resolving references
    /// (Java `load(path, false)` — used while merging base configuration files).
    pub fn load_raw(path: &str) -> Result<Self, ConfigError> {
        let mut reader = ConfigReader::default();
        reader.load_into(path)?;
        Ok(reader)
    }

    /// Build a reader from an existing nested map and resolve references
    /// (Java `load(Map)`).
    pub fn from_map(map: BTreeMap<String, ConfigValue>) -> Self {
        let mut reader = ConfigReader {
            map: MultiLevelMap::from_map(map),
            ..ConfigReader::default()
        };
        reader.resolve_references();
        reader
    }

    /// Crate-internal: build the **base** reader (the AppConfigReader's inner
    /// reader). Self-references resolve against this reader itself.
    pub(crate) fn new_base(map: MultiLevelMap) -> Self {
        let mut reader = ConfigReader {
            map,
            is_base: true,
            ..ConfigReader::default()
        };
        reader.resolve_references();
        reader
    }

    // ---- lookup API (Java ConfigBase) ----

    /// Retrieve a value by composite key. `None` for a missing key, an explicit
    /// null, or an unresolvable `${...}` reference (Java returns `null` in each
    /// case).
    pub fn get(&self, key: &str) -> Option<ConfigValue> {
        let mut visited = Vec::new();
        self.get_with(key, None, &mut visited)
    }

    /// Retrieve a value by composite key with a default (Java `get(key, defaultValue)`).
    pub fn get_or(&self, key: &str, default: ConfigValue) -> ConfigValue {
        let mut visited = Vec::new();
        self.get_with(key, Some(&default), &mut visited)
            .unwrap_or(default)
    }

    /// Retrieve a value enforced as a string (Java `getProperty`).
    pub fn get_property(&self, key: &str) -> Option<String> {
        self.get(key).map(|v| v.to_display_string())
    }

    /// Retrieve a value enforced as a string, with a default (Java `getProperty(key, default)`).
    pub fn get_property_or(&self, key: &str, default: &str) -> String {
        self.get_property(key)
            .unwrap_or_else(|| default.to_string())
    }

    /// True when the key resolves to a non-null value (Java `exists`).
    pub fn exists(&self, key: &str) -> bool {
        if key.is_empty() {
            return false;
        }
        self.map.exists(key)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// The raw underlying tree, without substitution (Java `getMap`).
    pub fn get_map(&self) -> &MultiLevelMap {
        &self.map
    }

    /// Flat map of composite key-values with substitution applied, computed once
    /// and cached (Java `getCompositeKeyValues`).
    pub fn get_composite_key_values(&self) -> &BTreeMap<String, ConfigValue> {
        self.flat_cache.get_or_init(|| {
            let flat = self.map.flat_map();
            flat.keys()
                .map(|k| (k.clone(), self.get(k).unwrap_or(ConfigValue::Null)))
                .collect()
        })
    }

    pub fn is_base_config(&self) -> bool {
        self.is_base
    }

    // ---- resolution engine ----

    /// Full lookup chain: override registry → own tree → `${...}` substitution.
    /// `visited` carries the loop-detection state across base-reference recursion.
    pub(crate) fn get_with(
        &self,
        key: &str,
        default: Option<&ConfigValue>,
        visited: &mut Vec<String>,
    ) -> Option<ConfigValue> {
        if key.is_empty() {
            return None;
        }
        // 1. process override (the System.getProperty analog) always wins
        if let Some(v) = overrides::get(key) {
            return Some(ConfigValue::Text(v));
        }
        // 2. own tree
        let value = match self.map.get_element(key) {
            Some(v) => v.clone(),
            None => return default.cloned(),
        };
        // 3. ${...} substitution (only when a base config is reachable — Java:
        //    `baseConfig != null`)
        if let ConfigValue::Text(text) = &value {
            if text.contains(REF_BEGIN) && self.base_available() {
                let segments = extract_segments(text);
                if !segments.is_empty() {
                    return self
                        .reconstruct(&segments, key, text, default, visited)
                        .map(ConfigValue::Text);
                }
            }
        }
        Some(value)
    }

    fn base_available(&self) -> bool {
        self.is_base || app_config_reader::try_base_reader().is_some()
    }

    fn base_get(
        &self,
        name: &str,
        default: Option<&ConfigValue>,
        visited: &mut Vec<String>,
    ) -> Option<ConfigValue> {
        if self.is_base {
            self.get_with(name, default, visited)
        } else {
            app_config_reader::try_base_reader()
                .and_then(|base| base.get_with(name, default, visited))
        }
    }

    /// Rebuild a text value from its `${...}` segments
    /// (Java `reconstructFromVarSegments`).
    fn reconstruct(
        &self,
        segments: &[(usize, usize)],
        key: &str,
        text: &str,
        default: Option<&ConfigValue>,
        visited: &mut Vec<String>,
    ) -> Option<String> {
        let mut sb = String::new();
        let mut start = 0;
        for &(s, e) in segments {
            sb.push_str(&text[start..s]);
            let statement = text[s + 2..e - 1].trim();
            if let Some(evaluated) = self.substitute_var(key, statement, default, visited) {
                sb.push_str(&evaluated);
            }
            start = e;
        }
        sb.push_str(&text[start..]);
        if sb.is_empty() {
            None
        } else {
            Some(sb)
        }
    }

    /// Resolve one `${statement}`: env var → base-config reference (loop-guarded)
    /// → `:default` fallback (Java `performEnvVarSubstitution`).
    fn substitute_var(
        &self,
        key: &str,
        statement: &str,
        default: Option<&ConfigValue>,
        visited: &mut Vec<String>,
    ) -> Option<String> {
        if statement.is_empty() {
            return default.map(|d| d.to_display_string());
        }
        let (name, middle_default) = match statement.find(':') {
            Some(colon) if colon > 0 => (&statement[..colon], Some(&statement[colon + 1..])),
            _ => (statement, None),
        };
        if let Ok(v) = std::env::var(name) {
            return Some(v);
        }
        let from_base = if visited.iter().any(|seen| seen == name) {
            log::warn!("Config loop for '{key}' detected");
            Some(String::new())
        } else {
            visited.push(name.to_string());
            self.base_get(name, default, visited)
                .map(|v| v.to_display_string())
        };
        from_base.or_else(|| middle_default.map(str::to_string))
    }

    /// Normalize the dataset and render `${...}` references
    /// (Java `resolveReferences`).
    fn resolve_references(&mut self) {
        if self.resolved {
            return;
        }
        self.resolved = true;
        let flat = self.map.flat_map();
        // normalization pass — rebuild from sorted flat keys
        self.map = MultiLevelMap::from_flat_map(&flat);
        let has_refs = flat.values().any(|v| match v {
            ConfigValue::Text(t) => {
                let start = t.find(REF_BEGIN);
                let end = t.find('}');
                matches!((start, end), (Some(s), Some(e)) if e > s)
            }
            _ => false,
        });
        if has_refs {
            let mut resolved = MultiLevelMap::new();
            for k in flat.keys() {
                let mut visited = Vec::new();
                let v = self
                    .get_with(k, None, &mut visited)
                    .unwrap_or(ConfigValue::Null);
                resolved.set_element(k, v);
            }
            self.map = resolved;
        }
    }

    // ---- file loading ----

    fn load_into(&mut self, path: &str) -> Result<(), ConfigError> {
        if path.contains("../") {
            // Java getPath: "Relative parent file path not allowed"
            return Err(ConfigError::Invalid(
                "Relative parent file path not allowed".to_string(),
            ));
        }
        let is_yaml = path.ends_with(".yml") || path.ends_with(".yaml");
        // ".yaml" and ".yml" can be used interchangeably
        let alternative = if is_yaml {
            let stem = &path[..path.rfind('.').expect("yaml path has a dot")];
            Some(if path.ends_with(".yml") {
                format!("{stem}.yaml")
            } else {
                format!("{stem}.yml")
            })
        } else {
            None
        };
        let resolved = if path.starts_with(FILEPATH) {
            resolve_file(path, alternative.as_deref())
        } else {
            resolve_classpath_entry(path, alternative.as_deref())
        };
        let Some(file) = resolved else {
            return Err(ConfigError::NotFound(path.to_string()));
        };
        let data = std::fs::read_to_string(&file)?;
        if is_yaml {
            self.load_yaml_text(&data)?;
        } else if path.ends_with(".json") {
            let value: serde_json::Value =
                serde_json::from_str(&data).map_err(|e| ConfigError::Invalid(e.to_string()))?;
            match ConfigValue::from_json(&value) {
                ConfigValue::Map(m) => self.map.reload(m),
                ConfigValue::Null => self.map.reload(BTreeMap::new()),
                _ => {
                    return Err(ConfigError::Invalid(format!(
                        "{path} must contain a JSON object"
                    )))
                }
            }
        } else if path.ends_with(".properties") {
            self.load_properties_text(&data)?;
        } else {
            return Err(ConfigError::Invalid(format!(
                "{path} has an unsupported extension (use .yml, .yaml, .json or .properties)"
            )));
        }
        Ok(())
    }

    /// Parse YAML text (tabs tolerated — replaced with two spaces, a ported quirk).
    fn load_yaml_text(&mut self, data: &str) -> Result<(), ConfigError> {
        let clean = if data.contains('\t') {
            data.replace('\t', "  ")
        } else {
            data.to_string()
        };
        let value: serde_yaml::Value =
            serde_yaml::from_str(&clean).map_err(|e| ConfigError::Invalid(e.to_string()))?;
        match ConfigValue::from_yaml(&value) {
            ConfigValue::Map(m) => self.map.reload(m),
            ConfigValue::Null => self.map.reload(BTreeMap::new()),
            _ => {
                return Err(ConfigError::Invalid(
                    "YAML root must be a mapping".to_string(),
                ))
            }
        }
        Ok(())
    }

    /// Minimal `.properties` support: `key=value` lines, `#`/`!` comments.
    /// Values are strings (Java `Properties` semantics); composite keys expand
    /// into the nested tree via `set_element`, sorted first (Java behavior).
    fn load_properties_text(&mut self, data: &str) -> Result<(), ConfigError> {
        let mut pairs: Vec<(String, String)> = Vec::new();
        for line in data.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('!') {
                continue;
            }
            if let Some(eq) = trimmed.find('=') {
                let key = trimmed[..eq].trim();
                let value = trimmed[eq + 1..].trim();
                if !key.is_empty() {
                    pairs.push((key.to_string(), value.to_string()));
                }
            }
        }
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        for (k, v) in pairs {
            self.map
                .try_set_element(&k, ConfigValue::Text(v))
                .map_err(ConfigError::Invalid)?;
        }
        Ok(())
    }
}

/// Find non-nested `${...}` segments; each result is the byte range including
/// the delimiters (Java `Utility.extractSegments`).
fn extract_segments(text: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0;
    while let Some(rel) = text[i..].find(REF_BEGIN) {
        let start = i + rel;
        match text[start + 2..].find('}') {
            Some(close) => {
                let end = start + 2 + close + 1;
                out.push((start, end));
                i = end;
            }
            None => break,
        }
    }
    out
}

fn resolve_file(path: &str, alternative: Option<&str>) -> Option<PathBuf> {
    let primary = Path::new(&path[FILEPATH.len()..]);
    if primary.is_file() {
        return Some(primary.to_path_buf());
    }
    if let Some(alt) = alternative {
        let secondary = Path::new(&alt[FILEPATH.len()..]);
        if secondary.is_file() {
            return Some(secondary.to_path_buf());
        }
    }
    None
}

fn resolve_classpath_entry(path: &str, alternative: Option<&str>) -> Option<PathBuf> {
    let strip = |p: &str| p.strip_prefix(CLASSPATH).unwrap_or(p).to_string();
    resources::resolve_classpath(&strip(path))
        .or_else(|| alternative.and_then(|alt| resources::resolve_classpath(&strip(alt))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_segments_finds_refs() {
        assert_eq!(extract_segments("no refs"), vec![]);
        assert_eq!(extract_segments("${a}"), vec![(0, 4)]);
        assert_eq!(extract_segments("x${a}y${b:z}"), vec![(1, 5), (6, 12)]);
        assert_eq!(extract_segments("broken ${a"), vec![]);
    }

    #[test]
    fn properties_text_expands_composite_keys() {
        let mut reader = ConfigReader::default();
        reader
            .load_properties_text("# comment\napp.name=mercury\nserver.port=8085\n")
            .unwrap();
        assert_eq!(
            reader.get("app.name"),
            Some(ConfigValue::Text("mercury".into()))
        );
        // properties values are strings, mirroring java.util.Properties
        assert_eq!(
            reader.get("server.port"),
            Some(ConfigValue::Text("8085".into()))
        );
    }

    #[test]
    fn yaml_text_with_tabs_is_tolerated() {
        let mut reader = ConfigReader::default();
        reader.load_yaml_text("hello:\n\tworld: ok\n").unwrap();
        assert_eq!(
            reader.get("hello.world"),
            Some(ConfigValue::Text("ok".into()))
        );
    }
}
