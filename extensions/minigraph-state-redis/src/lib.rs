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
//!   envelope `{cid, graph, node, ttl, model, seen, run}` opaquely (MsgPack
//!   bytes) under the key `graph:{graph_id}:{cid}` with the requested
//!   time-to-live (Redis SETEX — expiry is native, no sweeper needed). The
//!   graph ID scopes the record so the same business correlation ID may
//!   suspend independently in each domain's graph and in each subgraph. A
//!   2xx reply is the durability acknowledgement the `graph.suspend` skill
//!   requires before the graph run completes.
//! - **`v1.redis.retrieve.model`** (`type=get`) — returns the persisted
//!   record, or an empty map when absent-or-expired (a fresh transaction is
//!   the normal case, not an error). The record is CONSUMED atomically on
//!   retrieval, so a duplicate resume request cannot execute the
//!   continuation twice (at-most-once resume) — via native GETDEL on Redis
//!   6.2+, or a MULTI/EXEC GET+DEL transaction on older servers (the
//!   strategy is detected from `INFO server` and stated in the startup log,
//!   since enterprise deployments rarely control their managed Redis
//!   version and the redis-standalone Windows binary is 5.0.14).
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

use connection::store_key;

/// The persist route (`graph.suspend`'s `task`).
pub const PERSIST_ROUTE: &str = "v1.redis.persist.model";
/// The retrieve route (`graph.resume`'s `task`).
pub const RETRIEVE_ROUTE: &str = "v1.redis.retrieve.model";

const TYPE: &str = "type";
const PUT: &str = "put";
const GET: &str = "get";
const CID: &str = "cid";
const GRAPH: &str = "graph";
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
        let cid = required_field(input.body(), CID)?;
        let graph_id = required_field(input.body(), GRAPH)?;
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
                .arg(store_key(&graph_id, &cid))
                .arg(ttl_seconds)
                .arg(bytes)
                .query_async::<()>(&mut redis),
        )
        .await?;
        log::info!("Persisted workflow state for graph {graph_id}, cid {cid}, ttl={ttl_seconds}s");
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
        let cid = required_field(input.body(), CID)?;
        let graph_id = required_field(input.body(), GRAPH)?;
        let mut redis = connection::manager().await?;
        let key = store_key(&graph_id, &cid);
        let data: Option<Vec<u8>> = if connection::native_getdel() {
            with_deadline(redis::cmd("GETDEL").arg(&key).query_async(&mut redis)).await?
        } else {
            // servers older than 6.2: an equally atomic MULTI/EXEC GET+DEL -
            // the at-most-once resume guarantee holds on both paths (a plain
            // sequential GET then DEL would open a double-resume race). The
            // atomic pipeline is written as ONE contiguous batch on the
            // multiplexed connection, so another request cannot interleave
            // between MULTI and EXEC (the Java port serializes explicitly
            // for the same guarantee).
            let (value, _deleted): (Option<Vec<u8>>, i64) = with_deadline(
                redis::pipe()
                    .atomic()
                    .get(&key)
                    .del(&key)
                    .query_async(&mut redis),
            )
            .await?;
            value
        };
        match data {
            None => Ok(EventEnvelope::new().set_raw_body(Value::Map(vec![]))),
            Some(bytes) => {
                log::info!("Restored workflow state for graph {graph_id}, cid {cid}");
                Ok(EventEnvelope::new().set_raw_body(unpack(&bytes)))
            }
        }
    }
}

/// Extract a mandatory non-blank field (Java: "Missing cid" / "Missing graph").
/// A cid-only record would collapse every graph's records into one shared key
/// space, so both store functions fail fast when `graph` is absent.
fn required_field(body: &Value, name: &str) -> Result<String, AppError> {
    match map_get(body, name) {
        Some(Value::String(text)) => {
            let value = text.as_str().unwrap_or_default().trim();
            if value.is_empty() {
                Err(AppError::new(400, format!("Missing {name}")))
            } else {
                Ok(value.to_string())
            }
        }
        _ => Err(AppError::new(400, format!("Missing {name}"))),
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
