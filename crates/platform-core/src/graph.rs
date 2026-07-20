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

//! The minimalist in-memory property graph — Rust port of
//! `org.platformlambda.core.graph.MiniGraph` (+ `SimpleNode`,
//! `SimpleConnection`, `SimpleRelationship`): nodes with types and untyped
//! properties, unidirectional connections carrying optional typed relations,
//! alias/id/type/property lookups, neighbor traversal and BFS path
//! discovery. Designed for a few hundred nodes held entirely in memory —
//! deliberately never a database (the repo vision's non-goal); the Active
//! Knowledge Graph layer (layer 3) builds on it.
//!
//! Ownership model (the one Rust translation): Java hands out shared mutable
//! `SimpleNode` objects; here nodes, connections and relations are
//! `Arc`-shared with interior mutability, so engine code can hold node
//! handles and mutate properties exactly like the Java callers do. Property
//! values are `rmpv::Value` — the same currency as envelope bodies and the
//! layer-2 state machine.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use rmpv::Value;

use crate::function::AppError;

/// Node aliases that would collide with the knowledge-graph state-machine
/// namespaces (Java `RESERVED_NAMES`).
const RESERVED_NAMES: &[&str] = &[
    "input",
    "output",
    "model",
    "response",
    "result",
    "parameter",
    "none",
    "next",
    "api",
    "error",
];

fn invalid(message: impl Into<String>) -> AppError {
    AppError::new(400, message)
}

/// Java `GraphProperties.validateName`: aliases, types and property keys use
/// `0-9 A-Z a-z _ -` only.
fn validate_name(name: &str) -> Result<(), AppError> {
    let valid = !name.is_empty()
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');
    if valid {
        Ok(())
    } else {
        Err(invalid(format!(
            "Invalid syntax ({name}). Please use 0-9, A-Z, a-z, underscore and hyphen characters."
        )))
    }
}

/// Shared property-bag behavior (Java `GraphProperties`).
#[derive(Debug, Default)]
pub struct GraphProperties {
    properties: Mutex<HashMap<String, Value>>,
}

impl GraphProperties {
    /// A copy of all properties.
    pub fn get_properties(&self) -> HashMap<String, Value> {
        self.properties.lock().expect("graph properties").clone()
    }

    pub fn get_property(&self, key: &str) -> Option<Value> {
        self.properties
            .lock()
            .expect("graph properties")
            .get(key)
            .cloned()
    }

    /// Add a key-value (key syntax validated; null values rejected).
    pub fn add_property(&self, key: &str, value: Value) -> Result<(), AppError> {
        if key.is_empty() {
            return Err(invalid("key cannot be empty"));
        }
        if matches!(value, Value::Nil) {
            return Err(invalid("value cannot be null"));
        }
        validate_name(key)?;
        self.properties
            .lock()
            .expect("graph properties")
            .insert(key.to_string(), value);
        Ok(())
    }

    pub fn remove_property(&self, key: &str) -> Result<(), AppError> {
        if key.is_empty() {
            return Err(invalid("key cannot be empty"));
        }
        self.properties
            .lock()
            .expect("graph properties")
            .remove(key);
        Ok(())
    }
}

/// A graph node (Java `SimpleNode`): unique alias, one or more types, and a
/// property bag. Shared by `Arc`; equality is by node id.
#[derive(Debug)]
pub struct SimpleNode {
    id: String,
    alias: String,
    types: Mutex<HashSet<String>>,
    properties: GraphProperties,
}

impl PartialEq for SimpleNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for SimpleNode {}

