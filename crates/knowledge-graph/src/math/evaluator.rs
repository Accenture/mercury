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

//! AST evaluation (Java `Evaluator`): short-circuit logic, string
//! concatenation with JS-like number rendering, string/number relational
//! comparison, same-type-only equality (NaN never equals NaN).

use super::context::{ContextValue, EvalContext};
use super::{eval_err, Expr, MathError, Value};

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // the Java record toString forms, used in error messages
        match self {
            Value::Number(n) => write!(f, "Number({})", super::java_double_to_string(*n)),
            Value::Bool(b) => write!(f, "Boolean({b})"),
            Value::Str(s) => write!(f, "String({s})"),
        }
    }
}

pub(crate) fn eval(e: &Expr, ctx: &EvalContext) -> Result<Value, MathError> {
    match e {
        Expr::NumberLiteral(n) => Ok(Value::Number(*n)),
        Expr::StringLiteral(s) => Ok(Value::Str(s.clone())),
        Expr::BooleanLiteral(b) => Ok(Value::Bool(*b)),
        Expr::Variable(name) => eval_var(name, ctx),
        Expr::Unary { op, right } => eval_unary(op, right, ctx),
        Expr::Binary { op, left, right } => eval_binary(op, left, right, ctx),
        Expr::MemberAccess { .. } => eval_member_access(e, ctx),
        Expr::Call { callee, args } => eval_call(callee, args, ctx),
        Expr::Conditional {
            test,
            consequent,
            alternate,
        } => {
            if eval(test, ctx)?.as_boolean() {
                eval(consequent, ctx)
            } else {
                eval(alternate, ctx)
            }
        }
    }
}

fn eval_var(name: &str, ctx: &EvalContext) -> Result<Value, MathError> {
    match ctx.lookup(name) {
        None => Err(eval_err(format!("Unknown identifier: {name}"))),
        Some(ContextValue::Number(n)) => Ok(Value::Number(*n)),
        Some(ContextValue::Str(s)) => Ok(Value::Str(s.clone())),
        Some(ContextValue::Function(_)) => Err(eval_err(format!(
            "Identifier is a function, not a value: {name}"
        ))),
        Some(ContextValue::Namespace(_)) => {
            Err(eval_err(format!("Unsupported variable type for: {name}")))
        }
    }
}

fn eval_unary(op: &str, right: &Expr, ctx: &EvalContext) -> Result<Value, MathError> {
    let r = eval(right, ctx)?;
    match op {
        "+" => Ok(Value::Number(finite(
            as_number(&r, "unary '+'")?,
            "unary '+'",
        )?)),
        "-" => Ok(Value::Number(finite(
            -as_number(&r, "unary '-'")?,
            "unary '-'",
        )?)),
        "!" => Ok(Value::Bool(!r.as_boolean())),
        _ => Err(eval_err(format!("Unsupported unary operator: {op}"))),
    }
}

fn eval_call(callee: &Expr, args: &[Expr], ctx: &EvalContext) -> Result<Value, MathError> {
    let name = callee_name(callee);
    let fn_obj = match callee {
        Expr::MemberAccess { .. } | Expr::Variable(_) => resolve_object(callee, ctx),
        _ => None,
    };
    let function = match fn_obj {
        // a misspelled or unsupported function is rejected by name, never a silent no-op
        None => return Err(eval_err(format!("Unknown function: {name}"))),
        Some(ContextValue::Function(function)) => function,
        Some(_) => return Err(eval_err(format!("'{name}' is not a function"))),
    };
    let context = format!("argument of {name}()");
    let mut arg_values = Vec::with_capacity(args.len());
    for arg in args {
        arg_values.push(as_number(&eval(arg, ctx)?, &context)?);
    }
    let result = function(&arg_values)?;
    Ok(Value::Number(finite(result, &format!("{name}()"))?))
}

fn callee_name(e: &Expr) -> String {
    match e {
        Expr::Variable(name) => name.clone(),
        Expr::MemberAccess { target, property } => format!("{}.{property}", callee_name(target)),
        _ => "expression".to_string(),
    }
}

