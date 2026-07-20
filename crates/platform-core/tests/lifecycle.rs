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

//! Integration tests for increment 4 — the application lifecycle
//! (`AppStarter`/`EntryPoint`, Platform identity, elastic-store housekeeping).
//!
//! Note: `AppStarter` runs against the process-wide `Platform::get_instance()`,
//! so every test uses **unique route names** (the registry is shared across
//! the test binary).

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::util::elastic_queue;
use platform_core::{
    overrides, resources, AppConfigReader, AppError, AppStarter, ComposableFunction, EntryPoint,
    EventEnvelope, Platform, PostOffice,
};

fn setup_config() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let holding =
            std::env::temp_dir().join(format!("mercury-lifecycle-test-{}", std::process::id()));
        overrides::set("transient.data.store", &holding.display().to_string());
        let _ = AppConfigReader::get_instance();
    });
}

/// Records a named lifecycle event into a shared journal.
struct Recorder {
    label: &'static str,
    journal: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl EntryPoint for Recorder {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        self.journal
            .lock()
            .expect("journal mutex")
            .push(self.label.to_string());
        Ok(())
    }
}

struct Echo;

#[async_trait]
impl ComposableFunction for Echo {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Ok(EventEnvelope::new().set_raw_body(input.body().clone()))
    }
}

/// A main application that proves preloaded functions are callable before
/// mains run (the Java lifecycle guarantee).
struct RpcMain {
    route: &'static str,
    journal: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl EntryPoint for RpcMain {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        let platform = Platform::get_instance();
        assert!(platform.has_route(self.route), "preload must precede main");
        let po = PostOffice::new(&platform);
        let response = po
            .request(
                EventEnvelope::new().set_to(self.route).set_body("ping")?,
                Duration::from_secs(2),
            )
            .await?;
        assert_eq!(response.body_as::<String>()?, "ping");
        self.journal
            .lock()
            .expect("journal mutex")
            .push("main".to_string());
        Ok(())
    }
}

struct FailingHook;

#[async_trait]
impl EntryPoint for FailingHook {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Err(AppError::new(500, "preflight validation failed"))
    }
}

#[tokio::test]
async fn lifecycle_runs_phases_in_order() {
    setup_config();
    let journal = Arc::new(Mutex::new(Vec::new()));
    // before-application hooks registered OUT of order — sequence must win
    AppStarter::new()
        .before_application(
            5,
            Arc::new(Recorder {
                label: "before-5",
                journal: journal.clone(),
            }),
        )
        .before_application(
            3,
            Arc::new(Recorder {
                label: "before-3",
                journal: journal.clone(),
            }),
        )
        .preload("v1.lifecycle.echo", Arc::new(Echo), 2)
        .main_application(
            1,
            Arc::new(RpcMain {
                route: "v1.lifecycle.echo",
                journal: journal.clone(),
            }),
        )
        .run(vec![])
        .await
        .unwrap();
    assert_eq!(
        journal.lock().unwrap().clone(),
        vec!["before-3", "before-5", "main"]
    );
    // housekeeping started: the holding area exists (the RUNNING marker is
    // asserted in the shutdown test, which owns the marker's lifecycle —
    // tests share the process-wide holding area)
    assert!(elastic_queue::base_dir().is_dir());
}

#[tokio::test]
async fn multiple_mains_run_by_sequence() {
    setup_config();
    let journal = Arc::new(Mutex::new(Vec::new()));
    AppStarter::new()
        .preload("v1.lifecycle.multi", Arc::new(Echo), 1)
        .main_application(
            2,
            Arc::new(Recorder {
                label: "main-2",
                journal: journal.clone(),
            }),
        )
        .main_application(
            1,
            Arc::new(Recorder {
                label: "main-1",
                journal: journal.clone(),
            }),
        )
        .run(vec![])
        .await
        .unwrap();
    assert_eq!(journal.lock().unwrap().clone(), vec!["main-1", "main-2"]);
}

