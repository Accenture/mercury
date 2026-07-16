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

/// Parse `-Dkey=value` **runtime arguments** from this process's command line
/// into the override registry — the analog of the JVM's `-D` system
/// properties, e.g. `hello_world -Dlog.format=json`. Idempotent (parsed once);
/// other arguments are left untouched for the application. Called
/// automatically by `logging::init()` and `AppStarter::run()`; call it
/// directly only when reading configuration before either.
pub fn load_runtime_args() {
    use std::sync::OnceLock;
    static LOADED: OnceLock<()> = OnceLock::new();
    LOADED.get_or_init(|| {
        apply_runtime_args(std::env::args().skip(1));
    });
}

/// The testable core of [`load_runtime_args`]: apply every `-Dkey=value`
/// argument from an iterator. Malformed entries (`-Dnokey`, `-D=value`) are
/// ignored; non-`-D` arguments pass through untouched.
pub fn apply_runtime_args<I>(args: I)
where
    I: IntoIterator<Item = String>,
{
    for arg in args {
        if let Some(pair) = arg.strip_prefix("-D") {
            if let Some((key, value)) = pair.split_once('=') {
                if !key.is_empty() {
                    set(key, value);
                }
            }
        }
    }
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

    #[test]
    fn runtime_args_parse_dash_d_pairs() {
        apply_runtime_args(
            [
                "-Dlog.format=json",
                "-Druntime.arg.test=a=b", // value may itself contain '='
                "--not-a-property",
                "-Dmalformed",
                "-D=novalue",
                "plainarg",
            ]
            .into_iter()
            .map(String::from),
        );
        assert_eq!(get("log.format"), Some("json".to_string()));
        assert_eq!(get("runtime.arg.test"), Some("a=b".to_string()));
        assert_eq!(get("malformed"), None);
        clear("log.format");
        clear("runtime.arg.test");
    }
}
