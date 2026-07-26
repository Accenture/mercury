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

//! Fetch-feature registration semantics (annotation→macro consistency round):
//! the built-ins are `#[fetch_feature]` declarations, a duplicate name =
//! WARN + last-wins (D2, Java `PlaygroundLoader` wording), and a stacked
//! `#[optional_service]` marker gates a declared feature on configuration
//! (D3a, Java `@OptionalService` + `Feature.isRequired`). A dedicated test
//! binary: it installs a capturing logger and manipulates config overrides
//! before the `AppConfigReader` singleton freezes.

use std::sync::{Arc, Mutex, OnceLock};

use knowledge_graph::features::{self, FeatureRunner, HttpResponseView};
use knowledge_graph::fetch_feature;
use platform_core::automation::AsyncHttpRequest;
use platform_core::overrides;

struct CaptureLogger;

fn captured() -> &'static Mutex<Vec<String>> {
    static LOGS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    LOGS.get_or_init(|| Mutex::new(Vec::new()))
}

impl log::Log for CaptureLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        captured().lock().expect("capture log").push(format!(
            "{}: {}",
            record.level(),
            record.args()
        ));
    }

    fn flush(&self) {}
}

struct NoOp(bool);

impl FeatureRunner for NoOp {
    fn run_before(&self) -> bool {
        self.0
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

/// Gated on a config key this test sets: must LOAD.
#[fetch_feature("gated-present")]
#[optional_service("test.feature.present.key")]
struct GatedPresent;

impl FeatureRunner for GatedPresent {
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

/// Gated on a config key nothing sets: must be SKIPPED.
#[fetch_feature("gated-absent")]
#[optional_service("test.feature.absent.key")]
struct GatedAbsent;

impl FeatureRunner for GatedAbsent {
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

/// One sequential test fn: logger + config overrides must be in place before
/// the first `load_declared_features()` call, and the warn assertions read
/// the shared captured log.
#[test]
fn feature_registration_policy_matches_java() {
    log::set_logger(&CaptureLogger).expect("no other logger in this binary");
    log::set_max_level(log::LevelFilter::Debug);
    // the optional-service condition reads application config: satisfy the
    // "present" gate before the AppConfigReader singleton freezes
    overrides::set("test.feature.present.key", "true");

    // 1. declarative load: built-ins + gated features
    let count = features::load_declared_features();
    assert!(
        count >= 3,
        "built-ins + gated-present expected, got {count}"
    );
    assert!(
        features::get_feature("log-request-headers").is_some(),
        "built-in #[fetch_feature] declarations must be collected"
    );
    assert!(features::get_feature("log-response-headers").is_some());
    assert!(
        features::get_feature("gated-present").is_some(),
        "a satisfied #[optional_service] condition loads the feature"
    );
    assert!(
        features::get_feature("gated-absent").is_none(),
        "an unsatisfied #[optional_service] condition skips the feature"
    );
    let skipped = captured()
        .lock()
        .unwrap()
        .iter()
        .any(|line| line.contains("Skip optional FetchFeature - gated-absent"));
    assert!(
        skipped,
        "the skip must be logged with the Java wording: {:?}",
        captured().lock().unwrap()
    );

    // 2. duplicate name: WARN + last-wins (explicit register wins over
    //    declarative because it runs later and replaces)
    features::register("log-request-headers", Arc::new(NoOp(false)));
    let replaced = features::get_feature("log-request-headers").expect("still registered");
    assert!(!replaced.run_before(), "last registration wins");
    let warned = captured().lock().unwrap().iter().any(|line| {
        line.contains("WARN")
            && line.contains(
                "Reloading FetchFeature log-request-headers - please check duplicated feature name",
            )
    });
    assert!(
        warned,
        "a duplicate feature name must warn: {:?}",
        captured().lock().unwrap()
    );
}
