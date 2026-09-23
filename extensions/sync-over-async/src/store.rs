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

//! Redis-backed cross-pod state, keyed by correlation-id — Rust port of the
//! Java `ReturnRouteStore`. The key shapes are **normative** (port spec §4):
//!
//! - **route** `request:{cid}` — the originating pod's return channel (TTL =
//!   rendezvous lifetime: short for a one-shot request, session-scale for a
//!   stream).
//! - **queue** `queue:{cid}` — a Redis List holding the segments, the *source
//!   of truth*, appended (`RPUSH`) before any Pub/Sub notification and drained
//!   destructively (`LPOP`). One store serves both patterns: a one-shot
//!   response is the degenerate stream — a queue whose first entry is terminal.
//!
//! Pub/Sub is only a wake-up signal (see [`crate::coordinator`]); correctness
//! rests on these keys, so a missed notification is recovered by a final drain
//! — on timeout for the one-shot path, at edge idle expiry for a stream. A
//! fully drained list ceases to exist on its own; the TTLs are the crash
//! safety net.
//!
//! # Bounce recovery — the lifecycle-aware retry (port spec §5 item 6)
//!
//! After a Redis restart, the `redis` crate's `ConnectionManager` arms an
//! asynchronous reconnect but returns the failed command's error to the caller
//! (its retry configuration governs *connection attempts*, not command
//! replay), where the Java engine's Lettuce transparently requeues commands it
//! had not yet written. The store therefore runs on the foundation's
//! [`RedisBackend`] and its lifecycle (maintainer's ruling, 2026-09-22 —
//! `redis_connection::backend`, *Lifecycle*): a **heartbeat** notices the lost
//! connection within one interval and makes the manager reconnect eagerly, so
//! the first command after the restart usually finds a fresh connection; and a
//! command that fails while the connection was believed healthy is retried
//! **exactly once** when it is idempotent —
//! `save_route`/`get_route`/`cleanup`/`queue_length` (`SETEX`/`GET`/`DEL`/
//! `LLEN`) — awaiting the swapped-in reconnection future under its own timeout.
//! A command issued while the connection is already known down makes one
//! attempt and fails fast.
//!
//! Deliberately **never replayed**: `append_segment` (replaying an ambiguous
//! `RPUSH` risks a duplicate segment the no-sequence-number design cannot
//! detect — design D7; the crate reports `broken pipe` for a command it never
//! sent and for one whose reply was lost alike, so non-delivery cannot be
//! proven), `pop_segment` (replaying an ambiguous `LPOP` could silently discard
//! the popped segment), and `publish` (wake-ups are best-effort by contract — a
//! lost one is healed by the next drain). A timed-out attempt is never retried
//! either: a timeout is not a connection-death signal, and the caller's
//! deadline is the contract. The store keeps its own status mapping (500), as
//! the Java store does.

use platform_core::AppError;
use redis_connection::{CommandError, ConnectionLifecycle, RedisBackend, Replay};

use crate::connection::RedisSettings;

const ROUTE_PREFIX: &str = "request:";
const QUEUE_PREFIX: &str = "queue:";

/// Handle on the rendezvous key space over one multiplexed connection.
#[derive(Clone)]
pub struct ReturnRouteStore {
    backend: RedisBackend,
}

impl ReturnRouteStore {
    /// Connect the store's own standalone backend — the managed connection plus
    /// the heartbeat monitor — from the `soa.redis.*` settings.
    pub async fn connect(settings: &RedisSettings) -> Result<Self, AppError> {
        Ok(Self::new(RedisBackend::connect_standalone(settings).await?))
    }

    pub fn new(backend: RedisBackend) -> Self {
        ReturnRouteStore { backend }
    }

    /// The connection lifecycle (health, drops, retries, recoveries).
    pub fn lifecycle(&self) -> &ConnectionLifecycle {
        self.backend.lifecycle()
    }

    /// `request:{cid}` — the route key.
    pub fn route_key(business_correlation_id: &str) -> String {
        format!("{ROUTE_PREFIX}{business_correlation_id}")
    }

    /// `queue:{cid}` — the segment queue key.
    pub fn queue_key(business_correlation_id: &str) -> String {
        format!("{QUEUE_PREFIX}{business_correlation_id}")
    }

    pub async fn save_route(
        &self,
        business_correlation_id: &str,
        return_channel: &str,
        ttl_seconds: u64,
    ) -> Result<(), AppError> {
        let key = Self::route_key(business_correlation_id);
        self.run(Replay::Idempotent, || {
            let mut connection = self.backend.connection();
            let key = key.clone();
            let return_channel = return_channel.to_string();
            async move {
                redis::cmd("SETEX")
                    .arg(&key)
                    .arg(ttl_seconds)
                    .arg(&return_channel)
                    .query_async::<()>(&mut connection)
                    .await
            }
        })
        .await
    }

