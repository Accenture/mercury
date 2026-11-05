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

//! Rust port of the Java `PlatformGateway` (`org.platformlambda.agentloop.PlatformGateway`): the
//! loop's routes through the platform, the path `graph.task` uses. A local route is a local call, and
//! a language pack's route (`llm.chat`, an MCP binding) is one Event-over-HTTP hop. The requests are
//! issued from the `llm.run` function's own task, inside its trace, so every request carries the
//! `llm.run` span as its parent.
//!
//! "Reachable" is a local function or a declared Event-over-HTTP target. The Java engine's
//! `po.exists` also finds a route on the Kafka service mesh, which the Rust port does not have.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::automation::event_api::get_event_http_target;
use platform_core::{AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::gateway::LoopGateway;
use crate::value::{java_is_blank, java_trim, put};

/// The model turn's route, served by a language pack's LLM helper.
pub const LLM_CHAT: &str = "llm.chat";
/// The header a caller names its return route in, for progress events (RFC-0011 section 8.4).
pub const X_RETURN_PATH: &str = "x-return-path";
const CID: &str = "cid";

/// The loop's gateway to the platform.
pub struct PlatformGateway {
    platform: Platform,
    po: PostOffice,
    return_path: Option<String>,
    business_cid: Option<String>,
    warned: AtomicBool,
}

impl PlatformGateway {
    /// A gateway that sends no progress events.
    pub fn new(platform: &Platform) -> PlatformGateway {
        PlatformGateway::with_progress(platform, None, None)
    }

    /// A gateway that sends progress events to the caller's return route, when it names one,
    /// carrying the business correlation id when there is one.
    pub fn with_progress(
        platform: &Platform,
        return_path: Option<&str>,
        cid: Option<&str>,
    ) -> PlatformGateway {
        PlatformGateway {
            platform: platform.clone(),
            po: PostOffice::new(platform),
            return_path: return_path
                .filter(|route| !java_is_blank(route))
                .map(|route| java_trim(route).to_string()),
            business_cid: cid.map(str::to_string),
            warned: AtomicBool::new(false),
        }
    }

    async fn request(
        &self,
        route: &str,
        body: Value,
        timeout_ms: i64,
    ) -> Result<EventEnvelope, AppError> {
        let event = EventEnvelope::new().set_to(route).set_raw_body(body);
        let timeout = Duration::from_millis(u64::try_from(timeout_ms).unwrap_or(0));
        self.po.request(event, timeout).await
    }
}

#[async_trait]
impl LoopGateway for PlatformGateway {
    fn reachable(&self, route: &str) -> bool {
        // an invalid route name is served by nothing
        self.platform.has_route(route) || get_event_http_target(route).is_some()
    }

    async fn chat(&self, request: Value, timeout_ms: i64) -> Result<EventEnvelope, AppError> {
        self.request(LLM_CHAT, request, timeout_ms).await
    }

    async fn call(
        &self,
        route: &str,
        body: Value,
        timeout_ms: i64,
    ) -> Result<EventEnvelope, AppError> {
        self.request(route, body, timeout_ms).await
    }

    /// Untraced, so a run's progress adds no span per event; a failing return route is logged once.
    async fn progress(&self, event: Value) {
        let Some(route) = &self.return_path else {
            return;
        };
        let mut event = event;
        // Java putIfAbsent: an absent or null cid takes the business correlation id
        if let (Some(cid), Value::Map(entries)) = (&self.business_cid, &mut event) {
            if !entries
                .iter()
                .any(|(key, value)| key.as_str() == Some(CID) && !value.is_nil())
            {
                put(entries, CID, Value::from(cid.as_str()));
            }
        }
        let message = EventEnvelope::new().set_to(route).set_raw_body(event);
        if let Err(e) = self.po.send_untraced(message).await {
            if !self.warned.swap(true, Ordering::Relaxed) {
                log::warn!("Progress to {route} is not delivered - {}", e.message());
            }
        }
    }
}
