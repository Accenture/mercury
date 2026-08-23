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
- **status:** **Rust port of `mercury-composable`** (canonical Java v4.8.6), delivered bottom-up; all three in-scope layers (platform-core, event-script, active knowledge graph + Playground) ported and milestone-closed, **GRADUATED to github.com/Accenture/mercury 2026-07-20** (docs at accenture.github.io/mercury; regular PR process). Kafka service mesh + Spring out of scope. Current release **v4.11.10** (version tracks the Java line, contents by design). History/detail lives in `docs/INCREMENTS.md` (increment ledger), `docs/design/`, session logs, and CHANGELOG — not this line.
- **last_enabled:** 2026-07-15
- **last_session:** 2026-08-22 | agent: Claude Code (2026-08-22-185217)
- **last_review:** 2026-08-14 | through 2026-08-14-005444.md
- **last_invariant_check:** 2026-07-26 | 2026-07-26-014908.md (all five never-decay facts confirmed against live code — inv-never-couple-functions, inv-telemetry-presentation-parity, port-bottom-up-faithful, conventions-rust-baseline, and the Vision; two header drifts remedied; ui-fixture carve-out RATIFIED by Eric 2026-07-26)
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

- **Port bottom-up, faithfully to the Java original** — re-implement mercury-composable in
  Rust layer by layer, foundation → UI (platform-core, then event-script, then active
  knowledge graph), preserving the Java project's behavior. The Java repo is the canonical
  spec (map, don't mirror).
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-08-14 | uses: 100 | tier: active | origin: 2026-07-15-215538.md -->
## Conventions

> Established with the first code (increment 1, 2026-07-15); enforced from the first commit.

- **Suspend/resume is CORE functionality for a few field installations — this surface
  is regression-critical on BOTH engines (Eric, 2026-07-30, at the v4.11.0 publication).**
  Operating rule: behavior changes to the suspend/resume surface (the two skills, the
  walkers' suspension routing, the store contract, the reserved keys/normalization, the
  gate's suspend rules, reply shapes and presentation) get RELEASE-LEVEL care — regression
  suites on both engines, cross-engine interop verification when the wire contract is
  touched, and lock-step shipping. The permanent baseline is the interop report
  (docs/test-reports/suspend-resume-interop.md) + the twin test suites.
  <!-- id: conv-suspend-resume-regression-critical | created: 2026-07-30 | last_used: 2026-08-01 | uses: 2 | tier: archive-candidate | origin: 2026-07-30-181400.md -->

- **The Java repo's helper servers are the standard local test servers for Rust ports
  (Eric, 2026-07-30).** `helpers/redis-standalone` for the suspend/resume arc;
  `kafka-standalone` + the schema-registry-mock when minimalist-kafka is ported. WHY:
  the helpers embed REAL redis/kafka servers behind a plain `java -jar`, motivated by
  field reality — many developer machines are Windows, especially VDI environments with
  no virtualization system, so Docker is unavailable; a jar works everywhere. Tier: unit
  tests may use fast hermetic in-process doubles (e.g. the RESP2 test double — the
  double stands in for the SERVER, never the client); the helper is the
  integration/live-drive tier.
  <!-- id: conv-java-helper-servers-for-rust-tests | created: 2026-07-30 | last_used: 2026-07-30 | uses: 4 | tier: archive-candidate | origin: 2026-07-30-015038.md -->

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
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-08-14 | uses: 107 | tier: active | origin: 2026-07-15-224707.md -->

## Open Threads

- [x] (feature — **MERGED 2026-08-22 as
  [PR #212](https://github.com/Accenture/mercury/pull/212), merge `c49d6cd7` carrying
  `83b12c36` (tree verified), CI green (test 2m26s + the new agent-memory check); branches
  deleted both ends; rides the next release** via CHANGELOG Unreleased) **graph.task reaches declarative event-over-http targets — the polyglot
  initiative's only engine change (D5), lock-step with the Java engine (commit `10c53ca3`
  there, same day).** Guard in skills.rs consults `event_api::get_event_http_target`;
  unit-test-task-7 pin (fixture byte-identical to Java, stub /api/event peer, proven
  failing against unfixed code); compiled-set pin 49→50; workspace 63 suites + clippy 0 +
  fmt clean. The initiative's design record lives in the Java repo's memory
  (polyglot-event-over-http-design); wrapper repos mercury-python/mercury-nodejs carry
  their own memory. Full detail: origin log.
  <!-- id: thread-graph-task-event-over-http | created: 2026-08-22 | last_used: 2026-08-22 | uses: 1 | tier: working | origin: 2026-08-22-185217 -->

> Mark completed items `- [x]` and leave them in place — the review sweeps them to
> the archive once older than `archive_window` sessions. Don't archive them by hand.

- [x] (release — SHIPPED AND PUBLISHED 2026-08-21 local, **both repos in lock-step at
  v4.11.10**; both GitHub releases published by Eric) **v4.11.10 — the AI discovery release.**
  Rust: move PR #210 (`examples/`→`system/ai-contract-provider`, Eric's consistency ruling,
  merge `9d1e4c28` tree-verified) then release PR #211 merge `b77f17e8` carrying `1beff96d`
  (tree verified), gate 63/317 + clippy 0 + fmt, tag on the merge, dereference-verified.
  Java: release PR #291 squash `5cb65f04` == gated `689adf5e`, 34-pom sweep, full reactor
  green, tag on the squash, dereference-verified. Contents: f:setConfig +
  system/ai-contract-provider (+ Java: OTLP fixes, flow-binding docs fix).
  Lesson: content merges before the mechanical release PR — the Rust release branch was
  discarded and recreated on top of the move. Full detail: origin log.
  <!-- id: thread-release-4-11-10 | created: 2026-08-22 | last_used: 2026-08-22 | uses: 1 | tier: working | origin: 2026-08-22-032041 -->

- [x] (feature — lock-step port of the Java engine's AI discovery app, implemented and
  **MERGED 2026-08-22 as mercury PR #209, merge `c0f5245e` carrying `30497961` (tree
  verified identical), CI green (test 2m18s + recheck), branches deleted both ends; rides
  the next release via CHANGELOG Unreleased**) **`system/ai-contract-provider` — version-matched operational contract for AI
  discovery.** (Moved from examples/ to system/ 2026-08-21 at Eric's release review —
  same path as the Java module, completing the cross-repo `system/` convention;
  pure relocation, INCREMENTS 89.) Six REST endpoints on 8999 + `--export` offline skill; the seven flow YAMLs
  are BYTE-IDENTICAL to the Java app's and ran unchanged (portability proven on a whole
  app's orchestration). Adaptations: compile-time anchor verification (Class.forName
  analog; knowledge-graph dev-only), build.rs-embedded snapshot from files.list (missing
  doc = build failure), workspace-pinned version (mixed assembly structurally impossible),
  packaged references/llms.txt (self-contained, replaces Java's link rewrite). Contract ids
  identical to Java. Also: `system/AGENTS.md` consumer guide (same path convention as the
  Java repo, Eric's ruling) + root AGENTS.md fork with the role-resolution ladder +
  llms.txt/getting-started discovery entries. 11 tests across 4 binaries; CLI export
  proven live (43 files, hashes independently re-verified). Gate: 63 suites / 317 tests,
  clippy 0, fmt clean. CHANGELOG Unreleased; INCREMENTS 88. Origin log has the
  `#[path]`-inclusion super::-imports gotcha.
  <!-- id: thread-ai-contract-provider-port | created: 2026-08-22 | last_used: 2026-08-22 | uses: 1 | tier: working | origin: 2026-08-22-023108 -->

- [x] (feature — lock-step with the Java engine, implemented and **MERGED 2026-08-21 as
  mercury PR #208, merge `9a7b3a47` carrying `338fc895` (tree verified identical), CI green
  (test 2m20s + recheck), branches deleted both ends; the Java half merged the same day as
  [PR #289](https://github.com/Accenture/mercury-composable/pull/289) squash `b5aeaf56`,
  tree verified. Both ride the next release via CHANGELOG Unreleased.**) **Event Script
  `f:setConfig` simple plugin — set/override a config parameter at run-time via the process
  override registry (`overrides::set`, the System.setProperty analog).** Key = non-empty
  string; value = any object → `get_text_value` (Java String.valueOf); invalid input →
  false without side effect. BUILTIN_PLUGIN_COUNT 46→47; the loaded-flow-set parity pin
  gained `set-config-parameter`; flow fixture BYTE-IDENTICAL to Java's set-config.yml
  (set in task one, `map(key)` read-back in task two, runtime-asserted); unit twin of
  SetConfigParameterTest. Docs: syntax.md catalog row + configuration override detail
  (Rust wording: override registry / -D args); CHANGELOG Unreleased; INCREMENTS 87.
  Gate: 58 suites / 306 tests, clippy 0, fmt clean. Java half: mercury-composable branch
  `feature/config-plugin` (same day; Eric's plugin, reviewed + ruled there).
  <!-- id: thread-set-config-plugin | created: 2026-08-21 | last_used: 2026-08-21 | uses: 1 | tier: working | origin: 2026-08-21-234417 -->

- [x] (release — SHIPPED AND PUBLISHED 2026-08-10 local / 2026-08-11 UTC, **both repos in
  lock-step at v4.11.8**) **v4.11.8 — the dry-run suspend/resume regression-fix release.** Rust: release
  PR #205 merge `d16d68f0` carrying `5b659e50` (tree verified), CI green (test 2m34s +
  recheck), Cargo.toml 4.11.6→4.11.8 + Cargo.lock + CHANGELOG (with the v4.11.7-was-Java-only
  step note), gate 58/305 + clippy 0 + fmt clean, tag `v4.11.8` on the merge,
  dereference-verified. Java: [PR #279](https://github.com/Accenture/mercury-composable/pull/279)
  squash `92dd64a8`, tag on the squash. Sole content:
  [[thread-dry-run-graph-scope-fix-rust]] (PR #204). PUBLISHED by Eric 2026-08-10.
  <!-- id: thread-release-4-11-8-rust | created: 2026-08-11 | last_used: 2026-08-11 | uses: 2 | tier: active | origin: 2026-08-11-051701 -->

- [x] (release — SHIPPED AND PUBLISHED 2026-08-11, **both repos in lock-step at v4.11.9**; cut
  FOR FIELD DEPLOYMENT) **v4.11.9 — the dry-run graph identity simplification.** Rust: release PR #207 merge
  `27fa527e` carrying `40f99dc8` (tree verified), CI green (test 2m20s), Cargo.toml + lock +
  CHANGELOG, gate 58/305 + clippy 0 + fmt clean, tag `v4.11.9` on the merge, dereference-verified.
  Java: [PR #281](https://github.com/Accenture/mercury-composable/pull/281) squash `eff46c5f`,
  tag on the squash; its prep added a live drive of both the named and `untitled` paths against
  the built artifacts. Sole content: [[thread-untitled-dry-run-identity-rust]] (PR #206).
  Both GitHub releases PUBLISHED by Eric 2026-08-11.
  <!-- id: thread-release-4-11-9-rust | created: 2026-08-11 | last_used: 2026-08-11 | uses: 1 | tier: working | origin: 2026-08-11-220612 -->

- [x] (fix — **MERGED 2026-08-11 as mercury PR #206, merge `3bdcd3b3` carrying `a901b1b7`
  (tree verified), CI green (test 2m17s); Java twin
  [PR #280](https://github.com/Accenture/mercury-composable/pull/280) squash `68cd9d28`;
  rides the next release.**) **Dry-run graph identity simplified: an
  unnamed draft is scoped `untitled` instead of rejected.** The store contract needs the dry-run
  identity to be STABLE ACROSS INSTANTIATIONS, not derived from the model name — so v4.11.8's
  rejection guard was only defending against its own ephemeral `playground-{uuid}` fallback.
  `stable_graph_identity` → root name or `const UNTITLED`; guard, `root_name`, `uses_suspension`
  and the orphaned mirror test deleted. New twin pin
  (`companion_unnamed_draft_resumes_across_instantiations`, edge-mode draft sketched via companion
  commands) is **mutation-proven** against a per-instantiation handle. CHANGELOG `## Unreleased`
  (published v4.11.8 section untouched). Gates: 58/305 + clippy 0 + fmt clean.
  Relates [[thread-dry-run-graph-scope-fix-rust]].
  <!-- id: thread-untitled-dry-run-identity-rust | created: 2026-08-11 | last_used: 2026-08-11 | uses: 1 | tier: working | origin: 2026-08-11-220612 -->

- [x] (fix — **MERGED 2026-08-11 as mercury PR #204, merge `f5256ecc` carrying `8fc45b94`
  (tree verified identical), CI green (test 2m8s + authoritative recheck); Java twin
  [PR #278](https://github.com/Accenture/mercury-composable/pull/278) `573c62aa`;
  rides the next release.**) **v4.11.6 regression: dry-run suspend/resume
  never resumed — the playground lane's ephemeral `playground-{uuid}` graph id broke the
  graph-scoped store key** (`graph:{graph_id}:{cid}` never matched across instantiations;
  executor lane unaffected — stable deployed id). Fix: dry-run identity = root node's `name`
  property; **unnamed root + suspend/resume model REJECTED at instantiation with a teaching
  message (Eric's ruling — a silent fallback would break resume invisibly)**; guard-first (no
  side effects on rejection). Pins: resume-across-instantiations (store_file key pin + step
  counters + consume-on-retrieve) + the rejection; pre-run-check scratch graph gained a root
  name. Gates: 58/305 + clippy 0 + fmt clean. Porting gotcha recorded: scripted guard landed at
  the wrong `remove_instance` occurrence first — anchor on unique context.
  Relates [[thread-graph-scoped-state-and-error-context-rust]].
  <!-- id: thread-dry-run-graph-scope-fix-rust | created: 2026-08-11 | last_used: 2026-08-11 | uses: 2 | tier: active | origin: 2026-08-11-051701 -->

- [x] (release — SHIPPED AND PUBLISHED 2026-08-10, **both repos in lock-step at
  v4.11.6**) **v4.11.6 — the field-review follow-ups release.** Rust: release PR #203 merge `c008d11b` carrying
  `a3ae466f` (merge tree verified identical to the gated commit), CI green (test 2m18s +
  authoritative recheck), workspace Cargo.toml + Cargo.lock + CHANGELOG cut, gate =
  58 suites / 305 tests + clippy 0 + `cargo fmt --check` (exit codes verified unpiped),
  tag `v4.11.6` on the merge, dereference-verified. Java:
  [PR #275](https://github.com/Accenture/mercury-composable/pull/275) squash `c29915ee`,
  tag on the squash. Contents: graph-scoped workflow state (BREAKING store key
  `graph:{graph_id}:{cid}` — the CHANGELOG's `### Changed` LEADS with the upgrade note)
  + generic exception context incl. recovery + orchestrator pattern + dynamic statement
  variables. Both GitHub releases PUBLISHED by Eric 2026-08-10.
  <!-- id: thread-release-4-11-6-rust | created: 2026-08-10 | last_used: 2026-08-10 | uses: 1 | tier: active | origin: 2026-08-10-224037 -->

- [x] (feature+fix — **MERGED 2026-08-10 as mercury PR #201, merge `354c1134`** carrying
  `7d2da900`, CI green on the first run incl. the Format check (test job 2m9s); mirrors
  the Java engine's same-day fix, merged as
  [PR #273](https://github.com/Accenture/mercury-composable/pull/273) squash `96d9c35f`
  — **COMPLETE ON BOTH ENGINES**; rides v4.11.6. Increment 85.) **Dynamic
  variables in every statement command — completing the generic error handler — PLUS the
  recovery semantics follow-up (**MERGED as PR #202, merge `213b739a`** carrying
  `6c7cf134`, CI green; Java twin
  [PR #274](https://github.com/Accenture/mercury-composable/pull/274) squash `5a01c0c6`;
  Increment 86 — shipped in v4.11.6 same day, see [[thread-release-4-11-6-rust]]): a successful retry of error.source
  RESOLVES the virtual 'error' node (code=200, source kept, details removed; source match
  keeps parallel branches safe) — pinned by unit-test-error-recovery + a tutorial-12
  companion dry-run; 58/305 + clippy + fmt.** Eric's
  regression pass found RESET:/NEXT: took targets literally; now NEXT:/THEN:/ELSE:
  targets, RESET: entries and DELAY: values resolve {namespace.key} at execution time
  (`get_next_tag_resolved` + per-tag substitution in skills.rs; unresolved → "null":
  RESET no-op, DELAY skipped, jump fails loudly). No graph.js on this engine (retired) —
  math executor only. tutorial-12 genericized (fixture byte-identical; help adapted in
  the port's structure); unit-test-dynamic-jump pins THEN:/DELAY:. Gate: 58 suites /
  305 tests, clippy clean, fmt clean (verified by real exit code — a piped `head` masked
  the first check), webapp 212/212.
  Relates [[thread-graph-scoped-state-and-error-context-rust]].
  <!-- id: thread-dynamic-statement-targets-rust | created: 2026-08-10 | last_used: 2026-08-11 | uses: 2 | tier: active | origin: 2026-08-10-224037 -->

- [x] (feature — **MERGED 2026-08-10 as mercury PR #200, merge `283d41e2`** carrying
  `24eeef89` + the `cargo fmt` follow-up `7dadd1ff` — the first CI run FAILED the
  Format check because the scripted test edits weren't rustfmt-clean (this repo's gate
  is tests + clippy + FMT; run all three locally); CI green on re-run (test job 2m25s).
  Mirrors Java
  [PR #271](https://github.com/Accenture/mercury-composable/pull/271) squash `adfb2a0d`
  + polish PR #272 squash `0612ec6d`, both merged same day — **COMPLETE ON BOTH
  ENGINES; both ride the next release.** Rust ADR-0012/ADR-0013 accepted via the merge.)
  **Graph-scoped workflow state + generic exception context (field
  review follow-ups), Increment 84.** The suspend/resume store contract is scoped by
  graph + cid: envelope {cid, graph, node, ttl, model, seen, run}, get body {cid, graph},
  Redis key `graph:{graph_id}:{cid}` (formerly `graph:state:{cid}` — BREAKING, flag-day
  per Eric's R1; both store functions reject a missing graph; version-aware
  GETDEL/MULTI-EXEC consume unchanged, re-proven on the RESP double).
  `graph.extension`'s `build_forward` stamps the caller's model.cid as the
  `correlation_id` header (Event Script sub-flow parity) — the orchestrator pattern
  (parent delegating independently resumable subgraph paths) pinned by the
  byte-identical unit-test-orchestrator/unit-test-sub-suspend pair + per-graph isolation
  scenario. Generic exception context: `stage_error_context` in common.rs staged at both
  walker choke points (error.source/code/message; **error.stack only when a record
  carries one — this engine has NO native stack-trace transport, a documented port
  divergence**); 'error' was always reserved in the graph model (RESERVED_NAMES) so no
  gate change; `inspect error` works by construction; the alias fixture joins the
  compiled-or-404 negatives (compiler counts 47 valid / 14 invalid). Rust ADR-0012 +
  ADR-0013 proposed (Java twins ADR-0013/ADR-0014 — the numbering skew continues). Gate:
  58 suites / 305 tests green, clippy clean, webapp 212/212 (bundle regenerated for six
  help pages — three byte-copied, three adapted to port variants).
  <!-- id: thread-graph-scoped-state-and-error-context-rust | created: 2026-08-10 | last_used: 2026-08-11 | uses: 2 | tier: active | origin: 2026-08-10-190550 -->

- [x] (release — SHIPPED 2026-08-09, **lock-step with the Java engine at v4.11.5**)
  **v4.11.5 — the graph.task parity + teaching-surfaces release.** Release PR #199 merge
  `4380e29d` (workspace 4.11.4→4.11.5 + Cargo.lock, 305 tests green) + the post-merge
  diagram patch `82b020e6` straight to main; tag `v4.11.5` MOVED pre-publication onto
  the docs-inclusive `82b020e6` (the standing ruling), dereference-verified. Contents:
  [[thread-graph-task-model-staging-rust]] (incl. the default `Accept: */*` client
  ruling), tutorial-13 by configuration, the checkpoint/decision docs reframe. Java
  twin: PR #270 squash `9a6a9569` + patch `f8dd9cd7`, tagged.
  <!-- id: thread-release-4-11-5-rust | created: 2026-08-09 | last_used: 2026-08-09 | uses: 1 | tier: working | origin: 2026-08-09-164019 -->

- [x] (lock-step — **MERGED 2026-08-08 as mercury PR #197, merge `79212bc0` carrying
  `0530bd13`, CI green; rides the next release**; mirrors the Java engine's
  [PR #267](https://github.com/Accenture/mercury-composable/pull/267), squash `e16f4b40`)
  **graph.task `model.*` input staging (Event Script parity) + tutorial-13 as an HTTP
  client by configuration + the default-Accept client ruling.** stage_model_variable in
  skills.rs (guarded model.* RHS → state machine); tutorial-13/help byte-identical to
  Java (async.http.request, dynamic variables, ${...} load-time substitution, explicit
  headers.accept + headers.x-ttl); v1.hello.task mock retired; unit-test-task-6 gate
  negative; playground dry-run twin (ephemeral-port harness re-points the
  rest.server.port override at the bound port). **Eric's ruling: the async HTTP client
  sends a default `Accept: */*` when the caller gives none** (Java reactor-netty parity;
  both REST edges omit response content-type absent Accept, so the same model previously
  decoded JSON on Java and returned bytes here); explicit accept never overridden,
  wire-echo pinned both ways. Also repaired the INCREMENTS ledger (78/79 reconstructed,
  tail re-ordered 76→83, Overview extended). Increment 83.
  <!-- id: thread-graph-task-model-staging-rust | created: 2026-08-08 | last_used: 2026-08-09 | uses: 1 | tier: active | origin: 2026-08-09-043233 -->

- [x] (release — SHIPPED AND PUBLISHED 2026-08-08, **lock-step with the Java engine
  at v4.11.4**) **v4.11.4 — the suspend/resume rationalization release.** Release
  PR #196, merge `27c2cc8e`, CI green; workspace 4.11.1→4.11.4 + Cargo.lock refresh
  (the CHANGELOG notes 4.11.2/4.11.3 were Java-only releases); 58 suites green; tag
  `v4.11.4` on the verified merge, dereference-verified; published by Eric. Contents:
  [[thread-suspend-resume-rationalization-rust]] (ADR-0011) incl. the webapp UI
  refresh from the Java repo. Java twin: PR #266, squash `ad60f7e4`, tag verified.
  <!-- id: thread-release-4-11-4-rust | created: 2026-08-08 | last_used: 2026-08-08 | uses: 1 | tier: archive-candidate | origin: 2026-08-08-022929 -->

- [x] (lock-step — **MERGED 2026-08-08 as mercury PR #195, merge `4e6bdf43` carrying
  commit `995cfeb7` with its single co-author trailer, CI green; mirrors the Java
  reference engine's PR #265 (squash `392f7128`, ADR-0012); both engines identical —
  rides the next release via CHANGELOG Unreleased / INCREMENTS 82**)
  **Suspend/resume rationalization: suspension is a destination — edge/jump modes
  replace `suspend=true` (this port's ADR-0011, amending ADR-0009).** Edge mode = drawn
  edge + mandatory continuation, resume continues past, never re-executes (back-compat
  exact; property = deprecation-WARN no-op). Jump mode = graph.math IF-THEN-ELSE jump,
  RE-EXECUTED on every resume (wait loop, no RESET); routing-skill drawn edge to suspend
  + exception=suspend rejected with Java-exact teaching errors; jump-only suspend is
  island-anchored (island exempt from the continuation-edge rule). tutorial-14 +
  fixtures byte-identical from Java (await-decision/RESET gone); jump-mode + compat
  scenarios added to graph_runtime; knowledge-graph 9 suites (44-graph gate), playground
  e2e, fmt, clippy 0. **Webapp REPLACED from the Java repo's latest UI source (Eric's
  directive — brings the Java PR #262 UI work over), port path adaptations re-applied,
  webapp 212/212, bundle index-DqzF65vX.js.** Record/store contracts unchanged.
  Relates [[thread-tutorial-14-decision-rust]].
  <!-- id: thread-suspend-resume-rationalization-rust | created: 2026-08-08 | last_used: 2026-08-08 | uses: 2 | tier: archive-candidate | origin: 2026-08-08-005419 -->

- [x] (lock-step — **MERGED 2026-08-07 as PR #193, merge `bea95c80` carrying commit
  `8162b733`, CI green; same-day mirror of Java PR #263; rides the next release via
  INCREMENTS 81**) **tutorial-14's manager approval became a real three-outcome
  decision** (approved → checkpoint; explicit rejected → terminal manager-reject with
  the reason; anything else → re-suspend through await-decision looping back to
  check-approval). Model byte-identical to Java; decide-before-you-suspend + the
  suspensible capability envelope + the wait-loop RESET pattern stated across guide,
  tutorial help, skill help, and AI catalog (suspend entries byte-identical
  cross-engine); the suspend-on-routing-skill error TEACHES at both Rust enforcement
  sites (validator + traveler). **Durable engine facts (mirror of the Java lesson):**
  seen marks survive suspension and a seen node never re-executes — a wait loop across
  suspensions must `RESET:` its own nodes before the IFs; the Playground Tutorials tab
  bakes resources/help/*.md into the webapp bundle at build time — help edits need
  `npm run release` (bundle now index-DK_iWtSl.js). E2E: rejection + wait-loop
  sections in suspend_resume_tutorial.rs; knowledge-graph 9 suites, fmt, clippy 0.
  <!-- id: thread-tutorial-14-decision-rust | created: 2026-08-07 | last_used: 2026-08-07 | uses: 1 | tier: working | origin: 2026-08-07-150018 -->

- [x] (lock-step — **SHIPPED 2026-08-01: feature merged as PR #191 (`5db06a8f`),
  release merged as PR #192 (`7358f1a2`), tag `v4.11.1` on the verified merge commit;
  both engines in lock-step at 4.11.1**)
  **The Java v4.11.1 lock-step arc: version-aware Redis consume (GETDEL / atomic
  MULTI/EXEC below 6.2, field report), Event Script per-task ttl + honored sub-flow
  delay with teardown cancellation, minigraph node ttl + model-metadata immutability
  (the previously UNGUARDED model.* RHS closed), the traveler run-level watcher with
  exactly-one-terminal CAS arbitration, honest companion drain, fetcher x-ttl stamp,
  and end-to-end deadline propagation (the flow adapter now derives the budget from
  the delivered x-ttl, Java-exact ceil-to-seconds).** Adversarial review round: 14
  confirmed findings resolved (the Java-parity lens caught the raw-ms budget, the
  35s drain fallback, the gate message wording); three exact Java-parity residuals
  documented as shared follow-ups. Workspace 58 suites green / clippy 0 / fmt.
  Increments 76-80. Java reference: mercury-composable v4.11.1 (tag on `410e03bb`).
  <!-- id: thread-v4-11-1-lockstep | created: 2026-08-01 | last_used: 2026-08-01 | uses: 1 | tier: working | origin: 2026-08-01-233448 -->

- [x] (release — 2026-07-30; **TAGGED, then RE-TAGGED pre-publication (Eric's ruling:
  release tags include the updated docs): `v4.11.0` now on merge commit `167484bd` (PR
  [#190](https://github.com/Accenture/mercury/pull/190) — the docs-parity fix: home
  footer "Explore the docs" clusters + Project block with the cross-engine "Java
  version" line + the Release Notes nav link). The original tag on `cc529071` (PR #189)
  was DELETED before publication and re-created — the commit VERIFIED both times
  (Cargo.toml 4.11.0 + the docs fix + both ancestor commits at `167484bd`); remote
  dereference confirmed → `167484bd`. The 4.10.2-round pre-publication tag-move
  precedent applied; a tag NEVER moves after publication. **PUBLISHED 2026-07-30 (Eric
  confirmed, BOTH repos in lock-step) — the suspend/resume feature release is live;
  the arc is CLOSED end to end: design → P1-P5 → consistency review → interop evidence
  → release. Next likely arcs: the minimalist-kafka port (helper servers per
  [[conv-java-helper-servers-for-rust-tests]]) or field feedback on 4.11.0.**) **v4.11.0 release prep, lock-step with the Java
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
  <!-- id: thread-release-4-11-0 | created: 2026-07-30 | last_used: 2026-07-30 | uses: 4 | tier: archive-candidate | origin: 2026-07-30-172823.md -->

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
  <!-- id: thread-graph-suspend-resume-p5 | created: 2026-07-29 | last_used: 2026-07-30 | uses: 11 | tier: archive-candidate | origin: 2026-07-29-235442.md -->

### Blueprint — gaps from Current State (three layers shipped + graduated) to the Vision  (serves: vision-mercury)
> Derived 2026-07-15 from the maintainer-set Vision. Each `(blueprint)` thread is a
> Vision↔reality gap that closes when delivered. Bottom-up order (foundation → UI). The
> three in-scope layers are delivered; forward runway is the UI continuation + the
> connectors/sync-over-async backlog below. The authoritative behavior spec remains the
> Java mercury-composable project (map, don't mirror).

- [ ] **(blueprint)** Continue **foundation → user interface** now that the three core
  layers stand — reframe into concrete UI-layer increments as they are picked up.
  → serves: vision-mercury
  <!-- id: bp-foundation-to-ui | created: 2026-07-15 | last_used: 2026-08-14 | uses: 2 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(backlog) Port the lightweight cloud-native connectors + sync-over-async.** Maintainer
  scope refinement 2026-07-20 (stated while reviewing the docs site): `minimalist-kafka` and
  `twin-kafka` are **lightweight, cloud-native connectors** — distinct from the Kafka service
  mesh (service discovery + sync-over-Kafka, which stays permanently out of scope) — and will
  be ported in future iterations, along with **`sync-over-async`** (the request/response
  bridge over async transports). This is also why HTTP config keys keep their `http.` prefix
  (connector counterparts arrive later). Vision non-goals + instructions + the public
  `docs/background/port-scope.md` all updated to the refined wording. → serves: vision-mercury
  <!-- id: bp-kafka-connectors-backlog | created: 2026-07-20 | last_used: 2026-08-14 | uses: 5 | tier: working | origin: 2026-07-20-030615.md -->

## User Preferences

(none recorded yet — record ONLY what the user explicitly states; never infer)

## Team / Members

(none recorded yet)
