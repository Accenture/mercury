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
   > **Shipped 2026-07-17 (increment 22).** `knowledge_graph::math` in the new
   > `crates/knowledge-graph` (crate created one increment early to host it; engine
   > wiring still lands at K-3): lexer/parser/evaluator with Java-exact semantics —
   > strict JS `**` rule, short-circuit logic, dual number rendering (display `3.0` vs
   > concatenation `3`), `NaN != NaN`, the mirrored `Math.*` namespace. `MathError::
   > Parse`/`Eval` mirrors the two Java exception types; `random()` uses `getrandom`
   > (OS entropy, the `SecureRandom` analog); `round` is Java's `floor(x+0.5)`. All 14
   > `ExpressionEngineFullTest` methods + an arity/coercion test = 15 tests.
3. **K-3 — graph compiler + registry.** Graph JSON ⇄ MiniGraph (import/export shape),
   `CompiledGraphs`, `PlaygroundLoader` startup loading; the 13 tutorial fixtures compile
   verbatim. Engine crate + resource-root hook (K8) land here.
   > **Shipped 2026-07-17 (increment 23).** `compiler::compile_graphs`
   > (`#[before_application(sequence = 6)]`): manifest-gated load → `${...}` resolution
   > via `ConfigReader` → deprecated-syntax conversion (shared event-script converter)
   > → `MiniGraph::import_graph` structural validation → `graphs` registry (Java
   > `CompiledGraphs`). K8 hook at sequence 1 appends the crate's resources (append,
   > never prepend — the app wins). 13 tutorial fixtures in `resources/graph/` + 13
   > Java test fixtures and `graphs.yaml` in `tests/resources/`, all verbatim; all 26
   > compile. `CompileGraphTest` ported + a reference-resolution check (3 tests).
   > **Design correction:** Java `PlaygroundLoader` is the `FetchFeature` scanner, not
   > a graph loader — it moves to K-5 (`graph.api.fetcher`).
4. **K-4 — the graph runtime.** `GraphExecutor`/`GraphTraveler`/`GraphInstance`: state
   machine, `{id}@{node}` correlation, decision routing (`next` / node-name / `.sink`),
   loop detection, exception handler, health, housekeeper; core skills
   (`graph.data.mapper`, `graph.math`, `graph.task`, `graph.join`, `graph.island`).
   Tutorial graphs run E2E; the `graph-executor.yml` flow exposes
   `POST /api/graph/{graph-id}`.
   > **Shipped 2026-07-17 (increment 24).** `executor.rs` (interceptor; walk/decide
   > traversal with boxed async recursion), `common.rs` (the 756-line
   > `GraphLambdaFunction` base as free functions), `skills.rs` (data.mapper, math,
   > task incl. `for_each` fork-join, join, island), `services.rs` (housekeeper via
   > the end-flow listener, exception handler, health); `graph-executor.yml` +
   > `flows.yaml` ship with the crate (K8 root). **K5 enforced in code**: `graph.js`
   > is never registered and fails with the explicit retirement message.
   > **Scope adjustments:** `GraphTraveler` is the dev-only Playground walker → moved
   > to K-7 (sessions); tutorials needing fetcher/extension activate at K-5/K-6; the
   > `@OptionalService(app.env=dev)` mock gating lands with K-9. E2E parity suite:
   > tutorials 1/2/4/7/8/9/13, `GraphTaskTest` 1..5, Rust-supplement graphs
   > (join barrier, loop detection, retirement message).
5. **K-5 — platform-core HTTP client** (`async.http.request`, lockstep increment) →
   `graph.api.fetcher` with dictionary/provider nodes, caching, fork-join concurrency.
   Also activates the event-script http-client fixtures deferred at E-9.
   > **Shipped 2026-07-17 (increment 25).** platform-core: `automation/http_client.rs`
   > (interceptor at 500 instances, per-request hyper http1, Java header semantics,
   > trace/cid propagation via envelope + invocation headers — the
   > `PostOffice.trackable` model; deferrals documented: streams/multipart, XML as
   > text, https rejected explicitly) + the `AsyncHttpRequest` builder/parser.
   > event-script: E-9 http-client fixtures activated E2E incl. the full-shape W3C
   > trace-propagation test through the real HTTP edge. knowledge-graph:
   > `fetcher.rs` (dictionary/provider, `key:default` fallbacks, instance cache,
   > for-each fork-join) + `features.rs` (`FeatureRunner` registry — explicit
   > registration replaces the Java annotation scan; built-in log-request/
   > response-headers). Tutorials 3/5/6/12/114 + unit-test-1 pass over real HTTP
   > against the ported mocks. hello* graphs wait for K-6 (`graph.extension`);
   > tutorial-113 permanently skipped (retired `graph.js`).
