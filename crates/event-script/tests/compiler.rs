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

//! Increment E-1 parity tests: the compiler runs against the **canonical Java
//! fixtures copied verbatim** (see tests/resources/README.md) and must reach
//! the same outcomes — same flows loaded, same flows rejected, same tasks
//! dropped, same normalized mapping strings.
//!
//! The compiler is pure config→model (no bus, no runtime), so the tests are
//! synchronous; compilation runs once (flow ids are globally unique).

use std::sync::Once;

use event_script::{compiler, flows, model::Flow};
use platform_core::resources;

fn compile_once() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        compiler::compile_flows();
    });
}

fn flow(id: &str) -> std::sync::Arc<Flow> {
    flows::get_flow(id).unwrap_or_else(|| panic!("flow '{id}' should have loaded"))
}

/// The exact set of flows the Java engine loads from these fixtures:
/// all 55 flows listed in flows.yaml, plus the parser fixtures that
/// legitimately compile (some with their offending task dropped).
#[test]
fn loaded_flow_set_matches_the_java_engine() {
    compile_once();
    let expected = vec![
        // from flows.yaml (55)
        "arithmetic",
        "autostart",
        "body-test",
        "child-one",
        "child-three",
        "child-two",
        "daughter-greetings",
        "decision-test",
        "decision-with-no-op-test",
        "delayed-response-test",
        "echo-flow",
        "ext-state-machine",
        "externalize-get-key-value",
        "externalize-put-key-value",
        "externalize-put-key-value-flow",
        "file-vault-test",
        "for-loop-break",
        "for-loop-break-single-task",
        "for-loop-continue",
        "for-loop-test",
        "for-loop-test-single-task",
        "fork-n-join-flows",
        "fork-n-join-test",
        "fork-n-join-with-dynamic-model-test",
        "greetings",
        "header-and-json-path-test",
        "http-client-by-config",
        "http-client-sink-trace",
        "input-validation-1",
        "input-validation-2",
        "input-validation-3",
        "input-validation-4",
        "input-validation-5",
        "input-validation-6",
        // grammar-valid despite its legacy id: 'if (model.jump) break' is a
        // correct loop condition, so the Java engine loads it too
        "invalid-condition-mode",
        "missing-sub-flow",
        "null-transport",
        "numeric-decision-test",
        "parallel-test",
        "parent-greetings",
        "parse-date",
        "parse-date-time",
        "pipeline-exception",
        "pipeline-test",
        "resilience-demo",
        "response-test",
        "sequential-test",
        "simple-circuit-breaker",
        "string-util",
        "timeout-test",
        "type-conversion",
        "type-matching",
        "while-loop",
        "while-loop-break",
        "while-loop-continue",
        "wildcard-conversion-test",
        // from more-flows.yaml: the parser fixtures that compile.
        // dynamic-reserved-key: the reserved-key RHS is dynamic, checked at runtime
        "dynamic-reserved-key",
        // 23/25/26/27: an invalid mapping drops the TASK, the flow still loads
        "parser-test-23",
        "parser-test-25",
        "parser-test-26",
        "parser-test-27",
        // 28-31: reserved-key targets drop the task, the flow still loads
        "parser-test-28",
        "parser-test-29",
        "parser-test-30",
        "parser-test-31",
        // the 'ext.user' body key does not use the 'ext:' namespace, so no
        // external.state.machine is required (the Java code, not the fixture
        // comment, is authoritative)
        "parser-test-missing-external-state-machine",
    ];
    let mut expected: Vec<&str> = expected;
    expected.sort_unstable();
    assert_eq!(flows::get_all_flows(), expected);
}

