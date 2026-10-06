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
- **latest_release:** v4.12.20 (2026-10-01 03:12:43Z — **exact decimal arithmetic for money and a deterministic package format, lock-step
  with the Java engine (v4.12.20, 03:11:27Z)**: release PR #340 → merge `b4783c5b`, tag `v4.12.20` → `d63e102a` (two memory-only commits past
  the merge; the non-memory diff is empty), workspace version verified at the tag; the GitHub release is published (not a draft).
  **Content:** Increments 143–147 — the dialect docs (143), the `f:decimal*` plugins on one shared `event_script::decimal` core (144), the
  `DECIMAL` statement, the numeric-string comparison and `round` half-up (145), the bounce-test flake fix (146) and
  `platform_core::canonical_packager` (147), with the money-loop and packager guides and a late `f32` widening. **READ:** a numeric-looking
  string now compares as a number, and `round(-2.5)` is `-3`; both reach graphs that never say `DECIMAL`. Sweep BUILD FILES ONLY 13 Cargo.toml
  plus the lock refresh (48 lock lines, the 4.12.19 shape). Gates: fmt, clippy `-D warnings`, `cargo test --workspace` 123 suites / 655 tests /
  0 failed, claims, links. Java: #485 squash `9e515825`, tag → `fc940bea`. The Java–Rust byte-for-byte interop on 14 tutorials and 50
  fixtures found 0 differences (report in both repos, mercury #341, docs only). `rust` main CI success on the tag commit. **The twelve crates are on crates.io at 4.12.20 (verified 12/12):** published 03:21:45Z to 03:21:58Z (Eric ran `cargo publish --workspace` from the
  tag), in the sparse index and not yanked, and the published `mercury-platform-core` tarball's `canonical_packager.rs` is byte-identical to the tag's
  (SHA-256 match, `f32` widening present). The python/node packs need no change. Origin 2026-09-30-235931.md.
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
- **last_review:** 2026-10-04 | through 2026-10-04-160854.md (CADENCE + SIZE, on Eric's command — 11 sessions since the 2026-09-28 review
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

- **A traced HTTP request is ONE connected span tree whose root is the edge's round-trip span; a streamed response is
  traced at its head and its tail, never per token (Eric's rulings on the Dynatrace review of the v4.12.15 certification
  traces, 2026-09-22; Increment 133, PR #315, lock-step with mercury-composable #444; SHIPPED in v4.12.15).**
  `automation/server.rs` mints the span at receipt (`EdgeTrace`), every dispatch parents onto it, and the record
  `service=http.request` is emitted by `handle` (buffered response, edge error) or by the stream renderer at the terminal
  (head status, the in-band failure's status, or the idle 408) — `start` = receipt, `exec_time` = the round trip,
  `parent_span_id` = the inbound traceparent span. **All four OTel forwarders map SERVER iff `service == http.request`;
  every function execution is INTERNAL.** The stream relay's client leg parents onto the sender because `is_zero_traced`
  no longer consults `skip.rpc.tracing` — the list only suppresses the caller-side RPC `round_trip` record (Java
  `InboxBase` semantics; the RPC path had masked the drift for months). `EventStreamWriter` sends the first segment and
  the terminals traced and the data segments through `PostOffice::send_untraced`; the HTTP client relays stamp the client
  leg's own trace (`RelayTrace`) on synthesized head/eof/exception segments and forward raw token frames untraced;
  `StreamLaneService` annotates the terminal record with `frames` = the data-segment count. **Why it was invisible:** 0
  export failures in every drive; only the backend's trace tree showed the orphans — and the drive's fabricated
  `traceparent` broke every root, a drive artifact that looked like an engine defect (send `X-Trace-Id`, or nothing,
  without a real upstream span). **READ at 4.12.15:** one more span per traced request; the first function is INTERNAL;
  an Event-over-HTTP callee edge records its own round trip between the caller's span and `event.api.service`. Confirmed
  in Dynatrace by Eric (Scenario 9 and the token-bearing drive 9, `annotation.frames: 8`; reports in
  `docs/test-reports/`). Extends [[otel-forwarder-no-sdk]]; pinned by
  `event_over_http_stream::edge_relay_spans_are_connected`.
  <!-- id: connected-edge-spans | created: 2026-09-22 | last_used: 2026-10-02 | uses: 3 | tier: archive-candidate | origin: 2026-09-22-200854 -->

- **The Redis foundation retries intelligently — a heartbeat monitor plus one retry per lost connection for idempotent
  commands only, never a replay of a non-idempotent one (Eric's ruling on polyglot note 3, 2026-09-22; Increment 135, PR
  #320).** `redis-rs`'s `ConnectionManager` arms its reconnect when a command fails but returns that command's error
  (Lettuce requeues unwritten commands), so the first command after a Redis restart failed `broken pipe` and the second
  healed. `ConnectionLifecycle` (per `RedisBackend`, shared by its clones) tracks healthy/lost with drops/retries/recoveries
  counters; the heartbeat (`{prefix}heartbeat.ms` → `redis.heartbeat.ms`, default 1000, 0 = off, managed connections only)
  PINGs per interval — its failure makes the manager reconnect EAGERLY, so a command issued after it (a non-idempotent
  `RPUSH` included) succeeds first time; `RedisBackend::attempt(replay, op)` retries a `Replay::Idempotent` command exactly
  once when it failed while the connection was BELIEVED HEALTHY at issue time; a command issued while known-down makes one
  deadline-bounded attempt (408 while the manager reconnects, 503 on a refusal), never two; a timeout is never a lifecycle
  signal. **Why non-idempotent commands are never replayed:** on RESP2 the crate reports `broken pipe` both for a command
  it never sent and for one whose reply was lost (the RESP3 `Disconnection` push is not available to us), so non-delivery
  cannot be proven and an ambiguous `RPUSH` replay risks a duplicate segment (D7). Consumers: the cache marks
  GET/MGET/SETEX/MPUT/DEL/LLEN idempotent; the sync-over-async `ReturnRouteStore` runs on a `RedisBackend`
  (`connect_standalone`, the two-key `DEL` off the cluster path, its own 500 mapping as Java); `minigraph-state-redis`
  still drives its own manager (follow-up). Java needs no twin (Lettuce). Lesson: "fail fast" under a known outage means
  one deadline-bounded attempt, never two — a test expectation was wrong on the way, not the code. Extends
  [[redis-connection-foundation-rust]], [[redis-failure-classification-rust]]; closes the polyglot report's note 3.
  <!-- id: redis-restart-aware-retry | created: 2026-09-22 | last_used: 2026-10-04 | uses: 3 | tier: active | origin: 2026-09-22-235800 -->

- **A static decision table is GRAPH DATA — a skill-less node's properties, handed whole to a generic function by ONE
  `graph.task` input entry; never hard-coded in a function bundled with the graph (Eric, 2026-09-20; Increment 123, a doc
  gap and no engine change, PR #293; Java twin #430).** `initialize_with_node_properties` copies every node's properties
  into the state machine at instantiation (skill node → non-reserved keys at `{node}.{key}`; skill-less node → the whole
  map at `{node}`) and the shared LHS resolver reads any selector, so `state-rules -> table` maps the table in one entry.
  **Presentation (Eric):** each value is a JSON array written as text — `keys=[ "a", "b" ]`, `a=[ "CA", "TX" ]` — which
  reads as a table on the node and arrives as a string the function reconstructs (`serde_json::from_str`); `key[]=` lines
  build a real list; a nested table is one triple-quoted JSON text parsed by `f:json(state-rules.table)` at mapping time.
  **Why:** the product owner certifies the rules on the graph in the business vocabulary, and one table replaces a ladder
  of IF-THEN-ELSE. **The common case needs no function (Increment 124, PR #294; Java #431):** the `f:lookup(table, value,
  default)` simple plugin resolves the rule in one `graph.data.mapper` entry — table as map or JSON text, lists as lists
  or JSON arrays written as text, case-insensitive text compare, the optional third argument the default on a miss, the
  Java error messages verbatim. **A null mapping source — CHANGED 2026-09-23 (Increment 136, PR #323; mercury-composable
  #453):** Event Script's rule now applies — a null or unresolved source CLEARS a `model.*` target (removed; set to null
  when the source key exists or the target is indexed) and is IGNORED for any other target — via `common::apply_null_source`
  in the mapping entry, `for_each`, the `model.*` half of fetcher/extension parameters (a null parameter is not supplied)
  and the fetcher/task/extension output mapping; until 4.12.15 it removed ANY target (the claim `null-source-removes-target`
  now states the shared rule). A default for a model variable comes from the source side (the plugin's third argument or
  `f:defaultValue`), never from default-then-overlay; the same increment makes graph.math name every unresolved
  `{selector}` instead of the rendered text `null`. **Rule:** the product owner reads and certifies the table ON the graph,
  a new table is a new graph version and never a code change, and the function stays generic by reading rule names from
  `table.keys`. In `skills-reference.md`, the in-Playground help and the AI agent guide's checklist; pinned by
  `unit-test-task-9` (`graph_runtime.rs`) in lockstep with Java. Extends [[conventions-rust-baseline]].
  <!-- id: static-decision-table-is-graph-data-rust | created: 2026-09-20 | last_used: 2026-10-04 | uses: 4 | tier: active | origin: 2026-09-20-152809 -->

- **graph.math is typed and finite — a boolean is never a number, an unknown function and an overflow fail by name, and
  `CONDITION` is the declared boolean statement (Eric's rulings on a field page of nine "wrong answer" behaviours,
  2026-09-25; Increment 141, PR #330 squash `d97eab9b` MERGED 2026-09-25; the Java twin
  `feat/graph-math-condition-and-typed-arithmetic`, PR mercury-composable#462).** Both evaluators had the
  same shape — `as_number` coerced a boolean to 1/0 for arithmetic, `<`/`>` and function arguments while equality
  type-checked, `eval_call` failed generically, no finite check — so the same JSON `true` in a numeric slot computed
  three different ways and an overflow travelled on as `Infinity` to fail a later node as `Unknown identifier: Infinity`
  (the field's case: a boolean threshold negated into a number charged $3.5M where $1.5M was owed). **Rules, each a
  named failure and never a silent value:** `Boolean operand in '<op>': Boolean(true)` from the evaluator, mapped back to
  the selector by `name_offending_selectors` (the generalized `name_null_identifier`) as `Boolean operand: model.flag
  (true) in '…' - a boolean is not a number; store a boolean with CONDITION or assert the type with f:validate`;
  `eval_number` rejects a boolean RESULT; `Unknown function: mn` / `'PI' is not a function`; `finite()` on every unary,
  binary and call result (`Arithmetic overflow in '*' (result Infinity)`, `Division by zero or arithmetic overflow in
  '/'`, NaN by name). `CONDITION: var -> expr` substitutes in a logical context whatever operators it carries
  (`substitute_var_if_any_logical(text, state, true)`), evaluates with `eval_boolean`, stores a boolean at
  `{node}.result.{var}`; the compile gate counts it as a statement. **Minimalist boundary (Eric):** exact-decimal money
  (a rounding mode, integer cents) is NOT added to the dialect — a small composable function on `graph.task` with a
  decimal crate; the math package does not grow. **Documentation rulings, not engine changes:** `run` on the same
  Playground instance keeps `model.*` (a `model.x[]` append appends again) and `instantiate graph` / `start` is the
  reset (a fresh instance; a deployed graph gets one per request); a taken `IF` inside a `for_each` body ends the walk,
  so per-row rules are arithmetic gates; the end node is the terminus (last writer wins). READ: a graph that relied on
  `true`/`false` computing as 1/0, a boolean COMPUTE result storing 1.0, or `Infinity` propagating now fails at that
  statement by name. Pinned by `unit-test-math-2` (`graph_runtime.rs`) and `expression_engine.rs`, lockstep with Java.
  Extends [[static-decision-table-is-graph-data-rust]] (the same evaluator's null-source rule, Increment 136) and
  [[conventions-rust-baseline]]. **Partly superseded 2026-09-30 (Eric's rulings on RFC-0001, promoted to ADR-0025 in mercury-composable;
  Increment 145, shipped in v4.12.20):** the minimalist boundary no longer holds - exact decimal arithmetic IS a `graph.math` statement,
  `DECIMAL:` (canonical decimal strings at rest, rounding always explicit); the typed and finite `COMPUTE` rules above stand, as in the
  Java twin's note. (Found stale by the 2026-10-04 review's contradiction scan.)
  <!-- id: graph-math-typed-arithmetic-rust | created: 2026-09-25 | last_used: 2026-10-04 | uses: 5 | tier: active | origin: 2026-09-25-190229 -->

- **`graph.model.automation` accepts a comma-separated list of manifests, and the later manifest wins — the Rust twin
  (Increment 142, 2026-09-25; PR #332 merge `d3d82a3f` MERGED 2026-09-25, lock-step with mercury-composable #465 squash
  `40ce30a7`; SHIPPED in v4.12.19 on both engines).** Each manifest
  carries its own `location`, they compile in order, one that fails to load is skipped with a warning; `graphs.rs` records
  each graph's source location, and `list graphs` / the `import graph from` fallback span every location. **Rule (Eric):**
  the later manifest OWNS a duplicate id — its copy replaces the earlier one (`Graph X from B replaces the copy from A`) and
  a rejected later copy leaves the id not executable (404), never a silent fallback — because the prototyping loop is
  `import graph from` a deployed graph → correct → dry-run → export → stage in the deploy folder with its manifest → restart
  with BOTH manifests → curl the deployed behaviour → bundle. Here the override is a `-D` PROGRAM ARGUMENT
  (`overrides::apply_runtime_args`; `cargo run -p minigraph-playground -- -Dgraph.model.automation='classpath:/graphs.yaml,
  file:/tmp/graph/deploy/graphs.yaml'`), not a JVM flag. Entries are manifests, never bare folders (the manifest is the
  gate's allowlist). Claim `graph-manifest-list-later-wins` pinned to `compiler::later_manifest_wins_for_a_duplicate_graph_id`;
  the recipe lives in `ai-agent-guide.md#deploy-without-rebuild`.
  <!-- id: graph-manifest-list-later-wins-rust | created: 2026-09-25 | last_used: 2026-10-02 | uses: 5 | tier: archive-candidate | origin: 2026-09-25-224149 -->

- **The graph.math expression dialect is documented as the closed set it is, and pinned — the Rust twin of mercury-composable
  #467/#468 (Increment 143, 2026-09-28; PR #334 merge `41a7f418` MERGED 2026-09-28; SHIPPED in v4.12.20).** A live MiniGraph demo showed the gap: an agent building
  `a + b ** 2` had to read the evaluator to know whether `**` parses — the grammar page said "no function calls", the skills
  reference listed a partial function set, no page named the operators. Now `skills-reference.md#math-dialect` lists everything a
  `COMPUTE`/`CONDITION`/`IF` may contain (literals; `{…}` variables and how they render — a text value becomes a quoted string
  literal in a boolean context; operators by precedence with the strict `**` unary rule; the eighteen functions with arity, all
  under `Math.` too; `PI`, `E`; the exclusions), the grammar summary links to it, the in-band `help graph-math` carries the same
  catalog, and `minigraph-commands.json` has an `expression_dialect` object. **Gated:** claim `math-expression-dialect` →
  `tests/expression_dialect.rs`, set-equality on `EvalContext::with_defaults()` through the new `EvalContext::names()` /
  `namespace_names()` (the Java `snapshot()` analog; the one code change, no behaviour change) — a function or constant added or
  removed fails the build on both engines. **Rule:** a dialect is a closed set; document it as one and pin the set, or every agent
  re-derives it from source. Grouped one concern per test from the start (Sonar S5961 flagged the Java original at 29 assertions
  in one method — #468). Extends [[graph-math-typed-arithmetic-rust]] and [[conventions-rust-baseline]].
  <!-- id: graph-math-dialect-closed-set-rust | created: 2026-09-28 | last_used: 2026-09-28 | uses: 1 | tier: archive-candidate | origin: 2026-09-28-234016 -->

- **The starter template and the playground example carry their own flows config, mimicking the Java twins, and tutorial 13 is deployed in the example (Eric,
  2026-10-01; Increment 149, PR #343 merge `86b59cb0`).** `templates/starter-graph` gained `resources/flows.yaml` and `flows/graph-executor.yml`; `examples/minigraph-playground`
  gained `flows.yaml`, `flows/graph-executor.yml` and `flows/flow-11.yml`. The flow files are byte-identical with the Java template's and the Java example's (the example's equal the
  engine crate's defaults); the two manifests carry a short comment. **The resolution rule, proved by deletion controls rather than assumed:** the application's own `resources`
  come first (`auto_start_main!` prepends them), the engine crate's root is appended, and a file the application lacks falls through to the engine's PER FILE. So the copies are
  redundant at run time (delete them and the tests still pass), but they are what a developer reads, and the application's `flows.yaml` SHADOWS the engine's: a manifest that omits
  `flow-11.yml` breaks tutorial 11 (`flow://flow-11 does not exist`), and one that lists a missing flow breaks the graph endpoint (`Flow graph-executor not found`). A stale copy would also
  hide an engine fix, so `examples/minigraph-playground/tests/tutorials.rs` keeps the sample flow files equal to the engine's defaults (the template's apart from its opening comment).
  **Tutorial 13** was left out of the example's manifest behind a comment that it needs `v1.hello.task`, which Increment 83 retired when tutorial 13 became an `async.http.request` client of the
  app's own dev mock endpoint; the comment outlived it (the Java example omitted tutorial 13 for the same reason, fixed in mercury-composable #489). The app now compiles 15 graphs, and the test
  boots on a KNOWN port because CompileGraph resolves `${rest.server.port:8080}` at load time. `v1.hello.task` is NOT re-added (Eric: tutorial 13 no longer needs it). Not aligned: the
  distributed-cache example lists `graph-executor.yml` and resolves it from the engine without a local copy. Follow-up: [[hello-task-doc-references]] (closed 2026-10-02: PR #345 and #346).
  <!-- id: example-and-template-carry-their-flows | created: 2026-10-01 | last_used: 2026-10-02 | uses: 4 | tier: active | origin: 2026-10-02-001532 -->

- **The Rust engine certified the LLM helper without changing: the playground's AI nodes point at the helper app, and the helper's contract lives in the language packs (Increment 148, PR #342
  merge `755af30e`; Eric, 2026-10-01).** The helper (`llm.chat`, `llm.stream`, `llm.health` on the Anthropic SDK) is `examples/llm-helper` in mercury-python and mercury-nodejs (PRs #38 and #106),
  not engine code: the engine stays LLM-free and holds no credential. The playground's `support-triage` graph (byte-identical with Java's) and the `/api/llm/stream` relay reach it through
  `event-over-http.yaml` by route name. `docs/test-reports/llm-helper-certification.md` (byte-identical with the Java copy) records Java and Rust in front of both helpers, through a Layer 1
  streaming service, a Layer 2 flow and two Layer 3 graphs, with real Claude calls (40 results per pair, 124 model calls): every batch the helper forwarded reached the edge as its own frame
  (the cadence is the API's and depends on the model), the error contract holds on the real SDKs, and every trace is one tree ([[connected-edge-spans]]). The no-rebuild lane did the
  deploying ([[graph-manifest-list-later-wins-rust]]); Rust's `yaml.rest.automation` reads ONE location, so the chat flow's REST entry went into one combined `rest.yaml`. Opus 5.5, the helper's
  default model and kept by Eric, thinks before it answers and its thinking tokens count against `max_tokens`: the triage graph now asks for 2000 tokens (512 before) and the README's stream
  example for 2000 (300). AWS Bedrock through IAM is the helper's planned second backend, a thread in the packs.
  <!-- id: llm-helper-certification-rust | created: 2026-10-01 | last_used: 2026-10-02 | uses: 1 | tier: archive-candidate | origin: 2026-10-02-001532 -->

- **A mock-data upload travels like a command: it loads every member's instance (Eric's design, 2026-10-02; PR #349 merge `278bb023`, Increment 154; lock-step with mercury-composable #498 squash `ce0e7155`; both MERGED 2026-10-03 06:00Z).**
  `commands::upload_content` (REST `POST /api/mock/{id}`) no longer writes the uploader's instance alone: it sends an `upload` event to the command service,
  `handle_upload` loads the payload when the session is the primary and replays it (`forwarded`) into every subscriber's instance, or forwards a subscriber's payload to
  the primary (which replays it back), and `load_mock_content` sets `input.body` and confirms in that member's console (`Mock data loaded into 'input.body' namespace`);
  a session without an instance is refused at the REST edge. **Why:** another member's replayed `run` executed without the data and aborted. The Playground's run
  controls became three steps in the same round - Instantiate, Upload (optional; the form opens for the clicking session only, no console command) and Run - and the
  multi-select hint left the canvas; the UI lives in the Java repo and arrives here as the bundle `index-Bg13jQpc` ([[webapp-single-source-java-repo]]). Pinned by
  `mock_upload_loads_every_member_instance` in `tests/graph_runtime.rs`.
  <!-- id: mock-upload-loads-every-member | created: 2026-10-02 | last_used: 2026-10-03 | uses: 2 | tier: active | origin: 2026-10-02-232252 -->

- **A graph model imported from a file travels like a command: `POST /api/graph/import/{id}` makes it every member's draft (Eric's Playground usability sprint, 2026-10-03; PR #350 merge `dae6377d`, Increment 155; lock-step with mercury-composable #500 squash `856e084b`; both MERGED 2026-10-03 16:03Z).**
  `commands::import_content` validates first (`validate_graph_model`: a JSON object whose only top-level sections are `nodes`, a mandatory list, and `connections`, an optional list; then
  `MiniGraph::import_graph` on a scratch graph, so a node without alias or types is refused at the edge) and sends an `import` event to the command service; `handle_import` replaces the
  draft when the session is the primary and replays it (`forwarded`) into every subscriber's draft, or forwards a subscriber's model to the primary; `import_graph_model` (shared with
  `import graph from`) clears a graph instance and says `Graph model imported as draft` - the line the webapp refreshes on - and says `Graph model not imported - <reason>` for a model
  the importer rejects instead of leaving an empty draft silently. An unknown session is 404. Simple validation only; CompileGraph stays the quality gate (Eric). The UI arrives as the
  bundle `index-Cv2pdvxg` ([[webapp-single-source-java-repo]]): the Import Graph button, the `.json` file drop (a confirmation before replacing a loaded graph), the Download button
  (`<graph-id>.json`, the root node named after the id as `export graph as` does) and the Raw tab. A dev-route addition touches the example, the starter template and the cache example
  `rest.yaml`. Extends [[mock-upload-loads-every-member]]. Pinned by `graph_import_loads_every_member_draft` in `tests/graph_runtime.rs`.
  <!-- id: graph-import-travels-like-command | created: 2026-10-03 | last_used: 2026-10-03 | uses: 1 | tier: working | origin: 2026-10-03-153752 -->

- **A mapping source inserts its `{namespace.key}` values verbatim, never quoted; a JSONPath filter is the one place a text value is quoted (Eric's ruling, 2026-10-03;
  PR #351 merge `5c408037`, Increment 156, lock-step with mercury-composable #503 squash `8db2a46f`; both MERGED 2026-10-04 06:01Z).** A graph mapping source may embed a reference that resolves before the source
  is read: a key segment (`census-2020.{model.state}`, the keyed-table read beside `f:lookup`), a list index (`items[{model.i}]`), or text in a constant or a plugin
  argument, in `mapping[]`, `MAPPING:`, `for_each[]`, the task/extension/fetcher `input[]`/`output[]` and a Dictionary's `output[]` (`common::substitute_mapping_source`
  at the six call sites; `substitute_var_if_any` stays for expressions and statement commands). By design (Eric, 2026-10-03): an unresolved reference renders `null` and a
  mapping target is never resolved (literal); also documented: any namespace may be read (Event Script: `model.*` only), a composed key is case-sensitive. Pinned by the byte-identical fixture `unit-test-mapping-1`,
  `mapping_source_resolves_dynamic_variables_verbatim` and the claim `mapping-source-dynamic-variables`. Extends [[static-decision-table-is-graph-data-rust]]; applies
  [[webapp-single-source-java-repo]] (bundle `index-Bz9k-ffR`). The JSONPath result shape follows Jayway since Increment 157: [[jsonpath-jayway-result-shape]].
  <!-- id: mapping-source-verbatim-substitution | created: 2026-10-03 | last_used: 2026-10-04 | uses: 2 | tier: active | origin: 2026-10-04-054444 -->

- **A `$.` JSONPath result takes Jayway's shape - by the kind of path, not the number of matches (Increment 157, PR #352 merge `bdc7b5be`, MERGED 2026-10-04 06:08Z;
  the Java engine is the reference, its pin mercury-composable #504).** A definite path (child member names and single indexes only) yields the value or nothing; an
  indefinite path (a filter, a wildcard, a descendant segment, a slice or a union) always yields a list, `[x]` for one match and `[]` for none, except that a missing
  member name before its first indefinite step, or a name applied to a non-object, is not found (a missing index there only empties the list) - probed against Jayway
  3.0.0. `serde_json_path` keeps its parsed query private, so `mlm.rs` classifies the parsed path string (`path_shape`, `misses_a_member_name`); an unrecognized
  construct keeps the count rule. Holds in Event Script flows and graphs alike; pinned by `json_path_result_shape_follows_jayway`, the shared fixture
  `unit-test-jsonpath-1` and the claim `json-path-result-shape`. Closed [[rust-jsonpath-indefinite-list]].
  <!-- id: jsonpath-jayway-result-shape | created: 2026-10-03 | last_used: 2026-10-04 | uses: 1 | tier: working | origin: 2026-10-04-060541 -->
- **A graph holds no null property: `"key": null` is filtered out on deploy, pack and read, `"key": ""` is a value, and `serializer.null.transport` does not
  apply (Eric's rulings, 2026-10-05; Increment 158, PR #356, open at writing; Java twin `graph-null-property-filtered` in mercury-composable #508).** The Java
  configuration reader drops a null-valued key when it loads a graph; this reader kept it and `MiniGraph::import_graph` refused it, so a graph deployed on Java
  only. `model_gate::without_null_properties` (= `serializer::strip_nulls_always`, the transport strip without its switch: map values only, list elements and
  empty collections kept) runs in `graph_set::pack`/`read` and in the Playground's draft import (`import_graph_model` + `validate_graph_model`: the REST
  import, its replay and `import graph from`, which reads a file as text). **The deploy read reproduces Java's (Eric approved options A and E, 2026-10-05):**
  Java's flatten-and-rebuild also drops an empty map or list (and one left empty), turns such an element inside a list into null and drops it at the end;
  `model_gate::normalize_graph` does the same in `compiler::load_raw_graph` (startup and `instantiate graph`) and the packager's check copy, pinned by
  `tests/resources/graph-read-normalization-vectors.json`, byte-identical with the Java engine's. READ: a graph refused for a null property or a mapping list
  ending in null now deploys; an empty `{}`/`[]` property is no longer deployed. The switch governs only what the event transport keeps.
  <!-- id: graph-null-property-filtered-rust | created: 2026-10-06 | last_used: 2026-10-06 | uses: 1 | tier: working | origin: 2026-10-06-000759 -->

## Conventions

> Established with the first code (increment 1, 2026-07-15); enforced from the first commit.

- **`cargo fmt` + `cargo clippy --all-targets` clean, and `cargo audit` clean** is part of "done" for every
  change (default settings, no custom rustfmt.toml yet; the RustSec audit runs in CI on every PR and weekly since
  Increment 138, 2026-09-24 — its first run closed `rustls` RUSTSEC-2026-0285 with a lock refresh).
- **Apache-2.0 header** comment on every source file (ported from the Java originals'
  header style). EXCEPTION ruled by Eric 2026-09-11: `templates/*` starter sources carry a
  ONE-LINE scaffold attribution instead — templates seed field applications that are not
  open source, so the full Accenture copyright header must not ride into user code.
- **Release version bumps must include the starter templates (2026-09-11).** Each
  `templates/*/Cargo.toml` carries an EXPLICIT `version = "<version>"` and mercury-* dep
  pins (deliberately NOT workspace-inherited, so a copied-out template builds as-is after
  deleting the in-repo `path` keys) — the release edit list grows from 5 manifests to 8.
  **Extended 2026-09-22 (the v4.12.14 publish):** before a crate's FIRST publish, audit its manifest metadata as part of the release sweep — `keywords` ≤ 5 and each ≤ 20 characters, valid `categories`, a `readme` path inside the package — because cargo validates none of it locally and crates.io rejects at upload, after the dependency-ordered run has already published everything before it (`progressive-rendering`, 21 chars, cost `mercury-sync-over-async` its place in the 4.12.14 run).
  <!-- id: conv-template-version-sweep-rust | created: 2026-09-11 | last_used: 2026-09-25 | uses: 10 | tier: archive-candidate | origin: 2026-09-11-005808 -->
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

- **CI floats on `stable`, so a Rust release can turn `main` red with no change: diagnose by the failing step and the crate, and fix `main` first (2026-10-01; PR #344 merge `2ed5f29d`).**
  `dtolnay/rust-toolchain@stable` has no repo pin. On 2026-10-01 stable became 1.99.0 and its clippy (`double_must_use`) flagged the `#[must_use]` that async-trait 0.1.89 adds to every async trait
  method, at three trait methods of `mercury-platform-core`; `cargo clippy --workspace --all-targets -- -D warnings` stopped there, so the Clippy step failed on every branch (#342, #343) while
  neither PR touched `crates/`. The fix was `Cargo.lock` only: async-trait 0.1.92 stopped emitting the attribute, a root-cause fix and not a lint allow. **Lessons:** (1) read the failing step and
  check the diff for the failing crate before blaming the change; (2) re-running a failed job re-tests the SAME merge commit, so a fix landed on `main` reaches an open PR only through a new push
  (a rebase or GitHub's "Update branch"); (3) merge the fix PR first, and rebase the PR branch onto `main` so its CI tests the combined state; (4) `main` has no branch protection, so only discipline stops
  a red merge (#343 merged red while the fix PR was green and waiting); (5) a local toolchain behind CI cannot reproduce it, so CI is the check (or `rustup update stable`).
  <!-- id: ci-floats-on-stable-toolchain | created: 2026-10-01 | last_used: 2026-10-02 | uses: 2 | tier: archive-candidate | origin: 2026-10-02-001532 -->

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
  <!-- id: webapp-single-source-java-repo | created: 2026-10-02 | last_used: 2026-10-04 | uses: 5 | tier: active | supersedes: webapp-bundle-follows-help-edits | origin: 2026-10-02-180239 -->

- **A build from this checkout can reuse an engine artifact compiled in a worktree, and the engine's resource root is baked at compile time (found 2026-10-02).**
  `mercury-knowledge-graph` registers its `resources/` with `concat!(env!("CARGO_MANIFEST_DIR"), "/resources")` (`GraphResources`, `lib.rs`), so an rlib compiled in a worktree keeps the worktree's
  path, and cargo judged such an artifact fresh from this checkout: the Playground example then answers 404 for `/template/playground.html` and rejects every tutorial (`classpath:/graph/tutorial-N.json
  not found`) although the files are in place. **Rule:** after a worktree build, `touch crates/knowledge-graph/src/lib.rs` (or `cargo clean -p mercury-knowledge-graph`) before `cargo run`, and read
  the baked root with `strings target/debug/minigraph-playground | grep knowledge-graph/resources`; a 404 on the Playground page right after a deploy is this, not the deploy. Relates
  [[webapp-single-source-java-repo]].
  <!-- id: worktree-build-bakes-resource-root | created: 2026-10-02 | last_used: 2026-10-03 | uses: 3 | tier: active | origin: 2026-10-02-183818 -->

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
