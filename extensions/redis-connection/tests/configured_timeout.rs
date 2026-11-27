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

//! `redis.timeout.ms` is the one deadline of a command, as the Java
//! `RedisConfig` makes it Lettuce's command timeout (`RedisURI.withTimeout`): a
//! reply within it is answered however slow, and a reply beyond it is a 408 at
//! that time. The `redis` crate has timeouts of its own - 500 ms for a response,
//! one second for a connection attempt - which would answer first unless the
//! foundation set them from the configured value, on the managed connection and
//! on the single one alike. Driven through the relay of the RESP double, which
//! holds the server's replies back.

use std::time::{Duration, Instant};

use redis_connection::{RedisBackend, RedisConfig};
use redis_test_double::{start_resp_double, BounceProxy};

/// Longer than the `redis` crate's own response timeout, shorter than the configured one.
const SLOW_REPLY: Duration = Duration::from_millis(900);

async fn proxied(timeout_ms: u64) -> (RedisConfig, BounceProxy) {
    let (redis_port, _store, _journal) = start_resp_double("7.4.0").await;
    let proxy = BounceProxy::start(redis_port).await;
    // no heartbeat: the only traffic is the test's own commands
    let config =
        RedisConfig::new("127.0.0.1", proxy.port(), "", false, 0, timeout_ms).with_heartbeat(0);
    (config, proxy)
}

async fn written(backend: &RedisBackend) {
    backend
        .query_idempotent::<String>(redis::cmd("SETEX").arg("k").arg(60).arg("v"))
        .await
        .expect("a write on the unhindered connection");
}

/// The managed connection (the cache's, the store's): a reply that takes longer than
/// the crate's 500 ms and less than the configured 4 seconds is answered.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_slow_reply_within_the_configured_timeout_is_answered() {
    let (config, proxy) = proxied(4000).await;
    let backend = RedisBackend::connect_standalone(&config)
        .await
        .expect("backend connects");
    written(&backend).await;

    proxy.delay_replies(SLOW_REPLY);
    let started = Instant::now();
    let value: Option<String> = backend
        .query(redis::cmd("GET").arg("k"))
        .await
        .expect("the configured timeout bounds the reply, not the client library's default");
    assert_eq!(Some("v".to_string()), value);
    assert!(
        started.elapsed() >= SLOW_REPLY,
        "the reply was held back: {:?}",
        started.elapsed()
    );
    // a slow reply is no lifecycle signal
    let lifecycle = backend.lifecycle();
    assert!(lifecycle.healthy());
    assert_eq!(0, lifecycle.drops());
    assert_eq!(0, lifecycle.retries());
}

/// A reply beyond the configured timeout is a 408 when that time has passed - neither
/// earlier, at the crate's default, nor in the crate's words.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_reply_beyond_the_configured_timeout_is_a_408_at_that_time() {
    let (config, proxy) = proxied(1200).await;
    let backend = RedisBackend::connect_standalone(&config)
        .await
        .expect("backend connects");
    written(&backend).await;

    proxy.delay_replies(Duration::from_millis(3000));
    let started = Instant::now();
    let error = backend
        .query::<Option<String>>(redis::cmd("GET").arg("k"))
        .await
        .expect_err("no reply within the configured timeout");
    let elapsed = started.elapsed();
    assert_eq!(408, error.status(), "{}", error.message());
    assert_eq!("Redis request timed out after 1200ms", error.message());
    assert!(
        elapsed >= Duration::from_millis(1200) && elapsed < Duration::from_millis(2500),
        "the configured timeout is the deadline: {elapsed:?}"
    );
    // a timeout is not a lost connection: no retry, no health change
    let lifecycle = backend.lifecycle();
    assert!(lifecycle.healthy());
    assert_eq!(0, lifecycle.retries());
}

/// The single, non-reconnecting connection (the health probe's shape) takes the
/// configured timeout too.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_single_connection_takes_the_configured_timeout_too() {
    let (config, proxy) = proxied(4000).await;
    let backend = RedisBackend::connect_once(&config)
        .await
        .expect("backend connects");
    written(&backend).await;

    proxy.delay_replies(SLOW_REPLY);
    let value: Option<String> = backend
        .query(redis::cmd("GET").arg("k"))
        .await
        .expect("the configured timeout bounds the reply on the single connection");
    assert_eq!(Some("v".to_string()), value);
}

/// The manager a consumer takes from the configuration (`RedisConfig::manager`, for a
/// consumer that issues its own commands) is built with the configured timeout as well.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_manager_from_the_configuration_takes_the_configured_timeout() {
    let (config, proxy) = proxied(4000).await;
    let mut manager = config.manager().await.expect("manager connects");
    let _: () = redis::cmd("SETEX")
        .arg("k")
        .arg(60)
        .arg("v")
        .query_async(&mut manager)
        .await
        .expect("a write on the unhindered connection");

    proxy.delay_replies(SLOW_REPLY);
    let value: Option<String> = redis::cmd("GET")
        .arg("k")
        .query_async(&mut manager)
        .await
        .expect("the configured timeout bounds the reply on a manager used directly");
    assert_eq!(Some("v".to_string()), value);
}
