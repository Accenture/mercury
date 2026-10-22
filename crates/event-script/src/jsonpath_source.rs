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

//! A data-mapping source written as a JSONPath query (`$...`), checked against
//! RFC 9535 before it runs (RFC-0010) - Java `com.accenture.util.JsonPathSource`.
//!
//! A source may carry references that the engine replaces with their values
//! before the query is parsed: Event Script's `{model.key}` and `[model.i]`,
//! and the knowledge graph's `{namespace.key}`. The values are not known when a
//! flow or a graph is compiled, so the check parses the source with a stand-in
//! for each reference, chosen by where the reference lands: after a dot or
//! inside a member name the name `x`, inside a bracketed selector the index
//! `0`, inside a string literal the author wrote the letter `x`, and bare
//! inside a filter expression the string literal `'x'`. No value can make a
//! query of a reference anywhere else (directly after the root `$`, for
//! example), so the source is refused there.
//!
//! The reference scans copy this engine's own substitution: `{` pairs with the
//! next `}` (`mapping::substitute_runtime_vars`,
//! `substitute_var_if_any_logical` in the knowledge graph), and Event Script
//! pairs each `[` with the first `]` after the previous pair
//! (`executor::substitute_dynamic_index`) - so a `[model.i]` inside a filter is
//! not an index reference, and the parser refuses it.

use crate::conversions::display;
use crate::mapping::split_top_level_arguments;
use crate::mlm::compile_json_path_at;
use crate::util::str2int;
use rmpv::Value;

const MODEL_NAMESPACE: &str = "model.";

/// Which references the engine substitutes into a source before it reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    /// Event Script: `{model.key}` anywhere, and `[model.i]` as the whole
    /// content of a bracket
    EventScript,
    /// The knowledge graph: `{namespace.key}` anywhere
    Graph,
    /// None: the source is read as written (a data dictionary's default value)
    Plain,
}

/// Where a reference lands in the query, which decides what may stand there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Site {
    /// After a dot, or inside a member name
    Name,
    /// Inside a bracketed selector, outside a filter
    Index,
    /// Inside a string literal the author wrote
    Literal,
    /// Bare inside a filter expression
    Filter,
}

impl Site {
    /// The site's name in the shared vectors.
    pub fn name(self) -> &'static str {
        match self {
            Site::Name => "name",
            Site::Index => "index",
            Site::Literal => "literal",
            Site::Filter => "filter",
        }
    }
}

/// A reference in a source, by character index: `start` is its first
/// character and `end` the one after its last. `key` is the key it names;
/// `braced` is true for a `{key}` and false for Event Script's `[model.i]`,
/// whose span is the bracket's content; `quote` is the enclosing string
/// literal's quote character when the site is a literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    pub start: usize,
    pub end: usize,
    pub site: Site,
    pub key: String,
    pub braced: bool,
    pub quote: Option<char>,
}

/// True when a data-mapping source is a JSONPath query.
pub fn is_json_path(source: &str) -> bool {
    source.starts_with('$')
}

/// Refuse a JSONPath source that is not RFC 9535, with Mercury's hyphen rule
/// (a hyphen in a dotted name). The error names the reason and the position
/// in the source as written: `reason at position N of 'source'`.
pub fn check(source: &str, form: Form) -> Result<(), String> {
    let chars: Vec<char> = source.chars().collect();
    let references = references(source, form)?;
    let mut query = String::new();
    // the source character each query character came from
    let mut origin: Vec<usize> = Vec::new();
    let mut last = 0usize;
    for reference in &references {
        for (i, c) in chars.iter().enumerate().take(reference.start).skip(last) {
            query.push(*c);
            origin.push(i);
        }
        for c in stand_in(reference.site).chars() {
            query.push(c);
            origin.push(reference.start);
        }
        last = reference.end;
    }
    for (i, c) in chars.iter().enumerate().skip(last) {
        query.push(*c);
        origin.push(i);
    }
    match compile_json_path_at(&query) {
        Ok(_) => Ok(()),
        Err((reason, position)) => {
            // the stand-ins shift the text, so the position is restated in the source as written
            let at = origin.get(position).copied().unwrap_or(chars.len());
            Err(format!("{reason} at position {at} of '{source}'"))
        }
    }
}

