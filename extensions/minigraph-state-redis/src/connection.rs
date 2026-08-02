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

//! Lazily initialized, shared Redis connection for the graph state-store
//! functions — Rust port of the Java `RedisStateConnection` (Lettuce there,
//! the `redis` crate's `ConnectionManager` here: one shared multiplexed
//! connection with automatic reconnection after an outage).
//!
//! The connection is created on first use, so an application that includes
//! this extension boots normally even when Redis is unreachable — the first
//! suspend or resume then fails loudly instead. A failed first connection is
//! not cached: the next call retries.
//!
//! Configuration keys (shared with the sync-over-async family, so an
//! application configures Redis once): `redis.host`, `redis.port`,
//! `redis.password`, `redis.ssl`, `redis.database`, `redis.timeout.ms` — all
//! resolvable through the usual `${ENV_VAR:default}` substitution, e.g.
//! `redis.password=${REDIS_PASSWORD:}`.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use platform_core::{AppConfigReader, AppError};
use redis::aio::ConnectionManager;
use redis::{ConnectionAddr, IntoConnectionInfo, RedisConnectionInfo};
use tokio::sync::OnceCell;

/// One shared namespace so a workflow suspended by one application instance
/// can resume on any other instance sharing the same Redis.
pub(crate) const KEY_PREFIX: &str = "graph:state:";

const REDIS_VERSION: &str = "redis_version:";
const UNKNOWN: &str = "unknown";

static MANAGER: OnceCell<ConnectionManager> = OnceCell::const_new();

/// Whether the connected server supports GETDEL (Redis 6.2+). Enterprise
/// deployments rarely control their managed Redis version (AWS/Azure/GCP),
/// and the redis-standalone Windows binary is 5.0.14 — detected once from
/// `INFO server` when the shared manager is first built. Because the manager
/// reconnects internally, a mid-run failover to a DIFFERENT-version server
/// keeps the detected strategy (the Java port has the same exposure until
/// its connection closes).
static NATIVE_GETDEL: AtomicBool = AtomicBool::new(true);

/// True when the consume strategy is native GETDEL (Redis 6.2+); false
/// selects the equally atomic MULTI/EXEC GET+DEL transaction.
pub(crate) fn native_getdel() -> bool {
    NATIVE_GETDEL.load(Ordering::Relaxed)
}

/// The shared connection manager, connecting on first use (Java
/// `RedisStateConnection.commands()`). `ConnectionManager` is a cheap clone
/// over one multiplexed connection and reconnects automatically.
pub(crate) async fn manager() -> Result<ConnectionManager, AppError> {
    MANAGER
        .get_or_try_init(connect)
        .await
        .cloned()
        .map_err(|e| AppError::new(500, format!("Redis unavailable - {e}")))
}

/// The per-request deadline (`redis.timeout.ms`, default 5000).
pub(crate) fn request_timeout() -> Duration {
    let config = AppConfigReader::get_instance();
    let ms: u64 = config
        .get_property_or("redis.timeout.ms", "5000")
        .trim()
        .parse()
        .unwrap_or(5000);
    Duration::from_millis(ms.max(1))
}

async fn connect() -> Result<ConnectionManager, redis::RedisError> {
    let config = AppConfigReader::get_instance();
    let host = config.get_property_or("redis.host", "127.0.0.1");
    let port: u16 = config
        .get_property_or("redis.port", "6379")
        .trim()
        .parse()
        .unwrap_or(6379);
    let password = config.get_property_or("redis.password", "");
    let ssl = config
        .get_property_or("redis.ssl", "false")
        .eq_ignore_ascii_case("true");
    let database: i64 = config
        .get_property_or("redis.database", "0")
        .trim()
        .parse()
        .unwrap_or(0);
    let addr = if ssl {
        ConnectionAddr::TcpTls {
            host: host.clone(),
            port,
            insecure: false,
            tls_params: None,
        }
    } else {
        ConnectionAddr::Tcp(host.clone(), port)
    };
    let mut redis_settings = RedisConnectionInfo::default().set_db(database);
    if !password.trim().is_empty() {
        redis_settings = redis_settings.set_password(&password);
    }
    let info = addr
        .into_connection_info()?
        .set_redis_settings(redis_settings);
    let client = redis::Client::open(info)?;
    // establishes the first connection now, so an unreachable Redis fails
    // loudly on the first suspend/resume instead of hanging silently
    let manager = tokio::time::timeout(request_timeout(), client.get_connection_manager())
        .await
        .map_err(|_| {
            redis::RedisError::from(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "connection timed out",
            ))
        })??;
    // detect the consume strategy from the server version - an undetectable
    // version selects the transactional fallback, which works everywhere
    let mut probe = manager.clone();
    let version = match tokio::time::timeout(
        request_timeout(),
        redis::cmd("INFO")
            .arg("server")
            .query_async::<String>(&mut probe),
    )
    .await
    {
        Ok(Ok(info)) => redis_version(&info),
        _ => UNKNOWN.to_string(),
    };
    let getdel = supports_getdel(&version);
    NATIVE_GETDEL.store(getdel, Ordering::Relaxed);
    log::info!(
        "Graph state store connected to redis://{host}:{port}/{database} (Redis {version}, consume via {})",
        if getdel { "GETDEL" } else { "transactional GET+DEL" }
    );
    Ok(manager)
}

/// Extract the server version from `INFO server` output, or "unknown" when
/// absent (Java `RedisStateConnection.redisVersion`).
fn redis_version(server_info: &str) -> String {
    for line in server_info.lines() {
        if let Some(version) = line.strip_prefix(REDIS_VERSION) {
            return version.trim().to_string();
        }
    }
    UNKNOWN.to_string()
}

/// GETDEL needs Redis 6.2 or later; an unparseable version selects the
/// transactional fallback, which works on every server (Java
/// `RedisStateConnection.supportsGetdel`).
fn supports_getdel(version: &str) -> bool {
    let mut parts = version.split('.');
    let (Some(major), Some(minor)) = (parts.next(), parts.next()) else {
        return false;
    };
    match (major.parse::<i32>(), minor.parse::<i32>()) {
        (Ok(major), Ok(minor)) => major > 6 || (major == 6 && minor >= 2),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{redis_version, supports_getdel};

    #[test]
    fn version_is_extracted_from_info_server_output() {
        let info = "# Server\r\nredis_version:6.2.7\r\nredis_git_sha1:0\r\n";
        assert_eq!("6.2.7", redis_version(info));
        assert_eq!("5.0.14.1", redis_version("redis_version:5.0.14.1\n"));
        assert_eq!("unknown", redis_version("# Server\nos:Linux\n"));
        assert_eq!("unknown", redis_version(""));
    }

    #[test]
    fn getdel_needs_redis_6_2_or_later() {
        for version in ["6.2.0", "6.2.7", "7.4.1", "8.0"] {
            assert!(supports_getdel(version), "{version} must use GETDEL");
        }
        for version in ["6.0.9", "3.0.504", "unknown", "x.y.z", "7"] {
            assert!(!supports_getdel(version), "{version} must use the fallback");
        }
        // the Windows community binary reports four segments - major 5 fails the gate
        assert!(!supports_getdel("5.0.14.1"));
    }
}
