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

//! Renders the `{selectors}` of a graph.math expression into the text the evaluator parses (Java
//! `com.accenture.minigraph.math.ExpressionSource`).
//!
//! A statement's expression is the graph author's and a value comes from the data, so each value is
//! checked where it lands and is always one value, never more expression:
//!
//! - inside a string literal the author wrote, it is the text of that literal, escaped;
//! - next to a dot it is a name or digits;
//! - anywhere else a number or a boolean is itself, and a text value is a string literal in a boolean
//!   expression and one number in a number expression - text that does not spell a number fails there
//!   by name.
//!
//! A text value that spells `true`, `false` or `null` keeps its meaning in a number expression: the
//! first two read as booleans, and the third is reported as an unresolved variable.

use event_script::conversions::{display, to_json_string};
use event_script::jsonpath_source;
use rmpv::Value;

use super::DecimalEvaluator;

/// How the statement reads its expression, which decides how a text value is rendered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A COMPUTE or an IF without a comparison or boolean operator.
    Number,
    /// A DECIMAL statement: a number expression where a float renders as its plain decimal text.
    Decimal,
    /// A CONDITION, or a COMPUTE or an IF with a comparison or boolean operator.
    Boolean,
}

// bounds the work a value from the data can ask of the number test
const MAX_NUMBER_LENGTH: usize = 1000;
const KEYWORDS: [&str; 3] = ["true", "false", "null"];

struct Selector<'a> {
    key: &'a str,
    expression: &'a str,
    kind: Kind,
}

/// Render the selectors of an expression: `expression` as the author wrote it, `kind` how the
/// statement reads it, `lookup` resolving a selector's key to its value. The error names the
/// selector and never carries the value.
pub fn render(
    expression: &str,
    kind: Kind,
    lookup: &dyn Fn(&str) -> Result<Option<Value>, String>,
) -> Result<String, String> {
    let (Some(left), Some(right)) = (expression.find('{'), expression.rfind('}')) else {
        return Ok(expression.to_string());
    };
    if right <= left {
        return Ok(expression.to_string());
    }
    let chars: Vec<char> = expression.chars().collect();
    let mut out = String::new();
    let mut quote: Option<char> = None;
    let mut start = 0usize;
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] != '{' {
            i += 1;
            continue;
        }
        let Some(close) = chars[i..].iter().position(|c| *c == '}') else {
            break;
        };
        let seg_end = i + close + 1; // exclusive, includes '}'
        let heading: String = chars[start..i].iter().collect();
        out.push_str(&heading);
        quote = quote_after(quote, &heading);
        let key: String = chars[i + 1..seg_end - 1].iter().collect();
        if key.contains(['\r', '\n', '\t', ':']) {
            // a JavaScript block or a JSON object, left as written
            let written: String = chars[i..seg_end].iter().collect();
            out.push_str(&written);
            quote = quote_after(quote, &written);
        } else {
            // a {reference} names a key or a constant, never a JSONPath (RFC-0010)
            jsonpath_source::refuse_json_path_key(&key)?;
            let dot = heading.ends_with('.') || chars.get(seg_end) == Some(&'.');
            let selector = Selector {
                key: &key,
                expression,
                kind,
            };
            out.push_str(&rendered(&selector, lookup(&key)?, quote, dot)?);
        }
        start = seg_end;
        i = seg_end;
    }
    let tail: String = chars[start..].iter().collect();
    out.push_str(&tail);
    Ok(out)
}

/// True when the text spells one number literal of the dialect, with an optional sign.
pub fn is_one_number(text: &str) -> bool {
    let bytes = text.trim().as_bytes();
    if bytes.len() > MAX_NUMBER_LENGTH {
        return false;
    }
    let digits = |from: usize| {
        bytes[from..]
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count()
    };
    let mut at = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    let whole = digits(at);
    at += whole;
    if bytes.get(at) == Some(&b'.') {
        let fraction = digits(at + 1);
        if fraction == 0 {
            return false;
        }
        at += 1 + fraction;
    } else if whole == 0 {
        return false;
    }
    if matches!(bytes.get(at), Some(b'e' | b'E')) {
        at += 1 + usize::from(matches!(bytes.get(at + 1), Some(b'+' | b'-')));
        let exponent = digits(at);
        if exponent == 0 {
            return false;
        }
        at += exponent;
    }
    at == bytes.len()
}

