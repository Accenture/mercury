//! The graph contract's schema vocabulary (RFC-0007, WP2; Java `GraphSchema`):
//! a closed subset of OpenAPI 3.0 keywords, compiled from the `schema` property
//! of a root or end node and, for the root, validated against a run's input by
//! the built-in `graph.schema.validator` function ([`validate_request`]).
//!
//! The vocabulary is `type` (object, array, string, number, integer, boolean),
//! `properties`, `required`, `items`, `enum`, `minimum`, `maximum`,
//! `exclusiveMinimum`, `exclusiveMaximum` (booleans, as in OpenAPI 3.0),
//! `minLength`, `maxLength`, `pattern`, `minItems`, `maxItems`, `nullable` and
//! `additionalProperties`; `title`, `description`, `example` and `format` are
//! documentary and never validated. Any other keyword is refused by the
//! compiler, because a constraint the validator silently ignores teaches that
//! unflagged means safe; a keyword that cannot apply to the declared type is
//! refused for the same reason.
//!
//! Types are strict JSON types: a numeric string is not a number, a boolean is
//! not a number, and an integer is a number with no fractional part (so `1.0`
//! is one). A value the grammar stored as text - the Playground writes every
//! scalar property as text - is accepted where the schema expects a number or
//! a boolean (`minimum=0`, `nullable=true`), and an `enum` entry is read as the
//! declared type. A header is text: the header part's property schemas take
//! the types string, number, integer and boolean, and a number or boolean
//! header validates the parsed text. A `pattern` is searched, not anchored,
//! with this crate's Unicode-aware classes (the Java engine's
//! `UNICODE_CHARACTER_CLASS`), and must stay inside the subset common to both
//! engines (no lookaround, atomic group, backreference or possessive
//! quantifier). Character counts are code points.
//!
//! Every violation is collected - the names are the paths of the state machine
//! (`input.body.items[2].sku`, `input.header.X-Api-Key`), a header name as
//! declared - and [`report`] renders them as one message capped at
//! [`VIOLATION_CAP`]. The messages are pinned by the shared vector file
//! `graph-schema-vectors.json`, byte-identical in the Java repository.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use event_script::conversions::display;
use platform_core::{AppError, EventEnvelope};
use regex::Regex;
use rmpv::Value;

/// The route of the built-in validator.
pub const ROUTE: &str = "graph.schema.validator";
/// The number of violations one message carries; the rest are counted.
pub const VIOLATION_CAP: usize = 10;
pub const BODY: &str = "body";
pub const HEADER: &str = "header";
const SCHEMA: &str = "schema";
const TYPE: &str = "type";
const PROPERTIES: &str = "properties";
const REQUIRED: &str = "required";
const ITEMS: &str = "items";
const ENUM: &str = "enum";
const MINIMUM: &str = "minimum";
const MAXIMUM: &str = "maximum";
const EXCLUSIVE_MINIMUM: &str = "exclusiveMinimum";
const EXCLUSIVE_MAXIMUM: &str = "exclusiveMaximum";
const MIN_LENGTH: &str = "minLength";
const MAX_LENGTH: &str = "maxLength";
const PATTERN: &str = "pattern";
const MIN_ITEMS: &str = "minItems";
const MAX_ITEMS: &str = "maxItems";
const NULLABLE: &str = "nullable";
const ADDITIONAL_PROPERTIES: &str = "additionalProperties";
const OBJECT: &str = "object";
const ARRAY: &str = "array";
const STRING: &str = "string";
const NUMBER: &str = "number";
const INTEGER: &str = "integer";
const BOOLEAN: &str = "boolean";
/// The vocabulary, in the order the error message lists it.
pub const KEYWORDS: [&str; 16] = [
    TYPE,
    PROPERTIES,
    REQUIRED,
    ITEMS,
    ENUM,
    MINIMUM,
    MAXIMUM,
    EXCLUSIVE_MINIMUM,
    EXCLUSIVE_MAXIMUM,
    MIN_LENGTH,
    MAX_LENGTH,
    PATTERN,
    MIN_ITEMS,
    MAX_ITEMS,
    NULLABLE,
    ADDITIONAL_PROPERTIES,
];
/// Keywords that document and never validate.
pub const DOCUMENTARY: [&str; 4] = ["title", "description", "example", "format"];
const TYPES: [&str; 6] = [OBJECT, ARRAY, STRING, NUMBER, INTEGER, BOOLEAN];
const HEADER_TYPES: [&str; 4] = [STRING, NUMBER, INTEGER, BOOLEAN];
const OBJECT_KEYWORDS: [&str; 3] = [PROPERTIES, REQUIRED, ADDITIONAL_PROPERTIES];
const ARRAY_KEYWORDS: [&str; 3] = [ITEMS, MIN_ITEMS, MAX_ITEMS];
const STRING_KEYWORDS: [&str; 3] = [MIN_LENGTH, MAX_LENGTH, PATTERN];
const NUMBER_KEYWORDS: [&str; 4] = [MINIMUM, MAXIMUM, EXCLUSIVE_MINIMUM, EXCLUSIVE_MAXIMUM];
/// The header part itself takes only these (plus the documentary keywords).
const HEADER_PART_KEYWORDS: [&str; 3] = [TYPE, PROPERTIES, REQUIRED];
/// A header property never takes these: a header is one text value.
const NOT_FOR_A_HEADER: [&str; 7] = [
    PROPERTIES,
    REQUIRED,
    ITEMS,
    MIN_ITEMS,
    MAX_ITEMS,
    ADDITIONAL_PROPERTIES,
    NULLABLE,
];
const TYPE_NAMES: &str = "object, array, string, number, integer or boolean";
const HEADER_TYPE_NAMES: &str = "string, number, integer or boolean";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Body,
    HeaderPart,
    HeaderProperty,
}

