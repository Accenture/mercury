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

//! Runs the shared conformance vectors of the `f:decimal*` plugins (RFC-0001 item 8, ADR-0025). The file is
//! engine-neutral and byte-identical to the Java engine's `decimal-plugin-vectors.json`: an expected error is a
//! code, matched here against this engine's own message.

use event_script::plugins::{calculate, contains_simple_plugin};
use rmpv::Value;
use serde_json::Value as Json;

const VECTORS: &str = include_str!("resources/decimal-plugin-vectors.json");

fn argument(json: &Json) -> Value {
    match json {
        Json::Null => Value::Nil,
        Json::Bool(b) => Value::from(*b),
        Json::Number(n) => match n.as_i64() {
            Some(i) => Value::from(i),
            None => Value::from(n.as_f64().expect("a finite number")),
        },
        Json::String(s) => Value::from(s.as_str()),
        other => panic!("unsupported vector argument {other}"),
    }
}

/// The message fragment each engine-neutral error code must show.
fn fragments(code: &str) -> &'static [&'static str] {
    match code {
        "division-by-zero" => &["Division by zero"],
        "round-arity" => &["takes three arguments"],
        "rounding-mode" => &["mode of decimalRound must be"],
        "scale" => &["scale of decimalRound must be"],
        "boolean-operand" => &["Boolean operand"],
        "not-a-number" => &["Cannot convert", "Expected a decimal number"],
        "arity" => &["Expected"],
        other => panic!("unknown error code {other}"),
    }
}

fn vectors() -> Vec<Json> {
    let doc: Json = serde_json::from_str(VECTORS).expect("the vector file parses");
    assert_eq!(doc["format"], "mercury-decimal-plugin-vectors");
    doc["vectors"].as_array().expect("vectors").clone()
}

#[test]
fn every_vector_holds_in_the_plugins() {
    let all = vectors();
    assert!(
        all.len() > 50,
        "the vector file looks truncated: {}",
        all.len()
    );
    let mut ids = std::collections::HashSet::new();
    for v in &all {
        let id = v["id"].as_str().expect("id");
        assert!(ids.insert(id.to_string()), "duplicate vector id {id}");
        let args: Vec<Value> = v["args"]
            .as_array()
            .expect("args")
            .iter()
            .map(argument)
            .collect();
        let outcome = calculate(v["plugin"].as_str().expect("plugin"), &args);
        if let Some(expect) = v.get("expect") {
            let value = outcome.unwrap_or_else(|e| panic!("{id}: unexpected error {e}"));
            match expect {
                Json::String(s) => assert_eq!(value, Value::from(s.as_str()), "{id}"),
                Json::Number(n) => assert_eq!(value.as_i64(), n.as_i64(), "{id}"),
                other => panic!("{id}: unsupported expectation {other}"),
            }
        } else {
            let code = v["error"].as_str().expect("error code");
            let message = outcome.expect_err(&format!("{id} must fail with {code}"));
            assert!(
                fragments(code).iter().any(|f| message.contains(f)),
                "{id}: '{message}' does not match {code}"
            );
        }
    }
}

#[test]
fn the_seven_plugins_register() {
    for name in [
        "decimalAdd",
        "decimalSubtract",
        "decimalMultiply",
        "decimalDiv",
        "decimalMod",
        "decimalRound",
        "decimalCompare",
    ] {
        assert!(contains_simple_plugin(name), "{name} must register");
    }
}

#[test]
fn the_result_is_a_string_so_it_survives_suspend_and_resume() {
    let product = calculate("decimalMultiply", &[Value::from("1.5"), Value::from("2")]).unwrap();
    assert!(product.is_str());
    let quotient = calculate("decimalDiv", &[Value::from(1), Value::from(3)]).unwrap();
    assert!(quotient.is_str());
    // the comparison answers a whole number
    let order = calculate("decimalCompare", &[Value::from("2.0"), Value::from(2)]).unwrap();
    assert_eq!(order.as_i64(), Some(0));
}

#[test]
fn whole_numbers_and_floats_are_operands() {
    assert_eq!(
        calculate("decimalAdd", &[Value::from(1), Value::from(2u64)]).unwrap(),
        Value::from("3")
    );
    assert_eq!(
        calculate("decimalAdd", &[Value::F64(0.1), Value::F64(0.2)]).unwrap(),
        Value::from("0.3")
    );
    assert_eq!(
        calculate("decimalAdd", &[Value::F32(0.1), Value::from(0)]).unwrap(),
        Value::from("0.1")
    );
    assert!(calculate("decimalAdd", &[Value::F64(f64::NAN), Value::from(1)]).is_err());
    assert!(calculate("decimalAdd", &[Value::Nil, Value::from(1)]).is_err());
}
