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
    // 13 tutorials + 21 original fixtures + the 7 valid suspend fixtures
    // (incl. the jump-mode and retired-property compat shapes) + the 3 valid
    // ttl fixtures (node-ttl ok + the 2 x-ttl wire echoes) + the generic
    // exception-context fixture + the orchestrator pair + the dynamic-jump
    // fixture (THEN:/DELAY: dynamic variables); the 14
    // deliberately-invalid fixtures
    // (suspend err1-7, no-end, ttl err1-4, task-6, error-alias) are rejected
    // by the mandatory quality gate. Every graph a runtime test executes MUST
    // be listed here - deployed execution is compiled-or-404 (no lazy load)
    let mut all = graphs::get_all_graphs();
    all.sort();
    assert_eq!(
        48,
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
    // a graph.task node may declare a child-call deadline (ttl in the
    // suspend grammar) - the gate accepts it
    assert!(graphs::graph_exists("unit-test-ttl-ok"));
    // the generic exception-context fixture and the orchestrator pair pass
    // the gate (the subgraph is a complete resumable workflow on its own)
    assert!(graphs::graph_exists("unit-test-error-context"));
    assert!(graphs::graph_exists("unit-test-orchestrator"));
    assert!(graphs::graph_exists("unit-test-sub-suspend"));
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
        // ttl placement/grammar + model-metadata immutability (Java parity):
        // err1 ttl on a skill without deadline semantics (graph.math);
        // err2 malformed duration on a deadline skill; err3 a data mapping
        // writing to reserved model metadata (model.ttl); err4 the same
        // write embedded as a MAPPING: line in a graph.math statement
        "unit-test-ttl-err1",
        "unit-test-ttl-err2",
        "unit-test-ttl-err3",
        "unit-test-ttl-err4",
        // a graph.task input[] entry writing to reserved model metadata
        // (model.ttl) - the model-staging RHS is gate-checked like every
        // other model-writing path
        "unit-test-task-6",
        // a node aliased 'error' shadows the exception-context namespace
        // (error.source/code/message/stack) - the graph model itself rejects
        // reserved aliases at node creation, so the gate rejection is inherited
        "unit-test-error-alias",
    ] {
        assert!(
            !graphs::graph_exists(id),
            "{id} must be rejected by the quality gate"
        );
    }
}

/// Java parity (CompileGraphTest.nodeTtlPlacementAndMetadataImmutabilityAreValidated):
/// direct coverage of the composite validate() rules, independent of the manifest.
#[test]
fn node_ttl_placement_and_metadata_immutability_are_validated() {
    compile_once();
    let import = |id: &str| {
        let reader =
            ConfigReader::load(&format!("classpath:/graph/{id}.json")).expect("fixture loads");
        let json = ConfigValue::Map(reader.get_map().clone().into_map()).to_json();
        let model = event_script::conversions::from_json(&json);
        let graph = MiniGraph::new();
        graph.import_graph(&model).expect("fixture is importable");
        graph
    };
    // valid: a graph.task node may declare a child-call deadline
    assert!(model_validator::validate(&import("unit-test-ttl-ok")).is_ok());
    for id in [
        "unit-test-ttl-err1",
        "unit-test-ttl-err2",
        "unit-test-ttl-err3",
        "unit-test-ttl-err4",
        // graph.task input[] staging into reserved model metadata (model.ttl)
        "unit-test-task-6",
    ] {
        assert!(
            model_validator::validate(&import(id)).is_err(),
            "{id} must fail the static validator"
        );
    }
    // the three-skill rejection message is this engine's deliberate divergence
    // from Java's four (no graph.js in the Rust port)
    let err = model_validator::validate(&import("unit-test-ttl-err1")).unwrap_err();
    assert!(
        err.contains("graph.extension, graph.api.fetcher or graph.task"),
        "unexpected message: {err}"
    );
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
    // err1 graph.suspend node not named 'suspend'; err2 graph.math with a drawn
    // edge to 'suspend' (the retired suspend=true also present - a WARN only);
    // err3 exception=suspend (the suspend node cannot be an exception handler);
    // err4 suspend node without ttl; err5 graph.math with a drawn edge to
    // 'suspend' and no retired property; err6 suspend node without an outgoing
    // connection; err7 suspension point without a continuation edge (a resumed
    // run could not continue)
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
