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

//! Rust port of the Java `ToolSchema` (`org.platformlambda.agentloop.ToolSchema`): a tool's input
//! schema (RFC-0011, section 5), a closed subset of JSON Schema, compiled from a catalog row's
//! `input_schema` before the first model turn, and used to validate each input the model fills in
//! before the tool's route is called.
//!
//! The vocabulary is `type` (a name, or a list of names, from object, array, string, number, integer,
//! boolean and null), `properties`, `required`, `additionalProperties` (a boolean or a schema),
//! `items`, `enum`, `const`, `minimum`, `maximum`, `exclusiveMinimum`, `exclusiveMaximum` (numbers, as
//! in JSON Schema), `minLength`, `maxLength`, `pattern`, `minItems` and `maxItems`; `title`,
//! `description`, `default`, `examples` and `format` are documentary and never validated. Any other
//! keyword is refused, because a constraint the loop silently ignores teaches that unflagged means
//! safe; so is a keyword that cannot apply to any of the declared types. This is the JSON Schema
//! dialect the model is given, not the OpenAPI 3.0 subset of the graph contract.
//!
//! Types are strict JSON types: a numeric string is not a number, a boolean is not a number, and an
//! integer is a number with no fractional part (so `1.0` is one). A `pattern` is searched, not
//! anchored, with Unicode-aware classes, and stays inside the regular-expression subset common to
//! both engines. Lengths count code points. The messages are pinned by the shared vector file
//! `llm-run-vectors.json`.
//!
//! Numbers compare and print as exact decimals - the Java engine's `BigDecimal` - through
//! [`Decimal`]: a float is read as the shortest decimal that round-trips, which is what both engines
//! print for a double, so `1000.0` renders as `1000` and an integer beyond 2^53 still compares
//! exactly.

use std::cmp::Ordering;
use std::collections::HashMap;

use regex::Regex;
use rmpv::Value;

use crate::json::compact;
use crate::value::{and_list, contains_key, entries, get, key_text, sorted_keys};

/// The number of violations one message carries; the rest are counted.
pub const VIOLATION_CAP: usize = 10;
const TYPE: &str = "type";
const PROPERTIES: &str = "properties";
const REQUIRED: &str = "required";
const ADDITIONAL_PROPERTIES: &str = "additionalProperties";
const ITEMS: &str = "items";
const ENUM: &str = "enum";
const CONST: &str = "const";
const MINIMUM: &str = "minimum";
const MAXIMUM: &str = "maximum";
const EXCLUSIVE_MINIMUM: &str = "exclusiveMinimum";
const EXCLUSIVE_MAXIMUM: &str = "exclusiveMaximum";
const MIN_LENGTH: &str = "minLength";
const MAX_LENGTH: &str = "maxLength";
const PATTERN: &str = "pattern";
const MIN_ITEMS: &str = "minItems";
const MAX_ITEMS: &str = "maxItems";
pub(crate) const OBJECT: &str = "object";
const ARRAY: &str = "array";
const STRING: &str = "string";
const NUMBER: &str = "number";
const INTEGER: &str = "integer";
const BOOLEAN: &str = "boolean";
const NULL: &str = "null";
/// The vocabulary, in the order the error message lists it.
pub const KEYWORDS: [&str; 16] = [
    TYPE,
    PROPERTIES,
    REQUIRED,
    ADDITIONAL_PROPERTIES,
    ITEMS,
    ENUM,
    CONST,
    MINIMUM,
    MAXIMUM,
    EXCLUSIVE_MINIMUM,
    EXCLUSIVE_MAXIMUM,
    MIN_LENGTH,
    MAX_LENGTH,
    PATTERN,
    MIN_ITEMS,
    MAX_ITEMS,
];
/// Keywords that document and never validate.
pub const DOCUMENTARY: [&str; 5] = ["title", "description", "default", "examples", "format"];
const KNOWN_TYPES: [&str; 7] = [OBJECT, ARRAY, STRING, NUMBER, INTEGER, BOOLEAN, NULL];
const TYPE_NAMES: &str = "object, array, string, number, integer, boolean or null";
const EXPECTED: &str = ": expected ";
const MUST_BE: &str = ": must be ";

