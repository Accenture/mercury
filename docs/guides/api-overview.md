# API Overview — Platform & PostOffice

Two small classes are the whole runtime surface a composable function ever touches:

- **`Platform`** — the service registry: register a function at a route with a pool of
  worker instances, query and release routes.
- **`PostOffice`** — the messaging client: fire-and-forget send, RPC with timeout,
  scheduled delivery, and the trace-aware business APIs.

Functions themselves stay plain Rust (see
[Write Your First Function](event-driven/write-your-first-function.md)); these APIs are how
events move between them. Every method documented here exists on the structs in
`crates/platform-core/src/platform.rs` and `crates/platform-core/src/post_office.rs`.

```rust
use std::time::Duration;
use platform_core::{EventEnvelope, Platform, PostOffice};

let po = PostOffice::new(&Platform::get_instance());
let reply = po
    .request(
        EventEnvelope::new().set_to("v1.get.profile").set_body(request)?,
        Duration::from_secs(5),
    )
    .await?;
```

!!! note "Rust port"
    Java `PostOffice` capabilities not (yet) in this port: **broadcast**, **fork-n-join
    parallel RPC** (`request(List<EventEnvelope>, timeout)`), **event-over-HTTP**
    (`asyncRequest` against a remote instance), the `exists()` multi-route check
    (use `Platform::has_route` per route), journaling helpers, and the `Kv` convenience
    class. Java `Platform` extras not ported: `waitForProvider`, `setSelf`/personality,
    and the public/private function distinction (there is no service mesh, so every
    function is local). The port adds nothing that Java lacks — absent here means
    deferred or out of scope, never renamed.

## Platform

### `Platform::get_instance()`

| Signature | Returns |
|---|---|
| `Platform::get_instance()` | `Platform` |

The process-wide platform (Java `Platform.getInstance()`), created on first use. The handle
is cheap to clone. `Platform::new()` remains available for isolated registries in tests;
the application lifecycle uses the shared instance.

### `Platform::name()`

| Signature | Returns |
|---|---|
| `Platform::name()` | `String` |

The application name: the `application.name` configuration key, else `application`.

### `Platform::origin()`

| Signature | Returns |
|---|---|
| `Platform::origin()` | `&'static str` |

This process's unique origin id — a dash-less UUID minted once per process. It appears in
telemetry datasets and in the per-instance transient-store directory name.

### `register(route, function, instances)`

| Signature | Returns |
|---|---|
| `register(&self, route: &str, function: Arc<dyn ComposableFunction>, instances: usize)` | `Result<(), AppError>` |

Registers a function at a route with `instances` concurrent workers (Java
`platform.register(route, lambda, instances)`). Route names are lowercase letters, digits,
`.`, `-`, `_`, with at least one dot and no leading, trailing, or consecutive dots.
Registration fails (status 400) on an invalid name, a duplicate route, or zero instances.
Must be called within a tokio runtime — the manager and workers are spawned tasks.

Most applications never call this directly: `#[preload]` registers functions
declaratively at startup (see the [Macros Reference](macros-reference.md)).

```rust
platform.register("demo.echo", Arc::new(EchoFunction), 10)?;
```

### `register_with_options(route, function, instances, options)`

| Signature | Returns |
|---|---|
| `register_with_options(&self, route, function, instances, options: FunctionOptions)` | `Result<(), AppError>` |

Registration with explicit `FunctionOptions` — the programmatic form of the annotation
markers: `zero_traced` (the `#[zero_tracing]` analog — executions excluded from trace
recording) and `interceptor` (the `#[event_interceptor]` analog — the function replies
manually and the worker sends no automatic reply on success).

```rust
platform.register_with_options(
    "manual.reply.service",
    Arc::new(InterceptorFunction),
    10,
    FunctionOptions { zero_traced: false, interceptor: true },
)?;
```

### `has_route(route)`

| Signature | Returns |
|---|---|
| `has_route(&self, route: &str)` | `bool` |

True when the route is registered locally (Java `hasRoute`).

### `release(route)`

| Signature | Returns |
|---|---|
| `release(&self, route: &str)` | `bool` |

Releases a route (Java `release`): the manager stops, the workers exit as their channels
close, and the route's elastic queue is destroyed. Returns whether the route existed.

### `routes()`

| Signature | Returns |
|---|---|
| `routes(&self)` | `Vec<String>` |

All registered route names, sorted for stable output.

### `instances(route)`

| Signature | Returns |
|---|---|
| `instances(&self, route: &str)` | `Option<usize>` |

The worker count of a registered route, or `None` when the route does not exist.

## PostOffice

### `PostOffice::new(&platform)`

| Signature | Returns |
|---|---|
| `PostOffice::new(platform: &Platform)` | `PostOffice` |

