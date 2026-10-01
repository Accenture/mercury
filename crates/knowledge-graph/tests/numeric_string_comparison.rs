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

//! A string that is a canonical number compares as a number in COMPUTE, IF and CONDITION (RFC-0001; Java
//! `NumericStringComparisonTest`), so `'200' == '200'`, `200 == 200` and `200 == '200'` are the same comparison. A
//! string that is not a canonical number keeps today's behaviour.

use knowledge_graph::math::ExpressionEngine;

fn is_true(engine: &ExpressionEngine, expression: &str) {
    assert!(engine.eval_boolean(expression).unwrap(), "{expression}");
}

fn is_false(engine: &ExpressionEngine, expression: &str) {
    assert!(!engine.eval_boolean(expression).unwrap(), "{expression}");
}

#[test]
fn the_three_spellings_of_one_comparison_are_the_same() {
    let engine = ExpressionEngine::new();
    for expression in [
        "'200' == '200'",
        "200 == 200",
        "200 == '200'",
        "'200' == 200",
    ] {
        is_true(&engine, expression);
    }
    is_false(&engine, "'200' != 200");
    is_false(&engine, "200 != '200'");
}

#[test]
fn numeric_strings_order_as_numbers_not_as_text() {
    let engine = ExpressionEngine::new();
    is_false(&engine, "'9.5' > '10.25'");
    is_true(&engine, "'9.5' < '10.25'");
    is_false(&engine, "9.5 > '10.25'");
    is_false(&engine, "'9.5' > 10.25");
    is_true(&engine, "'10.25' >= 10.25");
    is_true(&engine, "'-5' < 0");
    is_true(&engine, "'-0' == 0");
}

#[test]
fn equal_numbers_written_differently_are_equal() {
    let engine = ExpressionEngine::new();
    is_true(&engine, "'1.0' == '1'");
    is_true(&engine, "'200' == '200.0'");
    is_true(&engine, "200 == '200.00'");
    is_true(&engine, "'0.30000000000000004' == 0.1 + 0.2");
    is_false(&engine, "'0.3' == 0.1 + 0.2");
}

#[test]
fn the_comparison_is_exact_so_distinct_long_ids_stay_distinct() {
    let engine = ExpressionEngine::new();
    is_false(&engine, "'12345678901234567890' == '12345678901234567891'");
    is_true(&engine, "'12345678901234567890' == '12345678901234567890'");
    is_true(&engine, "'12345678901234567890' < '12345678901234567891'");
}

#[test]
fn a_string_that_is_not_a_canonical_number_keeps_todays_behaviour() {
    let engine = ExpressionEngine::new();
    // leading zeros, a plus sign, an exponent: text
    is_false(&engine, "'007' == '7'");
    is_false(&engine, "'+5' == '5'");
    is_false(&engine, "'1e3' == '1000'");
    is_true(&engine, "'abc' == 'abc'");
    is_true(&engine, "'abc' < 'abd'");
    // a numeric string against text compares as two strings
    is_false(&engine, "'9.5' > 'abc'");
    is_false(&engine, "'200' == 'abc'");
}

#[test]
fn a_number_against_text_is_still_an_error() {
    let engine = ExpressionEngine::new();
    let e = engine.eval_boolean("200 == 'abc'").unwrap_err();
    assert!(e.message().starts_with("Type mismatch for equality"), "{e}");
    assert!(engine.eval_boolean("200 > 'abc'").is_err());
    assert!(engine.eval_boolean("200 == '007'").is_err());
}

#[test]
fn a_boolean_is_still_not_a_number() {
    let engine = ExpressionEngine::new();
    assert!(engine.eval_boolean("true == 'true'").is_err());
    assert!(engine.eval_boolean("true == '1'").is_err());
}

#[test]
fn number_against_number_is_untouched() {
    let engine = ExpressionEngine::new();
    is_true(&engine, "200 == 200");
    is_true(&engine, "1.0 == 1");
    is_false(&engine, "0.1 + 0.2 == 0.3");
    is_true(&engine, "0.1 + 0.2 > 0.3");
}
