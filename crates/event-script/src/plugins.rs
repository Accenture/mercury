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

//! Simple-plugin registry — the Rust analog of the Java `SimplePluginLoader`
//! (`@SimplePlugin` classes implementing `PluginFunction.calculate`).
//!
//! Increment E-2 shipped the core conversion/logical bodies (the set the
//! legacy-syntax converter emits); **increment E-8** completes the built-in
//! catalog (arithmetic, generators, date parsing, list-of-map operations,
//! `validate`, `ternary`, comparisons) and adds the user extension point:
//! a `#[simple_plugin]` function registers itself through the link-time
//! inventory, collected by the `SimplePluginLoader` before-application hook
//! (sequence 3 — before flows compile, so `f:` names validate).
//!
//! Divergence (design E6): Java verifies plugin **bytecode** against an
//! allowlist at load time; Rust plugins are compiled, linked code — no
//! runtime class loading exists, so the allowlist's threat model does not
//! apply. The name registry + `f:name(args)` resolution semantics are ported
//! faithfully. Argument convention (Java `calculate(Object... input)`): the
//! caller evaluates each argument to a value first; a nested `f:` argument is
//! passed as null (execution-loop guard).

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

use event_script_macros::simple_plugin;
use rmpv::Value;

use crate::conversions::{
    convert_boolean, convert_double, convert_float, convert_integer, convert_long, display,
    get_b64, get_binary_value, get_length, get_text_value, parse_json,
};
use crate::plugins_e8::value_type_name;

/// The number of built-in `#[simple_plugin]` declarations the engine itself
/// ships (this module, `plugins_e8`, `plugins_decimal` and `plugins_date`) —
/// the startup floor the `SimplePluginLoader` asserts against linker elision.
pub const BUILTIN_PLUGIN_COUNT: usize = 62;

/// A plugin body (Java `PluginFunction.calculate`): evaluated argument values
/// in, one value out; a descriptive error is the Java
/// `IllegalArgumentException` analog.
pub type PluginBody = fn(&[Value]) -> Result<Value, String>;

enum Registration {
    Implemented(PluginBody),
}

/// A user plugin registered by the `#[simple_plugin]` macro (Java
/// `@SimplePlugin` classpath-scan analog — link-time inventory).
pub struct SimplePluginEntry {
    pub name: &'static str,
    pub body: PluginBody,
}

platform_core::inventory::collect!(SimplePluginEntry);

fn registry() -> &'static RwLock<HashMap<String, Registration>> {
    static PLUGINS: OnceLock<RwLock<HashMap<String, Registration>>> = OnceLock::new();
    PLUGINS.get_or_init(|| {
        // Fold the link-time inventory exactly once. The engine's own
        // built-ins (BUILTIN_PLUGIN_COUNT of them) are #[simple_plugin]
        // declarations (the engine dogfoods
        // its extension point, like Java's @SimplePlugin classes under
        // com.accenture.services.plugins), collected together with user
        // plugins. A duplicate name warns + last-wins (the one conflict
        // policy, D2) — including a user plugin shadowing a built-in. The
        // SimplePluginLoader hook (before_application sequence 3) forces
        // this fold before flows compile and asserts the count.
        let mut map: HashMap<String, Registration> = HashMap::new();
        for entry in platform_core::inventory::iter::<SimplePluginEntry> {
            if map
                .insert(
                    entry.name.to_string(),
                    Registration::Implemented(entry.body),
                )
                .is_some()
            {
                log::warn!(
                    "Reloading SimplePlugin {} - please check duplicated plugin name",
                    entry.name
                );
            }
        }
        RwLock::new(map)
    })
}

/// True when a plugin name is registered (Java `containsSimplePlugin`).
pub fn contains_simple_plugin(name: &str) -> bool {
    registry()
        .read()
        .expect("plugin registry")
        .contains_key(name)
}

/// Register a plugin body programmatically. Explicit registration wins over
/// declarative (it runs later and replaces); a duplicate name warns +
/// last-wins — the one conflict policy (D2), matching Java's
/// `SimplePluginLoader` wording.
pub fn register_plugin(name: &str, body: PluginBody) {
    if registry()
        .write()
        .expect("plugin registry")
        .insert(name.to_string(), Registration::Implemented(body))
        .is_some()
    {
        log::warn!("Reloading SimplePlugin {name} - please check duplicated plugin name");
    }
}

/// Invoke a plugin by name with evaluated arguments (Java
/// `plugin.calculate(input)`).
pub fn calculate(name: &str, args: &[Value]) -> Result<Value, String> {
    let guard = registry().read().expect("plugin registry");
    match guard.get(name) {
        Some(Registration::Implemented(body)) => body(args),
        None => Err(format!("SimplePlugin '{name}' not found")),
    }
}

/// Force the one-time inventory fold (Java
/// `SimplePluginLoader.preloadSimplePlugins`) and return how many plugin
/// names are registered — built-ins and user `#[simple_plugin]` declarations
/// alike arrive through the same link-time inventory.
pub fn load_inventory_plugins() -> usize {
    registry().read().expect("plugin registry").len()
}

/// Java pattern shared by the conversion plugins: one argument converts to a
/// scalar, several arguments convert element-wise into a list.
fn one_or_list(
    args: &[Value],
    what: &str,
    convert: impl Fn(&Value) -> Result<Value, String>,
) -> Result<Value, String> {
    match args {
        [] => Err(format!("Input is required for {what}")),
        [one] => convert(one),
        many => Ok(Value::Array(
            many.iter().map(&convert).collect::<Result<Vec<_>, _>>()?,
        )),
    }
}

#[simple_plugin("text")]
fn plugin_text(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Text conversion", |v| {
        Ok(Value::from(get_text_value(v)))
    })
}

#[simple_plugin("int")]
fn plugin_int(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Integer conversion", |v| {
        Ok(Value::from(convert_integer(v)))
    })
}

#[simple_plugin("long")]
fn plugin_long(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Long conversion", |v| {
        Ok(Value::from(convert_long(v)))
    })
}

#[simple_plugin("float")]
fn plugin_float(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Float conversion", |v| {
        Ok(Value::F32(convert_float(v)))
    })
}

#[simple_plugin("double")]
fn plugin_double(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Double conversion", |v| {
        Ok(Value::F64(convert_double(v)))
    })
}

#[simple_plugin("boolean")]
fn plugin_boolean(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Boolean conversion", |v| {
        convert_boolean(v).map(Value::Boolean)
    })
}

#[simple_plugin("binary")]
fn plugin_binary(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Binary conversion", |v| {
        Ok(Value::Binary(get_binary_value(v)))
    })
}

#[simple_plugin("b64")]
fn plugin_b64(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Base64 conversion", get_b64)
}

#[simple_plugin("json")]
fn plugin_json(args: &[Value]) -> Result<Value, String> {
    // Java ParseJson: exactly one argument - JSON text as a string or byte
    // array (a Nil argument - an absent model variable or a nested plugin -
    // falls through to parse_json's own descriptive error)
    match args {
        [one] => parse_json(one),
        _ => Err("Input must be a JSON in string or byte array".to_string()),
    }
}

/// `f:lookup(table, value[, default])` — the name of the rule of a static
/// decision table that lists the value (Java `SimpleLookup`). The table is a
/// map: `keys` names the rules in priority order and each rule is a list of the
/// values that select it; `keys` and every rule may be a list or a JSON array
/// written as text (a node property authored as `keys=[ "a", "b" ]`), and the
/// table itself may be JSON text. Values are compared as text,
/// case-insensitively; no match returns the optional third argument (typically
/// a `text(...)` constant), or Nil when there is none.
#[simple_plugin("lookup")]
fn plugin_lookup(args: &[Value]) -> Result<Value, String> {
    let (table, value, default) = match args {
        [table, value] => (table, value, &Value::Nil),
        [table, value, default] => (table, value, default),
        _ => {
            return Err(format!(
                "Expected two or three input values - actual={}",
                args.len()
            ));
        }
    };
    let table = decision_table(table)?;
    let keys = map_entry(&table, "keys").ok_or_else(|| "Missing keys in input".to_string())?;
    let wanted = display(value).to_lowercase();
    for rule in decision_table_entry(keys)? {
        let members =
            map_entry(&table, &rule).ok_or_else(|| format!("Missing key in input: {rule}"))?;
        if decision_table_entry(members)?
            .iter()
            .any(|member| member.to_lowercase() == wanted)
        {
            return Ok(Value::from(rule));
        }
    }
    Ok(default.clone())
}

/// `f:rangeLookup(table, value[, default])` - the name of the first band of a
/// range table whose range contains the value (Java `SimpleRangeLookup`). The
/// table has the decision-table shape: `keys` names the bands in priority
/// order and each band is ONE range written as text - `min..max` runs from
/// `min` inclusive up to but EXCLUDING `max`, an omitted end is open, so
/// adjacent bands tile a scale with no gap and no overlap. The table decides
/// how values compare: when every bound is a number the bands are
/// numeric and the value must be a number or a numeric string, compared exactly
/// as a decimal; when the bounds are quoted text (`'0020'`, `"2026-11-15"`),
/// values compare as written, case-sensitively; when the bounds are unquoted
/// text, values compare as text without case (ISO-8601 dates and times,
/// alphabetic bands). Text orders by Unicode code point, and the value must be
/// a number or text: a boolean, a list, a map or bytes are refused by name on
/// every table, because a band is a scale and none of them sits on one. A table
/// that mixes kinds is refused. No band, or a Nil value, returns the optional
/// third argument, or Nil when there is none.
#[simple_plugin("rangeLookup")]
fn plugin_range_lookup(args: &[Value]) -> Result<Value, String> {
    const NAME: &str = "rangeLookup";
    let (table, value, default) = match args {
        [table, value] => (table, value, &Value::Nil),
        [table, value, default] => (table, value, default),
        _ => {
            return Err(format!(
                "Expected two or three input values - actual={}",
                args.len()
            ));
        }
    };
    let table = decision_table(table)?;
    let keys = map_entry(&table, "keys").ok_or_else(|| "Missing keys in input".to_string())?;
    // every band is parsed and checked before any value is placed, so a bad band is refused whatever the value
    let mut bands = Vec::new();
    for key in decision_table_entry(keys)? {
        let entry =
            map_entry(&table, &key).ok_or_else(|| format!("Missing key in input: {key}"))?;
        bands.push(Band::parse(&key, &display(entry))?);
    }
    let kind = Band::table_kind(&bands)?;
    if matches!(value, Value::Nil) {
        return Ok(default.clone());
    }
    check_value(value)?;
    let hit = match kind {
        Kind::Number => {
            let v = crate::decimal::operand(value, NAME)?;
            bands.iter().position(|band| band.contains_number(&v))
        }
        Kind::Quoted => {
            let v = display(value);
            bands.iter().position(|band| band.contains_text(&v, false))
        }
        Kind::Text => {
            let v = display(value).to_lowercase();
            bands.iter().position(|band| band.contains_text(&v, true))
        }
    };
    Ok(hit.map_or_else(|| default.clone(), |i| Value::from(bands[i].key.as_str())))
}

