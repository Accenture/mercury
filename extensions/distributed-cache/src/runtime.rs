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

//! Process-wide holder of the cache's **single shared, multiplexed** backend
//! and the [`RedisCacheStore`] over it — Rust port of the Java `CacheRuntime`.
//! Every `v1.cache.redis` worker instance calls [`store`] and shares this one
//! connection: `redis.cache.instances` is worker concurrency, not a connection
//! count (design spec §4.4: no pool; the client pipelines over one in-order
//! connection).
//!
//! The backend is built **lazily on first use**, re-resolving configuration
//! on every attempt until it connects. This is deliberate (spec §6): the
//! function is registered before a credential-bootstrap application publishes
//! a vault password, and — unlike sync-over-async, which must keep an eager
//! Pub/Sub subscriber live — a cache has nothing to maintain at start-up and
//! **must not fail application start-up when Redis is briefly unreachable**.
//! While the connection cannot be built the store stays empty and each call
//! retries (a late credential is picked up); once built, the client owns
//! reconnection under it and the store is reused.
//!
//! The one connection is released on shutdown through the platform's
//! lifecycle ([`Platform::on_shutdown`]), registered from the build so the
//! cleanup is wired only when a connection has actually been opened.

use std::sync::{Arc, RwLock};

use platform_core::{AppError, Platform};
use redis_connection::RedisBackend;

use crate::config::CacheConfig;
use crate::store::RedisCacheStore;

static STORE: RwLock<Option<Arc<RedisCacheStore>>> = RwLock::new(None);
/// Serializes the slow build so concurrent first callers open ONE connection
/// (the Java double-checked locking under a `ReentrantLock`).
static BUILD: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

/// The shared store, built on first use and reused thereafter. If the
/// connection cannot be built yet, the error propagates to the caller
/// (fail-fast — a cache failure is the caller's concern via the flow's
/// exception handler) and the next call retries with freshly resolved
/// configuration.
pub async fn store() -> Result<Arc<RedisCacheStore>, AppError> {
    if let Some(current) = current() {
        return Ok(current);
    }
    let _guard = BUILD.lock().await;
    if let Some(current) = current() {
        return Ok(current);
    }
    let built = build().await?;
    *STORE.write().unwrap_or_else(|p| p.into_inner()) = Some(built.clone());
    Ok(built)
}

/// The store if one has been built (diagnostics; `None` before the first
/// successful call and after [`shutdown`]).
pub fn current() -> Option<Arc<RedisCacheStore>> {
    STORE.read().unwrap_or_else(|p| p.into_inner()).clone()
}

/// Install a store as the process-wide instance — the reuse/test seam (a store
/// built against an embedded or in-process server). Replaces any current one.
pub fn set(store: Arc<RedisCacheStore>) {
    *STORE.write().unwrap_or_else(|p| p.into_inner()) = Some(store);
}

/// Release the shared backend (idempotent) — registered with
/// [`Platform::on_shutdown`] when the connection opens. Dropping the last
/// handle closes the connection; a later call rebuilds from live configuration.
pub fn shutdown() {
    if STORE
        .write()
        .unwrap_or_else(|p| p.into_inner())
        .take()
        .is_some()
    {
        log::info!("Redis cache connection released");
    }
}

async fn build() -> Result<Arc<RedisCacheStore>, AppError> {
    let config = CacheConfig::from_config();
    let redis = config.redis();
    let backend = RedisBackend::connect(redis).await?;
    // register cleanup now that a connection exists, via the platform's
    // shutdown lifecycle. build() runs once under the lock per connection, so
    // this registers exactly once per opened connection.
    Platform::get_instance().on_shutdown(shutdown);
    log::info!(
        "Redis cache connected (redis {}, ssl={}, cluster={}, keyPrefix='{}', defaultTtl={}s)",
        backend.endpoint(),
        redis.ssl(),
        backend.cluster(),
        config.key_prefix(),
        config.default_ttl_seconds()
    );
    Ok(Arc::new(RedisCacheStore::new(
        backend,
        config.key_prefix(),
        config.default_ttl_seconds(),
    )))
}
