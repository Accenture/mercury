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
//! Rust has no runtime annotation scanning, so features register explicitly
//! through [`register`] (applications typically do this in a
//! `#[before_application]` hook). The two built-in demonstration features —
//! `log-request-headers` and `log-response-headers` — are registered by the
//! engine itself.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

use event_script::mlm::MultiLevelMap;
use platform_core::automation::AsyncHttpRequest;
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

fn registry() -> &'static RwLock<HashMap<String, Arc<dyn FeatureRunner>>> {
    static FEATURES: OnceLock<RwLock<HashMap<String, Arc<dyn FeatureRunner>>>> = OnceLock::new();
    FEATURES.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Register a feature by name (the Java `@FetchFeature` value).
pub fn register(name: &str, feature: Arc<dyn FeatureRunner>) {
    registry()
        .write()
        .expect("feature registry poisoned")
        .insert(name.to_string(), feature);
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

/// Register the built-in demonstration features (idempotent).
pub fn register_builtins() {
    if get_feature("log-request-headers").is_none() {
        register("log-request-headers", Arc::new(LogRequestHeaders));
    }
    if get_feature("log-response-headers").is_none() {
        register("log-response-headers", Arc::new(LogResponseHeaders));
    }
}
