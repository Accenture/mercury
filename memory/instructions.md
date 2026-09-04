# Agent Instructions — mercury

## What This Project Is

mercury is a **Rust port of `mercury-composable`** — Accenture's event-driven, composable
application platform, whose canonical implementation is in Java. It carries the *same*
product vision as the Java project, re-delivered in Rust **bottom-up: foundation → user
interface**. Three core layers are in scope — **platform-core** (foundation), **event-script**,
and the **active knowledge graph**; the **Kafka service mesh is deliberately out of scope**.
The authoritative, detailed vision lives in the official mercury-composable (Java) project —
we *map, don't mirror* (see `memory/vision.md`).

At enable time (2026-07-15) this is a **greenfield / private prototyping repo**: it contained
only a placeholder `README.md` and a single commit — **no source code, build system, or
crates yet**. The porting will land those; record the concrete stack in
`memory/continuity.md` → `## Stack & Tools` as it takes shape. Once the foundation is
sufficient, the repo **graduates to the official Accenture repo** — the private repo keeps
rapid prototyping quiet for public readers. Treat the Vision as the north star and let each
delivered increment become the next Current State (the VBDI loop — see `DECAY.md` §12).

**Type:** Event-driven composable application platform (a Rust port of the Java engine)
**Primary language:** Rust (edition 2021) — multi-crate Cargo workspace
**Framework / stack:** tokio, hyper, serde, rmp-serde; see `memory/continuity.md` → `## Stack & Tools`

> High-level only. The precise dependency list and current versions live in
> `memory/continuity.md` → `## Stack & Tools` (the live source of truth) — keep this
> section enduring and don't duplicate them here.

## Repository Structure

Cargo workspace; the seven published crates live under `crates/`:

```
crates/
  platform-core          ← the engine: event bus, registry, PostOffice, REST automation (hyper)
  platform-macros        ← #[preload], #[main_application], … the link-time inventory carriers
  event-script           ← compiles & executes YAML flows (flow YAML is identical to Java's)
  event-script-macros    ← #[simple_plugin]
  knowledge-graph        ← MiniGraph: graph executor, skills, Playground
  knowledge-graph-macros ← #[fetch_feature]
  minigraph-state-redis  ← pluggable suspend/resume state store
examples/<name>/         ← standalone workspace crates, never cargo examples in a library crate
system/ai-contract-provider ← serves the version-matched AI documentation contract
docs/ (guides, llms.txt, INCREMENTS.md)   draft-design-specs/   scripts/
```

(At enable time, 2026-07-15, the repo held only `README.md` plus the agent-memory layer —
that history is preserved in the archive, not here.)

## Core Abstractions (inherited from mercury-composable — the behavior being ported)

Descends from the **actor model** (Akka lineage): a **function** is an isolated actor,
addressed only by its **route name**; the only thing passed between functions is an immutable
**`EventEnvelope`**. There are no direct calls between user functions.

- **Function** — implements the equivalent of `TypedLambdaFunction<I,O>`:
  `handle_event(headers: Map<String,String>, input: I, instance) -> O`. Stateless; registered
  by route name (lowercase dot-separated, ≥1 dot).
- **EventEnvelope** — immutable message with three parts: **metadata** (`id`, `to`, `from`,
  `reply_to`, `cid`/correlation, `trace_id`, `status` [HTTP-style, ≥400 = error], timing),
  **headers** (`Map<String,String>`), and **body** (payload). MsgPack on the bus, JSON at HTTP
  boundaries.
- **Platform** — the registry: register a function at a route with N instances (workers),
  `has_route`, `release`.
- **PostOffice** — the messaging client: `send` (fire-and-forget), `request(event, timeout)`
  (RPC), broadcast, scheduled send.
- **In-memory event bus** — the transport. Point-to-point → one worker instance; broadcast →
  all instances. Java uses Eclipse Vert.x + Java 21 virtual threads (blocking-style code that
  performs like reactive); the Rust port re-implements this on its own async runtime.
- Higher layers reuse the *same* function unchanged: wired by HTTP (a **service**), by a flow
  (**Event Script** task), or by a graph (**knowledge-graph** skill).

## Port Scope & Source Mapping

Canonical source: `mercury-composable` (Java, `com.accenture.mercury`; released lock-step — see
`continuity.md` → Project State for the current version). The
authoritative behavior spec is that repo + its `docs/guides/` — we **map, don't mirror**.