/// A band is a scale of numbers or text, so the value must be one of them;
/// anything else is refused by name (Java `SimpleRangeLookup.checkValue`).
fn check_value(value: &Value) -> Result<(), String> {
    let kind = match value {
        Value::Integer(_) | Value::F32(_) | Value::F64(_) | Value::String(_) => return Ok(()),
        Value::Boolean(_) => "a boolean",
        Value::Array(_) => "a list",
        Value::Map(_) => "a map",
        Value::Binary(_) => "bytes",
        _ => "an object",
    };
    Err(format!(
        "rangeLookup value must be a number or text - got {kind}"
    ))
}

/// `f:minRange(range[, default])` - the lower bound of one range written as
/// `rangeLookup`'s bands are (`min..max` from `min` inclusive up to but
/// excluding `max`; `min..` or `..max` leaves an end open), typed by the
/// notation: a number written without a decimal point, within the 64-bit
/// range, is an integer, any other number its exact decimal TEXT (the DECIMAL
/// convention - `f:double` converts it when a floating-point output value is
/// wanted), a text bound is text as written, a quoted one without its quotes,
/// and an open end answers the optional default, or Nil when there is none -
/// the null-source rule then leaves a mapping target unset. A Nil range answers
/// Nil, never the default: the band a lookup did not choose, read back through
/// a dynamic key, has no bounds, while the default belongs to an open end. A
/// malformed range (a quoted bound not closed, followed by text, or holding its
/// own quote character), a band given as a list, a range mixing kinds and an
/// empty one are refused by name, exactly as `rangeLookup` refuses such a band
/// (Java `SimpleMinRange`).
#[simple_plugin("minRange")]
fn plugin_min_range(args: &[Value]) -> Result<Value, String> {
    range_bound(args, true)
}

/// `f:maxRange(range[, default])` - the upper bound of one range, the first
/// value the range leaves out; otherwise as `minRange` (Java `SimpleMaxRange`).
#[simple_plugin("maxRange")]
fn plugin_max_range(args: &[Value]) -> Result<Value, String> {
    range_bound(args, false)
}

/// The body of `minRange` and `maxRange`.
fn range_bound(args: &[Value], lower_end: bool) -> Result<Value, String> {
    let (range, default) = match args {
        [range] => (range, &Value::Nil),
        [range, default] => (range, default),
        _ => {
            return Err(format!(
                "Expected one or two input values - actual={}",
                args.len()
            ));
        }
    };
    if matches!(range, Value::Nil) {
        // no range, no bounds: the null-source rule leaves the mapping target unset
        return Ok(Value::Nil);
    }
    let range = Range::parse(&display(range), "Range")?;
    let bound = if lower_end {
        &range.lower
    } else {
        &range.upper
    };
    Ok(match bound {
        None => default.clone(),
        Some(bound) if range.kind == Kind::Number => typed_number(bound),
        Some(bound) => Value::from(bound.as_str()),
    })
}

/// A whole number within the 64-bit range is an integer; any other number stays
/// its exact decimal text - the high-precision rule of the DECIMAL statement,
/// where a float would round it.
fn typed_number(bound: &str) -> Value {
    if !bound.contains('.') {
        if let Ok(whole) = bound.parse::<i64>() {
            return Value::from(whole);
        }
    }
    Value::from(bound)
}

const NOTATION: &str =
    " must be written as min..max, min.. or ..max (the upper bound is excluded) - got ";

/// How the bounds of a range, and of every band of a table, compare.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    /// canonical numbers, compared exactly as decimals
    Number,
    /// unquoted text, compared without case
    Text,
    /// quoted text, compared as written
    Quoted,
}

/// One range written as text - the notation shared by `rangeLookup`, `minRange`
/// and `maxRange` (Java `RangeNotation`): `min..max` runs from `min` inclusive
/// up to but EXCLUDING `max`, and `min..` or `..max` leaves an end open. A bound
/// is one of three kinds, and a range never mixes two of them: a canonical
/// number compares exactly as a decimal; text in single or double quotes
/// (`'0020'`, `"2026-11-15"`) compares as written, case-sensitively - the quotes
/// keep a zero-padded code or a case-sensitive band as text, and a quoted bound
/// may contain the separator; anything else (`16:00`, `h`) is unquoted text,
/// compared without case, which may not contain the separator. A range whose
/// lower bound is not below its upper bound (`3..3`, `5..1`) holds nothing - no
/// value could ever match it - and is refused; so is a band given as a list or a
/// map, because a band is one range. The three plugins read the notation here,
/// so a range is refused by one of them exactly when the others refuse it.
struct Range {
    /// the lower bound as written (a quoted bound without its quotes), or None when open
    lower: Option<String>,
    /// the upper bound as written (a quoted bound without its quotes), or None when open
    upper: Option<String>,
    /// the kind of the bounds - a range with two bounds has them of one kind
    kind: Kind,
    /// the lower bound as a decimal when the range is numeric and has one
    lower_number: Option<bigdecimal::BigDecimal>,
    /// the upper bound as a decimal when the range is numeric and has one
    upper_number: Option<bigdecimal::BigDecimal>,
}

/// One bound while parsing: its value and kind (None when open) and the unread rest.
struct Bound<'a> {
    value: Option<&'a str>,
    kind: Option<Kind>,
    rest: &'a str,
}

impl<'a> Bound<'a> {
    fn unquoted(value: &'a str, rest: &'a str) -> Bound<'a> {
        if value.is_empty() {
            return Bound {
                value: None,
                kind: None,
                rest,
            };
        }
        let kind = if crate::decimal::parse_canonical(value).is_some() {
            Kind::Number
        } else {
            Kind::Text
        };
        Bound {
            value: Some(value),
            kind: Some(kind),
            rest,
        }
    }

    fn quoted(
        value: &'a str,
        rest: &'a str,
        subject: &str,
        range: &str,
    ) -> Result<Bound<'a>, String> {
        if value.is_empty() {
            return Err(format!(
                "{subject} has an empty quoted bound - an open end is written by omission: '{range}'"
            ));
        }
        Ok(Bound {
            value: Some(value),
            kind: Some(Kind::Quoted),
            rest,
        })
    }
}

fn malformed(subject: &str, range: &str) -> String {
    format!("{subject}{NOTATION}'{range}'")
}

fn is_quote(c: char) -> bool {
    c == '\'' || c == '"'
}

impl Range {
    /// `subject` starts every refusal: "Range of 'gold'" for a band of a
    /// table, "Range" for a single range.
    fn parse(text: &str, subject: &str) -> Result<Range, String> {
        let range = text.trim();
        if range.starts_with('[') || range.starts_with('{') {
            return Err(format!(
                "{subject} must be one range, not a list or a map - got '{range}'"
            ));
        }
        let lower = Self::lower_bound(range, subject)?;
        let upper = Self::upper_bound(lower.rest, range, subject)?;
        let kind = match (lower.kind, upper.kind) {
            (Some(kind), _) | (None, Some(kind)) => kind,
            (None, None) => return Err(malformed(subject, range)),
        };
        if let (Some(lo), Some(hi), Some(lk), Some(hk)) =
            (lower.value, upper.value, lower.kind, upper.kind)
        {
            Self::check_bounds(range, subject, lo, lk, hi, hk)?;
        }
        let number = |bound: Option<&str>| {
            if kind == Kind::Number {
                bound.and_then(crate::decimal::parse_canonical)
            } else {
                None
            }
        };
        Ok(Range {
            lower: lower.value.map(str::to_string),
            upper: upper.value.map(str::to_string),
            kind,
            lower_number: number(lower.value),
            upper_number: number(upper.value),
        })
    }

    /// The lower bound - quoted up to its closing quote, else the text before
    /// the first separator - and the rest.
    fn lower_bound<'a>(range: &'a str, subject: &str) -> Result<Bound<'a>, String> {
        let Some(first) = range.chars().next() else {
            return Err(malformed(subject, range));
        };
        if is_quote(first) {
            let Some(close) = range[1..].find(first) else {
                return Err(malformed(subject, range));
            };
            let value = &range[1..1 + close];
            let rest = range[2 + close..].trim();
            let Some(rest) = rest.strip_prefix("..") else {
                return Err(malformed(subject, range));
            };
            return Bound::quoted(value, rest.trim(), subject, range);
        }
        let Some((value, rest)) = range.split_once("..") else {
            return Err(malformed(subject, range));
        };
        let value = value.trim();
        if value.ends_with('>') || value.ends_with('=') {
            return Err(malformed(subject, range));
        }
        Ok(Bound::unquoted(value, rest.trim()))
    }

    /// The upper bound from what follows the separator: quoted up to its
    /// closing quote, which must end the range, or unquoted text with no
    /// separator.
    fn upper_bound<'a>(rest: &'a str, range: &str, subject: &str) -> Result<Bound<'a>, String> {
        let Some(first) = rest.chars().next() else {
            return Ok(Bound::unquoted("", ""));
        };
        if is_quote(first) {
            // the first closing quote ends the bound, as it does for the lower
            // bound, and nothing may follow it
            let Some(close) = rest[1..].find(first) else {
                return Err(malformed(subject, range));
            };
            if 2 + close != rest.len() {
                return Err(malformed(subject, range));
            }
            return Bound::quoted(&rest[1..1 + close], "", subject, range);
        }
        if rest.contains("..") || first == '.' || first == '<' || first == '=' {
            return Err(malformed(subject, range));
        }
        Ok(Bound::unquoted(rest, ""))
    }

    /// A range with two bounds has them of one kind, and its lower bound below
    /// its upper bound.
    fn check_bounds(
        range: &str,
        subject: &str,
        lo: &str,
        lk: Kind,
        hi: &str,
        hk: Kind,
    ) -> Result<(), String> {
        if lk != hk {
            if lk == Kind::Quoted || hk == Kind::Quoted {
                return Err(format!(
                    "{subject} mixes a quoted and an unquoted bound - quote both bounds or neither: '{range}'"
                ));
            }
            return Err(format!(
                "{subject} mixes a numeric and a text bound - got '{range}' (a bound with a leading zero is text; quote both bounds to compare them as text)"
            ));
        }
        let empty = match lk {
            Kind::Number => {
                crate::decimal::parse_canonical(lo) >= crate::decimal::parse_canonical(hi)
            }
            Kind::Quoted => lo >= hi,
            Kind::Text => lo.to_lowercase() >= hi.to_lowercase(),
        };
        if empty {
            return Err(format!(
                "{subject} is empty - its lower bound is not below its upper bound: '{range}'"
            ));
        }
        Ok(())
    }
}

/// One band of a range table: its name and its range, checked when parsed.
struct Band {
    key: String,
    range: Range,
}

impl Band {
    fn parse(key: &str, text: &str) -> Result<Band, String> {
        let range = Range::parse(text, &format!("Range of '{key}'"))?;
        Ok(Band {
            key: key.to_string(),
            range,
        })
    }

    /// The kind every band shares; a table with no bands reads as text and
    /// matches nothing.
    fn table_kind(bands: &[Band]) -> Result<Kind, String> {
        let mut kind = None;
        for band in bands {
            match kind {
                None => kind = Some(band.range.kind),
                Some(k) if k != band.range.kind => {
                    if k == Kind::Quoted || band.range.kind == Kind::Quoted {
                        return Err(
                            "Range table mixes quoted and unquoted bounds - quote every bound or none"
                                .to_string(),
                        );
                    }
                    return Err(
                        "Range table mixes numeric and text bounds - a table is one or the other"
                            .to_string(),
                    );
                }
                Some(_) => {}
            }
        }
        Ok(kind.unwrap_or(Kind::Text))
    }

