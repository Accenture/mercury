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

//! `yaml.preload.override` end to end (the Java `PreloadOverrideTest`
//! analog): a config-driven transform over the collected `#[preload]` set —
//! rename, fan-out sharing one function object, keep-original, instances
//! override, multi-file merge (route-set union + first-set-wins instances),
//! a missing file skipped in a location chain, an alias-declared original,
//! and a non-matched function registering unchanged.
//!
//! One test function on purpose: `AutoStart::main` boots once per process
//! and registers routes on this test's runtime.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::{
    main_application, overrides, preload, resources, AppError, AutoStart, ComposableFunction,
    EntryPoint, EventEnvelope, Platform, PostOffice,
};

/// Minimal main application so the lifecycle completes.
#[main_application]
struct TestMain;

#[async_trait]
impl EntryPoint for TestMain {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

static RENAME_CALLS: AtomicUsize = AtomicUsize::new(0);

/// Renamed + fanned out by the override (po.renamed.one / po.renamed.two,
/// instances 2 → 8); the original route must NOT register.
#[preload(route = "po.rename.me", instances = 2)]
struct RenameMe;

#[async_trait]
impl ComposableFunction for RenameMe {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        RENAME_CALLS.fetch_add(1, Ordering::SeqCst);
        EventEnvelope::new().set_body("renamed")
    }
}

/// keep-original: the replacement set gains the original route back.
#[preload(route = "po.keep.me", instances = 2)]
struct KeepMe;

#[async_trait]
impl ComposableFunction for KeepMe {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new().set_body("kept")
    }
}

/// Multi-file merge target: file 1 maps it to po.merge.a with instances 5,
/// file 2 adds po.merge.b with instances 9 — routes UNION, first-set wins.
#[preload(route = "po.merge.me")]
struct MergeMe;

#[async_trait]
impl ComposableFunction for MergeMe {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new().set_body("merged")
    }
}

/// An override entry matches when ANY declared route (comma-split aliases)
/// appears as an `original` — po.alias.two matches, so the WHOLE declared
/// list is replaced by the override's route set.
#[preload(route = "po.alias.one, po.alias.two")]
struct AliasMatch;

#[async_trait]
impl ComposableFunction for AliasMatch {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new().set_body("alias replaced")
    }
}

/// No override entry names this function — it must register unchanged.
#[preload(route = "po.untouched", instances = 3)]
struct Untouched;

#[async_trait]
impl ComposableFunction for Untouched {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new().set_body("untouched")
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn preload_override_matches_java_semantics() {
    resources::prepend_resource_root("tests/resources");
    let holding = std::env::temp_dir().join(format!("mercury-po-test-{}", std::process::id()));
    overrides::set("transient.data.store", &holding.display().to_string());
    // a missing file first in the chain is logged and SKIPPED, never an
    // error (Java chains classpath:/not-found.yaml to prove the same)
    overrides::set(
        "yaml.preload.override",
        "classpath:/no-such-override.yaml, classpath:/preload-override.yaml, \
         classpath:/preload-override-2.yaml",
    );
    AutoStart::main(vec![]).await.expect("lifecycle");
    let platform = Platform::get_instance();
    let po = PostOffice::new(&platform);

    // rename + fan-out: the original is GONE, both replacements serve the
    // SAME function object, and the override's instances (8) replaced the
    // env-resolved count (2)
    assert!(!platform.has_route("po.rename.me"), "original renamed away");
    for route in ["po.renamed.one", "po.renamed.two"] {
        assert!(platform.has_route(route), "{route} must be registered");
        assert_eq!(platform.instances(route), Some(8), "{route} instances");
        let reply = po
            .request(
                EventEnvelope::new().set_to(route).set_body("go").unwrap(),
                Duration::from_secs(2),
            )
            .await
            .expect("renamed rpc");
        assert_eq!(reply.body_as::<String>().unwrap(), "renamed");
    }
    assert_eq!(
        RENAME_CALLS.load(Ordering::SeqCst),
        2,
        "one shared handler must have served both replacement routes"
    );

    // keep-original: replacement set + the original back; instances
    // untouched (the override sets none)
    assert!(platform.has_route("po.keep.me"), "keep-original");
    assert!(platform.has_route("po.keep.alias"));
    assert_eq!(platform.instances("po.keep.me"), Some(2));
    assert_eq!(platform.instances("po.keep.alias"), Some(2));

    // multi-file merge: route sets UNIONed across files; the FIRST file to
    // set instances wins (5, not 9)
    assert!(!platform.has_route("po.merge.me"));
    for route in ["po.merge.a", "po.merge.b"] {
        assert!(platform.has_route(route), "{route} from the merged set");
        assert_eq!(
            platform.instances(route),
            Some(5),
            "first file to set instances must win"
        );
    }

    // alias-declared original: matching ANY declared route replaces the
    // whole declared list with the override's route set
    assert!(!platform.has_route("po.alias.one"));
    assert!(!platform.has_route("po.alias.two"));
    assert!(platform.has_route("po.alias.replaced"));
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("po.alias.replaced")
                .set_body("go")
                .unwrap(),
            Duration::from_secs(2),
        )
        .await
        .expect("alias rpc");
    assert_eq!(reply.body_as::<String>().unwrap(), "alias replaced");

    // a non-matched function registers unchanged
    assert!(platform.has_route("po.untouched"));
    assert_eq!(platform.instances("po.untouched"), Some(3));
}
