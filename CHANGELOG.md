# Changelog

## Release notes

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The full increment-by-increment record lives in [`docs/INCREMENTS.md`](docs/INCREMENTS.md);
the design rationale in [`docs/design/`](docs/design/).

---
## Unreleased

### Added

1. **Comma-separated route aliases in `#[preload]`** — Java `@PreLoad` parity:
   `route = "hello.world, hello.declarative"` registers the same function object under
   every listed name with the same instance count and visibility. Empty segments are a
   compile error.
2. **Application log context is now on by default.** platform-core ships a built-in
   `default-log-context.yaml` (embedded at compile time) so the structured JSON formats
   (`log.format=json` or `compact`) stamp the standard trace context (`cid`, `traceId`,
   `tracePath`, `spanId`, `parentSpanId`, `service`, `timestamp`) into every log line a
   traced function emits — no setup required. An application can replace the template
   with its own `app-log-context.yaml`, or opt out with the new `app.log.context=false`
   key. Applications already providing an `app-log-context.yaml` are unaffected.
   Plain-text logging (`log.format=text`, the default) is unaffected.
3. **RPC telemetry records — exactly one record per span.** The caller now emits the
   `round_trip` trace record for each traced RPC response (Java `InboxBase.recordTrace`
   parity) while the worker suppresses its own record for an RPC-served execution whose
   reply reached the caller (Java `WorkerHandler.sendTracingInfo` gate) — so each span
   reports once, with full lineage: `parent_span_id` (the caller's span, unconditional)
   and `span_id` (the callee's span, adopted only from a **direct responder**; a relayed
   reply — e.g. a flow answering on behalf of the flow-adapter route — keeps the parent
   but omits the span, Java `spanIdFromResponder` parity). Callback-style invocations
   keep self-recording. The callee's trace annotations now ride the reply envelope (also
   on the Event-over-HTTP wire) and fold into the span's single record; the RPC reply
   itself carries the measured `round_trip` value. The programmatic `event_over_http`
   client stamps the calling function's trace context (incl. its span) onto the wire
   envelope, so remote functions parent onto the caller's span in both the declarative
   and the programmatic pattern.
4. **Event-over-HTTP demo endpoints in `hello-flow`** (the structural parallel of the
   Java composable-example, now on port **8100**): `/api/event/http/demo` (declarative —
   the flow's task is the foreign route `hello.declarative`, resolved through
   `event-over-http.yaml`) and `/api/event/http/programmatic` (the task passes the peer's
   `/api/event` URL directly to the request API). The `hello-world` echo registers the
   `hello.declarative` alias and is interchangeable with the Java lambda-example — same
   port 8085, same routes — so the demo doubles as a cross-language interop demo with
   zero configuration changes.

### Fixed

1. **A zero-traced hop no longer leaks a nested reply's span id** as its own on the
   response envelope (Java parity: its reply carries no span).

---
## Version 4.9.0, 7/20/2026

