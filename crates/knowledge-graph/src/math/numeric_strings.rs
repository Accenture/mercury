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

//! The numeric-string comparison rule shared by both evaluators (RFC-0001; Java `NumericStrings`).
//!
//! Every value is rendered into the statement text before it is parsed, so the evaluator sees literals and
//! not sources: a substituted string is indistinguishable from one the author typed. A string that is a
//! canonical number therefore compares as a number, which makes `'200' == '200'`, `200 == 200` and
//! `200 == '200'` the same comparison. The comparison is exact: a number operand is taken as the decimal
//! text it prints as, and a string as the decimal it spells, so two distinct 20-digit ids never collapse
//! into equal numbers.

use std::cmp::Ordering;

use bigdecimal::BigDecimal;
use event_script::decimal::{from_f64, parse_canonical};

use super::Value;

/// The exact decimal a value takes in a numeric comparison, or `None` when it cannot take part.
pub(crate) fn comparable(v: &Value) -> Option<BigDecimal> {
    match v {
        Value::Decimal(d) => Some(d.clone()),
        Value::Number(n) => from_f64(*n).ok(),
        Value::Str(s) => parse_canonical(s),
        Value::Bool(_) => None,
    }
}

/// Compare two operands numerically when the rule applies: at least one is a string, and both are numbers
/// or canonical numbers. Number against number is left to the evaluator's own paths, and a string that is
/// not a canonical number, or a boolean, keeps today's behaviour.
pub(crate) fn compare(left: &Value, right: &Value) -> Option<Ordering> {
    if !matches!(left, Value::Str(_)) && !matches!(right, Value::Str(_)) {
        return None;
    }
    Some(comparable(left)?.cmp(&comparable(right)?))
}

/// The outcome of a relational operator (`<`, `<=`, `>`, `>=`) over a comparison result.
pub(crate) fn relation(operator: &str, comparison: Ordering) -> bool {
    match operator {
        "<" => comparison.is_lt(),
        "<=" => comparison.is_le(),
        ">" => comparison.is_gt(),
        ">=" => comparison.is_ge(),
        other => unreachable!("not a relational operator: {other}"),
    }
}
