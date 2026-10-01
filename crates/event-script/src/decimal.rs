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

//! Exact decimal arithmetic (RFC-0001, ADR-0025) — the one implementation behind the `f:decimal*`
//! simple plugins and, in `knowledge-graph`, the `DECIMAL:` statement of `graph.math`, so a flow and
//! a graph compute the same answer. The Java engine keeps two implementations and cross-checks them;
//! here the core is shared and the conformance vectors run against it.
//!
//! A result is a **canonical decimal string**: plain notation, never scientific, the computed scale
//! kept, and a zero of any scale written `"0"` — the form that survives a serialization boundary
//! unchanged. An operand is a whole number, a float (taken through the shortest decimal text it
//! prints as), or a string that is a canonical number; anything else is an error naming it.

use std::cmp::Ordering;
use std::str::FromStr;

use bigdecimal::num_bigint::{BigInt, Sign};
use bigdecimal::{BigDecimal, RoundingMode, Zero};
use rmpv::Value;

/// A canonical-number string longer than this is refused (bounds the work a hostile operand can ask).
const MAX_LENGTH: usize = 1000;
/// The largest `scale` of a rounding.
pub const MAX_SCALE: i64 = 1000;
/// A result with more significant digits than this is refused.
pub const MAX_RESULT_PRECISION: u64 = 10_000;
/// Significant digits of a quotient that does not terminate (decimal128).
const DIVISION_DIGITS: i64 = 34;
/// The rounding modes `round(x, scale, mode)` accepts.
pub const MODES: &str = "HALF_UP, HALF_EVEN, HALF_DOWN, UP, DOWN, CEILING or FLOOR";

/// The canonical string of a decimal: plain notation, the computed scale kept, a zero of any scale `"0"`.
pub fn canonical(value: &BigDecimal) -> String {
    if value.is_zero() {
        "0".to_string()
    } else {
        normalize(value.clone()).to_plain_string()
    }
}

/// A negative scale (`1e3`) becomes scale 0 (`1000`): a scale is never negative.
pub fn normalize(value: BigDecimal) -> BigDecimal {
    if value.fractional_digit_count() < 0 {
        value.with_scale(0)
    } else {
        value
    }
}

/// Whether a string is a canonical number: plain notation, an optional minus sign, digits without leading
/// zeros, an optional fraction (`1e3`, `007`, `+1` and `.5` are not).
pub fn is_canonical(text: &str) -> bool {
    let digits = text.strip_prefix('-').unwrap_or(text);
    let (whole, fraction) = match digits.split_once('.') {
        Some((whole, fraction)) => (whole, Some(fraction)),
        None => (digits, None),
    };
    let whole_ok = !whole.is_empty()
        && whole.bytes().all(|b| b.is_ascii_digit())
        && (whole == "0" || !whole.starts_with('0'));
    let fraction_ok =
        fraction.is_none_or(|f| !f.is_empty() && f.bytes().all(|b| b.is_ascii_digit()));
    whole_ok && fraction_ok
}

/// The exact decimal a canonical string spells, or `None` when it is not one (or is too long).
pub fn parse_canonical(text: &str) -> Option<BigDecimal> {
    if text.len() > MAX_LENGTH || !is_canonical(text) {
        return None;
    }
    BigDecimal::from_str(text).ok()
}

/// The decimal a float prints as: the shortest text that identifies it, at its minimal scale and in plain
/// notation, so `5.0E-4` is `0.0005` and `100.0` is `100`. A float carries no scale, so a number and the
/// same decimal written as a string give the same answer; a float already computed in floating point is only
/// as exact as that computation.
pub fn from_f64(value: f64) -> Result<BigDecimal, String> {
    from_float_text(value.is_finite(), value.to_string())
}

/// As [`from_f64`], for a 32-bit float (its own shortest text, not the widened double's).
pub fn from_f32(value: f32) -> Result<BigDecimal, String> {
    from_float_text(value.is_finite(), value.to_string())
}

fn from_float_text(finite: bool, text: String) -> Result<BigDecimal, String> {
    if !finite {
        return Err(format!("Not a finite number: {text}"));
    }
    BigDecimal::from_str(&text)
        .map(|d| normalize(d.normalized()))
        .map_err(|_| format!("Not a finite number: {text}"))
}

