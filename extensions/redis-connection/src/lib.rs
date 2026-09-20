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

//! **The shared Redis client foundation** — Rust port of the Java engine's
//! `extensions/redis-connection` (`org.platformlambda.redis`), extracted from
//! sync-over-async so that the distributed cache and the streaming return
//! route depend on ONE client layer instead of each carrying its own
//! (`draft-design-specs/distributed-cache-port.md`; the Java design spec's Q2).
//!
//! | Type | Java class | Role |
//! |---|---|---|
//! | [`RedisConfig`] | `RedisConfig` | discrete connection parameters from a configurable key prefix, each falling back to the plain `redis.*` form |
//! | [`RedisBackend`] | `RedisBackend` + `RedisBackendFactory` | the standalone-or-cluster seam: two-key selection, `INFO` auto-detect, one multiplexed connection, one `query` |
//! | [`RedisHealthProbe`] | `RedisHealthProbe` | the reusable `/health` PING probe a module binds to its own route |
//!
//! Nothing here registers a route by itself (the Java class is deliberately
//! not `@PreLoad`, or it would auto-register in every consumer): a module
//! supplies the thin binding — `soa.redis.health` in sync-over-async,
//! `redis.health` in the distributed cache — over this one probe.
//!
//! **Cross-engine contract.** Key names and defaults are the Java engine's, so
//! a Java and a Rust pod configured alike address the same server with the
//! same credentials and the same topology selection.

mod backend;
mod config;
mod health;

pub use backend::{
    classify_command_error, command_timeout, ConnectError, RedisBackend, RedisConnection,
};
pub use config::{RedisConfig, BASE_PREFIX, SOA_PREFIX};
pub use health::{duration_seconds, resolve_duration, RedisHealthProbe};
