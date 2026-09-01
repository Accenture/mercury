- [x] (lock-step — **MERGED 2026-08-08 as mercury PR #195, merge `4e6bdf43` carrying
  commit `995cfeb7` with its single co-author trailer, CI green; mirrors the Java
  reference engine's PR #265 (squash `392f7128`, ADR-0012); both engines identical —
  rides the next release via CHANGELOG Unreleased / INCREMENTS 82**)
  **Suspend/resume rationalization: suspension is a destination — edge/jump modes
  replace `suspend=true` (this port's ADR-0011, amending ADR-0009).** Edge mode = drawn
  edge + mandatory continuation, resume continues past, never re-executes (back-compat
  exact; property = deprecation-WARN no-op). Jump mode = graph.math IF-THEN-ELSE jump,
  RE-EXECUTED on every resume (wait loop, no RESET); routing-skill drawn edge to suspend
  + exception=suspend rejected with Java-exact teaching errors; jump-only suspend is
  island-anchored (island exempt from the continuation-edge rule). tutorial-14 +
  fixtures byte-identical from Java (await-decision/RESET gone); jump-mode + compat
  scenarios added to graph_runtime; knowledge-graph 9 suites (44-graph gate), playground
  e2e, fmt, clippy 0. **Webapp REPLACED from the Java repo's latest UI source (Eric's
  directive — brings the Java PR #262 UI work over), port path adaptations re-applied,
  webapp 212/212, bundle index-DqzF65vX.js.** Record/store contracts unchanged.
  Relates [[thread-tutorial-14-decision-rust]].
  <!-- id: thread-suspend-resume-rationalization-rust | created: 2026-08-08 | last_used: 2026-08-08 | uses: 2 | tier: archive-candidate | origin: 2026-08-08-005419 -->
