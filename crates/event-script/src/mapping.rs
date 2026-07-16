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

//! The runtime data-mapping engine — Rust port of the resolution half of
//! `com.accenture.util.DataMappingHelper` (the compile-time validation half
//! lives in [`validator`](crate::validator)), plus the `{model.key}`
//! runtime-variable substitution from `TaskExecutor`.
//!
//! Resolution model (maintainer decision 2026-07-16 — see [`mlm`](crate::mlm)):
//! composite-key traversal is the primary, lightweight access path; `$.…`
//! sources are user-defined complex queries running on the JSONPath engine.
//! Failure semantics are Java's: a bad type command logs an ERROR and passes
//! the original value through; a broken plugin call is an error the task
//! executor turns into a task failure (E-4).

use rmpv::Value;

use platform_core::{AppConfigReader, ConfigValue};

use crate::conversions::{
    display, get_b64, get_binary_value, get_boolean_value, get_length, get_text_value,
};
use crate::mlm::MultiLevelMap;
use crate::plugins;
use crate::util::{str2double, str2float, str2int, str2long};

const MODEL_NAMESPACE: &str = "model.";
const TEXT_TYPE: &str = "text(";

/// Java `SimpleFileDescriptor`: parses `file(...)` / `classpath(...)` — an
/// optional `text:` / `json:` / `binary:` / `append:` mode prefix, then the
/// path (normalized to a leading `/`). Default mode is binary.
pub struct SimpleFileDescriptor {
    pub file_name: String,
    pub mode: FileMode,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FileMode {
    Text,
    Binary,
    Json,
    Append,
}

impl SimpleFileDescriptor {
    pub fn parse(value: &str) -> Self {
        let last = value.rfind(')').unwrap_or(value.len());
        let offset = if value.starts_with("file(") {
            "file(".len()
        } else if value.starts_with("classpath(") {
            "classpath(".len()
        } else {
            0
        };
        let file_path = value[offset..last].trim();
        let (name, mode) = if let Some(n) = file_path.strip_prefix("text:") {
            (n, FileMode::Text)
        } else if let Some(n) = file_path.strip_prefix("json:") {
            (n, FileMode::Json)
        } else if let Some(n) = file_path.strip_prefix("binary:") {
            (n, FileMode::Binary)
        } else if let Some(n) = file_path.strip_prefix("append:") {
            (n, FileMode::Append)
        } else {
            (file_path, FileMode::Binary)
        };
        let file_name = if name.starts_with('/') {
            name.to_string()
        } else {
            format!("/{name}")
        };
        SimpleFileDescriptor { file_name, mode }
    }
}

/// Java `getLhsOrConstant`: constants win, otherwise resolve from the source.
pub fn get_lhs_or_constant(lhs: &str, source: &MultiLevelMap) -> Result<Option<Value>, String> {
    match get_constant_value(lhs) {
        Some(constant) => Ok(Some(constant)),
        None => get_lhs_element(lhs, source),
    }
}

/// Java `getLhsElement`: `$.…` JSONPath, `f:plugin(...)` invocation, or a
/// composite-key lookup with an optional legacy `:type` qualifier on a
/// `model.*` selector. `Err` = a failed plugin call (task-level failure).
pub fn get_lhs_element(lhs: &str, source: &MultiLevelMap) -> Result<Option<Value>, String> {
    if lhs.starts_with('$') {
        return Ok(source.get_element(lhs));
    }
    let colon = model_type_index(lhs);
    let selector = match colon {
        Some(c) => lhs[..c].trim(),
        None => lhs,
    };
    if selector.starts_with("f:") {
        return get_value_from_simple_plugin(selector, source).map(|v| match v {
            Value::Nil => None,
            other => Some(other),
        });
    }
    let value = source.get_element(selector);
    if let Some(c) = colon {
        let type_command = lhs[c + 1..].trim();
        let base = value.unwrap_or(Value::Nil);
        let converted = get_value_by_type(type_command, &base, &format!("LHS '{lhs}'"), source);
        return Ok(match converted {
            Value::Nil => None,
            other => Some(other),
        });
    }
    Ok(value)
}

/// Java `getConstantValue`: `text(...)`, numeric/boolean constants,
/// `map(k=v,...)` or `map(config.key)`, and `file(...)` / `classpath(...)`
/// content. Returns `None` when the token is not a constant.
pub fn get_constant_value(lhs: &str) -> Option<Value> {
    let last = lhs.rfind(')')?;
    if last == 0 {
        return None;
    }
    if let Some(inner) = constant_body(lhs, TEXT_TYPE, last) {
        return Some(Value::from(inner));
    }
    if let Some(inner) = constant_body(lhs, "int(", last) {
        return Some(Value::from(str2int(inner.trim()) as i64));
    }
    if let Some(inner) = constant_body(lhs, "long(", last) {
        return Some(Value::from(str2long(inner.trim())));
    }
    if let Some(inner) = constant_body(lhs, "float(", last) {
        return Some(Value::F32(str2float(inner.trim())));
    }
    if let Some(inner) = constant_body(lhs, "double(", last) {
        return Some(Value::F64(str2double(inner.trim())));
    }
    if let Some(inner) = constant_body(lhs, "boolean(", last) {
        return Some(Value::Boolean(inner.trim().eq_ignore_ascii_case("true")));
    }
    if lhs.starts_with("map(") {
        return get_constant_map_value(lhs, last);
    }
    if lhs.starts_with("file(") {
        return get_constant_file_value(lhs);
    }
    if lhs.starts_with("classpath(") {
        return get_constant_classpath_value(lhs);
    }
    None
}

fn constant_body<'a>(lhs: &'a str, prefix: &str, last: usize) -> Option<&'a str> {
    if lhs.starts_with(prefix) {
        Some(&lhs[prefix.len()..last])
    } else {
        None
    }
}