fn rendered(
    selector: &Selector,
    value: Option<Value>,
    quote: Option<char>,
    dot: bool,
) -> Result<String, String> {
    let value = match value {
        None | Some(Value::Nil) => return Ok("null".to_string()),
        Some(value) => value,
    };
    let text = text_of(selector, &value)?;
    if let Some(open) = quote {
        return Ok(escape(&text, open));
    }
    if matches!(
        value,
        Value::Integer(_)
            | Value::F32(_)
            | Value::F64(_)
            | Value::Boolean(_)
            | Value::Map(_)
            | Value::Array(_)
    ) {
        return Ok(text);
    }
    let Selector {
        key, expression, ..
    } = selector;
    if dot {
        if !text.is_empty() && text.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Ok(text);
        }
        return Err(format!(
            "Not a name: {key} in '{expression}' - a text value next to a '.' must be a name or digits"
        ));
    }
    if selector.kind == Kind::Boolean {
        return Ok(format!("'{}'", escape(&text, '\'')));
    }
    if KEYWORDS.contains(&text.trim()) || is_one_number(&text) {
        return Ok(text);
    }
    Err(format!(
        "Not a number: {key} in '{expression}' - a text value is read here as one number, never as \
         part of the expression; write it inside quotes to read it as text ('{{{key}}}')"
    ))
}

fn text_of(selector: &Selector, value: &Value) -> Result<String, String> {
    let plain = match (selector.kind, value) {
        (Kind::Decimal, Value::F64(d)) => DecimalEvaluator::plain_text(*d),
        (Kind::Decimal, Value::F32(f)) => DecimalEvaluator::plain_text_f32(*f),
        (_, Value::Map(_) | Value::Array(_)) => return Ok(to_json_string(value)),
        _ => return Ok(display(value)),
    };
    plain.map_err(|e| format!("{e}: {} in '{}'", selector.key, selector.expression))
}

/// The expression lexer reads a backslash as an escape, so it is escaped first, then the quote.
fn escape(text: &str, quote: char) -> String {
    text.replace('\\', "\\\\")
        .replace(quote, &format!("\\{quote}"))
}

/// Follow the author's text with the lexer's rule for a string literal: the quote character of
/// the literal open after `written`, given the one open before it.
fn quote_after(quote: Option<char>, written: &str) -> Option<char> {
    let mut open = quote;
    let mut chars = written.chars();
    while let Some(c) = chars.next() {
        match open {
            None if c == '\'' || c == '"' => open = Some(c),
            None => {}
            // the escaped character belongs to the literal
            Some(_) if c == '\\' => {
                chars.next();
            }
            Some(q) if c == q => open = None,
            Some(_) => {}
        }
    }
    open
}

#[cfg(test)]
mod tests {
    use super::*;

    const TIMES_TWO: &str = "{input.body.x} * 2";
    const EQUALS_X: &str = "{input.body.x} == 'x'";
    const DOUBLE_QUOTED: &str = "\"{input.body.x}\" == 'x'";
    const MEMBER: &str = "Math.{input.body.x}(2)";
    const BRANCH: &str = "'a' == 'b' ? {input.body.x} : 0";
    const ISO_DATE: &str = "2025-07-15";

    fn render_value(expression: &str, kind: Kind, x: Option<Value>) -> Result<String, String> {
        render(expression, kind, &|key| {
            Ok(match key {
                "input.body.x" => x.clone(),
                "input.body.y" => Some(Value::from("2025-01-01")),
                _ => None,
            })
        })
    }

    fn text(expression: &str, kind: Kind, x: &str) -> Result<String, String> {
        render_value(expression, kind, Some(Value::from(x)))
    }

    #[test]
    fn a_text_value_that_spells_one_number_reads_as_written() {
        // every form the lexer reads as one number literal is rendered as it always was
        for value in [
            "100.25", "0", "1e3", "1E-5", "007", ".5", "+5", "-5", " 5 ", "true", "false", "null",
        ] {
            for kind in [Kind::Number, Kind::Decimal] {
                assert_eq!(
                    text(TIMES_TWO, kind, value).unwrap(),
                    format!("{value} * 2")
                );
            }
        }
    }

    #[test]
    fn any_other_text_in_a_number_expression_fails_naming_the_selector() {
        for value in [
            "2*3",
            "10-1",
            "(5)",
            ISO_DATE,
            "A4352",
            "5 5",
            "5.",
            "1,000",
            "0x10",
            "PI",
            "1/0",
            "max(1,2)",
            "3 > 2 ? 1 : 0",
            "1e",
            "--5",
            "Infinity",
            "",
        ] {
            for kind in [Kind::Number, Kind::Decimal] {
                let message = text(TIMES_TWO, kind, value).unwrap_err();
                assert!(
                    message.starts_with("Not a number: input.body.x in '{input.body.x} * 2'"),
                    "{message}"
                );
            }
        }
    }

    #[test]
    fn the_refusal_never_carries_the_value() {
        let message = text(TIMES_TWO, Kind::Decimal, ISO_DATE).unwrap_err();
        assert!(!message.contains(ISO_DATE), "{message}");
        assert_eq!(
            message,
            "Not a number: input.body.x in '{input.body.x} * 2' - a text value is read here as one \
             number, never as part of the expression; write it inside quotes to read it as text \
             ('{input.body.x}')"
        );
    }

    #[test]
    fn a_number_longer_than_the_bound_is_refused() {
        assert!(is_one_number(&"9".repeat(1000)));
        assert!(!is_one_number(&"9".repeat(1001)));
        assert!(text(TIMES_TWO, Kind::Number, &"9".repeat(1001)).is_err());
    }

