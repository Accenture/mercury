# Design — active knowledge graph → Rust (layer-3 port)

> **Status:** DRAFT v1 for maintainer review · **Realizes:** `bp-active-knowledge-graph`
> (Blueprint) · **Serves:** `vision-mercury` · **Author:** Claude Code · **Date:** 2026-07-17
> **Canonical source:** `mercury-composable` (Java, v4.8.6) —
> `system/minigraph-playground-engine` (~9.8K LOC / 51 main files + a React webapp) and
> **`MiniGraph` in platform-core** (827 lines — the property graph is a layer-1 feature in
> Java, and stays one here). Authoritative docs: `docs/guides/knowledge-graph/` (index,
> property-graph, skills-reference, command-reference, playground-and-companion,
> composing-the-layers — 1,579 lines). This is a *Design-altitude* artifact in the VBDI
> loop. **Approved 2026-07-17** (defaults accepted; K5 upgraded to retirement — see below).

## 1. Goal & scope

Port the **Active Knowledge Graph** — layer 3, the semantic layer: a property graph whose
nodes carry executable **skills** (composable functions). Traversing the graph IS running
the application; behavior lives in the model. The engine rides both lower layers without
coupling: skills are route-named functions (layer 1), a deployed graph is exposed through
an **event-script flow** (`graph-executor.yml` → `POST /api/graph/{graph-id}`, layer 2),
and `graph.extension` delegates to sub-graphs or `flow://` flows.

The **MiniGraph Playground** — a React workbench over a WebSocket session (`/ws/graph`)
with an AI-companion command API — is part of the layer and part of this port
(maintainer-directed: reuse the React app; only the `npm run release` deploy path
changes).

## 2. Proposed decisions (maintainer gate)

