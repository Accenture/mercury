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
- **status:** **Rust port of `mercury-composable`** (canonical Java v4.8.6), same vision, delivered bottom-up. **All three in-scope layers are ported and milestone-closed** — platform-core (2026-07-16; benchmarked: RPC 155K ops/s @ 6µs, ~8.4× the Java record), event-script (2026-07-17; full engine validated on the canonical Java fixtures), active knowledge graph + Playground webapp (2026-07-18). Kafka service mesh + Spring out of scope. 49 increments — ledger: `docs/INCREMENTS.md`; designs: `docs/design/`; AI-companion validation sweep COMPLETE (all 13 tutorials passed, 2026-07-19; AI grammar self-sufficient — 10 consecutive zero-lookup first-attempt passes incl. two post-sweep drives). Companion surface byte-identical in both ports (Java upstream PRs #188–#199 merged). Human docs site COMPLETE (MkDocs, 20 pages, published via gh-deploy). **GRADUATED to github.com/Accenture/mercury 2026-07-20** — regular PR process from here on.
- **last_enabled:** 2026-07-15
- **last_session:** 2026-07-20 | agent: Claude Code (2026-07-20-174741)
- **last_review:** 2026-07-20 | through 2026-07-20-040852
- **last_invariant_check:** 2026-07-18 | through 2026-07-18-061457 (confirmed — inv-never-couple-functions + Vision both hold)
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
deliberately not a web framework: rest.yaml IS the router), **tokio-rustls 0.26 (ring) +
rustls-native-certs 0.8** (increment 48 — outbound HTTPS with OS-trust-store verification +
`trust_all_cert`; rcgen dev-dep for the self-signed TLS test). Stack rationale:
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
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-07-20 | uses: 57 | tier: active | origin: 2026-07-15-215538.md -->
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
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-07-20 | uses: 56 | tier: active | origin: 2026-07-15-224707.md -->

## Open Threads

> Mark completed items `- [x]` and leave them in place — the review sweeps them to
> the archive once older than `archive_window` sessions. Don't archive them by hand.

