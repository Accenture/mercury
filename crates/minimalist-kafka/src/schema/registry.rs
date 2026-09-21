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

//! The Schema Registry REST client (Java `ManagedCacheSchemaRegistryClient`):
//! the two lookups the codec needs — a schema by global id, and a
//! `(subject, version)` resolved to an id — over the platform's own
//! `async.http.request` client, fronted by platform-core's `ManagedCache`.
//!
//! **Positive results only.** A global schema id is immutable (ids are
//! content-addressed), so a resolved schema is cached by id and re-served on
//! later lookups; a not-found id is never cached (the lookup fails before the
//! cache write), so a schema registered while the application is running
//! becomes visible on the very next lookup — no stale "not found" lingers
//! until a TTL elapses or the pod restarts. A pinned numeric version is
//! immutable too and lives in the long-TTL version cache; a subject's
//! `latest` is mutable (a new registration moves it) and shares the short-TTL
//! id cache under a `latest/`-namespaced key that cannot collide with the
//! digit-only id keys.
//!
//! The parsed schema is cached with the text: parsing an Avro schema or
//! compiling a JSON Schema validator once per id is what makes per-record
//! encode/decode cheap.

use std::sync::Arc;
use std::time::Duration;

use platform_core::automation::{AsyncHttpRequest, ASYNC_HTTP_REQUEST};
use platform_core::{AppError, EventEnvelope, ManagedCache, Platform, PostOffice};

use super::auth::{RegistrySettings, TokenCache};
use super::avro::AvroSchema;
use super::json::JsonSchema;
use super::{ResolvedSchema, SchemaType, LATEST};

/// The registry's own request/response media type; `application/json` is
/// accepted alongside for mocks that answer plain JSON.
const REGISTRY_MEDIA_TYPE: &str = "application/vnd.schemaregistry.v1+json";
/// Per-call HTTP deadline for a registry lookup.
const REGISTRY_CALL_TIMEOUT_SECONDS: u64 = 10;

/// A schema fetched from the registry, parsed once for its type (Java
/// `ParsedSchema`).
pub struct RegisteredSchema {
    /// The global schema id.
    pub id: i32,
    /// The registered type.
    pub schema_type: SchemaType,
    /// The schema text exactly as registered.
    pub text: String,
    parsed: Parsed,
}

/// The type-specific parsed form.
pub(crate) enum Parsed {
    Json(JsonSchema),
    Avro(AvroSchema),
    Protobuf,
}

impl RegisteredSchema {
    pub(crate) fn parsed(&self) -> &Parsed {
        &self.parsed
    }
}

impl std::fmt::Debug for RegisteredSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisteredSchema")
            .field("id", &self.id)
            .field("schema_type", &self.schema_type)
            .field("text", &self.text)
            .finish()
    }
}

/// The client: settings + the two caches + the bearer-token cache.
pub struct RegistryClient {
    settings: RegistrySettings,
    cache: Arc<ManagedCache>,
    version_cache: Arc<ManagedCache>,
    strict_json: bool,
    token: TokenCache,
}

impl RegistryClient {
    pub(crate) fn new(
        settings: RegistrySettings,
        cache: Arc<ManagedCache>,
        version_cache: Arc<ManagedCache>,
        strict_json: bool,
    ) -> Self {
        RegistryClient {
            settings,
            cache,
            version_cache,
            strict_json,
            token: TokenCache::default(),
        }
    }

    pub(crate) fn strict_json(&self) -> bool {
        self.strict_json
    }

    /// The schema for a global id: the cached parsed schema when present,
    /// otherwise `GET /schemas/ids/{id}`, parsed and cached. A not-found id
    /// fails here, before any cache write.
    pub async fn schema_by_id(&self, id: i32) -> Result<Arc<RegisteredSchema>, AppError> {
        let key = id.to_string();
        if let Some(cached) = self.cache.get_as::<RegisteredSchema>(&key) {
            return Ok(cached);
        }
        let entity = self.get(&format!("/schemas/ids/{id}")).await.map_err(|e| {
            AppError::new(
                e.status(),
                format!("Unable to resolve schema id {id}: {}", e.message()),
            )
        })?;
        let schema = self.parse_entity(id, &entity)?;
        self.cache.put(&key, schema);
        self.cache
            .get_as::<RegisteredSchema>(&key)
            .ok_or_else(|| AppError::new(500, format!("schema id {id} vanished from the cache")))
    }

