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

use std::time::Duration;

use platform_core::AppError;
use redis::aio::ConnectionManager;

const ROUTE_PREFIX: &str = "request:";
const QUEUE_PREFIX: &str = "queue:";

/// Handle on the rendezvous key space over one multiplexed connection.
#[derive(Clone)]
pub struct ReturnRouteStore {
    connection: ConnectionManager,
    timeout: Duration,
}

impl ReturnRouteStore {
    pub fn new(connection: ConnectionManager, timeout: Duration) -> Self {
        ReturnRouteStore {
            connection,
            timeout,
        }
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
        let mut connection = self.connection.clone();
        self.run(
            redis::cmd("SETEX")
                .arg(Self::route_key(business_correlation_id))
                .arg(ttl_seconds)
                .arg(return_channel)
                .query_async::<()>(&mut connection),
        )
        .await
    }

    /// The return channel for this correlation-id, or `None` when it is
    /// absent or expired (an orphan).
    pub async fn get_route(
        &self,
        business_correlation_id: &str,
    ) -> Result<Option<String>, AppError> {
        let mut connection = self.connection.clone();
        self.run(
            redis::cmd("GET")
                .arg(Self::route_key(business_correlation_id))
                .query_async::<Option<String>>(&mut connection),
        )
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
        let mut connection = self.connection.clone();
        let key = Self::queue_key(business_correlation_id);
        self.run(
            redis::pipe()
                .atomic()
                .cmd("RPUSH")
                .arg(&key)
                .arg(segment_json)
                .ignore()
                .cmd("EXPIRE")
                .arg(&key)
                .arg(ttl_seconds)
                .ignore()
                .query_async::<()>(&mut connection),
        )
        .await
    }

    /// Destructively pop the oldest queued segment. The pop is atomic, so
    /// concurrent drains cannot deliver one segment twice; a duplicate
    /// wake-up simply pops nothing.
    pub async fn pop_segment(
        &self,
        business_correlation_id: &str,
    ) -> Result<Option<String>, AppError> {
        let mut connection = self.connection.clone();
        self.run(
            redis::cmd("LPOP")
                .arg(Self::queue_key(business_correlation_id))
                .query_async::<Option<String>>(&mut connection),
        )
        .await
    }

    /// The number of queued segments (0 for an absent queue) — used by the
    /// drain's lost-wakeup re-check.
    pub async fn queue_length(&self, business_correlation_id: &str) -> Result<u64, AppError> {
        let mut connection = self.connection.clone();
        self.run(
            redis::cmd("LLEN")
                .arg(Self::queue_key(business_correlation_id))
                .query_async::<u64>(&mut connection),
        )
        .await
    }

    /// Delete both keys for a completed rendezvous. The TTLs are the safety
    /// net for crashes and timeouts; deleting on success frees the keys
    /// immediately instead of waiting them out. The route's disappearance is
    /// also what tells every remaining producer to stop.
    pub async fn cleanup(&self, business_correlation_id: &str) -> Result<(), AppError> {
        let mut connection = self.connection.clone();
        self.run(
            redis::cmd("DEL")
                .arg(Self::route_key(business_correlation_id))
                .arg(Self::queue_key(business_correlation_id))
                .query_async::<()>(&mut connection),
        )
        .await
    }

    /// Publish a bare correlation-id wake-up on a return channel.
    pub async fn publish(
        &self,
        channel: &str,
        business_correlation_id: &str,
    ) -> Result<(), AppError> {
        let mut connection = self.connection.clone();
        self.run(
            redis::cmd("PUBLISH")
                .arg(channel)
                .arg(business_correlation_id)
                .query_async::<i64>(&mut connection),
        )
        .await
        .map(|_| ())
    }

    async fn run<T>(
        &self,
        operation: impl std::future::Future<Output = redis::RedisResult<T>>,
    ) -> Result<T, AppError> {
        tokio::time::timeout(self.timeout, operation)
            .await
            .map_err(|_| AppError::new(500, "Redis command timed out"))?
            .map_err(|e| AppError::new(500, format!("Redis error - {e}")))
    }
}
