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
//! a routing-skill node must not draw an edge to 'suspend'; needs the suspend node
//! and must draw its checkpoint edge to it; every suspension point needs a
//! continuation edge; the suspend node needs 'task', a valid 'ttl' and an
//! outgoing connection; a 'resume' node needs 'task').

use event_script::conversions::display;
use platform_core::graph::{MiniGraph, SimpleNode};
use rmpv::Value;
use std::sync::Arc;

use crate::common::{assert_mutable_model_target, get_entries, EXCEPTION, NODE_NAME, SKILL};
use crate::suspend::{get_valid_ttl_seconds, RESUME_ROUTE, SUSPEND_ALIAS, SUSPEND_ROUTE};

const TASK: &str = "task";
const TTL: &str = "ttl";
const JS_ROUTE: &str = "graph.js";
const MAP_TO: &str = "->";
const MAPPING_TAG: &str = "mapping:";
/// Skills whose `ttl` node parameter is the child-call DEADLINE override.
/// Deliberate divergence from Java (maintainer-ruled): this engine does not
/// carry graph.js, so the set — and the rejection message — names THREE
/// skills where Java names four.
const DEADLINE_TTL_SKILLS: [&str; 3] = [
    crate::extension::ROUTE,
    crate::fetcher::ROUTE,
    crate::skills::TASK_ROUTE,
];
/// The mapping-list node properties a data mapping can appear in.
const MAPPING_PROPERTIES: [&str; 4] = ["mapping", "input", "output", "for_each"];

/// Validate the whole-graph contract of a complete graph model: the
/// suspend/resume rules, per-node ttl placement and grammar, and model
/// metadata immutability (Java `GraphModelValidator.validate`), returning
/// the first violated rule.
pub fn validate(graph: &MiniGraph) -> Result<(), String> {
    validate_suspend_resume(graph)?;
    validate_node_ttl(graph)?;
    validate_model_metadata_immutability(graph)
}

/// The per-node `ttl` is grammar-validated on the deadline skills and
/// rejected anywhere else — except the suspend node, whose mandatory ttl
/// (the store-record expiry, a different meaning on the same grammar) is
/// checked by the suspend/resume rules (Java
/// `GraphModelValidator.validateNodeTtl`).
fn validate_node_ttl(graph: &MiniGraph) -> Result<(), String> {
    for node in graph.get_nodes() {
        let alias = node.get_alias();
        let skill = node.get_property(SKILL).map(|v| display(&v));
        let ttl = node.get_property(TTL);
        let (Some(skill), Some(ttl)) = (skill, ttl) else {
            continue;
        };
        if alias == SUSPEND_ALIAS {
            continue;
        }
        if DEADLINE_TTL_SKILLS.contains(&skill.as_str()) {
            // fails for a blank, invalid or overflowing duration (long-math guard)
            get_valid_ttl_seconds(Some(&ttl), alias).map_err(|e| e.message().to_string())?;
        } else {
            return Err(format!(
                "{NODE_NAME}{alias} - 'ttl' is only applicable to the suspend node or a node \
                 with skill {}, {} or {}",
                crate::extension::ROUTE,
                crate::fetcher::ROUTE,
                crate::skills::TASK_ROUTE
            ));
        }
    }
    Ok(())
}

/// Model metadata (model.cid/instance/flow/ttl/trace/parent/root/none/run)
/// is engine-managed and immutable: reject any data mapping whose right-hand
/// side writes to it — in the four mapping-list properties AND in `MAPPING:`
/// lines embedded in `statement` arrays (the same idiom the runtime guard
/// sees). The runtime guard in `common` enforces the identical rule in both
/// walker lanes; this compile-side twin fails the deployment gate and the
/// playground pre-run check early, so a statically detectable violation can
/// never abort a live traversal (Java
/// `GraphModelValidator.validateModelMetadataImmutability`).
fn validate_model_metadata_immutability(graph: &MiniGraph) -> Result<(), String> {
    for node in graph.get_nodes() {
        let alias = node.get_alias();
        for property in MAPPING_PROPERTIES {
            for entry in get_entries(node.get_property(property)) {
                assert_not_metadata_write(alias, &entry)?;
            }
        }
        for entry in get_entries(node.get_property("statement")) {
            let trimmed = entry.trim();
            if let Some(colon) = trimmed.find(':') {
                if trimmed[..colon + 1].eq_ignore_ascii_case(MAPPING_TAG) {
                    assert_not_metadata_write(alias, &trimmed[colon + 1..])?;
                }
            }
        }
    }
    Ok(())
}

