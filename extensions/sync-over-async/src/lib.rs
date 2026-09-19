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

//! **Cross-pod progressive rendering** — the Rust port of the Java
//! `extensions/sync-over-async` streaming return route
//! (`draft-design-specs/sync-over-async-port.md`).
//!
//! A pod holding a user's HTTP connection opens a *rendezvous* keyed by a
//! business correlation id; any pod — or any other process, in any language —
//! posts ordered segments into it through Redis alone, and the holding pod
//! renders them progressively out its SSE edge. There is **no broker** on this
//! path: the rendezvous is a Redis List plus a wake-up channel.
//!
//! ```text
//!   producer pod                        Redis                      UI pod
//!   ------------                        -----                      ------
//!   post(cid, data, "The")   --RPUSH--> queue:{cid}
//!                            --PUBLISH-> request:{cid} --wake-up--> drain -> sink -> SSE
//!   post(cid, eof, metadata) --RPUSH-->              ... terminal --> close + delete keys
//! ```
//!
//! **The wire contract is normative and shared with the Java engine** (port
//! spec §4): identical key shapes (`request:{cid}`, `queue:{cid}`), the compact
//! segment envelope, the `{prefix}:{origin}` channel naming, and the `sync.*`
//! configuration keys with the same defaults. A Rust producer can therefore
//! stream into a Java pod's rendezvous and vice versa.
//!
//! # What this crate ships
//!
//! | Type | Role |
//! |---|---|
//! | [`StreamSegment`] | the queued unit and its compact JSON wire form |
//! | [`ReturnRouteStore`] | the Redis key space: atomic append, destructive pop, cleanup |
//! | [`ReturnRouteCoordinator`] | one pod's engine — both rendezvous patterns, one subscription |
//! | [`StreamResponder`] | the producer API: post a segment, learn whether the rendezvous is live |
//! | [`SegmentSink`] | the consumer end a facade implements to render segments |
//! | [`StreamBridge`] / [`EventStreamSink`] | the UI-pod facade half: rendezvous → HTTP edge reply lane, with the idle-expiry watchdog |
//! | [`RedisHealthCheck`] | the `soa.redis.health` function for the `/health` endpoint — a thin binding of the `redis-connection` foundation's probe |
//! | [`runtime`] | process-wide holder for the one coordinator a real application runs |
//! | [`SyncOverAsyncConfig`] / [`RedisSettings`] | the `sync.*` startup parameters, and the `soa.redis.*` connection parameters (falling back to plain `redis.*`) from the `redis-connection` foundation |
//!
//! # Contracts worth knowing
//!
//! - **Post in the order you mean.** There is no sequence number; ordering is
//!   the producer's posting discipline over one connection. An ordered stream
//!   therefore needs a *single* sequential poster — and that binds forwarders
//!   too: a relay that forwards an upstream stream concurrently can sequence a
//!   data segment behind the terminal one, which is then discarded by design.
//! - **A `false` from `post` means stop.** The rendezvous is over (consumer
//!   disconnected, timed out, crashed, or another producer closed it).
//! - **Wake-ups are best-effort.** Correctness rests on the queue: a lost
//!   notification costs latency only (the next post's drain, or the consumer's
//!   final drain, recovers it), and a duplicate pops nothing.
//! - **A one-shot response is the degenerate stream** — a queue whose first
//!   entry is terminal — so both patterns share one mechanism and one contract.

mod bridge;
mod config;
mod connection;
mod coordinator;
mod health;
mod pending;
mod responder;
mod store;

pub mod runtime;
pub mod segment;

pub use bridge::{EventStreamSink, SharedStreamWriter, StreamBridge, EDGE_GRACE_SECONDS};
pub use config::{
    SyncOverAsyncConfig, MAX_PENDING_REQUESTS, MAX_PENDING_STREAMS, RESPONSE_TTL_SECONDS,
    RETURN_CHANNEL_PREFIX, ROUTE_TTL_SECONDS, STREAM_TTL_SECONDS,
};
pub use connection::RedisSettings;
pub use coordinator::ReturnRouteCoordinator;
pub use health::{RedisHealthCheck, REDIS_HEALTH_ROUTE};
pub use pending::{PendingEntry, PendingRequests, PendingStreams, SegmentSink, StreamEntry};
pub use responder::{StreamResponder, DEFAULT_TTL_SECONDS};
pub use segment::StreamSegment;
pub use store::ReturnRouteStore;

/// The module's self-contained correlation-id key (`"cid"`) — the flow-level
/// contract shared by the facade tasks and their data mappings.
///
/// Deliberately **not** a transport's wire header name: the header that
/// carries the id between pods belongs to the transport and is configurable
/// there, so this module never references a transport constant.
pub const CID: &str = "cid";