/// Reads a plugin argument as an exact decimal; `context` names the plugin in the error.
pub fn operand(value: &Value, context: &str) -> Result<BigDecimal, String> {
    match value {
        Value::Nil => Err(format!("Cannot convert null to a decimal in {context}")),
        Value::Boolean(b) => Err(format!("Boolean operand in {context}: {b}")),
        Value::Integer(i) => match (i.as_i64(), i.as_u64()) {
            (Some(n), _) => Ok(BigDecimal::from(n)),
            (None, Some(n)) => Ok(BigDecimal::from(n)),
            _ => Err(format!(
                "Cannot convert the object to a decimal in {context}: {value}"
            )),
        },
        Value::F32(f) => from_f32(*f),
        Value::F64(f) => from_f64(*f),
        Value::String(s) => {
            let text = s.as_str().unwrap_or_default();
            parse_canonical(text)
                .ok_or_else(|| format!("Expected a decimal number in {context}, got '{text}'"))
        }
        other => Err(format!(
            "Cannot convert the object to a decimal in {context}: {other}"
        )),
    }
}

fn checked(value: BigDecimal, context: &str) -> Result<BigDecimal, String> {
    if value.digits() > MAX_RESULT_PRECISION {
        return Err(format!("Arithmetic result too large in '{context}'"));
    }
    Ok(normalize(value))
}

/// `a + b`, exact; the scale is the larger operand scale.
pub fn add(a: &BigDecimal, b: &BigDecimal) -> Result<BigDecimal, String> {
    checked(a + b, "add")
}

/// `a - b`, exact; the scale is the larger operand scale.
pub fn subtract(a: &BigDecimal, b: &BigDecimal) -> Result<BigDecimal, String> {
    checked(a - b, "subtract")
}

/// `a * b`, exact; the scale is the sum of the operand scales.
pub fn multiply(a: &BigDecimal, b: &BigDecimal) -> Result<BigDecimal, String> {
    checked(a * b, "multiply")
}

/// The remainder of `a / b`, exact; division by zero is an error naming `context`.
pub fn remainder(a: &BigDecimal, b: &BigDecimal, context: &str) -> Result<BigDecimal, String> {
    if b.is_zero() {
        return Err(format!("Division by zero in '{context}'"));
    }
    checked(a % b, context)
}

/// `a / b`: never truncates. The exact quotient when it terminates (at the smallest scale that represents it,
/// not below the dividend scale minus the divisor scale); otherwise 34 significant digits, HALF_EVEN. Division
/// by zero is an error naming `context`.
pub fn divide(a: &BigDecimal, b: &BigDecimal, context: &str) -> Result<BigDecimal, String> {
    if b.is_zero() {
        return Err(format!("Division by zero in '{context}'"));
    }
    let (unscaled_a, scale_a) = a.as_bigint_and_exponent();
    let (unscaled_b, scale_b) = b.as_bigint_and_exponent();
    // a / b = (ua * 10^sb) / (ub * 10^sa), with either scale possibly negative
    let mut numerator = unscaled_a;
    let mut denominator = unscaled_b;
    scale_by_power_of_ten(&mut numerator, &mut denominator, scale_b);
    scale_by_power_of_ten(&mut denominator, &mut numerator, scale_a);
    if numerator.is_zero() {
        return Ok(BigDecimal::from(0));
    }
    let negative = (numerator.sign() == Sign::Minus) != (denominator.sign() == Sign::Minus);
    let numerator = BigInt::from(numerator.magnitude().clone());
    let denominator = BigInt::from(denominator.magnitude().clone());
    let preferred_scale = a.fractional_digit_count() - b.fractional_digit_count();
    let quotient = match exact_quotient(&numerator, &denominator, negative, preferred_scale) {
        Some(exact) => exact,
        None => rounded_quotient(&numerator, &denominator, negative),
    };
    checked(quotient, context)
}

/// `value * 10^exponent` on the numerator side when `exponent` is positive, on the denominator side otherwise.
fn scale_by_power_of_ten(numerator: &mut BigInt, denominator: &mut BigInt, exponent: i64) {
    if exponent >= 0 {
        *numerator *= pow10(exponent as u32);
    } else {
        *denominator *= pow10((-exponent) as u32);
    }
}

fn pow10(n: u32) -> BigInt {
    BigInt::from(10).pow(n)
}

