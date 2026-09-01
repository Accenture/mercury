- [x] (feature — **MERGED 2026-08-29 as
  [PR #217](https://github.com/Accenture/mercury/pull/217) merge `ec4c2702` carrying
  gated `7f5681b5` (tree verified), CI green, branch deleted both ends**) **Progressive
  SSE consumption in the HTTP client — Java Phase 1 twin (Java PR #300/ADR-0019 →
  this repo's ADR-0016 Proposed, Increment 92).** Accept-gated activation (D1);
  spawned tokio relay task per stream (worker freed; per-read idle allowance =
  request TTL, comments reset it; in-band 408 "Timeout for N seconds"/500);
  SseParser incremental frame parser; buffered fallback + no-Accept backward-compat;
  self-relay e2e. Same PR: **structure parity** — draft-design-specs/ created
  (lifecycle README), the six docs/design/ port docs MOVED into it (23 references
  re-pointed; docs/design removed; site-neutral), Java's docs/css/extra.css adopted
  (it was live there, missing here). Lessons: Platform::new() = isolated registry
  (tests use the server()-returned handle); the mock upstream needed its own
  thread+runtime (third per-test-runtime catch). Next: Phase 2 hybrid envelope-mode
  relay (engine⇄engine), Phase 3 wrapper twins. origin: 2026-08-29-174058.
  <!-- id: thread-sse-client-twin | created: 2026-08-29 | last_used: 2026-08-30 | uses: 3 | tier: active | origin: 2026-08-29-174058 -->
