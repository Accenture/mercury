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

//! # event-script (Rust port)
//!
//! Rust port of mercury-composable's `event-script-engine` — layer 2,
//! **composable orchestration**: a YAML DSL describing an end-to-end
//! transaction as a *flow* that choreographs composable functions by route
//! name, executed over the platform-core event bus with a per-transaction
//! state machine. Canonical behavior spec: the Java project
//! (`system/event-script-engine`, v4.8.6) and its
//! `docs/guides/event-script/flow-grammar.md`; design:
//! `docs/design/event-script-port.md` (this repo).
//!
//! **Increment E-1 — flow model + compiler:** [`model`] (compiled
//! `Flow`/`Task`), [`flows`] (the template registry), [`compiler`]
//! (`yaml.flow.automation` discovery + the full grammar validation),
//! [`converter`] (legacy `:type` → `f:plugin(...)` rewriting), [`validator`]
//! (mapping-syntax + reserved-key checks) and [`plugins`] (the plugin *name*
//! registry — execution bodies arrive with increment E-8). The runtime
//! (state machine, task executor, adapters) arrives with increments E-2+.
//!
//! Linking this crate makes the engine self-register through the
//! platform-core annotation inventory: [`CompileFlows`] runs as a
//! before-application hook at sequence 5 (essential services 0, plugins 3,
//! flows 5, user code ≥ 6 — the Java sequence contract).

pub mod compiler;
pub mod converter;
pub mod flows;
pub mod model;
pub mod plugins;
pub mod util;
pub mod validator;

use async_trait::async_trait;
use platform_core::{before_application, AppError, EntryPoint};

/// The flow compiler hook (Java `CompileFlows`, `@BeforeApplication(sequence=5)`).
#[before_application(sequence = 5)]
pub struct CompileFlows;

#[async_trait]
impl EntryPoint for CompileFlows {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        compiler::compile_flows();
        Ok(())
    }
}