/// A schema number: an integer as i128, anything else as f64 - the Java
/// engine's exact decimals agree on every value a JSON body carries.
#[derive(Clone, Copy, Debug)]
enum Num {
    Int(i128),
    Float(f64),
}

impl Num {
    fn of(value: &Value) -> Option<Num> {
        match value {
            Value::Integer(i) => i
                .as_i64()
                .map(|v| Num::Int(v as i128))
                .or_else(|| i.as_u64().map(|v| Num::Int(v as i128))),
            Value::F64(f) if f.is_finite() => Some(Num::Float(*f)),
            Value::F32(f) if f.is_finite() => Some(Num::Float(*f as f64)),
            Value::String(s) => Num::parse(s.as_str().unwrap_or("")),
            _ => None,
        }
    }

    fn parse(text: &str) -> Option<Num> {
        let t = text.trim();
        if t.is_empty() {
            return None;
        }
        if let Ok(i) = t.parse::<i128>() {
            return Some(Num::Int(i));
        }
        match t.parse::<f64>() {
            Ok(f) if f.is_finite() => Some(Num::Float(f)),
            _ => None,
        }
    }

    fn is_integral(&self) -> bool {
        match self {
            Num::Int(_) => true,
            Num::Float(f) => f.fract() == 0.0,
        }
    }

    fn compare(&self, other: &Num) -> Ordering {
        match (self, other) {
            (Num::Int(a), Num::Int(b)) => a.cmp(b),
            _ => self
                .as_f64()
                .partial_cmp(&other.as_f64())
                .unwrap_or(Ordering::Equal),
        }
    }

    fn as_f64(&self) -> f64 {
        match self {
            Num::Int(i) => *i as f64,
            Num::Float(f) => *f,
        }
    }

    /// Rendered as both engines render it: plain notation, no trailing zeros.
    fn render(&self) -> String {
        match self {
            Num::Int(i) => i.to_string(),
            Num::Float(f) => {
                if f.fract() == 0.0 && f.abs() < 1e18 {
                    format!("{}", *f as i128)
                } else {
                    format!("{f}")
                }
            }
        }
    }
}

/// An `enum` entry, read as the declared type (or by its own kind when no
/// type is declared).
#[derive(Clone, Debug)]
enum Entry {
    Num(Num),
    Text(String),
    Flag(bool),
    Raw(Value),
}

/// A compiled schema object.
pub struct Schema {
    type_: Option<String>,
    properties: BTreeMap<String, Schema>,
    required: Vec<String>,
    items: Option<Box<Schema>>,
    enum_values: Option<Vec<Entry>>,
    minimum: Option<Num>,
    maximum: Option<Num>,
    exclusive_minimum: bool,
    exclusive_maximum: bool,
    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<(String, Regex)>,
    min_items: Option<usize>,
    max_items: Option<usize>,
    nullable: bool,
    no_additional: bool,
    additional: Option<Box<Schema>>,
}

/// The two parts of a node's `schema` property, compiled.
pub struct Contract {
    /// The body part, or None when the node declares none.
    pub body: Option<Schema>,
    /// The header part, or None when the node declares none.
    pub header: Option<Schema>,
}

