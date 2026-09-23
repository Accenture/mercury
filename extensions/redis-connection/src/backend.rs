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
//!
//! # Lifecycle — the restart-aware retry
//!
//! Lettuce (Java) requeues commands it has not yet written across a reconnect,
//! so the first command after a Redis restart simply works there. The `redis`
//! crate's [`ConnectionManager`] arms an asynchronous reconnect when a command
//! fails but returns that command's error to the caller — so on this engine the
//! first command after a restart used to fail (`broken pipe`) and the second
//! heal. The maintainer's ruling (2026-09-22): retry **intelligently** — only
//! when the failure is a Redis restart or reconnection — which takes some simple
//! lifecycle monitoring. Two pieces, both on the standalone (managed) connection:
//!
//! - **The heartbeat monitor** ([`RedisConfig::heartbeat`], `heartbeat.ms`,
//!   default 1 s, `0` = off): one `PING` per interval. A lost connection is
//!   noticed within one interval — the failed `PING` is what makes the manager
//!   reconnect *eagerly*, so a command issued after that awaits the fresh
//!   connection and succeeds on its first attempt (the producer's first `RPUSH`
//!   after a restart, the note-3 symptom, heals before it is sent). The monitor
//!   flips [`ConnectionLifecycle::healthy`] and logs the loss and the recovery
//!   once each.
//! - **One retry per transition, idempotent commands only.** A command that fails
//!   with a connection-loss error *and was issued while the connection was
//!   believed healthy* is the restart itself: an idempotent command
//!   ([`RedisBackend::query_idempotent`], the caller's declaration) is retried
//!   exactly once, awaiting the manager's swapped-in reconnection future under
//!   its own deadline. A command issued while the connection is already known to
//!   be down makes ONE attempt, bounded by the command deadline (408 while the
//!   manager is still trying to reconnect, 503 on an outright refusal) — never a
//!   second one, so an outage never doubles the deadline. A non-idempotent
//!   command is never replayed: on RESP2 the crate
//!   reports `broken pipe` both for a command it never sent and for one whose
//!   reply was lost (`closed_connection_error`), so non-delivery cannot be
//!   proven, and replaying an ambiguous `RPUSH` risks a duplicate — the
//!   heartbeat is what heals those ahead of the next call. A timeout is never a
//!   lifecycle signal (408, no retry), nor is anything the server answered.

use std::fmt::Display;
use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::time::{Duration, Instant};

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
    /// A connect failure on a caller's path (the cache runtime's lazy build):
    /// a refused, dropped or unanswered connection is **503** `Redis
    /// unavailable` — the `redis.health` vocabulary — while a configuration
    /// that cannot build a client at all is a defect, not an outage (500).
    fn from(error: ConnectError) -> Self {
        let status = match &error {
            ConnectError::Unbuildable(_) => 500,
            ConnectError::Redis(_) | ConnectError::TimedOut(_) => 503,
        };
        AppError::new(status, format!("Redis unavailable - {error}"))
    }
}

/// Classify a failed command the way the Java `RedisFailure.classify` does, so
/// a caller — or a flow's / graph's exception handler, which passes the status
/// through — sees the failure for what it is: a timeout is **408**, a refused,
/// dropped or otherwise unreachable connection (and a cluster that cannot
/// route) is **503** `Redis unavailable - …`, and anything the server answered
/// (a wrong type, an unknown command, a rejected credential) stays **500**.
pub fn classify_command_error(error: &redis::RedisError) -> AppError {
    if error.is_timeout() {
        AppError::new(408, format!("Redis request timed out - {error}"))
    } else if error.is_connection_refusal()
        || error.is_connection_dropped()
        || error.is_io_error()
        || error.is_cluster_error()
    {
        AppError::new(503, format!("Redis unavailable - {error}"))
    } else {
        AppError::new(500, format!("Redis error - {error}"))
    }
}

/// The per-command deadline expired without an answer: **408**, the same
/// status the platform gives an RPC timeout.
pub fn command_timeout(timeout: Duration) -> AppError {
    AppError::new(
        408,
        format!("Redis request timed out after {}ms", timeout.as_millis()),
    )
}

