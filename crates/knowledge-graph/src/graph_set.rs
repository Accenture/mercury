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

//! Rust port of `com.accenture.minigraph.common.GraphSet` — a graph set:
//! graph models delivered together as one canonical package (RFC-0005; the
//! package format is ADR-0026, [`platform_core::canonical_packager`]).
//!
//! One package is one set, and its file is `<set>.pack`. Each graph is one
//! entry named `<graph-id>.json`, whose id follows the file-name rule and,
//! when the root node declares a 'name', equals it. The manifest holds the
//! packager's 'format' and 'format_version', the set name in 'set', and caller
//! fields as text - for example 'version', 'description' or 'author', and the
//! optional 'graph_id' naming the set's entry-point graph. Nothing is taken
//! from the environment or the clock, so the same graphs and fields always
//! give the same bytes, in either engine.
//!
//! A graph holds no null property: `"key": null` is filtered out when a set is
//! packed or read, as the Java engine's serializer does by default
//! ([`model_gate::without_null_properties`]). An empty string is a value and
//! is kept.
//!
//! The rules live here so the graph packager, the deployment of packaged sets
//! and the Playground apply the same ones; the messages match the Java engine
//! word for word.

use std::collections::BTreeMap;
use std::fmt;

use platform_core::canonical_packager::{self, Builder};
use platform_core::{ConfigReader, ConfigValue};
use rmpv::Value;

use crate::model_gate;

/// The file extension of a graph set.
pub const EXTENSION: &str = ".pack";
/// The manifest field holding the set name.
pub const SET: &str = "set";
/// The optional manifest field naming the set's entry-point graph.
pub const GRAPH_ID: &str = "graph_id";
const JSON_EXT: &str = ".json";
const ID_RULE: &str = "use letters, digits, '_' and '-' only";

/// The content of a graph set that passed the read checks, in the order the
/// package holds it.
#[derive(Clone, Debug, PartialEq)]
pub struct Contents {
    /// The manifest fields, `format` and `format_version` included.
    pub manifest: Vec<(String, String)>,
    /// The graph models keyed by graph id.
    pub graphs: Vec<(String, Value)>,
}