/// The types a type-specific keyword applies to.
fn applies_to(key: &str) -> Option<&'static [&'static str]> {
    match key {
        PROPERTIES | REQUIRED | ADDITIONAL_PROPERTIES => Some(&[OBJECT]),
        ITEMS | MIN_ITEMS | MAX_ITEMS => Some(&[ARRAY]),
        MIN_LENGTH | MAX_LENGTH | PATTERN => Some(&[STRING]),
        MINIMUM | MAXIMUM | EXCLUSIVE_MINIMUM | EXCLUSIVE_MAXIMUM => Some(&[NUMBER, INTEGER]),
        _ => None,
    }
}

/// A compiled tool input schema.
#[derive(Debug)]
pub struct ToolSchema {
    types: Vec<String>,
    property_schemas: HashMap<String, ToolSchema>,
    required_names: Vec<String>,
    no_additional: bool,
    additional: Option<Box<ToolSchema>>,
    item_schema: Option<Box<ToolSchema>>,
    enum_values: Vec<Value>,
    has_const: bool,
    const_value: Value,
    lower_bound: Option<Decimal>,
    upper_bound: Option<Decimal>,
    exclusive_lower_bound: Option<Decimal>,
    exclusive_upper_bound: Option<Decimal>,
    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<(String, Regex)>,
    min_items: Option<usize>,
    max_items: Option<usize>,
}

impl ToolSchema {
    /// Compile a schema.
    ///
    /// `place` is the schema's place, named in an error (e.g. `tools[0].input_schema`). The error
    /// names the first problem.
    pub fn compile(place: &str, schema: &Value) -> Result<ToolSchema, String> {
        if matches!(schema, Value::Map(_)) {
            ToolSchema::new(place, schema)
        } else {
            Err(format!("{place} must be a JSON schema object"))
        }
    }

    fn new(place: &str, schema: &Value) -> Result<ToolSchema, String> {
        check_keywords(place, schema)?;
        let types = compile_type(place, get(schema, TYPE))?;
        check_applicable(place, schema, &types)?;
        let property_schemas = compile_properties(place, get(schema, PROPERTIES))?;
        let required_names = compile_required(place, get(schema, REQUIRED))?;
        let extra = get(schema, ADDITIONAL_PROPERTIES);
        let no_additional = matches!(extra, Some(Value::Boolean(false)));
        let additional = match extra {
            None | Some(Value::Boolean(_)) => None,
            Some(value) => Some(Box::new(ToolSchema::compile(
                &format!("{place}.{ADDITIONAL_PROPERTIES}"),
                value,
            )?)),
        };
        let item_schema = match get(schema, ITEMS) {
            None => None,
            Some(items) => Some(Box::new(ToolSchema::compile(
                &format!("{place}.{ITEMS}"),
                items,
            )?)),
        };
        let enum_values = compile_enum(place, get(schema, ENUM))?;
        let has_const = contains_key(schema, CONST);
        let const_value = get(schema, CONST).cloned().unwrap_or(Value::Nil);
        let lower_bound = number(place, MINIMUM, get(schema, MINIMUM))?;
        let upper_bound = number(place, MAXIMUM, get(schema, MAXIMUM))?;
        let exclusive_lower_bound =
            number(place, EXCLUSIVE_MINIMUM, get(schema, EXCLUSIVE_MINIMUM))?;
        let exclusive_upper_bound =
            number(place, EXCLUSIVE_MAXIMUM, get(schema, EXCLUSIVE_MAXIMUM))?;
        let min_length = count(place, MIN_LENGTH, get(schema, MIN_LENGTH))?;
        let max_length = count(place, MAX_LENGTH, get(schema, MAX_LENGTH))?;
        let min_items = count(place, MIN_ITEMS, get(schema, MIN_ITEMS))?;
        let max_items = count(place, MAX_ITEMS, get(schema, MAX_ITEMS))?;
        let pattern = match get(schema, PATTERN) {
            None => None,
            Some(Value::String(text)) => {
                let text = text.as_str().unwrap_or_default().to_string();
                let compiled = compile_pattern(place, &text)?;
                Some((text, compiled))
            }
            Some(_) => return Err(format!("{place}.{PATTERN} must be a regular expression")),
        };
        Ok(ToolSchema {
            types,
            property_schemas,
            required_names,
            no_additional,
            additional,
            item_schema,
            enum_values,
            has_const,
            const_value,
            lower_bound,
            upper_bound,
            exclusive_lower_bound,
            exclusive_upper_bound,
            min_length,
            max_length,
            pattern,
            min_items,
            max_items,
        })
    }

