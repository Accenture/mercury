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

//! The bounded, cache-shaped operation set of `v1.cache.redis` — Rust port of
//! the Java `CacheAction` enum (design spec §2, Q5). Each maps to a
//! cluster-correct Redis call through the `RedisBackend` seam:
//!
//! - [`CacheAction::Put`] — `SETEX` (value + TTL);
//! - [`CacheAction::Get`] — `GET` (value, or null on a miss);
//! - [`CacheAction::Mget`] — `MGET` (per-slot scatter-gather on a cluster);
//! - [`CacheAction::Mput`] — a pipelined per-entry `SETEX` (TTL-preserving,
//!   non-atomic across the map — never raw `MSET`, which sets no TTL);
//! - [`CacheAction::Delete`] — `DEL` (count removed);
//! - [`CacheAction::PutIfNotPresent`] — atomic `SET key value NX EX ttl`;
//! - [`CacheAction::ListPush`] — atomic `RPUSH` + `EXPIRE` (new length);
//! - [`CacheAction::ListPop`] — destructive `LPOP` (oldest value, or null);
//! - [`CacheAction::ListLen`] — `LLEN`.
//!
//! `PING` is intentionally not here — it backs the `redis.health` check, not
//! a general action.

use platform_core::AppError;

/// One cache operation, selected by the `action` header (case-insensitive).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheAction {
    Put,
    Get,
    Mget,
    Mput,
    Delete,
    PutIfNotPresent,
    ListPush,
    ListPop,
    ListLen,
}

impl CacheAction {
    /// Every action, in the Java enum's declaration order (the order the
    /// error messages list them in).
    pub const ALL: [CacheAction; 9] = [
        CacheAction::Put,
        CacheAction::Get,
        CacheAction::Mget,
        CacheAction::Mput,
        CacheAction::Delete,
        CacheAction::PutIfNotPresent,
        CacheAction::ListPush,
        CacheAction::ListPop,
        CacheAction::ListLen,
    ];

    /// The wire name — the `action` header value (Java `Enum.name()`).
    pub fn name(self) -> &'static str {
        match self {
            CacheAction::Put => "PUT",
            CacheAction::Get => "GET",
            CacheAction::Mget => "MGET",
            CacheAction::Mput => "MPUT",
            CacheAction::Delete => "DELETE",
            CacheAction::PutIfNotPresent => "PUT_IF_NOT_PRESENT",
            CacheAction::ListPush => "LIST_PUSH",
            CacheAction::ListPop => "LIST_POP",
            CacheAction::ListLen => "LIST_LEN",
        }
    }

    /// Resolve an `action` header (case-insensitive) to an action, with a
    /// clear error naming the supported set when it does not match (Java
    /// `CacheAction.from`).
    pub fn from_header(action: Option<&str>) -> Result<CacheAction, AppError> {
        let text = action.unwrap_or("");
        if text.trim().is_empty() {
            return Err(AppError::new(
                400,
                format!("Missing 'action' - one of {}", Self::supported()),
            ));
        }
        let wanted = text.trim().to_ascii_uppercase();
        Self::ALL
            .into_iter()
            .find(|candidate| candidate.name() == wanted)
            .ok_or_else(|| {
                AppError::new(
                    400,
                    format!("Unsupported action '{text}' - one of {}", Self::supported()),
                )
            })
    }

    fn supported() -> String {
        Self::ALL
            .iter()
            .map(|action| action.name())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Java `CacheActionTest.resolvesCaseInsensitively`.
    #[test]
    fn resolves_case_insensitively() {
        assert_eq!(
            CacheAction::Get,
            CacheAction::from_header(Some("get")).unwrap()
        );
        assert_eq!(
            CacheAction::Put,
            CacheAction::from_header(Some(" Put ")).unwrap()
        );
        assert_eq!(
            CacheAction::PutIfNotPresent,
            CacheAction::from_header(Some("put_if_not_present")).unwrap()
        );
        assert_eq!(
            CacheAction::ListPush,
            CacheAction::from_header(Some("LIST_PUSH")).unwrap()
        );
    }

    /// Java `CacheActionTest.missingActionNamesTheSupportedSet`.
    #[test]
    fn missing_action_names_the_supported_set() {
        for absent in [None, Some(""), Some("  ")] {
            let error = CacheAction::from_header(absent).unwrap_err();
            assert_eq!(400, error.status());
            assert!(error
                .message()
                .starts_with("Missing 'action' - one of PUT, GET, MGET"));
            assert!(error.message().ends_with("LIST_LEN"));
        }
    }

    /// Java `CacheActionTest.unsupportedActionIsNamedInTheError`.
    #[test]
    fn unsupported_action_is_named_in_the_error() {
        let error = CacheAction::from_header(Some("INCR")).unwrap_err();
        assert_eq!(400, error.status());
        assert!(error
            .message()
            .starts_with("Unsupported action 'INCR' - one of "));
        assert!(error.message().contains("PUT_IF_NOT_PRESENT"));
    }
}
