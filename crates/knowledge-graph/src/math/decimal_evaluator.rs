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

//! The DECIMAL statement's evaluator (RFC-0001, ADR-0025; Java `DecimalEvaluator`): the high-precision
//! `COMPUTE`. A parallel evaluator - the double path in [`super::evaluator`] is untouched - over the shared
//! [`event_script::decimal`] core, which the `f:decimal*` plugins use too. Number literals, substituted numbers
//! included, parse from their text straight into a `BigDecimal`, never through a double.
//!
//! * `+ - *` are exact, with the scales the core gives them (the larger operand scale for `+ -`, the sum of
//!   scales for `*`); a scale is never negative.
//! * `/` never truncates: the exact quotient when it terminates, otherwise 34 significant digits (decimal128)
//!   rounded HALF_EVEN; division by zero is an error naming the operator.
//! * `**` and `pow` take a whole-number exponent within a bound; a fractional one is refused.
//! * Rounding is always explicit: `round(x, scale, mode)`.
//! * What cannot be exact - `sqrt`, `log`, `exp`, trigonometry, `random()` and the constants `PI` and `E` - is
//!   refused by name at run time.
//!
//! A result is a canonical string: plain notation, never scientific, the computed scale kept, and a zero of any
//! scale written `"0"`.

use std::cmp::Ordering;

use bigdecimal::{BigDecimal, ToPrimitive};
use event_script::decimal::{self, canonical, MAX_SCALE};

use super::{eval_err, numeric_strings, parser, Expr, MathError, Value};

// the exponent of ** and pow() is bounded so an expression cannot exhaust memory
const MAX_EXPONENT: i64 = 999;
const MAX_LITERAL_SCALE: i64 = 1000;
const MAX_LITERAL_PRECISION: u64 = 1000;
const MATH: &str = "Math.";
const ROUND: &str = "round";
const REFUSED_FUNCTIONS: [&str; 10] = [
    "sin", "cos", "tan", "asin", "acos", "atan", "sqrt", "log", "log10", "exp",
];
const REFUSED_FUNCTIONS_EXTRA: [&str; 1] = ["random"];
const REFUSED_CONSTANTS: [&str; 2] = ["PI", "E"];
const FUNCTIONS: [&str; 7] = ["abs", "floor", "ceil", "min", "max", "pow", ROUND];

fn refused_function(name: &str) -> bool {
    REFUSED_FUNCTIONS.contains(&name) || REFUSED_FUNCTIONS_EXTRA.contains(&name)
}

/// The DECIMAL statement evaluator.
pub struct DecimalEvaluator;

impl DecimalEvaluator {
    /// Evaluate a DECIMAL statement's expression (the text after its variables were rendered into it).
    pub fn evaluate(expression: &str) -> Result<String, MathError> {
        Ok(canonical(&Self::evaluate_decimal(expression)?))
    }

    /// Evaluate to the exact result, its scale never negative.
    pub fn evaluate_decimal(expression: &str) -> Result<BigDecimal, MathError> {
        let ast = parser::parse_decimal(expression)?;
        let value = eval(&ast)?;
        match value {
            Value::Decimal(d) => Ok(d),
            Value::Str(ref s) => decimal::parse_canonical(s).ok_or_else(|| {
                eval_err(format!(
                    "Expected a number as the result of a DECIMAL statement, got {value}"
                ))
            }),
            Value::Bool(_) => Err(eval_err(format!(
                "Boolean result where a number was expected: {value}"
            ))),
            Value::Number(_) => Err(eval_err("A double in a DECIMAL statement")),
        }
    }

    /// The canonical string of a decimal.
    pub fn canonical(value: &BigDecimal) -> String {
        canonical(value)
    }

    /// The decimal text of a double: the shortest text it prints as, at its minimal scale, in plain notation.
    pub fn plain_text(value: f64) -> Result<String, String> {
        decimal::from_f64(value).map(|d| d.to_plain_string())
    }

    /// The decimal text of a 32-bit float.
    pub fn plain_text_f32(value: f32) -> Result<String, String> {
        decimal::from_f32(value).map(|d| d.to_plain_string())
    }
}

