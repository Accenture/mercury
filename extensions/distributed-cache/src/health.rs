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

//! The distributed cache's Redis health check — Rust port of the Java
//! `CacheRedisHealthCheck`: the thin binding of the shared
//! [`RedisHealthProbe`] to the reserved route `redis.health` and the plain
//! `redis.*` config namespace. The route name was reserved for exactly this
//! module when sync-over-async took `soa.redis.health`, so the two coexist
//! without collision, each reporting on its own server.
//!
//! Gated by the same `redis.cache.enabled` switch as the cache function, so it
//! registers only when the cache is enabled. Add `redis.health` to
//! `mandatory.health.dependencies` (or `optional.health.dependencies`) and
//! `/health` will include the cache Redis status. The probe's configuration is
//! resolved lazily — never at registration — so a credential a bootstrap
//! publishes after start-up is picked up (see the foundation's probe).
//! `redis.health.timeout` (default `5s`) bounds the probe;
//! `redis.health.startup.grace` (default `30s`) is the start-up placeholder
//! window.

use std::collections::HashMap;
use std::sync::OnceLock;

use async_trait::async_trait;
use platform_core::{preload, AppError, ComposableFunction, EventEnvelope};
use redis_connection::{resolve_duration, RedisConfig, RedisHealthProbe, BASE_PREFIX};

/// The cache's health-check route — the plain-named counterpart of
/// sync-over-async's `soa.redis.health`.
pub const HEALTH_ROUTE: &str = "redis.health";

const TIMEOUT_KEY: &str = "redis.health.timeout";
const GRACE_KEY: &str = "redis.health.startup.grace";
const DEFAULT_TIMEOUT: &str = "5s";
const DEFAULT_GRACE: &str = "30s";

/// `redis.health` — registered by the preload inventory alongside the cache
/// function when `redis.cache.enabled=true`.
#[preload(route = "redis.health", instances = 5)]
#[optional_service("redis.cache.enabled")]
pub struct CacheRedisHealthCheck;

impl CacheRedisHealthCheck {
    /// The one probe behind every worker of this route, built on the first
    /// event: settings re-resolved from `redis.*` on every rebuild, timeout
    /// and grace from `redis.health.*`.
    fn probe() -> &'static RedisHealthProbe {
        static PROBE: OnceLock<RedisHealthProbe> = OnceLock::new();
        PROBE.get_or_init(|| {
            RedisHealthProbe::new(
                || RedisConfig::from_prefix(BASE_PREFIX),
                resolve_duration(TIMEOUT_KEY, TIMEOUT_KEY, DEFAULT_TIMEOUT),
                resolve_duration(GRACE_KEY, GRACE_KEY, DEFAULT_GRACE),
            )
        })
    }
}

#[async_trait]
impl ComposableFunction for CacheRedisHealthCheck {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Self::probe().handle_event(headers, input, instance).await
    }
}
