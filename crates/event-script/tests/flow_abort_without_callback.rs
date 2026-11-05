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

//! An aborted flow is closed even when its caller cannot be told (the twin of the Java
//! `FlowAbortWithoutCallbackTest`, mercury-composable #610).
//!
//! A flow launched with no callback has no reply-to address, and a callback route that is not
//! served cannot take the error. Either way, when the flow aborts the executor must still end
//! the flow instance, so the instance leaves the flow registry and its task references are
//! released. The probe records the flow instance id, then fails, stalls past the flow TTL, or
//! passes on to a task whose function is not deployed.
//!
//! Its own test binary with its own flow manifest: one platform for the seven cases, and the
//! shared `flows.yaml` untouched.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use event_script::conversions::from_json;
use event_script::FlowExecutor;
use platform_core::{
    main_application, preload, AppError, AutoStart, ComposableFunction, EntryPoint, EventEnvelope,
    Platform, PostOffice,
};

const FLOW_ID: &str = "abort-without-callback-test";
const CALLBACK: &str = "abort.probe.callback";
// a callback route that no function serves, as when the caller's instance has gone
const GONE_CALLBACK: &str = "abort.probe.callback.gone";
const FAIL: &str = "fail";
const STALL: &str = "stall";
const PASS: &str = "pass";
// a short TTL for the timeout cases, and a probe that stalls well past it and past the closing
// deadline. The other cases keep the flow's own TTL of 10 seconds, so only the abort itself can
// close them in time.
const SHORT_TTL_MS: u64 = 500;
const STALL_MS: u64 = 5000;
const CLOSE_DEADLINE: Duration = Duration::from_millis(2500);

/// The flow instance id each launch's probe recorded, by the launch's token.
fn instances() -> &'static Mutex<HashMap<String, String>> {
    static INSTANCES: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
    INSTANCES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The errors the callback received.
fn callbacks() -> &'static Mutex<Vec<serde_json::Value>> {
    static CALLBACKS: OnceLock<Mutex<Vec<serde_json::Value>>> = OnceLock::new();
    CALLBACKS.get_or_init(|| Mutex::new(Vec::new()))
}

#[preload(route = "abort.probe", instances = 10)]
struct AbortProbe;

#[async_trait]
impl ComposableFunction for AbortProbe {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        let token = body["token"].as_str().unwrap_or_default().to_string();
        let instance = body["instance"].as_str().unwrap_or_default().to_string();
        instances()
            .lock()
            .expect("instances")
            .insert(token, instance);
        let mode = body["mode"].as_str().unwrap_or_default().to_string();
        if mode == FAIL {
            return Err(AppError::new(500, "Simulated task failure"));
        }
        if mode == STALL {
            tokio::time::sleep(Duration::from_millis(STALL_MS)).await;
        }
        EventEnvelope::new().set_body(serde_json::json!({"mode": mode}))
    }
}

#[preload(route = "abort.probe.callback", instances = 1)]
struct AbortCallback;

#[async_trait]
impl ComposableFunction for AbortCallback {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        callbacks().lock().expect("callbacks").push(body);
        EventEnvelope::new().set_body(serde_json::Value::Null)
    }
}

#[main_application]
struct AbortTestApp;

#[async_trait]
impl EntryPoint for AbortTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

fn setup_config() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        platform_core::resources::prepend_resource_root("tests/resources");
        platform_core::overrides::set("yaml.flow.automation", "classpath:/abort-flows.yaml");
        let holding = test_support::temp_path("mercury-flow-abort-test");
        platform_core::overrides::set("transient.data.store", &holding.display().to_string());
        let _ = platform_core::AppConfigReader::get_instance();
    });
}

/// Launch the flow and return the flow instance id the probe recorded.
///
/// `ttl_ms` of 0 keeps the flow's own TTL; `callback` of `None` launches with no callback.
async fn launch(platform: &Platform, mode: &str, ttl_ms: u64, callback: Option<&str>) -> String {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let token = format!("token-{n}");
    let cid = format!("abort-cid-{n}");
    let mut dataset = serde_json::json!({"header": {}, "body": {"token": token, "mode": mode}});
    if ttl_ms > 0 {
        dataset["ttl"] = serde_json::json!(ttl_ms);
    }
    match callback {
        None => FlowExecutor::launch(platform, FLOW_ID, from_json(&dataset), &cid, None)
            .await
            .expect("launch"),
        Some(route) => {
            let event = EventEnvelope::new()
                .set_to(event_script::manager::SERVICE_NAME)
                .set_header("flow_id", FLOW_ID)
                .set_header(event_script::manager::BUSINESS_CORRELATION_ID, &cid)
                .set_correlation_id(&format!("abort-launch-{n}"))
                .set_reply_to(route)
                .set_raw_body(from_json(&dataset));
            PostOffice::new(platform).send(event).await.expect("launch");
        }
    }
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(id) = instances().lock().expect("instances").remove(&token) {
            return id;
        }
        assert!(Instant::now() < deadline, "the probe of {mode} never ran");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

/// Wait for the flow instance to leave the flow registry; true when it does before the deadline.
async fn closed_in_time(id: &str) -> bool {
    let deadline = Instant::now() + CLOSE_DEADLINE;
    while Instant::now() < deadline {
        if event_script::flows::get_flow_instance(id).is_none() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    event_script::flows::get_flow_instance(id).is_none()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn an_aborted_flow_is_closed_whether_or_not_its_caller_can_be_told() {
    test_support::run_at_exit(platform_core::util::elastic_queue::shutdown_cleanup);
    setup_config();
    AutoStart::main(vec![]).await.expect("lifecycle");
    let platform = Platform::get_instance();

    // no callback
    let id = launch(&platform, FAIL, 0, None).await;
    assert!(
        closed_in_time(&id).await,
        "a flow that aborts on a task failure must be closed"
    );
    // the probe is still stalling when the deadline passes, so only the timeout can close the flow
    let id = launch(&platform, STALL, SHORT_TTL_MS, None).await;
    assert!(
        closed_in_time(&id).await,
        "a flow that times out must be closed at its ttl"
    );
    let id = launch(&platform, PASS, 0, None).await;
    assert!(
        closed_in_time(&id).await,
        "a flow that aborts on a missing route must be closed"
    );

    // control: the same timeout with a callback, so the failures above come from the missing callback
    callbacks().lock().expect("callbacks").clear();
    let id = launch(&platform, STALL, SHORT_TTL_MS, Some(CALLBACK)).await;
    let deadline = Instant::now() + Duration::from_secs(5);
    let error = loop {
        if let Some(error) = callbacks().lock().expect("callbacks").pop() {
            break error;
        }
        assert!(
            Instant::now() < deadline,
            "the callback must receive the timeout"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    };
    assert_eq!(
        error["status"], 408,
        "the callback must receive status 408: {error}"
    );
    assert!(
        closed_in_time(&id).await,
        "a flow that times out must be closed at its ttl"
    );

    // a callback route that is gone
    let id = launch(&platform, FAIL, 0, Some(GONE_CALLBACK)).await;
    assert!(
        closed_in_time(&id).await,
        "a flow that aborts on a task failure must be closed when its callback is gone"
    );
    let id = launch(&platform, STALL, SHORT_TTL_MS, Some(GONE_CALLBACK)).await;
    assert!(
        closed_in_time(&id).await,
        "a flow that times out must be closed at its ttl when its callback is gone"
    );
    let id = launch(&platform, PASS, 0, Some(GONE_CALLBACK)).await;
    assert!(
        closed_in_time(&id).await,
        "a flow that aborts on a missing route must be closed when its callback is gone"
    );
}
