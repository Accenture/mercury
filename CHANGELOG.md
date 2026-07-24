# Changelog

## Release notes

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The full increment-by-increment record lives in [`docs/INCREMENTS.md`](docs/INCREMENTS.md);
the design rationale in [`docs/design/`](docs/design/).

---
## Version 4.10.3, 7/23/2026

Patch release for **field deployment**, in lock-step with the Java engine's v4.10.3 —
releases are immutable, so the post-4.10.2 fixes roll up into a new patch. No engine
behavior changes: demo hygiene and refreshed playground webapp dependencies.

### Changed

1. **The demo echo displays the clean envelope-header view
   ([#175](https://github.com/Accenture/mercury/pull/175)).** The hello-world echo now
   reflects the function's ENVELOPE headers (the transported view) rather than the
   injected input copy — making the demo a live proof that the reserved `my_*` metadata
   never rides the wire: the injected keys are visible in the function (and documented),
   but the echoed transport view is clean. The hello-flow example adopts
   `log.format=json`, so a side-by-side java/rust demo run reads identically in both
   terminals (presentation parity down to the demo experience).
2. **Playground webapp dependencies refreshed from npm
   ([#174](https://github.com/Accenture/mercury/pull/174)).** Fresh resolution of the
   knowledge-graph Playground webapp lockfile within the declared semver ranges
   (including the dependabot react-router advisory bump,
   [#173](https://github.com/Accenture/mercury/pull/173)): `npm audit` clean, package
   registry and integrity checksums verified. Webapp-only — no Rust crate dependency
   changes.

---
## Version 4.10.2, 7/23/2026

Patch release: **the metadata contract, hardened in lock-step with the Java engine**. A
composable function has exactly three inputs — headers, body and instance; the headers
are a **copy** of the envelope headers with read-only metadata **injected by the worker
at entry and sanitized at exit** — metadata is never transported in the event itself.
The business correlation-id rides an engine-managed envelope tag (`my_cid`) and is echoed
on every HTTP response, and the RPC reply path is aligned with the Java design: a
**single reserved `temporary.inbox` route** keyed by correlation id — the `inbox.*`
namespace belongs to applications — registered with deterministic essential-service
sequencing, with the mesh-era `route@origin` syntax never generated. Re-verified live in
all four direction combinations at an **empty trace-signature diff** — the full battery
(functionality, correlation echo, injected-metadata parity, authentication, signature) is
recorded in the [Interop Test Report](docs/test-reports/event-over-http-interop.md).
The release also mirrors the team-contributed Event Script **collection plugins**
(`isEmpty`, `getFirst`, `getLast` — flows are engine-portable, so plugins must exist on
both engines). Metadata/reply-path changes in PR
[#171](https://github.com/Accenture/mercury/pull/171).

### Added

1. **The HTTP response echoes the business correlation-id
   ([#171](https://github.com/Accenture/mercury/pull/171)).** REST automation returns the
   request's correlation-id (inbound or edge-generated) on the response under the
   configured header name (default `X-Correlation-Id`), so an edge caller can correlate
   without parsing the body. A response header of the same name set by the function takes
   precedence. The edge also stamps the resolved value onto the request dataset headers,
   so the function, the flow engine (`model.cid`) and the response all see the SAME id.

2. **Collection plugins for Event Script: `isEmpty`, `getFirst`, `getLast`.** Contributed
   to the Java engine in mercury-composable
   [PR #220](https://github.com/Accenture/mercury-composable/pull/220) and mirrored here
   for flow portability — Event Script flows are engine-portable YAML, so
   `f:isEmpty(...)` behaves identically on both engines, including error text (which
   reads the same in aggregated logs). `isEmpty`: a single Collection/Map/String/array —
   true when it has no elements (use `isNull`/`notNull` for null checks; null or an
   unsupported type is an error). `getFirst`/`getLast`: a single non-empty List — its
   first/last element.

3. **The `inbox.*` route namespace belongs to applications
   ([#171](https://github.com/Accenture/mercury/pull/171)).** RPC replies now resolve
   through the ONE reserved reply-listener route, `temporary.inbox` (Java `TemporaryInbox`
   parity: private, zero-tracing, 500 instances, registered by the essential-service step
   at the highest startup priority and present on every platform from construction), keyed
   by correlation id — no per-request pseudo-route and no reserved prefix. A workflow
   application is free to register routes like `inbox.approval` (a human-operator staging
   area); they are reachable and traced like any user function. The worker's RPC-served
   detection now uses the reserved `rpc` envelope tag (Java `EventEmitter.RPC`) instead of
   a reply-address prefix. Per Eric's ruling, the legacy `route@origin` addressing syntax
   (meaningful only under the Kafka service mesh) is never generated; an inbound `@origin`
   suffix is parsed away.

### Fixed

1. **Protected metadata is never transported in the event
   ([#171](https://github.com/Accenture/mercury/pull/171)).** The business correlation-id
   now rides an engine-managed envelope tag (`tags` wire field — wire-compatible with the
   Java engine) instead of a `my_correlation_id` envelope header, and the worker injects
   the `my_*` read-only keys (`my_route`, `my_trace_id`, `my_trace_path`,
   `my_correlation_id`) into the function's input header copy at delivery — this port now
   injects the same four keys as the Java engine, so the echo demos are replicas. At exit
   the worker sanitizes a returned envelope's headers symmetrically: the `my_*` keys and
   the engine-internal `x-event-api` relay guard never leave a function as response
   headers, and neither reaches a function's view on the way in (tags are engine-visible
   only). A callee still honors the legacy header from a pre-4.10.2 peer (injected, then
   stripped), but no longer sends it — business-cid continuity in mixed fleets requires
   both sides on this version, the same upgrade-together posture as the wire format.

---
## Version 4.10.1, 7/23/2026

Patch release: **telemetry presentation parity with the Java reference engine**. Field
installations stay polyglot — DevSecOps teams aggregate both engines' telemetry and logs
in one place — so the trace-record topology and log presentation of this port must be an
**exact structural replica** of the Java engine's. After this release they are: the
normalized-signature diff is **empty in all four direction combinations** (java→java,
rust→rust, java→rust, rust→java; both calling patterns, incl. authentication). The
`/api/event` edge is a visible span aligned with the Java reference, the application log
context appears only on lines with a real request trace, the declarative demo endpoint is
renamed for symmetry, the new `event.api.auth` demo shows endpoint protection with
session-info forwarding, and the full interop story — evidence, defects, fixes, and the
learnings kept as a playbook for future language ports — is deposited in this repo's docs
as the [Interop Test Report](docs/test-reports/event-over-http-interop.md). All changes
in PR [#169](https://github.com/Accenture/mercury/pull/169).

### Added

1. **Event-over-HTTP authentication demo
   ([#169](https://github.com/Accenture/mercury/pull/169)).** The hello-world example overrides the default
   `/api/event` endpoint with a demo authentication service (`event.api.auth`) that
   validates the caller's `authorization` header against a shared secret resolved from
   the environment (`demo.peer.token: ${DEMO_PEER_TOKEN:demo}` on both peers — no
   hard-coded credential). The hello-flow example presents the token declaratively (a
   `headers` block in `event-over-http.yaml`) and programmatically (the request API's
   security headers), and session info injected by the auth service rides to the target
   function as read-only headers — REST automation now forwards auth-verdict headers as
   the request's `session` map (Java parity). The echo also forwards to a new
   `hello.pojo` function so span propagation is visible in the trace (lambda-example
   parity).

### Changed

1. **The declarative demo endpoint is renamed for symmetry with its programmatic twin
   ([#169](https://github.com/Accenture/mercury/pull/169)):**
   `/api/event/http/demo` → `/api/event/http/declarative` and flow id
   `event-over-http-demo` → `event-over-http-declarative` in the hello-flow example.
2. **REST automation dispatches the endpoint service as a CALLBACK
   ([#169](https://github.com/Accenture/mercury/pull/169))** (Java `HttpRouter`
   parity): the event carries `reply_to = async.http.response` and its `cid` is the HTTP
   context id, while the business correlation-id rides the `my_correlation_id` envelope
   header (the worker's trace bracket prefers it, so `po.my_correlation_id()` is
   unchanged). The endpoint service's worker now self-records its span — the first leg
   of every trace is a real span record — and the response leg (`async.http.response`)
   is itself a visible function span parenting onto the replying function's span. The
   telemetry topology of a two-app Event-over-HTTP call is now an exact structural
   replica of the Java engine's — verified record-for-record against the Java reference
   signature (both patterns, incl. the deliberate cross-pattern asymmetry of the
   caller-side response leg).

### Fixed

1. **The application log context no longer leaks onto context-less lines
   ([#169](https://github.com/Accenture/mercury/pull/169)).** The
   `context` block appears ONLY on log lines emitted inside a traced function execution
   with a real request trace (Java parity: the log context registers per worker
   execution in lockstep with the trace bracket). Telemetry records and framework/system
   lines carry no context block at all — previously they carried a partial block with
   constants and a timestamp.
2. **Reserved `my_*` metadata is stripped from HTTP response headers
   ([#169](https://github.com/Accenture/mercury/pull/169))** (Java
   `copyResponseHeaders` protected-metadata parity): `my_route`, `my_trace_id`,
   `my_trace_path` and `my_correlation_id` never reach the wire.
3. **The Event-over-HTTP client returns a non-envelope response as-is
   ([#169](https://github.com/Accenture/mercury/pull/169))** (e.g. an
   authentication-layer 401 in the REST error shape) with its HTTP status, instead of
   failing with "Invalid event-over-http response" (Java `handleFutureResponse` parity).

---
## Version 4.10.0, 7/22/2026

Feature release: cross-language Event-over-HTTP interoperability with the canonical
[Java implementation](https://github.com/Accenture/mercury-composable) (its v4.10.0
shipped the same day — one version, two languages). The language-neutral wire format, the
`/api/event` service and client, declarative routing, a ready-to-run demo pair covering
both calling patterns, application log context on by default, and RPC span-lineage
telemetry. Validated by live bidirectional Java ⇄ Rust interop drives — see the
[Interop Test Report](https://accenture.github.io/mercury-composable/test-reports/event-over-http-interop/)
on the Java docs site.

### Added

1. **Language-neutral event envelope wire format
   ([#166](https://github.com/Accenture/mercury/pull/166)).** The envelope's MsgPack
   map with descriptive string keys is now a cross-language contract shared verbatim
   with the Java engine (normative spec: the Java repo's
   [Event Envelope Wire Format](https://accenture.github.io/mercury-composable/guides/event-envelope-wire-format/)
   reference), proven by golden conformance vectors kept byte-identical in both repos.
   Decoders treat an absent and a nil field alike and ignore unknown keys; the v1
   service accepts the **standard** format only — a legacy Java *compact* envelope
   (single-character keys) is rejected with a clear 400 (Java 4.10+ defaults to
   standard).
2. **Event over HTTP: the `/api/event` service + client
   ([#166](https://github.com/Accenture/mercury/pull/166)).** `POST /api/event` ships in
   the default `rest.yaml` (merged like the actuators — zero configuration): RPC and
   async dispatch with `x-ttl`/`x-async` semantics, 403 for private targets, in-band
   404/400/408, and trace propagation via `x-trace-id` + W3C `traceparent`. Preloaded
   functions are now **private by default** with the `is_private = false` opt-out (Java
   `@PreLoad` parity) and every engine internal is registered private — an application
   instance is a closed world unless a function is deliberately published. The
   `event_over_http` client posts a serialized envelope to a peer and returns the reply.
3. **Declarative Event over HTTP — `yaml.event.over.http`
   ([#166](https://github.com/Accenture/mercury/pull/166)).** Routes listed in
   `event-over-http.yaml` (with optional per-target security headers and `${...}`
   substitution) forward transparently: `po.request` returns the peer's reply,
   `po.send` with a `reply_to` runs the callback dance, a plain `po.send` is
   drop-n-forget with the 202 ack, and `send_later` honors the map. The `x-event-api`
   marker is the recursion guard — a forwarded event crosses the wire exactly once.
4. **Comma-separated route aliases in `#[preload]`
   ([#167](https://github.com/Accenture/mercury/pull/167))** — Java `@PreLoad` parity:
   `route = "hello.world, hello.declarative"` registers the same function object under
   every listed name with the same instance count and visibility. Empty segments are a
   compile error.
5. **Application log context is now on by default
   ([#167](https://github.com/Accenture/mercury/pull/167)).** platform-core ships a
   built-in `default-log-context.yaml` (embedded at compile time) so the structured JSON
   formats (`log.format=json` or `compact`) stamp the standard trace context (`cid`,
   `traceId`, `tracePath`, `spanId`, `parentSpanId`, `service`, `timestamp`) into every
   log line a traced function emits — no setup required. An application can replace the
   template with its own `app-log-context.yaml`, or opt out with the new
   `app.log.context=false` key. Applications already providing an `app-log-context.yaml`
   are unaffected. Plain-text logging (`log.format=text`, the default) is unaffected.
6. **RPC telemetry records — exactly one record per span
   ([#167](https://github.com/Accenture/mercury/pull/167)).** The caller now emits the
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
7. **Event-over-HTTP demo endpoints in `hello-flow` — both patterns
   ([#167](https://github.com/Accenture/mercury/pull/167))** (the structural parallel of
   the Java composable-example, now on port **8100**): `/api/event/http/demo`
   (declarative — the flow's task is the foreign route `hello.declarative`, resolved
   through `event-over-http.yaml`) and `/api/event/http/programmatic` (the task passes
   the peer's `/api/event` URL directly to the request API). The `hello-world` echo
   registers the `hello.declarative` alias and is interchangeable with the Java
   lambda-example — same port 8085, same routes — so the demo doubles as a
   cross-language interop demo with zero configuration changes; see the walk-through in
   the [Event over HTTP](docs/guides/event-over-http.md) guide.

### Fixed

1. **HTTP client read timeout no longer truncates a sub-second TTL to 1 second
   ([#166](https://github.com/Accenture/mercury/pull/166)).**
   `AsyncHttpRequest::timeout_seconds()` rounds the TTL up, the response-timeout site
   adds a one-second wire grace, and the `event_over_http` client waits 100 ms beyond
   the remote TTL — so a peer that spends its whole TTL still delivers its in-band 408
   instead of losing to a local transport abort.
2. **The `hello-world` echo no longer drops MsgPack-binary bodies
   ([#166](https://github.com/Accenture/mercury/pull/166))** — it reflects the raw
   value instead of taking a JSON detour (JSON has no byte type; found by the
   cross-language interop matrix).
3. **A zero-traced hop no longer leaks a nested reply's span id
   ([#167](https://github.com/Accenture/mercury/pull/167))** as its own on the
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
