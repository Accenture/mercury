- [x] **(blueprint — CLOSED at the closure gate 2026-09-17, stalled 93 sessions)** Port the lightweight
  cloud-native connectors + sync-over-async. Eric: "close bp-kafka-connectors-backlog because the
  service-mesh using kafka is not in scope for Rust. Minimal-kafka is in scope." Outcome: sync-over-async
  shipped (`ot-sync-over-async-port`, R1–R4, closed 2026-09-13); minimalist-kafka is in scope and mid-flight
  under its own thread `ot-minimalist-kafka-port` (K1–K2 done, K3 next; now traced `→ serves: vision-mercury`);
  twin-kafka stays deferred (port spec Q3); the Kafka service mesh stays out of scope. → serves: vision-mercury
  <!-- id: bp-kafka-connectors-backlog | created: 2026-07-20 | last_used: 2026-09-17 | uses: 6 | tier: archive-candidate | origin: 2026-07-20-030615.md -->
