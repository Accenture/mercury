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

//! The graph runtime — Rust port of `GraphExecutor` (`graph.executor`, a
//! zero-tracing event interceptor). Invoked through the `graph-executor`
//! event-script flow: a plain correlation id starts a traversal from the
//! root node; a composite `{flowInstanceId}@{nodeName}` correlation id is a
//! skill callback that decides the next hop (`next`, a node name to jump to,
//! or `.sink` to stop a branch). Loop protection aborts a node that executes
//! too frequently within `graph.max.loop.interval`.
//!
//! `graph.js` is RETIRED (maintainer decision, 2026-07-17): an embedded
//! interpreter running arbitrary user-supplied code is an attack surface the
//! Rust engine does not reproduce. A graph carrying `skill: graph.js` fails
//! with an explicit message pointing at `graph.math` / `graph.task`.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::OnceLock;

use event_script::conversions::display;
use platform_core::graph::SimpleNode;
use platform_core::{AppConfigReader, AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::common::{
    self, get_error_map, initialize_with_node_properties, invalid, EXCEPTION, EXECUTE, IN, NODE,
    OUTPUT_BODY_NAMESPACE, OUTPUT_HEADER_NAMESPACE, SINK, SKILL, TYPE,
};
use crate::model::{self, GraphInstance};
use crate::{compiler, graphs};

pub const ROUTE: &str = "graph.executor";
pub const HOUSEKEEPER_ROUTE: &str = "graph.housekeeper";
const INSTANCE: &str = "instance";
const GRAPH: &str = "graph";
const NEXT: &str = "next";
const RETIRED_JS_MESSAGE: &str =
    "Skill graph.js is retired for security reasons - use graph.math or graph.task instead";

fn is_dev_env() -> bool {
    static DEV: OnceLock<bool> = OnceLock::new();
    *DEV.get_or_init(|| AppConfigReader::get_instance().get_property_or("app.env", "dev") == "dev")
}

fn deployed_graph_location() -> &'static str {
    static LOCATION: OnceLock<String> = OnceLock::new();
    LOCATION.get_or_init(|| {
        let configured = AppConfigReader::get_instance()
            .get_property_or("location.graph.deployed", "classpath:/graph");
        if configured.starts_with("file:") || configured.starts_with("classpath:") {
            configured
        } else {
            log::error!(
                "location.graph.deployed must start with file:/ or classpath:/. \
                 Fallback to classpath:/graph"
            );
            "classpath:/graph".to_string()
        }
    })
}

/// The interceptor body (Java `handleEvent`).
pub async fn handle(
    platform: &Platform,
    headers: HashMap<String, String>,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let po = PostOffice::new(platform);
    if let Some(cid) = event.correlation_id() {
        if cid.contains('@') {
            handle_skill_response(platform, &po, &event).await;
        } else if event.reply_to().is_some() {
            execute_graph(platform, &po, &headers, &event).await;
        }
    }
    EventEnvelope::new().set_body("ignored")
}

async fn execute_graph(
    platform: &Platform,
    po: &PostOffice,
    headers: &HashMap<String, String>,
    event: &EventEnvelope,
) {
    // the span that triggered this graph is the parent span for the graph's
    // first node, establishing telemetry lineage into the graph
    let parent_span = event.span_id().map(str::to_string);
    let reply_to = event.reply_to().unwrap_or_default().to_string();
    let cid = event.correlation_id().unwrap_or_default().to_string();
    let outcome = start_traversal(platform, po, headers, &reply_to, &cid, &parent_span).await;
    if let Err(e) = outcome {
        let mut error = EventEnvelope::new()
            .set_to(&reply_to)
            .set_status(e.status())
            .set_raw_body(Value::from(e.message()))
            .set_correlation_id(&cid);
        if let Some(span) = &parent_span {
            error = error.set_span_id(span);
        }
        let _ = po.send(error).await;
    }
}

async fn start_traversal(
    platform: &Platform,
    po: &PostOffice,
    headers: &HashMap<String, String>,
    reply_to: &str,
    cid: &str,
    parent_span: &Option<String>,
) -> Result<(), AppError> {
    let instance = create_instance(headers, reply_to, cid)?;
    begin_traversal(platform, po, &instance, parent_span).await
}