    /// Resolve a `(subject, version)` to a global id and its type. The type
    /// is taken from the fetched schema, so it is authoritative — never
    /// guessed from a possibly absent `schemaType` — and that fetch also
    /// warms the id cache, so the subsequent encode needs no extra round
    /// trip. `version` is `latest` (or blank) or a positive integer.
    pub async fn resolve(&self, subject: &str, version: &str) -> Result<ResolvedSchema, AppError> {
        let version = version.trim();
        let latest = version.is_empty() || version.eq_ignore_ascii_case(LATEST);
        let pinned = if latest { 0 } else { parse_version(version)? };
        let (key, name_cache) = if latest {
            (format!("{LATEST}/{subject}"), &self.cache)
        } else {
            (format!("{subject}/{pinned}"), &self.version_cache)
        };
        if let Some(cached) = name_cache.get_as::<ResolvedSchema>(&key) {
            return Ok(*cached);
        }
        let selector = if latest {
            LATEST.to_string()
        } else {
            pinned.to_string()
        };
        let path = format!(
            "/subjects/{}/versions/{selector}",
            url_encode_segment(subject)
        );
        let entity = self.get(&path).await.map_err(|e| {
            AppError::new(
                e.status(),
                format!(
                    "Unable to resolve subject '{subject}' version '{version}': {}",
                    e.message()
                ),
            )
        })?;
        let id = entity
            .get("id")
            .and_then(serde_json::Value::as_i64)
            .and_then(|id| i32::try_from(id).ok())
            .ok_or_else(|| {
                AppError::new(
                    500,
                    format!(
                        "Unable to resolve subject '{subject}' version '{version}': the registry \
                         answered without an id"
                    ),
                )
            })?;
        let schema = self.schema_by_id(id).await?;
        let resolved = ResolvedSchema {
            id,
            schema_type: schema.schema_type,
        };
        name_cache.put(&key, resolved);
        Ok(resolved)
    }

    /// Parse a registry schema entity (`schema`, optional `schemaType`,
    /// `references`, `ruleSet`) into the type-specific form, refusing what
    /// this port does not implement rather than serving it half-way.
    fn parse_entity(
        &self,
        id: i32,
        entity: &serde_json::Value,
    ) -> Result<RegisteredSchema, AppError> {
        let text = entity
            .get("schema")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                AppError::new(
                    500,
                    format!("schema id {id}: the registry answered without a 'schema' text"),
                )
            })?;
        let schema_type = SchemaType::from_registry(
            entity.get("schemaType").and_then(serde_json::Value::as_str),
        )?;
        if let Some(references) = entity
            .get("references")
            .and_then(serde_json::Value::as_array)
        {
            if !references.is_empty() {
                let names: Vec<&str> = references
                    .iter()
                    .filter_map(|r| r.get("name").and_then(serde_json::Value::as_str))
                    .collect();
                return Err(AppError::new(
                    501,
                    format!(
                        "schema id {id} uses schema references {names:?}, which this port does not \
                         support - register a self-contained schema"
                    ),
                ));
            }
        }
        if let Some(rules) = rule_types(entity) {
            return Err(AppError::new(
                501,
                format!(
                    "schema id {id} carries a ruleSet ({rules}) - Confluent data-contract rules, \
                     including CSFLE field encryption, are not supported by this port; the payload \
                     is refused rather than served in plaintext"
                ),
            ));
        }
        let parsed = match schema_type {
            SchemaType::Json => Parsed::Json(
                JsonSchema::parse(text, self.strict_json)
                    .map_err(|m| AppError::new(500, format!("schema id {id} (JSON) - {m}")))?,
            ),
            SchemaType::Avro => Parsed::Avro(
                AvroSchema::parse(text)
                    .map_err(|m| AppError::new(500, format!("schema id {id} (AVRO) - {m}")))?,
            ),
            SchemaType::Protobuf => Parsed::Protobuf,
        };
        Ok(RegisteredSchema {
            id,
            schema_type,
            text: text.to_string(),
            parsed,
        })
    }

    /// `GET <registry>/<path>` with the registry media type, the configured
    /// authentication and the optional installation headers. A non-2xx answer
    /// is an error carrying the registry's `message` (and `error_code`).
    async fn get(&self, path: &str) -> Result<serde_json::Value, AppError> {
        let platform = Platform::get_instance();
        let mut request = AsyncHttpRequest::new()
            .set_method("GET")
            .set_target_host(&self.settings.registry.host)
            .set_url(&self.settings.registry.join(path))
            .set_header(
                "accept",
                &format!("{REGISTRY_MEDIA_TYPE}, application/json"),
            )
            .set_timeout_seconds(REGISTRY_CALL_TIMEOUT_SECONDS);
        for (name, value) in &self.settings.extra_headers {
            request = request.set_header(name, value);
        }
        if let Some(authorization) = self.settings.auth.header(&self.token).await? {
            request = request.set_header("authorization", &authorization);
        }
        let (status, body) = call(&platform, request).await?;
        if !(200..300).contains(&status) {
            let (code, message) = registry_error(&body);
            return Err(AppError::new(
                status,
                match code {
                    Some(code) => format!("{message}; error code: {code}"),
                    None => message,
                },
            ));
        }
        Ok(body)
    }
}