/// A number literal: bounded, and a negative scale (`1e3`) becomes scale 0 (`1000`).
pub(super) fn literal(value: BigDecimal) -> Option<BigDecimal> {
    if value.fractional_digit_count().abs() > MAX_LITERAL_SCALE
        || value.digits() > MAX_LITERAL_PRECISION
    {
        return None;
    }
    Some(decimal::normalize(value))
}

fn checked(value: Result<BigDecimal, String>) -> Result<Value, MathError> {
    value.map(Value::Decimal).map_err(eval_err)
}

fn eval(e: &Expr) -> Result<Value, MathError> {
    match e {
        Expr::DecimalLiteral(d) => Ok(Value::Decimal(d.clone())),
        Expr::NumberLiteral(_) => Err(eval_err("A double literal in a DECIMAL statement")),
        Expr::StringLiteral(s) => Ok(Value::Str(s.clone())),
        Expr::BooleanLiteral(b) => Ok(Value::Bool(*b)),
        Expr::Variable(name) => variable(name),
        Expr::Unary { op, right } => unary(op, right),
        Expr::Binary { op, left, right } => binary(op, left, right),
        Expr::MemberAccess { .. } => member(e),
        Expr::Call { callee, args } => call(callee, args),
        Expr::Conditional {
            test,
            consequent,
            alternate,
        } => {
            if eval(test)?.as_boolean() {
                eval(consequent)
            } else {
                eval(alternate)
            }
        }
    }
}

fn variable(name: &str) -> Result<Value, MathError> {
    if REFUSED_CONSTANTS.contains(&name) {
        return Err(refused_constant(name));
    }
    if FUNCTIONS.contains(&name) || refused_function(name) {
        return Err(eval_err(format!(
            "Identifier is a function, not a value: {name}"
        )));
    }
    Err(eval_err(format!("Unknown identifier: {name}")))
}

fn member(e: &Expr) -> Result<Value, MathError> {
    let name = callee_name(e);
    let bare = name.strip_prefix(MATH).unwrap_or(&name);
    if REFUSED_CONSTANTS.contains(&bare) {
        return Err(refused_constant(&name));
    }
    if FUNCTIONS.contains(&bare) || refused_function(bare) {
        return Err(eval_err("Member is a function; call it with '()'."));
    }
    Err(eval_err("Unknown member access"))
}

fn refused_constant(name: &str) -> MathError {
    eval_err(format!(
        "'{name}' is not available in a DECIMAL statement: it is not an exact decimal; \
         use COMPUTE or a graph.task function"
    ))
}

fn unary(op: &str, right: &Expr) -> Result<Value, MathError> {
    let operand = eval(right)?;
    match op {
        "+" => Ok(Value::Decimal(as_decimal(&operand, "unary '+'")?)),
        "-" => Ok(Value::Decimal(-as_decimal(&operand, "unary '-'")?)),
        "!" => Ok(Value::Bool(!operand.as_boolean())),
        _ => Err(eval_err(format!("Unsupported unary operator: {op}"))),
    }
}

fn binary(op: &str, left: &Expr, right: &Expr) -> Result<Value, MathError> {
    if op == "&&" {
        return Ok(Value::Bool(
            eval(left)?.as_boolean() && eval(right)?.as_boolean(),
        ));
    }
    if op == "||" {
        return Ok(Value::Bool(
            eval(left)?.as_boolean() || eval(right)?.as_boolean(),
        ));
    }
    let l = eval(left)?;
    let r = eval(right)?;
    match op {
        "+" => checked(decimal::add(&as_decimal(&l, op)?, &as_decimal(&r, op)?)),
        "-" => checked(decimal::subtract(
            &as_decimal(&l, op)?,
            &as_decimal(&r, op)?,
        )),
        "*" => checked(decimal::multiply(
            &as_decimal(&l, op)?,
            &as_decimal(&r, op)?,
        )),
        "/" => checked(decimal::divide(
            &as_decimal(&l, op)?,
            &as_decimal(&r, op)?,
            op,
        )),
        "%" => checked(decimal::remainder(
            &as_decimal(&l, op)?,
            &as_decimal(&r, op)?,
            op,
        )),
        "**" => power(&as_decimal(&l, op)?, &as_decimal(&r, op)?, op),
        "<" | "<=" | ">" | ">=" | "==" | "!=" => compare(op, &l, &r),
        _ => Err(eval_err(format!("Unsupported binary operator: {op}"))),
    }
}

