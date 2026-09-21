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

//! The Avro codec (Java `AvroSchemaSerde` + `AvroConversions`): the plain
//! JSON the flows use ↔ the Avro binary datum Confluent frames.
//!
//! **JSON → Avro** walks the *writer schema* rather than the value, so a
//! record field absent from the document takes its **schema default** (and a
//! missing field with no default fails fast, as Avro requires). This is why an
//! open, partial input — a request carrying only some of a record's fields —
//! serializes cleanly where Avro's own strict JSON decoder would reject it.
//! Records, arrays, maps, enums, unions (each branch tried in order, so the
//! common `[null, X]` nullable union and multi-branch unions both resolve),
//! fixed, bytes (a JSON string in Avro's ISO-8859-1 byte-string convention,
//! or an array of byte values), the logical types on their underlying
//! primitives, and numeric coercion so a JSON number fits `int` / `long` /
//! `float` / `double`.
//!
//! **Avro → dynamic body** renders a decoded datum as the flow body: a record
//! or map becomes a map (field order preserved), `bytes` / `fixed` / decimals
//! stay binary, enum symbols and UUIDs become strings, dates and timestamps
//! their integer representation — what Confluent's generic Avro deserializer
//! hands the Java module before it renders the record to a map.

use std::collections::HashMap;

use apache_avro::reader::datum::GenericDatumReader;
use apache_avro::schema::{
    InnerDecimalSchema, Name, Names, NamespaceRef, RecordSchema, ResolvedSchema, Schema,
    UnionSchema,
};
use apache_avro::types::Value as AvroValue;
use apache_avro::writer::datum::GenericDatumWriter;
use apache_avro::Duration;

/// A parsed Avro schema with its named types resolved.
pub(crate) struct AvroSchema {
    schema: Schema,
    /// Every named type in the schema by fully qualified name, for `Ref`s.
    names: Names,
}

impl AvroSchema {
    /// Parse the registered schema text.
    pub(crate) fn parse(text: &str) -> Result<Self, String> {
        let schema = Schema::parse_str(text).map_err(|e| format!("invalid Avro schema: {e}"))?;
        let names: Names = ResolvedSchema::try_from(&schema)
            .map_err(|e| format!("invalid Avro schema: {e}"))?
            .get_names()
            .iter()
            .map(|(name, schema)| (name.clone(), (*schema).clone()))
            .collect();
        Ok(AvroSchema { schema, names })
    }

    /// The Avro binary datum for a JSON document (no container header, as the
    /// Confluent frame carries it).
    pub(crate) fn encode(&self, value: &serde_json::Value) -> Result<Vec<u8>, String> {
        let avro = to_avro(value, &self.schema, &self.names, None)?;
        GenericDatumWriter::builder(&self.schema)
            .build()
            .and_then(|writer| writer.write_value_to_vec(avro))
            .map_err(|e| format!("Avro encoding failed: {e}"))
    }

    /// The dynamic body for an Avro binary datum written with this schema.
    pub(crate) fn decode(&self, payload: &[u8]) -> Result<rmpv::Value, String> {
        let mut reader = payload;
        let value = GenericDatumReader::builder(&self.schema)
            .build()
            .and_then(|datum_reader| datum_reader.read_value(&mut reader))
            .map_err(|e| format!("Avro decoding failed: {e}"))?;
        Ok(from_avro(value))
    }
}

