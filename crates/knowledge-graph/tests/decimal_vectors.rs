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

//! Runs the shared conformance vectors of the DECIMAL statement (RFC-0001; Java `DecimalVectorsTest`). The file
//! is engine-neutral and byte-identical to the Java engine's `decimal-vectors.json`: an expected error is a
//! code, and each engine matches the code against its own message. The plugin vectors
//! (`decimal-plugin-vectors.json`, the `f:decimal*` family) carry the same computation as a DECIMAL expression
//! where one exists, and the statement must agree with them.

use knowledge_graph::math::{DecimalEvaluator, MathError};
use serde_json::Value;

const STATEMENT_VECTORS: &str = include_str!("resources/decimal-vectors.json");
const PLUGIN_VECTORS: &str = include_str!("resources/decimal-plugin-vectors.json");

fn fragment(code: &str) -> &'static str {
    match code {
        "division-by-zero" => "Division by zero",
        "refused-function" | "refused-constant" => "is not available in a DECIMAL statement",
        "whole-number" => "must be a whole number",
        "bound" => "is limited to",
        "round-arity" => "takes three arguments",
        "rounding-mode" => "mode of round() must be",
        "unknown-function" => "Unknown function",
        "unknown-identifier" => "Unknown identifier",
        "identifier-is-function" => "Identifier is a function",
        "boolean-operand" => "Boolean operand",
        "boolean-result" => "Boolean result",
        "not-a-number" => "Expected number",
        "type-mismatch" => "Type mismatch",
        "invalid-number" => "Invalid number",
        other => panic!("unknown error code {other}"),
    }
}

fn vectors(text: &str, format: &str) -> Vec<Value> {
    let doc: Value = serde_json::from_str(text).expect("the vector file parses");
    assert_eq!(doc["format"], format);
    doc["vectors"].as_array().expect("vectors").clone()
}

#[test]
fn every_vector_holds_in_the_decimal_evaluator() {
    let all = vectors(STATEMENT_VECTORS, "mercury-decimal-vectors");
    assert!(
        all.len() > 100,
        "the vector file looks truncated: {}",
        all.len()
    );
    let mut ids = std::collections::HashSet::new();
    for v in &all {
        let id = v["id"].as_str().expect("id");
        assert!(ids.insert(id.to_string()), "duplicate vector id {id}");
        let expression = v["expression"].as_str().expect("expression");
        let outcome = DecimalEvaluator::evaluate(expression);
        if let Some(expect) = v.get("expect") {
            assert_eq!(
                outcome.as_deref(),
                Ok(expect.as_str().expect("expect")),
                "{id}: {expression}"
            );
        } else {
            let code = v["error"].as_str().expect("error code");
            let error = outcome.expect_err(&format!("{id}: {expression} must fail with {code}"));
            if code == "parse-error" {
                assert!(matches!(error, MathError::Parse(_)), "{id}: {error}");
            } else {
                assert!(
                    error.message().contains(fragment(code)),
                    "{id}: '{}' does not match {code}",
                    error.message()
                );
            }
        }
    }
}

#[test]
fn the_decimal_statement_agrees_with_the_plugin_vectors() {
    let mut checked = 0;
    for v in vectors(PLUGIN_VECTORS, "mercury-decimal-plugin-vectors") {
        if let Some(expression) = v.get("expression").and_then(Value::as_str) {
            let id = v["id"].as_str().expect("id");
            let expect = v["expect"].as_str().expect("expect");
            assert_eq!(
                DecimalEvaluator::evaluate(expression).as_deref(),
                Ok(expect),
                "{id}: the DECIMAL statement disagrees with the vector"
            );
            checked += 1;
        }
    }
    assert!(
        checked > 40,
        "too few vectors are cross-checked against the statement: {checked}"
    );
}
