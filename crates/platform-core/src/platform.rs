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

//! Rust port of the Java `Platform` registry + the **manager-worker dispatch**
//! (`org.platformlambda.core.system.Platform` / `ServiceQueue` / `WorkerHandler`).
//!
//! Each route gets a **manager task** (the `ServiceQueue` analog) running the
//! FIFO reactive back-pressure state machine, and `instances` **worker tasks**
//! that *pull* work via ready signals:
//!
//! - a worker announces `Ready` (Java's `ready:<route>#<n>` bus signal), waits
//!   for one event, processes it, and announces `Ready` again — at most one
//!   in-flight event per worker;
//! - the manager keeps a FIFO of ready workers. With no free worker it enters
//!   **buffering** mode and spills events into the per-route [`ElasticQueue`]
//!   (first [`MEMORY_BUFFER`](crate::util::elastic_queue::MEMORY_BUFFER) events
//!   in memory, overflow to segment files), draining one event per ready signal
//!   until the queue is empty — then the elastic queue closes and direct
//!   dispatch resumes;
//! - the manager's inbound **mailbox is bounded**
//!   (`elastic.queue.dispatch.mailbox.size`, default 1024, min 20): when it
//!   fills, senders await — back-pressure, not drops (the Java vthread-dispatch
//!   mailbox behavior).
//!
//! Events are serialized (MsgPack) only when they cross into the elastic
//! queue — a deliberate divergence from Java, where every bus message is
//! already `byte[]`; in-process Rust moves the envelope for free, and the
//! on-disk record format stays byte-identical to the Java store.
//!
//! `register` must be called within a Tokio runtime (managers/workers are
//! spawned tasks) — the analog of the Java platform's Vert.x runtime
//! requirement. The manager runs the spill I/O inline on its own task, exactly
//! as Java runs it on the per-route dispatch virtual thread.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, OnceLock, RwLock};
use std::time::Instant;

use tokio::sync::{mpsc, Notify};

use crate::envelope::EventEnvelope;
use crate::function::{AppError, ComposableFunction};
use crate::trace::{self, TraceState};
use crate::util::app_config_reader::AppConfigReader;
use crate::util::elastic_queue::{ElasticQueue, MEMORY_BUFFER};

const DISPATCH_MAILBOX_SIZE: &str = "elastic.queue.dispatch.mailbox.size";
const DEFAULT_DISPATCH_MAILBOX_SIZE: usize = 1024;

/// Everything entering a route's manager mailbox: an event to dispatch, or a
/// worker's ready signal (Java sends both through the same route consumer).
/// The envelope is boxed to keep the two variants close in size (clippy
/// `large_enum_variant`).
enum MailboxMessage {
    Event(Box<EventEnvelope>),
    Ready(usize),
}

struct RouteEntry {
    /// Java ServiceDef.isPrivateFunction (see FunctionOptions::private).
    private: bool,
    mailbox: mpsc::Sender<MailboxMessage>,
    stop: Arc<Notify>,
    instances: usize,
    function: Arc<dyn ComposableFunction>,
}

/// Reserved function route names for Event Script (Java `EventEmitter`
/// parity): events to these routes are executed DIRECTLY on a fresh task,
/// bypassing the manager-worker queue, so the engine behaves as part of the
/// event core — orchestration never waits on its own bounded mailbox
/// (liveness) and worker-instance count is irrelevant (unbounded concurrency,
/// like Java's virtual-thread submit). Both functions are stateless event
/// routers, so functional isolation is guaranteed.
///
/// Deliberately NOT exposed through registration options or the annotation
/// macros (maintainer decision, 2026-07-16): application functions must stay
/// on the reactive back-pressure path — this bypass is an engine privilege,
/// not an API.
const RESERVED_ENGINE_ROUTES: &[&str] = &[
    "event.script.manager",
    "task.executor",
    // the RPC reply listener: replies must complete on the SENDER's runtime —
    // the registered workers are the route's addressable identity, but a
    // multi-runtime process (test binaries) cannot rely on their liveness
    crate::inbox::TEMPORARY_INBOX,
];

type RouteRegistry = Arc<RwLock<HashMap<String, RouteEntry>>>;

/// The service registry: route name → manager + worker pool. Cheap to clone.
#[derive(Clone, Default)]
pub struct Platform {
    routes: RouteRegistry,
}