    /// The declared types, empty when the schema declares none.
    pub fn types(&self) -> &[String] {
        &self.types
    }

    /// Validate a value, collecting every violation in a fixed order: the type, then enum and const,
    /// then the keywords of the value's type. An object reports its missing required properties in
    /// the order declared, then its properties in name order.
    ///
    /// `path` is the value's path, e.g. `input`.
    pub fn validate(&self, path: &str, value: &Value, violations: &mut Vec<String>) {
        let actual = kind_of(value);
        if !self.types.is_empty() && !self.matches_type(actual) {
            violations.push(format!(
                "{path}{EXPECTED}{}, got {actual}",
                self.type_text()
            ));
            return;
        }
        if !self.enum_values.is_empty() && !self.enum_values.iter().any(|v| json_equals(v, value)) {
            violations.push(format!(
                "{path}: must be one of {}",
                compact(&Value::Array(self.enum_values.clone()))
            ));
        }
        if self.has_const && !json_equals(&self.const_value, value) {
            violations.push(format!("{path}{MUST_BE}{}", compact(&self.const_value)));
        }
        match (actual, value) {
            (INTEGER | NUMBER, _) => {
                if let Some(n) = Decimal::of(value) {
                    self.check_bounds(path, &n, violations);
                }
            }
            (STRING, Value::String(text)) => {
                self.check_string(path, text.as_str().unwrap_or_default(), violations)
            }
            (ARRAY, Value::Array(list)) => self.check_array(path, list, violations),
            (OBJECT, _) => self.check_object(path, value, violations),
            // null and boolean have no keywords of their own
            _ => {}
        }
    }

    fn matches_type(&self, actual: &str) -> bool {
        self.types.iter().any(|t| t == actual)
            || (actual == INTEGER && self.types.iter().any(|t| t == NUMBER))
    }

    fn type_text(&self) -> String {
        match self.types.split_last() {
            Some((last, rest)) if !rest.is_empty() => format!("{} or {last}", rest.join(", ")),
            Some((last, _)) => last.clone(),
            None => String::new(),
        }
    }

    fn check_bounds(&self, path: &str, n: &Decimal, violations: &mut Vec<String>) {
        if let Some(bound) = &self.lower_bound {
            if n < bound {
                violations.push(format!("{path}{MUST_BE}>= {}", bound.render()));
            }
        }
        if let Some(bound) = &self.upper_bound {
            if n > bound {
                violations.push(format!("{path}{MUST_BE}<= {}", bound.render()));
            }
        }
        if let Some(bound) = &self.exclusive_lower_bound {
            if n <= bound {
                violations.push(format!("{path}{MUST_BE}> {}", bound.render()));
            }
        }
        if let Some(bound) = &self.exclusive_upper_bound {
            if n >= bound {
                violations.push(format!("{path}{MUST_BE}< {}", bound.render()));
            }
        }
    }

    fn check_string(&self, path: &str, text: &str, violations: &mut Vec<String>) {
        let length = text.chars().count();
        if let Some(min) = self.min_length {
            if length < min {
                violations.push(format!("{path}: must be at least {min} characters"));
            }
        }
        if let Some(max) = self.max_length {
            if length > max {
                violations.push(format!("{path}: must be at most {max} characters"));
            }
        }
        if let Some((text_of_pattern, compiled)) = &self.pattern {
            if !compiled.is_match(text) {
                violations.push(format!(
                    "{path}: must match the pattern '{text_of_pattern}'"
                ));
            }
        }
    }

    fn check_array(&self, path: &str, list: &[Value], violations: &mut Vec<String>) {
        if let Some(min) = self.min_items {
            if list.len() < min {
                violations.push(format!("{path}: must have at least {min} items"));
            }
        }
        if let Some(max) = self.max_items {
            if list.len() > max {
                violations.push(format!("{path}: must have at most {max} items"));
            }
        }
        if let Some(items) = &self.item_schema {
            for (i, item) in list.iter().enumerate() {
                items.validate(&format!("{path}[{i}]"), item, violations);
            }
        }
    }

