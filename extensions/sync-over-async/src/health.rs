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

//! sync-over-async's Redis health check for the platform's `/health` endpoint
//! — Rust port of the Java `SoaRedisHealthCheck`: the thin binding of the
//! shared [`RedisHealthProbe`] (the `redis-connection` foundation) to this
//! module's route ([`REDIS_HEALTH_ROUTE`], `soa.redis.health`) and config
//! namespace (`soa.redis.*`, with the plain `redis.*` fallback).
//!
//! Add [`REDIS_HEALTH_ROUTE`] to `mandatory.health.dependencies` (or
//! `optional.health.dependencies`) and `/health` will include the
//! sync-over-async Redis status. The probe semantics — `type=info` /
//! `type=health`, the single PING on a dedicated connection, lazy
//! configuration resolution, the passing `Waiting for Redis connection`
//! status while a credential has not landed, the 503 `{text, code}` on a
//! genuine outage, the start-up placeholder with background warm-up — live in
//! the foundation's probe and are shared with the distributed cache's
//! `redis.health`, which reports on ITS `redis.*` server. Both the route and
//! the config keys carry the `soa.` prefix deliberately so the two coexist
//! without route or config collisions.
//!
//! `soa.redis.health.timeout` (default `5s`) bounds the probe;
//! `soa.redis.health.startup.grace` (default `30s`) is the start-up
//! placeholder window — each falling back to the legacy un-prefixed key
//! (`redis.health.timeout` / `redis.health.startup.grace`) when absent, the
//! same policy the connection keys follow.

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use platform_core::{AppError, ComposableFunction, EventEnvelope, Platform};
use redis_connection::{resolve_duration, RedisHealthProbe};

use crate::connection::RedisSettings;

/// The health-check route name. The `soa.` prefix is normative: the plain
/// `redis.health` route belongs to the distributed cache's check.
pub const REDIS_HEALTH_ROUTE: &str = "soa.redis.health";

const TIMEOUT_KEY: &str = "soa.redis.health.timeout";
const GRACE_KEY: &str = "soa.redis.health.startup.grace";
const LEGACY_TIMEOUT_KEY: &str = "redis.health.timeout";
const LEGACY_GRACE_KEY: &str = "redis.health.startup.grace";
const DEFAULT_TIMEOUT: &str = "5s";
const DEFAULT_GRACE: &str = "30s";

/// The `soa.redis.health` function (see the module documentation). Register
/// it with [`RedisHealthCheck::register`] or hold it and call it directly.
#[derive(Clone)]
pub struct RedisHealthCheck {
    probe: RedisHealthProbe,
}

impl RedisHealthCheck {
    /// Read the probe's timeout and grace from application configuration
    /// (`soa.redis.health.*`, falling back to the legacy `redis.health.*`);
    /// the connection parameters themselves come from `soa.redis.*` (with the
    /// `redis.*` fallback), re-resolved on every rebuild.
    pub fn from_config() -> Self {
        RedisHealthCheck::new(
            RedisSettings::from_config,
            resolve_duration(TIMEOUT_KEY, LEGACY_TIMEOUT_KEY, DEFAULT_TIMEOUT),
            resolve_duration(GRACE_KEY, LEGACY_GRACE_KEY, DEFAULT_GRACE),
        )
    }

    /// Reuse/test seam. `settings` is invoked when the probe connection is
    /// built — and again on every rebuild after a failure — so values
    /// published later in the start-up sequence are resolved on the next probe
    /// instead of being frozen at construction.
    pub fn new(
        settings: impl Fn() -> RedisSettings + Send + Sync + 'static,
        timeout: Duration,
        grace: Duration,
    ) -> Self {
        RedisHealthCheck {
            probe: RedisHealthProbe::new(settings, timeout, grace),
        }
    }

    /// Register this function at [`REDIS_HEALTH_ROUTE`] with the platform —
    /// several workers, because `/health` is polled concurrently.
    pub fn register(self, platform: &Platform) -> Result<(), AppError> {
        self.probe.register(platform, REDIS_HEALTH_ROUTE, 5)
    }
}

#[async_trait]
impl ComposableFunction for RedisHealthCheck {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        self.probe.handle_event(headers, input, instance).await
    }
}