impl Contract {
    /// Validate a run's input against the declaration: every violation, in
    /// order; empty when the input passes.
    pub fn check(&self, body: Option<&Value>, header: Option<&Value>) -> Vec<String> {
        let mut violations = Vec::new();
        if let Some(schema) = &self.body {
            schema.validate(&format!("input.{BODY}"), body, &mut violations);
        }
        if let Some(schema) = &self.header {
            schema.validate_headers(&format!("input.{HEADER}"), header, &mut violations);
        }
        violations
    }
}

/// Compile a node's `schema` property: an object with a `body` part and/or a
/// `header` part. The error names the reason and its location.
pub fn compile_contract(schema: &Value) -> Result<Contract, String> {
    let Value::Map(parts) = schema else {
        return Err("schema: must be an object with body and/or header".to_string());
    };
    for (key, _) in parts {
        let part = display(key);
        if part != BODY && part != HEADER {
            return Err(format!(
                "schema: unknown part '{part}' - use body or header"
            ));
        }
    }
    let body = match get(parts, BODY) {
        Some(value) => Some(compile(&format!("schema.{BODY}"), value)?),
        None => None,
    };
    let header = match get(parts, HEADER) {
        Some(value) => Some(compile_header_part(&format!("schema.{HEADER}"), value)?),
        None => None,
    };
    Ok(Contract { body, header })
}

/// Compile a schema object; `where_` is the location for error messages, e.g.
/// `schema.body`.
pub fn compile(where_: &str, schema: &Value) -> Result<Schema, String> {
    Schema::compile(where_, schema, Mode::Body)
}

/// Compile the header part: an object schema whose properties are header names.
pub fn compile_header_part(where_: &str, schema: &Value) -> Result<Schema, String> {
    Schema::compile(where_, schema, Mode::HeaderPart)
}

/// Render the violations of one run as the validator's message:
/// `Input validation failed - a; b; ...`, capped at [`VIOLATION_CAP`] with the
/// rest counted.
pub fn report(violations: &[String]) -> String {
    let shown = if violations.len() > VIOLATION_CAP {
        &violations[..VIOLATION_CAP]
    } else {
        violations
    };
    let mut message = format!("Input validation failed - {}", shown.join("; "));
    if violations.len() > VIOLATION_CAP {
        message.push_str(&format!("; and {} more", violations.len() - VIOLATION_CAP));
    }
    message
}

/// The built-in input validator (Java `GraphSchemaValidator`): the request body
/// is `{body, header, schema}`; 200 passes with `{valid: true}`, 400 carries
/// every violation in one message, and a schema the compiler refuses answers
/// 400 with `Invalid schema - ` and the reason.
pub async fn validate_request(
    _headers: HashMap<String, String>,
    input: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    const NO_SCHEMA: &str = "Invalid schema - schema: must be an object with body and/or header";
    let Value::Map(entries) = input.body() else {
        return Err(AppError::new(400, NO_SCHEMA));
    };
    let Some(schema) = get(entries, SCHEMA) else {
        return Err(AppError::new(400, NO_SCHEMA));
    };
    let contract = compile_contract(schema)
        .map_err(|e| AppError::new(400, format!("Invalid schema - {e}")))?;
    let violations = contract.check(get(entries, BODY), get(entries, HEADER));
    if !violations.is_empty() {
        return Err(AppError::new(400, report(&violations)));
    }
    Ok(EventEnvelope::new()
        .set_raw_body(Value::Map(vec![(Value::from("valid"), Value::from(true))])))
}

fn get<'a>(map: &'a [(Value, Value)], key: &str) -> Option<&'a Value> {
    map.iter().find(|(k, _)| display(k) == key).map(|(_, v)| v)
}

fn has(map: &[(Value, Value)], key: &str) -> bool {
    map.iter().any(|(k, _)| display(k) == key)
}

fn unknown_keyword(where_: &str, keyword: &str) -> String {
    format!(
        "{where_}: unknown keyword '{keyword}' - the vocabulary is {}, {} and {} ({}, {} and {} are documentary)",
        KEYWORDS[..KEYWORDS.len() - 2].join(", "),
        KEYWORDS[KEYWORDS.len() - 2],
        KEYWORDS[KEYWORDS.len() - 1],
        DOCUMENTARY[..DOCUMENTARY.len() - 2].join(", "),
        DOCUMENTARY[DOCUMENTARY.len() - 2],
        DOCUMENTARY[DOCUMENTARY.len() - 1]
    )
}

fn is_flag(value: &Value) -> bool {
    match value {
        Value::Boolean(_) => true,
        Value::String(s) => matches!(s.as_str(), Some("true") | Some("false")),
        _ => false,
    }
}