/// Is this failure the connection going away (the manager's own reconnect
/// trigger, plus the dropped/refused shapes), as opposed to a timeout or an
/// answer from the server?
pub fn is_connection_loss(error: &redis::RedisError) -> bool {
    !error.is_timeout()
        && (error.is_unrecoverable_error()
            || error.is_io_error()
            || error.is_connection_dropped()
            || error.is_connection_refusal())
}

/// The caller's declaration of whether a command may be replayed after a lost
/// connection (module docs: *Lifecycle*).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Replay {
    /// Safe to run twice (`GET`, `SETEX`, `DEL`, `LLEN`, `MGET`…): retried once
    /// when the loss is the restart itself.
    Idempotent,
    /// Never replayed (`RPUSH`, `LPOP`, `SET NX`…): the heartbeat heals the
    /// connection ahead of the next call instead.
    NotIdempotent,
}

/// A failed command, before classification — for a consumer that keeps its own
/// status mapping (the sync-over-async store); [`CommandError::classify`] is the
/// 408/503/500 mapping [`RedisBackend::query`] applies.
#[derive(Debug)]
pub enum CommandError {
    /// No answer within the per-command deadline.
    TimedOut(Duration),
    /// The `redis` crate's own error — a lost connection or a server answer.
    Redis(redis::RedisError),
}

impl CommandError {
    /// Java `RedisFailure.classify`: 408 for a timeout, 503 for an unreachable
    /// Redis, 500 for a server answer (see [`classify_command_error`]).
    pub fn classify(self) -> AppError {
        match self {
            CommandError::TimedOut(timeout) => command_timeout(timeout),
            CommandError::Redis(error) => classify_command_error(&error),
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::TimedOut(timeout) => {
                write!(
                    f,
                    "Redis command timed out after {} ms",
                    timeout.as_millis()
                )
            }
            CommandError::Redis(error) => write!(f, "{error}"),
        }
    }
}

/// What the backend currently believes about its connection, maintained by the
/// heartbeat monitor and by every command's outcome (module docs: *Lifecycle*).
/// Shared by all clones of one [`RedisBackend`]; the counters are for tests,
/// health reporting and operators reading a recovery.
pub struct ConnectionLifecycle {
    endpoint: String,
    healthy: AtomicBool,
    drops: AtomicU64,
    retries: AtomicU64,
    recoveries: AtomicU64,
    lost_at: Mutex<Option<Instant>>,
}

impl ConnectionLifecycle {
    fn new(endpoint: String) -> Self {
        ConnectionLifecycle {
            endpoint,
            healthy: AtomicBool::new(true),
            drops: AtomicU64::new(0),
            retries: AtomicU64::new(0),
            recoveries: AtomicU64::new(0),
            lost_at: Mutex::new(None),
        }
    }

    /// `true` while the connection is believed up — a lost connection that no
    /// command or heartbeat has seen restored yet reads `false`.
    pub fn healthy(&self) -> bool {
        self.healthy.load(Ordering::Acquire)
    }

    /// Transitions from healthy to lost observed so far.
    pub fn drops(&self) -> u64 {
        self.drops.load(Ordering::Relaxed)
    }

    /// Second attempts spent on idempotent commands.
    pub fn retries(&self) -> u64 {
        self.retries.load(Ordering::Relaxed)
    }

    /// Transitions from lost back to healthy.
    pub fn recoveries(&self) -> u64 {
        self.recoveries.load(Ordering::Relaxed)
    }

    /// A connection-loss failure was observed. Returns whether the connection
    /// was believed healthy until now — i.e. whether this failure IS the
    /// transition (logged once per transition).
    fn mark_lost(&self, cause: &dyn Display) -> bool {
        let was_healthy = self.healthy.swap(false, Ordering::AcqRel);
        if was_healthy {
            self.drops.fetch_add(1, Ordering::Relaxed);
            *self.lost_at.lock().expect("lifecycle") = Some(Instant::now());
            log::warn!(
                "Redis connection to {} lost - {cause}; reconnecting",
                self.endpoint
            );
        }
        was_healthy
    }