    fn check_object(&self, path: &str, map: &Value, violations: &mut Vec<String>) {
        for name in &self.required_names {
            if !contains_key(map, name) {
                violations.push(format!("{path}.{name}: is required"));
            }
        }
        for name in sorted_keys(map) {
            let item = last_value(map, &name);
            if let Some(declared) = self.property_schemas.get(&name) {
                declared.validate(&format!("{path}.{name}"), item, violations);
            } else if self.no_additional {
                violations.push(format!("{path}.{name}: is not a declared property"));
            } else if let Some(additional) = &self.additional {
                additional.validate(&format!("{path}.{name}"), item, violations);
            }
        }
    }

    /// Render violations as one message, capped at [`VIOLATION_CAP`].
    pub fn report(violations: &[String]) -> String {
        if violations.len() <= VIOLATION_CAP {
            return violations.join("; ");
        }
        format!(
            "{}; and {} more",
            violations[..VIOLATION_CAP].join("; "),
            violations.len() - VIOLATION_CAP
        )
    }
}

/// A map's value under a key as text (a non-text key matched by its text), the last one when the key
/// repeats; nil when absent.
fn last_value<'a>(map: &'a Value, name: &str) -> &'a Value {
    const NIL: &Value = &Value::Nil;
    entries(map)
        .iter()
        .rev()
        .find(|(key, _)| key_text(key) == name)
        .map(|(_, value)| value)
        .unwrap_or(NIL)
}

/// Keys in name order, so both engines name the same first problem whatever order a map keeps.
fn check_keywords(place: &str, schema: &Value) -> Result<(), String> {
    for key in sorted_keys(schema) {
        if !KEYWORDS.contains(&key.as_str()) && !DOCUMENTARY.contains(&key.as_str()) {
            return Err(format!(
                "{place}: unknown keyword '{key}' - the vocabulary is {} ({} are documentary)",
                and_list(&KEYWORDS),
                and_list(&DOCUMENTARY)
            ));
        }
    }
    Ok(())
}

fn compile_type(place: &str, type_: Option<&Value>) -> Result<Vec<String>, String> {
    let Some(type_) = type_ else {
        return Ok(Vec::new());
    };
    let names = match type_ {
        Value::Array(list) => type_names(place, list)?,
        other => type_names(place, std::slice::from_ref(other))?,
    };
    if names.is_empty() {
        return Err(format!(
            "{place}.{TYPE} must be {TYPE_NAMES}, or a list of them"
        ));
    }
    Ok(names)
}

fn type_names(place: &str, names: &[Value]) -> Result<Vec<String>, String> {
    let mut result: Vec<String> = Vec::new();
    for name in names {
        let text = match name.as_str() {
            Some(text) if KNOWN_TYPES.contains(&text) => text,
            _ => {
                return Err(format!(
                    "{place}.{TYPE} must be {TYPE_NAMES}, or a list of them"
                ))
            }
        };
        if result.iter().any(|t| t == text) {
            return Err(format!("{place}.{TYPE} lists '{text}' twice"));
        }
        result.push(text.to_string());
    }
    Ok(result)
}

/// A keyword that applies to none of the declared types is refused: it would never constrain
/// anything.
fn check_applicable(place: &str, schema: &Value, types: &[String]) -> Result<(), String> {
    if types.is_empty() {
        return Ok(());
    }
    for key in sorted_keys(schema) {
        if let Some(needs) = applies_to(&key) {
            if !needs.iter().any(|need| types.iter().any(|t| t == need)) {
                return Err(format!(
                    "{place}.{key} applies to the type {}, which the schema does not declare",
                    needs.join(" or ")
                ));
            }
        }
    }
    Ok(())
}

fn compile_properties(
    place: &str,
    props: Option<&Value>,
) -> Result<HashMap<String, ToolSchema>, String> {
    let mut result = HashMap::new();
    let Some(props) = props else {
        return Ok(result);
    };
    if !matches!(props, Value::Map(_)) {
        return Err(format!("{place}.{PROPERTIES} must be a map of schemas"));
    }
    // a TreeMap: compiled in name order, the last value of a repeated key
    for name in sorted_keys(props) {
        let schema = ToolSchema::compile(
            &format!("{place}.{PROPERTIES}.{name}"),
            last_value(props, &name),
        )?;
        result.insert(name, schema);
    }
    Ok(result)
}