fn flag(where_: &str, keyword: &str, value: &Value) -> Result<bool, String> {
    match value {
        Value::Boolean(b) => Ok(*b),
        Value::String(s) if s.as_str() == Some("true") => Ok(true),
        Value::String(s) if s.as_str() == Some("false") => Ok(false),
        _ => Err(format!("{where_}.{keyword}: must be true or false")),
    }
}

fn number(where_: &str, keyword: &str, value: &Value) -> Result<Num, String> {
    match value {
        Value::Boolean(_) => Err(format!("{where_}.{keyword}: must be a number")),
        _ => Num::of(value).ok_or_else(|| format!("{where_}.{keyword}: must be a number")),
    }
}

fn count(where_: &str, keyword: &str, value: &Value) -> Result<usize, String> {
    let error = || format!("{where_}.{keyword}: must be a non-negative integer");
    match value {
        Value::Boolean(_) => Err(error()),
        _ => match Num::of(value) {
            Some(Num::Int(i)) if (0..=i32::MAX as i128).contains(&i) => Ok(i as usize),
            Some(n @ Num::Float(_)) if n.is_integral() => {
                let f = n.as_f64();
                if (0.0..=i32::MAX as f64).contains(&f) {
                    Ok(f as usize)
                } else {
                    Err(error())
                }
            }
            _ => Err(error()),
        },
    }
}

fn applies_to(keyword: &str, type_: &str) -> bool {
    if OBJECT_KEYWORDS.contains(&keyword) {
        return type_ == OBJECT;
    }
    if ARRAY_KEYWORDS.contains(&keyword) {
        return type_ == ARRAY;
    }
    if STRING_KEYWORDS.contains(&keyword) {
        return type_ == STRING;
    }
    if NUMBER_KEYWORDS.contains(&keyword) {
        return type_ == NUMBER || type_ == INTEGER;
    }
    true
}

fn text_of(value: &Value) -> String {
    match value {
        Value::String(s) => s.as_str().unwrap_or("").to_string(),
        other => display(other),
    }
}

/// Lookaround, atomic groups, backreferences and possessive quantifiers are
/// Java-only constructs the regex crate has none of, so a pattern that uses one
/// is refused on both engines by this textual check (escapes and character
/// classes are skipped).
fn outside_common_subset(p: &str) -> bool {
    let b = p.as_bytes();
    let mut in_class = false;
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c == b'\\' {
            if i + 1 < b.len() {
                let next = b[i + 1];
                if !in_class && (next.is_ascii_digit() && next != b'0' || next == b'k') {
                    return true;
                }
                i += 1;
            }
            i += 1;
            continue;
        }
        if in_class {
            if c == b']' {
                in_class = false;
            }
            i += 1;
            continue;
        }
        if c == b'[' {
            in_class = true;
        } else if c == b'(' && b[i..].starts_with(b"(?") {
            let rest = &b[i + 2..];
            if rest.starts_with(b"=")
                || rest.starts_with(b"!")
                || rest.starts_with(b"<=")
                || rest.starts_with(b"<!")
                || rest.starts_with(b">")
            {
                return true;
            }
        } else if c == b'+' && i > 0 {
            let previous = b[i - 1];
            if matches!(previous, b'*' | b'+' | b'?' | b'}') && !escaped(b, i - 1) {
                return true;
            }
        }
        i += 1;
    }
    false
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

fn compile_pattern(where_: &str, text: &str) -> Result<Regex, String> {
    if outside_common_subset(text) {
        return Err(format!(
            "{where_}.{PATTERN}: '{text}' uses lookaround, an atomic group, a backreference or a \
             possessive quantifier - outside the regex subset common to both engines"
        ));
    }
    Regex::new(text)
        .map_err(|_| format!("{where_}.{PATTERN}: '{text}' is not a valid regular expression"))
}

