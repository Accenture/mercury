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

//! End-to-end tests for increment 7 — actuator endpoints (via the default
//! endpoint merge) and static HTML content from `resources/public`.
//! Server-per-test (each `#[tokio::test]` owns its runtime).

use std::collections::HashMap;
use std::sync::{Arc, Once};

use async_trait::async_trait;
use platform_core::actuator::{ActuatorContext, ActuatorKind, ActuatorServices};
use platform_core::{
    automation, overrides, resources, AppConfigReader, AppError, ComposableFunction, EventEnvelope,
    Platform,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// A health-check function honoring the type=info / type=health protocol.
struct DemoHealth {
    healthy: bool,
}

#[async_trait]
impl ComposableFunction for DemoHealth {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        match headers.get("type").map(String::as_str) {
            Some("info") => EventEnvelope::new().set_body(serde_json::json!({
                "service": "demo.store",
                "href": "memory://demo",
            })),
            Some("health") => {
                if self.healthy {
                    EventEnvelope::new().set_body("demo store is running")
                } else {
                    Ok(EventEnvelope::new()
                        .set_status(500)
                        .set_body("demo store is down")?)
                }
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
            std::env::temp_dir().join(format!("mercury-actuator-test-{}", std::process::id()));
        overrides::set("transient.data.store", &holding.display().to_string());
        // a rest.yaml with one app endpoint; actuators come from the default merge
        let rest_file =
            std::env::temp_dir().join(format!("rest-actuator-{}.yaml", std::process::id()));
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
        overrides::set("application.name", "actuator-test");
        overrides::set("info.app.description", "actuator test app");
        overrides::set("show.env.variables", "PC_ACTUATOR_ENV_DEMO");
        overrides::set(
            "show.application.properties",
            "application.name, rest.server.port",
        );
        std::env::set_var("PC_ACTUATOR_ENV_DEMO", "visible-value");
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

/// Start a per-test server with actuators registered (as the lifecycle's
/// essential phase does) and an optional health dependency.
async fn server(healthy_dep: Option<bool>) -> (u16, Platform) {
    setup_config();
    let platform = Platform::new();
    platform.register("noop.demo", Arc::new(Noop), 1).unwrap();
    let context = ActuatorContext::new(&platform);
    for (route, kind) in [
        (platform_core::actuator::INFO_ACTUATOR, ActuatorKind::Info),
        (platform_core::actuator::ENV_ACTUATOR, ActuatorKind::Env),
        (
            platform_core::actuator::HEALTH_ACTUATOR,
            ActuatorKind::Health,
        ),
        (
            platform_core::actuator::LIVENESS_ACTUATOR,
            ActuatorKind::Liveness,
        ),
    ] {
        platform
            .register(
                route,
                Arc::new(ActuatorServices::new(kind, context.clone())),
                1,
            )
            .unwrap();
    }
    if let Some(healthy) = healthy_dep {
        platform
            .register("demo.health", Arc::new(DemoHealth { healthy }), 1)
            .unwrap();
    }
    let addr = automation::start_http_server(&platform).await.unwrap();
    (addr.port(), platform)
}

async fn http_get(port: u16, path: &str) -> (u16, HashMap<String, String>, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let request = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).await.expect("read");
    let text = String::from_utf8_lossy(&raw).to_string();
    let (head, payload) = text.split_once("\r\n\r\n").unwrap_or((text.as_str(), ""));
    let mut lines = head.lines();
    let status: u16 = lines
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap_or_else(|| panic!("no status in: {text:?}"));
    let mut headers = HashMap::new();
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            headers.insert(name.trim().to_lowercase(), value.trim().to_string());
        }
    }
    (status, headers, payload.to_string())
}

/// The three health tests share the `mandatory.health.dependencies` override
/// (read at ActuatorContext construction) — serialize them.
static HEALTH_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

// ---- actuator endpoints (default merge puts them on the server) ----

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn info_reports_app_identity_and_uptime() {
    let (port, _platform) = server(None).await;
    let (status, headers, body) = http_get(port, "/info").await;
    assert_eq!(status, 200);
    assert_eq!(headers["content-type"], "application/json");
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["app"]["name"], "actuator-test");
    assert_eq!(json["app"]["description"], "actuator test app");
    assert_eq!(json["runtime"]["language"], "rust");
    assert!(json["origin"].as_str().is_some_and(|o| !o.is_empty()));
    assert!(json["up_time"]
        .as_str()
        .is_some_and(|u| u.contains("second")));
    assert!(json["time"]["start"]
        .as_str()
        .is_some_and(|t| t.ends_with('Z')));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn env_shows_only_selected_values() {
    let (port, _platform) = server(None).await;
    let (status, _, body) = http_get(port, "/env").await;
    assert_eq!(status, 200);
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(
        json["env"]["environment"]["PC_ACTUATOR_ENV_DEMO"],
        "visible-value"
    );
    assert_eq!(
        json["env"]["properties"]["application.name"],
        "actuator-test"
    );
    // opt-in only: nothing else is dumped
    assert_eq!(json["env"]["environment"].as_object().unwrap().len(), 1);
    assert_eq!(json["env"]["properties"].as_object().unwrap().len(), 2);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn liveness_is_ok_by_default() {
    let (port, _platform) = server(None).await;
    let (status, headers, body) = http_get(port, "/livenessprobe").await;
    assert_eq!(status, 200);
    assert_eq!(headers["content-type"], "text/plain");
    assert_eq!(body, "OK");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn health_without_dependencies_hints_configuration() {
    let _guard = HEALTH_LOCK.lock().await;
    let (port, _platform) = server(None).await;
    let (status, _, body) = http_get(port, "/health").await;
    assert_eq!(status, 200);
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["status"], "UP");
    assert!(json["message"].as_str().unwrap().contains("Did you forget"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn health_up_with_mandatory_dependency() {
    let _guard = HEALTH_LOCK.lock().await;
    overrides::set("mandatory.health.dependencies", "demo.health");
    let (port, _platform) = server(Some(true)).await;
    let (status, _, body) = http_get(port, "/health").await;
    overrides::clear("mandatory.health.dependencies");
    assert_eq!(status, 200);
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["status"], "UP");
    let dep = &json["dependency"][0];
    assert_eq!(dep["route"], "demo.health");
    assert_eq!(dep["required"], true);
    assert_eq!(dep["status_code"], 200);
    assert_eq!(dep["message"], "demo store is running");
    assert_eq!(dep["service"], "demo.store"); // merged from type=info
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn health_down_flips_liveness() {
    let _guard = HEALTH_LOCK.lock().await;
    overrides::set("mandatory.health.dependencies", "demo.health");
    let (port, _platform) = server(Some(false)).await;
    let (status, _, body) = http_get(port, "/health").await;
    overrides::clear("mandatory.health.dependencies");
    assert_eq!(status, 400); // Java parity: DOWN = 400
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["status"], "DOWN");
    assert_eq!(json["dependency"][0]["status_code"], 500);
    // the failed health check flips the liveness probe
    let (status, _, body) = http_get(port, "/livenessprobe").await;
    assert_eq!(status, 400);
    assert!(body.contains("Unhealthy"));
}

// ---- static HTML content from resources/public ----

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn root_serves_index_html_when_rest_yaml_has_no_root() {
    let (port, _platform) = server(None).await;
    let (status, headers, body) = http_get(port, "/").await;
    assert_eq!(status, 200);
    assert_eq!(headers["content-type"], "text/html");
    assert!(body.contains("static-index-marker"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn nested_static_asset_with_content_type() {
    let (port, _platform) = server(None).await;
    let (status, headers, body) = http_get(port, "/assets/style.css").await;
    assert_eq!(status, 200);
    assert_eq!(headers["content-type"], "text/css");
    assert!(body.contains("static-css-marker"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn static_misses_and_traversal_are_404() {
    let (port, _platform) = server(None).await;
    let (status, _, _) = http_get(port, "/no-such-page.html").await;
    assert_eq!(status, 404);
    let (status, _, _) = http_get(port, "/../memory/continuity.md").await;
    assert_eq!(status, 404);
    // POST never serves static content
    let (status, _, _) = {
        let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
            .await
            .unwrap();
        stream
            .write_all(
                b"POST / HTTP/1.1\r\nHost: x\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            )
            .await
            .unwrap();
        let mut raw = Vec::new();
        stream.read_to_end(&mut raw).await.unwrap();
        let text = String::from_utf8_lossy(&raw).to_string();
        let code: u16 = text
            .lines()
            .next()
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|c| c.parse().ok())
            .unwrap();
        (code, (), ())
    };
    assert_eq!(status, 404);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rest_yaml_endpoints_still_win_over_defaults_and_static() {
    let (port, _platform) = server(None).await;
    // the app's own endpoint works alongside the merged defaults
    let (status, _, body) = http_get(port, "/api/noop").await;
    assert_eq!(status, 200);
    assert_eq!(body, "ok");
}
