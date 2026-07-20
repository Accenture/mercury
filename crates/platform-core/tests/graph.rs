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

//! Parity port of the Java `GraphTest` (platform-core) — same graph shapes,
//! same assertions, same error messages.

use std::collections::HashSet;
use std::sync::Arc;

use platform_core::graph::MiniGraph;
use rmpv::Value;

fn types_of(node: &Arc<platform_core::graph::SimpleNode>) -> HashSet<String> {
    node.get_types()
}

/// Java `getGraphWithRootNode`: a graph holding only the root node, whose
/// default type is "root".
fn graph_with_root_node() -> MiniGraph {
    let graph = MiniGraph::new();
    let root = graph.create_root_node().unwrap();
    let types = root.get_types();
    assert_eq!(1, types.len());
    assert!(types.contains("root"));
    graph
}

#[test]
fn node_test() {
    let graph = graph_with_root_node();
    graph.get_root_node().unwrap().add_type("hello").unwrap();
    let node_a = graph.create_node("A", "transaction").unwrap();
    let node_b = graph.create_node("B", "data").unwrap();
    let node_c = graph.create_node("C", "data").unwrap();
    let node_d = graph.create_node("D", "data").unwrap();
    let node_e = graph.create_node("E", "data").unwrap();
    graph.connect("root", "A").unwrap();
    let r1 = graph.connect("A", "B").unwrap();
    // connection between 2 nodes is unique — the existing one is returned
    let r1a = graph.connect("A", "B").unwrap();
    assert_eq!(r1, r1a);
    let r2 = graph.connect("A", "C").unwrap();
    // Java compareRelation: relation on an existing connection + lookups
    let r2_again = graph.connect("A", "C").unwrap();
    let r2_relation = r2_again.add_relation("Demo");
    r2_relation
        .add_property("some", Value::from("relation"))
        .unwrap();
    let found_relations = graph.find_relation_by_type("demo").unwrap();
    assert_eq!(1, found_relations.len());
    assert_eq!(*r2_relation, *found_relations[0]);
    let r2a = graph
        .find_connection(
            found_relations[0].get_source_alias(),
            found_relations[0].get_target_alias(),
        )
        .unwrap()
        .unwrap();
    assert_eq!(r2_again, r2a);
    let all = r2_again.get_relations();
    assert_eq!(1, all.len());
    assert_eq!(*found_relations[0], *all[0]);
    let r3 = graph.connect("C", "D").unwrap();
    let r4 = graph.connect("D", "E").unwrap();
    // Java compareNodes: findNodesByType("data") = {B, C, D, E}
    let data_nodes = graph.find_nodes_by_type("data").unwrap();
    assert_eq!(4, data_nodes.len());
    for node in [&node_b, &node_c, &node_d, &node_e] {
        assert!(data_nodes.iter().any(|n| n == node));
    }
    let transaction_nodes = graph.find_nodes_by_type("transaction").unwrap();
    assert_eq!(1, transaction_nodes.len());
    assert!(transaction_nodes.contains(&node_a));
    assert_eq!(r1, graph.find_connection("A", "B").unwrap().unwrap());
    assert_eq!(r2, graph.find_connection("A", "C").unwrap().unwrap());
    assert_eq!(r3, graph.find_connection("C", "D").unwrap().unwrap());
    assert_eq!(r4, graph.find_connection("D", "E").unwrap().unwrap());
    // mutate node A through one handle, observe through an id lookup
    node_a.add_type("hello").unwrap();
    node_a.add_type("service").unwrap();
    node_a.add_property("hello", Value::from("world")).unwrap();
    node_a.add_property("test", Value::from("message")).unwrap();
    let node_aa = graph.find_node_by_id(node_a.get_id()).unwrap().unwrap();
    assert_eq!(Some(Value::from("world")), node_aa.get_property("hello"));
    assert_eq!(Some(Value::from("message")), node_aa.get_property("test"));
    assert!(node_aa.get_types().contains("hello"));
    assert!(node_aa.get_types().contains("service"));
    node_a.remove_type("hello").unwrap();
    node_a.remove_property("hello").unwrap();
    assert_eq!(None, node_aa.get_property("hello"));
    assert!(!node_aa.get_types().contains("hello"));
    // neighbors of A = {root, B, C}
    let neighbors = graph.get_neighbors("A").unwrap();
    let neighbor_aliases: HashSet<String> = neighbors
        .iter()
        .map(|n| n.get_alias().to_string())
        .collect();
    assert_eq!(3, neighbor_aliases.len());
    assert!(neighbor_aliases.contains("root"));
    assert!(neighbor_aliases.contains("B"));
    assert!(neighbor_aliases.contains("C"));
    // Java validatePaths1 — BFS from A: [[A], [B, root, C], [D], [E]]
    let paths_from_a = graph.find_paths("A").unwrap();
    assert_eq!(4, paths_from_a.len());
    assert_eq!(1, paths_from_a[0].len());
    assert_eq!(3, paths_from_a[1].len());
    assert_eq!(1, paths_from_a[2].len());
    assert_eq!(1, paths_from_a[3].len());
    assert_eq!("A", paths_from_a[0][0]);
    assert!(paths_from_a[1].contains(&"B".to_string()));
    assert!(paths_from_a[1].contains(&"C".to_string()));
    assert_eq!("D", paths_from_a[2][0]);
    assert_eq!("E", paths_from_a[3][0]);
    // Java validateNodesThenRemoveConnection
    let nodes = graph.get_nodes();
    assert_eq!(6, nodes.len());
    for alias in ["root", "A", "B", "C", "D", "E"] {
        let node = graph.find_node_by_alias(alias).unwrap().unwrap();
        assert!(nodes.contains(&node));
    }
    assert_eq!(5, graph.get_connections().len());
    graph.remove_connection("C", "D").unwrap();
    assert_eq!(4, graph.get_connections().len());
    // Java validatePaths2 — from A: [[A], [C, root, B]]; from E: [[E], [D]]
    let paths_from_a2 = graph.find_paths("A").unwrap();
    assert_eq!(2, paths_from_a2.len());
    assert_eq!(1, paths_from_a2[0].len());
    assert_eq!(3, paths_from_a2[1].len());
    assert_eq!("A", paths_from_a2[0][0]);
    assert!(paths_from_a2[1].contains(&"B".to_string()));
    assert!(paths_from_a2[1].contains(&"C".to_string()));
    let paths_from_e = graph.find_paths("E").unwrap();
    assert_eq!(2, paths_from_e.len());
    assert_eq!(1, paths_from_e[0].len());
    assert_eq!(1, paths_from_e[1].len());
    assert_eq!("E", paths_from_e[0][0]);
    assert_eq!("D", paths_from_e[1][0]);
    // add the connection again, then removing node D orphans node E
    graph.connect("C", "D").unwrap();
    graph.remove_node("D").unwrap();
    // Java validatePath3
    let paths_from_e2 = graph.find_paths("E").unwrap();
    assert_eq!(1, paths_from_e2.len());
    assert_eq!(1, paths_from_e2[0].len());
    assert_eq!("E", paths_from_e2[0][0]);
    // case-insensitive alias search
    let node1 = graph.find_node_by_alias("B").unwrap().unwrap();
    let node2 = graph.find_node_by_alias("b").unwrap().unwrap();
    assert_eq!(node1, node2);
    assert!(graph.find_nodes_by_type("Transaction").is_ok());
    // find node by property — node A now has exactly {test: message}
    let by_property = graph
        .find_nodes_by_property("test", &Value::from("message"))
        .unwrap();
    assert_eq!(1, by_property.len());
    let properties = by_property[0].get_properties();
    assert_eq!(1, properties.len());
    assert_eq!(Some(&Value::from("message")), properties.get("test"));
    assert_eq!(
        HashSet::from(["transaction".to_string(), "service".to_string()]),
        types_of(&by_property[0])
    );
    // Java validateEndNode
    let root_node = graph.get_root_node().unwrap();
    assert_eq!(
        HashSet::from(["root".to_string(), "hello".to_string()]),
        types_of(&root_node)
    );
    let end = graph.create_end_node().unwrap();
    assert_eq!("end", end.get_alias());
    assert_eq!(end, graph.get_end_node().unwrap());
    graph.reset();
    assert!(graph.is_empty());
}