The messaging client, holding a handle to the platform. Cheap to clone; create one wherever
you need it.

When used **inside a traced function**, every `send`/`request` automatically stamps the
current trace context onto the outbound event: the trace id/path, this function's span id
(the receiver's parent span), the sender route (when `from` is unset), and the business
correlation-id (when the event carries none). Outside a trace this is a no-op.

### `send(event)`

| Signature | Returns |
|---|---|
| `async send(&self, event: EventEnvelope)` | `Result<(), AppError>` |

Fire-and-forget delivery to `event.to()` (Java `po.send`). Errors: 400 when `to` is
missing, 404 when the route is not registered. When the destination's bounded mailbox is
full the call awaits — reactive back-pressure, not drops.

```rust
po.send(
    EventEnvelope::new()
        .set_to("v1.audit.logger")
        .set_body(audit_record)?,
)
.await?;
```

### `request(event, timeout)`

| Signature | Returns |
|---|---|
| `async request(&self, event: EventEnvelope, timeout: Duration)` | `Result<EventEnvelope, AppError>` |

RPC (Java `po.request(event, timeout)`): delivers the event with a temporary one-shot inbox
as `reply_to` and awaits the reply. The caller's correlation id is kept, or one is minted.
On expiry the call fails with status **408**. `request(...).await` reads as synchronous but
never blocks a runtime thread — the calling task is suspended until the reply arrives.

```rust
let reply = po
    .request(
        EventEnvelope::new().set_to("greeting.demo").set_body(req)?,
        Duration::from_secs(5),
    )
    .await?;
let body: GreetingResponse = reply.body_as()?;
```

A reply with `status() >= 400` is a delivered error response — check `has_error()` when the
callee can fail meaningfully.

### `send_later(event, delay)`

| Signature | Returns |
|---|---|
| `send_later(&self, event: EventEnvelope, delay: Duration)` | `String` (timer id) |

Schedules a future one-time delivery (Java `po.sendLater(event, time)`): the event is sent
after `delay`. The returned timer id cancels it via `cancel_future_event`. The Event Script
engine uses this for flow TTL watchdogs.

### `cancel_future_event(timer_id)`

| Signature | Returns |
|---|---|
| `cancel_future_event(&self, timer_id: &str)` | `bool` |

Cancels a scheduled delivery (Java `po.cancelFutureEvent(id)`). Returns whether the timer
was still pending.

### `my_trace_id()` / `my_trace_path()`

| Signature | Returns |
|---|---|
| `my_trace_id(&self)` / `my_trace_path(&self)` | `Option<String>` |

The trace id and trace path of the current traced request (Java `getTraceId` /
`getTracePath` on the trace-aware PostOffice). `None` outside a trace.

### `my_correlation_id()`

| Signature | Returns |
|---|---|
| `my_correlation_id(&self)` | `Option<String>` |

The **business correlation-id** of the current traced request (Java
`getMyCorrelationId`) — the end-to-end identifier captured at the HTTP edge (see
[Reserved Names & Headers](reserved-names-and-headers.md)). `None` outside a trace or when
the incoming request carried none.

### `annotate_trace(key, value)`

| Signature | Returns |
|---|---|
| `annotate_trace(&self, key: &str, value: impl Serialize)` | `&Self` |

Attaches business context to the **distributed-trace dataset** that flows to the telemetry
sink (Java `annotateTrace`) — it appears in the `annotations` block of this execution's
trace record. Silent no-op outside a trace.

```rust
po.annotate_trace("customer_tier", "gold")
    .annotate_trace("items", cart.len());
```

### `update_context(key, value)`

| Signature | Returns |
|---|---|
| `update_context(&self, key: &str, value: impl Serialize)` | `Result<(), AppError>` |

Attaches business context to the **application log** stream (Java `updateContext`): the
key-value appears in the `context` block of every subsequent structured log line of this
request (the log context is on by default — see the
[Configuration Reference](configuration-reference.md)). A null value removes the key. The
reserved context keys (`cid`, `traceId`, `tracePath`, `spanId`, `parentSpanId`, `service`,
`utc`) are rejected with status 400; outside a trace the call is a silent no-op.

## Errors: `AppError`

Both APIs (and every function) speak `AppError` — an HTTP-style status code plus a message:

```rust
use platform_core::AppError;

let err = AppError::new(404, "profile not found");
assert_eq!(404, err.status());
assert_eq!("profile not found", err.message());
```

Returning `Err(AppError)` from a function becomes an error reply (`status >= 400`) to the
caller; on a fire-and-forget delivery it is logged instead.

---

*Adapted from the mercury-composable guide `docs/guides/api-overview.md`; keys/APIs enumerated from this repository's source.*
