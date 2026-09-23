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

//! The cache operations over a [`RedisBackend`] whose values are opaque bytes
//! — Rust port of the Java `RedisCacheStore` (design spec §4.4, Q3). The
//! caller owns serialisation, which maximises cross-layer and cross-language
//! interop. Keys are strings, each transparently namespaced by an optional
//! application key-prefix (Q6) so several apps can share one Redis without
//! colliding; the prefix is stripped again on the way out of `MGET`.
//!
//! Every operation is **cluster-safe by construction** (spec §4.5): all
//! single-key ops route to their slot as-is; `MGET` keys may span slots and
//! the cluster client scatter-gathers them; `MPUT` is a pipelined batch of
//! single-key `SETEX` (each routes to its own slot, non-atomic across the map
//! — correct for a cache, and unavoidable on a cluster). Every stored key
//! carries a **TTL from creation**: `SETEX` for values, atomic `SET NX EX` for
//! put-if-absent, and `RPUSH` + `EXPIRE` as ONE atomic `MULTI`/`EXEC` step for
//! list push — never a two-command sequence that could leave a TTL-less key
//! if the client died between them (the discipline the sync-over-async return
//! route follows; the Java module uses an `EVAL` for the same step — the
//! transaction is this port's ruled equivalent, port spec §4).
//!
//! Thread-safe: the backend multiplexes over the one shared connection, so
//! every worker instance uses this store concurrently.

use platform_core::AppError;
use redis_connection::RedisBackend;

/// The cache operations (Java `RedisCacheStore`).
/// Commands that are safe to run twice (`SETEX`, `GET`, `MGET`, `MPUT`, `DEL`,
/// `LLEN`) go through the backend's idempotent path and are retried once when
/// the connection was lost to a restart; `SET NX`, `RPUSH` and `LPOP` are never
/// replayed (`redis_connection::backend`, *Lifecycle*).
pub struct RedisCacheStore {
    backend: RedisBackend,
    key_prefix: String,
    default_ttl_seconds: u64,
}

impl RedisCacheStore {
    /// `backend` is the standalone-or-cluster backend (one shared, multiplexed
    /// connection); `key_prefix` is prepended to every key (blank = none);
    /// `default_ttl_seconds` applies to writes that do not specify one.
    pub fn new(
        backend: RedisBackend,
        key_prefix: impl Into<String>,
        default_ttl_seconds: u64,
    ) -> Self {
        RedisCacheStore {
            backend,
            key_prefix: key_prefix.into(),
            default_ttl_seconds,
        }
    }

    /// The default TTL (seconds) applied when a write omits one.
    pub fn default_ttl_seconds(&self) -> u64 {
        self.default_ttl_seconds
    }

    /// The backend this store runs on (diagnostics: `cluster()`, `endpoint()`).
    pub fn backend(&self) -> &RedisBackend {
        &self.backend
    }

    /// `SETEX key ttl value`.
    pub async fn put(
        &self,
        key: Option<&str>,
        value: &[u8],
        ttl_seconds: u64,
    ) -> Result<(), AppError> {
        let key = self.prefixed(key)?;
        self.backend
            .query_idempotent::<String>(redis::cmd("SETEX").arg(key).arg(ttl_seconds).arg(value))
            .await
            .map(|_| ())
    }

    /// `GET key` — the value, or `None` on a miss.
    pub async fn get(&self, key: Option<&str>) -> Result<Option<Vec<u8>>, AppError> {
        let key = self.prefixed(key)?;
        self.backend
            .query_idempotent(redis::cmd("GET").arg(key))
            .await
    }

