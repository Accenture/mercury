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

//! Increment E-2 integration: the mappings the compiler produced from the
//! **canonical greetings fixture** (increment E-1) are evaluated through the
//! data-mapping engine against a simulated HTTP dataset, and the resulting
//! function-input body must match what the Java engine feeds `greeting.test`.
//!
//! The apply-loop here is a slim test double for the E-4 task executor:
//! substitute runtime vars, resolve the LHS (constant or source), route the
//! RHS (bare key → function input body; `header.*` → input headers;
//! `model.*` → the state machine so later mappings observe it).

use std::sync::Once;

use event_script::mapping::{get_constant_value, get_lhs_element, substitute_runtime_vars};
use event_script::mlm::MultiLevelMap;
use event_script::{compiler, flows};
use platform_core::resources;
use rmpv::Value;

fn compile_once() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        compiler::compile_flows();
    });
}

/// Evaluate one normalized input-mapping line (the E-4 executor in miniature).
fn apply_input_mapping(
    line: &str,
    source: &mut MultiLevelMap,
    body: &mut MultiLevelMap,
    headers: &mut MultiLevelMap,
) {
    let substituted = substitute_runtime_vars(line, source);
    let sep = substituted.rfind(" -> ").expect("normalized mapping");
    let lhs = substituted[..sep].trim();
    let rhs = substituted[sep + 4..].trim();
    let input_like = lhs.starts_with("input.")
        || lhs.eq_ignore_ascii_case("input")
        || lhs.starts_with("model.")
        || lhs.starts_with("error.")
        || lhs.starts_with("f:")
        || lhs.starts_with('$');
    let value = if input_like {
        get_lhs_element(lhs, source).expect("mapping resolution")
    } else {
        get_constant_value(lhs)
    };
    let Some(value) = value else {
        return; // Java: a null LHS clears/removes the target — not needed here
    };
    if let Some(key) = rhs.strip_prefix("header.") {
        headers.set_element(key, value).unwrap();
    } else if rhs.starts_with("model.") {
        source.set_element(rhs, value).unwrap();
    } else if rhs == "header" {
        // whole input-header map
        headers.set_element("all", value).unwrap();
    } else {
        body.set_element(rhs, value).unwrap();
    }
}

#[test]
fn greetings_input_mappings_evaluate_like_java() {
    compile_once();
    let greetings = flows::get_flow("greetings").expect("fixture compiled");
    let task = &greetings.tasks["my.greeting.task"];

    // the simulated HTTP dataset (what the flow adapter would seed)
    let mut source = MultiLevelMap::new();
    source
        .set_element("input.path_parameter.user", Value::from("12345"))
        .unwrap();
    source
        .set_element("input.header.accept", Value::from("application/json"))
        .unwrap();

    let mut body = MultiLevelMap::new();
    let mut headers = MultiLevelMap::new();
    for line in &task.input {
        apply_input_mapping(line, &mut source, &mut body, &mut headers);
    }

    // plain copy
    assert_eq!(body.get_element("user"), Some(Value::from("12345")));
    // 3-part decomposition: f:int into model.user, then copied to the body
    assert_eq!(source.get_element("model.user"), Some(Value::from(12345)));
    assert_eq!(body.get_element("user_number"), Some(Value::from(12345)));
    // constants
    assert_eq!(
        body.get_element("greeting"),
        Some(Value::from("hello world"))
    );
    assert_eq!(body.get_element("long_number"), Some(Value::from(12345)));
    assert_eq!(body.get_element("float_number"), Some(Value::F32(12.345)));
    assert_eq!(body.get_element("double_number"), Some(Value::F64(12.345)));
    assert_eq!(
        body.get_element("boolean_value"),
        Some(Value::Boolean(true))
    );
    // ${PATH} was substituted by the config layer at compile time
    let path = body.get_element("path").expect("path constant");
    assert_ne!(path, Value::from("${PATH}"));
    // negation chain: f:not(boolean(true)) -> model.bool; copy; double negate
    assert_eq!(
        source.get_element("model.bool"),
        Some(Value::Boolean(false))
    );
    assert_eq!(
        body.get_element("negate_value"),
        Some(Value::Boolean(false))
    );
    assert_eq!(
        body.get_element("double_negate_value"),
        Some(Value::Boolean(true))
    );
    // {model.pointer} runtime interpolation
    assert_eq!(
        body.get_element("runtime_value1"),
        Some(Value::from("wonderful day"))
    );
    assert_eq!(
        body.get_element("runtime_value2"),
        Some(Value::from("new world"))
    );
    assert_eq!(
        body.get_element("runtime_value3"),
        Some(Value::from("keep {this}/{ one } unchanged"))
    );
    // boolean null-matching on the model.none null constant
    assert_eq!(body.get_element("none_is_true"), Some(Value::Boolean(true)));
    assert_eq!(
        body.get_element("none_is_false"),
        Some(Value::Boolean(false))
    );
    // uuid generation: id1 fresh; id2 stored in model.uuid then copied as id3
    let id1 = body.get_element("unique_id1").expect("uuid 1");
    let id2 = body.get_element("unique_id2").expect("uuid 2");
    let id3 = body.get_element("unique_id3").expect("uuid 3");
    assert_ne!(id1, id2);
    assert_eq!(id2, id3);
    // map constant
    assert_eq!(
        source.get_element("model.map.direction"),
        Some(Value::from("right"))
    );
    // concat via the converted f:concat plugin call:
    // 'a' + ' ' + 'b' + ',' + 'c' (model.space holds a single space)
    assert_eq!(
        body.get_element("concat_string"),
        Some(Value::from("a b,c"))
    );
    // header targets
    assert_eq!(headers.get_element("user"), Some(Value::from("12345")));
    assert_eq!(headers.get_element("demo"), Some(Value::from("ok")));
}