fn compile_enum(where_: &str, value: &Value, type_: Option<&str>) -> Result<Vec<Entry>, String> {
    let entries = match value {
        Value::Array(entries) if !entries.is_empty() => entries,
        _ => return Err(format!("{where_}.{ENUM}: must be a non-empty list")),
    };
    let mut result = Vec::with_capacity(entries.len());
    for entry in entries {
        let compiled = match type_ {
            Some(NUMBER) | Some(INTEGER) => {
                let integer = type_ == Some(INTEGER);
                match Num::of(entry) {
                    Some(n)
                        if !matches!(entry, Value::Boolean(_)) && (!integer || n.is_integral()) =>
                    {
                        Entry::Num(n)
                    }
                    _ => {
                        return Err(format!(
                            "{where_}.{ENUM}: '{}' is not {}",
                            display(entry),
                            if integer { "an integer" } else { "a number" }
                        ))
                    }
                }
            }
            Some(BOOLEAN) => {
                if !is_flag(entry) {
                    return Err(format!(
                        "{where_}.{ENUM}: '{}' is not a boolean",
                        display(entry)
                    ));
                }
                Entry::Flag(display(entry) == "true")
            }
            Some(STRING) => Entry::Text(text_of(entry)),
            _ => match entry {
                Value::Integer(_) | Value::F32(_) | Value::F64(_) => match Num::of(entry) {
                    Some(n) => Entry::Num(n),
                    None => Entry::Raw(entry.clone()),
                },
                Value::String(s) => Entry::Text(s.as_str().unwrap_or("").to_string()),
                Value::Boolean(b) => Entry::Flag(*b),
                other => Entry::Raw(other.clone()),
            },
        };
        result.push(compiled);
    }
    Ok(result)
}

fn plural(n: usize, noun: &str) -> String {
    if n == 1 {
        format!("{n} {noun}")
    } else {
        format!("{n} {noun}s")
    }
}

