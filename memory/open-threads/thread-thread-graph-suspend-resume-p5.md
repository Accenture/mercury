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