- [x] **AI-companion validation sweep (tutorials 1–13) — COMPLETE 2026-07-19: ALL 13 PASSED** — dogfooding the full human–AI
  collaboration path: a *fresh* companion agent builds each Playground tutorial from the **canonical
  AI-agent docs alone** (`mercury-composable/docs/llms.txt` + `.../knowledge-graph/ai-agent-guide.md`
  + `command-reference.md` + `minigraph-commands.json` — never the tutorial walkthroughs), one node /
  one connection at a time with human screenshot validation, through the instantiate→run→inspect
  dry-run (from tut-4 on: L3 = problem-only brief, no syntax hints, build-whole-then-dry-run). Running
  log: `docs/AI-companion-test.md`. **Done: ALL (1–13 + the tut-5 retest).** Tut-3 surfaced + drove the
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
  #29 (graph.math for_each thinly specified — CLOSED 2026-07-19, increment 38: engine-verified
  probe `rust-foreach.json` + `#math-for-each` grammar section across all four doc surfaces;
  the probe also surfaced that COMPUTE doubles don't feed the whole-number-only `f:add` —
  accumulators stay in COMPUTE) + #30
  RESOLVED (maintainer agreed): island required with config/data-entity nodes, encouraged
  otherwise — landed in the grammar; `listOfMap`/JSONPath impedance-matching rationale captured
  in syntax.md. **Tut-9 (L8, reuse under governance: off-path `graph.math`
  Module + `EXECUTE`) PASSED with ONE self-corrected iteration** (2026-07-19, ws-783755-2):
  19/19 ok, zero lookups; the first functional defect of the sweep (empty output) was
  self-diagnosed in-band via three `inspect`s — the EXECUTE result-landing rule (results belong to
  the INVOKER; the module "just lends the logic") was missing from the grammar → #31 FIXED same
  day (EXECUTE semantics + reusable-module worked example), plus #32 (type-case contradiction) and
  #33 (island scope now covers Module nodes; "executes only to sink" phrasing) FIXED. Maintainer
  correction absorbed: tut-9 = reusable module, NOT subgraphs (graph.extension comes later).
  **Tut-10 (L9, composition by delegation: `graph.extension`
  sub-graph) PASSED — 16/16, zero failures, ZERO lookups, first attempt** (2026-07-19,
  ws-783755-2): delegation chosen over import unprompted (`extension=tutorial-3`, zero
  re-implementation); the island documents the delegation itself (entity + external-model node
  with graph_id/contract properties). Findings #34–#36 FIXED same day (engine-verified):
  graph.extension delegation contract rule-stated; node-name charset corrected to include DIGITS
  (canonical uses fetcher-1; engine has no charset validation); skill-less End blessed.
  **Tut-11 (L10, cross-layer composition: `extension=flow://flow-11`)
  PASSED — 15/15, zero failures, ZERO lookups, first attempt** (2026-07-19, ws-783755-2): the
  companion explicitly credited the rule-stated grammar (#22/#30/#34/#36) for the zero-error run —
  the feedback loop is compounding. Findings: #37 (no `*` on graph.extension — engine-verified,
  now stated) + #39 (model-description endpoint noted) FIXED; #38 (no flow/graph id discovery
  command — engine work, maintainer call) open. **Tut-12 (L11, resilience engineering: `exception=` routing +
  bounded retry + recovery) PASSED — 27/27, zero failures, ZERO lookups, both dry-runs first
  attempt** (2026-07-19, ws-783755-2; the hardest exercise yet): three-node retry machine with
  mutual RESETs, arguably cleaner than canonical. Pre-flight scratch-probe of canonical tut-12
  surfaced #40 (`/sync` ok-heuristic false-negative on import's benign fallback line — engine
  fix landed 2026-07-19, see [[ot-sync-ok-heuristic-fix]]). Findings #41–#46 (exception= semantics, RESET truths
  incl. comma lists + loop guard, IF-jump-ends-list, NEXT/DELAY written forms + timing,
  unresolvable-source skip, dedup per-instance success-only) — ALL engine-verified and FIXED same
  day (new Failure-routing section with the canonical retry pattern). #38 saved as
  [[ot-discovery-commands-backlog]]. **Tut-13 (L12, the custom-logic seam: `graph.task` +
  `v1.hello.task`, header value deliberately ≠ the doc example) PASSED — 16/16, zero failures,
  ZERO lookups, first attempt** (2026-07-19); #47/#48 fixed same day. **SWEEP COMPLETE: 13/13
  tutorials passed** — eight consecutive zero-lookup first-attempt passes from the tut-5 retest
  on; final scorecard in `docs/AI-companion-test.md` ("Sweep complete — the scorecard"). 48
  rollup findings all resolved or tracked. Follow-ups live in their own threads:
  [[ot-help-pages-rewrite]] (done), [[ot-discovery-commands-backlog]], and the `/sync`
  ok-heuristic engine fix (#40 — done, [[ot-sync-ok-heuristic-fix]]).
  **Post-sweep forced test-drive: `f:listOfMap` (2026-07-20) PASSED — 17/17, ZERO lookups,
  first attempt** (headless orchestrator-owned session ws-301207-1 — raw-WS client + /sync, no
  human session needed): columnar→row impedance-matching exercise with a confidential column
  and variable export length; the agent chained file(json:) → f:removeKey → f:listOfMap
  (drops the column BEFORE rows exist) + f:length + an unprompted island. Findings #49–#52
  (mapping[] in-order rule, listOfMap order guarantee, file() read-at-execution — empirically
  proven by swapping the file between runs — f:length discoverability) all FIXED same day.
  Streak extended by **drive #2
  (2026-07-20): discovery-driven delegation** — brief named NO graph/flow id; the agent found
  its target via `list graphs` + empirical probes, delegated to tutorial-3, island documented
  the delegation (21/21, zero lookups, first attempt) — **TEN consecutive zero-lookup
  first-attempt passes**. Findings #55–#57 fixed same day; **#53 + #54 IMPLEMENTED
  2026-07-20 (increment 47, both ports)** — new `describe graph {graph-id}` contract view
  (purpose + derived input/output surface; list → contract → delegate, all read-only) +
  differentiated tutorial-3/5 purposes; Java upstream PR
  [#200](https://github.com/Accenture/mercury-composable/pull/200) MERGED (2026-07-20) —
  the discovery arc (list → contract → delegate) is live in BOTH engines. **Drive #3
  (2026-07-20): live HTTPS fetch — PASSED 16/16, zero lookups, first attempt** — increment
  48's TLS transport field-validated end-to-end against the real google.com CA chain
  (truthful 301 + f:length page size; unprompted island); findings #58–#61 doc-FIXED same
  day, #62–#63 (shared /sync contract gaps) → [[ot-sync-companion-contract-gaps]] —
  **ELEVEN consecutive zero-lookup first-attempt passes**.
  **Layer-1/2 AI docs ported for tutorials 6+ (2026-07-19, maintainer-directed):**
  `docs/guides/event-script/` (ai-agent-guide, flow-grammar, event-script-flow.json, syntax.md —
  flow YAML identical to Java, code examples in the Rust API) + `docs/guides/event-driven/`
  (ai-agent-guide — `#[preload]`/`ComposableFunction`/PostOffice, verified against source);
  `docs/llms.txt` maps them, so companion briefs for extension/task tutorials can stay
  "llms.txt + follow the map".
  → serves: vision-mercury (faithful delivery; a fresh agent orients + operates from the docs alone)
  <!-- id: ot-companion-validation-sweep | created: 2026-07-18 | last_used: 2026-07-20 | uses: 36 | tier: active | origin: 2026-07-18-061457.md -->

- [x] **Bug FIXED (Rust; Java prepared): `/sync` ok-heuristic false-negative on import's benign
  fallback (rollup #40).** Found in the tut-12 pre-flight ([[ot-companion-validation-sweep]]):
  `import graph from {deployed}` succeeds via the classpath fallback but returned `ok:false` —
  the per-line `is_error_line` heuristic tripped on the benign "Graph model not found in /tmp/…"
  line the engine prints before "Found deployed graph model…". Fix (identical design, both
  engines): classification is now **whole-output-aware** — the not-found line is forgiven *only*
  when the same output also carries the fallback's success marker; a genuine miss prints the
  not-found line alone and stays `ok:false` (verified against both import handlers: a real miss
  emits nothing after the not-found line, so the rule can't mask real failures). Rust:
  `first_error_line()` in `rest.rs` + both-direction test in `graph_runtime.rs`
  (workspace/clippy/fmt green). Java: mirrored in `PostCompanionCommandSync` +
  `CompanionSyncTest` (66-test module suite green), committed on local branch
  **`fix/companion-sync-ok-heuristic`**, pushed to origin at maintainer direction — **PR
  [#195](https://github.com/Accenture/mercury-composable/pull/195) MERGED upstream
  (2026-07-19)**. Both engines live-validated by the maintainer; the fix is live in BOTH. Docs aligned: `ai-agent-guide.md` caveat → fixed semantics,
  `minigraph-commands.json` sync_envelope note, rollup #40 → DONE. `/sync` contract stays
  byte-identical across engines. → serves: vision-mercury
  <!-- id: ot-sync-ok-heuristic-fix | created: 2026-07-19 | last_used: 2026-07-19 | uses: 3 | tier: archive-candidate | origin: 2026-07-19-205854.md -->

- [x] **HTTP-boundary content-type dispatch mirrors Java exactly (parity fix, maintainer-directed
  2026-07-19).** The maintainer's manual `/sync` test surfaced that the Rust boundary was laxer than
  Java's `HttpRouter.handlePayload`. Three divergences fixed in `parse_body`
  (`crates/platform-core/src/automation/server.rs`): JSON sniffing under non-JSON content types
  removed (Java never parses outside `application/json`; bracket-guard + text fallback; empty JSON
  body → `{}`); missing/unknown content type → **MsgPack binary** body (Java `byte[]`; empty → null);
  `application/x-www-form-urlencoded` → fields into `parameters.query` with null body. Content-type
  matched on the `;charset`-stripped value, case-sensitively like Java. **Key wire-verified fact: the
  Java client sends NO default content-type** — a Map body POSTs out header-less (chunked), so POST
  providers only work because the canonical fixtures map
  `text(application/json) -> header.content-type` (tutorial-6, both repos; the AI grammar's POST
  example #19 already teaches this — it is load-bearing in BOTH engines now). Remaining deliberate
  divergence: `application/xml` arrives as raw text (no XML parser in the stack; same deferral as the
  HTTP client response side) — noted in code + D10 (`docs/design/platform-core-port.md`). End-to-end
  test `body_dispatch_mirrors_java_content_type_rules` (BodyProbe) in `tests/rest_automation.rs`;
  workspace 202 tests / clippy 0 / fmt clean. `/sync` unaffected (commands still arrive as strings in
  both engines, even mislabeled as JSON). → serves: vision-mercury
  <!-- id: http-boundary-content-type-parity | created: 2026-07-19 | last_used: 2026-07-19 | uses: 1 | tier: working | origin: 2026-07-19-213516.md -->

- [x] **Numeric promotion for the simple-plugin arithmetic family (increment 39, BOTH ports).**
  Maintainer decision 2026-07-19 (prompted by increment 38's probe: `f:add` couldn't consume a
  `COMPUTE` double — "simple arithmetic belongs in a simple plugin"): `promoteNumber` now
  promotes whole numbers/strings to **long**, decimals to **double**; result type decided over
  ALL args before folding (order-independent) — any floating arg ⇒ double computation,
  all-integral ⇒ exact long incl. integer division (**strictly widening**: no previously-working
  call changes). Covers add/subtract/multiply/div/mod + increment/decrement + gt/lt (whole pairs
  compare exact). Rust: `plugins_e8.rs` + mixed-type unit cases + the `f:add`-on-doubles
  accumulator in the `rust-foreach` probe (workspace 202/clippy 0/fmt). Java: upstream
  **PR [#196](https://github.com/Accenture/mercury-composable/pull/196) MERGED** (2026-07-19;
  `SimplePluginUtils.reduceNumbers`, 7 arithmetic classes, gt/lt, promotion test + `f:round`,
  142+67 module tests green; incl. the maintainer's unused-import review touch-up). Docs relaxed across all surfaces (grammar/#math-for-each, JSON
  catalog, skills-reference, help page + bundle, event-script syntax.md matrix). Boundary
  documented: once a double enters, IEEE-754 (ints exact to 2^53). **Follow-up same session:
  new `f:round(number[, places])` plugin (both ports, second commit `80e64166` on the branch)**
  — half-up on the shortest DECIMAL representation (1.005 → 1.01 at 2 places; binary error
  never leaks into the decision); whole inputs pass through. → serves: vision-mercury
  <!-- id: plugin-numeric-promotion | created: 2026-07-19 | last_used: 2026-07-19 | uses: 1 | tier: archive-candidate | origin: 2026-07-19-225257.md -->

- [x] **LATENT BUG fixed (both ports): join barrier counted failed/reset branches (increment 40).**
  Backlog probe #3 confirmed it: `skill_run` (the join's completion source) meant "ran" not
  "completed" — stamped even when a skill failed into its `exception=` route — and `RESET`
  cleared `node_seen`+state but left the stale mark, so a fork whose failing branch retries
  could fire the join **prematurely** and silently lose that branch's data (empirically proven:
  probe test red on old code in BOTH engines, `expected: Peter, got: null`). Fix (maintainer
  decision + companion tightening; **traveler ≡ executor**, both ports): `skill_run` is
  **success-only** (not marked when `{node}.status`+`{node}.error` are set) and `RESET` clears
  it with the guard and state — join completion is now "success-only and current" (a
  permanently failing branch times out visibly instead of corrupting output). Probes:
  `rust-join-retry.json` + `join_barrier_waits_for_a_retrying_branch` (Rust),
  `unit-test-join-retry.json` + `joinBarrierWaitsForRetryingBranch` (Java, 68-test suite
  green, branch **`fix/join-barrier-retry-interplay`** pushed — maintainer opens the PR).
  Docs updated across grammar/catalog/skills-reference/help pages + bundle. Upstream: PR
  [#197](https://github.com/Accenture/mercury-composable/pull/197) created by the maintainer
  (CI pending). **Follow-on observation FIXED same day (increment 41, both ports):** chained
  joins now judge an upstream join by its recorded OUTCOME (`node_seen[join] == true` = fired)
  instead of the run mark — probe `rust-join-chain.json`/`unit-test-join-chain.json` red on
  old code in both engines (`expected: X, got: null`); Java: #197
  MERGED, then `fix/chained-join-outcome` rebased onto post-#197 main and **PR
  [#198](https://github.com/Accenture/mercury-composable/pull/198) MERGED (2026-07-20)** —
  the full join-barrier arc (increments 40+41) is live in BOTH engines.
  → serves: vision-mercury
  <!-- id: join-barrier-valid-completions | created: 2026-07-19 | last_used: 2026-07-19 | uses: 1 | tier: archive-candidate | origin: 2026-07-19-233602.md -->

- [x] **Prepare `docs/guides` for the human reader — COMPLETE 2026-07-20 (increments 43–46,
  all four phases).** MkDocs+Material site, 20 nav pages strict-green: Home + Getting Started
  (43); Foundations + Layer 1/2 guides with the layer-organized "AI-enabled" nav (44); Layer 3
  human set incl. composing-the-layers (sweep finding #9 closed) + rest-automation +
  flow-schema + six D-H2 source-enumerated references (45); port-scope + polish + final pass
  (46). ~4,600 lines, all source-verified, ~40 port-note boxes. **This was the stated gate
  before graduation to github.com/Accenture/mercury.** Upstream doc-fix candidates parked:
  Java flow-schema documents `error.status` but the engine key is `error.code` (both
  implementations); `error.stack` null in Rust. Original backlog text:** **Design approved (Q1–Q4
  answered: guides/ tree for both audiences; identity = github.com/Accenture/mercury —
  graduation after the docs; deep-purple palette; help pages unchanged) + PHASES 1–2 SHIPPED
  2026-07-20 (increments 43–44)**: scaffold + Home + Getting Started + strict CI; then
  Foundations trio + Layer 1/2 guides (7 source-verified pages, 17 port-note boxes,
  observability records verbatim from a live run) with the Java site's LAYER-ORGANIZED nav
  (each layer's AI agent guide inside its section — "the repo is AI-enabled"). PHASE 3
  SHIPPED same day (increment 45: 12 pages — KG human set incl. composing-the-layers closing
  sweep finding #9, rest-automation, flow-schema, the six D-H2 reference conversions with
  source-enumerated keys; 19 nav pages, strict build green; Java-doc bugs surfaced:
  `error.status`→`error.code`, `error.stack` null — upstream doc-fix candidates). Remaining:
  phase 4 (background/port-scope page + Home polish + final link pass). Design
  (`docs/design/human-docs.md` v1): MkDocs+Material per the agent-memory
  recipe; core rule D-H2 = no wide reference tables (entry-per-heading, the maintainer's
  presentation critique); human pages under `docs/guides-human/` (AI-doc paths untouched);
  AI set surfaced under Reference (link, never restate); 18 Java pages mapped, 8 skipped
  (out of port scope); 4 increments; 4 open questions. Maintainer decision 2026-07-19:
  the guides tree currently serves AI agents (the knowledge-graph AI set + event-script +
  event-driven AI guides, hardened by the validation sweep); the **human developer documentation
  was deferred** and is now backlog. Source map: the Java upstream's human guides
  (`~/sandbox/mercury-composable/docs/guides/` — getting-started, architecture, methodology,
  event-driven walkthroughs like write-your-first-function / function-execution, rest-automation
  human pages, api-overview, annotations/configuration/event-envelope references, observability,
  index/home) — adapted to the Rust port the way the help pages were (2026-07-19): map don't
  mirror, all code in the Rust API, port truths (tokio not virtual threads, `#[preload]` not
  `@PreLoad`, no Kafka/Spring), verified against source. The AI docs stay agent-optimized and
  separate; the in-app help pages are already done. → serves: vision-mercury
  <!-- id: ot-human-guides-backlog | created: 2026-07-19 | last_used: 2026-07-20 | uses: 7 | tier: active | origin: 2026-07-19-181641.md -->

- [x] **Discovery commands for deployed flows and graph models — DONE 2026-07-20 (increment 42,
  BOTH ports).** From tut-11 finding #38: the `list` command grows two read-only forms —
  `list graphs` (compiled registry ∪ deployed-folder models, each with the root's `purpose`:
  living documentation) and `list flows` (Event Script flows) — valid `extension=` /
  `extension=flow://` targets discoverable without an out-of-band brief, on the console AND both
  companion endpoints. Rust: `commands.rs` + `playground.rs` assertions (workspace 202/clippy
  0/fmt). Java: `GraphCommandService` + `/sync` test in `CompanionSyncTest` (70-test suite
  green); **PR [#199](https://github.com/Accenture/mercury-composable/pull/199) MERGED (2026-07-20)**
  (3 commits: feature + description/purpose refinements + webapp-bundle refresh) — discovery
  is live in BOTH engines. AI grammar +
  catalog + agent-guide recipe + skills-reference + `help list.md` all updated; rollup #38 →
  DONE. Design note: discovery rides the command surface (no new REST endpoints — `/sync`
  already gives agents REST access to every command). → serves: vision-mercury
  <!-- id: ot-discovery-commands-backlog | created: 2026-07-19 | last_used: 2026-07-20 | uses: 6 | tier: active | origin: 2026-07-19-171653.md -->

### Blueprint — gaps from Current State (greenfield) to the Vision  (serves: vision-mercury)
> Derived 2026-07-15 from the maintainer-set Vision. Each `(blueprint)` thread is a
> Vision↔reality gap that closes when delivered. Bottom-up order (foundation → UI). Detailed
> per-layer Designs are TODO — the authoritative behavior spec is the Java mercury-composable
> project (map, don't mirror); harvest it into per-layer Designs when a local checkout is
> available and authorized (see the harvest thread below).

  **Design drafted 2026-07-17** (`docs/design/knowledge-graph-port.md` v1) — gate pending.
- [ ] **(blueprint)** Continue **foundation → user interface** once the three layers stand.
  → serves: vision-mercury
  <!-- id: bp-foundation-to-ui | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->
- [x] **(blueprint)** **Graduate to the official Accenture repo — DONE 2026-07-20.** The
  maintainer repurposed `github.com/Accenture/mercury` (the original Java v1–v3 repo; legacy
  releases kept as branches) as the official Rust home; the full R&D history (122 commits)
  was merged onto it (`--allow-unrelated-histories`, branch `graduation`), README
  de-privatized + legacy section preserved, MkDocs publishing automated (gh-deploy on main);
  **PR [#147](https://github.com/Accenture/mercury/pull/147) MERGED** (true merge commit —
  history intact).
  Thereafter the regular PR process applies (no more direct pushes to main). The private
  R&D repo (acn-ericlaw/mercury) freezes as the prototyping archive. → serves: vision-mercury
  <!-- id: bp-graduate-to-accenture | created: 2026-07-15 | last_used: 2026-07-20 | uses: 4 | tier: active | origin: 2026-07-15-215538.md -->
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
  <!-- id: bp-companion-sync | created: 2026-07-18 | last_used: 2026-07-19 | uses: 22 | tier: working | origin: 2026-07-18-162832.md -->

- [ ] **(backlog) Port the lightweight cloud-native connectors + sync-over-async.** Maintainer
  scope refinement 2026-07-20 (stated while reviewing the docs site): `minimalist-kafka` and
  `twin-kafka` are **lightweight, cloud-native connectors** — distinct from the Kafka service
  mesh (service discovery + sync-over-Kafka, which stays permanently out of scope) — and will
  be ported in future iterations, along with **`sync-over-async`** (the request/response
  bridge over async transports). This is also why HTTP config keys keep their `http.` prefix
  (connector counterparts arrive later). Vision non-goals + instructions + the public
  `docs/background/port-scope.md` all updated to the refined wording. → serves: vision-mercury
  <!-- id: bp-kafka-connectors-backlog | created: 2026-07-20 | last_used: 2026-07-20 | uses: 1 | tier: working | origin: 2026-07-20-030615.md -->

- [x] **`/sync` contract gaps (findings #62–#63) — FIXED 2026-07-20 (increment 49, BOTH
  ports).** `/sync` marks its dispatch `direct` → the 1s dedup guard does not apply
  (repeats always execute; WS path unchanged); `Syntax:` usage hints classify `ok:false`
  with the hint in-band. Red/green-verified in Rust (`companion_sync_contract_gaps_closed`,
  workspace 206/clippy 0/fmt); Java mirror in `CompanionSyncTest` (71-test suite green,
  PR [#201](https://github.com/Accenture/mercury-composable/pull/201) MERGED 2026-07-20 —
  the /sync contract fixes are live in BOTH engines; test note:
  the WS-guard pair must dispatch via the 1-instance singleton — `graph.command.service`
  runs 50 instances and races). Envelope contract documented (agent guide + catalog).
  Original finding text:** **#62:** the command surface silently drops an identical
  command repeated within 1s on the same session (`is_duplicate`, `commands.rs`; Java
  `GraphCommandService:178` — a UI double-submit guard) — a `/sync` caller gets `ok:true`
  with EMPTY output, violating the documented envelope (echo always present); LLM pacing
  masked it for the whole sweep, but scripted/pipelining AI callers hit it. Recommend:
  `/sync` bypasses the dedup (an RPC caller is not a flaky WS client) + an explicit
  "duplicate ignored" line. **#63:** malformed commands answer with a `Syntax: …` usage line
  that `first_error_line` doesn't classify → false-positive `ok:true` for a command that did
  nothing (mirror of #40's false-negative). Recommend: usage response ⇒ `ok:false` + `error`.
  Both fixes are small and shared-design (identical in both ports, like #40).
  → serves: vision-mercury
  <!-- id: ot-sync-companion-contract-gaps | created: 2026-07-20 | last_used: 2026-07-20 | uses: 2 | tier: active | origin: 2026-07-20-051617.md -->

- [x] **HTTP redirection backlog — RESOLVED 2026-07-20: story WITHDRAWN (parity confirmed
  empirically; maintainer's stated criterion met).** A temporary JUnit probe in the Java
  repo (`RedirectProbeTest`, PostOffice → `async.http.request`, live `https://google.com`)
  returned **301 / location: www.google.com / 220-char body — byte-identical to the Rust
  port's drive-#3 result**. Reactor-Netty supports `followRedirect` but mercury-composable
  never enables it, so raw-3xx-to-the-caller is canonical behavior in BOTH engines; finding
  #61's grammar rule ("the fetcher never follows redirects; point the Provider url at the
  target") documents the shared contract. Nothing to port. Probe file removed after
  evidence capture (live-endpoint dependency — not CI material); verbatim transcript +
  decision record now in `docs/design/platform-core-port.md` §5j. **Maintainer rationale
  (2026-07-20): deliberate design, not an omission** — the engines target BACKEND
  applications; redirect-following is browser-side automation; a backend needing it (e.g.
  SSO) handles it programmatically (layer 1) or declaratively via `{node}.status` routing
  (layers 2/3); years of Java production confirm it is not normally required. The raw 3xx
  IS the correct answer for a backend client. → serves: vision-mercury
  <!-- id: ot-http-redirect-backlog | created: 2026-07-20 | last_used: 2026-07-20 | uses: 2 | tier: active | origin: 2026-07-20-051617.md -->

- [x] **Back-port the AI-doc grammar hardening to the Java repo — EXECUTED + VALIDATED
  2026-07-20 (PR [#203](https://github.com/Accenture/mercury-composable/pull/203) MERGED 2026-07-20 —
  the battle-tested grammar is live in BOTH engines).** Gold
  test PASSED: a fresh agent drove the JAVA Playground from the back-ported docs alone —
  17/17, ZERO lookups, first attempt (columnar listOfMap exercise + unprompted
  variable-length proof) — **streak 12, the twelfth on the Java engine** (grammar is
  engine-neutral). graph.js restored as live (Rust-only retirement); flow-schema
  `error.status`→`error.code` fixed (engine-verified); Java suite 71 green. Headless-client
  lesson: Java WS idle-closes without APPLICATION-level ping text frames (the UI contract);
  Rust tolerates protocol pings. Original survey text:** Marker audit: Java ENGINE is fully
  synced (PRs #187–#201; CompileGraph purpose enforcement verified), but **31/40
  grammar-hardening markers are absent from the Java AI docs** — findings #9–#63 landed on
  the Rust copies only after the 2026-07-18 port. Scope: ~6 Java files
  (`docs/guides/knowledge-graph/{command-reference,minigraph-commands.json,skills-reference,
  ai-agent-guide}.md`, `docs/llms.txt`, `docs/guides/event-script/syntax.md` — numeric
  promotion/f:round/removeKey/listOfMap) + two help-page behavior blocks
  (`help graph-api-fetcher.md`: failure-routing pattern + HTTP semantics). **EXCLUDE
  intentional divergences:** graph.js (alive in Java!), Rust-API examples, profiles rename,
  port notes. Validation: a fresh-agent drive against the JAVA Playground from the
  back-ported docs alone. Java human-doc bugs also parked: flow-schema `error.status` →
  engine's `error.code`. → serves: vision-mercury
  <!-- id: ot-java-ai-docs-backport | created: 2026-07-20 | last_used: 2026-07-20 | uses: 3 | tier: active | origin: 2026-07-20-163856.md -->

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

- [x] **Profile selection renamed `APP_PROFILES_ACTIVE` / `app.profiles.active` — no alias
  (increment 37).** Maintainer decision 2026-07-19 (supersedes the 2026-07-15 alias plan; its
  "foundation robust" gate is met): Spring is irrelevant to the Rust port, so the generic names
  replace `SPRING_PROFILES_ACTIVE`/`spring.profiles.active` outright — the **one deliberate
  exception to D9's verbatim-config rule** (a `spring.profiles.active` line in a ported config
  file does NOT carry over). Mechanism/precedence unchanged (env → override registry →
  consolidated key; `application-{profile}.yml` overlays). Code + manifest + tests + design doc
  (§3, §8 Q1 DECIDED) + ledger updated; workspace 202 tests / clippy 0 / fmt clean.
  **Follow-up DONE same day (maintainer-directed): `spring.application.name` retired too** —
  `Platform::name()` reads `application.name` alone (Java's own primary key) and the default
  aligns to Java's `"application"` (was an unnoted `"untitled"` divergence). No Spring-named
  config key remains live anywhere in the port. → serves: vision-mercury
  <!-- id: profiles-renamed-app-active | created: 2026-07-19 | last_used: 2026-07-19 | uses: 1 | tier: archive-candidate | supersedes: ot-profiles-alias-backlog | origin: 2026-07-19-215701.md -->

## User Preferences

(none recorded yet — record ONLY what the user explicitly states; never infer)

## Team / Members

(none recorded yet)
