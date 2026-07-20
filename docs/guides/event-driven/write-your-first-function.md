# Write Your First Function

Build a working composable function, expose it over HTTP, call it from another function, and
run the application — every code sample on this page comes from the shipped
[`examples/hello-world`](https://github.com/Accenture/mercury/blob/main/examples/hello-world/src/main.rs)
application.

A composable function is **plain Rust** — no framework base type, no dependency-injection
container, no coupling to other functions. The framework constrains only *coupling*:
functions communicate through named routes and `EventEnvelope`, never by direct call.

## 1. Define the function

A function is a struct with a `#[preload]` attribute. There are two authoring surfaces.

**Typed** — implement `TypedFunction<I, O>` with your own `serde` types and add the `typed`
flag; the platform deserializes the input and wraps your output back into an envelope:

```rust
use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::{preload, AppError, TypedFunction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GreetingRequest {
    user: String,
}

#[derive(Serialize, Deserialize)]
struct GreetingResponse {
    message: String,
    handled_by_instance: usize,
}

#[preload(
    route = "greeting.demo",
    instances = 10,
    env_instances = "greeting.instances",
    typed
)]
struct Greetings;

#[async_trait]
impl TypedFunction<GreetingRequest, GreetingResponse> for Greetings {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: GreetingRequest,
        instance: usize,
    ) -> Result<GreetingResponse, AppError> {
        Ok(GreetingResponse {
            message: format!("Welcome, {}", input.user),
            handled_by_instance: instance,
        })
    }
}
```

**Untyped** — implement `ComposableFunction` to work directly with the raw `EventEnvelope`
(useful for HTTP-facing functions, pass-through, and free-form payloads). This is the
example's HTTP-facing function: it receives the request event from the REST edge, composes
with the typed `greeting.demo` through the `PostOffice` (step 4 explains the call), and
shapes the JSON response:

```rust
use std::time::Duration;
use platform_core::{ComposableFunction, EventEnvelope, Platform, PostOffice};

#[preload(route = "greeting.api", instances = 5)]
struct GreetingApi;

#[async_trait]
impl ComposableFunction for GreetingApi {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        let user = request["parameters"]["path"]["user"]
            .as_str()
            .unwrap_or("world")
            .to_string();
        let po = PostOffice::new(&Platform::get_instance());
        let reply = po
            .request(
                EventEnvelope::new()
                    .set_to("greeting.demo")
                    .set_body(GreetingRequest { user })?,
                Duration::from_secs(5),
            )
            .await?;
        let body: GreetingResponse = reply.body_as()?;
        EventEnvelope::new().set_body(serde_json::json!({
            "message": body.message,
            "handled_by_instance": body.handled_by_instance,
            "trace_id": po.my_trace_id(),
            "correlation_id": po.my_correlation_id(),
        }))
    }
}
```

In both forms, `handle_event` receives the event **headers** (`HashMap<String, String>`),
the **input**, and the 1-based worker **instance** number. Returning `Err(AppError)` becomes
an error reply with an HTTP-style status code (≥ 400 is an error). Functions must stay
stateless — the worker pool is the concurrency model, not your struct's fields.

## 2. Registration parameters

`route` (required)
:   The function's identity on the bus. Route names are **lowercase** letters, digits, `.`,
    `-`, and `_`, with **at least one dot** and no leading, trailing, or consecutive dots
    (`v1.my.function`, `greeting.demo`). An invalid name or a duplicate route fails the
    application at startup, so wiring mistakes never reach runtime.

`instances` (default 1)
:   How many concurrent workers the platform starts for the route. Each worker processes one
    event at a time; see [Function execution](function-execution.md) for sizing guidance.

`env_instances`
:   A configuration key to read the worker count from at startup (the value may itself use
    `${SOME_ENV:default}` syntax); the literal `instances` is the fallback. The example above
    lets `greeting.instances` in `application.yml` — or a `-Dgreeting.instances=` launch
    override — control the pool size.

`typed`
:   Declares that the struct implements `TypedFunction<I, O>` instead of
    `ComposableFunction`; the platform bridges it with an adapter.

Three more markers can stack with `#[preload]`: `#[zero_tracing]` (exclude the route from
distributed tracing), `#[event_interceptor]` (manual-reply mode — see
[Function execution](function-execution.md)), and `#[optional_service("condition")]`
(register only when a configuration condition holds, e.g. `"app.env=dev"`). The
[AI agent guide](ai-agent-guide.md) carries the full, engine-verified contract for all of
them — it is the reference; this page is the tour.

## 3. Expose it over HTTP

Declare the endpoint in `resources/rest.yaml` — the `service:` value is the route name, and
that string is the *only* link between the HTTP layer and your function:

```yaml
rest:
  - service: "greeting.api"
    methods: ['GET']
    url: "/api/greeting/{user}"
    timeout: 10s
    tracing: true
```

Enable REST automation in `resources/application.yml`:

```yaml
rest.automation: true
rest.server.port: 8085
```

The HTTP edge delivers an `AsyncHttpRequest`-shaped event to your function: the body carries
`method`, `url`, `ip`, `headers`, `parameters.path` / `parameters.query`, and `body` — which
is how `GreetingApi` above reads its `{user}` path parameter
(`request["parameters"]["path"]["user"]`).

## 4. Call it from another function

Look again at the middle of `GreetingApi`: to reach `greeting.demo` it never imports or
constructs anything — it asks the `PostOffice` for an RPC to a route name:

```rust
let po = PostOffice::new(&Platform::get_instance());
let reply = po
    .request(
        EventEnvelope::new()
            .set_to("greeting.demo")
            .set_body(GreetingRequest { user })?,
        Duration::from_secs(5),
    )
    .await?;
let body: GreetingResponse = reply.body_as()?;
```

`request(...).await` reads as synchronous but never blocks a runtime thread — the calling
task is suspended until the reply arrives (or the timeout expires with status 408). When the
call happens inside a traced request, the trace propagates to `greeting.demo` automatically —
the response's `trace_id` in step 6 is proof. For fire-and-forget delivery use `po.send`;
[Function execution](function-execution.md) covers the full messaging surface.

## 5. Bootstrap the application

An application declares its entry point with `#[main_application]` and generates its whole
`main()` with one line:

```rust
use platform_core::{main_application, EntryPoint};

#[main_application]
struct MainApp;

#[async_trait]
impl EntryPoint for MainApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!("hello-world started");
        Ok(())
    }
}

platform_core::auto_start_main!();
```

`auto_start_main!()` is the Java `AutoStart.main(args)` analog: it builds the tokio runtime,
loads `-Dkey=value` launch overrides, installs structured logging, collects every annotated
item from the link-time inventory, runs the lifecycle, and — when the application serves
HTTP — stays alive until Ctrl-C. The startup order matches the Java sequence exactly:

1. **Essential services** (sequence 0, framework-reserved) — the telemetry sink, actuators
   (`/info`, `/env`, `/health`, `/livenessprobe`), the `no.op` echo function, and the async
   HTTP client.
2. **Before-application hooks** (`#[before_application]`), ordered by `sequence` (1–999,
   lower first; user code conventionally uses 3–999). A failing hook **aborts startup**.
3. **Preload** — every `#[preload]` function is registered and bound to its route.
4. **REST automation** — the HTTP server starts when `rest.automation: true`.
5. **Main applications** (`#[main_application]`), ordered by `sequence`. A missing main
   application is a startup error.

A before-application hook does validation or compilation work that must precede
registration — the same `EntryPoint` trait, run earlier:

```rust
use platform_core::{before_application, AppConfigReader};

#[before_application(sequence = 5)]
struct PreflightCheck;

#[async_trait]
impl EntryPoint for PreflightCheck {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        let config = AppConfigReader::get_instance();
        if !config.exists("greeting.user") {
            return Err(AppError::new(
                500,
                "greeting.user missing from application.yml",
            ));
        }
        Ok(())
    }
}
```

(The Event Script engine uses exactly this mechanism: its flow compiler is a
before-application hook at sequence 5.)

### The `resources/` convention

Each application is a standalone workspace crate with its configuration in a `resources/`
folder beside its `Cargo.toml` — `auto_start_main!()` registers that folder as the
application's `classpath:/` root at compile time:

```text
examples/hello-world/
  Cargo.toml
  src/main.rs
  resources/
    application.yml     configuration (+ application-{profile}.yml overlays)
    rest.yaml           declarative HTTP endpoints
    public/             static content served at "/"
```

Configuration syntax is kept verbatim from the Java original (`classpath:/`, `file:/`,
`${ENV_VAR:default}`, dot/bracket composite keys), so config files port between the two
implementations unchanged.

!!! note "Rust port"
    The build tool is **cargo**, not maven, and the base configuration file is
    `application.yml` rather than `application.properties` (the properties *format* is also
    supported). Java's `@PreLoad` classpath scanning becomes the `#[preload]` link-time
    inventory, and JVM `-D` flags become `-Dkey=value` arguments after `--` on the cargo
    command line.

## 6. Run and test

```bash
cargo run -p hello-world
```

In another terminal:

```bash
curl -s http://127.0.0.1:8085/api/greeting/eric
```

```json
{
  "message": "Welcome, eric",
  "handled_by_instance": 3,
  "trace_id": "…",
  "correlation_id": "…"
}
```

Watch the console: the endpoint declared `tracing: true`, so the HTTP edge started a
distributed trace, and both functions' executions appear as spans in the telemetry records —
same trace id, parent-child span lineage, per-function timing.

## Next steps

- Orchestrate the function as a flow task — without changing a line of its code — in
  [Composable Orchestration](../event-script/index.md).
- Understand worker pools, RPC semantics, and interceptors in
  [Function execution](function-execution.md).
- Hand the [AI agent guide](ai-agent-guide.md) to an agent and let it write the next
  function for you.

---

*Adapted from the mercury-composable guide `docs/guides/event-driven/write-your-first-function.md`; behavior verified against this repository's source.*