/// Check the RHS (after the LAST `->`) of one mapping entry against the
/// reserved model metadata; an entry without `->` is left for the shape
/// checks that own that rule.
fn assert_not_metadata_write(alias: &str, entry: &str) -> Result<(), String> {
    if let Some(sep) = entry.rfind(MAP_TO) {
        let rhs = entry[sep + MAP_TO.len()..].trim();
        // the compile-side message quotes the WHOLE offending entry (Java
        // GraphModelValidator wording), where the runtime guard quotes only
        // its write target
        if assert_mutable_model_target(alias, rhs).is_err() {
            return Err(format!(
                "{NODE_NAME}{alias} - invalid mapping ({}), model metadata is immutable",
                entry.trim()
            ));
        }
    }
    Ok(())
}

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
        warn_if_retired_suspend_property(graph, &node);
        validate_no_routing_skill_suspend_edge(graph, &node)?;
        validate_exception_target(&node)?;
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

/// The 'suspend=true' property is retired: a drawn edge to the 'suspend' node is the
/// suspension declaration (edge mode), and a decision jumps to the checkpoint instead
/// (jump mode). The property is accepted and ignored for one deprecation window so
/// v4.11.x models deploy unmodified - every valid v4.11.x suspensible node already
/// draws the checkpoint edge, which now declares the same behavior.
fn warn_if_retired_suspend_property(graph: &MiniGraph, node: &Arc<SimpleNode>) {
    if node
        .get_property(SUSPEND_ALIAS)
        .map(|v| display(&v).eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        let alias = node.get_alias();
        if has_edge_to_suspend(graph, alias) {
            log::warn!(
                "Node '{alias}' uses the retired 'suspend=true' property - it is ignored; \
                 the drawn edge to the '{SUSPEND_ALIAS}' node already declares the suspension point \
                 (remove the property)"
            );
        } else {
            log::warn!(
                "Node '{alias}' uses the retired 'suspend=true' property and has no drawn \
                 edge to the '{SUSPEND_ALIAS}' node - it will NOT suspend; draw the edge from a \
                 working node, or jump from a decision's IF-THEN-ELSE"
            );
        }
    }
}

/// A decision's forward links are outcome alternatives, not branches: if a
/// routing-skill node drew an edge to 'suspend', a resumed run would fan out its
/// alternatives as if they were parallel branches. A decision reaches the checkpoint
/// by jumping (return 'suspend' from IF-THEN-ELSE) and is re-executed on resume.
fn validate_no_routing_skill_suspend_edge(
    graph: &MiniGraph,
    node: &Arc<SimpleNode>,
) -> Result<(), String> {
    let skill = node.get_property(SKILL).map(|v| display(&v));
    if matches!(
        skill.as_deref(),
        Some(crate::skills::MATH_ROUTE) | Some(JS_ROUTE)
    ) && has_edge_to_suspend(graph, node.get_alias())
    {
        let skill_name = skill.unwrap_or_default();
        return Err(format!(
            "{NODE_NAME}{} has a drawn edge to the '{SUSPEND_ALIAS}' node but uses routing skill {skill_name} - \
             a decision reaches the checkpoint by jumping: return '{SUSPEND_ALIAS}' from its IF-THEN-ELSE \
             and draw edges to '{SUSPEND_ALIAS}' only from working nodes",
            node.get_alias()
        ));
    }
    Ok(())
}

/// The suspend node cannot be an exception handler - checkpoint-on-failure would give
/// a failed node retry-on-resume semantics through the back door. Route failures to a
/// handler node.
fn validate_exception_target(node: &Arc<SimpleNode>) -> Result<(), String> {
    if node
        .get_property(EXCEPTION)
        .map(|v| display(&v) == SUSPEND_ALIAS)
        .unwrap_or(false)
    {
        return Err(format!(
            "{NODE_NAME}{} routes its 'exception' to the '{SUSPEND_ALIAS}' node - the suspend node cannot \
             be an exception handler; route failures to a handler node",
            node.get_alias()
        ));
    }
    Ok(())
}

fn has_edge_to_suspend(graph: &MiniGraph, alias: &str) -> bool {
    graph
        .get_forward_links(alias)
        .unwrap_or_default()
        .iter()
        .any(|next| next.get_alias() == SUSPEND_ALIAS)
}

fn validate_continuation_edge(graph: &MiniGraph, node: &Arc<SimpleNode>) -> Result<(), String> {
    // a node with a drawn edge to the checkpoint is an edge-mode suspension point:
    // a resumed run continues along its forward links excluding 'suspend', so at
    // least one continuation edge must exist - a suspend-only node would loop on
    // resume. Shape-only rule: it applies regardless of skill (inspecting a
    // decision's IF-THEN-ELSE logic is deliberately out of scope). The one
    // exemption is also shape-level: an island's outgoing edges are never traversed
    // (the branch stops there), so an island-to-suspend edge is the ANCHOR that
    // keeps a jump-only suspend node non-orphan, not a checkpoint path
    if node.get_property(SKILL).map(|v| display(&v)).as_deref() == Some(crate::skills::ISLAND_ROUTE)
    {
        return Ok(());
    }
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