#[test]
fn directional_test() {
    let graph = MiniGraph::new();
    assert!(graph.is_empty());
    assert_eq!(32, graph.get_id().len());
    let node_a = graph.create_node("A", "transaction").unwrap();
    let node_b = graph.create_node("B", "data").unwrap();
    assert_eq!(2, graph.get_node_count());
    let c1 = graph.connect("A", "B").unwrap();
    let r1 = c1.add_relation("demo1");
    r1.add_property("hello", Value::from("world")).unwrap();
    let c2 = graph.connect("B", "A").unwrap();
    c2.add_relation("demo2");
    let connections = graph.find_bi_directional_connection("A", "B").unwrap();
    assert_eq!(2, connections.len());
    let conn1 = &connections[0];
    let conn2 = &connections[1];
    assert_eq!(c1, *conn1);
    assert_eq!(c2, *conn2);
    assert_eq!(node_a, *conn1.get_source());
    assert_eq!(node_b, *conn1.get_target());
    assert_eq!(node_b, *conn2.get_source());
    assert_eq!(node_a, *conn2.get_target());
    let relation = conn1.get_relation("demo1").unwrap();
    assert_eq!(*r1, *relation);
    assert_eq!(Some(Value::from("world")), relation.get_property("hello"));
}

#[test]
fn import_export_test() {
    let graph1 = MiniGraph::new();
    let a1 = graph1.create_node("A", "transaction").unwrap();
    a1.add_type("service").unwrap();
    a1.add_property("hello", Value::from("world")).unwrap();
    let b1 = graph1.create_node("B", "data").unwrap();
    b1.add_type("service").unwrap();
    b1.add_property("test", Value::from("message")).unwrap();
    b1.add_property("graph", Value::from("minimalist")).unwrap();
    graph1.create_node("C", "data").unwrap();
    graph1.create_node("D", "data").unwrap();
    graph1.create_node("E", "data").unwrap();
    graph1.connect("A", "B").unwrap();
    graph1.connect("A", "C").unwrap();
    let r1 = graph1.connect("C", "D").unwrap();
    graph1.connect("D", "E").unwrap();
    r1.add_relation("Demo")
        .add_property("some", Value::from("relation"))
        .unwrap();
    let relation1 = r1.add_relation("Conversion");
    relation1.add_property("US", Value::from(1)).unwrap();
    relation1.add_property("HK", Value::from(7.8)).unwrap();
    let map1 = graph1.export_graph();
    let graph2 = MiniGraph::new();
    graph2.import_graph(&map1).unwrap();
    let map2 = graph2.export_graph();
    assert_eq!(map1, map2);
    assert!(graph1.same_as(&graph2));
    assert!(graph2.same_as(&graph1));
    assert_eq!(graph1.to_json(), graph2.to_json());
    // export/import round-trips through JSON too (the graph file format)
    let json: serde_json::Value = serde_json::from_str(&graph1.to_json()).unwrap();
    let value = rmpv::ext::to_value(&json).unwrap();
    let graph3 = MiniGraph::new();
    graph3.import_graph(&value).unwrap();
    assert!(graph1.same_as(&graph3));
}