/// `map(k=v, ...)` literal, or `map(config.key)` reading the base application
/// configuration (Java parity).
fn get_constant_map_value(lhs: &str, last: usize) -> Option<Value> {
    let reference = lhs["map(".len()..last].trim();
    if reference.contains('=') || reference.contains(',') {
        let mut entries: Vec<(Value, Value)> = Vec::new();
        for kv in reference.split(',').filter(|s| !s.trim().is_empty()) {
            let (k, v) = match kv.find('=') {
                Some(eq) => (kv[..eq].trim(), kv[eq + 1..].trim()),
                None => (kv.trim(), ""),
            };
            if !k.is_empty() {
                entries.retain(|(key, _)| key.as_str() != Some(k));
                entries.push((Value::from(k), Value::from(v)));
            }
        }
        Some(Value::Map(entries))
    } else {
        let config = AppConfigReader::get_instance();
        config.get(reference).map(|v| config_to_value(&v))
    }
}

/// Bridge a configuration value into the state-machine tree.
fn config_to_value(value: &ConfigValue) -> Value {
    match value {
        ConfigValue::Null => Value::Nil,
        ConfigValue::Bool(b) => Value::Boolean(*b),
        ConfigValue::Int(i) => Value::from(*i),
        ConfigValue::Float(f) => Value::F64(*f),
        ConfigValue::Text(s) => Value::from(s.as_str()),
        ConfigValue::List(list) => Value::Array(list.iter().map(config_to_value).collect()),
        ConfigValue::Map(map) => Value::Map(
            map.iter()
                .map(|(k, v)| (Value::from(k.as_str()), config_to_value(v)))
                .collect(),
        ),
    }
}

fn get_constant_file_value(lhs: &str) -> Option<Value> {
    let fd = SimpleFileDescriptor::parse(lhs);
    let path = std::path::Path::new(&fd.file_name);
    if path.is_file() {
        read_content(lhs, &fd, &std::fs::read(path).ok()?)
    } else {
        None
    }
}

fn get_constant_classpath_value(lhs: &str) -> Option<Value> {
    let fd = SimpleFileDescriptor::parse(lhs);
    // the classpath analog: search the resource roots (Java getResourceAsStream)
    let resolved = platform_core::resources::resolve_classpath(&fd.file_name)?;
    read_content(lhs, &fd, &std::fs::read(resolved).ok()?)
}

