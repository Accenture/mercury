# Getting Started

Build the workspace, run the three example applications — one per layer — and see the
platform's core ideas in ten minutes.

## Prerequisites

- **Rust** stable toolchain (the workspace builds on 1.95+): [rustup.rs](https://rustup.rs)
- **git**
- Optional: **Node.js** (only to rebuild the Playground web UI) and **uv** (only to build
  this documentation site locally)

## Build and test

```bash
git clone https://github.com/Accenture/mercury.git
cd mercury
cargo build
cargo test --workspace
```

Everything is a standard **Cargo workspace**: the three engine crates live under `crates/`
and each example application is a standalone crate under `examples/` with its configuration
in a `resources/` folder beside its `Cargo.toml`.

```text
crates/
  platform-core/      the event-driven foundation (layer 1)
  event-script/       the YAML flow engine (layer 2)
  knowledge-graph/    the active knowledge graph + Playground (layer 3)
examples/
  hello-world/        functions + REST automation (layer 1)
  hello-flow/         an Event Script flow behind an HTTP endpoint (layer 2)
  minigraph-playground/  the graph Playground web app (layer 3)
```

## Layer 1 — hello-world: functions and REST automation

```bash
cargo run -p hello-world
```

The application starts a REST endpoint on port **8085**. In another terminal:

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

Three ideas just happened:

**1. No controller code — `rest.yaml` *is* the router.** The endpoint is declared, not
programmed; the route string is the only link to the function:

```yaml
rest:
  - service: "greeting.api"
    methods: ['GET']
    url: "/api/greeting/{user}"
    timeout: 10s
    tracing: true
```

**2. Functions are self-contained actors.** A function is a struct with a `#[preload]`
attribute; the runtime registers it at the route with a pool of worker instances:

```rust
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
        // …
    }
}
```

**3. Functions never call each other directly.** `GreetingApi` composes with a second,
*typed* function through the `PostOffice` — coupling is a route name plus an immutable
`EventEnvelope`, nothing else:

```rust
let reply = po
    .request(
        EventEnvelope::new()
            .set_to("greeting.demo")
            .set_body(GreetingRequest { user })?,
        Duration::from_secs(5),
    )
    .await?;
```

The distributed trace you see in the response was started at the HTTP edge and propagated
through both functions automatically — check the console for the telemetry records.

The full example (typed functions, worker-pool sizing from configuration, log context,
a static-content filter) is in
[`examples/hello-world/src/main.rs`](https://github.com/Accenture/mercury/blob/main/examples/hello-world/src/main.rs).

!!! note "Rust port"
    The Java original registers functions by classpath scanning (`@PreLoad`). Rust has no
    runtime scanning, so `#[preload]` performs **compile-time registration** — same
    annotation shape, resolved at link time.

## Layer 2 — hello-flow: orchestration as configuration

```bash
cargo run -p hello-flow
curl -s http://127.0.0.1:8100/api/hello/eric
```

The endpoint binds to a **flow** instead of a function (`flow: 'hello-flow'` in its
`rest.yaml`), and the flow YAML in
[`examples/hello-flow/resources/flows/`](https://github.com/Accenture/mercury/blob/main/examples/hello-flow/resources/flows/hello-flow.yml)
sequences the work: tasks, data mapping between a per-transaction state machine, and an
exception handler — orchestration without orchestration code. The flow syntax is **identical
to the Java original**, so flow files port unchanged. See the
[flow grammar](event-script/flow-grammar.md) for the full DSL.

## Layer 3 — the Playground: graphs that execute

```bash
cargo run -p minigraph-playground
```

Open <http://127.0.0.1:8100> in a browser. The Playground console builds **active knowledge
graphs** interactively — try:

```text
list graphs
help tutorial 1
```

`list graphs` shows the deployable graph models with their purposes (including thirteen
tutorials that take you from hello-world to custom error handling and composable-function
tasks); each `help tutorial N` page walks one concept. An AI agent can drive the same console
through the synchronous companion endpoint — see the
[AI agent guide](knowledge-graph/ai-agent-guide.md).

## Configuration in one paragraph

Every application reads layered configuration from its `resources/` folder —
`application.yml` (+ `application-{profile}.yml` overlays selected by `APP_PROFILES_ACTIVE`),
with `${ENV_VAR:default}` substitution and dot/bracket composite keys. Config file **syntax**
is kept verbatim from the Java original so files port between the two implementations; the
Spring-specific key names are retired in the Rust port (`APP_PROFILES_ACTIVE`,
`application.name`).

## Next steps

- Write your own function: [the authoring walkthrough](event-driven/write-your-first-function.md)
  covers typed/untyped functions, `#[preload]` parameters, and the application lifecycle.
- Explore the thirteen Playground tutorials (`help tutorial 1` … `help tutorial 13`), then
  [build your first graph](knowledge-graph/build-your-first-graph.md).
- Understand the whole: [Architecture](architecture.md) and
  [Composing the Layers](knowledge-graph/composing-the-layers.md).
- Point an AI agent at [`docs/llms.txt`](../llms.txt) and let it build a graph for you.