impl SimpleNode {
    fn new(id: &str, alias: &str, node_type: &str) -> Result<Self, AppError> {
        validate_name(alias)?;
        validate_name(node_type)?;
        let node = SimpleNode {
            id: id.to_string(),
            alias: alias.to_string(),
            types: Mutex::new(HashSet::new()),
            properties: GraphProperties::default(),
        };
        node.types
            .lock()
            .expect("node types")
            .insert(node_type.to_string());
        Ok(node)
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_alias(&self) -> &str {
        &self.alias
    }

    pub fn get_types(&self) -> HashSet<String> {
        self.types.lock().expect("node types").clone()
    }

    pub fn add_type(&self, node_type: &str) -> Result<(), AppError> {
        if node_type.is_empty() {
            return Err(invalid("type cannot be empty"));
        }
        validate_name(node_type)?;
        self.types
            .lock()
            .expect("node types")
            .insert(node_type.to_string());
        Ok(())
    }

    /// Replace all types with one initial type (the Java Playground's
    /// `update node` mutates the live type set; this is the explicit analog).
    pub fn reset_types(&self, initial: &str) -> Result<(), AppError> {
        validate_name(initial)?;
        let mut types = self.types.lock().expect("node types");
        types.clear();
        types.insert(initial.to_string());
        Ok(())
    }

    /// Remove all properties (the Java Playground's `update node` analog).
    pub fn clear_properties(&self) {
        self.properties
            .properties
            .lock()
            .expect("graph properties")
            .clear();
    }

    /// A node keeps at least one type (Java parity).
    pub fn remove_type(&self, node_type: &str) -> Result<(), AppError> {
        if node_type.is_empty() {
            return Err(invalid("type cannot be empty"));
        }
        let mut types = self.types.lock().expect("node types");
        if types.len() == 1 {
            return Err(invalid(
                "Cannot remove type because a node must have at least one type",
            ));
        }
        types.remove(node_type);
        Ok(())
    }

    pub fn get_properties(&self) -> HashMap<String, Value> {
        self.properties.get_properties()
    }

    pub fn get_property(&self, key: &str) -> Option<Value> {
        self.properties.get_property(key)
    }

    pub fn add_property(&self, key: &str, value: Value) -> Result<(), AppError> {
        self.properties.add_property(key, value)
    }

    pub fn remove_property(&self, key: &str) -> Result<(), AppError> {
        self.properties.remove_property(key)
    }
}

/// A typed relation on a connection (Java `SimpleRelationship`).
#[derive(Debug)]
pub struct SimpleRelationship {
    relation_type: String,
    source_alias: String,
    target_alias: String,
    properties: GraphProperties,
}

impl PartialEq for SimpleRelationship {
    fn eq(&self, other: &Self) -> bool {
        self.relation_type == other.relation_type
            && self.source_alias == other.source_alias
            && self.target_alias == other.target_alias
    }
}

impl SimpleRelationship {
    pub fn get_type(&self) -> &str {
        &self.relation_type
    }

    pub fn get_source_alias(&self) -> &str {
        &self.source_alias
    }

    pub fn get_target_alias(&self) -> &str {
        &self.target_alias
    }

    pub fn get_properties(&self) -> HashMap<String, Value> {
        self.properties.get_properties()
    }

    pub fn get_property(&self, key: &str) -> Option<Value> {
        self.properties.get_property(key)
    }

    pub fn add_property(&self, key: &str, value: Value) -> Result<(), AppError> {
        self.properties.add_property(key, value)
    }
}

/// A unidirectional connection between two nodes, optionally carrying typed
/// relations (Java `SimpleConnection`). Equality is by connection id.
#[derive(Debug)]
pub struct SimpleConnection {
    id: String,
    source: Arc<SimpleNode>,
    target: Arc<SimpleNode>,
    relationships: Mutex<HashMap<String, Arc<SimpleRelationship>>>,
}

impl PartialEq for SimpleConnection {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl SimpleConnection {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_source(&self) -> &Arc<SimpleNode> {
        &self.source
    }

    pub fn get_target(&self) -> &Arc<SimpleNode> {
        &self.target
    }

    /// Add (or replace) a relation of the given type.
    pub fn add_relation(&self, relation_type: &str) -> Arc<SimpleRelationship> {
        let relation = Arc::new(SimpleRelationship {
            relation_type: relation_type.to_string(),
            source_alias: self.source.get_alias().to_string(),
            target_alias: self.target.get_alias().to_string(),
            properties: GraphProperties::default(),
        });
        self.relationships
            .lock()
            .expect("relations")
            .insert(relation_type.to_lowercase(), relation.clone());
        relation
    }

