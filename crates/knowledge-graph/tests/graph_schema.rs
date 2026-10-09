//! The graph contract's schema vocabulary (RFC-0007, WP2) pinned by the shared vector file
//! `graph-schema-vectors.json`: what the gate accepts and refuses, with the exact refusal, and what
//! the validator reports for a given input, every violation in order, plus the one-line message
//! with its cap. The same file, byte-identical, drives the Java engine's test.

use event_script::conversions::from_json;
use knowledge_graph::schema::{self, compile, compile_contract, compile_header_part, report};

fn vectors() -> serde_json::Value {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/resources/graph-schema-vectors.json"
    );
    serde_json::from_str(&std::fs::read_to_string(path).expect("the vector file"))
        .expect("valid JSON")
}

#[test]
fn the_gate_accepts_the_vocabulary_and_refuses_the_rest() {
    let vectors = vectors();
    let cases = vectors["compile"].as_array().expect("compile cases");
    assert!(!cases.is_empty());
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let schema = from_json(&case["schema"]);
        let outcome = match case["part"].as_str().unwrap_or("body") {
            "contract" => compile_contract(&schema).map(|_| ()),
            "header" => compile_header_part("schema.header", &schema).map(|_| ()),
            _ => compile("schema.body", &schema).map(|_| ()),
        };
        if case["ok"] == true {
            assert!(outcome.is_ok(), "{name}: {outcome:?}");
        } else {
            let expected = case["error"].as_str().expect("error");
            assert_eq!(Err(expected.to_string()), outcome, "{name}");
        }
    }
}

#[test]
fn the_validator_reports_every_violation_in_order() {
    let vectors = vectors();
    let cases = vectors["validate"].as_array().expect("validate cases");
    assert!(!cases.is_empty());
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let contract = compile_contract(&from_json(&case["schema"])).expect(name);
        let body = case.get("body").map(from_json);
        let header = case.get("header").map(from_json);
        let violations = contract.check(body.as_ref(), header.as_ref());
        let expected: Vec<String> = case["violations"]
            .as_array()
            .expect("violations")
            .iter()
            .map(|v| v.as_str().expect("text").to_string())
            .collect();
        assert_eq!(expected, violations, "{name}");
    }
}

#[test]
fn the_message_carries_the_violations_up_to_the_cap() {
    let vectors = vectors();
    assert_eq!(
        schema::VIOLATION_CAP as u64,
        vectors["cap"].as_u64().expect("cap")
    );
    let cases = vectors["report"].as_array().expect("report cases");
    assert!(!cases.is_empty());
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let violations: Vec<String> = case["violations"]
            .as_array()
            .expect("violations")
            .iter()
            .map(|v| v.as_str().expect("text").to_string())
            .collect();
        assert_eq!(
            case["message"].as_str().expect("message"),
            report(&violations),
            "{name}"
        );
    }
}