/// Find the references in a source and where each one lands, in the order
/// they appear. A reference that stands where no value can make a query is an
/// error.
pub fn references(source: &str, form: Form) -> Result<Vec<Reference>, String> {
    let chars: Vec<char> = source.chars().collect();
    let spans = match form {
        Form::EventScript => event_script_spans(&chars),
        Form::Graph => graph_spans(&chars),
        Form::Plain => Vec::new(),
    };
    let mut result = Vec::new();
    let mut lexer = Lexer::default();
    let mut next = 0usize;
    let mut i = 0usize;
    while i < chars.len() {
        if next < spans.len() && spans[next].0 == i {
            let (start, end) = spans[next];
            next += 1;
            let Some(site) = lexer.site() else {
                return Err(format!(
                    "a reference is not allowed here at position {i} of '{source}'"
                ));
            };
            let braced = chars[start] == '{';
            let key = if braced {
                let key = text(&chars, start + 1, end - 1);
                // Event Script trims its key; the knowledge graph reads the key as written
                if form == Form::EventScript {
                    key.trim().to_string()
                } else {
                    key
                }
            } else {
                text(&chars, start, end).trim().to_string()
            };
            result.push(Reference {
                start,
                end,
                site,
                key,
                braced,
                quote: lexer.quote,
            });
            // the lexer reads the stand-in in place of the reference, so the context after it is the same
            for c in stand_in(site).chars() {
                lexer.accept(c);
            }
            i = end;
        } else {
            lexer.accept(chars[i]);
            i += 1;
        }
    }
    Ok(result)
}

/// Refuse a data-mapping source whose JSONPath is not RFC 9535: the source
/// itself when it is a JSONPath query, and each argument of a simple plugin
/// that is one. Nested plugin calls are not resolved, so their arguments are
/// not read (Java `DataMappingHelper.validateJsonPaths`).
pub fn validate_json_paths(lhs: &str, form: Form) -> Result<(), String> {
    if is_json_path(lhs) {
        return check(lhs, form);
    }
    if lhs.starts_with("f:") {
        if let (Some(start), Some(end)) = (lhs.find('('), lhs.rfind(')')) {
            if start > 0 && end > start {
                for argument in split_top_level_arguments(&lhs[start + 1..end]) {
                    let arg = argument.trim();
                    if is_json_path(arg) {
                        check(arg, form)?;
                    }
                }
            }
        }
    }
    Ok(())
}

/// The knowledge graph's rule for a `{key}` that is a reference: a key with a
/// line break, a tab or a colon is text (a JavaScript block or a JSON object),
/// left as written.
pub fn is_graph_key(key: &str) -> bool {
    !(key.contains('\r') || key.contains('\n') || key.contains('\t') || key.contains(':'))
}

/// A knowledge-graph `{reference}` names a state-machine key or a constant,
/// never a JSONPath query: JSONPath is a data mapping's source, read where the
/// source and its target are separated by `->`.
pub fn refuse_json_path_key(key: &str) -> Result<(), String> {
    if is_json_path(key.trim()) {
        Err(format!(
            "JSONPath is not supported in a reference - {{{key}}}; map the value to a model variable first"
        ))
    } else {
        Ok(())
    }
}

