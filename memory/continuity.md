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
- **status:** **Rust port of `mercury-composable`** (canonical Java, released lock-step), delivered bottom-up; all three in-scope layers (platform-core, event-script, active knowledge graph + Playground) ported and milestone-closed, **GRADUATED to github.com/Accenture/mercury 2026-07-20** (docs at accenture.github.io/mercury; regular PR process). Kafka service mesh + Spring out of scope. Current release **v4.12.6** (2026-09-10, lock-step with Java v4.12.6 same-day — the sprint release, Increments 110–115: the Playground webapp UX-modernization mirror and the UI's switch to the live session graph as its single source; stepwise traversal logging as structured records `{id, text, graph}`; the temp graph-model endpoint 404; the f:camelCase/f:snakeCase key-normalization plugins; all seven mercury-* crates on crates.io at 4.12.6, API-verified. Prior: v4.12.5). History lives in `docs/INCREMENTS.md`, session logs, and CHANGELOG — not this line. (Condensed 2026-09-04: the smoke test flagged this line for carrying version history against its own rule.)
- **last_enabled:** 2026-07-15
- **last_review:** 2026-09-10 | through 2026-09-10-030853.md
- **last_invariant_check:** 2026-09-02 | 2026-09-02-184705.md (all 4 never-decay facts + the Vision (5 ids total) CONFIRMED by Eric — inv-never-couple-functions, inv-telemetry-presentation-parity, port-bottom-up-faithful, conventions-rust-baseline, vision-mercury; the review's two core-tier drift restorations re-ratified; thread-reverify-invariants-20260902 closed. Prior walkthrough: 2026-07-26 | 2026-07-26-014908.md (all five never-decay facts confirmed against live code — inv-never-couple-functions, inv-telemetry-presentation-parity, port-bottom-up-faithful, conventions-rust-baseline, and the Vision; two header drifts remedied; ui-fixture carve-out RATIFIED by Eric 2026-07-26))
- **repo:** github.com/Accenture/mercury (official home; graduated 2026-07-20 from the private R&D repo acn-ericlaw/mercury)
- **vision:** `memory/vision.md` (north star, set at enable — Blueprint gaps to be derived)

## Stack & Tools

> Canonical live home for the current stack — language version, dependencies, tool
> versions. `instructions.md` keeps only a high-level descriptor and points here.

**Rust edition 2021**, toolchain = **current stable, kept in sync with CI** (1.98.1 as of
2026-09-08; CI installs `dtolnay/rust-toolchain@stable` with no repo pin, so run
`rustup update stable` when formatting disagrees — a 1.95-vs-1.98 rustfmt skew over
match-arm block wrapping failed PR #242's format gate). Cargo **workspace**
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
Java 21, Maven reactor) at `~/sandbox/mercury-composable` (added by the maintainer
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
  `docs/arch-decisions/ADR.md` (ADR-0001…0007 adapted from the Java repo; later entries
  native — read on demand).
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
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-08-30 | uses: 104 | tier: core | origin: 2026-07-15-215538.md -->

- **The claims-fixture gate drift-tests documentation BEHAVIOR claims (ADR-0018, accepted
  2026-09-06 — lock-step twin of the Java repo's ADR-0023).** `docs/guides/claims-registry.json`
  registers high-value prose claims —
  a claim states exactly what its named engine test pins, never more; `scripts/check-doc-claims.py`
  verifies the normative sentence still appears on its page AND the test pin still exists as a fn
  DEFINITION (test ref `crates/<crate-dir>::<test-file-stem>::<fn>`; rglob fallback resolves
  src-embedded `#[cfg(test)]` pins); the pinned tests run in `cargo test --workspace`. Wired into
  BOTH docs.yml and rust.yml — a test rename does not trigger docs.yml's path filter. The registry
  is a grammar asset (packaged contract + llms.txt; agents may treat registered claims as
  source-verified). New claims enter via the field feedback circuit.
  <!-- id: claims-fixture-gate | created: 2026-09-06 | last_used: 2026-09-07 | uses: 1 | tier: archive-candidate | origin: 2026-09-07-052947.md -->

## Conventions

> Established with the first code (increment 1, 2026-07-15); enforced from the first commit.

- **`cargo fmt` + `cargo clippy --all-targets` clean** is part of "done" for every change
  (default settings, no custom rustfmt.toml yet).
- **Apache-2.0 header** comment on every source file (ported from the Java originals'
  header style). EXCEPTION ruled by Eric 2026-09-11: `templates/*` starter sources carry a
  ONE-LINE scaffold attribution instead — templates seed field applications that are not
  open source, so the full Accenture copyright header must not ride into user code.
- **Release version bumps must include the starter templates (2026-09-11).** Each
  `templates/*/Cargo.toml` carries an EXPLICIT `version = "<version>"` and mercury-* dep
  pins (deliberately NOT workspace-inherited, so a copied-out template builds as-is after
  deleting the in-repo `path` keys) — the release edit list grows from 5 manifests to 8.
  <!-- id: conv-template-version-sweep-rust | created: 2026-09-11 | last_used: 2026-09-11 | uses: 1 | tier: working | origin: 2026-09-11-005808 -->
- Each ported module's `//!` doc names the **Java class it ports** (e.g.
  `org.platformlambda.core.util.ConfigReader`) so reviewers can diff behavior side-by-side.
- **Tests:** unit tests in-module (`#[cfg(test)]`), integration tests in `tests/` with
  fixtures under `tests/resources/` (mirrors Java's `src/test/resources`).
- **Behavior-parity notes** in doc comments wherever the Rust port deliberately mirrors a
  Java quirk (e.g. YAML-tab tolerance) or deliberately diverges — no silent divergence.
- Config-file syntax verbatim (D9): `classpath:/`, `file:/`, `${ENV:default}`, dotted routes.
- **`docs/INCREMENTS.md` is the historical ledger** (maintainer-requested, 2026-07-16):
  one overview row + one section per increment, added as part of each increment's
  definition of done (design rationale stays in `draft-design-specs/platform-core-port.md`;
  the ledger records what shipped when).
- **Example apps are standalone `examples/<name>/` workspace crates** (increment 10,
  2026-07-16): annotated functions + `platform_core::auto_start_main!();` with the app's
  `resources/` beside its `Cargo.toml` — never cargo examples inside a library crate.
  Event-script and knowledge-graph demos land as sibling `examples/<name>/` crates.
- **`tests/ui` compile-fail FIXTURES are test resources — no license headers** (Eric,
  2026-07-26: "ok with the tests/ui without license headers"): a header shifts every
  `.stderr` line and forces TRYBUILD regeneration; treated like Java's
  `src/test/resources` files. The ui RUNNERS (`tests/ui.rs`) do carry headers.
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-09-02 | uses: 113 | tier: core | origin: 2026-07-15-224707.md -->

- **Declare a Memory Reference when a fact is CONSULTED to make a decision — not only when it is
  edited (Eric agreed, 2026-09-04).** `## Memory References` is the sole input to
  `refresh-metadata`, so an undeclared consultation reads as non-use and decays the fact. In the
  Java sibling this demoted a 42-use core convention after one log declared `(none)` while
  reasoning explicitly from it. Rule of thumb: if you would have decided differently without the
  fact, it is a reference. Twin of `conv-declare-consulted-references` in mercury-composable.
  <!-- id: conv-declare-consulted-references-rust | created: 2026-09-04 | last_used: 2026-09-04 | uses: 1 | tier: core -->

- **Playground webapp lock-step sync: replace `crates/knowledge-graph/webapp/` wholesale
  from the Java repo's `system/minigraph-playground-engine/webapp/`, EXCEPT the
  engine-layout spots (2026-09-08, first codified sync; amended 2026-09-09 after Java
  PR #341 — two files moved from preserve-wholesale to PORT).** Exclude Java-only agent
  artifacts (webapp `memory/`, `AGENTS.md`, `CLAUDE.md`, `agent-skills/`, `.cursorrules`,
  `codex.md`, `.codesight/`) and gitignored `node_modules`/`dist`.
  **Preserve wholesale** (git checkout after rsync): `scripts/deploy.js` + `scripts/clean.js`
  (target `../resources/public`), `src/utils/__tests__/graphTransformer.test.ts` fixture
  imports (`../../../../resources/graph/`, `../../../../tests/resources/graph/`), and
  `docs/COMPANION_ENDPOINT_README.md`'s graph.js line ("this Rust port never carried
  `graph.js`").
  **Port** (take the Java file, then re-apply the Rust path lines — these now carry real
  feature content): `src/data/helpContent.ts` (glob `'../../../resources/help/*.md'` + the
  "resolves to crates/knowledge-graph/resources/help/" doc line),
  `src/utils/__tests__/localHelpCommand.test.ts` (the "resources/help/" comment), and
  `vite.config.ts` (the fs-allow comment `../../../resources/help/*.md`).
  **Engine help surface**: `crates/knowledge-graph/resources/help/help.md` prose is
  DELIBERATELY divergent from Java (Rust-adapted wording, graph.js retired) — port Java
  ADDITIONS into it (e.g. the 2026-09-09 Keyboard-shortcuts section), never overwrite.
  README.md and `docs/Technical Documentation.md` deliberately ride the Java wording
  verbatim (established precedent). After sync: `npm run typecheck && npm test`,
  `npm run release` (commits the rebuilt `resources/public` bundle), and a live smoke
  against the Rust engine. The index-chunk hash legitimately differs from Java's — the
  help markdown is inlined at build time by the glob and the prose differs.
  <!-- id: webapp-lockstep-sync-preserved-paths | created: 2026-09-08 | last_used: 2026-09-10 | uses: 6 | tier: active | origin: 2026-09-08-203614.md -->

## Blueprint  *(gap from Current State → Vision; `(blueprint)` threads serve `vision-mercury`)*

> The `(blueprint)` items live one-per-file in `memory/open-threads/` (v4.39.0). This section is
> the visible Vision link PROTOCOL expects; the threads carry the detail.
>
> - `thread-bp-foundation-to-ui` — continue **foundation → user interface** now that the three
>   core layers stand; reframe into concrete UI-layer increments as they are picked up.
> - `thread-bp-kafka-connectors-backlog` — port the lightweight cloud-native connectors
>   (`minimalist-kafka`, `twin-kafka`) + sync-over-async. These are NOT the Kafka service mesh,
>   which stays out of scope.
>
> <!-- restored 2026-09-04 after the smoke test found no Blueprint→Vision link in continuity -->

## Open Threads

> Open Threads live **one per file** in `memory/open-threads/` (`thread-<id>.md`;
> filename = the thread's fact id) so concurrent thread work never merge-conflicts
> (v4.39.0). List that directory to see them; unchecked `- [ ]` threads are the live
> workstreams and never decay. Mark a completed thread `- [x]` in its file and leave
> it — the review sweeps it to the archive once older than `archive_window` sessions.
> Don't archive by hand. See `.agent/schema.md`.


## User Preferences

- **Release rhythm (Eric; confirmed for this repo 2026-09-04, in force for many iterations
  already).** Claude Code prepares every release artifact — branch, version sweep, build and test
  verification, CHANGELOG, release notes — but never merges, tags, or publishes without Eric's
  explicit go-ahead for that specific step; **PR-open and tag/publish are each individually
  gated.** Same rhythm as the Java engine's `eric-release-rhythm`, which the two repos exercise
  in lock-step at each shared version. Recorded here after the 2026-09-04 smoke test found this
  section empty while the practice was visible throughout the archive and every recent log —
  raised to Eric rather than inferred, since this section forbids inferring, and confirmed by him.
  <!-- id: eric-release-rhythm-rust | created: 2026-09-04 | last_used: 2026-09-04 | uses: 1 | tier: core | note: an operating preference that does not decay in relevance; core so it cannot fade out of the layer as its Java twin nearly did -->

## Team / Members

- **Eric Law** — maintainer. Directs commits, gates every merge/tag/publish (see the release
  rhythm above), and is the human who confirms invariants and the Vision at the
  `verify_invariants_every` cadence. Rulings recorded through the memory layer are his; the
  agent never ratifies on his behalf.
  <!-- id: team-eric-maintainer | created: 2026-09-04 | last_used: 2026-09-04 | uses: 1 | tier: core -->
