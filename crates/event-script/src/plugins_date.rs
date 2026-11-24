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
//! `com.accenture.services.plugins.date.DaysBetween`, `com.accenture.services.plugins.date.AddMonths`,
//! `com.accenture.services.plugins.date.MonthsBetween`, `com.accenture.services.plugins.date.AddYears`,
//! `com.accenture.services.plugins.date.YearsBetween` and their shared helper
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
//!
//! `f:addMonths(date, months)` and `f:addYears(date, years)` do the same in calendar months and years:
//! the day of the month is kept, and a day the resulting month lacks becomes that month's last day
//! (2027-01-31 and one month give 2027-02-28) while no other date moves (2027-02-28 and one month give
//! 2027-03-28). `f:monthsBetween(from, to)` and `f:yearsBetween(from, to)` count the whole months and
//! years completed between two dates, in either order: a month is complete when the later date reaches
//! the earlier date's day of the month, a year when it reaches the month and day. So after an addition
//! that ended on a shorter month the count is one short of the number added.
//!
//! Parity note: the Java plugins call `LocalDate.plusMonths`, `plusYears` and `ChronoUnit.MONTHS` /
//! `YEARS.between`; this port calls jiff's `Date::checked_add` with a month or year span and
//! `Date::until` with the largest unit month or year, which give `java.time`'s answers (measured on
//! every pair of dates from 2027-11-01 to 2029-03-31 in both orders, and pinned by the shared vectors).
//! jiff can hold a year below 0000 where `java.time`'s writable range ends, so `text` refuses it.

use bigdecimal::ToPrimitive;
use event_script_macros::simple_plugin;
use jiff::civil::Date;
use jiff::{Span, Unit};
use rmpv::Value;

use crate::conversions::display;
use crate::decimal;

/// The widest span between two writable dates (0000-01-01 to 9999-12-31); a larger number of days
/// never lands on one, so it is refused before any date arithmetic.
const MAX_DAYS: i64 = 3_652_424;
/// The same span in whole months and in whole years.
const MAX_MONTHS: i64 = 119_999;
const MAX_YEARS: i64 = 9_999;

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

fn date(value: &Value, context: &str) -> Result<Date, String> {
    if let Value::String(s) = value {
        let text = s.as_str().unwrap_or_default();
        if is_iso_date_shape(text) {
            let field = |range: std::ops::Range<usize>| text[range].parse::<u32>().unwrap_or(0);
            let year = i16::try_from(field(0..4)).ok();
            let month = i8::try_from(field(5..7)).ok();
            let day = i8::try_from(field(8..10)).ok();
            return year
                .zip(month)
                .zip(day)
                .and_then(|((year, month), day)| Date::new(year, month, day).ok())
                .ok_or_else(|| format!("Not a calendar date in {context} - got '{text}'"));
        }
    }
    Err(format!(
        "Expected a date written as YYYY-MM-DD, with no time and no time zone, in {context} - got {}",
        describe(value)
    ))
}

/// A number of days: a whole number, or a number or numeric string whose value is whole.
fn days(value: &Value, context: &str) -> Result<i64, String> {
    count(value, context, "days", MAX_DAYS)
}

/// A number of months, by the rule that reads a number of days.
fn months(value: &Value, context: &str) -> Result<i64, String> {
    count(value, context, "months", MAX_MONTHS)
}

/// A number of years, by the rule that reads a number of days.
fn years(value: &Value, context: &str) -> Result<i64, String> {
    count(value, context, "years", MAX_YEARS)
}

/// Java `DatePluginUtils.count`: a whole number of the unit, at most the widest writable span
/// either way - a larger one is refused here, before any date arithmetic.
fn count(value: &Value, context: &str, unit: &str, max: i64) -> Result<i64, String> {
    let number = decimal::operand(value, context).map_err(|_| count_error(value, context, unit))?;
    if !number.is_integer() {
        return Err(count_error(value, context, unit));
    }
    number
        .to_i64()
        .filter(|count| (-max..=max).contains(count))
        .ok_or_else(|| out_of_range(context))
}

