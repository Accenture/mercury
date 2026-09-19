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

//! The Redis connection seam that hides **standalone vs cluster** from a
//! consumer — Rust port of the Java `RedisBackend` + `RedisBackendFactory`
//! (`StandaloneRedisBackend` / `ClusterRedisBackend`).
//!
//! Java reaches both topologies through one Lettuce command type. The `redis`
//! crate's equivalent is [`redis::aio::ConnectionLike`]: the standalone
//! [`ConnectionManager`] (auto-reconnecting), the plain
//! [`MultiplexedConnection`] and the cluster-aware [`ClusterConnection`] all
//! implement it, so [`RedisConnection`] wraps the three and every command is
//! issued through one [`RedisBackend::query`] with no per-topology branching
//! in the caller. `MGET`, the cache's only multi-key op, is routed per slot by
//! the cluster client itself.
//!
//! **Selection is the Java two-key scheme:** when
//! [`RedisConfig::auto_detect_cluster`] (`cluster.detect=auto`) a dedicated
//! short-lived connection runs `INFO cluster` once and `cluster_enabled:1`
//! means cluster; otherwise the boolean [`RedisConfig::cluster_enabled`]
//! (`cluster.mode`) decides. An inconclusive probe (unreachable, `INFO`
//! restricted) resolves to that same boolean — the caller's own connection
//! then surfaces a genuine outage, and the health probe's waiting semantics
//! still apply. Credentials and TLS ride in the config for both topologies.
//!
//! One backend = **one long-lived, multiplexed connection**; the `redis` crate
//! pipelines any number of concurrent callers over it and correlates the
//! ordered replies, so no connection pool is needed for a non-blocking,
//! transaction-free command set (the Java design spec §4.4).
//!
//! Deliberate delta (port spec §4): the Java backend is generic in the value
//! codec (`String` vs `byte[]`); the `redis` crate is codec-free, so typing
//! lives at the call site (`query::<Vec<u8>>`, `query::<String>`).

use std::future::Future;
use std::time::Duration;

use platform_core::AppError;
use redis::aio::{ConnectionLike, ConnectionManager, MultiplexedConnection};
use redis::cluster::ClusterClient;
use redis::cluster_async::ClusterConnection;
use redis::{Cmd, FromRedisValue, Pipeline, RedisFuture, Value};

use crate::config::RedisConfig;

/// The three connection shapes behind one command seam. Cheap to clone —
/// each is a handle onto a shared connection task.
#[derive(Clone)]
pub enum RedisConnection {
    /// A plain multiplexed connection (the health probe: a failed probe drops
    /// it and the next probe rebuilds from re-resolved configuration).
    Multiplexed(MultiplexedConnection),
    /// The auto-reconnecting manager (long-lived consumers such as the cache).
    Managed(ConnectionManager),
    /// The cluster-aware connection (`MOVED`/`ASK` redirects and multi-slot
    /// routing handled by the client).
    Cluster(ClusterConnection),
}

impl ConnectionLike for RedisConnection {
    fn req_packed_command<'a>(&'a mut self, cmd: &'a Cmd) -> RedisFuture<'a, Value> {
        match self {
            RedisConnection::Multiplexed(c) => c.req_packed_command(cmd),
            RedisConnection::Managed(c) => c.req_packed_command(cmd),
            RedisConnection::Cluster(c) => c.req_packed_command(cmd),
        }
    }

    fn req_packed_commands<'a>(
        &'a mut self,
        cmd: &'a Pipeline,
        offset: usize,
        count: usize,
    ) -> RedisFuture<'a, Vec<Value>> {
        match self {
            RedisConnection::Multiplexed(c) => c.req_packed_commands(cmd, offset, count),
            RedisConnection::Managed(c) => c.req_packed_commands(cmd, offset, count),
            RedisConnection::Cluster(c) => c.req_packed_commands(cmd, offset, count),
        }
    }

    fn get_db(&self) -> i64 {
        match self {
            RedisConnection::Multiplexed(c) => c.get_db(),
            RedisConnection::Managed(c) => c.get_db(),
            RedisConnection::Cluster(c) => c.get_db(),
        }
    }
}

