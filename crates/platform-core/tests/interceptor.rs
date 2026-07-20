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

//! Increment E-3 — the event-interceptor registration mode (Java
//! `@EventInterceptor`) and scheduled events (`send_later` / cancel), built
//! for the event-script engine (manager/executor/adapters are interceptors;
//! the flow TTL watcher is a scheduled event).

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::platform::FunctionOptions;
use platform_core::{
    overrides, resources, AppConfigReader, AppError, ComposableFunction, EventEnvelope, Platform,
    PostOffice,
};

fn setup_config() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let holding =
            std::env::temp_dir().join(format!("mercury-interceptor-test-{}", std::process::id()));
        overrides::set("transient.data.store", &holding.display().to_string());
        let _ = AppConfigReader::get_instance();
    });
}

/// A manual-replying interceptor: reads `reply_to`/`cid` from the raw
/// envelope (they stay intact — the Java contract) and answers via po.send.
struct ManualReplier {
    platform: Platform,
}

#[async_trait]
impl ComposableFunction for ManualReplier {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if let (Some(reply_to), Some(cid)) = (input.reply_to(), input.correlation_id()) {
            let po = PostOffice::new(&self.platform);
            let reply = EventEnvelope::new()
                .set_to(reply_to)
                .set_correlation_id(cid)
                .set_body("manual reply")?;
            po.send(reply).await?;
        }
        // an interceptor's return value must be IGNORED by the worker
        EventEnvelope::new().set_body("this must never reach the caller")
    }
}

/// An interceptor that never replies — proves the worker sends no auto-reply.
struct SilentSink {
    calls: Arc<Mutex<u32>>,
}

#[async_trait]
impl ComposableFunction for SilentSink {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        *self.calls.lock().expect("calls") += 1;
        EventEnvelope::new().set_body("ignored")
    }
}

/// A failing interceptor — a FAILURE still routes to reply_to (Java
/// WorkerHandler: only the success reply is interceptor-guarded).
struct Failing;

#[async_trait]
impl ComposableFunction for Failing {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Err(AppError::new(409, "interceptor failure"))
    }
}

/// Captures delivered event bodies (for the scheduler tests).
struct Capture {
    seen: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl ComposableFunction for Capture {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        self.seen
            .lock()
            .expect("seen")
            .push(input.body_as::<String>().unwrap_or_default());
        Ok(EventEnvelope::new())
    }
}

const INTERCEPTOR: FunctionOptions = FunctionOptions {
    zero_traced: false,
    interceptor: true,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn interceptor_replies_manually_and_return_value_is_ignored() {
    setup_config();
    let platform = Platform::new();
    platform
        .register_with_options(
            "manual.replier",
            Arc::new(ManualReplier {
                platform: platform.clone(),
            }),
            1,
            INTERCEPTOR,
        )
        .unwrap();
    let po = PostOffice::new(&platform);
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("manual.replier")
                .set_body("ping")
                .unwrap(),
            Duration::from_secs(2),
        )
        .await
        .expect("manual reply");
    // the manual reply arrives; the returned envelope never does
    assert_eq!(reply.body_as::<String>().unwrap(), "manual reply");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn interceptor_success_sends_no_auto_reply() {
    setup_config();
    let platform = Platform::new();
    let calls = Arc::new(Mutex::new(0));
    platform
        .register_with_options(
            "silent.sink",
            Arc::new(SilentSink {
                calls: calls.clone(),
            }),
            1,
            INTERCEPTOR,
        )
        .unwrap();
    let po = PostOffice::new(&platform);
    let result = po
        .request(
            EventEnvelope::new()
                .set_to("silent.sink")
                .set_body("ping")
                .unwrap(),
            Duration::from_millis(500),
        )
        .await;
    // the function ran, but no auto-reply came back: the RPC times out (408)
    assert_eq!(*calls.lock().expect("calls"), 1);
    let err = result.expect_err("no auto-reply expected");
    assert_eq!(err.status(), 408);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn interceptor_failure_still_reaches_the_caller() {
    setup_config();
    let platform = Platform::new();
    platform
        .register_with_options("failing.interceptor", Arc::new(Failing), 1, INTERCEPTOR)
        .unwrap();
    let po = PostOffice::new(&platform);
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("failing.interceptor")
                .set_body("ping")
                .unwrap(),
            Duration::from_secs(2),
        )
        .await
        .expect("error envelope expected");
    assert_eq!(reply.status(), 409);
    assert!(reply.has_error());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn send_later_delivers_after_the_delay() {
    setup_config();
    let platform = Platform::new();
    let seen = Arc::new(Mutex::new(Vec::new()));
    platform
        .register("timer.capture", Arc::new(Capture { seen: seen.clone() }), 1)
        .unwrap();
    let po = PostOffice::new(&platform);
    po.send_later(
        EventEnvelope::new()
            .set_to("timer.capture")
            .set_body("delayed")
            .unwrap(),
        Duration::from_millis(100),
    );
    // not yet delivered
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert!(seen.lock().expect("seen").is_empty());
    // delivered after the delay
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert_eq!(seen.lock().expect("seen").clone(), vec!["delayed"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancel_future_event_stops_delivery() {
    setup_config();
    let platform = Platform::new();
    let seen = Arc::new(Mutex::new(Vec::new()));
    platform
        .register(
            "timer.cancel.capture",
            Arc::new(Capture { seen: seen.clone() }),
            1,
        )
        .unwrap();
    let po = PostOffice::new(&platform);
    let timer_id = po.send_later(
        EventEnvelope::new()
            .set_to("timer.cancel.capture")
            .set_body("never")
            .unwrap(),
        Duration::from_millis(150),
    );
    assert!(po.cancel_future_event(&timer_id));
    // a second cancel reports the timer is gone
    assert!(!po.cancel_future_event(&timer_id));
    tokio::time::sleep(Duration::from_millis(400)).await;
    assert!(seen.lock().expect("seen").is_empty());
    // cancelling an unknown id is a no-op
    assert!(!po.cancel_future_event("no-such-timer"));
}
