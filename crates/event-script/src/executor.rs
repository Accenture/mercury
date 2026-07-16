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

//! The task executor — Rust port of `com.accenture.automation.TaskExecutor`
//! (`task.executor`, an event interceptor) plus the programmatic
//! `FlowExecutor` API.
//!
//! Increment E-4 scope: `sequential` / `response` / `end` / `decision` /
//! `sink`, exception routing (task-level handler beats `flow.exception`),
//! TTL abort (408), per-task trace + metrics, and the flow summary span.
//! `parallel` + `fork`/`join` arrive with E-5, `pipeline` loops with E-6,
//! `flow://` sub-flows and `ext:` with E-7 — reaching them aborts the flow
//! with an explicit message rather than misbehaving silently.
//!
//! State-machine semantics: the consolidated data-mapping view is built **in
//! the flow instance's own dataset tree** (scratch keys added per callback,
//! stripped after), so `model.*` writes persist exactly like Java's
//! shared-reference map — no copies of the model.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use platform_core::automation::MY_CORRELATION_ID;
use platform_core::{AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::conversions::{display, from_json, get_binary_value};
use crate::flows;
use crate::instance::{FlowInstance, TaskMetrics};
use crate::mapping::{
    self, get_constant_value, substitute_runtime_vars, FileMode, SimpleFileDescriptor,
};
use crate::mlm::MultiLevelMap;
use crate::model::Task;
use crate::util::{str2int, str2long};
use crate::validator;

pub const SERVICE_NAME: &str = "task.executor";

/// Top-level keys of the consolidated view that never persist in the
/// state machine (per-callback scratch).
const SCRATCH_KEYS: &[&str] = &[
    "status", "header", "result", "datatype", "output", "decision", "error",
];

#[derive(Clone)]
struct TaskReference {
    flow_instance_id: String,
    process_id: String,
    error_task: Option<String>,
    span_id: Option<String>,
}

fn task_refs() -> &'static Mutex<HashMap<String, TaskReference>> {
    static REFS: OnceLock<Mutex<HashMap<String, TaskReference>>> = OnceLock::new();
    REFS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Handle one event (the interceptor body): a first-task trigger from the
/// manager, a TTL timeout, or a function callback.
pub async fn handle(
    platform: &Platform,
    headers: HashMap<String, String>,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let Some(composite) = event.correlation_id().map(str::to_string) else {
        log::error!("Event dropped - missing correlation ID");
        return EventEnvelope::new().set_body("dropped");
    };
    let (internal_correlation_id, seq) = match composite.split_once('#') {
        Some((head, tail)) => (head.to_string(), str2int(tail)),
        None => (composite.clone(), -1),
    };
    // resolve and release the task reference (first task and timeout events
    // have none — their correlation id IS the flow instance id)
    let mut reference = task_refs()
        .lock()
        .expect("task refs")
        .remove(&internal_correlation_id);
    let ref_id = reference
        .as_ref()
        .map(|r| r.flow_instance_id.clone())
        .unwrap_or_else(|| internal_correlation_id.clone());
    let Some(instance) = flows::get_flow_instance(&ref_id) else {
        log::debug!("Flow instance {ref_id} is invalid or expired");
        return EventEnvelope::new().set_body("expired");
    };
    let flow_name = instance.template.id.clone();
    if headers.contains_key("timeout") && matches!(event.body(), Value::Array(_)) {
        log::warn!("Flow {flow_name}:{} expired", instance.id);
        let message = format!("Flow timeout for {} ms", instance.ttl_ms());
        abort_flow(
            platform,
            &instance,
            408,
            Value::from(message),
            instance.parent_span_id(),
        )
        .await;
        return EventEnvelope::new().set_body("timeout");
    }
    let outcome = if let Some(first_task) = headers.get("first_task") {
        // the first task's parent is the span of the function that triggered the flow
        execute_task(
            platform,
            &instance,
            first_task,
            instance.parent_span_id(),
            -1,
            None,
        )
        .await
    } else {
        // task completion: close its metrics and anchor its span as the
        // parent span for whatever this callback dispatches next
        if let Some(r) = reference.as_mut() {
            if let Some(metrics) = instance
                .metrics
                .lock()
                .expect("metrics")
                .get(&internal_correlation_id)
            {
                metrics.complete();
            }
            if let Some(span) = event.span_id() {
                r.span_id = Some(span.to_string());
            }
        }
        handle_function_callback(platform, &instance, reference, &event, seq).await
    };
    if let Err(e) = outcome {
        log::error!(
            "Unable to execute flow {flow_name}:{} - {}",
            instance.id,
            e.message()
        );
        abort_flow(
            platform,
            &instance,
            e.status(),
            Value::from(e.message()),
            instance.parent_span_id(),
        )
        .await;
    }
    EventEnvelope::new().set_body("done")
}

async fn handle_function_callback(
    platform: &Platform,
    instance: &Arc<FlowInstance>,
    reference: Option<TaskReference>,
    event: &EventEnvelope,
    seq: i32,
) -> Result<(), AppError> {
    let from = reference
        .as_ref()
        .map(|r| r.process_id.clone())
        .or_else(|| event.from().map(str::to_string))
        .ok_or_else(|| AppError::new(500, "task does not provide 'from' address"))?;
    let caller = from.split('@').next().unwrap_or(&from).to_string();
    let parent_span = reference
        .as_ref()
        .and_then(|r| r.span_id.clone())
        .or_else(|| event.span_id().map(str::to_string));
    if !instance.template.tasks.contains_key(&caller) {
        log::error!(
            "Unable to process callback {}:{} - missing task in {caller}",
            instance.template.id,
            instance.id
        );
        return Ok(());
    }
    let status = event.status();
    if status >= 400 {
        handle_function_exception(platform, instance, &caller, event, seq, status, parent_span)
            .await;
        return Ok(());
    }
    instance.set_exception_at_top_level(false);
    handle_callback(platform, instance, reference, &caller, event, parent_span).await
}

async fn handle_function_exception(
    platform: &Platform,
    instance: &Arc<FlowInstance>,
    caller: &str,
    event: &EventEnvelope,
    _seq: i32,
    status: i32,
    parent_span: Option<String>,
) {
    // pipeline-queue cleanup on exception arrives with E-6 (no pipes exist yet)
    let task = &instance.template.tasks[caller];
    let is_task_level = task.exception_task.is_some();
    let handler = task
        .exception_task
        .clone()
        .or_else(|| instance.template.exception.clone());
    // the top-level handler catches all unhandled exceptions; abort if the
    // top-level handler itself throws (exception-loop guard)
    if let Some(handler) = handler.filter(|_| !instance.top_level_exception_happened()) {
        if !is_task_level {
            instance.set_exception_at_top_level(true);
        }
        let error = task_error_map(caller, status, event);
        if let Err(e) = execute_task(
            platform,
            instance,
            &handler,
            parent_span.clone(),
            -1,
            Some(error),
        )
        .await
        {
            abort_flow(
                platform,
                instance,
                e.status(),
                Value::from(e.message()),
                parent_span,
            )
            .await;
        }
    } else {
        abort_flow(
            platform,
            instance,
            status,
            event.body().clone(),
            parent_span,
        )
        .await;
    }
}

/// Java `getTaskErrorMap`: `{task, code, message}` with the nested sub-flow
/// error unwrapped when present.
fn task_error_map(task_name: &str, status: i32, event: &EventEnvelope) -> Value {
    let mut message = event.body().clone();
    // nested error from a sub-flow: {type: error, status, message}
    if let Value::Map(entries) = &message {
        let view = MultiLevelMap::from_value(message.clone());
        let is_nested = view.get_element("type") == Some(Value::from("error"))
            && view.get_element("status").map(|v| str2int(&display(&v))) == Some(status)
            && view.key_exists("message")
            && (entries.len() == 3 || (entries.len() == 4 && view.key_exists("stack")));
        if is_nested {
            message = view.get_element("message").unwrap_or(Value::Nil);
        }
    }
    Value::Map(vec![
        (Value::from("task"), Value::from(task_name)),
        (Value::from("code"), Value::from(status as i64)),
        (Value::from("message"), message),
    ])
}

/// Java `getFlowErrorMap`: pass an already-shaped error map through,
/// otherwise wrap `{type: error, status, message}`.
fn flow_error_map(status: i32, message: Value) -> Value {
    if let Value::Map(_) = &message {
        let view = MultiLevelMap::from_value(message.clone());
        if view.get_element("type") == Some(Value::from("error"))
            && view.get_element("status").map(|v| str2int(&display(&v))) == Some(status)
            && view.key_exists("message")
        {
            return message;
        }
    }
    Value::Map(vec![
        (Value::from("type"), Value::from("error")),
        (Value::from("status"), Value::from(status as i64)),
        (Value::from("message"), message),
    ])
}

async fn abort_flow(
    platform: &Platform,
    instance: &Arc<FlowInstance>,
    status: i32,
    message: Value,
    parent_span: Option<String>,
) {
    if instance.claim_response() {
        let result = flow_error_map(status, message);
        instance.set_error_reference(result.clone());
        if let Some(reply_to) = &instance.reply_to {
            let mut error = EventEnvelope::new()
                .set_to(reply_to)
                .set_correlation_id(&instance.internal_correlation_id)
                .set_status(status)
                .set_raw_body(result);
            if let Some(span) = &parent_span {
                error = error.set_span_id(span);
            }
            if let (Some(trace_id), Some(path)) = (instance.trace_id(), instance.trace_path()) {
                error = error.set_trace(trace_id, path);
            }
            let po = PostOffice::new(platform);
            let _ = po.send(error).await;
        }
    }
    end_flow(platform, instance, false).await;
}

async fn end_flow(platform: &Platform, instance: &Arc<FlowInstance>, normal: bool) {
    if !instance.close() {
        return; // already closed
    }
    flows::close_flow_instance(&instance.id);
    let po = PostOffice::new(platform);
    // cancel the TTL watcher
    if let Some(timer) = instance.take_timeout_timer() {
        po.cancel_future_event(&timer);
    }
    // release task references
    {
        let metrics = instance.metrics.lock().expect("metrics");
        let mut refs = task_refs().lock().expect("task refs");
        for uuid in metrics.keys() {
            refs.remove(uuid);
        }
    }
    // flow summary span (only when the flow is traced — Java parity)
    if let Some(trace_id) = instance.trace_id() {
        let (task_info, count) = {
            let tasks = instance.tasks.lock().expect("tasks");
            let info: Vec<serde_json::Value> = tasks
                .iter()
                .map(|m| serde_json::json!({"name": m.route, "spent": m.elapsed()}))
                .collect();
            (info, tasks.len())
        };
        let elapsed = instance.elapsed_ms();
        let payload = serde_json::json!({
            "trace": {
                "origin": Platform::origin(),
                "id": trace_id,
                "service": SERVICE_NAME,
                "from": crate::manager::SERVICE_NAME,
                "exec_time": elapsed as f64,
                "path": instance.trace_path().unwrap_or("?"),
                "status": if normal { 200 } else { 400 },
                "success": normal,
                "span_id": platform_core::trace::new_span_id(),
            },
            "annotations": {
                "execution": format!("Run {count} task{} in {elapsed} ms",
                                     if count == 1 { "" } else { "s" }),
                "tasks": task_info,
                "flow": instance.template.id,
            },
        });
        let summary = EventEnvelope::new()
            .set_to("distributed.tracing")
            .set_raw_body(from_json(&payload));
        let _ = po.send(summary).await;
    }
    // end-of-flow advice to registered listeners
    for route in instance.end_flow_listeners() {
        if platform.has_route(&route) {
            let body = instance
                .error_reference()
                .unwrap_or_else(|| Value::Map(vec![(Value::from("type"), Value::from("end"))]));
            let advice = EventEnvelope::new()
                .set_to(&route)
                .set_header("type", "end")
                .set_header("flow_id", &instance.template.id)
                .set_header("instance_id", &instance.id)
                .set_correlation_id(&instance.internal_correlation_id)
                .set_raw_body(body);
            let _ = po.send(advice).await;
        } else {
            log::error!(
                "Unable to deliver end-of-flow advice because route '{route}' does not exist"
            );
        }
    }
}

/// Output data mapping + routing by execution type (Java `handleCallback`).
async fn handle_callback(
    platform: &Platform,
    instance: &Arc<FlowInstance>,
    reference: Option<TaskReference>,
    caller: &str,
    event: &EventEnvelope,
    parent_span: Option<String>,
) -> Result<(), AppError> {
    let task = &instance.template.tasks[caller];
    // build the consolidated view IN the instance dataset (scratch keys are
    // stripped after; model.* writes persist)
    let (response_parts, decision_value) = {
        let mut dataset = instance.dataset.lock().expect("dataset");
        dataset
            .set_element("status", Value::from(event.status() as i64))
            .map_err(|e| AppError::new(500, e))?;
        let header_map: Vec<(Value, Value)> = event
            .headers()
            .iter()
            .map(|(k, v)| (Value::from(k.as_str()), Value::from(v.as_str())))
            .collect();
        dataset
            .set_element("header", Value::Map(header_map))
            .map_err(|e| AppError::new(500, e))?;
        dataset
            .set_element("result", event.body().clone())
            .map_err(|e| AppError::new(500, e))?;
        let outcome = perform_output_mapping(&mut dataset, task);
        // capture the per-callback outputs before stripping scratch
        let response_parts = (
            dataset.get_element("output.status"),
            dataset.get_element("output.header"),
            dataset.get_element("output.body"),
        );
        let decision_value = dataset.get_element("decision");
        for key in SCRATCH_KEYS {
            dataset.remove_element(key);
        }
        outcome.map_err(|e| AppError::new(400, e))?;
        (response_parts, decision_value)
    };
    match task.execution.as_str() {
        "response" => {
            send_response(platform, instance, &parent_span, response_parts).await;
            if let Some(next) = task.next_steps.first() {
                execute_task(platform, instance, next, parent_span, -1, None).await?;
            }
        }
        "end" => {
            send_response(platform, instance, &parent_span, response_parts).await;
            end_flow(platform, instance, true).await;
        }
        "decision" => {
            handle_decision(
                platform,
                instance,
                reference,
                task,
                decision_value,
                parent_span,
            )
            .await?;
        }
        "sequential" => {
            if let Some(next) = task.next_steps.first() {
                execute_task(platform, instance, next, parent_span, -1, None).await?;
            }
        }
        "sink" => {} // terminal branch: no response, no next task
        other => {
            // parallel (E-5), fork (E-5), pipeline (E-6)
            return Err(AppError::new(
                500,
                format!("execution type '{other}' arrives with a later increment"),
            ));
        }
    }
    Ok(())
}

/// Apply the task's output mappings over the consolidated view.
fn perform_output_mapping(consolidated: &mut MultiLevelMap, task: &Task) -> Result<(), String> {
    for entry in &task.output {
        let Some(sep) = entry.rfind("->") else {
            continue;
        };
        let lhs = substitute_dynamic_index(entry[..sep].trim(), consolidated, false)?;
        let rhs = substitute_dynamic_index(entry[sep + 2..].trim(), consolidated, true)?;
        let input_like = lhs.starts_with("input.")
            || lhs.eq_ignore_ascii_case("input")
            || lhs.starts_with("model.")
            || lhs.starts_with("f:")
            || lhs.starts_with('$')
            || lhs == "header"
            || lhs.starts_with("header.")
            || lhs == "status"
            || lhs == "datatype"
            || lhs == "result"
            || lhs.starts_with("result.");
        let value = if input_like {
            let resolved = mapping::get_lhs_element(&lhs, consolidated)?;
            if resolved.is_none() {
                if consolidated.key_exists(&lhs) {
                    consolidated.set_element(&rhs, Value::Nil)?;
                } else {
                    consolidated.remove_element(&rhs);
                }
            }
            resolved
        } else {
            get_constant_value(&lhs)
        };
        if rhs.starts_with("file(") {
            write_output_file(&rhs, value.as_ref());
        } else if let Some(value) = value {
            set_output_rhs(consolidated, &rhs, value, entry)?;
        } else if rhs.starts_with("ext:") {
            return Err("external state machine arrives with increment E-7".to_string());
        }
    }
    Ok(())
}

/// Java `setOutputDataMappingRhs`: `output.status` must be a valid HTTP code
/// and `output.header` must be a map; violations log an ERROR and skip.
fn set_output_rhs(
    consolidated: &mut MultiLevelMap,
    rhs: &str,
    value: Value,
    entry: &str,
) -> Result<(), String> {
    if rhs == "output.status" {
        let status = str2int(&display(&value));
        if !(100..=599).contains(&status) {
            log::error!(
                "Invalid output mapping '{entry}' - expect: valid HTTP status code, actual: {status}"
            );
            return Ok(());
        }
    }
    if rhs == "output.header" && !matches!(value, Value::Map(_)) {
        log::error!("Invalid output mapping '{entry}' - expect: Map, actual: non-map value");
        return Ok(());
    }
    if rhs.starts_with("ext:") {
        return Err("external state machine arrives with increment E-7".to_string());
    }
    consolidated.set_element(rhs, value)
}

/// Java `setOutputDataMappingFile`: null deletes the target; text/bytes save
/// as-is; maps/lists save as JSON.
fn write_output_file(rhs: &str, value: Option<&Value>) {
    let fd = SimpleFileDescriptor::parse(rhs);
    let path = std::path::PathBuf::from(&fd.file_name);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match value {
        None => {
            if path.exists() {
                let _ = std::fs::remove_file(&path);
            }
        }
        Some(v) => {
            let bytes = get_binary_value(v);
            let result = if fd.mode == FileMode::Append {
                use std::io::Write;
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
                    .and_then(|mut f| f.write_all(&bytes))
            } else {
                std::fs::write(&path, &bytes)
            };
            if let Err(e) = result {
                log::error!("Unable to write {} - {e}", fd.file_name);
            }
        }
    }
}

async fn send_response(
    platform: &Platform,
    instance: &Arc<FlowInstance>,
    parent_span: &Option<String>,
    (status, headers, body): (Option<Value>, Option<Value>, Option<Value>),
) {
    if !instance.claim_response() {
        return;
    }
    let Some(reply_to) = &instance.reply_to else {
        return;
    };
    let mut result = EventEnvelope::new()
        .set_to(reply_to)
        .set_correlation_id(&instance.internal_correlation_id);
    if let Some(span) = parent_span {
        result = result.set_span_id(span);
    }
    if let (Some(trace_id), Some(path)) = (instance.trace_id(), instance.trace_path()) {
        result = result.set_trace(trace_id, path);
    }
    if let Some(status) = status {
        let value = str2int(&display(&status));
        if value > 0 {
            result = result.set_status(value);
        } else {
            log::warn!(
                "Unable to set status in response {}:{} - return status is negative",
                instance.template.id,
                instance.id
            );
        }
    }
    if let Some(Value::Map(entries)) = headers {
        for (k, v) in &entries {
            if let Some(key) = k.as_str() {
                result = result.set_header(key, &display(v));
            }
        }
    }
    result = result.set_raw_body(body.unwrap_or(Value::Nil));
    let po = PostOffice::new(platform);
    let _ = po.send(result).await;
}

async fn handle_decision(
    platform: &Platform,
    instance: &Arc<FlowInstance>,
    reference: Option<TaskReference>,
    task: &Task,
    decision_value: Option<Value>,
    parent_span: Option<String>,
) -> Result<(), AppError> {
    let next_tasks = &task.next_steps;
    let decision_number: i64 = match &decision_value {
        Some(Value::Boolean(true)) => 1,
        Some(Value::Boolean(false)) => 2,
        Some(other) => (str2int(&display(other)) as i64).max(1),
        None => 0,
    };
    if decision_number < 1 || decision_number as usize > next_tasks.len() {
        let shown = decision_value
            .map(|v| display(&v))
            .unwrap_or_else(|| "null".to_string());
        log::error!(
            "Flow {}:{} {} returned invalid decision ({shown})",
            instance.template.id,
            instance.id,
            task.service
        );
        abort_flow(
            platform,
            instance,
            500,
            Value::from(format!(
                "Task {} returned invalid decision ({shown})",
                task.service
            )),
            parent_span,
        )
        .await;
        return Ok(());
    }
    // '@retry|fallback.task' supports the resilience handler (Java getDecision)
    let choice = &next_tasks[(decision_number - 1) as usize];
    let mut next_list: Vec<String> = if choice.contains('|') {
        let mut parts: Vec<String> = choice
            .split(['|', ' '])
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();
        parts.sort();
        parts
    } else {
        vec![choice.clone()]
    };
    if decision_number == 1 && next_list.first().map(String::as_str) == Some("@retry") {
        let error_task = reference.and_then(|r| r.error_task);
        if let Some(error_task) = error_task {
            execute_task(platform, instance, &error_task, parent_span, -1, None).await?;
        } else if next_list.len() > 1 {
            let fallback = next_list.remove(1);
            execute_task(platform, instance, &fallback, parent_span, -1, None).await?;
        } else {
            abort_flow(
                platform,
                instance,
                500,
                Value::from(format!(
                    "Task {} does not have a previous task",
                    task.service
                )),
                parent_span,
            )
            .await;
        }
    } else {
        execute_task(platform, instance, &next_list[0], parent_span, -1, None).await?;
    }
    Ok(())
}

/// Execute one task: input mapping → dispatch the event to the composable
/// function (or a deferred delivery via `send_later`). Java `executeTask`.
pub(crate) async fn execute_task(
    platform: &Platform,
    instance: &Arc<FlowInstance>,
    process_name: &str,
    parent_span: Option<String>,
    seq: i32,
    error: Option<Value>,
) -> Result<(), AppError> {
    let Some(task) = instance.template.tasks.get(process_name) else {
        log::error!(
            "Unable to process flow {}:{} - missing task '{process_name}'",
            instance.template.id,
            instance.id
        );
        abort_flow(
            platform,
            instance,
            500,
            Value::from(format!("Service {process_name} not defined")),
            parent_span,
        )
        .await;
        return Ok(());
    };
    let error_task = match &error {
        Some(map) => MultiLevelMap::from_value(map.clone())
            .get_element("task")
            .map(|v| display(&v)),
        None => None,
    };
    // input data mapping over the state machine (error map as scratch)
    let (target, optional_headers, delay_ms) = {
        let mut dataset = instance.dataset.lock().expect("dataset");
        if let Some(error) = error {
            dataset
                .set_element("error", error)
                .map_err(|e| AppError::new(500, e))?;
        }
        let outcome = perform_input_mapping(&mut dataset, task);
        dataset.remove_element("error");
        let (target, headers) = outcome.map_err(|e| AppError::new(400, e))?;
        // deferred execution?
        let delay_ms: u64 = if task.delay > 0 {
            task.delay as u64
        } else if let Some(delay_var) = &task.delay_var {
            match dataset.get_element(delay_var) {
                Some(v) => {
                    let d = str2long(&display(&v)).max(1) as u64;
                    if d < instance.ttl_ms() {
                        d
                    } else {
                        log::warn!(
                            "Unable to schedule future task for {} because {delay_var} is invalid (TTL={}, delay={d})",
                            task.service,
                            instance.ttl_ms()
                        );
                        0
                    }
                }
                None => {
                    log::warn!(
                        "Unable to schedule future task for {} because {delay_var} does not exist",
                        task.service
                    );
                    0
                }
            }
        } else {
            0
        };
        (target, headers, delay_ms)
    };
    let uuid = uuid::Uuid::new_v4().simple().to_string();
    task_refs().lock().expect("task refs").insert(
        uuid.clone(),
        TaskReference {
            flow_instance_id: instance.id.clone(),
            process_id: task.service.clone(),
            error_task,
            span_id: None,
        },
    );
    let metrics = Arc::new(TaskMetrics::new(&task.service));
    instance
        .metrics
        .lock()
        .expect("metrics")
        .insert(uuid.clone(), metrics.clone());
    instance.tasks.lock().expect("tasks").push(metrics);
    let composite = if seq > 0 {
        format!("{uuid}#{seq}")
    } else {
        uuid
    };
    if task.function_route.starts_with("flow://") {
        // sub-flows arrive with increment E-7
        abort_flow(
            platform,
            instance,
            500,
            Value::from(format!(
                "{} not supported until increment E-7 (sub-flows)",
                task.function_route
            )),
            parent_span,
        )
        .await;
        return Ok(());
    }
    // the '*' wildcard maps a whole object as the function input body
    let body = {
        let map = target.to_value();
        match target.get_element("*") {
            Some(whole) => whole,
            None => map,
        }
    };
    let mut event = EventEnvelope::new()
        .set_to(&task.function_route)
        .set_reply_to(SERVICE_NAME)
        .set_correlation_id(&composite)
        .set_raw_body(body);
    if let Some(span) = &parent_span {
        event = event.set_span_id(span);
    }
    if let (Some(trace_id), Some(path)) = (instance.trace_id(), instance.trace_path()) {
        event = event.set_trace(trace_id, path);
    }
    for (k, v) in &optional_headers {
        event = event.set_header(k, v);
    }
    // the read-only business correlation-id header is stamped LAST so a
    // mapped header cannot override the framework value (Java parity)
    event = event.set_header(MY_CORRELATION_ID, &instance.business_correlation_id);
    let po = PostOffice::new(platform);
    if delay_ms > 0 {
        po.send_later(event, Duration::from_millis(delay_ms));
        Ok(())
    } else {
        po.send(event).await
    }
}

/// Apply the task's input mappings; returns the function-input body tree and
/// the optional event headers. Java `performInputDataMapping`.
fn perform_input_mapping(
    dataset: &mut MultiLevelMap,
    task: &Task,
) -> Result<(MultiLevelMap, Vec<(String, String)>), String> {
    let mut target = MultiLevelMap::new();
    let mut optional_headers: Vec<(String, String)> = Vec::new();
    for entry in &task.input {
        let Some(sep) = entry.rfind("->") else {
            continue;
        };
        let mut lhs = substitute_dynamic_index(entry[..sep].trim(), dataset, false)?;
        let rhs = substitute_dynamic_index(entry[sep + 2..].trim(), dataset, true)?;
        let input_like = lhs.starts_with("input.")
            || lhs.eq_ignore_ascii_case("input")
            || lhs == "datatype"
            || lhs.starts_with("model.")
            || lhs.starts_with("error.")
            || lhs.starts_with("f:")
            || lhs.starts_with('$');
        if lhs.starts_with("input.header.") {
            lhs = lhs.to_lowercase();
        }
        let value = if input_like {
            mapping::get_lhs_element(&lhs, dataset)?
        } else {
            get_constant_value(&lhs)
        };
        if rhs.starts_with("ext:") {
            return Err("external state machine arrives with increment E-7".to_string());
        } else if rhs.starts_with("model.") {
            // model writes go straight into the state machine
            if input_like {
                match &value {
                    None if !dataset.key_exists(&lhs) => {
                        dataset.remove_element(&rhs);
                    }
                    None => dataset.set_element(&rhs, Value::Nil)?,
                    Some(v) => dataset.set_element(&rhs, v.clone())?,
                }
            } else {
                match get_constant_value(&lhs) {
                    Some(v) => dataset.set_element(&rhs, v)?,
                    None => dataset.remove_element(&rhs),
                }
            }
        } else if input_like {
            match value {
                Some(v) => set_input_rhs(&mut target, &mut optional_headers, &rhs, v, entry)?,
                None => {
                    if dataset.key_exists(&lhs) {
                        target.set_element(&rhs, Value::Nil)?;
                    }
                }
            }
        } else {
            // constant into the function input (or an event header)
            if let Some(key) = rhs.strip_prefix("header.") {
                if let Some(v) = get_constant_value(&lhs) {
                    if !key.is_empty() {
                        optional_headers.push((key.to_string(), display(&v)));
                    }
                }
            } else {
                match get_constant_value(&lhs) {
                    Some(v) => target.set_element(&rhs, v)?,
                    None => target.remove_element(&rhs),
                }
            }
        }
    }
    Ok((target, optional_headers))
}

/// Java `setInputDataMappingRhs`: `*` reloads the whole body; `header` /
/// `header.*` become event headers; everything else lands in the body tree.
fn set_input_rhs(
    target: &mut MultiLevelMap,
    optional_headers: &mut Vec<(String, String)>,
    rhs: &str,
    value: Value,
    entry: &str,
) -> Result<(), String> {
    if rhs == "*" {
        if matches!(value, Value::Map(_)) {
            *target = MultiLevelMap::from_value(value);
        } else {
            target.set_element("*", value)?;
        }
    } else if rhs == "header" {
        if let Value::Map(entries) = value {
            for (k, v) in &entries {
                if let Some(key) = k.as_str() {
                    optional_headers.push((key.to_string(), display(v)));
                }
            }
        } else {
            log::error!("Invalid input mapping '{entry}' - expect: Map, actual: non-map value");
        }
    } else if let Some(key) = rhs.strip_prefix("header.") {
        if !key.is_empty() {
            optional_headers.push((key.to_string(), display(&value)));
        }
    } else {
        target.set_element(rhs, value)?;
    }
    Ok(())
}

/// Java `substituteDynamicIndex`: `{model.key}` interpolation, then dynamic
/// list indices (`[model.n]` → the resolved integer; a numeric RHS index must
/// not be negative), and finally the reserved-key re-check on a dynamically
/// resolved RHS (the compiler cannot see runtime-substituted targets).
fn substitute_dynamic_index(
    statement: &str,
    source: &MultiLevelMap,
    is_rhs: bool,
) -> Result<String, String> {
    let mut text = substitute_runtime_vars(statement, source);
    if text.contains('[') && text.contains(']') {
        let mut output = String::new();
        let mut rest = text.as_str();
        loop {
            let (Some(open), Some(close)) = (rest.find('['), rest.find(']')) else {
                output.push_str(rest);
                break;
            };
            if close < open {
                output.push_str(rest);
                break;
            }
            output.push_str(&rest[..=open]);
            let index = rest[open + 1..close].trim();
            if index.starts_with("model.") && !index.ends_with('.') {
                let resolved = source
                    .get_element(index)
                    .map(|v| display(&v))
                    .unwrap_or_else(|| "null".to_string());
                let n = str2int(&resolved);
                if is_rhs && n < 0 {
                    return Err(format!("Cannot set RHS to negative index - {statement}"));
                }
                output.push_str(&n.to_string());
            } else {
                if is_rhs && !index.is_empty() && str2int(index) < 0 {
                    return Err(format!("Cannot set RHS to negative index - {statement}"));
                }
                output.push_str(index);
            }
            output.push(']');
            rest = &rest[close + 1..];
        }
        text = output;
    }
    // a dynamic RHS bypasses compile-time validation — re-check it here
    if is_rhs && text != statement {
        if let Some(reserved) = validator::reserved_model_key_violation(&text) {
            return Err(format!(
                "Cannot set RHS to the reserved state-machine key '{reserved}' - {statement}"
            ));
        }
    }
    Ok(text)
}

/// The programmatic flow API — Rust port of `com.accenture.adapters.FlowExecutor`.
pub struct FlowExecutor;

impl FlowExecutor {
    /// Fire-and-forget flow launch (Java `launch`). `dataset` is the flow
    /// input (a map value, typically with `body`/`header` keys); the business
    /// correlation id becomes `model.cid`.
    pub async fn launch(
        platform: &Platform,
        flow_id: &str,
        dataset: Value,
        business_correlation_id: &str,
        trace: Option<(&str, &str)>,
    ) -> Result<(), AppError> {
        let po = PostOffice::new(platform);
        po.send(Self::launch_event(
            flow_id,
            dataset,
            business_correlation_id,
            trace,
        ))
        .await
    }

    /// Launch a flow and wait for its response (Java `request`).
    pub async fn request(
        platform: &Platform,
        flow_id: &str,
        dataset: Value,
        business_correlation_id: &str,
        timeout: Duration,
        trace: Option<(&str, &str)>,
    ) -> Result<EventEnvelope, AppError> {
        let po = PostOffice::new(platform);
        po.request(
            Self::launch_event(flow_id, dataset, business_correlation_id, trace),
            timeout,
        )
        .await
    }

    fn launch_event(
        flow_id: &str,
        dataset: Value,
        business_correlation_id: &str,
        trace: Option<(&str, &str)>,
    ) -> EventEnvelope {
        let mut event = EventEnvelope::new()
            .set_to(crate::manager::SERVICE_NAME)
            .set_header("flow_id", flow_id)
            .set_header(
                crate::manager::BUSINESS_CORRELATION_ID,
                business_correlation_id,
            )
            .set_correlation_id(&uuid::Uuid::new_v4().simple().to_string())
            .set_raw_body(dataset);
        if let Some((trace_id, trace_path)) = trace {
            event = event.set_trace(trace_id, trace_path);
        }
        event
    }
}
