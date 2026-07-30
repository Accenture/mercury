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

//! Reusable whole-graph contract checks — Rust port of the Java
//! `com.accenture.minigraph.common.GraphModelValidator`, shared by two
//! callers:
//!
//! 1. `compiler` (CompileGraph) — the deployment quality gate validates every
//!    manifest graph at startup; only graphs that pass become executable by
//!    the graph executor.
//! 2. The playground's `run` command — draft authoring deliberately allows
//!    partial models (a product owner builds a graph step by step and may
//!    save an incomplete draft). These rules are checked at the moment the
//!    author asks to execute, just before the graph traveler takes over.
//!
//! The rules here are whole-graph properties that per-command input
//! validation cannot express: the suspend/resume contract ('suspend' is a
//! reserved alias bound to the 'graph.suspend' skill in both directions; a
//! suspensible node must not use a routing skill, requires the suspend node
//! and must draw its checkpoint edge to it; every suspension point needs a
//! continuation edge; the suspend node needs 'task', a valid 'ttl' and an
//! outgoing connection; a 'resume' node needs 'task').

use event_script::conversions::display;
use platform_core::graph::{MiniGraph, SimpleNode};
use rmpv::Value;
use std::sync::Arc;

use crate::common::{NODE_NAME, SKILL};
use crate::suspend::{get_valid_ttl_seconds, RESUME_ROUTE, SUSPEND_ALIAS, SUSPEND_ROUTE};

const TASK: &str = "task";
const TTL: &str = "ttl";
const JS_ROUTE: &str = "graph.js";

/// Validate the suspend/resume contract of a complete graph model,
/// returning the first violated rule (Java throws IllegalArgumentException).
pub fn validate_suspend_resume(graph: &MiniGraph) -> Result<(), String> {
    let suspend_node = graph.find_node_by_alias(SUSPEND_ALIAS).ok().flatten();
    if let Some(node) = &suspend_node {
        validate_suspend_node(graph, node)?;
    }
    for node in graph.get_nodes() {
        let alias = node.get_alias().to_string();
        let skill = node.get_property(SKILL).map(|v| display(&v));
        if skill.as_deref() == Some(SUSPEND_ROUTE) && alias != SUSPEND_ALIAS {
            return Err(format!(
                "{NODE_NAME}{alias} - a node with skill {SUSPEND_ROUTE} must be named '{SUSPEND_ALIAS}'"
            ));
        }
        if skill.as_deref() == Some(RESUME_ROUTE) {
            validate_resume_node(&node)?;
        }
        if node
            .get_property(SUSPEND_ALIAS)
            .map(|v| display(&v).eq_ignore_ascii_case("true"))
            .unwrap_or(false)
        {
            validate_suspensible_node(graph, &node, suspend_node.as_ref())?;
        }
        validate_continuation_edge(graph, &node)?;
    }
    Ok(())
}

fn validate_suspend_node(graph: &MiniGraph, suspend_node: &Arc<SimpleNode>) -> Result<(), String> {
    let skill = suspend_node.get_property(SKILL).map(|v| display(&v));
    if skill.as_deref() != Some(SUSPEND_ROUTE) {
        return Err(format!(
            "the '{SUSPEND_ALIAS}' node must use skill {SUSPEND_ROUTE}"
        ));
    }
    if without_text(suspend_node.get_property(TASK).as_ref()) {
        return Err(format!(
            "{NODE_NAME}{SUSPEND_ALIAS} does not have a 'task' route"
        ));
    }
    // errors for a missing, blank, invalid or overflowing ttl (long-math guard)
    get_valid_ttl_seconds(suspend_node.get_property(TTL).as_ref(), SUSPEND_ALIAS)
        .map_err(|e| e.message().to_string())?;
    // without a forward path the record persists and the run then stalls - the
    // caller would time out despite a successful checkpoint
    if graph
        .get_forward_links(SUSPEND_ALIAS)
        .map(|links| links.is_empty())
        .unwrap_or(true)
    {
        return Err(format!(
            "{NODE_NAME}{SUSPEND_ALIAS} has no outgoing connection - \
             the run must complete after the checkpoint (connect it to 'end')"
        ));
    }
    Ok(())
}

fn validate_resume_node(node: &Arc<SimpleNode>) -> Result<(), String> {
    if without_text(node.get_property(TASK).as_ref()) {
        return Err(format!(
            "{NODE_NAME}{} does not have a 'task' route",
            node.get_alias()
        ));
    }
    Ok(())
}

fn validate_suspensible_node(
    graph: &MiniGraph,
    node: &Arc<SimpleNode>,
    suspend_node: Option<&Arc<SimpleNode>>,
) -> Result<(), String> {
    let alias = node.get_alias();
    let skill = node.get_property(SKILL).map(|v| display(&v));
    if matches!(
        skill.as_deref(),
        Some(crate::skills::MATH_ROUTE) | Some(JS_ROUTE)
    ) {
        return Err(format!(
            "{NODE_NAME}{alias} cannot use 'suspend=true' with skill {}",
            skill.unwrap_or_default()
        ));
    }
    if suspend_node.is_none() {
        return Err(format!(
            "{NODE_NAME}{alias} is suspensible but the graph has no '{SUSPEND_ALIAS}' node"
        ));
    }
    for next in graph.get_forward_links(alias).unwrap_or_default() {
        if next.get_alias() == SUSPEND_ALIAS {
            return Ok(());
        }
    }
    Err(format!(
        "{NODE_NAME}{alias} is suspensible but has no connection to the '{SUSPEND_ALIAS}' \
         node - the diagram must show the suspension path"
    ))
}

fn validate_continuation_edge(graph: &MiniGraph, node: &Arc<SimpleNode>) -> Result<(), String> {
    // any node that routes to the checkpoint (suspend=true or a plain drawn edge)
    // is a suspension point: a resumed run continues along its forward links
    // excluding 'suspend', so at least one continuation edge must exist
    let mut routes_to_suspend = false;
    let mut has_continuation = false;
    for next in graph
        .get_forward_links(node.get_alias())
        .unwrap_or_default()
    {
        if next.get_alias() == SUSPEND_ALIAS {
            routes_to_suspend = true;
        } else {
            has_continuation = true;
        }
    }
    if routes_to_suspend && !has_continuation {
        return Err(format!(
            "{NODE_NAME}{} suspends but has no continuation edge - a resumed run could not continue",
            node.get_alias()
        ));
    }
    Ok(())
}

fn without_text(value: Option<&Value>) -> bool {
    !matches!(value, Some(Value::String(text))
        if !text.as_str().unwrap_or_default().trim().is_empty())
}
