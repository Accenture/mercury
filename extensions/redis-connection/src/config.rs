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

//! Redis connection startup parameters — Rust port of the Java
//! `org.platformlambda.redis.RedisConfig`, read from application
//! configuration as discrete `<prefix>*` keys.
//!
//! **Configurable key prefix.** The struct holds resolved values and is
//! prefix-agnostic; [`RedisConfig::from_prefix`] reads the `<prefix>*` keys,
//! and two consumers share the one loader with different prefixes so they
//! never collide inside one application:
//!
//! - [`SOA_PREFIX`] (`soa.redis.*`) — sync-over-async's namespace;
//! - [`BASE_PREFIX`] (`redis.*`) — the distributed cache's namespace, and the
//!   universal fallback.
//!
//! [`RedisConfig::from_config`] defaults to [`SOA_PREFIX`] — Java parity
//! (`RedisConfig.from(config)`), and unchanged behaviour for sync-over-async's
//! existing callers. **Each key falls back to the un-prefixed `redis.*` form**
//! when the prefixed one is absent, so a pre-existing `redis.*` deployment
//! keeps working with no migration, and a deployment can run one Redis for
//! both consumers (set `redis.*`) or decouple them (also set `soa.redis.*`).
//! When the prefix is already `redis.` the prefixed and base keys coincide.
//!
//! ```properties
//! redis.host=127.0.0.1
//! redis.port=6379
//! redis.username=${REDIS_USERNAME:}   # blank = default user; set for an ACL/RBAC user
//! redis.password=${REDIS_PASSWORD:}   # blank = no auth
//! redis.ssl=false                     # true = TLS
//! redis.database=0                    # standalone only; a cluster is database 0
//! redis.timeout.ms=5000               # default command timeout
//! redis.cluster.detect=auto           # auto = probe INFO at start-up; anything else = decide by the boolean below
//! redis.cluster.mode=false            # true = cluster, false = standalone (also the inconclusive-probe fallback)
//! redis.cluster.nodes=                # cluster seeds host:port,host:port (blank = redis.host:redis.port)
//! ```
//!
//! Values resolve through [`AppConfigReader`], so `${ENV_VAR:default}`
//! substitution applies — keep the password out of the file. Clients are
//! built from live configuration, never frozen at construction: a credential
//! published by a start-up vault bootstrap has not landed while functions are
//! being constructed (the Java preload-before-bootstrap lesson).
//!
//! **Cluster selection (two keys).** `<prefix>cluster.detect=auto` (the
//! default) probes the seed at start-up and picks cluster or standalone from
//! what the server reports; otherwise the boolean `<prefix>cluster.mode`
//! decides. When detection is inconclusive (server unreachable, `INFO`
//! restricted), that same boolean is the fallback. Authentication is identical
//! for both topologies.

use std::time::Duration;

use platform_core::{AppConfigReader, AppError};
use redis::aio::ConnectionManager;
use redis::{ConnectionAddr, ConnectionInfo, IntoConnectionInfo, RedisConnectionInfo};

/// sync-over-async's key namespace (`soa.redis.*`).
pub const SOA_PREFIX: &str = "soa.redis.";
/// The base namespace (`redis.*`) — the distributed cache's prefix, and the
/// universal fallback.
pub const BASE_PREFIX: &str = "redis.";

const AUTO: &str = "auto";

/// Redis connection parameters (Java `RedisConfig` record).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedisConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    ssl: bool,
    database: i64,
    timeout_ms: u64,
    auto_detect_cluster: bool,
    cluster_enabled: bool,
    cluster_nodes: String,
}

impl Default for RedisConfig {
    /// The Java defaults: localhost standalone, no auth, `cluster.detect=auto`.
    fn default() -> Self {
        RedisConfig {
            host: "127.0.0.1".to_string(),
            port: 6379,
            username: String::new(),
            password: String::new(),
            ssl: false,
            database: 0,
            timeout_ms: 5000,
            auto_detect_cluster: true,
            cluster_enabled: false,
            cluster_nodes: String::new(),
        }
    }
}

