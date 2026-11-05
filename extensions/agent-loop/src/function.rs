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

//! Rust port of the Java `LlmRun` (`org.platformlambda.agentloop.LlmRun`): `llm.run`, the agent loop
//! (RFC-0011). A graph node calls it with `graph.task`, giving it a tool catalog, a prompt, a mode and
//! limits, and routes on the outcome envelope it returns. See [`AgentLoop`].
//!
//! Telemetry: the function's span parents every `llm.chat` turn and every tool call, and its
//! annotations carry the status, the turn counts, the model and the usage. They never carry a prompt,
//! a completion, a tool's input or a tool's result, and neither does the log line.
//!
//! Progress: when the caller passes `x-return-path` (the graph engine does, for a node with
//! `progress`), the loop sends its progress events there - `run.start`, `turn.start`, `turn.end`,
//! `tool.start`, `tool.end`, `approval.wait` and `run.end` - numbered from 1 for each call.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use platform_core::automation::MY_CORRELATION_ID;
use platform_core::{preload, AppError, ComposableFunction, EventEnvelope, Platform, PostOffice};

use crate::agent_loop::{AgentLoop, CAP, CONVERSATION, MODEL, STATUS, TURN, TURNS, USAGE};
use crate::platform_gateway::{PlatformGateway, X_RETURN_PATH};
use crate::settings::Defaults;
use crate::value::{copy, get, text, value_of};

/// The agent loop's route.
pub const LLM_RUN: &str = "llm.run";

/// `llm.run` - registered by the preload inventory when the application links this crate;
/// `worker.instances.llm.run` sets how many loops run at once (default 50).
#[preload(
    route = "llm.run",
    instances = 50,
    env_instances = "worker.instances.llm.run"
)]
pub struct LlmRun;

#[async_trait]
impl ComposableFunction for LlmRun {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let platform = Platform::get_instance();
        let po = PostOffice::new(&platform);
        let gateway = PlatformGateway::with_progress(
            &platform,
            headers.get(X_RETURN_PATH).map(String::as_str),
            headers.get(MY_CORRELATION_ID).map(String::as_str),
        );
        let agent = AgentLoop::new(&gateway, Defaults::from_config(), Box::new(now_ms));
        let envelope = agent.run(input.body()).await?;
        let status = value_of(get(&envelope, STATUS));
        let outcome = match get(&envelope, CAP) {
            None => status,
            reason => format!("{status}/{}", value_of(reason)),
        };
        po.annotate_trace(STATUS, &outcome);
        po.annotate_trace(TURNS, value_of(get(&envelope, TURNS)));
        if let Some(turn) = get(&envelope, TURN) {
            po.annotate_trace(TURN, value_of(Some(turn)));
        }
        if let Some(model) = text(get(&envelope, MODEL)) {
            po.annotate_trace(MODEL, model);
        }
        po.annotate_trace(USAGE, copy(&envelope, USAGE));
        log::info!(
            "llm.run {outcome} after {} turn(s), conversation {}, model {}",
            value_of(get(&envelope, TURNS)),
            value_of(get(&envelope, CONVERSATION)),
            value_of(get(&envelope, MODEL))
        );
        Ok(EventEnvelope::new().set_raw_body(envelope))
    }
}

/// The wall clock in milliseconds since the epoch (Java `System::currentTimeMillis`).
fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_millis() as i64)
        .unwrap_or(0)
}
