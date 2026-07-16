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

//! Rust port of `com.accenture.automation.CompileFlows` — the flow compiler.
//!
//! Runs as a `#[before_application(sequence = 5)]` hook (essential services
//! are 0, the plugin loader 3, user code ≥ 6 — the Java sequence contract):
//! reads `yaml.flow.automation` (default `classpath:/flows.yaml`), loads each
//! listed flow file, validates the full grammar (`flow-grammar.md` mirrors
//! this validation), and registers the compiled [`Flow`]s.
//!
//! Failure semantics (Java parity, load-time resilience):
//! - an unreadable *list* file is a WARN and the location is skipped;
//! - an invalid *flow* is an ERROR and that flow is skipped;
//! - an invalid *data mapping* drops the offending TASK (ERROR log) while the
//!   rest of the flow still loads — the runtime then fails fast if the flow
//!   reaches the missing task.

use platform_core::{AppConfigReader, ConfigReader, ConfigValue};

use crate::converter;
use crate::flows;
use crate::model::{Flow, Task};
use crate::util::{duration_in_seconds, is_numeric, split, str2long};
use crate::validator::{self, MAP_TO};

const SPACED_MAP_TO: &str = " -> ";
const EXECUTION_TYPES: &[&str] = &[
    "decision",
    "response",
    "end",
    "sequential",
    "parallel",
    "pipeline",
    "fork",
    "sink",
];

/// Compile and register every flow listed by `yaml.flow.automation`.
/// Returns the sorted ids of all flows in the registry (Java logs the same).
pub fn compile_flows() -> Vec<String> {
    let config = AppConfigReader::get_instance();
    let locations = config.get_property_or("yaml.flow.automation", "classpath:/flows.yaml");
    for path in split(&locations, ", ") {
        match ConfigReader::load(&path) {
            Ok(reader) => {
                log::info!("Loading event scripts from {path}");
                let prefix = reader.get_property_or("location", "classpath:/flows/");
                if let Some(ConfigValue::List(list)) = reader.get("flows") {
                    for name in unique_flows(&reader, list.len()) {
                        if let Err(e) = create_flow(&prefix, &name) {
                            log::error!("Unable to parse {name} - {e}");
                        }
                    }
                }
            }
            Err(e) => log::warn!("Unable to load Event Scripts from {path} - {e}"),
        }
    }
    let ids = flows::get_all_flows();
    for id in &ids {
        log::info!("Loaded {id}");
    }
    log::info!("Event scripts deployed: {}", ids.len());
    ids
}

/// Java `getUniqueFlows`: de-duplicate, keep only `.yml`/`.yaml`, sort.
fn unique_flows(reader: &ConfigReader, count: usize) -> Vec<String> {
    let mut unique = std::collections::HashSet::new();
    for i in 0..count {
        if let Some(name) = reader.get_property(&format!("flows[{i}]")) {
            if name.ends_with(".yml") || name.ends_with(".yaml") {
                unique.insert(name);
            } else {
                log::error!("Ignored {name} because it does not have .yml or .yaml file extension");
            }
        }
    }
    let mut ordered: Vec<String> = unique.into_iter().collect();
    ordered.sort();
    ordered
}

fn create_flow(prefix: &str, name: &str) -> Result<(), String> {
    log::info!("Parsing {name}");
    let reader = ConfigReader::load(&format!("{prefix}{name}")).map_err(|e| e.to_string())?;
    let id = reader.get("flow.id");
    let description = reader.get("flow.description");
    let ttl = reader.get("flow.ttl");
    let first_task = reader.get("first.task");
    match (id, description, ttl, first_task) {
        (
            Some(ConfigValue::Text(flow_id)),
            Some(ConfigValue::Text(_)),
            Some(ConfigValue::Text(ttl)),
            Some(ConfigValue::Text(first)),
        ) => {
            if flows::flow_exists(&flow_id) {
                return Err(format!("Flow '{flow_id}' already exists"));
            }
            let exception = match reader.get("flow.exception") {
                Some(ConfigValue::Text(e)) => Some(e),
                _ => None,
            };
            let ext_state = match reader.get("external.state.machine") {
                Some(ConfigValue::Text(e)) => Some(e),
                _ => None,
            };
            // minimum 1 second for TTL
            let ttl_seconds = duration_in_seconds(&ttl).max(1) as u64;
            let mut entry = Flow::new(&flow_id, &first, ext_state, ttl_seconds * 1000, exception);
            let task_count = match reader.get("tasks") {
                Some(ConfigValue::List(list)) => list.len(),
                _ => 0,
            };
            if task_count == 0 {
                return Err("'tasks' section is empty or invalid".to_string());
            }
            validate_entry(name, &mut entry, &reader, task_count)
        }
        _ => Err("check flow.id, flow.description, flow.ttl, first.task".to_string()),
    }
}

