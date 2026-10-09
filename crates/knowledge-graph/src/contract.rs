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

//! The contract of a graph model (RFC-0007; Java `GraphContract`): what a caller sends and what the
//! graph answers, derived from the model and merged with what the model declares.
//!
//! **Discovery** works in three tiers over every node's properties: (1) the path scan that
//! `describe graph` has always done - every `input.*` and `output.*` token, with nesting and the
//! `[]`, `[*]` and `[0]` array markers; (2) direct evidence - a typed constant or wrapper
//! (`int(...)`, `text(...)`), a plugin with a known result (`f:now`, `f:listOfMap`), a `for_each`
//! source, a `graph.math` result (`COMPUTE` a number, `CONDITION` a boolean, `DECIMAL` a string) and
//! an arithmetic or ordered comparison operand; (3) one hop of propagation through a variable - a
//! model variable or a node result that was typed by (2), or a `graph.extension` target's declared
//! output - into the output path it feeds. Each path records the nodes that reference it.
//!
//! **Declaration** is the optional `schema` property of the root node (the request: `schema.body`
//! for `input.body`, `schema.header` for `input.header`) and of the end node (the response:
//! `output.body` and `output.header`), each part a schema object in the OpenAPI 3.0 dialect. The
//! declaration wins over discovery; a discovered path the declaration lacks is kept untyped and
//! flagged; a declared path the model never references is kept and flagged. Header names are
//! case-insensitive. Query parameters are not part of the graph API (one endpoint for every graph),
//! so they are neither declared nor derived.
//!
//! The same derivation runs in the Java engine; the shared vector file pins both.

use std::collections::{BTreeMap, BTreeSet};

use event_script::conversions::from_json;
use rmpv::Value;
use serde_json::Value as Json;

pub const INPUT_BODY: &str = "input.body";
pub const INPUT_HEADER: &str = "input.header";
pub const OUTPUT_BODY: &str = "output.body";
pub const OUTPUT_HEADER: &str = "output.header";
pub const OUTPUT_STATUS: &str = "output.status";
const NAMESPACES: [&str; 4] = [INPUT_BODY, INPUT_HEADER, OUTPUT_BODY, OUTPUT_HEADER];
const MAPPING_PROPERTIES: [&str; 5] = ["mapping", "input", "output", "for_each", "statement"];
const STRING: &str = "string";
const NUMBER: &str = "number";
const INTEGER: &str = "integer";
const BOOLEAN: &str = "boolean";
const OBJECT: &str = "object";
const ARRAY: &str = "array";
const RESULT: &str = "result";
const MAP_TO: &str = "->";
const MODEL_PREFIX: &str = "model.";

/// The simple plugins whose result type is known from the plugin alone.
fn plugin_type(name: &str) -> Option<&'static str> {
    match name {
        "int" | "long" | "length" | "decimalCompare" => Some(INTEGER),
        "float" | "double" | "add" | "subtract" | "multiply" | "div" | "mod" | "increment"
        | "decrement" => Some(NUMBER),
        "boolean" | "eq" | "ne" | "gt" | "lt" | "and" | "or" | "not" | "isNull" | "notNull" => {
            Some(BOOLEAN)
        }
        "text" | "concat" | "substring" | "now" | "uuid" | "dateTime" | "b64" | "lookup"
        | "decimalAdd" | "decimalSubtract" | "decimalMultiply" | "decimalDiv" | "decimalMod"
        | "decimalRound" => Some(STRING),
        "listOfMap" | "updateListOfMap" => Some(ARRAY),
        _ => None,
    }
}

/// One path of a namespace tree: a property, a header, or the element of an array.
#[derive(Debug, Default)]
pub struct Node {
    pub name: String,
    pub type_name: Option<String>,
    pub array: bool,
    pub declared: bool,
    pub discovered: bool,
    pub required: bool,
    pub used_by: BTreeSet<String>,
    /// The declared keys of this path other than the structural ones, verbatim.
    pub fragment: Vec<(String, Json)>,
    /// For a declared array, the declared keys of its element other than the structural ones.
    pub item_fragment: Vec<(String, Json)>,
    pub children: BTreeMap<String, Node>,
}

impl Node {
    fn child(&mut self, key: &str, case_insensitive: bool) -> &mut Node {
        let k = if case_insensitive {
            key.to_lowercase()
        } else {
            key.to_string()
        };
        self.children.entry(k).or_insert_with(|| Node {
            name: key.to_string(),
            ..Default::default()
        })
    }

