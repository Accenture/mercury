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

//! A self-contained, in-process Confluent-compatible Schema Registry for the
//! test suites (the Java `EmbeddedSchemaRegistry` twin): a tiny hyper server on
//! a random loopback port implementing the endpoints the codec calls —
//!
//! - `POST /subjects/{subject}/versions` — register, answers `{"id": N}`
//!   (content-addressed global ids, deduplicated; a per-subject version)
//! - `GET /subjects/{subject}/versions/{version}` — the `latest` or a numeric
//!   version resolved to its global id
//! - `GET /schemas/ids/{id}` — `{"schema": ..., "schemaType": ...}` (the type
//!   omitted for Avro, as Confluent does), plus `references` / `ruleSet` when
//!   the fixture registered them
//!
//! Any other path answers 404 with the Confluent error shape, so a missing
//! endpoint surfaces loudly. Responses carry the registry's own media type
//! (`application/vnd.schemaregistry.v1+json`), which exercises the codec's
//! bytes-to-JSON path rather than the platform client's JSON auto-decode.
//!
//! **Optional OAuth 2.0 mode**: the registry then enforces `Authorization:
//! Bearer` with a token it issued, and serves a client-credentials token
//! endpoint at `POST /oauth/token` (client id/secret as HTTP Basic, or as form
//! fields) — the full `bearer.auth.credentials.source=OAUTHBEARER` flow
//! without an external identity provider. Counters expose how many tokens were
//! issued (a cache assertion) and how many registry lookups were served.

#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use base64::Engine;
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

const MEDIA_TYPE: &str = "application/vnd.schemaregistry.v1+json";
const TOKEN_PATH: &str = "/oauth/token";
const TOKEN_LIFETIME_SECONDS: u64 = 3600;

/// One registered schema.
struct Entry {
    schema_type: String,
    text: String,
    /// Optional fixture extras merged into the `GET /schemas/ids/{id}` entity
    /// (`references`, `ruleSet`) for the refusal tests.
    extras: serde_json::Map<String, serde_json::Value>,
}

#[derive(Default)]
struct Store {
    next_id: i32,
    content_to_id: HashMap<String, i32>,
    schemas: BTreeMap<i32, Entry>,
    /// subject -> (version -> global id)
    subjects: BTreeMap<String, BTreeMap<u32, i32>>,
}

struct State {
    oauth: Option<(String, String)>,
    store: Mutex<Store>,
    issued_tokens: Mutex<HashSet<String>>,
    token_requests: AtomicUsize,
    lookups: AtomicUsize,
}

/// A running embedded registry; dropping it leaves the server task to end
/// with the runtime (the port is random, so suites never collide).
pub struct EmbeddedRegistry {
    base_url: String,
    state: Arc<State>,
}

impl EmbeddedRegistry {
    /// Start without authentication.
    pub async fn start() -> EmbeddedRegistry {
        Self::start_with(None).await
    }

    /// Start in OAuth 2.0 mode: registry endpoints require a bearer token
    /// issued by this instance's `POST /oauth/token` client-credentials
    /// endpoint for exactly this client id and secret.
    pub async fn start_with_oauth(client_id: &str, client_secret: &str) -> EmbeddedRegistry {
        Self::start_with(Some((client_id.to_string(), client_secret.to_string()))).await
    }