    fn contains_number(&self, value: &bigdecimal::BigDecimal) -> bool {
        let at_or_above_lower = self
            .range
            .lower_number
            .as_ref()
            .is_none_or(|lo| lo <= value);
        let below_upper = self.range.upper_number.as_ref().is_none_or(|hi| value < hi);
        at_or_above_lower && below_upper
    }

    fn contains_text(&self, value: &str, ignore_case: bool) -> bool {
        let fold = |bound: &Option<String>| {
            bound.as_deref().map(|b| {
                if ignore_case {
                    b.to_lowercase()
                } else {
                    b.to_string()
                }
            })
        };
        let lower = fold(&self.range.lower);
        let upper = fold(&self.range.upper);
        let at_or_above_lower = lower.as_deref().is_none_or(|lo| lo <= value);
        let below_upper = upper.as_deref().is_none_or(|hi| value < hi);
        at_or_above_lower && below_upper
    }
}

/// The decision table as map entries: a map, or a JSON object written as text.
fn decision_table(value: &Value) -> Result<Vec<(Value, Value)>, String> {
    const NOT_A_TABLE: &str = "First argument must be a decision table as a map or JSON text";
    match value {
        Value::Map(entries) => Ok(entries.clone()),
        Value::String(text) => {
            let trimmed = text.as_str().unwrap_or_default().trim();
            if trimmed.starts_with('{') && trimmed.ends_with('}') {
                match parse_json(&Value::from(trimmed))? {
                    Value::Map(entries) => Ok(entries),
                    _ => Err(NOT_A_TABLE.to_string()),
                }
            } else {
                Err(NOT_A_TABLE.to_string())
            }
        }
        _ => Err(NOT_A_TABLE.to_string()),
    }
}

fn map_entry<'a>(entries: &'a [(Value, Value)], key: &str) -> Option<&'a Value> {
    entries
        .iter()
        .find(|(k, _)| k.as_str() == Some(key))
        .map(|(_, v)| v)
}

/// Java `SimplePluginUtils.normalizeDecisionTableEntry`: a list, or a JSON
/// array written as text, as the text form of each value.
fn decision_table_entry(value: &Value) -> Result<Vec<String>, String> {
    const NOT_A_LIST: &str = "Input is not a list of values";
    match value {
        Value::Array(items) => Ok(items.iter().map(display).collect()),
        Value::String(text) => {
            let trimmed = text.as_str().unwrap_or_default().trim();
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                match parse_json(&Value::from(trimmed))? {
                    Value::Array(items) => Ok(items.iter().map(display).collect()),
                    _ => Err(NOT_A_LIST.to_string()),
                }
            } else {
                Err(NOT_A_LIST.to_string())
            }
        }
        _ => Err(NOT_A_LIST.to_string()),
    }
}

#[simple_plugin("uuid")]
fn plugin_uuid(_args: &[Value]) -> Result<Value, String> {
    // Java UUIDGenerator ignores its input and returns a fresh v4 uuid
    Ok(Value::from(uuid::Uuid::new_v4().to_string()))
}

#[simple_plugin("length")]
fn plugin_length(args: &[Value]) -> Result<Value, String> {
    match args {
        [one] => Ok(Value::from(get_length(one))),
        _ => Err("Expected exactly one argument in order to get Length".to_string()),
    }
}

#[simple_plugin("not")]
fn plugin_not(args: &[Value]) -> Result<Value, String> {
    match args {
        [one] => Ok(Value::Boolean(!convert_boolean(one)?)),
        _ => Err("One single input is required for negation".to_string()),
    }
}

#[simple_plugin("and")]
fn plugin_and(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Input is required to 'AND' values".to_string());
    }
    for v in args {
        if !convert_boolean(v)? {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

#[simple_plugin("or")]
fn plugin_or(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Input is required to 'OR' values".to_string());
    }
    for v in args {
        if convert_boolean(v)? {
            return Ok(Value::Boolean(true));
        }
    }
    Ok(Value::Boolean(false))
}

#[simple_plugin("concat")]
fn plugin_concat(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Input is required for String Concatenation".to_string());
    }
    Ok(Value::from(
        args.iter().map(get_text_value).collect::<String>(),
    ))
}

#[simple_plugin("substring")]
fn plugin_substring(args: &[Value]) -> Result<Value, String> {
    let [value, rest @ ..] = args else {
        return Err("Input is required for substring".to_string());
    };
    let text = get_text_value(value);
    let start = rest.first().map(convert_integer).unwrap_or(-1);
    let end = rest.get(1).map(convert_integer).unwrap_or(-1);
    // Indexes address whole characters — Unicode SCALAR VALUES (maintainer
    // ruling 2026-07-26, superseding the increment-57/F20 UTF-16 retrofit):
    // Java's String.substring counts UTF-16 code units, a JVM legacy that
    // must not propagate to modern ports. Documented divergence bounded to
    // supplementary-plane characters (identical for English/Chinese/BMP
    // text) — and with scalar indexing there is no lossy case at all: the
    // retrofit's surrogate-split-to-U+FFFD note is retired, since exact
    // Java parity was structurally impossible anyway (Rust strings cannot
    // hold unpaired surrogates).
    let scalars: Vec<char> = text.chars().collect();
    let len = scalars.len() as i64;
    // Java isOutOfBounds: end past the text, start past the text, or flipped
    // (bounds measured in scalar values; message unchanged)
    let out_of_bounds = (end >= 0 && end > len)
        || (start >= 0 && start > len)
        || (start > end && start >= 0 && end >= 0);
    if out_of_bounds {
        return Err(format!(
            "Substring indexes are out of bounds: [{start}, {end}]"
        ));
    }
    if start >= 0 && end >= 0 {
        Ok(Value::from(
            scalars[start as usize..end as usize]
                .iter()
                .collect::<String>(),
        ))
    } else if start >= 0 && start < len {
        Ok(Value::from(
            scalars[start as usize..].iter().collect::<String>(),
        ))
    } else {
        Ok(Value::from(text))
    }
}

#[simple_plugin("isNull")]
fn plugin_is_null(args: &[Value]) -> Result<Value, String> {
    match args {
        [one] => Ok(Value::Boolean(matches!(one, Value::Nil))),
        _ => Err("Only one value is accepted".to_string()),
    }
}

#[simple_plugin("notNull")]
fn plugin_not_null(args: &[Value]) -> Result<Value, String> {
    match args {
        [one] => Ok(Value::Boolean(!matches!(one, Value::Nil))),
        _ => Err("Only one value is accepted".to_string()),
    }
}

/// Two-value equality shared by `eq` and `ne` (Java
/// `SimplePluginUtils.equalsWithModifiers`). Arguments 3 and 4 are optional
/// modifiers, each `ignoreCase` or `ignoreType`: `ignoreCase` compares two
/// strings case-insensitively (non-string values are unaffected — types
/// still matter); `ignoreType` compares the `String.valueOf` text forms,
/// allowing the relaxed comparison of numbers and booleans — "123" == 123,
/// "123.456" == 123.456 and "true" == true; both together compare the text
/// forms case-insensitively.
fn equals_with_modifiers(args: &[Value]) -> Result<bool, String> {
    if args.len() > 4 {
        return Err(
            "Expected two values plus optional 'ignoreCase' and/or 'ignoreType' modifiers"
                .to_string(),
        );
    }
    let mut ignore_case = false;
    let mut ignore_type = false;
    for value in &args[2..] {
        let modifier = display(value);
        if modifier == "ignoreCase" {
            ignore_case = true;
        } else if modifier == "ignoreType" {
            ignore_type = true;
        } else {
            return Err(format!(
                "Unknown modifier '{modifier}' - only 'ignoreCase' and 'ignoreType' are supported"
            ));
        }
    }
    let (a, b) = (&args[0], &args[1]);
    if ignore_type {
        let first = display(a);
        let second = display(b);
        return Ok(if ignore_case {
            first.to_lowercase() == second.to_lowercase()
        } else {
            first == second
        });
    }
    if ignore_case {
        if let (Value::String(s1), Value::String(s2)) = (a, b) {
            let first = s1.as_str().unwrap_or_default();
            let second = s2.as_str().unwrap_or_default();
            return Ok(first.to_lowercase() == second.to_lowercase());
        }
    }
    Ok(a == b)
}

#[simple_plugin("eq")]
fn plugin_eq(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Input is required to check for equality".to_string());
    }
    equals_with_modifiers(args).map(Value::Boolean)
}

#[simple_plugin("ne")]
fn plugin_ne(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Input is required to check for inequality".to_string());
    }
    equals_with_modifiers(args).map(|equal| Value::Boolean(!equal))
}

// ---- Collection operators (contributed to the Java engine in
// mercury-composable PR #220; mirrored for flow portability — a flow using
// f:isEmpty(...) must behave identically on both engines, including error
// text read in aggregated logs) ----

/// `f:isEmpty(value)` — true when a Collection/Map/String/array has no
/// elements (Java `IsEmptyOperator`). Null checks belong to `isNull` /
/// `notNull`: a null input is an error, as is an unsupported type.
#[simple_plugin("isEmpty")]
fn plugin_is_empty(args: &[Value]) -> Result<Value, String> {
    let [value] = args else {
        return Err("One input is required to check if value is empty".to_string());
    };
    match value {
        Value::Nil => Err("Input cannot be null to check if value is empty".to_string()),
        Value::Array(items) => Ok(Value::Boolean(items.is_empty())),
        Value::Map(entries) => Ok(Value::Boolean(entries.is_empty())),
        Value::String(text) => Ok(Value::Boolean(
            text.as_str().map(str::is_empty).unwrap_or(true),
        )),
        // the byte-array analog of a Java primitive array
        Value::Binary(bytes) => Ok(Value::Boolean(bytes.is_empty())),
        other => Err(format!(
            "Unsupported input type to check if value is empty: {}",
            value_type_name(other)
        )),
    }
}

/// `f:getFirst(list)` — the first element of a non-empty List (Java
/// `GetFirstOperator`). Null, non-list or empty input is an error.
#[simple_plugin("getFirst")]
fn plugin_get_first(args: &[Value]) -> Result<Value, String> {
    let [value] = args else {
        return Err("One input is required to get first item from list".to_string());
    };
    match value {
        Value::Nil => Err("Input cannot be null to get first item from list".to_string()),
        Value::Array(items) => match items.first() {
            Some(first) => Ok(first.clone()),
            None => Err("Input cannot be empty to get first item from list".to_string()),
        },
        _ => Err("Input must be a list to get first item".to_string()),
    }
}

/// `f:getLast(list)` — the last element of a non-empty List (Java
/// `GetLastOperator`). Null, non-list or empty input is an error.
#[simple_plugin("getLast")]
fn plugin_get_last(args: &[Value]) -> Result<Value, String> {
    let [value] = args else {
        return Err("One input is required to get last item from list".to_string());
    };
    match value {
        Value::Nil => Err("Input cannot be null to get last item from list".to_string()),
        Value::Array(items) => match items.last() {
            Some(last) => Ok(last.clone()),
            None => Err("Input cannot be empty to get last item from list".to_string()),
        },
        _ => Err("Input must be a list to get last item".to_string()),
    }
}