fn compile_required(place: &str, required: Option<&Value>) -> Result<Vec<String>, String> {
    let mut result: Vec<String> = Vec::new();
    let Some(required) = required else {
        return Ok(result);
    };
    let Value::Array(names) = required else {
        return Err(format!(
            "{place}.{REQUIRED} must be a list of property names"
        ));
    };
    for name in names {
        let Some(text) = name.as_str() else {
            return Err(format!(
                "{place}.{REQUIRED} must be a list of property names"
            ));
        };
        if result.iter().any(|n| n == text) {
            return Err(format!("{place}.{REQUIRED} lists '{text}' twice"));
        }
        result.push(text.to_string());
    }
    Ok(result)
}

fn compile_enum(place: &str, values: Option<&Value>) -> Result<Vec<Value>, String> {
    match values {
        None => Ok(Vec::new()),
        Some(Value::Array(list)) if !list.is_empty() => Ok(list.clone()),
        Some(_) => Err(format!("{place}.{ENUM} must be a non-empty list")),
    }
}

fn number(place: &str, key: &str, value: Option<&Value>) -> Result<Option<Decimal>, String> {
    match value {
        None => Ok(None),
        Some(value) => match Decimal::of(value) {
            Some(n) => Ok(Some(n)),
            None => Err(format!("{place}.{key} must be a number")),
        },
    }
}

fn count(place: &str, key: &str, value: Option<&Value>) -> Result<Option<usize>, String> {
    let Some(value) = value else {
        return Ok(None);
    };
    match Decimal::of(value).and_then(|n| n.whole()) {
        Some(n) if (0..=i32::MAX as i64).contains(&n) => Ok(Some(n as usize)),
        _ => Err(format!("{place}.{key} must be a whole number >= 0")),
    }
}

fn compile_pattern(place: &str, text: &str) -> Result<Regex, String> {
    if outside_common_subset(text) {
        return Err(format!(
            "{place}.{PATTERN}: '{text}' uses lookaround, an atomic group, a backreference or a \
             possessive quantifier - outside the regex subset common to both engines"
        ));
    }
    // the regex crate's classes are Unicode-aware by default (Java: UNICODE_CHARACTER_CLASS)
    Regex::new(text)
        .map_err(|_| format!("{place}.{PATTERN}: '{text}' is not a valid regular expression"))
}

/// Lookaround, atomic groups, backreferences and possessive quantifiers are Java-only constructs: the
/// Rust engine's regex crate has none of them, so a pattern that uses one is refused on both engines
/// by this textual check (escapes and character classes are skipped). The same rule as the graph
/// contract's. Byte-wise: every character it looks for is ASCII, and no byte of a multi-byte UTF-8
/// character is.
pub(crate) fn outside_common_subset(p: &str) -> bool {
    let b = p.as_bytes();
    let mut in_class = false;
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c == b'\\' {
            if !in_class && is_backreference(b, i) {
                return true;
            }
            i += 2;
        } else {
            if in_class {
                in_class = c != b']';
            } else if c == b'[' {
                in_class = true;
            } else if is_java_only_group(b, i) || is_possessive(b, i) {
                return true;
            }
            i += 1;
        }
    }
    false
}

fn is_backreference(b: &[u8], i: usize) -> bool {
    match b.get(i + 1) {
        Some(next) => (b'1'..=b'9').contains(next) || *next == b'k',
        None => false,
    }
}

fn is_java_only_group(b: &[u8], i: usize) -> bool {
    if !b[i..].starts_with(b"(?") {
        return false;
    }
    let rest = &b[i + 2..];
    rest.starts_with(b"=")
        || rest.starts_with(b"!")
        || rest.starts_with(b"<=")
        || rest.starts_with(b"<!")
        || rest.starts_with(b">")
}

fn is_possessive(b: &[u8], i: usize) -> bool {
    if b[i] != b'+' || i == 0 {
        return false;
    }
    matches!(b[i - 1], b'*' | b'+' | b'?' | b'}') && !escaped(b, i - 1)
}

fn escaped(b: &[u8], index: usize) -> bool {
    let mut backslashes = 0;
    let mut i = index;
    while i > 0 && b[i - 1] == b'\\' {
        backslashes += 1;
        i -= 1;
    }
    backslashes % 2 == 1
}

// ------------------------------------------------------------------ JSON values