/// Every rejected parser fixture must be absent (whole-flow failures).
#[test]
fn invalid_flows_are_rejected() {
    compile_once();
    for id in [
        "parser-test-no-description", // task description missing
        "missing-tasks",              // tasks: []
        "missing-join-task",          // fork without join
        "next-task-not-singular",     // sequential with 2 next
        "missing-pipeline-steps",     // pipeline: []
        "missing-next-steps",         // non-sink without next
        "invalid-for-statement",
        "invalid-for-statement-1",
        "invalid-for-statement-2",
        "invalid-for-statement-3",
        "invalid-for-statement-4",
        "invalid-while-statement",
        "invalid-while-statement-5",
        "invalid-loop-statement",
        "invalid-loop-statement-7",
        "parser-test-incomplete-mappings", // 'user.' RHS fails at finalize
        "parser-test-20",                  // process tag on a flow:// task
        "parser-test-21",                  // task without description
        "parser-test-22",                  // http:// is not flow://
        "parser-test-24",                  // flow-level description commented out
        "sample.yml",                      // present on disk but not listed
        "sample",
    ] {
        assert!(!flows::flow_exists(id), "'{id}' must not load");
    }
    // duplicate flow.id: parser-test-1 re-declares 'greetings' and must lose —
    // the flows.yaml version (first.task 'my.greeting.task') stays
    assert_eq!(flow("greetings").first_task, "my.greeting.task");
}

/// Mapping normalization matches the Java compiler exactly: 3-part entries
/// decompose, `!model.x` becomes `model.x:!`, and legacy `:type` qualifiers
/// are rewritten to `f:plugin(...)` calls.
#[test]
fn greetings_mappings_normalize_identically() {
    compile_once();
    let greetings = flow("greetings");
    assert_eq!(greetings.ttl, 10_000);
    assert_eq!(greetings.exception.as_deref(), Some("v1.hello.exception"));
    assert_eq!(greetings.tasks.len(), 2);
    let task = &greetings.tasks["my.greeting.task"];
    assert_eq!(task.function_route, "greeting.test");
    assert_eq!(task.execution, "end");
    let input = &task.input;
    // 3-part with type qualifier decomposes into a plugin call + a copy
    assert!(input.contains(&"f:int(input.path_parameter.user) -> model.user".to_string()));
    assert!(input.contains(&"model.user -> user_number".to_string()));
    // negation forms
    assert!(input.contains(&"f:not(boolean(true)) -> model.bool".to_string()));
    assert!(input.contains(&"model.bool -> negate_value".to_string()));
    assert!(input.contains(&"f:not(model.bool) -> double_negate_value".to_string()));
    // boolean null-matching and uuid generation
    assert!(input.contains(&"f:isNull(model.none) -> none_is_true".to_string()));
    assert!(input.contains(&"f:notNull(model.none) -> none_is_false".to_string()));
    assert!(input.contains(&"f:uuid(model.none) -> unique_id1".to_string()));
    assert!(input.contains(&"f:uuid(model.none) -> model.uuid".to_string()));
    // concat with mixed arguments (text(,) survives intact)
    assert!(input.contains(
        &"f:concat(model.a, model.space, model.b, text(,), model.c) -> concat_string".to_string()
    ));
    // dynamic model key passes through unchanged
    assert!(input.contains(&"model.{model.pointer} -> runtime_value1".to_string()));
    // output: 3-part negation into the response body
    assert!(task
        .output
        .contains(&"f:not(model.bool) -> model.bool".to_string()));
    assert!(task
        .output
        .contains(&"model.bool -> output.body.positive".to_string()));
    assert!(task
        .output
        .contains(&"model.cid -> output.body.cid".to_string()));
}

/// Java parity (FlowTests.reservedModelKeysAreProtectedAtCompileTime): a
/// mapping that overwrites reserved state-machine metadata drops the task at
/// compile time while the flow still loads.
#[test]
fn reserved_model_keys_drop_the_offending_task() {
    compile_once();
    for id in [
        "parser-test-28",
        "parser-test-29",
        "parser-test-30",
        "parser-test-31",
    ] {
        let violation = flow(id);
        assert!(
            !violation.tasks.contains_key("greeting.test"),
            "{id}: the task writing a reserved key must be dropped"
        );
    }
    // positive control
    assert!(!flow("greetings").tasks.is_empty());
}

