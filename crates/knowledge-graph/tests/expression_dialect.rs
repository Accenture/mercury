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

//! Claims-registry engine pin (ADR-0023) — `math-expression-dialect`: the
//! graph.math expression dialect is EXACTLY what the skills reference documents
//! (`skills-reference.md#math-dialect`) — the operator set, the eighteen
//! built-in functions (each also reachable under the `Math` namespace), the two
//! constants `PI` and `E`, and nothing else. A fresh AI agent generates
//! expressions from that list alone, so the list and the engine must not drift
//! apart in either direction.
//!
//! The set-equality pin fails when a function or constant is ADDED or REMOVED;
//! the behavioral checks tie each documented operator and arity to the
//! evaluator, one cohesive group per test; the negative checks pin that the
//! documented "not in the dialect" forms really are rejected. Twin of the Java
//! `ClaimMathExpressionDialectTest`.

use knowledge_graph::math::{EvalContext, ExpressionEngine, MathError};

/// The eighteen documented functions, in the order the skills reference lists them.
const FUNCTIONS: &str =
    "sin cos tan asin acos atan sqrt abs floor ceil round log log10 exp min max pow random";

/// The two documented constants.
const CONSTANTS: &str = "PI E";

fn approx(expected: f64, actual: f64) {
    assert!(
        (expected - actual).abs() < 1e-12,
        "expected {expected}, got {actual}"
    );
}

fn documented_names(with_namespace: bool) -> Vec<String> {
    let mut names: Vec<String> = FUNCTIONS
        .split_whitespace()
        .chain(CONSTANTS.split_whitespace())
        .map(str::to_string)
        .collect();
    if with_namespace {
        // the namespace object that mirrors every function and constant
        names.push("Math".to_string());
    }
    names.sort();
    names
}

fn eval_error(engine: &ExpressionEngine, expr: &str) -> MathError {
    match engine.evaluate_value(expr) {
        Err(e) => e,
        Ok(v) => panic!("'{expr}' must fail, got {v:?}"),
    }
}

fn number_error(engine: &ExpressionEngine, expr: &str) -> String {
    match engine.eval_number(expr) {
        Err(e) => e.message().to_string(),
        Ok(v) => panic!("'{expr}' must fail, got {v}"),
    }
}

#[test]
fn the_dialect_is_exactly_the_documented_functions_and_constants() {
    let ctx = EvalContext::with_defaults();
    assert_eq!(
        ctx.names(),
        documented_names(true),
        "the graph.math dialect must expose exactly the documented functions and constants - \
         adding or removing one changes the documented contract (claims-registry: \
         math-expression-dialect; update skills-reference.md#math-dialect, help graph-math.md \
         and minigraph-commands.json together)"
    );
    // every top-level function and constant is mirrored under Math.* - and nothing else is
    let mirrored = ctx.namespace_names("Math");
    assert_eq!(mirrored, Some(documented_names(false)));
    assert_eq!(
        ctx.namespace_names("PI"),
        None,
        "a constant is not a namespace"
    );
    let engine = ExpressionEngine::new();
    for name in FUNCTIONS.split_whitespace() {
        // a function value used without '()' is rejected by name, never read as a number
        let message = number_error(&engine, name);
        assert!(
            message.contains("function"),
            "{name} must be a function: {message}"
        );
    }
    for name in CONSTANTS.split_whitespace() {
        assert!(
            engine.eval_number(name).is_ok(),
            "{name} must be a numeric constant"
        );
    }
}

#[test]
fn one_argument_functions_evaluate() {
    let engine = ExpressionEngine::new();
    approx(0.0, engine.eval_number("sin(0)").unwrap());
    approx(1.0, engine.eval_number("cos(0)").unwrap());
    approx(0.0, engine.eval_number("tan(0)").unwrap());
    approx(
        std::f64::consts::FRAC_PI_2,
        engine.eval_number("asin(1)").unwrap(),
    );
    approx(0.0, engine.eval_number("acos(1)").unwrap());
    approx(
        std::f64::consts::FRAC_PI_4,
        engine.eval_number("atan(1)").unwrap(),
    );
    approx(4.0, engine.eval_number("sqrt(16)").unwrap());
    approx(2.5, engine.eval_number("abs(-2.5)").unwrap());
    approx(2.0, engine.eval_number("floor(2.9)").unwrap());
    approx(3.0, engine.eval_number("ceil(2.1)").unwrap());
    // round is half up toward positive infinity (Java Math.round)
    approx(3.0, engine.eval_number("round(2.5)").unwrap());
    approx(-2.0, engine.eval_number("round(-2.5)").unwrap());
    // log is the natural logarithm
    approx(1.0, engine.eval_number("log(E)").unwrap());
    approx(3.0, engine.eval_number("log10(1000)").unwrap());
    approx(std::f64::consts::E, engine.eval_number("exp(1)").unwrap());
}

#[test]
fn variadic_two_argument_and_zero_argument_functions_evaluate() {
    let engine = ExpressionEngine::new();
    approx(1.0, engine.eval_number("min(3, 1, 2)").unwrap());
    approx(3.0, engine.eval_number("max(3, 1, 2)").unwrap());
    approx(1024.0, engine.eval_number("pow(2, 10)").unwrap());
    let r = engine.eval_number("random()").unwrap();
    assert!((0.0..1.0).contains(&r), "random() is in [0, 1)");
}

