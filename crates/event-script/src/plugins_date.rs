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

//! The date simple plugins — ports `com.accenture.services.plugins.date.AddDays`,
//! `com.accenture.services.plugins.date.DaysBetween` and their shared helper
//! `com.accenture.util.DatePluginUtils`.
//!
//! `f:addDays(date, days)` answers the date a whole number of days later (earlier for a negative
//! number) and `f:daysBetween(from, to)` the whole days between two dates as an integer, the earlier
//! date excluded and the later one included, whatever the order of the arguments. A date is an ISO-8601
//! calendar date written exactly as `YYYY-MM-DD`, with no time and no time zone, and a real day of the
//! proleptic Gregorian calendar; a result is written the same way, so it must fall within the years
//! 0000 to 9999. The days are read through [`decimal::operand`], so an integer, canonical numeric text
//! or a float whose value is whole are whole numbers. Every refusal, a Nil argument included, carries
//! the Java engine's message. The shared conformance vectors run in `tests/date_plugin_vectors.rs`.

use bigdecimal::ToPrimitive;
use chrono::{Datelike, NaiveDate, TimeDelta};
use event_script_macros::simple_plugin;
use rmpv::Value;

use crate::conversions::display;
use crate::decimal;

/// The widest span between two writable dates (0000-01-01 to 9999-12-31); a larger number of days
/// never lands on one, so it is refused before any date arithmetic.
const MAX_DAYS: i64 = 3_652_424;

fn expect_two(args: &[Value], context: &str) -> Result<(), String> {
    if args.len() == 2 {
        Ok(())
    } else {
        Err(format!(
            "Expected two input values in {context} - actual={}",
            args.len()
        ))
    }
}

/// Java `String.valueOf`, with text quoted so an empty or padded value shows.
fn describe(value: &Value) -> String {
    match value {
        Value::String(s) => format!("'{}'", s.as_str().unwrap_or_default()),
        other => display(other),
    }
}

/// `YYYY-MM-DD` with ASCII digits only - the Java pattern `\d{4}-\d{2}-\d{2}`.
fn is_iso_date_shape(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() == 10
        && bytes.iter().enumerate().all(|(i, b)| match i {
            4 | 7 => *b == b'-',
            _ => b.is_ascii_digit(),
        })
}

fn date(value: &Value, context: &str) -> Result<NaiveDate, String> {
    if let Value::String(s) = value {
        let text = s.as_str().unwrap_or_default();
        if is_iso_date_shape(text) {
            let field = |range: std::ops::Range<usize>| text[range].parse::<u32>().unwrap_or(0);
            return i32::try_from(field(0..4))
                .ok()
                .and_then(|year| NaiveDate::from_ymd_opt(year, field(5..7), field(8..10)))
                .ok_or_else(|| format!("Not a calendar date in {context} - got '{text}'"));
        }
    }
    Err(format!(
        "Expected a date written as YYYY-MM-DD, with no time and no time zone, in {context} - got {}",
        describe(value)
    ))
}

fn days(value: &Value, context: &str) -> Result<i64, String> {
    let number = decimal::operand(value, context).map_err(|_| days_error(value, context))?;
    if !number.is_integer() {
        return Err(days_error(value, context));
    }
    number
        .to_i64()
        .filter(|count| (-MAX_DAYS..=MAX_DAYS).contains(count))
        .ok_or_else(|| out_of_range(context))
}

fn text(date: NaiveDate, context: &str) -> Result<String, String> {
    if (0..=9999).contains(&date.year()) {
        Ok(format!(
            "{:04}-{:02}-{:02}",
            date.year(),
            date.month(),
            date.day()
        ))
    } else {
        Err(out_of_range(context))
    }
}

fn days_error(value: &Value, context: &str) -> String {
    format!(
        "The days of {context} must be a whole number - got {}",
        describe(value)
    )
}

fn out_of_range(context: &str) -> String {
    format!(
        "The date from {context} falls outside 0000-01-01 to 9999-12-31, which YYYY-MM-DD can write"
    )
}

#[simple_plugin("addDays")]
fn plugin_add_days(args: &[Value]) -> Result<Value, String> {
    const NAME: &str = "addDays";
    expect_two(args, NAME)?;
    let from = date(&args[0], NAME)?;
    let count = days(&args[1], NAME)?;
    let result = TimeDelta::try_days(count)
        .and_then(|delta| from.checked_add_signed(delta))
        .ok_or_else(|| out_of_range(NAME))?;
    Ok(Value::from(text(result, NAME)?))
}

#[simple_plugin("daysBetween")]
fn plugin_days_between(args: &[Value]) -> Result<Value, String> {
    const NAME: &str = "daysBetween";
    expect_two(args, NAME)?;
    let from = date(&args[0], NAME)?;
    let to = date(&args[1], NAME)?;
    Ok(Value::from(to.signed_duration_since(from).num_days().abs()))
}