/// `f:setConfig(key, value)` — set or override a configuration parameter at
/// run-time (Java `SetConfigParameter`). The value lands in the process
/// override registry — this port's `System.setProperty` analog — which every
/// `ConfigReader` lookup consults first, so later `map(key)` constants and
/// configuration reads see the update. The key must be a non-empty string;
/// the value may be any object and is converted to text (Java
/// `String.valueOf`). Invalid input returns false without side effect.
#[simple_plugin("setConfig")]
fn plugin_set_config(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::String(key), value] if !value.is_nil() => {
            let key = key.as_str().unwrap_or_default();
            if key.trim().is_empty() {
                Ok(Value::Boolean(false))
            } else {
                platform_core::util::overrides::set(key, &get_text_value(value));
                Ok(Value::Boolean(true))
            }
        }
        _ => Ok(Value::Boolean(false)),
    }
}

/// Key-normalization core for `f:camelCase` / `f:snakeCase` (Java
/// `KeyNormalizationUtils`): legacy systems — often XML-to-JSON
/// transformations — deliver maps whose key formats vary per source
/// (MyExampleKey, My_Example_key, my_example_Key). Both plugins segmentize
/// each key and re-case the segments, recursively through nested maps and
/// lists (values are never touched; only keys).
///
/// Segmentation: underscore, hyphen and dot are separators; a
/// lower-case-or-digit to upper-case transition starts a new segment; an
/// upper-case run followed by a lower-case letter splits before its last
/// upper-case letter (the acronym rule: XMLKey -> XML + Key). Digits ride
/// with their segment. Empty segments drop; a key with no segments at all
/// (e.g. "___") is kept as-is. Colliding normalized keys resolve to the
/// later entry at the first key's position (Java LinkedHashMap semantics).
/// Normalization is idempotent.
fn key_segments(key: &str) -> Vec<String> {
    let chars: Vec<char> = key.chars().collect();
    let mut segments = Vec::new();
    let mut current = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if c == '_' || c == '-' || c == '.' {
            if !current.is_empty() {
                segments.push(std::mem::take(&mut current));
            }
            continue;
        }
        if let Some(previous) = current.chars().last() {
            let lower_to_upper =
                (previous.is_lowercase() || previous.is_numeric()) && c.is_uppercase();
            let acronym_end = previous.is_uppercase()
                && c.is_uppercase()
                && chars.get(i + 1).is_some_and(|next| next.is_lowercase());
            if lower_to_upper || acronym_end {
                segments.push(std::mem::take(&mut current));
            }
        }
        current.push(c);
    }
    if !current.is_empty() {
        segments.push(current);
    }
    segments
}

fn to_camel_case(key: &str) -> String {
    let segments = key_segments(key);
    if segments.is_empty() {
        return key.to_string();
    }
    let mut out = segments[0].to_lowercase();
    for segment in &segments[1..] {
        let lower = segment.to_lowercase();
        let mut rest = lower.chars();
        if let Some(first) = rest.next() {
            out.extend(first.to_uppercase());
            out.push_str(rest.as_str());
        }
    }
    out
}

fn to_snake_case(key: &str) -> String {
    let segments = key_segments(key);
    if segments.is_empty() {
        return key.to_string();
    }
    segments
        .iter()
        .map(|s| s.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

fn normalize_keys(value: &Value, mapper: fn(&str) -> String) -> Value {
    match value {
        Value::Map(entries) => {
            let mut normalized: Vec<(Value, Value)> = Vec::with_capacity(entries.len());
            for (key, item) in entries {
                let name = mapper(&get_text_value(key));
                let item = normalize_keys(item, mapper);
                // a collision resolves to the later entry at the first key's
                // position (Java LinkedHashMap put semantics)
                if let Some(existing) = normalized
                    .iter_mut()
                    .find(|(k, _)| k.as_str() == Some(name.as_str()))
                {
                    existing.1 = item;
                } else {
                    normalized.push((Value::from(name), item));
                }
            }
            Value::Map(normalized)
        }
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| normalize_keys(item, mapper))
                .collect(),
        ),
        other => other.clone(),
    }
}

fn normalize_top_level(
    args: &[Value],
    plugin: &str,
    mapper: fn(&str) -> String,
) -> Result<Value, String> {
    let [value] = args else {
        return Err(format!(
            "One input is required for {plugin} key normalization"
        ));
    };
    match value {
        Value::Nil => Err(format!(
            "Input cannot be null for {plugin} key normalization"
        )),
        Value::Map(_) | Value::Array(_) => Ok(normalize_keys(value, mapper)),
        _ => Err(format!(
            "Input must be a map or a list for {plugin} key normalization"
        )),
    }
}

/// `f:camelCase(mapOrList)` — normalize every key to camelCase, recursively
/// (Java `CamelCaseNormalization`).
#[simple_plugin("camelCase")]
fn plugin_camel_case(args: &[Value]) -> Result<Value, String> {
    normalize_top_level(args, "camelCase", to_camel_case)
}