| # | Decision | Rationale |
|---|---|---|
| K1 | **`MiniGraph` goes into platform-core** (`graph` module): nodes, unidirectional connections with optional relations, untyped properties, alias/id/type lookups, traversal | Java placement parity — the property graph is a layer-1 built-in the semantic layer builds on; sized for a few hundred in-memory nodes (never a database — the vision's non-goal). |
| K2 | **New crate `crates/knowledge-graph`** for the engine, depending on platform-core + event-script | The D5 workspace pattern; mirrors the Java module split. The engine self-registers through the annotation inventory like event-script does. |
| K3 | **Graph JSON verbatim; Java fixtures reused unchanged** (13 `tutorial-*.json` graphs, mock data, help markdown, `graph-executor.yml` + `flow-11.yml` flows) | The E2 principle extended to layer 3: a graph exported from the Java playground must import and run on the Rust engine. The help files power `describe skill` identically. |
| K4 | **The math expression engine is ported faithfully** (own lexer/parser/evaluator, ~700 lines — no dependency) | `graph.math` is the fast inline compute/branching path; the Java `ExpressionEngineFullTest` becomes the parity suite. |
| K5 | **`graph.js` is RETIRED** (maintainer decision, 2026-07-17 — not deferred): the skill is not registered; a graph carrying `skill: graph.js` fails with an explicit retirement message pointing to `graph.math`/`graph.task` | **Security**: an embedded interpreter executing arbitrary user-supplied code is an attack surface; Java carries GraalVM only for lack of a viable alternative. The Rust engine deliberately does not reproduce the risk — `graph.math` (typed, bounded) and `graph.task` (reviewed, compiled functions) cover the use cases. |
| K6 | **Two platform-core extensions, shipped in lockstep with their own tests**: (a) a **WebSocket server** (hyper upgrade + `tokio-tungstenite`; the Java `@WebSocketService` analog) for the Playground session; (b) an **HTTP client** service (`async.http.request`, hyper client — closing that long-standing §7 deferral) for `graph.api.fetcher` | Both are genuine layer-1 features other layers will reuse (the HTTP client also unblocks the event-script http-client fixtures deferred at E-9). |
| K7 | **The React webapp is copied verbatim** into `crates/knowledge-graph/webapp/`; only `scripts/clean.js` + `scripts/deploy.js` change: `../src/main/resources/public` → **`../resources/public`** (maintainer-directed). `node_modules`/`dist` gitignored; `npm run release` stays a human/dev step (no npm in CI — the no-code invariant applies to the tool chain, not the UI asset pipeline) | The compiled bundle lands in the engine crate's `resources/public`, served by REST automation exactly like the Java jar's resources. |
| K8 | **The engine contributes its own resource root** (a before-application hook appending `CARGO_MANIFEST_DIR/resources`) | The Rust analog of a jar's bundled resources: graphs, flows, help, mock data and the webapp bundle travel with the engine crate; the app's own `resources/` still wins (prepend beats append). |
| K9 | **Dev-gating parity**: the Playground WebSocket + companion endpoints register only when `app.env=dev` (Java parity) | The Playground is a dev workbench; production graphs run through `POST /api/graph/{graph-id}` only. |

## 3. Architecture mapping (Java → Rust)

| Java | Rust | Notes |
|---|---|---|
| `org.platformlambda.core.graph.MiniGraph` (platform-core) | `platform_core::graph` (K1) | two concurrent maps per graph; SimpleNode/SimpleConnection |
| `math/*` (Lexer/Parser/Evaluator/ExpressionEngine…) | `math.rs` module family | self-contained; no deps |
| `CompileGraph` + `CompiledGraphs` + `PlaygroundLoader` | `compiler.rs` + registry | graph JSON → MiniGraph instances; loads `graph/*.json` at startup |
| `GraphExecutor` / `GraphTraveler` / `GraphInstance` / `Visits` | `executor.rs` / `traveler.rs` / `instance.rs` | per-instance state machine (the layer-2 `MultiLevelMap` reused); `{instanceId}@{nodeName}` correlation; loop detection (`graph.max.loop.interval`, `graph.node.high.frequency`) |
| skills: `graph.data.mapper`, `graph.math`, `graph.task`, `graph.join`, `graph.island`, `graph.api.fetcher`, `graph.extension` (+ `graph.js` deferred) | `skills/*.rs`, each a `#[preload]` function | the mapping mini-language is shared with event-script (E-2 reuse) |
| `GraphCommandService` (1,494 lines — the Playground grammar) | `commands.rs` | create/connect/describe/instantiate/run/execute/inspect/export/import… |
| `GraphUserInterface` (`@WebSocketService`, `/ws/graph`) + `JsonPathHandler` | WS handler on the K6a server | collaborative sessions (`GraphSession`), console streaming |
| REST endpoints (`DescribeGraph`, `PostCompanionCommand`, uploads, `GetLiveGraph`, `InspectStateMachine`…) | `#[preload]` functions + rest.yaml | companion: `POST /api/companion/{session-id}` (text/plain command) |
| `GraphHealth` / `GraphHousekeeper` / `GraphExceptionHandler` | ported with the engine | health joins the actuator protocol |
| `webapp/` (React 19 + vite + @xyflow/react) | `crates/knowledge-graph/webapp/` verbatim (K7) | deploy path adjusted; bundle → `resources/public` |
| `graph-executor.yml` flow + rest.yaml | reused verbatim | layer 3 exposed through layer 2 |

## 4. Increment plan

1. **K-1 — MiniGraph in platform-core.** The property graph (827 Java lines) + its test
   suite. No engine yet.
   > **Shipped 2026-07-17 (increment 21).** `platform_core::graph` — `MiniGraph` +
   > `SimpleNode`/`SimpleConnection`/`SimpleRelationship`/`GraphProperties` with
   > Java-exact semantics and error messages; `Arc`-shared nodes with interior
   > mutability stand in for Java's shared mutable objects; property values are
   > `rmpv::Value`; export/import is deterministic (sorted) and matches the
   > tutorial-graph JSON shape. Full `GraphTest` ported (10 tests). One deliberate
   > divergence: `remove_node` uses the lowercased alias key (Java has a latent
   > case-sensitivity slip). Two Java behaviors kept faithfully: the max-nodes
   > check is `count > max` before increment (so `max + 1` nodes fit), and a failed
   > import resets the graph to empty.
2. **K-2 — the math expression engine.** Lexer/parser/evaluator/functions;
   `ExpressionEngineFullTest` ported as the parity suite. Pure functions, no bus.
3. **K-3 — graph compiler + registry.** Graph JSON ⇄ MiniGraph (import/export shape),
   `CompiledGraphs`, `PlaygroundLoader` startup loading; the 13 tutorial fixtures compile
   verbatim. Engine crate + resource-root hook (K8) land here.
4. **K-4 — the graph runtime.** `GraphExecutor`/`GraphTraveler`/`GraphInstance`: state
   machine, `{id}@{node}` correlation, decision routing (`next` / node-name / `.sink`),
   loop detection, exception handler, health, housekeeper; core skills
   (`graph.data.mapper`, `graph.math`, `graph.task`, `graph.join`, `graph.island`).
   Tutorial graphs run E2E; the `graph-executor.yml` flow exposes
   `POST /api/graph/{graph-id}`.
5. **K-5 — platform-core HTTP client** (`async.http.request`, lockstep increment) →
   `graph.api.fetcher` with dictionary/provider nodes, caching, fork-join concurrency.
   Also activates the event-script http-client fixtures deferred at E-9.
6. **K-6 — `graph.extension`** (sub-graph + `flow://` flow) + the remaining REST
   endpoints (describe/upload/inspect/live-graph).
7. **K-7 — platform-core WebSocket server** (lockstep increment) → the Playground:
   `GraphUserInterface` sessions, `GraphCommandService` grammar, the companion command
   API, dev-gating (K9).
8. **K-8 — the React webapp + milestone close.** Copy the webapp, adjust
   `clean.js`/`deploy.js` to `../resources/public`, run `npm run release`, live-verify
   the Playground end-to-end in a browser; README; milestone banner.

## 5. Out of scope (confirmed defaults)

- **`graph.js`** (K5 retirement — security decision; explicit failure message, name NOT registered).
- Kafka exposure of graphs (the mesh is out of scope repo-wide).
- The governance lifecycle (certify → stage → approve → promote) — roadmap in Java too,
  not in code.
- Session persistence across restarts (Java parity: in-memory sessions).

## 6. Open questions for the maintainer

1. **Crate name** — `crates/knowledge-graph` (layer name, matching the vision) or
   `crates/minigraph-playground-engine` (Java module name)? *(Default:
   `knowledge-graph`.)*
2. **`graph.js`** — accept the K5 deferral, or embed `boa_engine` (pure-Rust JS) now?
   *(Default: defer.)*
3. **WebSocket dependency** — `tokio-tungstenite` acceptable for the K6a server?
   *(It is the de-facto tokio WS library; hyper alone requires hand-rolling the
   protocol.)*
4. **Playground app shape** — the engine crate is directly runnable through any app that
   links it; should K-8 also add an `examples/minigraph-playground` app crate (mirroring
   the Java `examples/` module and our examples convention)? *(Default: yes.)*