/// Registration options (Java annotation analogs): `zero_traced` is
/// `@ZeroTracing` (this route's executions are excluded from trace recording);
/// Java `ServiceDef.MAX_INSTANCES`: the worker-count ceiling — registration
/// clamps to `1..=1000` exactly like `setConcurrency`.
const MAX_INSTANCES: usize = 1000;

/// `interceptor` is `@EventInterceptor` (the function receives the raw
/// envelope — `reply_to`/`cid` intact — and replies manually; the worker sends
/// no auto-reply on success, though a failure still routes to `reply_to`).
#[derive(Clone, Copy, Debug, Default)]
pub struct FunctionOptions {
    pub zero_traced: bool,
    /// Java `isPrivate`: a private function is reachable only inside this
    /// application instance — Event over HTTP rejects it with 403. `register`
    /// creates PUBLIC functions (Java parity); use `register_private` or the
    /// `#[preload]` macro (private by default, like Java `@PreLoad`).
    pub private: bool,
    pub interceptor: bool,
}

impl Platform {
    pub fn new() -> Self {
        let platform = Self::default();
        // the RPC reply listener is an ESSENTIAL service present from birth
        // on every platform — Java's EssentialServiceLoader registers it at
        // the highest startup priority; isolated registries (tests) get it
        // here so an RPC works before any lifecycle runs. Private,
        // zero-tracing, 500 instances (Java parity).
        let _ = platform.register_with_options(
            crate::inbox::TEMPORARY_INBOX,
            Arc::new(crate::inbox::TemporaryInbox),
            500,
            FunctionOptions {
                zero_traced: true,
                interceptor: false,
                private: true,
            },
        );
        platform
    }

    /// The process-wide platform (Java `Platform.getInstance()`), created on
    /// first use. `Platform::new()` remains available for isolated registries
    /// (tests); the lifecycle (`AppStarter`) uses this shared one.
    pub fn get_instance() -> Platform {
        static GLOBAL: OnceLock<Platform> = OnceLock::new();
        GLOBAL.get_or_init(Platform::new).clone()
    }

    /// The application name (Java `platform.getName()`): `application.name`,
    /// else `application` — Java's primary key and default. The Java
    /// `spring.application.name` fallback is retired with the other Spring
    /// names (maintainer decision, 2026-07-19; Spring is not ported).
    pub fn name() -> String {
        let config = AppConfigReader::get_instance();
        config
            .get_property("application.name")
            .unwrap_or_else(|| "application".to_string())
    }

