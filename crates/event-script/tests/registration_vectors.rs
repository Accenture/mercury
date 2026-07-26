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

//! Registration-metadata conformance for the PLUGIN kind: the golden vectors
//! in `registration-vectors/plugin.json` are shared VERBATIM with every
//! engine repository. The name rules ARE the contract — an explicit
//! positional name wins, and without one the name derives from the
//! declaration so that idiomatic declarations in every language yield the
//! SAME registered name (Java's `class VectorDerived` and this file's
//! `fn vector_derived` both register "vectorDerived").
//! See docs/guides/registration-metadata-contract.md.

use event_script::plugins;
use event_script::simple_plugin;
use rmpv::Value;

/// Explicit positional name (the fn name is deliberately unrelated).
#[simple_plugin("vectorEcho")]
fn any_fn_name_for_echo(args: &[Value]) -> Result<Value, String> {
    Ok(args.first().cloned().unwrap_or(Value::Nil))
}

/// No name argument: derives the camelCase of the fn name — the
/// cross-language name-rule proof (Java derives the same "vectorDerived"
/// from its class simple name).
#[simple_plugin]
fn vector_derived(_args: &[Value]) -> Result<Value, String> {
    Ok(Value::from("derived"))
}

#[test]
fn plugin_kind_matches_golden_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string("tests/resources/registration-vectors/plugin.json")
            .expect("golden vectors file must exist"),
    )
    .expect("valid vectors json");
    let entries = vectors["entries"].as_array().expect("entries");
    assert_eq!(entries.len(), 2, "vector entry count");
    for expected in entries {
        let name = expected["name"].as_str().expect("name");
        assert!(
            plugins::contains_simple_plugin(name),
            "{name} must be registered under exactly this name"
        );
    }
    // the two fixtures pin both halves of the naming rule
    assert_eq!(
        plugins::calculate("vectorEcho", &[Value::from("x")]),
        Ok(Value::from("x")),
        "explicit positional name wins"
    );
    assert_eq!(
        plugins::calculate("vectorDerived", &[]),
        Ok(Value::from("derived")),
        "derived name: Rust camelCases the snake_case fn name"
    );
}
