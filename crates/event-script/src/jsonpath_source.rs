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

use crate::mapping::split_top_level_arguments;
use crate::mlm::compile_json_path_at;

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
/// character and `end` the one after its last.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reference {
    pub start: usize,
    pub end: usize,
    pub site: Site,
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
            result.push(Reference { start, end, site });
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
