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
- **status:** **Rust port of `mercury-composable`** (Accenture's event-driven composable app platform; canonical impl in Java v4.8.6), carrying the same vision. In scope: three layers — platform-core → event-script → active knowledge graph (bottom-up, foundation → UI); **Kafka service mesh and Spring out of scope**. Private prototyping repo (pushed to `acn-ericlaw/mercury`); graduates to the official Accenture repo once the foundation is sufficient. **platform-core increments 1–8 implemented — an HTTP-SERVING, OBSERVABLE, OPERABLE foundation**: config management → event-bus foundation → FIFO reactive back-pressure (ElasticQueue + manager-worker; BDB ignored) → application lifecycle → OTel tracing + business cid + app-log-context (3-format logger, -D overrides) → REST automation core (rest.yaml per the Java grammar, hyper HTTP edge that starts traces/ensures cid) → actuators + static content (`/info` `/env` `/health` `/livenessprobe`, default-endpoint merge, `resources/public`) → **static-content protocol** (SHA-256 etag/HTTP-304 with comma-aware If-None-Match; no-cache pages default `/`+`/index.html`; `static-content.filter` request interceptor — the SSO-redirection hook: 200=serve, else pass-through, headers always copied) → **increment 9: lightweight RPC inbox (AsyncInbox parity) + benchmark-reporter** — **PLATFORM-CORE MILESTONE CLOSED 2026-07-16**: 103 tests, clippy/fmt clean, and a saved benchmark record (`benchmark/benchmark-reporter/analysis/rust-tokio.html`): baseline RPC 155K ops/s @ 6µs mean (8.4× the Java file-vthread record), balanced 411K ops/s (2.3×), overload ~1.4× loss-free through the disk spill, mixed latency probe 17µs mean / 210µs max vs Java 157µs/1.62ms (~9× — the no-GC tail story); 1,003,000 timed ops, 0 failures. **Increment 10 (2026-07-16): annotation macros** — `crates/platform-macros` (`#[preload]`/`#[before_application]`/`#[main_application]`/stacked `#[zero_tracing]`, link-time `inventory` registration = the Java classpath-scan analog, D6 closed) + `AutoStart` one-liner (`auto_start_main!()`; ships OS-signal shutdown) + the **`examples/<name>/` app-crate convention** (hello-world relocated; 105 tests, workspace-wide clippy 0). `docs/INCREMENTS.md` = the increment ledger. **Layer 2 (event-script) started 2026-07-16**: canonical `event-script-engine` surveyed (~8K LOC; grammar spec = `flow-grammar.md`); `docs/design/event-script-port.md` DRAFT v1 (decisions E1–E9, increment plan E-1…E-9) **approved 2026-07-16**; **E-1 shipped** (repo increment 11): `crates/event-script` — flow model + full `CompileFlows` port, all 90 Java fixtures reused verbatim with the loaded-set/rejection/task-drop outcomes pinned by tests; **E-2 shipped** (repo increment 12): data-mapping engine — runtime MultiLevelMap over `rmpv::Value` (maintainer refinement: direct composite-key access = PRIMARY tool; JSONPath `$.…` = user-defined complex queries via serde_json_path), DataMappingHelper resolution + 19 core plugin bodies; greetings-fixture mappings evaluate byte-for-byte like Java (139 workspace tests, clippy 0). **E-3 shipped** (repo increment 13): platform-core extensions — event-interceptor mode (`FunctionOptions`; success reply guarded, failures still route — Java WorkerHandler parity), `send_later`/`cancel_future_event`, rest.yaml `flow:` → x-flow-id, deep-copy satisfied by `rmpv::Value::clone` (145 tests, clippy 0). **E-4 shipped** (repo increment 14): core flow runtime — FlowInstance + registries, manager/executor interceptors, FlowExecutor launch/request; sequential/response/end/decision/sink + exceptions + TTL abort + metrics + flow-summary span; E2E over the canonical fixtures (146 tests, clippy 0). **FLOWS EXECUTE ON RUST.** **Increment 15**: hidden direct execution for the reserved engine routes (event.script.manager/task.executor run on the event core — private route list in Platform::deliver, NOT exposed via macros/API by maintainer decision; concurrency + liveness, proof tests incl. a serialized control; 148 tests). **E-5 shipped** (repo increment 16): parallel + fork/join with the pipe-map barrier, dynamic `source` iteration (.ITEM/.INDEX), Java-exact pipe cleanup on exceptions; canonical parallel/fork fixtures verbatim + a marked Rust-side dynamic-fork supplement (canonical needs E-7). **E-6 shipped** (repo increment 17): pipelines (PipelineState in the pipe map) with for/while loops + break/continue; canonical loop fixtures verbatim incl. file() round-trip and per-step delay; decision.case upgraded to the faithful DecisionCase port. **E-7 shipped** (repo increment 18): flow:// sub-flows via the manager (child response = parent callback), shared parent state (Arc<Mutex<tree>> per family; materialized at model.parent under the shared lock; model.root normalized), ext: state machine (route + flow:// forms), SimpleExceptionHandler built-in; canonical dynamic-fork fixture ACTIVATED (5 concurrent sub-flows, exactly-once shared appends). **E-8 shipped** (repo increment 19): all 42 plugin bodies + `#[simple_plugin]` macro (new event-script-macros crate; SimplePluginLoader at sequence 3); plugin fixtures activated; user plugin proven E2E. **E-9 shipped (repo increment 20) — EVENT-SCRIPT (LAYER 2) MILESTONE CLOSED 2026-07-17**: HttpToFlow, Resilience4Flow, EventScriptMock, examples/hello-flow (live-verified: YAML flow over HTTP, cid propagation). E-1…E-9 = the complete engine, validated on the canonical Java fixtures. **Layer 3 (knowledge graph) started 2026-07-17**: canonical minigraph-playground-engine surveyed (~9.8K LOC + React webapp; MiniGraph itself is a Java platform-core built-in); `docs/design/knowledge-graph-port.md` (K1–K9). **LAYER 3 MILESTONE CLOSED 2026-07-18** (increments K-1…K-8 / repo 21–29): MiniGraph → math engine → compiler+registry → runtime + core skills (graph.js retired) → HTTP client + graph.api.fetcher → graph.extension → WebSocket server + declarative `#[websocket_service]`/`#[fetch_feature]` macros → the Playground (command grammar, traveler, companion API, dev-gating K9) → React webapp + `examples/minigraph-playground` (live-verified). **Post-milestone refinements (repo 30–32, 2026-07-18):** `#[optional_service]` macro + dev mock data providers (mock.mdm.profile/account.details/hello.task) → **full declarative dev-gating** — `#[optional_service]` extended to `#[websocket_service]`/`#[before_application]`/`#[main_application]`, the Playground retired from programmatic registration to declarative `#[preload]`/`#[websocket_service]` + `#[optional_service("app.env=dev")]` (Java parity; the Java PlaygroundLoader is only the @FetchFeature loader), `app.env` env-overridable (`${APP_ENV:dev}`); prod-mode verified to skip the whole Playground. Increment 32: `inspect` **docs** `{…}`-placeholder fix (examples unbraced, both repos; Java canonical pushed) — the webapp autocomplete template was briefly changed then reverted (its `{variable_name}` is a correct fill-in placeholder, not an example) — surfaced by the AI-companion validation of tutorial-3, which PASSED end-to-end (a fresh companion built the data-dictionary graph from the canonical docs alone → exact match to tutorial-3.json; dry-run returned Peter/100 World Blvd). **All three layers ported bottom-up. 193 workspace tests green.**
- **last_enabled:** 2026-07-15
- **last_session:** 2026-07-18 | agent: Claude Code (2026-07-18-194853)
- **last_review:** 2026-07-18 | through 2026-07-18-061457
- **last_invariant_check:** 2026-07-18 | through 2026-07-18-061457 (confirmed — inv-never-couple-functions + Vision both hold)
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

- **Never couple functions directly** (ADR-0001) — inter-function coupling stays **route-name +
  `EventEnvelope`** only; no direct calls between user functions. This is the defining
  invariant inherited from mercury-composable (the actor-model decoupling); the whole
  three-layer design rests on it. Preserve it in the Rust port. Full ADR ledger:
  `docs/arch-decisions/ADR.md` (ADR-0001…0007, adapted from the Java repo — read on demand).
  <!-- id: inv-never-couple-functions | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: core | origin: 2026-07-15-221632.md -->

*(More invariants will be distilled from mercury-composable's docs/ADRs as each layer is
ported — e.g. stateless functions, HTTP-style status codes.)*

## Key Decisions

- **Port bottom-up, faithfully to the Java original** — re-implement mercury-composable in
  Rust layer by layer, foundation → UI (platform-core, then event-script, then active
  knowledge graph), preserving the Java project's behavior. The Java repo is the canonical
  spec (map, don't mirror).
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-07-18 | uses: 18 | tier: active | origin: 2026-07-15-215538.md -->
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
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-07-18 | uses: 36 | tier: archive-candidate | origin: 2026-07-15-224707.md -->

## Open Threads

> Mark completed items `- [x]` and leave them in place — the review sweeps them to
> the archive once older than `archive_window` sessions. Don't archive them by hand.

- [ ] **AI-companion validation sweep (tutorials 1–13)** — dogfooding the full human–AI
  collaboration path: a *fresh* companion agent builds each Playground tutorial from the **canonical
  AI-agent docs alone** (`mercury-composable/docs/llms.txt` + `.../knowledge-graph/ai-agent-guide.md`
  + `command-reference.md` + `minigraph-commands.json` — never the tutorial walkthroughs), one node /
  one connection at a time with human screenshot validation, through the instantiate→run→inspect
  dry-run (from tut-4 on: L3 = problem-only brief, no syntax hints, build-whole-then-dry-run). Running
  log: `docs/AI-companion-test.md`. **Done: 1, 2, 3, 4.** Tut-3 surfaced + drove the
  `#[optional_service]` / dev-mock / Dictionary bare-`input[]` / inspect-doc-placeholder arc (increments
  30–32, both repos). **Tut-4** (decision node) found an AI-grammar gap — the AI docs listed
  `graph.math` statement keywords but gave no statement syntax, so a fresh agent invented wrong
  branching that failed silently; **fixed** by documenting the `graph.math`/`graph.js` statement grammar
  (IF/THEN/ELSE etc.), landed via public-upstream PR
  [Accenture/mercury-composable#187](https://github.com/Accenture/mercury-composable/pull/187), then a
  second fresh agent solved it from the docs alone (dry-run PASS, all branches + `a==b`). Also validated
  MiniGraph **session-management** (`session subscribe` link + a zero-dep Node WS subscriber gave the
  orchestrator live console visibility). **Pending: 5–13** — each expected to surface more; that is the
  *purpose*, hardening the Rust port toward production quality. **Resume:** user restarts the Rust server
  (`app.env=dev`) + provides a fresh `ws-…` primary; spin a companion the same way.
  → serves: vision-mercury (faithful delivery; a fresh agent orients + operates from the docs alone)
  <!-- id: ot-companion-validation-sweep | created: 2026-07-18 | last_used: 2026-07-18 | uses: 4 | tier: working | origin: 2026-07-18-061457.md -->

- [x] **Re-verify invariants — CONFIRMED 2026-07-18 (maintainer): both still hold.**
  `inv-never-couple-functions` (the sole Architectural Invariant — inter-function coupling stays
  route-name + `EventEnvelope` only) and the **Vision** (`vision-mercury`, `memory/vision.md`)
  re-confirmed unchanged. First invariant check since enable (cadence:
  `verify_invariants_every: 40` sessions).
  <!-- id: ot-reverify-invariants-2026-07 | created: 2026-07-18 | last_used: 2026-07-18 | uses: 1 | tier: working | origin: 2026-07-18-061457.md -->

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
  <!-- id: bp-platform-core | created: 2026-07-15 | last_used: 2026-07-18 | uses: 27 | tier: archive-candidate | origin: 2026-07-15-215538.md -->
- [x] **(blueprint)** Port **event-script** to Rust (on top of platform-core).
  **MILESTONE CLOSED 2026-07-17** (increments E-1…E-9 / repo 11–20; design
  `docs/design/event-script-port.md` §5a–§5i): the complete Event Script engine — compiler,
  mapping engine, all 8 execution types, sub-flows + shared parent state, external state
  machine, 42 plugins + #[simple_plugin], HTTP flow adapter, resilience, mock,
  examples/hello-flow — validated against the canonical Java fixture suite (66 flows).
  Remaining (traced, non-blocking): HTTP-client fixtures await platform-core §7 http client;
  stream payloads; mock monitors. → serves: vision-mercury
  <!-- id: bp-event-script | created: 2026-07-15 | last_used: 2026-07-17 | uses: 17 | tier: archive-candidate | origin: 2026-07-15-215538.md -->
- [x] **(blueprint)** Port the **active knowledge graph** to Rust.
  **MILESTONE CLOSED 2026-07-18** (increments K-1…K-8 / repo 21–29; design
  `docs/design/knowledge-graph-port.md` K1–K9): MiniGraph property graph (platform-core
  built-in) → math engine → graph compiler+registry → runtime + core skills (graph.js
  retired) → platform-core HTTP client + graph.api.fetcher → graph.extension → the
  WebSocket server + declarative `#[websocket_service]`/`#[fetch_feature]` macros → the
  Playground (command grammar, traveler, companion API, dev-gating K9) → the React webapp +
  `examples/minigraph-playground`, live-verified in the browser. **Three layers ported
  bottom-up: platform-core → event-script → active knowledge graph.** → serves: vision-mercury
  <!-- id: bp-active-knowledge-graph | created: 2026-07-15 | last_used: 2026-07-18 | uses: 20 | tier: active | origin: 2026-07-15-215538.md -->
  **Design drafted 2026-07-17** (`docs/design/knowledge-graph-port.md` v1) — gate pending.
- [ ] **(blueprint)** Continue **foundation → user interface** once the three layers stand.
  → serves: vision-mercury
  <!-- id: bp-foundation-to-ui | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(blueprint)** **Graduate to the official Accenture repo** once the foundation is
  sufficient (this private repo is the prototyping stage). → serves: vision-mercury
  <!-- id: bp-graduate-to-accenture | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(blueprint)** **Synchronous AI-companion feedback** — make the companion a real AI *tool*, not
  a write-then-poll bus. The current `POST /api/companion/{id}` is fire-and-forget (`{status:accepted}`);
  command outcome + errors stream WS-only, so an AI caller is blind (Tut-4: HTTP 200 while the run had
  aborted). Design: an **additive** `POST /api/companion/{id}/sync` returning a structured envelope
  `{ok, output, error, run}` — errors in-band, run/inspect results folded in; WS console + existing
  endpoint unchanged. Mechanism reuses `PostOffice::request` (RPC) + a private capture route
  (`Platform::register`). Design note: `docs/design/ai-companion-sync.md`. **Rust prototype SHIPPED +
  verified end-to-end** (`52318c3`, `post.companion.command.sync`, dev-gated; integration test
  `companion_sync_returns_outcome_in_band`): a fresh Tut-4 companion (session ws-776722-2) rebuilt +
  ran + self-validated all 3 cases **fully autonomously** via `/command`, explicitly never needing the
  WS console or a GET fallback; errors + run results both return in-band. **Real-time human+AI
  collaboration DONE (v1):** the capture sink now **tees** each line to the session's real WS `.out`
  (maintainer's refinement), so a watching human sees it live and — via the command service's existing
  subscriber fan-out — any `session subscribe`d watcher (e.g. a PO on the UI) does too; suspend/resume
  across sprints via export/import. Deterministically tested (`OutTap` in `companion_sync_returns_outcome_in_band`).
  **Live multi-party demo DONE (2026-07-18, session ws-240155-2):** a fresh companion built+ran tut-4
  via `/command` while the maintainer watched TWO UI sessions (architect + PO subscribed); it even hit
  `graph.js`-retired / bitwise / `parseInt` dead-ends and **self-corrected from the in-band errors,
  live** — the whole thesis proven. **AI docs ported into this repo (b DONE):** `docs/llms.txt` +
  `docs/guides/knowledge-graph/{ai-agent-guide,command-reference,minigraph-commands,skills-reference}`
  now live here (reflect Rust: `graph.js` retired, sync endpoint documented, graph.math dialect
  narrowness). URI renamed `/command` → **`/sync`** (`b89a997`; communicates intent). Future companion
  tests reference the Rust repo. **Java upstream: ADR-0008 MERGED (#188) + implementation MERGED
  (#189** — `PostCompanionCommandSync`, mirrors the Rust mechanism: RPC + private capture route + FIFO
  sentinel + WS tee; `CompanionSyncTest`; 64-test module suite green; ADR-0008 → Accepted**) + follow-up
  [#190](https://github.com/Accenture/mercury-composable/pull/190) OPEN** (the standalone
  `examples/minigraph-playground` app loads its own rest.yaml which #189 missed → the route 404'd on the
  example app until #190 adds it). **Java live regression PASSED** (example app :8085): `/sync`
  bad→`ok:false`+error, build+run hello→`ok:true`, `inspect output.body`→`outcome:"hello world"`, and the
  **tee confirmed** (the maintainer's UI streamed every command + rendered the graph live).
  **Drain fix (`b37b131` Rust + PR [#191](https://github.com/Accenture/mercury-composable/pull/191)
  Java):** the earlier "cross-port note" (Java `run` emits less to console) was **WRONG** — the Java WS
  console already echoes output.body/Executed/completed (maintainer screenshot). The real gap was the
  **`/sync` drain**: `run` is the only *asynchronous* command (handler replies, then the traveler
  streams its output), so the post-reply FIFO sentinel raced (and beat) the traversal tail →
  truncated `/sync run` to "Walk to root/end". Fix (both ports): drain a traversal on the traveler's
  **terminal line** (`Graph traversal completed in N ms` | `Graph traversal aborted`, always last);
  sync commands keep the sentinel; the wait is a safety net, the signal drives correctness. Made the
  terminal a contract: every `run` now ends with completed/aborted (early-failure paths emit the reason
  + `emit_aborted`/`emitAborted`), so `run` before `instantiate` returns promptly (ok:false), not on the
  timeout. **`/sync` REST contract now byte-identical across engines** (the companion surface is
  language-neutral). Tests: Rust `playground.rs` (full traversal capture + tee + early-failure
  terminal), Java `CompanionSyncTest` (same); 64-test Java module green. **ALL UPSTREAM PRs MERGED
  (2026-07-18):** #188 (ADR) + #189 (impl) + #190 (example rest.yaml) + the drain fix + #192
  (Thread.sleep→`Utility.sleep` Sonar cleanup) are on `Accenture/mercury-composable` `main`. Note: the
  drain-fix PR #191 was **auto-closed** by GitHub when #190's base branch was deleted on squash-merge
  (GitHub closed the stacked PR instead of retargeting it); recovered as **#193** (same commit
  cherry-picked onto the updated main) and merged. Lesson → [[github-squash-merge-closes-stacked-pr]].
  **The companion `/sync` feature is COMPLETE and MERGED in both ports** (Rust R&D main + Java upstream
  main); the `/sync` REST contract is byte-identical and language-neutral. Blueprint gap closed.
  → serves: vision-mercury
  <!-- id: bp-companion-sync | created: 2026-07-18 | last_used: 2026-07-18 | uses: 9 | tier: working | origin: 2026-07-18-162832.md -->

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

- [x] **Design + port the knowledge graph (layer 3)** — the working thread realizing
  `bp-active-knowledge-graph`. **COMPLETE 2026-07-18 (K-1…K-8 / repo increments 21–29).**
  **Design DRAFT v1 written 2026-07-17**
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
  (10 tests). **K-2 SHIPPED 2026-07-17 (increment 22)**: `knowledge_graph::math` in the NEW
  `crates/knowledge-graph` (created one increment early; engine wiring lands at K-3) —
  lexer/shunting-yard parser/evaluator with Java-exact semantics (strict JS `**` rule,
  short-circuit logic, dual number rendering display `3.0` vs concat `3`, `NaN != NaN`,
  mirrored `Math.*`); all 14 ExpressionEngineFullTest methods ported (15 tests); workspace
  173 green / clippy 0 / fmt clean. **K-3 SHIPPED 2026-07-17 (increment 23)**:
  `compile_graphs` quality gate (manifest-gated, ${...} resolution, shared event-script
  converter reuse, MiniGraph structural validation) + `graphs` registry + K8 resource-root
  hook (seq 1, append-only); 26 fixtures verbatim all compiling; CompileGraphTest ported
  (3 tests); workspace 176 green. Design correction: Java PlaygroundLoader = FetchFeature
  scanner → moved to K-5. **K-4 SHIPPED 2026-07-17 (increment 24)**: graph.executor
  (interceptor; walk/decide, loop detection, compiled+lazy models) + core skills
  (data.mapper / math / task incl. fork-join / join / island) + housekeeper (end-flow
  listener) / exception handler / health; GraphLambdaFunction → common.rs;
  **graph.js retirement enforced in code** (explicit message → graph.math/graph.task);
  GraphTraveler → K-7 (dev-only Playground walker). E2E: tutorials 1/2/4/7/8/9/13 +
  GraphTaskTest 1-5 + rust supplements (join/loop/retirement); workspace 177 green /
  clippy 0 / fmt clean. **K-5 SHIPPED 2026-07-17 (increment 25)**: platform-core
  `async.http.request` (hyper http1 interceptor; trace via envelope+invocation headers —
  the trackable model; streams/XML/https deferrals documented) + AsyncHttpRequest builder;
  E-9 http-client fixtures ACTIVATED incl. full-shape W3C trace propagation through the
  real edge; graph.api.fetcher + FeatureRunner registry (explicit registration replaces
  the Java annotation scan) + mocks; tutorials 3/5/6/12/114 + unit-test-1 green over real
  HTTP first run; §7 http-client deferral CLOSED; workspace 177 green. **K-6 SHIPPED
  2026-07-17 (increment 26)**: graph.extension (sub-graph via graph-executor flow +
  flow:// delegation, for-each fork-join); flow-11.yml shipped; two lockstep parity
  fixes — no.op promoted to platform-core built-in (Java NoOpFunction) and Jayway-parity
  hyphen tolerance in event-script JSONPath; tutorials 10/11 + helloworld2 green;
  K-6's REST endpoints moved to K-7 (they hang off GraphCommandService); the
  graph.js-carrying fixtures (hello/helloworld/hellojs/tutorial-113) ACTIVATED by
  maintainer-directed skill swap to graph.math (identical statement grammar; one
  genuinely-JS node adapted); rust-js-retired stays the single retirement case;
  workspace 178 green. **K-7a SHIPPED 2026-07-17 (increment 27)**: platform-core
  WebSocket server (hyper upgrade + tokio-tungstenite; Java WsRequestHandler protocol —
  session route pairs, lifecycle events, transmitter, idle sweep, housekeeper; explicit
  register_ws_service replaces @WebSocketService scan; PLUS maintainer-directed
  declarative #[websocket_service] macro — inventory + lifecycle URL registration +
  Java startHttpServerIfAny start condition; PLUS declarative #[fetch_feature] macro —
  new knowledge-graph-macros crate, inventory loaded at engine startup, proven with the
  OAuth-bearer DemoAuth pattern on the wire); E2E with a real tungstenite client
  (programmatic + declarative); workspace 180 green. **K-7b SHIPPED 2026-07-17 (increment
  28)**: the Playground — commands.rs (1,494-line GraphCommandService grammar), session.rs,
  traveler.rs, ws_ui.rs (GraphUserInterface + JsonPathHandler), rest.rs (AI-companion
  `POST /api/companion/{id}` hop + K-6-deferred dev endpoints), 39 verbatim help/*.md,
  dev-gated PlaygroundLoader (app.env=dev default, K9). **platform-core contract fix surfaced
  by integration**: booting the engine registers WS services → `AutoStart::main` used to block
  on ctrl_c and hang embedders/tests; moved the serve-until-Ctrl-C wait into `AutoStart::run`
  (standalone fn main()), `main` now returns once booted (accept loop runs in the background);
  added `server_address()` (first-bind) for ephemeral-port recovery; start_http_server still
  binds per call (each #[tokio::test] keeps its own server — actuator suite proved the earlier
  global-idempotent attempt wrong). E2E tests/playground.rs (grammar → companion REST →
  live-graph download). Workspace **181 green**, clippy 0, fmt clean. **K-8 SHIPPED
  2026-07-18 (increment 29) — LAYER 3 MILESTONE CLOSED**: React webapp copied verbatim into
  `crates/knowledge-graph/webapp/`; clean.js/deploy.js retargeted to `../resources/public`;
  a 3rd path of the same class (maintainer-approved) — helpContent.ts's build-time help glob →
  `../../../resources/help/*.md` (else the in-app Help panel is empty). `npm run release` →
  committed bundle in resources/public (served at `/`; source maps gitignored). New
  `examples/minigraph-playground` app (open Q4 = yes; own application.yml/rest.yaml, port 8100,
  hello-flow convention). Live-verified against the running app (Chrome ext down → exercised the
  exact browser protocol/paths): static `/` + assets 200, `/ws/graph/playground` session +
  command round-trip (help streamed), `POST /api/companion/{id}` → WS console. → serves:
  vision-mercury
  <!-- id: ot-design-knowledge-graph | created: 2026-07-17 | last_used: 2026-07-18 | uses: 11 | tier: archive-candidate | origin: 2026-07-17-010622.md -->

- [ ] **(backlog) Generic `app.profiles.active` alias for profile selection.** Maintainer
  decision 2026-07-15: keep `SPRING_PROFILES_ACTIVE`/`spring.profiles.active` **verbatim**
  during migration for side-by-side comparison with the Java original; add a generic alias
  once the foundation port is robust. Don't build until then. → serves: vision-mercury
  <!-- id: ot-profiles-alias-backlog | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-224707.md -->

## User Preferences

(none recorded yet — record ONLY what the user explicitly states; never infer)

## Team / Members

(none recorded yet)
