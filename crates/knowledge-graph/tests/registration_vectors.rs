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

//! Registration-metadata conformance for the FEATURE kind: the golden
//! vectors in `registration-vectors/feature.json` are shared VERBATIM with
//! every engine repository. Feature names are explicit on the carrier, and
//! features honor optional-service gating — a feature whose condition
//! evaluates false at boot is skipped and must be ABSENT from the registry.
//! See docs/guides/registration-metadata-contract.md.

use knowledge_graph::features::{self, FeatureRunner, FetchFeatureEntry, HttpResponseView};
use knowledge_graph::fetch_feature;
use platform_core::automation::AsyncHttpRequest;
use platform_core::{inventory, overrides};

/// Satisfied condition: must load.
#[fetch_feature("vector-feature")]
#[optional_service("vector.feature.on")]
struct VectorFeature;

impl FeatureRunner for VectorFeature {
    fn run_before(&self) -> bool {
        true
    }

    fn execute(
        &self,
        _request: Option<&mut AsyncHttpRequest>,
        _response: Option<&HttpResponseView>,
        _state: &mut event_script::mlm::MultiLevelMap,
        _node_name: &str,
    ) {
    }
}

/// Negated condition: must be gated out.
#[fetch_feature("vector-feature-off")]
#[optional_service("!vector.feature.on")]
struct VectorFeatureOff;

impl FeatureRunner for VectorFeatureOff {
    fn run_before(&self) -> bool {
        false
    }

    fn execute(
        &self,
        _request: Option<&mut AsyncHttpRequest>,
        _response: Option<&HttpResponseView>,
        _state: &mut event_script::mlm::MultiLevelMap,
        _node_name: &str,
    ) {
    }
}

/// Find a declared inventory entry by feature name.
fn declared_entry(name: &str) -> Option<&'static FetchFeatureEntry> {
    inventory::iter::<FetchFeatureEntry>
        .into_iter()
        .find(|entry| entry.name == name)
}

#[test]
fn feature_kind_matches_golden_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string("tests/resources/registration-vectors/feature.json")
            .expect("golden vectors file must exist"),
    )
    .expect("valid vectors json");
    // the conformance run provides exactly the vectors' assumedConfig,
    // before the AppConfigReader singleton freezes
    for (key, value) in vectors["assumedConfig"].as_object().expect("assumedConfig") {
        overrides::set(key, value.as_str().expect("config value"));
    }
    features::load_declared_features();

    let entries = vectors["entries"].as_array().expect("entries");
    assert_eq!(entries.len(), 1, "vector entry count");
    for expected in entries {
        let name = expected["name"].as_str().expect("name");
        assert!(
            features::get_feature(name).is_some(),
            "{name} must be registered"
        );
        // the fixture's declared condition matches the vectors
        let declared = declared_entry(name)
            .unwrap_or_else(|| panic!("a declared fixture must exist for {name}"));
        assert_eq!(
            expected["optionalService"].as_str(),
            declared.optional_service,
            "{name}: optionalService"
        );
    }
    // gated-out fixtures must never register — but their declarations exist
    // in the inventory with the negated condition
    for name in vectors["gatedOut"].as_array().expect("gatedOut") {
        let name = name.as_str().expect("name");
        assert!(
            features::get_feature(name).is_none(),
            "{name} carries a false optional-service condition and must not register"
        );
        assert!(
            declared_entry(name).is_some(),
            "{name} must still be a declared fixture"
        );
    }
}
