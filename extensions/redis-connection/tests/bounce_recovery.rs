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

//! The restart-aware retry (`backend` module docs, *Lifecycle*), driven through
//! a severable relay in front of the RESP double: a **bounce** severs every
//! connection with the server back at once (a Redis restart), a **refusal**
//! severs them and refuses reconnects (an outage).

use std::time::Duration;

use redis_connection::{is_connection_loss, RedisBackend, RedisConfig};
use redis_test_double::{start_resp_double, BounceProxy};

/// A standalone backend through the relay; `heartbeat_ms = 0` switches the
/// monitor off so a test can meet the lost connection with a command.
async fn backend_through_proxy(timeout_ms: u64, heartbeat_ms: u64) -> (RedisBackend, BounceProxy) {
    let (redis_port, _store, _journal) = start_resp_double("7.4.0").await;
    let proxy = BounceProxy::start(redis_port).await;
    let config = RedisConfig::new("127.0.0.1", proxy.port(), "", false, 0, timeout_ms)
        .with_heartbeat(heartbeat_ms);
    let backend = RedisBackend::connect_standalone(&config)
        .await
        .expect("backend connects");
    (backend, proxy)
}

async fn wait_until(deadline: Duration, condition: impl Fn() -> bool) -> bool {
    let started = std::time::Instant::now();
    while started.elapsed() < deadline {
        if condition() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    condition()
}

/// An idempotent command that meets the lost connection is the transition
/// itself: retried exactly once, on the manager's fresh connection.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn idempotent_command_heals_across_a_bounce_with_one_retry() {
    let (backend, proxy) = backend_through_proxy(5000, 0).await;
    backend
        .query_idempotent::<String>(redis::cmd("SETEX").arg("k").arg(60).arg("v"))
        .await
        .expect("healthy write");
    assert!(backend.lifecycle().healthy());

    proxy.bounce().await;
    let value: Option<String> = backend
        .query_idempotent(redis::cmd("GET").arg("k"))
        .await
        .expect("GET heals on its own retry");
    assert_eq!(Some("v".to_string()), value);
    let lifecycle = backend.lifecycle();
    assert_eq!(1, lifecycle.drops(), "one transition observed");
    assert_eq!(1, lifecycle.retries(), "exactly one second attempt");
    assert_eq!(1, lifecycle.recoveries());
    assert!(lifecycle.healthy());
}

/// A non-idempotent command is never replayed: it fails on the bounced
/// connection (503 - Redis unavailable), and the caller's own next call lands
/// on the healed connection. Nothing was pushed twice.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn non_idempotent_command_is_not_replayed() {
    let (backend, proxy) = backend_through_proxy(5000, 0).await;
    let length: i64 = backend
        .query(redis::cmd("RPUSH").arg("list").arg("one"))
        .await
        .expect("healthy push");
    assert_eq!(1, length);

    proxy.bounce().await;
    let failure = backend
        .query::<i64>(redis::cmd("RPUSH").arg("list").arg("two"))
        .await
        .expect_err("a non-idempotent command fails on the bounced connection");
    assert_eq!(503, failure.status(), "{}", failure.message());
    assert!(failure.message().starts_with("Redis unavailable"));
    assert_eq!(0, backend.lifecycle().retries(), "never replayed");
    assert!(!backend.lifecycle().healthy());

    let length: i64 = backend
        .query(redis::cmd("RPUSH").arg("list").arg("two"))
        .await
        .expect("the caller's own retry lands on the healed connection");
    assert_eq!(2, length, "the failed attempt pushed nothing");
    assert!(backend.lifecycle().healthy());
    assert_eq!(1, backend.lifecycle().recoveries());
}

/// The heartbeat notices the restart and makes the manager reconnect ahead of
/// the next command - so even a non-idempotent command issued after a restart
/// succeeds on its FIRST attempt (the producer's first RPUSH after a restart,
/// the field's note-3 symptom).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn heartbeat_heals_the_connection_ahead_of_the_next_command() {
    let (backend, proxy) = backend_through_proxy(5000, 100).await;
    backend
        .query::<i64>(redis::cmd("RPUSH").arg("list").arg("one"))
        .await
        .expect("healthy push");

    proxy.bounce().await;
    // the failed heartbeat marks the loss ...
    assert!(
        wait_until(Duration::from_secs(3), || !backend.lifecycle().healthy()).await,
        "the heartbeat must notice the lost connection"
    );
    // ... and the next heartbeat finds the manager's fresh connection
    assert!(
        wait_until(Duration::from_secs(3), || backend.lifecycle().healthy()).await,
        "the heartbeat must see the connection restored"
    );
    let length: i64 = backend
        .query(redis::cmd("RPUSH").arg("list").arg("two"))
        .await
        .expect("healed ahead of the command: first attempt succeeds");
    assert_eq!(2, length);
    assert_eq!(
        0,
        backend.lifecycle().retries(),
        "no command needed a retry"
    );
    assert_eq!(1, backend.lifecycle().drops());
    assert_eq!(1, backend.lifecycle().recoveries());
}

/// Under a known outage a command makes ONE attempt - no doubled deadline: the
/// heartbeat has marked the connection down, so an idempotent command is not
/// retried. The one attempt is bounded by the command deadline: while the
/// manager is still trying to reconnect the command waits on that and times
/// out (408); an outright refusal fails 503.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn known_outage_fails_fast_without_a_second_attempt() {
    let (backend, proxy) = backend_through_proxy(1500, 100).await;
    backend
        .query_idempotent::<String>(redis::cmd("SETEX").arg("k").arg(60).arg("v"))
        .await
        .expect("healthy write");

    proxy.refuse().await;
    assert!(
        wait_until(Duration::from_secs(3), || !backend.lifecycle().healthy()).await,
        "the heartbeat must notice the outage"
    );
    let started = std::time::Instant::now();
    let failure = backend
        .query_idempotent::<Option<String>>(redis::cmd("GET").arg("k"))
        .await
        .expect_err("an outage still fails");
    assert!(
        matches!(failure.status(), 408 | 503),
        "the deadline or a refusal, never a server answer: {} {}",
        failure.status(),
        failure.message()
    );
    assert_eq!(
        0,
        backend.lifecycle().retries(),
        "known down: a single attempt"
    );
    assert!(
        started.elapsed() < Duration::from_millis(3000),
        "one bounded attempt (1500 ms), not two (took {:?})",
        started.elapsed()
    );
    assert!(!backend.lifecycle().healthy());
}

/// The classification behind the retry: a dropped, refused or reset connection
/// is a loss; a timeout or a server answer is not.
#[test]
fn connection_loss_classification() {
    fn io(kind: std::io::ErrorKind) -> redis::RedisError {
        redis::RedisError::from(std::io::Error::new(kind, "socket"))
    }
    assert!(is_connection_loss(&io(std::io::ErrorKind::BrokenPipe)));
    assert!(is_connection_loss(&io(std::io::ErrorKind::ConnectionReset)));
    assert!(is_connection_loss(&io(
        std::io::ErrorKind::ConnectionRefused
    )));
    assert!(is_connection_loss(&io(std::io::ErrorKind::UnexpectedEof)));
    assert!(!is_connection_loss(&io(std::io::ErrorKind::TimedOut)));
    let server: redis::RedisError = (redis::ErrorKind::UnexpectedReturnType, "not a string").into();
    assert!(!is_connection_loss(&server));
}
