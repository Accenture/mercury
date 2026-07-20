---
title: AI agent guide — writing composable functions (Layer 1)
summary: The authoritative context an AI agent needs to write composable functions for this Rust
  port correctly — the preload attribute contract, the trait contracts, a pre-write checklist,
  PostOffice RPC patterns, and worked examples.
layer: platform-core
audience: [ai-agent, developer]
keywords: [ai agent, composable function, preload, ComposableFunction, TypedFunction, PostOffice, EventEnvelope, layer 1, rust]
related:
  - guides/event-script/ai-agent-guide.md
  - guides/event-script/syntax.md
---

# AI agent guide — writing composable functions (Layer 1)

> **At a glance**
>
> - **Read this if you are an AI agent** asked to write or review a composable function for this
>   Rust port. It is the single context you need — you should not need to read the engine source.
> - **Generate from contracts.** The `#[preload]` attribute and the `ComposableFunction` trait are
>   the source of truth. Validate against the checklist below before proposing code.
> - A composable function is **plain Rust** — no framework base type, no DI container, no coupling
>   to other functions by import. The only constraints are the `#[preload]` attribute and the trait.
> - This is the Rust analog of the Java engine's `@PreLoad` + `TypedLambdaFunction` contract; the
>   *concepts* are identical, the code is not — do not paste Java patterns into Rust.

---

## How functions are loaded {#deploy}

Functions are **not called directly**. The framework loads them at startup:

1. Annotate a struct with `#[preload(route = "…")]` (a unit struct, or one implementing `Default`).
2. Implement the `ComposableFunction` trait for it (or `TypedFunction<I, O>` — see below).
3. Registration is a **link-time inventory** (the Java classpath-scan analog): every `#[preload]`
   struct compiled into the application — including from library crates it depends on — registers
   itself on the in-memory event bus under its `route` name at startup, with `instances`
   concurrent workers.
4. Any caller (REST endpoint, Event Script flow, knowledge-graph skill, another function)
   addresses the function **only by its route name string**.

A duplicate route causes the application to fail at startup — correctness is checkable before
runtime. All routes are process-local: this port is deliberately single-runtime (in-memory event
bus only; no Kafka service mesh), so there is no private/public route distinction.

---

## `#[preload]` attribute — full contract {#annotation}

```rust
#[preload(
    route = "my.function",           // REQUIRED — unique route name (dot-separated, lowercase)
    instances = 10,                  // optional — concurrent workers (default 1)
    env_instances = "my.fn.workers", // optional — configuration key to read instances from
    typed,                           // optional flag — struct implements TypedFunction<I, O>
    zero_tracing,                    // optional flag — exclude from distributed tracing
    interceptor,                     // optional flag — event-interceptor mode (raw envelope)
)]
struct MyFunction;
```

| Parameter | Type | Default | Notes |
|:---|:---|:---|:---|
| `route` | string | **required** | Unique; one route per struct. Dot-separated lowercase convention (`v1.my.function`). |
| `instances` | int | `1` | Concurrent workers. Production services typically use 10–100. |
| `env_instances` | string | — | A key in the application configuration to read the worker count from at startup (the value may itself use `${SOME_ENV:default}` syntax); falls back to `instances`. |
| `typed` | flag | off | The struct implements `TypedFunction<I, O>` instead of `ComposableFunction`; the engine bridges it with a `TypedAdapter`. |
| `zero_tracing` | flag | off | Exclude this route from distributed tracing (also stackable as `#[zero_tracing]`). |
| `interceptor` | flag | off | Event-interceptor mode: the function sees the raw incoming `EventEnvelope` and the engine does **not** auto-reply on success (failures still route to the caller). Also stackable as `#[event_interceptor]`. |