    /// This process's unique origin id (Java `platform.getOrigin()`,
    /// simplified: a uuid per process; Java's optional appId derivation is not
    /// ported until something needs it).
    pub fn origin() -> &'static str {
        static ORIGIN: OnceLock<String> = OnceLock::new();
        ORIGIN.get_or_init(|| uuid::Uuid::new_v4().simple().to_string())
    }

    /// Register a function at a route with `instances` concurrent workers
    /// (Java `platform.register(route, lambda, instances)`).
    pub fn register(
        &self,
        route: &str,
        function: Arc<dyn ComposableFunction>,
        instances: usize,
    ) -> Result<(), AppError> {
        self.register_with_options(route, function, instances, FunctionOptions::default())
    }

    /// Register a PRIVATE function (Java `platform.registerPrivate`): callable
    /// only inside this application instance — Event over HTTP rejects it
    /// with 403. Engine internals register this way (increment 60).
    pub fn register_private(
        &self,
        route: &str,
        function: Arc<dyn ComposableFunction>,
        instances: usize,
    ) -> Result<(), AppError> {
        self.register_with_options(
            route,
            function,
            instances,
            FunctionOptions {
                private: true,
                ..FunctionOptions::default()
            },
        )
    }

    /// Whether a registered route is private (Java `ServiceDef.isPrivateFunction`);
    /// `None` when the route is not registered.
    pub fn is_private(&self, route: &str) -> Option<bool> {
        self.routes
            .read()
            .expect("route registry poisoned")
            .get(route)
            .map(|entry| entry.private)
    }

    /// Register with explicit options (increment E-3 extends the increment-10
    /// zero-trace flag with the event-interceptor mode).
    pub fn register_with_options(
        &self,
        route: &str,
        function: Arc<dyn ComposableFunction>,
        instances: usize,
        options: FunctionOptions,
    ) -> Result<(), AppError> {
        validate_route(route)?;
        // Java ServiceDef.setConcurrency: Math.max(1, Math.min(n, 1000)) —
        // zero is accepted (→ 1) and the worker count is capped, silently
        // (F10 parity fix, 2026-07-21; previously 0 was rejected and there
        // was no ceiling)
        let instances = instances.clamp(1, MAX_INSTANCES);
        let (mailbox_tx, mailbox_rx) = mpsc::channel::<MailboxMessage>(dispatch_mailbox_size());
        let stop = Arc::new(Notify::new());
        {
            let mut routes = self.routes.write().expect("route registry poisoned");
            // Java Platform.register: an existing route is RELOADED — the old
            // service is released and the new one takes its place (F10 parity
            // fix; previously rejected with "already exists")
            if let Some(previous) = routes.remove(route) {
                log::warn!("Reloading LambdaFunction {route}");
                previous.stop.notify_one();
            }
            routes.insert(
                route.to_string(),
                RouteEntry {
                    private: options.private,
                    mailbox: mailbox_tx.clone(),
                    stop: stop.clone(),
                    instances,
                    function: function.clone(),
                },
            );
        }
        // workers: capacity-1 event channels; each announces Ready through the
        // shared mailbox, waits for one event, processes, repeats (1-based ids)
        let mut worker_txs = Vec::with_capacity(instances);
        for instance in 1..=instances {
            let (worker_tx, worker_rx) = mpsc::channel::<EventEnvelope>(1);
            worker_txs.push(worker_tx);
            tokio::spawn(worker_loop(
                route.to_string(),
                instance,
                function.clone(),
                worker_rx,
                mailbox_tx.clone(),
                self.routes.clone(),
                options,
            ));
        }
        // the manager (ServiceQueue analog) owns the state machine + elastic queue
        tokio::spawn(manager_loop(
            route.to_string(),
            mailbox_rx,
            stop,
            worker_txs,
        ));
        Ok(())
    }

    /// True when the route is registered locally (Java `hasRoute`).
    pub fn has_route(&self, route: &str) -> bool {
        self.routes
            .read()
            .expect("route registry poisoned")
            .contains_key(route)
    }

    /// Release a route (Java `release`): signals the manager to stop; workers
    /// exit as their channels close; the elastic queue is destroyed. Returns
    /// whether the route existed.
    pub fn release(&self, route: &str) -> bool {
        let removed = self
            .routes
            .write()
            .expect("route registry poisoned")
            .remove(route);
        match removed {
            Some(entry) => {
                entry.stop.notify_one();
                true
            }
            None => false,
        }
    }

    /// Registered route names (sorted, for stable output).
    pub fn routes(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .routes
            .read()
            .expect("route registry poisoned")
            .keys()
            .cloned()
            .collect();
        names.sort();
        names
    }

    /// Worker count for a route, if registered.
    pub fn instances(&self, route: &str) -> Option<usize> {
        self.routes
            .read()
            .expect("route registry poisoned")
            .get(route)
            .map(|entry| entry.instances)
    }

    /// Crate-internal: deliver an event into a route's manager mailbox. Awaits
    /// when the bounded mailbox is full — back-pressure, not drops.
    ///
    /// An `@origin`/`@instance` suffix on the route is tolerated and ignored
    /// (Eric's ruling: the `route@origin` syntax is only meaningful under the
    /// legacy Kafka service mesh — this port parses it away on inbound data,
    /// e.g. envelopes from a Java peer, and never generates it).
    pub(crate) async fn deliver(&self, route: &str, event: EventEnvelope) -> Result<(), AppError> {
        let route = bare_route(route);
        let mut event = event;
        normalize_null_transport(&mut event);
        // reserved engine routes run directly (see RESERVED_ENGINE_ROUTES)
        if let Some(function) = reserved_route_function(&self.routes, route) {
            spawn_direct(function, event);
            return Ok(());
        }
        // clone the sender out of the lock before awaiting
        let sender = self
            .routes
            .read()
            .expect("route registry poisoned")
            .get(route)
            .map(|entry| entry.mailbox.clone());
        let Some(sender) = sender else {
            return Err(AppError::new(404, format!("Route {route} not found")));
        };
        sender
            .send(MailboxMessage::Event(Box::new(event)))
            .await
            .map_err(|_| AppError::new(500, format!("Route {route} is closed")))
    }
}

/// Deterministic null transport on EVERY hop (increment 58, the F2 decision —
/// maintainer chose normalization over documentation): Java serializes every
/// event-bus hop, so `Nil` map entries are stripped consistently when
/// `serializer.null.transport=false`. The Rust fast path deliberately skips
/// serialization (the documented performance divergence stays), so the strip
/// is applied explicitly here — predicate-guarded: a body without Nil map
/// entries costs one allocation-free read-only walk.
fn normalize_null_transport(event: &mut EventEnvelope) {
    if !crate::serializer::null_transport() && crate::serializer::has_nil_map_entry(event.body()) {
        let stripped = crate::serializer::strip_nulls_always(event.body());
        event.set_body_internal(stripped);
    }
}

