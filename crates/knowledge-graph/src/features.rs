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

//! API-fetcher features (Java `FeatureRunner` + `@FetchFeature` + the
//! `PlaygroundLoader` scan): named pre/post-processing hooks a provider node
//! lists in its `feature` property. A before-feature updates the outbound
//! HTTP request (e.g. an `oauth-bearer` implementation acquiring a token); an
//! after-feature reads the HTTP response. Both may read/write the graph
//! instance's state machine.
//!
//! Rust has no runtime annotation scanning, so features register either
//! **declaratively** with the `#[fetch_feature("name")]` macro (the Java
//! annotation analog — used in field installations for cases like fetching/
//! refreshing an OAuth 2.0 access token and inserting the bearer token into
//! the outbound request) or explicitly through [`register`]. The two
//! built-in demonstration features — `log-request-headers` and
//! `log-response-headers` — are themselves `#[fetch_feature]` declarations:
//! the engine dogfoods its extension point, like Java's `@FetchFeature`
//! classes. A stacked `#[optional_service("condition")]` marker gates a
//! declared feature on application configuration (Java `@OptionalService`,
//! evaluated at boot).

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

use event_script::mlm::MultiLevelMap;
use knowledge_graph_macros::fetch_feature;
use platform_core::automation::AsyncHttpRequest;
use platform_core::AppConfigReader;
use rmpv::Value;

/// The observable parts of an HTTP response for after-features.
pub struct HttpResponseView {
    pub status: i32,
    pub headers: Vec<(String, String)>,
    pub body: Value,
}

/// Java `FeatureRunner`. `run_before() == true` receives the mutable request
/// (no response yet); after-features receive the response.
pub trait FeatureRunner: Send + Sync {
    fn run_before(&self) -> bool;
    fn execute(
        &self,
        request: Option<&mut AsyncHttpRequest>,
        response: Option<&HttpResponseView>,
        state: &mut MultiLevelMap,
        node_name: &str,
    );
}

/// A `#[fetch_feature]`-annotated feature (Java `@FetchFeature`) collected
/// from the link-time inventory. `optional_service` carries a stacked
/// `#[optional_service("condition")]` marker (Java `@OptionalService`),
/// evaluated at boot by [`load_declared_features`].
pub struct FetchFeatureEntry {
    pub name: &'static str,
    pub optional_service: Option<&'static str>,
    pub factory: fn() -> Arc<dyn FeatureRunner>,
}

platform_core::inventory::collect!(FetchFeatureEntry);

/// Load every `#[fetch_feature]` from the link-time inventory (the Java
/// `PlaygroundLoader` classpath-scan analog). Runs at startup, before any
/// graph executes. An `optional_service` condition that does not hold skips
/// the feature (Java `Feature.isRequired`); a duplicate name warns +
/// last-wins (the one conflict policy) — and a later explicit [`register`]
/// call still replaces any feature (explicit wins over declarative). Returns
/// how many feature names are registered.
pub fn load_declared_features() -> usize {
    let config = AppConfigReader::get_instance();
    for entry in platform_core::inventory::iter::<FetchFeatureEntry> {
        if !platform_core::util::feature::is_required(entry.optional_service, config) {
            log::info!("Skip optional FetchFeature - {}", entry.name);
            continue;
        }
        register(entry.name, (entry.factory)());
    }
    registry().read().expect("feature registry poisoned").len()
}

fn registry() -> &'static RwLock<HashMap<String, Arc<dyn FeatureRunner>>> {
    static FEATURES: OnceLock<RwLock<HashMap<String, Arc<dyn FeatureRunner>>>> = OnceLock::new();
    FEATURES.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Register a feature by name (the Java `@FetchFeature` value). A duplicate
/// name warns + last-wins (the one conflict policy, D2 — Java
/// `PlaygroundLoader` wording); explicit registration wins over declarative
/// because it runs later and replaces.
pub fn register(name: &str, feature: Arc<dyn FeatureRunner>) {
    if registry()
        .write()
        .expect("feature registry poisoned")
        .insert(name.to_string(), feature)
        .is_some()
    {
        log::warn!("Reloading FetchFeature {name} - please check duplicated feature name");
    }
    log::info!("Feature {name} loaded as API fetcher feature");
}

pub fn get_feature(name: &str) -> Option<Arc<dyn FeatureRunner>> {
    registry()
        .read()
        .expect("feature registry poisoned")
        .get(name)
        .cloned()
}

/// Java `LogRequestHeaders` (`log-request-headers`): saves outbound request
/// headers into the state machine under `{node}.header.request.*`.
#[fetch_feature("log-request-headers")]
struct LogRequestHeaders;

impl FeatureRunner for LogRequestHeaders {
    fn run_before(&self) -> bool {
        true
    }

    fn execute(
        &self,
        request: Option<&mut AsyncHttpRequest>,
        _response: Option<&HttpResponseView>,
        state: &mut MultiLevelMap,
        node_name: &str,
    ) {
        if let Some(request) = request {
            for (key, value) in request.headers() {
                let _ = state.set_element(
                    &format!("{node_name}.header.request.{key}"),
                    Value::from(value.as_str()),
                );
            }
        }
    }
}

/// Java `LogResponseHeaders` (`log-response-headers`): saves response
/// headers under `{node}.header.response.*`.
#[fetch_feature("log-response-headers")]
struct LogResponseHeaders;

impl FeatureRunner for LogResponseHeaders {
    fn run_before(&self) -> bool {
        false
    }

    fn execute(
        &self,
        _request: Option<&mut AsyncHttpRequest>,
        response: Option<&HttpResponseView>,
        state: &mut MultiLevelMap,
        node_name: &str,
    ) {
        if let Some(response) = response {
            for (key, value) in &response.headers {
                let _ = state.set_element(
                    &format!("{node_name}.header.response.{key}"),
                    Value::from(value.as_str()),
                );
            }
        }
    }
}
