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

//! Declarative Event over HTTP (increment 62) — the Rust twin of the Java
//! `EventHttpTest.configTest` + `declarativeEventOverHttpTest`: routes listed
//! in `yaml.event.over.http` forward transparently over `/api/event`, so user
//! code with ZERO http-awareness reaches a remote function and gets its reply.
//!
//! Everything runs in ONE test function: the config registry is a
//! process-wide one-shot load whose `${server.port}` reference must resolve
//! AFTER the ephemeral test server's port is known — a single function makes
//! that sequencing deterministic (parallel test functions each boot their own
//! runtime-bound server, which a shared static registry cannot follow).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::automation::http_client::AsyncHttpClientService;
use platform_core::automation::{self, get_event_http_target, EventApiService};
use platform_core::{
    overrides, resources, AppConfigReader, AppError, ComposableFunction, EventEnvelope,
    FunctionOptions, Platform, PostOffice,
};
use rmpv::Value;

/// The remote store (Java EventHttpTest's `event.save.get` fixture): header
/// `type=save` stores the body and replies "saved"; `type=get` replies with
/// the stored value. Registered PUBLIC — it is the Event-over-HTTP target.
struct SaveGet {
    store: Mutex<Option<Value>>,
}

#[async_trait]
impl ComposableFunction for SaveGet {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        match headers.get("type").map(String::as_str) {
            Some("save") => {
                *self.store.lock().expect("store") = Some(input.body().clone());
                EventEnvelope::new().set_body("saved")
            }
            Some("get") => {
                let stored = self
                    .store
                    .lock()
                    .expect("store")
                    .clone()
                    .unwrap_or(Value::Nil);
                Ok(EventEnvelope::new().set_raw_body(stored))
            }
            _ => Err(AppError::new(400, "unknown type")),
        }
    }
}

/// The local callback target (Java's `blocking.event.wait`): captures the
/// body of the event delivered to it.
struct CaptureCallback {
    tx: tokio::sync::mpsc::UnboundedSender<Value>,
}

#[async_trait]
impl ComposableFunction for CaptureCallback {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let _ = self.tx.send(input.body().clone());
        Ok(EventEnvelope::new())
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn declarative_event_over_http() {
    resources::prepend_resource_root("tests/resources");
    let holding = std::env::temp_dir().join(format!("mercury-eoh-decl-{}", std::process::id()));
    overrides::set("transient.data.store", &holding.display().to_string());
    overrides::set("rest.server.port", "0");
    // a minimal rest.yaml; /api/event arrives via the default-endpoint merge
    overrides::set("yaml.rest.automation", "classpath:/event-rest.yaml");
    overrides::set("yaml.event.over.http", "classpath:/event-over-http.yaml");
    let _ = AppConfigReader::get_instance();
    let platform = Platform::new();
    platform
        .register_private(
            automation::EVENT_API_SERVICE,
            Arc::new(EventApiService::new(&platform)),
            10,
        )
        .unwrap();
    // the PUBLIC remote target of the declarative routes
    platform
        .register(
            "event.save.get",
            Arc::new(SaveGet {
                store: Mutex::new(None),
            }),
            2,
        )
        .unwrap();
    platform
        .register_with_options(
            automation::ASYNC_HTTP_REQUEST,
            Arc::new(AsyncHttpClientService::new(&platform)),
            10,
            FunctionOptions {
                interceptor: true,
                private: true,
                ..FunctionOptions::default()
            },
        )
        .unwrap();
    let addr = automation::start_http_server(&platform).await.unwrap();
    // the fixture's ${server.port} resolves when the registry first loads —
    // set the port BEFORE the first declarative lookup below
    overrides::set("server.port", &addr.port().to_string());

    // ---- config load (Java EventHttpTest.configTest) ----
    let entry = get_event_http_target("event.http.test").expect("configured route");
    assert_eq!(
        entry.target,
        format!("http://127.0.0.1:{}/api/event", addr.port())
    );
    assert_eq!(
        entry.headers.get("authorization").map(String::as_str),
        Some("demo")
    );
    // an @instance suffix strips for the lookup (Java getEventHttpTarget)
    assert!(get_event_http_target("event.save.get@2").is_some());
    assert!(get_event_http_target("no.such.route").is_none());

    // ---- declarative round trip (Java declarativeEventOverHttpTest):
    // user code below is pure PostOffice — zero http-awareness ----
    let po = PostOffice::new(&platform);
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    platform
        .register_private("blocking.event.wait", Arc::new(CaptureCallback { tx }), 1)
        .unwrap();
    // send with a reply_to = the callback dance: forwarded over HTTP as RPC,
    // the peer's response delivered locally to blocking.event.wait
    po.send(
        EventEnvelope::new()
            .set_to("event.save.get")
            .set_header("type", "save")
            .set_reply_to("blocking.event.wait")
            .set_body("hello")
            .unwrap(),
    )
    .await
    .expect("declarative send");
    let callback_body = tokio::time::timeout(Duration::from_secs(5), rx.recv())
        .await
        .expect("callback within 5s")
        .expect("callback value");
    assert_eq!(callback_body.as_str(), Some("saved"));

    // request() to the configured route returns the peer's reply directly
    let response = po
        .request(
            EventEnvelope::new()
                .set_to("event.save.get")
                .set_header("type", "get"),
            Duration::from_secs(10),
        )
        .await
        .expect("declarative request");
    assert_eq!(response.body().as_str(), Some("hello"));
}