fn validate_entry(
    name: &str,
    entry: &mut Flow,
    reader: &ConfigReader,
    task_count: usize,
) -> Result<(), String> {
    let mut end_task_found = false;
    for i in 0..task_count {
        let md = FlowConfigMetadata::read(reader, i)?;
        let mut task = Task::new(
            &md.unique_task_name,
            md.function_route.as_deref(),
            &md.execution,
        );
        validate_delay_parameter(&md, entry, &mut task)?;
        if let Some(te) = &md.task_exception {
            task.exception_task = Some(te.clone());
        }
        if md.execution == "end" {
            end_task_found = true;
        } else {
            set_fork_join_if_any(&mut task, &md, reader, i)?;
            set_non_sink_task_if_any(&mut task, &md, reader, i)?;
            set_pipeline_if_any(&mut task, &md, reader, i)?;
        }
        validate_input_output(name, entry, task, &md, reader, i)?;
    }
    if end_task_found {
        finalize_entry(entry)
    } else {
        Err("flow must have at least one end task".to_string())
    }
}

/// Java `validateInputOutput`: an invalid mapping drops the TASK (ERROR log),
/// not the flow.
fn validate_input_output(
    name: &str,
    entry: &mut Flow,
    mut task: Task,
    md: &FlowConfigMetadata,
    reader: &ConfigReader,
    i: usize,
) -> Result<(), String> {
    let mut input_list = Vec::new();
    for j in 0..md.input_count {
        input_list.push(
            reader
                .get_property(&format!("tasks[{i}].input[{j}]"))
                .unwrap_or_default(),
        );
    }
    let mut output_list = Vec::new();
    for j in 0..md.output_count {
        output_list.push(
            reader
                .get_property(&format!("tasks[{i}].output[{j}]"))
                .unwrap_or_default(),
        );
    }
    if valid_input_mapping(name, &input_list, &mut task, md)
        && valid_output_mapping(name, &output_list, &mut task, md)
    {
        entry.add_task(task);
    }
    Ok(())
}

fn valid_input_mapping(
    name: &str,
    input_list: &[String],
    task: &mut Task,
    md: &FlowConfigMetadata,
) -> bool {
    for raw in filter_data_mapping(input_list) {
        // convert deprecated "simple type matching" syntax to plugin syntax
        let line = converter::convert(&raw);
        if line != raw {
            log::warn!(
                "Deprecated input syntax in task {} of {name} - '{raw}' converted to '{line}'",
                md.unique_task_name
            );
        }
        if !validator::valid_input(&line) {
            log::error!(
                "Skip invalid task {} in {name} that has invalid input mapping - {line}",
                md.unique_task_name
            );
            return false;
        }
        let sep = line.rfind(MAP_TO).expect("validated");
        let rhs = line[sep + 2..].trim();
        if rhs.starts_with("input.") || rhs == "input" {
            log::warn!(
                "Task {} in {name} uses input namespace in right-hand-side - {line}",
                md.unique_task_name
            );
        }
        if rejected_reserved_target("input", rhs, name, md, &line) {
            return false;
        }
        if line.contains("model.parent.") || line.contains("model.root.") {
            task.input_parent_ref = true;
        }
        task.input.push(line);
    }
    true
}