/// Build the Avro value of `value` against `schema`, applying field defaults
/// (Java `AvroConversions.toAvro`).
fn to_avro(
    value: &serde_json::Value,
    schema: &Schema,
    names: &Names,
    enclosing: NamespaceRef<'_>,
) -> Result<AvroValue, String> {
    use serde_json::Value as Json;
    match schema {
        Schema::Null => match value {
            Json::Null => Ok(AvroValue::Null),
            other => Err(format!("expected null, got {}", kind(other))),
        },
        Schema::Boolean => value
            .as_bool()
            .map(AvroValue::Boolean)
            .ok_or_else(|| format!("expected a boolean, got {}", kind(value))),
        Schema::Int => int_of(value).map(AvroValue::Int),
        Schema::Long => long_of(value).map(AvroValue::Long),
        Schema::Float => double_of(value).map(|d| AvroValue::Float(d as f32)),
        Schema::Double => double_of(value).map(AvroValue::Double),
        Schema::Bytes => bytes_of(value).map(AvroValue::Bytes),
        Schema::String => match value {
            Json::String(s) => Ok(AvroValue::String(s.clone())),
            Json::Null => Err("expected a string, got null".to_string()),
            Json::Number(n) => Ok(AvroValue::String(n.to_string())),
            Json::Bool(b) => Ok(AvroValue::String(b.to_string())),
            other => Err(format!("expected a string, got {}", kind(other))),
        },
        Schema::Array(array) => {
            let Json::Array(items) = value else {
                return Err(format!(
                    "expected a list for an Avro array, got {}",
                    kind(value)
                ));
            };
            items
                .iter()
                .map(|item| to_avro(item, &array.items, names, enclosing))
                .collect::<Result<Vec<_>, _>>()
                .map(AvroValue::Array)
        }
        Schema::Map(map) => {
            let Json::Object(entries) = value else {
                return Err(format!(
                    "expected a map for an Avro map, got {}",
                    kind(value)
                ));
            };
            let mut out = HashMap::with_capacity(entries.len());
            for (key, item) in entries {
                out.insert(key.clone(), to_avro(item, &map.types, names, enclosing)?);
            }
            Ok(AvroValue::Map(out))
        }
        Schema::Union(union) => to_union(value, union, names, enclosing),
        Schema::Record(record) => to_record(value, record, names, enclosing),
        Schema::Enum(enumeration) => {
            let symbol = match value {
                Json::String(s) => s.clone(),
                other => other.to_string(),
            };
            let index = enumeration
                .symbols
                .iter()
                .position(|s| *s == symbol)
                .ok_or_else(|| {
                    format!(
                        "'{symbol}' is not a symbol of enum {} {:?}",
                        enumeration.name.fullname(enclosing),
                        enumeration.symbols
                    )
                })?;
            Ok(AvroValue::Enum(index as u32, symbol))
        }
        Schema::Fixed(fixed) => {
            let bytes = bytes_of(value)?;
            if bytes.len() != fixed.size {
                return Err(format!(
                    "fixed {} expects {} bytes, got {}",
                    fixed.name.fullname(enclosing),
                    fixed.size,
                    bytes.len()
                ));
            }
            Ok(AvroValue::Fixed(fixed.size, bytes))
        }
        Schema::Decimal(decimal) => {
            let bytes = bytes_of(value)?;
            match &decimal.inner {
                InnerDecimalSchema::Bytes => Ok(AvroValue::Bytes(bytes)),
                InnerDecimalSchema::Fixed(fixed) => Ok(AvroValue::Fixed(fixed.size, bytes)),
            }
        }
        Schema::BigDecimal => {
            Err("Avro big-decimal fields are not supported by this port".to_string())
        }
        Schema::Uuid(_) => {
            let Json::String(text) = value else {
                return Err(format!("expected a UUID string, got {}", kind(value)));
            };
            apache_avro::Uuid::parse_str(text)
                .map(AvroValue::Uuid)
                .map_err(|e| format!("'{text}' is not a UUID - {e}"))
        }
        Schema::Date => int_of(value).map(AvroValue::Date),
        Schema::TimeMillis => int_of(value).map(AvroValue::TimeMillis),
        Schema::TimeMicros => long_of(value).map(AvroValue::TimeMicros),
        Schema::TimestampMillis => long_of(value).map(AvroValue::TimestampMillis),
        Schema::TimestampMicros => long_of(value).map(AvroValue::TimestampMicros),
        Schema::TimestampNanos => long_of(value).map(AvroValue::TimestampNanos),
        Schema::LocalTimestampMillis => long_of(value).map(AvroValue::LocalTimestampMillis),
        Schema::LocalTimestampMicros => long_of(value).map(AvroValue::LocalTimestampMicros),
        Schema::LocalTimestampNanos => long_of(value).map(AvroValue::LocalTimestampNanos),
        Schema::Duration(_) => {
            let bytes = bytes_of(value)?;
            let fixed: [u8; 12] = bytes
                .try_into()
                .map_err(|_| "an Avro duration is exactly 12 bytes".to_string())?;
            Ok(AvroValue::Duration(Duration::from(fixed)))
        }
        Schema::Ref { name } => {
            let target = resolve_ref(name, names, enclosing)?;
            to_avro(value, target, names, enclosing)
        }
    }
}

