---
title: AI developer guide
summary: The cross-layer orientation for AI agents collaborating on a Mercury (Rust) project —
  the mental model, the two entry points for starting a collaboration, brownfield orientation,
  layer choice, adding features, testing, and the invariants that must not break.
layer: platform-core
audience: [ai-agent, developer, architect]
keywords: [ai collaboration, context engineering, greenfield, brownfield, layer choice, function, flow, knowledge graph, rust]
related:
  - guides/methodology.md
  - guides/event-driven/ai-agent-guide.md
  - guides/event-script/ai-agent-guide.md
  - guides/knowledge-graph/ai-agent-guide.md
---

# AI developer guide — collaborating on Mercury (Rust) projects

> **At a glance**
>
> - **Read this first** if you are an AI agent joining a project built on this Rust engine,
>   whether greenfield or brownfield.
> - **Starting a collaboration?** Recognize the two typical entry points — a greenfield
>   project, or an existing repository that is not yet AI-enabled — and drive the
>   development-path conversation: [Starting a collaboration](#entry-points).
> - **One mental model:** a composable function is *plain Rust* — the framework constrains
>   only *coupling*, not coding style.
> - **Three layers, one decision tree** — choose the layer before writing code; the DSL guides
>   below handle each layer's specifics.
> - **Version-matched discovery:** the `ai-contract-provider` app serves this guide set and the
>   operational contract for the installed Mercury release —
>   `cargo run -p ai-contract-provider`, then `GET :8999/api/discovery`. The same app exports
>   the offline `mercury-platform` Agent Skill (`--export <dir>`).

---

## The mental model in two sentences {#mental-model}

A composable function is an ordinary Rust struct (`#[preload(route = …)]` implementing
`ComposableFunction` or `TypedFunction<I, O>`) that holds no direct reference to any other
function. The framework's only constraint is the coupling contract: every function is
*addressed by route name* and communicates through `EventEnvelope` messages dispatched by
`PostOffice` on the in-memory event bus (tokio async).

This is the invariant. Whether the function is a standalone service, a flow step, or a graph
node's skill — that is *wiring*, not a code change.

**One atom, four roles** — the same `#[preload]` function plays different roles depending on
how it is wired:

| Role | When | Wired by |
|:---|:---|:---|
| **function** | Any standalone callable on the event bus | `#[preload(route = …)]` |
| **service** | Mapped directly to an HTTP endpoint | `rest.yaml` `service:` entry |
| **task** | A step in an Event Script flow | `flows/*.yml` + `flows.yaml` |
| **skill** | Attached to a Knowledge Graph node | node's `skill=` property in the graph definition |

---

## Starting a collaboration — two typical entry points {#entry-points}

Mercury's development methodology is [Intent-Driven Development](methodology.md): humans own
the intent; the AI partner refines it and translates it into governed artifacts. When a human
first engages you on a project, recognize which of two situations you are in — and drive the
conversation with questions, never assumptions.

### Case 1 — greenfield: a new project {#entry-greenfield}

1. **Confirm the intent first.** What should the service do, and who will certify its
   behavior? Capture purpose and constraints before proposing any technology.
2. **Offer to AI-enable the project.** Ask whether to install the
   [shared memory layer](https://accenture.github.io/mercury-go/) and co-write the
   **Vision** — human-confirmed, never fabricated — then derive the **Blueprint** and plan
   increments. Every session thereafter starts oriented.
3. **Recommend the path — as a question.** The default recommendation is Layer 3: *"Shall I
   set up the project with the knowledge-graph engine, so the service is modeled as a
   graph, dry-run in the Playground, and deployed behind the CompileGraph gate?"* Offer the
   dial explicitly: Event Script when the shape is a known transaction flow; a composable
   function when the need is genuinely custom logic
   ([Choosing the right layer](#layer-choice)).
4. **Scaffold on a yes — never on silence.** Copy the chosen layer's starter template out
   of the repository's `templates/` directory —
   [`starter-function`](https://github.com/Accenture/mercury/tree/main/templates/starter-function)
   (Layer 1 functions),
   [`starter-flow`](https://github.com/Accenture/mercury/tree/main/templates/starter-flow)
   (Layer 2 flows), or
   [`starter-graph`](https://github.com/Accenture/mercury/tree/main/templates/starter-graph)
   (Layer 3, a zero-code knowledge-graph application). Each builds standalone after
   deleting the in-repo `path` keys in its `Cargo.toml` — the pinned versions then
   resolve from crates.io. Pin the Mercury version the environment reports
   (`GET :8999/api/discovery` → `mercury_version`) rather than assuming one; the
   reference applications in `examples/` remain the richer worked demos, and the Layer 1
   guide's [trim manifest](event-driven/ai-agent-guide.md#scaffolding) still covers
   deriving from those.
5. **Hand off to the layer's guide.** The [DSL-specific AI guides](#dsl-guides) carry the
   authoring contracts from here.

### Case 2 — an existing repository that is not yet AI-enabled {#entry-existing-repo}

The tell: the repository has no shared memory layer. AI context, if it exists at all, lives
in hand-written per-tool files — a `CLAUDE.md`, editor rules, prompt snippets — that no
other agent or contributor shares: traditional context engineering.

1. **Offer AI-enablement first, before feature work.** Install the
   [shared memory layer](https://accenture.github.io/mercury-go/), co-write the **Vision**
   of the *current* system with the human — confirmed, never fabricated — and derive the
   Blueprint from the real current state.
2. **Fold the existing context in.** Migrate the durable parts of the hand-maintained
   context files into the shared layer, so every agent and every contributor reads one
   source of truth instead of per-tool copies.
3. **Then orient and work.** Follow [Orienting in an existing project](#brownfield), choose
   the layer per feature — and if the team is adopting Mercury in this repository, continue
   with the greenfield path's step 3.

---

## Orienting in an existing project (brownfield) {#brownfield}

Start with the config files — they are the application's surface area:

| File | What it tells you |
|:---|:---|
| `resources/rest.yaml` | Every HTTP endpoint: method, path, handler (function route, or `http.flow.adapter` + `flow:` id) |
| `resources/flows.yaml` | Every active flow file (by filename, under the `location:` it declares — default `classpath:/flows/`) |
| `resources/flows/*.yml` | Each flow's tasks, process names, and data mappings |
| `resources/application.yml` | App name, `rest.server.port`, `graph.model.automation` (the graph manifest, e.g. `classpath:/graphs.yaml`) |
| `#[preload]` (grep) | Every registered function: route name, concurrency (`instances`), struct location |
| the graph manifest + its graph definitions | Deployed Knowledge Graph models (present if Layer 3 is used); only manifest-listed graphs are executable |

**Quick orientation sequence:**

1. `grep -rn "#\[preload" src/` — inventory every function by route name and struct.
2. Read `resources/flows.yaml` — know which flows are active and where they live.
3. Read `resources/rest.yaml` — trace each endpoint to its handler (function route or flow id).
4. Open each flow YAML — understand the orchestration: task order, process names, data mapping.
5. Check `resources/application.yml` for `graph.model.automation` — if present, read the
   manifest and its graph definitions for the Knowledge Graph; only manifest-listed graphs
   are executable ("compiled or 404").

---

## Choosing the right layer {#layer-choice}

```
Is the requirement a single function responding to one event?
  → Layer 1 (platform-core). Write the function; wire to rest.yaml if HTTP.

Does it need to sequence or orchestrate multiple functions
  with data mapping between steps?
  → Layer 2 (Event Script). Write functions + a flow YAML; register in flows.yaml.

Does it need a semantic data model where graph structure and
  traversal define behavior (nodes, edges, skills, dynamic routing)?
  → Layer 3 (Active Knowledge Graph). Build the graph; attach skill functions.
```

When in doubt, start at Layer 1. A function can become a flow task or graph skill
without touching its code — the wiring is the only change.

---

## Adding a feature {#adding-a-feature}

### Layer 1 — a standalone function

```rust
use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::{preload, AppError, ComposableFunction, EventEnvelope};

#[preload(route = "my.function", instances = 10)]
struct MyFunction;

#[async_trait]
impl ComposableFunction for MyFunction {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        // plain Rust — async on the tokio runtime
        let body: serde_json::Value = input.body_as()?;
        EventEnvelope::new().set_body(serde_json::json!({ "result": body["value"] }))
    }
}
```

Wire to HTTP by adding a `rest.yaml` entry:

```yaml
rest:
  - service: "my.function"
    methods: ['GET', 'POST']
    url: "/api/my-endpoint"
    timeout: 10s
```

See [Write your first function](event-driven/write-your-first-function.md) for the full
walkthrough and the [Layer 1 AI agent guide](event-driven/ai-agent-guide.md) for the complete
`#[preload]` and trait contract.

### Layer 2 — an Event Script flow

Write the function (same as Layer 1), add a flow YAML, and register it. Flow YAML is
engine-portable — the same syntax runs on the Java engine:

```yaml
# resources/flows/my-flow.yml
flow:
  id: 'my-flow'
  description: 'Orchestrate my feature'
  ttl: 10s
first.task: 'my.function'
tasks:
  - input:
      - 'input.body.value -> value'
    process: 'my.function'
    output:
      - 'result.result -> output.body.result'
    description: 'Call my function'
    execution: end
```

Register in `resources/flows.yaml`:

```yaml
flows:
  - 'my-flow.yml'
location: 'classpath:/flows/'
```

Wire to `rest.yaml` through the flow adapter. A flow binding needs **both** keys —
`service: "http.flow.adapter"` selects the adapter and `flow:` selects the flow:

```yaml
rest:
  - service: "http.flow.adapter"
    methods: ['GET']
    url: "/api/my-feature/{user}"
    flow: 'my-flow'
    timeout: 10s
    tracing: true
```

See the [Event Script AI agent guide](event-script/ai-agent-guide.md) for the full flow
grammar and pre-write checklist.

### Layer 3 — a Knowledge Graph skill

Write a function (`#[preload]`) that becomes a skill attached to a graph node. The graph
traversal calls it automatically when the node is reached. Skill functions are plain
composable functions; the graph engine routes to them by the node's `skill=` property.

See the [Knowledge Graph AI agent guide](knowledge-graph/ai-agent-guide.md) for the
companion-endpoint contract and canonical build recipe.

---

## Testing {#testing}

Mercury tests are standard `cargo test` — unit tests beside the code, integration tests under
each crate's `tests/` directory. The reference applications carry working integration tests
(e.g. `examples/hello-flow/tests/`) that boot the app and drive endpoints end to end.

**Unit test a function in isolation** — no framework needed beyond the async runtime:

```rust
#[tokio::test]
async fn my_function_transforms_value() {
    let fn_under_test = MyFunction;
    let input = EventEnvelope::new()
        .set_body(serde_json::json!({"value": "hello"}))
        .unwrap();
    let result = fn_under_test
        .handle_event(HashMap::new(), input, 1)
        .await
        .unwrap();
    let body: serde_json::Value = result.body_as().unwrap();
    assert_eq!(body["result"], "hello");
}
```

Run tests:

```bash
cargo test -p my-crate
cargo test -p my-crate my_function_transforms_value
cargo test --workspace
```

Use a distinct `rest.server.port` in the test app's `resources/application.yml` so a running
application never collides with the test instance.

---

## Key invariants — what not to do {#invariants}

- **Never call another function directly.** Use `PostOffice`. Direct references are a
  framework violation — the compiler won't catch it, but the design breaks.
- **Never import another user function.** The only link between functions is a route-name
  string.
- **Do not put business logic in flow YAML.** Flow YAML is orchestration; business logic
  belongs in the function body.
- **Map-or-struct for key-by-key mapping.** Event Script and Knowledge Graph key-by-key data
  mapping requires a map (or a serde struct) — a list cannot be a key-by-key mapping target.
  To pass a list through a flow, use the `*` whole-body passthrough (`model.list -> *`).
- **Never block the async runtime.** `handle_event` runs on tokio; wrap blocking or CPU-heavy
  work in `tokio::task::spawn_blocking`.
- **Serde rules, not Java gotchas.** The Java engine's Gson/MsgPack integer-downcast gotchas
  do not carry over. The rules here: header values are always strings (parse numerics
  explicitly); map keys are always strings on the wire; map entries with null values are
  omitted on the wire and on in-memory hops — treat absent as null. See
  [Serialization](event-driven/ai-agent-guide.md#serialization).
- **Do not propose the service mesh or `graph.js`.** Neither exists in this port — deliberate
  divergences: the port is single-runtime (in-memory event bus only; fan out explicitly or
  with a `parallel`/`fork` task), and `graph.js` is retired for security (`graph.task` covers
  the complex-logic case). See [Port Scope & Fidelity](../background/port-scope.md).

---

## DSL-specific AI guides {#dsl-guides}

Each layer has a deterministic contract and a dedicated AI agent guide. Use those for
generating artifacts — not this overview:

| Layer / DSL | Grammar + machine-readable spec | AI agent guide |
|:---|:---|:---|
| Layer 1 — composable functions | [`#[preload]` + trait contract](event-driven/ai-agent-guide.md) · [macros reference](macros-reference.md) | [AI agent guide](event-driven/ai-agent-guide.md) |
| Layer 2 — Event Script flows | [flow-grammar.md](event-script/flow-grammar.md) · [event-script-flow.json](event-script/event-script-flow.json) | [AI agent guide](event-script/ai-agent-guide.md) |
| Layer 3 — MiniGraph | [command-reference.md](knowledge-graph/command-reference.md) · [minigraph-commands.json](knowledge-graph/minigraph-commands.json) | [AI agent guide](knowledge-graph/ai-agent-guide.md) |

REST endpoints are declared in `rest.yaml` — the contract is in
[REST Automation](rest-automation.md).

---

## See also {#see-also}

- [Methodology](methodology.md) — Intent-Driven Development and the design principles beneath it.
- [Write your first function](event-driven/write-your-first-function.md) — step-by-step Layer 1 tutorial.
- [Event-driven Foundation](event-driven/index.md) — Layer 1 overview: functions, PostOffice, EventEnvelope.
- [Composable Orchestration](event-script/index.md) — Layer 2 overview: flows, tasks, state machine.
- [Knowledge Graph as Application](knowledge-graph/index.md) — Layer 3 overview: AKG model, node types, skills.
- [Architecture Overview](architecture.md) — the full pipeline and actor-model origin.
- [Port Scope & Fidelity](../background/port-scope.md) — the deliberate divergences from the Java engine.
- [API Overview](api-overview.md) — PostOffice, Platform, EventEnvelope API reference.