fn read_content(lhs: &str, fd: &SimpleFileDescriptor, bytes: &[u8]) -> Option<Value> {
    match fd.mode {
        FileMode::Text => Some(Value::from(String::from_utf8_lossy(bytes).to_string())),
        FileMode::Json => Some(json_file_content(lhs, &String::from_utf8_lossy(bytes))),
        _ => Some(Value::Binary(bytes.to_vec())),
    }
}

/// Java `getJsonFileContent`: parse a JSON object/array, otherwise return the
/// text as-is (with a warning on malformed JSON).
fn json_file_content(lhs: &str, content: &str) -> Value {
    let json = content.trim();
    if (json.starts_with('[') && json.ends_with(']'))
        || (json.starts_with('{') && json.ends_with('}'))
    {
        match serde_json::from_str::<serde_json::Value>(json) {
            Ok(parsed) => return crate::conversions::from_json(&parsed),
            Err(e) => log::warn!("Unable to decode JSON file {lhs} - {e}"),
        }
    }
    Value::from(content)
}

fn model_type_index(text: &str) -> Option<usize> {
    if text.starts_with(MODEL_NAMESPACE) {
        text.find(':')
    } else {
        None
    }
}

/// Java `getValueFromSimplePlugin`: parse `f:name(arg, ...)`, evaluate each
/// argument (constants or source lookups; nested `f:` becomes null to avoid
/// execution loops), and invoke the plugin.
fn get_value_from_simple_plugin(selector: &str, source: &MultiLevelMap) -> Result<Value, String> {
    let start_paren = selector.find('(');
    let end_paren = selector.rfind(')');
    let (Some(start), Some(end)) = (start_paren, end_paren) else {
        return Ok(Value::Nil);
    };
    if start == 0 || end <= start {
        return Ok(Value::Nil);
    }
    let plugin_name = selector[2..start].trim();
    let mut args = Vec::new();
    for param in split_top_level_arguments(&selector[start + 1..end]) {
        let lhs = param.trim();
        if lhs.is_empty() || lhs.starts_with("f:") {
            args.push(Value::Nil);
        } else {
            args.push(get_lhs_or_constant(lhs, source)?.unwrap_or(Value::Nil));
        }
    }
    plugins::calculate(plugin_name, &args)
        .map_err(|e| format!("Unable to process SimplePlugin: {selector} - {e}"))
}

/// Split plugin arguments by top-level comma only, ignoring commas nested
/// inside parentheses (e.g. a `text(,)` constant argument) — Java
/// `splitTopLevelArguments`.
pub fn split_top_level_arguments(text: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut depth = 0usize;
    let mut current = String::new();
    for c in text.chars() {
        match c {
            '(' => {
                depth += 1;
                current.push(c);
            }
            ')' => {
                depth = depth.saturating_sub(1);
                current.push(c);
            }
            ',' if depth == 0 => {
                result.push(std::mem::take(&mut current));
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        result.push(current);
    }
    result
}

/// Java `getValueByType` — the legacy `:type` command on a non-model source
/// (model sources were rewritten to plugins at compile time). A bad command
/// logs an ERROR and returns the original value (Java parity).
pub fn get_value_by_type(
    type_command: &str,
    value: &Value,
    path: &str,
    data: &MultiLevelMap,
) -> Value {
    match apply_type_command(type_command, value, data) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Unable to do {type_command} of {path} - {e}");
            value.clone()
        }
    }
}