impl Schema {
    fn compile(where_: &str, schema: &Value, mode: Mode) -> Result<Schema, String> {
        let Value::Map(map) = schema else {
            return Err(format!("{where_}: must be an object"));
        };
        for (key, _) in map {
            let keyword = display(key);
            let keyword = keyword.as_str();
            if !KEYWORDS.contains(&keyword) && !DOCUMENTARY.contains(&keyword) {
                return Err(unknown_keyword(where_, keyword));
            }
            if mode == Mode::HeaderPart
                && !HEADER_PART_KEYWORDS.contains(&keyword)
                && !DOCUMENTARY.contains(&keyword)
            {
                return Err(format!(
                    "{where_}: '{keyword}' does not apply to the header part"
                ));
            }
            if mode == Mode::HeaderProperty && NOT_FOR_A_HEADER.contains(&keyword) {
                return Err(format!("{where_}: '{keyword}' does not apply to a header"));
            }
        }
        let type_ = Self::compile_type(where_, map, mode)?;
        if let Some(t) = &type_ {
            for (key, _) in map {
                let keyword = display(key);
                if !applies_to(&keyword, t) {
                    return Err(format!("{where_}: '{keyword}' does not apply to type {t}"));
                }
            }
        }
        let mut properties = BTreeMap::new();
        if let Some(value) = get(map, PROPERTIES) {
            let Value::Map(props) = value else {
                return Err(format!(
                    "{where_}.{PROPERTIES}: must be an object of schemas"
                ));
            };
            let child_mode = if mode == Mode::HeaderPart {
                Mode::HeaderProperty
            } else {
                Mode::Body
            };
            for (name, child) in props {
                let name = display(name);
                let compiled =
                    Schema::compile(&format!("{where_}.{PROPERTIES}.{name}"), child, child_mode)?;
                properties.insert(name, compiled);
            }
        }
        let mut required = Vec::new();
        if let Some(value) = get(map, REQUIRED) {
            let Value::Array(names) = value else {
                return Err(format!(
                    "{where_}.{REQUIRED}: must be a list of property names"
                ));
            };
            for name in names {
                match name {
                    Value::String(s) if !s.as_str().unwrap_or("").trim().is_empty() => {
                        required.push(s.as_str().unwrap_or("").to_string());
                    }
                    _ => {
                        return Err(format!(
                            "{where_}.{REQUIRED}: must be a list of property names"
                        ))
                    }
                }
            }
        }
        let items = match get(map, ITEMS) {
            Some(value) => Some(Box::new(Schema::compile(
                &format!("{where_}.{ITEMS}"),
                value,
                Mode::Body,
            )?)),
            None => None,
        };
        let enum_values = match get(map, ENUM) {
            Some(value) => Some(compile_enum(where_, value, type_.as_deref())?),
            None => None,
        };
        let minimum = match get(map, MINIMUM) {
            Some(value) => Some(number(where_, MINIMUM, value)?),
            None => None,
        };
        let maximum = match get(map, MAXIMUM) {
            Some(value) => Some(number(where_, MAXIMUM, value)?),
            None => None,
        };
        let exclusive_minimum = match get(map, EXCLUSIVE_MINIMUM) {
            Some(value) => flag(where_, EXCLUSIVE_MINIMUM, value)?,
            None => false,
        };
        let exclusive_maximum = match get(map, EXCLUSIVE_MAXIMUM) {
            Some(value) => flag(where_, EXCLUSIVE_MAXIMUM, value)?,
            None => false,
        };
        if has(map, EXCLUSIVE_MINIMUM) && minimum.is_none() {
            return Err(format!("{where_}.{EXCLUSIVE_MINIMUM}: needs {MINIMUM}"));
        }
        if has(map, EXCLUSIVE_MAXIMUM) && maximum.is_none() {
            return Err(format!("{where_}.{EXCLUSIVE_MAXIMUM}: needs {MAXIMUM}"));
        }
        if let (Some(lo), Some(hi)) = (&minimum, &maximum) {
            if lo.compare(hi) == Ordering::Greater {
                return Err(format!("{where_}: {MINIMUM} is greater than {MAXIMUM}"));
            }
        }
        let min_length = match get(map, MIN_LENGTH) {
            Some(value) => Some(count(where_, MIN_LENGTH, value)?),
            None => None,
        };
        let max_length = match get(map, MAX_LENGTH) {
            Some(value) => Some(count(where_, MAX_LENGTH, value)?),
            None => None,
        };
        if let (Some(lo), Some(hi)) = (min_length, max_length) {
            if lo > hi {
                return Err(format!(
                    "{where_}: {MIN_LENGTH} is greater than {MAX_LENGTH}"
                ));
            }
        }
        let min_items = match get(map, MIN_ITEMS) {
            Some(value) => Some(count(where_, MIN_ITEMS, value)?),
            None => None,
        };
        let max_items = match get(map, MAX_ITEMS) {
            Some(value) => Some(count(where_, MAX_ITEMS, value)?),
            None => None,
        };
        if let (Some(lo), Some(hi)) = (min_items, max_items) {
            if lo > hi {
                return Err(format!("{where_}: {MIN_ITEMS} is greater than {MAX_ITEMS}"));
            }
        }
        let pattern = match get(map, PATTERN) {
            Some(Value::String(s)) => {
                let text = s.as_str().unwrap_or("").to_string();
                let regex = compile_pattern(where_, &text)?;
                Some((text, regex))
            }
            Some(_) => return Err(format!("{where_}.{PATTERN}: must be text")),
            None => None,
        };
        let nullable = match get(map, NULLABLE) {
            Some(value) => flag(where_, NULLABLE, value)?,
            None => false,
        };
        let (no_additional, additional) = match get(map, ADDITIONAL_PROPERTIES) {
            None => (false, None),
            Some(value @ Value::Map(_)) => (
                false,
                Some(Box::new(Schema::compile(
                    &format!("{where_}.{ADDITIONAL_PROPERTIES}"),
                    value,
                    Mode::Body,
                )?)),
            ),
            Some(value) if is_flag(value) => (!flag(where_, ADDITIONAL_PROPERTIES, value)?, None),
            Some(_) => {
                return Err(format!(
                    "{where_}.{ADDITIONAL_PROPERTIES}: must be true, false or a schema"
                ))
            }
        };
        Ok(Schema {
            type_,
            properties,
            required,
            items,
            enum_values,
            minimum,
            maximum,
            exclusive_minimum,
            exclusive_maximum,
            min_length,
            max_length,
            pattern,
            min_items,
            max_items,
            nullable,
            no_additional,
            additional,
        })
    }

    fn compile_type(
        where_: &str,
        map: &[(Value, Value)],
        mode: Mode,
    ) -> Result<Option<String>, String> {
        let Some(value) = get(map, TYPE) else {
            return Ok(None);
        };
        let names = if mode == Mode::HeaderProperty {
            HEADER_TYPE_NAMES
        } else {
            TYPE_NAMES
        };
        let Value::String(s) = value else {
            return Err(format!("{where_}.{TYPE}: must be a type name - {names}"));
        };
        let text = s.as_str().unwrap_or("");
        if mode == Mode::HeaderPart {
            if text != OBJECT {
                return Err(format!("{where_}.{TYPE}: must be object"));
            }
            return Ok(Some(text.to_string()));
        }
        if mode == Mode::HeaderProperty && !HEADER_TYPES.contains(&text) {
            return Err(format!(
                "{where_}.{TYPE}: unknown type '{text}' - a header is text: use {names}"
            ));
        }
        if !TYPES.contains(&text) {
            return Err(format!(
                "{where_}.{TYPE}: unknown type '{text}' - use {names}"
            ));
        }
        Ok(Some(text.to_string()))
    }

