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

//! The `soa.redis.health` contract end to end against the in-process double
//! (Java `RedisHealthCheckTest` + `RedisHealthCheckLazyConfigTest` +
//! `RedisHealthCheckAuthTest` twins): the info shape, the live probe, the
//! `{text, code}` 503 on a genuine outage, the start-up placeholder with the
//! background warm-up, and the late-credential vault pattern — a probe whose
//! `redis.password` has not been published yet is a passing "waiting" status,
//! never a failed `/health`, and the check goes live on the first probe after
//! the credential lands, with no restart in between.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use platform_core::{ComposableFunction, EventEnvelope};
use redis_test_double::{start_resp_double, start_resp_double_with_password};
use sync_over_async::{RedisHealthCheck, RedisSettings};

const PASSWORD: &str = "local-test-secret";

fn settings(port: u16, password: &str) -> RedisSettings {
    RedisSettings::new("127.0.0.1", port, password, false, 0, 2000)
}

async fn call(check: &RedisHealthCheck, kind: &str) -> EventEnvelope {
    let headers = HashMap::from([("type".to_string(), kind.to_string())]);
    check
        .handle_event(headers, EventEnvelope::new(), 1)
        .await
        .expect("health function replies")
}

fn body(event: &EventEnvelope) -> serde_json::Value {
    event.body_as().expect("json body")
}

fn status_of(event: &EventEnvelope) -> String {
    body(event)["status"]
        .as_str()
        .unwrap_or_default()
        .to_string()
}

/// Poll the health type until the predicate holds (the warm-up and the
/// late-credential scenarios heal asynchronously or on the next probe).
async fn await_status(check: &RedisHealthCheck, expected: &str) -> EventEnvelope {
    for _ in 0..500 {
        let reply = call(check, "health").await;
        if status_of(&reply) == expected {
            return reply;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("health status never became '{expected}'");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn info_reports_service_and_href() {
    let (port, _store, _journal) = start_resp_double("7.4.0").await;
    let check = RedisHealthCheck::new(
        move || settings(port, ""),
        Duration::from_secs(2),
        Duration::ZERO,
    );
    let reply = call(&check, "info").await;
    let info = body(&reply);
    assert_eq!("redis", info["service"]);
    assert_eq!(format!("127.0.0.1:{port}"), info["href"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn reachable_server_reports_healthy() {
    let (port, _store, _journal) = start_resp_double("7.4.0").await;
    let check = RedisHealthCheck::new(
        move || settings(port, ""),
        Duration::from_secs(2),
        Duration::ZERO, // no grace - probe live immediately
    );
    let reply = call(&check, "health").await;
    assert!(!reply.has_error());
    let healthy = body(&reply);
    assert_eq!("Redis is reachable", healthy["status"]);
    assert_eq!(format!("127.0.0.1:{port}"), healthy["href"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn outage_fails_health_with_text_and_code() {
    // bind-and-drop: a port that answers connection-refused
    let dead_port = {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
        listener.local_addr().expect("addr").port()
    };
    let check = RedisHealthCheck::new(
        move || settings(dead_port, ""),
        Duration::from_secs(2),
        Duration::ZERO,
    );
    let reply = call(&check, "health").await;
    // the 503 status is what the health aggregation (and Kubernetes) detects;
    // the key-value body keeps the code visible to the DevOps reader too
    assert_eq!(503, reply.status());
    let down = body(&reply);
    assert_eq!(503, down["code"]);
    let text = down["text"].as_str().unwrap_or_default();
    assert!(
        text.starts_with("Redis is not reachable - "),
        "unexpected text: {text}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn startup_grace_reports_placeholder_then_warms_up() {
    let (port, _store, _journal) = start_resp_double("7.4.0").await;
    let check = RedisHealthCheck::new(
        move || settings(port, ""),
        Duration::from_secs(2),
        Duration::from_secs(30), // generous grace - the placeholder window
    );
    // during the grace period the check must not fail (or block) while the
    // application start-up sequence is still coming up
    let first = call(&check, "health").await;
    assert!(!first.has_error());
    assert_eq!("Redis client is starting up", status_of(&first));
    // the background warm-up goes live without waiting out the grace period
    let live = await_status(&check, "Redis is reachable").await;
    assert!(!live.has_error());
}

/// The late-credential scenario end to end, against a double that REQUIRES a
/// password (`requirepass`): a probe whose `redis.password` has not been
/// published yet is a passing "waiting" status — never a failed `/health` —
/// and the check goes live on the first probe after the credential lands,
/// with no restart in between. This is the vault-bootstrap pattern: the
/// application fetches secrets and publishes them long after this function
/// was constructed at startup registration time.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn late_credential_waits_then_goes_live_without_restart() {
    let (port, _store, _journal) = start_resp_double_with_password("7.4.0", PASSWORD).await;
    let vault: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let supplier_vault = vault.clone();
    let check = RedisHealthCheck::new(
        move || settings(port, &supplier_vault.lock().expect("vault").clone()),
        Duration::from_secs(2),
        Duration::ZERO,
    );

    // no credential published yet: the server answers NOAUTH - a passing
    // waiting status, because a pod restart cannot produce the password
    let waiting = call(&check, "health").await;
    assert!(!waiting.has_error(), "waiting must never fail /health");
    assert_eq!("Waiting for Redis connection", status_of(&waiting));

    // a wrong credential (stale vault value) is the same waiting signature
    *vault.lock().expect("vault") = "stale-secret".to_string();
    let wrong = call(&check, "health").await;
    assert!(!wrong.has_error());
    assert_eq!("Waiting for Redis connection", status_of(&wrong));

    // the credential lands: the next probe re-resolves the configuration and
    // the check heals itself - no restart in between
    *vault.lock().expect("vault") = PASSWORD.to_string();
    let live = call(&check, "health").await;
    assert!(!live.has_error());
    assert_eq!("Redis is reachable", status_of(&live));
}

/// The third waiting signature: a password presented to a server that wants
/// none (`ERR Client sent AUTH`) — a misconfiguration to heal, not an outage.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn password_against_open_server_is_waiting_not_outage() {
    let (port, _store, _journal) = start_resp_double("7.4.0").await;
    let check = RedisHealthCheck::new(
        move || settings(port, "unnecessary-secret"),
        Duration::from_secs(2),
        Duration::ZERO,
    );
    let reply = call(&check, "health").await;
    assert!(!reply.has_error());
    assert_eq!("Waiting for Redis connection", status_of(&reply));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unknown_type_is_a_client_error() {
    let (port, _store, _journal) = start_resp_double("7.4.0").await;
    let check = RedisHealthCheck::new(
        move || settings(port, ""),
        Duration::from_secs(2),
        Duration::ZERO,
    );
    let headers = HashMap::from([("type".to_string(), "bogus".to_string())]);
    let error = check
        .handle_event(headers, EventEnvelope::new(), 1)
        .await
        .expect_err("bogus type refused");
    assert_eq!(400, error.status());
}
