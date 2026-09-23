- [x] (port — minimalist-kafka) **CLOSED 2026-09-23 at the v4.12.15 seam — the port SHIPPED in v4.12.14.** K1–K5 (outbound +
  `kafka.health`, the inbound adapter, second-level routing with the full validation table, the `kafka-demo` twin driven
  live with the Java demo, the Schema Registry wire format on a native codec, the sync-over-async facade) plus optimistic
  `group.protocol=auto` (#302) and the SIGTERM stop (#301) — released as mercury #308 (`b83c493f`, tag v4.12.14) and
  published to crates.io 2026-09-22 (`mercury-minimalist-kafka` and `mercury-sync-over-async` first-time — both held items
  released); the graceful-shutdown contract followed in 4.12.15 (#312/#313). Still deferred by ruling: twin-kafka (Q3).
  Lesson: a configuration-only activation still needs one use line per crate (Finding 5), and `auto` needed no probe — a
  refused join is the probe. origin: 2026-09-14-015014 … 2026-09-21-233114; close 2026-09-23-014725.
  → served: vision-mercury
  <!-- id: ot-minimalist-kafka-port | created: 2026-09-14 | last_used: 2026-09-23 | uses: 13 | tier: active | origin: 2026-09-14-015014 -->
