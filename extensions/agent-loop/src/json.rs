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

//! Rust port of the Java `Json` (`org.platformlambda.agentloop.Json`): compact JSON text, written the
//! way the language packs write a tool result (`separators=(",", ":")`, non-ASCII characters as they
//! are): a map's keys in the order the map gives them, and a number as both engines print it (no
//! exponent, no trailing zeros). The loop uses it to measure a result against `result_limit`, to cut
//! one, and to quote a value in a message.
//!
//! MessagePack's binary and extension values have no JSON form; Java writes a `byte[]` as its object
//! identity (`[B@...`), which says nothing, so the port writes the bytes as UTF-8 text (lossy) and a
//! binary tool result is measured by its content.

use rmpv::Value;

use crate::tool_schema::Decimal;
use crate::value::key_text;

/// A value as compact JSON text.
pub fn compact(value: &Value) -> String {
    let mut out = String::new();
    write(&mut out, value);
    out
}

fn write(out: &mut String, value: &Value) {
    match value {
        Value::Nil => out.push_str("null"),
        Value::Boolean(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Integer(_) | Value::F32(_) | Value::F64(_) => match Decimal::of(value) {
            Some(n) => out.push_str(&n.render()),
            // a non-finite float has no decimal form: NaN and the infinities as Rust prints them
            None => out.push_str(&non_finite(value)),
        },
        Value::String(s) => quote(out, &String::from_utf8_lossy(s.as_bytes())),
        Value::Map(entries) => {
            out.push('{');
            for (i, (key, item)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                quote(out, &key_text(key));
                out.push(':');
                write(out, item);
            }
            out.push('}');
        }
        Value::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write(out, item);
            }
            out.push(']');
        }
        Value::Binary(bytes) => quote(out, &String::from_utf8_lossy(bytes)),
        Value::Ext(_, bytes) => quote(out, &String::from_utf8_lossy(bytes)),
    }
}

fn non_finite(value: &Value) -> String {
    match value {
        Value::F32(f) => f.to_string(),
        Value::F64(f) => f.to_string(),
        _ => String::new(),
    }
}

fn quote(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::map_of;

    #[test]
    fn compact_text_as_the_packs_write_it() {
        let value = map_of(vec![
            ("b", Value::from("é\n\u{1}\"")),
            (
                "a",
                Value::Array(vec![Value::from(1), Value::F64(2.0), Value::F64(1.5)]),
            ),
            ("n", Value::Nil),
            ("t", Value::Boolean(true)),
        ]);
        assert_eq!(
            compact(&value),
            "{\"b\":\"é\\n\\u0001\\\"\",\"a\":[1,2,1.5],\"n\":null,\"t\":true}"
        );
        assert_eq!(compact(&Value::F64(1e21)), "1000000000000000000000");
        assert_eq!(compact(&Value::F64(-0.0)), "0");
    }
}
