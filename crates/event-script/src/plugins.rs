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

use rmpv::Value;

use crate::conversions::{
    convert_boolean, convert_double, convert_float, convert_integer, convert_long, get_b64,
    get_binary_value, get_length, get_text_value,
};

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
    PLUGINS.get_or_init(|| RwLock::new(builtin_registrations()))
}

/// True when a plugin name is registered (Java `containsSimplePlugin`).
pub fn contains_simple_plugin(name: &str) -> bool {
    registry()
        .read()
        .expect("plugin registry")
        .contains_key(name)
}

/// Register a plugin body (the E-8 `#[simple_plugin]` macro will call this).
pub fn register_plugin(name: &str, body: PluginBody) {
    registry()
        .write()
        .expect("plugin registry")
        .insert(name.to_string(), Registration::Implemented(body));
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

/// Load every `#[simple_plugin]` from the link-time inventory (Java
/// `SimplePluginLoader.preloadSimplePlugins`). Returns how many were added.
pub fn load_inventory_plugins() -> usize {
    let mut count = 0;
    for entry in platform_core::inventory::iter::<SimplePluginEntry> {
        register_plugin(entry.name, entry.body);
        count += 1;
    }
    count
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

fn plugin_text(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Text conversion", |v| {
        Ok(Value::from(get_text_value(v)))
    })
}

fn plugin_int(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Integer conversion", |v| {
        Ok(Value::from(convert_integer(v)))
    })
}

fn plugin_long(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Long conversion", |v| {
        Ok(Value::from(convert_long(v)))
    })
}

fn plugin_float(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Float conversion", |v| {
        Ok(Value::F32(convert_float(v)))
    })
}

fn plugin_double(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Double conversion", |v| {
        Ok(Value::F64(convert_double(v)))
    })
}

fn plugin_boolean(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Boolean conversion", |v| {
        convert_boolean(v).map(Value::Boolean)
    })
}

fn plugin_binary(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Binary conversion", |v| {
        Ok(Value::Binary(get_binary_value(v)))
    })
}

fn plugin_b64(args: &[Value]) -> Result<Value, String> {
    one_or_list(args, "Base64 conversion", get_b64)
}

fn plugin_uuid(_args: &[Value]) -> Result<Value, String> {
    // Java UUIDGenerator ignores its input and returns a fresh v4 uuid
    Ok(Value::from(uuid::Uuid::new_v4().to_string()))
}

fn plugin_length(args: &[Value]) -> Result<Value, String> {
    match args {
        [one] => Ok(Value::from(get_length(one))),
        _ => Err("Expected exactly one argument in order to get Length".to_string()),
    }
}

fn plugin_not(args: &[Value]) -> Result<Value, String> {
    match args {
        [one] => Ok(Value::Boolean(!convert_boolean(one)?)),
        _ => Err("One single input is required for negation".to_string()),
    }
}

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

fn plugin_concat(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Input is required for String Concatenation".to_string());
    }
    Ok(Value::from(
        args.iter().map(get_text_value).collect::<String>(),
    ))
}

fn plugin_substring(args: &[Value]) -> Result<Value, String> {
    let [value, rest @ ..] = args else {
        return Err("Input is required for substring".to_string());
    };
    let text = get_text_value(value);
    let start = rest.first().map(convert_integer).unwrap_or(-1);
    let end = rest.get(1).map(convert_integer).unwrap_or(-1);
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len() as i64;
    // Java isOutOfBounds: end past the text, start past the text, or flipped
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
            chars[start as usize..end as usize]
                .iter()
                .collect::<String>(),
        ))
    } else if start >= 0 && start < len {
        Ok(Value::from(
            chars[start as usize..].iter().collect::<String>(),
        ))
    } else {
        Ok(Value::from(text))
    }
}

fn plugin_is_null(args: &[Value]) -> Result<Value, String> {
    match args {
        [one] => Ok(Value::Boolean(matches!(one, Value::Nil))),
        _ => Err("Only one value is accepted".to_string()),
    }
}

fn plugin_not_null(args: &[Value]) -> Result<Value, String> {
    match args {
        [one] => Ok(Value::Boolean(!matches!(one, Value::Nil))),
        _ => Err("Only one value is accepted".to_string()),
    }
}

fn plugin_eq(args: &[Value]) -> Result<Value, String> {
    let [first, rest @ ..] = args else {
        return Err("Input is required to check for equality".to_string());
    };
    if rest.is_empty() {
        return Err("Input is required to check for equality".to_string());
    }
    Ok(Value::Boolean(rest.iter().all(|v| v == first)))
}

fn plugin_ne(args: &[Value]) -> Result<Value, String> {
    let [first, rest @ ..] = args else {
        return Err("Input is required to check for inequality".to_string());
    };
    if rest.is_empty() {
        return Err("Input is required to check for inequality".to_string());
    }
    Ok(Value::Boolean(rest.iter().any(|v| v != first)))
}

fn builtin_registrations() -> HashMap<String, Registration> {
    let mut map = HashMap::new();
    let implemented: &[(&str, PluginBody)] = &[
        ("text", plugin_text),
        ("int", plugin_int),
        ("long", plugin_long),
        ("float", plugin_float),
        ("double", plugin_double),
        ("boolean", plugin_boolean),
        ("binary", plugin_binary),
        ("b64", plugin_b64),
        ("uuid", plugin_uuid),
        ("length", plugin_length),
        ("not", plugin_not),
        ("and", plugin_and),
        ("or", plugin_or),
        ("concat", plugin_concat),
        ("substring", plugin_substring),
        ("isNull", plugin_is_null),
        ("notNull", plugin_not_null),
        ("eq", plugin_eq),
        ("ne", plugin_ne),
    ];
    for (name, body) in implemented {
        map.insert(name.to_string(), Registration::Implemented(*body));
    }
    let completed: &[(&str, PluginBody)] = &[
        ("add", plugin_add),
        ("subtract", plugin_subtract),
        ("multiply", plugin_multiply),
        ("div", plugin_div),
        ("mod", plugin_mod),
        ("increment", plugin_increment),
        ("decrement", plugin_decrement),
        ("now", plugin_now),
        ("dateTime", plugin_date_time),
        ("gt", plugin_gt),
        ("lt", plugin_lt),
        ("ternary", plugin_ternary),
        ("startsWith", plugin_starts_with),
        ("endsWith", plugin_ends_with),
        ("includes", plugin_includes),
        ("parseDate", plugin_parse_date),
        ("parseDateTime", plugin_parse_date_time),
        ("listOfMap", plugin_list_of_map),
        ("updateListOfMap", plugin_update_list_of_map),
        ("removeKey", plugin_remove_key),
        ("uniqueSet", plugin_unique_set),
        ("defaultValue", plugin_default_value),
        ("validate", plugin_validate),
    ];
    for (name, body) in completed {
        map.insert(name.to_string(), Registration::Implemented(*body));
    }
    map
}

include!("plugins_e8.rs");

#[cfg(test)]
mod tests {
    use super::*;

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
}
