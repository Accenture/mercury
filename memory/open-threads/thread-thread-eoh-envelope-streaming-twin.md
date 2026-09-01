- [x] (feature — **MERGED 2026-08-30 as
  [PR #218](https://github.com/Accenture/mercury/pull/218) merge `1723ace6` carrying
  gated `54436ca4` (tree verified), CI green, branch deleted both ends**)
  **Event-over-HTTP peer streaming in envelope mode — Java Phase 2 twin (Java PR #301;
  this repo's ADR-0015/0016 flipped ACCEPTED, Increment 93). Phase 2 is lock-step on
  both engines.** Send with reply_to + `accept: text/event-stream` event header relays
  a remote streaming function through /api/event on one call; hybrid dialect (envelope
  frames for head/terminals/non-text, raw frames for tokens); pinned 406/503 refusals;
  error-triple alignment (writer fail / client in-band / renderer idle terminal). Port
  idioms: the EDGE decides the mode (stream_dispatch envelope_mode inherits the whole
  lane lifecycle); EventApiService became a true event INTERCEPTOR; one wrap site
  (capable-path errors ride raw; the edge wire-wraps unmarked lane replies — outer
  status 200 with the real status packed, caller-visible envelope identical to Java).
  Lesson: the suite's first run caught the unpacked single-shot reply — wrap at the
  SingleShot outcome. 14 tests on a shared dedicated-thread fixture (process-wide
  registry) with runtime-written config on ephemeral ports. Next: Phase 3 wrapper
  twins (mercury-python/nodejs). origin: 2026-08-30-003334.
  <!-- id: thread-eoh-envelope-streaming-twin | created: 2026-08-30 | last_used: 2026-08-30 | uses: 1 | tier: working | origin: 2026-08-30-003334 -->
