- [x] (release — SHIPPED AND PUBLISHED 2026-08-21 local, **both repos in lock-step at
  v4.11.10**; both GitHub releases published by Eric) **v4.11.10 — the AI discovery release.**
  Rust: move PR #210 (`examples/`→`system/ai-contract-provider`, Eric's consistency ruling,
  merge `9d1e4c28` tree-verified) then release PR #211 merge `b77f17e8` carrying `1beff96d`
  (tree verified), gate 63/317 + clippy 0 + fmt, tag on the merge, dereference-verified.
  Java: release PR #291 squash `5cb65f04` == gated `689adf5e`, 34-pom sweep, full reactor
  green, tag on the squash, dereference-verified. Contents: f:setConfig +
  system/ai-contract-provider (+ Java: OTLP fixes, flow-binding docs fix).
  Lesson: content merges before the mechanical release PR — the Rust release branch was
  discarded and recreated on top of the move. Full detail: origin log.
  <!-- id: thread-release-4-11-10 | created: 2026-08-22 | last_used: 2026-08-22 | uses: 1 | tier: archive-candidate | origin: 2026-08-22-032041 -->