    /// Case-insensitive relation lookup.
    pub fn get_relation(&self, relation_type: &str) -> Option<Arc<SimpleRelationship>> {
        self.relationships
            .lock()
            .expect("relations")
            .get(&relation_type.to_lowercase())
            .cloned()
    }

    pub fn get_relations(&self) -> Vec<Arc<SimpleRelationship>> {
        self.relationships
            .lock()
            .expect("relations")
            .values()
            .cloned()
            .collect()
    }
}

struct GraphState {
    nodes_by_alias: HashMap<String, Arc<SimpleNode>>,
    nodes_by_id: HashMap<String, Arc<SimpleNode>>,
    connections: HashMap<String, Arc<SimpleConnection>>,
    successors: HashMap<String, HashSet<String>>,
    predecessors: HashMap<String, HashSet<String>>,
}

/// The mini-graph (Java `MiniGraph`): an in-memory property graph designed to
/// handle a small number of nodes very efficiently (default cap 750).
pub struct MiniGraph {
    graph_id: String,
    max_nodes: usize,
    node_count: AtomicUsize,
    state: Mutex<GraphState>,
}

impl Default for MiniGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl MiniGraph {
    /// Create a mini-graph with the default maximum of 750 nodes.
    pub fn new() -> Self {
        Self::with_max_nodes(750)
    }

    /// Create a mini-graph with an explicit node cap — be conservative:
    /// everything lives in memory and performance decreases as nodes grow.
    pub fn with_max_nodes(max_nodes: usize) -> Self {
        MiniGraph {
            graph_id: uuid::Uuid::new_v4().simple().to_string(),
            max_nodes,
            node_count: AtomicUsize::new(0),
            state: Mutex::new(GraphState {
                nodes_by_alias: HashMap::new(),
                nodes_by_id: HashMap::new(),
                connections: HashMap::new(),
                successors: HashMap::new(),
                predecessors: HashMap::new(),
            }),
        }
    }

    /// The unique id of this mini-graph.
    pub fn get_id(&self) -> &str {
        &self.graph_id
    }

    pub fn get_node_count(&self) -> usize {
        self.node_count.load(Ordering::SeqCst)
    }

    pub fn is_empty(&self) -> bool {
        self.get_node_count() == 0
    }

    pub fn create_root_node(&self) -> Result<Arc<SimpleNode>, AppError> {
        self.create_node("root", "root")
    }

    pub fn create_end_node(&self) -> Result<Arc<SimpleNode>, AppError> {
        self.create_node("end", "end")
    }

    pub fn get_root_node(&self) -> Option<Arc<SimpleNode>> {
        self.find_node_by_alias("root").ok().flatten()
    }

    pub fn get_end_node(&self) -> Option<Arc<SimpleNode>> {
        self.find_node_by_alias("end").ok().flatten()
    }

    /// Create a node with a unique alias and an initial type.
    pub fn create_node(&self, alias: &str, node_type: &str) -> Result<Arc<SimpleNode>, AppError> {
        if alias.is_empty() {
            return Err(invalid("alias must not be empty"));
        }
        if node_type.is_empty() {
            return Err(invalid("type must not be empty"));
        }
        let alias_lower = alias.to_lowercase();
        if RESERVED_NAMES.contains(&alias_lower.as_str()) {
            return Err(invalid(format!("alias '{alias_lower}' is a reserved name")));
        }
        let mut state = self.state.lock().expect("graph state");
        if state.nodes_by_alias.contains_key(&alias_lower) {
            return Err(invalid(format!("alias '{alias_lower}' already exists")));
        }
        if self.node_count.load(Ordering::SeqCst) > self.max_nodes {
            return Err(invalid(format!(
                "max number of nodes is {}",
                self.max_nodes
            )));
        }
        let count = self.node_count.fetch_add(1, Ordering::SeqCst) + 1;
        let id = uuid::Uuid::new_v4().simple().to_string();
        let node = Arc::new(SimpleNode::new(&id, alias, node_type)?);
        state.nodes_by_alias.insert(alias_lower, node.clone());
        state.nodes_by_id.insert(id, node.clone());
        log::debug!("Created {node_type} as {alias}, total={count}");
        Ok(node)
    }

