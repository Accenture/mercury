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

//! Test-time flow mocking — Rust port of `com.accenture.mock.EventScriptMock`:
//! reassign a task's function route to a mock composable function without
//! touching the compiled flow. Java mutates the `Task` object; the Rust flow
//! template is immutable and shared, so overrides live in a registry the
//! executor consults when dispatching (same observable behavior, safer under
//! concurrency). The Java before/after task monitors are not ported (no
//! fixture in this repo exercises them; documented divergence).

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use platform_core::AppError;

use crate::flows;

fn overrides() -> &'static Mutex<HashMap<(String, String), String>> {
    static OVERRIDES: OnceLock<Mutex<HashMap<(String, String), String>>> = OnceLock::new();
    OVERRIDES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The route the executor should dispatch for a task (the override when one
/// is registered, otherwise the compiled route).
pub(crate) fn effective_route(flow_id: &str, task_name: &str, compiled: &str) -> String {
    overrides()
        .lock()
        .expect("mock overrides")
        .get(&(flow_id.to_string(), task_name.to_string()))
        .cloned()
        .unwrap_or_else(|| compiled.to_string())
}

/// Java `EventScriptMock`: scope mocks to one flow.
pub struct EventScriptMock {
    flow_id: String,
}

impl EventScriptMock {
    /// Fails when the flow does not exist (Java parity).
    pub fn new(flow_id: &str) -> Result<Self, AppError> {
        if !flows::flow_exists(flow_id) {
            return Err(AppError::new(400, format!("Flow {flow_id} does not exist")));
        }
        Ok(EventScriptMock {
            flow_id: flow_id.to_string(),
        })
    }

    /// The currently effective function route of a task.
    pub fn get_function_route(&self, task_name: &str) -> Result<String, AppError> {
        let flow = flows::get_flow(&self.flow_id)
            .ok_or_else(|| AppError::new(400, format!("Flow {} does not exist", self.flow_id)))?;
        let task = flow
            .tasks
            .get(task_name)
            .ok_or_else(|| AppError::new(400, format!("Task {task_name} does not exist")))?;
        Ok(effective_route(
            &self.flow_id,
            task_name,
            &task.function_route,
        ))
    }

    /// Reassign a task's function route to a mock (Java `assignFunctionRoute`).
    pub fn assign_function_route(
        &self,
        task_name: &str,
        mock_function: &str,
    ) -> Result<&Self, AppError> {
        let flow = flows::get_flow(&self.flow_id)
            .ok_or_else(|| AppError::new(400, format!("Flow {} does not exist", self.flow_id)))?;
        if !flow.tasks.contains_key(task_name) {
            return Err(AppError::new(
                400,
                format!("Task {task_name} does not exist"),
            ));
        }
        if mock_function.trim().is_empty() {
            return Err(AppError::new(400, "Mock function route must not be empty"));
        }
        overrides().lock().expect("mock overrides").insert(
            (self.flow_id.clone(), task_name.to_string()),
            mock_function.to_string(),
        );
        log::info!(
            "Reassigned '{mock_function}' to task '{task_name}' of flow '{}'",
            self.flow_id
        );
        Ok(self)
    }

    /// Remove a task's mock, restoring the compiled route.
    pub fn restore_function_route(&self, task_name: &str) -> &Self {
        overrides()
            .lock()
            .expect("mock overrides")
            .remove(&(self.flow_id.clone(), task_name.to_string()));
        self
    }
}