    /// The return channel for this correlation-id, or `None` when it is
    /// absent or expired (an orphan).
    pub async fn get_route(
        &self,
        business_correlation_id: &str,
    ) -> Result<Option<String>, AppError> {
        let key = Self::route_key(business_correlation_id);
        self.run(Replay::Idempotent, || {
            let mut connection = self.backend.connection();
            let key = key.clone();
            async move {
                redis::cmd("GET")
                    .arg(&key)
                    .query_async::<Option<String>>(&mut connection)
                    .await
            }
        })
        .await
    }

    /// Append one serialized segment to the rendezvous queue and refresh the
    /// queue TTL, **atomically** (store-first: call this *before* publishing
    /// the wake-up).
    ///
    /// Java performs this as a two-line Lua script; the Rust port uses a
    /// pipelined `MULTI`/`EXEC` block, which is the same atomic step in one
    /// round trip (port spec §5.1). Two discrete commands would leave a
    /// TTL-less queue key if the client died between them — every key this
    /// module creates must carry a TTL from birth, so abandoned rendezvous
    /// state always ages out.
    pub async fn append_segment(
        &self,
        business_correlation_id: &str,
        segment_json: &str,
        ttl_seconds: u64,
    ) -> Result<(), AppError> {
        let key = Self::queue_key(business_correlation_id);
        self.run(Replay::NotIdempotent, || {
            let mut connection = self.backend.connection();
            let key = key.clone();
            let segment_json = segment_json.to_string();
            async move {
                redis::pipe()
                    .atomic()
                    .cmd("RPUSH")
                    .arg(&key)
                    .arg(&segment_json)
                    .ignore()
                    .cmd("EXPIRE")
                    .arg(&key)
                    .arg(ttl_seconds)
                    .ignore()
                    .query_async::<()>(&mut connection)
                    .await
            }
        })
        .await
    }

    /// Destructively pop the oldest queued segment. The pop is atomic, so
    /// concurrent drains cannot deliver one segment twice; a duplicate
    /// wake-up simply pops nothing.
    pub async fn pop_segment(
        &self,
        business_correlation_id: &str,
    ) -> Result<Option<String>, AppError> {
        let key = Self::queue_key(business_correlation_id);
        self.run(Replay::NotIdempotent, || {
            let mut connection = self.backend.connection();
            let key = key.clone();
            async move {
                redis::cmd("LPOP")
                    .arg(&key)
                    .query_async::<Option<String>>(&mut connection)
                    .await
            }
        })
        .await
    }

    /// The number of queued segments (0 for an absent queue) — used by the
    /// drain's lost-wakeup re-check.
    pub async fn queue_length(&self, business_correlation_id: &str) -> Result<u64, AppError> {
        let key = Self::queue_key(business_correlation_id);
        self.run(Replay::Idempotent, || {
            let mut connection = self.backend.connection();
            let key = key.clone();
            async move {
                redis::cmd("LLEN")
                    .arg(&key)
                    .query_async::<u64>(&mut connection)
                    .await
            }
        })
        .await
    }

    /// Delete both keys for a completed rendezvous. The TTLs are the safety
    /// net for crashes and timeouts; deleting on success frees the keys
    /// immediately instead of waiting them out. The route's disappearance is
    /// also what tells every remaining producer to stop.
    pub async fn cleanup(&self, business_correlation_id: &str) -> Result<(), AppError> {
        let route_key = Self::route_key(business_correlation_id);
        let queue_key = Self::queue_key(business_correlation_id);
        self.run(Replay::Idempotent, || {
            let mut connection = self.backend.connection();
            let route_key = route_key.clone();
            let queue_key = queue_key.clone();
            async move {
                redis::cmd("DEL")
                    .arg(&route_key)
                    .arg(&queue_key)
                    .query_async::<()>(&mut connection)
                    .await
            }
        })
        .await
    }

    /// Publish a bare correlation-id wake-up on a return channel.
    pub async fn publish(
        &self,
        channel: &str,
        business_correlation_id: &str,
    ) -> Result<(), AppError> {
        self.run(Replay::NotIdempotent, || {
            let mut connection = self.backend.connection();
            let channel = channel.to_string();
            let cid = business_correlation_id.to_string();
            async move {
                redis::cmd("PUBLISH")
                    .arg(&channel)
                    .arg(&cid)
                    .query_async::<i64>(&mut connection)
                    .await
            }
        })
        .await
        .map(|_| ())
    }

    /// Run one operation through the foundation's lifecycle-aware retry
    /// (module docs: *Bounce recovery*) — a second attempt only for
    /// [`Replay::Idempotent`], only when the lost connection was believed
    /// healthy when the command was issued — keeping this store's own status
    /// mapping: every failure is a 500 to the rendezvous caller.
    async fn run<T, F, Fut>(&self, replay: Replay, operation: F) -> Result<T, AppError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = redis::RedisResult<T>>,
    {
        self.backend
            .attempt(replay, operation)
            .await
            .map_err(|failure| match failure {
                CommandError::TimedOut(_) => AppError::new(500, "Redis command timed out"),
                CommandError::Redis(error) => AppError::new(500, format!("Redis error - {error}")),
            })
    }
}