fn apply_type_command(
    type_command: &str,
    value: &Value,
    data: &MultiLevelMap,
) -> Result<Value, String> {
    if type_command.starts_with("substring(")
        || type_command.starts_with("concat(")
        || type_command.starts_with("and(")
        || type_command.starts_with("or(")
        || (type_command.starts_with("boolean") && type_command != "boolean")
    {
        let Some(open) = type_command.find('(') else {
            return Err("missing close bracket".to_string());
        };
        if !type_command.ends_with(')') {
            return Err("missing close bracket".to_string());
        }
        let command = type_command[open + 1..type_command.len() - 1].trim();
        if type_command.starts_with("substring(") {
            return get_substring(value, command);
        }
        if type_command.starts_with("concat(") {
            return get_concat_string(value, command, data);
        }
        if type_command.starts_with("and(") || type_command.starts_with("or(") {
            return get_logical_operation(value, command, data, type_command.starts_with("and("));
        }
        return get_boolean_value(value, command).map(Value::Boolean);
    }
    match type_command {
        "text" => Ok(Value::from(get_text_value(value))),
        "binary" => Ok(Value::Binary(get_binary_value(value))),
        "boolean" => Ok(Value::Boolean(display(value).eq_ignore_ascii_case("true"))),
        "!" => Ok(Value::Boolean(!display(value).eq_ignore_ascii_case("true"))),
        "int" => Ok(Value::from(str2int(&display(value)) as i64)),
        "long" => Ok(Value::from(str2long(&display(value)))),
        "float" => Ok(Value::F32(str2float(&display(value)))),
        "double" => Ok(Value::F64(str2double(&display(value)))),
        "uuid" => Ok(Value::from(uuid::Uuid::new_v4().to_string())),
        "length" => Ok(Value::from(get_length(value))),
        "b64" => get_b64(value),
        _ => Err(
            "matching type must be substring(start, end), concat, boolean, !, and, or, \
                  text, binary, uuid or b64"
                .to_string(),
        ),
    }
}

/// Java `getSubstring` (the legacy `:substring(a[,b])` command).
fn get_substring(value: &Value, command: &str) -> Result<Value, String> {
    let parts: Vec<&str> = command
        .split([',', ' '])
        .filter(|s| !s.is_empty())
        .collect();
    if parts.is_empty() || parts.len() > 2 {
        return Err("invalid syntax".to_string());
    }
    let Value::String(s) = value else {
        return Err("value is not a string".to_string());
    };
    let text = s.as_str().unwrap_or_default();
    let chars: Vec<char> = text.chars().collect();
    let start = str2int(parts[0]) as i64;
    let end = if parts.len() == 1 {
        chars.len() as i64
    } else {
        str2int(parts[1]) as i64
    };
    if end > start && start >= 0 && end <= chars.len() as i64 {
        Ok(Value::from(
            chars[start as usize..end as usize]
                .iter()
                .collect::<String>(),
        ))
    } else {
        Err("index out of bound".to_string())
    }
}

/// Java `getConcatString` (the legacy `:concat(...)` command): parameters are
/// model variables and/or text constants.
fn get_concat_string(value: &Value, command: &str, data: &MultiLevelMap) -> Result<Value, String> {
    let parts = tokenize_concat_parameters(command);
    if parts.is_empty() {
        return Err("parameters must be model variables and/or text constants".to_string());
    }
    let mut out = display(value);
    for p in parts {
        if let Some(rest) = p.strip_prefix(TEXT_TYPE) {
            out.push_str(&rest[..rest.len() - 1]);
        } else if p.starts_with(MODEL_NAMESPACE) {
            out.push_str(&display(&data.get_element(&p).unwrap_or(Value::Nil)));
        }
    }
    Ok(Value::from(out))
}

/// Java `getLogicalOperation` (the legacy `:and(model.k)` / `:or(model.k)`
/// commands): case-sensitive "true" comparison on both display forms.
fn get_logical_operation(
    value: &Value,
    command: &str,
    data: &MultiLevelMap,
    is_and: bool,
) -> Result<Value, String> {
    if command.starts_with(MODEL_NAMESPACE) && command.len() > MODEL_NAMESPACE.len() {
        let v1 = display(value) == "true";
        let v2 = display(&data.get_element(command).unwrap_or(Value::Nil)) == "true";
        Ok(Value::Boolean(if is_and { v1 && v2 } else { v1 || v2 }))
    } else {
        Err(format!("'{command}' is not a model variable"))
    }
}