    /// `MGET k1 k2 …` — misses omitted, request order kept, keys returned
    /// without the prefix. On a cluster the keys may span slots; the cluster
    /// client scatter-gathers the request.
    pub async fn mget(&self, keys: &[String]) -> Result<Vec<(String, Vec<u8>)>, AppError> {
        if keys.is_empty() {
            return Ok(Vec::new());
        }
        let mut cmd = redis::cmd("MGET");
        for key in keys {
            cmd.arg(self.prefixed(Some(key))?);
        }
        let values: Vec<Option<Vec<u8>>> = self.backend.query_idempotent(&cmd).await?;
        Ok(keys
            .iter()
            .zip(values)
            .filter_map(|(key, value)| value.map(|bytes| (key.clone(), bytes)))
            .collect())
    }

    /// Bulk write as a **pipelined** batch of single-key `SETEX` — one round
    /// trip, each key keeping its TTL (raw `MSET` sets none). Non-atomic
    /// across the map, and each key routes to its own slot, so the map may
    /// span cluster slots freely.
    pub async fn mput(
        &self,
        entries: &[(String, Vec<u8>)],
        ttl_seconds: u64,
    ) -> Result<(), AppError> {
        if entries.is_empty() {
            return Ok(());
        }
        let mut pipe = redis::pipe();
        for (key, value) in entries {
            pipe.cmd("SETEX")
                .arg(self.prefixed(Some(key))?)
                .arg(ttl_seconds)
                .arg(value.as_slice());
        }
        self.backend
            .query_pipeline_idempotent::<Vec<String>>(&pipe)
            .await
            .map(|_| ())
    }

    /// `DEL key` — the number of keys removed (0 or 1).
    pub async fn delete(&self, key: Option<&str>) -> Result<i64, AppError> {
        let key = self.prefixed(key)?;
        self.backend
            .query_idempotent(redis::cmd("DEL").arg(key))
            .await
    }

    /// `SET key value NX EX ttl` — atomic put-if-absent with a TTL in one
    /// command (not `SETNX` then `EXPIRE`, which leaves a TTL-less key if the
    /// process dies between them). `true` if stored, `false` if the key existed.
    pub async fn put_if_absent(
        &self,
        key: Option<&str>,
        value: &[u8],
        ttl_seconds: u64,
    ) -> Result<bool, AppError> {
        let key = self.prefixed(key)?;
        let reply: Option<String> = self
            .backend
            .query(
                redis::cmd("SET")
                    .arg(key)
                    .arg(value)
                    .arg("NX")
                    .arg("EX")
                    .arg(ttl_seconds),
            )
            .await?;
        Ok(reply.as_deref() == Some("OK"))
    }

    /// `RPUSH key value` then `EXPIRE key ttl` as one atomic `MULTI`/`EXEC`
    /// step (so the list key is never left TTL-less). The new list length.
    pub async fn list_push(
        &self,
        key: Option<&str>,
        value: &[u8],
        ttl_seconds: u64,
    ) -> Result<i64, AppError> {
        let key = self.prefixed(key)?;
        let (length, _expire_set): (i64, i64) = self
            .backend
            .query_pipeline(
                redis::pipe()
                    .atomic()
                    .cmd("RPUSH")
                    .arg(&key)
                    .arg(value)
                    .cmd("EXPIRE")
                    .arg(&key)
                    .arg(ttl_seconds),
            )
            .await?;
        Ok(length)
    }

    /// `LPOP key` — destructive: the oldest value, or `None` when the list is
    /// empty.
    pub async fn list_pop(&self, key: Option<&str>) -> Result<Option<Vec<u8>>, AppError> {
        let key = self.prefixed(key)?;
        self.backend.query(redis::cmd("LPOP").arg(key)).await
    }

    /// `LLEN key` — the list length (0 for an absent list).
    pub async fn list_len(&self, key: Option<&str>) -> Result<i64, AppError> {
        let key = self.prefixed(key)?;
        self.backend
            .query_idempotent(redis::cmd("LLEN").arg(key))
            .await
    }

    fn prefixed(&self, key: Option<&str>) -> Result<String, AppError> {
        match key.map(str::trim) {
            Some(key) if !key.is_empty() => Ok(format!("{}{key}", self.key_prefix)),
            _ => Err(AppError::new(400, "Missing 'key'")),
        }
    }
}
