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

//! Compiled flow model — Rust port of `com.accenture.models.Flow` / `Task`.
//!
//! A [`Flow`] is the immutable, validated output of the compiler; a `Task` is
//! one step: the composable function it runs (`function_route`, or a
//! `flow://{id}` sub-flow), its input/output data-mapping rules (already
//! normalized — 3-part entries decomposed, `!model.x` negation and legacy
//! `:type` qualifiers rewritten to the `f:plugin(...)` form), and the routing
//! metadata its execution type needs.
//!
//! Divergence (doc'd): the Java `Task` carries mutable monitor fields used by
//! `EventScriptMock` — those arrive with the mock port (design E8), so the
//! Rust model stays fully immutable for now.

use std::collections::HashMap;

/// One compiled task (Java `Task`). Field names mirror the Java original for
/// side-by-side review; all lists hold the normalized mapping strings.
#[derive(Debug)]
pub struct Task {
    /// Unique task name (Java `service`) — `name`, defaulting to `process`.
    pub service: String,
    /// The composable function route (or `flow://{id}`); defaults to `service`.
    pub function_route: String,
    /// One of the eight execution types (validated).
    pub execution: String,
    /// For-loop initializer parts: `[model.key, start-value]`.
    pub init: Vec<String>,
    /// For-loop comparator parts: `[lhs, op, rhs]`.
    pub comparator: Vec<String>,
    /// For-loop sequencer parts: `[model.key, "++" | "--"]`.
    pub sequencer: Vec<String>,
    /// Pipeline loop conditions: each `[model.key, "break" | "continue"]`.
    pub conditions: Vec<Vec<String>>,
    /// Normalized input data-mapping rules.
    pub input: Vec<String>,
    /// Normalized output data-mapping rules.
    pub output: Vec<String>,
    /// Next task name(s), shaped by the execution type.
    pub next_steps: Vec<String>,
    /// Ordered pipeline step task names (pipeline tasks only).
    pub pipeline_steps: Vec<String>,
    /// The join task (fork tasks only).
    pub join_task: Option<String>,
    /// Task-level exception handler route (overrides `flow.exception`).
    pub exception_task: Option<String>,
    /// `"for"` / `"while"` / `"none"`.
    pub loop_type: String,
    /// Model key controlling a while loop.
    pub while_model_key: Option<String>,
    /// `model.*` list to iterate for a dynamic fork.
    pub source_model_key: Option<String>,
    /// Fixed completion delay in ms (−1 = none; Java parity).
    pub delay: i64,
    /// Model variable holding a runtime delay.
    pub delay_var: Option<String>,
    /// True when any input mapping touches `model.parent.*` / `model.root.*`.
    pub input_parent_ref: bool,
    /// True when any output mapping touches `model.parent.*` / `model.root.*`.
    pub output_parent_ref: bool,
}

impl Task {
    pub fn new(service: &str, function_route: Option<&str>, execution: &str) -> Self {
        Task {
            service: service.to_string(),
            function_route: function_route.unwrap_or(service).to_string(),
            execution: execution.to_string(),
            init: Vec::new(),
            comparator: Vec::new(),
            sequencer: Vec::new(),
            conditions: Vec::new(),
            input: Vec::new(),
            output: Vec::new(),
            next_steps: Vec::new(),
            pipeline_steps: Vec::new(),
            join_task: None,
            exception_task: None,
            loop_type: "none".to_string(),
            while_model_key: None,
            source_model_key: None,
            delay: -1,
            delay_var: None,
            input_parent_ref: false,
            output_parent_ref: false,
        }
    }
}

/// One compiled flow (Java `Flow`): the validated template that
/// [`Flows`](crate::flows) serves to the runtime.
#[derive(Debug)]
pub struct Flow {
    pub id: String,
    /// The mandatory `flow.description` (retained for discovery listings).
    pub description: String,
    /// Time-to-live in milliseconds (compile floor: 1s).
    pub ttl: u64,
    pub first_task: String,
    /// Route or `flow://{id}` of the external state machine (`ext:` targets).
    pub external_state_machine: Option<String>,
    /// Route of the flow-level unhandled-exception handler.
    pub exception: Option<String>,
    /// Tasks keyed by task name (Java `service`).
    pub tasks: HashMap<String, Task>,
}

impl Flow {
    pub fn new(
        id: &str,
        description: &str,
        first_task: &str,
        external_state_machine: Option<String>,
        ttl_ms: u64,
        exception: Option<String>,
    ) -> Self {
        Flow {
            id: id.to_string(),
            description: description.to_string(),
            ttl: ttl_ms,
            first_task: first_task.to_string(),
            external_state_machine,
            exception,
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.service.clone(), task);
    }
}