#[test]
fn constants_and_the_math_namespace_evaluate() {
    let engine = ExpressionEngine::new();
    approx(std::f64::consts::PI, engine.eval_number("PI").unwrap());
    approx(std::f64::consts::E, engine.eval_number("Math.E").unwrap());
    approx(8.0, engine.eval_number("Math.pow(2, 3)").unwrap());
    approx(1.0, engine.eval_number("Math.sin(Math.PI / 2)").unwrap());
}

#[test]
fn a_wrong_arity_fails_by_name() {
    // never a silent default
    let engine = ExpressionEngine::new();
    let e1 = number_error(&engine, "pow(2)");
    assert!(e1.contains("pow expects 2 args"), "{e1}");
    let e2 = number_error(&engine, "sqrt(4, 9)");
    assert!(e2.contains("Expected 1 argument"), "{e2}");
    let e3 = number_error(&engine, "random(1)");
    assert!(e3.contains("random expects 0 args"), "{e3}");
}

#[test]
fn arithmetic_operators_are_accepted() {
    let engine = ExpressionEngine::new();
    // exponent: right-associative, binds tighter than unary minus - the strict JS rule
    approx(512.0, engine.eval_number("2 ** 3 ** 2").unwrap());
    approx(-4.0, engine.eval_number("-(2 ** 2)").unwrap());
    let strict = eval_error(&engine, "-2 ** 2");
    assert!(matches!(strict, MathError::Parse(_)), "{strict}");
    // unary, multiplicative (incl. remainder) and additive
    approx(-3.0, engine.eval_number("-3").unwrap());
    approx(3.0, engine.eval_number("+3").unwrap());
    approx(1.0, engine.eval_number("7 % 3").unwrap());
    approx(14.0, engine.eval_number("2 + 3 * 4").unwrap());
    approx(20.0, engine.eval_number("(2 + 3) * 4").unwrap());
    approx(2.5, engine.eval_number("10 / 4").unwrap());
    // '+' concatenates when either side is a string
    let concatenated = engine.evaluate_value("'id-' + 7").unwrap().as_string();
    assert_eq!("id-7", concatenated);
}

#[test]
fn comparison_logical_and_ternary_operators_are_accepted() {
    let engine = ExpressionEngine::new();
    // relational on numbers and on two strings (lexical - ISO-8601 timestamps compare correctly)
    let relational = engine
        .eval_boolean("1 < 2 && 2 <= 2 && 3 > 2 && 3 >= 3")
        .unwrap();
    assert!(relational);
    let timestamps = engine.eval_boolean("'2026-03-02T01:00:01Z' > '2026-03-02T01:00:00Z'");
    assert!(timestamps.unwrap());
    // equality is same-type only
    let equality = engine.eval_boolean("5 == 5.0 && 1 != 2 && 'a' == 'a' && true == true");
    assert!(equality.unwrap());
    let mismatch = eval_error(&engine, "'1' == 1");
    assert!(
        mismatch.message().contains("Type mismatch for equality"),
        "{mismatch}"
    );
    // logical not / and / or (short-circuit) and the ternary
    assert!(engine.eval_boolean("!false && (false || true)").unwrap());
    approx(1.0, engine.eval_number("2 > 1 ? 1 : 0").unwrap());
    approx(0.0, engine.eval_number("2 < 1 ? 1 : 0").unwrap());
}

#[test]
fn literals_are_accepted() {
    // integer, decimal, leading-dot and exponent numbers; single- and double-quoted strings; booleans
    let engine = ExpressionEngine::new();
    approx(0.5, engine.eval_number(".5").unwrap());
    approx(1230.0, engine.eval_number("1.23e3").unwrap());
    let single_quoted = engine.evaluate_value("'a\"b'").unwrap().as_string();
    assert_eq!("a\"b", single_quoted);
    let double_quoted = engine.evaluate_value("\"a'b\"").unwrap().as_string();
    assert_eq!("a'b", double_quoted);
    assert!(engine.eval_boolean("true").unwrap());
    assert!(!engine.eval_boolean("false").unwrap());
}

#[test]
fn the_documented_exclusions_are_rejected() {
    let engine = ExpressionEngine::new();
    // no bitwise or shift operators, no assignment, no user identifiers, no user-defined functions
    for expr in ["1 & 2", "1 | 2", "1 ^ 2", "~1", "1 << 2", "x = 1"] {
        let rejected = eval_error(&engine, expr);
        assert!(
            matches!(rejected, MathError::Parse(_)),
            "'{expr}' must not parse: {rejected}"
        );
    }
    let e1 = number_error(&engine, "total + 1");
    assert!(e1.starts_with("Unknown identifier: total"), "{e1}");
    assert_eq!(
        "Unknown function: hypot",
        number_error(&engine, "hypot(3, 4)")
    );
    let namespaced = number_error(&engine, "Math.hypot(3, 4)");
    assert_eq!("Unknown function: Math.hypot", namespaced);
}
