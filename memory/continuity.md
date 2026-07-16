# Continuity — mercury

> Shared ground truth for project state across all agents and sessions.
> Update at the end of every session. Never delete — only archive (see `REVIEW.md`).
>
> Each fact carries a metadata footer in an HTML comment, maintained by the review
> ritual — invisible when rendered, read/written by agents:
> `<!-- id: kebab-id | created: YYYY-MM-DD | last_used: YYYY-MM-DD | uses: N | tier: active -->`
> See `.agent/schema.md` for the fields and `memory/decay-policy.md` for the windows.

---

## Project State

- **project:** mercury
- **status:** **Rust port of `mercury-composable`** (Accenture's event-driven composable app platform; canonical impl in Java v4.8.6), carrying the same vision. In scope: three layers — platform-core → event-script → active knowledge graph (bottom-up, foundation → UI); **Kafka service mesh and Spring out of scope**. Private prototyping repo (pushed to `acn-ericlaw/mercury`); graduates to the official Accenture repo once the foundation is sufficient. **platform-core increments 1–5 implemented, bootable, and OBSERVABLE**: config management → event-bus foundation → FIFO reactive back-pressure (ElasticQueue + manager-worker; BDB ignored) → application lifecycle (AppStarter/EntryPoint, Platform identity, housekeeping) → **OpenTelemetry-compatible distributed tracing + business correlation-id + app-log-context** (Telemetry `distributed.tracing` service, task-local trace bracket, W3C ids, automatic propagation, JSON logger with context block; `hello_world` demos logs↔spans join live) — 66 tests, clippy/fmt clean. Next: increment 6+ per `docs/design/platform-core-port.md` §7 (REST automation is the natural next: it starts traces at the HTTP edge via the ported w3c_trace).
- **last_enabled:** 2026-07-15
- **last_session:** 2026-07-16 | agent: Claude Code (2026-07-16-001704)
- **last_review:** (none yet)
- **last_invariant_check:** (none yet)
- **repo:** ~/sandbox/mercury
- **vision:** `memory/vision.md` (north star, set at enable — Blueprint gaps to be derived)

## Stack & Tools

> Canonical live home for the current stack — language version, dependencies, tool
> versions. `instructions.md` keeps only a high-level descriptor and points here.

**Rust edition 2021**, toolchain 1.95.0 (latest stable at increments 1–2). Cargo **workspace**
(`Cargo.toml` root, members `crates/*`); `crates/platform-core` is the first crate.
**Deps in use:** serde 1, serde_json 1, serde_yaml 0.9 (⚠ archived upstream — works fine;
swap for a maintained fork only if it ever blocks), thiserror 1, log 0.4, tokio 1
(rt-multi-thread/sync/time/macros), async-trait 0.1, async-channel 2 (per-route MPMC
queue), rmp-serde 1 + rmpv 1 (with-serde), uuid 1 (v4). Stack rationale:
`platform-core-stack` + design doc D1–D6. `.gitignore` is stack-aware (Rust section:
`target/`, `**/*.rs.bk`, `*.pdb`; Cargo.lock tracked).

**Canonical source:** `mercury-composable` (Java, `com.accenture.mercury:parent-mercury`
**v4.8.6**, Java 21, Maven reactor) at `~/sandbox/mercury-composable` (added by the maintainer
2026-07-15, read-only reference). Its `docs/guides/` (architecture, event-envelope-reference,
api-overview, event-script, knowledge-graph) is the authoritative behavior spec — map, don't
mirror. Key Java deps to find Rust equivalents for: Vert.x event bus + Java 21 virtual threads
(→ async runtime), MsgPack (→ rmp-serde), Gson/JSON (→ serde_json), classgraph annotation
scanning (→ compile-time registration; no runtime scanning in Rust). platform-core alone is
~24.5K LOC / 121 files — a multi-increment port.

## Architectural Invariants

> Hard constraints that must never change. These never decay (treated as `core`).

- **Never couple functions directly** — inter-function coupling stays **route-name +
  `EventEnvelope`** only; no direct calls between user functions. This is the defining
  invariant inherited from mercury-composable (the actor-model decoupling); the whole
  three-layer design rests on it. Preserve it in the Rust port.
  <!-- id: inv-never-couple-functions | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: core | origin: 2026-07-15-221632.md -->

*(More invariants will be distilled from mercury-composable's docs/ADRs as each layer is
ported — e.g. stateless functions, HTTP-style status codes.)*

## Key Decisions

- **AI-enabled at the greenfield stage** — the shared memory layer + Vision were installed
  *before* any code, so development is guided by shared memory and intent-traceability
  (VBDI) from the first commit rather than retrofitted later.
  <!-- id: ai-enabled-greenfield | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->
- **Port bottom-up, faithfully to the Java original** — re-implement mercury-composable in
  Rust layer by layer, foundation → UI (platform-core, then event-script, then active
  knowledge graph), preserving the Java project's behavior. The Java repo is the canonical
  spec (map, don't mirror).
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-07-15 | uses: 3 | tier: active | origin: 2026-07-15-215538.md -->
- **Kafka service mesh is out of scope** — deliberately not ported, for simplicity.
  <!-- id: kafka-mesh-out-of-scope | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: active | origin: 2026-07-15-215538.md -->
- **Config management first; Spring fully out of scope** (maintainer, 2026-07-15): port
  `AppConfigReader`/`ConfigReader`/`MultiLevelMap` + the `resources/` folder convention as
  **increment 1** — everything (main app, unit tests, integration tests) relies on
  configuration management. Spring (`rest-spring-3/-4`) is **not ported** (Java-only);
  platform-core's own Vert.x-based REST automation remains in scope later. Config-file syntax
  kept verbatim (D9) so config files port unchanged between Java and Rust.
  <!-- id: config-first-spring-out | created: 2026-07-15 | last_used: 2026-07-15 | uses: 4 | tier: active | origin: 2026-07-15-222816.md -->
- **Back-pressure = the file-based elastic FIFO; Berkeley DB store NOT ported** (maintainer,
  2026-07-15): the manager-worker dispatch (ServiceQueue ready-signal state machine) rides an
  `ElasticQueue` — first 20 events in memory, overflow to segmented append-only files
  (`[4-byte BE length][payload]`, byte-identical to Java; sealed → consumed → deleted
  immediately, no compaction). Java's `bdb` legacy store is ignored, so the `ElasticStore`
  strategy facade collapses into one type. Config keys verbatim:
  `elastic.queue.segment.size.bytes`, `elastic.queue.dispatch.mailbox.size`,
  `transient.data.store`, `running.in.cloud`. Design: `docs/design/platform-core-port.md` §5b.
  <!-- id: elastic-fifo-no-bdb | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: active | origin: 2026-07-15-232127.md -->
- **Private prototyping repo → official Accenture repo later** — iterate rapidly here to keep
  noise away from public readers; move to the official Accenture repo once the foundation is
  sufficient.
  <!-- id: private-repo-then-accenture | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->
- **platform-core Rust stack (confirmed at the gate, 2026-07-15):** **tokio** async runtime
  (virtual-thread analog; N `instances` = N worker tasks/route), **async_trait**, **serde +
  rmp-serde** (MsgPack bus) + **serde_json** (HTTP edge), **idiomatic serde wire format** (NOT
  byte-compatible with Java — cross-JVM interop is out of scope), Cargo **workspace**
  (`crates/platform-core` first), **explicit registration** now (compile-time `inventory`-style
  macro later; Rust has no runtime annotation scanning). Full rationale + type designs:
  `docs/design/platform-core-port.md`.
  <!-- id: platform-core-stack | created: 2026-07-15 | last_used: 2026-07-15 | uses: 6 | tier: active | origin: 2026-07-15-222242.md -->

## Conventions

> Established with the first code (increment 1, 2026-07-15); enforced from the first commit.

- **`cargo fmt` + `cargo clippy --all-targets` clean** is part of "done" for every change
  (default settings, no custom rustfmt.toml yet).
- **Apache-2.0 header** comment on every source file (ported from the Java originals'
  header style).
- Each ported module's `//!` doc names the **Java class it ports** (e.g.
  `org.platformlambda.core.util.ConfigReader`) so reviewers can diff behavior side-by-side.
- **Tests:** unit tests in-module (`#[cfg(test)]`), integration tests in `tests/` with
  fixtures under `tests/resources/` (mirrors Java's `src/test/resources`).
- **Behavior-parity notes** in doc comments wherever the Rust port deliberately mirrors a
  Java quirk (e.g. YAML-tab tolerance) or deliberately diverges — no silent divergence.
- Config-file syntax verbatim (D9): `classpath:/`, `file:/`, `${ENV:default}`, dotted routes.
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-07-15 | uses: 4 | tier: active | origin: 2026-07-15-224707.md -->

## Open Threads

> Mark completed items `- [x]` and leave them in place — the review sweeps them to
> the archive once older than `archive_window` sessions. Don't archive them by hand.

- [x] **(vision-bootstrap) Vision set by the maintainer at enable — blueprint derived.**
  Greenfield inverts the bootstrap: the maintainer set the Vision directly (Rust port of
  mercury-composable). `memory/vision.md` is confirmed (no DRAFT), and the high-level
  Blueprint below was derived from the stated plan. VBDI drift-detection is now active
  (advisory until the Designs land).
  <!-- id: ot-vision-confirm-blueprint | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->

- [x] **Greenfield — no code yet.** Resolved 2026-07-15: increment 1 landed the first real
  code (Cargo workspace + `crates/platform-core` config management). Stack recorded in
  `## Stack & Tools`, conventions seeded in `## Conventions` (`conventions-rust-baseline`),
  first invariant recorded at harvest (`inv-never-couple-functions`).
  <!-- id: ot-greenfield-no-code | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: active | origin: 2026-07-15-215538.md -->

### Blueprint — gaps from Current State (greenfield) to the Vision  (serves: vision-mercury)
> Derived 2026-07-15 from the maintainer-set Vision. Each `(blueprint)` thread is a
> Vision↔reality gap that closes when delivered. Bottom-up order (foundation → UI). Detailed
> per-layer Designs are TODO — the authoritative behavior spec is the Java mercury-composable
> project (map, don't mirror); harvest it into per-layer Designs when a local checkout is
> available and authorized (see the harvest thread below).

- [ ] **(blueprint)** Port **platform-core** to Rust — the foundation layer; everything else
  builds on it. First real porting increment. → serves: vision-mercury
  <!-- id: bp-platform-core | created: 2026-07-15 | last_used: 2026-07-15 | uses: 8 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(blueprint)** Port **event-script** to Rust (on top of platform-core).
  → serves: vision-mercury
  <!-- id: bp-event-script | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(blueprint)** Port the **active knowledge graph** to Rust. → serves: vision-mercury
  <!-- id: bp-active-knowledge-graph | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(blueprint)** Continue **foundation → user interface** once the three layers stand.
  → serves: vision-mercury
  <!-- id: bp-foundation-to-ui | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(blueprint)** **Graduate to the official Accenture repo** once the foundation is
  sufficient (this private repo is the prototyping stage). → serves: vision-mercury
  <!-- id: bp-graduate-to-accenture | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->

- [ ] **(knowledge-harvest) Harvest the canonical vision/specs from mercury-composable (Java).**
  **Gate satisfied 2026-07-15** — the maintainer added `~/sandbox/mercury-composable` and
  authorized reading it (read-only reference). **Harvested this session:** the north-star
  vision (AKG-is-the-application / AI-assisted Semantic Application Development), the accurate
  three-layer model, platform-core's architecture (functions/route-name/`EventEnvelope`/
  `PostOffice`/`Platform`/in-memory bus, virtual-thread execution, lifecycle), the module map,
  and the canonical version (4.8.6) — folded into vision/instructions/invariants above.
  **Still to harvest** (as each layer is ported): platform-core internals (EventEmitter,
  WorkerHandler, serializers), then event-script and knowledge-graph specs + their ADRs.
  → serves: vision-mercury
  <!-- id: ot-harvest-mercury-composable | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: working | origin: 2026-07-15-215538.md -->

- [ ] **(design) Rust port of platform-core — increments 1+2 DONE; later increments per §7.**
  Realizes `bp-platform-core`. Design doc: **`docs/design/platform-core-port.md`** (D1–D9).
  **Increment 1 (configuration management) implemented 2026-07-15**: `MultiLevelMap`/
  `ConfigValue`, `ConfigReader` (`${}` substitution + loop detection), `AppConfigReader`
  (manifest, merge order, profile overlays), `resources` roots, `overrides` registry.
  **Increment 2 (event-bus foundation) implemented 2026-07-15**: `EventEnvelope` (idiomatic
  serde MsgPack wire format, fluent builders, dynamic `rmpv::Value` body),
  `ComposableFunction` (untyped currency) + `TypedFunction`/`TypedAdapter` (typed authoring),
  `AppError`, `Platform` (route validation per `Utility.validServiceName`, per-route
  `async-channel` MPMC + N 1-based worker tasks → point-to-point to one free instance,
  release-by-channel-close), `PostOffice` (`send` 400/404 errors; `request` RPC via temporary
  `inbox.<uuid>` route + oneshot + cid correlation, timeout → 408, inbox always released;
  worker stamps `from`/`exec_time`, converts `Err(AppError)` → error envelope). **41 tests
  total (13 unit + 18 config + 10 event-bus incl. concurrency-bounding + config-driven worker
  count), clippy + fmt clean.** **Remaining (§7, each its own increment):** broadcast, streams,
  kernel-thread analog (`spawn_blocking`), lifecycle (AutoStart/AppStarter), `#[preload]`
  macro, REST automation (no Spring), event-over-HTTP, tracing, full envelope fields,
  Utility grab-bag. **Increment 3 (FIFO reactive back-pressure) implemented 2026-07-15**
  (maintainer hint; see `elastic-fifo-no-bdb`): `ElasticQueue` (two-tier memory/disk FIFO,
  segment sealing + immediate reclamation, generations, startup purge) + manager-worker
  dispatch (per-route manager task with ready-worker FIFO + buffering state machine; workers
  pull via ready signals, one in-flight event each; bounded manager mailbox → senders await =
  back-pressure). Divergences doc-noted: serialize-only-on-spill; RPC inboxes ride the route
  machinery (Java AsyncInbox is lighter — later refinement); elastic housekeeping
  (keep-alive/expired-scan/shutdown) deferred to lifecycle. **48 tests, clippy/fmt clean.**
  **Increment 4 (application lifecycle) implemented 2026-07-15** (§5c): `AppStarter`/`EntryPoint`
  builder with the exact Java phase order (essential-services → before-application by sequence
  [failure aborts] → preload → rest.automation-notice → mains by sequence [missing = error]);
  `Platform::get_instance()`/`name()`/`origin()` identity; elastic housekeeping completed
  (keep-alive, expired-store scan, `shutdown_cleanup`); **runnable `hello_world` example**
  (config `${GREETING_USER:world}` → preflight hook → `greeting.demo` RPC) — verified live, both
  default and env-overridden runs. **55 tests, clippy/fmt clean.**
  **Increment 5 (OTel tracing + business cid + app-log-context) implemented 2026-07-16**
  (maintainer directive: telemetry is foundation, before REST automation; §5d): `Telemetry`
  service at `distributed.tracing` (essential-services phase; logs span datasets real-time;
  `distributed.trace.forwarder` + `transaction.journal.recorder` hooks), per-execution
  **task-local trace bracket** (the Java per-worker-anchor design — deliberately not
  ThreadLocal/MDC; W3C 32/16-hex ids; envelope carries `span_id` for parent lineage),
  automatic trace/cid propagation in PostOffice (+ `my_correlation_id`, `annotate_trace` →
  span, `update_context` → log, two distinct sinks), `w3c_trace` traceparent port,
  opt-in `app-log-context.yaml` + JSON logger (`log.format=json`; context block joins logs to
  spans). hello_world demos the join live (same trace/span ids on log line + span). One real
  bug fixed: OnceLock re-entrancy deadlock (config logging inside its own initializer) →
  eager init in `logging::init()`. **66 tests, clippy/fmt clean.** → serves: vision-mercury
  <!-- id: ot-design-platform-core | created: 2026-07-15 | last_used: 2026-07-16 | uses: 8 | tier: working | origin: 2026-07-15-221632.md -->

- [ ] **(backlog) Generic `app.profiles.active` alias for profile selection.** Maintainer
  decision 2026-07-15: keep `SPRING_PROFILES_ACTIVE`/`spring.profiles.active` **verbatim**
  during migration for side-by-side comparison with the Java original; add a generic alias
  once the foundation port is robust. Don't build until then. → serves: vision-mercury
  <!-- id: ot-profiles-alias-backlog | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-224707.md -->

## User Preferences

(none recorded yet — record ONLY what the user explicitly states; never infer)

## Team / Members

(none recorded yet)