#[cfg(test)]
mod tests {
    use rmpv::Value;

    use crate::plugins::calculate;

    fn add(date: &str, days: Value) -> Result<Value, String> {
        calculate("addDays", &[Value::from(date), days])
    }

    fn between(from: &str, to: &str) -> Result<Value, String> {
        calculate("daysBetween", &[Value::from(from), Value::from(to)])
    }

    #[test]
    fn add_days_matches_java_semantics() {
        for (date, days, expect) in [
            ("2027-01-15", 15, "2027-01-30"),
            ("2027-01-15", 0, "2027-01-15"),
            ("2027-12-31", 1, "2028-01-01"),
            ("2028-02-28", 1, "2028-02-29"),
            ("2027-02-28", 1, "2027-03-01"),
            ("2100-02-28", 1, "2100-03-01"),
            ("2027-01-30", -15, "2027-01-15"),
            ("9999-12-30", 1, "9999-12-31"),
            ("0000-01-02", -1, "0000-01-01"),
            ("0998-12-31", 1, "0999-01-01"),
            ("0000-01-01", 3_652_424, "9999-12-31"),
        ] {
            assert_eq!(Ok(Value::from(expect)), add(date, Value::from(days)));
        }
        // the days may be numeric text or a float whose value is whole
        assert_eq!(
            Ok(Value::from("2027-01-30")),
            add("2027-01-15", Value::from("15"))
        );
        assert_eq!(
            Ok(Value::from("2027-01-14")),
            add("2027-01-15", Value::from("-1"))
        );
        assert_eq!(
            Ok(Value::from("2027-01-30")),
            add("2027-01-15", Value::from(15.0))
        );
    }

    #[test]
    fn days_between_matches_java_semantics() {
        for (from, to, expect) in [
            ("2027-04-10", "2027-05-11", 31),
            ("2027-05-11", "2027-04-10", 31),
            ("2027-03-01", "2027-03-01", 0),
            ("2027-03-01", "2027-03-02", 1),
            ("2027-03-02", "2027-03-01", 1),
            ("2028-02-28", "2028-03-01", 2),
            ("2028-01-01", "2029-01-01", 366),
            ("0000-01-01", "9999-12-31", 3_652_424),
        ] {
            assert_eq!(Ok(Value::from(expect)), between(from, to), "{from} {to}");
        }
    }

    #[test]
    fn refusals_carry_the_java_messages() {
        assert_eq!(
            Err("Expected two input values in addDays - actual=1".to_string()),
            calculate("addDays", &[Value::from("2027-01-15")])
        );
        assert_eq!(
            Err(
                "Expected a date written as YYYY-MM-DD, with no time and no time zone, in addDays \
                 - got '2027-01-15T10:00:00'"
                    .to_string()
            ),
            add("2027-01-15T10:00:00", Value::from(1))
        );
        assert_eq!(
            Err("Expected a date written as YYYY-MM-DD, with no time and no time zone, in daysBetween \
                 - got ''"
                .to_string()),
            between("", "2027-01-16")
        );
        assert_eq!(
            Err(
                "Expected a date written as YYYY-MM-DD, with no time and no time zone, in addDays \
                 - got 20270115"
                    .to_string()
            ),
            calculate("addDays", &[Value::from(20_270_115), Value::from(1)])
        );
        assert_eq!(
            Err(
                "Expected a date written as YYYY-MM-DD, with no time and no time zone, in addDays \
                 - got null"
                    .to_string()
            ),
            calculate("addDays", &[Value::Nil, Value::from(1)])
        );
        assert_eq!(
            Err("Not a calendar date in daysBetween - got '2027-02-30'".to_string()),
            between("2027-03-01", "2027-02-30")
        );
        assert_eq!(
            Err("The days of addDays must be a whole number - got 1.5".to_string()),
            add("2027-01-15", Value::from(1.5))
        );
        assert_eq!(
            Err("The days of addDays must be a whole number - got 'fifteen'".to_string()),
            add("2027-01-15", Value::from("fifteen"))
        );
        assert_eq!(
            Err("The days of addDays must be a whole number - got null".to_string()),
            add("2027-01-15", Value::Nil)
        );
        assert_eq!(
            Err("The date from addDays falls outside 0000-01-01 to 9999-12-31, which YYYY-MM-DD can write"
                .to_string()),
            add("9999-12-31", Value::from(1))
        );
        assert_eq!(
            Err("The date from addDays falls outside 0000-01-01 to 9999-12-31, which YYYY-MM-DD can write"
                .to_string()),
            add("2027-01-15", Value::from(i64::MIN))
        );
    }
}
