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

//! Rust port of `com.accenture.automation.SimpleTypeMatchingConverter`:
//! rewrites the deprecated "simple type matching" syntax (a `:type` qualifier
//! on a `model.*` token) into the equivalent `f:plugin(...)` call at compile
//! time, so the runtime only deals with plugin syntax. An unrecognized or
//! malformed qualifier is returned unchanged for legacy resolution.

use crate::util::split;

const MODEL_NAMESPACE: &str = "model.";
const MAP_TO: &str = "->";
const SIMPLE_TYPES: &[&str] = &[
    "text", "binary", "int", "long", "float", "double", "boolean", "uuid", "length", "b64",
];

/// Convert one normalized 2-part `LHS -> RHS` entry (Java `convert`).
pub fn convert(entry: &str) -> String {
    let Some(sep) = entry.rfind(MAP_TO) else {
        return entry.to_string();
    };
    let lhs = entry[..sep].trim();
    let rhs = entry[sep + MAP_TO.len()..].trim();
    if let Some(colon) = type_colon_index(lhs) {
        let key = lhs[..colon].trim();
        if let Some(plugin) = to_plugin_call(key, lhs[colon + 1..].trim()) {
            return format!("{plugin} -> {rhs}");
        }
        return entry.to_string();
    }
    if let Some(colon) = type_colon_index(rhs) {
        let key = rhs[..colon].trim();
        if let Some(plugin) = to_plugin_call(lhs, rhs[colon + 1..].trim()) {
            return format!("{plugin} -> {key}");
        }
    }
    entry.to_string()
}

fn type_colon_index(token: &str) -> Option<usize> {
    if token.starts_with(MODEL_NAMESPACE) {
        token.find(':')
    } else {
        None
    }
}

fn to_plugin_call(source: &str, qualifier: &str) -> Option<String> {
    if SIMPLE_TYPES.contains(&qualifier) {
        return Some(format!("f:{qualifier}({source})"));
    }
    if qualifier == "!" {
        return Some(format!("f:not({source})"));
    }
    for (prefix, name) in [("and(", "and"), ("or(", "or")] {
        if let Some(inner) = qualifier
            .strip_prefix(prefix)
            .and_then(|q| q.strip_suffix(')'))
        {
            return Some(format!("f:{name}({source}, {})", inner.trim()));
        }
    }
    if let Some(inner) = qualifier
        .strip_prefix("concat(")
        .and_then(|q| q.strip_suffix(')'))
    {
        return Some(format!("f:concat({source}, {})", inner.trim()));
    }
    if let Some(inner) = qualifier
        .strip_prefix("substring(")
        .and_then(|q| q.strip_suffix(')'))
    {
        return Some(convert_substring(source, inner));
    }
    if is_boolean_value_match(qualifier) {
        return Some(convert_boolean_value_match(source, qualifier));
    }
    None
}

fn convert_substring(source: &str, inner: &str) -> String {
    let mut args = source.to_string();
    for part in split(inner, ",") {
        let v = part.trim().to_string();
        if v.starts_with(MODEL_NAMESPACE) {
            args.push_str(&format!(", {v}"));
        } else {
            args.push_str(&format!(", int({v})"));
        }
    }
    format!("f:substring({args})")
}

fn is_boolean_value_match(qualifier: &str) -> bool {
    qualifier.starts_with("boolean")
        && qualifier.ends_with(')')
        && qualifier["boolean".len()..].trim_start().starts_with('(')
}

fn convert_boolean_value_match(source: &str, qualifier: &str) -> String {
    let open = qualifier
        .find('(')
        .expect("checked by is_boolean_value_match");
    let inner = qualifier[open + 1..qualifier.len() - 1].trim();
    let (token, flag) = match inner.find('=') {
        Some(eq) => (inner[..eq].trim(), Some(inner[eq + 1..].trim())),
        None => (inner, None),
    };
    let condition_true = flag.is_none_or(|f| f.eq_ignore_ascii_case("true"));
    if token == "null" {
        let name = if condition_true { "isNull" } else { "notNull" };
        return format!("f:{name}({source})");
    }
    // string-coerce a model variable so value matching is type-independent
    let compared = if source.starts_with(MODEL_NAMESPACE) {
        format!("{source}:text")
    } else {
        source.to_string()
    };
    let name = if condition_true { "eq" } else { "ne" };
    format!("f:{name}({compared}, text({token}))")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_java_documented_cases() {
        // simple type qualifiers
        assert_eq!(convert("model.n:int -> value"), "f:int(model.n) -> value");
        assert_eq!(
            convert("input.x -> model.user:int"),
            "f:int(input.x) -> model.user"
        );
        // negate
        assert_eq!(convert("model.bool:! -> v"), "f:not(model.bool) -> v");
        // boolean null matching
        assert_eq!(
            convert("model.none:boolean (null = true) -> v"),
            "f:isNull(model.none) -> v"
        );
        assert_eq!(
            convert("model.none:boolean (null=false) -> v"),
            "f:notNull(model.none) -> v"
        );
        // boolean value matching string-coerces a model source
        assert_eq!(
            convert("model.x:boolean (yes) -> v"),
            "f:eq(model.x:text, text(yes)) -> v"
        );
        // concat with mixed args
        assert_eq!(
            convert("model.a:concat(model.space, text(,)) -> v"),
            "f:concat(model.a, model.space, text(,)) -> v"
        );
        // substring wraps numeric args
        assert_eq!(
            convert("model.s:substring(0, 5) -> v"),
            "f:substring(model.s, int(0), int(5)) -> v"
        );
        // unrecognized qualifier left unchanged
        assert_eq!(convert("model.s:bogus -> v"), "model.s:bogus -> v");
        // non-model tokens keep their qualifier (legacy runtime resolution)
        assert_eq!(convert("input.a:int -> v"), "input.a:int -> v");
    }
}
