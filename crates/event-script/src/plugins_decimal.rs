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

//! The `f:decimal*` simple plugins (RFC-0001 item 8, ADR-0025): exact decimal arithmetic for flows and
//! mapper nodes, with the same rules as the `DECIMAL:` statement of `graph.math` — both are the
//! [`crate::decimal`] core. A result is a canonical decimal string (plain notation, the computed scale kept,
//! a zero of any scale `"0"`), which survives a serialization boundary unchanged; `decimalCompare` answers
//! `-1`, `0` or `1`. The `f:add` family is unchanged. The shared conformance vectors run in
//! `tests/decimal_plugin_vectors.rs`.

use std::cmp::Ordering;

use bigdecimal::BigDecimal;
use event_script_macros::simple_plugin;
use rmpv::Value;

use crate::decimal;

type Operation = fn(&BigDecimal, &BigDecimal) -> Result<BigDecimal, String>;

fn fold(args: &[Value], name: &str, what: &str, operation: Operation) -> Result<Value, String> {
    if args.len() < 2 {
        return Err(format!("Expected at least two numbers to {what}"));
    }
    let mut total = decimal::operand(&args[0], name)?;
    for arg in &args[1..] {
        total = operation(&total, &decimal::operand(arg, name)?)?;
    }
    Ok(Value::from(decimal::canonical(&total)))
}

fn two_operands(args: &[Value], name: &str) -> Result<(BigDecimal, BigDecimal), String> {
    match args {
        [a, b] => Ok((decimal::operand(a, name)?, decimal::operand(b, name)?)),
        _ => Err(format!("Expected two numbers for {name}")),
    }
}

#[simple_plugin("decimalAdd")]
fn plugin_decimal_add(args: &[Value]) -> Result<Value, String> {
    fold(args, "decimalAdd", "add", decimal::add)
}

#[simple_plugin("decimalSubtract")]
fn plugin_decimal_subtract(args: &[Value]) -> Result<Value, String> {
    fold(args, "decimalSubtract", "subtract", decimal::subtract)
}

#[simple_plugin("decimalMultiply")]
fn plugin_decimal_multiply(args: &[Value]) -> Result<Value, String> {
    fold(args, "decimalMultiply", "multiply", decimal::multiply)
}

#[simple_plugin("decimalDiv")]
fn plugin_decimal_div(args: &[Value]) -> Result<Value, String> {
    let (a, b) = two_operands(args, "decimalDiv")?;
    Ok(Value::from(decimal::canonical(&decimal::divide(
        &a,
        &b,
        "decimalDiv",
    )?)))
}

#[simple_plugin("decimalMod")]
fn plugin_decimal_mod(args: &[Value]) -> Result<Value, String> {
    let (a, b) = two_operands(args, "decimalMod")?;
    Ok(Value::from(decimal::canonical(&decimal::remainder(
        &a,
        &b,
        "decimalMod",
    )?)))
}

/// Rounding is always explicit: `x`, `scale` and `mode` are all required.
#[simple_plugin("decimalRound")]
fn plugin_decimal_round(args: &[Value]) -> Result<Value, String> {
    let [x, scale, mode] = args else {
        return Err(format!(
            "decimalRound takes three arguments, decimalRound(x, scale, mode), got {}",
            args.len()
        ));
    };
    let x = decimal::operand(x, "decimalRound")?;
    let places = decimal::places(&decimal::operand(scale, "decimalRound")?, "decimalRound")?;
    let mode = match mode {
        Value::String(name) => {
            decimal::rounding_mode(name.as_str().unwrap_or_default(), "decimalRound")?
        }
        _ => {
            return Err(format!(
                "The mode of decimalRound must be {}",
                decimal::MODES
            ))
        }
    };
    Ok(Value::from(decimal::canonical(&decimal::round(
        &x, places, mode,
    ))))
}

#[simple_plugin("decimalCompare")]
fn plugin_decimal_compare(args: &[Value]) -> Result<Value, String> {
    let (a, b) = two_operands(args, "decimalCompare")?;
    Ok(Value::from(match decimal::compare(&a, &b) {
        Ordering::Less => -1i64,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }))
}
