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

use std::time::Duration;

use platform_core::{AppConfigReader, AppError};
use redis::aio::ConnectionManager;
use redis::{ConnectionAddr, IntoConnectionInfo, RedisConnectionInfo};
use tokio::sync::OnceCell;

/// One shared namespace so a workflow suspended by one application instance
/// can resume on any other instance sharing the same Redis.
pub(crate) const KEY_PREFIX: &str = "graph:state:";

static MANAGER: OnceCell<ConnectionManager> = OnceCell::const_new();

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
    log::info!("Graph state store connected to redis://{host}:{port}/{database}");
    Ok(manager)
}
