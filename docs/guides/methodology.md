# Composability Methodology

Software development is a long fight against complexity — and most of that complexity is
*coupling*: code that cannot change without breaking something else. Composable methodology
attacks coupling at its source. You build an application from **self-contained functions**
that know nothing about one another, then wire them together with **configuration instead
of code**. Functions become plug-and-play: you mix and match them into new applications,
run multiple versions side by side, and retire one without side effects on the rest. The
[Architecture Overview](architecture.md) is the technical companion to this page.

## From product design to running code

A composable project starts from **product design, not technology**. The output of product
design is a business transaction's **event flow diagram** — typically from an Event Storming
workshop or a whiteboard session among product owners, domain experts, and architects. The
diagram has a start and an end; each processing step is a **task** (a function in its role
as a flow step); some tasks calculate, some decide the next step.

That diagram is not a picture that gets translated into something else — in Event Script it
*is* the application's orchestration, expressed as a YAML flow. Data attributes in the
events flowing between tasks give product designers and engineers a shared, precise
vocabulary, closing the traditional gap between business intent and technical design.

## The four principles

**First — input-process-output.** Each function is self-contained, and its input and output
are immutable. Given input, it performs its business logic and returns output; it cannot
reach outside its functional scope or mutate a business object it does not own, which
eliminates unintended side effects by construction.

**Second — zero to one dependency.** A function has zero dependencies on other *user*
functions. To reach the world outside its scope it may depend on at most one platform or
infrastructure component — and it consumes that component by **sending an event**, never by
a direct method call. In composable design, a library can be packaged as a reusable
composable function.

**Third — platform abstraction.** Platform and infrastructure are encapsulated as adapters,
gateways, and wrappers. The HTTP flow adapter serves inbound requests and outbound
responses; a custom adapter can front any event source. Because functions are self-contained
and independent, they can be repackaged into different applications — plug-and-play by
definition.

**Fourth — event choreography.** Without direct coupling, the framework must route events
between functions according to the event flow diagram for each use case. That is Event
Script's job: choreography is an **Event Flow Configuration**, executed by the flow engine
with a per-transaction state machine and declarative input/output data mapping.

## What decoupling buys you

- **Maintainability** — isolated functions are easy to understand, test, and change.
- **Reusability** — the same function serves as an HTTP service, a flow task, or a graph
  skill, unchanged.
- **Performance** — loose coupling enables asynchronous, parallel execution without
  bottlenecks (the ported bus benchmarks at ~155K RPC ops/s at 6 µs round trip).
- **Testability** — explicit input/output contracts make unit tests straightforward, and
  integration mocks are just mock functions assigned to tasks in a flow.
- **Debuggability** — independent functions and end-to-end traces make faults easy to
  localize (see the [Observability Model](observability.md)).
- **Technology freedom** — inside a function you may use any style or library; nothing
  leaks across the boundary.

## Zero code by default, escape hatches by design

The three layers form a ladder of defaults. At the top, an **active knowledge graph is the
application**: a graph model executes behavior through skills embedded on nodes, so the
common case — data sourcing, mapping, decisions, iteration, composition — needs **zero
imperative code** (the [skills reference](knowledge-graph/skills-reference.md) catalogs
what nodes can do). One rung down, **Event Script** makes orchestration configuration: the
flow YAML sequences functions, maps data, and handles exceptions declaratively (the
[flow grammar](event-script/flow-grammar.md) is the contract). At the bottom, when genuine
custom logic is needed, you write a **composable function** in Rust and address it from a
flow (`process:`) or from a graph node — the deliberate seam between the model and code.

This is explicitly **not a "no code ever" dogma**. Zero-code is the default; Event Script
and custom functions remain first-class escape hatches. The discipline is that the escape
hatch is always a *function with an explicit contract*, never a side channel.

## Evolving systems by editing models, not rewriting code

Because orchestration lives in configuration, a system evolves by **editing the model that
describes it**. Reordering tasks, adding a validation step, changing a decision branch, or
rerouting an error handler is a YAML edit and a redeploy — the functions themselves do not
change. A graph model evolves the same way, one node or connection at a time, and the graph
doubles as **living documentation** of the domain: its entities, data dictionaries, and
providers are linked into the same executable model that runs the behavior.

The same property derisks growth. New use cases are new flows over the existing function
inventory; two versions of a function can run side by side under different route names;
teams scale because a developer building one function needs no knowledge of its neighbors —
which is also why the methodology is naturally test-driven: each function is a
self-contained unit with no external dependencies to fake.

## Co-authoring with AI agents

Composable design turns out to be the shape AI collaboration needs. A function's
input-process-output contract is exactly what an agent can generate against; a flow or a
graph is a **validated artifact** the engine can accept or reject deterministically. Each
DSL therefore ships an agent-ready specification — a grammar plus a machine-readable
catalog — mapped in [`llms.txt`](../llms.txt), so an agent authors from rules rather than
inferring from examples: the [function guide](event-driven/ai-agent-guide.md), the
[Event Script guide](event-script/ai-agent-guide.md), and the
[knowledge-graph guide](knowledge-graph/ai-agent-guide.md).

At the graph layer this becomes live, human-in-the-loop co-authoring. The Playground
(port **8100**) exposes a **companion endpoint** for AI agents:
`POST /api/companion/{session-id}/sync` executes one console command and returns the
outcome **in-band** (`{ok, output, error, result}`), while the same output is teed to the
human's WebSocket console — the agent builds the graph one validated step at a time, and
the human watches it appear on screen in real time. A companion is an *assistant to* a
session, never its owner: session-topology commands are rejected, and only the read-only
session status query is allowed. Both companion endpoints are development-mode only
(`app.env=dev`). The contract is engine-verified in both the Rust and Java engines, with
byte-identical responses — and it has been validated end to end: a fresh agent can build
every Playground tutorial from the agent documentation alone. Details in the
[knowledge-graph agent guide](knowledge-graph/ai-agent-guide.md).

## Packaging and deployment

Composable functions are granular and independent, so packaging is a deliberate
architectural choice rather than a constraint: related functions for a set of flows are
compiled into a single executable, sized to scale horizontally. In this repository each
application is a standalone crate under `examples/` with its configuration in a
`resources/` folder — see [Getting Started](getting-started.md). Functions are invoked by
events on demand and hold no state between events, keeping the memory footprint small and
predictable.

!!! note "Rust port"
    The methodology is unchanged from the Java original — it is the platform's reason for
    being, not a platform feature. Two port-level notes: the flow YAML and the event
    envelope scheme were designed language-neutral, and the Rust port keeps the **flow
    configuration syntax identical**, so flows written for the Java engine run unchanged
    here; and where Java composes with Spring and a Kafka service mesh, this port stays a
    single-runtime, in-memory system by design.

*Adapted from the mercury-composable guide `methodology.md`; behavior verified against this repository's source.*
