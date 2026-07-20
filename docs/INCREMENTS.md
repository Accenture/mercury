# Migration Increments — Historical Record

> The chronological ledger of the mercury port: **mercury-composable (Java, canonical
> v4.8.6) → Rust**, delivered bottom-up in verified increments. Each increment traces to
> the Blueprint (`bp-platform-core` → `vision-mercury` in `memory/`); the *design
> rationale* lives in [`docs/design/platform-core-port.md`](design/platform-core-port.md)
> (§4–§5i, D1–D10); the full working narrative lives in `memory/sessions/`.
>
> **Convention:** add one row + one section here as each increment lands (part of the
> increment's definition of done).

## Overview

| # | Increment | Date | Design | Tests (cumulative) |
|---|---|---|---|---|
| — | AI-enable + Vision + Blueprint (greenfield) | 2026-07-15 | — | — |
| — | Design doc v1/v2 (stack gate; config-first re-scope) | 2026-07-15 | D1–D9 | — |
| 1 | Configuration management | 2026-07-15 | §4 | 30 |
| 2 | Event-bus foundation | 2026-07-15 | §5 | 41 |
| 3 | FIFO reactive back-pressure | 2026-07-15 | §5b | 48 |
| 4 | Application lifecycle + example app | 2026-07-15 | §5c | 55 |
| 5 | OTel tracing, correlation-id, app-log-context | 2026-07-16 | §5d | 68 |
| 6 | REST automation (core) | 2026-07-16 | §5e | 83 |
| 7 | Actuator endpoints + static HTML content | 2026-07-16 | §5f | 94 |
| 8 | Static-content protocol: etag/304, no-cache, filter | 2026-07-16 | §5g | 101 |
| 9 | Lightweight RPC inbox + benchmark-reporter — **milestone closed** | 2026-07-16 | §5h | 103 |
| 10 | Annotation macros + `AutoStart` one-liner + `examples/` convention | 2026-07-16 | §5i | 105 |
| — | Event-script design doc v1 (layer-2 gate) | 2026-07-16 | E1–E9 | — |
| 11 | event-script E-1: flow model + compiler (fixture parity) | 2026-07-16 | ES §5a | 120 |
| 12 | event-script E-2: data-mapping engine (MLM primary, JSONPath queries) | 2026-07-16 | ES §5b | 139 |
| 13 | event-script E-3: platform-core extensions (interceptor, send_later, flow:) | 2026-07-16 | ES §5c | 145 |
| 14 | event-script E-4: core flow runtime (manager, executor, FlowExecutor) | 2026-07-16 | ES §5d | 146 |
| 15 | direct execution for reserved engine routes (hidden optimization) | 2026-07-16 | ES §5d.1 | 148 |
| 16 | event-script E-5: parallel + fork/join (dynamic source, ITEM/INDEX) | 2026-07-16 | ES §5e | 148 |
| 17 | event-script E-6: pipelines with for/while loops, break/continue | 2026-07-16 | ES §5f | 148 |
| 18 | event-script E-7: sub-flows, shared parent state, ext state machine | 2026-07-16 | ES §5g | 148 |
| 19 | event-script E-8: complete plugin catalog + #[simple_plugin] macro | 2026-07-17 | ES §5h | 148 |
| 20 | event-script E-9: HTTP adapter, resilience, mock, hello-flow — **layer-2 milestone closed** | 2026-07-17 | ES §5i | 148 |
| — | Knowledge-graph design doc v1 (layer-3 gate) | 2026-07-17 | KG K1–K9 | — |
| 21 | knowledge-graph K-1: MiniGraph property graph in platform-core | 2026-07-17 | KG K1 | 158 |
| 22 | knowledge-graph K-2: math expression engine (`knowledge-graph` crate) | 2026-07-17 | KG K4 | 173 |
| 23 | knowledge-graph K-3: graph compiler + registry, fixtures verbatim, resource-root hook | 2026-07-17 | KG K3/K8 | 176 |
| 24 | knowledge-graph K-4: graph runtime (executor + core skills), graph.js retired | 2026-07-17 | KG K1–K5 | 177 |
| 25 | knowledge-graph K-5: platform-core HTTP client + graph.api.fetcher | 2026-07-17 | KG K6b | 177 |
| 26 | knowledge-graph K-6: graph.extension (sub-graph + flow:// delegation) | 2026-07-17 | KG K2 | 178 |
| 27 | knowledge-graph K-7a: platform-core WebSocket server (hyper upgrade + tungstenite) | 2026-07-17 | KG K6a | 179 |
| 28 | knowledge-graph K-7b: the Playground (command grammar, traveler, companion API, dev-gating K9) | 2026-07-17 | KG K9 | 181 |
| 29 | knowledge-graph K-8: React webapp + `minigraph-playground` app — **LAYER 3 MILESTONE CLOSED** | 2026-07-18 | KG K7/K8 | 181 |
| 30 | `#[optional_service]` macro (Java `@OptionalService`) + dev mock data providers | 2026-07-18 | KG K9 | 188 |
| 31 | full declarative dev-gating for the Playground (`#[optional_service]` on all registration kinds) | 2026-07-18 | KG K9 | — |
| 32 | `inspect` docs: `{…}` is a placeholder, not literal (both repos) | 2026-07-18 | — | — |
| 33 | `serializer.null.transport` (Java null-omission parity) | 2026-07-18 | D3 | — |
| 34 | `#[optional_service]` promoted to a first-class, order-independent attribute | 2026-07-19 | KG K9 | 201 |
| 35 | companion `/sync` ok-heuristic: whole-output classification (finding #40, both ports) | 2026-07-19 | ADR-0008 | — |
| 36 | HTTP-boundary content-type dispatch: exact Java parity (no sniffing, binary path, form fields) | 2026-07-19 | D10 | — |
| 37 | Spring config names retired: `APP_PROFILES_ACTIVE`/`app.profiles.active` rename + `application.name` alone for the app name | 2026-07-19 | §8 Q1 | 202 |
| 38 | `graph.math` `for_each`/`BEGIN`/`END` engine-verified spec (finding #29) — probe fixture + grammar/catalog/help docs | 2026-07-19 | — | 202 |
| 39 | numeric promotion for the simple-plugin arithmetic family + new `f:round` half-up decimal rounding (both ports) | 2026-07-19 | — | 202 |
| 40 | join barrier counts only valid completions: success-only `skill_run` + `RESET` clears the completion mark (latent premature-join bug, both ports) | 2026-07-19 | — | 202 |
| 41 | chained join judges an upstream join by its recorded outcome (fired vs sank; both ports) | 2026-07-19 | — | 202 |
| 42 | discovery commands `list graphs` / `list flows` — self-service `extension=` delegation targets (finding #38, both ports) | 2026-07-20 | — | 202 |
| 43 | human docs site phase 1: MkDocs+Material scaffold, Home + Getting Started, strict-build CI | 2026-07-20 | D-H1/D-H2 | — |
| 44 | human docs site phase 2: Foundations trio + Layer 1/2 guides; layer-organized nav (human + AI docs per layer) | 2026-07-20 | D-H2 | — |
| 45 | human docs site phase 3: KG human pages + REST automation + the six D-H2 reference conversions (12 pages) | 2026-07-20 | D-H2 | — |
| 46 | human docs site phase 4 — COMPLETE: port-scope page, Home/Getting-Started polish, final strict pass (20 nav pages) | 2026-07-20 | D-H1/D-H2 | — |
| 47 | `describe graph {graph-id}` — a deployed model's contract view (finding #53) + differentiated tutorial purposes (#54); both ports | 2026-07-20 | — | 202 |
| 48 | outbound HTTPS for the async HTTP client (rustls + OS trust store, `trust_all_cert` parity) — Rust-only parity work | 2026-07-20 | — | 206 |

Every increment ships with `cargo build` + `cargo test` + `cargo clippy --all-targets` +
`cargo fmt --check` clean, and (from increment 4 on) a live run of the hello-world
example app (`cargo run -p hello-world`; before increment 10, a cargo example)
demonstrating the new capability end-to-end.

## Scope decisions (maintainer)

- **In scope:** the three layers — platform-core → event-script → active knowledge graph —
  ported bottom-up, foundation → UI. The Java repo is the canonical behavior spec
  (*map, don't mirror*).
- **Out of scope:** the Kafka service mesh (`minimalist-kafka`, `twin-kafka`,
  `connectors/`) — for simplicity; **Spring** (`rest-spring-3/-4`) — Java-only
  (platform-core's own REST automation is the HTTP boundary instead).
- **Wire format:** idiomatic serde MsgPack — *not* byte-compatible with Java (cross-JVM
  interop is out of scope with the mesh).
- **Config compatibility:** configuration files are data — `classpath:/`, `file:/`,
  `${ENV:default}`, dotted keys, and the Spring-named keys (`SPRING_PROFILES_ACTIVE`,
  `spring.application.name`) are kept **verbatim** so files port unchanged between the
  Java and Rust versions (side-by-side comparison during migration; a generic
  `app.profiles.active` alias may come once the foundation is robust).

---

## Increment 1 — Configuration management (2026-07-15)

*Maintainer-directed first step: "everything relies on configuration management — main
app, unit tests and integration tests."*

- `MultiLevelMap` + `ConfigValue` — composite dot-bracket keys (`a.b[0].c`), flat-map ↔
  tree normalization (the `Utility.getFlatMap` analog).
- `ConfigReader` — `.yml`/`.yaml` (interchangeable), `.json`, `.properties`;
  `classpath:`/`file:` resolution over the **resource-roots convention** (the classpath
  analog; tests shadow main resources); `${VAR:default}` substitution with the exact Java
  precedence (override registry → env var → base-config reference with loop detection →
  default); `../` traversal rejected; YAML-tab tolerance (ported quirk).
- `AppConfigReader` — the base-config singleton: `app-config-reader.yml` manifest
  (embedded default, app copy overrides), merge order `bootstrap.properties` →
  `bootstrap.yml` → `application.properties` → `application.yml`, active-profile overlays,
  resolve-once-after-merge.
- **Override registry** — the `System.getProperty` analog, checked first in every lookup.
- Notable: Java's file-alt-path bug (opens the primary when the secondary matched) was
  fixed rather than ported.

## Increment 2 — Event-bus foundation (2026-07-15)

The actor-model core: functions addressed **only by route name**, exchanging immutable
envelopes — no direct calls between functions (the defining invariant).

- `EventEnvelope` — metadata + headers + dynamic body; MsgPack wire format (idiomatic
  serde, D4); fluent builders; `status()` defaults 200, `has_error()` ≥ 400.
- `ComposableFunction` (untyped registry currency) + `TypedFunction<I,O>`/`TypedAdapter`
  (the `TypedLambdaFunction` authoring surface); `AppError` = `AppException`.
- `Platform` registry — route validation (`Utility.validServiceName` rules), N worker
  instances per route (1-based, Java parity).
- `PostOffice` — `send` (fire-and-forget) + `request` (RPC via temporary `inbox.<uuid>`
  route + oneshot + correlation id; timeout → 408).
- tokio = the virtual-thread analog; `instances = N` → N worker tasks per route.

## Increment 3 — FIFO reactive back-pressure (2026-07-15)

*Maintainer-directed: port the FIFO reactive back-pressure handler; ignore Berkeley DB.*

- `ElasticQueue` (the `FileElasticStore` semantics; with BDB ignored the `ElasticStore`
  strategy facade collapses into one type): per-route two-tier FIFO — first 20 events in
  memory, overflow to segmented append-only files, record format **byte-identical to
  Java** (`[4-byte BE length][payload]`); sealed + fully-consumed segments deleted
  immediately (O(1) reclamation — the reason the file store replaced Berkeley DB);
  drained → counters reset, generation++.
- **Manager-worker dispatch** (the `ServiceQueue` state machine): workers *pull* via ready
  signals (one in-flight event each); no free worker → buffering through the elastic
  queue; bounded manager mailbox → senders await (back-pressure, not drops).
- Verified live: a 60-event burst against a single slow worker spills to disk, preserves
  strict FIFO order, and reclaims every segment after drain.

## Increment 4 — Application lifecycle + example app (2026-07-15)

- `AppStarter`/`EntryPoint` — the `AutoStart`/`AppStarter` port with the exact Java phase
  order: essential services (seq 0, framework) → before-application hooks by sequence
  (failure aborts) → preload → HTTP server slot → main applications by sequence (missing
  main = error). Explicit builder (no classpath scanning — a `#[preload]` macro is the
  later ergonomic layer).
- Platform identity: `get_instance()` (process-wide), `name()`, `origin()`.
- Elastic-store housekeeping completed: RUNNING keep-alive, expired-store scan,
  `shutdown_cleanup()`.
- **`hello_world` example** — the mercury-composable README "greeting.demo" taste,
  bootable: config → lifecycle → route-name RPC.

## Increment 5 — OTel tracing, correlation-id, app-log-context (2026-07-16)

*Maintainer-directed: telemetry is foundation, before REST automation.*

- W3C/OpenTelemetry-compatible ids (32-hex trace, 16-hex span); the envelope carries the
  sender's `span_id` → the receiver's `parent_span_id` (causal span tree).
- The worker trace bracket as a **tokio `task_local!`** (the Java per-worker anchor —
  deliberately not ThreadLocal/MDC); automatic trace + business-cid propagation in
  `PostOffice`; `annotate_trace` (span sink) vs `update_context` (log sink) — two sinks,
  neither leaks into the other; `my_correlation_id()`.
- `Telemetry` service at `distributed.tracing` (essential phase): logs span datasets in
  real time; `distributed.trace.forwarder` (future OTLP exporter) + journal hooks.
- App-log-context (`app-log-context.yaml`, opt-in): `$token` | `${ENV:default}` | literal;
  the `context` block joins logs to spans (same trace/span ids).
- Logger: `text` | `json` (pretty) | `compact` (jsonl) + **`-Dkey=value` runtime
  overrides** (the JVM `-D` analog, feeding the increment-1 override registry).
- `W3cTrace` (traceparent format/parse) ported for the HTTP edge.
- Two real bugs found in verification: a OnceLock re-entrancy deadlock (log-context config
  logging inside its own initializer) and a registration TOCTOU race.

## Increment 6 — REST automation core (2026-07-16)

The HTTP protocol boundary, ported against the Java project's own agent-ready grammar
(`docs/guides/rest-automation/rest-grammar.md`).

- `rest.yaml` — function binding, methods (OPTIONS auto), `{param}` + trailing-`*` URLs
  (exact > param > wildcard, case-insensitive), timeout clamp 1 s–5 m, CORS blocks,
  header add/drop/keep transforms, simple-route authentication, per-entry trace/cid
  header impedance overrides. Grammar invariants enforced at load.
- **hyper** HTTP server (D10 — deliberately no web framework: rest.yaml *is* the router).
- The edge **always ensures a business correlation-id** (exposed via the reserved
  `my_correlation_id` header) and **starts traces** (`tracing: true`: valid W3C
  `traceparent` wins and its parent-id becomes our parent span → else trace-id header →
  else generated; legacy conflation yields one id).
- `AsyncHttpRequest`-shaped events (Java keys); envelope → HTTP response mapping; the Java
  error shape `{status, message, type: "error"}`.
- Verified live: an upstream `traceparent` flows through `greeting.api` → `greeting.demo`
  with correct parent-span lineage at every hop and the business cid end-to-end.
- Deferred: flow binding (needs event-script), HTTP(S) relay, A/B dual service, multipart
  upload, response streaming.

## Increment 7 — Actuator endpoints + static HTML content (2026-07-16)

*Maintainer-directed scope; `/info/lib` deferred by agreement (no runtime dependency
manifest in a Rust binary — a build.rs-embedded cargo metadata could provide it later).*

- `/info` (identity, runtime, origin, uptime), `/env` (**opt-in** lists only —
  `show.env.variables` / `show.application.properties`), `/health`
  (mandatory/optional dependency routes via the `type=info` → `type=health` protocol;
  DOWN = HTTP 400, Java parity), `/livenessprobe` (follows the last health outcome).
- **Default-endpoint merge** (the `default-rest.yaml` semantics): actuators appear only
  when `rest.yaml` doesn't claim the URL — user entries always win.
- **Static HTML content from `resources/public`**: `/` → `index.html`, directory paths,
  traversal-guarded, content type by extension; a rest.yaml `/` entry always wins.
- The example is now a complete miniature app: a static landing page linking a traced
  API endpoint and all four actuators.

## Increment 8 — Static-content protocol: etag/304, no-cache pages, request filter (2026-07-16)

*Maintainer-directed; reference: the Java platform-core `test/resources/rest.yaml`
`static-content` block.*

- **ETag / HTTP-304**: quoted SHA-256 content hash; comma-aware `If-None-Match` → 304
  with an empty body; stale tags re-serve.
- **No-cache pages** (default `["/", "/index.html"]`): `Cache-Control: no-cache,
  no-store` + `Pragma` + epoch `Expires` — entry pages always revalidate (the SSO case).
- **Request filter** (`static-content.filter`: path/exclusion/service; exact / `prefix*`
  / `*suffix` patterns): a composable function inspects matching static requests; its
  response headers are always copied; 200 continues serving, any other status (e.g.
  302 + `Location`) passes through — the SSO-redirection hook. The hello_world demo ships
  an `http.request.filter` interceptor logging url/ip/user-agent.
- Path resolution tightened to Java rules (extensionless → `.html`).
- Verified live: no-cache + `x-filter` headers on `/`, a real 304 revalidation cycle,
  and the interceptor's inspection log.

---

## Increment 9 — Lightweight RPC inbox + benchmark-reporter (2026-07-16) — **platform-core milestone closed**

*Maintainer-directed closure: benchmark the foundation the event-script and
knowledge-graph layers will ride on.*

- **Lightweight RPC inbox** (Java `AsyncInbox` parity): an RPC reply is now a one-shot
  correlation-map entry, not a throwaway route registration — pulled forward from the
  deferred list so the benchmark measures dispatch, not inbox overhead.
- **`benchmark/benchmark-reporter`**: the Java harness ported — same six scenarios, same
  stats, same self-contained HTML report; `-Dbench.*` runtime parameters.
- **The record** ([`analysis/rust-tokio.html`](../benchmark/benchmark-reporter/analysis/rust-tokio.html),
  defaults, Apple Silicon 12-core, vs the Java file-vthread record on the same machine
  class): baseline RPC **155K ops/s @ 6 µs mean (8.4× Java)**; balanced **411K ops/s
  (2.3×)**; overload ~1.4× and loss-free through the disk spill; the mixed latency probe
  **17 µs mean / 210 µs max vs 157 µs / 1.62 ms (~9×)** — no GC, no GC pause, tails stay
  near the median. 1,003,000 timed operations, **0 failures**. Analysis:
  [`analysis/README.md`](../benchmark/benchmark-reporter/analysis/README.md).

---

## Increment 10 — Annotation macros + `AutoStart` one-liner + `examples/` convention (2026-07-16)

*Maintainer-directed: two enhancements before event-script (layer 2).*

- **`crates/platform-macros`** — `#[preload(route, instances, env_instances, typed)]`,
  `#[before_application(sequence)]`, `#[main_application]`, and the stacked
  `#[zero_tracing]` marker: the Java `@PreLoad` / `@BeforeApplication` /
  `@MainApplication` / `@ZeroTracing` annotation analogs. Registration is **link-time**
  (the `inventory` crate) — the D6 answer to Java's classpath scanning, and it works
  across crates, so layer-2/3 library functions will register like app-local ones.
- **`AutoStart`** (Java `AutoStart.main(args)` parity): overrides → logging → collect
  annotations → lifecycle → serve until Ctrl-C → graceful shutdown (this also ships the
  deferred OS-signal wiring). A user application's whole `main()` is now the one line
  `platform_core::auto_start_main!();`.
- **`examples/hello-world/`** — the hello-world demo moved from a cargo example to a
  standalone workspace app crate (annotated functions + the one-liner; its `resources/`
  beside it). The convention for the coming event-script and knowledge-graph example
  apps: one `examples/<name>/` crate each.
- Verified: end-to-end annotation lifecycle test (hook ordering, `env_instances`
  config resolution, typed RPC in a trace bracket, `#[zero_tracing]` suppression) + live
  run of the relocated app (REST, etag/304, filter, actuators unchanged).

---

## Increment 11 — event-script E-1: flow model + compiler (2026-07-16)

*Layer 2 begins. Design `docs/design/event-script-port.md` approved same day
(decisions E1–E9, defaults accepted).*

- **`crates/event-script`** (new workspace member, depends only on platform-core):
  compiled `Flow`/`Task` model, flow-template registry, and the full `CompileFlows`
  port — `yaml.flow.automation` discovery, the complete grammar validation
  (`flow-grammar.md` is the spec), and Java failure semantics (invalid flow skipped
  with ERROR; invalid data mapping drops the *task* while the flow loads).
- **Legacy-syntax converter** (`:type` qualifiers → `f:plugin(...)`) and the
  compile-time mapping validator (incl. the reserved `model.cid/instance/flow/ttl/
  trace/none` guard) ported; the simple-plugin **name** registry pulled forward from
  E-8 (42 built-in names) because input mappings validate `f:` names at compile time.
- **Fixture parity (E2):** all 90 Java flow fixtures copied verbatim; tests pin the
  exact loaded-flow set, every rejection, task-drop semantics, and the normalized
  mapping strings. Two legacy-named fixtures verified against the Java *code* as
  loading (not their comments): `invalid-condition-mode`, `ext.user` dot-form.
- Engine self-registers through the increment-10 annotation layer
  (`#[before_application(sequence = 5)]`).

---

## Increment 12 — event-script E-2: data-mapping engine (2026-07-16)

*Maintainer refinement at the gate: **MultiLevelMap (direct composite-key access) is the
primary data-mapping tool** — lightweight; **JSONPath (`$.…`) serves user-defined complex
queries**. The Java code is layered the same way, so parity and the refinement coincide.*

- **Runtime `MultiLevelMap`** over `rmpv::Value` — the bus currency, so state↔envelope
  moves need no conversion and byte arrays stay real binary (Java `byte[]` parity).
  Java semantics throughout: null vs missing, list padding, stable indices on removal,
  `key[]` append. `$.…` delegates to `serde_json_path` (RFC 9535) on an on-demand JSON
  view.
- **Mapping resolution** (`DataMappingHelper` port): constants (incl. `map(config.key)`
  and `file()`/`classpath()` content), `f:plugin(...)` invocation (top-level-comma
  argument split, nested-plugin null guard), legacy `:type` commands
  (error → pass-through + ERROR log), `{model.key}` runtime interpolation.
- **19 core plugin bodies** (the ones the legacy-syntax converter emits) now execute;
  remaining built-ins fail loudly until E-8. Type conversions match Java exactly
  (String.valueOf display parity, −1 numeric fallbacks with decimal-drop).
- **Parity capstone test**: the compiled greetings fixture's mappings evaluated against
  a simulated HTTP dataset produce the exact function-input body the Java engine feeds
  `greeting.test`.

---

## Increment 13 — event-script E-3: platform-core extensions (2026-07-16)

*The four extensions the flow engine rides on (design E5), landed in platform-core
with their own tests.*

- **Event-interceptor mode** (Java `@EventInterceptor`): `FunctionOptions` on
  registration; the worker ignores an interceptor's successful return (manual replies
  via `po.send`) while failures still route to `reply_to` — the exact Java
  `WorkerHandler` split. `#[preload]` gains `interceptor` / stacked
  `#[event_interceptor]`.
- **Scheduled events**: `send_later`/`cancel_future_event` (abortable tokio timer,
  self-removing) — the flow TTL watcher's substrate.
- **rest.yaml `flow:` binding**: injected as the `x-flow-id` header for
  `http.flow.adapter`; closes the increment-6 flow-binding deferral.
- **Deep-copy**: satisfied by design (`rmpv::Value::clone()` is a deep copy) — no API.

---

## Increment 14 — event-script E-4: core flow runtime (2026-07-16)

*Flows execute. The engine (compiler → manager → executor) self-registers through the
annotation inventory; every task execution is an event over the layer-1 bus.*

- **`FlowInstance`** (state machine `{input, model}` + TTL watcher on `send_later`),
  instance registry, **`EventScriptManager`** + **`TaskExecutor`** as event
  interceptors (one instance each — Java parity, callbacks serialize), and
  **`FlowExecutor::launch`/`request`**.
- Execution types `sequential`/`response`/`end`/`decision`/`sink` with exception
  routing, TTL abort (408), per-task metrics, the traced flow-summary span, deferred
  tasks, `@retry` decisions, `file()` output targets, and the `*` wildcard body.
  Later-increment constructs abort with explicit messages.
- Consolidated mapping view built **in the instance's dataset tree** (scratch keys
  stripped per callback) — `model.*` writes persist like Java's shared-reference map,
  zero model copies; dynamic RHS targets re-checked against the reserved-key guard.
- E2E over the canonical fixtures: greetings, decisions (bool/numeric/out-of-range),
  sequential + wildcard, response-before-end, exception → handler, TTL abort,
  dynamic reserved-key rejection, fire-and-forget launch.

---

## Increment 15 — direct execution for the reserved engine routes (2026-07-16)

*From a maintainer design review of Java's `EventEmitter.sendWithEventBus`: the two
Event Script routes run as part of the event core. Ported — and deliberately hidden
(no macro flag, no registration option), so application functions cannot opt out of
reactive back-pressure.*

- `Platform::deliver` + the worker reply path check a **private** reserved-route list
  (`event.script.manager`, `task.executor`) and run those functions directly on a
  fresh task — no queue, no trace bracket (Java parity: only the flow-summary span).
- Rust rationale: not serialization (our bus is zero-copy) but **concurrency** (no
  single-worker orchestration bottleneck) and **liveness** (the engine can't deadlock
  on its own bounded mailbox under saturation).
- Proof: reserved routes reach peak concurrency > 1 with one worker instance while a
  normal control route serializes (peak exactly 1); 20 simultaneous flows complete.

---

## Increment 16 — event-script E-5: parallel + fork/join (2026-07-16)

- **`parallel`** fan-out and **`fork`/`join`** with the pipe-map barrier
  (`JoinTaskInfo`); dynamic `source` iteration replicates a single branch per model-list
  element with `.ITEM`/`.INDEX` pseudo-keys; Java-exact exception cleanup of pipe
  queues.
- Rides increment 15's direct execution: forked callbacks are genuinely concurrent;
  the dataset and pipe-map mutexes carry the thread-safety (proven by concurrent
  `[]`-append assertions).
- Canonical parallel-test + fork-n-join-test fixtures run verbatim; a marked Rust-side
  supplement covers dynamic fork until its canonical fixture's E-7 dependencies land.

---

## Increment 17 — event-script E-6: pipelines + loops (2026-07-16)

- **`pipeline`** execution with `PipelineState` in the pipe map: ordered steps, pass
  completion, exit task; **`for`** (initializer/comparator/sequencer) and **`while`**
  loops; **`break`/`continue`** conditions evaluated after every step (continue clears
  its flag — Java parity).
- Canonical fixtures verbatim: pipeline-test, for-loop-test (incl. the `file()`
  append/read/delete round-trip), for-loop-break, while-loop (per-step `delay`),
  pipeline-exception (step handler + pipe cleanup). `decision.case` upgraded to the
  faithful Java `DecisionCase` port.

---

## Increment 18 — event-script E-7: sub-flows + shared state + ext (2026-07-16)

- **`flow://` sub-flows** launched through the manager; the child's response returns
  as the parent task's callback (mappings/exceptions/fork barriers apply unchanged);
  ttl + business cid + shared state inherited.
- **Shared parent state**: `Arc<Mutex<tree>>` per family (Java aliases by reference —
  Rust materializes at `model.parent` per mapping pass under the shared lock;
  `model.root.*` normalizes to `model.parent.*`).
- **`ext:` external state machine** (route + `flow://` forms; calls dispatched after
  lock release); `SimpleExceptionHandler` built-in ported.
- Activated fixtures: parent/daughter greetings (alias round-trip), missing-sub-flow,
  externalize put/get, fork-n-join-flows, and the **canonical dynamic-fork fixture** —
  five concurrent sub-flows, shared-state appends exactly-once.

---

## Increment 19 — event-script E-8: the plugin catalog + `#[simple_plugin]` (2026-07-17)

- All **42 built-in plugins** execute (arithmetic, generators, dates, comparisons,
  list-of-map operations, the full `validate` rule engine) with Java-exact semantics
  and error messages.
- **`#[simple_plugin]`** (new `event-script-macros` crate): user plugin functions
  register through the link-time inventory, collected by the `SimplePluginLoader`
  at sequence 3 — before flows compile, so `f:` names validate.
- Fixtures activated: arithmetic, type-conversion (real-bytes body asserted on the
  rmpv tree), string-util, parse-date(-time), input-validation; plus a user plugin
  proven end-to-end.

---

## Increment 20 — event-script E-9: adapter + resilience + mock (2026-07-17) — **EVENT-SCRIPT MILESTONE CLOSED**

- **`HttpToFlow`**: `flow:`-bound endpoints launch flows with the HTTP edge's reply
  routing preserved; the edge correlation header becomes `model.cid`.
- **`Resilience4Flow`**: retry / abort / alternative-path decisions, attempt counting,
  delayed retries, cumulative-failure backoff — the resilience-demo and
  simple-circuit-breaker fixtures run verbatim.
- **`EventScriptMock`**: task-route reassign/restore via a dispatch-time override
  registry (monitors not ported — documented).
- **`examples/hello-flow`**: a YAML flow served over HTTP with two annotated
  functions and a one-line main — live-verified in both languages with cid
  propagation.

**Layer 2 closed**: E-1…E-9 = the complete Event Script engine on the measured
layer-1 foundation. Next layer: **active knowledge graph (layer 3)**.

---

## Increment 21 — knowledge-graph K-1: MiniGraph in platform-core (2026-07-17)

- **`platform_core::graph`** — faithful port of the Java `MiniGraph` (827 lines) +
  its models (`SimpleNode`, `SimpleConnection`, `SimpleRelationship`,
  `GraphProperties`): reserved aliases, the `0-9 A-Z a-z _ -` name charset with
  Java-exact error messages, case-insensitive alias/type/property-key lookups,
  idempotent `connect` (the existing connection is returned), successor/predecessor
  adjacency, neighbors/forward/backward links, BFS level discovery (`find_paths`),
  deterministic sorted `export_graph`/`import_graph` (the graph JSON file format —
  the layer-3 tutorial fixtures' shape), `same_as` deep comparison, `reset`, and the
  750-node default cap.
- **Rust translation choices**: Java's shared mutable objects become `Arc`-shared
  nodes/connections/relations with interior mutability; property values are
  `rmpv::Value` (the envelope/state-machine currency), so graph JSON round-trips
  through the same conversions as layer 2; errors are `Result<_, AppError>` with
  status 400 (Java throws `IllegalArgumentException`). One deliberate divergence:
  `remove_node` removes the lowercased alias key (Java has a latent
  case-sensitivity slip there).
- **Parity suite**: `tests/graph.rs` — all 8 Java `GraphTest` methods ported (node,
  directional, import/export incl. a JSON round-trip, six exception suites) plus a
  max-nodes/import-failure test; 10 tests.

## Increment 22 — knowledge-graph K-2: the math expression engine (2026-07-17)

- **New crate `crates/knowledge-graph`** (created one increment ahead of the K-3 plan to
  host the engine's first module; the compiler/registry and resource-root hook still land
  at K-3). Its doc-comment records the `graph.js` retirement rationale.
- **`knowledge_graph::math`** — faithful port of the Java `com.accenture.minigraph.math`
  package (979 lines): character-addressed lexer, the non-recursive shunting-yard parser
  with postfix call/member chains and the strict JS rule (`-2 ** 2` is a parse error),
  and the recursive evaluator — short-circuit `&&`/`||`, ternaries, string concatenation
  (JS-like number rendering: `'answer=' + 3` → `answer=3`) vs display rendering
  (`as_string()` keeps Java's `3.0`), string/number relational comparison,
  same-type-only equality with `NaN != NaN`, and the `EvalContext` whose constants and
  functions mirror into the `Math.*` namespace.
- **Rust translation choices**: `MathError::Parse`/`Eval` split mirrors Java's
  `ParseException` vs `IllegalArgumentException`; functions are `Arc<dyn Fn(&[f64]) ->
  Result<f64>>` so a user function can fail (the short-circuit tests rely on it);
  `random()` draws OS entropy via `getrandom` (the `SecureRandom` analog); `round`
  reproduces Java `Math.round` (floor(x+0.5), NaN→0) rather than Rust's
  half-away-from-zero.
- **Parity suite**: all 14 `ExpressionEngineFullTest` methods + an added
  random/arity/coercion test; 15 tests, green first run.
## Increment 23 — knowledge-graph K-3: graph compiler + registry (2026-07-17)

- **`compiler::compile_graphs`** (Java `CompileGraph`, `@BeforeApplication(sequence=6)`)
  — the graph-model quality gate: reads the opt-in `graph.model.automation` manifest,
  loads each `{location.graph.deployed}/{id}.json` through `ConfigReader` (so `${...}`
  references resolve against the app config, Java parity), converts deprecated
  "simple type matching" mapping entries to plugin syntax via the shared event-script
  converter (layer 3 riding layer 2), validates structurally through
  `MiniGraph::import_graph`, and registers the model. An invalid graph is skipped with
  an error log; an unreadable manifest is a warning (Java failure semantics).
- **`graphs` registry** (Java `CompiledGraphs`): process-wide validated-model store the
  graph executor will consult before lazy per-request loading.
- **K8 resource-root hook**: `#[before_application(sequence = 1)]` appends the engine
  crate's `CARGO_MANIFEST_DIR/resources` — the jar-classpath analog; appended (never
  prepended) so the application's own `resources/` always wins. Runs before both the
  flow compiler (5) and the graph compiler (6).
- **Fixtures verbatim**: the 13 `tutorial-*.json` graphs travel with the engine crate
  (`resources/graph/`); the 13 Java test-only graphs + `graphs.yaml` manifest mirror
  `src/test/resources` (`tests/resources/`). All 26 manifest graphs compile.
- **Correction to the design sketch**: Java's `PlaygroundLoader` is the `FetchFeature`
  scanner (API-fetcher features), not a graph loader — it moves to K-5 with
  `graph.api.fetcher`.
- **Parity suite**: `CompileGraphTest` ported (manifest gating, deprecated-syntax
  conversion) + a `${...}`-resolution check; 3 tests.
## Increment 24 — knowledge-graph K-4: the graph runtime (2026-07-17)

- **`graph.executor`** (zero-tracing event interceptor, Java `GraphExecutor`): a plain
  correlation id starts a traversal from the root node; a composite
  `{flowInstanceId}@{nodeName}` id is a skill callback deciding the next hop (`next`,
  a node alias to jump to, `.sink` to stop a branch). Compiled models are reused;
  unlisted graphs load lazily; `tutorial-*` ids are dev-gated. Loop detection aborts a
  node exceeding `graph.node.high.frequency` hits within `graph.max.loop.interval`.
  Exposed through the verbatim `graph-executor.yml` flow (`POST /api/graph/{graph_id}`
  once REST automation binds it) — layer 3 riding layer 2.
- **Core skills**: `graph.data.mapper` (the event-script mapping mini-language over the
  graph state machine), `graph.math` (statements: COMPUTE/IF-THEN-ELSE/MAPPING/RESET/
  DELAY/NEXT, `EXECUTE:` merge, BEGIN/END for-each blocks — powered by the K-2
  expression engine), `graph.task` (whole-body `*` staging, request/response headers,
  `for_each` fork-join with clamped concurrency, per-node exception routing),
  `graph.join` (barrier over backward links) and `graph.island` (terminal `.sink`).
- **Support services**: `graph.housekeeper` wired as an end-flow listener (the
  event-script end-flow advice clears the traversal state), `graph.exception.handler`
  (the flow-level error normalizer), `graph.health` (the template actuator check).
- **`GraphLambdaFunction` base** ported as `common.rs` free functions: `{var}`
  substitution with logical-quoting rules, RHS validation with reserved properties,
  node-property seeding, `for_each` resolution, fetcher-style output mapping,
  statement-block splitting. `GraphInstance` state is a mutex-scoped rmpv
  `MultiLevelMap` (guards never cross awaits).
- **`graph.js` RETIRED in code**: the route is never registered and the executor fails
  a `skill: graph.js` node with an explicit message pointing at `graph.math` /
  `graph.task` (maintainer security decision — deliberate divergence from Java, which
  ships a GraalVM interpreter for lack of an alternative).
- **Deferred within layer 3**: `GraphTraveler` (dev-only Playground walker) moves to
  K-7 with sessions; fetcher/extension tutorials activate at K-5/K-6.
- **`AutoStart` idempotency restored** (maintainer review): Java guards `AutoStart.main`
  with an `AtomicBoolean` (repeated execution is a no-op); the Rust port was missing the
  guard — added in platform-core and asserted in the E2E suite.
- **E2E suite** (one flow-engine boot): tutorials 1/2/4/7/8/9/13 + `GraphTaskTest`
  unit-test-task-1..5 (Java-parity task functions ported) + Rust-supplement graphs for
  the join barrier, loop detection and the retirement message; graph.health checks.
## Increment 25 — knowledge-graph K-5: HTTP client + API fetcher (2026-07-17)

- **platform-core `async.http.request`** (Java `AsyncHttpClient`, closing the design §7
  deferral — a lockstep layer-1 extension): an event interceptor registered by the app
  starter (Java `EssentialServiceLoader` parity, 500 instances, untraced via the
  existing `skip.rpc.tracing` default). Per-request hyper http1 connections (Java
  creates a client per request too), the header ignore-list, `user-agent:
  async-http-client`, cookies/session, per-request `x-ttl` timeout (default 30s),
  connect timeout config, and content-type-driven response decoding (JSON object/
  array/text, `x-content-length` when the server omits content-length). **Outbound
  trace propagation** reads the ENVELOPE trace + the injected invocation headers —
  exactly Java's `PostOffice.trackable(headers)` model, since the route itself is
  untraced: `X-Trace-Id` (configurable) + W3C `traceparent` + the business
  correlation-id header. Documented deferrals: object streams/multipart (await the
  streams port), XML bodies pass through as text, `https` rejected with an explicit
  error until a TLS stack is adopted. Header values are trimmed at wire time (netty
  strips OWS; hyper strictly rejects it — e.g. a token file's trailing newline).
- **`AsyncHttpRequest`** map-backed builder/parser in platform-core (method, host,
  url + `{path}` substitution + query merge, headers, body, cookies, session).
- **E-9 http-client fixtures ACTIVATED** in event-script: an `echo.endpoint` +
  rest.yaml join the test resources; `http-client-by-config` runs E2E (bearer token
  from `classpath(text:...)`, query/path parameters on the wire) and the Java
  trace-propagation test runs in its full shape — outer request through the real HTTP
  edge with a W3C `traceparent`, adopted by the adapter, carried by the flow's
  declarative `async.http.request` task, observed by the downstream echo.
- **`graph.api.fetcher`** (Java `GraphApiFetcher`, 512 lines): dictionary/provider
  model with `key:default` input fallbacks, provider `input` mapping
  (`path_parameter.*`/`query.*`/`header.*`/`body.*`), per-instance provider cache,
  `response.*`→`result.*` dictionary output mapping (`[]`-appended per fork-join
  response), `for_each` fan-out with clamped concurrency, break-on-exception vs
  per-node exception routing. **Features** (Java `@FetchFeature` scan → explicit
  registry): `FeatureRunner` trait, `features::register`, built-ins
  `log-request-headers`/`log-response-headers`; unimplemented features get the
  throttled advisory.
- **E2E**: the mock services (`mock.mdm.profile`, `mock.account.details`) + 7 mock
  JSON fixtures verbatim + mock rest.yaml; the test boots the real REST server —
  tutorials 3 (+negative), 5, 6, 12, 114 and unit-test-1 all pass over real HTTP.
  hello/helloworld/helloworld2 wait for `graph.extension` (K-6); the graphs carrying
  `graph.js` are activated at K-6 by the maintainer-directed swap to `graph.math`.
## Increment 26 — knowledge-graph K-6: graph.extension (2026-07-17)

- **`graph.extension`** (Java `GraphExtension`, 266 lines): a node delegates to another
  deployed graph (launched as its own flow instance through the `graph-executor` flow
  with `path_parameter.graph_id`) or to an event-script flow (`flow://<id>`, validated
  against the flow registry). Input mapping stages the delegated body; `for_each` fans
  the delegation out with clamped concurrency; responses land in the node's `result`
  (`[]`-appended per fork-join call) for fetcher-style output mapping;
  break-on-exception vs per-node exception routing (Java parity).
- **`flow-11.yml`** joins the engine-shipped flows (verbatim; the flows.yaml manifest
  now matches the Java module).
- **Two more parity gaps found and fixed at the right layer**:
  (a) **`no.op` is a platform-core built-in in Java** (`NoOpFunction`, 500 instances,
  `worker.instances.no.op` override) — the port had it only in the event-script test
  binary; now registered by the app starter, echoing headers + body like Java.
  (b) **JSONPath member names with hyphens**: Java's Jayway engine tolerates
  `$.fetcher-ext.result` in dot notation; `serde_json_path` is RFC 9535-strict — the
  event-script `MultiLevelMap` now rewrites such segments to bracket notation when
  strict parsing fails (unit-tested).
- **Scope adjustment recorded**: the "remaining REST endpoints" sketched for K-6
  (describe/upload/inspect/live-graph) all hang off `GraphCommandService` and the
  Playground draft/temp dirs — they move to K-7 with the Playground.
- **E2E**: tutorial-10 (extension → the tutorial-3 sub-graph), tutorial-11
  (extension → `flow://flow-11` echo), and `helloworld2` (`GraphExecutionTest` MATH
  variant: fetcher → for-each extension over the `helloext` sub-graph → math → end,
  incl. the `$.result[*]` JSONPath output and the `x-hello` response header).
- **`graph.js` fixtures activated by skill swap** (maintainer direction): the graphs
  carrying the retired skill (`hello`, `helloworld`, `hellojs`, `tutorial-113`) now use
  `graph.math` — the statement grammar (IF/COMPUTE/EXECUTE/MAPPING/RESET/NEXT/DELAY)
  is identical, which is exactly why `graph.math` is the sanctioned replacement. One
  node (`helloworld` js-3) carried genuine JavaScript (`.filter()`, object literals,
  `.toFixed()`) and was adapted to math-grammar computes; the former JS variant now
  renders numbers as doubles (math semantics: 558.0 not 558). `rust-js-retired`
  remains the single `graph.js` case proving the retirement error. E2E: hello,
  helloworld, hellojs and the tutorial-113 retry pattern (error-handler +
  clear-exception + DELAY) all pass.
## Increment 27 — knowledge-graph K-7a: the WebSocket server (2026-07-17)

- **platform-core WebSocket server** (design K6a — the second lockstep layer-1
  extension): `automation/ws_server.rs` rides the REST automation server's HTTP
  upgrade path (hyper upgrade + tokio-tungstenite). Java `WsRequestHandler` protocol
  parity: a service listens at `/ws/{name}/{token}`; each connection becomes a private
  route pair `{session}.in` (the service function) / `{session}.out` (the
  transmitter), `session = ws.{random}.{seq}`; lifecycle events `open` (route,
  tx_path, ip, path, query, token) / `string` / `bytes` / `close` (code, reason;
  reply-to housekeeper releases the pair); transmitter semantics: string → text
  frame, bytes → binary frame, map/list → JSON text segmented above 62 KB,
  `type: close` with status/message closes the socket; idle sweep
  (`websocket.idle.timeout`, default 60s, min 10).
- **Declarative `#[websocket_service]` macro** (maintainer direction — full Java
  `@WebSocketService(value, namespace)` parity): the annotated struct registers
  through the link-time inventory like `#[preload]`; the AppStarter lifecycle loads
  the URL paths (with Java's `validServiceName` check — an invalid name logs an error
  and is skipped) before the HTTP server starts, and the server now starts when REST
  automation is enabled **or** any websocket service exists (Java
  `startHttpServerIfAny` semantics — the app also stays alive for WS-only services).
  Positional and named forms both work: `#[websocket_service("graph")]` /
  `#[websocket_service(name = "json", namespace = "ws")]`. The programmatic
  `register_ws_service(_with_namespace)` stays available for tests and dynamic cases.
- **E2E** (`tests/ws_server.rs`, a real tungstenite client through the real server):
  the 101 handshake, open greeting via the tx path, text/binary echoes, JSON-map
  framing, client- and server-initiated close (close event observed by the service),
  and the negative case (an unregistered `/ws/*` path does not upgrade). A second
  suite (`tests/ws_macro.rs`) proves the declarative path end-to-end: a
  `#[websocket_service]` struct served by an `AutoStart`-booted app with
  `rest.automation` disabled.
- **Declarative `#[fetch_feature]` macro** (maintainer direction — field
  installations use the declarative form for load-bearing cases such as fetching/
  refreshing an OAuth 2.0 access token and inserting the bearer token into the
  provider request): full Java `@FetchFeature(value)` parity via a new
  `knowledge-graph-macros` crate (the `#[simple_plugin]` pattern) — a link-time
  `FetchFeatureEntry` inventory loaded by the engine at startup (the
  `PlaygroundLoader` scan analog); explicit `features::register` remains for
  dynamic cases. E2E: the Java test feature `DemoAuth` declared with the macro, a
  provider carrying `feature: [demo-auth, ...]`, and the wire-echoed
  `Authorization: Bearer {node}` asserted through a real HTTP round trip.
- **Increment split recorded**: K-7 = K-7a (this, the layer-1 substrate) + K-7b (the
  Playground: `GraphUserInterface` sessions, the 1,494-line `GraphCommandService`
  grammar, `GraphTraveler`, companion API, the K-6-deferred REST endpoints,
  dev-gating K9) — next increment.

## Increment 28 — knowledge-graph K-7b: the Playground (2026-07-17)

- **The Playground command grammar** (`commands.rs` — the Rust port of the 1,494-line Java
  `GraphCommandService`): a per-session draft-graph workbench driven by a text grammar —
  `open`/`close`/`command`, word-alias normalization (`start`→instantiate, `clear`→delete),
  create/update/delete node, connect, list nodes/connections, describe graph/skill/node/
  connection (help served from the 39 ported `help/*.md` resources), edit, export/import
  graph/node, instantiate (mock-data grammar), execute a single node, run the traveler,
  inspect the state machine, `seen`, and session subscribe/unsubscribe/reset — with duplicate
  suppression, temp-dir housekeeping, and the `graph.command.singleton` handler for orderly
  AI-companion requests.
- **Sessions, traveler and websocket UI** (`session.rs`, `traveler.rs`, `ws_ui.rs`):
  `GraphSession` + registries, the `{route}.in`/`.out` ↔ public `ws-{id}` route↔id conversion,
  the dev-only `graph.traveler` walker (zero-tracing interceptor, idempotent per run), and the
  `GraphUserInterface` (`/ws/graph`) + `JsonPathHandler` (`/ws/json`) handlers (XML branch a
  documented deferral).
- **The AI-companion REST hop** (`rest.rs` — the field use case): `POST /api/companion/{id}`
  dispatches a text command to the session's singleton handler and the output streams to the
  session console; plus the K-6-deferred dev endpoints (home/workbench pages, mock/JSON
  uploads into a live session, draft-graph description, live-graph download, state-machine
  inspection).
- **Dev-gating (K9)** — `PlaygroundLoader` (`#[before_application(sequence = 8)]`) registers
  the command service, singleton, traveler, both websocket services and every dev REST
  endpoint **only when `app.env=dev`** (Java `@OptionalService("app.env=dev")` parity;
  `app.env` defaults to `dev`, matching the Java `application.properties`); the home page is
  registered regardless (it serves `/template` outside dev). Production graphs still run only
  through `POST /api/graph/{graph-id}`.
- **platform-core fix surfaced by integration (lockstep)**: booting the engine now registers
  websocket services, which made `AutoStart::main` block on `ctrl_c` (the serve-forever wait)
  — hanging every test that boots the engine and awaits `main`. Corrected the entry-point
  contract: the serve-until-Ctrl-C wait moved into `AutoStart::run` (the standalone `fn main()`
  path); `AutoStart::main` now **returns once the app is booted** (the accept loop runs as a
  background task), so an embedder gets control back. `start_http_server` gained a
  `server_address()` accessor (first-bind wins) so an ephemeral-port (`rest.server.port=0`)
  boot can recover its assigned port; it still binds a fresh listener per call (each
  `#[tokio::test]` keeps its own server). `graph_runtime` now reads `server_address()` instead
  of starting a second server.
- **E2E** (`tests/playground.rs`): a booted dev app drives the grammar end-to-end through the
  command service as the websocket UI would — help, describe skill, build (root/end/mapper +
  connections), list, instantiate + run, inspect — then the AI-companion REST hop
  (`POST /api/companion/{public_id}` → console) and the live-graph download
  (`GET /api/graph/session/{public_id}`), then close. Graph-executor fixtures reused: the
  mapper writes `input.body.id → output.body` (the graph's result namespace, what
  `execution_complete` returns); `list connections` renders Java's `source -[relation]-> target`.
- **Next**: K-8 — copy the React webapp verbatim, adjust `clean.js`/`deploy.js` to
  `../resources/public`, `npm run release`, live-verify in a browser, and close the layer-3
  milestone.

## Increment 29 — knowledge-graph K-8: React webapp + Playground app — LAYER 3 MILESTONE CLOSED (2026-07-18)

> **🎧 Active knowledge graph (layer 3) complete.** The MiniGraph Playground runs on the
> Rust engine: build a graph in the browser, traverse it (traversal *is* execution),
> inspect the state machine, drive it by AI-companion command. Three layers ported
> bottom-up — platform-core → event-script → active knowledge graph — foundation to UI.

- **The React webapp, copied verbatim** into `crates/knowledge-graph/webapp/` (React 19 +
  Vite + `@xyflow/react`, 573 modules). Per the maintainer's K7 decision only the deploy
  path changes: `scripts/clean.js` + `scripts/deploy.js` retarget
  `../src/main/resources/public` → **`../resources/public`**. A **third** path of the same
  class needed retargeting (maintainer-approved, not in the original K7 note): the in-app
  Help panel bundles the help markdown at build time via `import.meta.glob`, so
  `src/data/helpContent.ts` (+ the `vite.config.ts` dev-server comment) moved from the Java
  `../../../src/main/resources/help/*.md` to **`../../../resources/help/*.md`** — the engine
  crate's help dir — or the Help panel would render empty.
- **`npm run release`** (clean → `vite build` → deploy) lands the compiled bundle in the
  engine crate's `resources/public/`, served by REST automation as static content at `/`
  (the Rust analog of the Java jar's bundled resources; K8 resource-root hook). The served
  bundle (js/css/html, ~1 MB) is **committed** so a fresh clone serves the Playground with no
  npm step (Java parity); the 3+ MB of Vite **source maps are gitignored** as regenerable
  debug artifacts (`crates/knowledge-graph/resources/public/**/*.map` — map, don't mirror).
- **`examples/minigraph-playground`** — the runnable app (open question 4, default yes):
  a one-line `auto_start_main!` app that links the engine, with `resources/application.yml`
  (`app.env=dev`, `rest.automation`, port 8100) and `resources/rest.yaml` (the Playground/
  companion endpoints ported from the Java engine `rest.yaml`; the two demo-mock routes are
  omitted — those services are test fixtures). Mirrors the `hello-flow` example convention
  (the app ships its own rest.yaml/application.yml; the engine stays a clean library).
- **Live-verified against the running app** (`cargo run -p minigraph-playground`): the Chrome
  extension was unavailable, so verification exercised the exact protocol/paths the browser's
  React app uses — (1) static serving: `GET /` → `index.html` (title "Minigraph Playground")
  and `/assets/*.js` 200; (2) the websocket workbench: connect `ws://…/ws/graph/playground`
  → session greeting → `create node root` → `node root created` → `help connect` streamed the
  ported help content; (3) the AI-companion REST hop: `POST /api/companion/{id}` → 202
  accepted → the command output streamed to the session's WebSocket console.
- **Verification:** `cargo test --workspace` 181 green (K-8 adds no Rust tests — the webapp +
  runnable app are verified live), `cargo clippy --workspace --all-targets` 0 warnings,
  `cargo fmt --all --check` clean.

## Increment 30 — `#[optional_service]` macro + dev mock data providers (2026-07-18)

- **`#[optional_service("condition")]` — the Java `@OptionalService` annotation** (platform-core).
  A config-condition gate that registers a `#[preload]` route only when the condition holds at
  startup. Implemented as a stacked marker consumed by `#[preload]` (like `#[zero_tracing]` /
  `#[event_interceptor]`), plus an equivalent `optional_service = "…"` parameter; adds
  `PreloadEntry.optional_service`, and the AppStarter skips a gated route whose condition fails
  (logging `Skip optional {route}`). The condition evaluator is a faithful port of Java
  `Feature.isRequired` (`util/feature.rs`): comma-separated **OR**, `!key` negation,
  `key=value` / `key=` / `key` forms, all case-insensitive; **unset key never matches** (no
  implicit default). 7 unit tests over the condition forms.
- **Dev mock data providers** (`knowledge-graph/src/mock.rs`) — the Rust port of the Java
  `com.accenture.minigraph.mock` package: `MdmProfile` (`mock.mdm.profile`), `AccountDetails`
  (`mock.account.details`), `HelloTask` (`v1.hello.task`), each `#[preload]` +
  `#[optional_service("app.env=dev")]`, with the profile/account fixtures shipped in the engine
  crate's `resources/mock/`. The tutorials' data-dictionary / API-fetcher exercises call these
  over HTTP as stand-in enterprise services; the `minigraph-playground` example app
  (`app.env=dev`) wires their routes (`/api/mdm/profile`, `/api/account/details`).
- **Motivation:** the AI-companion validation of tutorial-3 surfaced that its fetcher needs the
  `mdm-profile` provider, which the Java engine ships (`@OptionalService` dev) but the Rust port
  only had as a test fixture. This closes that parity gap — a **Rust-only** increment (Java
  already has both the annotation and the mocks). `graph_runtime` is untouched: it runs with
  `app.env` unset, so the dev-gated engine mocks skip there and its own test mocks still register
  (no route collision).
- **Verification:** `cargo test --workspace` **188 green** (+7 feature unit tests),
  `cargo clippy --workspace --all-targets` 0 warnings, `cargo fmt --all --check` clean; the mock
  provider live-verified on a temp instance (`GET /api/mdm/profile/100` → Peter / 100 World Blvd;
  `/api/mdm/profile/10` → 400).

---

## Increment 31 — full declarative dev-gating for the Playground (2026-07-18)

- **`#[optional_service]` extended to the other three registration macros** (platform-core;
  commit `d582123`). Increment 30 only gated `#[preload]`; Java's `@OptionalService` also applies
  to `@WebSocketService`, `@BeforeApplication`, and `@MainApplication`. The marker (and the
  `optional_service = "…"` parameter) is now consumed by `#[websocket_service]`,
  `#[before_application]`, and `#[main_application]` too; `WsServiceEntry`/`BeforeAppEntry`/
  `MainAppEntry` each gained `optional_service`, and `AppStarter` skips a gated websocket
  service / entry-point whose condition fails (logging `Skip optional …`).
- **Playground registration is now declarative** (`knowledge-graph`; commit `448f125`). The former
  programmatic loader is retired: every Playground REST endpoint (`get.ws.html`,
  `post.companion.command`, `upload.json.content`, `upload.mock.content`, `show.graph.model`,
  `get.live.graph`, `inspect.state.machine`, `graph.command.service`/`.singleton`), the
  `graph.traveler` interceptor, and both websocket UIs (`GraphUserInterface` `/ws/graph`,
  `JsonPathHandler` `/ws/json`) now carry `#[preload]`/`#[websocket_service]` +
  `#[optional_service("app.env=dev")]`, plus a `#[before_application]` housekeeping hook. This
  mirrors Java, which registers these through `@PreLoad`/`@WebSocketService` + `@OptionalService`
  (the Java `PlaygroundLoader` is only the `@FetchFeature` loader, not the service registrar).
  `get.index.html` stays always-on (Java's non-optional `GetIndexHtml`).
- **`graph_runtime` now runs `app.env: dev`** and inherits the engine's dev-gated mocks; its local
  copies of `v1.hello.task` / `mock.mdm.profile` / `mock.account.details` are removed (they would
  otherwise collide with the now-registering engine mocks). This supersedes increment 30's
  "graph_runtime runs with app.env unset" arrangement.
- **`app.env` is env-overridable** in the example app: `${APP_ENV:dev}` (Java parity). Default is
  dev; `APP_ENV=prod` skips the entire Playground — closing loose-end #1 (conditional
  Playground load).
- **Verification:** `cargo test --workspace` **193 green**, `clippy --workspace --all-targets` 0,
  `fmt --all --check` clean. Live-verified both ways on a temp instance: **dev** loads all
  Playground routes + both websockets, "Playground loaded (app.env=dev)", `/api/mdm/profile/100`
  → Peter; **prod** (`APP_ENV=prod`) logs `Skip optional …` for every dev-gated service and both
  websockets, `/` static home + `http.flow.adapter` still serve, `/api/mdm/profile/100` → 404
  (also confirming loose-end #2: a rest.yaml entry cleanly 404s when its service isn't registered).

---

## Increment 32 — `inspect` docs: `{…}` is a placeholder, not literal (2026-07-18)

- **Documentation/UX fix, both repos** (Rust `0252c05`; Java canonical
  `Accenture/mercury-composable` `c04036f8`). Surfaced by the AI-companion validation of
  tutorial-3: the `inspect` grammar used `{…}` as a placeholder in the *syntax* line but repeated
  the braces in the *examples* (`inspect {output.body}`), so a literal-minded reader — or an AI
  agent — types the braces. Both engines then resolve `{output.body}` as the composite key
  `{output`→`body}` = empty `outcome`. **Not a code bug** — Java `handleInspectCommand` →
  `MultiLevelMap.getElement` splits on `.` without stripping braces, identical to Rust; the docs
  were the defect.
- **Fix (docs only):** examples unbraced (`inspect output.body`); braces kept only in syntax
  lines; a placeholder-convention note added to `command-reference.md`, `ai-agent-guide.md`
  (pre-send checklist), `help inspect.md`, and `minigraph-commands.json` (machine-readable
  `notes`).
- **Webapp autocomplete template — NOT changed (initial change reverted).** A first pass changed
  the `inspect` autocomplete `template` `inspect {variable_name}` → `inspect output.body`, but the
  maintainer correctly noted the webapp `template` field is a fill-in **template**, not an example
  — its `{…}` is the placeholder convention shared by every sibling (`execute node {name}`,
  `import node {node-name} from {graph-name}`, `instantiate graph … {constant} -> input.body.{key}`).
  So the template was reverted to `inspect {variable_name}` (Java `029a4912`, Rust revert commit).
  The bundle was still rebuilt — to carry the help-doc *example* fix embedded via
  `import.meta.glob('../../../resources/help/*.md')`.
- **Rust scope:** `crates/knowledge-graph/resources/help/help inspect.md` + rebuilt
  `resources/public` (help-example fix); `webapp/src/utils/commandSuggestions.ts` net-unchanged.
  `resources/help`/`resources/public` are served from disk (not compile-time embedded), so no Rust
  rebuild required; workspace unaffected.
- **Validation context:** tutorial-3 itself passed end-to-end — a fresh AI companion built the
  data-dictionary graph (7 nodes + 7 connections, exact structural match to canonical
  `tutorial-3.json`) from the canonical docs alone and the dry-run returned
  `output.body = {name:"Peter", address:"100 World Blvd"}`.

---

## Increment 33 — `serializer.null.transport` (Java null-omission parity) (2026-07-18)

- **platform-core serializer parity.** Java strips `null`s from **both** wire serializers by default,
  gated on one config `serializer.null.transport` (default `false`): Gson omits null object/map fields
  unless `serializeNulls()` (`SimpleMapper`), and `MsgPack.packMap` skips null map values
  (`if (supportNulls || value != null)`). The Rust port did the **opposite** — it always transported
  nulls via serde — so a successful `/api/companion/{id}/sync` response emitted `"error": null` where
  Java omits the field. Surfaced by a field test comparing the two engines' `/sync` output.
- **Rationale (maintainer).** A PoJo rarely initializes every field, so serializing it emits many
  `null` fields that are pure noise — omitting them keeps the payload clean (hence omission is the
  default). The `=true` case exists for applications that must distinguish "key present with a null
  value" from "key absent".
- **Invariants (must match Java exactly):** (1) affects **map key-values only**; (2) **array elements
  are always kept, including `Nil`** — dropping one would shift the rest and break array ordering
  (Gson / `packList` keep null slots too); (3) an **empty `[]` or `{}` is a real value, never a null**
  — only `Nil` is dropped.
- **Fix:** new `crates/platform-core/src/serializer.rs` — `null_transport()` (cached read, default
  false) + `strip_nulls`/`strip_nulls_always` (recursively drop `Nil` **map** entries; **array**
  elements preserved, matching Gson field-omission and Java `packList`). Applied at every wire
  boundary: the JSON HTTP response (`server.rs` `envelope_payload`), the WebSocket text frame
  (`ws_server.rs` — the companion tee), the outbound HTTP request body (`http_client.rs`), and the
  MsgPack envelope encoder (`envelope.rs::to_bytes`, guarded so a scalar body / transport-on path
  encodes with no extra clone).
- **Behavior:** default (`false`) now matches Java byte-for-byte on null handling — the `/sync`
  success response omits `error`; `serializer.null.transport: true` restores explicit null transport.
  Documented in `examples/minigraph-playground/resources/application.yml`.
- **Tests:** 5 unit tests in `serializer.rs` (map-drop, nested recursion, array-slot preservation,
  maps-in-arrays, scalar pass-through); `playground.rs` `/sync` HTTP round-trip now asserts `error` is
  omitted on success. Full workspace green (the only failure is the pre-existing two-`#[tokio::test]`-
  per-binary `graph_runtime` boot flake — reproduces on HEAD, unrelated). fmt + clippy clean.
- **Java side:** unchanged — it is the source of truth this mirrors.

---

## Increment 34 — `#[optional_service]` promoted to a first-class attribute (2026-07-19)

- **Maintainer direction:** `OptionalService` is a first-class citizen annotation — it makes a
  composable function, a websocket server function, a `#[before_application]` or a
  `#[main_application]` optional. It must not live only *inside* the `#[preload]` macro.
- **Before:** `#[optional_service("…")]` was an **inert marker** consumed by the four registration
  macros — it only compiled when written *below* them (attribute macros expand top-down), and
  `#[preload]` additionally accepted an `optional_service = "…"` parameter. The Java annotation
  order (`@OptionalService` on top) failed with *cannot find attribute*.
- **After:** `platform-macros` gains a real `#[proc_macro_attribute] optional_service`:
  written **above** a registration attribute it validates the condition, checks one of the four
  primaries is present (helpful compile error otherwise), and re-attaches the condition below,
  where the primary consumes it — so **both stacking orders work**. The redundant
  `optional_service = "…"` `#[preload]` parameter is **removed** (nothing used it; one canonical
  form, the Java way). Re-exported from `platform_core` alongside the other macros.
- **Tests:** `annotations.rs` — condition-above registers (`anno.gated.on`), condition-below still
  registers (`anno.gated.below`), unsatisfied condition skips (`anno.gated.off`), and a gated
  `#[before_application]` never runs (proven by the exact journal-sequence assertion). Workspace:
  **201 tests**, clippy 0, fmt clean.
- **Docs:** `docs/guides/event-driven/ai-agent-guide.md` — `optional_service` removed from the
  `#[preload]` parameter table; new first-class `#[optional_service]` subsection (all four kinds,
  either order, condition semantics). `platform-macros` crate docs updated the same way.

---

## Increment 35 — companion `/sync` ok-heuristic: whole-output classification (2026-07-19)

**Sweep finding #40; fixed in BOTH ports, `/sync` contract stays byte-identical.**

- **Before:** the per-line `is_error_line` heuristic classified import's benign
  "Graph model not found in /tmp/…" fallback line as a failure — `import graph from {deployed}`
  succeeded via the classpath fallback yet returned `ok:false`, misleading an AI caller into
  "fixing" a working command.
- **After:** classification runs over the whole captured output (`first_error_line` /
  `firstErrorLine`): the not-found line is forgiven **only** when the same output also carries
  the fallback's success marker ("Found deployed graph model"); a genuine miss prints the
  not-found line alone and stays `ok:false` (verified in both import handlers — a real miss
  emits nothing after it, so the rule can't mask real failures).
- **Tests:** Rust `companion_sync_import_fallback_reports_ok` (both directions) in
  `graph_runtime.rs`; Java `companionSyncImportFallbackReportsOk` in `CompanionSyncTest`
  (66-test module suite green). Both engines live-validated by the maintainer.
- **Upstream:** Java PR [#195](https://github.com/Accenture/mercury-composable/pull/195).
- **Docs:** `ai-agent-guide.md` caveat → fixed semantics; `minigraph-commands.json`
  sync_envelope note; rollup #40 → DONE in `docs/AI-companion-test.md`.

---

## Increment 36 — HTTP-boundary content-type dispatch: exact Java parity (2026-07-19)

**Maintainer-directed after a manual `/sync` probe; design D10 dispatch section updated.**

- **Before:** the Rust boundary was laxer than Java's `HttpRouter.handlePayload` — it sniffed
  JSON-looking bodies under any content type, text-decoded unknown/missing content types, and
  mapped an empty `application/json` body to null.
- **After:** `parse_body` mirrors Java exactly: `application/json` → bracket-guarded parse with
  raw-text fallback (empty → `{}`); `application/xml` → raw text (XML parse deferred, as on the
  client's response side); `application/x-www-form-urlencoded` (exact) → fields into
  `parameters.query`, body null (new path); `text/html`/`text/plain` → raw text; anything else
  incl. **missing content type → MsgPack binary** (Java `byte[]`; empty → null). Content-type
  matched on the `;charset`-stripped value, case-sensitively like Java.
- **Wire-verified fact:** the Java client sends **no default content-type** (raw-socket capture
  of a fetcher-style Map POST: header-less + chunked) — POST providers work in both engines
  because the canonical fixtures map `text(application/json) -> header.content-type`; the AI
  grammar's POST example already teaches this (finding #19), so nothing grammar-conformant
  relied on the sniffing.
- **Tests:** rewritten `body_parsing` unit test + end-to-end
  `body_dispatch_mirrors_java_content_type_rules` (`BodyProbe` reports the body *kind* reaching
  a function: map / text / bytes / null / query-merge) in `tests/rest_automation.rs`.

---

## Increment 37 — profile selection renamed `APP_PROFILES_ACTIVE` (2026-07-19)

**Maintainer decision closing the oldest backlog item (2026-07-15): a rename, not an alias.**

- **Before:** `SPRING_PROFILES_ACTIVE` / `spring.profiles.active` kept verbatim for
  side-by-side comparison with the Java original during migration (design §8 Q1 left the
  rename open, gated on "once the foundation port is robust").
- **After:** the gate is met — renamed outright to **`APP_PROFILES_ACTIVE`** (environment
  variable) / **`app.profiles.active`** (override registry + consolidated config key), no
  Spring alias: Spring is irrelevant to the Rust port. Precedence and the overlay mechanism
  (`application-{profile}.yml` merged on top) are unchanged. Divergence noted in the module
  doc and `app-config-reader.yml` (behavior-parity convention).
- **Tests:** `config.rs` profile-overlay tests updated to the new name (18 pass); workspace
  **202 tests**, clippy 0, fmt clean.
- **Follow-up (same day, maintainer-directed):** the flagged `spring.application.name`
  fallback in `Platform::name()` is retired too — `application.name` is Java's own primary
  key, so the Rust port reads it alone; the default aligns to Java's `"application"`
  (was an unnoted `"untitled"` divergence). All repo configs already used
  `application.name`, so nothing else moved.

---

## Increment 38 — `graph.math` `for_each`/`BEGIN`/`END` engine-verified spec (2026-07-19)

**Closes sweep finding #29 — the last thinly-specified corner of the statement grammar.**

- **Verification first:** Java `GraphMath.executeNode`/`executeForEach`/`splitBlocks` read
  side-by-side with the Rust port (line-for-line parity), then a probe fixture
  (`tests/resources/graph/rust-foreach.json`) + permanent test
  `math_for_each_blocks_and_iteration` (`graph_runtime.rs`) pinned every behavior empirically:
  pre/each/post blocks around `BEGIN`/`END` (no `BEGIN` ⇒ the whole list loops — including any
  accumulator seeding), strictly sequential in-order iteration inside **one** node execution
  (no loop-guard interaction), lockstep parallel arrays (equal lengths enforced), scalar
  `for_each` entries bind once at resolution, an unresolvable LHS **removes** the model key, a
  taken `IF` jump breaks the loop and skips the post-block (and routes traversal), empty lists
  run zero iterations but keep pre/post.
- **New dialect truth surfaced by the probe:** `COMPUTE` yields doubles while `f:add`/… simple
  plugins are whole-number-only (Java parity; *"Cannot convert the object to a whole number"*)
  — numeric accumulators therefore stay inside `COMPUTE` (read the model key back into the
  expression); `f:add` remains right for integer counters. The documented worked example (line
  totals with a running sum, `total: 500.0`) is executed verbatim by the probe.
- **Docs:** new [for_each section](guides/knowledge-graph/command-reference.md#math-for-each)
  in the command grammar; a structured `for_each` object on the `graph.math` entry in
  `minigraph-commands.json`; skills-reference paragraph; `help graph-math.md` "Iterating
  lists" section (webapp bundle re-released — 124 webapp tests green).
- **Tests:** workspace **202** (the probe runs inside the orchestrating
  `graph_runtime_end_to_end` test), clippy 0, fmt clean. Rollup #29 → DONE in
  `docs/AI-companion-test.md`.

---

## Increment 39 — numeric promotion for the simple-plugin arithmetic family (2026-07-19)

**Maintainer decision, prompted by increment 38's probe finding: the `f:` arithmetic plugins
were whole-number-only, so `f:add` could not consume a `COMPUTE` double (or any decimal API
value) — "it does not make sense to use a composable function to do simple calculation if a
simple-plugin can do the job."**

- **Rule (both ports, identical):** `promoteNumber`/`promote_number` now promotes whole numbers
  (and whole-number strings) to **long** and floating-point values (and decimal strings) to
  **double**. The result type is decided over **all** arguments before folding — order-independent:
  any floating argument promotes the whole computation to double; all-integral inputs keep exact
  64-bit arithmetic **including integer division**, so every previously-working call returns the
  identical result and only previously-erroring calls start working (strictly widening).
  Covers `add`/`subtract`/`multiply`/`div`/`mod` + `increment`/`decrement`; `gt`/`lt` compare
  exactly for whole pairs, as doubles otherwise. Divide-by-zero rejects `0` and `0.0`.
- **Rust:** `plugins_e8.rs` (`Number` enum + two-op `fold_numbers`); mixed-type cases added to
  `e8_plugins_match_java_semantics`; the `rust-foreach` probe gains the `f:add`-on-COMPUTE-doubles
  accumulator (`lsum: 500.0`) as the live regression — the exact composition that failed in
  increment 38. Workspace 202 tests / clippy 0 / fmt clean.
- **Java (upstream branch `feat/simple-plugin-number-promotion`):** `SimplePluginUtils`
  (`promoteNumber` → `Number`, new `reduceNumbers`), the seven arithmetic plugin classes,
  `GreaterThanOperator`/`LessThanOperator`; new `SimplePluginNumberPromotionTest`;
  event-script-engine 140 tests + playground engine 67 tests green.
- **Docs relaxed** (increment 38's "whole-number-only" caveats replaced by the promotion rule):
  `command-reference.md#math-for-each` (+ the `f:add` accumulator form now shown as equivalent),
  `minigraph-commands.json`, `skills-reference.md`, `help graph-math.md` (bundle re-released,
  124 webapp tests), event-script `syntax.md` plugin matrix + promotion note.
- **Documented boundary:** once a double enters, precision is IEEE-754 (integers exact to 2^53);
  the all-integral path stays exact 64-bit.
- **New `f:round(number[, decimal_places])` plugin (maintainer follow-up, same session):** the
  companion to the promotion — half-up rounding (ties away from zero) applied to the number's
  **shortest decimal representation** (Java `BigDecimal.valueOf`; the Rust port reproduces the
  same semantics on the shortest-repr string), so binary representation error never leaks into
  the rounding decision: `f:round(1.005, int(2))` → `1.01`, where a naive multiply-round-divide
  gives `1.0`. `decimal_places` optional (default 0, whole ≥ 0); whole-number inputs pass
  through unchanged. Registered in both engines (`RoundNumbers` + `plugin_round`), tested in
  both suites (Java 142, Rust mixed-type cases), documented in the syntax.md matrix, the KG
  grammar/catalog/help page.

---

## Increment 40 — join barrier counts only valid completions (2026-07-19)

**Backlog probe item #3 (Join + RESET interplay) — confirmed a LATENT BUG in both engines,
fixed identically in both ports.** The join barrier consults `skill_run` to decide whether an
upstream branch completed, but the mark meant "ran", not "completed": the traveler stamped it
even when the skill **failed** into its `exception=` route, and `RESET` cleared `node_seen` +
node state while leaving the stale completion mark. A fork whose failing branch retries could
fire the join prematurely off the stale mark — the assembled output **silently lost the
retrying branch's data** (empirically demonstrated: both engines' new probe test fails on the
old code with `expected: Peter, got: null`).

- **Fix (two complementary rules, identical in the traveler [dry-run] and the executor
  [deployed graphs] — the maintainer's parity requirement):**
  1. `skill_run` is marked **only when the skill did not fail** (no `{node}.status` +
     `{node}.error` pair) — closing the window between a failure and its handler's `RESET`;
  2. `reset_nodes` clears `skill_run` along with `node_seen` and node state — a deliberately
     reset branch stops satisfying the barrier until it re-executes successfully.
- **Probe:** `rust-join-retry.json` (Rust) / `unit-test-join-retry.json` (Java) — root forks
  into a paced branch B (100 ms) and a fetcher branch A that fails on the exception flag,
  pauses 300 ms, then recovers via `RESET` + retry; branch B reaches the join squarely inside
  the failed-but-not-reset window. Test `join_barrier_waits_for_a_retrying_branch`
  (`graph_runtime.rs`) / `joinBarrierWaitsForRetryingBranch` (`GraphTests`). Verified **red on
  old code, green on new** in both engines.
- **Java upstream:** branch `fix/join-barrier-retry-interplay` pushed (68-test module suite
  green); Rust workspace 202 tests / clippy 0 / fmt clean (manifest gate 28 graphs).
- **Docs:** `RESET` semantics (guard + completion mark + state) and the join's
  "success-only and current" completion rule across `command-reference.md`,
  `minigraph-commands.json`, `skills-reference.md`, `help graph-math.md`, `help graph-join.md`
  (webapp bundle re-released, 124 tests green).
- **Recorded observation → fixed same day as increment 41** (chained joins judged by
  recorded outcome).

---

## Increment 41 — chained join judges an upstream join by its recorded outcome (2026-07-19)

**The follow-on observation from increment 40, maintainer-directed — fixed in both ports.**
A join's own skill runs (and lands in `skill_run`) on every arriving branch, **including
evaluations that sink** — so a downstream join in a chained-join topology counted a sunk
upstream join as complete and fired prematurely, dropping the slow branch's data.

- **Fix:** `node_completed` / `nodeCompleted` judges a **join predecessor** by the outcome the
  join records in `node_seen` (`true` = fired) instead of the run mark. Regular skill nodes
  keep the success-only `skill_run` check (increment 40); skill-less nodes keep `node_seen`
  presence.
- **Probe:** `rust-join-chain.json` / `unit-test-join-chain.json` — `slow-pre` (200 ms) →
  `slow-x` and `fast-y` feed `j-one`; `j-one` chains into `j-two` alongside `pace-z` (100 ms).
  `fast-y` makes `j-one` evaluate-and-sink at ~1 ms; `pace-z` reaches `j-two` at ~100 ms.
  **Red on old code in both engines** (`expected: X, got: null`), green on new. The probe
  design itself surfaced a documentation-worthy subtlety: `DELAY:` pauses *inside* the math
  skill but its `MAPPING` writes state *before* the pause — so pacing a genuinely incomplete
  branch requires the delay and the write on separate nodes.
- **Gates:** Rust workspace 202 tests / clippy 0 / fmt clean (manifest gate 29 graphs); Java
  module suite **69 tests** green, branch **`fix/chained-join-outcome`** pushed (stacked on
  `fix/join-barrier-retry-interplay` — rebase onto main after PR #197 merges, per the #191
  auto-close lesson).
- **Docs:** the "multi-stage joins compose safely" clause added to `skills-reference.md`,
  `help graph-join.md`, and the JSON catalog's join entry (webapp bundle re-released).

---

## Increment 42 — discovery commands: `list graphs` / `list flows` (2026-07-20)

**Closes sweep finding #38 (tut-11) — the read-only discovery surface that makes
`extension=` delegation self-service, in both ports.**

- **Commands:** the `list` command grows two forms. `list graphs` enumerates the deployable
  graph models — the compiled registry **united** with the deployed location's `*.json` files
  (Rust: every resource root; Java: exploded classpath directories, with the compiled registry
  covering packaged-jar models) — each with its root node's `purpose`, so the listing reads as
  **living documentation of enterprise knowledge**. `list flows` enumerates the Event Script
  flows for `extension=flow://{flow-id}`. Both are read-only and available on the WS console
  and both companion endpoints.
- **Rust:** `list_graphs`/`list_flows`/`deployed_dirs`/`graph_purpose` in `commands.rs`;
  assertions in `tests/playground.rs`. Workspace 202 tests / clippy 0 / fmt clean.
- **Java:** mirrored in `GraphCommandService` (+ `CompiledGraphs`/`Flows` registries); tested
  via `/sync` in `CompanionSyncTest` (the agent-facing path); module suite **70 tests** green;
  branch **`feat/discovery-commands`** pushed for the upstream PR.
- **Docs:** grammar `#describe` section + discovery paragraph, JSON catalog entries, the AI
  agent guide's recipe step 1 (discover before delegating), `skills-reference.md#extension`
  pointer, `help list.md` (webapp bundle re-released, 124 tests green). Rollup #38 → DONE.
- **Browser-test refinements (maintainer, same session):** the `Flow` model now **retains the
  mandatory `flow.description`** (validated at compile, previously discarded) and `list flows`
  shows it; the graph compiler **enforces the discovery contract** — a manifest graph whose
  root node lacks a non-empty `purpose` is rejected (`rust-no-purpose` fixture proves it;
  `unit-test-1` gained a purpose in both repos); `help.md` overview + `help list.md` updated
  in both engines. Java suites: event-script **142**, playground **70**.

---

## Increment 43 — human docs site, phase 1 (2026-07-20)

**First increment of the human-documentation design (`docs/design/human-docs.md` D-H1/D-H2;
realizes `ot-human-guides-backlog`).** The maintainer approved the toolchain by providing the
`uv` environment; remaining design questions ride on later phases.

- **Scaffold:** `mkdocs.yml` (Material, pinned via `docs-requirements.txt`) reusing the
  agent-memory recipe — tabs/sections/indexes navigation, def_list + admonitions + mermaid
  (superfences) + tabbed content + snippets, strict link validation; `docs/` is the docs tree
  with internal material excluded (`design/`, `INCREMENTS.md`, `AI-companion-test.md`) and the
  machine artifacts (`llms.txt`, the two JSON catalogs) kept out of nav; the engine-verified
  **AI docs surface under Reference as-is**; ADR ledger under Background.
- **Pages:** `docs/index.md` (the three layers with a mermaid overview, why-Rust, port-truth
  admonition convention) + `docs/getting-started.md` (build/test, all three example apps with
  **source-verified** endpoints and code — hello-world 8085 `/api/greeting/{user}` +
  `#[preload]` snippets from the real `main.rs`, hello-flow 8086, Playground 8100 with the
  new `list graphs`; configuration in one paragraph incl. the increment-37 renames).
- **CI:** `.github/workflows/docs.yml` — build-only `mkdocs build --strict` on docs changes
  (Pages deployment deferred until graduation); `site/` gitignored.
- Local build: `mkdocs build --strict` green on the venv (`uv venv /tmp/mkdocs-venv`).

---

## Increment 44 — human docs site, phase 2 (2026-07-20)

**Seven pages, all source-verified; the nav adopts the Java site's layer organization** —
each layer section carries its human pages AND its AI agent guide side by side (the
maintainer's "the repo is AI-enabled" navigation statement).

- **Foundations:** `guides/architecture.md` (actor lineage → three layers, one mermaid
  pipeline), `guides/methodology.md` (decoupling, zero-code default + escape hatches,
  human+AI co-authoring via `/sync`), `guides/observability.md` — the telemetry record and
  context-block log line are **verbatim from a live hello-world run** (matching
  trace/span ids prove the join-up).
- **Layer 1:** `guides/event-driven/{index, write-your-first-function, function-execution}.md`
  — typed/untyped authoring, `#[preload]` parameter reference as definition lists, worker
  pools/elastic queue/back-pressure, send/request/send_later/interceptors, per-call tracing.
- **Layer 2:** `guides/event-script/index.md` — orchestration-as-configuration + the real
  hello-flow walkthrough.
- **Divergence honesty (17 `!!! note "Rust port"` boxes):** no broadcast/multicast, no
  fork-n-join RPC, no execution strategies (one async model), no `round_trip` telemetry
  metric, task-local trace propagation (no per-request PostOffice rule), OTLP forwarder
  extension not ported, MsgPack only on the spill path, plus two engine truths the Java
  docs never state (`send_later` doesn't stamp the ambient trace; an interceptor failure
  still auto-routes to `reply_to`).
- Nav restructured: Foundations / Layer 1 / Layer 2 / Layer 3 (AI docs fill Layer 3 until
  phase 3). `mkdocs build --strict` green.

---

## Increment 45 — human docs site, phase 3 (2026-07-20)

**Twelve pages by three parallel writers, reviewed and integrated; the site now covers all
three layers plus a Reference tab.** `mkdocs build --strict` green across 19 nav pages.

- **Layer 3 human pages:** `knowledge-graph/{index, build-your-first-graph,
  playground-and-companion, composing-the-layers}.md` — the property-graph concepts folded
  into the overview; the Playground/companion page documents `/sync`, the read-only session
  rule, the live tee and discovery; **`composing-the-layers` closes sweep finding #9's
  dangling link** with the full delegation story (sub-graph, `flow://`, `graph.task`) and a
  mermaid composition diagram.
- **Layer 1/2 additions:** `rest-automation.md` (rest.yaml grammar entry-per-heading, the
  exact increment-36 content-type dispatch, deferral boxes derived from source) and
  `flow-schema-reference.md` (all 13 task fields the compiler actually parses).
- **The D-H2 showcase:** `configuration-reference.md` (31 keys **enumerated from source
  greps**, grouped by area, entry-per-heading — the direct fix for the Java site's
  overflowing table), `macros-reference.md` (8 macros + 2 stacked markers),
  `event-envelope-reference.md` (27 methods), `api-overview.md` (Platform 9 + PostOffice 10 +
  AppError), `reserved-names-and-headers.md` (routes/headers/node names, dev-gating noted).
- **Java-doc bugs found by source verification (upstream doc-fix candidates):** the Java
  flow-schema reference documents **`error.status`, but the engine key is `error.code`** in
  both implementations (a null mapping if followed); `error.stack` resolves null in this
  port; the Java timeout doc understates accepted units; the Java claim that missing
  content-length streams the body is not how either boundary path behaves here.
- Review fixes: the stale "Java upstream pending" discovery notes corrected to cite the
  merged [#199](https://github.com/Accenture/mercury-composable/pull/199).

---

## Increment 46 — human docs site, phase 4: COMPLETE (2026-07-20)

**The human-documentation backlog (`ot-human-guides-backlog`) is closed — the documentation
gate for the repo's graduation to `github.com/Accenture/mercury` is done.**

- **`background/port-scope.md`** — the public-facing scope statement: map-don't-mirror, fully
  ported / deliberately out (Kafka mesh, Spring, `graph.js`) / deferred-not-never, the
  upstream-contribution relationship (features that originated here and merged into the Java
  engine), the fidelity mechanisms, and the performance posture.
- **Polish:** Home's "Where to go next" now fans out to Architecture, the three layer
  overviews, port-scope; Getting Started's next steps point at the human walkthroughs first.
- **Final pass:** site-wide sweep clean (no TODOs, no stale repo URLs, no phantom paths);
  `mkdocs build --strict` green across **20 nav pages** (Home, Getting Started, 3
  Foundations, 5+5+7 layer pages, 6 references, 2 background).
- The site totals ~4,600 lines of new human documentation across phases 1–4, every code
  sample and configuration key verified against this repository's source, with ~40
  `!!! note "Rust port"` divergence boxes — no silent divergence anywhere.

---

## Increment 47 — `describe graph {graph-id}`: the deployed-model contract view (2026-07-20)

**Closes discovery-drive findings #53 and #54 — self-service delegation is now complete.**

- **#53:** the new read-only **`describe graph {graph-id}`** shows a deployed model's
  **contract view**: purpose, node/connection counts, and the `input.*` / `output.*` data
  surface derived by scanning the model's own node properties (mapping entries, plugin args,
  statement substitutions). An agent wires `extension=` delegation `input[]`/`output[]` from
  it — no out-of-band brief, no trial execution. Plain `describe graph` still describes the
  draft; the `list graphs` footer advertises the new form.
- **#54:** tutorial-3 and tutorial-5 carried identical purposes, defeating purpose-based
  discovery — now differentiated in BOTH repos (single fetch-by-id vs parallel fan-out
  composition); the tutorial-3 help transcript synced.
- **Rust:** `describe_deployed_graph`/`deployed_model_json`/`model_data_surface`/
  `collect_path_tokens` in `commands.rs`; `playground.rs` assertions (contract view +
  not-found). Workspace 202 tests / clippy 0 / fmt clean; webapp bundle re-released.
- **Java:** mirrored in `GraphCommandService` (+ `deployedModel` shared with `graphPurpose`);
  `/sync` coverage in `CompanionSyncTest`; module suite **70 tests** green; branch
  **`feat/describe-deployed-graph`** pushed for the upstream PR.
- **Docs:** grammar `#describe` + discovery paragraph, catalog entry, `help describe.md` +
  `help list.md`, the AI agent guide's recipe (discover → **contract** → delegate). Rollups
  #53/#54 → DONE.

---

## Increment 48 — outbound HTTPS for the async HTTP client (2026-07-20)

**Maintainer requirement: systems of record in the field require HTTPS** for outbound calls
by `async.http.request` — directly or via MiniGraph's API Fetcher (which rides the same
route and already passed `https://` Provider URLs through). Java has supported this from the
start (Reactor-Netty `secure()` + `InsecureTrustManagerFactory` escape hatch), so this is
**Rust-only parity work** that closes the module's documented `https` deferral.

- **TLS stack:** [rustls](https://github.com/rustls/rustls) via `tokio-rustls` (`ring`
  provider — no native cmake/asm toolchain) + `rustls-native-certs`. Strict mode verifies
  against the **OS certificate store** (the JDK-default-truststore analog — corporate CAs
  honored); client configs are built once per mode and cached.
- **`trust_all_cert` parity** on `AsyncHttpRequest`: parsed from the map, fluent
  `set_trust_all_cert`, emitted alongside `host` in `to_value` (Java `toMap` shape). The
  trust-all verifier skips chain validation only — handshake signatures still verify —
  mirroring Java's `InsecureTrustManagerFactory` semantics for self-signed endpoints.
- **`validate_url`** accepts `https` (default port 443); the host header omits the scheme's
  default port (80/443). TLS failures surface **in-band** (500 `TLS handshake failed …`),
  like every other client error.
- **Tests** (`tests/http_client_tls.rs`, no external network): a local rustls server with an
  `rcgen` self-signed cert proves both modes — `trust_all_cert` → 200 + decoded JSON body;
  strict → in-band certificate rejection; plus `to_value` round-trip and protocol-rejection
  cases. Workspace **206 tests** / clippy 0 / fmt clean.
- **Docs:** `actuators-and-http-client.md` target-host section rewritten (HTTPS + the
  trust-all caveat); module doc deferral note replaced. The REST-automation *server-side*
  HTTP(S) relay (`rest.yaml` URL services) remains deferred — unrelated to this client-side
  support.

---

## Deferred backlog (as of increment 10)

See `docs/design/platform-core-port.md` §7 for the authoritative list: broadcast delivery,
streams, kernel-thread analog, flow binding + HTTP relay + A/B +
upload + streaming (REST), event-over-HTTP, OTLP forwarder extension, `/info/lib` +
`/info/routes`, `yaml.preload.override`, etag/cache, the
`Utility` grab-bag, crypto/caches, a dedicated lightweight RPC inbox.

**Next layer:** event-script (layer 2) — the YAML flow DSL, unlocking REST automation's
`flow:` binding and the composable-application programming model.