fn eval_binary(op: &str, left: &Expr, right: &Expr, ctx: &EvalContext) -> Result<Value, MathError> {
    // short-circuit boolean ops
    if op == "&&" {
        let l = eval(left, ctx)?;
        if !l.as_boolean() {
            return Ok(Value::Bool(false));
        }
        return Ok(Value::Bool(eval(right, ctx)?.as_boolean()));
    }
    if op == "||" {
        let l = eval(left, ctx)?;
        if l.as_boolean() {
            return Ok(Value::Bool(true));
        }
        return Ok(Value::Bool(eval(right, ctx)?.as_boolean()));
    }
    // evaluate operands once for non-short-circuit ops
    let lv = eval(left, ctx)?;
    let rv = eval(right, ctx)?;
    // string concatenation if either side is string
    if op == "+" && (matches!(lv, Value::Str(_)) || matches!(rv, Value::Str(_))) {
        return Ok(Value::Str(format!("{}{}", as_string(&lv), as_string(&rv))));
    }
    // numeric arithmetic
    match op {
        "+" => {
            return Ok(Value::Number(finite(
                as_number(&lv, op)? + as_number(&rv, op)?,
                op,
            )?))
        }
        "-" => {
            return Ok(Value::Number(finite(
                as_number(&lv, op)? - as_number(&rv, op)?,
                op,
            )?))
        }
        "*" => {
            return Ok(Value::Number(finite(
                as_number(&lv, op)? * as_number(&rv, op)?,
                op,
            )?))
        }
        "/" => {
            return Ok(Value::Number(finite(
                as_number(&lv, op)? / as_number(&rv, op)?,
                op,
            )?))
        }
        "%" => {
            return Ok(Value::Number(finite(
                as_number(&lv, op)? % as_number(&rv, op)?,
                op,
            )?))
        }
        "**" => {
            let powered = as_number(&lv, op)?.powf(as_number(&rv, op)?);
            return Ok(Value::Number(finite(powered, op)?));
        }
        _ => {}
    }
    // relational / equality with string and number support
    match op {
        "<" | "<=" | ">" | ">=" => {
            let outcome = if let (Value::Str(ls), Value::Str(rs)) = (&lv, &rv) {
                let cmp = ls.as_str().cmp(rs.as_str());
                match op {
                    "<" => cmp.is_lt(),
                    "<=" => cmp.is_le(),
                    ">" => cmp.is_gt(),
                    _ => cmp.is_ge(),
                }
            } else {
                let l = as_number(&lv, op)?;
                let r = as_number(&rv, op)?;
                match op {
                    "<" => l < r,
                    "<=" => l <= r,
                    ">" => l > r,
                    _ => l >= r,
                }
            };
            Ok(Value::Bool(outcome))
        }
        "==" | "!=" => {
            let eq = match (&lv, &rv) {
                (Value::Str(l), Value::Str(r)) => l == r,
                (Value::Bool(l), Value::Bool(r)) => l == r,
                (Value::Number(l), Value::Number(r)) => l == r, // NaN == NaN -> false
                _ => {
                    return Err(eval_err(format!(
                        "Type mismatch for equality: {lv} {op} {rv}"
                    )));
                }
            };
            Ok(Value::Bool((op == "==") == eq))
        }
        _ => Err(eval_err(format!("Unsupported binary operator: {op}"))),
    }
}

fn eval_member_access(e: &Expr, ctx: &EvalContext) -> Result<Value, MathError> {
    match resolve_object(e, ctx) {
        Some(ContextValue::Number(n)) => Ok(Value::Number(*n)),
        Some(ContextValue::Function(_)) => {
            Err(eval_err("Member is a function; call it with '()'."))
        }
        None => Err(eval_err("Unknown member access")),
        Some(_) => Err(eval_err("Unsupported member type in member access")),
    }
}

fn resolve_object<'a>(e: &Expr, ctx: &'a EvalContext) -> Option<&'a ContextValue> {
    match e {
        Expr::Variable(name) => ctx.lookup(name),
        Expr::MemberAccess { target, property } => {
            ctx.member(resolve_object(target, ctx), property)
        }
        _ => None,
    }
}

/// A number for arithmetic, a relational comparison or a function argument. A
/// boolean is never a number here - one uniform rejection, whichever operator
/// met it - so a JSON boolean that reached a numeric slot fails by name instead
/// of computing as 1 or 0 (equality already type-checks). Java `asNumber`.
fn as_number(v: &Value, context: &str) -> Result<f64, MathError> {
    match v {
        Value::Number(n) => Ok(*n),
        Value::Bool(_) => Err(eval_err(format!("Boolean operand in '{context}': {v}"))),
        Value::Str(_) => Err(eval_err(format!("Expected number in {context}, got {v}"))),
    }
}

/// Arithmetic stays finite: a double that overflowed to infinity or lost
/// meaning (NaN) is an error naming the operator, never a value that travels
/// on to render as the identifier `Infinity` or a settled amount. Java `finite`.
fn finite(d: f64, context: &str) -> Result<f64, MathError> {
    if d.is_nan() {
        return Err(eval_err(format!(
            "Arithmetic result is not a number (NaN) in '{context}'"
        )));
    }
    if d.is_infinite() {
        let cause = if context == "/" || context == "%" {
            "Division by zero or arithmetic overflow"
        } else {
            "Arithmetic overflow"
        };
        let rendered = if d > 0.0 { "Infinity" } else { "-Infinity" };
        return Err(eval_err(format!(
            "{cause} in '{context}' (result {rendered})"
        )));
    }
    Ok(d)
}

/// String view for concatenation (Java `asString`): numbers render in the
/// JS-like minimal form — integral doubles without a decimal point.
fn as_string(v: &Value) -> String {
    match v {
        Value::Str(s) => s.clone(),
        Value::Number(n) => number_to_string(*n),
        Value::Bool(b) => b.to_string(),
    }
}

fn number_to_string(d: f64) -> String {
    if d.is_nan() {
        "NaN".to_string()
    } else if d.is_infinite() {
        if d > 0.0 { "Infinity" } else { "-Infinity" }.to_string()
    } else if d == 0.0 {
        // BigDecimal has no signed zero: -0.0 renders "0" like Java
        // (increment 57, parity F23 — Rust's Display keeps the sign)
        "0".to_string()
    } else {
        // Rust's Display is already the minimal plain-decimal form
        // (BigDecimal stripTrailingZeros + toPlainString in Java)
        format!("{d}")
    }
}