/// Substitute each reference in a JSONPath source with its value, checked
/// where the reference lands, so that a value cannot change the query's
/// structure (RFC-0010 WP4; Java `JsonPathSource.substitute`): in a name a key
/// path (names joined by single dots, each optionally followed by `[n]`
/// indexes), in an index an integer, inside a string literal the author wrote
/// its text escaped for that literal, and bare inside a filter a number, a
/// boolean or null as written and text as a string literal - except that
/// Event Script, which has always inserted text as written, keeps text that is
/// already one literal (a number, `true`, `false`, `null` or a quoted string)
/// as written. A list or a map is never substituted. Event Script's
/// `[model.i]` keeps its integer rendering, and a value renders as text the
/// way each engine form always has.
pub fn substitute(
    source: &str,
    form: Form,
    values: &dyn Fn(&str) -> Result<Option<Value>, String>,
) -> Result<String, String> {
    let references = references(source, form)?;
    if references.is_empty() {
        return Ok(source.to_string());
    }
    let chars: Vec<char> = source.chars().collect();
    let mut query = String::new();
    let mut last = 0usize;
    for reference in &references {
        query.push_str(&text(&chars, last, reference.start));
        let value = values(&reference.key)?;
        query.push_str(&render(source, &chars, reference, form, value.as_ref())?);
        last = reference.end;
    }
    query.push_str(&text(&chars, last, chars.len()));
    Ok(query)
}

fn render(
    source: &str,
    chars: &[char],
    reference: &Reference,
    form: Form,
    value: Option<&Value>,
) -> Result<String, String> {
    if matches!(value, Some(Value::Map(_)) | Some(Value::Array(_))) {
        return Err(invalid(
            source,
            chars,
            reference,
            "a list or a map cannot be substituted into a JSONPath",
        ));
    }
    let text = text_of(form, value);
    if !reference.braced {
        // Event Script's [model.i] has always rendered as an integer
        return Ok(str2int(&text).to_string());
    }
    match reference.site {
        Site::Name if is_key_path(&text) => Ok(text),
        Site::Name => Err(invalid(
            source,
            chars,
            reference,
            &format!("'{text}' is not a key path"),
        )),
        Site::Index if is_integer(&text) => Ok(text),
        Site::Index => Err(invalid(
            source,
            chars,
            reference,
            &format!("'{text}' is not an integer index"),
        )),
        Site::Literal => Ok(escape(&text, reference.quote.unwrap_or('\''))),
        Site::Filter => Ok(filter_value(form, value)),
    }
}

fn filter_value(form: Form, value: Option<&Value>) -> String {
    match value {
        None | Some(Value::Nil) => "null".to_string(),
        Some(v @ (Value::Integer(_) | Value::F32(_) | Value::F64(_) | Value::Boolean(_))) => {
            display(v)
        }
        Some(v) => {
            let text = display(v);
            if form == Form::EventScript && is_literal(&text) {
                text
            } else {
                format!("'{}'", escape(&text, '\''))
            }
        }
    }
}

/// A value as text, the way each engine form has always rendered it: Event
/// Script renders a string or a number and anything else as null
/// (`mapping::string_from_model_value`); the knowledge graph renders any value.
fn text_of(form: Form, value: Option<&Value>) -> String {
    match value {
        None | Some(Value::Nil) => "null".to_string(),
        Some(v @ (Value::String(_) | Value::Integer(_) | Value::F32(_) | Value::F64(_))) => {
            display(v)
        }
        Some(v) if form != Form::EventScript => display(v),
        Some(_) => "null".to_string(),
    }
}

fn invalid(source: &str, chars: &[char], reference: &Reference, reason: &str) -> String {
    let written = if reference.braced {
        text(chars, reference.start, reference.end)
    } else {
        format!("[{}]", reference.key)
    };
    format!("Cannot substitute {written} into '{source}' - {reason}")
}

