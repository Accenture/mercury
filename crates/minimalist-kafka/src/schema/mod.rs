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

//! **Schema Registry support** (Rust port of the Java
//! `org.platformlambda.mini.kafka.schema` package): the Confluent Schema
//! Registry wire format — `[magic 0x00][4-byte global schema id][payload]` —
//! for **JSON Schema** and **Avro** values, so the raw-`byte[]` building blocks
//! interoperate with existing Confluent client projects.
//!
//! - **Produce** is subject-driven: `simple.kafka.notification` resolves a
//!   `subject` (+ optional `version`, default `latest`) to a pre-registered
//!   global schema id and its type, then frames the body — a JSON document —
//!   with that id. The producer never registers a schema: schemas are governed
//!   artifacts registered out-of-band, and whoever registers owns the subject
//!   naming strategy.
//! - **Consume** decodes by embedded id: a `schema.enabled` binding reads the
//!   magic byte + id, looks up the registered schema's type, dispatches to the
//!   matching decoder and hands the flow a map body instead of bytes. A decode
//!   failure is a poison message and is dead-lettered at once (retrying cannot
//!   help).
//!
//! **What the Java module delegates to Confluent's own serializers, this port
//! implements with the Apache Avro reference crate and a JSON Schema
//! validator** — there is no Confluent client library for Rust. The frame,
//! the subject/version resolution, the id-immutable positive-results-only
//! cache (`ManagedCache`, TTL `schema.registry.cache.ttl`) and the registry
//! client template (`schema-registry.yml`: OAuth 2.0 client credentials,
//! static bearer, basic auth, SASL inheritance) are the same. Stated deltas
//! (port spec §7): the registry template is **interpreted**, not passed
//! through verbatim — a key this client has no analog for is logged and
//! ignored; TLS trust comes from the OS trust store (the platform's HTTP
//! client); Confluent **CSFLE** (field-level encryption) and **schema
//! references** are refused with a clear error rather than silently served
//! in plaintext or half-resolved; **Protobuf** is recognized and refused, as
//! on Java (`SchemaType::Protobuf`).
//!
//! The codec is shared: unlike Confluent's serdes it is thread-safe, so one
//! `Arc<SchemaCodec>` serves every producer instance and every binding
//! consumer of the registry it was built for. A second registry (twin-kafka's
//! seam) builds a second codec under its own key prefix — global ids are only
//! unique within one registry, so the caches are prefixed too.

pub mod auth;
mod avro;
mod json;
pub mod registry;

use std::collections::BTreeMap;
use std::sync::Arc;

use platform_core::{AppConfigReader, AppError, ManagedCache};

use registry::{Parsed, RegisteredSchema, RegistryClient};

/// The feature switch: unset/blank = schema features off, the library keeps
/// its raw-bytes behaviour.
pub const REGISTRY_URL: &str = "schema.registry.url";
/// The default config-key and cache-name prefix (`schema.registry`).
pub const DEFAULT_KEY_PREFIX: &str = "schema.registry";
/// The reserved version alias that resolves to a subject's newest version.
pub const LATEST: &str = "latest";
/// The Confluent wire-format magic byte.
pub const MAGIC_BYTE: u8 = 0x0;
/// Magic byte + 4-byte big-endian global id.
pub const FRAME_HEADER_LEN: usize = 5;

const DEFAULT_CACHE_TTL: &str = "30m";
/// A pinned subject + numeric version is immutable, so it can be cached
/// effectively forever; a long TTL plus a bounded item count keeps it fresh
/// enough while removing any unbounded-growth risk (Java parity).
const DEFAULT_VERSION_CACHE_TTL: &str = "10d";
const VERSION_CACHE_MAX_ITEMS: u64 = 3000;
/// The one Confluent serde setting with an analog in this client: validate
/// JSON payloads against their schema on both encode and decode (Confluent's
/// `json.fail.invalid.schema`, default false).
const STRICT_JSON_KEY: &str = "json.fail.invalid.schema";

/// The registered schema type — `schemaType` in the registry (Java
/// `SchemaType`). Protobuf is recognized so a misconfigured attempt fails
/// clearly, never silently.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SchemaType {
    Json,
    Avro,
    Protobuf,
}

impl SchemaType {
    /// Parse the registry's `schemaType` field: Confluent omits it for Avro
    /// (the default) and includes `JSON` / `PROTOBUF` otherwise.
    pub fn from_registry(value: Option<&str>) -> Result<SchemaType, AppError> {
        match value.map(str::trim).filter(|v| !v.is_empty()) {
            None => Ok(SchemaType::Avro),
            Some(text) => Self::parse(text),
        }
    }