/// Other invalid mappings drop the task the same way (23: bad map() key-values;
/// 25: output namespace as input source; 27: whole-model access).
#[test]
fn invalid_mappings_drop_the_task_but_keep_the_flow() {
    compile_once();
    assert!(!flow("parser-test-23").tasks.contains_key("greeting.test"));
    assert!(!flow("parser-test-25")
        .tasks
        .contains_key("greeting.test.alias"));
    assert!(!flow("parser-test-27")
        .tasks
        .contains_key("greeting.test.alias"));
    // 26: 'output.something' as an output LHS drops the task (its other
    // runtime-invalid values — bad status, text into header map — are
    // compile-valid and would be rejected at runtime)
    assert!(!flow("parser-test-26")
        .tasks
        .contains_key("greeting.test.alias"));
}

#[test]
fn loop_metadata_compiles_like_java() {
    compile_once();
    // for (model.n = 0; model.n < 4; model.n++) style fixtures
    let for_loop = flow("for-loop-test");
    let pipeline_task = for_loop
        .tasks
        .values()
        .find(|t| t.execution == "pipeline")
        .expect("for-loop-test has a pipeline task");
    assert_eq!(pipeline_task.loop_type, "for");
    assert_eq!(pipeline_task.init.len(), 2);
    assert_eq!(pipeline_task.comparator.len(), 3);
    assert_eq!(pipeline_task.sequencer[1], "++");
    assert!(!pipeline_task.pipeline_steps.is_empty());
    // while (model.key) style fixtures
    let while_loop = flow("while-loop");
    let while_task = while_loop
        .tasks
        .values()
        .find(|t| t.execution == "pipeline")
        .expect("while-loop has a pipeline task");
    assert_eq!(while_task.loop_type, "while");
    assert!(while_task
        .while_model_key
        .as_deref()
        .is_some_and(|k| k.starts_with("model.")));
    // break/continue conditions compile to [model.key, action] pairs
    let break_loop = flow("for-loop-break");
    let break_task = break_loop
        .tasks
        .values()
        .find(|t| !t.conditions.is_empty())
        .expect("for-loop-break has a loop condition");
    assert_eq!(break_task.conditions[0].len(), 2);
    assert!(break_task.conditions[0][0].starts_with("model."));
    assert!(["break", "continue"].contains(&break_task.conditions[0][1].as_str()));
}

#[test]
fn fork_join_and_dynamic_source_compile_like_java() {
    compile_once();
    let fork_flow = flow("fork-n-join-test");
    let fork_task = fork_flow
        .tasks
        .values()
        .find(|t| t.execution == "fork")
        .expect("fork-n-join-test has a fork task");
    assert!(fork_task.join_task.is_some());
    assert!(fork_task.next_steps.len() >= 2);
    // dynamic fork iterates a model list source
    let dynamic = flow("fork-n-join-with-dynamic-model-test");
    let dynamic_fork = dynamic
        .tasks
        .values()
        .find(|t| t.execution == "fork")
        .expect("dynamic fixture has a fork task");
    assert!(dynamic_fork
        .source_model_key
        .as_deref()
        .is_some_and(|s| s.starts_with("model.")));
}

#[test]
fn sub_flow_references_compile() {
    compile_once();
    // parent-greetings delegates to flow://daughter-greetings
    let parent = flow("parent-greetings");
    assert!(
        parent
            .tasks
            .values()
            .any(|t| t.function_route.starts_with("flow://")),
        "parent-greetings should reference a sub-flow"
    );
    // missing-sub-flow compiles (the dangling flow:// target fails at runtime)
    assert!(flows::flow_exists("missing-sub-flow"));
}