/// One HTTP round trip through the platform's `async.http.request` service:
/// the HTTP status and the body as JSON (`Null` when empty or not JSON).
pub(crate) async fn call(
    platform: &Platform,
    request: AsyncHttpRequest,
) -> Result<(i32, serde_json::Value), AppError> {
    let timeout = Duration::from_secs(request.timeout_seconds() + 2);
    let po = PostOffice::new(platform);
    let response = po
        .request(
            EventEnvelope::new()
                .set_to(ASYNC_HTTP_REQUEST)
                .set_raw_body(request.to_value()),
            timeout,
        )
        .await?;
    Ok((response.status(), body_json(response.body())))
}

/// The response body as JSON whatever content type the server declared: the
/// platform client already decoded `application/json` into a map; the
/// registry's own `application/vnd.schemaregistry.v1+json` arrives as bytes.
fn body_json(body: &rmpv::Value) -> serde_json::Value {
    match body {
        rmpv::Value::Map(_) | rmpv::Value::Array(_) => {
            rmpv::ext::from_value(body.clone()).unwrap_or(serde_json::Value::Null)
        }
        rmpv::Value::Binary(bytes) => serde_json::from_slice(bytes).unwrap_or_else(|_| {
            serde_json::Value::String(String::from_utf8_lossy(bytes).to_string())
        }),
        rmpv::Value::String(text) => {
            let text = text.as_str().unwrap_or_default();
            serde_json::from_str(text)
                .unwrap_or_else(|_| serde_json::Value::String(text.to_string()))
        }
        _ => serde_json::Value::Null,
    }
}

/// The Confluent error shape `{"error_code": n, "message": text}`, or the
/// body's text when it is not that shape.
fn registry_error(body: &serde_json::Value) -> (Option<i64>, String) {
    let code = body.get("error_code").and_then(serde_json::Value::as_i64);
    let message = body
        .get("message")
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| match body {
            serde_json::Value::Null => "no response body".to_string(),
            serde_json::Value::String(text) => text.clone(),
            other => other.to_string(),
        });
    (code, message)
}