    /// A command or heartbeat succeeded: healthy again (logged once per
    /// recovery, with how long the connection was gone).
    fn mark_alive(&self) {
        if !self.healthy.swap(true, Ordering::AcqRel) {
            self.recoveries.fetch_add(1, Ordering::Relaxed);
            let gone = self.lost_at.lock().expect("lifecycle").take();
            match gone {
                Some(since) => log::info!(
                    "Redis connection to {} restored after {} ms",
                    self.endpoint,
                    since.elapsed().as_millis()
                ),
                None => log::info!("Redis connection to {} restored", self.endpoint),
            }
        }
    }

    fn count_retry(&self) {
        self.retries.fetch_add(1, Ordering::Relaxed);
    }
}

/// One topology-agnostic Redis backend: the connection, what it is, the
/// per-command deadline and the connection lifecycle (Java `RedisBackend`,
/// built by `RedisBackendFactory`).
#[derive(Clone)]
pub struct RedisBackend {
    connection: RedisConnection,
    cluster: bool,
    timeout: Duration,
    endpoint: String,
    lifecycle: Arc<ConnectionLifecycle>,
}

impl RedisBackend {
    /// Build a **long-lived** backend for `config` — the auto-reconnecting
    /// manager on a standalone server, the cluster connection on a cluster —
    /// choosing the topology by the two-key scheme (Java
    /// `RedisBackendFactory.create`).
    pub async fn connect(config: &RedisConfig) -> Result<Self, ConnectError> {
        let cluster = Self::resolve_cluster(config).await;
        if cluster {
            let connection = RedisConnection::Cluster(Self::cluster_connection(config).await?);
            return Ok(Self::new(connection, true, config));
        }
        Self::connect_standalone(config).await
    }

