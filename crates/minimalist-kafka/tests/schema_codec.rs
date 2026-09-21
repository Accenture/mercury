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

//! The Schema Registry codec against the in-process registry double (the Java
//! `SchemaCodecTest` + `SchemaRegistryOAuthTest` twins): the Confluent frame
//! round trip for JSON Schema and Avro, subject/version resolution and its
//! two-tier cache, positive-results-only caching, the strict JSON flag, the
//! refusals (Protobuf, references, rule sets, an unframed payload), and the
//! OAuth 2.0 client-credentials flow with its token cache. One runtime, one
//! lifecycle — the scenarios run sequentially in one test, because the
//! platform's HTTP client service belongs to the runtime that registers it.

#[path = "support/embedded_registry.rs"]
mod embedded_registry;

use std::sync::{Arc, Once};

use embedded_registry::EmbeddedRegistry;
use minimalist_kafka::schema::{json_view, REGISTRY_URL};
use minimalist_kafka::{ResolvedSchema, SchemaCodec, SchemaType};
use platform_core::automation::http_client::AsyncHttpClientService;
use platform_core::automation::ASYNC_HTTP_REQUEST;
use platform_core::platform::FunctionOptions;
use platform_core::{overrides, resources, AppConfigReader, ManagedCache, Platform};

const JSON_SCHEMA: &str =
    r#"{"type":"object","properties":{"hello":{"type":"string"}},"additionalProperties":true}"#;
const STRICT_SCHEMA: &str = r#"{"type":"object","properties":{"hello":{"type":"string"}},"required":["hello"],"additionalProperties":false}"#;
const AVRO_SCHEMA: &str =
    r#"{"type":"record","name":"Greeting","fields":[{"name":"hello","type":"string"}]}"#;

fn config() -> &'static AppConfigReader {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
    });
    AppConfigReader::get_instance()
}

/// The process platform with the `async.http.request` service the codec
/// calls the registry through (the app starter registers it in a real
/// application).
fn http_platform() -> Platform {
    let platform = Platform::get_instance();
    if !platform.has_route(ASYNC_HTTP_REQUEST) {
        platform
            .register_with_options(
                ASYNC_HTTP_REQUEST,
                Arc::new(AsyncHttpClientService::new(&platform)),
                10,
                FunctionOptions {
                    zero_traced: false,
                    interceptor: true,
                    private: true,
                },
            )
            .expect("register http client");
    }
    platform
}

fn cached(cache: &str, key: &str) -> bool {
    ManagedCache::get_instance(cache).is_some_and(|c| c.exists(key))
}