/// The rule types a registered schema carries (`ruleSet.domainRules` and
/// `ruleSet.migrationRules`), rendered for the refusal message; `None` when
/// the schema has no rules.
fn rule_types(entity: &serde_json::Value) -> Option<String> {
    let rule_set = entity.get("ruleSet")?;
    let mut types: Vec<String> = Vec::new();
    for section in ["domainRules", "migrationRules"] {
        if let Some(rules) = rule_set.get(section).and_then(serde_json::Value::as_array) {
            for rule in rules {
                let kind = rule
                    .get("type")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("UNKNOWN");
                let name = rule
                    .get("name")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("unnamed");
                types.push(format!("{kind} '{name}'"));
            }
        }
    }
    if types.is_empty() {
        None
    } else {
        Some(types.join(", "))
    }
}

/// `version` as a positive integer (Java `parseVersion`).
fn parse_version(version: &str) -> Result<u32, AppError> {
    let parsed: u32 = version.parse().map_err(|_| {
        AppError::new(
            400,
            format!("'version' must be '{LATEST}' or a positive integer, got '{version}'"),
        )
    })?;
    if parsed < 1 {
        return Err(AppError::new(
            400,
            format!("'version' must be >= 1, got '{version}'"),
        ));
    }
    Ok(parsed)
}

/// Percent-encode one URL path segment (a subject may carry characters such
/// as `:` or a space).
fn url_encode_segment(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());
    for byte in segment.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_grammar_is_latest_or_a_positive_integer() {
        assert_eq!(3, parse_version("3").unwrap());
        assert!(parse_version("0")
            .expect_err("rejected")
            .message()
            .contains("must be >= 1"));
        let bad = parse_version("v2").expect_err("rejected");
        assert_eq!(400, bad.status());
        assert!(bad
            .message()
            .contains("'version' must be 'latest' or a positive integer, got 'v2'"));
    }

    #[test]
    fn registry_error_shape_is_read_and_other_bodies_pass_as_text() {
        let confluent = serde_json::json!({"error_code": 40403, "message": "Schema 9 not found"});
        assert_eq!(
            (Some(40403), "Schema 9 not found".to_string()),
            registry_error(&confluent)
        );
        assert_eq!(
            (None, "no response body".to_string()),
            registry_error(&serde_json::Value::Null)
        );
        assert_eq!(
            (None, "gateway timeout".to_string()),
            registry_error(&serde_json::Value::String("gateway timeout".into()))
        );
    }

    #[test]
    fn rule_sets_are_named_for_the_refusal() {
        let entity = serde_json::json!({
            "schema": "{}",
            "ruleSet": {"domainRules": [
                {"name": "encryptPII", "kind": "TRANSFORM", "type": "ENCRYPT"},
                {"name": "checkSsn", "kind": "CONDITION", "type": "CEL"}
            ]}
        });
        assert_eq!(
            Some("ENCRYPT 'encryptPII', CEL 'checkSsn'".to_string()),
            rule_types(&entity)
        );
        assert_eq!(None, rule_types(&serde_json::json!({"schema": "{}"})));
        assert_eq!(
            None,
            rule_types(&serde_json::json!({"schema": "{}", "ruleSet": {"domainRules": []}}))
        );
    }

    #[test]
    fn body_json_reads_bytes_text_and_maps() {
        let bytes = rmpv::Value::Binary(b"{\"id\": 7}".to_vec());
        assert_eq!(serde_json::json!({"id": 7}), body_json(&bytes));
        let text = rmpv::Value::from("{\"id\": 8}");
        assert_eq!(serde_json::json!({"id": 8}), body_json(&text));
        let map = rmpv::Value::Map(vec![(rmpv::Value::from("id"), rmpv::Value::from(9))]);
        assert_eq!(serde_json::json!({"id": 9}), body_json(&map));
        assert_eq!(serde_json::Value::Null, body_json(&rmpv::Value::Nil));
        assert_eq!(
            serde_json::Value::String("not json".into()),
            body_json(&rmpv::Value::Binary(b"not json".to_vec()))
        );
    }

    #[test]
    fn subjects_are_percent_encoded_per_segment() {
        assert_eq!("orders-value", url_encode_segment("orders-value"));
        assert_eq!(
            "com.acme%3AOrder%20v2",
            url_encode_segment("com.acme:Order v2")
        );
    }
}