/// A record from a JSON object: fields in schema order, an absent field takes
/// its default, an absent field without a default fails fast.
fn to_record(
    value: &serde_json::Value,
    record: &RecordSchema,
    names: &Names,
    enclosing: NamespaceRef<'_>,
) -> Result<AvroValue, String> {
    let fullname = record.name.fullname(enclosing);
    let serde_json::Value::Object(entries) = value else {
        return Err(format!(
            "expected a map for Avro record {fullname}, got {}",
            kind(value)
        ));
    };
    // the record's own namespace encloses its fields' named types
    let qualified = record.name.fully_qualified_name(enclosing);
    let inner_namespace = qualified.namespace();
    let mut fields = Vec::with_capacity(record.fields.len());
    for field in &record.fields {
        let converted = match entries.get(&field.name) {
            Some(present) => to_avro(present, &field.schema, names, inner_namespace)
                .map_err(|m| format!("field '{}' of {fullname}: {m}", field.name))?,
            None => match &field.default {
                // Avro JSON defaults: a union default is written in the FIRST
                // branch's encoding, which the union conversion resolves
                Some(default) => to_avro(default, &field.schema, names, inner_namespace)
                    .map_err(|m| format!("default of field '{}' of {fullname}: {m}", field.name))?,
                None => {
                    return Err(format!(
                        "field '{}' of {fullname} is missing and has no default",
                        field.name
                    ))
                }
            },
        };
        fields.push((field.name.clone(), converted));
    }
    Ok(AvroValue::Record(fields))
}

/// A union: null takes the null branch; anything else the first branch it
/// converts to, in declaration order (Java resolves against the first
/// non-null branch — this is that, generalized to unions of several types).
fn to_union(
    value: &serde_json::Value,
    union: &UnionSchema,
    names: &Names,
    enclosing: NamespaceRef<'_>,
) -> Result<AvroValue, String> {
    let variants = union.variants();
    if value.is_null() {
        return match variants.iter().position(|v| matches!(v, Schema::Null)) {
            Some(index) => Ok(AvroValue::Union(index as u32, Box::new(AvroValue::Null))),
            None => Err("null is not allowed by a union without a null branch".to_string()),
        };
    }
    let mut failures = Vec::new();
    for (index, variant) in variants.iter().enumerate() {
        if matches!(variant, Schema::Null) {
            continue;
        }
        match to_avro(value, variant, names, enclosing) {
            Ok(converted) => return Ok(AvroValue::Union(index as u32, Box::new(converted))),
            Err(m) => failures.push(m),
        }
    }
    Err(format!(
        "{} matches no branch of the union: {}",
        kind(value),
        failures.join("; ")
    ))
}

/// A named-type reference: the type it names, under the enclosing namespace.
fn resolve_ref<'n>(
    name: &Name,
    names: &'n Names,
    enclosing: NamespaceRef<'_>,
) -> Result<&'n Schema, String> {
    let qualified = name.fully_qualified_name(enclosing);
    names
        .get(qualified.as_ref())
        .or_else(|| names.get(name))
        .ok_or_else(|| {
            format!(
                "unresolved named type reference {}",
                qualified.fullname(None)
            )
        })
}

fn int_of(value: &serde_json::Value) -> Result<i32, String> {
    let n = long_of(value)?;
    i32::try_from(n).map_err(|_| format!("{n} does not fit an Avro int"))
}

fn long_of(value: &serde_json::Value) -> Result<i64, String> {
    match value {
        serde_json::Value::Number(n) => n
            .as_i64()
            .or_else(|| n.as_f64().filter(|f| f.fract() == 0.0).map(|f| f as i64))
            .ok_or_else(|| format!("{n} is not an integer")),
        serde_json::Value::String(s) => s
            .trim()
            .parse::<i64>()
            .map_err(|_| format!("'{s}' is not an integer")),
        other => Err(format!("expected a number, got {}", kind(other))),
    }
}

fn double_of(value: &serde_json::Value) -> Result<f64, String> {
    match value {
        serde_json::Value::Number(n) => n
            .as_f64()
            .ok_or_else(|| format!("{n} is not a floating-point number")),
        serde_json::Value::String(s) => s
            .trim()
            .parse::<f64>()
            .map_err(|_| format!("'{s}' is not a number")),
        other => Err(format!("expected a number, got {}", kind(other))),
    }
}

/// Bytes from a JSON value: Avro's JSON convention writes bytes as a string
/// whose characters are the byte values (ISO-8859-1); a list of byte values
/// is accepted too.
fn bytes_of(value: &serde_json::Value) -> Result<Vec<u8>, String> {
    match value {
        serde_json::Value::String(text) => text
            .chars()
            .map(|c| {
                u8::try_from(u32::from(c)).map_err(|_| {
                    format!("'{c}' is not a byte value (bytes are written as ISO-8859-1 text)")
                })
            })
            .collect(),
        serde_json::Value::Array(items) => items
            .iter()
            .map(|item| {
                item.as_u64()
                    .and_then(|n| u8::try_from(n).ok())
                    .ok_or_else(|| format!("{item} is not a byte value"))
            })
            .collect(),
        other => Err(format!(
            "expected bytes (a string or a list of byte values), got {}",
            kind(other)
        )),
    }
}

