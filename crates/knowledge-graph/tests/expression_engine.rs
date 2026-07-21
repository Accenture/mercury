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

//! Parity port of the Java `ExpressionEngineFullTest` — same expressions,
//! same expected values and error shapes.

use knowledge_graph::math::{EvalContext, ExpressionEngine, MathError};

fn approx(expected: f64, actual: f64) {
    assert!(
        (expected - actual).abs() < 1e-12,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn string_handling() {
    let engine = ExpressionEngine::new();
    assert_eq!(
        "3.0",
        engine.evaluate_value("10 % 3 + 2").unwrap().as_string()
    );
    assert_eq!("true", engine.evaluate_value("10 > 3").unwrap().as_string());
    assert_eq!(
        "hello world",
        engine
            .evaluate_value(" 'hello' + ' world' ")
            .unwrap()
            .as_string()
    );
}

#[test]
fn arithmetic_precedence_and_exponent_right_assoc() {
    let engine = ExpressionEngine::new();
    approx(14.0, engine.eval_number("2 + 3 * 4").unwrap());
    approx(20.0, engine.eval_number("(2 + 3) * 4").unwrap());
    approx(
        2f64.powf(3f64.powf(2.0)),
        engine.eval_number("2 ** 3 ** 2").unwrap(),
    );
    approx(0.56, engine.eval_number("2 * (3 + 4) / 5 ** 2").unwrap());
    approx(1.0, engine.eval_number("10 % 3").unwrap());
}

#[test]
fn strict_js_rule_unary_cannot_be_left_of_exponent() {
    // -2 ** 2 must be a parse error (JS behavior)
    let error = ExpressionEngine::new().eval_number("-2 ** 2").unwrap_err();
    assert!(matches!(error, MathError::Parse(_)));
    assert!(error.message().contains("Unary expression"));
    // parenthesized is OK
    approx(
        -4.0,
        ExpressionEngine::new().eval_number("-(2 ** 2)").unwrap(),
    );
}

#[test]
fn booleans_and_short_circuit() {
    let ctx = EvalContext::with_defaults()
        .define_function("boom", |_| Err(MathError::Eval("boom".to_string())));
    let engine = ExpressionEngine::with_context(ctx);
    assert!(!engine.eval_boolean("0 && boom()").unwrap()); // short-circuit false
    assert!(engine.eval_boolean("1 || boom()").unwrap()); // short-circuit true

    let engine = ExpressionEngine::new();
    // short-circuit happens early without parenthesis
    assert!(engine.eval_boolean("0 || 1 && 1").unwrap());
    // (true) && false -> false
    assert!(!engine.eval_boolean("(0 || 1) && 0").unwrap());
}

#[test]
fn truthiness_for_numbers_and_strings() {
    let engine = ExpressionEngine::new();
    assert!(engine.eval_boolean("3.14").unwrap());
    assert!(!engine.eval_boolean("0").unwrap());
    assert!(engine.eval_boolean("'x'").unwrap());
    assert!(!engine.eval_boolean("''").unwrap());
}

#[test]
fn ternary_returns_branch_value_and_precedence() {
    let engine = ExpressionEngine::new();
    approx(42.0, engine.eval_number("true ? 42 : 7").unwrap());
    approx(7.0, engine.eval_number("false ? 42 : 7").unwrap());
    // (0 || 1) ? 2 : 3 -> 2
    approx(2.0, engine.eval_number("0 || 1 ? 2 : 3").unwrap());
    approx(3.0, engine.eval_number("0 || 0 ? 2 : 3").unwrap());
}

#[test]
fn string_literals_and_comparisons() {
    let engine = ExpressionEngine::new();
    assert!(engine.eval_boolean("('XYZ' > 'ABC' == true)").unwrap());
    // ISO-8601 timestamp comparison
    assert!(engine
        .eval_boolean("'2026-03-02T01:00:01.000Z' > '2026-03-02T01:00:00.001Z' == true")
        .unwrap());
    assert!(engine.eval_boolean("'abc' < 'abd'").unwrap());
    assert!(engine.eval_boolean("'a' == 'a'").unwrap());
    assert!(!engine.eval_boolean("'a' != 'a'").unwrap());
}

#[test]
fn string_concatenation_and_mixed_types() {
    let engine = ExpressionEngine::new();
    // pure strings
    assert!(engine.eval_boolean("('a' + 'b') == 'ab'").unwrap());
    // string + number
    assert!(engine
        .eval_boolean("('answer=' + (1 + 2)) == 'answer=3'")
        .unwrap());
    // number + string
    assert!(engine
        .eval_boolean("((1 + 2) + ' apples') == '3 apples'")
        .unwrap());
    // string + boolean
    assert!(engine.eval_boolean("('x' + true) == 'xtrue'").unwrap());
    // boolean + string
    assert!(engine.eval_boolean("(false + 'y') == 'falsey'").unwrap());
    // combine multiple
    assert!(engine
        .eval_boolean("('A' + 1 + true + 'Z') == 'A1trueZ'")
        .unwrap());
}

#[test]
fn variables_and_context_values_numbers_and_strings() {
    let ctx = EvalContext::with_defaults()
        .define_variable("x", 3.0)
        .define_variable("rate", 0.15)
        .define_string("greet", "Hello");
    let engine = ExpressionEngine::with_context(ctx);
    approx(10.5, engine.eval_number("x ** 2 + 10 * rate").unwrap()); // 9 + 1.5
    assert!(engine
        .eval_boolean("(greet + ', world') == 'Hello, world'")
        .unwrap());
}

#[test]
fn math_functions_and_namespace() {
    let engine = ExpressionEngine::new();
    approx(1.0, engine.eval_number("sin(PI/2)").unwrap());
    approx(1.0, engine.eval_number("Math.sin(Math.PI/2)").unwrap());
    approx(8.0, engine.eval_number("pow(2,3)").unwrap());
}

#[test]
fn numbers_lexing_edge_cases() {
    let engine = ExpressionEngine::new();
    approx(0.5, engine.eval_number(".5").unwrap());
    assert!((123.0 - engine.eval_number(".123e3").unwrap()).abs() < 1e-9);
    approx(1.23e3, engine.eval_number("1.23e3").unwrap());
}

#[test]
fn division_by_zero_and_nan() {
    let engine = ExpressionEngine::new();
    assert_eq!(f64::INFINITY, engine.eval_number("1 / 0.0").unwrap());
    assert_eq!(f64::NEG_INFINITY, engine.eval_number("-1 / 0.0").unwrap());
    // NaN truthiness
    assert!(engine.eval_boolean("!(0/0)").unwrap());
    // NaN == NaN -> false
    assert!(!engine.eval_boolean("(0/0) == (0/0)").unwrap());
}

#[test]
fn simple_math_and_logical_operation() {
    let engine = ExpressionEngine::new();
    approx(
        350.0,
        engine
            .eval_number("(1 + 2 + 3 + 4 + 5 + 6 + 7) * 100 / 8")
            .unwrap(),
    );
    // 175 >= 174
    assert!(engine
        .eval_boolean("(1 + 2 + 3 + 4 + 5 + 6 - 7) * 100 / 8 >= 150 + 24")
        .unwrap());
    // 350 > 345
    assert!(engine
        .eval_boolean("(1 + 2 + 3 + 4 + 5 + 6 + 7) * 100 / 8 > 150 * 2.3")
        .unwrap());
    // 350 <= 360
    assert!(engine
        .eval_boolean("(1 + 2 + 3 + 4 + 5 + 6 + 7) * 100 / 8 < 150 * 2.4")
        .unwrap());
}

#[test]
fn error_cases_unknowns_and_misuse() {
    let engine = ExpressionEngine::new();
    // unknown identifier
    let e1 = engine.eval_number("foo + 1").unwrap_err();
    assert!(matches!(e1, MathError::Eval(_)));
    assert!(e1.message().to_lowercase().contains("unknown identifier"));
    // calling a non-function
    let e2 = engine.eval_number("PI(2)").unwrap_err();
    assert!(matches!(e2, MathError::Eval(_)));
    assert!(e2.message().to_lowercase().contains("non-function"));
    // type mismatch in equality
    let e3 = engine.eval_boolean("'1' == 1").unwrap_err();
    assert!(matches!(e3, MathError::Eval(_)));
    assert!(e3.message().contains("Type mismatch"));
}

#[test]
fn random_and_argument_checks() {
    let engine = ExpressionEngine::new();
    for _ in 0..20 {
        let r = engine.eval_number("random()").unwrap();
        assert!((0.0..1.0).contains(&r));
    }
    // wrong arity messages (Java helper parity)
    let e1 = engine.eval_number("pow(2)").unwrap_err();
    assert_eq!("Function pow expects 2 args, got 1", e1.message());
    let e2 = engine.eval_number("sin(1, 2)").unwrap_err();
    assert_eq!("Expected 1 argument", e2.message());
    // string argument to a numeric function never coerces
    let e3 = engine.eval_number("sqrt('4')").unwrap_err();
    assert_eq!("Cannot coerce string to number: \"4\"", e3.message());
}

/// Increment 57 (parity F23): a CONCATENATED negative zero renders "0" like
/// Java (numberToString goes through BigDecimal, which has no signed zero),
/// while the direct display view keeps "-0.0" — exactly Java's
/// Double.toString. Two surfaces, two Java behaviors, both mirrored.
#[test]
fn negative_zero_renders_like_java() {
    let engine = ExpressionEngine::new();
    assert!(engine.eval_boolean("('n=' + (0 * -1)) == 'n=0'").unwrap());
    assert_eq!("-0.0", engine.evaluate_value("0 * -1").unwrap().as_string());
}
