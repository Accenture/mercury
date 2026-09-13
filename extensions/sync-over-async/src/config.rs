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

//! The module's `sync.*` startup parameters — Rust port of the Java
//! `SyncOverAsyncConfig`. Key names and defaults are **normative**: a Rust and
//! a Java pod configured alike must address the same rendezvous, so these are
//! part of the cross-engine wire contract (port spec §4).

use platform_core::AppConfigReader;

/// `sync.return.channel.prefix` — the per-pod Pub/Sub return channel is
/// `{prefix}:{origin}`.
pub const RETURN_CHANNEL_PREFIX: &str = "sync.return.channel.prefix";
/// `sync.route.ttl.seconds` — TTL of a one-shot rendezvous's route key.
pub const ROUTE_TTL_SECONDS: &str = "sync.route.ttl.seconds";
/// `sync.response.ttl.seconds` — queue TTL of a one-shot response.
pub const RESPONSE_TTL_SECONDS: &str = "sync.response.ttl.seconds";
/// `sync.max.pending.requests` — per-pod ceiling on in-flight one-shot requests.
pub const MAX_PENDING_REQUESTS: &str = "sync.max.pending.requests";
/// `sync.stream.ttl.seconds` — TTL of a streaming rendezvous (session scale).
pub const STREAM_TTL_SECONDS: &str = "sync.stream.ttl.seconds";
/// `sync.max.pending.streams` — per-pod ceiling on concurrently open streams.
pub const MAX_PENDING_STREAMS: &str = "sync.max.pending.streams";

/// Startup parameters of the return-route engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncOverAsyncConfig {
    return_channel_prefix: String,
    route_ttl_seconds: u64,
    response_ttl_seconds: u64,
    max_pending_requests: usize,
    stream_ttl_seconds: u64,
    max_pending_streams: usize,
}

impl Default for SyncOverAsyncConfig {
    /// The normative defaults, identical to the Java module's.
    fn default() -> Self {
        SyncOverAsyncConfig {
            return_channel_prefix: "svc-return".to_string(),
            route_ttl_seconds: 90,
            response_ttl_seconds: 30,
            max_pending_requests: 10_000,
            stream_ttl_seconds: 1800,
            max_pending_streams: 1000,
        }
    }
}

impl SyncOverAsyncConfig {
    /// Build from application configuration, falling back to [`Default`] for
    /// any unset or unparseable key (Java `SyncOverAsyncConfig.from`).
    pub fn from_config() -> Self {
        let config = AppConfigReader::get_instance();
        let defaults = SyncOverAsyncConfig::default();
        SyncOverAsyncConfig {
            return_channel_prefix: config
                .get_property_or(RETURN_CHANNEL_PREFIX, &defaults.return_channel_prefix),
            route_ttl_seconds: number(ROUTE_TTL_SECONDS, defaults.route_ttl_seconds),
            response_ttl_seconds: number(RESPONSE_TTL_SECONDS, defaults.response_ttl_seconds),
            max_pending_requests: number(MAX_PENDING_REQUESTS, defaults.max_pending_requests),
            stream_ttl_seconds: number(STREAM_TTL_SECONDS, defaults.stream_ttl_seconds),
            max_pending_streams: number(MAX_PENDING_STREAMS, defaults.max_pending_streams),
        }
    }

    /// Convenience constructor for tests and embedders.
    pub fn new(
        return_channel_prefix: impl Into<String>,
        route_ttl_seconds: u64,
        response_ttl_seconds: u64,
        max_pending_requests: usize,
        stream_ttl_seconds: u64,
        max_pending_streams: usize,
    ) -> Self {
        SyncOverAsyncConfig {
            return_channel_prefix: return_channel_prefix.into(),
            route_ttl_seconds,
            response_ttl_seconds,
            max_pending_requests,
            stream_ttl_seconds,
            max_pending_streams,
        }
    }

    pub fn return_channel_prefix(&self) -> &str {
        &self.return_channel_prefix
    }

    pub fn route_ttl_seconds(&self) -> u64 {
        self.route_ttl_seconds
    }

    pub fn response_ttl_seconds(&self) -> u64 {
        self.response_ttl_seconds
    }

    pub fn max_pending_requests(&self) -> usize {
        self.max_pending_requests
    }

    pub fn stream_ttl_seconds(&self) -> u64 {
        self.stream_ttl_seconds
    }

    pub fn max_pending_streams(&self) -> usize {
        self.max_pending_streams
    }
}

fn number<T: std::str::FromStr>(key: &str, fallback: T) -> T {
    AppConfigReader::get_instance()
        .get_property(key)
        .and_then(|value| value.trim().parse::<T>().ok())
        .unwrap_or(fallback)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_the_java_module() {
        let defaults = SyncOverAsyncConfig::default();
        assert_eq!("svc-return", defaults.return_channel_prefix());
        assert_eq!(90, defaults.route_ttl_seconds());
        assert_eq!(30, defaults.response_ttl_seconds());
        assert_eq!(10_000, defaults.max_pending_requests());
        assert_eq!(1800, defaults.stream_ttl_seconds());
        assert_eq!(1000, defaults.max_pending_streams());
    }
}
