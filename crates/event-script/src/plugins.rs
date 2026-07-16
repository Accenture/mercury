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

//! Simple-plugin **name registry** — the compile-time half of the Java
//! `SimplePluginLoader`. `CompileFlows` validates every `f:name(args)` input
//! mapping against the registered plugin names, so the names must exist
//! before flows compile. Plugin *execution* (`PluginFunction` bodies and the
//! `#[simple_plugin]` registration macro) ships with design increment E-8;
//! until then the built-in names below are pre-registered so the canonical
//! fixtures — and the legacy `:type` qualifiers the converter rewrites into
//! plugin calls — compile exactly as they do on the Java engine.
//!
//! Divergence (design E6): Java discovers `@SimplePlugin` classes by classpath
//! scan and verifies their **bytecode** against an allowlist; Rust plugins are
//! compiled, linked code — no runtime class loading exists, so the allowlist's
//! threat model does not apply.

use std::collections::HashSet;
use std::sync::{OnceLock, RwLock};

/// Names of the Java built-in plugins (v4.8.6): arithmetic, generators,
/// logical operators, and type conversions. Kept in sync with E-8 as the
/// executable bodies land.
const BUILTIN_NAMES: &[&str] = &[
    // arithmetic
    "add",
    "subtract",
    "multiply",
    "div",
    "mod",
    "increment",
    "decrement",
    // generators
    "now",
    "dateTime",
    "uuid",
    // logical
    "eq",
    "ne",
    "gt",
    "lt",
    "and",
    "or",
    "not",
    "ternary",
    "isNull",
    "notNull",
    "startsWith",
    "endsWith",
    "includes",
    // types & strings
    "text",
    "int",
    "long",
    "float",
    "double",
    "boolean",
    "binary",
    "b64",
    "length",
    "substring",
    "concat",
    "parseDate",
    "parseDateTime",
    "listOfMap",
    "updateListOfMap",
    "removeKey",
    "uniqueSet",
    "defaultValue",
    "validate",
];

fn registry() -> &'static RwLock<HashSet<String>> {
    static PLUGINS: OnceLock<RwLock<HashSet<String>>> = OnceLock::new();
    PLUGINS.get_or_init(|| RwLock::new(BUILTIN_NAMES.iter().map(|s| s.to_string()).collect()))
}

/// True when a plugin name is registered (Java `containsSimplePlugin`).
pub fn contains_simple_plugin(name: &str) -> bool {
    registry().read().expect("plugin registry").contains(name)
}

/// Register a plugin name (the E-8 `#[simple_plugin]` macro will call this).
pub fn register_plugin_name(name: &str) {
    registry()
        .write()
        .expect("plugin registry")
        .insert(name.to_string());
}