    /// Remove a node and every connection touching it.
    pub fn remove_node(&self, alias: &str) -> Result<(), AppError> {
        if alias.is_empty() {
            return Err(invalid("alias must not be empty"));
        }
        let Some(node) = self.find_node_by_alias(alias)? else {
            return Ok(());
        };
        for neighbor in self.get_forward_links(alias)? {
            self.remove_connection(alias, neighbor.get_alias())?;
        }
        for neighbor in self.get_backward_links(alias)? {
            self.remove_connection(neighbor.get_alias(), alias)?;
        }
        let mut state = self.state.lock().expect("graph state");
        state.nodes_by_id.remove(node.get_id());
        // divergence note: Java removes with the raw alias here (a latent
        // case-sensitivity slip); the lowercased key matches every other
        // lookup and is the intended behavior
        state.nodes_by_alias.remove(&alias.to_lowercase());
        let count = self.node_count.fetch_sub(1, Ordering::SeqCst) - 1;
        log::debug!("Removed {alias}, total={count}");
        Ok(())
    }

    pub fn get_nodes(&self) -> Vec<Arc<SimpleNode>> {
        self.state
            .lock()
            .expect("graph state")
            .nodes_by_id
            .values()
            .cloned()
            .collect()
    }

    pub fn get_connections(&self) -> Vec<Arc<SimpleConnection>> {
        self.state
            .lock()
            .expect("graph state")
            .connections
            .values()
            .cloned()
            .collect()
    }

    /// Clear the graph, de-referencing all nodes and connections.
    pub fn reset(&self) {
        let mut state = self.state.lock().expect("graph state");
        state.connections.clear();
        state.successors.clear();
        state.predecessors.clear();
        state.nodes_by_alias.clear();
        state.nodes_by_id.clear();
        self.node_count.store(0, Ordering::SeqCst);
    }

    fn resolve_pair(
        &self,
        source_alias: &str,
        target_alias: &str,
    ) -> Result<(Arc<SimpleNode>, Arc<SimpleNode>), AppError> {
        if source_alias.is_empty() {
            return Err(invalid("source alias cannot be null"));
        }
        if target_alias.is_empty() {
            return Err(invalid("target alias cannot be null"));
        }
        if source_alias.eq_ignore_ascii_case(target_alias) {
            return Err(invalid("source and target aliases cannot be the same"));
        }
        let source = self
            .find_node_by_alias(source_alias)?
            .ok_or_else(|| invalid("source node does not exist"))?;
        let target = self
            .find_node_by_alias(target_alias)?
            .ok_or_else(|| invalid("target node does not exist"))?;
        Ok((source, target))
    }

    /// Connect two nodes (idempotent: an existing connection is returned).
    pub fn connect(
        &self,
        source_alias: &str,
        target_alias: &str,
    ) -> Result<Arc<SimpleConnection>, AppError> {
        let (source, target) = self.resolve_pair(source_alias, target_alias)?;
        let key = pair_key(source.get_id(), target.get_id());
        let mut state = self.state.lock().expect("graph state");
        if let Some(existing) = state.connections.get(&key) {
            return Ok(existing.clone());
        }
        let connection = Arc::new(SimpleConnection {
            id: uuid::Uuid::new_v4().simple().to_string(),
            source: source.clone(),
            target: target.clone(),
            relationships: Mutex::new(HashMap::new()),
        });
        state.connections.insert(key, connection.clone());
        state
            .successors
            .entry(source.get_id().to_string())
            .or_default()
            .insert(target.get_id().to_string());
        state
            .predecessors
            .entry(target.get_id().to_string())
            .or_default()
            .insert(source.get_id().to_string());
        log::debug!("Created connection {source_alias} to {target_alias}");
        Ok(connection)
    }