    /// An array result marks the path as an array (OpenAPI needs its items); any other type is the leaf type.
    fn set_type(&mut self, type_name: &str) {
        if type_name == ARRAY {
            self.array = true;
        } else {
            self.type_name = Some(type_name.to_string());
        }
    }
}

struct Entry {
    alias: String,
    property: &'static str,
    text: String,
}

/// The derived and declared contract of one graph model.
pub struct GraphContract {
    graph_id: String,
    purpose: Option<String>,
    roots: BTreeMap<&'static str, Node>,
    declared: BTreeMap<&'static str, bool>,
    status_codes: BTreeSet<i64>,
    issues: Vec<String>,
    /// Variables typed by direct evidence: model.x, {node}.result.y, {node}.y.
    variables: BTreeMap<String, String>,
}

impl GraphContract {
    /// Derive the contract of a graph model; `other_models` resolves another deployed model by id,
    /// for a `graph.extension` target.
    pub fn derive(
        graph_id: &str,
        model: &Json,
        other_models: &dyn Fn(&str) -> Option<Json>,
    ) -> Self {
        let mut contract = GraphContract {
            graph_id: graph_id.to_string(),
            purpose: root_property(model, "purpose"),
            roots: BTreeMap::new(),
            declared: BTreeMap::new(),
            status_codes: BTreeSet::new(),
            issues: Vec::new(),
            variables: BTreeMap::new(),
        };
        for ns in NAMESPACES {
            contract.roots.insert(
                ns,
                Node {
                    name: ns.to_string(),
                    ..Default::default()
                },
            );
            contract.declared.insert(ns, false);
        }
        contract.discover(model, other_models);
        contract.declare(model);
        contract.reconcile();
        contract
    }

    pub fn graph_id(&self) -> &str {
        &self.graph_id
    }

    pub fn purpose(&self) -> Option<&str> {
        self.purpose.as_deref()
    }

    pub fn status_codes(&self) -> &BTreeSet<i64> {
        &self.status_codes
    }

    pub fn is_declared(&self, namespace: &str) -> bool {
        self.declared.get(namespace).copied().unwrap_or(false)
    }

    /// True when the namespace has any path, discovered or declared.
    pub fn has(&self, namespace: &str) -> bool {
        self.roots
            .get(namespace)
            .map(|root| root.discovered || root.declared || !root.children.is_empty())
            .unwrap_or(false)
    }

    pub fn root(&self, namespace: &str) -> &Node {
        self.roots.get(namespace).expect("a known namespace")
    }

    pub fn issues(&self) -> &[String] {
        &self.issues
    }

    // ---------------------------------------------------------------- discovery

    fn discover(&mut self, model: &Json, other_models: &dyn Fn(&str) -> Option<Json>) {
        let nodes = node_list(model);
        // tier 1: the path scan over every node's properties, in the JSON text form both engines scan
        for node in &nodes {
            let alias = alias_of(node);
            if let Some(properties) = node.get("properties").filter(|p| p.is_object()) {
                let text = properties.to_string();
                for ns in NAMESPACES {
                    for token in collect_path_tokens(&text, ns) {
                        self.record_path(ns, &token, &alias);
                    }
                }
            }
        }
        // tier 2: direct evidence, then tier 3: one hop through a variable (two ordered passes)
        let entries = mapping_entries(&nodes);
        for e in &entries {
            self.direct_evidence(&e.alias, e.property, &e.text);
        }
        for node in &nodes {
            self.extension_target(node, other_models);
        }
        for e in &entries {
            self.variable_from_input(&e.alias, &e.text);
        }
        for e in &entries {
            self.output_from_variable(&e.alias, &e.text);
        }
    }

    /// Record a discovered path token of a namespace, with the node that references it.
    fn record_path(&mut self, namespace: &'static str, token: &str, alias: &str) {
        if let Some((node, _)) = self.locate(namespace, token, true) {
            node.discovered = true;
            node.used_by.insert(alias.to_string());
        }
    }

    /// Find or create the node of a path token such as `input.body.items[*].sku`; the namespace
    /// itself names the root. Returns the node and whether it is the root; None when the token is
    /// not of the namespace.
    fn locate(
        &mut self,
        namespace: &'static str,
        token: &str,
        create: bool,
    ) -> Option<(&mut Node, bool)> {
        let headers = namespace == INPUT_HEADER || namespace == OUTPUT_HEADER;
        let root = self.roots.get_mut(namespace)?;
        if token == namespace {
            return Some((root, true));
        }
        let rest = token.strip_prefix(namespace)?.strip_prefix('.')?;
        let mut node = root;
        for segment in rest.split('.') {
            if segment.is_empty() {
                continue;
            }
            let (key, bracket) = match segment.find('[') {
                Some(i) => (&segment[..i], true),
                None => (segment, false),
            };
            if key.is_empty() {
                continue;
            }
            let lookup = if headers {
                key.to_lowercase()
            } else {
                key.to_string()
            };
            if !create && !node.children.contains_key(&lookup) {
                return None;
            }
            node = node.child(key, headers);
            if bracket {
                node.array = true;
            }
        }
        Some((node, false))
    }