/// Why a connect attempt failed, kept apart so the health probe can classify
/// it against its waiting-vs-outage boundary (Java: an `IllegalArgument` /
/// `IllegalState` while mapping the values is "unbuildable", a Redis error is
/// the server's answer, a timeout is an outage).
#[derive(Debug)]
pub enum ConnectError {
    /// The client cannot even be built from the resolved values (e.g. an
    /// unresolved placeholder) — the not-yet-usable-configuration signature.
    Unbuildable(String),
    /// The server (or the network) answered: refused, rejected credentials…
    Redis(redis::RedisError),
    /// No answer within the configured timeout.
    TimedOut(Duration),
}

impl std::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectError::Unbuildable(cause) => write!(f, "{cause}"),
            ConnectError::Redis(error) => write!(f, "{error}"),
            ConnectError::TimedOut(timeout) => {
                write!(f, "connect timed out after {} ms", timeout.as_millis())
            }
        }
    }
}

impl std::error::Error for ConnectError {}

impl From<ConnectError> for AppError {
    fn from(error: ConnectError) -> Self {
        AppError::new(500, format!("Redis unavailable - {error}"))
    }
}

/// One topology-agnostic Redis backend: the connection, what it is, and the
/// per-command deadline (Java `RedisBackend`, built by `RedisBackendFactory`).
#[derive(Clone)]
pub struct RedisBackend {
    connection: RedisConnection,
    cluster: bool,
    timeout: Duration,
    endpoint: String,
}

impl RedisBackend {
    /// Build a **long-lived** backend for `config` — the auto-reconnecting
    /// manager on a standalone server, the cluster connection on a cluster —
    /// choosing the topology by the two-key scheme (Java
    /// `RedisBackendFactory.create`).
    pub async fn connect(config: &RedisConfig) -> Result<Self, ConnectError> {
        let cluster = Self::resolve_cluster(config).await;
        let connection = if cluster {
            RedisConnection::Cluster(Self::cluster_connection(config).await?)
        } else {
            let client = Self::client(config)?;
            RedisConnection::Managed(
                with_timeout(config.timeout(), client.get_connection_manager()).await?,
            )
        };
        Ok(Self::new(connection, cluster, config))
    }

    /// Build a **single, non-reconnecting** connection for `config` — the
    /// health probe's shape: one connection whose failure is reported rather
    /// than healed underneath, dropped and rebuilt from re-resolved
    /// configuration by the next probe. A cluster still gets the cluster
    /// connection (there is no lighter cluster shape).
    pub async fn connect_once(config: &RedisConfig) -> Result<Self, ConnectError> {
        let cluster = Self::resolve_cluster(config).await;
        let connection = if cluster {
            RedisConnection::Cluster(Self::cluster_connection(config).await?)
        } else {
            let client = Self::client(config)?;
            RedisConnection::Multiplexed(
                with_timeout(config.timeout(), client.get_multiplexed_async_connection()).await?,
            )
        };
        Ok(Self::new(connection, cluster, config))
    }

    /// Probe the seed node to decide whether it is a cluster (Java
    /// `RedisBackendFactory.detectCluster`): a dedicated short-lived connection
    /// runs `INFO cluster`; `cluster_enabled:1` means cluster. Any failure —
    /// unreachable, `INFO` not permitted, credentials not yet landed — is
    /// inconclusive and resolves to `fallback` (the configured `cluster.mode`
    /// boolean).
    pub async fn detect_cluster(config: &RedisConfig, fallback: bool) -> bool {
        let probe = async {
            let client = config.client()?;
            let mut connection = client
                .get_multiplexed_async_connection()
                .await
                .map_err(|e| AppError::new(500, e.to_string()))?;
            let info: String = redis::cmd("INFO")
                .arg("cluster")
                .query_async(&mut connection)
                .await
                .map_err(|e| AppError::new(500, e.to_string()))?;
            Ok::<bool, AppError>(info.contains("cluster_enabled:1"))
        };
        match tokio::time::timeout(config.timeout(), probe).await {
            Ok(Ok(cluster)) => {
                log::debug!(
                    "Redis auto-detect at {} -> {}",
                    config.endpoint(),
                    if cluster { "cluster" } else { "standalone" }
                );
                cluster
            }
            Ok(Err(e)) => {
                log::debug!(
                    "Redis cluster auto-detect at {} inconclusive ({}); using configured cluster.mode={fallback}",
                    config.endpoint(),
                    e.message()
                );
                fallback
            }
            Err(_) => {
                log::debug!(
                    "Redis cluster auto-detect at {} timed out; using configured cluster.mode={fallback}",
                    config.endpoint()
                );
                fallback
            }
        }
    }

