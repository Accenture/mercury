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
    class. Java `Platform` extras not ported: `waitForProvider` and `setSelf`/personality.
    The **public/private function distinction IS ported**: functions are private by
    default (`is_private = true` on `#[preload]`, `register` vs `register_public`), and a
    private function is not reachable through the `/api/event` endpoint (403), exactly as
    in Java. The port adds nothing that Java lacks — absent here means deferred or out of
    scope, never renamed.

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

### `register_route_pool(prefix, function, count)`

| Signature | Returns |
|---|---|
| `register_route_pool(&self, prefix: &str, function: Arc<dyn ComposableFunction>, count: usize)` | `Result<Vec<String>, AppError>` |

Registers a route pool - a set of private singleton routes `{prefix}.{n}` for n = 0 to
count-1 (Java `registerRoutePool`). Each member runs one worker, so it is a strict FIFO
lane: an application checks out a lane for one conversation's lifetime to keep that
conversation's events in order, while other conversations flow through their own lanes
concurrently - the pattern behind the HTTP edge's streaming reply lanes
(`async.http.response.stream.{n}`). One function instance is shared across all members
(stateless, like any multi-instance function). Returns the generated member routes in
order, ready to seed the application's own checkout structure. Registering an existing
pool reloads it: the previous member set is released first. Lane checkout and return
are the application's concern; route pools are always private.

### `release_route_pool(prefix)`

| Signature | Returns |
|---|---|
| `release_route_pool(&self, prefix: &str)` | `bool` |

Releases a route pool - removes all members and the pool itself (Java
`releaseRoutePool`). Returns whether the pool existed.

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

RPC (Java `po.request(event, timeout)`): delivers the event with the reserved
`temporary.inbox` reply listener as `reply_to` and a unique correlation id, and awaits
the reply (the caller's original correlation id is restored on it). On expiry the call
fails with status **408**. `request(...).await` reads as synchronous but never blocks a
runtime thread — the calling task is suspended until the reply arrives.

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

> **Runnable example.** `greeting.api` in `examples/hello-world` (`GET /api/greeting/{user}`) is the
> whole idiom in one REST-bound function: build the event, `request(...).await?`, **check
> `has_error()` before reading the body** (a callee that fails replies with its error status and
> message), return the result. A timeout is not a reply — `request` returns `Err(AppError 408)`, and
> propagating it with `?` makes the function's own reply a 408 for the REST caller.

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

## Deterministic packaging: `canonical_packager`

`platform_core::canonical_packager` turns a set of maps into **one byte array whose bytes depend only on its content**: the same
maps give the same bytes whatever order their keys were built in, and the Java engine writes the same bytes. Use it when related
documents (graphs, flows, rules, tables) are promoted as one artifact whose identity must be recorded or compared. The format is
specified in [Canonical Package Format](canonical-package-format.md); this section is the Rust API.

```rust
use platform_core::canonical_packager::{self, Builder};
use rmpv::Value;

let bytes = Builder::new()
    .manifest("graph_id", "quote")?          // caller-defined string fields; format and format_version are reserved
    .manifest("version", "1.0.0")?
    .add("quote.json", quote)?               // a Value::Map: keys are sorted at every depth by the packager
    .add("quote-fees.json", fees)?
    .build()?;                               // the order of manifest() and add() calls does not matter

let package = canonical_packager::unpack(&bytes)?;      // strict: rejects bytes that are not canonical
let graph_id = package.manifest.iter().find(|(k, _)| k == "graph_id");
let quote: &Value = &package.maps[1].1;                 // ordered, the order the bytes hold
canonical_packager::unpack_with(&bytes, false)?;        // non-strict: decodes any valid content
```

- **The ordering is the packager's own step.** A `Value::Map` keeps the order it was built in, so the packager converts keys to
  text and sorts them in **UTF-8 byte order** at every depth (maps inside lists included); a `String` sort on the JVM would be
  UTF-16, which differs from this engine's bytes above U+FFFF.
- **The caller writes exact numbers and dates as strings**, because `rmpv::Value` has no decimal, big-integer or date type: a
  decimal in plain notation with its scale kept and a zero of any scale as `"0"`, a big integer as its digits, a date as
  ISO-8601. An integer above 2^63-1 is rejected (`PackagerError::Invalid`: write it as text), as is an `f32`, NaN, Infinity, an
  extension type and a null key.
- **The packager is faithful to a value's type**: the integer `1` and the float `1.0` are different content, and strings are
  written as given with no Unicode normalization.
- **Errors:** `PackagerError::Invalid` is a value or builder call the profile rejects; `PackagerError::Malformed` is bytes that are
  not a canonical package (a non-text or duplicate key, trailing bytes, a wrong `format` or `format_version`, nesting beyond 64
  levels, a package whose bytes differ from the canonical form of its content). `encode(&Value)` and `decode(&[u8])` expose the
  same profile for any value.
- **No integrity logic.** A hash or a signature, and its algorithm, is your application's decision: compute it over the exact
  bytes `build()` returns, for example a SHA-256 recorded in a release record, or a detached signature from your own signer.

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

## Utility: `ManagedCache`

The Java `ManagedCache` ported (design record: `draft-design-specs/managed-cache-port.md`) — a
named, **self-expiring** (expire-after-write), **size-bounded** in-memory cache with a
process-wide registry. The engine (moka, Caffeine's Rust lineage) is an internal detail;
eviction under capacity pressure is **deterministic LRU** — a deliberate, documented
improvement over the Java original's approximate W-TinyLFU.

```rust
use platform_core::ManagedCache;

// idempotent by name — the first creation's parameters win
let cache = ManagedCache::create_cache("my.cache", 5000);              // TTL ms (min 1000), max 2000 items
let sized = ManagedCache::create_cache_with_limit("big.cache", 60_000, 10_000);

cache.put("key", MyStruct { .. });                 // any Any + Send + Sync value
let value = cache.get_as::<MyStruct>("key");       // Option<Arc<MyStruct>> — None if absent or wrong type
let exists = cache.exists("key");
cache.remove("key");
cache.clear();

let same = ManagedCache::get_instance("my.cache"); // resolve by name from any module
let all = ManagedCache::get_cache_collection();    // sorted snapshot for introspection
```

Values are type-erased `Arc` handles (the Rust carrier of Java's `Object` reference
semantics) — store one value shape per named cache. For a value that is *already* an
`Arc`, use `put_arc` (passing an `Arc` to `put` would nest it and `get_as` would miss).
Expired entries are reclaimed lazily on access plus a 10-minute housekeeper the
application lifecycle starts automatically; correctness never depends on the sweep.

---

*Adapted from the mercury-composable guide `docs/guides/api-overview.md`; keys/APIs enumerated from this repository's source.*