/// Resolve a reserved engine route to its registered function (None for
/// normal routes — they take the manager-worker path).
fn reserved_route_function(
    registry: &RouteRegistry,
    route: &str,
) -> Option<Arc<dyn ComposableFunction>> {
    if RESERVED_ENGINE_ROUTES.contains(&route) {
        registry
            .read()
            .expect("route registry poisoned")
            .get(route)
            .map(|entry| entry.function.clone())
    } else {
        None
    }
}

/// Direct execution (Java `EventEmitter.runTaskExecutor`): invoke the engine
/// function on a fresh task with instance 1 — no queue, no trace bracket, no
/// auto-reply; failures are logged (the engine functions manage their own
/// replies and error routing).
fn spawn_direct(function: Arc<dyn ComposableFunction>, event: EventEnvelope) {
    tokio::spawn(async move {
        let headers = event.headers().clone();
        if let Err(e) = function.handle_event(headers, event, 1).await {
            log::error!(
                "Unable to execute event script - ({}) {}",
                e.status(),
                e.message()
            );
        }
    });
}

/// Mailbox capacity (Java `ServiceQueue.dispatchMailboxSize`):
/// `elastic.queue.dispatch.mailbox.size`, default 1024, floor `MEMORY_BUFFER`.
fn dispatch_mailbox_size() -> usize {
    let configured = AppConfigReader::get_instance()
        .get_property_or(
            DISPATCH_MAILBOX_SIZE,
            &DEFAULT_DISPATCH_MAILBOX_SIZE.to_string(),
        )
        .parse::<usize>()
        .unwrap_or(DEFAULT_DISPATCH_MAILBOX_SIZE);
    let size = if configured > 0 {
        configured
    } else {
        DEFAULT_DISPATCH_MAILBOX_SIZE
    };
    size.max(MEMORY_BUFFER as usize)
}

/// Validate a route name — port of Java `Utility.validServiceName` plus the
/// at-least-one-dot rule: lowercase alphanumeric with `.` `-` `_`, no leading/
/// trailing/consecutive dots. Crate-visible so the declarative
/// Event-over-HTTP config loader applies the same rule (Java parity).
pub(crate) fn validate_route(route: &str) -> Result<(), AppError> {
    let valid_chars = !route.is_empty()
        && route.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'-' | b'_')
        });
    if !valid_chars {
        return Err(AppError::new(
            400,
            format!("Invalid route '{route}' — use lowercase letters, digits, '.', '-', '_'"),
        ));
    }
    if !route.contains('.')
        || route.starts_with('.')
        || route.ends_with('.')
        || route.contains("..")
    {
        return Err(AppError::new(
            400,
            format!("Invalid route '{route}' — a route needs at least one '.' separator (e.g. 'v1.my.function')"),
        ));
    }
    Ok(())
}

/// The route manager (Java `ServiceQueue.ServiceHandler` state machine):
/// dispatch to a ready worker when one is free, otherwise buffer through the
/// elastic queue; drain one buffered event per ready signal; close the elastic
/// queue when it empties. `buffering` starts **true** (no worker has announced
/// readiness yet — Java parity).
async fn manager_loop(
    route: String,
    mut mailbox: mpsc::Receiver<MailboxMessage>,
    stop: Arc<Notify>,
    worker_txs: Vec<mpsc::Sender<EventEnvelope>>,
    // workers are 1-based; worker_txs[n - 1] is worker #n
) {
    let mut elastic = ElasticQueue::new(&route);
    let mut ready_fifo: VecDeque<usize> = VecDeque::new();
    let mut ready_set: HashSet<usize> = HashSet::new();
    let mut buffering = true;
    loop {
        let message = tokio::select! {
            _ = stop.notified() => break,
            received = mailbox.recv() => match received {
                Some(message) => message,
                None => break,
            },
        };
        match message {
            MailboxMessage::Ready(worker) => {
                // guarantee a unique entry per worker (Java idx + fifo)
                if ready_set.insert(worker) {
                    ready_fifo.push_back(worker);
                }
                if buffering {
                    match elastic.read() {
                        Ok(bytes) if bytes.is_empty() => {
                            // close elastic queue when all messages are cleared
                            buffering = false;
                            elastic.close();
                        }
                        Ok(bytes) => match EventEnvelope::from_bytes(&bytes) {
                            Ok(event) => {
                                // guaranteed: this ready signal just enqueued a worker
                                if let Some(next) = ready_fifo.pop_front() {
                                    ready_set.remove(&next);
                                    let _ = worker_txs[next - 1].send(event).await;
                                }
                            }
                            Err(e) => log::error!("{route} corrupted buffered event dropped - {e}"),
                        },
                        Err(e) => log::error!("{route} dispatch error - {e}"),
                    }
                }
            }
            MailboxMessage::Event(event) => {
                let event = *event;
                if buffering {
                    // once the elastic queue is started, continue buffering
                    spill(&route, &mut elastic, &event);
                } else if let Some(next) = ready_fifo.pop_front() {
                    // deliver the event to the next free worker
                    ready_set.remove(&next);
                    let _ = worker_txs[next - 1].send(event).await;
                } else {
                    // no worker available — start buffering
                    buffering = true;
                    spill(&route, &mut elastic, &event);
                }
            }
        }
    }
    // route released: workers exit via channel close; elastic queue cleaned up
    drop(worker_txs);
    elastic.destroy();
}

