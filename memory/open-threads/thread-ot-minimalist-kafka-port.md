- [ ] (port — minimalist-kafka, 2026-09-14) **Bring the Kafka flow adapter + notification
  library to the Rust engine** — inbound topics→flows (groups, patterns, pinning,
  second-level routing, retry+DLQ, delivery modes), outbound `simple.kafka.notification`,
  `kafka.health`, externalized client templates. Design spec:
  `draft-design-specs/minimalist-kafka-port.md` — APPROVED 2026-09-14, Q1–Q5 ruled (§9):
  `crates/minimalist-kafka` / `mercury-minimalist-kafka` (Q1 — clarified: `crates/` is the Java `system/` analog; the dev double moved to `extensions/redis-test-double` under the same convention); Schema Registry deferred to
  its own post-K5 spec (Q2); twin-kafka deferred (Q3); port `kafka-demo` at K4 (Q4);
  MockCluster ratified (Q5).
  **K1 DONE 2026-09-14** (origin: this session's log): `crates/minimalist-kafka` — outbound
  contract + `kafka.health` + template pipeline + auto-activation via inventory; 16 tests
  green against MockCluster; platform-core gained `PostOffice::my_span_id()` + public
  `w3c_trace`. Partitioner delta: librdkafka `murmur2_random` config replaces the Java
  partitioner class (same semantics, Java-compatible key hash — spec §7 item 4).
  **K2 DONE 2026-09-14** (origin: this session's log): the inbound adapter core — YAML
  validation (later-increment fields rejected BY NAME), one consumer task per binding,
  commit-after-process via block_in_place, retry + confirmed DLQ with origin facts,
  DATA-LOSS liveness drop; e2e through the REAL flow engine against MockCluster,
  configuration only. New deltas: max.poll.records needs no analog (per-record recv);
  the one-use-line linker caveat for pure-config activation; group.protocol=auto
  deferred to K3 (spec §7 items 5-7).
  **K3 DONE 2026-09-21** (origin: 2026-09-21-170607.md; branch `feat/minimalist-kafka-k3` `887bea91`,
  PR #299 MERGED 2026-09-21, merge `d58b1ef2`): second-level routing (`flows` rules → `flow://` | `task://`, validated against
  the live registries), `topic-pattern` (anchored regex subscribe), `partition` pinning
  (`assign`), `auto-commit` + an explicit `max-poll-records` → `queued.min.messages`,
  per-binding header overrides, `serializer: 'json'`, `ttl`, the derived
  `max.poll.interval.ms`, `group.protocol=auto` → classic (WARN). The full Java validation
  table; `schema.enabled` alone still deferred by name (Q2). 34 unit + the 14-scenario e2e
  against MockCluster. Deltas: spec §7 items 5, 7, 9, 10. **Open (Eric):** `auto` = classic
  on librdkafka (no feature probe) — accept as the delta, or a trial-join probe at K4?
  **K4 DONE 2026-09-21** (origin: 2026-09-21-175430.md; branch `feat/minimalist-kafka-k4` `e9f07188` + `30cdc289`, PR #300 MERGED 2026-09-21, merge `4363bae9`):
  `examples/kafka-demo` ported one to one (four functions + Java tests, three flows, two-binding
  adapter YAML, Node helpers copied, an `interop` relay profile) and driven live on the Java
  `kafka-standalone` (Kafka 4.3.1) with the Java demo: routing both styles, the DLQ path, both
  chained interop legs under one trace id (span chain exact across the boundary), a mixed
  Java+Rust consumer group (20 split 10/10), a SIGKILLed member taken over at the 45 s session
  timeout (20/20 once; an in-flight poison order redelivered to Java and dead-lettered), SIGINT
  graceful LeaveGroup. Report `docs/test-reports/minimalist-kafka-interop.md`. Finding fixed in
  platform-core: [[headless-app-keep-running]] (a headless app exited after boot). Follow-up
  recorded, platform-wide: `SIGTERM` not handled by `AutoStart::run` (backlog P11). Rulings at the
  gate (Eric): `auto` = classic accepted (spec §7 item 7), helpers copied, chained-hop shape.
  **`group.protocol=auto` RE-RULED and IMPLEMENTED 2026-09-21** (origin 2026-09-21-184821.md; branch
  `feat/kafka-group-protocol-auto` `0beba0f8` + `03959e60`, PR #302 MERGED 2026-09-21, merge `0432ed42`): optimistic `auto` —
  `consumer` first, rebuilt once as `classic` on librdkafka's fatal `ConsumerGroupHeartbeat` refusal;
  both engines ship `auto` in the bundled template (Java `feat/kafka-group-protocol-auto-default`
  `ee37b907`, PR #436 MERGED squash `7def2bb4`). Proven live both ways on the standalone broker (spec §7 item 7; report addendum).
  P11 (`SIGTERM`) closed on `fix/sigterm-graceful-stop` `5688e643` = PR #301, MERGED 2026-09-21 (`922e13c7`).
  **K5 RULED and DONE 2026-09-21** (origin: 2026-09-21-233114.md; Eric: port the Schema Registry if viable, mirror the Java
  demo, v4.12.14 on both engines after the lock-step round). **K5a** `feat/minimalist-kafka-schema-registry`
  `0da85068` — the Confluent wire format for JSON Schema and Avro on this engine's own codec (`apache-avro`,
  `jsonschema`), subject-driven produce, decode by embedded id, positive-only caches, the interpreted
  `schema-registry.yml` (OAuth 2.0 client credentials); CSFLE / rules / references / Protobuf refused with a
  501 (spec §7 items 12–16, §9 Q2 re-ruled; [[schema-registry-native-codec]]). **K5c docs**
  `docs/minimalist-kafka-guide` `424e1515` (stacked) — the guide twin, nav, llms, files.list, the config
  reference's Kafka section, two claims, README + port-scope. **K5b** `feat/sync-over-async-facade`
  `cca0c1bf` (stacked) — `sync.prepare`/`sync.await`/`soa.reply` + the extension's auto-start, the
  `RestFlowMvpTest` twin, the demo mirrored (facade/backend, raw/JSON/Avro legs), the live drive with the Java
  demo (report addendum Scenarios 8–13: frames decoded both ways by both engines, a mixed backend group, the
  Java facade delivering replies to the waiting Rust facade through the Redis return route). Finding 5: the
  configuration-only link line applies to every crate (the demo shipped without `http.flow.adapter`).
  **Lock-step round** `feat/lockstep-4-12-13` `48506b99` (from main; Increment 128): the task↔skill gate,
  the case-insensitive `input.header.*` fallback, snake_case log-context keys + automatic timestamp; P2
  confirmed no change; **P10 needs a ruling**. PRs: **#303 = K5a + the docs twin** (opened 2026-09-21 from `docs/minimalist-kafka-guide`, which is
  stacked on K5a — two commits; CI pending); **K5b = #304** (opened 2026-09-21, stacked on K5a — carries `0da85068` until #303 merges); **#305 opened from the K5a branch by mistake** (duplicates #303's first commit; close-and-reopen from `feat/lockstep-4-12-13` recommended); the lock-step branch awaits its PR; Java release = mercury-composable #437. Increment 127 records the whole port.
  Next gate: **the release** — after the four Rust PRs and the Java `release/v4.12.14` PR merge, the Rust
  release branch (4.12.12 → 4.12.14 in one step) and the crates.io publication of `mercury-sync-over-async`
  + `mercury-minimalist-kafka` (Eric publishes).
  Client decision: `rdkafka` (only maintained Rust client with the full group protocol;
  vendored librdkafka builds with cc+make, no CMake — verified). Unit tests:
  `rdkafka::mocking::MockCluster` (mockforge-kafka 0.3.221 investigated head-to-head and
  not adopted: 338-crate tree, topic auto-create unreachable by a real client's metadata
  path, malformed LeaveGroup v1 — evidence in spec §5). Integration/interop: the Java
  `kafka-standalone` + `schema-registry-standalone` helpers (staged in `helpers/`, Eric).
  **This arc gates two held items** (Eric, 2026-09-14): the `mercury-sync-over-async`
  crates.io publication and the sync-over-async Q1 facade tasks both wait for K5.
  → serves: vision-mercury (the connectors Blueprint gap closed at the 2026-09-17 gate; this port is
  the live Vision-serving work — Eric: minimal-kafka is in scope, the Kafka service mesh is not)
  <!-- id: ot-minimalist-kafka-port | created: 2026-09-14 | last_used: 2026-09-21 | uses: 9 | tier: working | origin: 2026-09-14-015014 -->
