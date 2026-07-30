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
- **status:** **Rust port of `mercury-composable`** (canonical Java v4.8.6), delivered bottom-up; all three in-scope layers (platform-core, event-script, active knowledge graph + Playground) ported and milestone-closed, **GRADUATED to github.com/Accenture/mercury 2026-07-20** (docs at accenture.github.io/mercury; regular PR process). Kafka service mesh + Spring out of scope. Current release **v4.11.0** (version tracks the Java line, contents by design). History/detail lives in `docs/INCREMENTS.md` (increment ledger), `docs/design/`, session logs, and CHANGELOG — not this line.
- **last_enabled:** 2026-07-15
- **last_session:** 2026-07-30 | agent: Claude Code (2026-07-30-180659)
- **last_review:** 2026-07-30 | through 2026-07-30-011111.md
- **last_invariant_check:** 2026-07-26 | 2026-07-26-014908.md (all five confirmed against live code; two header drifts remedied; ui-fixture carve-out RATIFIED by Eric 2026-07-26)
- **repo:** github.com/Accenture/mercury (official home; graduated 2026-07-20 from the private R&D repo acn-ericlaw/mercury)
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
deliberately not a web framework: rest.yaml IS the router), **chrono 0.4 + chrono-tz 0.10 +
iana-time-zone 0.1** (event-script date/time plugins; chrono-tz = the ZoneId.of analog,
increment 53), **tokio-rustls 0.26 (ring) +
rustls-native-certs 0.8** (increment 48 — outbound HTTPS with OS-trust-store verification +
`trust_all_cert`; rcgen dev-dep for the self-signed TLS test), **moka 0.12 (sync)**
(increment 71 — the ManagedCache engine, Caffeine's Rust lineage, wrapped as an internal
detail; built with `EvictionPolicy::lru` per Eric's deterministic-eviction ruling). Stack rationale:
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
  `docs/arch-decisions/ADR.md` (ADR-0001…0007 adapted from the Java repo; ADR-0008 native —
  read on demand).
  <!-- id: inv-never-couple-functions | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: core | origin: 2026-07-15-221632.md -->

- **Telemetry/log presentation parity with the Java reference implementation** — the
  trace-record topology (record count per trace, service names, parent edges,
  round_trip-vs-exec kinds, paths) and the log presentation (app-log-context gating,
  header hygiene) of this port must remain an exact structural replica of the Java
  engine's, which is THE reference. Rationale (Eric, 2026-07-23): field installations
  stay POLYGLOT for a long time — DevSecOps teams see both engines' telemetry and logs
  in one aggregation, and any presentation difference is a support burden they will
  flag. This is a standing invariant, not a one-off acceptance criterion; the Java-to-
  Java normalized signature is the acceptance instrument (see increment 64).
  <!-- id: inv-telemetry-presentation-parity | created: 2026-07-23 | last_used: 2026-07-23 | uses: 1 | tier: core | origin: 2026-07-23-152724.md -->

*(More invariants will be distilled from mercury-composable's docs/ADRs as each layer is
ported — e.g. stateless functions, HTTP-style status codes.)*

## Key Decisions

- **String plugins use Unicode scalar values, by maintainer ruling (2026-07-26).**
  `f:length` / `f:substring` count and index WHOLE CHARACTERS (Unicode scalar values /
  code points) — NOT Java's UTF-16 code units (a JVM legacy that must not propagate to
  future Python/Node/Go ports) and NOT UTF-8 bytes (你好 = 2, never 6). Supersedes the
  increment-57/F20 UTF-16 retrofit (code + ledger only — no continuity fact existed to
  supersede formally). Divergence bounded to supplementary-plane characters (emoji = 1
  here, 2 in Java; BMP text identical); out-of-bounds semantics + error messages
  unchanged; the retrofit's surrogate-split micro-divergence is retired (no lossy case
  under scalar indexing). Do NOT re-retrofit UTF-16 in the name of parity — the ruling
  is deliberate and Eric-verified. Docs: syntax-guide Rust-port note + CHANGELOG.
  <!-- id: string-plugins-unicode-scalars | created: 2026-07-26 | last_used: 2026-07-28 | uses: 3 | tier: active | origin: 2026-07-26-022229.md -->

- **Registration metadata is a cross-language contract; carriers are per-language idioms.
  (ADR-0008)** One canonical model + fixed semantics for #[preload] and family (boot-time
  env_instances resolution; optional-service OR/!/= grammar; order-free marker stacking; one
  conflict policy — explicit > declarative, duplicates WARN + last-wins; extension-point
  naming: explicit positional name or same-name derivation from idiomatic declarations;
  plugins = flow vocabulary never gated, features honor gating; discover → register →
  override → resolve → validate → route table; loud-failure discovery; misuse is a tested
  error surface). Spec: docs/guides/registration-metadata-contract.md (adapted from the Java
  reference page). Conformance: golden vectors shared verbatim
  (registration-vectors/{core,plugin,feature}.json, byte-identical to the Java copies) —
  the wire-format golden-vector method applied to the declaration surface. New ports pass
  the three vector suites before their declaration surface is done. Twin of the Java
  ledger's ADR-0009.
  <!-- id: registration-metadata-contract | created: 2026-07-26 | last_used: 2026-07-28 | uses: 4 | tier: active | origin: 2026-07-26-013851.md -->