/// Serialize an event into the elastic queue (a spill I/O failure loses that
/// event and is logged — the Java drainLoop catch-and-continue behavior).
fn spill(route: &str, elastic: &mut ElasticQueue, event: &EventEnvelope) {
    match event.to_bytes() {
        Ok(bytes) => {
            if let Err(e) = elastic.write(&bytes) {
                log::error!("{route} dispatch error - {e}");
            }
        }
        Err(e) => log::error!("{route} dispatch error - {e}"),
    }
}

/// One worker (Java `WorkerHandler`): announce Ready → take one event →
/// invoke the function → deliver the reply (if `reply_to`) with the request's
/// correlation id → repeat. Exits when the route is released.
async fn worker_loop(
    route: String,
    instance: usize,
    function: Arc<dyn ComposableFunction>,
    mut events: mpsc::Receiver<EventEnvelope>,
    manager: mpsc::Sender<MailboxMessage>,
    registry: RouteRegistry,
    options: FunctionOptions,
) {
    // a route that is itself telemetry plumbing or the RPC reply listener
    // (or listed in skip.rpc.tracing, or registered with @ZeroTracing
    // semantics) never traces its own executions
    let zero_traced = options.zero_traced || is_zero_traced(&route);
    loop {
        if manager.send(MailboxMessage::Ready(instance)).await.is_err() {
            break; // manager gone — route released
        }
        let Some(event) = events.recv().await else {
            break;
        };
        let started = Instant::now();
        let mut event = event;
        // the RPC marker (Java `event.getTag(RPC)`): RPC requests carry the
        // reserved rpc tag; the reply address is just routing
        let served_rpc = event.tag(crate::post_office::RPC_TAG).is_some();
        // the reply listener receives its envelope PRISTINE: the envelope IS
        // the payload the waiting caller gets back (annotations for the
        // round_trip record, the request's correlation id for resolution) —
        // Java executeFunction exempts TEMPORARY_INBOX from metadata handling
        let is_reply_listener = route == crate::inbox::TEMPORARY_INBOX;
        let (business_cid, headers) = if is_reply_listener {
            (None, event.headers().clone())
        } else {
            // annotations belong to REPLY envelopes only — never leak a prior
            // hop's annotations into a function's input (Java WorkerHandler)
            event.clear_annotations_internal();
            // Metadata contract (Java WorkerHandler parity): a user function
            // receives a COPY of the envelope headers with read-only metadata
            // INJECTED at delivery time; metadata is never transported in the
            // event itself. The business correlation-id arrives on the
            // engine-managed tag (a legacy pre-4.10.2 peer transported it as
            // an envelope header — honor that value, then remove the header),
            // the engine-internal relay guard never reaches a user function,
            // and tags are engine-visible only.
            let tag_cid = event
                .tag(crate::post_office::BUSINESS_CID_TAG)
                .map(str::to_string);
            event.clear_tags_internal();
            let legacy_cid = event.remove_header_internal(crate::automation::MY_CORRELATION_ID);
            event.remove_header_internal(crate::automation::X_EVENT_API);
            // the port's cid-slot convention is the last fallback: a direct
            // bus caller may put the business id in the envelope cid
            let business_cid = tag_cid
                .or(legacy_cid)
                .or_else(|| event.correlation_id().map(str::to_string));
            // the function's input header copy with the my_* read-only keys
            let mut headers = event.headers().clone();
            headers.insert(MY_ROUTE.to_string(), route.clone());
            if let Some(trace_id) = event.trace_id() {
                headers.insert(MY_TRACE_ID.to_string(), trace_id.to_string());
            }
            if let Some(trace_path) = event.trace_path() {
                headers.insert(MY_TRACE_PATH.to_string(), trace_path.to_string());
            }
            if let Some(cid) = &business_cid {
                headers.insert(
                    crate::automation::MY_CORRELATION_ID.to_string(),
                    cid.clone(),
                );
            }
            (business_cid, headers)
        };
        let reply_to = event.reply_to().map(str::to_string);
        let cid = event.correlation_id().map(str::to_string);
        let event_from = event.from().map(str::to_string);
        // trace bracket (Java WorkerHandler): a traced request carries trace id
        // + path; this execution gets its own span, parented to the sender's
        // span carried on the envelope. A ZERO-TRACED route still keeps the
        // bracket when the incoming event is traced — Java gates only
        // startTracing + sendTracingInfo on the flag, while the reply and any
        // nested calls carry the trace onward unconditionally (F3 parity fix,
        // 2026-07-21) — but the hop emits no telemetry and contributes no span.
        // Deliberate minor divergence: the hop's own JSON log lines still
        // resolve trace tokens (Java registers no log context here); log-only,
        // nothing changes on the wire.
        let trace_state = match (event.trace_id(), event.trace_path()) {
            (Some(trace_id), Some(trace_path)) => {
                // the business correlation-id resolved above (tag > legacy
                // header > cid-slot convention) — the cid slot itself may
                // carry an internal correlation id (the HTTP context id of a
                // REST callback dispatch, or a flow task's composite id)
                let mut state = TraceState::new(
                    &route,
                    trace_id,
                    trace_path,
                    event.span_id(),
                    business_cid.as_deref(),
                );
                state.zero_traced = zero_traced;
                Some(state)
            }
            _ => None,
        };
        let (result, finished_state) =
            trace::run_scoped(trace_state, function.handle_event(headers, event, instance)).await;
        // execution-time metric, standardized to 3 decimal points at the source
        // (Java `WorkerHandler.getExecTime` parity: clamp ≥ 0, round to 3 dp).
        // Rounding here means every consumer — the reply envelope, the telemetry
        // dataset, and the Playground traveler's "Executed … in T ms" narration —
        // reports the same value rather than a raw full-precision float.
        let elapsed_ms = started.elapsed().as_secs_f32() * 1000.0;
        let elapsed_ms = (elapsed_ms.max(0.0) * 1000.0).round() / 1000.0;
        // propagate the trace to the response so the next hop chains correctly;
        // a zero-traced hop contributes no span of its own (Java parity)
        let trace_triple = finished_state.as_ref().map(|s| {
            (
                s.trace_id.clone(),
                s.trace_path.clone(),
                (!s.zero_traced).then(|| s.span_id.clone()),
            )
        });
        // trace annotations ride the REPLY (Java applyTraceContext): the RPC
        // caller folds them into the round_trip record; a zero-traced hop
        // attaches none (Java has no live TraceInfo there)
        let reply_annotations: HashMap<String, rmpv::Value> = finished_state
            .as_ref()
            .filter(|s| !s.zero_traced)
            .map(|s| {
                s.annotations
                    .iter()
                    .filter_map(|(k, v)| {
                        rmpv::ext::to_value(v).ok().map(|value| (k.clone(), value))
                    })
                    .collect()
            })
            .unwrap_or_default();
        // pre-compute the outcome for the telemetry record — the result is
        // consumed by the reply delivery below (Java reads it from
        // ProcessStatus after processEvent has already sent the response)
        let outcome = match &result {
            Ok(response) => (response.status(), !response.has_error(), None),
            Err(e) => (e.status(), false, Some(e.message().to_string())),
        };
        // whether an RPC reply failed to reach its waiting caller (the Java
        // ProcessStatus.isNotDelivered signal); an interceptor's success path
        // attempts no delivery and is NOT a delivery failure (Java parity)
        let mut not_delivered = false;
        match (reply_to, result) {
            // an event interceptor replies MANUALLY (Java @EventInterceptor):
            // its successful return value is ignored and no auto-reply is sent
            // — but a FAILURE still routes to reply_to (Java WorkerHandler:
            // only the success reply is interceptor-guarded)
            (Some(_), Ok(_)) if options.interceptor => {}
            (Some(reply_route), result) => {
                let mut response = match result {
                    Ok(envelope) => envelope,
                    Err(e) => EventEnvelope::new()
                        .set_status(e.status())
                        .set_raw_body(rmpv::Value::String(e.message().into())),
                };
                response.set_cid_internal(cid);
                response.set_from_internal(&route);
                response.set_to_internal(&reply_route);
                response.set_exec_time_internal(elapsed_ms);
                response.set_annotations_internal(reply_annotations.clone());
                // exit-side sanitization, symmetric with the entry-side
                // injection (Java copyResponseHeaders): the read-only my_*
                // keys and engine-internal keys never leave a function as
                // response headers, even if it accidentally copies its input
                // headers onto the returned envelope
                sanitize_response_headers(&mut response);
                // the reply is a bus hop too — same deterministic null strip
                normalize_null_transport(&mut response);
                if let Some((trace_id, trace_path, span_id)) = trace_triple {
                    response.set_trace_internal(&trace_id, &trace_path);
                    match span_id {
                        Some(span_id) => response.set_span_id_internal(&span_id),
                        // a zero-traced hop owns no span — and must not leak
                        // a nested reply's span as its own (Java rebuilds the
                        // response envelope, so its reply never carries one)
                        None => response.clear_span_id_internal(),
                    }
                }
                // RPC replies route to the reserved temporary.inbox service
                // like any other destination (Java parity) — no special path.
                // A legacy '@origin' suffix is parsed away (never generated).
                let reply_route = bare_route(&reply_route).to_string();
                if let Some(function) = reserved_route_function(&registry, &reply_route) {
                    // replies to the reserved engine routes (task callbacks)
                    // take the same direct path as sends
                    spawn_direct(function, response);
                } else {
                    // clone the reply mailbox out of the lock before awaiting
                    let sender = registry
                        .read()
                        .expect("route registry poisoned")
                        .get(&reply_route)
                        .map(|entry| entry.mailbox.clone());
                    if let Some(sender) = sender {
                        // route may already be released — drop silently
                        let _ = sender.send(MailboxMessage::Event(Box::new(response))).await;
                    } else {
                        // the reply had nowhere to go (Java ProcessStatus
                        // notDelivered) — the worker keeps its own record
                        not_delivered = true;
                    }
                }
            }
            (None, Err(e)) => {
                // fire-and-forget failure has nowhere to go — log it (Java parity)
                log::warn!(
                    "Unhandled exception in {route}#{instance}: ({}) {}",
                    e.status(),
                    e.message()
                );
            }
            (None, Ok(_)) => {} // fire-and-forget success: result discarded
        }
        // send the performance-metrics dataset to the telemetry sink — never
        // for a zero-traced route, and never for an RPC-served execution whose
        // reply reached the caller: the caller's round_trip record is THE
        // record for this span (Java WorkerHandler.sendTracingInfo gate:
        // `journaled || rpc == null || notDelivered`; journaling not ported)
        if let Some(state) = finished_state.filter(|s| !s.zero_traced) {
            if !served_rpc || not_delivered {
                emit_telemetry(&registry, &route, event_from, &state, outcome, elapsed_ms).await;
            }
        }
    }
}