/// Java `tokenizeConcatParameters`: a comma-separated sequence of `model.*`
/// tokens and `text(...)` constants; anything else empties the list.
fn tokenize_concat_parameters(text: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut command = text.trim();
    while !command.is_empty() {
        if command.starts_with(MODEL_NAMESPACE) {
            match command.find(',') {
                None => {
                    result.push(command.to_string());
                    return result;
                }
                Some(sep) => {
                    let token = command[..sep].trim();
                    if token == MODEL_NAMESPACE {
                        return Vec::new();
                    }
                    result.push(token.to_string());
                    command = command[sep + 1..].trim_start();
                }
            }
        } else if command.starts_with(TEXT_TYPE) {
            let Some(close) = command.find(')') else {
                return Vec::new();
            };
            if close == TEXT_TYPE.len() - 1 {
                return Vec::new();
            }
            result.push(command[..close + 1].to_string());
            match command[close..].find(',') {
                None => return result,
                Some(offset) => command = command[close + offset + 1..].trim_start(),
            }
        } else {
            return Vec::new();
        }
    }
    result
}

/// The `{model.key}` runtime interpolation (Java
/// `TaskExecutor.substituteRuntimeVarsIfAny`): every `{...}` whose content is
/// a `model.*` key is replaced with the model value's display form (strings
/// and numbers render as-is, anything else — including a missing key — as
/// "null"); other bracketed text passes through unchanged.
pub fn substitute_runtime_vars(text: &str, source: &MultiLevelMap) -> String {
    if !text.contains('{') || !text.contains('}') {
        return text.to_string();
    }
    let mut out = String::new();
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        let Some(close_rel) = rest[open..].find('}') else {
            break;
        };
        let close = open + close_rel;
        out.push_str(&rest[..open]);
        let middle = rest[open + 1..close].trim();
        if middle.starts_with(MODEL_NAMESPACE) && !middle.ends_with('.') {
            out.push_str(&string_from_model_value(source.get_element(middle)));
        } else {
            out.push_str(&rest[open..=close]);
        }
        rest = &rest[close + 1..];
    }
    out.push_str(rest);
    out
}

