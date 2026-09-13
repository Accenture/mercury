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

//! The discrete `redis.*` startup parameters — Rust port of the Java
//! `RedisConfig`, and deliberately the **same key family** the
//! `minigraph-state-redis` extension reads, so an application configures
//! Redis once and one health probe covers either or both.
//!
//! Clients are built lazily from live configuration, never frozen at
//! construction time: the Java module learned that a credential published by
//! a start-up vault bootstrap has not landed while functions are being
//! constructed.

use std::time::Duration;

use platform_core::{AppConfigReader, AppError};
use redis::aio::ConnectionManager;
use redis::{ConnectionAddr, IntoConnectionInfo, RedisConnectionInfo};

/// Connection parameters for the rendezvous store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedisSettings {
    host: String,
    port: u16,
    password: String,
    ssl: bool,
    database: i64,
    timeout_ms: u64,
}

impl Default for RedisSettings {
    fn default() -> Self {
        RedisSettings {
            host: "127.0.0.1".to_string(),
            port: 6379,
            password: String::new(),
            ssl: false,
            database: 0,
            timeout_ms: 5000,
        }
    }
}

impl RedisSettings {
    /// Read the `redis.*` family from application configuration (Java
    /// `RedisConfig.from`). Unset or unparseable values fall back to
    /// [`Default`].
    pub fn from_config() -> Self {
        let config = AppConfigReader::get_instance();
        let defaults = RedisSettings::default();
        RedisSettings {
            host: config.get_property_or("redis.host", &defaults.host),
            port: config
                .get_property_or("redis.port", "6379")
                .trim()
                .parse()
                .unwrap_or(defaults.port),
            password: config.get_property_or("redis.password", ""),
            ssl: config
                .get_property_or("redis.ssl", "false")
                .eq_ignore_ascii_case("true"),
            database: config
                .get_property_or("redis.database", "0")
                .trim()
                .parse()
                .unwrap_or(defaults.database),
            timeout_ms: config
                .get_property_or("redis.timeout.ms", "5000")
                .trim()
                .parse()
                .unwrap_or(defaults.timeout_ms),
        }
    }

    /// Explicit constructor for tests and embedders (Java's discrete-parameter
    /// `RedisConfig` constructor).
    pub fn new(
        host: impl Into<String>,
        port: u16,
        password: impl Into<String>,
        ssl: bool,
        database: i64,
        timeout_ms: u64,
    ) -> Self {
        RedisSettings {
            host: host.into(),
            port,
            password: password.into(),
            ssl,
            database,
            timeout_ms,
        }
    }

    /// The per-request deadline (`redis.timeout.ms`), at least 1 ms.
    pub fn timeout(&self) -> Duration {
        Duration::from_millis(self.timeout_ms.max(1))
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    /// Build a client. Pub/Sub needs its own dedicated connection, so the
    /// client — not only a multiplexed manager — is the unit this module
    /// keeps (see `coordinator`).
    pub fn client(&self) -> Result<redis::Client, AppError> {
        let addr = if self.ssl {
            ConnectionAddr::TcpTls {
                host: self.host.clone(),
                port: self.port,
                insecure: false,
                tls_params: None,
            }
        } else {
            ConnectionAddr::Tcp(self.host.clone(), self.port)
        };
        let mut redis_settings = RedisConnectionInfo::default().set_db(self.database);
        if !self.password.trim().is_empty() {
            redis_settings = redis_settings.set_password(&self.password);
        }
        let info = addr
            .into_connection_info()
            .map_err(|e| AppError::new(500, format!("Invalid Redis address - {e}")))?
            .set_redis_settings(redis_settings);
        redis::Client::open(info)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_settings_build_a_client() {
        let settings = RedisSettings::new("127.0.0.1", 16379, "", false, 0, 5000);
        assert_eq!(Duration::from_millis(5000), settings.timeout());
        assert!(settings.client().is_ok());
    }

    #[test]
    fn a_zero_timeout_is_clamped_to_one_millisecond() {
        let settings = RedisSettings::new("127.0.0.1", 6379, "", false, 0, 0);
        assert_eq!(Duration::from_millis(1), settings.timeout());
    }
}
