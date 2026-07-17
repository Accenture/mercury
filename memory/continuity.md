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
- **status:** **Rust port of `mercury-composable`** (Accenture's event-driven composable app platform; canonical impl in Java v4.8.6), carrying the same vision. In scope: three layers — platform-core → event-script → active knowledge graph (bottom-up, foundation → UI); **Kafka service mesh and Spring out of scope**. Private prototyping repo (pushed to `acn-ericlaw/mercury`); graduates to the official Accenture repo once the foundation is sufficient. **platform-core increments 1–8 implemented — an HTTP-SERVING, OBSERVABLE, OPERABLE foundation**: config management → event-bus foundation → FIFO reactive back-pressure (ElasticQueue + manager-worker; BDB ignored) → application lifecycle → OTel tracing + business cid + app-log-context (3-format logger, -D overrides) → REST automation core (rest.yaml per the Java grammar, hyper HTTP edge that starts traces/ensures cid) → actuators + static content (`/info` `/env` `/health` `/livenessprobe`, default-endpoint merge, `resources/public`) → **static-content protocol** (SHA-256 etag/HTTP-304 with comma-aware If-None-Match; no-cache pages default `/`+`/index.html`; `static-content.filter` request interceptor — the SSO-redirection hook: 200=serve, else pass-through, headers always copied) → **increment 9: lightweight RPC inbox (AsyncInbox parity) + benchmark-reporter** — **PLATFORM-CORE MILESTONE CLOSED 2026-07-16**: 103 tests, clippy/fmt clean, and a saved benchmark record (`benchmark/benchmark-reporter/analysis/rust-tokio.html`): baseline RPC 155K ops/s @ 6µs mean (8.4× the Java file-vthread record), balanced 411K ops/s (2.3×), overload ~1.4× loss-free through the disk spill, mixed latency probe 17µs mean / 210µs max vs Java 157µs/1.62ms (~9× — the no-GC tail story); 1,003,000 timed ops, 0 failures. **Increment 10 (2026-07-16): annotation macros** — `crates/platform-macros` (`#[preload]`/`#[before_application]`/`#[main_application]`/stacked `#[zero_tracing]`, link-time `inventory` registration = the Java classpath-scan analog, D6 closed) + `AutoStart` one-liner (`auto_start_main!()`; ships OS-signal shutdown) + the **`examples/<name>/` app-crate convention** (hello-world relocated; 105 tests, workspace-wide clippy 0). `docs/INCREMENTS.md` = the increment ledger. **Layer 2 (event-script) started 2026-07-16**: canonical `event-script-engine` surveyed (~8K LOC; grammar spec = `flow-grammar.md`); `docs/design/event-script-port.md` DRAFT v1 (decisions E1–E9, increment plan E-1…E-9) **approved 2026-07-16**; **E-1 shipped** (repo increment 11): `crates/event-script` — flow model + full `CompileFlows` port, all 90 Java fixtures reused verbatim with the loaded-set/rejection/task-drop outcomes pinned by tests; **E-2 shipped** (repo increment 12): data-mapping engine — runtime MultiLevelMap over `rmpv::Value` (maintainer refinement: direct composite-key access = PRIMARY tool; JSONPath `$.…` = user-defined complex queries via serde_json_path), DataMappingHelper resolution + 19 core plugin bodies; greetings-fixture mappings evaluate byte-for-byte like Java (139 workspace tests, clippy 0). **E-3 shipped** (repo increment 13): platform-core extensions — event-interceptor mode (`FunctionOptions`; success reply guarded, failures still route — Java WorkerHandler parity), `send_later`/`cancel_future_event`, rest.yaml `flow:` → x-flow-id, deep-copy satisfied by `rmpv::Value::clone` (145 tests, clippy 0). **E-4 shipped** (repo increment 14): core flow runtime — FlowInstance + registries, manager/executor interceptors, FlowExecutor launch/request; sequential/response/end/decision/sink + exceptions + TTL abort + metrics + flow-summary span; E2E over the canonical fixtures (146 tests, clippy 0). **FLOWS EXECUTE ON RUST.** **Increment 15**: hidden direct execution for the reserved engine routes (event.script.manager/task.executor run on the event core — private route list in Platform::deliver, NOT exposed via macros/API by maintainer decision; concurrency + liveness, proof tests incl. a serialized control; 148 tests). **E-5 shipped** (repo increment 16): parallel + fork/join with the pipe-map barrier, dynamic `source` iteration (.ITEM/.INDEX), Java-exact pipe cleanup on exceptions; canonical parallel/fork fixtures verbatim + a marked Rust-side dynamic-fork supplement (canonical needs E-7). **E-6 shipped** (repo increment 17): pipelines (PipelineState in the pipe map) with for/while loops + break/continue; canonical loop fixtures verbatim incl. file() round-trip and per-step delay; decision.case upgraded to the faithful DecisionCase port. **E-7 shipped** (repo increment 18): flow:// sub-flows via the manager (child response = parent callback), shared parent state (Arc<Mutex<tree>> per family; materialized at model.parent under the shared lock; model.root normalized), ext: state machine (route + flow:// forms), SimpleExceptionHandler built-in; canonical dynamic-fork fixture ACTIVATED (5 concurrent sub-flows, exactly-once shared appends). **E-8 shipped** (repo increment 19): all 42 plugin bodies + `#[simple_plugin]` macro (new event-script-macros crate; SimplePluginLoader at sequence 3); plugin fixtures activated; user plugin proven E2E. **E-9 shipped (repo increment 20) — EVENT-SCRIPT (LAYER 2) MILESTONE CLOSED 2026-07-17**: HttpToFlow, Resilience4Flow, EventScriptMock, examples/hello-flow (live-verified: YAML flow over HTTP, cid propagation). E-1…E-9 = the complete engine, validated on the canonical Java fixtures. **Layer 3 (knowledge graph) started 2026-07-17**: canonical minigraph-playground-engine surveyed (~9.8K LOC + React webapp; MiniGraph itself is a Java platform-core built-in); `docs/design/knowledge-graph-port.md` DRAFT v1 (K1–K9, increments K-1…K-8) **awaiting the maintainer gate** — then K-1 (MiniGraph in platform-core).
- **last_enabled:** 2026-07-15
- **last_session:** 2026-07-17 | agent: Claude Code (2026-07-17-013151)
- **last_review:** 2026-07-16 | through 2026-07-16-194946
- **last_invariant_check:** (none yet)
- **repo:** ~/sandbox/mercury
- **vision:** `memory/vision.md` (north star, set at enable — Blueprint gaps to be derived)

## Stack & Tools

> Canonical live home for the current stack — language version, dependencies, tool
> versions. `instructions.md` keeps only a high-level descriptor and points here.

**Rust edition 2021**, toolchain 1.95.0 (latest stable at increments 1–2). Cargo **workspace**
(`Cargo.toml` root, members `crates/*`); `crates/platform-core` is the first crate.
**Deps in use:** serde 1, serde_json 1, serde_yaml 0.9 (⚠ archived upstream — works fine;
swap for a maintained fork only if it ever blocks), thiserror 1, log 0.4 (std feature),
tokio 1 (rt-multi-thread/sync/time/macros/net/signal/io-util), async-trait 0.1,
async-channel 2 (per-route MPMC queue), rmp-serde 1 + rmpv 1 (with-serde), uuid 1 (v4),
**hyper 1 (http1/server) + hyper-util 0.1 + http-body-util 0.1** (D10 — REST automation;
deliberately not a web framework: rest.yaml IS the router). Stack rationale:
`platform-core-stack` + design doc D1–D10. `.gitignore` is stack-aware (Rust section:
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

- **Port bottom-up, faithfully to the Java original** — re-implement mercury-composable in
  Rust layer by layer, foundation → UI (platform-core, then event-script, then active
  knowledge graph), preserving the Java project's behavior. The Java repo is the canonical
  spec (map, don't mirror).
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-07-16 | uses: 4 | tier: archive-candidate | origin: 2026-07-15-215538.md -->
- **Config management first; Spring fully out of scope** (maintainer, 2026-07-15): port
  `AppConfigReader`/`ConfigReader`/`MultiLevelMap` + the `resources/` folder convention as
  **increment 1** — everything (main app, unit tests, integration tests) relies on
  configuration management. Spring (`rest-spring-3/-4`) is **not ported** (Java-only);
  platform-core's own Vert.x-based REST automation remains in scope later. Config-file syntax
  kept verbatim (D9) so config files port unchanged between Java and Rust.
  <!-- id: config-first-spring-out | created: 2026-07-15 | last_used: 2026-07-16 | uses: 8 | tier: archive-candidate | origin: 2026-07-15-222816.md -->
- **Back-pressure = the file-based elastic FIFO; Berkeley DB store NOT ported** (maintainer,
  2026-07-15): the manager-worker dispatch (ServiceQueue ready-signal state machine) rides an
  `ElasticQueue` — first 20 events in memory, overflow to segmented append-only files
  (`[4-byte BE length][payload]`, byte-identical to Java; sealed → consumed → deleted
  immediately, no compaction). Java's `bdb` legacy store is ignored, so the `ElasticStore`
  strategy facade collapses into one type. Config keys verbatim:
  `elastic.queue.segment.size.bytes`, `elastic.queue.dispatch.mailbox.size`,
  `transient.data.store`, `running.in.cloud`. Design: `docs/design/platform-core-port.md` §5b.
  <!-- id: elastic-fifo-no-bdb | created: 2026-07-15 | last_used: 2026-07-16 | uses: 4 | tier: archive-candidate | origin: 2026-07-15-232127.md -->
- **platform-core Rust stack (confirmed at the gate, 2026-07-15):** **tokio** async runtime
  (virtual-thread analog; N `instances` = N worker tasks/route), **async_trait**, **serde +
  rmp-serde** (MsgPack bus) + **serde_json** (HTTP edge), **idiomatic serde wire format** (NOT
  byte-compatible with Java — cross-JVM interop is out of scope), Cargo **workspace**
  (`crates/platform-core` first), **explicit registration** now (compile-time `inventory`-style
  macro later; Rust has no runtime annotation scanning). Full rationale + type designs:
  `docs/design/platform-core-port.md`.
  <!-- id: platform-core-stack | created: 2026-07-15 | last_used: 2026-07-16 | uses: 11 | tier: archive-candidate | origin: 2026-07-15-222242.md -->

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
- **`docs/INCREMENTS.md` is the historical ledger** (maintainer-requested, 2026-07-16):
  one overview row + one section per increment, added as part of each increment's
  definition of done (design rationale stays in `docs/design/platform-core-port.md`;
  the ledger records what shipped when).
- **Example apps are standalone `examples/<name>/` workspace crates** (increment 10,
  2026-07-16): annotated functions + `platform_core::auto_start_main!();` with the app's
  `resources/` beside its `Cargo.toml` — never cargo examples inside a library crate.
  Event-script and knowledge-graph demos land as sibling `examples/<name>/` crates.
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-07-17 | uses: 21 | tier: active | origin: 2026-07-15-224707.md -->

## Open Threads

> Mark completed items `- [x]` and leave them in place — the review sweeps them to
> the archive once older than `archive_window` sessions. Don't archive them by hand.

### Blueprint — gaps from Current State (greenfield) to the Vision  (serves: vision-mercury)
> Derived 2026-07-15 from the maintainer-set Vision. Each `(blueprint)` thread is a
> Vision↔reality gap that closes when delivered. Bottom-up order (foundation → UI). Detailed
> per-layer Designs are TODO — the authoritative behavior spec is the Java mercury-composable
> project (map, don't mirror); harvest it into per-layer Designs when a local checkout is
> available and authorized (see the harvest thread below).

- [x] **(blueprint)** Port **platform-core** to Rust — the foundation layer; everything else
  builds on it. **MILESTONE CLOSED 2026-07-16** (increments 1–9; benchmarked vs the Java
  original — see `docs/INCREMENTS.md` and `benchmark/benchmark-reporter/analysis/`). Remaining
  §7 items (broadcast, streams, #[preload] macro, flow binding, relay, event-over-HTTP, …) are
  enhancements to fold in as event-script needs them, not blockers. → serves: vision-mercury
  <!-- id: bp-platform-core | created: 2026-07-15 | last_used: 2026-07-16 | uses: 16 | tier: active | origin: 2026-07-15-215538.md -->
- [x] **(blueprint)** Port **event-script** to Rust (on top of platform-core).
  **MILESTONE CLOSED 2026-07-17** (increments E-1…E-9 / repo 11–20; design
  `docs/design/event-script-port.md` §5a–§5i): the complete Event Script engine — compiler,
  mapping engine, all 8 execution types, sub-flows + shared parent state, external state
  machine, 42 plugins + #[simple_plugin], HTTP flow adapter, resilience, mock,
  examples/hello-flow — validated against the canonical Java fixture suite (66 flows).
  Remaining (traced, non-blocking): HTTP-client fixtures await platform-core §7 http client;
  stream payloads; mock monitors. → serves: vision-mercury
  <!-- id: bp-event-script | created: 2026-07-15 | last_used: 2026-07-17 | uses: 14 | tier: active | origin: 2026-07-15-215538.md -->
- [ ] **(blueprint)** Port the **active knowledge graph** to Rust. → serves: vision-mercury
  <!-- id: bp-active-knowledge-graph | created: 2026-07-15 | last_used: 2026-07-17 | uses: 3 | tier: working | origin: 2026-07-15-215538.md -->
  **Design drafted 2026-07-17** (`docs/design/knowledge-graph-port.md` v1) — gate pending.
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
  eager init in `logging::init()`. **66 tests, clippy/fmt clean.**
  **Increment 6 (REST automation core) implemented 2026-07-16** (§5e): `rest.yaml` per the
  Java project's own agent-ready grammar doc (`rest-grammar.md`) — function binding, methods,
  `{param}`/trailing-`*` URLs (exact > param > wildcard), timeout clamp 1s–5m, CORS blocks,
  header add/drop/keep transforms, simple-route authentication, per-entry
  trace/cid header impedance overrides; **hyper 1** HTTP edge (D10 — no web framework;
  rest.yaml IS the router) wired into AppStarter phase 4 (`rest.automation=true`,
  `rest.server.port`); the edge **always ensures a business cid** (exposed via reserved
  `my_correlation_id` header) and **starts traces** (`tracing: true`: valid W3C traceparent
  wins + its parent-id becomes our parent span → else trace-id header → else generated;
  legacy conflation = one id); AsyncHttpRequest-shaped event (Java keys); envelope→HTTP
  response mapping (status, JSON/text/binary by body type, transforms + CORS headers,
  Java `{status,message,type:error}` errors, 408 on timeout). Deferred: flow binding, relay,
  A/B, upload, static-content, actuator default-rest merge. **Verified live**: hello_world
  serves `GET /api/greeting/{user}` — upstream traceparent → greeting.api → greeting.demo,
  one trace id, correct parent-span lineage at every hop, cid `order-9000` end-to-end;
  404/CORS-preflight/generated-cid all curl-checked. Two test-infra bugs fixed: tokio-test
  shared-server runtime death (server-per-test now) + timeout-clamp expectation.
  **83 tests, clippy/fmt clean.**
  **Increment 7 (actuators + static content) implemented 2026-07-16** (maintainer-directed;
  §5f): `src/actuator.rs` — one impl, four registrations by `ActuatorKind` (vs Java's
  `my_route` header switch), essential-phase registration with shared `ActuatorContext`
  (liveness follows last health outcome). `/info` (app identity/origin/uptime; JVM blocks
  omitted honestly), `/env` (opt-in `show.env.variables`/`show.application.properties` —
  never a wholesale dump), `/health` (mandatory/optional dependency routes, type=info +
  type=health protocol, DOWN=400 Java parity), `/livenessprobe`. Default-endpoint merge
  (Java default-rest.yaml: user rest.yaml entries always win) + **static HTML from
  `resources/public`** (`/`→index.html, traversal-guarded, mime by extension; rest.yaml `/`
  wins). Deferred: `/info/lib` (no runtime dep manifest in Rust — build.rs metadata later),
  `/info/routes`, etag, XML. All five endpoints curl-verified live on hello_world (health
  UP with demo.health mandatory dep, static index at `/`). **94 tests, clippy/fmt clean.**
  **Increment 8 (static-content protocol) implemented 2026-07-16** (maintainer-directed;
  §5g; ref: Java test/resources/rest.yaml static-content block): **etag/HTTP-304** (quoted
  SHA-256 via new dep `sha2`; comma-aware If-None-Match; 304 + content-length 0),
  **no-cache pages** (default `/`+`/index.html`: Cache-Control no-cache,no-store + Pragma +
  epoch Expires — entry pages always revalidate, the SSO case), **request filter**
  (`static-content.filter` path/exclusion/service; exact/`prefix*`/`*suffix` patterns;
  filter's response headers ALWAYS copied; 200 = continue serving, else pass-through e.g.
  302+Location; 10s timeout; unregistered service → warn+serve). Path resolution tightened
  to Java getStaticFile (extensionless → `.html`). Demo: `http.request.filter` interceptor
  logging url/ip/user-agent. Live-verified: no-cache+x-filter on `/`, real 304 cycle.
  Ops lesson: `kill` on a `cargo run` wrapper orphans the child binary — a stale server on
  8085 masked the new build until pkill'd (demo scripts now kill the binary by name).
  **101 tests, clippy/fmt clean.**
  **Increment 9 (lightweight RPC inbox + benchmark-reporter) implemented 2026-07-16 —
  PLATFORM-CORE MILESTONE CLOSED** (§5h): RPC replies now ride a one-shot correlation-map
  inbox (Java AsyncInbox parity; `src/inbox.rs`; reply bypasses ServiceQueue; pulled forward
  from §7 so the benchmark measures dispatch, not per-request route registration — all prior
  tests unchanged). `benchmark/benchmark-reporter` (new workspace member): the Java harness
  ported — same 6 scenarios (normal/overload/mixed-isolation), same Stats (nearest-rank,
  log-spaced bins), same self-contained HTML report; `-Dbench.*` params, Java defaults;
  doc'd divergence: paced scenarios ride tokio's ~1ms timer (compare latency there, not
  throughput). **Saved record** `analysis/rust-tokio.html` + comparison README vs the Java
  file-vthread record (same machine class): 1→50 155K ops/s @ 6µs (8.4×); 50→50 411K
  (2.3×); overload ~1.4× loss-free; probe 17µs/210µs vs 157µs/1.62ms (~9×). 1,003,000 ops,
  0 failures. **103 tests, clippy/fmt clean.**
  **Increment 10 (annotation macros + AutoStart one-liner + examples/ convention)
  implemented 2026-07-16** (§5i; maintainer-directed before layer 2): `crates/platform-macros`
  proc-macro crate — `#[preload(route, instances, env_instances, typed)]`,
  `#[before_application(sequence)]`, `#[main_application]`, stacked `#[zero_tracing]` (the
  Java @PreLoad/@BeforeApplication/@MainApplication/@ZeroTracing analogs); registration is
  **link-time** via `inventory` 0.3 (`registry::*Entry` + `inventory::submit!` — collects
  across crates, so layer-2/3 library functions register like app-local ones; **closes D6**).
  `AutoStart::main` (Java parity): -D overrides → logging → collect inventories
  (env_instances via config, override-aware) → lifecycle → Ctrl-C park under
  rest.automation → shutdown_cleanup (ships the deferred OS-signal wiring);
  `auto_start_main!()` = the whole user `fn main()` incl. the invoking crate's resources/.
  hello-world moved to the standalone `examples/hello-world/` app crate (annotated one-liner
  main; deps: platform-core+serde+async-trait+log only — no tokio). Test gotcha honored:
  ONE end-to-end annotations test (global platform's workers die with the first test
  runtime). Whole-workspace clippy swept to 0 (five pre-existing warnings fixed).
  **105 tests, clippy/fmt clean**; live-verified (REST/etag-304/filter/actuators unchanged).
  → serves: vision-mercury
  <!-- id: ot-design-platform-core | created: 2026-07-15 | last_used: 2026-07-16 | uses: 14 | tier: working | origin: 2026-07-15-221632.md -->

- [ ] **Design + port event-script (layer 2)** — the working thread realizing
  `bp-event-script`. **Design DRAFT v1 written 2026-07-16** (`docs/design/event-script-port.md`;
  survey of `system/event-script-engine` ~8K LOC, 90 tests, 51+20 fixtures). Decisions E1–E9:
  new `crates/event-script`; flow YAML verbatim + **Java fixtures reused unchanged**; flow state
  on `serde_json::Value` + `serde_json_path` (Java uses Jayway JsonPath); engine self-registers
  via the increment-10 annotations (cross-crate inventory); four platform-core extensions in
  lockstep (event-interceptor mode — Java engine classes are @EventInterceptor; `send_later`/cancel
  for the TTL watcher; rest.yaml `flow:` → x-flow-id injection; body deep-copy);
  `#[simple_plugin]` compiled registry (bytecode allowlist not ported — divergence doc'd);
  abortable-timer TTL; EventScriptMock later; streams deferred. Increments E-1…E-9: compiler →
  mapping engine → platform-core extensions → core runtime → parallel/fork-join → pipelines/loops →
  sub-flows/ext → plugins → HTTP adapter + resilience + `examples/hello-flow` (milestone).
  **Gate APPROVED 2026-07-16** ("assumptions are fine" — defaults accepted on all 4 questions).
  **E-1 (flow model + compiler) SHIPPED 2026-07-16** (repo increment 11; ES design §5a):
  `crates/event-script` — model/flows/compiler/converter/validator/plugins modules; Java failure
  semantics exact (invalid mapping drops the TASK, flow loads; end-task check precedes mapping
  validation); plugin NAME registry pulled forward from E-8 (42 names — compile-time `f:` checks
  need them; execution stays E-8); all 90 Java fixtures verbatim (attributed; local edits
  forbidden), tests pin the exact 65-flow loaded set + rejections + task-drops + normalized
  mapping strings. Finding: fixture comments are NOT spec — parser-test-7 (legacy
  'invalid-condition-mode' id) and parser-test-19 ('ext.user' dot form) legitimately LOAD per the
  Java code; documented in design §5a. 120 workspace tests / clippy 0 / fmt clean.
  **E-2 (data-mapping engine) SHIPPED 2026-07-16** (repo increment 12; ES design §5b): maintainer
  refinement folded in — **MultiLevelMap direct composite-key access is the PRIMARY mapping tool,
  JSONPath (`$.…`) is the user-defined complex query** (verified: Java layers it identically —
  only `$` paths reach Jayway). E3 value tree refined serde_json→**rmpv::Value** (bus currency:
  zero state↔envelope conversion + byte fidelity — serde_json can't hold real bytes; dated
  amendment in the design decision table). Shipped: runtime `mlm` (Java get/set/remove semantics,
  `[]` append, Jayway-style query results), `mapping` (constants/file()/classpath()/f: invocation
  with nested-null guard/legacy :type commands/{model.key} interpolation), `conversions`
  (String.valueOf display parity; str2int/long → −1 with decimal-drop, verified in Java Utility),
  19 core plugin bodies (converter-emitted set; rest fail loudly until E-8). Parity capstone: the
  compiled greetings fixture's mappings evaluate to Java's exact function-input body. 139
  workspace tests / clippy 0 / fmt clean.
  **E-3 (platform-core extensions) SHIPPED 2026-07-16** (repo increment 13; ES design §5c):
  event-interceptor registration mode (`FunctionOptions{zero_traced, interceptor}` replaces the
  increment-10 bool; worker ignores an interceptor's successful return — manual replies via
  po.send — but failures STILL route to reply_to, verified against Java WorkerHandler's guard
  placement); `#[preload]` `interceptor` flag + stacked `#[event_interceptor]`;
  `PostOffice::send_later/cancel_future_event` (abortable tokio timer, E7); rest.yaml `flow:` →
  x-flow-id injection (closes the increment-6 flow-binding deferral); deep-copy = satisfied by
  design (rmpv clone is deep). New tests: interceptor suite ×5 (incl. the no-auto-reply 408
  proof), flow-binding echo, annotations marker. 145 workspace tests / clippy 0; hello-world
  live-checked after the registration-signature change.
  **E-4 (core flow runtime) SHIPPED 2026-07-16** (repo increment 14; ES design §5d): FlowInstance
  (state machine + TTL watcher on send_later) + instance registry; manager + executor as
  `#[event_interceptor]` preloads (1 instance each — Java parity, callbacks serialize);
  FlowExecutor launch/request. Execution types sequential/response/end/decision/sink; exception
  routing (task > flow handler, loop guard); TTL abort 408; @retry decisions; deferred tasks;
  file() output targets; `*` wildcard body; my_correlation_id stamped last. KEY TRANSLATION:
  consolidated mapping view built IN the instance dataset (scratch keys stripped per callback) —
  Java's shared-reference persistence with zero model copies; model.parent/root aliases deferred
  to E-7 (need a shared-subtree design). Later constructs (parallel/fork E-5, pipeline E-6,
  flow://+ext: E-7) abort with explicit messages. E2E over canonical fixtures incl. the dynamic
  reserved-key runtime rejection (FlowTests parity). 146 workspace tests / clippy 0.
  **Increment 15 (2026-07-16): direct execution for reserved engine routes** — maintainer design
  review of Java `sendWithEventBus`: manager/executor run directly on the event core. Rust
  rationale differs from Java (bus is zero-copy, so no serialization saving): the wins are
  CONCURRENCY (E-4's 1-instance registration had serialized all orchestration — caught by this
  review) and LIVENESS (self-feeding router + bounded mailboxes = circular-wait risk). **Hidden by
  maintainer decision**: private RESERVED_ENGINE_ROUTES in Platform::deliver + worker reply path;
  no macro flag, no FunctionOptions field — "an engine privilege, not an API". Tracing parity:
  direct path never trace-brackets (Java bypasses WorkerHandler too; E-4 note corrected). Proofs:
  peak concurrency >1 on reserved routes with 1 instance vs peak==1 on a normal control route;
  20 concurrent E2E flows. 148 workspace tests / clippy 0.
  **E-5 (parallel + fork/join) SHIPPED 2026-07-16** (repo increment 16; ES design §5e):
  pipe_counter/pipe_map on FlowInstance (PipeInfo::Join barrier; E-6 extends the enum);
  parallel fan-out (no barrier — Java parity, convergence via model state); dynamic `source`
  replication with .ITEM/.INDEX pseudo-keys in input mapping; non-list source → 400; Java-exact
  exception pipe cleanup (own handler → remove seq; else clear all). Fixture strategy: canonical
  parallel-test + fork-n-join-test verbatim; **Rust-side supplement** flows-rust/dynamic-fork-test
  (documented in fixtures README) until the canonical dynamic fixture's flow://+ext: deps land in
  E-7 — compiler expected-set now 66. Concurrent [] appends proven under increment-15 direct
  execution. 148 workspace tests / clippy 0.
  **E-6 (pipelines + loops) SHIPPED 2026-07-16** (repo increment 17; ES design §5f):
  PipelineState (ptr/completed/clamped nextStep) in the pipe map; three-way seq dispatch
  (Join/Pipeline/fall-through); for (init/comparator/sequencer) + while (strict Boolean true) loops;
  break/continue after every step (continue clears its flag); singleton short-circuit. decision.case
  is now the faithful Java DecisionCase (loop workhorse). Fixtures verbatim: pipeline-test,
  for-loop-test (file() append/read/delete), for-loop-break, while-loop (delay + boolean(3=false)
  stop), pipeline-exception. 148 workspace tests / clippy 0.
  **E-7 (sub-flows + shared state + ext) SHIPPED 2026-07-16** (repo increment 18; ES design §5g):
  flow:// launches via the manager (dataset {ttl, body, header?}; parent/flow_id/business-cid
  headers; composite cid — child response returns as the parent task's callback, so
  mappings/exceptions/fork barriers unchanged); root-ancestor resolution; **shared parent state**:
  Arc<Mutex<Value>> per family, materialized at model.parent per mapping pass UNDER the shared
  lock (Java ancestor.modelSafety analog; lock order shared→dataset; holding it across the pass
  makes concurrent [] appends lossless), model.root.*→model.parent.* normalization; ext: calls
  collected in-pass, dispatched post-lock (route + flow:// forms); SimpleExceptionHandler ported;
  app.id: A12 test-config parity. Canonical fixtures activated: parent/daughter (alias
  round-trip), missing-sub-flow, externalize put/get (trace-scoped store), fork-n-join-flows,
  fork-n-join-with-dynamic-model-test (5 concurrent sub-flows, exactly-once). First-run pass.
  148 workspace tests / clippy 0.
  **E-8 (plugin catalog + #[simple_plugin]) SHIPPED 2026-07-17** (repo increment 19; ES design
  §5h): all 42 built-in bodies execute (arithmetic w/ promoteNumber, now/dateTime with a
  Java-pattern→chrono converter, gt/lt/ternary/string ops, parseDate(-Time), list-of-map
  normalization+merge, full validate rule engine — Java-exact messages); bodies in plugins_e8.rs
  (include!); chrono dep. `#[simple_plugin]` in NEW crates/event-script-macros (name defaults to
  camelCase fn name); SimplePluginLoader #[before_application(sequence=3)] — the Java slot, so
  user plugin names pass compile-time f: validation. Parity fixes: plugin errors propagate
  UNWRAPPED (input-validation-2's exact "user (ABC) < CCC"); type-conversion asserted on the rmpv
  tree (real bytes from f:binary). Fixtures: arithmetic, type-conversion, string-util, parse-date,
  input-validation-1/-2 + user `shout` plugin E2E. 148 workspace tests / clippy 0.
  **E-9 SHIPPED 2026-07-17 (repo increment 20) — LAYER-2 MILESTONE CLOSED** (ES design §5i):
  HttpToFlow (edge reply routing preserved; route timeout → ttl added to the platform-core
  request map; configurable cid header), Resilience4Flow (retry/abort/alternative + backoff,
  fixtures verbatim incl. temp-file store), EventScriptMock (dispatch-time override registry —
  divergence doc'd; monitors not ported), examples/hello-flow (live: Bonjour/Hello with cid
  propagation; linker gotcha caught — an app must REFERENCE event-script for its inventory to
  link; the main-app announces flows via get_all_flows(), usage = linkage). exception.simulator
  upgraded to the faithful Java port. 148 workspace tests / clippy 0. **Layer 2 done in one
  day (increments 11–20).** → serves: vision-mercury
  <!-- id: ot-design-event-script | created: 2026-07-16 | last_used: 2026-07-17 | uses: 11 | tier: working | origin: 2026-07-16-174644.md -->

- [ ] **Design + port the knowledge graph (layer 3)** — the working thread realizing
  `bp-active-knowledge-graph`. **Design DRAFT v1 written 2026-07-17**
  (`docs/design/knowledge-graph-port.md`; survey: minigraph-playground-engine ~9.8K LOC + React
  webapp; MiniGraph = a Java platform-core built-in, 827 lines). Decisions K1–K9: MiniGraph into
  Rust platform-core; new `crates/knowledge-graph` (deps: platform-core + event-script — layer 3
  rides layer 2, e.g. the graph-executor.yml flow exposes POST /api/graph/{id}); graph JSON +
  13 tutorial fixtures + help markdown verbatim; math expression engine ported faithfully;
  **graph.js RETIRED** (maintainer, 2026-07-17 — an arbitrary-code interpreter is a security
  risk; the name is not registered and fails with a message pointing to graph.math/graph.task);
  TWO platform-core
  lockstep extensions — **WebSocket server** (hyper upgrade + tokio-tungstenite; Java
  @WebSocketService) for the Playground and **HTTP client** (async.http.request — closes the §7
  deferral, also activates E-9's deferred http-client fixtures); **React webapp copied verbatim,
  only scripts/clean.js+deploy.js path changes ../src/main/resources/public → ../resources/public
  (maintainer-directed)**; engine appends its own resource root (jar-resources analog);
  app.env=dev gating. Increments K-1…K-8 (MiniGraph → math → compiler → runtime+core skills →
  http client+api.fetcher → extension+REST → WebSocket+Playground → webapp+milestone).
  **GATE APPROVED 2026-07-17** (defaults accepted; graph.js upgraded deferral → retirement).
  **K-1 SHIPPED 2026-07-17 (increment 21)**: `platform_core::graph` — MiniGraph + models,
  Java-exact semantics/messages, Arc-shared nodes with interior mutability, rmpv::Value
  properties, deterministic export/import (tutorial-JSON shape), full GraphTest parity
  (10 tests); workspace 158 green / clippy 0 / fmt clean. Next: K-2 math expression engine.
  → serves: vision-mercury
  <!-- id: ot-design-knowledge-graph | created: 2026-07-17 | last_used: 2026-07-17 | uses: 1 | tier: working | origin: 2026-07-17-010622.md -->

- [ ] **(backlog) Generic `app.profiles.active` alias for profile selection.** Maintainer
  decision 2026-07-15: keep `SPRING_PROFILES_ACTIVE`/`spring.profiles.active` **verbatim**
  during migration for side-by-side comparison with the Java original; add a generic alias
  once the foundation port is robust. Don't build until then. → serves: vision-mercury
  <!-- id: ot-profiles-alias-backlog | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-224707.md -->

## User Preferences

(none recorded yet — record ONLY what the user explicitly states; never infer)

## Team / Members

(none recorded yet)
