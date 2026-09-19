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

//! The distributed cache as one composable action function — route
//! `v1.cache.redis` (Rust port of the Java `RedisCache`, design spec §4.4).
//! Opt in with `redis.cache.enabled=true`; the function then registers and is
//! reachable at Layer 1 (`po.request`), Layer 2 (an Event Script task with
//! output data mapping) and Layer 3 (a `graph.task` node) — all three are just
//! "call a route".
//!
//! **Contract.** The `action` header selects the operation ([`CacheAction`]);
//! the payload rides in headers and the body:
//!
//! | action | headers | body (input) | result |
//! |---|---|---|---|
//! | `GET` | `key` | — | value bytes, or null (miss) |
//! | `PUT` | `key`, `ttl`? | value bytes | `true` |
//! | `DELETE` | `key` | — | count removed (integer) |
//! | `PUT_IF_NOT_PRESENT` | `key`, `ttl`? | value bytes | boolean (stored?) |
//! | `MGET` | — | list of keys | map key → bytes (misses omitted) |
//! | `MPUT` | `ttl`? | map key → bytes | `true` |
//! | `LIST_PUSH` | `key`, `ttl`? | value bytes | new length (integer) |
//! | `LIST_POP` | `key` | — | value bytes, or null (empty) |
//! | `LIST_LEN` | `key` | — | length (integer) |
//!
//! Values are opaque bytes — a MsgPack **binary** body
//! (`EventEnvelope::set_raw_body(Value::Binary(..))`; the Java `byte[]`) — the
//! caller owns serialisation; a string body is accepted as a UTF-8
//! convenience. `ttl` is a duration string (`30s`/`5m`/`1h`, or bare
//! seconds); when omitted a write uses `redis.cache.default.ttl`. Every worker
//! instance (`redis.cache.instances`) shares the one connection held by
//! [`runtime`](crate::runtime).

use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::{preload, AppError, ComposableFunction, EventEnvelope};
use redis_connection::duration_seconds;
use rmpv::Value;

use crate::action::CacheAction;
use crate::runtime;
use crate::store::RedisCacheStore;

/// The cache function's route.
pub const CACHE_ROUTE: &str = "v1.cache.redis";

const ACTION: &str = "action";
const KEY: &str = "key";
const TTL: &str = "ttl";

/// `v1.cache.redis` — the cache as one action function. Registered by the
/// preload inventory when the application links this crate and
/// `redis.cache.enabled=true`; every call resolves the shared store lazily.
#[preload(
    route = "v1.cache.redis",
    instances = 20,
    env_instances = "redis.cache.instances"
)]
#[optional_service("redis.cache.enabled")]
pub struct RedisCache;

#[async_trait]
impl ComposableFunction for RedisCache {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let store = runtime::store().await?;
        handle(&headers, input.body(), &store).await
    }
}

/// The action dispatch over a given store (Java `RedisCache.handleEvent`) —
/// public as the reuse/test seam the Java constructor-injected `Supplier`
/// provides: drive the contract against any store, e.g. one built against an
/// in-process server.
pub async fn handle(
    headers: &HashMap<String, String>,
    input: &Value,
    store: &RedisCacheStore,
) -> Result<EventEnvelope, AppError> {
    let action = CacheAction::from_header(headers.get(ACTION).map(String::as_str))?;
    let key = headers.get(KEY).map(String::as_str);
    let reply = match action {
        CacheAction::Get => binary_or_nil(store.get(key).await?),
        CacheAction::Put => {
            store
                .put(key, &as_bytes(input)?, ttl(headers, store)?)
                .await?;
            Value::Boolean(true)
        }
        CacheAction::Delete => Value::from(store.delete(key).await?),
        CacheAction::PutIfNotPresent => Value::Boolean(
            store
                .put_if_absent(key, &as_bytes(input)?, ttl(headers, store)?)
                .await?,
        ),
        CacheAction::Mget => Value::Map(
            store
                .mget(&as_key_list(input)?)
                .await?
                .into_iter()
                .map(|(key, value)| (Value::from(key), Value::Binary(value)))
                .collect(),
        ),
        CacheAction::Mput => {
            store
                .mput(&as_entry_map(input)?, ttl(headers, store)?)
                .await?;
            Value::Boolean(true)
        }
        CacheAction::ListPush => Value::from(
            store
                .list_push(key, &as_bytes(input)?, ttl(headers, store)?)
                .await?,
        ),
        CacheAction::ListPop => binary_or_nil(store.list_pop(key).await?),
        CacheAction::ListLen => Value::from(store.list_len(key).await?),
    };
    Ok(EventEnvelope::new().set_raw_body(reply))
}

fn binary_or_nil(value: Option<Vec<u8>>) -> Value {
    value.map(Value::Binary).unwrap_or(Value::Nil)
}

/// The write-TTL: the `ttl` header (a duration string) when present, else the
/// configured default. A header that does not parse to a positive duration is
/// rejected (the Java engine's `getDurationInSeconds` would degrade it to a
/// zero TTL the server rejects — the same outcome, said clearly).
fn ttl(headers: &HashMap<String, String>, store: &RedisCacheStore) -> Result<u64, AppError> {
    match headers.get(TTL).map(|text| text.trim()) {
        Some(text) if !text.is_empty() => duration_seconds(text)
            .filter(|seconds| *seconds > 0)
            .ok_or_else(|| AppError::new(400, format!("Invalid 'ttl' - {text}"))),
        _ => Ok(store.default_ttl_seconds()),
    }
}

/// The value payload: binary as-is, or a string as UTF-8 (a convenience).
fn as_bytes(input: &Value) -> Result<Vec<u8>, AppError> {
    match input {
        Value::Binary(bytes) => Ok(bytes.clone()),
        Value::String(text) => Ok(text.as_bytes().to_vec()),
        _ => Err(AppError::new(
            400,
            "A value (byte[] or String) is required in the body",
        )),
    }
}

/// The MGET key list: any list whose elements are read as their string form.
fn as_key_list(input: &Value) -> Result<Vec<String>, AppError> {
    match input {
        Value::Array(items) => Ok(items
            .iter()
            .filter(|item| !item.is_nil())
            .map(text_of)
            .collect()),
        _ => Err(AppError::new(
            400,
            "MGET requires a List of keys in the body",
        )),
    }
}

/// The MPUT entries: any map — string keys, binary/string values.
fn as_entry_map(input: &Value) -> Result<Vec<(String, Vec<u8>)>, AppError> {
    match input {
        Value::Map(entries) => entries
            .iter()
            .map(|(key, value)| Ok((text_of(key), as_bytes(value)?)))
            .collect(),
        _ => Err(AppError::new(
            400,
            "MPUT requires a Map of key -> value in the body",
        )),
    }
}

fn text_of(value: &Value) -> String {
    match value {
        Value::String(text) => text.as_str().unwrap_or_default().to_string(),
        other => other.to_string(),
    }
}