    /// Tier 2: a typed constant, wrapper, plugin, for_each source or graph.math statement.
    fn direct_evidence(&mut self, alias: &str, property: &str, text: &str) {
        let tag = tag_of(text);
        match tag {
            Some("COMPUTE") | Some("CONDITION") | Some("DECIMAL") => {
                self.math_statement(alias, tag.unwrap_or_default(), &after_tag(text));
                return;
            }
            Some("IF") => {
                let first = after_tag(text);
                self.comparison_operands(first.split('\n').next().unwrap_or(""));
                return;
            }
            Some(other) if other != "MAPPING" => return,
            _ => {}
        }
        let body = after_tag(text);
        let Some(sep) = body.rfind(MAP_TO) else {
            return;
        };
        if sep == 0 {
            return;
        }
        let lhs = body[..sep].trim().to_string();
        let rhs = body[sep + MAP_TO.len()..].trim().to_string();
        if property == "for_each" {
            for ns in [INPUT_BODY, OUTPUT_BODY] {
                if let Some((node, is_root)) = self.locate(ns, &lhs, false) {
                    if !is_root {
                        node.array = true;
                    }
                }
            }
            return;
        }
        let type_name = self.type_of(&lhs);
        if rhs == OUTPUT_STATUS {
            if let Some(code) = integer_constant(&lhs) {
                self.status_codes.insert(code);
            }
            return;
        }
        if let Some(t) = type_name {
            self.assign(&rhs, &t);
        }
    }

    /// COMPUTE a number, CONDITION a boolean, DECIMAL a string, at {alias}.result.{var}.
    fn math_statement(&mut self, alias: &str, tag: &str, body: &str) {
        let Some(sep) = body.find(MAP_TO) else {
            return;
        };
        if sep == 0 {
            return;
        }
        let variable = body[..sep].trim();
        let expression = body[sep + MAP_TO.len()..].trim();
        let type_name = match tag {
            "COMPUTE" => NUMBER,
            "CONDITION" => BOOLEAN,
            _ => STRING,
        };
        if !variable.is_empty() {
            self.variables.insert(
                format!("{alias}.{RESULT}.{variable}"),
                type_name.to_string(),
            );
        }
        if tag == "COMPUTE" && has_arithmetic(expression) {
            // the operands of arithmetic are numbers: a boolean is never a number (4.12.18)
            for ns in [INPUT_BODY, INPUT_HEADER] {
                for token in collect_path_tokens(expression, ns) {
                    self.type_input(ns, &token, NUMBER);
                }
            }
        } else if tag == "CONDITION" {
            self.comparison_operands(expression);
        }
    }

    /// The operands of an ordered comparison (<, >, <=, >=) are numbers.
    fn comparison_operands(&mut self, expression: &str) {
        for clause in expression.split("&&").flat_map(|c| c.split("||")) {
            if clause.contains("==") || clause.contains("!=") {
                continue;
            }
            if clause.contains('<') || clause.contains('>') {
                for ns in [INPUT_BODY, INPUT_HEADER] {
                    for token in collect_path_tokens(clause, ns) {
                        self.type_input(ns, &token, NUMBER);
                    }
                }
            }
        }
    }

    fn type_input(&mut self, namespace: &'static str, token: &str, type_name: &str) {
        if let Some((node, is_root)) = self.locate(namespace, token, false) {
            if !is_root && node.type_name.is_none() {
                node.set_type(type_name);
            }
        }
    }

    /// Give a target - an output path or a variable - a type found on the source side.
    fn assign(&mut self, rhs: &str, type_name: &str) {
        for ns in [OUTPUT_BODY, OUTPUT_HEADER] {
            if let Some((node, is_root)) = self.locate(ns, rhs, false) {
                if node.type_name.is_none() && (!is_root || node.children.is_empty()) {
                    node.set_type(type_name);
                }
                return;
            }
        }
        if rhs.starts_with(MODEL_PREFIX)
            || rhs.contains(&format!(".{RESULT}."))
            || is_node_variable(rhs)
        {
            self.variables
                .entry(rhs.to_string())
                .or_insert_with(|| type_name.to_string());
        }
    }

