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

//! RFC-0010 WP3: a JSONPath source is checked against RFC 9535 when a flow or
//! a graph is compiled. `jsonpath-source-vectors.json` is byte-identical with
//! the Java engine's (Java `JsonPathSourceTest`).

use event_script::conversions::from_json;
use event_script::jsonpath_source::{check, references, refuse_json_path_key, substitute, Form};
use event_script::mapping::split_top_level_arguments;
use event_script::mlm::compile_json_path;
use serde_json::Value as Json;

const VECTORS: &str = include_str!("resources/jsonpath-source-vectors.json");

fn section(name: &str) -> Vec<Json> {
    let vectors: Json = serde_json::from_str(VECTORS).expect("the vector file is JSON");
    vectors[name].as_array().expect("a section").clone()
}

fn form(name: &str) -> Form {
    match name {
        "event-script" => Form::EventScript,
        "graph" => Form::Graph,
        "plain" => Form::Plain,
        other => panic!("unknown form {other}"),
    }
}

#[test]
fn sources_are_checked_where_each_reference_lands() {
    let vectors = section("sources");
    assert_eq!(vectors.len(), 35, "source vector count");
    for v in vectors {
        let source = v["source"].as_str().expect("source");
        let form = form(v["form"].as_str().expect("form"));
        match v.get("sites") {
            Some(Json::Array(expected)) => {
                let sites: Vec<String> = references(source, form)
                    .unwrap_or_else(|e| panic!("{source}: {e}"))
                    .iter()
                    .map(|r| r.site.name().to_string())
                    .collect();
                let expected: Vec<String> = expected
                    .iter()
                    .map(|s| s.as_str().expect("a site").to_string())
                    .collect();
                assert_eq!(sites, expected, "{source}: where each reference lands");
            }
            _ => assert!(
                references(source, form).is_err(),
                "{source}: no place for a reference"
            ),
        }
        let valid = v["valid"].as_bool().expect("valid");
        assert_eq!(
            check(source, form).is_ok(),
            valid,
            "{source}: {:?}",
            check(source, form)
        );
    }
}

#[test]
fn plugin_arguments_split_outside_a_json_paths_brackets_and_literals() {
    let vectors = section("plugin_arguments");
    assert_eq!(vectors.len(), 11, "plugin argument vector count");
    for v in vectors {
        let arguments = v["arguments"].as_str().expect("arguments");
        let split: Vec<String> = split_top_level_arguments(arguments)
            .iter()
            .map(|a| a.trim().to_string())
            .collect();
        let expected: Vec<String> = v["split"]
            .as_array()
            .expect("split")
            .iter()
            .map(|s| s.as_str().expect("an argument").to_string())
            .collect();
        assert_eq!(split, expected, "f:x({arguments})");
    }
}

#[test]
fn a_reference_key_is_never_a_json_path() {
    let vectors = section("reference_keys");
    assert_eq!(vectors.len(), 5, "reference key vector count");
    for v in vectors {
        let key = v["key"].as_str().expect("key");
        if v["refused"].as_bool().expect("refused") {
            assert_eq!(
                refuse_json_path_key(key).unwrap_err(),
                format!("JSONPath is not supported in a reference - {{{key}}}; map the value to a model variable first")
            );
        } else {
            assert!(refuse_json_path_key(key).is_ok(), "{{{key}}}");
        }
    }
}

#[test]
fn substitutions_are_checked_where_each_value_lands() {
    let vectors = section("substitutions");
    assert_eq!(vectors.len(), 39, "substitution vector count");
    for v in vectors {
        let source = v["source"].as_str().expect("source");
        let form = form(v["form"].as_str().expect("form"));
        let values = v["values"].as_object().expect("values").clone();
        let lookup = |key: &str| Ok(values.get(key).map(from_json));
        let result = substitute(source, form, &lookup);
        match v.get("error") {
            Some(expected) => assert_eq!(
                result.expect_err(source),
                expected.as_str().expect("an error"),
                "{source} {values:?}"
            ),
            None => {
                let query = result.unwrap_or_else(|e| panic!("{source} {values:?}: {e}"));
                assert_eq!(
                    query,
                    v["query"].as_str().expect("a query"),
                    "{source} {values:?}"
                );
                assert!(
                    compile_json_path(&query).is_ok(),
                    "the query compiles: {query}"
                );
            }
        }
    }
}
