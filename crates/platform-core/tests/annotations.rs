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

//! Integration tests for increment 10 — the annotation macros (`#[preload]`,
//! `#[before_application]`, `#[main_application]`, stacked `#[zero_tracing]`)
//! and the `AutoStart` link-time inventory collection (the Java
//! classpath-scanning analog).
//!
//! One test function on purpose: `AutoStart::main` registers routes on the
//! **global** platform, whose manager/worker tasks live on the runtime of the
//! test that started them — a second `#[tokio::test]` would find them dead
//! (each test owns its runtime). All assertions therefore share one run.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::{
    before_application, main_application, optional_service, overrides, preload, resources, trace,
    AppError, AutoStart, ComposableFunction, EntryPoint, EventEnvelope, Platform, PostOffice,
    TypedFunction,
};

static JOURNAL: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
static TYPED_HAD_TRACE: AtomicBool = AtomicBool::new(false);
static ZERO_TRACED_HAD_TRACE: AtomicBool = AtomicBool::new(false);
static ZERO_TRACED_RAN: AtomicBool = AtomicBool::new(false);

fn journal() -> &'static Mutex<Vec<String>> {
    JOURNAL.get_or_init(|| Mutex::new(Vec::new()))
}

// ---- annotated items (Java-style declarative registration) ----

#[derive(serde::Serialize, serde::Deserialize)]
struct Ping {
    n: u64,
}

/// Typed function via the `typed` flag (Java `@PreLoad` on a
/// `TypedLambdaFunction`); literal instance count.
#[preload(route = "anno.typed.echo", instances = 4, typed)]
struct TypedEcho;

#[async_trait]
impl TypedFunction<Ping, Ping> for TypedEcho {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: Ping,
        _instance: usize,
    ) -> Result<Ping, AppError> {
        TYPED_HAD_TRACE.store(
            trace::with_current(|_| true).unwrap_or(false),
            Ordering::SeqCst,
        );
        Ok(Ping { n: input.n + 1 })
    }
}

/// Untyped function whose instance count comes from configuration (Java
/// `envInstances`): `anno.pool.size` is supplied as a process override in the
/// test setup, so the literal `instances = 2` must lose to it.
#[preload(
    route = "anno.untyped.echo",
    env_instances = "anno.pool.size",
    instances = 2
)]
struct UntypedEcho;

#[async_trait]
impl ComposableFunction for UntypedEcho {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Ok(EventEnvelope::new().set_raw_body(input.body().clone()))
    }
}

/// Stacked marker (Java `@PreLoad` + `@ZeroTracing`): a traced request to
/// this route must execute WITHOUT a trace bracket (no telemetry, no
/// propagation).
#[preload(route = "anno.zero.traced")]
#[zero_tracing]
struct ZeroTracedFn;

#[async_trait]
impl ComposableFunction for ZeroTracedFn {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        ZERO_TRACED_RAN.store(true, Ordering::SeqCst);
        ZERO_TRACED_HAD_TRACE.store(
            trace::with_current(|_| true).unwrap_or(false),
            Ordering::SeqCst,
        );
        EventEnvelope::new().set_body("ok")
    }
}

/// The interceptor flag (Java @PreLoad + @EventInterceptor): manual reply
/// through the raw envelope's reply_to/cid; no auto-reply on success.
#[preload(route = "anno.interceptor")]
#[event_interceptor]
struct AnnoInterceptor;

#[async_trait]
impl ComposableFunction for AnnoInterceptor {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if let (Some(reply_to), Some(cid)) = (input.reply_to(), input.correlation_id()) {
            let po = PostOffice::new(&Platform::get_instance());
            po.send(
                EventEnvelope::new()
                    .set_to(reply_to)
                    .set_correlation_id(cid)
                    .set_body("manual")?,
            )
            .await?;
        }
        EventEnvelope::new().set_body("ignored by the worker")
    }
}

/// `#[optional_service]` as a first-class attribute (Java `@OptionalService`),
/// written ABOVE the primary attribute — Java stacking order. A satisfied
/// condition registers the route.
#[optional_service("profile.indicator=base")]
#[preload(route = "anno.gated.on")]
struct GatedOn;

#[async_trait]
impl ComposableFunction for GatedOn {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new().set_body("gated on")
    }
}

/// An unsatisfied condition (above-order) skips registration silently.
#[optional_service("profile.indicator=no-such-profile")]
#[preload(route = "anno.gated.off")]
struct GatedOff;

#[async_trait]
impl ComposableFunction for GatedOff {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new().set_body("never registered")
    }
}

/// The marker order (condition BELOW the primary attribute) keeps working.
#[preload(route = "anno.gated.below")]
#[optional_service("profile.indicator=base")]
struct GatedBelow;