/// Escape text for an RFC 9535 string literal written with the given quote:
/// the quote, the backslash and the control characters.
pub fn escape(text: &str, quote: char) -> String {
    let mut out = String::new();
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// True when a value's text is a key path: names joined by single dots, each
/// optionally followed by `[n]` indexes (`items[0].name`).
pub fn is_key_path(text: &str) -> bool {
    !text.is_empty()
        && text.split('.').all(|part| {
            let name = part.chars().take_while(|c| is_name_char(*c)).count();
            let rest: Vec<char> = part.chars().skip(name).collect();
            name > 0 && are_indexes(&rest)
        })
}

fn are_indexes(chars: &[char]) -> bool {
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] != '[' {
            return false;
        }
        let digits = chars[i + 1..]
            .iter()
            .take_while(|c| c.is_ascii_digit())
            .count();
        if digits == 0 || chars.get(i + 1 + digits) != Some(&']') {
            return false;
        }
        i += digits + 2;
    }
    true
}

fn is_integer(text: &str) -> bool {
    let digits = text.strip_prefix('-').unwrap_or(text);
    !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())
}

fn is_number(text: &str) -> bool {
    let rest = text.strip_prefix('-').unwrap_or(text);
    let int_len = rest.chars().take_while(|c| c.is_ascii_digit()).count();
    if int_len == 0 || (int_len > 1 && rest.starts_with('0')) {
        return false;
    }
    let mut rest = &rest[int_len..];
    if let Some(fraction) = rest.strip_prefix('.') {
        let n = fraction.chars().take_while(|c| c.is_ascii_digit()).count();
        if n == 0 {
            return false;
        }
        rest = &fraction[n..];
    }
    if let Some(exponent) = rest.strip_prefix(['e', 'E']) {
        let exponent = exponent.strip_prefix(['-', '+']).unwrap_or(exponent);
        return !exponent.is_empty() && exponent.chars().all(|c| c.is_ascii_digit());
    }
    rest.is_empty()
}

/// Text that is already exactly one RFC 9535 literal: a number, `true`,
/// `false`, `null`, or a string in single or double quotes whose own quote
/// character appears inside only escaped.
fn is_literal(text: &str) -> bool {
    if is_number(text) || matches!(text, "true" | "false" | "null") {
        return true;
    }
    let chars: Vec<char> = text.chars().collect();
    if chars.len() < 2 || !matches!(chars[0], '\'' | '"') || chars[chars.len() - 1] != chars[0] {
        return false;
    }
    let quote = chars[0];
    let mut escaped = false;
    for c in &chars[1..chars.len() - 1] {
        if escaped {
            escaped = false;
        } else if *c == '\\' {
            escaped = true;
        } else if *c == quote {
            return false;
        }
    }
    !escaped
}

fn stand_in(site: Site) -> &'static str {
    match site {
        Site::Name | Site::Literal => "x",
        Site::Index => "0",
        Site::Filter => "'x'",
    }
}

fn is_model_key(key: &str) -> bool {
    key.starts_with(MODEL_NAMESPACE) && !key.ends_with('.')
}

fn text(chars: &[char], from: usize, to: usize) -> String {
    chars[from..to].iter().collect()
}

fn find(chars: &[char], from: usize, target: char) -> Option<usize> {
    chars[from..]
        .iter()
        .position(|c| *c == target)
        .map(|p| p + from)
}

fn event_script_spans(chars: &[char]) -> Vec<(usize, usize)> {
    let mut braces = Vec::new();
    if chars.contains(&'{') && chars.contains(&'}') {
        let mut from = 0usize;
        while let Some(open) = find(chars, from, '{') {
            let Some(close) = find(chars, open, '}') else {
                break;
            };
            if is_model_key(text(chars, open + 1, close).trim()) {
                braces.push((open, close + 1));
            }
            from = close + 1;
        }
    }
    let mut masked = chars.to_vec();
    for (start, end) in &braces {
        for c in masked.iter_mut().take(*end).skip(*start) {
            *c = 'x';
        }
    }
    let mut indexes = Vec::new();
    if chars.contains(&'[') && chars.contains(&']') {
        let mut from = 0usize;
        while let (Some(open), Some(close)) = (find(&masked, from, '['), find(&masked, from, ']')) {
            if close < open {
                break;
            }
            if is_model_key(text(&masked, open + 1, close).trim()) {
                indexes.push((open + 1, close));
            }
            from = close + 1;
        }
    }
    let mut result = indexes.clone();
    for brace in braces {
        if !indexes.iter().any(|(s, e)| *s <= brace.0 && brace.1 <= *e) {
            result.push(brace);
        }
    }
    result.sort_by_key(|span| span.0);
    result
}