/// Strip a legacy `@origin`/`@instance` suffix from a route reference —
/// meaningful only under the Java Kafka service mesh, which is out of scope
/// here: inbound values are parse-tolerant, outbound values never carry one.
pub(crate) fn bare_route(route: &str) -> &str {
    match route.find('@') {
        Some(at) => &route[..at],
        None => route,
    }
}

/// Read-only metadata keys injected into a function's input header copy at
/// delivery (Java `WorkerHandler` MY_ROUTE / MY_TRACE_ID / MY_TRACE_PATH).
pub(crate) const MY_ROUTE: &str = "my_route";
pub(crate) const MY_TRACE_ID: &str = "my_trace_id";
pub(crate) const MY_TRACE_PATH: &str = "my_trace_path";

/// Exit-side sanitization (Java `WorkerHandler.copyResponseHeaders`): the
/// injected read-only metadata and engine-internal keys are filtered from a
/// function's returned envelope before it becomes a reply.
fn sanitize_response_headers(response: &mut EventEnvelope) {
    for key in [
        MY_ROUTE,
        MY_TRACE_ID,
        MY_TRACE_PATH,
        crate::automation::MY_CORRELATION_ID,
        crate::automation::X_EVENT_API,
    ] {
        response.remove_header_internal(key);
    }
}