    /// Remove the connection between two nodes (with its relations).
    pub fn remove_connection(
        &self,
        source_alias: &str,
        target_alias: &str,
    ) -> Result<(), AppError> {
        let (source, target) = self.resolve_pair(source_alias, target_alias)?;
        let key = pair_key(source.get_id(), target.get_id());
        let mut state = self.state.lock().expect("graph state");
        if state.connections.remove(&key).is_some() {
            if let Some(targets) = state.successors.get_mut(source.get_id()) {
                targets.remove(target.get_id());
                if targets.is_empty() {
                    state.successors.remove(source.get_id());
                }
            }
            if let Some(sources) = state.predecessors.get_mut(target.get_id()) {
                sources.remove(source.get_id());
                if sources.is_empty() {
                    state.predecessors.remove(target.get_id());
                }
            }
            log::debug!(
                "Removed connection {} to {}",
                source.get_alias(),
                target.get_alias()
            );
        }
        Ok(())
    }

    /// Case-insensitive alias lookup. Errors on an empty alias (Java: null).
    pub fn find_node_by_alias(&self, alias: &str) -> Result<Option<Arc<SimpleNode>>, AppError> {
        if alias.is_empty() {
            return Err(invalid("alias cannot be null"));
        }
        Ok(self
            .state
            .lock()
            .expect("graph state")
            .nodes_by_alias
            .get(&alias.to_lowercase())
            .cloned())
    }

    pub fn find_node_by_id(&self, id: &str) -> Result<Option<Arc<SimpleNode>>, AppError> {
        if id.is_empty() {
            return Err(invalid("id cannot be null"));
        }
        Ok(self
            .state
            .lock()
            .expect("graph state")
            .nodes_by_id
            .get(id)
            .cloned())
    }

    /// All nodes carrying the given type (case-insensitive).
    pub fn find_nodes_by_type(&self, node_type: &str) -> Result<Vec<Arc<SimpleNode>>, AppError> {
        if node_type.is_empty() {
            return Err(invalid("type cannot be empty"));
        }
        Ok(self
            .get_nodes()
            .into_iter()
            .filter(|node| {
                node.get_types()
                    .iter()
                    .any(|t| t.eq_ignore_ascii_case(node_type))
            })
            .collect())
    }

    /// All relations of the given type across every connection.
    pub fn find_relation_by_type(
        &self,
        relation_type: &str,
    ) -> Result<Vec<Arc<SimpleRelationship>>, AppError> {
        if relation_type.is_empty() {
            return Err(invalid("type cannot be empty"));
        }
        Ok(self
            .get_connections()
            .into_iter()
            .filter_map(|conn| conn.get_relation(relation_type))
            .collect())
    }

    /// All nodes with a property whose key matches case-insensitively and
    /// whose value equals the given one.
    pub fn find_nodes_by_property(
        &self,
        key: &str,
        value: &Value,
    ) -> Result<Vec<Arc<SimpleNode>>, AppError> {
        if key.is_empty() {
            return Err(invalid("key cannot be empty"));
        }
        Ok(self
            .get_nodes()
            .into_iter()
            .filter(|node| {
                node.get_properties()
                    .iter()
                    .any(|(k, v)| k.eq_ignore_ascii_case(key) && v == value)
            })
            .collect())
    }

    pub fn find_connection(
        &self,
        source_alias: &str,
        target_alias: &str,
    ) -> Result<Option<Arc<SimpleConnection>>, AppError> {
        let (source, target) = self.resolve_pair(source_alias, target_alias)?;
        Ok(self
            .state
            .lock()
            .expect("graph state")
            .connections
            .get(&pair_key(source.get_id(), target.get_id()))
            .cloned())
    }

    /// Both directions between two nodes: 0 to 2 connections, forward first.
    pub fn find_bi_directional_connection(
        &self,
        source_alias: &str,
        target_alias: &str,
    ) -> Result<Vec<Arc<SimpleConnection>>, AppError> {
        let mut both = Vec::new();
        if let Some(forward) = self.find_connection(source_alias, target_alias)? {
            both.push(forward);
        }
        if let Some(backward) = self.find_connection(target_alias, source_alias)? {
            both.push(backward);
        }
        Ok(both)
    }

    fn require_node(&self, alias: &str) -> Result<Arc<SimpleNode>, AppError> {
        self.find_node_by_alias(alias)?
            .ok_or_else(|| invalid("node does not exist"))
    }

