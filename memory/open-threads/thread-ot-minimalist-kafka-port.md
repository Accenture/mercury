- [ ] (port — minimalist-kafka, 2026-09-14) **Bring the Kafka flow adapter + notification
  library to the Rust engine** — inbound topics→flows (groups, patterns, pinning,
  second-level routing, retry+DLQ, delivery modes), outbound `simple.kafka.notification`,
  `kafka.health`, externalized client templates. Design spec:
  `draft-design-specs/minimalist-kafka-port.md` — APPROVED 2026-09-14, Q1–Q5 ruled (§9):
  `system/minimalist-kafka` / `mercury-minimalist-kafka` (Q1); Schema Registry deferred to
  its own post-K5 spec (Q2); twin-kafka deferred (Q3); port `kafka-demo` at K4 (Q4);
  MockCluster ratified (Q5). Next gate: K1.
  Client decision: `rdkafka` (only maintained Rust client with the full group protocol;
  vendored librdkafka builds with cc+make, no CMake — verified). Unit tests:
  `rdkafka::mocking::MockCluster` (mockforge-kafka 0.3.221 investigated head-to-head and
  not adopted: 338-crate tree, topic auto-create unreachable by a real client's metadata
  path, malformed LeaveGroup v1 — evidence in spec §5). Integration/interop: the Java
  `kafka-standalone` + `schema-registry-standalone` helpers (staged in `helpers/`, Eric).
  **This arc gates two held items** (Eric, 2026-09-14): the `mercury-sync-over-async`
  crates.io publication and the sync-over-async Q1 facade tasks both wait for K5.
  <!-- id: ot-minimalist-kafka-port | created: 2026-09-14 | last_used: 2026-09-14 | uses: 1 | tier: working | origin: 2026-09-14-015014 -->