#[tokio::test]
async fn failing_before_hook_aborts_startup() {
    setup_config();
    let main_ran = Arc::new(AtomicBool::new(false));
    struct FlagMain(Arc<AtomicBool>);
    #[async_trait]
    impl EntryPoint for FlagMain {
        async fn start(&self, _args: &[String]) -> Result<(), AppError> {
            self.0.store(true, Ordering::SeqCst);
            Ok(())
        }
    }
    let err = AppStarter::new()
        .before_application(3, Arc::new(FailingHook))
        .preload("v1.lifecycle.aborted", Arc::new(Echo), 1)
        .main_application(1, Arc::new(FlagMain(main_ran.clone())))
        .run(vec![])
        .await
        .unwrap_err();
    assert!(err
        .message()
        .contains("BeforeApplication (sequence 3) failed"));
    // the failure aborted startup: no preload, no main
    assert!(!Platform::get_instance().has_route("v1.lifecycle.aborted"));
    assert!(!main_ran.load(Ordering::SeqCst));
}

#[tokio::test]
async fn missing_main_application_is_an_error() {
    setup_config();
    let err = AppStarter::new()
        .preload("v1.lifecycle.nomain", Arc::new(Echo), 1)
        .run(vec![])
        .await
        .unwrap_err();
    assert_eq!(err.status(), 400);
    assert!(err.message().contains("Missing main application"));
}

#[tokio::test]
async fn global_platform_is_shared() {
    setup_config();
    let a = Platform::get_instance();
    AppStarter::new()
        .preload("v1.lifecycle.shared", Arc::new(Echo), 1)
        .main_application(
            1,
            Arc::new(Recorder {
                label: "noop",
                journal: Arc::new(Mutex::new(Vec::new())),
            }),
        )
        .run(vec![])
        .await
        .unwrap();
    // a Platform handle taken BEFORE the run sees the route (shared registry)
    assert!(a.has_route("v1.lifecycle.shared"));
    // identity: origin is stable per process; name comes from config
    assert_eq!(Platform::origin(), Platform::origin());
    assert!(!Platform::name().is_empty());
}

#[tokio::test]
async fn unknown_holding_area_with_segments_is_removed() {
    setup_config();
    let tmp_root = elastic_queue::base_dir().parent().unwrap().to_path_buf();
    // fabricate debris: a sibling holding area with a segment file and no RUNNING marker
    let debris = tmp_root.join("dead-process-deadbeef");
    std::fs::create_dir_all(&debris).unwrap();
    std::fs::write(debris.join("eq-x.y-0-0.dat"), b"junk").unwrap();
    // parallel lifecycle tests may trigger the one-time housekeeping scan at
    // the same moment; two concurrent remove_dir_all calls can both abort
    // mid-removal — retry until the debris is gone (production runs the scan
    // once, so the concurrency is a test-only artifact)
    let mut removed = false;
    for _ in 0..20 {
        elastic_queue::scan_expired_stores();
        if !debris.exists() {
            removed = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    assert!(removed, "unknown holding area should be removed");
    // a sibling with a FRESH marker survives (not expired)
    let alive = tmp_root.join("alive-process-cafe");
    std::fs::create_dir_all(&alive).unwrap();
    std::fs::write(alive.join("eq-x.y-0-0.dat"), b"junk").unwrap();
    std::fs::write(alive.join("RUNNING"), b"now").unwrap();
    elastic_queue::scan_expired_stores();
    assert!(alive.exists(), "live holding area must survive the scan");
    std::fs::remove_dir_all(&alive).ok();
}

#[tokio::test]
async fn shutdown_cleanup_removes_marker_and_segments() {
    setup_config();
    // ensure the holding area + marker exist
    let dir = elastic_queue::base_dir().to_path_buf();
    elastic_queue::start_housekeeping();
    assert!(dir.join("RUNNING").is_file());
    elastic_queue::shutdown_cleanup();
    assert!(!dir.join("RUNNING").exists());
    // no segment files left behind
    let leftovers: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.starts_with("eq-") && n.ends_with(".dat"))
        .collect();
    assert!(
        leftovers.is_empty(),
        "segments should be purged: {leftovers:?}"
    );
    // restore the marker for any test still using the shared holding area
    std::fs::write(dir.join("RUNNING"), b"restored").unwrap();
}