    /// Parse a type name case-insensitively (`JSON`, `AVRO`, `PROTOBUF`).
    pub fn parse(text: &str) -> Result<SchemaType, AppError> {
        match text.trim().to_ascii_uppercase().as_str() {
            "JSON" => Ok(SchemaType::Json),
            "AVRO" => Ok(SchemaType::Avro),
            "PROTOBUF" => Ok(SchemaType::Protobuf),
            other => Err(AppError::new(
                500,
                format!("Unknown schema type '{other}' (expected JSON, AVRO or PROTOBUF)"),
            )),
        }
    }

    /// The registry spelling.
    pub fn name(&self) -> &'static str {
        match self {
            SchemaType::Json => "JSON",
            SchemaType::Avro => "AVRO",
            SchemaType::Protobuf => "PROTOBUF",
        }
    }
}

impl std::fmt::Display for SchemaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// A `(subject, version)` resolved to its global schema id and type — the
/// producer-side convenience that lets a flow name a subject instead of
/// knowing the id (Java `ResolvedSchema`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResolvedSchema {
    pub id: i32,
    pub schema_type: SchemaType,
}

/// Bridges the minimalist bytes transport to the Confluent wire format — see
/// the module documentation. Build one per registry with
/// [`SchemaCodec::from_config`] (the default `schema.registry.*` keys) or
/// [`SchemaCodec::for_registry`] (a caller-selected key prefix).
pub struct SchemaCodec {
    client: RegistryClient,
    registry_url: String,
    key_prefix: String,
}

impl SchemaCodec {
    /// Build the codec from application config, or `None` when
    /// `schema.registry.url` is unset/blank (schema features off).
    pub fn from_config(config: &AppConfigReader) -> Result<Option<Arc<SchemaCodec>>, AppError> {
        let url = config.get_property(REGISTRY_URL);
        Self::for_registry(config, url.as_deref(), DEFAULT_KEY_PREFIX)
    }

    /// Build a codec for a registry under a caller-selected key prefix — the
    /// reuse seam for a library whose second Kafka cluster has its own Schema
    /// Registry. Every key and cache name derives from the prefix (example
    /// `secondary.schema.registry`): cache TTL keys `<prefix>.cache.ttl` /
    /// `<prefix>.version.cache.ttl`; serde pass-through prefix
    /// `<prefix>.serde.`; `ManagedCache` names `<prefix>` and
    /// `<prefix>.version`; the client template location key
    /// `<prefix>.properties` with the default
    /// `classpath:/<prefix-with-dashes>.yml`. Distinct cache names per prefix
    /// are a correctness requirement: global ids are only unique within one
    /// registry. `None` when `registry_url` is unset/blank.
    pub fn for_registry(
        config: &AppConfigReader,
        registry_url: Option<&str>,
        key_prefix: &str,
    ) -> Result<Option<Arc<SchemaCodec>>, AppError> {
        let Some(url) = registry_url.map(str::trim).filter(|u| !u.is_empty()) else {
            return Ok(None);
        };
        let ttl_ms = duration_ms(
            config,
            &format!("{key_prefix}.cache.ttl"),
            DEFAULT_CACHE_TTL,
        )?;
        let version_ttl_ms = duration_ms(
            config,
            &format!("{key_prefix}.version.cache.ttl"),
            DEFAULT_VERSION_CACHE_TTL,
        )?;
        // the schema cache is rebuildable from the registry, so it is cleared
        // at startup - a stale entry left by an earlier run in this process
        // (another test, a restart-in-place) is never served
        let cache = ManagedCache::create_cache(key_prefix, ttl_ms);
        cache.clear();
        let version_cache = ManagedCache::create_cache_with_limit(
            &format!("{key_prefix}.version"),
            version_ttl_ms,
            VERSION_CACHE_MAX_ITEMS,
        );
        version_cache.clear();
        // the registry client template: authentication and the optional
        // installation headers, interpreted (Java passes it verbatim to the
        // Confluent client; this client names what it honours)
        let template = auth::load_template(config, key_prefix)?;
        let settings = auth::RegistrySettings::from_template(&template, url)?;
        // the serdes inherit the registry client template, then the
        // <prefix>.serde.* overrides apply on top (Java parity); the only
        // serde setting with an analog here is json.fail.invalid.schema
        let overrides = serde_overrides(config, &format!("{key_prefix}.serde."));
        let mut serde_config = template.clone();
        serde_config.extend(overrides.iter().map(|(k, v)| (k.clone(), v.clone())));
        let strict_json = serde_config
            .get(STRICT_JSON_KEY)
            .is_some_and(|v| v.trim().eq_ignore_ascii_case("true"));
        let unsupported: Vec<&String> = overrides
            .keys()
            .filter(|key| key.as_str() != STRICT_JSON_KEY)
            .collect();
        if !unsupported.is_empty() {
            log::warn!(
                "{key_prefix}.serde.* keys {unsupported:?} have no analog in this client and are \
                 ignored - Confluent serde settings such as CSFLE (client-side field level \
                 encryption) are not supported by this port; a schema carrying an ENCRYPT rule is \
                 refused, never served in plaintext"
            );
        }
        let auth_label = settings.auth_label();
        let client = RegistryClient::new(settings, cache, version_cache, strict_json);
        log::info!(
            "Schema codec ready (registry={url}, cache={key_prefix}, ttlMs={ttl_ms}, types=[JSON, \
             AVRO], strictJson={strict_json}, csfle=unsupported, auth={auth_label})"
        );
        Ok(Some(Arc::new(SchemaCodec {
            client,
            registry_url: url.to_string(),
            key_prefix: key_prefix.to_string(),
        })))
    }