fn power(base: &BigDecimal, exponent: &BigDecimal, context: &str) -> Result<Value, MathError> {
    let n = whole_number(exponent, &format!("exponent of '{context}'"), MAX_EXPONENT)?;
    if n >= 0 {
        return checked(check_power(base.powi(n)));
    }
    checked(
        check_power(base.powi(-n)).and_then(|p| decimal::divide(&BigDecimal::from(1), &p, context)),
    )
}

/// `pow` can produce a huge value from a bounded exponent: refuse it before it travels on.
fn check_power(value: BigDecimal) -> Result<BigDecimal, String> {
    if value.digits() > decimal::MAX_RESULT_PRECISION {
        return Err("Arithmetic result too large in 'pow'".to_string());
    }
    Ok(value)
}

/// A number operand for arithmetic: a decimal, or a string that is a canonical number.
fn as_decimal(v: &Value, context: &str) -> Result<BigDecimal, MathError> {
    match v {
        Value::Decimal(d) => return Ok(d.clone()),
        Value::Str(s) => {
            if let Some(d) = decimal::parse_canonical(s) {
                return Ok(d);
            }
        }
        _ => {}
    }
    if matches!(v, Value::Bool(_)) {
        return Err(eval_err(format!("Boolean operand in {context}: {v}")));
    }
    Err(eval_err(format!("Expected number in {context}, got {v}")))
}

fn whole_number(value: &BigDecimal, what: &str, bound: i64) -> Result<i64, MathError> {
    let whole = value.normalized();
    if whole.fractional_digit_count() > 0 {
        return Err(eval_err(format!(
            "The {what} must be a whole number in a DECIMAL statement, got {}",
            canonical(value)
        )));
    }
    let limited = || {
        eval_err(format!(
            "The {what} is limited to {bound}, got {}",
            canonical(value)
        ))
    };
    let n = whole.with_scale(0).to_i32().ok_or_else(limited)? as i64;
    if n.abs() > bound {
        return Err(eval_err(format!(
            "The {what} is limited to {bound}, got {n}"
        )));
    }
    Ok(n)
}

fn compare(op: &str, left: &Value, right: &Value) -> Result<Value, MathError> {
    if let Some(numeric) = compare_numbers(left, right) {
        return Ok(Value::Bool(match op {
            "==" => numeric.is_eq(),
            "!=" => numeric.is_ne(),
            _ => numeric_strings::relation(op, numeric),
        }));
    }
    let equality = op == "==" || op == "!=";
    if let (Value::Str(ls), Value::Str(rs)) = (left, right) {
        // two strings that are not both numbers compare as text, as they do in COMPUTE
        let cmp = ls.as_str().cmp(rs.as_str());
        return Ok(Value::Bool(match op {
            "==" => cmp.is_eq(),
            "!=" => cmp.is_ne(),
            _ => numeric_strings::relation(op, cmp),
        }));
    }
    if equality {
        if let (Value::Bool(lb), Value::Bool(rb)) = (left, right) {
            return Ok(Value::Bool((op == "==") == (lb == rb)));
        }
        return Err(eval_err(format!(
            "Type mismatch for equality: {left} {op} {right}"
        )));
    }
    if matches!(left, Value::Bool(_)) || matches!(right, Value::Bool(_)) {
        let culprit = if matches!(left, Value::Bool(_)) {
            left
        } else {
            right
        };
        return Err(eval_err(format!("Boolean operand in '{op}': {culprit}")));
    }
    let culprit = if numeric_strings::comparable(left).is_none() {
        left
    } else {
        right
    };
    Err(eval_err(format!("Expected number in {op}, got {culprit}")))
}

/// Two decimals, a decimal and a canonical string, or two canonical strings compare exactly.
fn compare_numbers(left: &Value, right: &Value) -> Option<Ordering> {
    if let (Value::Decimal(a), Value::Decimal(b)) = (left, right) {
        return Some(a.cmp(b));
    }
    Some(numeric_strings::comparable(left)?.cmp(&numeric_strings::comparable(right)?))
}

