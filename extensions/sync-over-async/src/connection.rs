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

//! The Redis connection parameters — since the `redis-connection` foundation
//! crate landed (the Java `extensions/redis-connection` extraction, lock-step
//! with v4.12.9), the type is the foundation's [`RedisConfig`]; this module
//! keeps the crate's original name `RedisSettings` as a compatibility alias so
//! existing callers, tests and the demo compile unchanged.
//!
//! `RedisSettings::from_config()` reads sync-over-async's **`soa.redis.*`**
//! namespace with the un-prefixed `redis.*` fallback (Java parity — the same
//! keys, the same fallback), so a pre-existing `redis.*` deployment is
//! unchanged and an application that also runs the distributed cache
//! (plain `redis.*`) can point the rendezvous at its own server by setting
//! the whole `soa.redis.*` connection set.
//!
//! Clients are built lazily from live configuration, never frozen at
//! construction time: the Java module learned that a credential published by
//! a start-up vault bootstrap has not landed while functions are being
//! constructed.

pub use redis_connection::RedisConfig as RedisSettings;