fn create_instance(
    headers: &HashMap<String, String>,
    reply_to: &str,
    cid: &str,
) -> Result<Arc<GraphInstance>, AppError> {
    let flow_instance_id = headers
        .get(INSTANCE)
        .ok_or_else(|| invalid("Missing instance ID in header"))?;
    let graph_id = headers
        .get(GRAPH)
        .ok_or_else(|| invalid("Missing graph ID in header"))?;
    let flow_instance = event_script::flows::get_flow_instance(flow_instance_id)
        .ok_or_else(|| invalid(format!("Invalid flow instance {flow_instance_id}")))?;
    // the housekeeper clears this graph instance when the flow ends
    flow_instance.set_end_flow_listeners(&[HOUSEKEEPER_ROUTE]);
    let model = get_graph_model(graph_id)?;
    if !matches!(&model, Value::Map(entries) if !entries.is_empty()) {
        return Err(invalid(format!(
            "Unable to load graph model '{graph_id}' - missing or invalid"
        )));
    }
    let instance = Arc::new(GraphInstance::new(graph_id));
    instance.set_flow_instance_id(flow_instance_id);
    instance.set_correlation_id(cid);
    instance.set_reply_to(reply_to);
    instance
        .graph
        .import_graph(&model)
        .map_err(|e| invalid(e.message()))?;
    model::add_instance(flow_instance_id, instance.clone());
    // seed the graph state machine with copies of the flow's input and model
    {
        let flow_dataset = flow_instance.dataset.lock().expect("flow dataset");
        let input_copy = flow_dataset.get_element("input").unwrap_or(Value::Nil);
        let model_copy = flow_dataset.get_element("model").unwrap_or(Value::Nil);
        drop(flow_dataset);
        let mut state = instance.state.lock().expect("graph state machine");
        state.set_element("input", input_copy).map_err(invalid)?;
        state.set_element("model", model_copy).map_err(invalid)?;
    }
    initialize_with_node_properties(&instance)?;
    Ok(instance)
}

async fn begin_traversal(
    platform: &Platform,
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    parent_span: &Option<String>,
) -> Result<(), AppError> {
    let root = instance
        .graph
        .get_root_node()
        .ok_or_else(|| invalid("Root node does not exist"))?;
    instance
        .graph
        .get_end_node()
        .ok_or_else(|| invalid("End node does not exist"))?;
    walk(platform, po, instance, root, parent_span.clone()).await
}

async fn handle_skill_response(platform: &Platform, po: &PostOffice, response: &EventEnvelope) {
    let composite_id = response.correlation_id().unwrap_or_default();
    let Some(at) = composite_id.find('@') else {
        return;
    };
    let flow_instance_id = &composite_id[..at];
    let node_name = &composite_id[at + 1..];
    let Some(instance) = model::get_instance(flow_instance_id) else {
        return;
    };
    if event_script::flows::get_flow_instance(flow_instance_id).is_none() {
        return;
    }
    // the completed node's own span (stamped on its reply by the worker) is
    // the parent span for whatever this callback dispatches next
    let parent_span = response.span_id().map(str::to_string);
    let target = {
        let state = instance.state.lock().expect("graph state machine");
        state.get_element(&format!("{node_name}.{}", common::TARGET))
    };
    // unrecoverable error from the node itself
    if response.has_error() {
        if target.is_some() {
            let mut state = instance.state.lock().expect("graph state machine");
            let error_map = get_error_map(state.get_element(OUTPUT_BODY_NAMESPACE), target);
            let _ = state.set_element(OUTPUT_BODY_NAMESPACE, error_map);
        }
        handle_error_response(po, &instance, response, &parent_span).await;
        return;
    }
    let node = match instance.graph.find_node_by_alias(node_name) {
        Ok(Some(node)) => node,
        _ => {
            send_error(
                po,
                &instance,
                &format!("{}{node_name}{}", common::NODE_NAME, common::NOT_FOUND),
                &parent_span,
            )
            .await;
            return;
        }
    };
    instance
        .skill_run
        .lock()
        .expect("skill run")
        .insert(node_name.to_string(), true);
    check_frequency(po, &instance, node_name, &parent_span).await;
    // a skill can set status and error in its node properties instead of
    // failing (e.g. an HTTP status >= 400 from the API fetcher)
    let (process_status, result_error) = {
        let state = instance.state.lock().expect("graph state machine");
        (
            state.get_element(&format!("{node_name}.{}", common::STATUS)),
            state.get_element(&format!("{node_name}.{}", common::ERROR)),
        )
    };
    let error_handler = node.get_property(EXCEPTION);
    if let (Some(Value::Integer(rc)), Some(error), None) =
        (&process_status, &result_error, &error_handler)
    {
        let error_map = get_error_map(Some(error.clone()), target);
        let mut event = EventEnvelope::new()
            .set_to(&instance.get_reply_to())
            .set_correlation_id(&instance.get_correlation_id())
            .set_raw_body(error_map)
            .set_status(rc.as_i64().unwrap_or(500) as i32);
        if let Some(span) = &parent_span {
            event = event.set_span_id(span);
        }
        let _ = po.send(event).await;
        instance.set_complete();
    } else if !instance.is_complete() {
        let next = display(response.body());
        decide_next(platform, po, &instance, node, &next, &parent_span).await;
    }
}