fn valid_output_mapping(
    name: &str,
    output_list: &[String],
    task: &mut Task,
    md: &FlowConfigMetadata,
) -> bool {
    let is_decision = md.execution == "decision";
    for raw in filter_data_mapping(output_list) {
        let line = converter::convert(&raw);
        if line != raw {
            log::warn!(
                "Deprecated output syntax in task {} of {name} - '{raw}' converted to '{line}'",
                md.unique_task_name
            );
        }
        if validator::valid_output(&line, is_decision) {
            let rhs = line[line.rfind(MAP_TO).expect("validated") + 2..].trim();
            if rejected_reserved_target("output", rhs, name, md, &line) {
                return false;
            }
            if line.contains("model.parent.") || line.contains("model.root.") {
                task.output_parent_ref = true;
            }
            task.output.push(line);
        } else {
            log::error!(
                "Skip invalid task {} in {name} that has invalid output mapping - {line}",
                md.unique_task_name
            );
            return false;
        }
    }
    if is_decision && task.next_steps.len() < 2 {
        log::error!(
            "Decision task {} in {name} must have at least 2 next tasks",
            md.unique_task_name
        );
        return false;
    }
    true
}

fn rejected_reserved_target(
    direction: &str,
    rhs: &str,
    name: &str,
    md: &FlowConfigMetadata,
    line: &str,
) -> bool {
    if let Some(reserved) = validator::reserved_model_key_violation(rhs) {
        log::error!(
            "Skip invalid task ({direction}) {} in {name} that overwrites the reserved \
             state-machine key '{reserved}' - {line}",
            md.unique_task_name
        );
        return true;
    }
    false
}

fn finalize_entry(entry: &mut Flow) -> Result<(), String> {
    let ext_found = entry
        .tasks
        .values()
        .any(|t| has_external_state(&t.input) || has_external_state(&t.output));
    let incomplete = entry
        .tasks
        .values()
        .any(|t| has_incomplete_mapping(&t.input) || has_incomplete_mapping(&t.output));
    if ext_found && entry.external_state_machine.is_none() {
        Err("flow is missing external.state.machine".to_string())
    } else if incomplete {
        Err("flow has invalid data mappings".to_string())
    } else {
        flows::add_flow(std::mem::replace(entry, Flow::new("", "", None, 0, None)));
        Ok(())
    }
}

fn has_external_state(mapping: &[String]) -> bool {
    mapping.iter().any(|m| {
        m.find(MAP_TO).is_some_and(|sep| {
            let rhs = m[sep + 2..].trim();
            rhs.starts_with("ext:") && rhs != "ext:"
        })
    })
}

fn has_incomplete_mapping(mapping: &[String]) -> bool {
    mapping.iter().any(|m| match m.find(MAP_TO) {
        Some(sep) => {
            let lhs = m[..sep].trim();
            let rhs = m[sep + 2..].trim();
            lhs.ends_with('.') || rhs.ends_with('.') || rhs.ends_with(':')
        }
        None => true,
    })
}

fn set_fork_join_if_any(
    task: &mut Task,
    md: &FlowConfigMetadata,
    reader: &ConfigReader,
    i: usize,
) -> Result<(), String> {
    if md.execution == "fork" {
        match reader.get(&format!("tasks[{i}].join")) {
            Some(ConfigValue::Text(join)) => {
                task.join_task = Some(join);
                Ok(())
            }
            _ => Err(format!(
                "invalid task {}. Missing a join task",
                md.unique_task_name
            )),
        }
    } else {
        Ok(())
    }
}

fn set_non_sink_task_if_any(
    task: &mut Task,
    md: &FlowConfigMetadata,
    reader: &ConfigReader,
    i: usize,
) -> Result<(), String> {
    if md.execution == "sink" {
        return Ok(());
    }
    match reader.get_or(&format!("tasks[{i}].next"), ConfigValue::List(Vec::new())) {
        ConfigValue::List(list) => {
            if list.is_empty() {
                return Err(format!(
                    "Invalid task {}. Missing a list of next tasks",
                    md.unique_task_name
                ));
            }
            let next_tasks: Vec<String> = list.iter().map(|v| v.to_display_string()).collect();
            if next_tasks.len() > 1 && (md.execution == "sequential" || md.execution == "pipeline")
            {
                return Err(format!(
                    "Invalid {} task {}. Expected one next task, Actual: {}",
                    md.execution,
                    md.unique_task_name,
                    next_tasks.len()
                ));
            }
            validate_source_model_key(task, md, &next_tasks)?;
            task.next_steps.extend(next_tasks);
            Ok(())
        }
        _ => Err(format!(
            "invalid task {}. 'next' should be a list",
            md.unique_task_name
        )),
    }
}