    /// Tier 3, first hop: a variable fed by a typed input path takes its type.
    fn variable_from_input(&mut self, alias: &str, text: &str) {
        if let Some(tag) = tag_of(text) {
            if tag != "MAPPING" {
                return;
            }
        }
        let body = after_tag(text);
        let Some((lhs, rhs)) = split_mapping(&body) else {
            return;
        };
        if let Some(type_name) = self.input_type(&lhs) {
            if rhs.starts_with(MODEL_PREFIX) || is_node_variable(&rhs) {
                self.variables
                    .entry(qualified(alias, &rhs))
                    .or_insert(type_name);
            }
        }
    }

    /// Tier 3, second hop: an output path fed by a typed variable or a typed input takes the type.
    fn output_from_variable(&mut self, alias: &str, text: &str) {
        if let Some(tag) = tag_of(text) {
            if tag != "MAPPING" {
                return;
            }
        }
        let body = after_tag(text);
        let Some((lhs, rhs)) = split_mapping(&body) else {
            return;
        };
        let type_name = self
            .variables
            .get(&lhs)
            .cloned()
            .or_else(|| self.variables.get(&qualified(alias, &lhs)).cloned())
            .or_else(|| self.input_type(&lhs));
        if let Some(t) = type_name {
            for ns in [OUTPUT_BODY, OUTPUT_HEADER] {
                if let Some((node, is_root)) = self.locate(ns, &rhs, false) {
                    if !is_root && node.type_name.is_none() {
                        node.set_type(&t);
                    }
                }
            }
        }
    }

    fn input_type(&mut self, selector: &str) -> Option<String> {
        for ns in [INPUT_BODY, INPUT_HEADER] {
            if let Some((node, is_root)) = self.locate(ns, selector, false) {
                if !is_root {
                    return node.type_name.clone();
                }
                return None;
            }
        }
        None
    }

    /// A graph.extension target's declared output types the node's result.
    fn extension_target(&mut self, node: &Json, other_models: &dyn Fn(&str) -> Option<Json>) {
        let Some(properties) = node.get("properties").filter(|p| p.is_object()) else {
            return;
        };
        if properties.get("skill").and_then(|s| s.as_str()) != Some("graph.extension") {
            return;
        }
        let alias = alias_of(node);
        let Some(id) = properties.get("extension").and_then(|e| e.as_str()) else {
            return;
        };
        if id.contains("://") || id.trim().is_empty() {
            return;
        }
        let Some(model) = other_models(id.trim()) else {
            return;
        };
        let Some(end) = node_properties(&model, "end") else {
            return;
        };
        if let Some(properties) = end
            .get("schema")
            .and_then(|s| s.get("body"))
            .and_then(|b| b.get("properties"))
            .and_then(|p| p.as_object())
        {
            for (key, property) in properties {
                if let Some(type_name) = property.get("type").and_then(|t| t.as_str()) {
                    self.variables
                        .entry(format!("{alias}.{RESULT}.{key}"))
                        .or_insert_with(|| type_name.to_string());
                }
            }
        }
    }

    /// The type a mapping source carries: a constant wrapper, a plugin with a known result, or a
    /// wrapper around an input path (which types the path as well).
    fn type_of(&mut self, lhs: &str) -> Option<String> {
        let open = lhs.find('(')?;
        if open == 0 || !lhs.ends_with(')') {
            return None;
        }
        let name = lhs[..open].trim();
        let args = split_arguments(&lhs[open + 1..lhs.len() - 1]);
        let wrapper = match name {
            "int" | "long" => Some(INTEGER),
            "float" | "double" => Some(NUMBER),
            "boolean" => Some(BOOLEAN),
            "text" => Some(STRING),
            _ => None,
        };
        if let Some(w) = wrapper {
            if args.len() == 1 {
                self.type_input_path(&args[0], w);
            }
            return Some(w.to_string());
        }
        if let Some(plugin) = name.strip_prefix("f:") {
            if plugin == "defaultValue" || plugin == "ternary" {
                if args.len() > 1 {
                    let type_name = self.type_of(&args[1]);
                    if let Some(t) = &type_name {
                        if plugin == "defaultValue" {
                            self.type_input_path(&args[0], t);
                        }
                    }
                    return type_name;
                }
                return None;
            }
            let type_name = plugin_type(plugin);
            if let Some(t) = type_name {
                if args.len() == 1
                    && matches!(
                        plugin,
                        "int" | "long" | "float" | "double" | "boolean" | "text"
                    )
                {
                    self.type_input_path(&args[0], t);
                }
            }
            return type_name.map(str::to_string);
        }
        None
    }