6. **K-6 — `graph.extension`** (sub-graph + `flow://` flow) + the remaining REST
   endpoints (describe/upload/inspect/live-graph).
   > **Shipped 2026-07-17 (increment 26).** `extension.rs`: sub-graph delegation
   > through the `graph-executor` flow (`path_parameter.graph_id`), `flow://`
   > delegation validated against the flow registry, for-each fork-join with clamped
   > concurrency; `flow-11.yml` joins the engine flows. Two lockstep parity fixes:
   > `no.op` promoted to a platform-core built-in (Java `NoOpFunction`), and the
   > event-script `MultiLevelMap` gained Jayway-parity tolerance for hyphenated
   > JSONPath member names (`$.fetcher-ext.result`). E2E: tutorials 10/11 +
   > `helloworld2` (fetcher → for-each extension → math). **Scope adjustment:** the
   > REST endpoints sketched here (describe/upload/inspect/live-graph) all depend on
   > `GraphCommandService` + the Playground draft dirs → moved to K-7.
   > **Maintainer follow-up (same day):** the `graph.js`-carrying fixtures (`hello`,
   > `helloworld`, `hellojs`, `tutorial-113`) are activated by swapping the skill to
   > `graph.math` (identical statement grammar; one genuinely-JS node adapted);
   > `rust-js-retired` remains the single retirement test case. All four run E2E.
7. **K-7 — platform-core WebSocket server** (lockstep increment) → the Playground:
   `GraphUserInterface` sessions, `GraphCommandService` grammar, the companion command
   API, dev-gating (K9).
   > **Split into K-7a + K-7b. K-7a shipped 2026-07-17 (increment 27):** the
   > platform-core WebSocket server (`automation/ws_server.rs`, hyper upgrade +
   > tokio-tungstenite — the K6a decision): Java `WsRequestHandler` protocol parity —
   > `/ws/{name}/{token}`, per-connection `{session}.in`/`.out` route pair,
   > open/string/bytes/close lifecycle events, transmitter text/binary/JSON-segmented
   > frames + `type: close`, idle sweep, housekeeper cleanup. **Declarative
   > `#[websocket_service]` macro** (maintainer direction) mirrors the Java
   > annotation: inventory registration, lifecycle-loaded URL paths, and the server
   > starts when REST automation is on OR any websocket service exists. E2E with a
   > real tungstenite client (programmatic + declarative suites). **Also
   > maintainer-directed: the declarative `#[fetch_feature]` macro** (new
   > `knowledge-graph-macros` crate) — Java `@FetchFeature` parity for
   > field-installation cases like OAuth 2.0 bearer injection; proven E2E with the
   > DemoAuth pattern on the wire. **K-7b shipped 2026-07-17 (increment 28):** the
   > Playground — `GraphUserInterface` + `GraphSession`, `GraphCommandService`
   > (`commands.rs`, the 1,494-line grammar), `GraphTraveler`, `JsonPathHandler`,
   > the AI-companion REST hop (`POST /api/companion/{id}`) + the K-6-deferred dev
   > REST endpoints, the 39 ported `help/*.md` files, dev-gating (K9): the
   > `PlaygroundLoader` registers the workbench only when `app.env=dev` (default
   > `dev`, Java `application.properties` parity); production graphs run only
   > through `POST /api/graph/{graph-id}`. **platform-core contract fix surfaced by
   > integration:** the serve-until-Ctrl-C wait moved from `AutoStart::main` to
   > `AutoStart::run` (the standalone `fn main()` path) — `main` now returns once
   > booted (the accept loop runs in the background), so an embedder that awaits it
   > no longer hangs when websocket services are registered; `server_address()`
   > recovers an ephemeral (`rest.server.port=0`) port. E2E in `tests/playground.rs`
   > (grammar → companion REST hop → live-graph download). **K-8 (next):** the
   > React webapp + milestone close.
8. **K-8 — the React webapp + milestone close.** Copy the webapp, adjust
   `clean.js`/`deploy.js` to `../resources/public`, run `npm run release`, live-verify
   the Playground end-to-end in a browser; README; milestone banner.
   > **K-8 SHIPPED 2026-07-18 (increment 29) — LAYER 3 MILESTONE CLOSED.** The React
   > webapp copied verbatim into `crates/knowledge-graph/webapp/`; `clean.js`/`deploy.js`
   > retargeted to `../resources/public`. A **third** path of the same class needed the
   > same retarget (maintainer-approved): `src/data/helpContent.ts`'s build-time
   > `import.meta.glob` for the in-app Help panel → `../../../resources/help/*.md` (else the
   > panel renders empty). `npm run release` → the committed bundle in `resources/public`
   > (served at `/`; source maps gitignored as regenerable). **`examples/minigraph-playground`**
   > (open question 4 = yes): a one-line `auto_start_main!` app with its own
   > `application.yml`/`rest.yaml`, mirroring the `hello-flow` convention. Live-verified
   > against the running app (Chrome ext unavailable → verified the exact browser
   > protocol/paths): static serving at `/`, the `/ws/graph/playground` session + command
   > round-trip (help served from the ported `help/*.md`), and the `POST /api/companion/{id}`
   > hop streaming to the WS console. **Three layers ported bottom-up: platform-core →
   > event-script → active knowledge graph.**

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
   **RESOLVED (K-8, 2026-07-18): yes.** `examples/minigraph-playground` ships a one-line
   `auto_start_main!` app with its own `application.yml` (`app.env=dev`, port 8100) and
   `rest.yaml` (Playground/companion endpoints) — the `hello-flow` convention (app owns the
   deployment config; the engine stays a clean library).