fn call(callee: &Expr, args: &[Expr]) -> Result<Value, MathError> {
    let name = function_name(callee)?;
    match name.as_str() {
        ROUND => round(args),
        "min" | "max" => min_max(&name, args),
        "pow" => {
            arity(&name, args, 2)?;
            power(
                &argument(&name, &args[0])?,
                &argument(&name, &args[1])?,
                &name,
            )
        }
        _ => {
            arity(&name, args, 1)?;
            let x = argument(&name, &args[0])?;
            Ok(Value::Decimal(match name.as_str() {
                "abs" => x.abs(),
                "floor" => {
                    decimal::normalize(x.with_scale_round(0, bigdecimal::RoundingMode::Floor))
                }
                _ => decimal::normalize(x.with_scale_round(0, bigdecimal::RoundingMode::Ceiling)),
            }))
        }
    }
}

/// The name of a supported function, with any `Math.` prefix removed; a refused or unknown one fails by name.
fn function_name(callee: &Expr) -> Result<String, MathError> {
    if !matches!(callee, Expr::Variable(_) | Expr::MemberAccess { .. }) {
        return Err(eval_err("Unknown function: expression"));
    }
    let full = callee_name(callee);
    let name = full.strip_prefix(MATH).unwrap_or(&full).to_string();
    if refused_function(&name) {
        return Err(eval_err(format!(
            "{name}() is not available in a DECIMAL statement: the result cannot be exact; \
             use COMPUTE or a graph.task function"
        )));
    }
    if !FUNCTIONS.contains(&name.as_str()) {
        // a misspelled or unsupported function is rejected by name, never a silent no-op
        return Err(eval_err(format!("Unknown function: {full}")));
    }
    Ok(name)
}

fn arity(name: &str, args: &[Expr], expected: usize) -> Result<(), MathError> {
    if args.len() != expected {
        return Err(eval_err(format!(
            "Function {name} expects {expected} args, got {}",
            args.len()
        )));
    }
    Ok(())
}

fn argument(function: &str, arg: &Expr) -> Result<BigDecimal, MathError> {
    as_decimal(&eval(arg)?, &format!("argument of {function}()"))
}

fn min_max(name: &str, args: &[Expr]) -> Result<Value, MathError> {
    if args.is_empty() {
        return Err(eval_err(format!(
            "Function {name} needs at least one argument"
        )));
    }
    let mut values = Vec::with_capacity(args.len());
    for arg in args {
        values.push(argument(name, arg)?);
    }
    let mut best = values[0].clone();
    for value in values {
        // the first of equal values wins, so the result is the same in every engine
        let cmp = value.cmp(&best);
        if (name == "min" && cmp.is_lt()) || (name == "max" && cmp.is_gt()) {
            best = value;
        }
    }
    Ok(Value::Decimal(best))
}

fn round(args: &[Expr]) -> Result<Value, MathError> {
    if args.len() != 3 {
        return Err(eval_err(format!(
            "round() takes three arguments in a DECIMAL statement, round(x, scale, mode), \
             where mode is {}; got {}",
            decimal::MODES,
            args.len()
        )));
    }
    let x = argument(ROUND, &args[0])?;
    let scale = whole_number(&argument(ROUND, &args[1])?, "scale of round()", MAX_SCALE)?;
    let mode = rounding_mode(&args[2])?;
    Ok(Value::Decimal(decimal::round(&x, scale, mode)))
}

fn rounding_mode(arg: &Expr) -> Result<bigdecimal::RoundingMode, MathError> {
    // the mode is a name, written bare (HALF_UP) or quoted ('HALF_UP')
    let name = match arg {
        Expr::Variable(n) => Some(n.as_str()),
        Expr::StringLiteral(s) => Some(s.as_str()),
        _ => None,
    };
    name.and_then(|n| decimal::rounding_mode(n, "round()").ok())
        .ok_or_else(|| eval_err(format!("The mode of round() must be {}", decimal::MODES)))
}

fn callee_name(e: &Expr) -> String {
    match e {
        Expr::Variable(name) => name.clone(),
        Expr::MemberAccess { target, property } => format!("{}.{property}", callee_name(target)),
        _ => "expression".to_string(),
    }
}
