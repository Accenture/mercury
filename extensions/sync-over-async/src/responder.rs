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

//! The segment-producer side of the streaming return route — Rust port of the
//! Java `StreamResponder`.
//!
//! A backend service posts progressive events (and the terminal `eof` /
//! `exception`) straight to Redis, and whichever pod holds the user's HTTP
//! connection drains and renders them. Deliberately lightweight: it is built
//! from the same discrete `redis.*` parameters but needs **no coordinator** —
//! no subscriber, no return channel of its own — only the key contract
//! (`queue:{cid}` + `request:{cid}`).
//!
//! ```no_run
//! # use sync_over_async::{RedisSettings, StreamResponder, segment};
//! # async fn demo(cid: &str) -> Result<(), platform_core::AppError> {
//! let responder = StreamResponder::connect(&RedisSettings::from_config()).await?;
//! responder.post(cid, segment::DATA, None, Some("Hello")).await?;
//! let live = responder.post(cid, segment::EOF, None, Some("{\"tokens\":1}")).await?;
//! // false = the rendezvous is over (orphan) - stop producing for that cid
//! # let _ = live;
//! # Ok(())
//! # }
//! ```
//!
//! **Ordering is the posting discipline.** There is no sequence number: Redis
//! executes each connection's commands in arrival order, so a producer that
//! requires strict ordering (the AI-chat case) posts sequentially — one task,
//! one responder — and list order equals generation order for free. Several
//! producers on one cid (the event-notification case) interleave at segment
//! granularity, by design. Concurrent tasks sharing one responder are safe (the
//! connection multiplexes), but their relative order is then scheduling-
//! dependent — use that only where ordering does not matter.
//!
//! **Any producer may close the channel** by posting a terminal segment; the
//! consumer then deletes the route, and every other producer's next `post`
//! returns `false`. A `false` always means stop: the consumer disconnected,
//! timed out, crashed, or another producer already closed the channel. (The
//! just-appended segment stays queued under its TTL and simply ages out — the
//! store-first order is deliberate, so a wake-up can never precede its data.)

use platform_core::AppError;

use crate::connection::RedisSettings;
use crate::segment::StreamSegment;
use crate::store::ReturnRouteStore;

/// Default queue TTL — matches the `sync.stream.ttl.seconds` default on the
/// consumer side.
pub const DEFAULT_TTL_SECONDS: u64 = 1800;

/// A producer's handle on the rendezvous key space.
#[derive(Clone)]
pub struct StreamResponder {
    store: ReturnRouteStore,
    ttl_seconds: u64,
}

impl StreamResponder {
    /// Connect with the default queue TTL.
    pub async fn connect(settings: &RedisSettings) -> Result<Self, AppError> {
        StreamResponder::connect_with_ttl(settings, DEFAULT_TTL_SECONDS).await
    }

    /// Connect with an explicit queue TTL, refreshed on every post (the crash
    /// safety net; align it with the consumer side's
    /// `sync.stream.ttl.seconds`).
    pub async fn connect_with_ttl(
        settings: &RedisSettings,
        ttl_seconds: u64,
    ) -> Result<Self, AppError> {
        Ok(StreamResponder {
            store: ReturnRouteStore::connect(settings).await?,
            ttl_seconds,
        })
    }

    /// Post one segment: append it to `queue:{cid}` with a TTL refresh
    /// (store-first), then wake the consuming pod via its return channel. The
    /// whole producer contract is *post in the order you mean* — no sequence
    /// header to stamp, no per-cid state to hold.
    ///
    /// Returns `true` while the rendezvous is live; `false` for an orphan —
    /// stop producing for that cid.
    pub async fn post(
        &self,
        business_correlation_id: &str,
        segment_type: &str,
        name: Option<&str>,
        body: Option<&str>,
    ) -> Result<bool, AppError> {
        let segment = StreamSegment::of(segment_type, name, body)?;
        self.store
            .append_segment(
                business_correlation_id,
                &segment.to_json(),
                self.ttl_seconds,
            )
            .await?;
        match self.store.get_route(business_correlation_id).await? {
            Some(channel) => {
                self.store
                    .publish(&channel, business_correlation_id)
                    .await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// The underlying store — the chaos hook a dry-run uses to append a
    /// segment **without** publishing its wake-up.
    pub fn store(&self) -> &ReturnRouteStore {
        &self.store
    }
}
