# Migration Increments — Historical Record

> The chronological ledger of the mercury port: **mercury-composable (Java, canonical
> v4.8.6) → Rust**, delivered bottom-up in verified increments. Each increment traces to
> the Blueprint (`bp-platform-core` → `vision-mercury` in `memory/`); the *design
> rationale* lives in [`docs/design/platform-core-port.md`](design/platform-core-port.md)
> (§5a–§5f, D1–D10); the full working narrative lives in `memory/sessions/`.
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

Every increment ships with `cargo build` + `cargo test` + `cargo clippy --all-targets` +
`cargo fmt --check` clean, and (from increment 4 on) a live run of the `hello_world`
example demonstrating the new capability end-to-end.

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

---

## Deferred backlog (as of increment 7)

See `docs/design/platform-core-port.md` §7 for the authoritative list: broadcast delivery,
streams, kernel-thread analog, `#[preload]` macro, flow binding + HTTP relay + A/B +
upload + streaming (REST), event-over-HTTP, OTLP forwarder extension, `/info/lib` +
`/info/routes`, OS-signal shutdown wiring, `yaml.preload.override`, etag/cache, the
`Utility` grab-bag, crypto/caches, a dedicated lightweight RPC inbox.

**Next layer:** event-script (layer 2) — the YAML flow DSL, unlocking REST automation's
`flow:` binding and the composable-application programming model.