impl Contents {
    /// The value of a manifest field.
    pub fn manifest_field(&self, key: &str) -> Option<&str> {
        self.manifest
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    /// The model of a graph of the set.
    pub fn graph(&self, id: &str) -> Option<&Value> {
        self.graphs.iter().find(|(k, _)| k == id).map(|(_, v)| v)
    }
}

/// Why a graph set is not accepted.
#[derive(Clone, Debug, PartialEq)]
pub enum GraphSetError {
    /// Every rule the set breaks, one reason each (Java `RefusedException`).
    Refused(Vec<String>),
    /// The bytes are not a canonical package, or a value cannot be written.
    Format(String),
}

impl fmt::Display for GraphSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphSetError::Refused(reasons) => write!(f, "{}", reasons.join("; ")),
            GraphSetError::Format(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for GraphSetError {}

/// Pack graph models into a set. Every rule is checked before anything is
/// packed - the set name and each graph id against the file-name rule, each
/// root 'name' against its graph id, the 'graph_id' field against the graphs,
/// and each model against the deployment gate's checks - and the set is
/// refused with every reason when any fails. A null property is filtered out
/// first: a graph holds none.
///
/// The gate checks a copy read the way the gate reads a deployed model,
/// normalized and with its `${...}` references resolved, while the model is
/// packed as written: a reference belongs to the environment the set is
/// deployed to, and resolving it here would make the bytes depend on the
/// machine that packs them.
pub fn pack(
    set_name: &str,
    fields: &[(String, String)],
    graphs: &BTreeMap<String, Value>,
) -> Result<Vec<u8>, GraphSetError> {
    let mut reasons = Vec::new();
    if !model_gate::is_valid_graph_id(set_name) {
        reasons.push(format!("set name '{set_name}' - {ID_RULE}"));
    }
    for (key, _) in fields {
        if key.is_empty() {
            reasons.push("a manifest field needs a name".to_string());
        } else if key == SET {
            reasons.push(format!(
                "manifest field '{SET}' - it is written from the set name"
            ));
        } else if key == canonical_packager::FORMAT_KEY
            || key == canonical_packager::FORMAT_VERSION_KEY
        {
            reasons.push(format!(
                "manifest field '{key}' - it is written by the packager"
            ));
        }
    }
    if graphs.is_empty() {
        reasons.push("a set needs at least one graph".to_string());
    }
    if let Some((_, entry_point)) = fields.iter().find(|(k, _)| k == GRAPH_ID) {
        if !graphs.contains_key(entry_point) {
            reasons.push(format!(
                "manifest field '{GRAPH_ID}' - '{entry_point}' is not a graph of the set"
            ));
        }
    }
    // a graph holds no null property: "key": null is filtered out before the checks and the pack
    let models: BTreeMap<&String, Value> = graphs
        .iter()
        .map(|(graph_id, model)| (graph_id, model_gate::without_null_properties(model)))
        .collect();
    for (graph_id, model) in &models {
        if let Some(reason) = check(graph_id, model) {
            reasons.push(format!("{graph_id}: {reason}"));
        }
    }
    if !reasons.is_empty() {
        return Err(GraphSetError::Refused(reasons));
    }
    let refused = |e: canonical_packager::PackagerError| {
        GraphSetError::Refused(vec![e.message().to_string()])
    };
    let mut builder = Builder::new();
    for (key, value) in fields {
        builder = builder.manifest(key, value).map_err(refused)?;
    }
    builder = builder.manifest(SET, set_name).map_err(refused)?;
    for (graph_id, model) in models {
        builder = builder
            .add(&format!("{graph_id}{JSON_EXT}"), model)
            .map_err(refused)?;
    }
    builder
        .build()
        .map_err(|e| GraphSetError::Format(e.message().to_string()))
}

/// Read a set: the strict canonical read, then the entry names, the root names
/// and the 'graph_id' field are checked before anything is built from them, so
/// a crafted entry name never becomes a path.
pub fn read(bytes: &[u8]) -> Result<Contents, GraphSetError> {
    let package = canonical_packager::unpack(bytes)
        .map_err(|e| GraphSetError::Format(e.message().to_string()))?;
    let mut reasons = Vec::new();
    let mut graphs = Vec::with_capacity(package.maps.len());
    for (name, model) in &package.maps {
        let id = name.strip_suffix(JSON_EXT).unwrap_or("");
        if !model_gate::is_valid_graph_id(id) {
            reasons.push(format!(
                "entry '{name}' - expect <graph-id>.json, the id in letters, digits, '_' and '-'"
            ));
            continue;
        }
        let declared = model_gate::declared_root_name(model);
        if !declared.is_empty() && declared != id {
            reasons.push(format!("{id}: {}", root_name_differs(&declared)));
        }
        if let Some(path) = find_binary(model, "") {
            reasons.push(format!(
                "{id}: binary data at '{path}' - a graph model is JSON"
            ));
        }
        graphs.push((id.to_string(), model_gate::without_null_properties(model)));
    }
    if package.maps.is_empty() {
        reasons.push("the package holds no graph".to_string());
    }
    if let Some((_, entry_point)) = package.manifest.iter().find(|(k, _)| k == GRAPH_ID) {
        if !graphs.iter().any(|(id, _)| id == entry_point) {
            reasons.push(format!(
                "manifest field '{GRAPH_ID}' - '{entry_point}' is not a graph of the set"
            ));
        }
    }
    if !reasons.is_empty() {
        return Err(GraphSetError::Refused(reasons));
    }
    Ok(Contents {
        manifest: package.manifest,
        graphs,
    })
}

/// Readable JSON for a graph model taken out of a set: the canonical key
/// order and a two-space indent, so two versions of a set diff cleanly and the
/// file packs to the same bytes again. Ends with a new line.
pub fn to_json(model: &Value) -> String {
    let json = event_script::conversions::to_json(model).unwrap_or(serde_json::Value::Null);
    let mut text = serde_json::to_string_pretty(&json).unwrap_or_default();
    text.push('\n');
    text
}

fn check(graph_id: &str, model: &Value) -> Option<String> {
    if !model_gate::is_valid_graph_id(graph_id) {
        return Some(format!("graph id - {ID_RULE}"));
    }
    let declared = model_gate::declared_root_name(model);
    if !declared.is_empty() && declared != graph_id {
        return Some(root_name_differs(&declared));
    }
    let mut deployed = as_deployed(model);
    model_gate::validate(graph_id, &mut deployed).err()
}

/// The model the way the gate reads a deployed graph: normalized, with its
/// `${...}` references resolved (Java `new ConfigReader().load(map)`).
fn as_deployed(model: &Value) -> Value {
    let Some(json) = event_script::conversions::to_json(model) else {
        return model.clone();
    };
    match ConfigValue::from_json(&json) {
        ConfigValue::Map(map) => {
            let reader = ConfigReader::from_map(map);
            let resolved = ConfigValue::Map(reader.get_map().clone().into_map()).to_json();
            event_script::conversions::from_json(&resolved)
        }
        _ => model.clone(),
    }
}

fn root_name_differs(declared: &str) -> String {
    format!("the root node's name '{declared}' differs from the graph id")
}

fn find_binary(value: &Value, path: &str) -> Option<String> {
    match value {
        Value::Binary(_) => Some(path.to_string()),
        Value::Map(entries) => entries.iter().find_map(|(key, v)| {
            let key = event_script::conversions::display(key);
            let child = if path.is_empty() {
                key
            } else {
                format!("{path}.{key}")
            };
            find_binary(v, &child)
        }),
        Value::Array(items) => items
            .iter()
            .enumerate()
            .find_map(|(i, v)| find_binary(v, &format!("{path}[{i}]"))),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph(json: &str) -> Value {
        event_script::conversions::from_json(&serde_json::from_str(json).expect("valid JSON"))
    }

    fn valid(id: &str) -> Value {
        graph(&format!(
            r#"{{"nodes": [
              {{"alias": "root", "types": ["Root"], "properties": {{"purpose": "a valid graph", "name": "{id}"}}}},
              {{"alias": "end", "types": ["End"],
                "properties": {{"skill": "graph.data.mapper", "mapping": ["text(ok) -> output.body"]}}}}],
             "connections": [{{"source": "root", "target": "end", "relations": [{{"type": "done", "properties": {{}}}}]}}]}}"#
        ))
    }