    // ------------------------------------------------------------- validation

    /// Validate a value against this schema, collecting every violation;
    /// `path` is the value's path, e.g. `input.body`.
    pub fn validate(&self, path: &str, value: Option<&Value>, violations: &mut Vec<String>) {
        let value = match value {
            Some(v) if !matches!(v, Value::Nil) => v,
            _ => {
                if !self.nullable {
                    if let Some(t) = &self.type_ {
                        violations.push(format!("{path}: expected {t}, got null"));
                    }
                }
                return;
            }
        };
        let actual = kind_of(value);
        if let Some(t) = &self.type_ {
            if !matches(t, value, actual) {
                violations.push(format!("{path}: expected {t}, got {actual}"));
                return;
            }
        }
        if let Some(entries) = &self.enum_values {
            if !in_enum(entries, value, actual) {
                violations.push(format!("{path}: must be one of {}", render_enum(entries)));
            }
        }
        match actual {
            NUMBER => self.check_bounds(path, Num::of(value), violations),
            STRING => self.check_text(path, &text_of(value), violations),
            ARRAY => {
                if let Value::Array(list) = value {
                    self.check_array(path, list, violations);
                }
            }
            OBJECT => {
                if let Value::Map(map) = value {
                    self.check_object(path, map, violations);
                }
            }
            _ => {}
        }
    }

    /// Validate a run's headers against this header part: the required names
    /// must be present and each declared header's text must satisfy its
    /// schema. Names match case-insensitively.
    pub fn validate_headers(
        &self,
        path: &str,
        headers: Option<&Value>,
        violations: &mut Vec<String>,
    ) {
        let empty = Vec::new();
        let headers = match headers {
            Some(Value::Map(entries)) => entries,
            _ => &empty,
        };
        for name in &self.required {
            if lookup(headers, name).is_none() {
                violations.push(format!("{path}.{name}: required"));
            }
        }
        for (name, schema) in &self.properties {
            if let Some(text) = lookup(headers, name) {
                schema.validate_header(&format!("{path}.{name}"), &text, violations);
            }
        }
    }

    fn validate_header(&self, path: &str, text: &str, violations: &mut Vec<String>) {
        match self.type_.as_deref() {
            Some(NUMBER) | Some(INTEGER) => {
                let integer = self.type_.as_deref() == Some(INTEGER);
                let n = match Num::parse(text) {
                    Some(n) if !integer || n.is_integral() => n,
                    _ => {
                        violations.push(format!(
                            "{path}: expected {}",
                            self.type_.as_deref().unwrap_or(NUMBER)
                        ));
                        return;
                    }
                };
                if let Some(entries) = &self.enum_values {
                    if !entries
                        .iter()
                        .any(|e| matches!(e, Entry::Num(m) if m.compare(&n) == Ordering::Equal))
                    {
                        violations.push(format!("{path}: must be one of {}", render_enum(entries)));
                    }
                }
                self.check_bounds(path, Some(n), violations);
            }
            Some(BOOLEAN) => {
                let lower = text.to_lowercase();
                if lower != "true" && lower != "false" {
                    violations.push(format!("{path}: expected {BOOLEAN}"));
                    return;
                }
                if let Some(entries) = &self.enum_values {
                    let value = lower == "true";
                    if !entries
                        .iter()
                        .any(|e| matches!(e, Entry::Flag(b) if *b == value))
                    {
                        violations.push(format!("{path}: must be one of {}", render_enum(entries)));
                    }
                }
            }
            _ => {
                if let Some(entries) = &self.enum_values {
                    if !entries
                        .iter()
                        .any(|e| matches!(e, Entry::Text(t) if t == text))
                    {
                        violations.push(format!("{path}: must be one of {}", render_enum(entries)));
                    }
                }
                self.check_text(path, text, violations);
            }
        }
    }

    fn check_bounds(&self, path: &str, n: Option<Num>, violations: &mut Vec<String>) {
        let Some(n) = n else {
            return;
        };
        if let Some(minimum) = &self.minimum {
            let cmp = n.compare(minimum);
            let below = if self.exclusive_minimum {
                cmp != Ordering::Greater
            } else {
                cmp == Ordering::Less
            };
            if below {
                violations.push(format!(
                    "{path}: must be {} {}",
                    if self.exclusive_minimum {
                        "more than"
                    } else {
                        "at least"
                    },
                    minimum.render()
                ));
            }
        }
        if let Some(maximum) = &self.maximum {
            let cmp = n.compare(maximum);
            let above = if self.exclusive_maximum {
                cmp != Ordering::Less
            } else {
                cmp == Ordering::Greater
            };
            if above {
                violations.push(format!(
                    "{path}: must be {} {}",
                    if self.exclusive_maximum {
                        "less than"
                    } else {
                        "at most"
                    },
                    maximum.render()
                ));
            }
        }
    }

