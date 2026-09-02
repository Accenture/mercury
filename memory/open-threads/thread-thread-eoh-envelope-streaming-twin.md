- [x] (feature) **Event-over-HTTP peer streaming in envelope mode — Java Phase 2 twin;
  Phase 2 lock-step on both engines.** MERGED 2026-08-30 as PR #218 (merge `1723ace6`),
  CI green; ADR-0015/0016 Accepted; Increment 93. Durable lesson: the EDGE decides the
  mode — one wrap site (the edge wire-wraps unmarked lane replies); the suite's first
  run caught the unpacked single-shot reply. origin: 2026-08-30-003334
<!-- id: thread-eoh-envelope-streaming-twin | created: 2026-08-30 | last_used: 2026-08-30 | uses: 1 | tier: archive-candidate | origin: 2026-08-30-003334 -->