    /// Build a **long-lived standalone** backend — the auto-reconnecting manager
    /// plus the heartbeat monitor — without consulting the cluster keys. For a
    /// consumer whose command set is not cluster-safe (the sync-over-async
    /// store's two-key `DEL`), where the cluster parity is a separate item.
    pub async fn connect_standalone(config: &RedisConfig) -> Result<Self, ConnectError> {
        let client = Self::client(config)?;
        let manager = with_timeout(config.timeout(), client.get_connection_manager()).await?;
        let backend = Self::new(RedisConnection::Managed(manager), false, config);
        if let Some(interval) = config.heartbeat() {
            backend.start_heartbeat(interval);
        }
        Ok(backend)
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
    /// timeout; a failure surfaces as a classified `AppError` — 408 for a
    /// timeout, 503 for an unreachable Redis, 500 for a server answer (see
    /// [`classify_command_error`]) — so a broken store fails the caller for
    /// what it is instead of hanging or hiding behind a generic 500.
    pub async fn query<T: FromRedisValue>(&self, cmd: &Cmd) -> Result<T, AppError> {
        self.attempt(Replay::NotIdempotent, || {
            let mut connection = self.connection.clone();
            async move { cmd.query_async::<T>(&mut connection).await }
        })
        .await
        .map_err(CommandError::classify)
    }

    /// [`RedisBackend::query`] for a command that is safe to run twice: after a
    /// lost connection that was believed healthy when the command was issued, it
    /// is retried exactly once (module docs: *Lifecycle*).
    pub async fn query_idempotent<T: FromRedisValue>(&self, cmd: &Cmd) -> Result<T, AppError> {
        self.attempt(Replay::Idempotent, || {
            let mut connection = self.connection.clone();
            async move { cmd.query_async::<T>(&mut connection).await }
        })
        .await
        .map_err(CommandError::classify)
    }

    /// Issue a pipeline (a plain batch, or an atomic `MULTI`/`EXEC` block when
    /// built with `.atomic()`) over the shared connection — one round trip, the
    /// replies awaited together (the Java "fire without waiting, then await
    /// all" shape of a pipelined `MPUT`).
    pub async fn query_pipeline<T: FromRedisValue>(&self, pipe: &Pipeline) -> Result<T, AppError> {
        self.attempt(Replay::NotIdempotent, || {
            let mut connection = self.connection.clone();
            async move { pipe.query_async::<T>(&mut connection).await }
        })
        .await
        .map_err(CommandError::classify)
    }

    /// [`RedisBackend::query_pipeline`] for a batch that is safe to run twice
    /// (e.g. the cache's `MPUT`, pipelined `SETEX`es).
    pub async fn query_pipeline_idempotent<T: FromRedisValue>(
        &self,
        pipe: &Pipeline,
    ) -> Result<T, AppError> {
        self.attempt(Replay::Idempotent, || {
            let mut connection = self.connection.clone();
            async move { pipe.query_async::<T>(&mut connection).await }
        })
        .await
        .map_err(CommandError::classify)
    }

    /// The connection lifecycle shared by every clone of this backend.
    pub fn lifecycle(&self) -> &ConnectionLifecycle {
        &self.lifecycle
    }

    /// Run one operation with the lifecycle-aware retry and the per-command
    /// deadline, returning the raw failure — for a consumer that keeps its own
    /// status mapping; [`RedisBackend::query`] is this plus
    /// [`CommandError::classify`]. `operation` builds a fresh future per attempt
    /// (it may run twice for [`Replay::Idempotent`]).
    pub async fn attempt<T, F, Fut>(&self, replay: Replay, operation: F) -> Result<T, CommandError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = redis::RedisResult<T>>,
    {
        // the belief at issue time decides: every command in flight when the
        // connection goes is the transition and gets its retry; a command issued
        // while the connection is already known down makes one attempt
        let issued_healthy = self.lifecycle.healthy();
        let error = match tokio::time::timeout(self.timeout, operation()).await {
            Ok(Ok(value)) => {
                self.lifecycle.mark_alive();
                return Ok(value);
            }
            // a timeout is not a lifecycle signal: no retry, no health change
            Err(_) => return Err(CommandError::TimedOut(self.timeout)),
            Ok(Err(error)) => error,
        };
        if !is_connection_loss(&error) {
            return Err(CommandError::Redis(error));
        }
        self.lifecycle.mark_lost(&error);
        let managed = matches!(self.connection, RedisConnection::Managed(_));
        if !(managed && issued_healthy && replay == Replay::Idempotent) {
            return Err(CommandError::Redis(error));
        }
        self.lifecycle.count_retry();
        log::info!(
            "Retrying an idempotent Redis command once after the lost connection to {}",
            self.endpoint
        );
        // the manager swapped in its reconnection future when the first attempt
        // failed - this attempt awaits it rather than racing it
        match tokio::time::timeout(self.timeout, operation()).await {
            Ok(Ok(value)) => {
                self.lifecycle.mark_alive();
                Ok(value)
            }
            Ok(Err(error)) => Err(CommandError::Redis(error)),
            Err(_) => Err(CommandError::TimedOut(self.timeout)),
        }
    }

    /// The heartbeat monitor (module docs: *Lifecycle*): one `PING` per
    /// interval on the managed connection. Holds only a weak reference to the
    /// lifecycle, so it ends with the last clone of the backend.
    fn start_heartbeat(&self, interval: Duration) {
        let connection = self.connection.clone();
        let timeout = self.timeout;
        let lifecycle = Arc::downgrade(&self.lifecycle);
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            ticker.tick().await; // the first tick completes at once
            loop {
                ticker.tick().await;
                let Some(lifecycle) = Weak::upgrade(&lifecycle) else {
                    return;
                };
                let mut connection = connection.clone();
                let ping = redis::cmd("PING");
                let round_trip = ping.query_async::<String>(&mut connection);
                match tokio::time::timeout(timeout, round_trip).await {
                    Ok(Ok(_)) => lifecycle.mark_alive(),
                    // the failed PING is what makes the manager reconnect
                    Ok(Err(error)) if is_connection_loss(&error) => {
                        lifecycle.mark_lost(&error);
                    }
                    _ => {} // a timeout or a server answer is not a lifecycle signal
                }
            }
        });
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
        let endpoint = config.endpoint();
        RedisBackend {
            connection,
            cluster,
            timeout: config.timeout(),
            lifecycle: Arc::new(ConnectionLifecycle::new(endpoint.clone())),
            endpoint,
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
