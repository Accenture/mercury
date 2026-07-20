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

//! Rust port of `com.accenture.util.TypeConversionUtils` over the flow
//! state-machine value tree (`rmpv::Value` — the bus currency, so a byte
//! array is a real `Binary` value exactly like Java's `byte[]`).
//!
//! `display()` mirrors Java `String.valueOf` where the engine relies on it:
//! `null` renders as the string `"null"` (the boolean value-match commands
//! depend on that), and floats render with their decimal point (`12.0`,
//! not `12`).

use base64::Engine;
use rmpv::Value;

use crate::util::{str2double, str2float, str2int, str2long};

/// Java `String.valueOf` analog for state-machine values.
pub fn display(value: &Value) -> String {
    match value {
        Value::Nil => "null".to_string(),
        Value::String(s) => s.as_str().unwrap_or_default().to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Integer(i) => i.to_string(),
        // Java Float/Double.toString always keeps the decimal point
        Value::F32(f) => format!("{f:?}"),
        Value::F64(f) => format!("{f:?}"),
        // Java String.valueOf(byte[]) is meaningless ("[B@..."); rendering
        // UTF-8 is the pragmatic Rust divergence (doc'd)
        Value::Binary(b) => String::from_utf8_lossy(b).to_string(),
        other => other.to_string(),
    }
}

/// Java `getTextValue`: strings pass through, bytes decode as UTF-8, maps
/// render as JSON, everything else via `display`.
pub fn get_text_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.as_str().unwrap_or_default().to_string(),
        Value::Binary(b) => String::from_utf8_lossy(b).to_string(),
        Value::Map(_) | Value::Array(_) => to_json_string(value),
        other => display(other),
    }
}

/// Java `getBinaryValue`: bytes pass through, strings encode as UTF-8, maps
/// render as JSON bytes, everything else via `display` bytes.
pub fn get_binary_value(value: &Value) -> Vec<u8> {
    match value {
        Value::Binary(b) => b.clone(),
        Value::String(s) => s.as_str().unwrap_or_default().as_bytes().to_vec(),
        Value::Map(_) | Value::Array(_) => to_json_string(value).into_bytes(),
        other => display(other).into_bytes(),
    }
}

/// Java `getLength`: null → 0; bytes/list → element count; string → character
/// count; everything else → length of its display form.
pub fn get_length(value: &Value) -> i64 {
    match value {
        Value::Nil => 0,
        Value::Binary(b) => b.len() as i64,
        Value::String(s) => s.as_str().unwrap_or_default().chars().count() as i64,
        Value::Array(list) => list.len() as i64,
        other => display(other).chars().count() as i64,
    }
}

/// Java `getB64`: bytes → base64 text; base64 text → bytes (invalid input is
/// an error); anything else passes through unchanged.
pub fn get_b64(value: &Value) -> Result<Value, String> {
    match value {
        Value::Binary(b) => Ok(Value::from(
            base64::engine::general_purpose::STANDARD.encode(b),
        )),
        Value::String(s) => base64::engine::general_purpose::STANDARD
            .decode(s.as_str().unwrap_or_default())
            .map(Value::from)
            .map_err(|_| "invalid base64 text".to_string()),
        other => Ok(other.clone()),
    }
}

/// Java `getBooleanValue(value, command)` — the `boolean(target[=flag])`
/// value-match: the value's display form (null → "null") is compared with the
/// target; a match yields the flag (default true), a mismatch its inverse.
pub fn get_boolean_value(value: &Value, command: &str) -> Result<bool, String> {
    let filtered: Vec<&str> = command
        .split([',', '='])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if filtered.is_empty() || filtered.len() > 2 {
        return Err("invalid syntax".to_string());
    }
    let text = display(value);
    let condition = filtered.len() == 1 || filtered[1].eq_ignore_ascii_case("true");
    Ok(if text == filtered[0] {
        condition
    } else {
        !condition
    })
}

/// Java `convertBoolean`: booleans pass through, strings parse leniently
/// (only case-insensitive "true" is true), anything else is an error.
pub fn convert_boolean(value: &Value) -> Result<bool, String> {
    match value {
        Value::Boolean(b) => Ok(*b),
        Value::String(s) => Ok(s.as_str().unwrap_or_default().eq_ignore_ascii_case("true")),
        other => Err(format!(
            "Cannot convert input to boolean: {}",
            display(other)
        )),
    }
}