fn text(date: Date, context: &str) -> Result<String, String> {
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

fn count_error(value: &Value, context: &str, unit: &str) -> String {
    format!(
        "The {unit} of {context} must be a whole number - got {}",
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
    let result = Span::new()
        .try_days(count)
        .ok()
        .and_then(|span| from.checked_add(span).ok())
        .ok_or_else(|| out_of_range(NAME))?;
    Ok(Value::from(text(result, NAME)?))
}

#[simple_plugin("daysBetween")]
fn plugin_days_between(args: &[Value]) -> Result<Value, String> {
    const NAME: &str = "daysBetween";
    expect_two(args, NAME)?;
    let from = date(&args[0], NAME)?;
    let to = date(&args[1], NAME)?;
    // two writable dates are at most MAX_DAYS apart, so the span always fits
    let days = to.since(from).map_err(|_| out_of_range(NAME))?.get_days();
    Ok(Value::from(i64::from(days).abs()))
}

#[simple_plugin("addMonths")]
fn plugin_add_months(args: &[Value]) -> Result<Value, String> {
    const NAME: &str = "addMonths";
    expect_two(args, NAME)?;
    let from = date(&args[0], NAME)?;
    let count = months(&args[1], NAME)?;
    // the day of the month is kept, or becomes the last day of a shorter month - LocalDate.plusMonths
    let result = Span::new()
        .try_months(count)
        .ok()
        .and_then(|span| from.checked_add(span).ok())
        .ok_or_else(|| out_of_range(NAME))?;
    Ok(Value::from(text(result, NAME)?))
}

#[simple_plugin("monthsBetween")]
fn plugin_months_between(args: &[Value]) -> Result<Value, String> {
    const NAME: &str = "monthsBetween";
    expect_two(args, NAME)?;
    let from = date(&args[0], NAME)?;
    let to = date(&args[1], NAME)?;
    // the whole months completed, truncated toward zero either way - ChronoUnit.MONTHS.between
    let months = from
        .until((Unit::Month, to))
        .map_err(|_| out_of_range(NAME))?
        .get_months();
    Ok(Value::from(i64::from(months).abs()))
}

#[simple_plugin("addYears")]
fn plugin_add_years(args: &[Value]) -> Result<Value, String> {
    const NAME: &str = "addYears";
    expect_two(args, NAME)?;
    let from = date(&args[0], NAME)?;
    let count = years(&args[1], NAME)?;
    // 29 February becomes 28 February in a year that lacks it - LocalDate.plusYears
    let result = Span::new()
        .try_years(count)
        .ok()
        .and_then(|span| from.checked_add(span).ok())
        .ok_or_else(|| out_of_range(NAME))?;
    Ok(Value::from(text(result, NAME)?))
}

#[simple_plugin("yearsBetween")]
fn plugin_years_between(args: &[Value]) -> Result<Value, String> {
    const NAME: &str = "yearsBetween";
    expect_two(args, NAME)?;
    let from = date(&args[0], NAME)?;
    let to = date(&args[1], NAME)?;
    // every twelve whole months - ChronoUnit.YEARS.between
    let years = from
        .until((Unit::Year, to))
        .map_err(|_| out_of_range(NAME))?
        .get_years();
    Ok(Value::from(i64::from(years).abs()))
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

    fn add_months(date: &str, months: Value) -> Result<Value, String> {
        calculate("addMonths", &[Value::from(date), months])
    }

    fn add_years(date: &str, years: Value) -> Result<Value, String> {
        calculate("addYears", &[Value::from(date), years])
    }

    fn months_between(from: &str, to: &str) -> Result<Value, String> {
        calculate("monthsBetween", &[Value::from(from), Value::from(to)])
    }

    fn years_between(from: &str, to: &str) -> Result<Value, String> {
        calculate("yearsBetween", &[Value::from(from), Value::from(to)])
    }

    fn outside(plugin: &str) -> Result<Value, String> {
        Err(format!(
            "The date from {plugin} falls outside 0000-01-01 to 9999-12-31, which YYYY-MM-DD can write"
        ))
    }

    #[test]
    fn add_months_keeps_the_day_of_the_month() {
        for (date, months, expect) in [
            ("2027-03-15", 12, "2028-03-15"),
            ("2027-01-15", 1, "2027-02-15"),
            ("2027-11-15", 3, "2028-02-15"),
            ("2027-02-15", -3, "2026-11-15"),
            ("2027-01-31", 0, "2027-01-31"),
            // the last day of February does not become the last day of March
            ("2027-02-28", 1, "2027-03-28"),
            // a day the resulting month lacks becomes that month's last day
            ("2027-01-31", 1, "2027-02-28"),
            ("2028-01-31", 1, "2028-02-29"),
            ("2027-03-31", 1, "2027-04-30"),
            ("2027-03-31", -1, "2027-02-28"),
            // 2100 is not a leap year, 2000 was
            ("2100-01-31", 1, "2100-02-28"),
            ("2000-01-31", 1, "2000-02-29"),
            ("2027-01-31", 2, "2027-03-31"),
            // the ends of the writable years
            ("0000-01-01", 119_999, "9999-12-01"),
            ("9999-12-31", -119_999, "0000-01-31"),
        ] {
            assert_eq!(
                Ok(Value::from(expect)),
                add_months(date, Value::from(months)),
                "{date} {months}"
            );
        }
        // two steps of one month are not one step of two: a monthly schedule adds from its start date
        let first = add_months("2027-01-31", Value::from(1)).expect("a date");
        assert_eq!(
            Ok(Value::from("2027-03-28")),
            calculate("addMonths", &[first, Value::from(1)])
        );
    }

    #[test]
    fn add_years_keeps_the_month_and_day() {
        for (date, years, expect) in [
            ("2027-03-15", 1, "2028-03-15"),
            ("2027-03-15", -27, "2000-03-15"),
            ("2027-02-28", 1, "2028-02-28"),
            // 29 February in a year that lacks it becomes 28 February
            ("2028-02-29", 1, "2029-02-28"),
            ("2028-02-29", -1, "2027-02-28"),
            ("2028-02-29", 4, "2032-02-29"),
            ("2096-02-29", 4, "2100-02-28"),
            // the ends of the writable years; year 0000 is a leap year of the proleptic Gregorian calendar
            ("0000-06-15", 9_999, "9999-06-15"),
            ("9999-06-15", -9_999, "0000-06-15"),
            ("0000-02-29", 1, "0001-02-28"),
        ] {
            assert_eq!(
                Ok(Value::from(expect)),
                add_years(date, Value::from(years)),
                "{date} {years}"
            );
        }
        // a number of years is twelve times as many months
        assert_eq!(
            add_months("2028-02-29", Value::from(12)),
            add_years("2028-02-29", Value::from(1))
        );
    }

    #[test]
    fn the_months_and_years_are_any_whole_number() {
        for months in [Value::from(12), Value::from("12"), Value::from(12.0)] {
            assert_eq!(
                Ok(Value::from("2028-03-15")),
                add_months("2027-03-15", months)
            );
        }
        assert_eq!(
            Ok(Value::from("2027-02-15")),
            add_months("2027-03-15", Value::from("-1"))
        );
        assert_eq!(
            Ok(Value::from("2037-03-15")),
            add_years("2027-03-15", Value::from("10"))
        );
        assert_eq!(
            Ok(Value::from("2028-03-15")),
            add_years("2027-03-15", Value::from(1.0))
        );
        // away from the end of a month, the months of a monthsBetween answer bring the earlier date to the later one
        let months = months_between("2026-06-15", "2027-06-15").expect("a count");
        assert_eq!(
            Ok(Value::from("2027-06-15")),
            add_months("2026-06-15", months)
        );
    }

    #[test]
    fn months_between_counts_the_whole_months_in_either_order() {
        for (from, to, expect) in [
            ("2026-06-01", "2027-06-01", 12),
            ("2026-06-01", "2027-05-31", 11),
            ("2027-03-01", "2027-03-01", 0),
            ("2027-01-15", "2027-02-14", 0),
            ("2027-01-15", "2027-02-15", 1),
            ("2027-02-28", "2027-03-31", 1),
            ("2027-01-31", "2027-03-30", 1),
            ("2027-01-31", "2027-03-31", 2),
            ("2027-12-31", "2028-01-01", 0),
            // whatever the order, never negative
            ("2027-05-31", "2026-06-01", 11),
            ("2027-03-31", "2027-02-28", 1),
            ("2027-02-28", "2027-01-31", 0),
            ("0000-01-01", "9999-12-31", 119_999),
        ] {
            assert_eq!(
                Ok(Value::from(expect)),
                months_between(from, to),
                "{from} {to}"
            );
        }
    }

    #[test]
    fn years_between_counts_the_whole_years_in_either_order() {
        for (from, to, expect) in [
            ("2027-03-15", "2028-03-15", 1),
            ("2027-03-15", "2028-03-14", 0),
            // an age in whole years, the day before the birthday and on it
            ("1990-07-04", "2027-07-03", 36),
            ("1990-07-04", "2027-07-04", 37),
            ("2027-03-01", "2027-03-01", 0),
            ("2028-02-29", "2032-02-28", 3),
            ("2028-02-29", "2032-02-29", 4),
            ("2027-07-04", "1990-07-04", 37),
            ("0000-01-01", "9999-12-31", 9_999),
        ] {
            assert_eq!(
                Ok(Value::from(expect)),
                years_between(from, to),
                "{from} {to}"
            );
        }
    }

    #[test]
    fn the_count_is_one_short_after_an_addition_that_ended_on_a_shorter_month() {
        // 28 February does not reach day 31, so the month addMonths added is not a whole month yet
        assert_eq!(
            Ok(Value::from("2027-02-28")),
            add_months("2027-01-31", Value::from(1))
        );
        assert_eq!(
            Ok(Value::from(0)),
            months_between("2027-01-31", "2027-02-28")
        );
        // the same for a year from 29 February: eleven whole months and no whole year until 1 March
        assert_eq!(
            Ok(Value::from("2029-02-28")),
            add_months("2028-02-29", Value::from(12))
        );
        assert_eq!(
            Ok(Value::from(11)),
            months_between("2028-02-29", "2029-02-28")
        );
        assert_eq!(
            Ok(Value::from(0)),
            years_between("2028-02-29", "2029-02-28")
        );
        assert_eq!(
            Ok(Value::from(12)),
            months_between("2028-02-29", "2029-03-01")
        );
        assert_eq!(
            Ok(Value::from(1)),
            years_between("2028-02-29", "2029-03-01")
        );
    }

    #[test]
    fn month_and_year_refusals_carry_the_java_messages() {
        assert_eq!(
            Err("Expected two input values in addMonths - actual=1".to_string()),
            calculate("addMonths", &[Value::from("2027-01-15")])
        );
        assert_eq!(
            Err("Expected two input values in monthsBetween - actual=0".to_string()),
            calculate("monthsBetween", &[])
        );
        assert_eq!(
            Err("Expected two input values in addYears - actual=3".to_string()),
            calculate(
                "addYears",
                &[Value::from("2027-01-15"), Value::from(1), Value::from(2)]
            )
        );
        assert_eq!(
            Err(
                "Expected a date written as YYYY-MM-DD, with no time and no time zone, in addMonths \
                 - got '2027-01-15T10:00:00'"
                    .to_string()
            ),
            add_months("2027-01-15T10:00:00", Value::from(1))
        );
        assert_eq!(
            Err(
                "Expected a date written as YYYY-MM-DD, with no time and no time zone, in yearsBetween \
                 - got null"
                    .to_string()
            ),
            calculate("yearsBetween", &[Value::from("2027-01-15"), Value::Nil])
        );
        assert_eq!(
            Err("Not a calendar date in monthsBetween - got '2027-04-31'".to_string()),
            months_between("2027-04-31", "2027-05-31")
        );
        assert_eq!(
            Err("Not a calendar date in addYears - got '2027-02-29'".to_string()),
            add_years("2027-02-29", Value::from(1))
        );
        // the months and the years are a whole number, and the message names the unit
        assert_eq!(
            Err("The months of addMonths must be a whole number - got 1.5".to_string()),
            add_months("2027-01-15", Value::from(1.5))
        );
        assert_eq!(
            Err("The months of addMonths must be a whole number - got 'twelve'".to_string()),
            add_months("2027-01-15", Value::from("twelve"))
        );
        assert_eq!(
            Err("The months of addMonths must be a whole number - got null".to_string()),
            add_months("2027-01-15", Value::Nil)
        );
        assert_eq!(
            Err("The months of addMonths must be a whole number - got true".to_string()),
            add_months("2027-01-15", Value::from(true))
        );
        assert_eq!(
            Err("The years of addYears must be a whole number - got 0.5".to_string()),
            add_years("2027-01-15", Value::from(0.5))
        );
        assert_eq!(
            Err("The years of addYears must be a whole number - got 'ten'".to_string()),
            add_years("2027-01-15", Value::from("ten"))
        );
        assert!(add_years("2027-01-15", Value::Array(vec![Value::from(1)]))
            .expect_err("a list is not a number of years")
            .starts_with("The years of addYears must be a whole number"));
    }

    #[test]
    fn a_month_or_year_result_outside_the_writable_years_is_refused() {
        for (date, months) in [
            ("9999-12-31", 1),
            // jiff holds year -0001, which YYYY-MM-DD cannot write
            ("0000-01-01", -1),
            // a count within the widest span that still lands outside, and a count beyond the span
            ("2027-01-15", 95_676),
            ("2027-01-15", -24_325),
            ("2027-01-15", 120_000),
            ("2027-01-15", i64::MAX),
        ] {
            assert_eq!(
                outside("addMonths"),
                add_months(date, Value::from(months)),
                "{date} {months}"
            );
        }
        for (date, years) in [
            ("9999-12-31", 1),
            ("2027-01-15", -2_028),
            ("2027-01-15", 10_000),
            ("2027-01-15", i64::MIN),
        ] {
            assert_eq!(
                outside("addYears"),
                add_years(date, Value::from(years)),
                "{date} {years}"
            );
        }
    }
}
