- [x] (feature) **HTTP response streaming ported engine-identical (Java PR #299/ADR-0018
  twin).** MERGED 2026-08-29 as PR #216 (merge `b191cf1e`), CI green; Increment 91.
  Reply-lane pool of 500 + HTTP-503 on exhaustion; the renderer-task idle deadline
  replaces Java's housekeeper (wire-identical). Known deferral: the x-stream-id relay.
  origin: 2026-08-29-012914
<!-- id: thread-http-streaming-twin | created: 2026-08-29 | last_used: 2026-08-30 | uses: 4 | tier: archive-candidate | origin: 2026-08-29-012914 -->
