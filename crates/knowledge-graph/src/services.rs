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

//! Support services (Java `services` package): the housekeeper that clears a
//! graph instance when its wrapping flow ends, the flow-level exception
//! handler the `graph-executor` flow wires, and the template health check.

use std::collections::HashMap;
use std::sync::OnceLock;

use event_script::conversions::display;
use platform_core::{AppConfigReader, AppError, EventEnvelope};
use rmpv::Value;

use crate::common::invalid;
use crate::model;

/// Java `GraphHousekeeper` (`graph.housekeeper`, zero-tracing): registered
/// as an end-flow listener; clears the traversal state for the flow.
pub async fn housekeeper(
    headers: HashMap<String, String>,
    _event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    if headers.get("type").map(String::as_str) == Some("end") {
        if let Some(instance_id) = headers.get("instance_id") {
            if let Some(instance) = model::remove_instance(instance_id) {
                log::info!(
                    "Graph instance {instance_id} for model '{}' cleared",
                    instance.graph_id
                );
            }
        }
    }
    EventEnvelope::new().set_body("done")
}

fn is_dev_env() -> bool {
    static DEV: OnceLock<bool> = OnceLock::new();
    *DEV.get_or_init(|| AppConfigReader::get_instance().get_property_or("app.env", "dev") == "dev")
}

/// Java `GraphExceptionHandler` (`graph.exception.handler`): normalizes a
/// flow error `{status, message, stack?}` into the standard error map; a
/// map-shaped message passes through minus the stack trace.
pub async fn exception_handler(
    _headers: HashMap<String, String>,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let Value::Map(input) = event.body() else {
        return Ok(EventEnvelope::new().set_raw_body(Value::Map(vec![])));
    };
    let get = |key: &str| -> Option<Value> {
        input
            .iter()
            .find(|(k, _)| k.as_str() == Some(key))
            .map(|(_, v)| v.clone())
    };
    let (Some(status), Some(message)) = (get("status"), get("message")) else {
        return Ok(EventEnvelope::new().set_raw_body(Value::Map(vec![])));
    };
    // log the error only in the dev environment
    if is_dev_env() {
        match get("stack") {
            Some(Value::String(stack)) => log::error!(
                "User defined exception handler received from {}, rc={}, error={}, stack={}",
                get("task")
                    .map(|v| display(&v))
                    .unwrap_or_else(|| "previous task".to_string()),
                display(&status),
                display(&message),
                stack.as_str().unwrap_or_default()
            ),
            _ => log::error!(
                "User defined exception handler received from {}, rc={}, error={}",
                get("task")
                    .map(|v| display(&v))
                    .unwrap_or_else(|| "previous task".to_string()),
                display(&status),
                display(&message)
            ),
        }
    }
    let result = match message {
        Value::Map(map) => {
            // remove stack, keep original content, add the status code
            let mut entries: Vec<(Value, Value)> = map
                .into_iter()
                .filter(|(k, _)| k.as_str() != Some("stack"))
                .collect();
            entries.push((Value::from("status"), status));
            Value::Map(entries)
        }
        other => Value::Map(vec![
            (Value::from("status"), status),
            (Value::from("message"), other),
            (Value::from("type"), Value::from("error")),
        ]),
    };
    Ok(EventEnvelope::new().set_raw_body(result))
}

/// Java `GraphHealth` (`graph.health`): the template health-check service
/// (`mandatory.health.dependencies=graph.health`).
pub async fn health(
    headers: HashMap<String, String>,
    _event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    match headers.get("type").map(String::as_str) {
        Some("info") => Ok(EventEnvelope::new().set_raw_body(Value::Map(vec![
            (Value::from("service"), Value::from("mini-graph service")),
            (Value::from("href"), Value::from("http://127.0.0.1")),
        ]))),
        Some("health") => Ok(EventEnvelope::new().set_raw_body(Value::Map(vec![(
            Value::from("mini-graph"),
            Value::from("I am doing fine"),
        )]))),
        _ => Err(invalid("type must be info or health")),
    }
}