    fn type_input_path(&mut self, argument: &str, type_name: &str) {
        let selector = argument.trim().to_string();
        for ns in [INPUT_BODY, INPUT_HEADER] {
            if let Some((node, is_root)) = self.locate(ns, &selector, false) {
                if !is_root && node.type_name.is_none() {
                    node.set_type(type_name);
                }
            }
        }
    }

    // ---------------------------------------------------------------- declaration

    fn declare(&mut self, model: &Json) {
        let root = node_properties(model, "root");
        let end = node_properties(model, "end");
        self.declare_part(root.as_ref(), "body", INPUT_BODY);
        self.declare_part(root.as_ref(), "header", INPUT_HEADER);
        self.declare_part(end.as_ref(), "body", OUTPUT_BODY);
        self.declare_part(end.as_ref(), "header", OUTPUT_HEADER);
    }

    fn declare_part(&mut self, properties: Option<&Json>, part: &str, namespace: &'static str) {
        let Some(schema) = properties
            .and_then(|p| p.get("schema"))
            .and_then(|s| s.get(part))
            .filter(|d| d.is_object())
        else {
            return;
        };
        self.declared.insert(namespace, true);
        let headers = namespace == INPUT_HEADER || namespace == OUTPUT_HEADER;
        let root = self.roots.get_mut(namespace).expect("a known namespace");
        root.declared = true;
        overlay(root, schema, headers);
    }

    // ---------------------------------------------------------------- reconcile

    /// Flag the gaps between declaration and discovery; a namespace without a declaration flags nothing.
    fn reconcile(&mut self) {
        let mut issues = Vec::new();
        for ns in NAMESPACES {
            if !self.is_declared(ns) {
                continue;
            }
            walk(self.root(ns), ns, &mut |path, node| {
                if node.declared && !node.discovered && node.children.is_empty() {
                    issues.push(format!(
                        "{path} is declared but never referenced by the model"
                    ));
                } else if node.discovered && !node.declared {
                    let by: Vec<&str> = node.used_by.iter().map(String::as_str).collect();
                    issues.push(format!(
                        "{path} is referenced by {} but not declared",
                        by.join(", ")
                    ));
                }
            });
        }
        self.issues = issues;
    }

    // ---------------------------------------------------------------- views

    /// The OpenAPI schema object of a namespace (None when nothing is referenced or declared).
    pub fn schema(&self, namespace: &str) -> Option<Value> {
        if !self.has(namespace) {
            return None;
        }
        Some(schema_of(self.root(namespace)))
    }

    /// The contract view: the merged schemas with their evidence, for the Playground and for agents.
    pub fn to_view(&self) -> Value {
        let mut view: Vec<(Value, Value)> =
            vec![(Value::from("graph"), Value::from(self.graph_id.as_str()))];
        if let Some(purpose) = &self.purpose {
            view.push((Value::from("purpose"), Value::from(purpose.as_str())));
        }
        view.push((
            Value::from("input"),
            map(vec![
                ("body", self.namespace_view(INPUT_BODY)),
                ("header", self.namespace_view(INPUT_HEADER)),
            ]),
        ));
        view.push((
            Value::from("output"),
            map(vec![
                ("body", self.namespace_view(OUTPUT_BODY)),
                ("header", self.namespace_view(OUTPUT_HEADER)),
                (
                    "status",
                    Value::Array(self.status_codes.iter().map(|c| Value::from(*c)).collect()),
                ),
            ]),
        ));
        view.push((
            Value::from("issues"),
            Value::Array(
                self.issues
                    .iter()
                    .map(|i| Value::from(i.as_str()))
                    .collect(),
            ),
        ));
        Value::Map(view)
    }

    fn namespace_view(&self, namespace: &'static str) -> Value {
        let mut view: Vec<(&str, Value)> =
            vec![("declared", Value::from(self.is_declared(namespace)))];
        if let Some(schema) = self.schema(namespace) {
            view.push(("schema", schema));
        }
        let mut paths = Vec::new();
        walk(self.root(namespace), namespace, &mut |path, node| {
            let mut entry: Vec<(&str, Value)> = vec![("path", Value::from(path))];
            if node.array {
                entry.push(("type", Value::from(ARRAY)));
            } else if let Some(t) = &node.type_name {
                entry.push(("type", Value::from(t.as_str())));
            } else if !node.children.is_empty() {
                entry.push(("type", Value::from(OBJECT)));
            }
            let origin = if node.declared && node.discovered {
                "both"
            } else if node.declared {
                "declared"
            } else {
                "discovered"
            };
            entry.push(("origin", Value::from(origin)));
            if node.required {
                entry.push(("required", Value::from(true)));
            }
            if !node.used_by.is_empty() {
                entry.push((
                    "usedBy",
                    Value::Array(
                        node.used_by
                            .iter()
                            .map(|a| Value::from(a.as_str()))
                            .collect(),
                    ),
                ));
            }
            paths.push(map(entry));
        });
        view.push(("paths", Value::Array(paths)));
        map(view)
    }