async fn check_frequency(
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    node_name: &str,
    parent_span: &Option<String>,
) {
    let now = chrono::Utc::now().timestamp_millis();
    let (total, last) = {
        let mut hits = instance.hits.lock().expect("visit counters");
        let entry = hits.entry(node_name.to_string()).or_default();
        if now - entry.last_visit > common::loop_interval() {
            entry.last_visit = now;
            entry.hits = 0;
        }
        entry.hits += 1;
        (entry.hits, entry.last_visit)
    };
    if total > common::high_frequency() {
        log::error!(
            "Looping detected - {} hits in {} ms for {} in {}",
            total,
            now - last,
            node_name,
            instance.graph_id
        );
        let response = EventEnvelope::new()
            .set_raw_body(Value::from(format!(
                "Node {node_name} executed too frequently"
            )))
            .set_status(400);
        handle_error_response(po, instance, &response, parent_span).await;
    }
}

async fn decide_next(
    platform: &Platform,
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    node: Arc<SimpleNode>,
    next: &str,
    parent_span: &Option<String>,
) {
    let is_end = instance
        .graph
        .get_end_node()
        .map(|end| end.get_id() == node.get_id())
        .unwrap_or(false);
    if is_end {
        execution_complete(po, instance, parent_span).await;
    } else {
        next_or_jump(platform, po, instance, node, next, parent_span).await;
    }
}

/// Traverse to a node once (join nodes are exempt from the seen check).
/// Boxed for async recursion.
fn walk<'a>(
    platform: &'a Platform,
    po: &'a PostOffice,
    instance: &'a Arc<GraphInstance>,
    node: Arc<SimpleNode>,
    parent_span: Option<String>,
) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'a>> {
    Box::pin(async move {
        if instance.is_complete() {
            return Ok(());
        }
        let node_name = node.get_alias().to_string();
        let skill = node.get_property(SKILL).map(|v| display(&v));
        let seen = skill.as_deref() != Some(crate::skills::JOIN_ROUTE)
            && instance
                .node_seen
                .lock()
                .expect("node seen")
                .contains_key(&node_name);
        if !seen {
            instance
                .node_seen
                .lock()
                .expect("node seen")
                .insert(node_name, true);
            walk_to(platform, po, skill, instance, node, parent_span).await?;
        }
        Ok(())
    })
}

async fn walk_to(
    platform: &Platform,
    po: &PostOffice,
    skill: Option<String>,
    instance: &Arc<GraphInstance>,
    node: Arc<SimpleNode>,
    parent_span: Option<String>,
) -> Result<(), AppError> {
    let is_end = instance
        .graph
        .get_end_node()
        .map(|end| end.get_id() == node.get_id())
        .unwrap_or(false);
    match skill {
        Some(skill) => execute_skill(platform, po, &skill, instance, &node, &parent_span).await,
        None if is_end => {
            execution_complete(po, instance, &parent_span).await;
            Ok(())
        }
        None => walk_next(platform, po, instance, &node, &parent_span).await,
    }
}

