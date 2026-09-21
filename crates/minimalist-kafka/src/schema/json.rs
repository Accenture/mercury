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

//! The JSON Schema codec (Java `JsonSchemaSerde`): the payload behind the
//! Confluent frame is the JSON document itself. Like Confluent's serializer,
//! the document is validated against the registered schema only when
//! `json.fail.invalid.schema=true` (the default is off) — on both encode and
//! decode.

use jsonschema::Validator;

/// A registered JSON Schema, with its compiled validator when validation is on.
pub(crate) struct JsonSchema {
    validator: Option<Validator>,
}

impl JsonSchema {
    /// Parse the registered schema text; `strict` compiles the validator.
    pub(crate) fn parse(text: &str, strict: bool) -> Result<Self, String> {
        let document: serde_json::Value =
            serde_json::from_str(text).map_err(|e| format!("invalid JSON Schema text: {e}"))?;
        let validator = if strict {
            Some(
                jsonschema::validator_for(&document)
                    .map_err(|e| format!("invalid JSON Schema: {e}"))?,
            )
        } else {
            None
        };
        Ok(JsonSchema { validator })
    }

    fn validate(&self, value: &serde_json::Value) -> Result<(), String> {
        match &self.validator {
            None => Ok(()),
            Some(validator) => validator.validate(value).map_err(|e| {
                format!(
                    "JSON document does not conform to its schema: {e} (at {})",
                    e.instance_path()
                )
            }),
        }
    }

    /// The compact JSON document bytes (validated when strict).
    pub(crate) fn encode(&self, value: &serde_json::Value) -> Result<Vec<u8>, String> {
        self.validate(value)?;
        serde_json::to_vec(value).map_err(|e| format!("unable to render JSON: {e}"))
    }

    /// The dynamic body of a JSON payload (validated when strict).
    pub(crate) fn decode(&self, payload: &[u8]) -> Result<rmpv::Value, String> {
        let value: serde_json::Value =
            serde_json::from_slice(payload).map_err(|e| format!("payload is not JSON: {e}"))?;
        self.validate(&value)?;
        rmpv::ext::to_value(&value).map_err(|e| format!("unable to map JSON: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRICT: &str = r#"{"type":"object","properties":{"hello":{"type":"string"}},
        "required":["hello"],"additionalProperties":false}"#;

    #[test]
    fn default_codec_does_not_validate() {
        let schema = JsonSchema::parse(STRICT, false).expect("schema");
        let bytes = schema
            .encode(&serde_json::json!({"wrong": "shape"}))
            .expect("not validated");
        assert_eq!(b"{\"wrong\":\"shape\"}".to_vec(), bytes);
        let decoded = schema.decode(&bytes).expect("decoded");
        assert_eq!(
            serde_json::json!({"wrong": "shape"}),
            rmpv::ext::from_value::<serde_json::Value>(decoded).unwrap()
        );
    }

    #[test]
    fn strict_codec_rejects_non_conforming_documents_both_ways() {
        let schema = JsonSchema::parse(STRICT, true).expect("schema");
        let error = schema
            .encode(&serde_json::json!({"wrong": "shape"}))
            .expect_err("rejected");
        assert!(error.contains("does not conform to its schema"), "{error}");
        assert!(schema
            .encode(&serde_json::json!({"hello": "world"}))
            .is_ok());
        let error = schema
            .decode(b"{\"wrong\":\"shape\"}")
            .expect_err("rejected");
        assert!(error.contains("does not conform to its schema"), "{error}");
        assert!(schema
            .decode(b"not json")
            .expect_err("rejected")
            .contains("payload is not JSON"));
    }

    #[test]
    fn invalid_schema_text_is_reported() {
        assert!(JsonSchema::parse("{", false)
            .err()
            .expect("rejected")
            .contains("invalid JSON Schema text"));
    }
}