    /// The lines of `describe graph`: the surfaces with their types, then the declaration.
    pub fn describe(&self) -> String {
        let mut sb = String::from("Input surface:\n");
        let mut inputs = self.surface_lines(INPUT_BODY);
        inputs.extend(self.surface_lines(INPUT_HEADER));
        if inputs.is_empty() {
            sb.push_str("  (none referenced)\n");
        }
        for line in &inputs {
            sb.push_str(&format!("  {line}\n"));
        }
        sb.push_str("Output surface:\n");
        let mut outputs = self.surface_lines(OUTPUT_BODY);
        outputs.extend(self.surface_lines(OUTPUT_HEADER));
        if !self.status_codes.is_empty() {
            let codes: Vec<String> = self.status_codes.iter().map(|c| c.to_string()).collect();
            outputs.push(format!("{OUTPUT_STATUS} {}", codes.join(", ")));
        }
        if outputs.is_empty() {
            sb.push_str("  (none referenced)\n");
        }
        for line in &outputs {
            sb.push_str(&format!("  {line}\n"));
        }
        let mut parts = Vec::new();
        if self.is_declared(INPUT_BODY) || self.is_declared(INPUT_HEADER) {
            parts.push("input");
        }
        if self.is_declared(OUTPUT_BODY) || self.is_declared(OUTPUT_HEADER) {
            parts.push("output");
        }
        sb.push_str("Declared schema: ");
        sb.push_str(if parts.is_empty() {
            "none"
        } else if parts.len() == 2 {
            "input, output"
        } else {
            parts[0]
        });
        sb.push('\n');
        sb
    }

    fn surface_lines(&self, namespace: &'static str) -> Vec<String> {
        let mut lines = Vec::new();
        let root = self.root(namespace);
        if root.discovered {
            lines.push(match &root.type_name {
                Some(t) => format!("{namespace} ({t})"),
                None => namespace.to_string(),
            });
        }
        walk(root, namespace, &mut |path, node| {
            if !node.children.is_empty() {
                return;
            }
            let mut line = path.to_string();
            if let Some(t) = &node.type_name {
                line.push_str(&format!(" ({t})"));
            }
            if node.declared && !node.discovered {
                line.push_str(" [declared]");
            }
            lines.push(line);
        });
        lines
    }
}

// -------------------------------------------------------------------- helpers

fn node_list(model: &Json) -> Vec<Json> {
    model
        .get("nodes")
        .and_then(|n| n.as_array())
        .map(|nodes| nodes.iter().filter(|n| n.is_object()).cloned().collect())
        .unwrap_or_default()
}

fn alias_of(node: &Json) -> String {
    match node.get("alias") {
        Some(Json::String(s)) => s.clone(),
        Some(other) => other.to_string(),
        None => "null".to_string(),
    }
}