    async fn start_with(oauth: Option<(String, String)>) -> EmbeddedRegistry {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind embedded registry");
        let addr = listener.local_addr().expect("local addr");
        let state = Arc::new(State {
            oauth,
            store: Mutex::new(Store {
                next_id: 1,
                ..Store::default()
            }),
            issued_tokens: Mutex::new(HashSet::new()),
            token_requests: AtomicUsize::new(0),
            lookups: AtomicUsize::new(0),
        });
        let served = state.clone();
        tokio::spawn(async move {
            loop {
                let Ok((stream, _)) = listener.accept().await else {
                    break;
                };
                let state = served.clone();
                tokio::spawn(async move {
                    let service = service_fn(move |request| handle(state.clone(), request));
                    let _ = http1::Builder::new()
                        .serve_connection(TokioIo::new(stream), service)
                        .await;
                });
            }
        });
        EmbeddedRegistry {
            base_url: format!("http://127.0.0.1:{}", addr.port()),
            state,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// The client-credentials token endpoint (OAuth mode).
    pub fn token_url(&self) -> String {
        format!("{}{TOKEN_PATH}", self.base_url)
    }

    /// How many tokens the token endpoint issued.
    pub fn token_requests(&self) -> usize {
        self.state.token_requests.load(Ordering::Acquire)
    }

    /// How many registry GET lookups were served (a cache-hit assertion).
    pub fn lookups(&self) -> usize {
        self.state.lookups.load(Ordering::Acquire)
    }

    /// The global id the next distinct registration will take.
    pub fn next_id(&self) -> i32 {
        self.state.store.lock().expect("store").next_id
    }

    /// Register a schema in-process (no HTTP): the global id, content-addressed.
    pub fn register(&self, subject: &str, schema_type: &str, text: &str) -> i32 {
        self.state
            .register(subject, schema_type, text, serde_json::Map::new())
    }

    /// Register a schema whose `GET /schemas/ids/{id}` entity carries fixture
    /// extras (`references`, `ruleSet`).
    pub fn register_with_extras(
        &self,
        subject: &str,
        schema_type: &str,
        text: &str,
        extras: serde_json::Value,
    ) -> i32 {
        let extras = extras
            .as_object()
            .cloned()
            .expect("extras must be a JSON object");
        self.state.register(subject, schema_type, text, extras)
    }
}

impl State {
    fn register(
        &self,
        subject: &str,
        schema_type: &str,
        text: &str,
        extras: serde_json::Map<String, serde_json::Value>,
    ) -> i32 {
        let mut store = self.store.lock().expect("store");
        let schema_type = schema_type.to_ascii_uppercase();
        let content_key = format!("{schema_type}:{text}");
        let id = match store.content_to_id.get(&content_key) {
            Some(id) => *id,
            None => {
                let id = store.next_id;
                store.next_id += 1;
                store.content_to_id.insert(content_key, id);
                store.schemas.insert(
                    id,
                    Entry {
                        schema_type,
                        text: text.to_string(),
                        extras,
                    },
                );
                id
            }
        };
        let versions = store.subjects.entry(subject.to_string()).or_default();
        if !versions.values().any(|existing| *existing == id) {
            let next = versions.keys().max().map_or(1, |v| v + 1);
            versions.insert(next, id);
        }
        id
    }

    fn entity_by_id(&self, id: i32) -> Option<serde_json::Value> {
        let store = self.store.lock().expect("store");
        let entry = store.schemas.get(&id)?;
        let mut entity = serde_json::Map::new();
        entity.insert(
            "schema".into(),
            serde_json::Value::String(entry.text.clone()),
        );
        if entry.schema_type != "AVRO" {
            entity.insert(
                "schemaType".into(),
                serde_json::Value::String(entry.schema_type.clone()),
            );
        }
        for (key, value) in &entry.extras {
            entity.insert(key.clone(), value.clone());
        }
        Some(serde_json::Value::Object(entity))
    }

    fn version(
        &self,
        subject: &str,
        selector: &str,
    ) -> Result<serde_json::Value, (u16, i64, String)> {
        let store = self.store.lock().expect("store");
        let Some(versions) = store.subjects.get(subject).filter(|v| !v.is_empty()) else {
            return Err((404, 40401, format!("Subject '{subject}' not found")));
        };
        let version = if selector.eq_ignore_ascii_case("latest") {
            versions.keys().max().copied()
        } else {
            selector.parse::<u32>().ok()
        };
        let Some(id) = version.and_then(|v| versions.get(&v)).copied() else {
            return Err((404, 40402, format!("Version '{selector}' not found")));
        };
        let entry = store.schemas.get(&id).expect("registered id");
        let mut entity = serde_json::Map::new();
        entity.insert(
            "subject".into(),
            serde_json::Value::String(subject.to_string()),
        );
        entity.insert("version".into(), serde_json::json!(version));
        entity.insert("id".into(), serde_json::json!(id));
        entity.insert(
            "schema".into(),
            serde_json::Value::String(entry.text.clone()),
        );
        if entry.schema_type != "AVRO" {
            entity.insert(
                "schemaType".into(),
                serde_json::Value::String(entry.schema_type.clone()),
            );
        }
        Ok(serde_json::Value::Object(entity))
    }
}

fn respond(status: u16, body: serde_json::Value) -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        .header("content-type", MEDIA_TYPE)
        .body(Full::new(Bytes::from(body.to_string())))
        .expect("response")
}

fn error(status: u16, code: i64, message: &str) -> Response<Full<Bytes>> {
    respond(
        status,
        serde_json::json!({"error_code": code, "message": message}),
    )
}

fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(value) = u8::from_str_radix(&text[i + 1..i + 3], 16) {
                out.push(value);
                i += 3;
                continue;
            }
        }
        out.push(if bytes[i] == b'+' { b' ' } else { bytes[i] });
        i += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}

