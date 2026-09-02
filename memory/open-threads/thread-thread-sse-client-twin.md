- [x] (feature) **Progressive SSE consumption in the HTTP client — Java Phase 1 twin
  (ADR-0016, Increment 92).** MERGED 2026-08-29 as PR #217 (merge `ec4c2702`), CI green;
  same PR created the draft-design-specs/ structure parity. Durable lessons:
  Platform::new() = isolated registry (tests use the server()-returned handle); a mock
  upstream needs its own thread+runtime. origin: 2026-08-29-174058
<!-- id: thread-sse-client-twin | created: 2026-08-29 | last_used: 2026-08-30 | uses: 3 | tier: archive-candidate | origin: 2026-08-29-174058 -->