fn mapping_entries(nodes: &[Json]) -> Vec<Entry> {
    let mut result = Vec::new();
    for node in nodes {
        let alias = alias_of(node);
        let Some(properties) = node.get("properties").filter(|p| p.is_object()) else {
            continue;
        };
        for property in MAPPING_PROPERTIES {
            match properties.get(property) {
                Some(Json::String(s)) => result.push(Entry {
                    alias: alias.clone(),
                    property,
                    text: s.clone(),
                }),
                Some(Json::Array(list)) => {
                    for item in list {
                        if let Json::String(s) = item {
                            result.push(Entry {
                                alias: alias.clone(),
                                property,
                                text: s.clone(),
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }
    result
}

fn root_property(model: &Json, key: &str) -> Option<String> {
    node_properties(model, "root")
        .and_then(|p| {
            p.get(key)
                .and_then(|v| v.as_str())
                .map(str::trim)
                .map(str::to_string)
        })
        .filter(|s| !s.is_empty())
}

fn node_properties(model: &Json, alias: &str) -> Option<Json> {
    node_list(model)
        .into_iter()
        .find(|n| n.get("alias").and_then(|a| a.as_str()) == Some(alias))
        .and_then(|n| n.get("properties").filter(|p| p.is_object()).cloned())
}

/// Collect the dotted-path tokens starting with the prefix from free text.
pub fn collect_path_tokens(text: &str, prefix: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let bytes = text.as_bytes();
    let mut start = 0;
    while let Some(pos) = text[start..].find(prefix) {
        let begin = start + pos;
        // a mid-word match is part of a longer identifier, not a path token - except the JSONPath
        // form "$.input.body...", whose dot follows the root symbol
        if begin > 0 {
            let prev = bytes[begin - 1] as char;
            let json_path = prev == '.' && begin > 1 && bytes[begin - 2] as char == '$';
            if is_word_char(prev) && !json_path {
                start = begin + prefix.len();
                continue;
            }
        }
        let mut end = begin + prefix.len();
        // a namespace prefix must end the token or be followed by a separator (input.body vs
        // input.bodyish); a prefix that ends with the separator itself ("input.") continues
        if !prefix.ends_with('.') && end < bytes.len() {
            let next = bytes[end] as char;
            if next.is_ascii_alphanumeric() || next == '_' {
                start = end;
                continue;
            }
        }
        while end < bytes.len() && is_token_char(bytes[end] as char) {
            end += 1;
        }
        let token = trim_token(&text[begin..end]);
        if token.len() >= prefix.len() {
            found.insert(token);
        }
        start = end;
    }
    found
}

fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.'
}

fn is_token_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || c == '.'
        || c == '_'
        || c == '-'
        || c == '['
        || c == ']'
        || c == '*'
}

/// Strip trailing separators, then every trailing `]` the token's own `[` do not balance (an
/// enclosing list's closing bracket absorbed from an unquoted serialization form).
fn trim_token(token: &str) -> String {
    let mut result = token.to_string();
    while result.ends_with('.') || result.ends_with('-') || result.ends_with('[') {
        result.pop();
    }
    while result.ends_with(']') && result.matches(']').count() > result.matches('[').count() {
        result.pop();
    }
    result
}

fn tag_of(text: &str) -> Option<&str> {
    let colon = text.find(':')?;
    if colon == 0 {
        return None;
    }
    let tag = text[..colon].trim();
    match tag {
        "MAPPING" | "COMPUTE" | "CONDITION" | "DECIMAL" | "IF" | "RESET" | "NEXT" | "DELAY"
        | "THEN" | "ELSE" => Some(tag),
        _ => None,
    }
}

fn after_tag(text: &str) -> String {
    match tag_of(text) {
        Some(_) => text[text.find(':').unwrap_or(0) + 1..].trim().to_string(),
        None => text.trim().to_string(),
    }
}

fn split_mapping(body: &str) -> Option<(String, String)> {
    let sep = body.rfind(MAP_TO)?;
    if sep == 0 {
        return None;
    }
    Some((
        body[..sep].trim().to_string(),
        body[sep + MAP_TO.len()..].trim().to_string(),
    ))
}

/// A node-relative selector such as `result.total` is `{alias}.result.total`.
fn qualified(alias: &str, selector: &str) -> String {
    if selector == RESULT || selector.starts_with("result.") {
        format!("{alias}.{selector}")
    } else {
        selector.to_string()
    }
}

fn is_node_variable(selector: &str) -> bool {
    !selector.starts_with("input.")
        && !selector.starts_with("output.")
        && selector.find('.').map(|i| i > 0).unwrap_or(false)
        && !selector.contains('(')
}

fn has_arithmetic(expression: &str) -> bool {
    let mut depth = 0i32;
    for c in expression.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            '+' | '-' | '*' | '/' | '%' if depth == 0 => return true,
            _ => {}
        }
    }
    false
}

fn integer_constant(lhs: &str) -> Option<i64> {
    if (lhs.starts_with("int(") || lhs.starts_with("long(")) && lhs.ends_with(')') {
        let digits = lhs[lhs.find('(')? + 1..lhs.len() - 1].trim();
        if !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()) {
            return digits.parse().ok();
        }
    }
    None
}

fn split_arguments(text: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut depth = 0i32;
    let mut start = 0;
    for (i, c) in text.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                result.push(text[start..i].trim().to_string());
                start = i + 1;
            }
            _ => {}
        }
    }
    let last = text[start..].trim().to_string();
    if !last.is_empty() || !result.is_empty() {
        result.push(last);
    }
    result
}