impl RedisConfig {
    /// Default entry point — the [`SOA_PREFIX`] namespace with the
    /// [`BASE_PREFIX`] fallback (Java `RedisConfig.from(config)`), which is what
    /// sync-over-async's callers have always read.
    pub fn from_config() -> Self {
        Self::from_prefix(SOA_PREFIX)
    }

    /// Read the connection parameters from the `<prefix>*` keys, each falling
    /// back to the un-prefixed base `redis.*` form when the prefixed key is
    /// absent (Java `RedisConfig.from(config, prefix)`). Unset or unparseable
    /// values fall back to [`Default`].
    pub fn from_prefix(prefix: &str) -> Self {
        let config = AppConfigReader::get_instance();
        let defaults = RedisConfig::default();
        let get = |suffix: &str| -> Option<String> {
            config
                .get_property(&format!("{prefix}{suffix}"))
                .or_else(|| config.get_property(&format!("{BASE_PREFIX}{suffix}")))
        };
        let detect = get("cluster.detect").unwrap_or_else(|| AUTO.to_string());
        RedisConfig {
            host: get("host").unwrap_or(defaults.host),
            port: parse_or(get("port"), defaults.port),
            username: get("username").unwrap_or_default(),
            password: get("password").unwrap_or_default(),
            ssl: is_true(get("ssl")),
            database: parse_or(get("database"), defaults.database),
            timeout_ms: parse_or(get("timeout.ms"), defaults.timeout_ms),
            auto_detect_cluster: detect.trim().eq_ignore_ascii_case(AUTO),
            cluster_enabled: is_true(get("cluster.mode")),
            cluster_nodes: get("cluster.nodes").unwrap_or_default(),
        }
    }

    /// Backward-compatible standalone constructor (default user, no cluster,
    /// no probe) — the Java discrete-parameter constructor, and the signature
    /// sync-over-async's `RedisSettings::new` always had.
    pub fn new(
        host: impl Into<String>,
        port: u16,
        password: impl Into<String>,
        ssl: bool,
        database: i64,
        timeout_ms: u64,
    ) -> Self {
        RedisConfig {
            host: host.into(),
            port,
            password: password.into(),
            ssl,
            database,
            timeout_ms,
            auto_detect_cluster: false,
            cluster_enabled: false,
            ..RedisConfig::default()
        }
    }

