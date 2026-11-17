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

//! The limit on the decimal places of a DECIMAL result (ADR-0034; Java `DecimalPrecisionTest`): a result with
//! more decimal places than the limit is rounded HALF_UP to it, one with fewer keeps its computed scale, and
//! the 'precision' setting that states the limit is a whole number from 0 to 1000.

use bigdecimal::BigDecimal;
use knowledge_graph::math::DecimalEvaluator;
use std::str::FromStr;

fn evaluate(expression: &str, max_scale: i64) -> String {
    DecimalEvaluator::evaluate_with_limit(expression, max_scale).expect(expression)
}

#[test]
fn a_longer_result_is_rounded_half_up_to_the_limit() {
    assert_eq!("0.333333", evaluate("1 / 3", 6));
    assert_eq!("0.666667", evaluate("2 / 3", 6));
    assert_eq!(
        "-0.666667",
        evaluate("-2 / 3", 6),
        "half up is away from zero"
    );
    assert_eq!("0.13", evaluate("0.125", 2));
    assert_eq!("-0.13", evaluate("-0.125", 2));
    assert_eq!("3", evaluate("2.5", 0));
    assert_eq!(
        "1.000000",
        evaluate("0.9999999", 6),
        "a carry keeps the limit's scale"
    );
}

#[test]
fn a_shorter_result_keeps_its_computed_scale() {
    assert_eq!("3.0", evaluate("1.5 * 2", 6), "the limit never pads");
    assert_eq!("10.50", evaluate("round(10.5, 2, HALF_UP)", 6));
    assert_eq!("100", evaluate("100", 2));
    assert_eq!(
        "0",
        evaluate("0.0000004", 6),
        "a value that rounds to zero is \"0\""
    );
    assert_eq!("0", evaluate("1.50 - 1.50", 2));
}

#[test]
fn the_limit_applies_to_the_result_not_to_the_arithmetic() {
    // the quotient inside the expression keeps its 34 digits, so 1 / 3 * 3 is 0.99...9 and rounds to one
    assert_eq!("1.000000", evaluate("1 / 3 * 3", 6));
    // an explicit round() to more places than the limit is rounded again to the limit
    assert_eq!("0.333333", evaluate("round(1 / 3, 8, HALF_UP)", 6));
    assert_eq!("0.33333333", evaluate("round(1 / 3, 8, HALF_UP)", 8));
    // the largest limit leaves every result as the unbounded evaluator gives it
    let unbounded = DecimalEvaluator::evaluate("1 / 3").expect("1 / 3");
    assert_eq!(unbounded, evaluate("1 / 3", 1000));
    assert_eq!(
        34,
        BigDecimal::from_str(&unbounded).expect("decimal").digits()
    );
}

#[test]
fn limit_scale_rounds_only_beyond_the_limit() {
    let kept = BigDecimal::from_str("1.50").expect("decimal");
    let same = |value: &BigDecimal, text: &str| {
        assert_eq!(text, value.to_plain_string());
    };
    same(&DecimalEvaluator::limit_scale(kept.clone(), 6), "1.50");
    same(&DecimalEvaluator::limit_scale(kept.clone(), 2), "1.50");
    same(&DecimalEvaluator::limit_scale(kept.clone(), 1), "1.5");
    same(&DecimalEvaluator::limit_scale(kept, 0), "2");
}

#[test]
fn a_precision_setting_is_a_whole_number_from_zero_to_the_maximum_scale() {
    assert_eq!(Ok(6), DecimalEvaluator::precision("6"));
    assert_eq!(Ok(2), DecimalEvaluator::precision(" 2 "));
    assert_eq!(Ok(0), DecimalEvaluator::precision("0"));
    assert_eq!(Ok(1000), DecimalEvaluator::precision("1000"));
    for bad in ["-1", "1001", "two", "2.5", "", "true"] {
        let err = DecimalEvaluator::precision(bad).expect_err(bad);
        assert!(
            err.starts_with("must be a whole number from 0 to 1000, got '"),
            "{err}"
        );
    }
}
