- [x] (release+feature — **MERGED 2026-08-30 as
  [PR #219](https://github.com/Accenture/mercury/pull/219) merge `bc1a9fd2` carrying
  gated `8b73e01c` (tree verified), branch deleted both ends; **v4.12.0 milestone,
  all four repos lock-step** — the wrappers jumped 0.1.0 → 4.12.0)
  **Progressive rendering v4.12.0: engine-to-wrapper streaming relay demo
  (hello.remote.relay + auto-loaded event-over-http.yaml; hello.sse public), the
  progressive-rendering interop report (ten-combination matrix, span-lineage +
  business-cid verification, telemetry/app-log-context appendix), workspace
  version bump.** Lessons: the Rust PostOffice needed no demo trace fix (apply_current_trace
  is unconditional — the Java twin's raw-EventEmitter demo did); /api/event serves
  local routes only by design (loop guard = the x-event-api marker, Eric ratified).
  origin: 2026-08-30-050502.
  <!-- id: ot-progressive-rendering-v4120 | created: 2026-08-30 | last_used: 2026-08-30 | uses: 2 | tier: archive-candidate | origin: 2026-08-30-050502 -->
