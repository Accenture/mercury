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

//! Registration-metadata conformance for the FUNCTION kind: the golden
//! vectors in `registration-vectors/core.json` are shared VERBATIM with
//! every engine repository (the Java reference declares the same fixture set
//! through its annotations). This binary declares the fixtures through the
//! Rust carrier — including one marker deliberately ABOVE `#[preload]`, so
//! the conformance fixture itself exercises the order-freedom the contract
//! requires — boots, and asserts declared metadata (straight from the
//! link-time inventory) and resolved registration against the golden
//! entries. See docs/guides/registration-metadata-contract.md.

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use platform_core::registry::PreloadEntry;
// note: only the ABOVE-order marker (#[event_interceptor]) needs its macro in
// scope — a below-order marker is consumed by #[preload] before the compiler
// ever resolves it, so #[zero_tracing] / #[optional_service] need no import
use platform_core::{
    event_interceptor, inventory, main_application, overrides, preload, resources, AppError,
    AutoStart, ComposableFunction, EntryPoint, EventEnvelope, Platform,
};

/// Golden entry 1: comma-separated aliases + envInstances resolved at boot.
#[preload(
    route = "vector.alias.one, vector.alias.two",
    instances = 5,
    env_instances = "vector.instances"
)]
struct VectorAliasFunction;

#[async_trait]
impl ComposableFunction for VectorAliasFunction {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Ok(EventEnvelope::new().set_raw_body(input.body().clone()))
    }
}

/// Golden entry 2: the full marker stack — one marker deliberately ABOVE the
/// primary attribute (order-free stacking is part of the contract), the rest
/// below, plus a satisfied optional-service condition.
#[event_interceptor]
#[preload(route = "vector.marked")]
#[zero_tracing]
#[optional_service("vector.feature.on")]
struct VectorMarkedFunction;

#[async_trait]
impl ComposableFunction for VectorMarkedFunction {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        // an interceptor's returned envelope is ignored by the engine
        EventEnvelope::new().set_body("ignored")
    }
}

/// gatedOut fixture: a false optional-service condition must skip
/// registration entirely.
#[preload(route = "vector.gated.out")]
#[optional_service("!vector.feature.on")]
struct VectorGatedOutFunction;

#[async_trait]
impl ComposableFunction for VectorGatedOutFunction {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Ok(EventEnvelope::new().set_raw_body(input.body().clone()))
    }
}

/// Minimal main application so the lifecycle completes.
#[main_application]
struct TestMain;

#[async_trait]
impl EntryPoint for TestMain {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

/// Find the declared inventory entry whose sorted comma-split route list
/// equals the golden entry's `routes`.
fn declared_entry(routes: &[String]) -> Option<&'static PreloadEntry> {
    inventory::iter::<PreloadEntry>.into_iter().find(|entry| {
        let mut declared: Vec<String> = entry
            .route
            .split(',')
            .map(|r| r.trim().to_string())
            .collect();
        declared.sort();
        declared == routes
    })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn function_kind_matches_golden_vectors() {
    resources::prepend_resource_root("tests/resources");
    let holding = std::env::temp_dir().join(format!("mercury-vec-test-{}", std::process::id()));
    overrides::set("transient.data.store", &holding.display().to_string());
    let vectors: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string("tests/resources/registration-vectors/core.json")
            .expect("golden vectors file must exist"),
    )
    .expect("valid vectors json");
    // the conformance run provides exactly the vectors' assumedConfig
    for (key, value) in vectors["assumedConfig"].as_object().expect("assumedConfig") {
        overrides::set(key, value.as_str().expect("config value"));
    }
    AutoStart::main(vec![]).await.expect("lifecycle");
    let platform = Platform::get_instance();

    let entries = vectors["entries"].as_array().expect("entries");
    assert_eq!(entries.len(), 2, "vector entry count");
    for expected in entries {
        let routes: Vec<String> = expected["routes"]
            .as_array()
            .expect("routes")
            .iter()
            .map(|r| r.as_str().expect("route").to_string())
            .collect();
        let primary = &routes[0];
        // declared metadata matches the vectors (read straight from the
        // link-time inventory — the Rust analog of Java's reflection)
        let declared = declared_entry(&routes)
            .unwrap_or_else(|| panic!("a declared fixture must exist for {primary}"));
        assert_eq!(
            expected["declaredInstances"].as_u64().expect("instances") as usize,
            declared.instances,
            "{primary}: declaredInstances"
        );
        assert_eq!(
            expected["envInstances"].as_str().expect("envInstances"),
            declared.env_instances.unwrap_or(""),
            "{primary}: envInstances"
        );
        assert_eq!(
            expected["isPrivate"].as_bool().expect("isPrivate"),
            declared.is_private,
            "{primary}: isPrivate"
        );
        let expected_condition = expected["optionalService"].as_str();
        assert_eq!(
            expected_condition, declared.optional_service,
            "{primary}: optionalService"
        );
        assert_eq!(
            expected["zeroTracing"].as_bool().expect("zeroTracing"),
            declared.zero_tracing,
            "{primary}: zeroTracing"
        );
        assert_eq!(
            expected["eventInterceptor"]
                .as_bool()
                .expect("eventInterceptor"),
            declared.interceptor,
            "{primary}: eventInterceptor"
        );
        // resolved registration matches the vectors, for every alias
        let resolved_instances = expected["resolvedInstances"].as_u64().expect("resolved") as usize;
        for route in &routes {
            assert!(platform.has_route(route), "{route} must be registered");
            assert_eq!(
                platform.instances(route),
                Some(resolved_instances),
                "{route}: resolvedInstances (envInstances resolved at boot)"
            );
            assert_eq!(
                platform.is_private(route),
                Some(expected["isPrivate"].as_bool().expect("isPrivate")),
                "{route}: isPrivate resolved"
            );
        }
    }
    // gated-out fixtures must never register
    for route in vectors["gatedOut"].as_array().expect("gatedOut") {
        let route = route.as_str().expect("route");
        assert!(
            !platform.has_route(route),
            "{route} carries a false optional-service condition and must not register"
        );
    }
    // sanity: the marked route is callable (the interceptor swallows its
    // reply, so a fire-and-forget send suffices to prove liveness)
    let po = platform_core::PostOffice::new(&platform);
    po.send(
        EventEnvelope::new()
            .set_to("vector.marked")
            .set_body("ping")
            .expect("body"),
    )
    .await
    .expect("send to the marked route");
    tokio::time::sleep(Duration::from_millis(50)).await;
}