#[async_trait]
impl ComposableFunction for GatedBelow {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new().set_body("gated below")
    }
}

/// `#[optional_service]` gates entry points too: this hook must never run
/// (its journal entry would break the exact-sequence assertion below).
#[optional_service("profile.indicator=no-such-profile")]
#[before_application(sequence = 7)]
struct SkippedHook;

#[async_trait]
impl EntryPoint for SkippedHook {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        journal()
            .lock()
            .expect("journal mutex")
            .push("before-7-skipped".into());
        Ok(())
    }
}

#[before_application(sequence = 5)]
struct SecondHook;

#[async_trait]
impl EntryPoint for SecondHook {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        journal()
            .lock()
            .expect("journal mutex")
            .push("before-5".into());
        Ok(())
    }
}

#[before_application(sequence = 3)]
struct FirstHook;

#[async_trait]
impl EntryPoint for FirstHook {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        journal()
            .lock()
            .expect("journal mutex")
            .push("before-3".into());
        Ok(())
    }
}

#[main_application]
struct TheApp;

#[async_trait]
impl EntryPoint for TheApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        journal().lock().expect("journal mutex").push("main".into());
        Ok(())
    }
}

// ---- the single lifecycle run + all assertions ----

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn annotation_macros_end_to_end() {
    resources::prepend_resource_root("tests/resources");
    let holding = std::env::temp_dir().join(format!("mercury-anno-test-{}", std::process::id()));
    overrides::set("transient.data.store", &holding.display().to_string());
    // envInstances source for anno.untyped.echo (a -D style process override)
    overrides::set("anno.pool.size", "7");

    // the one-liner lifecycle: collects every annotated item in this binary
    AutoStart::main(vec![]).await.expect("lifecycle");

    // before-application hooks ran in sequence order (3 before 5), then main
    assert_eq!(
        journal().lock().expect("journal mutex").clone(),
        vec!["before-3", "before-5", "main"]
    );

    // every #[preload] route is registered on the global platform
    let platform = Platform::get_instance();
    assert!(platform.has_route("anno.typed.echo"));
    assert!(platform.has_route("anno.untyped.echo"));
    assert!(platform.has_route("anno.zero.traced"));

    // #[optional_service] is first-class and order-independent: a satisfied
    // condition registers (both stacking orders); an unsatisfied one skips
    assert!(platform.has_route("anno.gated.on"), "condition-above order");
    assert!(
        platform.has_route("anno.gated.below"),
        "condition-below order"
    );
    assert!(
        !platform.has_route("anno.gated.off"),
        "unsatisfied condition must skip registration"
    );
    // instance counts: the literal for the typed echo; env_instances (7 from
    // the override) beats the literal 2 for the untyped one
    assert_eq!(platform.instances("anno.typed.echo"), Some(4));
    assert_eq!(platform.instances("anno.untyped.echo"), Some(7));

    // a typed annotated function serves RPC with (de)serialization
    let po = PostOffice::new(&platform);
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("anno.typed.echo")
                .set_trace(&trace::new_trace_id(), "TEST /anno")
                .set_body(Ping { n: 41 })
                .expect("body"),
            Duration::from_secs(2),
        )
        .await
        .expect("typed rpc");
    assert_eq!(reply.body_as::<Ping>().expect("typed reply").n, 42);
    // a normal preloaded route IS trace-bracketed when the request is traced
    assert!(TYPED_HAD_TRACE.load(Ordering::SeqCst));

    // an untyped annotated function echoes through the same bus
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("anno.untyped.echo")
                .set_body("hello")
                .expect("body"),
            Duration::from_secs(2),
        )
        .await
        .expect("untyped rpc");
    assert_eq!(reply.body_as::<String>().expect("untyped reply"), "hello");

    // the stacked #[zero_tracing] marker suppresses the trace bracket even
    // for a traced request
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("anno.zero.traced")
                .set_trace(&trace::new_trace_id(), "TEST /zero")
                .set_body("go")
                .expect("body"),
            Duration::from_secs(2),
        )
        .await
        .expect("zero-traced rpc");
    assert_eq!(reply.body_as::<String>().expect("zero reply"), "ok");
    assert!(ZERO_TRACED_RAN.load(Ordering::SeqCst));
    assert!(
        !ZERO_TRACED_HAD_TRACE.load(Ordering::SeqCst),
        "zero-traced route must not execute inside a trace bracket"
    );

    // the stacked #[event_interceptor] marker: the manual reply arrives and
    // the returned envelope is ignored by the worker
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("anno.interceptor")
                .set_body("ping")
                .expect("body"),
            Duration::from_secs(2),
        )
        .await
        .expect("interceptor manual reply");
    assert_eq!(reply.body_as::<String>().expect("manual reply"), "manual");
}