fn signed(value: BigInt, negative: bool) -> BigInt {
    if negative {
        -value
    } else {
        value
    }
}

/// The exact quotient, or `None` when it does not terminate.
fn exact_quotient(
    n: &BigInt,
    d: &BigInt,
    negative: bool,
    preferred_scale: i64,
) -> Option<BigDecimal> {
    // terminating iff, after stripping every 2 and 5 from the denominator, what is left divides the numerator
    let mut rest = d.clone();
    for factor in [2, 5] {
        let factor = BigInt::from(factor);
        while (&rest % &factor).is_zero() {
            rest /= &factor;
        }
    }
    if !(n % &rest).is_zero() {
        return None;
    }
    // the smallest k with d | n * 10^k
    let mut k = 0i64;
    let mut scaled = n.clone();
    while !(&scaled % d).is_zero() {
        scaled *= 10;
        k += 1;
    }
    let mut unscaled = signed(&scaled / d, negative);
    let mut scale = k;
    // strip trailing zeros down to the preferred scale, never below it
    let ten = BigInt::from(10);
    while scale > preferred_scale && (&unscaled % &ten).is_zero() {
        unscaled /= &ten;
        scale -= 1;
    }
    Some(BigDecimal::new(unscaled, scale))
}

/// 34 significant digits, HALF_EVEN, decided on the exact remainder (no double rounding).
fn rounded_quotient(n: &BigInt, d: &BigInt, negative: bool) -> BigDecimal {
    // find e with 10^(e-1) <= n/d < 10^e
    let mut e = n.to_string().len() as i64 - d.to_string().len() as i64;
    loop {
        if !at_least_power(n, d, e - 1) {
            e -= 1;
        } else if at_least_power(n, d, e) {
            e += 1;
        } else {
            break;
        }
    }
    let scale = DIVISION_DIGITS - e;
    let (numerator, denominator) = if scale >= 0 {
        (n * pow10(scale as u32), d.clone())
    } else {
        (n.clone(), d * pow10((-scale) as u32))
    };
    let mut quotient = &numerator / &denominator;
    let twice_remainder: BigInt = (&numerator % &denominator) * 2;
    let round_up = match twice_remainder.cmp(&denominator) {
        Ordering::Greater => true,
        Ordering::Equal => !(&quotient % BigInt::from(2)).is_zero(),
        Ordering::Less => false,
    };
    if round_up {
        quotient += 1;
    }
    let mut scale = scale;
    if quotient == pow10(DIVISION_DIGITS as u32) {
        // rounding carried into a 35th digit: 10^34 is 10^33 one scale lower
        quotient = pow10(DIVISION_DIGITS as u32 - 1);
        scale -= 1;
    }
    normalize(BigDecimal::new(signed(quotient, negative), scale))
}

/// Whether `n / d >= 10^power`.
fn at_least_power(n: &BigInt, d: &BigInt, power: i64) -> bool {
    if power >= 0 {
        *n >= d * pow10(power as u32)
    } else {
        n * pow10((-power) as u32) >= *d
    }
}

/// The rounding mode named `name` (any case); UNNECESSARY is not offered.
pub fn rounding_mode(name: &str, context: &str) -> Result<RoundingMode, String> {
    match name.trim().to_ascii_uppercase().as_str() {
        "HALF_UP" => Ok(RoundingMode::HalfUp),
        "HALF_EVEN" => Ok(RoundingMode::HalfEven),
        "HALF_DOWN" => Ok(RoundingMode::HalfDown),
        "UP" => Ok(RoundingMode::Up),
        "DOWN" => Ok(RoundingMode::Down),
        "CEILING" => Ok(RoundingMode::Ceiling),
        "FLOOR" => Ok(RoundingMode::Floor),
        _ => Err(format!("The mode of {context} must be {MODES}")),
    }
}

/// A whole number of decimal places from 0 to [`MAX_SCALE`].
pub fn places(scale: &BigDecimal, context: &str) -> Result<i64, String> {
    let error = || {
        format!(
            "The scale of {context} must be a whole number from 0 to {MAX_SCALE}, got {}",
            canonical(scale)
        )
    };
    let whole = scale.normalized();
    if whole.fractional_digit_count() > 0 {
        return Err(error());
    }
    let n: i64 = whole
        .with_scale(0)
        .to_string()
        .parse()
        .map_err(|_| error())?;
    if (0..=MAX_SCALE).contains(&n) {
        Ok(n)
    } else {
        Err(error())
    }
}