    /// Whether the bytes are Confluent-framed (magic byte + 4-byte id + payload).
    pub fn is_framed(data: &[u8]) -> bool {
        data.len() >= FRAME_HEADER_LEN && data[0] == MAGIC_BYTE
    }

    /// The global schema id embedded in a Confluent frame (see [`Self::is_framed`]).
    pub fn schema_id(data: &[u8]) -> i32 {
        i32::from_be_bytes([data[1], data[2], data[3], data[4]])
    }

    /// Frame a serialized payload with its global schema id.
    pub fn frame(schema_id: i32, payload: &[u8]) -> Vec<u8> {
        let mut framed = Vec::with_capacity(FRAME_HEADER_LEN + payload.len());
        framed.push(MAGIC_BYTE);
        framed.extend_from_slice(&schema_id.to_be_bytes());
        framed.extend_from_slice(payload);
        framed
    }

    /// The registry this codec was built for.
    pub fn registry_url(&self) -> &str {
        &self.registry_url
    }

    /// The `ManagedCache` name holding id→schema lookups (and `latest/<subject>`
    /// resolutions); `<prefix>.version` holds the pinned-version resolutions.
    pub fn cache_name(&self) -> &str {
        &self.key_prefix
    }

    /// Whether JSON payloads are validated against their schema
    /// (`json.fail.invalid.schema=true`).
    pub fn strict_json(&self) -> bool {
        self.client.strict_json()
    }

    /// Resolve a `(subject, version)` to a global schema id and type — cached
    /// two-tier (see [`RegistryClient::resolve`]). `version` is `latest`
    /// (or blank) for the newest version, or a positive integer to pin one;
    /// anything else is a 400, an unresolvable subject/version a 500.
    pub async fn resolve(&self, subject: &str, version: &str) -> Result<ResolvedSchema, AppError> {
        self.client.resolve(subject, version).await
    }

    /// The registered schema for a global id (cached, positive results only).
    pub async fn schema_by_id(&self, id: i32) -> Result<Arc<RegisteredSchema>, AppError> {
        self.client.schema_by_id(id).await
    }

    /// Serialize a JSON value into the Confluent wire format for a resolved
    /// schema: the schema is fetched by id (never registered), the value is
    /// converted with the codec for the schema's registered type and framed
    /// with the id. The resolved type must match the registered one.
    pub async fn encode(
        &self,
        resolved: ResolvedSchema,
        value: &serde_json::Value,
    ) -> Result<Vec<u8>, AppError> {
        let schema = self.client.schema_by_id(resolved.id).await?;
        if schema.schema_type != resolved.schema_type {
            return Err(AppError::new(
                500,
                format!(
                    "schema id {} is {}, not {}",
                    resolved.id, schema.schema_type, resolved.schema_type
                ),
            ));
        }
        let payload = match schema.parsed() {
            Parsed::Json(json) => json.encode(value),
            Parsed::Avro(avro) => avro.encode(value),
            Parsed::Protobuf => Err(format!(
                "schema-type {} is not supported",
                SchemaType::Protobuf
            )),
        }
        .map_err(|m| {
            AppError::new(
                if matches!(schema.parsed(), Parsed::Protobuf) {
                    501
                } else {
                    400
                },
                format!(
                    "Unable to serialize with schema id {} ({}) - {m}",
                    resolved.id, schema.schema_type
                ),
            )
        })?;
        Ok(Self::frame(resolved.id, &payload))
    }

