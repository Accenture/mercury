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

## Deferred backlog (as of increment 10)

See `docs/design/platform-core-port.md` §7 for the authoritative list: broadcast delivery,
streams, kernel-thread analog, flow binding + HTTP relay + A/B +
upload + streaming (REST), event-over-HTTP, OTLP forwarder extension, `/info/lib` +
`/info/routes`, `yaml.preload.override`, etag/cache, the
`Utility` grab-bag, crypto/caches, a dedicated lightweight RPC inbox.

**Next layer:** event-script (layer 2) — the YAML flow DSL, unlocking REST automation's
`flow:` binding and the composable-application programming model.