fn validate_source_model_key(
    task: &mut Task,
    md: &FlowConfigMetadata,
    next_tasks: &[String],
) -> Result<(), String> {
    let Some(source) = md.source.as_deref().filter(|s| !s.is_empty()) else {
        return Ok(());
    };
    if next_tasks.len() > 1 {
        return Err(format!(
            "Invalid {} task {}. Expected one next task if dynamic model source is used, Actual: {}",
            md.execution,
            md.unique_task_name,
            next_tasks.len()
        ));
    }
    if source.starts_with("model.") && !source.ends_with('.') {
        task.source_model_key = Some(source.to_string());
        Ok(())
    } else {
        Err(format!(
            "Invalid {} task {}. Source must start with model namespace, Actual: {source}",
            md.execution, md.unique_task_name
        ))
    }
}

fn set_pipeline_if_any(
    task: &mut Task,
    md: &FlowConfigMetadata,
    reader: &ConfigReader,
    i: usize,
) -> Result<(), String> {
    if md.execution != "pipeline" {
        return Ok(());
    }
    match reader.get_or(
        &format!("tasks[{i}].pipeline"),
        ConfigValue::List(Vec::new()),
    ) {
        ConfigValue::List(list) => {
            if list.is_empty() {
                return Err(format!(
                    "invalid task {}. Missing a list of pipeline steps",
                    md.unique_task_name
                ));
            }
            task.pipeline_steps
                .extend(list.iter().map(|v| v.to_display_string()));
            handle_loop_statement_if_any(task, md)?;
            handle_loop_conditions(task, md)
        }
        _ => Err(format!(
            "invalid task {}. 'pipeline' should be a list",
            md.unique_task_name
        )),
    }
}

fn handle_loop_statement_if_any(task: &mut Task, md: &FlowConfigMetadata) -> Result<(), String> {
    let Some(statement) = &md.loop_statement else {
        return Ok(());
    };
    let Some(bracket) = statement.find('(') else {
        return Err(format!(
            "invalid task {}. Please check loop.statement",
            md.unique_task_name
        ));
    };
    let loop_type = statement[..bracket].trim();
    if loop_type != "for" && loop_type != "while" {
        return Err(format!(
            "invalid task {}. loop.statement must be 'for' or 'while'",
            md.unique_task_name
        ));
    }
    task.loop_type = loop_type.to_string();
    let parts = split(&statement[bracket + 1..], "(;)");
    if loop_type == "for" {
        handle_for_loop(task, md, &parts)
    } else {
        handle_while_loop(task, md, &parts)
    }
}

fn handle_while_loop(
    task: &mut Task,
    md: &FlowConfigMetadata,
    parts: &[String],
) -> Result<(), String> {
    if parts.len() != 1 {
        return Err(format!(
            "invalid task {}. 'while' loop should have only one value",
            md.unique_task_name
        ));
    }
    let model_key = parts[0].trim();
    if !model_key.starts_with("model.") || model_key.contains('=') || model_key.contains(' ') {
        return Err(format!(
            "invalid task {}. 'while' should use a model key",
            md.unique_task_name
        ));
    }
    task.while_model_key = Some(model_key.to_string());
    Ok(())
}

