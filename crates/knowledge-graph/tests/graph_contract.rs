//! The graph contract (RFC-0007) pinned by the shared vector file `graph-contract-vectors.json`: for
//! each case the contract view, the OpenAPI 3.0 document and the describe lines the engine must
//! derive from the model alone. The same file, byte-identical, drives the Java engine's test; a
//! value compared through the canonical packager is order- and integer-width-insensitive, so the
//! two engines' maps are compared as values.

use event_script::conversions::from_json;
use knowledge_graph::contract::GraphContract;
use knowledge_graph::openapi;
use platform_core::canonical_packager::encode;

fn vectors() -> serde_json::Value {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/resources/graph-contract-vectors.json"
    );
    serde_json::from_str(&std::fs::read_to_string(path).expect("the vector file"))
        .expect("valid JSON")
}

#[test]
fn shared_vectors_pin_the_contract_and_the_document() {
    let vectors = vectors();
    let cases = vectors["cases"].as_array().expect("cases");
    assert!(!cases.is_empty());
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let graph = &case["graph"];
        let others = case.get("others").cloned().unwrap_or(serde_json::json!({}));
        let contract = GraphContract::derive(name, graph, &|id| others.get(id).cloned());
        assert_eq!(
            encode(&from_json(&case["contract"])).expect("expected view"),
            encode(&contract.to_view()).expect("actual view"),
            "contract view of {name}: {}",
            openapi::to_json(&contract.to_view())
        );
        let document = openapi::document(&contract, "1.0.0", None);
        assert_eq!(
            encode(&from_json(&case["openapi"])).expect("expected document"),
            encode(&document).expect("actual document"),
            "OpenAPI document of {name}: {}",
            openapi::to_json(&document)
        );
        assert_eq!(
            case["describe"].as_str().expect("describe"),
            contract.describe(),
            "describe lines of {name}"
        );
        // the YAML rendering reads back as the same document
        let yaml = openapi::to_yaml(&document).expect("yaml");
        assert!(yaml.starts_with("openapi: "), "YAML of {name}:\n{yaml}");
        let parsed: serde_json::Value = serde_yaml::from_str(&yaml).expect("the YAML parses");
        assert_eq!(
            encode(&document).expect("document"),
            encode(&from_json(&parsed)).expect("parsed"),
            "YAML round trip of {name}"
        );
    }
}

#[test]
fn the_document_names_its_server_and_version() {
    let model = serde_json::json!({
        "nodes": [
            {"alias": "root", "types": ["Root"], "properties": {"purpose": "Hello"}},
            {"alias": "end", "types": ["End"], "properties": {"mapping": ["input.body.name -> output.body.greeting"]}}
        ],
        "connections": [{"from": "root", "to": "end", "label": "contains"}]
    });
    let contract = GraphContract::derive("hello", &model, &|_| None);
    let document = openapi::document(&contract, "2.3.4", Some("http://127.0.0.1:8085"));
    let json: serde_json::Value = serde_json::from_str(&openapi::to_json(&document)).expect("json");
    assert_eq!(json["openapi"], "3.0.3");
    assert_eq!(json["servers"][0]["url"], "http://127.0.0.1:8085");
    assert_eq!(json["info"]["version"], "2.3.4");
    assert_eq!(json["info"]["title"], "hello");
    assert_eq!(json["info"]["description"], "Hello");
    let compact: String = openapi::to_json(&document)
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    assert!(compact.starts_with("{\"openapi\":\"3.0.3\""), "{compact}");
    assert!(contract.describe().contains("  input.body.name\n"));
    assert!(contract.describe().contains("Declared schema: none\n"));
}
