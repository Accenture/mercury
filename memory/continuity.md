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
- **status:** **Rust port of `mercury-composable`** (canonical Java v4.8.6), same vision, delivered bottom-up. **All three in-scope layers are ported and milestone-closed** — platform-core (2026-07-16; benchmarked: RPC 155K ops/s @ 6µs, ~8.4× the Java record), event-script (2026-07-17; full engine validated on the canonical Java fixtures), active knowledge graph + Playground webapp (2026-07-18). Kafka service mesh + Spring out of scope. 34 increments — ledger: `docs/INCREMENTS.md`; designs: `docs/design/`; AI-companion validation sweep in progress (tutorials 1–5 passed; AI grammar self-sufficient as of 2026-07-18; layer-1/2 AI docs ported 2026-07-19). Companion `/sync` complete and byte-identical in both ports (Java upstream PRs #188–#194 merged).
- **last_enabled:** 2026-07-15
- **last_session:** 2026-07-19 | agent: Claude Code (2026-07-19-161507)
- **last_review:** 2026-07-18 | through 2026-07-18-233807
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
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-07-19 | uses: 30 | tier: active | origin: 2026-07-15-215538.md -->
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
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-07-19 | uses: 38 | tier: active | origin: 2026-07-15-224707.md -->

## Open Threads

> Mark completed items `- [x]` and leave them in place — the review sweeps them to
> the archive once older than `archive_window` sessions. Don't archive them by hand.

- [ ] **AI-companion validation sweep (tutorials 1–13)** — dogfooding the full human–AI
  collaboration path: a *fresh* companion agent builds each Playground tutorial from the **canonical
  AI-agent docs alone** (`mercury-composable/docs/llms.txt` + `.../knowledge-graph/ai-agent-guide.md`
  + `command-reference.md` + `minigraph-commands.json` — never the tutorial walkthroughs), one node /
  one connection at a time with human screenshot validation, through the instantiate→run→inspect
  dry-run (from tut-4 on: L3 = problem-only brief, no syntax hints, build-whole-then-dry-run). Running
  log: `docs/AI-companion-test.md`. **Done: 1, 2, 3, 4, 5, 6, 7, 8.** Tut-3 surfaced + drove the
  `#[optional_service]` / dev-mock / Dictionary bare-`input[]` / inspect-doc-placeholder arc (increments
  30–32, both repos). **Tut-4** (decision node) found an AI-grammar gap — the AI docs listed
  `graph.math` statement keywords but gave no statement syntax, so a fresh agent invented wrong
  branching that failed silently; **fixed** by documenting the `graph.math`/`graph.js` statement grammar
  (IF/THEN/ELSE etc.), landed via public-upstream PR
  [Accenture/mercury-composable#187](https://github.com/Accenture/mercury-composable/pull/187), then a
  second fresh agent solved it from the docs alone (dry-run PASS, all branches + `a==b`). Also validated
  MiniGraph **session-management** (`session subscribe` link + a zero-dep Node WS subscriber gave the
  orchestrator live console visibility). **Pending: 6–13** — each expected to surface more; that is the
  *purpose*, hardening the Rust port toward production quality. **Tut-5 (L4, composition:
  parallel fan-out + `graph.join` + data sourcing) PASSED on the FIRST attempt — 18/18 commands ok,
  zero failures** — first test on this repo's own AI docs, driven wholly via `/sync` on the human's
  live UI session; design semantically equivalent to canonical (one whole-profile Dictionary,
  deterministic post-join `profile[0]/[1]` assembly — avoids canonical's unordered concurrent
  append; the disjoint-`model.*`-scratch lesson derived unaided). Key finding: the five AI docs
  alone were NOT sufficient — Provider `{name}` URL placeholders + Dictionary bare `input[]` live
  only behind the non-existent `composing-the-layers.md`; the companion bridged in-band
  (`describe skill` → `help data-dictionary`). New rollup findings #9–#15 in
  `docs/AI-companion-test.md` (dangling links incl. the constant-set page, `help {topic}`
  undocumented, stale `/command` prose, `/sync` envelope null-omission vs docs + undocumented `id`,
  fan-out parallelism implied only, subscribe-via-sync bug → [[ot-subscribe-via-sync-bug]],
  `session reset` doesn't clear the draft graph — contaminated-session hazard, orchestrator had to
  delete 7 leftover nodes pre-brief). **Grammar gaps #9–#13 FIXED same day (maintainer-directed):**
  Provider/Dictionary authoring section + closed constant set (deprecated `:type` excluded; Dictionary
  `:` = default only) + `help {topic}` + `/sync` prose + truthful envelope (absent ⇒ null, `id`) +
  explicit parallel fork/join/state-safety rules — across `command-reference.md`,
  `minigraph-commands.json`, `skills-reference.md`, `ai-agent-guide.md`, `llms.txt`; all dangling
  links removed (the four AI docs are now self-sufficient — agents should never need the human help
  pages; help-page polish → [[ot-help-pages-rewrite]]). **Tut-6 (L5, data-driven iteration: `for_each` + chained
  fetch + POST provider) PASSED on the FIRST attempt — 18 observed POSTs, all ok, zero failures**
  (2026-07-19, session ws-876960-4). The planned tut-5 retest was **skipped (maintainer direction:
  proceed to tut-6)** — its tightened criterion was applied observationally instead and the #9–#13
  fixes **held**: Provider/Dictionary authoring (incl. the POST `body.*` form) came straight from
  the grammar with ZERO lookups; the single in-band lookup (`help graph-api-fetcher`) was for the
  `for_each` idiom → new findings #17–#21 (for_each idiom + aggregation semantics undocumented,
  `output[]` required-vs-optional contradiction, POST example missing, constants "closed set" vs
  `f:`/`$.` actually resolvable in graph mappings — code-confirmed). The WS-side watcher +
  read-only companion rule (#194) validated live (`subscribed by ["ws-759054-5"]`, mirror held
  the whole run). **Findings #17–#21 FIXED same day (maintainer-directed,
  engine-verified):** `for_each` section (idiom + GUARANTEED ordered aggregation — verified in
  `fetcher.rs`), corrected fetcher matrix (`output[]` optional; only `dictionary[]` hard-required),
  POST-body Provider example, `f:`/`$.` non-constant source forms documented. **NEW #22 (maintainer
  decision): `graph.island` is REQUIRED** — islands link data entities/dictionaries into the
  graph's entity-relationship diagram (living documentation of enterprise knowledge); the AI
  grammar now mandates **no node left unconnected**
  (`root -[contains]-> island -[data]-> dictionary -[provider]-> provider`). **Tut-5 RETEST DONE (2026-07-19) — PASSED on the tightened
  criterion: ZERO in-band lookups** (first run needed two), 20/20 ok, first-attempt dry-run pass;
  the companion also wired the **Island knowledge layer unprompted** (#22 verified) and cited the
  documented fork/state-safety rules (#13 verified) — the grammar self-sufficiency claim is now
  proven end-to-end. Two inference-only frictions → #23 (array-index mapping targets) + #24 (whole-subtree
  Dictionary output) — **both FIXED same day** (composite-keys block + examples across the
  grammar docs). **Tut-7 (L6, mapping-language depth: constants, nested extraction,
  ordered array assembly, execution-time `f:now`) PASSED — cleanest run yet: 8/8 commands, zero
  failures, ZERO in-band lookups, first attempt** (2026-07-19, session ws-783755-2). Pre-flight:
  `f:now()` verified present/correct in the port (maintainer flagged possible unconverted plugins —
  live-probed iso/local/ms before briefing). The updated grammar visibly drove the design (indexed
  assembly with the documented rationale, cross-layer plugin discovery via llms.txt, no cargo-cult
  island). Frictions #26 (example `f:` plugins unnamed in KG grammar) + #27 (nested instantiate
  seeds unshown) FIXED same day. **Tut-8 (L7, reshaping toolbox: JSONPath + `f:listOfMap`/`f:removeKey`)
  PASSED — 17/17, zero failures, ZERO lookups, first attempt** (2026-07-19, ws-783755-2): the
  companion chose the canonical's own "easier" `f:removeKey` route (form inferred → #28 FIXED
  same day with a code-verified syntax+example), and — with NO config nodes — volunteered an
  island with data-entity nodes documenting the domain (#22 shaping designs unprompted). Open:
  #29 (graph.math for_each thinly specified — defer to a tutorial that exercises it) + #30
  (island scope for pure-transformation graphs — maintainer question; proposed: required with
  config/data-entity nodes, encouraged otherwise). **Resume — NEXT: tutorial-9.** Same recipe:
  fresh primary (**verify EMPTY first**), brief = llms.txt + its map, drive via `/sync`, watcher
  subscribes over the WS connection; tutorial help pages carry known errors (backlog
  [[ot-help-pages-rewrite]]) — derive contracts carefully.
  **Layer-1/2 AI docs ported for tutorials 6+ (2026-07-19, maintainer-directed):**
  `docs/guides/event-script/` (ai-agent-guide, flow-grammar, event-script-flow.json, syntax.md —
  flow YAML identical to Java, code examples in the Rust API) + `docs/guides/event-driven/`
  (ai-agent-guide — `#[preload]`/`ComposableFunction`/PostOffice, verified against source);
  `docs/llms.txt` maps them, so companion briefs for extension/task tutorials can stay
  "llms.txt + follow the map".
  → serves: vision-mercury (faithful delivery; a fresh agent orients + operates from the docs alone)
  <!-- id: ot-companion-validation-sweep | created: 2026-07-18 | last_used: 2026-07-19 | uses: 13 | tier: working | origin: 2026-07-18-061457.md -->

- [x] **Bug FIXED (Rust; Java prepared): companion endpoints limit `session` to read-only.** Found
  during tut-5 ([[ot-companion-validation-sweep]]): `session subscribe` via `/sync` registered the
  **ephemeral per-request capture route** (`companion.sync.<uuid>`, released when the POST returns) as
  a durable subscriber — observed live as primary `subscribed by ["companion.sync.3da2143d…"]` while
  the watcher claimed `subscribed to <primary>` (the mid-test mirror death itself was collateral of an
  accidental browser restart — maintainer-confirmed). **Maintainer decision (2026-07-18): a companion
  is an *assistant to* a session, not a WebSocket session — BOTH companion endpoints (fire-and-forget
  + `/sync`) now reject `session subscribe`/`unsubscribe`/`reset` before dispatch; only the read-only
  `session` status query is allowed.** Refusal returned in-band (`ok:false` on `/sync`, 400 on legacy)
  AND teed to the live console; refusal text byte-identical across engines. Rust: guard + helper in
  `rest.rs`, test `companion_sync_rejects_session_topology_commands` (both endpoints) in
  `graph_runtime.rs`; workspace green (tests/clippy 0/fmt). Java: same guard in
  PostCompanionCommand/-Sync + shared statics in GraphCommandService, test
  `companionEndpointsLimitSessionCommandToReadOnly`; 65-test module suite green; **MERGED upstream as
  [#194](https://github.com/Accenture/mercury-composable/pull/194) (2026-07-18)** — the fix is live in
  BOTH engines. Note: the running dev server needs a restart to pick up the
  Rust guard (and to clear the in-memory poisoned subscriber entry). Related hygiene: `session reset`
  does not clear the draft graph and the UI restores drafts on reconnect (rollup #15 — verify a
  companion-test primary is EMPTY before briefing). Rollup #14 in `docs/AI-companion-test.md`.
  → serves: vision-mercury
  <!-- id: ot-subscribe-via-sync-bug | created: 2026-07-18 | last_used: 2026-07-19 | uses: 4 | tier: active | origin: 2026-07-18-231641.md -->

- [ ] **(backlog) Rewrite the interactive help pages for human operators.** Maintainer decision
  2026-07-18 (post tut-5): the `crates/knowledge-graph/resources/help/*.md` pages are designed for
  human operators, while AI agents must get **everything deterministically from the AI grammar**
  (which tut-5's fixes made self-sufficient — rollup #9–#13 DONE). The help pages now deserve a
  dedicated human-UX polish pass in a **separate session**: clarity/structure/consistency, the
  `session reset` semantics note (rollup #15), and pruning agent-oriented duplication where the AI
  grammar is now authoritative. **Constraint:** the pages are verbatim ports from the Java upstream
  (`map, don't mirror` still applies to behavior docs) — coordinate the rewrite with
  `Accenture/mercury-composable` so the two engines' in-band help doesn't drift apart. Rollup #16 in
  `docs/AI-companion-test.md`. → serves: vision-mercury
  <!-- id: ot-help-pages-rewrite | created: 2026-07-18 | last_used: 2026-07-19 | uses: 4 | tier: working | origin: 2026-07-18-231641.md -->

- [x] **Re-verify invariants — CONFIRMED 2026-07-18 (maintainer): both still hold.**
  `inv-never-couple-functions` (the sole Architectural Invariant — inter-function coupling stays
  route-name + `EventEnvelope` only) and the **Vision** (`vision-mercury`, `memory/vision.md`)
  re-confirmed unchanged. First invariant check since enable (cadence:
  `verify_invariants_every: 40` sessions).
  <!-- id: ot-reverify-invariants-2026-07 | created: 2026-07-18 | last_used: 2026-07-18 | uses: 1 | tier: archive-candidate | origin: 2026-07-18-061457.md -->

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
- [x] **(blueprint)** Port the **active knowledge graph** to Rust.
  **MILESTONE CLOSED 2026-07-18** (increments K-1…K-8 / repo 21–29; design
  `docs/design/knowledge-graph-port.md` K1–K9): MiniGraph property graph (platform-core
  built-in) → math engine → graph compiler+registry → runtime + core skills (graph.js
  retired) → platform-core HTTP client + graph.api.fetcher → graph.extension → the
  WebSocket server + declarative `#[websocket_service]`/`#[fetch_feature]` macros → the
  Playground (command grammar, traveler, companion API, dev-gating K9) → the React webapp +
  `examples/minigraph-playground`, live-verified in the browser. **Three layers ported
  bottom-up: platform-core → event-script → active knowledge graph.** → serves: vision-mercury
  <!-- id: bp-active-knowledge-graph | created: 2026-07-15 | last_used: 2026-07-18 | uses: 20 | tier: archive-candidate | origin: 2026-07-15-215538.md -->
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
  <!-- id: bp-companion-sync | created: 2026-07-18 | last_used: 2026-07-19 | uses: 15 | tier: working | origin: 2026-07-18-162832.md -->

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

- [x] **(design) Rust port of the knowledge graph (layer 3) — COMPLETE (K-1…K-8 / repo
  increments 21–29; milestone closed 2026-07-18).** Realizes `bp-active-knowledge-graph`. Design
  doc: **`docs/design/knowledge-graph-port.md`** (K1–K9, gate approved 2026-07-17; `graph.js`
  RETIRED for security — maintainer decision); per-increment detail: `docs/INCREMENTS.md` §21–§29 +
  the session logs. Delivered: MiniGraph (platform-core built-in) → math engine → graph
  compiler+registry → runtime + core skills → platform-core HTTP client + graph.api.fetcher →
  graph.extension → WebSocket server + `#[websocket_service]`/`#[fetch_feature]` macros → the
  Playground (grammar, traveler, companion API, dev-gating) → React webapp +
  `examples/minigraph-playground`, live-verified. Notable platform-core contract fix: `AutoStart::main`
  returns once booted; `AutoStart::run` serves until Ctrl-C. → serves: vision-mercury
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
