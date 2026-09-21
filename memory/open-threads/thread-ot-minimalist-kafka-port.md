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
  PR pending): second-level routing (`flows` rules → `flow://` | `task://`, validated against
  the live registries), `topic-pattern` (anchored regex subscribe), `partition` pinning
  (`assign`), `auto-commit` + an explicit `max-poll-records` → `queued.min.messages`,
  per-binding header overrides, `serializer: 'json'`, `ttl`, the derived
  `max.poll.interval.ms`, `group.protocol=auto` → classic (WARN). The full Java validation
  table; `schema.enabled` alone still deferred by name (Q2). 34 unit + the 14-scenario e2e
  against MockCluster. Deltas: spec §7 items 5, 7, 9, 10. **Open (Eric):** `auto` = classic
  on librdkafka (no feature probe) — accept as the delta, or a trial-join probe at K4?
  Next gate: K4 — live dry-run + interop against `kafka-standalone`: port `kafka-demo` (Q4),
  Rust↔Java flow adapters both ways, DLQ + rebalance chaos, the report kept as permanent
  record. **K5 docs items:** the `docs/guides/minimalist-kafka.md` twin for the AI contract,
  the README non-goals paragraph (still says minimalist-kafka is "planned"), one INCREMENTS
  entry for the whole port (K1–K5 record in spec §8 until then).
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
  <!-- id: ot-minimalist-kafka-port | created: 2026-09-14 | last_used: 2026-09-21 | uses: 6 | tier: working | origin: 2026-09-14-015014 -->
