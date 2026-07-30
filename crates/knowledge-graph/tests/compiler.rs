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
//! crate (`resources/graph/`), the test-only graphs and the `graphs.yaml`
//! manifest mirror the Java `src/test/resources` (`tests/resources/`).

use std::sync::Once;

use event_script::mlm::MultiLevelMap;
use knowledge_graph::{compiler, graphs, model_validator};
use platform_core::graph::MiniGraph;
use platform_core::resources;
use platform_core::{ConfigReader, ConfigValue};
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
    // a graph id that is not listed in graphs.yaml is not compiled and
    // therefore not executable - the graph executor answers 404 for it
    assert!(!graphs::graph_exists("tutorial-99"));
    // the discovery contract is enforced at compile: a manifest graph whose
    // root node has no 'purpose' is rejected (rust-no-purpose fixture)
    assert!(!graphs::graph_exists("rust-no-purpose"));
    // 13 tutorials + 21 original fixtures + the 5 valid suspend fixtures;
    // the 8 deliberately-invalid fixtures (err1-7 + no-end) are rejected by
    // the mandatory quality gate. Every graph a runtime test executes MUST
    // be listed here - deployed execution is compiled-or-404 (no lazy load)
    let mut all = graphs::get_all_graphs();
    all.sort();
    assert_eq!(
        39,
        all.len(),
        "expected all valid manifest graphs to compile: {all:?}"
    );
}

#[test]
fn valid_suspend_resume_graphs_are_compiled() {
    compile_once();
    assert!(graphs::graph_exists("unit-test-suspend-1"));
    assert!(graphs::graph_exists("unit-test-suspend-2"));
    assert!(graphs::graph_exists("unit-test-suspend-3"));
    assert!(graphs::graph_exists("unit-test-suspend-4"));
    assert!(graphs::graph_exists("unit-test-suspend-5"));
}

#[test]
fn invalid_manifest_graphs_are_not_compiled() {
    compile_once();
    // every deliberately invalid manifest graph must fail the gate; deployed
    // execution is served exclusively from the compiled registry, so a rejected
    // graph answers 404 as if it does not exist (CompileFlows parity) -
    // err1-err7 break the suspend/resume contract, unit-test-no-end has no
    // 'end' node (a run could never complete)
    for id in [
        "unit-test-suspend-err1",
        "unit-test-suspend-err2",
        "unit-test-suspend-err3",
        "unit-test-suspend-err4",
        "unit-test-suspend-err5",
        "unit-test-suspend-err6",
        "unit-test-suspend-err7",
        "unit-test-no-end",
    ] {
        assert!(
            !graphs::graph_exists(id),
            "{id} must be rejected by the quality gate"
        );
    }
}

#[test]
fn manifest_location_defaults_to_classpath_graph() {
    compile_once();
    // the engine's test manifest declares no 'location' - the CompileFlows-style
    // default applies (the playground example app's manifest sets it explicitly)
    assert_eq!("classpath:/graph", graphs::deployed_location());
}

#[test]
fn static_validator_rejects_every_invalid_suspend_resume_shape() {
    compile_once();
    // direct coverage of every static rule, independent of the manifest:
    // err1 graph.suspend node not named 'suspend'; err2 suspend=true on graph.math;
    // err3 suspensible node without a suspend node; err4 suspend node without ttl;
    // err5 suspensible node without a drawn edge to 'suspend'; err6 suspend node
    // without an outgoing connection; err7 suspension point without a
    // continuation edge (a resumed run could not continue)
    for id in [
        "unit-test-suspend-err1",
        "unit-test-suspend-err2",
        "unit-test-suspend-err3",
        "unit-test-suspend-err4",
        "unit-test-suspend-err5",
        "unit-test-suspend-err6",
        "unit-test-suspend-err7",
    ] {
        let reader =
            ConfigReader::load(&format!("classpath:/graph/{id}.json")).expect("fixture loads");
        let json = ConfigValue::Map(reader.get_map().clone().into_map()).to_json();
        let model = event_script::conversions::from_json(&json);
        let graph = MiniGraph::new();
        graph.import_graph(&model).expect("fixture is importable");
        assert!(
            model_validator::validate_suspend_resume(&graph).is_err(),
            "{id} must fail the static validator"
        );
    }
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

/// Java parity (`GraphSuspendResumeTest.helpFilesFollowNamingConvention`):
/// the hyphenated help pages for the two new skills must exist — the
/// Playground's `describe skill {route}` / `help graph-suspend` topics
/// resolve route names to these filenames at runtime, so a rename or
/// packaging regression would silently break the help topic.
#[test]
fn help_files_follow_naming_convention() {
    for file in ["help/help graph-suspend.md", "help/help graph-resume.md"] {
        assert!(
            platform_core::resources::resolve_classpath(file).is_some(),
            "{file} must ship with the engine resources"
        );
    }
}
