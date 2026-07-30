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

//! Redis implementation of the graph suspend/resume state-store contract —
//! Rust port of the Java `extensions/minigraph-state-redis` module.
//!
//! Two composable functions self-register when this crate is linked into an
//! application (reference the crate from `main.rs` so the linker keeps its
//! annotation inventory — the Java "include the jar" deployment story):
//!
//! - **`v1.redis.persist.model`** (`type=put`) — stores the persistence
//!   envelope `{cid, node, ttl, model, seen, run}` opaquely (MsgPack bytes)
//!   under the business correlation ID with the requested time-to-live
//!   (Redis SETEX — expiry is native, no sweeper needed). A 2xx reply is the
//!   durability acknowledgement the `graph.suspend` skill requires before
//!   the graph run completes.
//! - **`v1.redis.retrieve.model`** (`type=get`) — returns the persisted
//!   record, or an empty map when absent-or-expired (a fresh transaction is
//!   the normal case, not an error). The record is CONSUMED atomically on
//!   retrieval (Redis GETDEL — requires Redis 6.2+), so a duplicate resume
//!   request cannot execute the continuation twice (at-most-once resume).
//!
//! This crate is imported by the APPLICATION (e.g. the playground example),
//! NEVER by the knowledge-graph engine — the store behind a checkpoint is an
//! application deployment choice; engine tests use a temp-file mock instead.
//! Connection management and the `redis.*` configuration keys live in
//! [`connection`].

mod connection;

use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::{preload, AppError, ComposableFunction, EventEnvelope};
use rmpv::Value;

use connection::KEY_PREFIX;

/// The persist route (`graph.suspend`'s `task`).
pub const PERSIST_ROUTE: &str = "v1.redis.persist.model";
/// The retrieve route (`graph.resume`'s `task`).
pub const RETRIEVE_ROUTE: &str = "v1.redis.retrieve.model";

const TYPE: &str = "type";
const PUT: &str = "put";
const GET: &str = "get";
const CID: &str = "cid";
const TTL: &str = "ttl";

/// `v1.redis.persist.model` — the PERSIST half of the state-store contract,
/// invoked by the `graph.suspend` skill through the node's `task` property.
#[preload(
    route = "v1.redis.persist.model",
    instances = 50,
    env_instances = "worker.instances.v1.redis.persist.model"
)]
pub struct PersistModel;

#[async_trait]
impl ComposableFunction for PersistModel {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if headers.get(TYPE).map(String::as_str) != Some(PUT) {
            return Err(AppError::new(400, "Type must be put"));
        }
        let cid = required_cid(input.body())?;
        let ttl_seconds = match map_get(input.body(), TTL) {
            Some(Value::Integer(n)) => n.as_i64().unwrap_or(0),
            _ => 0,
        };
        if ttl_seconds < 1 {
            return Err(AppError::new(400, "Invalid ttl"));
        }
        let bytes = pack(input.body());
        let mut redis = connection::manager().await?;
        with_deadline(
            redis::cmd("SETEX")
                .arg(format!("{KEY_PREFIX}{cid}"))
                .arg(ttl_seconds)
                .arg(bytes)
                .query_async::<()>(&mut redis),
        )
        .await?;
        log::info!("Persisted workflow state for cid {cid}, ttl={ttl_seconds}s");
        Ok(EventEnvelope::new()
            .set_raw_body(Value::Map(vec![(Value::from("stored"), Value::from(true))])))
    }
}

/// `v1.redis.retrieve.model` — the RETRIEVE half of the state-store
/// contract, invoked by the `graph.resume` skill through the node's `task`
/// property.
#[preload(
    route = "v1.redis.retrieve.model",
    instances = 50,
    env_instances = "worker.instances.v1.redis.retrieve.model"
)]
pub struct RetrieveModel;

#[async_trait]
impl ComposableFunction for RetrieveModel {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if headers.get(TYPE).map(String::as_str) != Some(GET) {
            return Err(AppError::new(400, "Type must be get"));
        }
        let cid = required_cid(input.body())?;
        let mut redis = connection::manager().await?;
        let data: Option<Vec<u8>> = with_deadline(
            redis::cmd("GETDEL")
                .arg(format!("{KEY_PREFIX}{cid}"))
                .query_async(&mut redis),
        )
        .await?;
        match data {
            None => Ok(EventEnvelope::new().set_raw_body(Value::Map(vec![]))),
            Some(bytes) => {
                log::info!("Restored workflow state for cid {cid}");
                Ok(EventEnvelope::new().set_raw_body(unpack(&bytes)))
            }
        }
    }
}

/// Extract the mandatory non-blank `cid` (Java: "Missing cid").
fn required_cid(body: &Value) -> Result<String, AppError> {
    match map_get(body, CID) {
        Some(Value::String(text)) => {
            let cid = text.as_str().unwrap_or_default().trim();
            if cid.is_empty() {
                Err(AppError::new(400, "Missing cid"))
            } else {
                Ok(cid.to_string())
            }
        }
        _ => Err(AppError::new(400, "Missing cid")),
    }
}

fn map_get<'a>(body: &'a Value, key: &str) -> Option<&'a Value> {
    if let Value::Map(entries) = body {
        for (k, v) in entries {
            if k.as_str() == Some(key) {
                return Some(v);
            }
        }
    }
    None
}

fn pack(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    // encoding an in-memory rmpv value cannot fail on a Vec sink
    let _ = rmpv::encode::write_value(&mut out, value);
    out
}

fn unpack(bytes: &[u8]) -> Value {
    rmpv::decode::read_value(&mut &bytes[..]).unwrap_or(Value::Map(vec![]))
}

/// Bound a Redis round-trip by `redis.timeout.ms` and normalize failures to
/// AppError, so a broken store surfaces as the suspend/resume skill's loud
/// failure instead of an indefinite hang.
async fn with_deadline<T>(
    fut: impl std::future::Future<Output = Result<T, redis::RedisError>>,
) -> Result<T, AppError> {
    let timeout = connection::request_timeout();
    match tokio::time::timeout(timeout, fut).await {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(e)) => Err(AppError::new(500, format!("Redis error - {e}"))),
        Err(_) => Err(AppError::new(
            500,
            format!("Redis request timed out after {}ms", timeout.as_millis()),
        )),
    }
}