/// Whether a route's executions are excluded from trace recording: the
/// telemetry plumbing and the RPC reply listener (Java `@ZeroTracing` +
/// filter — exact names only, no prefixes), and any route listed in
/// `skip.rpc.tracing` (default `async.http.request`).
fn is_zero_traced(route: &str) -> bool {
    if crate::telemetry::ZERO_TRACING_FILTER.contains(&route) {
        return true;
    }
    in_skip_rpc_tracing_list(route)
}

/// Whether a route is listed in `skip.rpc.tracing` (default
/// `async.http.request`) — shared by the worker's zero-trace resolution above
/// and the caller-side RPC `round_trip` record (Java `InboxBase.getSkipTracing`
/// reads the same key).
pub(crate) fn in_skip_rpc_tracing_list(route: &str) -> bool {
    AppConfigReader::get_instance()
        .get_property_or("skip.rpc.tracing", "async.http.request")
        .split([',', ' '])
        .map(str::trim)
        .any(|skipped| skipped == route)
}

/// Build the performance-metrics dataset for one traced execution and send it
/// to the `distributed.tracing` sink (Java `WorkerHandler.sendTracingInfo` +
/// `getMetrics`). Fire-and-forget; silently skipped when the sink is not
/// registered on this platform.
async fn emit_telemetry(
    registry: &RouteRegistry,
    route: &str,
    from: Option<String>,
    state: &TraceState,
    outcome: (i32, bool, Option<String>),
    elapsed_ms: f32,
) {
    let sender = registry
        .read()
        .expect("route registry poisoned")
        .get(crate::telemetry::DISTRIBUTED_TRACING)
        .map(|entry| entry.mailbox.clone());
    let Some(sender) = sender else {
        return; // no telemetry sink on this platform
    };
    let (status, success, exception) = outcome;
    let mut metrics = serde_json::Map::new();
    let mut put = |k: &str, v: serde_json::Value| {
        metrics.insert(k.to_string(), v);
    };
    put("id", serde_json::Value::String(state.trace_id.clone()));
    put("path", serde_json::Value::String(state.trace_path.clone()));
    put("service", serde_json::Value::String(route.to_string()));
    put("start", serde_json::Value::String(state.start_time.clone()));
    put(
        "origin",
        serde_json::Value::String(Platform::origin().to_string()),
    );
    // round to 3 decimals in f64 so the JSON stays clean (f32 noise otherwise)
    put(
        "exec_time",
        serde_json::Value::from(((elapsed_ms as f64) * 1000.0).round() / 1000.0),
    );
    put("status", serde_json::Value::from(status));
    put("success", serde_json::Value::Bool(success));
    if let Some(exception) = exception {
        put("exception", serde_json::Value::String(exception));
    }
    if let Some(from) = from {
        put("from", serde_json::Value::String(from));
    }
    put("span_id", serde_json::Value::String(state.span_id.clone()));
    if let Some(parent) = &state.parent_span_id {
        put("parent_span_id", serde_json::Value::String(parent.clone()));
    }
    let mut dataset = serde_json::Map::new();
    dataset.insert("trace".to_string(), serde_json::Value::Object(metrics));
    if !state.annotations.is_empty() {
        dataset.insert(
            "annotations".to_string(),
            serde_json::Value::Object(state.annotations.clone().into_iter().collect()),
        );
    }
    // the telemetry event itself carries NO trace fields (no recursion)
    match EventEnvelope::new()
        .set_to(crate::telemetry::DISTRIBUTED_TRACING)
        .set_body(serde_json::Value::Object(dataset))
    {
        Ok(event) => {
            let _ = sender.send(MailboxMessage::Event(Box::new(event))).await;
        }
        Err(e) => log::error!("Unable to send to distributed.tracing - {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_validation_rules() {
        assert!(validate_route("v1.get.profile").is_ok());
        assert!(validate_route("hello.world-2_x").is_ok());
        assert!(validate_route("badroute").is_err()); // no dot
        assert!(validate_route("UPPER.case").is_err()); // uppercase
        assert!(validate_route(".leading.dot").is_err());
        assert!(validate_route("trailing.dot.").is_err());
        assert!(validate_route("double..dot").is_err());
        assert!(validate_route("").is_err());
        assert!(validate_route("with space.x").is_err());
    }
}