async fn handle(
    state: Arc<State>,
    request: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let authorization = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    let body = request.into_body().collect().await?.to_bytes();
    if let Some((client_id, client_secret)) = &state.oauth {
        if method == hyper::Method::POST && path == TOKEN_PATH {
            return Ok(issue_token(
                &state,
                client_id,
                client_secret,
                authorization.as_deref(),
                &body,
            ));
        }
        let accepted = authorization
            .as_deref()
            .and_then(|a| a.strip_prefix("Bearer "))
            .is_some_and(|token| state.issued_tokens.lock().expect("tokens").contains(token));
        if !accepted {
            return Ok(error(401, 401, "Bearer token required"));
        }
    }
    let segments: Vec<&str> = path.split('/').collect();
    let response = match (method.as_str(), segments.as_slice()) {
        ("POST", ["", "subjects", subject, "versions"]) => {
            let Ok(entity) = serde_json::from_slice::<serde_json::Value>(&body) else {
                return Ok(error(422, 42201, "Invalid schema"));
            };
            let Some(text) = entity.get("schema").and_then(serde_json::Value::as_str) else {
                return Ok(error(422, 42201, "Missing schema"));
            };
            let schema_type = entity
                .get("schemaType")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("AVRO");
            let id = state.register(
                &percent_decode(subject),
                schema_type,
                text,
                serde_json::Map::new(),
            );
            respond(200, serde_json::json!({"id": id}))
        }
        ("GET", ["", "subjects", subject, "versions", selector]) => {
            state.lookups.fetch_add(1, Ordering::AcqRel);
            match state.version(&percent_decode(subject), selector) {
                Ok(entity) => respond(200, entity),
                Err((status, code, message)) => error(status, code, &message),
            }
        }
        ("GET", ["", "schemas", "ids", id]) => {
            state.lookups.fetch_add(1, Ordering::AcqRel);
            match id.parse::<i32>().ok().and_then(|id| state.entity_by_id(id)) {
                Some(entity) => respond(200, entity),
                None => error(404, 40403, &format!("Schema {id} not found")),
            }
        }
        _ => error(404, 404, &format!("Not found: {method} {path}")),
    };
    Ok(response)
}

/// `POST /oauth/token`: the client-credentials grant, accepting the client
/// id/secret as HTTP Basic (what Kafka's retriever and this codec send) or as
/// `client_id` / `client_secret` form fields. Answers the standard token
/// response with an opaque token this instance then honours.
fn issue_token(
    state: &State,
    client_id: &str,
    client_secret: &str,
    authorization: Option<&str>,
    body: &[u8],
) -> Response<Full<Bytes>> {
    let form = String::from_utf8_lossy(body);
    let mut params: HashMap<String, String> = HashMap::new();
    for pair in form.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            params.insert(percent_decode(key), percent_decode(value));
        }
    }
    let (mut id, mut secret) = (
        params.get("client_id").cloned(),
        params.get("client_secret").cloned(),
    );
    if let Some(basic) = authorization.and_then(|a| a.strip_prefix("Basic ")) {
        if let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(basic) {
            if let Some((user, password)) = String::from_utf8_lossy(&decoded).split_once(':') {
                id = Some(user.to_string());
                secret = Some(password.to_string());
            }
        }
    }
    if id.as_deref() != Some(client_id) || secret.as_deref() != Some(client_secret) {
        return respond(401, serde_json::json!({"error": "invalid_client"}));
    }
    if params.get("grant_type").map(String::as_str) != Some("client_credentials") {
        return respond(400, serde_json::json!({"error": "unsupported_grant_type"}));
    }
    let count = state.token_requests.fetch_add(1, Ordering::AcqRel) + 1;
    let token = format!("tok-{count}-{}", uuid::Uuid::new_v4().simple());
    state
        .issued_tokens
        .lock()
        .expect("tokens")
        .insert(token.clone());
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(
            serde_json::json!({
                "access_token": token,
                "token_type": "Bearer",
                "expires_in": TOKEN_LIFETIME_SECONDS,
                "scope": params.get("scope").cloned().unwrap_or_else(|| "registry".to_string())
            })
            .to_string(),
        )))
        .expect("response")
}