/// Java `getStringFromModelValue`: only strings and numbers interpolate;
/// everything else (including a missing key) renders as "null".
fn string_from_model_value(value: Option<Value>) -> String {
    match value {
        Some(Value::String(_))
        | Some(Value::Integer(_))
        | Some(Value::F32(_))
        | Some(Value::F64(_)) => display(&value.expect("checked")),
        _ => "null".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model() -> MultiLevelMap {
        let mut m = MultiLevelMap::new();
        m.set_element("model.pointer", Value::from("world"))
            .unwrap();
        m.set_element("model.world", Value::from("wonderful day"))
            .unwrap();
        m.set_element("model.n", Value::from(5)).unwrap();
        m.set_element("model.flag", Value::from("true")).unwrap();
        m
    }

    #[test]
    fn constants_resolve_like_java() {
        assert_eq!(
            get_constant_value("text(hello world)"),
            Some(Value::from("hello world"))
        );
        assert_eq!(get_constant_value("int(42)"), Some(Value::from(42)));
        assert_eq!(get_constant_value("long(12345)"), Some(Value::from(12345)));
        assert_eq!(
            get_constant_value("boolean(TRUE)"),
            Some(Value::Boolean(true))
        );
        assert_eq!(
            get_constant_value("boolean(nope)"),
            Some(Value::Boolean(false))
        );
        // text constants preserve inner spaces and special characters
        assert_eq!(get_constant_value("text( )"), Some(Value::from(" ")));
        assert_eq!(
            get_constant_value("text(a -> b)"),
            Some(Value::from("a -> b"))
        );
        // map literal (duplicate keys keep the last)
        let map = get_constant_value("map(direction=right, test=message)").unwrap();
        let Value::Map(entries) = map else {
            panic!("expected map")
        };
        assert_eq!(entries.len(), 2);
        // not constants
        assert_eq!(get_constant_value("input.body"), None);
        assert_eq!(get_constant_value("model.x"), None);
    }

    #[test]
    fn runtime_var_interpolation_matches_java() {
        let m = model();
        // {model.pointer} resolves to "world"
        assert_eq!(
            substitute_runtime_vars("model.{model.pointer}", &m),
            "model.world"
        );
        assert_eq!(
            substitute_runtime_vars("new {model.pointer}", &m),
            "new world"
        );
        // non-model bracketed text passes through unchanged
        assert_eq!(
            substitute_runtime_vars("keep {this}/{ one } unchanged", &m),
            "keep {this}/{ one } unchanged"
        );
        // missing model key renders as "null"
        assert_eq!(substitute_runtime_vars("x {model.gone} y", &m), "x null y");
    }

    #[test]
    fn plugin_sources_resolve_through_the_registry() {
        let m = model();
        // f:int over a model lookup
        let v = get_lhs_element("f:int(model.n)", &m).unwrap();
        assert_eq!(v, Some(Value::from(5)));
        // f:concat with a text(,) constant argument (top-level comma safety)
        let v = get_lhs_element("f:concat(model.pointer, text(,), model.n)", &m).unwrap();
        assert_eq!(v, Some(Value::from("world,5")));
        // nested plugin arguments are nulled (execution-loop guard)
        let v = get_lhs_element("f:isNull(f:uuid(model.n))", &m).unwrap();
        assert_eq!(v, Some(Value::Boolean(true)));
        // unknown plugin is an error
        assert!(get_lhs_element("f:bogus(model.n)", &m).is_err());
    }

    #[test]
    fn legacy_type_commands_match_java() {
        let m = model();
        let v = get_value_by_type("int", &Value::from("12.9"), "test", &m);
        assert_eq!(v, Value::from(12));
        let v = get_value_by_type("!", &Value::from("true"), "test", &m);
        assert_eq!(v, Value::Boolean(false));
        let v = get_value_by_type("boolean(null=true)", &Value::Nil, "test", &m);
        assert_eq!(v, Value::Boolean(true));
        let v = get_value_by_type("and(model.flag)", &Value::from("true"), "test", &m);
        assert_eq!(v, Value::Boolean(true));
        let v = get_value_by_type("or(model.missing)", &Value::from("false"), "test", &m);
        assert_eq!(v, Value::Boolean(false));
        let v = get_value_by_type("substring(0, 3)", &Value::from("hello"), "test", &m);
        assert_eq!(v, Value::from("hel"));
        // substring out of bounds returns the original value (ERROR logged)
        let v = get_value_by_type("substring(0, 99)", &Value::from("hello"), "test", &m);
        assert_eq!(v, Value::from("hello"));
        let v = get_value_by_type(
            "concat(model.pointer, text(!))",
            &Value::from("hello "),
            "test",
            &m,
        );
        assert_eq!(v, Value::from("hello world!"));
        // unknown command returns the original value (ERROR logged)
        let v = get_value_by_type("bogus", &Value::from("x"), "test", &m);
        assert_eq!(v, Value::from("x"));
    }

    #[test]
    fn lhs_resolution_covers_all_source_kinds() {
        let m = model();
        // plain composite key
        assert_eq!(
            get_lhs_element("model.n", &m).unwrap(),
            Some(Value::from(5))
        );
        // legacy :type on a model selector (runtime form)
        assert_eq!(
            get_lhs_element("model.n:text", &m).unwrap(),
            Some(Value::from("5"))
        );
        // JSONPath complex query
        assert_eq!(
            get_lhs_element("$.model.n", &m).unwrap(),
            Some(Value::from(5))
        );
        // constant via get_lhs_or_constant
        assert_eq!(
            get_lhs_or_constant("text(hi)", &m).unwrap(),
            Some(Value::from("hi"))
        );
        // missing key
        assert_eq!(get_lhs_element("model.gone", &m).unwrap(), None);
    }

    #[test]
    fn file_descriptor_parses_like_java() {
        let fd = SimpleFileDescriptor::parse("file(text:/tmp/x.txt)");
        assert_eq!(fd.mode, FileMode::Text);
        assert_eq!(fd.file_name, "/tmp/x.txt");
        let fd = SimpleFileDescriptor::parse("file(append:tmp/y.log)");
        assert_eq!(fd.mode, FileMode::Append);
        assert_eq!(fd.file_name, "/tmp/y.log");
        let fd = SimpleFileDescriptor::parse("classpath(json:data/sample.json)");
        assert_eq!(fd.mode, FileMode::Json);
        assert_eq!(fd.file_name, "/data/sample.json");
        // no mode prefix defaults to binary
        let fd = SimpleFileDescriptor::parse("file(/tmp/z.bin)");
        assert_eq!(fd.mode, FileMode::Binary);
    }
}
