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

//! Cache tunables from application configuration, plus the shared
//! [`RedisConfig`] connection parameters resolved from the plain `redis.*`
//! namespace ([`BASE_PREFIX`]) — Rust port of the Java `CacheConfig`. The
//! cache and sync-over-async (`soa.redis.*`) therefore never collide, while a
//! deployment can still point both at one Redis by setting only `redis.*`.
//!
//! ```properties
//! redis.cache.enabled=true            # opt-in master switch (the two functions register only when true)
//! redis.cache.instances=20            # worker instances (function concurrency), NOT connections -
//!                                     #   every worker shares the runtime's one multiplexed connection
//! redis.cache.default.ttl=1h          # default TTL applied when a PUT/MPUT/LIST_PUSH omits one
//! redis.cache.key.prefix=             # optional namespace prepended to every key (isolate apps sharing one Redis)
//! ```

use platform_core::AppConfigReader;
use redis_connection::{duration_seconds, RedisConfig, BASE_PREFIX};

/// `redis.cache.enabled` — the opt-in master switch.
pub const CACHE_ENABLED_KEY: &str = "redis.cache.enabled";
/// `redis.cache.instances` — worker instances of `v1.cache.redis`.
pub const CACHE_INSTANCES_KEY: &str = "redis.cache.instances";
/// `redis.cache.key.prefix` — the application namespace prepended to every key.
pub const KEY_PREFIX_KEY: &str = "redis.cache.key.prefix";
/// `redis.cache.default.ttl` — the TTL a write uses when it omits `ttl`.
pub const DEFAULT_TTL_KEY: &str = "redis.cache.default.ttl";

const DEFAULT_TTL: &str = "1h";
const DEFAULT_TTL_SECONDS: u64 = 3600;

/// The resolved cache configuration (Java `CacheConfig` record).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheConfig {
    redis: RedisConfig,
    key_prefix: String,
    default_ttl_seconds: u64,
}

impl CacheConfig {
    /// Read the tunables and the plain-`redis.*` connection parameters from
    /// application configuration (Java `CacheConfig.from`). A missing or
    /// unparseable `redis.cache.default.ttl` degrades to the built-in `1h`
    /// rather than to a zero TTL the server would reject (a port-side
    /// tightening of the Java `getDurationInSeconds` → 0 behaviour).
    pub fn from_config() -> Self {
        let config = AppConfigReader::get_instance();
        let ttl_text = config.get_property_or(DEFAULT_TTL_KEY, DEFAULT_TTL);
        CacheConfig {
            redis: RedisConfig::from_prefix(BASE_PREFIX),
            key_prefix: config.get_property(KEY_PREFIX_KEY).unwrap_or_default(),
            default_ttl_seconds: duration_seconds(&ttl_text)
                .filter(|seconds| *seconds > 0)
                .unwrap_or(DEFAULT_TTL_SECONDS),
        }
    }

    /// Explicit constructor for tests and embedders.
    pub fn new(
        redis: RedisConfig,
        key_prefix: impl Into<String>,
        default_ttl_seconds: u64,
    ) -> Self {
        CacheConfig {
            redis,
            key_prefix: key_prefix.into(),
            default_ttl_seconds,
        }
    }

    /// The shared connection parameters (host/port/auth/ssl/cluster) from `redis.*`.
    pub fn redis(&self) -> &RedisConfig {
        &self.redis
    }

    /// Prepended to every cache key; blank = no prefix.
    pub fn key_prefix(&self) -> &str {
        &self.key_prefix
    }

    /// Default TTL in seconds for writes that omit one.
    pub fn default_ttl_seconds(&self) -> u64 {
        self.default_ttl_seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use platform_core::overrides;

    fn serial() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    const KEYS: &[&str] = &[
        "redis.cache.key.prefix",
        "redis.cache.default.ttl",
        "redis.host",
        "redis.port",
        "soa.redis.host",
    ];

    fn clear() {
        for key in KEYS {
            overrides::clear(key);
        }
    }

    /// Java `CacheConfigTest.defaultsWhenUnset`.
    #[test]
    fn defaults_when_unset() {
        let _guard = serial();
        clear();
        let config = CacheConfig::from_config();
        assert_eq!("", config.key_prefix());
        assert_eq!(3600, config.default_ttl_seconds());
        assert_eq!("127.0.0.1", config.redis().host());
        assert_eq!(6379, config.redis().port());
    }

    /// Java `CacheConfigTest.readsCacheTunablesAndPlainRedisNamespace`.
    #[test]
    fn reads_cache_tunables_and_the_plain_redis_namespace() {
        let _guard = serial();
        clear();
        overrides::set("redis.cache.key.prefix", "app1:");
        overrides::set("redis.cache.default.ttl", "10m");
        overrides::set("redis.host", "cache.example.com");
        overrides::set("redis.port", "6380");
        let config = CacheConfig::from_config();
        assert_eq!("app1:", config.key_prefix());
        assert_eq!(600, config.default_ttl_seconds());
        assert_eq!("cache.example.com", config.redis().host());
        assert_eq!(6380, config.redis().port());
        // a garbage ttl degrades to the built-in default, never to zero
        overrides::set("redis.cache.default.ttl", "forever");
        assert_eq!(3600, CacheConfig::from_config().default_ttl_seconds());
        clear();
    }

    /// Java `CacheConfigTest.ignoresTheSoaNamespace`: sync-over-async's keys
    /// never leak into the cache's connection.
    #[test]
    fn ignores_the_soa_namespace() {
        let _guard = serial();
        clear();
        overrides::set("soa.redis.host", "rendezvous.example.com");
        overrides::set("redis.host", "cache.example.com");
        assert_eq!(
            "cache.example.com",
            CacheConfig::from_config().redis().host()
        );
        overrides::clear("redis.host");
        assert_eq!(
            "127.0.0.1",
            CacheConfig::from_config().redis().host(),
            "without redis.host the cache falls to the default, not to soa.redis.host"
        );
        clear();
    }
}