*(The Java engine's `isPrivate`, `inputPojoClass`, `customSerializer`, and
`inputStrategy`/`outputStrategy` parameters have no Rust equivalent — everything is process-local,
and serde handles typing; see [Serialization](#serialization).)*

### `#[optional_service]` — conditional registration (first-class) {#optional-service}

The Java `@OptionalService` analog is its **own attribute**, not a `#[preload]` parameter. Stacked
with any of the four registration attributes — `#[preload]`, `#[websocket_service]`,
`#[before_application]`, `#[main_application]` — it makes the item **conditional on application
configuration**: it registers/runs only when the condition holds at startup, in either stacking
order:

```rust
#[optional_service("app.env=dev")]     // condition may sit above …
#[preload(route = "dev.only.service")]
struct DevOnlyService;

#[preload(route = "also.dev.only")]    // … or below the registration attribute
#[optional_service("app.env=dev")]
struct AlsoDevOnly;
```

Condition semantics (Java `Feature.isRequired` parity): `key=value` / bare `key` (present) /
`key=` (blank), `!key` negation, comma-separated alternatives (OR), case-insensitive values.

---

## Trait contract {#interface}

### `ComposableFunction` — the standard form

```rust
#[async_trait]
pub trait ComposableFunction: Send + Sync {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        instance: usize,
    ) -> Result<EventEnvelope, AppError>;
}
```

- `headers` — the event headers. Lookup is exact-match; headers arriving from the HTTP boundary
  are lowercased, so use lowercase keys.
- `input` — the incoming envelope. Read the payload with `input.body_as::<T>()` (any
  `serde`-deserializable type — `serde_json::Value` for free-form access, your own struct, a
  `Vec<T>`), or `input.body()` for the raw `rmpv::Value`. For key-by-key data mapping
  (Event Script / Knowledge Graph), the body must be a map (or a struct) — not a list.
- `instance` — the worker number, **1-based** (1 to `instances`).
- Return `Ok(EventEnvelope)` — build it with `EventEnvelope::new().set_body(value)` (any
  `serde`-serializable value; note `set_body` returns `Result`, so it can be the tail expression)
  plus optional `.set_status(code)` / `.set_header(k, v)`.
- **Return `Err(AppError::new(status, message))` for errors** — the framework catches it and
  returns an error envelope to the caller (HTTP-style status codes; ≥400 = error). No panic
  escapes to crash the process, but idiomatic code returns `Err`, never panics.

### `TypedFunction<I, O>` — the typed form

```rust
#[async_trait]
pub trait TypedFunction<I, O>: Send + Sync
where I: DeserializeOwned + Send, O: Serialize {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: I,
        instance: usize,
    ) -> Result<O, AppError>;
}
```

Declare with `#[preload(route = "…", typed)]`; the engine's `TypedAdapter` does the
`body_as::<I>()` / `set_body(O)` conversion at the boundary (the `TypedLambdaFunction<I, O>`
analog). Use it when a strict input/output contract helps; use `ComposableFunction` when you need
envelope details (status, headers on the reply) or varying body shapes.

---

## Pre-write checklist {#checklist}

> **Validate before proposing code:**
> - [ ] `route` is set and unique across the application; dot-separated lowercase convention.
> - [ ] `instances` is appropriate for concurrency needs (default `1`; typical services `10–100`).
> - [ ] The struct is a unit struct or implements `Default`.
> - [ ] `impl ComposableFunction` is wrapped in `#[async_trait]`.
> - [ ] Input body is a map/struct when the function participates in key-by-key data mapping —
>       not a list.
> - [ ] The function holds **no direct reference to another user function** (no
>       `OtherFunction::…` calls) — coupling is route-name + envelope only.
> - [ ] Errors are returned as `Err(AppError::new(status, message))` — no `unwrap()`/`expect()`
>       on input data, no panics.
> - [ ] The function is stateless; anything kept across calls (a cache, a counter) lives in an
>       explicit `static` with interior mutability, not in `&self` fields.
> - [ ] Blocking or CPU-heavy work is wrapped in `tokio::task::spawn_blocking` — `handle_event`
>       runs on the async runtime and must not block it.

---

## Patterns {#patterns}

### Standard function — free-form JSON I/O

```rust
use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::{preload, AppError, ComposableFunction, EventEnvelope};

#[preload(route = "hello.function", instances = 10)]
struct HelloFunction;

#[async_trait]
impl ComposableFunction for HelloFunction {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as()?;
        let name = body["name"].as_str().unwrap_or("world");
        EventEnvelope::new().set_body(serde_json::json!({
            "message": format!("Hello, {name}!"),
        }))
    }
}
```

### Typed function — struct I/O

```rust
#[derive(serde::Deserialize)]
struct Profile { name: String, address: String }

#[derive(serde::Serialize)]
struct ProfileConfirmation { id: String, name: String }

#[preload(route = "v1.create.profile", instances = 10, typed)]
struct CreateProfile;

#[async_trait]
impl platform_core::TypedFunction<Profile, ProfileConfirmation> for CreateProfile {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: Profile,          // already deserialized by the TypedAdapter
        _instance: usize,
    ) -> Result<ProfileConfirmation, AppError> {
        Ok(ProfileConfirmation { id: new_id(), name: input.name })
    }
}
```

### Event interceptor — raw envelope, no auto-reply

```rust
#[preload(route = "my.interceptor", instances = 5, interceptor)]
struct MyInterceptor;

#[async_trait]
impl ComposableFunction for MyInterceptor {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,     // the original envelope, incl. reply_to/cid metadata
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        // forward, log, or transform; the engine does not send your Ok() back to the caller
        EventEnvelope::new().set_body("ok")
    }
}
```

### Blocking or CPU-heavy work

There is no kernel-thread annotation in this port — `handle_event` is `async` on the tokio
runtime. Wrap anything blocking (legacy sync I/O, heavy computation) so it doesn't stall the
event loop:

```rust
let result = tokio::task::spawn_blocking(move || expensive_sync_work(data))
    .await
    .map_err(|e| AppError::new(500, e.to_string()))?;
```

---

## Calling another function (PostOffice) {#postoffice}

```rust
use platform_core::{Platform, PostOffice};

let po = PostOffice::new(&Platform::get_instance());
```

**Trace context is automatic.** When called from inside a traced function, the platform propagates
the current trace id/path, span parentage, and business correlation-id onto outbound events —
there is no per-request `PostOffice(headers, instance)` construction rule as in the Java engine.
`po.my_trace_id()` / `my_correlation_id()` / `annotate_trace(k, v)` expose the current context.

### RPC (request-response)

```rust
use std::time::Duration;

let response = po.request(
    EventEnvelope::new()
        .set_to("target.route")
        .set_body(serde_json::json!({"key": "value"}))?,
    Duration::from_secs(5),                 // timeout → Err(AppError 408)
).await?;

if response.has_error() {                   // status >= 400
    return Err(AppError::new(response.status(), format!("{:?}", response.body())));
}
let result: serde_json::Value = response.body_as()?;
```

### Fire-and-forget

```rust
po.send(
    EventEnvelope::new().set_to("target.route").set_body(payload)?,
).await?;   // 400 = missing 'to'; 404 = route not registered
```

### Parallel RPC (multiple targets)

There is no batch-request API; compose futures:

```rust
let (a, b) = tokio::join!(
    po.request(event_a, Duration::from_secs(5)),
    po.request(event_b, Duration::from_secs(5)),
);
```

### Scheduled delivery

```rust
let timer_id = po.send_later(event, Duration::from_secs(30)); // returns a cancellable timer id
po.cancel_future_event(&timer_id);
```

> **No broadcast / multicast in this port.** The Java engine's `po.broadcast()` (service mesh) and
> `multicast.yaml` (local fan-out) are not ported — the Kafka service mesh is out of scope, and
> local fan-out has not been needed yet. Fan out explicitly (multiple `send`s), or orchestrate with
> an Event Script `parallel`/`fork` task.

---

## Serialization {#serialization}

Typing is serde end-to-end (`rmpv::Value` on the bus — MsgPack-compatible; JSON at HTTP
boundaries). The Java engine's Gson/MsgPack integer-downcast gotchas do not carry over, but note:

| Scenario | Rule |
|:---|:---|
| Reading the body | `body_as::<T>()` converts numbers per serde rules; deserialize into the type you want (`i64`, `f64`, your struct) rather than matching raw `rmpv::Value` variants |
| Header values | always `String` — parse numerics explicitly (`str::parse`), defaulting on failure |
| Map keys | always strings on the wire — never rely on non-string keys |
| Null handling | map entries with null values are **omitted** on the wire by default (`serializer.null.transport=false`, Java parity) — treat absent as null when reading |
| Field-name style | put `#[serde(rename_all = "…")]` on your own types; there are no engine-level snake/camel switches |

---

## Worked example — full function + HTTP wiring {#example}

```rust
// 1. The function (in an examples/<name>/ app crate)
#[preload(route = "greeting.function", instances = 10)]
struct GreetingFunction;

#[async_trait]
impl ComposableFunction for GreetingFunction {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as()?;
        let name = body["name"].as_str().unwrap_or("world");
        EventEnvelope::new().set_body(serde_json::json!({
            "greeting": format!("Hello, {name}!"),
        }))
    }
}

// 2. The application entry point (once per app)
platform_core::auto_start_main!();
```

```yaml
# 3. Wire to HTTP in resources/rest.yaml
rest:
  - service: "greeting.function"
    methods: ['GET', 'POST']
    url: "/api/greeting"
    timeout: 10s
```

```bash
# 4. Test (rest.server.port from resources/application.yml)
curl -s -X POST http://127.0.0.1:8086/api/greeting \
     -H "content-type: application/json" \
     -d '{"name": "Mercury"}'
# → {"greeting": "Hello, Mercury!"}
```

---

## See also {#see-also}

- [Event Script AI agent guide](../event-script/ai-agent-guide.md) — orchestrate this function as a flow task.
- [Event Script Syntax](../event-script/syntax.md) — the flow DSL and data-mapping catalog.
- [MiniGraph command grammar](../knowledge-graph/command-reference.md) — call it from a graph via `graph.task`.
- `examples/hello-flow/` and `examples/hello-world/` — working applications using this contract.