/// `x` at exactly `places` decimal places under `mode`.
pub fn round(x: &BigDecimal, places: i64, mode: RoundingMode) -> BigDecimal {
    normalize(x.with_scale_round(places, mode))
}

/// The order of two decimals by numeric value (`2.0` equals `2.00`).
pub fn compare(a: &BigDecimal, b: &BigDecimal) -> Ordering {
    a.cmp(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(text: &str) -> BigDecimal {
        BigDecimal::from_str(text).unwrap()
    }

    #[test]
    fn canonical_keeps_the_scale_and_writes_any_zero_as_zero() {
        assert_eq!(canonical(&d("10.50")), "10.50");
        assert_eq!(canonical(&d("0.00")), "0");
        assert_eq!(canonical(&d("1e3")), "1000");
        assert_eq!(canonical(&d("-0.5")), "-0.5");
    }

    #[test]
    fn a_canonical_string_is_plain_notation_only() {
        for ok in ["0", "-0", "7", "-12.5", "0.05", "100"] {
            assert!(is_canonical(ok), "{ok}");
        }
        for bad in [
            "", "-", "1e3", "007", "+1", ".5", "1.", "1.2.3", "0x1", " 1", "1 ",
        ] {
            assert!(!is_canonical(bad), "{bad}");
        }
        assert!(parse_canonical(&"1".repeat(1001)).is_none());
    }

    #[test]
    fn a_float_is_taken_through_its_shortest_decimal_text() {
        assert_eq!(canonical(&from_f64(0.1).unwrap()), "0.1");
        assert_eq!(canonical(&from_f64(5.0e-4).unwrap()), "0.0005");
        assert_eq!(canonical(&from_f64(100.0).unwrap()), "100");
        assert_eq!(canonical(&from_f32(0.1f32).unwrap()), "0.1");
        assert!(from_f64(f64::NAN)
            .unwrap_err()
            .contains("Not a finite number"));
        assert!(from_f64(f64::INFINITY).is_err());
        assert!(from_f32(f32::NEG_INFINITY).is_err());
    }

    #[test]
    fn division_never_truncates() {
        let q = |a: &str, b: &str| canonical(&divide(&d(a), &d(b), "div").unwrap());
        assert_eq!(q("7", "2"), "3.5");
        assert_eq!(q("6", "3"), "2");
        assert_eq!(q("100", "0.5"), "200");
        assert_eq!(q("1", "3"), "0.3333333333333333333333333333333333");
        assert_eq!(q("2", "3"), "0.6666666666666666666666666666666667");
        assert_eq!(q("-1", "3"), "-0.3333333333333333333333333333333333");
        assert!(divide(&d("1"), &d("0.00"), "div")
            .unwrap_err()
            .contains("Division by zero"));
    }

    #[test]
    fn a_quotient_that_rounds_into_a_new_digit_keeps_34_digits() {
        // (3*10^35 - 1) / (3*10^35) is 0.999...9 (35 nines) then 6666...: rounding to 34 digits carries
        let three_e35 = format!("3{}", "0".repeat(35));
        let almost = format!("2{}", "9".repeat(35));
        let q = divide(&d(&almost), &d(&three_e35), "div").unwrap();
        assert_eq!(canonical(&q), "1.000000000000000000000000000000000");
        assert_eq!(q.digits(), 34);
    }

    #[test]
    fn an_oversized_result_is_refused() {
        let big = BigDecimal::new(BigInt::from(10).pow(6000), 0);
        assert!(multiply(&big, &big).unwrap_err().contains("too large"));
    }

    #[test]
    fn places_and_modes_are_validated() {
        assert_eq!(places(&d("2"), "round").unwrap(), 2);
        assert_eq!(places(&d("2.0"), "round").unwrap(), 2);
        assert!(places(&d("1.5"), "round").is_err());
        assert!(places(&d("-1"), "round").is_err());
        assert!(places(&d("1001"), "round").is_err());
        assert!(rounding_mode("half_up", "round").is_ok());
        assert!(rounding_mode("UNNECESSARY", "round").is_err());
        assert!(rounding_mode("NEAREST", "round").is_err());
    }
}
