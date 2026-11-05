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

//! Rust port of the Java `LoopGateway` (`org.platformlambda.agentloop.LoopGateway`): how the loop
//! reaches the routes it calls, the model turn (`llm.chat`) and each tool's route.
//!
//! Java answers a request with a future whose failure the loop turns into an error response. Here a
//! request answers `Result<EventEnvelope, AppError>`, the platform's own RPC shape: an `Err` (a
//! timeout is status 408, a route nothing serves 404) becomes an error response with that status and
//! message ([`failure`](crate::failure)), so a tool's timeout becomes its error result.

use async_trait::async_trait;
use platform_core::{AppError, EventEnvelope};
use rmpv::Value;

/// How the loop reaches `llm.chat` and the tools' routes.
#[async_trait]
pub trait LoopGateway: Send + Sync {
    /// Whether a route is served: a local function, or a declared Event-over-HTTP target.
    fn reachable(&self, route: &str) -> bool;

    /// Send one model turn to `llm.chat`, waiting at most `timeout_ms`; a timeout is status 408.
    async fn chat(&self, request: Value, timeout_ms: i64) -> Result<EventEnvelope, AppError>;

    /// Call a tool's route with the model's input as the event body, waiting at most `timeout_ms`; a
    /// timeout is status 408.
    async fn call(
        &self,
        route: &str,
        body: Value,
        timeout_ms: i64,
    ) -> Result<EventEnvelope, AppError>;

    /// Send one progress event to the caller's return route (RFC-0011 section 8.4), without waiting
    /// for it to be handled: a return route that fails never fails the run. A caller that names no
    /// return route gets none. The event carries names, routes, counts and statuses only.
    ///
    /// Async where Java's is not: the platform's send awaits the route's mailbox accepting the event
    /// (the Java event bus enqueues synchronously), so the events reach the return route in order and
    /// before the loop's own response.
    async fn progress(&self, _event: Value) {
        // nobody watches
    }
}