/// `f:snakeCase(mapOrList)` — normalize every key to snake_case, recursively
/// (Java `SnakeCaseNormalization`).
#[simple_plugin("snakeCase")]
fn plugin_snake_case(args: &[Value]) -> Result<Value, String> {
    normalize_top_level(args, "snakeCase", to_snake_case)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugins_e8::java_pattern_to_chrono;

    fn table_of_json_text() -> Value {
        // the shape of a node authored as keys=[ "community-property", "separate-property" ]:
        // every value is a JSON array written as text
        Value::Map(vec![
            (
                Value::from("purpose"),
                Value::from("a property the lookup ignores"),
            ),
            (
                Value::from("keys"),
                Value::from("[ \"community-property\", \"separate-property\" ]"),
            ),
            (
                Value::from("community-property"),
                Value::from("[ \"CA\", \"TX\" ]"),
            ),
            (Value::from("separate-property"), Value::from("[ \"NY\" ]")),
        ])
    }

    /// Twin of the Java `SimpleLookupTest`: the rule that lists the value, from
    /// list values or JSON-text values, the whole table as JSON text, text
    /// comparison ignoring case, Nil on a miss, exact error messages.
    #[test]
    fn lookup_matches_java_semantics() {
        assert!(contains_simple_plugin("lookup"));
        let of_lists = Value::Map(vec![
            (
                Value::from("keys"),
                Value::Array(vec![
                    Value::from("community-property"),
                    Value::from("separate-property"),
                ]),
            ),
            (
                Value::from("community-property"),
                Value::Array(vec![Value::from("CA"), Value::from("TX")]),
            ),
            (
                Value::from("separate-property"),
                Value::Array(vec![Value::from("NY")]),
            ),
        ]);
        assert_eq!(
            calculate("lookup", &[of_lists.clone(), Value::from("TX")]),
            Ok(Value::from("community-property"))
        );
        assert_eq!(
            calculate("lookup", &[table_of_json_text(), Value::from("NY")]),
            Ok(Value::from("separate-property"))
        );
        // the whole table as JSON text, numbers compared as text
        let json_table =
            Value::from("{\"keys\": [\"low\", \"high\"], \"low\": [1, 2, 3], \"high\": [4, 5]}");
        assert_eq!(
            calculate("lookup", &[json_table.clone(), Value::from(2)]),
            Ok(Value::from("low"))
        );
        assert_eq!(
            calculate("lookup", &[json_table, Value::from("5")]),
            Ok(Value::from("high"))
        );
        // case-insensitive text comparison
        assert_eq!(
            calculate("lookup", &[of_lists.clone(), Value::from("tx")]),
            Ok(Value::from("community-property"))
        );
        // a miss is Nil when no default is given
        assert_eq!(
            calculate("lookup", &[of_lists.clone(), Value::from("ZZ")]),
            Ok(Value::Nil)
        );
        assert_eq!(
            calculate("lookup", &[table_of_json_text(), Value::Nil]),
            Ok(Value::Nil)
        );
        // f:lookup(state-rules, input.body.state, text(unknown)): a miss returns the
        // optional third argument, a hit ignores it
        assert_eq!(
            calculate(
                "lookup",
                &[of_lists.clone(), Value::from("ZZ"), Value::from("unknown")]
            ),
            Ok(Value::from("unknown"))
        );
        assert_eq!(
            calculate(
                "lookup",
                &[table_of_json_text(), Value::Nil, Value::from("unknown")]
            ),
            Ok(Value::from("unknown"))
        );
        assert_eq!(
            calculate(
                "lookup",
                &[of_lists.clone(), Value::from("TX"), Value::from("unknown")]
            ),
            Ok(Value::from("community-property"))
        );
        // invalid inputs: exact Java-parity messages
        assert_eq!(
            calculate("lookup", std::slice::from_ref(&of_lists)),
            Err("Expected two or three input values - actual=1".to_string())
        );
        assert_eq!(
            calculate(
                "lookup",
                &[
                    of_lists.clone(),
                    Value::from("TX"),
                    Value::from("unknown"),
                    Value::from("extra")
                ]
            ),
            Err("Expected two or three input values - actual=4".to_string())
        );
        assert_eq!(
            calculate("lookup", &[Value::from("state-rules"), Value::from("TX")]),
            Err("First argument must be a decision table as a map or JSON text".to_string())
        );
        assert_eq!(
            calculate("lookup", &[Value::Nil, Value::from("TX")]),
            Err("First argument must be a decision table as a map or JSON text".to_string())
        );
        let no_keys = Value::Map(vec![(
            Value::from("community-property"),
            Value::Array(vec![Value::from("CA")]),
        )]);
        assert_eq!(
            calculate("lookup", &[no_keys, Value::from("CA")]),
            Err("Missing keys in input".to_string())
        );
        let missing_rule = Value::Map(vec![(
            Value::from("keys"),
            Value::Array(vec![Value::from("community-property")]),
        )]);
        assert_eq!(
            calculate("lookup", &[missing_rule, Value::from("CA")]),
            Err("Missing key in input: community-property".to_string())
        );
        let not_a_list = Value::Map(vec![
            (Value::from("keys"), Value::from("community-property")),
            (Value::from("community-property"), Value::from("CA")),
        ]);
        assert_eq!(
            calculate("lookup", &[not_a_list, Value::from("CA")]),
            Err("Input is not a list of values".to_string())
        );
    }

    /// Twin of the Java `IsEmptyOperatorTest` (PR #220): positives across
    /// the supported types — incl. the byte-array analog of an empty /
    /// non-empty primitive array — and invalid inputs with exact messages.
    #[test]
    fn is_empty_matches_java_semantics() {
        // discovery: the plugin is registered under its f: name
        assert!(contains_simple_plugin("isEmpty"));
        let t = Value::Boolean(true);
        let f = Value::Boolean(false);
        // Collection (list)
        assert_eq!(calculate("isEmpty", &[Value::Array(vec![])]), Ok(t.clone()));
        assert_eq!(
            calculate("isEmpty", &[Value::Array(vec![Value::from(1)])]),
            Ok(f.clone())
        );
        // Map
        assert_eq!(calculate("isEmpty", &[Value::Map(vec![])]), Ok(t.clone()));
        assert_eq!(
            calculate(
                "isEmpty",
                &[Value::Map(vec![(Value::from("k"), Value::from("v"))])]
            ),
            Ok(f.clone())
        );
        // CharSequence (string)
        assert_eq!(calculate("isEmpty", &[Value::from("")]), Ok(t.clone()));
        assert_eq!(calculate("isEmpty", &[Value::from("x")]), Ok(f.clone()));
        // primitive array analog (byte array)
        assert_eq!(
            calculate("isEmpty", &[Value::Binary(vec![])]),
            Ok(t.clone())
        );
        assert_eq!(
            calculate("isEmpty", &[Value::Binary(vec![7u8])]),
            Ok(f.clone())
        );
        // invalid inputs: exact Java-parity messages
        assert_eq!(
            calculate("isEmpty", &[]),
            Err("One input is required to check if value is empty".to_string())
        );
        assert_eq!(
            calculate("isEmpty", &[Value::from(""), Value::from("")]),
            Err("One input is required to check if value is empty".to_string())
        );
        assert_eq!(
            calculate("isEmpty", &[Value::Nil]),
            Err("Input cannot be null to check if value is empty".to_string())
        );
        assert_eq!(
            calculate("isEmpty", &[Value::from(42)]),
            Err("Unsupported input type to check if value is empty: Integer".to_string())
        );
        assert_eq!(
            calculate("isEmpty", &[Value::Boolean(true)]),
            Err("Unsupported input type to check if value is empty: Boolean".to_string())
        );
    }

    /// Twin of the Java `GetFirstOperatorTest` / `GetLastOperatorTest`
    /// (PR #220): a single non-empty List; null, non-list or empty input is
    /// an error with the exact message.
    #[test]
    fn get_first_and_get_last_match_java_semantics() {
        assert!(contains_simple_plugin("getFirst"));
        assert!(contains_simple_plugin("getLast"));
        let list = Value::Array(vec![Value::from("a"), Value::from(2), Value::from("z")]);
        assert_eq!(
            calculate("getFirst", std::slice::from_ref(&list)),
            Ok(Value::from("a"))
        );
        assert_eq!(
            calculate("getLast", std::slice::from_ref(&list)),
            Ok(Value::from("z"))
        );
        // single-element list: first == last
        let one = Value::Array(vec![Value::from(7)]);
        assert_eq!(
            calculate("getFirst", std::slice::from_ref(&one)),
            Ok(Value::from(7))
        );
        assert_eq!(calculate("getLast", &[one]), Ok(Value::from(7)));
        // invalid inputs: exact Java-parity messages
        assert_eq!(
            calculate("getFirst", &[]),
            Err("One input is required to get first item from list".to_string())
        );
        assert_eq!(
            calculate("getLast", &[]),
            Err("One input is required to get last item from list".to_string())
        );
        assert_eq!(
            calculate("getFirst", &[Value::Nil]),
            Err("Input cannot be null to get first item from list".to_string())
        );
        assert_eq!(
            calculate("getLast", &[Value::Nil]),
            Err("Input cannot be null to get last item from list".to_string())
        );
        assert_eq!(
            calculate("getFirst", &[Value::from("not-a-list")]),
            Err("Input must be a list to get first item".to_string())
        );
        assert_eq!(
            calculate("getLast", &[Value::from(9)]),
            Err("Input must be a list to get last item".to_string())
        );
        assert_eq!(
            calculate("getFirst", &[Value::Array(vec![])]),
            Err("Input cannot be empty to get first item from list".to_string())
        );
        assert_eq!(
            calculate("getLast", &[Value::Array(vec![])]),
            Err("Input cannot be empty to get last item from list".to_string())
        );
    }

    #[test]
    fn core_plugins_match_java_semantics() {
        assert_eq!(calculate("int", &[Value::from("41")]), Ok(Value::from(41)));
        // multi-arg conversions return a list (Java pattern)
        assert_eq!(
            calculate("int", &[Value::from("1"), Value::from("2")]),
            Ok(Value::Array(vec![Value::from(1), Value::from(2)]))
        );
        assert_eq!(
            calculate("not", &[Value::Boolean(true)]),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            calculate("not", &[Value::from("false")]),
            Ok(Value::Boolean(true))
        );
        assert_eq!(calculate("isNull", &[Value::Nil]), Ok(Value::Boolean(true)));
        assert_eq!(
            calculate("notNull", &[Value::Nil]),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            calculate("eq", &[Value::from("a"), Value::from("a")]),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            calculate("ne", &[Value::from("a"), Value::from("b")]),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            calculate(
                "concat",
                &[Value::from("a"), Value::from(" "), Value::from("b")]
            ),
            Ok(Value::from("a b"))
        );
        // a null operand renders as "null" (String.valueOf semantics; the Java
        // engine relaxed its getTextValue to match this behavior)
        assert_eq!(
            calculate("concat", &[Value::from("a"), Value::Nil, Value::from("b")]),
            Ok(Value::from("anullb"))
        );
        assert_eq!(
            calculate(
                "substring",
                &[Value::from("hello world"), Value::from(0), Value::from(5)]
            ),
            Ok(Value::from("hello"))
        );
        assert_eq!(
            calculate("and", &[Value::Boolean(true), Value::from("true")]),
            Ok(Value::Boolean(true))
        );
        // eq is type-sensitive like Java Objects.equals
        assert_eq!(
            calculate("eq", &[Value::from("42"), Value::from(42)]),
            Ok(Value::Boolean(false))
        );
    }

    #[test]
    fn eq_ne_modifiers_match_java_semantics() {
        // strict comparison without modifiers stays case- and type-sensitive
        assert_eq!(
            calculate("eq", &[Value::from("Hello"), Value::from("HELLO")]),
            Ok(Value::Boolean(false))
        );
        // 'ignoreCase' compares two strings case-insensitively
        assert_eq!(
            calculate(
                "eq",
                &[
                    Value::from("Hello"),
                    Value::from("HELLO"),
                    Value::from("ignoreCase")
                ]
            ),
            Ok(Value::Boolean(true))
        );
        // ...and has no effect on non-string values (types still matter)
        assert_eq!(
            calculate(
                "eq",
                &[
                    Value::from("123"),
                    Value::from(123),
                    Value::from("ignoreCase")
                ]
            ),
            Ok(Value::Boolean(false))
        );
        // 'ignoreType' compares the String.valueOf text forms
        assert_eq!(
            calculate(
                "eq",
                &[
                    Value::from("123"),
                    Value::from(123),
                    Value::from("ignoreType")
                ]
            ),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            calculate(
                "eq",
                &[
                    Value::from("123.456"),
                    Value::from(123.456),
                    Value::from("ignoreType")
                ]
            ),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            calculate(
                "eq",
                &[
                    Value::from("true"),
                    Value::Boolean(true),
                    Value::from("ignoreType")
                ]
            ),
            Ok(Value::Boolean(true))
        );
        // 'ignoreType' alone stays case-sensitive
        assert_eq!(
            calculate(
                "eq",
                &[
                    Value::from("TRUE"),
                    Value::Boolean(true),
                    Value::from("ignoreType")
                ]
            ),
            Ok(Value::Boolean(false))
        );
        // both modifiers combine, in either order
        assert_eq!(
            calculate(
                "eq",
                &[
                    Value::from("TRUE"),
                    Value::Boolean(true),
                    Value::from("ignoreCase"),
                    Value::from("ignoreType")
                ]
            ),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            calculate(
                "eq",
                &[
                    Value::from("TRUE"),
                    Value::Boolean(true),
                    Value::from("ignoreType"),
                    Value::from("ignoreCase")
                ]
            ),
            Ok(Value::Boolean(true))
        );
        // ne is the exact complement under the same modifiers
        assert_eq!(
            calculate(
                "ne",
                &[
                    Value::from("Hello"),
                    Value::from("HELLO"),
                    Value::from("ignoreCase")
                ]
            ),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            calculate(
                "ne",
                &[
                    Value::from("TRUE"),
                    Value::Boolean(true),
                    Value::from("ignoreType")
                ]
            ),
            Ok(Value::Boolean(true))
        );
        // error messages match the Java engine
        assert_eq!(
            calculate("eq", &[Value::from(1)]),
            Err("Input is required to check for equality".to_string())
        );
        assert_eq!(
            calculate("ne", &[Value::from(1)]),
            Err("Input is required to check for inequality".to_string())
        );
        assert_eq!(
            calculate(
                "eq",
                &[Value::from(1), Value::from(2), Value::from("bogus")]
            ),
            Err(
                "Unknown modifier 'bogus' - only 'ignoreCase' and 'ignoreType' are supported"
                    .to_string()
            )
        );
        // a third plain value is no longer a chained comparison
        assert_eq!(
            calculate("eq", &[Value::from(5), Value::from(5), Value::from(5)]),
            Err(
                "Unknown modifier '5' - only 'ignoreCase' and 'ignoreType' are supported"
                    .to_string()
            )
        );
        assert_eq!(
            calculate(
                "ne",
                &[
                    Value::from(1),
                    Value::from(2),
                    Value::from("ignoreCase"),
                    Value::from("ignoreType"),
                    Value::from("ignoreCase")
                ]
            ),
            Err(
                "Expected two values plus optional 'ignoreCase' and/or 'ignoreType' modifiers"
                    .to_string()
            )
        );
    }

    #[test]
    fn e8_plugins_match_java_semantics() {
        // arithmetic with string promotion
        assert_eq!(
            calculate("add", &[Value::from(6), Value::from("2"), Value::from(3)]),
            Ok(Value::from(11))
        );
        assert_eq!(
            calculate(
                "subtract",
                &[Value::from(6), Value::from(2), Value::from(3)]
            ),
            Ok(Value::from(1))
        );
        assert_eq!(
            calculate("div", &[Value::from(6), Value::from(2)]),
            Ok(Value::from(3))
        );
        assert!(calculate("div", &[Value::from(6), Value::from(0)]).is_err());
        assert_eq!(
            calculate("mod", &[Value::from(7), Value::from(3)]),
            Ok(Value::from(1))
        );
        assert_eq!(
            calculate("increment", &[Value::from(6)]),
            Ok(Value::from(7))
        );
        // numeric promotion (generalized from whole-number-only): any
        // floating-point arg promotes the whole computation to a double,
        // decided over ALL args (order-independent); all-integral stays
        // exact i64 — including integer division (asserted above)
        assert_eq!(
            calculate("add", &[Value::from(6), Value::from(2.5)]),
            Ok(Value::from(8.5))
        );
        assert_eq!(
            calculate("add", &[Value::from(2.5), Value::from(6)]),
            Ok(Value::from(8.5))
        );
        assert_eq!(
            calculate("add", &[Value::from(1), Value::from("2.5"), Value::from(3)]),
            Ok(Value::from(6.5))
        );
        assert_eq!(
            calculate("multiply", &[Value::from(2.5), Value::from(4)]),
            Ok(Value::from(10.0))
        );
        assert_eq!(
            calculate("div", &[Value::from(6.0), Value::from(4)]),
            Ok(Value::from(1.5))
        );
        assert!(calculate("div", &[Value::from(6), Value::from(0.0)]).is_err());
        assert_eq!(
            calculate("mod", &[Value::from(7.5), Value::from(2)]),
            Ok(Value::from(1.5))
        );
        assert_eq!(
            calculate("increment", &[Value::from(2.5)]),
            Ok(Value::from(3.5))
        );
        assert_eq!(
            calculate("decrement", &[Value::from(2.5)]),
            Ok(Value::from(1.5))
        );
        assert!(calculate("add", &[Value::from(1), Value::from("not-a-number")]).is_err());
        // round: half-up (ties away from zero) on the shortest DECIMAL
        // representation - binary artifacts don't leak into the decision
        assert_eq!(
            calculate("round", &[Value::from(2.345), Value::from(2)]),
            Ok(Value::from(2.35))
        );
        assert_eq!(
            // the classic trap: a naive multiply-round-divide yields 1.0
            calculate("round", &[Value::from(1.005), Value::from(2)]),
            Ok(Value::from(1.01))
        );
        assert_eq!(
            calculate("round", &[Value::from(2.5)]),
            Ok(Value::from(3.0))
        );
        assert_eq!(
            calculate("round", &[Value::from(-2.5)]),
            Ok(Value::from(-3.0))
        );
        assert_eq!(
            calculate("round", &[Value::from(4.86459), Value::from(3)]),
            Ok(Value::from(4.865))
        );
        assert_eq!(
            calculate("round", &[Value::from("2.345"), Value::from(2)]),
            Ok(Value::from(2.35))
        );
        // a whole number is already exact and passes through unchanged
        assert_eq!(
            calculate("round", &[Value::from(5), Value::from(2)]),
            Ok(Value::from(5))
        );
        assert!(calculate("round", &[Value::from(1.5), Value::from(1.5)]).is_err());
        assert!(calculate("round", &[Value::from(1.5), Value::from(-1)]).is_err());
        // comparisons follow the same promotion (whole pairs stay exact)
        assert_eq!(
            calculate("gt", &[Value::from(2.5), Value::from(2)]),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            calculate("lt", &[Value::from("2.5"), Value::from(3)]),
            Ok(Value::Boolean(true))
        );
        // comparisons + ternary
        assert_eq!(
            calculate("gt", &[Value::from(3), Value::from(2)]),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            calculate("lt", &[Value::from(3), Value::from(2)]),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            calculate(
                "ternary",
                &[Value::Boolean(true), Value::from("a"), Value::from("b")]
            ),
            Ok(Value::from("a"))
        );
        // string operators are case-insensitive
        assert_eq!(
            calculate(
                "startsWith",
                &[Value::from("Hello World"), Value::from("hello")]
            ),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            calculate(
                "includes",
                &[Value::from("this is a test"), Value::from("TEST")]
            ),
            Ok(Value::Boolean(true))
        );
        // list membership uses value equality
        let list = Value::Array(vec![Value::from("xyz"), Value::from("abc")]);
        assert_eq!(
            calculate("includes", &[list, Value::from("xyz")]),
            Ok(Value::Boolean(true))
        );
        // date parsing: 01/15/2026 at local midnight → epoch ms (non-zero)
        let ms = calculate(
            "parseDate",
            &[Value::from("01/15/2026"), Value::from("MM/dd/yyyy; ms")],
        )
        .unwrap();
        assert!(matches!(ms, Value::Integer(_)));
        // defaults + unique set + removeKey
        assert_eq!(
            calculate("defaultValue", &[Value::Nil, Value::from(101)]),
            Ok(Value::from(101))
        );
        let dup = Value::Array(vec![Value::from("a"), Value::from("b"), Value::from("a")]);
        assert_eq!(
            calculate("uniqueSet", &[dup]),
            Ok(Value::Array(vec![Value::from("a"), Value::from("b")]))
        );
        // listOfMap normalization: {world: [1,2], test: [a,b]} → 2 rows
        let map = Value::Map(vec![
            (
                Value::from("world"),
                Value::Array(vec![Value::from(1), Value::from(2)]),
            ),
            (
                Value::from("test"),
                Value::Array(vec![Value::from("a"), Value::from("b")]),
            ),
        ]);
        let normalized = calculate("listOfMap", &[map]).unwrap();
        let Value::Array(rows) = &normalized else {
            panic!("expected rows")
        };
        assert_eq!(rows.len(), 2);
        // validation: pass, range violation, and evaluate mode
        assert_eq!(
            calculate(
                "validate",
                &[Value::from("12345"), Value::from("user; String; required")]
            ),
            Ok(Value::from("12345"))
        );
        let err = calculate(
            "validate",
            &[Value::from("ABC"), Value::from("user; String; CCC; DDD")],
        )
        .unwrap_err();
        assert_eq!(err, "user (ABC) < CCC");
        assert_eq!(
            calculate(
                "validate",
                &[Value::from("abc"), Value::from("user; Integer; evaluate")]
            ),
            Ok(Value::Boolean(false))
        );
        assert!(calculate("noSuchPlugin", &[]).is_err());
    }

    /// Increment 53 (parity F5): full Java DateTimeFormatter pattern support
    /// and the f:dateTime zone argument (previously silently dropped).
    #[test]
    fn date_time_patterns_and_zone_match_java_semantics() {
        // the converter tokenizes properly: names, 12-hour clock, AM/PM,
        // millis, quoted literals, escaped quotes, offsets
        assert_eq!(
            java_pattern_to_chrono("MM/dd/yyyy HH:mm:ss"),
            Ok("%m/%d/%Y %H:%M:%S".to_string())
        );
        assert_eq!(
            java_pattern_to_chrono("yyyy-MM-dd'T'HH:mm:ss.SSS"),
            Ok("%Y-%m-%dT%H:%M:%S.%3f".to_string())
        );
        assert_eq!(
            java_pattern_to_chrono("EEE, dd MMM yyyy hh:mm a"),
            Ok("%a, %d %b %Y %I:%M %p".to_string())
        );
        assert_eq!(java_pattern_to_chrono("yyyy''MM"), Ok("%Y'%m".to_string()));
        // unsupported letters fail loudly instead of emitting garbage
        let err = java_pattern_to_chrono("yyyy VV").expect_err("V unsupported");
        assert!(err.contains("'V'"), "{err}");

        // f:dateTime honors the zone argument (Java ZoneId.of analog) —
        // deterministic via the offset pattern
        assert_eq!(
            calculate("dateTime", &[Value::from("XXX"), Value::from("UTC")]),
            Ok(Value::from("+00:00"))
        );
        assert_eq!(
            calculate(
                "dateTime",
                &[Value::from("XXX"), Value::from("Asia/Kolkata")]
            ),
            Ok(Value::from("+05:30"))
        );
        let err = calculate("dateTime", &[Value::from("HH"), Value::from("Not/AZone")])
            .expect_err("invalid zone");
        assert!(err.contains("Unknown time zone"), "{err}");
        // no-arg form: ISO_DATE_TIME with the [zone-id] suffix (Java
        // ZonedDateTime + DateTimeFormatter.ISO_DATE_TIME)
        let stamp = get_text_value(&calculate("dateTime", &[]).expect("no-arg dateTime"));
        assert!(
            stamp.contains('T') && stamp.ends_with(']') && stamp.contains('['),
            "{stamp}"
        );

        // parseDateTime with AM/PM + 12-hour clock (previously garbage)
        let ms = calculate(
            "parseDateTime",
            &[
                Value::from("07/04/2026 09:30:00 PM"),
                Value::from("MM/dd/yyyy hh:mm:ss a; ms"),
            ],
        )
        .expect("parse AM/PM");
        // the same instant computed directly (local zone, like Java
        // LocalDateTime.parse().atZone(systemDefault()))
        use chrono::TimeZone;
        let expected = chrono::Local
            .from_local_datetime(
                &chrono::NaiveDate::from_ymd_opt(2026, 7, 4)
                    .unwrap()
                    .and_hms_opt(21, 30, 0)
                    .unwrap(),
            )
            .earliest()
            .unwrap()
            .timestamp_millis();
        assert_eq!(ms, Value::from(expected));
        // milliseconds in the pattern round-trip too
        let ms = calculate(
            "parseDateTime",
            &[
                Value::from("2026-07-04 21:30:00.250"),
                Value::from("yyyy-MM-dd HH:mm:ss.SSS; ms"),
            ],
        )
        .expect("parse millis");
        assert_eq!(ms, Value::from(expected + 250));
    }

    /// Maintainer ruling (2026-07-26, supersedes the increment-57/F20
    /// UTF-16 retrofit): substring indexes address whole characters —
    /// Unicode SCALAR VALUES. The emoji occupies exactly ONE index, and CJK
    /// slicing is identical in both engines (BMP text has one code unit per
    /// scalar).
    #[test]
    fn substring_indexes_unicode_scalar_values() {
        // "a😀b": the emoji is index 1, so index 2 starts at "b"
        assert_eq!(
            calculate("substring", &[Value::from("a😀b"), Value::from(2)]),
            Ok(Value::from("b"))
        );
        assert_eq!(
            calculate(
                "substring",
                &[Value::from("a😀b"), Value::from(0), Value::from(2)]
            ),
            Ok(Value::from("a😀"))
        );
        // the CJK boundary case: whole characters, never UTF-8 bytes
        assert_eq!(
            calculate(
                "substring",
                &[Value::from("你好世界"), Value::from(0), Value::from(2)]
            ),
            Ok(Value::from("你好"))
        );
        // out-of-bounds uses the scalar length (3 for "a😀b"); the error
        // message is unchanged from Java's wording
        assert_eq!(
            calculate(
                "substring",
                &[Value::from("a😀b"), Value::from(0), Value::from(4)]
            ),
            Err("Substring indexes are out of bounds: [0, 4]".to_string())
        );
    }

    /// Twin of the Java `SetConfigParameterTest`: two valid arguments set the
    /// process override (the `System.setProperty` analog) and return true; a
    /// non-string value is converted to text; invalid input returns false
    /// without side effect.
    #[test]
    fn set_config_plugin_sets_override_and_validates_input() {
        use platform_core::util::overrides;
        assert!(contains_simple_plugin("setConfig"));
        // happy path: the override is visible to every ConfigReader lookup
        assert_eq!(
            calculate(
                "setConfig",
                &[
                    Value::from("set.config.plugin.direct"),
                    Value::from("hello")
                ]
            ),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            overrides::get("set.config.plugin.direct"),
            Some("hello".to_string())
        );
        // setting the same parameter again overrides the previous value
        assert_eq!(
            calculate(
                "setConfig",
                &[
                    Value::from("set.config.plugin.direct"),
                    Value::from("world")
                ]
            ),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            overrides::get("set.config.plugin.direct"),
            Some("world".to_string())
        );
        // a non-string value is converted to text (Java String.valueOf)
        assert_eq!(
            calculate(
                "setConfig",
                &[Value::from("set.config.plugin.number"), Value::from(8088)]
            ),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            overrides::get("set.config.plugin.number"),
            Some("8088".to_string())
        );
        assert_eq!(
            calculate(
                "setConfig",
                &[Value::from("set.config.plugin.flag"), Value::Boolean(true)]
            ),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            overrides::get("set.config.plugin.flag"),
            Some("true".to_string())
        );
        // invalid input returns false without side effect
        assert_eq!(calculate("setConfig", &[]), Ok(Value::Boolean(false)));
        assert_eq!(
            calculate("setConfig", &[Value::from("key.only")]),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            calculate(
                "setConfig",
                &[Value::from("too"), Value::from("many"), Value::from("args")]
            ),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            calculate(
                "setConfig",
                &[Value::from(100), Value::from("non-string-key")]
            ),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            calculate("setConfig", &[Value::from(""), Value::from("empty-key")]),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            calculate("setConfig", &[Value::from("   "), Value::from("blank-key")]),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            calculate(
                "setConfig",
                &[Value::from("set.config.plugin.null.value"), Value::Nil]
            ),
            Ok(Value::Boolean(false))
        );
        assert_eq!(overrides::get("set.config.plugin.null.value"), None);
        // test hygiene: leave no overrides behind
        overrides::clear("set.config.plugin.direct");
        overrides::clear("set.config.plugin.number");
        overrides::clear("set.config.plugin.flag");
    }

    /// Twin of the Java `KeyNormalizationTest`: the field's legacy key
    /// variants converge, the acronym rule, hyphen/dot separators, digits
    /// riding with their segment, recursion through nested maps and lists,
    /// last-wins collisions, idempotency, and exact Java-parity error
    /// messages.
    #[test]
    fn key_normalization_matches_java_semantics() {
        assert!(contains_simple_plugin("camelCase"));
        assert!(contains_simple_plugin("snakeCase"));
        let map = |k: &str| Value::Map(vec![(Value::from(k), Value::from(1))]);
        for variant in ["MyExampleKey", "My_Example_key", "my_example_Key"] {
            assert_eq!(
                calculate("camelCase", &[map(variant)]),
                Ok(map("myExampleKey"))
            );
            assert_eq!(
                calculate("snakeCase", &[map(variant)]),
                Ok(map("my_example_key"))
            );
        }
        // the acronym rule: an upper-case run splits before its last upper
        assert_eq!(
            calculate("camelCase", &[map("myXMLKey")]),
            Ok(map("myXmlKey"))
        );
        assert_eq!(
            calculate("snakeCase", &[map("customerID")]),
            Ok(map("customer_id"))
        );
        assert_eq!(
            calculate("camelCase", &[map("XMLHttpRequest")]),
            Ok(map("xmlHttpRequest"))
        );
        // hyphen/dot separators; digits ride with their segment
        assert_eq!(
            calculate("camelCase", &[map("my-example.key")]),
            Ok(map("myExampleKey"))
        );
        assert_eq!(
            calculate("snakeCase", &[map("Address1_Line")]),
            Ok(map("address1_line"))
        );
        // recursion: keys normalize at every depth; values are never touched
        let nested = Value::Map(vec![(
            Value::from("Outer_Key"),
            Value::Array(vec![
                Value::Map(vec![(Value::from("Item_Name"), Value::from("Some_Value"))]),
                Value::from("scalar"),
            ]),
        )]);
        let expected = Value::Map(vec![(
            Value::from("outerKey"),
            Value::Array(vec![
                Value::Map(vec![(Value::from("itemName"), Value::from("Some_Value"))]),
                Value::from("scalar"),
            ]),
        )]);
        assert_eq!(calculate("camelCase", &[nested]), Ok(expected));
        // collision: the later entry wins, at the first key's position
        let colliding = Value::Map(vec![
            (Value::from("MyKey"), Value::from(1)),
            (Value::from("my_key"), Value::from(2)),
        ]);
        assert_eq!(
            calculate("camelCase", &[colliding]),
            Ok(Value::Map(vec![(Value::from("myKey"), Value::from(2))]))
        );
        // idempotency
        let once = calculate("camelCase", &[map("My_Example_Key")]).unwrap();
        assert_eq!(
            calculate("camelCase", std::slice::from_ref(&once)),
            Ok(once)
        );
        // a key with no segments is kept as-is
        assert_eq!(calculate("snakeCase", &[map("-.-")]), Ok(map("-.-")));
        // exact Java-parity error messages
        assert_eq!(
            calculate("camelCase", &[]),
            Err("One input is required for camelCase key normalization".to_string())
        );
        assert_eq!(
            calculate("snakeCase", &[Value::Nil]),
            Err("Input cannot be null for snakeCase key normalization".to_string())
        );
        assert_eq!(
            calculate("camelCase", &[Value::from("MyKey")]),
            Err("Input must be a map or a list for camelCase key normalization".to_string())
        );
    }

    /// Java `SimpleRangeLookupTest`: numeric half-open and inclusive bands, text
    /// bands (times, initials, ISO dates) compared without case, the default on a
    /// miss or a Nil value, the whole table as JSON text, the error messages.
    #[test]
    fn range_lookup_matches_java_semantics() {
        assert!(contains_simple_plugin("rangeLookup"));
        let tiers = Value::Map(vec![
            (Value::from("purpose"), Value::from("ignored")),
            (
                Value::from("keys"),
                Value::from("[ \"platinum\", \"gold\", \"silver\", \"standard\" ]"),
            ),
            (Value::from("platinum"), Value::from("25000..")),
            (Value::from("gold"), Value::from("10000..25000")),
            (Value::from("silver"), Value::from("2500..10000")),
            (Value::from("standard"), Value::from("..2500")),
        ]);
        let place = |v: Value| calculate("rangeLookup", &[tiers.clone(), v]);
        assert_eq!(place(Value::from(25000)), Ok(Value::from("platinum")));
        assert_eq!(place(Value::from("24999.99")), Ok(Value::from("gold")));
        assert_eq!(place(Value::from(10000)), Ok(Value::from("gold")));
        assert_eq!(place(Value::from(9999.99)), Ok(Value::from("silver")));
        assert_eq!(place(Value::from(0)), Ok(Value::from("standard")));
        assert_eq!(place(Value::from("-50")), Ok(Value::from("standard")));
        let scores = Value::Map(vec![
            (
                Value::from("keys"),
                Value::Array(vec![Value::from("excellent"), Value::from("poor")]),
            ),
            (Value::from("excellent"), Value::from("800..")),
            (Value::from("poor"), Value::from("300..580")),
        ]);
        assert_eq!(
            calculate("rangeLookup", &[scores.clone(), Value::from(850)]),
            Ok(Value::from("excellent"))
        );
        assert_eq!(
            calculate("rangeLookup", &[scores.clone(), Value::from(299)]),
            Ok(Value::Nil)
        );
        assert_eq!(
            calculate(
                "rangeLookup",
                &[
                    scores.clone(),
                    Value::from("299.99"),
                    Value::from("unknown")
                ]
            ),
            Ok(Value::from("unknown"))
        );
        assert_eq!(
            calculate(
                "rangeLookup",
                &[scores.clone(), Value::Nil, Value::from("unknown")]
            ),
            Ok(Value::from("unknown"))
        );
        // text bands: a same-day cut-off by time, initials, an ISO date window
        let cut_off = Value::Map(vec![
            (
                Value::from("keys"),
                Value::Array(vec![Value::from("same_day"), Value::from("next_day")]),
            ),
            (Value::from("same_day"), Value::from("00:00..16:00")),
            (Value::from("next_day"), Value::from("16:00..")),
        ]);
        assert_eq!(
            calculate("rangeLookup", &[cut_off.clone(), Value::from("15:59:59")]),
            Ok(Value::from("same_day"))
        );
        assert_eq!(
            calculate("rangeLookup", &[cut_off, Value::from("16:00")]),
            Ok(Value::from("next_day"))
        );
        let initials = Value::Map(vec![
            (
                Value::from("keys"),
                Value::Array(vec![Value::from("branch-a"), Value::from("branch-b")]),
            ),
            (Value::from("branch-a"), Value::from("a..h")),
            (Value::from("branch-b"), Value::from("h..")),
        ]);
        assert_eq!(
            calculate("rangeLookup", &[initials.clone(), Value::from("Garcia")]),
            Ok(Value::from("branch-a"))
        );
        assert_eq!(
            calculate("rangeLookup", &[initials, Value::from("HARRIS")]),
            Ok(Value::from("branch-b"))
        );
        let json =
            Value::from("{\"keys\": [\"low\", \"high\"], \"low\": \"..100\", \"high\": \"100..\"}");
        assert_eq!(
            calculate("rangeLookup", &[json.clone(), Value::from(99.5)]),
            Ok(Value::from("low"))
        );
        assert_eq!(
            calculate("rangeLookup", &[json, Value::from("100")]),
            Ok(Value::from("high"))
        );
        // errors by name
        let table = |range: &str| {
            Value::Map(vec![
                (Value::from("keys"), Value::Array(vec![Value::from("a")])),
                (Value::from("a"), Value::from(range)),
            ])
        };
        assert_eq!(
            calculate("rangeLookup", std::slice::from_ref(&scores)),
            Err("Expected two or three input values - actual=1".to_string())
        );
        assert!(calculate("rangeLookup", &[table("1-2"), Value::from(1)])
            .unwrap_err()
            .starts_with("Range of 'a' must be written as"));
        assert!(calculate("rangeLookup", &[table(".."), Value::from(1)])
            .unwrap_err()
            .starts_with("Range of 'a' must be written as"));
        assert!(calculate("rangeLookup", &[table("1..<2"), Value::from(1)])
            .unwrap_err()
            .starts_with("Range of 'a' must be written as"));
        assert!(calculate("rangeLookup", &[table("5..1"), Value::from(3)])
            .unwrap_err()
            .starts_with("Range of 'a' is empty"));
        assert!(calculate("rangeLookup", &[table("2..2"), Value::from(2)])
            .unwrap_err()
            .starts_with("Range of 'a' is empty"));
        let mixed = Value::Map(vec![
            (
                Value::from("keys"),
                Value::Array(vec![Value::from("a"), Value::from("b")]),
            ),
            (Value::from("a"), Value::from("1..2")),
            (Value::from("b"), Value::from("x..y")),
        ]);
        assert!(calculate("rangeLookup", &[mixed, Value::from(1)])
            .unwrap_err()
            .starts_with("Range table mixes numeric and text bounds"));
        assert!(
            calculate("rangeLookup", &[scores.clone(), Value::from("abc")])
                .unwrap_err()
                .contains("Expected a decimal number in rangeLookup")
        );
        assert!(calculate("rangeLookup", &[scores, Value::from(true)])
            .unwrap_err()
            .starts_with("rangeLookup value must be a number or text - got a boolean"));
    }

    /// Java `SimpleRangeLookupTest.shouldCheckEveryBandWhenTheTableIsRead`: a band
    /// that could never match, or one mixing a number and text, is refused
    /// whatever the value - even when an earlier band answers, even for Nil.
    #[test]
    fn range_lookup_checks_every_band_when_the_table_is_read() {
        let table = Value::Map(vec![
            (
                Value::from("keys"),
                Value::Array(vec![Value::from("a"), Value::from("b")]),
            ),
            (Value::from("a"), Value::from("1..2")),
            (Value::from("b"), Value::from("3 .. 3")),
        ]);
        assert_eq!(
            Err(
                "Range of 'b' is empty - its lower bound is not below its upper bound: '3 .. 3'"
                    .to_string()
            ),
            calculate("rangeLookup", &[table.clone(), Value::from(1)])
        );
        assert!(calculate("rangeLookup", &[table, Value::Nil])
            .unwrap_err()
            .starts_with("Range of 'b' is empty"));
        let mixed_band = Value::Map(vec![
            (Value::from("keys"), Value::Array(vec![Value::from("a")])),
            (Value::from("a"), Value::from("1..x")),
        ]);
        assert!(calculate("rangeLookup", &[mixed_band, Value::from(1)])
            .unwrap_err()
            .starts_with("Range of 'a' mixes a numeric and a text bound - got '1..x'"));
    }

    /// Java `RangeBoundsPluginTest`: a whole number is an integer, any other
    /// number its exact decimal text, a text or quoted bound text as written,
    /// an open end the default or Nil, and the refusals are rangeLookup's.
    #[test]
    fn range_bounds_match_java_semantics() {
        assert!(contains_simple_plugin("minRange"));
        assert!(contains_simple_plugin("maxRange"));
        let min = |args: &[Value]| calculate("minRange", args);
        let max = |args: &[Value]| calculate("maxRange", args);
        assert_eq!(
            Ok(Value::from(10000i64)),
            min(&[Value::from("10000..25000")])
        );
        assert_eq!(
            Ok(Value::from(25000i64)),
            max(&[Value::from("10000..25000")])
        );
        assert_eq!(Ok(Value::from(-5i64)), min(&[Value::from("-5..5")]));
        // a decimal bound is its exact text - the DECIMAL convention; so is a whole number beyond 64 bits
        assert_eq!(
            Ok(Value::from("2500.50")),
            min(&[Value::from("2500.50..10000")])
        );
        assert_eq!(Ok(Value::from("0.1")), max(&[Value::from("..0.1")]));
        assert_eq!(Ok(Value::from("2.0")), max(&[Value::from("1.0..2.0")]));
        assert_eq!(
            Ok(Value::from("100000000000000000000")),
            min(&[Value::from("100000000000000000000..")])
        );
        assert_eq!(Ok(Value::from(10i64)), min(&[Value::from(" 10 .. 20 ")]));
        assert_eq!(
            Ok(Value::from("00:00")),
            min(&[Value::from("00:00..16:00")])
        );
        assert_eq!(
            Ok(Value::from("16:00")),
            max(&[Value::from("00:00..16:00")])
        );
        assert_eq!(Ok(Value::from("H")), max(&[Value::from("A..H")]));
        // quoted text: as written, the quotes removed, a zero-padded code never a number
        assert_eq!(
            Ok(Value::from("0020")),
            min(&[Value::from("'0020'..'0030'")])
        );
        assert_eq!(
            Ok(Value::from("0030")),
            max(&[Value::from("\"0020\"..\"0030\"")])
        );
        assert_eq!(Ok(Value::from("a..b")), min(&[Value::from("'a..b'..'c'")]));
        assert_eq!(Ok(Value::from("1000")), min(&[Value::from("'1000'..")]));
        // the other quote character is ordinary text inside a quoted bound, in either position - the YAML convention
        assert_eq!(
            Ok(Value::from("this is a 'nice' drawing")),
            min(&[Value::from("\"this is a 'nice' drawing\"..\"zebra\"")])
        );
        assert_eq!(
            Ok(Value::from("this is a \"nice\" drawing")),
            max(&[Value::from("'a'..'this is a \"nice\" drawing'")])
        );
        // text orders by code point: U+FF10 sits below U+1F600, as the Java engine orders it since its UTF-16 fix
        assert_eq!(
            Ok(Value::from("\u{FF10}")),
            min(&[Value::from("'\u{FF10}'..'\u{1F600}'")])
        );
        // an open end: the default, or Nil
        assert_eq!(Ok(Value::Nil), min(&[Value::from("..2500")]));
        assert_eq!(Ok(Value::Nil), max(&[Value::from("25000..")]));
        assert_eq!(
            Ok(Value::from(0)),
            min(&[Value::from("..2500"), Value::from(0)])
        );
        assert_eq!(
            Ok(Value::from("none")),
            max(&[Value::from("25000.."), Value::from("none")])
        );
        assert_eq!(Ok(Value::from(2500i64)), max(&[Value::from("..2500")]));
        assert_eq!(
            Ok(Value::from(1i64)),
            min(&[Value::from("1..2"), Value::from(9)])
        );
        // the refusals
        assert!(min(&[])
            .unwrap_err()
            .starts_with("Expected one or two input values - actual=0"));
        // a Nil range has no bounds and never answers the default
        assert_eq!(Ok(Value::Nil), min(&[Value::Nil]));
        assert_eq!(Ok(Value::Nil), max(&[Value::Nil, Value::from(0)]));
        assert_eq!(
            Err("Range must be written as min..max, min.. or ..max (the upper bound is excluded) - got '1-2'".to_string()),
            max(&[Value::from("1-2")])
        );
        assert!(min(&[Value::from("1..<2")])
            .unwrap_err()
            .starts_with("Range must be written as"));
        assert!(min(&[Value::from(5)])
            .unwrap_err()
            .starts_with("Range must be written as"));
        assert!(min(&[Value::from("1..x")])
            .unwrap_err()
            .starts_with("Range mixes a numeric and a text bound - got '1..x'"));
        assert_eq!(
            Err(
                "Range is empty - its lower bound is not below its upper bound: '3 .. 3'"
                    .to_string()
            ),
            max(&[Value::from("3 .. 3")])
        );
        assert!(max(&[Value::from("h..a")])
            .unwrap_err()
            .starts_with("Range is empty"));
        // quoted bounds compare as written: 'a' is above 'B'
        assert!(max(&[Value::from("'a'..'B'")])
            .unwrap_err()
            .starts_with("Range is empty"));
        // a list is not a range, a second separator is malformed, quoting is all or nothing
        assert!(min(&[Value::Array(vec![Value::from("10000..25000")])])
            .unwrap_err()
            .starts_with("Range must be one range, not a list or a map"));
        assert!(min(&[Value::from("a..b..c")])
            .unwrap_err()
            .starts_with("Range must be written as"));
        assert!(min(&[Value::from("1...2")])
            .unwrap_err()
            .starts_with("Range must be written as"));
        assert!(min(&[Value::from("'a..b")])
            .unwrap_err()
            .starts_with("Range must be written as"));
        // a quoted bound never holds its own quote character, in either position - the other style does that
        assert!(min(&[Value::from("'a'b'..'z'")])
            .unwrap_err()
            .starts_with("Range must be written as"));
        assert!(max(&[Value::from("'a'..'b'c'")])
            .unwrap_err()
            .starts_with("Range must be written as"));
        assert!(max(&[Value::from("'a'..'b' x'")])
            .unwrap_err()
            .starts_with("Range must be written as"));
        assert!(min(&[Value::from("'a'..b")]).unwrap_err().starts_with(
            "Range mixes a quoted and an unquoted bound - quote both bounds or neither"
        ));
        assert!(min(&[Value::from("''..'m'")])
            .unwrap_err()
            .starts_with("Range has an empty quoted bound - an open end is written by omission"));
    }

    /// Quoted bounds make a table case-sensitive and keep zero-padded codes as text;
    /// a list band and a table mixing quoting are refused when the table is read.
    #[test]
    fn range_lookup_reads_quoted_bounds_as_written() {
        let table = |a: &str, b: &str| {
            Value::Map(vec![
                (
                    Value::from("keys"),
                    Value::Array(vec![Value::from("a"), Value::from("b")]),
                ),
                (Value::from("a"), Value::from(a)),
                (Value::from("b"), Value::from(b)),
            ])
        };
        let initials = table("'A'..'M'", "'M'..");
        assert_eq!(
            Ok(Value::from("b")),
            calculate("rangeLookup", &[initials.clone(), Value::from("m")])
        );
        assert_eq!(
            Ok(Value::from("a")),
            calculate("rangeLookup", &[initials, Value::from("B")])
        );
        let codes = table("'0000'..'1000'", "'1000'..");
        assert_eq!(
            Ok(Value::from("a")),
            calculate("rangeLookup", &[codes.clone(), Value::from("0450")])
        );
        assert_eq!(
            Ok(Value::from("b")),
            calculate("rangeLookup", &[codes, Value::from("1000")])
        );
        assert!(
            calculate("rangeLookup", &[table("'x'..'y'", "1..2"), Value::from(1)])
                .unwrap_err()
                .starts_with("Range table mixes quoted and unquoted bounds")
        );
        let list_band = Value::Map(vec![
            (Value::from("keys"), Value::Array(vec![Value::from("a")])),
            (
                Value::from("a"),
                Value::Array(vec![Value::from("10000..25000")]),
            ),
        ]);
        assert!(calculate("rangeLookup", &[list_band, Value::from(1)])
            .unwrap_err()
            .starts_with("Range of 'a' must be one range, not a list or a map"));
        // the other quote character is ordinary text inside a quoted bound, and text orders by code point
        let names = table("\"O'Brien\"..\"O'Neil\"", "'Z'..");
        assert_eq!(
            Ok(Value::from("a")),
            calculate("rangeLookup", &[names, Value::from("O'Hara")])
        );
        let wide = table("'\u{FF10}'..'\u{1F600}'", "'\u{1F600}'..");
        assert_eq!(
            Ok(Value::from("a")),
            calculate("rangeLookup", &[wide, Value::from("\u{FFE5}")])
        );
    }

    /// Java `SimpleRangeLookupTest.shouldRefuseAValueThatIsNotANumberOrText`: a
    /// band is a scale of numbers or text, so a boolean, a list, a map or bytes
    /// are refused by name on every table - a list compared as its text differed
    /// by engine.
    #[test]
    fn range_lookup_refuses_a_value_that_is_not_a_number_or_text() {
        let band = |range: &str| {
            Value::Map(vec![
                (Value::from("keys"), Value::Array(vec![Value::from("a")])),
                (Value::from("a"), Value::from(range)),
            ])
        };
        assert_eq!(
            Err("rangeLookup value must be a number or text - got a list".to_string()),
            calculate(
                "rangeLookup",
                &[band("a..h"), Value::Array(vec![Value::from("b")])]
            )
        );
        assert_eq!(
            Err("rangeLookup value must be a number or text - got a map".to_string()),
            calculate(
                "rangeLookup",
                &[
                    band("'a'..'h'"),
                    Value::Map(vec![(Value::from("b"), Value::from(1))])
                ]
            )
        );
        assert_eq!(
            Err("rangeLookup value must be a number or text - got a boolean".to_string()),
            calculate("rangeLookup", &[band("1..2"), Value::from(true)])
        );
        assert_eq!(
            Err("rangeLookup value must be a number or text - got bytes".to_string()),
            calculate("rangeLookup", &[band("a..h"), Value::Binary(vec![])])
        );
        // a Nil value still takes the default
        assert_eq!(
            Ok(Value::from("none")),
            calculate(
                "rangeLookup",
                &[band("a..h"), Value::Nil, Value::from("none")]
            )
        );
    }
}