**Graduation release — and the adoption of the canonical version line.** The version jumps
from 0.1.0 to **4.9.0** to track `mercury-composable` (Java), with which this engine is
behavior-synced: the companion REST contract is byte-identical, the graph/flow DSLs are
shared, and every engine fix since the port began landed in both implementations
(mercury-composable PRs #187–#204, released there as 4.9.0 the same day). One version, two
languages.

Everything since 0.1.0, in brief (increments 30–49 — the full record in
[`docs/INCREMENTS.md`](docs/INCREMENTS.md)):

### Added

1. **The synchronous AI-companion endpoint** `POST /api/companion/{id}/sync` (ADR-0008) —
   command outcomes in-band (`{ok, output, error, result}`, whole-traversal capture, WS tee
   for real-time human+AI collaboration), with a truthful contract: whole-output-aware `ok`
   classification, no silent dedup for RPC callers, `Syntax:` usage hints classify as
   failures.
2. **Discovery + contract commands**: `list graphs` / `list flows` / `describe graph
   {graph-id}` — self-service delegation (list → contract → delegate) with the root
   `purpose` enforced at compile as living documentation.
3. **Outbound HTTPS for the async HTTP client** (rustls + OS trust store, per-request
   `trust_all_cert`) — field-validated end-to-end against a live CA chain. Redirects are
   deliberately not followed (backend design; documented decision record in
   `docs/design/platform-core-port.md` §5j).
4. **Numeric promotion + `f:round`** for the simple-plugin arithmetic family.
5. **The battle-tested AI-agent documentation**: hardened by 25 fresh-agent exercises
   across both engines (the last thirteen passing with zero documentation lookups) and
   kept in lock-step with the Java repo (back-port #203 there).
6. **The human documentation site** — 20 pages, published at
   [accenture.github.io/mercury](https://accenture.github.io/mercury/) (automated via
   `mkdocs gh-deploy` on pushes to main).
7. **Rust CI quality gates** — fmt + clippy (zero warnings) + the full workspace test
   suite on every PR.

### Fixed

1. **Join barrier: only valid completions count** — success-only completion marks cleared
   by `RESET`, and chained joins judged by recorded outcome (latent data-loss bugs found by
   probe, fixed in both engines).
2. **Companion `session` limited to the read-only status query** (topology subcommands are
   a WebSocket-session privilege).
3. **HTTP-boundary content-type dispatch** mirrors the Java `handlePayload` rules exactly.
4. Spring-named configuration keys retired: `app.profiles.active` / `application.name`.

**Verified:** 206 workspace tests green, `clippy` zero warnings, `fmt` clean — enforced in
CI from this release on.

---
## Version 0.1.0, 7/18/2026

The first end-to-end port of `mercury-composable` (Java, canonical v4.8.6) to Rust: the three
foundational layers, ported bottom-up (foundation → UI) across 29 verified increments and
validated against the canonical Java fixture suite. **This is the first port ready for manual
end-to-end testing** — 181 workspace tests green, `clippy` clean, `fmt` clean.

Out of scope by design: the Kafka service mesh (`minimalist-kafka`, `twin-kafka`, connectors)
and Spring (`rest-spring-3/-4`). `graph.js` is deliberately **retired** — an interpreter running
arbitrary user code is an attack surface the port does not carry; `graph.math` (typed, bounded)
and `graph.task` (reviewed, compiled functions) cover its use cases.

### Added

**platform-core (layer 1) — the actor-model event bus and operable runtime**

1. **Configuration management, event bus, and reactive back-pressure** — the `-D`/YAML config
   reader, the route-addressed event bus (functions coupled only by route name + `EventEnvelope`),
   and the FIFO ElasticQueue with manager–worker back-pressure (disk spill under overload).
2. **Application lifecycle and annotation macros** — `#[preload]` / `#[before_application]` /
   `#[main_application]` / `#[zero_tracing]` with link-time `inventory` registration (the Java
   classpath-scan analog), plus the one-line `auto_start_main!()` entry point.
3. **Observability** — OpenTelemetry-style distributed tracing, business correlation-id, and
   app-log-context (three-format logger).
4. **REST automation and operability** — `rest.yaml` as the router on a hyper HTTP edge,
   actuators (`/info`, `/env`, `/health`, `/livenessprobe`), and the static-content protocol
   (SHA-256 etag / HTTP-304, no-cache pages, the `static-content.filter` request interceptor).
5. **RPC inbox, HTTP client, and WebSocket server** — the lightweight RPC inbox (`AsyncInbox`
   parity), the async HTTP client (`async.http.request`), and the WebSocket server on the HTTP
   upgrade path with the declarative `#[websocket_service]` macro.

**event-script (layer 2) — the composable-flow engine**

6. **Flow model, compiler, and data-mapping engine** — the full `CompileFlows` port (all Java
   fixtures reused verbatim) and the runtime MultiLevelMap mapping engine over `rmpv::Value`
   (direct composite-key access primary; JSON-Path `$.…` for complex queries).
7. **The complete flow runtime** — sequential / response / decision / sink, parallel and
   fork/join (pipe-map barrier), pipelines with for/while loops and break/continue, `flow://`
   sub-flows with shared parent state, and the external state machine — plus TTL abort, metrics,
   and the flow-summary span.
8. **Plugins, HTTP, and resilience** — all 42 built-in plugins with the `#[simple_plugin]` macro,
   the HTTP flow adapter, the resilience handler, and the event-script mock.

**active knowledge graph (layer 3) — the MiniGraph Playground**

9. **MiniGraph and the graph toolchain** — the MiniGraph property graph (a platform-core
   built-in), the math expression engine, and the graph compiler + registry (13 tutorial
   fixtures compiled verbatim; the engine crate ships its own bundled resources).
10. **The graph runtime and core skills** — the executor state machine (composite `{flow}@{node}`
    correlation, decision routing, loop detection) and the core skills `graph.data.mapper`,
    `graph.math`, `graph.task`, `graph.join`, `graph.island`, `graph.api.fetcher`,
    `graph.extension`, plus the declarative `#[fetch_feature]` macro (OAuth 2.0 bearer
    injection). `graph.js` is retired.
11. **The Playground** — the command grammar (`GraphCommandService` port), the graph traveler,
    the WebSocket UI (`/ws/graph`, `/ws/json`), the AI-companion REST hop
    (`POST /api/companion/{id}`), and dev-gating (`app.env=dev`); the React webapp
    (`@xyflow/react`) served as static content by REST automation.

**Examples**

12. `examples/hello-world` (layer 1), `examples/hello-flow` (layer 2 — a YAML flow over HTTP),
    and `examples/minigraph-playground` (layer 3 — the runnable Playground app at
    `http://127.0.0.1:8100/`).