fn kind(value: &serde_json::Value) -> &'static str {
    match value {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "a boolean",
        serde_json::Value::Number(_) => "a number",
        serde_json::Value::String(_) => "a string",
        serde_json::Value::Array(_) => "a list",
        serde_json::Value::Object(_) => "a map",
    }
}

/// Render a decoded Avro value as the dynamic flow body (Java
/// `AvroConversions.fromAvro`).
fn from_avro(value: AvroValue) -> rmpv::Value {
    match value {
        AvroValue::Null => rmpv::Value::Nil,
        AvroValue::Boolean(b) => rmpv::Value::from(b),
        AvroValue::Int(i) => rmpv::Value::from(i),
        AvroValue::Long(l) => rmpv::Value::from(l),
        AvroValue::Float(f) => rmpv::Value::from(f),
        AvroValue::Double(d) => rmpv::Value::from(d),
        AvroValue::Bytes(bytes) => rmpv::Value::Binary(bytes),
        AvroValue::String(s) => rmpv::Value::from(s),
        AvroValue::Fixed(_, bytes) => rmpv::Value::Binary(bytes),
        AvroValue::Enum(_, symbol) => rmpv::Value::from(symbol),
        AvroValue::Union(_, inner) => from_avro(*inner),
        AvroValue::Array(items) => rmpv::Value::Array(items.into_iter().map(from_avro).collect()),
        AvroValue::Map(entries) => {
            let mut sorted: Vec<(String, AvroValue)> = entries.into_iter().collect();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            rmpv::Value::Map(
                sorted
                    .into_iter()
                    .map(|(k, v)| (rmpv::Value::from(k), from_avro(v)))
                    .collect(),
            )
        }
        AvroValue::Record(fields) => rmpv::Value::Map(
            fields
                .into_iter()
                .map(|(k, v)| (rmpv::Value::from(k), from_avro(v)))
                .collect(),
        ),
        AvroValue::Date(d) => rmpv::Value::from(d),
        AvroValue::Decimal(decimal) => {
            rmpv::Value::Binary(Vec::<u8>::try_from(&decimal).unwrap_or_default())
        }
        AvroValue::BigDecimal(big) => rmpv::Value::from(big.to_string()),
        AvroValue::TimeMillis(t) => rmpv::Value::from(t),
        AvroValue::TimeMicros(t) => rmpv::Value::from(t),
        AvroValue::TimestampMillis(t) => rmpv::Value::from(t),
        AvroValue::TimestampMicros(t) => rmpv::Value::from(t),
        AvroValue::TimestampNanos(t) => rmpv::Value::from(t),
        AvroValue::LocalTimestampMillis(t) => rmpv::Value::from(t),
        AvroValue::LocalTimestampMicros(t) => rmpv::Value::from(t),
        AvroValue::LocalTimestampNanos(t) => rmpv::Value::from(t),
        AvroValue::Duration(d) => rmpv::Value::Binary(<[u8; 12]>::from(d).to_vec()),
        AvroValue::Uuid(u) => rmpv::Value::from(u.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GREETING: &str = r#"{"type":"record","name":"Greeting","namespace":"demo",
        "fields":[{"name":"hello","type":"string"},{"name":"count","type":"int","default":0},
        {"name":"note","type":["null","string"],"default":null},
        {"name":"tags","type":{"type":"array","items":"string"},"default":[]}]}"#;

    fn map(value: rmpv::Value) -> serde_json::Value {
        super::super::json_view(&value)
    }

    /// A partial document round-trips: absent fields take their defaults.
    #[test]
    fn map_round_trips_through_a_record_with_defaults() {
        let schema = AvroSchema::parse(GREETING).expect("schema");
        let datum = schema
            .encode(&serde_json::json!({"hello": "avro", "tags": ["a", "b"]}))
            .expect("encode");
        let decoded = map(schema.decode(&datum).expect("decode"));
        assert_eq!(
            serde_json::json!({"hello": "avro", "count": 0, "note": null, "tags": ["a", "b"]}),
            decoded
        );
        // the hand-computed datum: "avro" (len 4 -> zigzag 8), count 0, union branch 0 (null), array end
        assert_eq!(
            vec![0x08, b'a', b'v', b'r', b'o', 0x00, 0x00, 0x04, 0x02, b'a', 0x02, b'b', 0x00],
            datum
        );
    }

    #[test]
    fn nested_record_enum_map_and_nullable_union_round_trip() {
        let schema = AvroSchema::parse(
            r#"{"type":"record","name":"Order","namespace":"demo","fields":[
                {"name":"id","type":"long"},
                {"name":"status","type":{"type":"enum","name":"Status","symbols":["NEW","PAID"]}},
                {"name":"customer","type":{"type":"record","name":"Customer","fields":[
                    {"name":"name","type":"string"},{"name":"vip","type":"boolean","default":false}]}},
                {"name":"again","type":["null","Customer"],"default":null},
                {"name":"attributes","type":{"type":"map","values":"double"}},
                {"name":"price","type":"float"},
                {"name":"raw","type":"bytes"}]}"#,
        )
        .expect("schema");
        let document = serde_json::json!({
            "id": 7, "status": "PAID", "customer": {"name": "Ann"},
            "again": {"name": "Bob", "vip": true},
            "attributes": {"weight": 1.5, "height": 2},
            "price": 9.75, "raw": "\u{0}\u{1}\u{ff}"
        });
        let datum = schema.encode(&document).expect("encode");
        let decoded = schema.decode(&datum).expect("decode");
        // bytes stay binary in the dynamic body
        let rmpv::Value::Map(entries) = &decoded else {
            panic!("record -> map")
        };
        let raw = entries
            .iter()
            .find(|(k, _)| k.as_str() == Some("raw"))
            .map(|(_, v)| v.clone())
            .expect("raw");
        assert_eq!(rmpv::Value::Binary(vec![0, 1, 255]), raw);
        let json = map(decoded);
        assert_eq!(7, json["id"]);
        assert_eq!("PAID", json["status"]);
        assert_eq!(
            serde_json::json!({"name": "Ann", "vip": false}),
            json["customer"]
        );
        assert_eq!(
            serde_json::json!({"name": "Bob", "vip": true}),
            json["again"]
        );
        assert_eq!(2.0, json["attributes"]["height"]);
        assert_eq!(9.75, json["price"]);
    }

    #[test]
    fn missing_required_field_fails_fast() {
        let schema = AvroSchema::parse(GREETING).expect("schema");
        let error = schema
            .encode(&serde_json::json!({"count": 3}))
            .expect_err("rejected");
        assert!(
            error.contains("field 'hello' of demo.Greeting is missing and has no default"),
            "{error}"
        );
    }

    #[test]
    fn non_map_for_a_record_and_bad_symbols_are_rejected() {
        let schema = AvroSchema::parse(GREETING).expect("schema");
        let error = schema
            .encode(&serde_json::json!(["not", "a", "map"]))
            .expect_err("rejected");
        assert!(
            error.contains("expected a map for Avro record demo.Greeting, got a list"),
            "{error}"
        );
        let enumeration = AvroSchema::parse(
            r#"{"type":"record","name":"E","fields":[{"name":"s","type":{"type":"enum","name":"S","symbols":["A"]}}]}"#,
        )
        .expect("schema");
        let error = enumeration
            .encode(&serde_json::json!({"s": "B"}))
            .expect_err("rejected");
        assert!(error.contains("'B' is not a symbol of enum S"), "{error}");
    }

    #[test]
    fn numeric_coercion_and_union_branch_selection() {
        let schema = AvroSchema::parse(
            r#"{"type":"record","name":"N","fields":[
                {"name":"i","type":"int"},{"name":"l","type":"long"},
                {"name":"d","type":"double"},{"name":"u","type":["null","long","string"]}]}"#,
        )
        .expect("schema");
        let datum = schema
            .encode(&serde_json::json!({"i": 5.0, "l": "42", "d": 3, "u": "text"}))
            .expect("encode");
        let json = map(schema.decode(&datum).expect("decode"));
        assert_eq!(
            serde_json::json!({"i": 5, "l": 42, "d": 3.0, "u": "text"}),
            json
        );
        let datum = schema
            .encode(&serde_json::json!({"i": 1, "l": 2, "d": 0.5, "u": 99}))
            .expect("encode");
        assert_eq!(99, map(schema.decode(&datum).expect("decode"))["u"]);
        let error = schema
            .encode(&serde_json::json!({"i": 3000000000u64, "l": 1, "d": 1, "u": null}))
            .expect_err("rejected");
        assert!(error.contains("does not fit an Avro int"), "{error}");
    }

    #[test]
    fn invalid_schema_text_is_reported() {
        assert!(AvroSchema::parse("{\"type\":\"nonsense\"}")
            .err()
            .expect("rejected")
            .contains("invalid Avro schema"));
    }
}
