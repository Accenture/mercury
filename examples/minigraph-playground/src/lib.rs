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
//! The Playground's own composable functions - a library target so the integration
//! tests link the same annotation inventory the binary does (a `#[preload]` in
//! `main.rs` would exist for the binary alone).
use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::automation::{get_event_http_target, AsyncHttpRequest};
use platform_core::{
    preload, AppError, ComposableFunction, EventEnvelope, EventStreamWriter, Platform, PostOffice,
};

/// The route of the AI-token streaming relay (`POST /api/llm/stream` in `rest.yaml`).
pub const LLM_STREAM_RELAY_ROUTE: &str = "llm.stream.relay";
/// The AI node the relay forwards to - a function on a python/node wrapper host, reached
/// through `event-over-http.yaml`.
pub const LLM_STREAM_ROUTE: &str = "llm.stream";

/// The AI-token streaming composition (the agent-orchestration follow-up to experiment
/// E0, the Java `LlmStreamRelay` twin): this endpoint's function forwards its own reply
/// lane and correlation id into a `send` to the event-over-http mapped `llm.stream`
/// function - a python/node demo app's streaming AI node, which pulls the provider's
/// REAL token stream (Gemini or Claude models) - and opts in with the
/// `accept: text/event-stream` event header. The provider's token batches relay through
/// the peer's /api/event in envelope mode and re-render progressively out this
/// application's HTTP edge as SSE, with no imperative streaming code in between.
///
/// `curl -N -H 'accept: text/event-stream' -H 'content-type: application/json' \
///   -d '{"prompt":"Write two sentences about event-driven design","params":{"provider":"gemini"}}' \
///   http://127.0.0.1:8085/api/llm/stream`
#[preload(route = "llm.stream.relay", instances = 50, interceptor)]
struct LlmStreamRelay;

#[async_trait]
impl ComposableFunction for LlmStreamRelay {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        const REMOTE_ROUTE: &str = LLM_STREAM_ROUTE;
        let platform = Platform::get_instance();
        // reachable when mapped declaratively (or registered locally by a test double)
        if !platform.has_route(REMOTE_ROUTE) && get_event_http_target(REMOTE_ROUTE).is_none() {
            let mut out = EventStreamWriter::from_request(&platform, &input)?;
            out.fail(&AppError::new(
                503,
                "AI streaming demo is not configured - start a wrapper demo app with an LLM \
                 provider credential and map llm.stream in event-over-http.yaml",
            ))
            .await?;
            return Ok(EventEnvelope::new());
        }
        let http = AsyncHttpRequest::from_value(input.body());
        let forward = EventEnvelope::new()
            .set_to(REMOTE_ROUTE)
            .set_reply_to(input.reply_to().unwrap_or_default())
            .set_correlation_id(input.correlation_id().unwrap_or_default())
            // the POST body carries the AI node's request surface (prompt | messages,
            // system, params)
            .set_raw_body(http.body().clone())
            // the event-level opt-in for progressive streaming over Event-over-HTTP
            .set_header("accept", "text/event-stream")
            // idle allowance between stream events on both hops (ms) - an LLM can pause
            // between token batches while it reasons
            .set_header("x-ttl", "60000");
        // the trace-aware PostOffice stamps the current trace onto the outbound event,
        // so the distributed trace continues across the hop into the AI node
        PostOffice::new(&platform).send(forward).await?;
        Ok(EventEnvelope::new())
    }
}
