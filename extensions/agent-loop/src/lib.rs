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

//! **The agent loop, `llm.run`** - Rust port of the Java engine's `extensions/agent-loop`
//! (`org.platformlambda.agentloop`, v4.12.29; design spec RFC-0011,
//! `draft-design-specs/llm-helper-agent-loop.md` in the Java repo).
//!
//! A graph node calls `llm.run` with a tool catalog and a prompt. The loop sends each model turn to
//! `llm.chat`, the LLM helper of a language pack (mercury-python or mercury-nodejs). When the model
//! asks for a tool, the loop calls the route the catalog maps the tool's name to, and sends every
//! result of the turn back in one message. It repeats until the model answers, a person must approve a
//! call, the model refuses, or a limit stops it. The graph routes on the envelope it returns.
//!
//! | Type | Java class | Role |
//! |---|---|---|
//! | [`AgentLoop`] | `AgentLoop` | the loop: model turns, tool calls, approval, resume, limits, progress events |
//! | [`ToolSchema`] | `ToolSchema` | a tool's input schema, a closed JSON Schema subset |
//! | [`ToolCatalog`] | `ToolCatalog` | the node's tool catalog, checked whole before the first turn |
//! | [`LoopSettings`], [`Defaults`] | `LoopSettings`, `LoopSettings.Defaults` | the node's settings and their global defaults |
//! | [`LoopGateway`] | `LoopGateway` | how the loop reaches `llm.chat` and the tools' routes |
//! | [`PlatformGateway`] | `PlatformGateway` | the gateway through the platform |
//! | [`json::compact`] | `Json` | compact JSON text, to measure, cut and quote a value |
//! | [`LlmRun`] | `LlmRun` | the `#[preload]` function `llm.run` |
//!
//! The crate is opt-in: linking it registers `llm.run` (reference the crate from `main.rs` so the
//! linker keeps its registration inventory). It links no provider SDK, no MCP client and no store
//! client. The transcript stays in the language pack's transcript store, and the loop holds only a
//! reference to it.
//!
//! **Cross-engine contract.** The shared vector file `llm-run-vectors.json` (byte-identical with the
//! Java repository) pins the loop's requests, envelopes, errors and progress events, and the schema
//! validator's messages.

mod agent_loop;
mod function;
mod gateway;
pub mod json;
mod platform_gateway;
mod settings;
mod tool_catalog;
mod tool_schema;
mod value;

pub use agent_loop::{capped, failure, AgentLoop, Clock, MIN_STEP_MS, RESERVE_MS};
pub use function::{LlmRun, LLM_RUN};
pub use gateway::LoopGateway;
pub use platform_gateway::{PlatformGateway, LLM_CHAT, X_RETURN_PATH};
pub use settings::{Defaults, LoopSettings, AUTO, MANUAL, UNATTENDED, WRITE};
pub use tool_catalog::{Row, ToolCatalog, NAME_PATTERN};
pub use tool_schema::{json_equals, ToolSchema, DOCUMENTARY, KEYWORDS, VIOLATION_CAP};