#[test]
fn exception_test_1() {
    let graph = MiniGraph::new();
    let ex1 = graph.create_node("", "transaction").unwrap_err();
    assert_eq!("alias must not be empty", ex1.message());
    let ex3 = graph.create_node("test", "").unwrap_err();
    assert_eq!("type must not be empty", ex3.message());
    graph.create_node("test", "transaction").unwrap();
    let ex5 = graph.create_node("test", "hello").unwrap_err();
    assert_eq!("alias 'test' already exists", ex5.message());
    let ex6 = graph.remove_node("").unwrap_err();
    assert_eq!("alias must not be empty", ex6.message());
    let ex8 = graph.create_node("input", "data").unwrap_err();
    assert_eq!("alias 'input' is a reserved name", ex8.message());
}

#[test]
fn exception_test_2() {
    let graph = MiniGraph::new();
    let ex1 = graph.remove_connection("", "B").unwrap_err();
    assert_eq!("source alias cannot be null", ex1.message());
    let ex2 = graph.remove_connection("not found", "X").unwrap_err();
    assert_eq!("source node does not exist", ex2.message());
    let ex3 = graph.remove_connection("test", "").unwrap_err();
    assert_eq!("target alias cannot be null", ex3.message());
    let ex4 = graph.remove_connection("A", "A").unwrap_err();
    assert_eq!(
        "source and target aliases cannot be the same",
        ex4.message()
    );
    graph.create_node("test", "transaction").unwrap();
    let ex5 = graph.remove_connection("test", "hello").unwrap_err();
    assert_eq!("target node does not exist", ex5.message());
}

#[test]
fn exception_test_3() {
    let graph = MiniGraph::new();
    let ex1 = graph.find_node_by_alias("").unwrap_err();
    assert_eq!("alias cannot be null", ex1.message());
    let ex2 = graph.find_node_by_id("").unwrap_err();
    assert_eq!("id cannot be null", ex2.message());
    let ex3 = graph.find_nodes_by_type("").unwrap_err();
    assert_eq!("type cannot be empty", ex3.message());
    let ex4 = graph
        .find_nodes_by_property("", &Value::from("test"))
        .unwrap_err();
    assert_eq!("key cannot be empty", ex4.message());
}