fn frame(id: i32, payload: &[u8]) -> Vec<u8> {
    SchemaCodec::frame(id, payload)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn schema_codec_end_to_end() {
    let config = config();
    let _platform = http_platform();
    let registry = EmbeddedRegistry::start().await;
    let codec = SchemaCodec::for_registry(config, Some(registry.base_url()), "schema.registry")
        .expect("codec builds")
        .expect("registry url given");
    assert_eq!(registry.base_url(), codec.registry_url());
    assert!(
        !codec.strict_json(),
        "validation is off by default (Confluent parity)"
    );

    // --- 1. serialize by id, then decode: the frame round-trips and the id is cached
    let id = registry.register("orders-value", "JSON", JSON_SCHEMA);
    let resolved = ResolvedSchema {
        id,
        schema_type: SchemaType::Json,
    };
    let framed = codec
        .encode(resolved, &serde_json::json!({"hello": "world"}))
        .await
        .expect("framed");
    assert!(SchemaCodec::is_framed(&framed), "magic byte + id");
    assert_eq!(id, SchemaCodec::schema_id(&framed));
    assert_eq!(b"{\"hello\":\"world\"}", &framed[5..]);
    let decoded = json_view(
        &codec
            .decode("orders", Some(&framed))
            .await
            .expect("decoded"),
    );
    assert_eq!("world", decoded["hello"]);
    assert!(
        cached("schema.registry", &id.to_string()),
        "schema cached by id"
    );
    assert_eq!(
        1,
        registry.lookups(),
        "one registry round trip: the decode was served from the cache"
    );

    // --- 2. json.fail.invalid.schema=true rejects a non-conforming payload both ways
    let strict_id = registry.register("strict-orders-value", "JSON", STRICT_SCHEMA);
    let strict = SchemaCodec::for_registry(config, Some(registry.base_url()), "schema.strict")
        .expect("codec builds")
        .expect("registry url given");
    assert!(
        strict.strict_json(),
        "schema.strict.serde.json.fail.invalid.schema=true"
    );
    let strict_resolved = ResolvedSchema {
        id: strict_id,
        schema_type: SchemaType::Json,
    };
    let rejected = strict
        .encode(strict_resolved, &serde_json::json!({"wrong": "shape"}))
        .await
        .expect_err("an invalid payload must not serialize when validation is on");
    assert_eq!(400, rejected.status());
    assert!(
        rejected
            .message()
            .contains("does not conform to its schema"),
        "{}",
        rejected.message()
    );
    // ...while a conforming payload passes - the rejection above is validation, not setup
    let conforming = strict
        .encode(strict_resolved, &serde_json::json!({"hello": "world"}))
        .await
        .expect("framed");
    assert!(strict.decode("orders", Some(&conforming)).await.is_ok());
    // deserializer side: hand-framed bytes carrying a non-conforming document under the strict id
    let invalid = frame(strict_id, b"{\"wrong\":\"shape\"}");
    let rejected = strict
        .decode("orders", Some(&invalid))
        .await
        .expect_err("an invalid payload must not decode when validation is on");
    assert!(rejected
        .message()
        .contains("does not conform to its schema"));
    // the default (non-validating) codec accepts the same bytes - the flag is what rejects
    assert!(codec.decode("orders", Some(&invalid)).await.is_ok());

    // --- 3. subject + version resolve to the id; latest and pinned cache in
    // their tiers (a distinct schema text, so the id is not yet cached)
    let subject = "resolve-demo-value";
    let resolve_id = registry.register(
        subject,
        "JSON",
        r#"{"type":"object","properties":{"resolve":{"type":"string"}},"additionalProperties":true}"#,
    );
    let before = registry.lookups();
    let by_latest = codec.resolve(subject, "latest").await.expect("latest");
    assert_eq!(
        resolve_id, by_latest.id,
        "latest resolves to the registered global id"
    );
    assert_eq!(
        SchemaType::Json,
        by_latest.schema_type,
        "type derived authoritatively from the fetched schema"
    );
    assert_eq!(
        before + 2,
        registry.lookups(),
        "the first latest costs two round trips: the version, then the schema (warming the id cache)"
    );
    assert_eq!(
        resolve_id,
        codec.resolve(subject, "").await.unwrap().id,
        "blank = latest"
    );
    assert_eq!(
        resolve_id,
        codec.resolve(subject, " Latest ").await.unwrap().id
    );
    assert_eq!(
        before + 2,
        registry.lookups(),
        "repeats of latest are cache hits"
    );
    assert_eq!(
        resolve_id,
        codec.resolve(subject, "1").await.unwrap().id,
        "pinned version 1"
    );
    assert_eq!(
        before + 3,
        registry.lookups(),
        "the pin costs one round trip (the version); the schema was cached"
    );
    assert_eq!(resolve_id, codec.resolve(subject, "1").await.unwrap().id);
    assert_eq!(
        before + 3,
        registry.lookups(),
        "a repeated pin is a cache hit"
    );
    assert!(
        cached("schema.registry", &format!("latest/{subject}")),
        "latest resolution cached in the id cache under a namespaced key"
    );
    assert!(
        cached("schema.registry.version", &format!("{subject}/1")),
        "numeric version resolution cached in the long-TTL version cache"
    );

    // --- 4. a bad version is a 400 up front; an unknown subject cannot resolve
    let bad = codec.resolve(subject, "v2").await.expect_err("rejected");
    assert_eq!(400, bad.status());
    assert!(bad
        .message()
        .contains("'version' must be 'latest' or a positive integer, got 'v2'"));
    let unknown = codec
        .resolve("no-such-subject", "latest")
        .await
        .expect_err("rejected");
    assert_eq!(404, unknown.status());
    assert!(
        unknown
            .message()
            .contains("Unable to resolve subject 'no-such-subject' version 'latest'"),
        "{}",
        unknown.message()
    );
    assert!(
        unknown.message().contains("40401"),
        "the registry error code is carried"
    );

    // --- 5. Avro by id round-trips, and bytes from a stock Confluent Avro
    // serializer (the binary datum, hand-computed) decode
    let avro_id = registry.register("greeting-avro-value", "AVRO", AVRO_SCHEMA);
    let avro_resolved = ResolvedSchema {
        id: avro_id,
        schema_type: SchemaType::Avro,
    };
    let framed = codec
        .encode(avro_resolved, &serde_json::json!({"hello": "avro"}))
        .await
        .expect("framed");
    assert_eq!(avro_id, SchemaCodec::schema_id(&framed));
    assert_eq!(
        &[0x08, b'a', b'v', b'r', b'o'],
        &framed[5..],
        "the Avro datum: len 4 (zigzag 8) + text"
    );
    let decoded = json_view(
        &codec
            .decode("greetings", Some(&framed))
            .await
            .expect("decoded"),
    );
    assert_eq!(serde_json::json!({"hello": "avro"}), decoded);
    let mut external = vec![0x1A];
    external.extend_from_slice(b"external-avro");
    let decoded = json_view(
        &codec
            .decode("greetings", Some(&frame(avro_id, &external)))
            .await
            .expect("decoded"),
    );
    assert_eq!("external-avro", decoded["hello"]);
    // the resolved type must match the registered one
    let mismatch = codec
        .encode(
            ResolvedSchema {
                id: avro_id,
                schema_type: SchemaType::Json,
            },
            &serde_json::json!({"hello": "x"}),
        )
        .await
        .expect_err("rejected");
    assert!(mismatch
        .message()
        .contains(&format!("schema id {avro_id} is AVRO, not JSON")));

    // --- 6. Protobuf is recognized but not supported - a clear 501, never silence
    let proto_id = registry.register("proto-value", "PROTOBUF", "syntax = \"proto3\";");
    let proto = ResolvedSchema {
        id: proto_id,
        schema_type: SchemaType::Protobuf,
    };
    let refused = codec
        .encode(proto, &serde_json::json!({"hello": "protobuf"}))
        .await
        .expect_err("refused");
    assert_eq!(501, refused.status());
    assert!(refused
        .message()
        .contains("schema-type PROTOBUF is not supported"));
    let refused = codec
        .decode("proto", Some(&frame(proto_id, b"\x0a\x01x")))
        .await
        .expect_err("refused");
    assert_eq!(501, refused.status());

    // --- 7. an unframed payload (and a tombstone) is a 400
    let unframed = codec
        .decode("orders", Some(b"{\"hello\":\"x\"}"))
        .await
        .expect_err("rejected");
    assert_eq!(400, unframed.status());
    assert!(unframed
        .message()
        .contains("payload on 'orders' is not Confluent schema-framed (missing magic byte)"));
    assert_eq!(
        400,
        codec
            .decode("orders", None)
            .await
            .expect_err("rejected")
            .status()
    );

    // --- 8. a not-found id is never cached (positive results only)
    let missing = codec.schema_by_id(987_654).await.expect_err("not found");
    assert_eq!(404, missing.status());
    assert!(missing
        .message()
        .contains("Unable to resolve schema id 987654"));
    assert!(
        !cached("schema.registry", "987654"),
        "a not-found schema id is never cached"
    );

    // --- 9. the codec recovers once the schema appears: the miss was never cached
    let future_id = registry.next_id();
    let probe = ResolvedSchema {
        id: future_id,
        schema_type: SchemaType::Json,
    };
    assert!(codec
        .encode(probe, &serde_json::json!({"hello": "x"}))
        .await
        .is_err());
    assert!(!cached("schema.registry", &future_id.to_string()));
    let assigned = registry.register(
        "recover-value",
        "JSON",
        r#"{"type":"object","properties":{"recover":{"type":"string"}},"additionalProperties":true}"#,
    );
    assert_eq!(
        future_id, assigned,
        "the next registration takes the id we probed"
    );
    let framed = codec
        .encode(probe, &serde_json::json!({"hello": "recovered"}))
        .await
        .expect("the same codec now succeeds");
    assert_eq!(future_id, SchemaCodec::schema_id(&framed));

    // --- 10. no registry url = schema features off
    assert!(
        SchemaCodec::for_registry(config, Some("  "), "schema.registry")
            .expect("ok")
            .is_none()
    );
    assert!(config
        .get_property(REGISTRY_URL)
        .unwrap_or_default()
        .is_empty());
    assert!(SchemaCodec::from_config(config).expect("ok").is_none());

    // --- 11. schema references and rule sets (CSFLE) are refused, never served half-way
    // (distinct schema texts: the registry double is content-addressed, and a
    // text already fetched would be served from the codec's cache)
    let with_references = registry.register_with_extras(
        "referencing-value",
        "JSON",
        r#"{"type":"object","properties":{"item":{"$ref":"common.json"}}}"#,
        serde_json::json!({"references": [{"name": "common.json", "subject": "common", "version": 1}]}),
    );
    let refused = codec
        .schema_by_id(with_references)
        .await
        .expect_err("refused");
    assert_eq!(501, refused.status());
    assert!(refused
        .message()
        .contains("uses schema references [\"common.json\"]"));
    let with_rules = registry.register_with_extras(
        "encrypted-value",
        "AVRO",
        r#"{"type":"record","name":"Person","fields":[{"name":"ssn","type":"string","confluent:tags":["PII"]}]}"#,
        serde_json::json!({"ruleSet": {"domainRules": [
            {"name": "encryptPII", "kind": "TRANSFORM", "type": "ENCRYPT", "mode": "WRITEREAD"}
        ]}}),
    );
    let refused = codec
        .decode(
            "pii",
            Some(&frame(with_rules, &[0x08, b'a', b'v', b'r', b'o'])),
        )
        .await
        .expect_err("refused");
    assert_eq!(501, refused.status());
    assert!(
        refused
            .message()
            .contains("carries a ruleSet (ENCRYPT 'encryptPII')"),
        "{}",
        refused.message()
    );

    // --- 12. OAuth 2.0 client credentials end to end: the token is fetched
    // once, cached, and sent as Authorization: Bearer on every registry call
    let oauth_registry = EmbeddedRegistry::start_with_oauth("test-client", "test-secret").await;
    let template =
        std::env::temp_dir().join(format!("schema-registry-oauth-{}.yml", std::process::id()));
    std::fs::write(
        &template,
        format!(
            "bearer.auth.credentials.source: OAUTHBEARER\n\
             bearer.auth.issuer.endpoint.url: '{}'\n\
             bearer.auth.client.id: test-client\n\
             bearer.auth.client.secret: test-secret\n\
             bearer.auth.scope: registry\n\
             bearer.auth.logical.cluster: lsrc-test\n\
             schema.registry.ssl.truststore.location: /nowhere/truststore.jks\n",
            oauth_registry.token_url()
        ),
    )
    .expect("write template");
    overrides::set(
        "schema.oauth.properties",
        &format!("file:{}", template.display()),
    );
    let oauth_codec =
        SchemaCodec::for_registry(config, Some(oauth_registry.base_url()), "schema.oauth")
            .expect("codec builds")
            .expect("registry url given");
    let oauth_id = oauth_registry.register("oauth-hello-value", "JSON", JSON_SCHEMA);
    let resolved = oauth_codec
        .resolve("oauth-hello-value", "latest")
        .await
        .expect("authenticated resolve");
    assert_eq!(oauth_id, resolved.id);
    assert_eq!(SchemaType::Json, resolved.schema_type);
    let framed = oauth_codec
        .encode(resolved, &serde_json::json!({"hello": "bearer"}))
        .await
        .expect("framed");
    assert_eq!(
        "bearer",
        json_view(
            &oauth_codec
                .decode("oauth", Some(&framed))
                .await
                .expect("decoded")
        )["hello"]
    );
    assert_eq!(
        1,
        oauth_registry.token_requests(),
        "one token fetch served every registry call - the bearer token is cached"
    );
    let _ = std::fs::remove_file(&template);
    overrides::clear("schema.oauth.properties");

    // --- 13. an unauthenticated client is rejected by the OAuth registry
    let anonymous =
        SchemaCodec::for_registry(config, Some(oauth_registry.base_url()), "schema.anonymous")
            .expect("codec builds")
            .expect("registry url given");
    let denied = anonymous
        .resolve("oauth-hello-value", "latest")
        .await
        .expect_err("denied");
    assert_eq!(401, denied.status());
    assert!(
        denied.message().contains("Bearer token required"),
        "{}",
        denied.message()
    );
}