    /// `true` when this backend talks to a Redis Cluster (start-up logging /
    /// diagnostics).
    pub fn cluster(&self) -> bool {
        self.cluster
    }

    /// The configured `host:port` (or the first seed).
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// The per-command deadline this backend applies in [`query`](Self::query).
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// A handle onto the shared connection for callers that drive the `redis`
    /// crate's API directly (`cmd.query_async(&mut connection)`).
    pub fn connection(&self) -> RedisConnection {
        self.connection.clone()
    }

    /// Issue one command over the shared connection, bounded by the configured
    /// timeout; a Redis error or a timeout surfaces as an `AppError` (500) so a
    /// broken store fails the caller loudly instead of hanging.
    pub async fn query<T: FromRedisValue>(&self, cmd: &Cmd) -> Result<T, AppError> {
        let mut connection = self.connection.clone();
        with_deadline(self.timeout, cmd.query_async::<T>(&mut connection)).await
    }

    /// Issue a pipeline (a plain batch, or an atomic `MULTI`/`EXEC` block when
    /// built with `.atomic()`) over the shared connection — one round trip, the
    /// replies awaited together (the Java "fire without waiting, then await
    /// all" shape of a pipelined `MPUT`).
    pub async fn query_pipeline<T: FromRedisValue>(&self, pipe: &Pipeline) -> Result<T, AppError> {
        let mut connection = self.connection.clone();
        with_deadline(self.timeout, pipe.query_async::<T>(&mut connection)).await
    }

    /// One `PING` round trip — proves connectivity, TLS and authentication in a
    /// single request. The raw failure is returned so the health probe can
    /// classify it (a rejected credential is "waiting", a refused connection an
    /// outage).
    pub async fn ping(&self) -> Result<(), ConnectError> {
        let mut connection = self.connection.clone();
        with_timeout(
            self.timeout,
            redis::cmd("PING").query_async::<()>(&mut connection),
        )
        .await
    }

    fn new(connection: RedisConnection, cluster: bool, config: &RedisConfig) -> Self {
        RedisBackend {
            connection,
            cluster,
            timeout: config.timeout(),
            endpoint: config.endpoint(),
        }
    }

    async fn resolve_cluster(config: &RedisConfig) -> bool {
        if config.auto_detect_cluster() {
            Self::detect_cluster(config, config.cluster_enabled()).await
        } else {
            config.cluster_enabled()
        }
    }

    fn client(config: &RedisConfig) -> Result<redis::Client, ConnectError> {
        config
            .client()
            .map_err(|e| ConnectError::Unbuildable(e.message().to_string()))
    }

    async fn cluster_connection(config: &RedisConfig) -> Result<ClusterConnection, ConnectError> {
        let seeds = config
            .seed_infos()
            .map_err(|e| ConnectError::Unbuildable(e.message().to_string()))?;
        let client = ClusterClient::new(seeds).map_err(ConnectError::Redis)?;
        with_timeout(config.timeout(), client.get_async_connection()).await
    }
}

async fn with_timeout<T>(
    timeout: Duration,
    future: impl Future<Output = Result<T, redis::RedisError>>,
) -> Result<T, ConnectError> {
    match tokio::time::timeout(timeout, future).await {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => Err(ConnectError::Redis(error)),
        Err(_) => Err(ConnectError::TimedOut(timeout)),
    }
}

async fn with_deadline<T>(
    timeout: Duration,
    future: impl Future<Output = Result<T, redis::RedisError>>,
) -> Result<T, AppError> {
    match tokio::time::timeout(timeout, future).await {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => Err(AppError::new(500, format!("Redis error - {error}"))),
        Err(_) => Err(AppError::new(
            500,
            format!("Redis request timed out after {}ms", timeout.as_millis()),
        )),
    }
}