#[test]
fn exception_test_4() {
    let graph = MiniGraph::new();
    let ex1 = graph.connect("", "B").unwrap_err();
    assert_eq!("source alias cannot be null", ex1.message());
    let ex2 = graph.connect("not found", "X").unwrap_err();
    assert_eq!("source node does not exist", ex2.message());
    let ex3 = graph.connect("test", "").unwrap_err();
    assert_eq!("target alias cannot be null", ex3.message());
    let ex4 = graph.connect("A", "A").unwrap_err();
    assert_eq!(
        "source and target aliases cannot be the same",
        ex4.message()
    );
    graph.create_node("test", "transaction").unwrap();
    let ex5 = graph.connect("test", "hello").unwrap_err();
    assert_eq!("target node does not exist", ex5.message());
}

#[test]
fn exception_test_5() {
    let graph = MiniGraph::new();
    let ex1 = graph.find_connection("", "B").unwrap_err();
    assert_eq!("source alias cannot be null", ex1.message());
    let ex2 = graph.find_connection("not found", "X").unwrap_err();
    assert_eq!("source node does not exist", ex2.message());
    let ex3 = graph.find_connection("test", "").unwrap_err();
    assert_eq!("target alias cannot be null", ex3.message());
    let ex4 = graph.find_connection("A", "A").unwrap_err();
    assert_eq!(
        "source and target aliases cannot be the same",
        ex4.message()
    );
    graph.create_node("test", "transaction").unwrap();
    let ex5 = graph.find_connection("test", "hello").unwrap_err();
    assert_eq!("target node does not exist", ex5.message());
    let ex6 = graph.get_neighbors("").unwrap_err();
    assert_eq!("alias cannot be null", ex6.message());
    let ex7 = graph.get_forward_links("").unwrap_err();
    assert_eq!("alias cannot be null", ex7.message());
    let ex8 = graph.get_backward_links("").unwrap_err();
    assert_eq!("alias cannot be null", ex8.message());
    let ex9 = graph.get_neighbors("not found").unwrap_err();
    assert_eq!("node does not exist", ex9.message());
    let ex10 = graph.get_forward_links("not found").unwrap_err();
    assert_eq!("node does not exist", ex10.message());
    let ex11 = graph.get_backward_links("not found").unwrap_err();
    assert_eq!("node does not exist", ex11.message());
    let ex12 = graph.find_paths("not found").unwrap_err();
    assert_eq!("node does not exist", ex12.message());
}

#[test]
fn exception_test_6() {
    let graph = MiniGraph::new();
    let node = graph.create_node("hello", "world").unwrap();
    let ex1 = node.add_type("hello.world").unwrap_err();
    assert_eq!(
        "Invalid syntax (hello.world). Please use 0-9, A-Z, a-z, underscore and hyphen characters.",
        ex1.message()
    );
    let ex2 = node
        .add_property("my.key", Value::from("someValue"))
        .unwrap_err();
    assert_eq!(
        "Invalid syntax (my.key). Please use 0-9, A-Z, a-z, underscore and hyphen characters.",
        ex2.message()
    );
    // invalid alias at creation
    let ex3 = graph.create_node("bad.alias", "data").unwrap_err();
    assert_eq!(
        "Invalid syntax (bad.alias). Please use 0-9, A-Z, a-z, underscore and hyphen characters.",
        ex3.message()
    );
}

#[test]
fn max_nodes_and_import_failure_test() {
    // max-nodes cap
    let small = MiniGraph::with_max_nodes(2);
    small.create_node("one", "data").unwrap();
    small.create_node("two", "data").unwrap();
    small.create_node("three", "data").unwrap();
    let ex = small.create_node("four", "data").unwrap_err();
    assert_eq!("max number of nodes is 2", ex.message());
    // reset clears everything
    small.reset();
    assert!(small.is_empty());
    assert!(small.get_nodes().is_empty());
    assert!(small.get_connections().is_empty());
    // import failure resets the graph (Java parity)
    let graph = MiniGraph::new();
    let bad = rmpv::ext::to_value(serde_json::json!({
        "nodes": [{"types": ["data"], "properties": {}}]
    }))
    .unwrap();
    let ex1 = graph.import_graph(&bad).unwrap_err();
    assert_eq!("missing alias in node entry-1", ex1.message());
    assert!(graph.is_empty());
    let bad2 = rmpv::ext::to_value(serde_json::json!({
        "nodes": [{"alias": "A", "types": [], "properties": {}}]
    }))
    .unwrap();
    let ex2 = graph.import_graph(&bad2).unwrap_err();
    assert_eq!("invalid types in node entry-1", ex2.message());
    let bad3 = rmpv::ext::to_value(serde_json::json!({
        "nodes": [{"alias": "A", "types": ["data"], "properties": {}}],
        "connections": [{"relations": []}]
    }))
    .unwrap();
    let ex3 = graph.import_graph(&bad3).unwrap_err();
    assert_eq!(
        "invalid source/target alias in connection entry-1",
        ex3.message()
    );
    assert!(graph.is_empty());
}
