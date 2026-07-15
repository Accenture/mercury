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

**Type:** Event-driven composable application platform (a Rust port; greenfield — no code yet)
**Primary language:** Rust (target; nothing scaffolded yet)
**Framework / stack:** Rust — specifics TBD; see `memory/continuity.md` → `## Stack & Tools`

> High-level only. The precise dependency list and current versions live in
> `memory/continuity.md` → `## Stack & Tools` (the live source of truth) — keep this
> section enduring and don't duplicate them here.

## Repository Structure

At enable time the only project file is `README.md` (a title placeholder). Everything else
is the agent-memory layer: `memory/` (shared memory), the root steering files
(`AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.cursorrules`, `.windsurfrules`,
`.github/copilot-instructions.md`), `agent-skills/` (portable capabilities), and the
protocol docs (`DECAY.md`, `REVIEW.md`, `SKILLS.md`, `MERGE.md`). No source directories
exist yet — record the structure here as it emerges.

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

Canonical source: `mercury-composable` (Java, `com.accenture.mercury` v4.8.6). The
authoritative behavior spec is that repo + its `docs/guides/` — we **map, don't mirror**.

**In scope** (the three layers → Rust):
- `system/platform-core` → the foundation (event bus, EventEnvelope, PostOffice, Platform).
- `system/event-script-engine` → the YAML flow DSL + engine.
- `system/minigraph-playground-engine` (+ `core/graph`) → the active knowledge graph.

**Out of scope** (confirmed by maintainer):
- Kafka service mesh & distribution: `system/minimalist-kafka`, `system/twin-kafka`, all of
  `connectors/` (`cloud-connector`, `service-monitor`, `kafka-connector`, `kafka-presence`),
  `helpers/*-standalone`.
- **Spring adapters** (`rest-spring-3/-4`) — Spring is Java-only. Note: platform-core's *own*
  REST automation (`automation/` package, Vert.x-based, no Spring) **is** in scope as a later
  increment — the Rust port gets its HTTP boundary from there.

**Port order within platform-core** (maintainer, 2026-07-15): **configuration management
first** — `AppConfigReader` / `ConfigReader` / `MultiLevelMap` and the `resources/` folder
convention — because everything (main app, unit tests, integration tests) relies on it. The
event-bus foundation (EventEnvelope/Platform/PostOffice) is increment 2. Config file syntax
(`classpath:/`, `file:/`, `${ENV_VAR:default}`, dot-bracket keys) is kept **verbatim** so
config files port between the Java and Rust versions unchanged. See
`docs/design/platform-core-port.md`.

**Deferred / TBD** (decide when reached): `mini-scheduler`; `extensions/*`; `examples/*`
(port a reference example to validate the foundation).

## Conventions Observed

None yet — no Rust code has been written. Establish coding conventions (crate/module layout,
naming, formatting via `rustfmt`, `clippy` lint level, error-handling style, commit style) as
the first code lands, and record them here so every agent and teammate follows the same
patterns from the start.

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

None yet — no code, no test framework chosen. Set up and document the test approach with
the first code, and record the framework in `continuity.md` → `## Stack & Tools`.

## CI / CD

No project CI/CD yet. The agent-memory ritual floor
(`.github/workflows/agent-memory.yml`) runs `memory-lint` and an advisory session-log
check on push/PR — that governs the *memory layer*, not the (not-yet-existent) build.

## Editing These Instructions

Only modify this file if the user explicitly asks to change the project
description, rules, or conventions. Treat it as stable configuration.
