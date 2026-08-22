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

//! Twin of the Java `ContractCatalogTest`: every behavior anchor in
//! contracts.yaml is resolved at COMPILE TIME (the Class.forName analog -
//! knowledge-graph anchors resolve through the dev-only dependency), so a
//! renamed or removed behavior item fails the workspace build; plus the
//! negative validation fixtures.

use std::collections::BTreeSet;

#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

use app::catalog::{load, ContractCatalog};

/// Each returned anchor string is paired with a compile-time reference to the
/// item it names - this function stops compiling when an anchor drifts.
fn verified_anchors() -> BTreeSet<String> {
    let _ = std::any::type_name::<platform_core::AutoStart>();
    let _ = std::any::type_name::<platform_core::PostOffice>();
    let _ = std::any::type_name::<platform_core::EventEnvelope>();
    let _ = std::any::type_name::<platform_core::automation::routing::RoutingTable>();
    let _ = event_script::compiler::compile_flows;
    let _ = knowledge_graph::commands::handle;
    let _ = knowledge_graph::compiler::compile_graphs;
    let _ = knowledge_graph::model_validator::validate;
    [
        "platform_core::AutoStart",
        "platform_core::PostOffice",
        "platform_core::EventEnvelope",
        "platform_core::automation::routing::RoutingTable",
        "event_script::compiler::compile_flows",
        "knowledge_graph::commands::handle",
        "knowledge_graph::compiler::compile_graphs",
        "knowledge_graph::model_validator::validate",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

#[test]
fn catalog_anchors_are_compile_verified() {
    let catalog: BTreeSet<String> = ContractCatalog::get_instance()
        .contracts()
        .iter()
        .flat_map(|c| c.anchors.clone())
        .collect();
    assert_eq!(catalog, verified_anchors());
}

#[test]
fn catalog_loads_four_contracts_sorted_by_id() {
    let contracts = ContractCatalog::get_instance().contracts();
    let ids: Vec<&str> = contracts.iter().map(|c| c.id.as_str()).collect();
    // identical to the Java engine's contract ids - the consumer-facing vocabulary
    assert_eq!(
        ids,
        [
            "event-script",
            "minigraph",
            "platform-core",
            "rest-automation"
        ]
    );
    assert!(ContractCatalog::get_instance()
        .contract("platform-core")
        .is_some());
    assert!(ContractCatalog::get_instance().contract("nope").is_none());
}

#[test]
fn invalid_catalogs_are_rejected() {
    let cases = [
        ("no list", "other: 1"),
        ("empty list", "contracts: []"),
        ("not a map", "contracts:\n  - just-a-string"),
        (
            "missing id",
            "contracts:\n  - module: 'm-x'\n    summary: 'S'\n    anchors: ['a::b']\n    references: ['references/x.md']",
        ),
        (
            "duplicate id",
            "contracts:\n  - id: 'dup-id'\n    module: 'm-x'\n    summary: 'S'\n    anchors: ['a::b']\n    references: ['references/x.md']\n  - id: 'dup-id'\n    module: 'm-x'\n    summary: 'S'\n    anchors: ['a::b']\n    references: ['references/x.md']",
        ),
        (
            "anchor without path separator",
            "contracts:\n  - id: 'c-1'\n    module: 'm-x'\n    summary: 'S'\n    anchors: ['NotAPath']\n    references: ['references/x.md']",
        ),
        (
            "reference outside references/",
            "contracts:\n  - id: 'c-1'\n    module: 'm-x'\n    summary: 'S'\n    anchors: ['a::b']\n    references: ['../escape.md']",
        ),
        (
            "reference with parent traversal",
            "contracts:\n  - id: 'c-1'\n    module: 'm-x'\n    summary: 'S'\n    anchors: ['a::b']\n    references: ['references/../../x.md']",
        ),
        (
            "empty anchors",
            "contracts:\n  - id: 'c-1'\n    module: 'm-x'\n    summary: 'S'\n    anchors: []\n    references: ['references/x.md']",
        ),
    ];
    for (label, yaml) in cases {
        assert!(load(yaml).is_err(), "expected rejection: {label}");
    }
}

#[test]
fn valid_minimal_catalog_is_accepted() {
    let yaml = "contracts:\n  - id: 'c-1'\n    module: 'm-x'\n    summary: 'S'\n    anchors: ['a::b']\n    references: ['references/x.md']";
    let contracts = load(yaml).expect("minimal catalog");
    assert_eq!(contracts.len(), 1);
    assert_eq!(contracts[0].id, "c-1");
}