    fn check_text(&self, path: &str, text: &str, violations: &mut Vec<String>) {
        let length = text.chars().count();
        if let Some(min) = self.min_length {
            if length < min {
                violations.push(format!(
                    "{path}: must be at least {}",
                    plural(min, "character")
                ));
            }
        }
        if let Some(max) = self.max_length {
            if length > max {
                violations.push(format!(
                    "{path}: must be at most {}",
                    plural(max, "character")
                ));
            }
        }
        if let Some((pattern, regex)) = &self.pattern {
            if !regex.is_match(text) {
                violations.push(format!("{path}: does not match pattern {pattern}"));
            }
        }
    }

    fn check_array(&self, path: &str, list: &[Value], violations: &mut Vec<String>) {
        if let Some(min) = self.min_items {
            if list.len() < min {
                violations.push(format!(
                    "{path}: must have at least {}",
                    plural(min, "item")
                ));
            }
        }
        if let Some(max) = self.max_items {
            if list.len() > max {
                violations.push(format!("{path}: must have at most {}", plural(max, "item")));
            }
        }
        if let Some(items) = &self.items {
            for (i, element) in list.iter().enumerate() {
                items.validate(&format!("{path}[{i}]"), Some(element), violations);
            }
        }
    }

    fn check_object(&self, path: &str, map: &[(Value, Value)], violations: &mut Vec<String>) {
        for name in &self.required {
            if !has(map, name) {
                violations.push(format!("{path}.{name}: required"));
            }
        }
        for (name, schema) in &self.properties {
            if has(map, name) {
                schema.validate(&format!("{path}.{name}"), get(map, name), violations);
            }
        }
        if self.no_additional || self.additional.is_some() {
            let extras: BTreeSet<String> = map
                .iter()
                .map(|(k, _)| display(k))
                .filter(|name| !self.properties.contains_key(name))
                .collect();
            for name in extras {
                if self.no_additional {
                    violations.push(format!("{path}.{name}: not allowed"));
                } else if let Some(schema) = &self.additional {
                    schema.validate(&format!("{path}.{name}"), get(map, &name), violations);
                }
            }
        }
    }
}

fn lookup(headers: &[(Value, Value)], name: &str) -> Option<String> {
    let wanted = name.to_lowercase();
    headers
        .iter()
        .find(|(k, v)| !matches!(v, Value::Nil) && display(k).to_lowercase() == wanted)
        .map(|(_, v)| text_of(v))
}

fn kind_of(value: &Value) -> &'static str {
    match value {
        Value::Map(_) => OBJECT,
        Value::Array(_) => ARRAY,
        Value::Boolean(_) => BOOLEAN,
        Value::Integer(_) | Value::F32(_) | Value::F64(_) => NUMBER,
        _ => STRING,
    }
}

fn matches(type_: &str, value: &Value, actual: &str) -> bool {
    if type_ == INTEGER {
        return actual == NUMBER && Num::of(value).map(|n| n.is_integral()).unwrap_or(false);
    }
    type_ == actual
}

fn in_enum(entries: &[Entry], value: &Value, actual: &str) -> bool {
    match actual {
        NUMBER => {
            let Some(n) = Num::of(value) else {
                return false;
            };
            entries
                .iter()
                .any(|e| matches!(e, Entry::Num(m) if m.compare(&n) == Ordering::Equal))
        }
        STRING => {
            let text = text_of(value);
            entries
                .iter()
                .any(|e| matches!(e, Entry::Text(t) if *t == text))
        }
        BOOLEAN => {
            let Value::Boolean(b) = value else {
                return false;
            };
            entries
                .iter()
                .any(|e| matches!(e, Entry::Flag(f) if f == b))
        }
        _ => entries
            .iter()
            .any(|e| matches!(e, Entry::Raw(v) if v == value)),
    }
}

fn render_enum(entries: &[Entry]) -> String {
    entries
        .iter()
        .map(|e| match e {
            Entry::Num(n) => n.render(),
            Entry::Text(t) => t.clone(),
            Entry::Flag(b) => b.to_string(),
            Entry::Raw(v) => display(v),
        })
        .collect::<Vec<_>>()
        .join(", ")
}