fn handle_for_loop(
    task: &mut Task,
    md: &FlowConfigMetadata,
    parts: &[String],
) -> Result<(), String> {
    if parts.len() < 2 || parts.len() > 3 {
        return Err(format!(
            "invalid task {}. 'for' loop should have 2 or 3 segments",
            md.unique_task_name
        ));
    }
    if parts.len() == 2 {
        task.comparator = for_part2(&parts[0]);
        task.sequencer = for_part3(&parts[1]);
    } else {
        let initializer = for_part1(&parts[0]);
        if initializer.is_empty() {
            return Err(format!(
                "invalid task {}. check for-loop initializer. \
                 e.g. 'for (model.n = 0; model.n < 3; model.n++)'",
                md.unique_task_name
            ));
        }
        task.init = initializer;
        task.comparator = for_part2(&parts[1]);
        task.sequencer = for_part3(&parts[2]);
    }
    if task.comparator.is_empty() || task.sequencer.is_empty() {
        return Err(format!(
            "invalid task {}. check for-loop syntax. \
             e.g. 'for (model.n = 0; model.n < 3; model.n++)'",
            md.unique_task_name
        ));
    }
    if !valid_for_statement(&task.comparator, &task.sequencer) {
        return Err(format!(
            "invalid task {}. 'for' loop has invalid comparator or sequencer",
            md.unique_task_name
        ));
    }
    Ok(())
}

fn handle_loop_conditions(task: &mut Task, md: &FlowConfigMetadata) -> Result<(), String> {
    for statement in &md.loop_conditions {
        let condition = get_condition(statement);
        if condition.len() == 2 {
            task.conditions.push(condition);
        } else {
            return Err(format!(
                "invalid task {}. loop condition syntax error - {statement}",
                md.unique_task_name
            ));
        }
    }
    Ok(())
}

/// Java `getCondition`: `'if (model.key) break|continue'` → `[model.key, action]`.
fn get_condition(statement: &str) -> Vec<String> {
    let parts = split(statement, " ()");
    if parts.len() == 3
        && parts[0] == "if"
        && parts[1].starts_with("model.")
        && (parts[2] == "continue" || parts[2] == "break")
    {
        vec![parts[1].clone(), parts[2].clone()]
    } else {
        Vec::new()
    }
}

/// Java `getForPart1`: `model.n = 0` → `[model.n, 0]`.
fn for_part1(text: &str) -> Vec<String> {
    let parts = split(&text.replace('=', " = "), " ");
    if parts.len() == 3
        && parts[0].starts_with("model.")
        && is_numeric(&parts[2])
        && parts[1] == "="
    {
        vec![parts[0].clone(), parts[2].clone()]
    } else {
        Vec::new()
    }
}

/// Java `getForPart2`: `model.n < 3` → `[model.n, <, 3]`.
fn for_part2(text: &str) -> Vec<String> {
    let spaced = if text.contains(">=") {
        text.replace(">=", " >= ")
    } else if text.contains("<=") {
        text.replace("<=", " <= ")
    } else if text.contains('<') {
        text.replace('<', " < ")
    } else if text.contains('>') {
        text.replace('>', " > ")
    } else {
        text.to_string()
    };
    let parts = split(&spaced, " ");
    if parts.len() == 3
        && (parts[0].starts_with("model.") || is_numeric(&parts[0]))
        && (parts[2].starts_with("model.") || is_numeric(&parts[2]))
        && ["<", ">", "<=", ">="].contains(&parts[1].as_str())
    {
        parts
    } else {
        Vec::new()
    }
}

/// Java `getForPart3`: `model.n++` / `++model.n` → `[model.n, ++]`.
fn for_part3(text: &str) -> Vec<String> {
    let s = text.trim();
    let plus = if s.ends_with("++") || s.starts_with("++") {
        true
    } else if s.ends_with("--") || s.starts_with("--") {
        false
    } else {
        return Vec::new();
    };
    // reject a sequencer decorated on both sides (e.g. ++model.n++)
    if (s.starts_with('+') || s.starts_with('-')) && (s.ends_with('+') || s.ends_with('-')) {
        return Vec::new();
    }
    let key = s.replace(['+', '-'], " ").trim().to_string();
    if key.starts_with("model.") {
        vec![key, if plus { "++" } else { "--" }.to_string()]
    } else {
        Vec::new()
    }
}

