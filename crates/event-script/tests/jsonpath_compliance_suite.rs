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

//! The JSONPath Compliance Test Suite (cts.json, BSD-2, the commit in
//! `resources/cts/COMMIT`) against `serde_json_path`, the RFC 9535
//! implementation this engine evaluates `$` paths with. The file is
//! byte-identical with the Java engine's, whose own engine passes all 715 cases
//! (minimalist-jsonpath's `ComplianceSuiteTest`), so both engines refuse the
//! same queries and give the same answers (RFC-0010, evidence plan 1).
//!
//! One deviation is pinned, not hidden: `serde_json_path` 0.7.2 renders a
//! normalized path (RFC 9535 §2.7) without its escapes - a quote, a backslash
//! or a control character in a member name comes out as written (`$['a'']`
//! where the standard says `$['a\'']`). It affects the 16 name-selector cases
//! below and nothing in the engine, which reads values only, never normalized
//! paths (`MultiLevelMap::get_element`).

use serde_json::Value as Json;
use serde_json_path::JsonPath;

const SUITE: &str = include_str!("resources/cts/cts.json");

fn matches(actual: &[Json], expected: &Json) -> bool {
    expected.as_array().is_some_and(|e| e.as_slice() == actual)
}

#[test]
fn every_compliance_case_is_answered_as_the_standard_says() {
    let suite: Json = serde_json::from_str(SUITE).expect("cts.json is JSON");
    let cases = suite["tests"].as_array().expect("tests");
    let mut invalid = 0usize;
    let mut failures: Vec<String> = Vec::new();
    let mut unescaped: Vec<String> = Vec::new();
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let selector = case["selector"].as_str().expect("selector");
        let parsed = JsonPath::parse(selector);
        if case["invalid_selector"].as_bool() == Some(true) {
            invalid += 1;
            if parsed.is_ok() {
                failures.push(format!("{name}: accepted the invalid selector {selector}"));
            }
            continue;
        }
        let path = match parsed {
            Ok(path) => path,
            Err(e) => {
                failures.push(format!("{name}: refused {selector} - {e}"));
                continue;
            }
        };
        let located = path.query_located(&case["document"]);
        let values: Vec<Json> = located.nodes().cloned().collect();
        let paths: Vec<Json> = located
            .locations()
            .map(|p| Json::from(p.to_string()))
            .collect();
        let (results, result_paths): (Vec<&Json>, Vec<&Json>) = match case.get("results") {
            Some(Json::Array(alternatives)) => (
                alternatives.iter().collect(),
                case["results_paths"]
                    .as_array()
                    .map(|a| a.iter().collect())
                    .unwrap_or_default(),
            ),
            _ => (vec![&case["result"]], vec![&case["result_paths"]]),
        };
        if !results.iter().any(|r| matches(&values, r)) {
            failures.push(format!("{name}: {selector} answered {values:?}"));
        }
        if !result_paths.is_empty() && !result_paths.iter().any(|r| matches(&paths, r)) {
            unescaped.push(name.to_string());
        }
    }
    assert_eq!(
        cases.len(),
        715,
        "the suite's case count at the pinned commit"
    );
    assert_eq!(invalid, 249, "the suite's invalid selectors");
    assert!(
        failures.is_empty(),
        "{} of 715 cases:\n{}",
        failures.len(),
        failures.join("\n")
    );
    assert_eq!(
        unescaped, UNESCAPED_NORMALIZED_PATHS,
        "the normalized paths rendered without escapes"
    );
}

/// The cases whose normalized path `serde_json_path` 0.7.2 renders without
/// RFC 9535's escapes (see the module comment); their values are right.
const UNESCAPED_NORMALIZED_PATHS: [&str; 16] = [
    "name selector, name, double quotes, contains single quote",
    "name selector, double quotes, escaped reverse solidus",
    "name selector, double quotes, escaped backspace",
    "name selector, double quotes, escaped form feed",
    "name selector, double quotes, escaped line feed",
    "name selector, double quotes, escaped carriage return",
    "name selector, double quotes, escaped tab",
    "name selector, double quotes, escaped control character",
    "name selector, single quotes, escaped single quote",
    "name selector, single quotes, escaped reverse solidus",
    "name selector, single quotes, escaped backspace",
    "name selector, single quotes, escaped form feed",
    "name selector, single quotes, escaped line feed",
    "name selector, single quotes, escaped carriage return",
    "name selector, single quotes, escaped tab",
    "name selector, single quotes, escaped control character",
];
