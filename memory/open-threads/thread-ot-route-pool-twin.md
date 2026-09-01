- [x] (feature — **MERGED 2026-08-30 as
  [PR #220](https://github.com/Accenture/mercury/pull/220) merge `9dea9182` carrying
  gated `d753d004` (tree verified), CI green, branches deleted both ends; ADR-0017
  Accepted via the merge (`e2914058`); Increment 94; rides the next release via
  CHANGELOG Unreleased) **Route pool platform API — register_route_pool/
  release_route_pool, the Java engine's registerRoutePool twin (Java PR #303,
  same day).** Reply-lane pool adopts it with the per-test-runtime rebind idiom and
  once-per-process fill preserved; ASYNC_HTTP_RESPONSE_STREAM_PREFIX →
  ASYNC_HTTP_RESPONSE_STREAM_POOL (names on the wire unchanged); INCREMENTS.md style
  cleanup (drifted Overview table + all `---` rules removed, Eric's direction).
  Lesson: verify clippy with an UNPIPED exit code — `grep -c` made a clean run look
  like exit 1. Design record: the Java repo's route-pool-registration-design +
  draft-design-specs/register-route-pool.md. origin: 2026-08-30-215256.
  **Follow-up MERGED 2026-09-01: reply-lane checkout ROTATES (VecDeque FIFO — .0,.1,.2;
  a released lane rejoins at the tail) — [PR #221](https://github.com/Accenture/mercury/pull/221)
  merge `0372313a`, Increment 95, Java lock-step (its PR #304); origin: 2026-09-01-022635.**
  <!-- id: ot-route-pool-twin | created: 2026-08-30 | last_used: 2026-09-01 | uses: 2 | tier: active | origin: 2026-08-30-215256 -->
