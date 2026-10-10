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
- **status:** **Rust port of `mercury-composable`** (canonical Java, released lock-step), delivered bottom-up; all three in-scope layers (platform-core, event-script, active knowledge graph + Playground) ported and milestone-closed, **GRADUATED to github.com/Accenture/mercury 2026-07-20** (docs at accenture.github.io/mercury; regular PR process). Kafka service mesh + Spring out of scope; `minimalist-kafka` is ported (K1–K5 under `ot-minimalist-kafka-port`, the Schema Registry included) and so is the OpenTelemetry forwarder (`extensions/opentelemetry-forwarder`, Increment 129, #307 merged 2026-09-22). The current release is the `latest_release` field below (both engines release in lock-step, one number for the same content). History lives in `docs/INCREMENTS.md`, session logs, and CHANGELOG — not this line. (Condensed 2026-09-04 when the smoke test flagged this line for carrying version history against its own rule; re-condensed 2026-09-21 when the release clause had gone stale at v4.12.7.)
- **latest_release:** v4.12.22 (2026-10-10 01:25:48Z — **the lock-step twin of Java 4.12.22: the graph contract, Increments 170–174**:
  release PR #374 merge `e39b5597`, tag `v4.12.22` → the same commit (the branch head `aa5ecd49` merged; the non-memory diff is empty),
  Cargo 4.12.22 verified at the tag; the GitHub release is published (not a draft); `rust`, `docs` and agent-memory CI green on the tag
  commit. **Content:** the graph contract (ADR-0029 in the Java repository's ledger: the Upload step's mock request headers 170; the
  `contract` module and the OpenAPI 3.0 document on demand 171; input validation at the root — the `schema` module, the assumed step
  `graph.schema.validator`, the gate rules — 172; the Schema panel bundle 173; the guide page, the mirrored help and five claims 174; the
  skill-inventory fix `525437a7` that the snapshot's link check demanded). The Java mini-scheduler change of the same release has no
  twin here (no scheduler in this port). **READ:** a deployed graph whose root already carries a `schema` property is validated from this
  release on and refused at the gate when the property is not the contract's shape; `regex` joins the knowledge-graph crate's
  dependencies; an app that wants the OpenAPI endpoint adds its two `rest.yaml` entries. Sweep: 13 manifests + the lock refresh (26
  lines). Readiness: fmt and clippy clean, `cargo test --workspace --no-fail-fast` 136 suites / 707 passed / 0 failed / 9 ignored.
  **Lockstep:** Java v4.12.22 one minute earlier (composable #542 squash `67e573e8` = tag, release 01:24:53Z, 1801 tests); the language
  packs have no change since v4.12.21 and stay at it. **The twelve crates VERIFIED on crates.io at 4.12.22** (created 01:30:03Z–01:30:16Z
  by Eric's `cargo publish --workspace` from the tag, none yanked, in the sparse index; the published platform-core tarball at 4.12.22
  carries the tag's 33 source files, with `lib.rs`, `serializer.rs` and `canonical_packager.rs` SHA-256 identical). Field acceptance not
  yet reported. Origin 2026-10-10-021241.md.
  Prior: v4.12.21 (2026-10-07 02:36:06Z — graph sets end to end and a hardened MsgPack decoder, Increments 148–169; #368 → `28457ccf`,
  tag → `9f524fa3`; [[graph-set-pack-and-deploy-rust]], the decoder pinned by shared vectors (160, 163–165); READ: HTTP 400 for a payload
  nested deeper than 64 levels or followed by extra bytes, empty input decodes as an empty map; 134 suites / 702 tests; crates 12/12;
  Java #528 `b76db298` → `87dd9e78`, the packs caught up from 4.12.15; FIELD-ACCEPTED 2026-10-08. Origin 2026-10-07-013301.md.)
  Prior: v4.12.20 (2026-10-01 03:12:43Z — exact decimal arithmetic for money and the deterministic package format, lock-step with Java;
  mercury #340 → merge `b4783c5b`, tag → `d63e102a`; Increments 143–147; 123 suites / 655 tests / 0 failed; the twelve crates verified
  12/12 on crates.io at 4.12.20. Origin 2026-09-30-235931.md.)
  Prior: v4.12.19 (2026-09-25 23:59:10Z — the rapid-prototyping deploy lane, Increment 142; #333 → `ff6e269c`, tag → `e659c683`;
  `graph.model.automation` takes a comma-separated list of manifests and the later one wins ([[graph-manifest-list-later-wins-rust]]); 606 / 0;
  crates 12/12; Java v4.12.19 `a261ff18`. Origin 2026-09-25-232953.md.)
  Prior: v4.12.18 (2026-09-25 20:55:02Z — the graph.math rulings on both engines, Increments 140–141; #331 → `568d71b2`, tag →
  `7d07e9bd`; [[graph-math-typed-arithmetic-rust]] — READ: `true` never computes as 1/0 and `Infinity` never propagates; crates
  12/12; Java v4.12.18 #464 `1e419a29` → `70e00474`, also carrying Java's #463 Snyk bumps with no analogue here. Origin
  2026-09-25-195421.md.)
  Prior: v4.12.17 (2026-09-25 00:20:16Z — the field's Kafka gap closed on both engines; PR #328 → `ad957930`, tag → `af9d6f30`;
  Increment 139 `SchemaCodec::for_consumer` ([[schema-registry-native-codec]]) plus the Rust-only 137 `yaml_serde` and 138 `cargo
  audit`; 603 / 0 / 9; crates 12/12 00:24Z; Java #461 `e9cde291`. Origin 2026-09-24-235353.md.)
  Prior: v4.12.16 (2026-09-24 00:09:26Z — the correctness round from two Java field reports; PR #324 → `743d4ea2`, tag → `cc138af3`;
  Increment 136: the shared null-source rule, graph.math naming the selector, every abort carrying its reason — READ notes in the
  CHANGELOG; 603 / 0 / 9; crates 12/12 00:12Z; Java #457 `df605533`. Origin 2026-09-23-235047.md.)
  Prior: v4.12.15 (2026-09-23 01:36:57Z — the lock-step round after the four-runtime certification; PR #322 → `87ee371f`, tag →
  `28cd3328`; Increments 132–135 — the E0 twin, [[connected-edge-spans]], the Kafka shutdown contract, the starter's dev mode +
  plain home page (P10), [[redis-restart-aware-retry]] with `redis.heartbeat.ms`; 603 tests; crates 12/12 01:57Z; Java #451
  `aafeff04`, the packs 4.12.1 → 4.12.15. READ notes in the CHANGELOG. Origin 2026-09-23-014725.md.)
  Prior: v4.12.14 (2026-09-22 — the first lock-step release with the Java engine; PR #308 → `b83c493f`, tag → `2c88fe2b`;
  the minimalist-kafka port complete incl. the Schema Registry wire format, the OpenTelemetry forwarder, the sync-over-async
  facade, `Platform::keep_running`, `group.protocol=auto`; 12 crates. Origin 2026-09-22-010413.md.) Prior: v4.12.12
  (2026-09-21 — the catch-up release 4.12.7 → 4.12.12 in one step, PR #296 → `983e7550`, tag → `1ef183cb`; Increments 118–124.)
- **last_enabled:** 2026-07-15
- **last_review:** 2026-10-09 | through 2026-10-09-032244.md (CADENCE — 17 sessions since the 2026-10-06 review: `refresh-metadata` refreshed 9
  footers, tier changes 8 (3 → active, 5 → archive-candidate); archived 10 faded facts after step 6 found no reliance - the window's 82
  commits on their paths were RFC-0007's and ADR-0027's work, not the rules (`redis-restart-aware-retry` 28, `static-decision-table-is-
  graph-data-rust` 29, `graph-math-typed-arithmetic-rust` 28, `graph-manifest-list-later-wins-rust` 23, `example-and-template-carry-their-
  flows` 34, `mapping-source-verbatim-substitution` 29, `jsonpath-jayway-result-shape` 29, `ci-floats-on-stable-toolchain` 37,
  `worktree-build-bakes-resource-root` 31, and `graph-null-property-filtered-rust` 21, which crossed the window with this review's own log); swept 3 closed threads (`hello-task-doc-references`, `reverify-invariants-20261004`,
  `rust-jsonpath-indefinite-list`); reactivated 0, superseded 0, archive-verify pass; invariants not due (28 of 40 since
  2026-10-04-160854); stalled threads none (no open thread remains); contradictions none; Doc Gaps none in the window. Live facts
  27 → 14, lint 0 errors. Smoke test not run.)
  Prior: 2026-10-06 | through 2026-10-06-013022.md (CADENCE — 10 sessions since the 2026-10-04 review: `refresh-metadata`
  refreshed 16 footers, 16 tier changes (13 → archive-candidate, 3 → active); archived 4 faded facts, `graph-math-dialect-closed-set-rust`
  (sslu 22), `conv-template-version-sweep-rust` (sslu 23), and `connected-edge-spans` and `llm-helper-certification-rust`, which crossed the
  window with the review's own log, after step 6 found no reliance in the window - its one commit on their paths,
  `1e8f3ad4`, changed `skills-reference.md` in the data-mapper section, not the dialect, and the v4.12.20 sweep `a033ea70` falls before the
  window; swept 0 (the three closed threads closed inside the window); reactivated 0, superseded 0, archive-verify pass; invariants not due
  (re-verified 2026-10-04); no stalled thread; contradiction scan: none; Doc Gaps: none open. Live facts 28 → 24.)
  Prior: 2026-10-04 | through 2026-10-04-160854.md (CADENCE + SIZE, on Eric's command — 11 sessions since the 2026-09-28 review
  (`review_every` 10) and continuity 604 lines > 600; `refresh-metadata` refreshed 19 footers (16 before the review log, 3 after it), 17 tier changes
  (8 working → active, 3 archive-candidate → active, 4 active → archive-candidate, 2 working → archive-candidate); archived 7 — 6 faded Key
  Decisions (`redis-connection-foundation-rust`, `distributed-cache-rust`, `redis-failure-classification-rust`, `headless-app-keep-running`,
  `otel-forwarder-no-sdk`, sslu 26–28, and `schema-registry-native-codec`, which crossed the window with the review's own log) and 1 superseded (`webapp-bundle-follows-help-edits` → `webapp-single-source-java-repo`);
  `redis-restart-aware-retry` was archived in the first pass and REVERSED before commit — Increment 146 (2026-09-30-235931, which declared
  none) pinned its contract, so the fade was a declaration gap ([[conv-declare-consulted-references-rust]]); declared in this review's log;
  swept 3 completed threads (`ot-minimalist-kafka-port`, `ot-sync-over-async-port`,
  `otel-forwarder-certification`) to `2026-Q4.md`; reactivated 0, superseded 0 new, archive-verify pass (memory-lint 0 errors);
  invariants DUE (45 of 40 since 2026-09-17-004239) → [[reverify-invariants-20261004]] raised; no stalled thread; contradiction scan: one
  stale statement annotated (`graph-math-typed-arithmetic-rust`'s "decimal is not added to the dialect" boundary, overtaken by `DECIMAL:`
  in v4.12.20), no altitude drift. Facts 36 → 27 (memory-lint live count, the new thread included); lines 604 → 487; memory-lint 0/0. Smoke test not run.)
  Prior: 2026-09-28 | through 2026-09-28-234016.md (ON COMMAND after the 13/13 smoke test — archived 0, swept 0; the smoke test's
  staleness fixes landed in `d482723a`; lines 483, facts 24) ·
  2026-09-25 | through 2026-09-25-010828.md (ON COMMAND, Eric — SIZE review at the 600-line cap after the v4.12.17 cycle: archived 1
  (`fork-join-awaits-on-calling-task`, faded), swept 0, reactivated 0; six shipped-decision facts, the stale `status` release clause and two
  release priors condensed — lines 600 → ~520, facts 29 → 28; three facts re-tiered active → archive-candidate; one stale "Open" line in
  `connected-edge-spans` corrected to SHIPPED) · 2026-09-24 | through 2026-09-24-003204.md (advisory sweep at the v4.12.16 seam) ·
  2026-09-23 | 2026-09-23-014725.md.
- **last_invariant_check:** 2026-10-04 | 2026-10-04-160854.md (COMPLETE — [[reverify-invariants-20261004]] closed: Eric CONFIRMED all
  7 never-decay facts and the Vision (8 ids) — `inv-never-couple-functions`, `inv-telemetry-presentation-parity`, `port-bottom-up-faithful`,
  `conventions-rust-baseline`, `conv-declare-consulted-references-rust` (after an elaboration), `conv-proposals-not-in-adr-ledger-rust` (its first
  check), `eric-release-rhythm-rust`, `vision-mercury`; the team record REMOVED at his request (a public repository does not list its team in
  memory). The next re-verify is due 40 sessions after 2026-10-04-160854.) Prior: 2026-09-17 | 2026-09-17-004239.md (all 7 never-decay facts + the Vision (8 ids) CONFIRMED by Eric after an evidence walkthrough — inv-never-couple-functions, inv-telemetry-presentation-parity, port-bottom-up-faithful, conventions-rust-baseline, conv-declare-consulted-references-rust, eric-release-rhythm-rust, the team record (since removed), vision-mercury; the Vision's current-state context refreshed, both Blueprint gaps having closed at the same review's closure gate; thread-reverify-invariants-20260917 closed. Prior: 2026-09-02 | 2026-09-02-184705.md (5 ids) and 2026-07-26 | 2026-07-26-014908.md)
- **repo:** github.com/Accenture/mercury (official home; graduated 2026-07-20 from the private R&D repo acn-ericlaw/mercury)
- **vision:** `memory/vision.md` (north star, set at enable; both derived Blueprint gaps closed 2026-09-17 — none open, new gaps surface as `(blueprint)` threads)

## Stack & Tools

> Canonical live home for the current stack — language version, dependencies, tool
> versions. `instructions.md` keeps only a high-level descriptor and points here.

**Rust edition 2021**, toolchain = **current stable, kept in sync with CI** (1.98.1 as of
2026-09-08; CI installs `dtolnay/rust-toolchain@stable` with no repo pin, so run
`rustup update stable` when formatting disagrees — a 1.95-vs-1.98 rustfmt skew over
match-arm block wrapping failed PR #242's format gate; the 1.99.0 clippy skew of 2026-10-01 is recorded in
[[ci-floats-on-stable-toolchain]], and `async-trait` is locked at 0.1.92). Cargo **workspace**
(`Cargo.toml` root, members `crates/*`); `crates/platform-core` is the first crate.
**Deps in use:** serde 1, serde_json 1, **yaml_serde 0.10** (the YAML Organization's maintained continuation
of the archived `serde_yaml`, wired as `serde_yaml = { package = "yaml_serde", version = "0.10" }` so every
`use serde_yaml::` stays — migrated 2026-09-24 after Eric saw `serde_yaml v0.9.34+deprecated` in the 4.12.16 publish
log; its parser `libyaml-rs` is libyaml transliterated by c2rust, the same technique as the retired `unsafe-libyaml`,
maintained), thiserror 1, log 0.4 (std feature),
tokio 1 (rt-multi-thread/sync/time/macros/net/signal/io-util), async-trait 0.1,
async-channel 2 (per-route MPMC queue), rmp-serde 1 + rmpv 1 (with-serde), uuid 1 (v4),
**hyper 1 (http1/server) + hyper-util 0.1 + http-body-util 0.1** (D10 — REST automation;
deliberately not a web framework: rest.yaml IS the router), **chrono 0.4 + chrono-tz 0.10 +
iana-time-zone 0.1** (event-script date/time plugins; chrono-tz = the ZoneId.of analog,
increment 53), **tokio-rustls 0.26 (ring) +
rustls-native-certs 0.8** (increment 48 — outbound HTTPS with OS-trust-store verification +
`trust_all_cert`; rcgen dev-dep for the self-signed TLS test), **moka 0.12 (sync)**
(increment 71 — the ManagedCache engine, Caffeine's Rust lineage, wrapped as an internal
detail; built with `EvictionPolicy::lru` per Eric's deterministic-eviction ruling), **redis 1.5 (`tokio-comp`, `connection-manager`,
`tokio-rustls-comp`, `cluster-async`)** (Increment 119 — the `mercury-redis-connection` foundation shared by
`mercury-sync-over-async` and `mercury-distributed-cache`; `mercury-minigraph-state-redis` keeps its own
connection, as its Java twin does). Stack rationale:
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

*(Further invariants are distilled from mercury-composable's ADRs when a port surfaces one — none has since the three
layers shipped; the two above have held through every re-verify.)*

## Key Decisions

- **Port bottom-up, faithfully to the Java original** — re-implement mercury-composable in
  Rust layer by layer, foundation → UI (platform-core, then event-script, then active
  knowledge graph), preserving the Java project's behavior. The Java repo is the canonical
  spec (map, don't mirror).
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-08-30 | uses: 104 | tier: core | origin: 2026-07-15-215538.md -->

- **A mock-data upload travels like a command: it loads every member's instance (Eric's design, 2026-10-02; PR #349 merge `278bb023`, Increment 154; lock-step with mercury-composable #498 squash `ce0e7155`; both MERGED 2026-10-03 06:00Z).**
  `commands::upload_content` (REST `POST /api/mock/{id}`) no longer writes the uploader's instance alone: it sends an `upload` event to the command service,
  `handle_upload` loads the payload when the session is the primary and replays it (`forwarded`) into every subscriber's instance, or forwards a subscriber's payload to
  the primary (which replays it back), and `load_mock_content` sets `input.body` and confirms in that member's console (`Mock data loaded into 'input.body' namespace`);
  a session without an instance is refused at the REST edge. **Why:** another member's replayed `run` executed without the data and aborted. The Playground's run
  controls became three steps in the same round - Instantiate, Upload (optional; the form opens for the clicking session only, no console command) and Run - and the
  multi-select hint left the canvas; the UI lives in the Java repo and arrives here as the bundle `index-Bg13jQpc` ([[webapp-single-source-java-repo]]). Pinned by
  `mock_upload_loads_every_member_instance` in `tests/graph_runtime.rs`.
  <!-- id: mock-upload-loads-every-member | created: 2026-10-02 | last_used: 2026-10-09 | uses: 5 | tier: active | origin: 2026-10-02-232252 -->

- **A graph model imported from a file travels like a command: `POST /api/graph/import/{id}` makes it every member's draft (Eric's Playground usability sprint, 2026-10-03; PR #350 merge `dae6377d`, Increment 155; lock-step with mercury-composable #500 squash `856e084b`; both MERGED 2026-10-03 16:03Z).**
  `commands::import_content` validates first (`validate_graph_model`: a JSON object whose only top-level sections are `nodes`, a mandatory list, and `connections`, an optional list; then
  `MiniGraph::import_graph` on a scratch graph, so a node without alias or types is refused at the edge) and sends an `import` event to the command service; `handle_import` replaces the
  draft when the session is the primary and replays it (`forwarded`) into every subscriber's draft, or forwards a subscriber's model to the primary; `import_graph_model` (shared with
  `import graph from`) clears a graph instance and says `Graph model imported as draft` - the line the webapp refreshes on - and says `Graph model not imported - <reason>` for a model
  the importer rejects instead of leaving an empty draft silently. An unknown session is 404. Simple validation only; CompileGraph stays the quality gate (Eric). The UI arrives as the
  bundle `index-Cv2pdvxg` ([[webapp-single-source-java-repo]]): the Import Graph button, the `.json` file drop (a confirmation before replacing a loaded graph), the Download button
  (`<graph-id>.json`, the root node named after the id as `export graph as` does) and the Raw tab. A dev-route addition touches the example, the starter template and the cache example
  `rest.yaml`. Extends [[mock-upload-loads-every-member]]. Pinned by `graph_import_loads_every_member_draft` in `tests/graph_runtime.rs`.
  <!-- id: graph-import-travels-like-command | created: 2026-10-03 | last_used: 2026-10-09 | uses: 3 | tier: active | origin: 2026-10-03-153752 -->

- **Graph sets follow ADR-0027 in the Java repository's ledger (RFC-0005, promoted 2026-10-06): one or more graphs in a canonical
  `<set>.pack`, checked by the gate when packed and deployed all or none through a generated manifest; a set of one graph is how one graph
  is signed (Eric, 2026-10-06; Increment 159, PR #357 merge `60e59aeb`, MERGED 2026-10-06 01:27Z).** This engine's homes: `knowledge_graph::model_gate` (the shared
  gate), `knowledge_graph::graph_set` (the set rules), `tools/graph-packager` (`publish = false`) and the loader
  `knowledge_graph::graph_set_loader` (WP3, Increment 161, PR #359 merge `42f6cf81`, MERGED 2026-10-06 04:46Z; Java twin mercury-composable
  #515): `sets` + `unpack` in `graphs.yaml`, each set unpacked into `<unpack>/<id>.json`, gated and registered all or none, and a generated
  `<unpack>/graphs.yaml` whose `graphs` lists what deployed and `generated` every file written, which the next start removes. WP4, the endpoints
  (`rest::pack_graph_set` and `rest::unpack_graph_set`, `POST /api/graph-set/pack` and `/api/graph-set/unpack` - moved there from
  `/api/graph/...` by Increment 167, PR #365 merge `b12931f5`, MERGED 2026-10-06 23:26Z, because the first paths collided with the executor's `/api/graph/{graph_id}` - the set
  name as `manifest.set`;
  Increment 166, PR #364 merge `f89c0997`, MERGED 2026-10-06 23:03Z; Java twin #524), shipped; WP5, the Playground's "Graph set packaging" panel, shipped in PR #366 (merge `82aa3c86`, Increment 168; Java twin #526 squash
  `f84c40cb`), MERGED 2026-10-07 00:26Z - the bundle deployed from the Java repository, packing on the engine through
  `/api/graph-set/pack`; WP6, the docs, shipped in PR #367 (merge `daa493ba`, Increment 169; Java twin #527), MERGED 2026-10-07 00:49Z - the guide
  pages, `help package` with the mirrored help, five loader claims; the sprint's engine work and docs are complete here. The one-graph rule is pinned by `tests/cli.rs`
  `a_single_graph_is_packed_alone_so_it_can_be_signed` and its Java twin: one 327-byte set, SHA-256 `c500281a…` in both engines. A change
  that alters those bytes breaks the signatures made over the old ones. Java twin fact `graph-set-pack-and-deploy`; the sprint thread is the
  Java repository's `graph-set-packaging`. Relates [[graph-null-property-filtered-rust]].
  <!-- id: graph-set-pack-and-deploy-rust | created: 2026-10-06 | last_used: 2026-10-07 | uses: 8 | tier: active | origin: 2026-10-06-012631 -->

- **A MsgPack payload may nest at most 64 maps and lists, the outermost container being level 1, the same rule as the Java engine (Eric,
  2026-10-06; Increment 160, PR #358 merge `49d599a0`, MERGED 2026-10-06 02:59Z; Java twin mercury-composable #514, fact
  `msgpack-nesting-limit-64`).** This engine
  decoded untrusted bytes with `rmp_serde::from_slice`: the envelope (`EventEnvelope::from_bytes`) and the Event API's compact-format
  check. Measured with a throwaway test: rmp-serde's default of 1,024 counts the outermost container, so a release build decodes 1,023
  levels, but a debug build's 2 MiB thread stack overflowed between 500 and 600 nested arrays and aborted the process. Now
  `serializer::from_msgpack` is `from_slice` with `set_max_depth(MAX_DEPTH + 1)` (rmp-serde refuses the container that brings its counter
  to zero) and reports `Nesting deeper than 64 levels`, which the Event API answers with HTTP 400. The envelope map is level 1, so a body
  nests at most 63. The distributed-cache example's `unpack` decodes through it too; the canonical packager keeps its own bound. Pinned by
  the `serializer` tests, `envelope_wire_format::an_envelope_nested_too_deep_is_a_decoding_error` and the claim `msgpack-nesting-limit`.
  Lesson: measure a library's depth limit against the stack, not against its documentation. Relates [[graph-set-pack-and-deploy-rust]] (the
  canonical packager's 64).
  <!-- id: msgpack-nesting-limit-64-rust | created: 2026-10-06 | last_used: 2026-10-06 | uses: 5 | tier: archive-candidate | origin: 2026-10-06-025704 -->

- **The MsgPack decoders of both engines are pinned by one shared hostile-header vector file, and the canonical decoder now
  refuses the never-used byte `0xc1` at the header (Eric's ask, 2026-10-06; branch `test/msgpack-hostile-header-vectors` commit
  `80040d61`, Increment 163; PR #361 MERGED 2026-10-06 as merge `59c964ab`; the Java twin mercury-composable #519 MERGED the same day as squash `3543a65a`).** `tests/resources/msgpack-hostile-header-vectors.json`, byte-identical with the Java engine's
  copy: 22 inputs every decoder must refuse (a str, bin or ext length or an array or map count beyond the remaining bytes, the
  CVE-2026-90473 `map 32` shape among them; fixed-width values cut short; `0xc1`; 65-level nesting) and 9 controls that must decode
  to exactly their JSON value. `msgpack_hostile_header_vectors` runs them through `serializer::from_msgpack`,
  `EventEnvelope::from_bytes` and `canonical_packager::decode`; the claim `msgpack-hostile-header` sits beside
  `msgpack-nesting-limit`. **The first run found the gap:** `rmpv` reads `0xc1` as nil, so the canonical decoder accepted `91 c1`
  while `rmp-serde` and the Java reader refuse it; `check_markers` (iterative, allocation-free) now walks the markers before
  decoding and refuses `0xc1` and any header that promises more than the input holds - the Java reader's rule - with `rmp`
  declared as a direct dependency for `Marker`. **The empty-input disparity, RULED (Eric, 2026-10-06): Java is the reference implementation
  and the Rust port follows** - Increment 164 (PR #362 merge `12702cc9`, MERGED 2026-10-06; the Java twin mercury-composable #520 squash `d755888d`, merged the same day): `from_msgpack` reads empty
  input as an empty map and `EventEnvelope` carries a struct-level serde default (its `id` had none), so `from_bytes(&[])` is an
  empty envelope with a fresh id as on Java and the Event API answers an empty body `400 Missing routing path`; the canonical
  decoder keeps refusing empty input, and the shared file carries the case as the first `canonical: reject` control. Lesson: parity believed is not parity tested - the shared-file method caught a divergence
  the first time it ran. Extends [[msgpack-nesting-limit-64-rust]]. **Exactly one value (Increment 165, branch `fix/msgpack-trailing-bytes` `14c0a687`, PR
  #363 MERGED 2026-10-06 as merge `3cae9f63`; the Java twin mercury-composable #521 squash `5141330a`, merged the same day):** an independent correctness review of the Java codec found
  that both engines' event codecs read one value and ignored what followed (`80 c1` decoded as an empty map) while both canonical
  decoders refused it; the Java engine fixed it as the reference and this engine follows - `from_msgpack` probes for a `u8` after the
  value (only a missing marker fails as `InvalidMarkerRead`/`UnexpectedEof`, since the probe reads at most nine bytes) and refuses the
  rest as `Unexpected bytes after the value at offset N`, at no cost on a clean input; the shared file gains `80 c1` and `80 c0` (24
  rejects), claim `msgpack-exactly-one-value`. Lesson: a probe for the end of the input must read a bounded number of bytes - an
  `IgnoredAny` probe ends inside a trailing truncated container with the same error a clean end gives.
  <!-- id: msgpack-hostile-header-vectors-rust | created: 2026-10-06 | last_used: 2026-10-06 | uses: 3 | tier: archive-candidate | origin: 2026-10-06-185502 -->

## Conventions

> Established with the first code (increment 1, 2026-07-15); enforced from the first commit.

- **`cargo fmt` + `cargo clippy --all-targets` clean, and `cargo audit` clean** is part of "done" for every
  change (default settings, no custom rustfmt.toml yet; the RustSec audit runs in CI on every PR and weekly since
  Increment 138, 2026-09-24 — its first run closed `rustls` RUSTSEC-2026-0285 with a lock refresh).
- **Apache-2.0 header** comment on every source file (ported from the Java originals'
  header style). EXCEPTION ruled by Eric 2026-09-11: `templates/*` starter sources carry a
  ONE-LINE scaffold attribution instead — templates seed field applications that are not
  open source, so the full Accenture copyright header must not ride into user code.
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
  **Packaging rule (2026-09-20):** every file under `docs/guides/**` and `docs/test-reports/**` must also be
  listed in `system/ai-contract-provider/resources/skill/files.list` — the snapshot test
  `inventory_equals_the_documentation_closure` walks both trees (PR #286's first push failed on a new report).
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-09-02 | uses: 113 | tier: core | origin: 2026-07-15-224707.md -->

- **Declare a Memory Reference when a fact is CONSULTED to make a decision, not only when it is edited (Eric agreed,
  2026-09-04) — since agent-memory v4.42.1 the protocol states the rule, and this fact keeps the local history.** The rule:
  `memory/PROTOCOL.md` (*Maintain memory while working*: a fact is relied on when it shaped a decision) and `DECAY.md` §2; its
  review-time half is `REVIEW.md` step 6, *declaration gaps* (the window's commits first, v4.42.2). Raised upstream from this
  repo and mercury-composable on 2026-10-04 and adopted the same day (upgraded here by #353 and #354), the path the RFC rule
  took ([[conv-proposals-not-in-adr-ledger-rust]]). Twin of `conv-declare-consulted-references` in mercury-composable. Local
  history - two archivals reversed before commit by reading the window for the fact's subject: `conv-template-version-sweep-rust`
  (2026-09-21; the v4.12.12 sweep had applied it undeclared) and `redis-restart-aware-retry` (2026-10-04; Increment 146 had
  pinned its contract undeclared).
  <!-- id: conv-declare-consulted-references-rust | created: 2026-09-04 | last_used: 2026-09-04 | uses: 1 | tier: core -->

- **A proposal is not a decision: raise it in `docs/arch-decisions/RFC.md` as `RFC-NNNN`, never as a
  `Proposed` ADR (Eric, 2026-09-18 on the Java side; adopted here 2026-09-19 in lock-step — "ADR were
  done in a lockstep so it would require the same treatment").** The ADR ledger is an immutable journey
  of decisions, so an entry is written when a decision is *accepted*, never before; a withdrawn
  proposal has no honest ledger status. `RFC-NNNN` and `ADR-NNNN` are separate sequences. The rule is
  also upstream — agent-memory protocol v4.41.2 and the v4.42.0 governance pair, whose `RFC.md`
  skeleton this repo carries byte-for-byte (status vocabulary `Open · Parked · Promoted → ADR-NNNN ·
  Withdrawn`, entries never deleted, newest first). Adopted in the sitting that accepted the three ADRs
  left at *Proposed* (0012–0014), each verified delivered in the tree first, with ADR-0012 amended in
  place rather than superseded because it had never left *Proposed*. **The snapshot rule, learned from
  the Java red main (#421/#424), applied before the change this time:** the AI-contract snapshot
  link-checks every relative link inside itself, so a doc page the ledger links to must be enumerated
  in TWO places here — `system/ai-contract-provider/resources/skill/files.list` (which `build.rs`
  embeds from) and `snapshot_test.rs`'s fixed extras — a Rust change, not docs-only. Twin of
  `conv-proposals-not-in-adr-ledger` in mercury-composable; relates [[eric-release-rhythm-rust]].
  <!-- id: conv-proposals-not-in-adr-ledger-rust | created: 2026-09-19 | last_used: 2026-09-19 | uses: 1 | tier: core | origin: 2026-09-19-022252 -->

- **The Playground webapp and its help pages come from the Java repo; this repo holds a deployed copy (Eric, 2026-10-02; PR #347 merge `781fae43`, Increment 152; the Java twin is mercury-composable #496, squash `a6dc9ce5`; both MERGED 2026-10-02).**
  `crates/knowledge-graph/webapp/` is retired (K7 of the port spec superseded). `npm run release:rust` (or `release:all`, both engines from one build) in
  `mercury-composable/system/minigraph-playground-engine/webapp` builds once and deploys the hashed assets to `resources/public/assets/`, the entry page to `resources/template/playground.html` and a
  MIRROR of the help pages to `resources/help/`: the help is compiled into the bundle and read by this engine for the console `help` command, so both copies have one source. **Rule:** never edit
  `resources/help/*.md` or the bundle here; edit in the Java repo and release to both (the next deploy overwrites). The Java repo expects this repo beside it (`…/sandbox/mercury`); `MERCURY_RUST_REPO`
  overrides (a worktree, for instance). The help sets were consolidated: this repo's 2026-07-19 rewrite is the base of the single set, Java-only content kept, engine-neutral wording, the differences
  stated in place (`graph.js` is deprecated in Java and not registered here), and one claim of this repo's `help session` corrected (`session reset` starts an empty draft on both engines); 12 pages
  changed here, 30 are byte-identical with the previous set. The bundle brought the three Java fixes the copy was behind (mercury-composable #493, #495; the test setup of #494 has no bundle effect) and
  is byte-identical with the Java engine's (`index-lxX8FQ68`; `index-CN-KsNrA` since PR #348 (merge `67a31296`, 2026-10-02), the deploy of the clipboard paste fix mercury-composable #497 (squash `029e5a9f`), the first deploy-only twin
  of a Java webapp fix). `Cargo.toml` has no `exclude` any more; `cargo package --list` carries 214 files, the 42 help pages, the bundle and the entry page, no
  webapp path and no source map. The source maps stay gitignored here. Supersedes [[webapp-bundle-follows-help-edits]] (the rule that a help edit needs the rebuilt bundle still holds, now from the
  Java repo). Relates [[example-and-template-carry-their-flows]].
  <!-- id: webapp-single-source-java-repo | created: 2026-10-02 | last_used: 2026-10-09 | uses: 10 | tier: active | supersedes: webapp-bundle-follows-help-edits | origin: 2026-10-02-180239 -->

- **Tests remove the temporary files they write when they complete, verified by measuring (Eric, 2026-10-06; Increment 162, PR
  #360 merge `acf767d7`, MERGED 2026-10-06 05:35Z).** A test binary runs its tests in parallel and has no after-all hook. Per-process files go under `test_support::temp_path`
  (`mercury-test-support`, dev-only: one folder per process, removed by one `atexit` hook), and a cleanup the code under test owns
  registers with `test_support::run_at_exit`: every test in a binary that keeps the default store registers
  `elastic_queue::shutdown_cleanup`, which a test binary never runs otherwise (only the lifecycle's graceful exit does). A starter
  template, which cannot depend on the workspace-only crate, uses a drop guard. A test that calls `shutdown_cleanup` needs a process
  of its own, because the cleanup removes the holding area. Verify per test binary: a marker file, each binary run from its package
  folder, then `find -newer` over the system temp folder and `/tmp`. Java twin fact `conv-tests-remove-temp-files`; the thread is the
  Java repository's `test-temp-file-housekeeping`.
  <!-- id: conv-tests-remove-temp-files-rust | created: 2026-10-06 | last_used: 2026-10-06 | uses: 2 | tier: archive-candidate | origin: 2026-10-06-053150 -->

## Blueprint  *(gap from Current State → Vision; `(blueprint)` threads serve `vision-mercury`)*

> The `(blueprint)` items live one-per-file in `memory/open-threads/` (v4.39.0). This section is
> the visible Vision link PROTOCOL expects; the threads carry the detail.
>
> - Both derived gaps closed at the 2026-09-17 review's closure gate (Eric): `bp-foundation-to-ui`
>   (delivered/absorbed — the UI is the lock-step Playground webapp) and `bp-kafka-connectors-backlog`
>   (the Kafka service mesh stays out of scope; minimalist-kafka is in scope and continues under
>   `ot-minimalist-kafka-port` → serves: vision-mercury; sync-over-async shipped 2026-09-13).
> - No open `(blueprint)` gap at present — new gaps are derived as threads when they surface
>   (`grep -l '(blueprint)' memory/open-threads/`).
>
> <!-- restored 2026-09-04 after the smoke test found no Blueprint→Vision link in continuity; updated 2026-09-17 at the closure gate -->

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

(none recorded — a public open-source repository does not list its team in memory; the maintainer record was removed
2026-10-04 at the maintainer's request, and it is not to be re-added)
