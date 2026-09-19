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

//! **The distributed cache** — Rust port of the Java engine's
//! `extensions/distributed-cache` (`org.platformlambda.cache`, v4.12.9;
//! design spec `draft-design-specs/distributed-cache.md` in the Java repo,
//! Q1–Q8; this port's spec `draft-design-specs/distributed-cache-port.md`).
//!
//! A generic Redis-backed L2 key-value cache exposed as **one composable
//! action function**, route [`CACHE_ROUTE`] (`v1.cache.redis`), over opaque
//! byte values — the caller owns serialisation, which is what lets any layer
//! and any language share the cache. An `action` header selects the
//! operation; key(s) and TTL ride in headers, value(s) in the body.
//!
//! | Type | Java class | Role |
//! |---|---|---|
//! | [`CacheAction`] | `CacheAction` | the bounded, cache-shaped operation set |
//! | [`CacheConfig`] | `CacheConfig` | `redis.cache.*` tunables over the plain `redis.*` connection namespace |
//! | [`RedisCacheStore`] | `RedisCacheStore` | the operations over the shared `RedisBackend` — cluster-safe, every key TTL'd from birth |
//! | [`runtime`] | `CacheRuntime` | the process-wide, lazily built store over ONE multiplexed connection |
//! | [`RedisCache`] | `RedisCache` | the `#[preload]` function `v1.cache.redis`, gated by `redis.cache.enabled` |
//! | [`CacheRedisHealthCheck`] | `CacheRedisHealthCheck` | the `redis.health` binding of the foundation's probe |
//!
//! Two composable functions self-register when this crate is linked into an
//! application AND `redis.cache.enabled=true` (reference the crate from
//! `main.rs` so the linker keeps its registration inventory — the Java
//! "include the jar" deployment story):
//!
//! - **`v1.cache.redis`** — the cache; `redis.cache.instances` (default 20)
//!   is worker concurrency, NOT a connection count: every worker shares the
//!   one multiplexed connection the runtime holds.
//! - **`redis.health`** — the `/health` probe of the cache's Redis; add it to
//!   `mandatory.health.dependencies` or `optional.health.dependencies`.
//!
//! This crate is imported by the APPLICATION, never by the engine — a cache
//! is a deployment choice. It does not depend on sync-over-async; both depend
//! on the `redis-connection` foundation, in the Java dependency direction.
//!
//! **Cross-engine contract.** Cache keys are plain Redis keys
//! (`{redis.cache.key.prefix}{key}`), values are the caller's bytes, the
//! action names and the configuration keys are the Java engine's — so a Java
//! pod and a Rust pod configured alike read and write one cache.

mod action;
mod config;
mod function;
mod health;
pub mod runtime;
mod store;

pub use action::CacheAction;
pub use config::{
    CacheConfig, CACHE_ENABLED_KEY, CACHE_INSTANCES_KEY, DEFAULT_TTL_KEY, KEY_PREFIX_KEY,
};
pub use function::{handle, RedisCache, CACHE_ROUTE};
pub use health::{CacheRedisHealthCheck, HEALTH_ROUTE};
pub use store::RedisCacheStore;