/// Java `validForStatement`: the sequencer's model key must appear in the
/// comparator, and a comparator of two identical keys is invalid.
fn valid_for_statement(comparator: &[String], sequencer: &[String]) -> bool {
    let keys: Vec<&String> = comparator
        .iter()
        .filter(|k| k.starts_with("model."))
        .collect();
    if keys.is_empty() {
        return false;
    }
    if keys.len() == 2 && keys[0] == keys[1] {
        return false;
    }
    keys.iter().any(|k| Some(*k) == sequencer.first())
}

fn validate_delay_parameter(
    md: &FlowConfigMetadata,
    entry: &Flow,
    task: &mut Task,
) -> Result<(), String> {
    let Some(delay) = md.delay.as_deref().filter(|d| !d.is_empty()) else {
        return Ok(());
    };
    // the "ms" suffix is for documentation purpose only
    let delay = delay.strip_suffix("ms").map(str::trim).unwrap_or(delay);
    if is_numeric(delay) {
        // delay must be positive and less than the flow TTL
        let n = str2long(delay).max(1);
        if (n as u64) < entry.ttl {
            task.delay = n;
            Ok(())
        } else {
            Err(format!(
                "invalid task {}. delay must be less than TTL",
                md.unique_task_name
            ))
        }
    } else if split(delay, ".").len() > 1 && delay.starts_with("model.") {
        task.delay_var = Some(delay.to_string());
        Ok(())
    } else {
        Err(format!(
            "invalid task {}. delay variable must starts with 'model.'",
            md.unique_task_name
        ))
    }
}

/// Java `filterDataMapping`: normalize entries — a `text(` constant keeps the
/// 2-part form (its content may contain `->`); other entries decompose 3-part
/// mappings and normalize `!model.x` negation. An entry with bad shape is
/// replaced by a poison string that fails validation downstream (Java parity).
fn filter_data_mapping(entries: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for line in entries {
        let entry = line.trim();
        if entry.starts_with("text(") {
            result.push(filter_mapping(entry));
        } else {
            filter_parts(entry, &mut result);
        }
    }
    result
}

fn filter_parts(entry: &str, result: &mut Vec<String>) {
    let parts = get_parts(entry);
    match parts.len() {
        2 => result.push(filter_mapping(&format!(
            "{}{SPACED_MAP_TO}{}",
            parts[0], parts[1]
        ))),
        3 => handle_three_part_mapping(&parts, result),
        _ => {
            result.push("Syntax must be (LHS -> RHS) or (LHS -> model.variable -> RHS)".to_string())
        }
    }
}

fn get_parts(text: &str) -> Vec<String> {
    let mut entry = text;
    let mut parts = Vec::new();
    while let Some(sep) = entry.find(MAP_TO) {
        parts.push(entry[..sep].trim().to_string());
        entry = entry[sep + 2..].trim();
    }
    parts.push(entry.to_string());
    parts
}

/// 3-part mapping decomposes into two 2-part entries; the middle must be a
/// model variable, whose type qualifier is dropped from the second LHS.
fn handle_three_part_mapping(parts: &[String], result: &mut Vec<String>) {
    if parts[1].starts_with("model.") || parts[1].starts_with("!model.") {
        result.push(filter_mapping(&format!(
            "{}{SPACED_MAP_TO}{}",
            parts[0], parts[1]
        )));
        let second_lhs = trim_type_qualifier(&parts[1]);
        result.push(filter_mapping(&format!(
            "{second_lhs}{SPACED_MAP_TO}{}",
            parts[2]
        )));
    } else {
        result.push("3-part data mapping must have model variable as the middle part".to_string());
    }
}

fn filter_mapping(mapping: &str) -> String {
    let text = mapping.trim();
    let Some(sep) = text.rfind(MAP_TO) else {
        return mapping.to_string();
    };
    let mut lhs = text[..sep].trim().to_string();
    let mut rhs = text[sep + 2..].trim().to_string();
    if lhs.starts_with("!model.") {
        lhs = normalized_negate_type_mapping(&lhs);
    }
    if rhs.starts_with("!model.") {
        rhs = normalized_negate_type_mapping(&rhs);
    }
    format!("{lhs}{SPACED_MAP_TO}{rhs}")
}