/// What a decoded JSON value is: null, boolean, integer (a number with no fractional part), number,
/// string, array or object. A MessagePack binary is named `byte[]`, as the Java engine names the
/// class it decodes one to.
pub(crate) fn kind_of(value: &Value) -> &'static str {
    match value {
        Value::Nil => NULL,
        Value::Boolean(_) => BOOLEAN,
        Value::Integer(_) => INTEGER,
        Value::F32(_) | Value::F64(_) => match Decimal::of(value) {
            Some(n) if n.is_integral() => INTEGER,
            _ => NUMBER,
        },
        Value::String(_) => STRING,
        Value::Array(_) => ARRAY,
        Value::Map(_) => OBJECT,
        Value::Binary(_) => "byte[]",
        Value::Ext(_, _) => "ext",
    }
}

/// JSON equality: numbers by value, lists in order, maps by keys and values. A boolean never equals a
/// number.
pub fn json_equals(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Nil, _) | (_, Value::Nil) => false,
        (
            Value::Integer(_) | Value::F32(_) | Value::F64(_),
            Value::Integer(_) | Value::F32(_) | Value::F64(_),
        ) => match (Decimal::of(a), Decimal::of(b)) {
            (Some(x), Some(y)) => x == y,
            _ => a == b,
        },
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(p, q)| json_equals(p, q))
        }
        (Value::Map(_), Value::Map(_)) => map_equals(a, b),
        (Value::String(x), Value::String(y)) => x.as_bytes() == y.as_bytes(),
        (Value::Boolean(x), Value::Boolean(y)) => x == y,
        (Value::Binary(x), Value::Binary(y)) => x == y,
        (Value::Ext(t, x), Value::Ext(u, y)) => t == u && x == y,
        _ => false,
    }
}

fn map_equals(x: &Value, y: &Value) -> bool {
    let x_keys = sorted_keys(x);
    let y_keys = sorted_keys(y);
    x_keys == y_keys
        && x_keys
            .iter()
            .all(|key| json_equals(last_value(x, key), last_value(y, key)))
}

// ------------------------------------------------------------------ exact decimals

/// An exact decimal, the Java engine's `BigDecimal` as the loop uses it: `(-1)^negative * digits *
/// 10^exponent`, with no leading or trailing zeros in `digits` (an empty `digits` is zero). Built from
/// an integer's digits or from the shortest decimal text of a finite float.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Decimal {
    negative: bool,
    digits: Vec<u8>,
    exponent: i64,
}

impl Decimal {
    /// A number's exact decimal; `None` for any other value and for a float that is not finite.
    pub(crate) fn of(value: &Value) -> Option<Decimal> {
        match value {
            Value::Integer(n) => match (n.as_u64(), n.as_i64()) {
                (Some(v), _) => Some(Decimal::from_parts(false, &v.to_string(), 0)),
                (None, Some(v)) => {
                    Some(Decimal::from_parts(v < 0, &v.unsigned_abs().to_string(), 0))
                }
                (None, None) => None,
            },
            // Rust prints a float as the shortest decimal that round-trips, never with an exponent
            Value::F64(f) if f.is_finite() => Decimal::parse(&f.to_string()),
            Value::F32(f) if f.is_finite() => Decimal::parse(&f.to_string()),
            _ => None,
        }
    }

    /// Decimal text: an optional sign, digits with an optional point, an optional exponent.
    pub(crate) fn parse(text: &str) -> Option<Decimal> {
        let (negative, rest) = match text.as_bytes().first() {
            Some(b'-') => (true, &text[1..]),
            Some(b'+') => (false, &text[1..]),
            _ => (false, text),
        };
        let (mantissa, exponent) = match rest.find(['e', 'E']) {
            Some(at) => (&rest[..at], rest[at + 1..].parse::<i64>().ok()?),
            None => (rest, 0),
        };
        let (whole, fraction) = match mantissa.find('.') {
            Some(at) => (&mantissa[..at], &mantissa[at + 1..]),
            None => (mantissa, ""),
        };
        let all_digits = |s: &str| s.bytes().all(|c| c.is_ascii_digit());
        if (whole.is_empty() && fraction.is_empty()) || !all_digits(whole) || !all_digits(fraction)
        {
            return None;
        }
        Some(Decimal::from_parts(
            negative,
            &format!("{whole}{fraction}"),
            exponent - fraction.len() as i64,
        ))
    }