fn graph_spans(chars: &[char]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut from = 0usize;
    while let Some(open) = find(chars, from, '{') {
        let Some(close) = find(chars, open, '}') else {
            break;
        };
        if is_graph_key(&text(chars, open + 1, close)) {
            result.push((open, close + 1));
        }
        from = close + 1;
    }
    result
}

fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-' || (c as u32) >= 0x80
}

#[derive(Default)]
struct Frame {
    filter: bool,
    parens: i32,
}

/// Just enough of RFC 9535's lexical structure to tell where a reference lands.
struct Lexer {
    frames: Vec<Frame>,
    quote: Option<char>,
    escaped: bool,
    previous: char,
}

impl Default for Lexer {
    fn default() -> Self {
        Lexer {
            frames: Vec::new(),
            quote: None,
            escaped: false,
            previous: ' ',
        }
    }
}

impl Lexer {
    fn accept(&mut self, c: char) {
        if let Some(quote) = self.quote {
            if self.escaped {
                self.escaped = false;
            } else if c == '\\' {
                self.escaped = true;
            } else if c == quote {
                self.quote = None;
            }
        } else if c == '\'' || c == '"' {
            self.quote = Some(c);
        } else {
            self.structure(c);
        }
        self.previous = c;
    }

    fn structure(&mut self, c: char) {
        match c {
            '[' => self.frames.push(Frame::default()),
            ']' => {
                self.frames.pop();
            }
            _ => {
                if let Some(top) = self.frames.last_mut() {
                    match c {
                        '?' => top.filter = true,
                        '(' => top.parens += 1,
                        ')' => top.parens -= 1,
                        // a comma between selectors ends a filter selector; a comma between arguments does not
                        ',' => top.filter = top.filter && top.parens > 0,
                        _ => {}
                    }
                }
            }
        }
    }

    fn site(&self) -> Option<Site> {
        if self.quote.is_some() {
            return Some(Site::Literal);
        }
        let top = self.frames.last();
        let filter = top.is_some_and(|f| f.filter);
        if self.previous == '.' || (is_name_char(self.previous) && (top.is_none() || filter)) {
            return Some(Site::Name);
        }
        top.map(|_| if filter { Site::Filter } else { Site::Index })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_position_is_stated_in_the_source_as_written() {
        // the stand-in 'x' is ten characters shorter than {model.key}
        let plain = check("$.input.body.x[?(@.a in [1])]", Form::Plain).unwrap_err();
        let templated =
            check("$.input.body.{model.key}[?(@.a in [1])]", Form::EventScript).unwrap_err();
        let position = |e: &str| -> usize {
            let tail = &e[e.find(" at position ").unwrap() + 13..];
            tail[..tail.find(' ').unwrap()].parse().unwrap()
        };
        assert_eq!(position(&plain) + 10, position(&templated));
        assert!(
            templated.ends_with(" of '$.input.body.{model.key}[?(@.a in [1])]'"),
            "{templated}"
        );
        assert_eq!(
            references("${model.key}", Form::EventScript).unwrap_err(),
            "a reference is not allowed here at position 1 of '${model.key}'"
        );
    }

    #[test]
    fn a_plugins_json_path_argument_is_checked() {
        assert!(validate_json_paths("f:length($.input.body['a','b'])", Form::EventScript).is_ok());
        assert!(validate_json_paths("f:concat(model.a, text(b))", Form::EventScript).is_ok());
        let e = validate_json_paths(
            "f:concat(model.a, $.input.body.items.first())",
            Form::EventScript,
        )
        .unwrap_err();
        assert!(e.ends_with(" of '$.input.body.items.first()'"), "{e}");
    }
}