- **Port bottom-up, faithfully to the Java original** — re-implement mercury-composable in
  Rust layer by layer, foundation → UI (platform-core, then event-script, then active
  knowledge graph), preserving the Java project's behavior. The Java repo is the canonical
  spec (map, don't mirror).
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-07-29 | uses: 96 | tier: active | origin: 2026-07-15-215538.md -->
## Conventions

> Established with the first code (increment 1, 2026-07-15); enforced from the first commit.

- **The Java repo's helper servers are the standard local test servers for Rust ports
  (Eric, 2026-07-30).** `helpers/redis-standalone` for the suspend/resume arc;
  `kafka-standalone` + the schema-registry-mock when minimalist-kafka is ported. WHY:
  the helpers embed REAL redis/kafka servers behind a plain `java -jar`, motivated by
  field reality — many developer machines are Windows, especially VDI environments with
  no virtualization system, so Docker is unavailable; a jar works everywhere. Tier: unit
  tests may use fast hermetic in-process doubles (e.g. the RESP2 test double — the
  double stands in for the SERVER, never the client); the helper is the
  integration/live-drive tier.
  <!-- id: conv-java-helper-servers-for-rust-tests | created: 2026-07-30 | last_used: 2026-07-30 | uses: 1 | tier: working | origin: 2026-07-30-015038.md -->

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
- **`tests/ui` compile-fail FIXTURES are test resources — no license headers** (Eric,
  2026-07-26: "ok with the tests/ui without license headers"): a header shifts every
  `.stderr` line and forces TRYBUILD regeneration; treated like Java's
  `src/test/resources` files. The ui RUNNERS (`tests/ui.rs`) do carry headers.
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-07-30 | uses: 100 | tier: active | origin: 2026-07-15-224707.md -->

## Open Threads

> Mark completed items `- [x]` and leave them in place — the review sweeps them to
> the archive once older than `archive_window` sessions. Don't archive them by hand.

- [x] (release — 2026-07-30; **TAGGED, then RE-TAGGED pre-publication (Eric's ruling:
  release tags include the updated docs): `v4.11.0` now on merge commit `167484bd` (PR
  [#190](https://github.com/Accenture/mercury/pull/190) — the docs-parity fix: home
  footer "Explore the docs" clusters + Project block with the cross-engine "Java
  version" line + the Release Notes nav link). The original tag on `cc529071` (PR #189)
  was DELETED before publication and re-created — the commit VERIFIED both times
  (Cargo.toml 4.11.0 + the docs fix + both ancestor commits at `167484bd`); remote
  dereference confirmed → `167484bd`. The 4.10.2-round pre-publication tag-move
  precedent applied; a tag NEVER moves after publication. Publication pending Eric,
  who has the drafted release notes.**) **v4.11.0 release prep, lock-step with the Java
  repo (Eric's plan; the suspend/resume feature release).** Contents this side: the
  complete suspend/resume arc (PR #186), the interop report + cid trim + 8085 port sync
  (PR #187), the nav consolidation (PR #188), plus the ManagedCache port + health-info
  cache + WS-dedup/up_time fixes that rode Unreleased since 4.10.6. Sweep 4.10.6→4.11.0:
  root Cargo.toml `[workspace.package]` (count-asserted single occurrence, no substring
  hazards), lockfile regenerated (11 members at 4.11.0, zero at 4.10.6), continuity
  status line; CHANGELOG Unreleased → `## Version 4.11.0, 7/30/2026`. Branch
  `chore/release-4.11.0`, NOT pushed — Eric gates push/PR/tag (verify the tag lands on
  the verified merge commit — the 4.10.2 tag-race lesson). Java side: 33 poms swept,
  skipTests hardcode removed from 26 poms. Close when tagged + published both repos.
  <!-- id: thread-release-4-11-0 | created: 2026-07-30 | last_used: 2026-07-30 | uses: 1 | tier: working | origin: 2026-07-30-172823.md -->

- [x] (feature — 2026-07-29; **MERGED 2026-07-30 as
  [PR #186](https://github.com/Accenture/mercury/pull/186), merge commit `d2791b09`
  — five commits `304fc5a0`→`9326cf55`; Rust ADR-0009/ADR-0010 thereby ACCEPTED (the
  merge was the ledger gate). The suspend/resume surface is now IDENTICAL on both
  engines. Likely next arcs: the lock-step official release (Eric planning), then the
  minimalist-kafka port — kafka-standalone + schema-registry-mock as the local test
  servers per [[conv-java-helper-servers-for-rust-tests]].**) (was: branch `feature/graph-suspend-resume`, P5-1
  committed, NOT pushed — Eric gates) **P5: graph suspend/resume Rust lock-step arc**
  (handoff /tmp/graph-suspend-resume-rust-handoff-p5.md; Java ALL MERGED PRs #238-#241,
  Java ADR-0010/0011 accepted; FINAL surface only — no missing=<node>, no rejected-graph
  registry). **P5-1 DONE (engine core):** new `suspend.rs` (graph.suspend/graph.resume as
  graph.task supersets: shared context ladder, get_required_correlation_id, ONE
  overflow-guarded ttl parser get_valid_ttl_seconds (64-bit, <1 or >i32::MAX rejected, NO
  default), persistence envelope {cid,node,ttl,model−reserved,seen,run}, synchronous 2xx
  durability ack, default `{"type":"suspended","cid"}` reply, warnIfBranchesInFlight,
  NON_PERSISTED_MODEL_KEYS ×9 both directions, restore: corrupted/unknown-node 500s via
  record_failure, merge-then-set model.run=resume, restoreMarks truthy-only EXCLUDING
  suspend, fresh path model.run=fresh at DEBUG); BOTH walkers converted (near-mirror:
  `resume:<alias>` directive, walk_to_suspend_node — executor trusts the gate, traveler
  keeps FULL guards incl. math/js + missing/mis-skilled suspend node; resume_traversal
  marks seen+run without executing; walk_next(after_resume) excludes `suspend` + forged
  leaf-record dead-end guard in BOTH; atomic putIfAbsent walk seen-check under one lock;
  traveler per-run reset also clears hits; executeSkill stamps FROM header + business-cid
  tag from model.cid — interceptor walkers don't auto-propagate; executor
  execution_complete applies declarative output.status). Registered
  graph.suspend/graph.resume (instances 300). Fixtures copied VERBATIM from Java
  (unit-test-suspend-1..5 + err1-7 + no-end; manifest updated). Tests: the
  GraphSuspendResumeTest twin suite (6 scenarios: envelope shape + no reserved keys
  persisted, no-re-execution + x-run=resume, multi-checkpoint 3-runs-1-cid,
  join-across-suspension, fresh-gate 404 + run=fresh + valid pass, 1s-expiry fallback,
  forged-record reserved-key strip with real-cid identity survival) + ttl parser unit
  tests (incl. overflow) + temp-file mock store (/tmp/suspend-resume, MsgPack
  {expires_at,data}, delete-on-read) + counting-step business-cid registry. Gate: 288 /
  clippy 0 / fmt. **P5-2 DONE (mandatory CompileGraph gate — compiled or 404):**
  compiler.rs rewritten as the deployment gate (obsolete `location.graph.deployed`
  startup warning; no-manifest WARN "no deployed graph models will be executable";
  manifest-carried `location` with prefix validation + fallback, flows.yaml convention,
  stored in the graphs registry — GraphCommandService reads it from there; per-graph:
  convert → import → root purpose → mandatory `end` node → validate_suspend_resume →
  register; rejection = `log::error!("Rejected graph {id} - {reason}")`, NOT registered);
  **property-aware mapping rejection** (entries without `->` reject the graph for
  mapping/for_each/output; bare `input` entries are skill vocabulary — fetcher
  dictionary params — and pass silently; the old blanket error log is gone); NEW
  `model_validator.rs` (GraphModelValidator twin, exact Java error strings, reuses the
  shared ttl parser) called by BOTH the gate and the playground `run` command as a
  pre-run check ("Unable to run - <reason>" + the uniform "Graph traversal aborted"
  terminal so the sync companion drain stays deterministic; keyed off the INSTANTIATED
  graph like Java); executor streamlined (deployed_graph_location + lazy per-request
  loading DELETED — registry-or-404 `"{id} not found"`, no filesystem access; empty-map
  re-check and per-request end-node check dropped — the gate guarantees); `model.run`
  joined event-script's RESERVED_MODEL_KEYS (compile + runtime dynamic-target guards via
  the one shared fn; parser-test-32 fixture twin copied verbatim). Tests: knowledge-graph
  tests/compiler.rs flipped to the CompileGraphTest twin (7 tests: 39 compiled, err1-7 +
  no-end rejected, location default, per-err static-validator asserts);
  rejected_deployed_graph_is_not_executable (8×404 "not found");
  companion_sync_pre_run_check_rejects_broken_suspend_contract (suspend node without a ttl rejected
  pre-run + terminal); bare-input vocabulary unit test. **Porting note: the runtime test
  manifest must now list EVERY graph a test executes** (5 rust-* fixtures had ridden the
  lazy path; compiled-or-404 exposed them — graphs.yaml is deployment intent). The Rust
  playground example app gained its graphs.yaml manifest (tutorials 1-12; 13 excluded
  like Java — it calls an engine-test fixture function; 14 arrives in P5-4) +
  `graph.model.automation` in application.yml. Gate: workspace 293 / clippy 0 / fmt.
  **P5-3 DONE (Redis store crate):** NEW workspace member
  `extensions/minigraph-state-redis` (Java module twin) — `v1.redis.persist.model`
  (type=put; SETEX graph:state:<cid>, MsgPack bytes, native expiry; 2xx = the durability
  ack) + `v1.redis.retrieve.model` (type=get; GETDEL atomic consume, Redis 6.2+;
  absent/expired = empty map = fresh), instances 50 each with the Java-named
  `worker.instances.v1.redis.*.model` env keys; exact Java error strings (Type must be
  put/get, Missing cid, Invalid ttl) + log lines. **Crate choice: `redis` (redis-rs)
  v1.5.0** — the official Rust Redis client; `ConnectionManager` = the Lettuce analog
  (one shared multiplexed connection, auto-reconnect, lazy first-use via
  tokio OnceCell get_or_try_init — a failed first connect is NOT cached, retried);
  explicit cmd("SETEX")/cmd("GETDEL") for exact command parity; features tokio-comp +
  connection-manager + tokio-rustls-comp (redis.ssl without OpenSSL); every round-trip
  bounded by redis.timeout.ms via tokio timeout (fred/deadpool rejected: pooling+cluster
  weight this store doesn't need). `redis.*` config keys shared with the sync-over-async
  family. **Engine independence held:** imported by examples/minigraph-playground ONLY
  (Cargo dep + main.rs inventory reference + startup log); live boot proof — gate
  compiles 12 tutorials, both functions register at 50 instances, no eager Redis
  connection. **Tests:** the 7-scenario Java RedisStateStoreTest twin in ONE test fn
  (register/round-trip-with-consume incl. binary fidelity + wire-visible TTL/forged-key
  checks/absent-normal/1s-expiry/wrong-type/missing-cid/invalid-ttl) driven through the
  real event system and the REAL redis client over TCP against an in-process **RESP2
  test double** (~150 lines: SETEX/GETDEL/GET/TTL/DEL/PING + tolerant handshake;
  ephemeral port + overrides::set for redis.host/port) — this env has no redis-server
  binary and no Docker daemon, so the double stands in for the server, never the client
  (the Java suite uses embedded redis-server). README adapted from the Java module.
  Gate: workspace 294 / clippy 0 / fmt. INCREMENTS.md rows for the P5 arc ride the P5-4
  docs pass. **P5-4 DONE — THE ARC IS CODE- AND DOCS-COMPLETE (pending the Java-side
  consistency review + Eric's push gate):** tutorial-14.json VERBATIM (byte-identical
  shasum) + playground manifest entry; app-level `SuspendResumeTutorialTest` twin
  (4 runs + fresh + 404-rejection over real HTTP and the real Redis client vs the RESP2
  double); LIVE four-run drive against the Java repo's redis-standalone helper (Eric's
  direction — see [[conv-java-helper-servers-for-rust-tests]]) with FULL §10.5 parity
  evidence: reply shape per run (stage/run/cid; full history run 4; 404+run=fresh),
  log-context cid = business id on every traced store line, span topology (store calls
  parented on graph.suspend/graph.resume spans annotated task+cid; NO re-executed
  checkpoint spans on resume — run 2 shows resume→mapper→suspend, no order/math spans).
  Docs: workflow-suspension guide, ten-skill tables (skills-reference + index + help.md),
  help tutorial 14 (incl. interactive dry-run section) + graph-suspend/graph-resume +
  run pre-run note + tutorial 2 manifest recipe, minigraph-commands.json (2 skills,
  model.run namespace, run note, suspend invariants — surgical edits, no reformat),
  syntax.md model.run row, flow-schema reserved list, reserved-names additions, redis.*
  config family, CHANGELOG (feature + BREAKING gate entry with migration), mkdocs nav,
  webapp bundle REBUILT (help pages bake at build time). ADR-0009 (suspend/resume) +
  ADR-0010 (gate) PROPOSED as twins of Java ADR-0010/0011; design-record "session
  persistence out of scope" line superseded for workflow state. INCREMENTS.md rows
  72-75. Memory review run at this seam (size-triggered). Gate: workspace 295 /
  clippy 0 / fmt. **Java-side consistency review COMPLETE (high fidelity; 22 confirmed
  findings) and the FIX ROUND APPLIED — all 22 addressed:** 4 blockers ([#0] the restore
  merge is now a LITERAL key-level putAll (Java parity) — a forged composite-path key
  "cid.x"/"ttl[0]" can no longer descend into and replace model.cid/model.ttl via
  set_element path interpretation; composite-forge regression added; Java assessed
  structurally immune (putAll), suggested dotted-key vectors there as immunity
  documentation; [#6] `instantiate graph` is now the dry-run's edge — auto-creates
  model.cid + the "No business correlation ID given…" reminder, both cases pinned;
  [#10] walker seen-marking is insert-if-absent (putIfAbsent parity — never overwrites a
  join's false barrier flag); [#11] 'suspend' joined RESERVED_PARAMETERS with Java's
  "ttl deliberately NOT reserved" comment). Should-fixes: [#1] restore_marks truthiness
  Java-exact (Boolean true | exact "true"); [#7] gate log carries the full path
  (ConfigError passed through); [#8] dry-run run/execute mint a fresh trace
  (set_from "minigraph.playground" + set_trace(cid, "/graph/playground") — the
  trackable twin; VERIFIED live: 22 /graph/playground telemetry records in the
  playground suite where there were zero); [#12] narration floats keep the trailing .0
  ({spent:?}); [#15/#16] catalog traversal.suspend + Suspend/Resume/Suspensible
  node_types; [#17] the span-topology twin test (forwarder capture; store-under-skill
  parentage + no-suspend-span-on-completed-resume). Nits: cid/ttl raw-untrimmed parity
  ([#2/#3/#9/#13]); [#4] {node}.error stages the extracted error (getError() port);
  [#5] consume-on-retrieve assertion; [#19] help-file naming guard; [#20] tutorial-14
  dry-run wording matches THIS engine's documented repeat-run semantics (+ bundle
  rebuild); [#21] x-run asserted over the REAL HTTP stack (engine test rest.yaml gained
  the /api/graph/{graph_id} route, Java test-config parity). Gate: workspace 296 /
  clippy 0 / fmt. **Pushed on Eric's gate 2026-07-30; PR #186 merged same day — ARC CLOSED.
  Post-merge: the live cross-engine interop drive (Java session, shared
  redis-standalone, two four-run interleavings + rejection probes) PASSED 50/50 — all
  six record handoffs crossed the engine boundary; the permanent report is mirrored in
  BOTH repos (docs/test-reports/suspend-resume-interop.md; Rust branch
  docs/suspend-resume-interop-report, twin of Java fd812f3a — Eric gates both PRs).
  Post-drive touch-ups (Eric, lock-step with Java 11bb7e60): (1) **business cid is
  TRIMMED again** — reverses the fix-round raw-cid ruling (an operator-entered order
  number may carry accidental padding; padding would split the store key space); the
  three cid sites use `java_trim` (Java-exact `<= U+0020`, NOT str::trim — Eric ruled
  either acceptable, the exact predicate kept as a free one-liner) with the blank check
  preserved; unit test = the GraphStateSkillTest twin. The interop report's first
  load-bearing bullet is now "normalized identically" with the adopted-after-the-drive
  note — the INVARIANT is identical normalization, both engines in lock-step.
  (2) **playground default port synced to 8085** (same as Java) so manual engine-swap
  tests reuse one browser URL; swept app config/README/main.rs + 12 help pages + the
  playground-context guides (hello-flow KEEPS 8100 — it pairs with the Java
  composable-example; history/design records untouched); webapp bundle rebuilt.
  **Report branch MERGED 2026-07-30 as
  [PR #187](https://github.com/Accenture/mercury/pull/187), merge commit `42e98bb2`
  (both commits `1e6c8844` + `2f0a4d17` verified on main). The cross-engine
  suspend/resume interop evidence is now permanent on BOTH doc sites (Java twin
  PR #243 in flight). Next arc: the lock-step official release (Eric planning).**
  → serves vision-mercury (the suspension blueprint).
  <!-- id: thread-graph-suspend-resume-p5 | created: 2026-07-29 | last_used: 2026-07-30 | uses: 3 | tier: working | origin: 2026-07-29-235442.md -->

- [x] (release — 2026-07-27; **PUBLISHED 2026-07-26 (Eric confirmed) — CLOSED.
  TAGGED: `v4.10.6` on merge commit `9732799e` (PR
  [#184](https://github.com/Accenture/mercury/pull/184)), the commit VERIFIED as carrying
  the release content before the tag pushed (Cargo.toml 4.10.6 at that commit — the
  4.10.2 tag-race lesson applied); publication pending Eric, who has the release notes.**)
  **v4.10.6 release prep — feature
  release re-aligning the version number with the Java repo (its 4.10.6 = the Sonar
  patch; contents differ by design, Eric's ruling).** Contents: the annotation→macro P2
  leftovers already on main (yaml.preload.override, registration-metadata contract +
  golden vectors, Unicode-scalar string plugins) + the typed-AsyncHttpRequest arc
  (PR #183: typed HTTP functions, single-source dataset, pretty-print parity,
  /info/routes, ops-tunable worker instances). Sweep 4.10.5→4.10.6 (root Cargo.toml —
  count-asserted, no substring hazards; lock regenerated; continuity status line);
  CHANGELOG Unreleased → `## Version 4.10.6, 7/26/2026` + release summary. Gate:
  workspace 273 / clippy 0 / fmt. Tagged + published — CLOSED. Design validation worth
  keeping (Eric's field observation): the pretty-print single-render-path design means one
  presentation fix reached EVERY JSON surface — functions, actuators, flows, AND the
  MiniGraph graph endpoint (live proof: POST /api/graph/tutorial-2 returns pretty JSON) —
  with zero additional code.
  <!-- id: thread-release-4-10-6 | created: 2026-07-27 | last_used: 2026-07-27 | uses: 1 | tier: active | origin: 2026-07-27-011116.md -->

- [x] (2026-07-26; **MERGED same day as PR
  [#183](https://github.com/Accenture/mercury/pull/183), merge commit `1c8fa91b` — the arc
  rode ONE commit (`0b5be4a6`) refined across six review rounds (typed input → fluent
  builder → single-source router → pretty print → /info/routes + default-rest.yaml
  relocation → ops-tunable instances), CI green first try (build 17s / test 1m54s). The
  Java twin PR #236 (worker.instances.actuator.services + truthful docs) still in CI at
  closeout — the shared family key goes live on both engines once it merges.**)
  **Typed AsyncHttpRequest functions (Eric-ratified fix for the gap
  report: the Rust port could not type a function's input as AsyncHttpRequest).** Design
  agreed with Eric — idiomatic, NO engine special case: `Serialize`/`Deserialize`
  hand-implemented as thin delegates onto the existing to_value()/from_value(&Value)
  pair, so `#[preload(..., typed)]` + `TypedFunction<AsyncHttpRequest, O>` flows through
  the ordinary TypedAdapter (`body_as::<I>`) — the knowledge lives on the type (Java, by
  contrast, special-cases AsyncHttpRequest.class in WorkerHandler.getMapBody). Template
  rule for Python/Node ports recorded in the impl comment: request classes constructible
  from the request map make typed signatures just work. Server-dataset diff closed:
  from_value/struct gained ip, https, timeout (route seconds; caller x-ttl wins in
  timeout_seconds()), raw query string — to_value emits what from_value parses
  (round-trip pinned). Accessor audit to Java parity: path_parameter(s),
  query_parameter/query_parameters (single/list), cookie(s), session_info, remote_ip,
  is_secure, query_string, body_as::<T>. hello-world GreetingApi rewritten typed
  (path_parameter replaces Value digging; behavior identical — its e2e passed unchanged).
  Regressions: unit round-trip incl. serde-delegate equivalence + e2e typed probe over
  the real automation server (method/path/query single+list/header/typed body/ip/
  timeout) + client side unchanged (set_raw_body path green). Docs:
  write-your-first-function + getting-started + rest-automation show the typed form as
  canonical (Java SimpleDemoEndpoint pattern), macros-reference typed flag mentions
  AsyncHttpRequest, CHANGELOG Added #7 (also merged the accidental duplicate
  `## Unreleased` sections from the P1/P2 rounds into one). **Eric's review round folded
  in: (1) full fluent-builder symmetry (set_remote_ip/set_secure/set_query_string/
  set_cookie/set_session_info/set_query_parameter_values +
  set_route_timeout_seconds — named distinctly from set_timeout_seconds which stays the
  Java x-ttl mapping; set_query_parameter gains Java put/replace semantics); (2) the
  unit test fluent-built per Eric's verbatim requirement (no serde_json in setup;
  division-of-labor comment: the unit round-trip = internal consistency, the e2e = THE
  server-shape pin); (3) single source of truth: BOTH server dataset-assembly sites
  (main dispatch + static-content filter) now construct through
  AsyncHttpRequest::new()+fluent+to_value() — build_event takes the struct, the binary
  body rides natively (substitution dance deleted), auth session via set_session_info;
  struct went tri-state where the wire distinguishes set-vs-absent (body
  Option<Value> for explicit null, https/trust_all_cert Option<bool>, parameters always
  emitted) so to_value reproduces the server shape EXACTLY — the full suite passed with
  ZERO test churn (272/272).** **Pretty-print parity folded in (Eric): every JSON response
  the automation server writes renders PRETTY (serde_json to_string_pretty = Gson's
  2-space shape) — one shared rule at both render sites (render_payload +
  envelope_payload), covering function responses AND the /info//health//env actuators by
  design (Java's actuators go through the same SimpleMapper default); the HTML <pre>
  shell wraps the same pretty text; /api/event untouched (MsgPack Binary arm). Test churn:
  NONE — no Rust test pinned compact JSON (the suite parses everywhere); two can't-regress
  newline assertions added (typed e2e + /info).** **/info/routes actuator PORTED (Eric;
  his manual typed+pretty test PASSED matching Java's shape): routes.actuator.service
  (exact Java name) + the default rest.yaml entry (GET /info/routes, 10s); response =
  {app, routing.public/private route→instances, BTreeMap-sorted for determinism} — Java's
  optional journal/route_substitution/network blocks omit-when-empty and none of those
  subsystems exist in the port; /info/lib stays deferred (no dependency manifest in a Rust
  binary, Eric concurs). E2E: 200 + pretty + app block + noop.demo public/1 +
  temporary.inbox private/500 + optional blocks absent. Addendum (Eric's review): the
  inline DEFAULT_REST_YAML const relocated to a real resource file
  crates/platform-core/resources/default-rest.yaml embedded via include_str! (the
  default-log-context.yaml pattern — discoverable + byte-diffable vs the Java copy); the
  const's stale "/info/routes deferred" doc line fixed. Zero churn from the relocation.**
  **Ops-tunability round (Eric, the endpoint's first payoff): actuator family 1→5 workers
  with ONE family knob worker.instances.actuator.services (SAME key as Java, whose
  actuators are one aliased class keyed by actuator.services — unported route; resolver
  actuator_instances() shared by lifecycle + tests); hello-world event.api.auth 10→30 +
  worker.instances.event.api.auth (doc: real deployments verify bearer tokens against an
  OAuth2 authority — I/O-bound, higher rule of thumb) and http.request.filter 2→20 +
  worker.instances.http.request.filter. Rules of thumb by design — ops teams tune in
  QA/Perf before production. Knob proven LIVE end-to-end: the actuator suite overrides
  the family key to 7 and /info/routes reports 7 for the family. Docs: actuators tuning
  note + configuration-reference family-key entry (demo keys skipped — the page documents
  no example-app keys); CHANGELOG extended.** Gate: workspace 273 / clippy 0 / fmt.
  Merged — CLOSED. Relates [[registration-metadata-contract]]
  (capability matrix: inputPojoClass N/A because typed I subsumes it — this delivers the
  HTTP-facing half of that story), [[port-bottom-up-faithful]].
  <!-- id: thread-typed-async-http-request | created: 2026-07-26 | last_used: 2026-07-27 | uses: 2 | tier: active | origin: 2026-07-26-233047.md -->

- [x] **Re-verify invariants (due — 40 sessions since the last check ≥ verify_invariants_every
  40).** Raised by the 2026-07-26 review (cadence); **VERIFIED 2026-07-26 against live code,
  authorized by Eric ("proceed with the invariant re-verification when D5 is done").
  Per-fact verdicts: `inv-never-couple-functions` CONFIRMED (route-string + envelope coupling
  only — examples address peers via set_to("route"); the P1/P2 arcs changed registration
  mechanics, not the coupling model; one substance refresh: the ADR-ledger parenthetical now
  reads 0001…0007 adapted + 0008 native). `inv-telemetry-presentation-parity` CONFIRMED and
  STRENGTHENED (rpc-tag one-record-per-span gate, ENGINE_METADATA_KEYS entry scrub + exit
  sanitize, app-log-context default-on gating all live; the ce_traceparent/hygiene rounds
  ended eight-echoes-identical in v4.10.4/5). `port-bottom-up-faithful` CONFIRMED (three
  layers + macro crates + standalone examples; the whole macro arc practiced
  Java-reference-first, map-don't-mirror). `conventions-rust-baseline` CONFIRMED with two
  remedied code drifts (util/mod.rs + automation/mod.rs lacked the Apache header — headers
  added in the verification commit) and one flagged carve-out for Eric: tests/ui compile-fail
  FIXTURES carry no license header (deliberate — .stderr line numbers depend on fixture
  content; treated as test resources, like Java's src/test/resources). Vision
  (`vision-mercury`) CONFIRMED — north star unchanged, 2026-07-21 current-state context still
  accurate, the annotation-macro arc directly serves the template-for-future-ports
  trajectory. No supersessions required.**
  <!-- id: thread-reverify-invariants-2026q3 | created: 2026-07-26 | last_used: 2026-07-26 | uses: 3 | tier: archive-candidate | origin: 2026-07-26-013015.md -->

- [x] (2026-07-26; **ARC COMPLETE — P2 MERGED same day: Rust PR
  [#182](https://github.com/Accenture/mercury/pull/182) (merge `fd4685d5`) in lock-step with
  Java PR #235 (squash `84c4957f`). The full ratified scope is delivered: D1/D2/D3a in P1,
  D4 (yaml.preload.override) + D5 (registration-metadata contract page + byte-identical
  golden vectors + 3 conformance suites + ADR-0008) in P2; D3b/D6 deferred by design. The
  branch also carried: the scalar-semantics ruling ([[string-plugins-unicode-scalars]] —
  f:length/f:substring on Unicode scalar values, UTF-16 retrofit superseded, with the
  anti-re-retrofit guard), the Eric-authorized invariant ceremony (all five never-decay
  facts CONFIRMED against live code, two header drifts remedied), and the docs
  housekeeping (AI-companion test log → docs/test-reports/, joined the site nav).**
  P1 was MERGED earlier the same day: Rust PR
  [#181](https://github.com/Accenture/mercury/pull/181) (merge `ecee2df6`, CI 2m26s — the
  first trybuild CI run passed, .stderr matched stable first try) in lock-step with Java
  PR #234 (squash `265f295d`, CI 6m37s). Remaining = P2: D4 yaml.preload.override port +
  D5 registration-metadata contract page + golden conformance fixture + ADR pair
  (trybuild already landed in P1). **P2/D4 IN PROGRESS 2026-07-26: yaml.preload.override
  PORTED on branch `feature/registration-metadata-contract` (1 commit, NOT pushed)** —
  new platform-core `preload_override` module (Java getPreloadOverride/overrideTasks/
  getMatchedPreload/overridePreloadInfo semantics verbatim: comma-separated locations,
  missing/malformed file logged+skipped whole-file, original/routes/instances/
  keep-original entries, multi-file merge = route-set UNION + first-file-set instances
  wins, match on ANY declared comma-split route, declared list REPLACED by sorted set,
  positive instances replaces the env-resolved count, Java log wording); applied in
  AutoStart between inventory collection and registration, after env_instances
  resolution (the resolved value is the "old" in the log). 7-scenario regression suite
  (rename+fan-out shared handler, keep-original, instances override, multi-file merge,
  missing-file chain, alias-declared original, non-matched untouched). Docs: config
  reference gains the key (removed from the not-read list), macros-reference +
  event-script/syntax.md "not ported" blocks rewritten, CHANGELOG Unreleased. Gate: 266
  (265+1) / clippy 0 / fmt. **P2/D5 DONE 2026-07-26 (2nd+3rd commits on the branch): golden vectors copied
  VERBATIM from Java 73a0d1be (diff-verified byte-identical, all three); fixtures through
  the Rust carriers (core kind incl. one marker deliberately ABOVE #[preload] — the
  conformance fixture itself exercises P1's order-freedom; plugin kind pins BOTH naming
  halves — explicit positional "vectorEcho" on an unrelated fn name + derived
  "vectorDerived" from fn vector_derived, the cross-language name-rule proof; feature kind
  gated-in + gated-out); 3 conformance suites (registration_vectors test binaries, declared
  metadata read straight from the inventory + resolved registration from the platform);
  adapted contract page docs/guides/registration-metadata-contract.md (Rust carrier
  canonical, capability N/As as the port's own note, mkdocs nav under Reference,
  macros-reference cross-linked both ways); ADR-0008 in the Rust ledger (twin of Java
  ADR-0009 — cross-ledger numbering note) formalizing the new Key Decision fact
  [[registration-metadata-contract]].**) **Annotation→macro consistency P1 (ratified arc; design:
  Java repo `draft-design-specs/annotation-macro-interop-design.md`; goal: the Rust macro
  surface reads like the Java annotation surface — the template for future Python/Node
  ports). Java's lock-step half (Platform javadoc + PlaygroundLoader WARN) rides the
  same-named Java branch.** (D1) The engine dogfoods its extension points: all 46 built-in
  mapping plugins converted to `#[simple_plugin(name = "...")]` declarations (bodies
  VERBATIM, twin tests pass unchanged; explicit name= on all 46 — the plugin_* fn prefix
  means camelCase derivation never matches, and keyword-named plugins never become fn
  idents); both built-in fetch features converted to `#[fetch_feature]`;
  builtin_registrations()/register_builtins() deleted; plugins_e8.rs is a proper module
  (include! retired); `extern crate self as ...` in both crates so the macros' absolute
  paths resolve in-crate; registry = one link-time inventory fold (OnceLock) + startup
  count assertions (>= 46 plugins at SimplePluginLoader seq 3, >= 2 features at
  GraphResources) so linker elision fails the boot loudly. (D2) One conflict policy:
  explicit register wins over declarative; duplicate name = WARN + last-wins everywhere —
  Java's exact wordings (Reloading SimplePlugin/FetchFeature {name} - please check
  duplicated ...); websocket adds the same-style WARN; features flip from skip-if-present
  to warn+replace; regressions in 2 dedicated test binaries with a capturing logger (incl.
  user #[simple_plugin] shadowing a built-in — the winner is link-order-dependent like a
  Java classpath scan, the WARN is the contract). (D3a) #[fetch_feature] accepts a stacked
  #[optional_service] (platform strip/fold pattern; FetchFeatureEntry.optional_service;
  is_required at boot; "Skip optional FetchFeature - {name}"); per Eric's ruling
  #[simple_plugin] takes NO optional_service (plugins are flow vocabulary — stated in
  macro docs). Stale docs fixed: syntax.md (#[preload] DOES take comma aliases),
  api-overview.md (public/private IS ported). Docs: macros-reference, design notes,
  CHANGELOG Unreleased, increment 70. Gate: workspace 262 (260+2) / clippy 0 / fmt.
  Two Eric-approved addenda folded in: POSITIONAL #[simple_plugin("name")] form (mirrors
  fetch_feature's grammar; all 46 built-ins flipped; name= stays as alias; no-arg keeps
  camelCase derivation — one grammar, portable to Python/Node decorators) and
  ORDER-INSENSITIVE marker stacking (#[zero_tracing]/#[event_interceptor] are real macros
  via the optional_service self-reattachment pattern — above/below/inline all equivalent,
  Java annotation semantics; no compile-fail harness in the workspace, behavioral
  regressions only at first — Eric then upgraded trybuild to THIS round: 3 tests/ui
  compile-fail suites in the runtime crates, 11 fixtures pinning every deliberate macro
  compile error, .stderr committed; no rust-toolchain pin, so a diagnostics-reshaping
  bump regenerates via TRYBUILD=overwrite). Final gate: 265 (262+3) / clippy 0 / fmt.
  P2 (same arc, gated separately): yaml.preload.override port + contract spec/ADR pair.
  P1 and P2 both merged — ARC CLOSED. Relates [[port-bottom-up-faithful]],
  [[registration-metadata-contract]], [[string-plugins-unicode-scalars]].
  <!-- id: thread-annotation-macro-consistency | created: 2026-07-26 | last_used: 2026-07-26 | uses: 3 | tier: archive-candidate | origin: 2026-07-26-002157.md -->

- [x] (release — 2026-07-24; CLOSED same day) **v4.10.5 SHIPPED AND PUBLISHED in lock-step
  (both repos) — tag `v4.10.5` on merge commit `5ae307c2` (PR
  [#180](https://github.com/Accenture/mercury/pull/180), CI green), release published; the
  Java v4.10.5 published the same day (PR #230, tag on squash `4c82eae0`). Dependabot #16
  CONFIRMED closed by Eric — the security warning is gone. Sixth lock-step release of the
  4.10 arc: 4.10.0 interop → 4.10.1 presentation parity → 4.10.2 boundary demarcation →
  4.10.3 field roll-up → 4.10.4 traceparent carrier + hygiene → 4.10.5 security patch.**
  Prep detail retained: security patch in lock-step with the Java engine's v4.10.5. Content: playground webapp migrated
  react-router-dom ^7.18.1 → react-router ^8.3.0 (coordinator's commit `a2fcb26b`),
  remediating dependabot #16 (React Router RSC Mode CSRF Bypass, follow-up to CVE-2026-22030;
  affected >= 7.12.0 < 8.3.0) — the -dom package ends at 7.18.1 and pins the vulnerable
  react-router exactly; v8 consolidated into the single package. Validation: npm audit 0,
  124 webapp tests, bundle redeployed via npm run release. This closes the dependabot #16
  loop flagged in the v4.10.4 closure push response. Prep commit: sweep 4.10.4→4.10.5 (root
  Cargo.toml; lock regenerated; continuity status line), CHANGELOG
  `## Version 4.10.5, 7/24/2026` Security section. Gate: workspace 260 / clippy 0 / fmt
  (one unidentified single-test failure during the first post-bump compile-storm run; did
  not reproduce across 4 subsequent full runs incl. --no-fail-fast — noted for honesty).
  Tagged + published on both repos — CLOSED.
  <!-- id: thread-release-4-10-5 | created: 2026-07-24 | last_used: 2026-07-24 | uses: 1 | tier: archive-candidate | origin: 2026-07-24-191554.md -->

- [x] (release — 2026-07-24; CLOSED same day) **v4.10.4 SHIPPED AND PUBLISHED in lock-step
  (both repos) — tag `v4.10.4` on merge commit `03424582` (PR
  [#179](https://github.com/Accenture/mercury/pull/179), CI green, 260 tests), release
  published; the Java v4.10.4 published the same day (PR #229, tag on squash `0125c17b`).
  Fifth lock-step release of the 4.10 arc: 4.10.0 interop → 4.10.1 presentation parity →
  4.10.2 boundary demarcation → 4.10.3 field roll-up → 4.10.4 traceparent carrier + interop
  hygiene.** Prep detail below retained: in lock-step with the Java engine's v4.10.4
  (Java PR #228 merge `fcd4fbc1`; Rust PR #178 merge `b58d2163`). Patch release: configurable traceparent carrier (standards-first —
  the optional name is backward-compat-only; standard traceparent wins inbound) + the
  interop hygiene round (clean delivered envelope view, /api/event wire alignment, x-ttl
  ingress parity), validated by the ce_traceparent four-way drive with all eight echoes
  identical (docs/test-reports/event-over-http-interop.md). Sweep 4.10.3→4.10.4 (root
  Cargo.toml only — members inherit; lock regenerated; continuity status line); CHANGELOG
  Unreleased → `## Version 4.10.4, 7/24/2026` + release summary. Gate: workspace 260 /
  clippy 0 / fmt. Tagged + published on both repos — CLOSED.
  <!-- id: thread-release-4-10-4 | created: 2026-07-24 | last_used: 2026-07-24 | uses: 2 | tier: archive-candidate | origin: 2026-07-24-183956.md -->

- [x] (in flight — 2026-07-24; RELEASED same day in v4.10.4, [[thread-release-4-10-4]])
  **Interop header-hygiene round, mirrored from the Java reference branch of the same
  name.** The ce_traceparent interop matrix PASSED (report on `docs/interop-ce-traceparent`,
  pushed as `62210de8`) but exposed pre-existing header-hygiene asymmetries; all fixed.
  (1) **Aligned invariant:** a function's delivered ENVELOPE view never contains my_route/
  my_trace_id/my_trace_path/my_correlation_id/x-event-api — the matrix leak was TRANSPORT (the
  hello-flow demo copied its injected view onto the outgoing envelope), NOT injection: the Rust
  delivery never injected my_* into the envelope view and already removed cid/x-event-api; the
  three my_* keys passed through as transported headers. The worker now scrubs all 5 keys from
  the delivered envelope for NON-interceptor functions (shared ENGINE_METADATA_KEYS with the
  exit filter); interceptors keep raw transport fidelity (also RESTORES Java semantics — the
  port previously stripped cid/x-event-api from interceptor envelopes too); legacy
  my_correlation_id honored-then-scrubbed. (2) Demo: hello-flow EventOverHttpRpc filters the 4
  injected my_* keys from its header-copy loop. (3) Wire hygiene on the /api/event client leg:
  insert-semantics engine stamps (stamp_header, Java http.set — kills the doubled
  x-trace-id/traceparent/custom-name headers); NO x-correlation-id on the transport leg (the
  business cid rides the my_cid tag; leg marked via an HTTP-level x-event-api client
  instruction, consumed by the client + HEADERS_TO_IGNORE — Java's "client-side instruction"
  precedent); + accept:*/* + x-small-payload-as-bytes:true (Java's header set, same order);
  + the 3 startup header-name log lines (Java wording). Wire VERIFIED with a raw header-dump
  listener: single trace headers, no x-correlation-id, full Java set, marker off the wire.
  2 regression twins of Java PostOfficeTest (CleanEnvelopeEcho probe reporting both views).
  Gate: workspace 259 (257+2) / clippy 0 / fmt. Docs: event-over-http engine-internals note,
  CHANGELOG Unreleased Fixed ×3, increment 69. **Matrix re-run VERIFIED the hygiene fixes
  independently (all 8 runs leak-free, wire = exact Java header set, startup parity line,
  cross-language span parenting intact); ONE residual fixed as a 2nd commit: the endpoint
  timeout now rides the request dataset as the x-ttl header (ms, Java setTimeoutSeconds
  semantics; caller-sent value WINS — route-wins broke the /api/event in-band remote-timeout
  race, caught by remote_timeout_arrives_in_band). Regression
  endpoint_timeout_rides_the_dataset_as_the_x_ttl_header; gate now workspace 260 / clippy 0 /
  fmt.** **FINAL RULING (Eric) applied as a 3rd commit, mirroring Java `5401f1f8`: inbound,
  the STANDARD traceparent always wins; the custom name is a fallback only when the standard
  is absent/malformed (a well-formed standard traceparent = the legacy system already
  upgraded; residual proprietary header safely ignored — self-consistent with the
  trace.id.header fallback). Precedence regression inverted
  (standard_traceparent_wins_over_custom_header_name); gateway-simulation test unchanged
  (proves the fallback); all "custom name first" doc phrases flipped; outbound dual stamping
  unchanged. Gate: workspace 260 / clippy 0 / fmt.** **MERGED 2026-07-24 as PR #178 (merge `b58d2163`),
  lock-step with Java PR #228 (merge `fcd4fbc1`); ships in v4.10.4
  ([[thread-release-4-10-4]]) — close with that release.** Relates
  [[inv-telemetry-presentation-parity]], [[thread-configurable-traceparent]].
  <!-- id: thread-interop-header-hygiene | created: 2026-07-24 | last_used: 2026-07-24 | uses: 2 | tier: archive-candidate | origin: 2026-07-24-173129.md -->

- [x] (in flight — 2026-07-24; **MERGED same day as PR #177**, merge commit `e99013cb`; the
  ce_traceparent interop matrix verified the custom carrier cross-language; **RELEASED in
  v4.10.4 with the standards-first precedence flip — CLOSED**) **Configurable traceparent header name (field request), mirrored from
  the Java reference in lock-step** (mercury-composable branch of the same name, commit
  `5ee496dd`). `http.traceparent.header` (default `traceparent`) + per-entry
  `traceparent.header` in rest.yaml (precedence per-entry > global > default). Inbound
  (REST automation): parse the custom name FIRST — a well-formed value under it wins, so a
  sidecar-injected standard `traceparent` cannot override the peer's context — standard header
  as fallback for compliant callers. Outbound (async HTTP client + the Event-over-HTTP encode
  path): the same W3C value stamped under BOTH names when they differ (case-insensitive).
  Escape hatch for a gateway allow-list that strips the standard header — the full W3C context
  crosses, so cross-app span parenting survives (beats trace-id conflation). Java's Kafka twins
  (`kafka.traceparent.header` / `secondary.kafka.traceparent.header` + the adapter per-binding
  override) have NO Rust surface (mesh not ported) — skipped per the mirror-what-exists rule.
  Test pattern mirrored: suite-wide `http.traceparent.header=X-Trace-Context` in
  tests/resources/application.properties (the whole platform-core suite runs feature-active,
  proving it additive) + 5 regression twins of the Java tests (custom carries context / custom
  wins over injected standard / standard fallback / per-entry override / dual-stamp
  echo-chain). Gate: workspace 257 (252+5) / clippy 0 / fmt. Docs: config reference,
  observability (impedance row + "renamed traceparent beats conflation" tip), rest-automation
  grammar, reserved-names, HTTP-client guide, CHANGELOG Unreleased, increment 68. Close when
  merged (+ released in the next lock-step patch). Relates [[inv-telemetry-presentation-parity]].
  <!-- id: thread-configurable-traceparent | created: 2026-07-24 | last_used: 2026-07-24 | uses: 3 | tier: archive-candidate | origin: 2026-07-24-160634.md -->

### Blueprint — gaps from Current State (three layers shipped + graduated) to the Vision  (serves: vision-mercury)
> Derived 2026-07-15 from the maintainer-set Vision. Each `(blueprint)` thread is a
> Vision↔reality gap that closes when delivered. Bottom-up order (foundation → UI). The
> three in-scope layers are delivered; forward runway is the UI continuation + the
> connectors/sync-over-async backlog below. The authoritative behavior spec remains the
> Java mercury-composable project (map, don't mirror).

- [ ] **(blueprint)** Continue **foundation → user interface** now that the three core
  layers stand — reframe into concrete UI-layer increments as they are picked up.
  → serves: vision-mercury
  <!-- id: bp-foundation-to-ui | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(backlog) Port the lightweight cloud-native connectors + sync-over-async.** Maintainer
  scope refinement 2026-07-20 (stated while reviewing the docs site): `minimalist-kafka` and
  `twin-kafka` are **lightweight, cloud-native connectors** — distinct from the Kafka service
  mesh (service discovery + sync-over-Kafka, which stays permanently out of scope) — and will
  be ported in future iterations, along with **`sync-over-async`** (the request/response
  bridge over async transports). This is also why HTTP config keys keep their `http.` prefix
  (connector counterparts arrive later). Vision non-goals + instructions + the public
  `docs/background/port-scope.md` all updated to the refined wording. → serves: vision-mercury
  <!-- id: bp-kafka-connectors-backlog | created: 2026-07-20 | last_used: 2026-07-28 | uses: 4 | tier: working | origin: 2026-07-20-030615.md -->

- [x] **(backlog — DELIVERED 2026-07-28: increment 71 MERGED as PR
  [#185](https://github.com/Accenture/mercury/pull/185), merge commit `6326da2e`, CI green
  first try (build 25s / test 1m53s). CLOSED.) Port `ManagedCache` — ONE cache type
  (Eric's ruling 2026-07-27: do NOT
  port `SimpleCache`; "just adopt a proper self-expiring in-memory cache").** Java
  platform-core ships `org.platformlambda.core.util.ManagedCache` — a named, self-managing
  TTL+size-bounded cache utility (Caffeine: `expireAfterWrite`, `maximumSize`, default
  2000 items, min TTL 1s; static registry createCache/getInstance/getCacheCollection).
  NOT ported — Rust platform-core has no cache utility; current stand-ins are ad-hoc
  (playground WS dedup = unbounded `Mutex<HashMap>` in `commands.rs::is_duplicate`;
  fetcher provider cache = per-instance state in BOTH engines, so not affected). Any Java
  `SimpleCache` site ported later (e.g. the actuator's per-dependency `health.info` 5s
  **info-lookup** cache — Java never caches the /health result itself; a known Rust parity
  gap) maps onto a `ManagedCache` instance (bounded + self-expiring is a
  superset of SimpleCache's unbounded lazy-expiry; document the mapping once). Needed for:
  the future connectors port ([[bp-kafka-connectors-backlog]] — minimalist-kafka's
  schema-registry client is a heavy ManagedCache user) and Java-API-surface completeness.
  Engine candidate per the ruling: a maintained self-expiring implementation (`moka`, the
  Rust Caffeine analog) rather than hand-rolled expiry; the WS dedup cache adopts it and
  gains bounded eviction. **GATE APPROVED 2026-07-27 (Eric: moka confirmed, ALL THREE
  adopters, test constructor ok) + a 4th ruling at approval: DETERMINISTIC EVICTION —
  moka `EvictionPolicy::lru()`, a documented divergence from Java Caffeine's W-TinyLFU
  (verified genuinely non-deterministic: frequency admission + deliberate HashDoS jitter,
  no policy switch; refactoring note handed to the Java session via
  /tmp/managed-cache-eviction-determinism-handoff.md). Increment 71 IMPLEMENTED on branch
  `feature/managed-cache` (commit `a2f89856`, NOT pushed): module + registry + lifecycle
  housekeeper + all 3 adopters (WS dedup anchored-window fix; actuator `health.info`
  info-lookup cache; ext-state-machine fixture restored to Java's singleton
  instances=1 — the pre-commit adversarial review caught a CRITICAL lost-update race the
  old global Mutex had masked, plus the elapsed_time whole-unit divergence [now an exact
  Utility.elapsedTime port, also fixing /info uptime] and a doubled duplicate debug log;
  12 confirmed findings, all fixed). Design doc marked APPROVED. Gate: workspace 287 /
  clippy 0 / fmt; flow_runtime 5/5. MERGED — CLOSED. The connectors-era follow-on
  (schema-registry adopting ManagedCache) rides [[bp-kafka-connectors-backlog]].
  **Java handoff RESOLVED 2026-07-27: Eric ruled option 1 (accept + document), shipped as
  Java PR #237 (squash `8a81950c`, CI green) — javadoc on both createCache overloads
  (size-eviction approximate + non-deterministic under maxItems pressure; expiry exact;
  never rely on which entry survives; explicit note that the Rust port deliberately
  differs with strict LRU) + CHANGELOG. Copilot's "frequency aging" remediation ruled a
  category error (caffeine-3.2.4's FrequencySketch already has reset()/RESET_MASK — aging
  is built in; it fixes stale popularity, not reproducibility; the admit() jitter is
  orthogonal). Parity boundary confirmed: eviction is internal state, NOT a presentation
  surface — the asymmetry (Rust strictly more predictable) is documented, not closed.
  Revisit trigger: first consumer that truly runs at capacity.**
  → serves: vision-mercury
  <!-- id: ot-managedcache-port | created: 2026-07-21 | last_used: 2026-07-28 | uses: 4 | tier: active | origin: 2026-07-21-030938.md -->

- [x] **(knowledge-harvest — CLOSED 2026-07-27 by Eric's ruling: per-layer harvest COMPLETE;
  the connectors/sync-over-async harvest rides [[bp-kafka-connectors-backlog]].)
  Harvest the canonical vision/specs from mercury-composable (Java).**
  **Gate satisfied 2026-07-15** — the maintainer added `~/sandbox/mercury-composable` and
  authorized reading it (read-only reference). **Harvested this session:** the north-star
  vision (AKG-is-the-application / AI-assisted Semantic Application Development), the accurate
  three-layer model, platform-core's architecture (functions/route-name/`EventEnvelope`/
  `PostOffice`/`Platform`/in-memory bus, virtual-thread execution, lifecycle), the module map,
  and the canonical version (4.8.6) — folded into vision/instructions/invariants above.
  **Still to harvest** (as each layer is ported): platform-core internals (EventEmitter,
  WorkerHandler, serializers), then event-script and knowledge-graph specs + their ADRs.
  **Review note (2026-07-27):** the enumerated per-layer harvest is effectively complete —
  all three in-scope layers are ported/milestone-closed and their specs folded into
  vision/instructions/invariants/ADRs. Remaining forward scope is the connectors +
  sync-over-async harvest (rides [[bp-kafka-connectors-backlog]]). **Human gate:** mark
  this `[x]` (per-layer harvest done) or re-scope it to the connectors harvest — Eric's call.
  → serves: vision-mercury
  <!-- id: ot-harvest-mercury-composable | created: 2026-07-15 | last_used: 2026-07-27 | uses: 3 | tier: active | origin: 2026-07-15-215538.md -->

## User Preferences

(none recorded yet — record ONLY what the user explicitly states; never infer)

## Team / Members

(none recorded yet)