    fn fields(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn the_root_name_is_read_from_the_root_node() {
        assert_eq!("quote", model_gate::declared_root_name(&valid(" quote ")));
        assert_eq!(
            "",
            model_gate::declared_root_name(&graph(
                r#"{"nodes": [{"alias": "root", "types": ["Root"], "properties": {"purpose": "p"}}]}"#
            ))
        );
        assert_eq!(
            "",
            model_gate::declared_root_name(&graph(r#"{"connections": []}"#))
        );
    }

    #[test]
    fn the_gate_converts_deprecated_syntax_in_place() {
        let mut model = valid("old");
        let mut mm = event_script::mlm::MultiLevelMap::from_value(model.clone());
        mm.set_element(
            "nodes[1].properties.mapping",
            Value::Array(vec![Value::from("model.number:int -> output.body")]),
        )
        .expect("set");
        model = mm.to_value();
        model_gate::validate("old", &mut model).expect("valid");
        let mm = event_script::mlm::MultiLevelMap::from_value(model);
        assert_eq!(
            Some(Value::Array(vec![Value::from(
                "f:int(model.number) -> output.body"
            )])),
            mm.get_element("nodes[1].properties.mapping")
        );
    }

    #[test]
    fn packing_refuses_with_every_reason() {
        let mut graphs = BTreeMap::new();
        graphs.insert("good".to_string(), valid("good"));
        graphs.insert("bad.id".to_string(), valid("bad.id"));
        graphs.insert("renamed".to_string(), valid("other"));
        graphs.insert(
            "no-end".to_string(),
            graph(r#"{"nodes": [{"alias": "root", "types": ["Root"], "properties": {"purpose": "p"}}], "connections": []}"#),
        );
        let fields = fields(&[("set", "x"), ("format", "y"), ("graph_id", "missing")]);
        assert_eq!(
            Err(GraphSetError::Refused(vec![
                "set name 'bad set' - use letters, digits, '_' and '-' only".to_string(),
                "manifest field 'set' - it is written from the set name".to_string(),
                "manifest field 'format' - it is written by the packager".to_string(),
                "manifest field 'graph_id' - 'missing' is not a graph of the set".to_string(),
                "bad.id: graph id - use letters, digits, '_' and '-' only".to_string(),
                "no-end: graph must have an 'end' node".to_string(),
                "renamed: the root node's name 'other' differs from the graph id".to_string(),
            ])),
            pack("bad set", &fields, &graphs)
        );
        assert_eq!(
            Err(GraphSetError::Refused(vec![
                "a set needs at least one graph".to_string()
            ])),
            pack("s", &[], &BTreeMap::new())
        );
    }

    #[test]
    fn a_set_packs_deterministically_and_reads_back() {
        let mut graphs = BTreeMap::new();
        graphs.insert("b".to_string(), valid("b"));
        graphs.insert("a".to_string(), valid("a"));
        let bytes = pack(
            "pair",
            &fields(&[("version", "2"), ("graph_id", "a")]),
            &graphs,
        )
        .expect("packs");
        let again = pack(
            "pair",
            &fields(&[("graph_id", "a"), ("version", "2")]),
            &graphs,
        )
        .expect("packs");
        assert_eq!(bytes, again);
        let contents = read(&bytes).expect("reads");
        let keys: Vec<&str> = contents.manifest.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(
            vec!["format", "format_version", "graph_id", "set", "version"],
            keys
        );
        assert_eq!(Some("pair"), contents.manifest_field("set"));
        let ids: Vec<&str> = contents.graphs.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(vec!["a", "b"], ids);
        assert_eq!(
            canonical_packager::encode(&valid("a")).expect("encodes"),
            canonical_packager::encode(contents.graph("a").expect("a")).expect("encodes")
        );
    }

    #[test]
    fn reading_refuses_what_a_set_must_not_hold() {
        let binary = Value::Map(vec![(
            Value::from("nodes"),
            Value::Array(vec![Value::Map(vec![(
                Value::from("data"),
                Value::Binary(vec![1, 2]),
            )])]),
        )]);
        let bytes = Builder::new()
            .manifest("graph_id", "absent")
            .and_then(|b| b.add("../escape.json", valid("escape")))
            .and_then(|b| b.add("notes.txt", graph(r#"{"a": 1}"#)))
            .and_then(|b| b.add("renamed.json", valid("other")))
            .and_then(|b| b.add("binary.json", binary))
            .and_then(|b| b.build())
            .expect("a package");
        assert_eq!(
            Err(GraphSetError::Refused(vec![
                "entry '../escape.json' - expect <graph-id>.json, the id in letters, digits, '_' and '-'".to_string(),
                "binary: binary data at 'nodes[0].data' - a graph model is JSON".to_string(),
                "entry 'notes.txt' - expect <graph-id>.json, the id in letters, digits, '_' and '-'".to_string(),
                "renamed: the root node's name 'other' differs from the graph id".to_string(),
                "manifest field 'graph_id' - 'absent' is not a graph of the set".to_string(),
            ])),
            read(&bytes)
        );
        let empty = Builder::new().build().expect("an empty package");
        assert_eq!(
            Err(GraphSetError::Refused(vec![
                "the package holds no graph".to_string()
            ])),
            read(&empty)
        );
        let mut trailing = empty.clone();
        trailing.push(0);
        assert!(matches!(read(&trailing), Err(GraphSetError::Format(_))));
    }

    const WITH_NULLS: &str = r#"{"nodes": [
      {"alias": "root", "types": ["Root"],
       "properties": {"purpose": "p", "name": "n", "note": null, "empty": "", "flags": [true, null]}},
      {"alias": "end", "types": ["End"], "properties": {}}],
     "connections": [{"source": "root", "target": "end", "relations": [{"type": "done", "properties": {"x": null}}]}]}"#;

    const WITHOUT_NULLS: &str = r#"{"nodes": [
      {"alias": "root", "types": ["Root"],
       "properties": {"purpose": "p", "name": "n", "empty": "", "flags": [true, null]}},
      {"alias": "end", "types": ["End"], "properties": {}}],
     "connections": [{"source": "root", "target": "end", "relations": [{"type": "done", "properties": {}}]}]}"#;

    fn one_graph(json: &str) -> BTreeMap<String, Value> {
        let mut graphs = BTreeMap::new();
        graphs.insert("n".to_string(), graph(json));
        graphs
    }

    #[test]
    fn a_null_property_is_filtered_out_and_an_empty_string_kept() {
        let bytes = pack("n", &[], &one_graph(WITH_NULLS))
            .expect("a null property is filtered out, not refused");
        // the same bytes as the graph written without its null properties
        assert_eq!(
            pack("n", &[], &one_graph(WITHOUT_NULLS)).expect("packs"),
            bytes
        );
        let contents = read(&bytes).expect("reads");
        let mm =
            event_script::mlm::MultiLevelMap::from_value(contents.graph("n").expect("n").clone());
        assert_eq!(None, mm.get_element("nodes[0].properties.note"));
        assert_eq!(
            Some(Value::from("")),
            mm.get_element("nodes[0].properties.empty")
        );
        // a list keeps its elements in place
        assert_eq!(
            Some(Value::Array(vec![Value::Boolean(true), Value::Nil])),
            mm.get_element("nodes[0].properties.flags")
        );
        let json = to_json(contents.graph("n").expect("n"));
        assert!(json.starts_with("{\n  \"connections\": [\n"), "{json}");
        assert!(
            !json.contains("\"note\"") && !json.contains("\"x\""),
            "{json}"
        );
        assert!(json.contains("\"empty\": \"\""), "{json}");
        assert!(json.ends_with("}\n"));
        assert_eq!(
            canonical_packager::encode(contents.graph("n").expect("n")).expect("encodes"),
            canonical_packager::encode(&graph(&json)).expect("encodes")
        );
    }

    #[test]
    fn reading_filters_a_null_property_out() {
        let bytes = Builder::new()
            .add("n.json", graph(WITH_NULLS))
            .and_then(|b| b.build())
            .expect("a package");
        assert_eq!(
            canonical_packager::encode(&graph(WITHOUT_NULLS)).expect("encodes"),
            canonical_packager::encode(read(&bytes).expect("reads").graph("n").expect("n"))
                .expect("encodes")
        );
    }
}