async fn execution_complete(
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    parent_span: &Option<String>,
) {
    let (body, headers) = {
        let state = instance.state.lock().expect("graph state machine");
        (
            state
                .get_element(OUTPUT_BODY_NAMESPACE)
                .unwrap_or(Value::Nil),
            state.get_element(OUTPUT_HEADER_NAMESPACE),
        )
    };
    let mut response = EventEnvelope::new()
        .set_to(&instance.get_reply_to())
        .set_correlation_id(&instance.get_correlation_id());
    if let Some(span) = parent_span {
        response = response.set_span_id(span);
    }
    if let Some(Value::Map(entries)) = headers {
        for (k, v) in &entries {
            response = response.set_header(k.as_str().unwrap_or_default(), &display(v));
        }
    }
    let _ = po.send(response.set_raw_body(body)).await;
    instance.set_complete();
}

async fn execute_skill(
    platform: &Platform,
    po: &PostOffice,
    skill: &str,
    instance: &Arc<GraphInstance>,
    node: &Arc<SimpleNode>,
    parent_span: &Option<String>,
) -> Result<(), AppError> {
    // the graph.js retirement (design K5): explicit failure, never registered
    if skill == "graph.js" {
        send_error(po, instance, RETIRED_JS_MESSAGE, parent_span).await;
        return Ok(());
    }
    if platform.has_route(skill) {
        let flow_instance_id = instance.get_flow_instance_id();
        let node_name = node.get_alias();
        let composite_id = format!("{flow_instance_id}@{node_name}");
        let mut event = EventEnvelope::new()
            .set_to(skill)
            .set_header(IN, &flow_instance_id)
            .set_header(TYPE, EXECUTE)
            .set_header(NODE, node_name)
            .set_reply_to(ROUTE)
            .set_correlation_id(&composite_id);
        if let Some(span) = parent_span {
            event = event.set_span_id(span);
        }
        po.send(event).await
    } else {
        send_error(
            po,
            instance,
            &format!("Skill {skill} does not exist"),
            parent_span,
        )
        .await;
        Ok(())
    }
}

async fn next_or_jump(
    platform: &Platform,
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    node: Arc<SimpleNode>,
    next: &str,
    parent_span: &Option<String>,
) {
    if next == SINK {
        return;
    }
    if next == NEXT {
        let _ = walk_next(platform, po, instance, &node, parent_span).await;
    } else {
        match instance.graph.find_node_by_alias(next) {
            Ok(Some(next_node)) => {
                let _ = walk(platform, po, instance, next_node, parent_span.clone()).await;
            }
            _ => {
                send_error(
                    po,
                    instance,
                    &format!("Next node '{next}' does not exist"),
                    parent_span,
                )
                .await;
            }
        }
    }
}

async fn walk_next(
    platform: &Platform,
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    node: &Arc<SimpleNode>,
    parent_span: &Option<String>,
) -> Result<(), AppError> {
    if instance.is_complete() {
        return Ok(());
    }
    let nodes = instance.graph.get_forward_links(node.get_alias())?;
    for next in nodes {
        walk(platform, po, instance, next, parent_span.clone()).await?;
    }
    Ok(())
}

fn get_graph_model(graph_id: &str) -> Result<Value, AppError> {
    if graph_id.starts_with("tutorial") && !is_dev_env() {
        return Err(invalid("tutorial graph models not allowed"));
    }
    // graphs validated and converted at startup are reused as-is
    if let Some(compiled) = graphs::get_graph(graph_id) {
        return Ok((*compiled).clone());
    }
    compiler::load_raw_graph(deployed_graph_location(), graph_id).map_err(invalid)
}

async fn handle_error_response(
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    response: &EventEnvelope,
    parent_span: &Option<String>,
) {
    let mut error = EventEnvelope::new()
        .set_to(&instance.get_reply_to())
        .set_correlation_id(&instance.get_correlation_id())
        .set_raw_body(response.body().clone())
        .set_status(response.status());
    if let Some(span) = parent_span {
        error = error.set_span_id(span);
    }
    let _ = po.send(error).await;
    instance.set_complete();
}

async fn send_error(
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    message: &str,
    parent_span: &Option<String>,
) {
    let mut error = EventEnvelope::new()
        .set_to(&instance.get_reply_to())
        .set_correlation_id(&instance.get_correlation_id())
        .set_raw_body(Value::from(message))
        .set_status(400);
    if let Some(span) = parent_span {
        error = error.set_span_id(span);
    }
    let _ = po.send(error).await;
    instance.set_complete();
}
