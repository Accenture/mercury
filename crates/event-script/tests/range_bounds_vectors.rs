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

//! The shared conformance vectors of the `minRange` and `maxRange` simple plugins -
//! byte-identical in the Java repository, where the same file drives `SimpleMinRange`
//! and `SimpleMaxRange`, so the two engines type a bound the same way and refuse the
//! same ranges.

use event_script::plugins::{calculate, contains_simple_plugin};
use rmpv::Value;
use serde_json::Value as Json;

const VECTORS: &str = include_str!("resources/range-bounds-vectors.json");

/// A vector value as the plugin receives or answers it: a JSON integer within
/// the 64-bit range an integer, any other number a float, a string text, null Nil.
fn typed(json: &Json) -> Value {
    match json {
        Json::Null => Value::Nil,
        Json::Bool(b) => Value::from(*b),
        Json::Number(n) => match n.as_i64() {
            Some(i) => Value::from(i),
            None => Value::from(n.as_f64().expect("a finite number")),
        },
        Json::String(s) => Value::from(s.as_str()),
        Json::Array(items) => Value::Array(items.iter().map(typed).collect()),
        Json::Object(entries) => Value::Map(
            entries
                .iter()
                .map(|(k, v)| (Value::from(k.as_str()), typed(v)))
                .collect(),
        ),
    }
}

fn fragments(code: &str) -> &'static [&'static str] {
    match code {
        "arity" => &["Expected one or two input values"],
        "bad-range" => &["must be written as min..max, min.. or ..max"],
        "mixed-range" => &["mixes a numeric and a text bound"],
        "empty-range" => &["is empty"],
        other => panic!("unknown error code {other}"),
    }
}

fn vectors() -> Vec<Json> {
    let doc: Json = serde_json::from_str(VECTORS).expect("the vector file parses");
    assert_eq!(doc["format"], "mercury-range-bounds-vectors");
    doc["vectors"].as_array().expect("vectors").clone()
}

#[test]
fn every_vector_holds_in_the_plugins() {
    assert!(contains_simple_plugin("minRange"));
    assert!(contains_simple_plugin("maxRange"));
    let all = vectors();
    assert!(
        all.len() > 30,
        "the vector file looks truncated: {}",
        all.len()
    );
    let mut ids = std::collections::BTreeSet::new();
    for v in &all {
        let id = v["id"].as_str().expect("id");
        assert!(ids.insert(id), "duplicate vector id {id}");
        let plugin = v["plugin"].as_str().expect("plugin");
        let args: Vec<Value> = v["args"]
            .as_array()
            .expect("args")
            .iter()
            .map(typed)
            .collect();
        let actual = calculate(plugin, &args);
        if let Some(code) = v["error"].as_str() {
            let message = actual.expect_err(&format!("{id} must fail with {code}"));
            assert!(
                fragments(code).iter().any(|f| message.contains(f)),
                "{id}: {message}"
            );
        } else {
            assert_eq!(actual, Ok(typed(&v["expect"])), "{id}");
        }
    }
}
