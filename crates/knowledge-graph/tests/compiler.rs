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

//! Parity port of the Java `CompileGraphTest`, against the canonical
//! fixtures copied verbatim: the 13 tutorial graphs travel with the engine
//! crate (`resources/graph/`), the 13 test-only graphs and the `graphs.yaml`
//! manifest mirror the Java `src/test/resources` (`tests/resources/`).

use std::sync::Once;

use event_script::mlm::MultiLevelMap;
use knowledge_graph::{compiler, graphs};
use platform_core::resources;
use rmpv::Value;

fn compile_once() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        // test resources shadow the engine's own resources (Java classpath
        // order); the engine root is the default `resources` under this crate
        resources::prepend_resource_root("tests/resources");
        compiler::compile_graphs();
    });
}

#[test]
fn manifest_listed_graphs_are_compiled() {
    compile_once();
    assert!(graphs::graph_exists("hellojs"));
    assert!(graphs::graph_exists("tutorial-1"));
    // a graph id that is not listed in graphs.yaml must not be compiled -
    // the graph executor falls back to lazy loading for it
    assert!(!graphs::graph_exists("tutorial-99"));
    // every graph in the manifest is valid: 13 tutorials + 15 test fixtures
    let mut all = graphs::get_all_graphs();
    all.sort();
    assert_eq!(
        28,
        all.len(),
        "expected all manifest graphs to compile: {all:?}"
    );
}

#[test]
fn deprecated_type_matching_syntax_is_converted_at_compile_time() {
    compile_once();
    let model = graphs::get_graph("hellojs").expect("hellojs should be compiled");
    let mm = MultiLevelMap::from_value((*model).clone());
    let Some(Value::Array(nodes)) = mm.get_element("nodes") else {
        panic!("hellojs model must have a node list");
    };
    let mut found = false;
    for i in 0..nodes.len() {
        if let Some(Value::Array(entries)) =
            mm.get_element(&format!("nodes[{i}].properties.mapping"))
        {
            for entry in entries {
                let line = event_script::conversions::display(&entry);
                // the deprecated colon syntax must be gone
                assert!(
                    !line.contains("model.number:int"),
                    "colon syntax should be converted: {line}"
                );
                if line == "f:int(model.number) -> hello.xyz" {
                    found = true;
                }
            }
        }
    }
    assert!(found, "expected converted mapping entry not found");
}

#[test]
fn config_references_are_resolved_at_compile_time() {
    compile_once();
    // hellojs carries "int(${rest.server.port}) -> hello.port"; the loader
    // resolves the reference against the app config (Java ConfigReader parity)
    let model = graphs::get_graph("hellojs").expect("hellojs should be compiled");
    let text = event_script::conversions::to_json_string(&model);
    assert!(
        text.contains("int(8090) -> hello.port"),
        "expected resolved port in model"
    );
    assert!(
        !text.contains("${rest.server.port}"),
        "unresolved reference"
    );
}
