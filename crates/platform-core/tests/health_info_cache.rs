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

//! Increment 71: the `/health` per-dependency info-lookup cache (Java parity —
//! `ActuatorServices`' `SimpleCache("health.info", 5000)`, mapped onto
//! `ManagedCache` per the maintainer's one-cache-type ruling;
//! `draft-design-specs/managed-cache-port.md` §6.2).
//!
//! Isolated in its OWN test binary on purpose: the `health.info` cache is
//! process-wide, and the other actuator tests all register their dependency
//! at the shared `demo.health` route — sharing a binary would bleed cached
//! info entries across tests in both directions. Here the config freeze and
//! the dependency route belong to this binary alone.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Once};

use async_trait::async_trait;
use platform_core::actuator::{ActuatorContext, ActuatorKind, ActuatorServices};
use platform_core::{
    automation, overrides, resources, AppConfigReader, AppError, ComposableFunction, EventEnvelope,
    Platform,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

static INFO_CALLS: AtomicUsize = AtomicUsize::new(0);
static HEALTH_CALLS: AtomicUsize = AtomicUsize::new(0);

/// A health-check dependency that counts how often each protocol arm is hit.
struct CountingHealth;

#[async_trait]
impl ComposableFunction for CountingHealth {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        match headers.get("type").map(String::as_str) {
            Some("info") => {
                INFO_CALLS.fetch_add(1, Ordering::SeqCst);
                EventEnvelope::new().set_body(serde_json::json!({
                    "service": "counting.store",
                    "href": "memory://counting",
                }))
            }
            Some("health") => {
                HEALTH_CALLS.fetch_add(1, Ordering::SeqCst);
                EventEnvelope::new().set_body("counting store is running")
            }
            _ => Err(AppError::new(400, "unknown type")),
        }
    }
}

fn setup_config() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let holding =
            std::env::temp_dir().join(format!("mercury-health-cache-test-{}", std::process::id()));
        overrides::set("transient.data.store", &holding.display().to_string());
        let rest_file =
            std::env::temp_dir().join(format!("rest-health-cache-{}.yaml", std::process::id()));
        std::fs::write(
            &rest_file,
            "rest:\n  - service: \"noop.demo\"\n    methods: ['GET']\n    url: \"/api/noop\"\n",
        )
        .unwrap();
        overrides::set(
            "yaml.rest.automation",
            &format!("file:{}", rest_file.display()),
        );
        overrides::set("rest.server.port", "0");
        overrides::set("application.name", "health-info-cache-test");
        // this binary's OWN dependency route (read at ActuatorContext
        // construction) — never the shared demo.health
        overrides::set("mandatory.health.dependencies", "counting.health.dep");
        let _ = AppConfigReader::get_instance();
    });
}

struct Noop;

#[async_trait]
impl ComposableFunction for Noop {
    async fn handle_event(
        &self,
        _h: HashMap<String, String>,
        _i: EventEnvelope,
        _n: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new().set_body("ok")
    }
}

async fn server() -> (u16, Platform) {
    setup_config();
    let platform = Platform::new();
    platform.register("noop.demo", Arc::new(Noop), 1).unwrap();
    platform
        .register("counting.health.dep", Arc::new(CountingHealth), 1)
        .unwrap();
    let context = ActuatorContext::new(&platform);
    platform
        .register(
            platform_core::actuator::HEALTH_ACTUATOR,
            Arc::new(ActuatorServices::new(ActuatorKind::Health, context)),
            1,
        )
        .unwrap();
    let addr = automation::start_http_server(&platform).await.unwrap();
    (addr.port(), platform)
}

async fn http_get(port: u16, path: &str) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let request = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).await.expect("read");
    let text = String::from_utf8_lossy(&raw).to_string();
    let (head, payload) = text.split_once("\r\n\r\n").unwrap_or((text.as_str(), ""));
    let status: u16 = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap_or_else(|| panic!("no status in: {text:?}"));
    (status, payload.to_string())
}

/// Java `isServiceUnhealthy` parity: the per-dependency `type=info` lookup is
/// served from the 5 s `health.info` cache on the second `/health` call, while
/// `type=health` re-runs on EVERY call — Java never caches the health probe
/// (nor the /health result itself).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn info_lookup_is_cached_health_is_not() {
    let (port, _platform) = server().await;
    let (status_first, body_first) = http_get(port, "/health").await;
    assert_eq!(status_first, 200);
    let (status_second, body_second) = http_get(port, "/health").await;
    assert_eq!(status_second, 200);
    assert_eq!(
        INFO_CALLS.load(Ordering::SeqCst),
        1,
        "the second /health must serve type=info from the 5 s cache"
    );
    assert_eq!(
        HEALTH_CALLS.load(Ordering::SeqCst),
        2,
        "type=health runs on every call - the health outcome is never cached"
    );
    // the cached info map still merges into the dependency entry on BOTH calls
    for body in [&body_first, &body_second] {
        let json: serde_json::Value = serde_json::from_str(body).unwrap();
        let dep = &json["dependency"][0];
        assert_eq!(dep["route"], "counting.health.dep");
        assert_eq!(dep["required"], true);
        assert_eq!(dep["service"], "counting.store");
        assert_eq!(dep["href"], "memory://counting");
        assert_eq!(dep["status_code"], 200);
        assert_eq!(dep["message"], "counting store is running");
    }
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&body_second).unwrap()["status"],
        "UP"
    );
}