/// Java `convertInteger` (-1 when not numeric, decimal dropped).
pub fn convert_integer(value: &Value) -> i64 {
    match value {
        Value::Integer(i) => i.as_i64().unwrap_or(-1),
        other => str2int(&display(other)) as i64,
    }
}

/// Java `convertLong` (-1 when not numeric, decimal dropped).
pub fn convert_long(value: &Value) -> i64 {
    match value {
        Value::Integer(i) => i.as_i64().unwrap_or(-1),
        other => str2long(&display(other)),
    }
}

/// Java `convertFloat` (-1.0 when not numeric).
pub fn convert_float(value: &Value) -> f32 {
    match value {
        Value::F32(f) => *f,
        other => str2float(&display(other)),
    }
}

/// Java `convertDouble` (-1.0 when not numeric).
pub fn convert_double(value: &Value) -> f64 {
    match value {
        Value::F64(f) => *f,
        Value::F32(f) => *f as f64,
        other => str2double(&display(other)),
    }
}

/// Render any state-machine value as a JSON string (Java `SimpleMapper`).
pub fn to_json_string(value: &Value) -> String {
    match to_json(value) {
        Some(json) => json.to_string(),
        None => display(value),
    }
}

/// Convert a state-machine value into the JSON view (used by the `$.…`
/// JSONPath queries and JSON rendering). Binary becomes an array of numbers —
/// the standard serde encoding.
pub fn to_json(value: &Value) -> Option<serde_json::Value> {
    serde_json::to_value(value).ok()
}

/// Convert a JSON value into the state-machine tree.
pub fn from_json(value: &serde_json::Value) -> Value {
    match value {
        serde_json::Value::Null => Value::Nil,
        serde_json::Value::Bool(b) => Value::Boolean(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::from(i)
            } else if let Some(u) = n.as_u64() {
                Value::from(u)
            } else {
                Value::F64(n.as_f64().unwrap_or(-1.0))
            }
        }
        serde_json::Value::String(s) => Value::from(s.as_str()),
        serde_json::Value::Array(list) => Value::Array(list.iter().map(from_json).collect()),
        serde_json::Value::Object(map) => Value::Map(
            map.iter()
                .map(|(k, v)| (Value::from(k.as_str()), from_json(v)))
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_matches_java_string_valueof() {
        assert_eq!(display(&Value::Nil), "null");
        assert_eq!(display(&Value::from(12.0f64)), "12.0");
        assert_eq!(display(&Value::from(12.345f64)), "12.345");
        assert_eq!(display(&Value::Boolean(true)), "true");
        assert_eq!(display(&Value::from(42)), "42");
    }

    #[test]
    fn boolean_value_match_treats_null_as_text() {
        // boolean(null = true): a null value matches the "null" token
        assert_eq!(get_boolean_value(&Value::Nil, "null = true"), Ok(true));
        assert_eq!(get_boolean_value(&Value::Nil, "null=false"), Ok(false));
        assert_eq!(
            get_boolean_value(&Value::from("x"), "null = true"),
            Ok(false)
        );
        // boolean(value) defaults the flag to true
        assert_eq!(get_boolean_value(&Value::from("yes"), "yes"), Ok(true));
        assert_eq!(get_boolean_value(&Value::from("no"), "yes"), Ok(false));
    }

    #[test]
    fn b64_round_trip() {
        let encoded = get_b64(&Value::Binary(b"hello".to_vec())).unwrap();
        assert_eq!(encoded, Value::from("aGVsbG8="));
        let decoded = get_b64(&encoded).unwrap();
        assert_eq!(decoded, Value::Binary(b"hello".to_vec()));
        assert!(get_b64(&Value::from("!!! not base64 !!!")).is_err());
    }

    #[test]
    fn numeric_conversions_fall_back_to_minus_one() {
        assert_eq!(convert_integer(&Value::from("12.9")), 12);
        assert_eq!(convert_integer(&Value::from("abc")), -1);
        assert_eq!(convert_long(&Value::from(7)), 7);
        assert_eq!(convert_double(&Value::from("x")), -1.0);
    }

    #[test]
    fn lengths_match_java() {
        assert_eq!(get_length(&Value::Nil), 0);
        assert_eq!(get_length(&Value::from("hello")), 5);
        assert_eq!(get_length(&Value::Binary(vec![1, 2, 3])), 3);
        assert_eq!(
            get_length(&Value::Array(vec![Value::from(1), Value::from(2)])),
            2
        );
    }
}