/// Overlay a declared schema object on a node: the declaration wins.
fn overlay(node: &mut Node, schema: &Json, headers: bool) {
    node.declared = true;
    let type_name = schema.get("type").and_then(|t| t.as_str());
    if type_name == Some(ARRAY) {
        node.array = true;
        if let Some(items) = schema.get("items").filter(|i| i.is_object()) {
            if let Some(item_type) = items.get("type").and_then(|t| t.as_str()) {
                if item_type != OBJECT {
                    node.type_name = Some(item_type.to_string());
                }
            }
            if let Some(properties) = items.get("properties").filter(|p| p.is_object()) {
                overlay_properties(node, properties, items.get("required"), headers);
            }
            keep_fragment(items, &mut node.item_fragment);
        }
        keep_fragment(schema, &mut node.fragment);
        return;
    }
    if let Some(t) = type_name {
        if t != OBJECT {
            node.type_name = Some(t.to_string());
        }
    }
    if let Some(properties) = schema.get("properties").filter(|p| p.is_object()) {
        overlay_properties(node, properties, schema.get("required"), headers);
    }
    keep_fragment(schema, &mut node.fragment);
}

fn overlay_properties(node: &mut Node, properties: &Json, required: Option<&Json>, headers: bool) {
    if let Some(map) = properties.as_object() {
        for (key, property_schema) in map {
            if property_schema.is_object() {
                let child = node.child(key, headers);
                child.name = key.clone();
                overlay(child, property_schema, headers);
            }
        }
    }
    if let Some(list) = required.and_then(|r| r.as_array()) {
        for name in list {
            let key = match name {
                Json::String(s) => s.clone(),
                other => other.to_string(),
            };
            let lookup = if headers { key.to_lowercase() } else { key };
            if let Some(child) = node.children.get_mut(&lookup) {
                child.required = true;
            }
        }
    }
}

fn keep_fragment(schema: &Json, fragment: &mut Vec<(String, Json)>) {
    if let Some(map) = schema.as_object() {
        for (key, value) in map {
            if key != "type" && key != "properties" && key != "items" && key != "required" {
                fragment.push((key.clone(), value.clone()));
            }
        }
    }
}

pub(crate) fn walk(node: &Node, path: &str, visitor: &mut dyn FnMut(&str, &Node)) {
    for child in node.children.values() {
        let child_path = format!(
            "{path}.{}{}",
            child.name,
            if child.array { "[]" } else { "" }
        );
        visitor(&child_path, child);
        walk(child, &child_path, visitor);
    }
}

pub(crate) fn map(entries: Vec<(&str, Value)>) -> Value {
    Value::Map(
        entries
            .into_iter()
            .map(|(k, v)| (Value::from(k), v))
            .collect(),
    )
}

fn put(entries: &mut Vec<(Value, Value)>, key: &str, value: Value) {
    if let Some(slot) = entries.iter_mut().find(|(k, _)| k.as_str() == Some(key)) {
        slot.1 = value;
    } else {
        entries.push((Value::from(key), value));
    }
}

/// The OpenAPI schema object of a path.
pub(crate) fn schema_of(node: &Node) -> Value {
    let mut schema: Vec<(Value, Value)> = Vec::new();
    if node.array {
        put(&mut schema, "type", Value::from(ARRAY));
        let mut items: Vec<(Value, Value)> = Vec::new();
        element_schema(node, &mut items, &node.item_fragment, false);
        put(&mut schema, "items", Value::Map(items));
        if !node.declared && !node.used_by.is_empty() {
            put(
                &mut schema,
                "description",
                Value::from(referenced_by(node).as_str()),
            );
        }
        for (key, value) in &node.fragment {
            put(&mut schema, key, from_json(value));
        }
        return Value::Map(schema);
    }
    element_schema(node, &mut schema, &node.fragment, true);
    Value::Map(schema)
}

fn referenced_by(node: &Node) -> String {
    let by: Vec<&str> = node.used_by.iter().map(String::as_str).collect();
    format!("Referenced by {}", by.join(", "))
}

fn element_schema(
    node: &Node,
    schema: &mut Vec<(Value, Value)>,
    fragment: &[(String, Json)],
    describe: bool,
) {
    if !node.children.is_empty() {
        put(schema, "type", Value::from(OBJECT));
        let mut properties: Vec<(Value, Value)> = Vec::new();
        let mut required = Vec::new();
        for child in node.children.values() {
            properties.push((Value::from(child.name.as_str()), schema_of(child)));
            if child.required {
                required.push(Value::from(child.name.as_str()));
            }
        }
        put(schema, "properties", Value::Map(properties));
        if !required.is_empty() {
            put(schema, "required", Value::Array(required));
        }
    } else if let Some(t) = &node.type_name {
        put(schema, "type", Value::from(t.as_str()));
    }
    if describe && !node.declared && !node.used_by.is_empty() && node.children.is_empty() {
        put(
            schema,
            "description",
            Value::from(referenced_by(node).as_str()),
        );
    }
    for (key, value) in fragment {
        put(schema, key, from_json(value));
    }
}