    fn from_parts(negative: bool, digits: &str, exponent: i64) -> Decimal {
        let lead = digits.trim_start_matches('0');
        let trimmed = lead.trim_end_matches('0');
        if trimmed.is_empty() {
            return Decimal {
                negative: false,
                digits: Vec::new(),
                exponent: 0,
            };
        }
        Decimal {
            negative,
            digits: trimmed.as_bytes().to_vec(),
            exponent: exponent + (lead.len() - trimmed.len()) as i64,
        }
    }

    fn signum(&self) -> i8 {
        match (self.digits.is_empty(), self.negative) {
            (true, _) => 0,
            (false, true) => -1,
            (false, false) => 1,
        }
    }

    /// No fractional part (`BigDecimal.stripTrailingZeros().scale() <= 0`).
    pub(crate) fn is_integral(&self) -> bool {
        self.exponent >= 0
    }

    /// The value as a whole number, when it is integral and fits in 18 digits.
    pub(crate) fn whole(&self) -> Option<i64> {
        if !self.is_integral() || self.digits.len() as i64 + self.exponent > 18 {
            return None;
        }
        let mut n: i64 = 0;
        for d in &self.digits {
            n = n * 10 + i64::from(d - b'0');
        }
        for _ in 0..self.exponent {
            n *= 10;
        }
        Some(if self.negative { -n } else { n })
    }

    /// A number as both engines print it: no exponent, no trailing zeros, no decimal point when whole.
    pub(crate) fn render(&self) -> String {
        if self.digits.is_empty() {
            return "0".to_string();
        }
        let digits = String::from_utf8_lossy(&self.digits);
        let body = if self.exponent >= 0 {
            format!("{digits}{}", "0".repeat(self.exponent as usize))
        } else {
            let point = self.digits.len() as i64 + self.exponent;
            if point > 0 {
                let (whole, fraction) = digits.split_at(point as usize);
                format!("{whole}.{fraction}")
            } else {
                format!("0.{}{digits}", "0".repeat((-point) as usize))
            }
        };
        if self.negative {
            format!("-{body}")
        } else {
            body
        }
    }

    fn magnitude_cmp(&self, other: &Decimal) -> Ordering {
        let a = self.digits.len() as i64 + self.exponent;
        let b = other.digits.len() as i64 + other.exponent;
        // the position of the most significant digit first, then the digits; with no trailing zeros
        // a longer run of equal leading digits is the larger magnitude
        a.cmp(&b).then_with(|| self.digits.cmp(&other.digits))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let (a, b) = (self.signum(), other.signum());
        if a != b || a == 0 {
            return a.cmp(&b);
        }
        let magnitude = self.magnitude_cmp(other);
        if a < 0 {
            magnitude.reverse()
        } else {
            magnitude
        }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(text: &str) -> Decimal {
        Decimal::parse(text).expect("decimal")
    }

    #[test]
    fn decimals_are_exact() {
        assert_eq!(d("1000.0").render(), "1000");
        assert_eq!(d("-0.00150").render(), "-0.0015");
        assert_eq!(d("1.5e3").render(), "1500");
        assert_eq!(d("-0").render(), "0");
        assert!(d("2.0").is_integral());
        assert!(!d("2.5").is_integral());
        assert!(d("1.5") > d("1"));
        assert!(d("-1.5") < d("-1"));
        assert!(d("0.1") < d("0.12"));
        assert!(d("-3") < d("0"));
        assert_eq!(d("10").cmp(&d("1e1")), Ordering::Equal);
        assert_eq!(d("9007199254740993").whole(), Some(9_007_199_254_740_993));
        assert!(
            Decimal::of(&Value::from(9_007_199_254_740_993u64))
                > Decimal::of(&Value::F64(9_007_199_254_740_992.0))
        );
        assert_eq!(
            Decimal::of(&Value::F64(0.1)).map(|n| n.render()).as_deref(),
            Some("0.1")
        );
    }

    #[test]
    fn the_common_regex_subset() {
        assert!(outside_common_subset("a++"));
        assert!(outside_common_subset("^(?=C-)"));
        assert!(outside_common_subset("(a)\\1"));
        assert!(!outside_common_subset("a\\++"));
        assert!(!outside_common_subset("[(?=]"));
        assert!(!outside_common_subset("^C-[0-9]+$"));
    }
}
