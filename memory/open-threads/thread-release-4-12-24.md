- [ ] **Release v4.12.24 — PREPARED 2026-10-12: the lock-step twin of Java 4.12.24 (Increments 177–179, the range plugins' bounds and
  refinements); the Java release also fixes the Guava resolution the field's Snyk gate rejected in 4.12.23, which this engine has no
  equivalent of.** `release/4.12.24` (`0bf12cd0`, PR #383 MERGED 2026-10-12 22:28Z, merge `de283537`; Java PR #559): 13 manifests / 24 occurrences, `cargo update -w` (27 lock lines), the CHANGELOG cut;
  139 suites / 719 passed / 0 failed / 9 ignored (4.12.23: 137 / 712). Java `release/4.12.24` (`649f51ab`): 45 / 102, reactor 249 suites /
  1830 tests / 0 failures. Gates: Eric opens and merges both PRs, tags `v4.12.24` on each, `cargo publish --workspace`; then verify the
  tag commit and the twelve crates, rewrite `latest_release`, close this thread.
  → serves: vision-mercury
  <!-- id: release-4-12-24 | created: 2026-10-12 | last_used: 2026-10-12 | uses: 1 | tier: working | origin: 2026-10-12-222006 -->