    /// Decode Confluent-framed bytes by their embedded id: look up the
    /// registered schema (and its type), dispatch to the matching decoder and
    /// return the value as a dynamic body — a map for a record/object, with
    /// Avro bytes kept binary. A payload that is not framed (including a
    /// tombstone) is a 400; an unresolvable id a 500; a Protobuf schema a 501.
    pub async fn decode(&self, topic: &str, data: Option<&[u8]>) -> Result<rmpv::Value, AppError> {
        let Some(data) = data.filter(|d| Self::is_framed(d)) else {
            return Err(AppError::new(
                400,
                format!("payload on '{topic}' is not Confluent schema-framed (missing magic byte)"),
            ));
        };
        let id = Self::schema_id(data);
        let schema = self.client.schema_by_id(id).await?;
        let payload = &data[FRAME_HEADER_LEN..];
        match schema.parsed() {
            Parsed::Json(json) => json.decode(payload),
            Parsed::Avro(avro) => avro.decode(payload),
            Parsed::Protobuf => {
                return Err(AppError::new(
                    501,
                    format!(
                        "schema id {id} on '{topic}' is {}, which is not supported",
                        SchemaType::Protobuf
                    ),
                ))
            }
        }
        .map_err(|m| {
            AppError::new(
                400,
                format!(
                    "Unable to decode '{topic}' payload with schema id {id} ({}) - {m}",
                    schema.schema_type
                ),
            )
        })
    }
}

/// A JSON view of a decoded dynamic body for the routing rules and tests:
/// maps, lists and scalars convert directly; binary (Avro `bytes`, `fixed`,
/// decimals) becomes a list of byte values, which has no other JSON form.
pub fn json_view(value: &rmpv::Value) -> serde_json::Value {
    match value {
        rmpv::Value::Nil => serde_json::Value::Null,
        rmpv::Value::Boolean(b) => serde_json::Value::Bool(*b),
        rmpv::Value::Integer(i) => i
            .as_i64()
            .map(serde_json::Value::from)
            .or_else(|| i.as_u64().map(serde_json::Value::from))
            .unwrap_or(serde_json::Value::Null),
        rmpv::Value::F32(f) => serde_json::Value::from(*f),
        rmpv::Value::F64(f) => serde_json::Value::from(*f),
        rmpv::Value::String(s) => serde_json::Value::from(s.as_str().unwrap_or_default()),
        rmpv::Value::Binary(bytes) => {
            serde_json::Value::Array(bytes.iter().map(|b| serde_json::Value::from(*b)).collect())
        }
        rmpv::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(json_view).collect())
        }
        rmpv::Value::Map(entries) => serde_json::Value::Object(
            entries
                .iter()
                .map(|(k, v)| {
                    let key = match k {
                        rmpv::Value::String(s) => s.as_str().unwrap_or_default().to_string(),
                        other => other.to_string(),
                    };
                    (key, json_view(v))
                })
                .collect(),
        ),
        rmpv::Value::Ext(_, bytes) => {
            serde_json::Value::Array(bytes.iter().map(|b| serde_json::Value::from(*b)).collect())
        }
    }
}

/// The `<prefix>.serde.*` pass-through properties, prefix stripped (Java
/// `SchemaCodec.extractSerdeConfig`).
fn serde_overrides(config: &AppConfigReader, serde_prefix: &str) -> BTreeMap<String, String> {
    config
        .get_composite_key_values()
        .iter()
        .filter_map(|(key, value)| {
            key.strip_prefix(serde_prefix)
                .map(|stripped| (stripped.to_string(), value.to_display_string()))
        })
        .collect()
}

/// A duration setting (`30m`, `10d`, `5000` ms) in milliseconds.
fn duration_ms(config: &AppConfigReader, key: &str, default: &str) -> Result<u64, AppError> {
    let text = config.get_property_or(key, default);
    crate::adapter::parse_duration_ms(&text).ok_or_else(|| {
        AppError::new(
            500,
            format!("'{key}' must be a positive duration (e.g. '30m'), got '{text}'"),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_type_parses_the_registry_spelling() {
        assert_eq!(SchemaType::Avro, SchemaType::from_registry(None).unwrap());
        assert_eq!(
            SchemaType::Avro,
            SchemaType::from_registry(Some("  ")).unwrap()
        );
        assert_eq!(
            SchemaType::Json,
            SchemaType::from_registry(Some("json")).unwrap()
        );
        assert_eq!(SchemaType::Protobuf, SchemaType::parse("Protobuf").unwrap());
        let unknown = SchemaType::parse("thrift").expect_err("rejected");
        assert!(unknown.message().contains("Unknown schema type 'THRIFT'"));
    }

    #[test]
    fn framing_round_trips_and_detects_unframed_bytes() {
        let framed = SchemaCodec::frame(258, b"{\"a\":1}");
        assert!(SchemaCodec::is_framed(&framed));
        assert_eq!(258, SchemaCodec::schema_id(&framed));
        assert_eq!(b"{\"a\":1}", &framed[FRAME_HEADER_LEN..]);
        assert!(!SchemaCodec::is_framed(b"{\"a\":1}"), "no magic byte");
        assert!(!SchemaCodec::is_framed(&[0, 0, 0]), "too short for an id");
        assert!(!SchemaCodec::is_framed(&[]));
    }
}