    fn linked_nodes(&self, node_id: &str, forward: bool) -> Vec<Arc<SimpleNode>> {
        let state = self.state.lock().expect("graph state");
        let map = if forward {
            &state.successors
        } else {
            &state.predecessors
        };
        map.get(node_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| state.nodes_by_id.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Nodes connected in either direction.
    pub fn get_neighbors(&self, alias: &str) -> Result<Vec<Arc<SimpleNode>>, AppError> {
        let node = self.require_node(alias)?;
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        for neighbor in self
            .linked_nodes(node.get_id(), true)
            .into_iter()
            .chain(self.linked_nodes(node.get_id(), false))
        {
            if seen.insert(neighbor.get_id().to_string()) {
                result.push(neighbor);
            }
        }
        Ok(result)
    }

    pub fn get_forward_links(&self, alias: &str) -> Result<Vec<Arc<SimpleNode>>, AppError> {
        let node = self.require_node(alias)?;
        Ok(self.linked_nodes(node.get_id(), true))
    }

    pub fn get_backward_links(&self, alias: &str) -> Result<Vec<Arc<SimpleNode>>, AppError> {
        let node = self.require_node(alias)?;
        Ok(self.linked_nodes(node.get_id(), false))
    }

    /// BFS level discovery from a node (direction-agnostic): one list of
    /// aliases per distance level; unreachable nodes are skipped.
    pub fn find_paths(&self, alias: &str) -> Result<Vec<Vec<String>>, AppError> {
        let start = self.require_node(alias)?;
        let mut distances: HashMap<String, i64> = HashMap::new();
        for node in self.get_nodes() {
            distances.insert(node.get_id().to_string(), -1);
        }
        distances.insert(start.get_id().to_string(), 0);
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(start.get_id().to_string());
        while let Some(current) = queue.pop_front() {
            let current_distance = distances[&current];
            let mut adjacent: HashSet<String> = HashSet::new();
            for n in self
                .linked_nodes(&current, true)
                .into_iter()
                .chain(self.linked_nodes(&current, false))
            {
                adjacent.insert(n.get_id().to_string());
            }
            for next in adjacent {
                if distances.get(&next) == Some(&-1) {
                    distances.insert(next.clone(), current_distance + 1);
                    queue.push_back(next);
                }
            }
        }
        let mut levels: HashMap<i64, Vec<String>> = HashMap::new();
        for (id, level) in &distances {
            if *level != -1 {
                if let Some(node) = self.find_node_by_id(id)? {
                    levels
                        .entry(*level)
                        .or_default()
                        .push(node.get_alias().to_string());
                }
            }
        }
        let mut sorted_levels: Vec<i64> = levels.keys().copied().collect();
        sorted_levels.sort_unstable();
        Ok(sorted_levels
            .into_iter()
            .map(|level| levels.remove(&level).unwrap_or_default())
            .collect())
    }

    /// Export the graph as a map value: nodes sorted by alias, connections by
    /// source:target, relations by type — deterministic for round-tripping.
    pub fn export_graph(&self) -> Value {
        let mut node_entries: Vec<(String, Value)> = self
            .get_nodes()
            .into_iter()
            .map(|node| {
                let mut types: Vec<String> = node.get_types().into_iter().collect();
                types.sort_unstable();
                let mut properties: Vec<(String, Value)> =
                    node.get_properties().into_iter().collect();
                properties.sort_by(|a, b| a.0.cmp(&b.0));
                let entry = Value::Map(vec![
                    (Value::from("alias"), Value::from(node.get_alias())),
                    (
                        Value::from("types"),
                        Value::Array(types.into_iter().map(Value::from).collect()),
                    ),
                    (
                        Value::from("properties"),
                        Value::Map(
                            properties
                                .into_iter()
                                .map(|(k, v)| (Value::from(k.as_str()), v))
                                .collect(),
                        ),
                    ),
                ]);
                (node.get_alias().to_string(), entry)
            })
            .collect();
        node_entries.sort_by(|a, b| a.0.cmp(&b.0));
        let mut connection_entries: Vec<(String, Value)> = self
            .get_connections()
            .into_iter()
            .map(|conn| {
                let mut relations: Vec<(String, Value)> = conn
                    .get_relations()
                    .into_iter()
                    .map(|relation| {
                        let mut properties: Vec<(String, Value)> =
                            relation.get_properties().into_iter().collect();
                        properties.sort_by(|a, b| a.0.cmp(&b.0));
                        let entry = Value::Map(vec![
                            (Value::from("type"), Value::from(relation.get_type())),
                            (
                                Value::from("properties"),
                                Value::Map(
                                    properties
                                        .into_iter()
                                        .map(|(k, v)| (Value::from(k.as_str()), v))
                                        .collect(),
                                ),
                            ),
                        ]);
                        (relation.get_type().to_string(), entry)
                    })
                    .collect();
                relations.sort_by(|a, b| a.0.cmp(&b.0));
                let key = format!(
                    "{}:{}",
                    conn.get_source().get_alias(),
                    conn.get_target().get_alias()
                );
                let entry = Value::Map(vec![
                    (
                        Value::from("source"),
                        Value::from(conn.get_source().get_alias()),
                    ),
                    (
                        Value::from("target"),
                        Value::from(conn.get_target().get_alias()),
                    ),
                    (
                        Value::from("relations"),
                        Value::Array(relations.into_iter().map(|(_, v)| v).collect()),
                    ),
                ]);
                (key, entry)
            })
            .collect();
        connection_entries.sort_by(|a, b| a.0.cmp(&b.0));
        Value::Map(vec![
            (
                Value::from("nodes"),
                Value::Array(node_entries.into_iter().map(|(_, v)| v).collect()),
            ),
            (
                Value::from("connections"),
                Value::Array(connection_entries.into_iter().map(|(_, v)| v).collect()),
            ),
        ])
    }

    /// Import a graph map (the export shape). The graph resets first; an
    /// invalid payload leaves it empty (Java parity).
    pub fn import_graph(&self, map: &Value) -> Result<(), AppError> {
        self.reset();
        let outcome = self.import_nodes_and_connections(map);
        if outcome.is_err() {
            self.reset();
        }
        outcome
    }

    fn import_nodes_and_connections(&self, map: &Value) -> Result<(), AppError> {
        let Value::Map(entries) = map else {
            return Ok(());
        };
        let get = |key: &str| -> Option<&Value> {
            entries
                .iter()
                .find(|(k, _)| k.as_str() == Some(key))
                .map(|(_, v)| v)
        };
        let Some(Value::Array(nodes)) = get("nodes") else {
            return Ok(());
        };
        for (i, entry) in nodes.iter().enumerate() {
            self.import_node(entry, i)?;
        }
        if let Some(Value::Array(connections)) = get("connections") {
            for (i, entry) in connections.iter().enumerate() {
                self.import_connection(entry, i)?;
            }
        }
        Ok(())
    }

    fn import_node(&self, entry: &Value, index: usize) -> Result<(), AppError> {
        let get = |key: &str| -> Option<&Value> {
            match entry {
                Value::Map(map) => map
                    .iter()
                    .find(|(k, _)| k.as_str() == Some(key))
                    .map(|(_, v)| v),
                _ => None,
            }
        };
        let alias = get("alias")
            .and_then(|v| v.as_str())
            .ok_or_else(|| invalid(format!("missing alias in node entry-{}", index + 1)))?;
        let Some(Value::Array(types)) = get("types") else {
            return Err(invalid(format!(
                "invalid types in node entry-{}",
                index + 1
            )));
        };
        if types.is_empty() {
            return Err(invalid(format!(
                "invalid types in node entry-{}",
                index + 1
            )));
        }
        let first = types[0].as_str().unwrap_or_default().to_string();
        let node = self.create_node(alias, &first)?;
        for t in types.iter().skip(1) {
            node.add_type(t.as_str().unwrap_or_default())?;
        }
        if let Some(Value::Map(properties)) = get("properties") {
            for (k, v) in properties {
                node.add_property(k.as_str().unwrap_or_default(), v.clone())?;
            }
        }
        Ok(())
    }

    fn import_connection(&self, entry: &Value, index: usize) -> Result<(), AppError> {
        let get = |key: &str| -> Option<&Value> {
            match entry {
                Value::Map(map) => map
                    .iter()
                    .find(|(k, _)| k.as_str() == Some(key))
                    .map(|(_, v)| v),
                _ => None,
            }
        };
        let (Some(source), Some(target)) = (
            get("source").and_then(|v| v.as_str()),
            get("target").and_then(|v| v.as_str()),
        ) else {
            return Err(invalid(format!(
                "invalid source/target alias in connection entry-{}",
                index + 1
            )));
        };
        let connection = self.connect(source, target)?;
        if let Some(Value::Array(relations)) = get("relations") {
            for relation_entry in relations {
                if let Value::Map(relation_map) = relation_entry {
                    let rget = |key: &str| -> Option<&Value> {
                        relation_map
                            .iter()
                            .find(|(k, _)| k.as_str() == Some(key))
                            .map(|(_, v)| v)
                    };
                    if let Some(relation_type) = rget("type").and_then(|v| v.as_str()) {
                        let relation = connection.add_relation(relation_type);
                        if let Some(Value::Map(properties)) = rget("properties") {
                            for (k, v) in properties {
                                relation.add_property(k.as_str().unwrap_or_default(), v.clone())?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Structural equality with another graph: same nodes (aliases, types,
    /// deep-equal properties) and same connections/relations.
    pub fn same_as(&self, that: &MiniGraph) -> bool {
        self.same_nodes(that) && self.same_connections(that)
    }

    fn same_nodes(&self, that: &MiniGraph) -> bool {
        let nodes1 = self.get_nodes();
        if nodes1.len() != that.get_node_count() {
            return false;
        }
        for n1 in nodes1 {
            let Ok(Some(n2)) = that.find_node_by_alias(n1.get_alias()) else {
                return false;
            };
            if n1.get_types() != n2.get_types() {
                return false;
            }
            if !same_properties(&n1.get_properties(), &n2.get_properties()) {
                return false;
            }
        }
        true
    }

    fn same_connections(&self, that: &MiniGraph) -> bool {
        let conn1 = self.get_connections();
        if conn1.len() != that.get_connections().len() {
            return false;
        }
        for c1 in conn1 {
            let Ok(Some(c2)) =
                that.find_connection(c1.get_source().get_alias(), c1.get_target().get_alias())
            else {
                return false;
            };
            let relations1 = c1.get_relations();
            if relations1.len() != c2.get_relations().len() {
                return false;
            }
            for r1 in relations1 {
                let Some(r2) = c2.get_relation(r1.get_type()) else {
                    return false;
                };
                if !same_properties(&r1.get_properties(), &r2.get_properties()) {
                    return false;
                }
            }
        }
        true
    }

    /// Export as a JSON string (deterministic ordering).
    pub fn to_json(&self) -> String {
        serde_json::to_value(self.export_graph())
            .map(|v| v.to_string())
            .unwrap_or_default()
    }
}

fn pair_key(source_id: &str, target_id: &str) -> String {
    format!("{source_id}:{target_id}")
}

/// Deep property comparison independent of map ordering (Java flattens both
/// sides with `getFlatMap` and compares keys and values).
fn same_properties(a: &HashMap<String, Value>, b: &HashMap<String, Value>) -> bool {
    let mut flat_a = Vec::new();
    let mut flat_b = Vec::new();
    for (k, v) in a {
        flatten(k, v, &mut flat_a);
    }
    for (k, v) in b {
        flatten(k, v, &mut flat_b);
    }
    flat_a.sort_by(|x, y| x.0.cmp(&y.0));
    flat_b.sort_by(|x, y| x.0.cmp(&y.0));
    flat_a == flat_b
}

fn flatten(prefix: &str, value: &Value, target: &mut Vec<(String, Value)>) {
    match value {
        Value::Map(entries) => {
            for (k, v) in entries {
                flatten(
                    &format!("{prefix}.{}", k.as_str().unwrap_or_default()),
                    v,
                    target,
                );
            }
        }
        Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                flatten(&format!("{prefix}[{i}]"), v, target);
            }
        }
        leaf => target.push((prefix.to_string(), leaf.clone())),
    }
}
