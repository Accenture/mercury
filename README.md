# mercury

A **Rust port of [mercury-composable](https://github.com/Accenture/mercury-composable)** —
Accenture's event-driven, composable application platform (canonical Java implementation,
v4.8.6), carrying the same vision: build applications from small, fully-decoupled functions
wired by route name, orchestrated as configuration, and modeled as an executable knowledge
graph.

> **Status: all three layers ported and milestone-closed** across 49 verified increments;
> 206 workspace tests green, `clippy` and `fmt` clean; benchmarked (RPC ~155K ops/s @ 6µs).
> The AI-agent documentation is battle-tested — twelve consecutive fresh-agent exercises
> passed with zero documentation lookups, across both engines. See
> [`CHANGELOG.md`](CHANGELOG.md) and [`docs/INCREMENTS.md`](docs/INCREMENTS.md).

## The three paradigm layers

Each layer builds on the one below (foundation → UI):

1. **platform-core** — the actor-model event bus: route-addressed functions coupled only by
   route name + an immutable `EventEnvelope`, over a tokio async runtime. Plus the operable
   runtime: REST automation (`rest.yaml` *is* the router, on hyper), actuators, tracing +
   correlation-id, an async HTTP client, and a WebSocket server.
2. **event-script** — composable orchestration: a YAML flow DSL that choreographs functions
   into transactions (sequential / decision / parallel + fork-join / pipelines / sub-flows),
   with data mapping, resilience, and an HTTP flow adapter. No orchestration in code.
3. **active knowledge graph** — the semantic layer: MiniGraph property graphs whose nodes
   carry executable **skills**, so traversing the graph *is* running the application — with
   the browser-based MiniGraph Playground for building, running and inspecting graphs.

## Quick start

Rust (stable) + Cargo. Run any example:

```bash
cargo run -p hello-world           # layer 1 — event bus + HTTP
cargo run -p hello-flow            # layer 2 — a YAML flow over HTTP (port 8086)
cargo run -p minigraph-playground  # layer 3 — the Playground; open http://127.0.0.1:8100/
```

Verify the workspace:

```bash
cargo test --workspace
cargo clippy --workspace --all-targets
cargo fmt --all --check
```

## Repository layout

| Path | What |
|---|---|
| `crates/platform-core` | layer 1 — event bus, runtime, REST/WebSocket automation |
| `crates/event-script` | layer 2 — the composable-flow engine |
| `crates/knowledge-graph` | layer 3 — MiniGraph engine + the Playground (`webapp/`) |
| `crates/*-macros` | annotation macros (`#[preload]`, `#[websocket_service]`, …) |
| `examples/` | runnable example apps, one per layer |
| `docs/INCREMENTS.md` | the increment-by-increment port ledger |
| `docs/design/` | per-layer design docs (the *why* behind the port) |
| `docs/arch-decisions/ADR.md` | the durable architecture decisions |
| `memory/` | the shared cross-session AI-memory layer |

## Documentation

The human developer guide is a 20-page MkDocs site under [`docs/`](docs/) (Getting Started,
per-layer guides, references, port scope) — published at
**<https://accenture.github.io/mercury/>**, or build it locally with `mkdocs serve`.
AI agents start at
[`docs/llms.txt`](docs/llms.txt): the machine-readable map of the AI-agent documentation
that the fresh-agent validation sweep was driven from.

## Non-goals

The Kafka **service mesh** (service discovery + sync-over-Kafka, the `connectors/` tree) and
Spring (`rest-spring-3/-4`) are out of scope. `minimalist-kafka` and `twin-kafka` are
lightweight cloud-native connectors — not part of the mesh exclusion — planned for future
iterations together with `sync-over-async` (see
[`docs/background/port-scope.md`](docs/background/port-scope.md)). `graph.js` is deliberately
retired in this port (an arbitrary-code interpreter is an attack surface); `graph.math` and
`graph.task` cover its use cases.

## Legacy versions

This repository previously hosted the original Java implementation of Mercury (up to
v3.0.19); it was repurposed as the home of the Rust version on 2026-07-20. Versions 1.13.0
and 2.1.0–2.7.0 remain available as release branches in this repository. All known field
installations have been upgraded to version 4.x using the
[mercury-composable](https://github.com/Accenture/mercury-composable) repo — the canonical
Java implementation this port follows.

## Contributing & license

See [`CONTRIBUTING.md`](CONTRIBUTING.md) and [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
Licensed under the [Apache License 2.0](LICENSE).