    #[test]
    fn a_text_value_in_a_boolean_expression_is_a_string_literal() {
        assert_eq!(
            text(EQUALS_X, Kind::Boolean, "2*3").unwrap(),
            "'2*3' == 'x'"
        );
        assert_eq!(
            text("{input.body.x} == 200", Kind::Boolean, "200").unwrap(),
            "'200' == 200"
        );
        // the quote and the backslash of a value stay inside its literal
        assert_eq!(
            text(EQUALS_X, Kind::Boolean, "O'Brien").unwrap(),
            "'O\\'Brien' == 'x'"
        );
        assert_eq!(
            text(EQUALS_X, Kind::Boolean, "a\\b").unwrap(),
            "'a\\\\b' == 'x'"
        );
    }

    #[test]
    fn a_selector_inside_quotes_the_author_wrote_is_the_text_of_that_literal() {
        for kind in [Kind::Number, Kind::Decimal, Kind::Boolean] {
            assert_eq!(
                text(
                    "'{input.body.x}' >= '{input.body.y}' ? 100 : 0",
                    kind,
                    ISO_DATE
                )
                .unwrap(),
                "'2025-07-15' >= '2025-01-01' ? 100 : 0"
            );
            // a value cannot close the literal it lands in, whichever quote the author used
            assert_eq!(
                text("'{input.body.x}' == 'x'", kind, "a' + 'b").unwrap(),
                "'a\\' + \\'b' == 'x'"
            );
            assert_eq!(
                text(DOUBLE_QUOTED, kind, "a\" + \"b").unwrap(),
                "\"a\\\" + \\\"b\" == 'x'"
            );
            // the other quote character is text inside the literal
            assert_eq!(
                text(DOUBLE_QUOTED, kind, "it's").unwrap(),
                "\"it's\" == 'x'"
            );
            // an escaped quote in the author's literal does not close it
            assert_eq!(
                text("'it\\'s {input.body.x}' == 'x'", kind, "2*3").unwrap(),
                "'it\\'s 2*3' == 'x'"
            );
        }
    }

    #[test]
    fn a_literal_that_ended_before_the_selector_does_not_quote_it() {
        assert_eq!(
            text(BRANCH, Kind::Decimal, "5").unwrap(),
            "'a' == 'b' ? 5 : 0"
        );
        assert!(text(BRANCH, Kind::Decimal, "5+5").is_err());
    }

    #[test]
    fn a_number_or_a_boolean_is_itself() {
        let number = |kind, x: Value| render_value(TIMES_TWO, kind, Some(x)).unwrap();
        assert_eq!(number(Kind::Number, Value::from(3)), "3 * 2");
        assert_eq!(number(Kind::Decimal, Value::from(-3i64)), "-3 * 2");
        assert_eq!(number(Kind::Number, Value::from(true)), "true * 2");
        assert_eq!(
            render_value(EQUALS_X, Kind::Boolean, Some(Value::from(false))).unwrap(),
            "false == 'x'"
        );
    }

    #[test]
    fn a_float_renders_as_its_plain_decimal_text_in_a_decimal_statement() {
        let decimal = |x: Value| render_value(TIMES_TWO, Kind::Decimal, Some(x));
        assert_eq!(decimal(Value::F64(5.0E-4)).unwrap(), "0.0005 * 2");
        assert_eq!(decimal(Value::F32(100.0)).unwrap(), "100 * 2");
        assert_eq!(
            decimal(Value::F64(f64::NAN)).unwrap_err(),
            "Not a finite number: NaN: input.body.x in '{input.body.x} * 2'"
        );
    }

    #[test]
    fn a_text_value_next_to_a_dot_is_a_name_or_digits() {
        assert_eq!(text(MEMBER, Kind::Number, "abs").unwrap(), "Math.abs(2)");
        assert_eq!(
            text("{input.body.x}.5 * 2", Kind::Number, "3").unwrap(),
            "3.5 * 2"
        );
        let message = text(MEMBER, Kind::Boolean, "abs(1) + max").unwrap_err();
        assert_eq!(
            message,
            "Not a name: input.body.x in 'Math.{input.body.x}(2)' - a text value next to a '.' must be \
             a name or digits"
        );
    }

    #[test]
    fn an_unresolved_selector_renders_null_and_other_text_is_left_as_written() {
        assert_eq!(
            render_value(TIMES_TWO, Kind::Number, None).unwrap(),
            "null * 2"
        );
        // a segment with a colon is not a selector
        assert_eq!(
            text("{\"a\": 1} {input.body.x}", Kind::Number, "5").unwrap(),
            "{\"a\": 1} 5"
        );
        assert_eq!(text("1 + 2", Kind::Decimal, "ignored").unwrap(), "1 + 2");
        let list = Value::Array(vec![Value::from(1), Value::from(2)]);
        assert_eq!(
            render_value(TIMES_TWO, Kind::Number, Some(list)).unwrap(),
            "[1,2] * 2"
        );
    }
}
