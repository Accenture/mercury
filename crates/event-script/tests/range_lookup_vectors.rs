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

//! The shared conformance vectors of the `rangeLookup` simple plugin - byte-identical
//! in the Java repository, where the same file drives `SimpleRangeLookup`, so the two
//! engines place a value in the same band and refuse the same tables.

use event_script::plugins::{calculate, contains_simple_plugin};
use rmpv::Value;
use serde_json::Value as Json;

const VECTORS: &str = include_str!("resources/range-lookup-vectors.json");

/// A vector argument as the plugin receives it: an object is the table (a map
/// of text keys), a number a Long or a Double, a string text, null Nil.
fn argument(json: &Json) -> Value {
    match json {
        Json::Null => Value::Nil,
        Json::Bool(b) => Value::from(*b),
        Json::Number(n) => match n.as_i64() {
            Some(i) => Value::from(i),
            None => Value::from(n.as_f64().expect("a finite number")),
        },
        Json::String(s) => Value::from(s.as_str()),
        Json::Array(items) => Value::Array(items.iter().map(argument).collect()),
        Json::Object(entries) => Value::Map(
            entries
                .iter()
                .map(|(k, v)| (Value::from(k.as_str()), argument(v)))
                .collect(),
        ),
    }
}

fn fragments(code: &str) -> &'static [&'static str] {
    match code {
        "arity" => &["Expected two or three input values"],
        "missing-keys" => &["Missing keys in input"],
        "missing-key" => &["Missing key in input"],
        "bad-range" => &["must be written as min..max, min.. or ..max"],
        "empty-range" => &["is empty"],
        "mixed-table" => &["mixes numeric and text bounds"],
        "not-a-number" => &["Expected a decimal number in rangeLookup", "Cannot convert"],
        "boolean-operand" => &["Boolean operand in rangeLookup"],
        other => panic!("unknown error code {other}"),
    }
}

fn vectors() -> Vec<Json> {
    let doc: Json = serde_json::from_str(VECTORS).expect("the vector file parses");
    assert_eq!(doc["format"], "mercury-range-lookup-vectors");
    doc["vectors"].as_array().expect("vectors").clone()
}

#[test]
fn every_vector_holds_in_the_plugin() {
    assert!(contains_simple_plugin("rangeLookup"));
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
        let args: Vec<Value> = v["args"]
            .as_array()
            .expect("args")
            .iter()
            .map(argument)
            .collect();
        let actual = calculate("rangeLookup", &args);
        if let Some(code) = v["error"].as_str() {
            let message = actual.expect_err(&format!("{id} must fail with {code}"));
            assert!(
                fragments(code).iter().any(|f| message.contains(f)),
                "{id}: {message}"
            );
        } else {
            let expect = argument(&v["expect"]);
            assert_eq!(actual, Ok(expect), "{id}");
        }
    }
}