/// `!model.something` → `model.something:!` (any existing qualifier dropped).
fn normalized_negate_type_mapping(negate: &str) -> String {
    let base = match negate.find(':') {
        Some(colon) => &negate[1..colon],
        None => &negate[1..],
    };
    format!("{base}:!")
}

fn trim_type_qualifier(lhs: &str) -> String {
    let step1 = lhs.strip_prefix('!').unwrap_or(lhs);
    match step1.find(':') {
        Some(colon) => step1[..colon].to_string(),
        None => step1.to_string(),
    }
}

/// Java inner class `FlowConfigMetadata`: one task's raw configuration with
/// structural validation (throws = the whole flow is rejected).
struct FlowConfigMetadata {
    input_count: usize,
    output_count: usize,
    function_route: Option<String>,
    execution: String,
    delay: Option<String>,
    task_exception: Option<String>,
    loop_statement: Option<String>,
    loop_conditions: Vec<String>,
    unique_task_name: String,
    source: Option<String>,
}

impl FlowConfigMetadata {
    fn read(reader: &ConfigReader, i: usize) -> Result<Self, String> {
        let input = reader.get_or(&format!("tasks[{i}].input"), ConfigValue::List(Vec::new()));
        let output = reader.get_or(&format!("tasks[{i}].output"), ConfigValue::List(Vec::new()));
        let task_name = reader.get_property(&format!("tasks[{i}].name"));
        let function_route = reader.get_property(&format!("tasks[{i}].process"));
        let description = reader.get(&format!("tasks[{i}].description"));
        let execution = reader.get(&format!("tasks[{i}].execution"));
        let unique_task_name = task_name.or_else(|| function_route.clone());
        let md_name = match &unique_task_name {
            Some(name) if !name.trim().is_empty() => name.clone(),
            _ => return Err(format!("task[{i}]. task name must not be empty")),
        };
        let input_count = match &input {
            ConfigValue::List(list) => list.len(),
            _ => return Err(format!("task {md_name}. input must be a list")),
        };
        let output_count = match &output {
            ConfigValue::List(list) => list.len(),
            _ => return Err(format!("task {md_name}. output must be a list")),
        };
        match description {
            Some(ConfigValue::Text(d)) if !d.trim().is_empty() => {}
            _ => return Err(format!("task {md_name}. description must not be empty")),
        }
        let execution = match execution {
            Some(ConfigValue::Text(e)) if !e.trim().is_empty() => e,
            _ => return Err(format!("task {md_name}. execution type must not be empty")),
        };
        if !EXECUTION_TYPES.contains(&execution.as_str()) {
            return Err(format!(
                "task {md_name}. execution type '{execution}' must be one of {EXECUTION_TYPES:?}"
            ));
        }
        // sub-flow syntax rules
        if md_name.contains("://") && !md_name.starts_with("flow://") {
            return Err(format!(
                "invalid task name={md_name}. Syntax is flow://{{flow-name}}"
            ));
        }
        if let Some(route) = &function_route {
            if route.contains("://") && !route.starts_with("flow://") {
                return Err(format!(
                    "invalid task process={route}. Syntax is flow://{{flow-name}}"
                ));
            }
            if md_name.starts_with("flow://") && !route.starts_with("flow://") {
                return Err(format!(
                    "invalid task process={route}. process tag not allowed when name is a sub-flow"
                ));
            }
        }
        // loop conditions may be a single string or a list
        let loop_conditions = match reader.get(&format!("tasks[{i}].loop.condition")) {
            Some(ConfigValue::Text(one)) => vec![one],
            Some(ConfigValue::List(list)) => list.iter().map(|v| v.to_display_string()).collect(),
            _ => Vec::new(),
        };
        Ok(FlowConfigMetadata {
            input_count,
            output_count,
            function_route,
            execution,
            delay: reader.get_property(&format!("tasks[{i}].delay")),
            task_exception: match reader.get(&format!("tasks[{i}].exception")) {
                Some(ConfigValue::Text(e)) => Some(e),
                _ => None,
            },
            loop_statement: reader.get_property(&format!("tasks[{i}].loop.statement")),
            loop_conditions,
            unique_task_name: md_name,
            source: reader.get_property(&format!("tasks[{i}].source")),
        })
    }
}
