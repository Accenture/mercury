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

//! Process-level configuration overrides — the Rust analog of Java's
//! `System.getProperty` / `-D` parameters, which the Java `ConfigReader`
//! consults *before* any configuration file value.
//!
//! Tests and applications set overrides programmatically:
//! `overrides::set("server.port", "9999")`.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

fn registry() -> &'static RwLock<HashMap<String, String>> {
    static REGISTRY: OnceLock<RwLock<HashMap<String, String>>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Set a process-level override (the `-D` analog). Checked first by every
/// `ConfigReader` lookup, exactly like Java's `System.getProperty(key)`.
pub fn set(key: &str, value: &str) {
    registry()
        .write()
        .expect("override registry poisoned")
        .insert(key.to_string(), value.to_string());
}

/// Get a process-level override, if set.
pub fn get(key: &str) -> Option<String> {
    if key.is_empty() {
        return None; // Java: getSystemProperty("") -> null
    }
    registry()
        .read()
        .expect("override registry poisoned")
        .get(key)
        .cloned()
}

/// Remove one override.
pub fn clear(key: &str) {
    registry()
        .write()
        .expect("override registry poisoned")
        .remove(key);
}

/// Remove all overrides (test hygiene).
pub fn clear_all() {
    registry()
        .write()
        .expect("override registry poisoned")
        .clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_clear() {
        set("unit.test.key", "value-1");
        assert_eq!(get("unit.test.key"), Some("value-1".to_string()));
        clear("unit.test.key");
        assert_eq!(get("unit.test.key"), None);
        assert_eq!(get(""), None);
    }
}