**In scope** (the three layers → Rust):
- `system/platform-core` → the foundation (event bus, EventEnvelope, PostOffice, Platform).
- `system/event-script-engine` → the YAML flow DSL + engine.
- `system/minigraph-playground-engine` (+ `core/graph`) → the active knowledge graph.

**Out of scope** (confirmed by maintainer; refined 2026-07-20):
- Kafka service mesh (service discovery + sync-over-Kafka): all of `connectors/`
  (`cloud-connector`, `service-monitor`, `kafka-connector`, `kafka-presence`),
  `helpers/*-standalone`.
- NOTE: `system/minimalist-kafka` and `system/twin-kafka` are lightweight cloud-native
  connectors, NOT mesh — reclassified to future-port backlog (with `sync-over-async`).
- **Spring adapters** (`rest-spring-3/-4`) — Spring is Java-only. Note: platform-core's *own*
  REST automation (`automation/` package, Vert.x-based, no Spring) **is** in scope as a later
  increment — the Rust port gets its HTTP boundary from there.

**Port order within platform-core** (maintainer, 2026-07-15): **configuration management
first** — `AppConfigReader` / `ConfigReader` / `MultiLevelMap` and the `resources/` folder
convention — because everything (main app, unit tests, integration tests) relies on it. The
event-bus foundation (EventEnvelope/Platform/PostOffice) is increment 2. Config file syntax
(`classpath:/`, `file:/`, `${ENV_VAR:default}`, dot-bracket keys) is kept **verbatim** so
config files port between the Java and Rust versions unchanged. See
`draft-design-specs/platform-core-port.md`.

**Deferred / TBD** (decide when reached): `mini-scheduler`; `extensions/*`; `examples/*`
(port a reference example to validate the foundation).

## Conventions Observed

The Rust baseline is established and live — the canonical statement is the
`conventions-rust-baseline` fact in `memory/continuity.md` → Conventions (kept there, not
duplicated here). Headlines: `cargo fmt` + `cargo clippy --all-targets` clean is the
definition of done; Apache-2.0 header on every source file (`tests/ui` compile-fail
fixtures exempt — Eric-ratified carve-out, 2026-07-26); each module's `//!` doc names the
Java class it ports; unit tests in-module, integration tests under `tests/` with
`tests/resources/`; behavior-parity notes for deliberate divergences; `docs/INCREMENTS.md`
ledger per increment. *(This section said "None yet — no Rust code" until 2026-08-22 —
stale since increment 1; caught by the memory smoke test.)*

## Tone & Style

- Be concise unless detail is explicitly requested.
- Prefer prose over bullet lists for explanations.
- When suggesting code changes, match the existing style and patterns in this repo.
- Always check `memory/continuity.md` for prior decisions before suggesting
  architectural changes.

## Core Rules

1. Never modify files outside the project scope without asking.
2. Follow the existing code style — do not reformat files unnecessarily.
3. When in doubt about a pattern or convention, ask rather than assume.
4. Record all significant decisions in the session log and continuity file.
5. If you see a TODO, open thread, or obvious issue, note it in continuity.md.

## Testing

Built-in Rust test harness. Unit tests in-module (`#[cfg(test)]`); integration tests in each
crate's `tests/` with fixtures under `tests/resources/`; compile-fail macro tests via `trybuild`
in `tests/ui/`. `cargo fmt` + `cargo clippy --all-targets` clean is part of "done".

```bash
cargo test --workspace          # all tests
cargo test -p mercury-event-script
cargo clippy --all-targets -- -D warnings
```

**One integration file boots one shared server.** A second `#[tokio::test]` in the same file gets
its own runtime, which drops the shared server the first test started — add further cases as
sequential `async fn`s called from the single booted test (documented at `graph_runtime.rs:527`).

## CI / CD

GitHub Actions: `rust.yml` (build + `cargo test`), `docs.yml` (llms.txt link-integrity check →
`mkdocs build --strict` → gh-pages deploy on main), and `agent-memory.yml` (the ritual floor:
`memory-lint` + an advisory session-log check).

## Editing These Instructions

Only modify this file if the user explicitly asks to change the project
description, rules, or conventions. Treat it as stable configuration.
