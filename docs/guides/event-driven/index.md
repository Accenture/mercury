# Event-Driven Foundation

Layer 1 — the event-driven core that everything above it is built on.

The building block is a **function**: a self-contained unit of business logic addressed only
by its **route name**. Functions hold no reference to any other user function; the only thing
that ever passes between them is an immutable **`EventEnvelope`** — metadata, headers, and a
body — over an **in-memory event bus**. Because coupling is a string, not an `import`, you can
replace, version, or relocate a function without touching its callers.

That decoupling is the point of the whole platform. The same function, unchanged, becomes a
**service** when `rest.yaml` maps an HTTP endpoint to its route, a **task** when an
[Event Script flow](../event-script/index.md) sequences it, and a **skill** when a knowledge
graph embeds it on a node. The layers above add composition; they never add coupling.

## When to work at this layer

Most application behavior lives higher up — flows are configuration and graphs are models.
You drop to Layer 1 when you are writing the *functions themselves*:

- business logic that a flow or graph will orchestrate,
- adapters at the system boundary (databases, external REST resources, notifications),
- request filters, authentication functions, and health checks,
- anything imperative that the declarative layers should treat as an atom.

## Functions, addressed by name

A function is a struct annotated with `#[preload]`, implementing either the untyped
`ComposableFunction` trait (envelope in, envelope out) or the typed `TypedFunction<I, O>`
(your own `serde` types in and out):

```rust
#[preload(route = "greeting.demo", instances = 10, typed)]
struct Greetings;
```

At startup the platform registers the struct on the event bus under its route name with a
pool of `instances` concurrent workers. From then on, every caller — an HTTP endpoint, a
flow task, a graph skill, another function — reaches it only by the string
`"greeting.demo"`.

!!! note "Rust port"
    The Java original discovers `@PreLoad` classes by classpath scanning at startup. Rust has
    no runtime scanning, so `#[preload]` performs **compile-time registration** through a
    link-time inventory — same annotation shape, resolved when the binary links. A duplicate
    route or an invalid route name still fails fast at startup.

## Talking through the event bus

Functions never call each other directly. They send events through the **`PostOffice`**, the
inter-function messaging client:

- **`send`** — fire-and-forget delivery to one worker instance of the target route.
- **`request`** — RPC: send and `.await` the reply, with a timeout.
- **`send_later`** — a scheduled, cancellable future delivery.

A `request(...).await` *reads* as synchronous — you send, and the reply arrives on the next
line — but it never blocks a runtime thread: the calling task is suspended until the reply
lands, and the worker pool keeps serving other events.

!!! note "Rust port"
    The Java engine runs on the Eclipse Vert.x event loop with Java 21 **virtual threads**
    making blocking-style code perform like reactive code. The Rust port reaches the same
    behavior natively with **tokio**: every function is an `async fn`, and `.await` *is* the
    suspend/resume mechanism — no separate thread-management strategy is needed. The Java
    guide's second delivery mode, **broadcast** to every instance of a route, is not ported:
    this port is deliberately single-runtime (no Kafka service mesh), and nothing in the
    in-scope layers needs it.

## The HTTP boundary

To expose a function over HTTP you don't write a controller — you declare the endpoint in
`rest.yaml`, where the `service:` value is the function's route name. **`rest.yaml` *is* the
router.** The [Getting Started](../getting-started.md) guide shows this end to end with the
`hello-world` example on port 8085.

## In this section

- **[Write your first function](write-your-first-function.md)** — the authoring walkthrough:
  typed and untyped functions, registration parameters, the application bootstrap, and the
  `resources/` convention, built from the real `hello-world` example.
- **[Function execution](function-execution.md)** — the execution semantics: worker-instance
  pools and back-pressure, `send` vs `request`, scheduled events, event interceptors, and how
  distributed tracing behaves on every call type.
- **[AI agent guide](ai-agent-guide.md)** — the agent-facing companion of this section. If an
  AI agent is writing functions for you, point it here: it is the engine-verified, single
  context an agent needs to author a correct composable function — the full `#[preload]`
  contract, the trait contracts, a pre-write checklist, and worked examples — with no need to
  read the engine source. It earned that claim in a thirteen-tutorial validation sweep where
  fresh agents built working applications from these documents alone.

## See also

- [Getting Started](../getting-started.md) — build the workspace and run `hello-world`.
- [Composable Orchestration](../event-script/index.md) — Layer 2: orchestrate these functions
  as YAML flows.

---

*Adapted from the mercury-composable guide `docs/guides/event-driven/index.md`; behavior verified against this repository's source.*
