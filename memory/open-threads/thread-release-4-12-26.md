- [ ] **Release v4.12.26 — PREPARED 2026-10-26: the lock-step twin of Java 4.12.26, catching up from 4.12.24 (Java 4.12.25 was
  Java only).** Branch `release/4.12.26`: the CHANGELOG's Unreleased section cut under `## Version 4.12.26, 10/26/2026` with Changed 8
  added (#387, the graph-set worked example, which the tag range caught missing), 13 manifests / 24 occurrences swept, `cargo update -w`
  27 lock lines; fmt, clippy and the doc checks clean, 142 suites / 732 passed / 0 failed / 9 ignored. Next (Eric's gates): open and
  merge the PR, tag `v4.12.26`, `cargo publish --workspace`; then verify the tag commit and the twelve crates, rewrite
  `latest_release`, close this thread.
  origin: 2026-10-26-164333
  → serves: vision-mercury
  <!-- id: release-4-12-26 | created: 2026-10-26 | last_used: 2026-10-26 | uses: 1 | tier: working | origin: 2026-10-26-164333 -->