    /// A copy with an ACL/RBAC username (blank = the default user).
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }

    /// A copy with the command timeout overridden — the health probe bounds
    /// its probe with its own timeout (Java `withTimeout`).
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// A copy with the cluster selection set explicitly: `auto_detect` probes
    /// the seed; otherwise `cluster_enabled` decides (and is the inconclusive
    /// fallback); `cluster_nodes` is the `host:port,host:port` seed list
    /// (blank = the single host:port).
    pub fn with_cluster(
        mut self,
        auto_detect: bool,
        cluster_enabled: bool,
        cluster_nodes: impl Into<String>,
    ) -> Self {
        self.auto_detect_cluster = auto_detect;
        self.cluster_enabled = cluster_enabled;
        self.cluster_nodes = cluster_nodes.into();
        self
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    /// ACL/RBAC username; blank = the default user.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Whether a password is configured (the value itself never leaves the
    /// struct except into the connection it authenticates).
    pub fn has_password(&self) -> bool {
        !self.password.trim().is_empty()
    }

    pub fn ssl(&self) -> bool {
        self.ssl
    }

    /// Logical database index (standalone only; a cluster is database 0).
    pub fn database(&self) -> i64 {
        self.database
    }

    pub fn timeout_ms(&self) -> u64 {
        self.timeout_ms
    }

    /// The per-request deadline (`timeout.ms`), at least 1 ms.
    pub fn timeout(&self) -> Duration {
        Duration::from_millis(self.timeout_ms.max(1))
    }

    /// `<prefix>cluster.detect=auto` — probe the seed to choose the topology.
    pub fn auto_detect_cluster(&self) -> bool {
        self.auto_detect_cluster
    }

    /// `<prefix>cluster.mode` — the topology when not auto-detecting, and the
    /// fallback when detection is inconclusive.
    pub fn cluster_enabled(&self) -> bool {
        self.cluster_enabled
    }

    /// `<prefix>cluster.nodes` — the raw seed list.
    pub fn cluster_nodes(&self) -> &str {
        &self.cluster_nodes
    }

    /// `host:port` — the dependency's href in health reports.
    pub fn endpoint(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// The single-node connection descriptor (standalone client / probe) —
    /// Java `toUri()`.
    pub fn connection_info(&self) -> Result<ConnectionInfo, AppError> {
        self.node_info(&self.host, self.port, self.database)
    }

    /// Cluster seed descriptors: the explicit `cluster.nodes` list when set,
    /// else the single `host:port` (a managed cluster's configuration endpoint
    /// is one seed — the client discovers the shards from it). Redis Cluster
    /// is database 0 only, so no database index is applied; auth and TLS carry
    /// over identically to every node (Java `seedUris()`).
    pub fn seed_infos(&self) -> Result<Vec<ConnectionInfo>, AppError> {
        let mut seeds = Vec::new();
        for node in self
            .cluster_nodes
            .split(',')
            .map(str::trim)
            .filter(|node| !node.is_empty())
        {
            let (host, port) = match node.rsplit_once(':') {
                Some((host, port)) if !host.is_empty() => {
                    (host, port.trim().parse::<u16>().unwrap_or(self.port))
                }
                _ => (node, self.port),
            };
            seeds.push(self.node_info(host, port, 0)?);
        }
        if seeds.is_empty() {
            seeds.push(self.node_info(&self.host, self.port, 0)?);
        }
        Ok(seeds)
    }

    /// Build a standalone client. Pub/Sub needs its own dedicated connection,
    /// so the client — not only a multiplexed manager — is what a subscriber
    /// keeps (sync-over-async's coordinator).
    pub fn client(&self) -> Result<redis::Client, AppError> {
        redis::Client::open(self.connection_info()?)
            .map_err(|e| AppError::new(500, format!("Unable to open Redis client - {e}")))
    }

    /// Connect a shared multiplexed manager for ordinary commands. It
    /// reconnects internally after an outage; Pub/Sub cannot ride it.
    pub async fn manager(&self) -> Result<ConnectionManager, AppError> {
        let client = self.client()?;
        tokio::time::timeout(self.timeout(), client.get_connection_manager())
            .await
            .map_err(|_| AppError::new(500, "Redis connection timed out"))?
            .map_err(|e| AppError::new(500, format!("Redis unavailable - {e}")))
    }

    /// Java `applyAuth`: a username selects RBAC (`AUTH user pass`), a bare
    /// password the legacy form (`AUTH pass`); blank = no authentication.
    fn node_info(&self, host: &str, port: u16, database: i64) -> Result<ConnectionInfo, AppError> {
        let addr = if self.ssl {
            ConnectionAddr::TcpTls {
                host: host.to_string(),
                port,
                insecure: false,
                tls_params: None,
            }
        } else {
            ConnectionAddr::Tcp(host.to_string(), port)
        };
        let mut redis_settings = RedisConnectionInfo::default().set_db(database);
        if !self.username.trim().is_empty() {
            redis_settings = redis_settings.set_username(self.username.trim());
        }
        if self.has_password() {
            redis_settings = redis_settings.set_password(&self.password);
        }
        Ok(addr
            .into_connection_info()
            .map_err(|e| AppError::new(500, format!("Invalid Redis address - {e}")))?
            .set_redis_settings(redis_settings))
    }
}

fn parse_or<T: std::str::FromStr>(value: Option<String>, fallback: T) -> T {
    value
        .and_then(|text| text.trim().parse::<T>().ok())
        .unwrap_or(fallback)
}

fn is_true(value: Option<String>) -> bool {
    value
        .map(|text| text.trim().eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use platform_core::overrides;

    /// The loader reads the process-wide configuration singleton, so the
    /// tests that set overrides run one at a time.
    fn serial() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn clear(keys: &[&str]) {
        for key in keys {
            overrides::clear(key);
        }
    }

    const ALL_KEYS: &[&str] = &[
        "redis.host",
        "redis.port",
        "redis.username",
        "redis.password",
        "redis.ssl",
        "redis.database",
        "redis.timeout.ms",
        "redis.cluster.detect",
        "redis.cluster.mode",
        "redis.cluster.nodes",
        "soa.redis.host",
        "soa.redis.port",
        "soa.redis.username",
        "soa.redis.password",
        "soa.redis.cluster.detect",
        "soa.redis.cluster.mode",
        "soa.redis.cluster.nodes",
        "cache.redis.host",
    ];

    /// Java `RedisConfigTest.defaultsWhenUnset`.
    #[test]
    fn defaults_when_unset() {
        let _guard = serial();
        clear(ALL_KEYS);
        let config = RedisConfig::from_prefix(BASE_PREFIX);
        assert_eq!("127.0.0.1", config.host());
        assert_eq!(6379, config.port());
        assert_eq!("", config.username());
        assert!(!config.has_password());
        assert!(!config.ssl());
        assert_eq!(0, config.database());
        assert_eq!(5000, config.timeout_ms());
        assert!(
            config.auto_detect_cluster(),
            "cluster.detect defaults to auto"
        );
        assert!(!config.cluster_enabled());
        assert_eq!("", config.cluster_nodes());
    }

    /// Java `readsDiscreteProperties` + `rbacUsernameIsRead` + `clusterSelectionIsTwoKeys`.
    #[test]
    fn reads_discrete_properties_and_the_two_cluster_keys() {
        let _guard = serial();
        clear(ALL_KEYS);
        overrides::set("redis.host", "cache.example.com");
        overrides::set("redis.port", "6380");
        overrides::set("redis.username", "app-user");
        overrides::set("redis.password", "s3cret");
        overrides::set("redis.ssl", "true");
        overrides::set("redis.database", "2");
        overrides::set("redis.timeout.ms", "1500");
        overrides::set("redis.cluster.detect", "off");
        overrides::set("redis.cluster.mode", "true");
        overrides::set("redis.cluster.nodes", "a:7000, b:7001");
        let config = RedisConfig::from_prefix(BASE_PREFIX);
        assert_eq!("cache.example.com", config.host());
        assert_eq!(6380, config.port());
        assert_eq!("app-user", config.username());
        assert!(config.has_password());
        assert!(config.ssl());
        assert_eq!(2, config.database());
        assert_eq!(1500, config.timeout_ms());
        assert!(
            !config.auto_detect_cluster(),
            "anything but 'auto' turns detection off"
        );
        assert!(config.cluster_enabled());
        assert_eq!("a:7000, b:7001", config.cluster_nodes());
        clear(ALL_KEYS);
    }

    /// Java `fallsBackToLegacyRedisKeysWhenSoaKeysAbsent` + `soaKeysWinOverLegacyKeysWhenBothPresent`.
    #[test]
    fn soa_prefix_falls_back_to_base_keys_and_wins_when_present() {
        let _guard = serial();
        clear(ALL_KEYS);
        overrides::set("redis.host", "shared.example.com");
        overrides::set("redis.port", "6390");
        let fallback = RedisConfig::from_config();
        assert_eq!(
            "shared.example.com",
            fallback.host(),
            "soa.redis.* absent -> redis.*"
        );
        assert_eq!(6390, fallback.port());
        overrides::set("soa.redis.host", "rendezvous.example.com");
        let decoupled = RedisConfig::from_config();
        assert_eq!(
            "rendezvous.example.com",
            decoupled.host(),
            "the soa key wins"
        );
        assert_eq!(
            6390,
            decoupled.port(),
            "a key without a soa override still falls back"
        );
        clear(ALL_KEYS);
    }

    /// Java `basePrefixReadsPlainRedisKeys` + `explicitPrefixIsIsolatedFromTheOtherNamespace`:
    /// the cache's namespace never sees sync-over-async's overrides.
    #[test]
    fn base_prefix_ignores_the_soa_namespace() {
        let _guard = serial();
        clear(ALL_KEYS);
        overrides::set("redis.host", "cache.example.com");
        overrides::set("soa.redis.host", "rendezvous.example.com");
        overrides::set("soa.redis.cluster.mode", "true");
        let cache = RedisConfig::from_prefix(BASE_PREFIX);
        assert_eq!("cache.example.com", cache.host());
        assert!(
            !cache.cluster_enabled(),
            "soa.redis.cluster.mode is not the cache's key"
        );
        let soa = RedisConfig::from_prefix(SOA_PREFIX);
        assert_eq!("rendezvous.example.com", soa.host());
        assert!(soa.cluster_enabled());
        // an unrelated prefix reads only its own keys and the base fallback
        overrides::set("cache.redis.host", "third.example.com");
        assert_eq!(
            "third.example.com",
            RedisConfig::from_prefix("cache.redis.").host()
        );
        clear(ALL_KEYS);
    }

    /// Java `seedUrisFromExplicitNodes` + `seedUrisFallBackToTheSingleHostPort`.
    #[test]
    fn seeds_come_from_the_node_list_or_the_single_host() {
        let explicit = RedisConfig::new("seed.example.com", 6379, "", false, 0, 5000).with_cluster(
            false,
            true,
            "a.example.com:7000, b.example.com:7001,, c.example.com",
        );
        let seeds = explicit.seed_infos().expect("seeds");
        assert_eq!(3, seeds.len());
        assert_eq!(
            &ConnectionAddr::Tcp("a.example.com".to_string(), 7000),
            seeds[0].addr()
        );
        assert_eq!(
            &ConnectionAddr::Tcp("b.example.com".to_string(), 7001),
            seeds[1].addr()
        );
        // a node without a port takes the configured port
        assert_eq!(
            &ConnectionAddr::Tcp("c.example.com".to_string(), 6379),
            seeds[2].addr()
        );
        // a cluster is database 0 whatever the standalone database says
        let single = RedisConfig::new("seed.example.com", 6380, "", false, 3, 5000);
        let seeds = single.seed_infos().expect("seeds");
        assert_eq!(1, seeds.len());
        assert_eq!(
            &ConnectionAddr::Tcp("seed.example.com".to_string(), 6380),
            seeds[0].addr()
        );
        assert_eq!(0, seeds[0].redis_settings().db());
        assert_eq!(
            3,
            single
                .connection_info()
                .expect("info")
                .redis_settings()
                .db()
        );
    }

    /// Java `mapsOntoRedisUri` — auth form and TLS follow the parameters.
    #[test]
    fn connection_info_carries_auth_and_tls() {
        let rbac = RedisConfig::new("h", 6379, "pw", true, 1, 5000).with_username("user");
        let info = rbac.connection_info().expect("info");
        assert!(matches!(info.addr(), ConnectionAddr::TcpTls { .. }));
        assert_eq!(Some("user"), info.redis_settings().username());
        assert_eq!(Some("pw"), info.redis_settings().password());
        let legacy = RedisConfig::new("h", 6379, "pw", false, 0, 5000);
        let info = legacy.connection_info().expect("info");
        assert_eq!(
            None,
            info.redis_settings().username(),
            "no username = the legacy AUTH form"
        );
        assert_eq!(Some("pw"), info.redis_settings().password());
        let open = RedisConfig::new("h", 6379, "  ", false, 0, 5000);
        assert_eq!(
            None,
            open.connection_info()
                .expect("info")
                .redis_settings()
                .password()
        );
    }

    #[test]
    fn explicit_settings_build_a_client_and_clamp_the_timeout() {
        let settings = RedisConfig::new("127.0.0.1", 16379, "", false, 0, 5000);
        assert_eq!(Duration::from_millis(5000), settings.timeout());
        assert!(settings.client().is_ok());
        assert_eq!("127.0.0.1:16379", settings.endpoint());
        assert_eq!(
            Duration::from_millis(1),
            RedisConfig::new("127.0.0.1", 6379, "", false, 0, 0).timeout()
        );
        assert_eq!(250, settings.with_timeout(250).timeout_ms());
    }
}
