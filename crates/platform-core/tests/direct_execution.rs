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

//! Direct execution for the reserved Event Script engine routes (Java
//! `EventEmitter.sendWithEventBus` parity): events to `event.script.manager`
//! and `task.executor` bypass the manager-worker queue and run on a fresh
//! task each — so the engine never queues behind itself and worker-instance
//! count is irrelevant. Every other route keeps the reactive back-pressure
//! path. The bypass is intentionally NOT reachable through registration
//! options or annotations.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Once};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use platform_core::{
    overrides, resources, AppConfigReader, AppError, ComposableFunction, EventEnvelope, Platform,
    PostOffice,
};

fn setup_config() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let holding =
            std::env::temp_dir().join(format!("mercury-direct-test-{}", std::process::id()));
        overrides::set("transient.data.store", &holding.display().to_string());
        let _ = AppConfigReader::get_instance();
    });
}

/// Sleeps briefly and records the peak number of concurrent executions.
struct ConcurrencyProbe {
    current: Arc<AtomicUsize>,
    peak: Arc<AtomicUsize>,
    done: Arc<AtomicUsize>,
}

#[async_trait]
impl ComposableFunction for ConcurrencyProbe {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let now = self.current.fetch_add(1, Ordering::SeqCst) + 1;
        self.peak.fetch_max(now, Ordering::SeqCst);
        tokio::time::sleep(Duration::from_millis(150)).await;
        self.current.fetch_sub(1, Ordering::SeqCst);
        self.done.fetch_add(1, Ordering::SeqCst);
        Ok(EventEnvelope::new())
    }
}

struct Probe {
    peak: Arc<AtomicUsize>,
    done: Arc<AtomicUsize>,
}

fn probe(platform: &Platform, route: &str) -> Probe {
    let current = Arc::new(AtomicUsize::new(0));
    let peak = Arc::new(AtomicUsize::new(0));
    let done = Arc::new(AtomicUsize::new(0));
    platform
        .register(
            route,
            Arc::new(ConcurrencyProbe {
                current: current.clone(),
                peak: peak.clone(),
                done: done.clone(),
            }),
            1, // one worker instance — the point of the test
        )
        .unwrap();
    Probe { peak, done }
}

async fn fire(platform: &Platform, route: &str, n: usize) {
    let po = PostOffice::new(platform);
    for i in 0..n {
        po.send(
            EventEnvelope::new()
                .set_to(route)
                .set_body(format!("event-{i}"))
                .unwrap(),
        )
        .await
        .unwrap();
    }
}

async fn wait_done(done: &Arc<AtomicUsize>, n: usize, deadline: Duration) {
    let end = Instant::now() + deadline;
    while done.load(Ordering::SeqCst) < n && Instant::now() < end {
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn reserved_engine_routes_run_unbounded_despite_one_instance() {
    setup_config();
    let platform = Platform::new();
    let task_executor = probe(&platform, "task.executor");
    let manager = probe(&platform, "event.script.manager");
    let started = Instant::now();
    fire(&platform, "task.executor", 10).await;
    fire(&platform, "event.script.manager", 10).await;
    wait_done(&task_executor.done, 10, Duration::from_secs(5)).await;
    wait_done(&manager.done, 10, Duration::from_secs(5)).await;
    let elapsed = started.elapsed();
    assert_eq!(task_executor.done.load(Ordering::SeqCst), 10);
    assert_eq!(manager.done.load(Ordering::SeqCst), 10);
    // 10 × 150 ms serialized would be 1.5 s per route; direct execution runs
    // them concurrently, so the whole batch finishes in a fraction of that
    assert!(
        elapsed < Duration::from_millis(1200),
        "direct execution should parallelize: took {elapsed:?}"
    );
    assert!(
        task_executor.peak.load(Ordering::SeqCst) > 1,
        "task.executor must run concurrently despite 1 instance"
    );
    assert!(
        manager.peak.load(Ordering::SeqCst) > 1,
        "event.script.manager must run concurrently despite 1 instance"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn normal_routes_keep_the_reactive_serialized_path() {
    setup_config();
    let platform = Platform::new();
    // the control: same probe on a NON-reserved route with 1 instance
    let control = probe(&platform, "normal.route.probe");
    fire(&platform, "normal.route.probe", 4).await;
    wait_done(&control.done, 4, Duration::from_secs(5)).await;
    assert_eq!(control.done.load(Ordering::SeqCst), 4);
    assert_eq!(
        control.peak.load(Ordering::SeqCst),
        1,
        "a normal 1-instance route must serialize through its worker"
    );
}
