- [ ] **Release v4.12.27 — PREPARED 2026-10-30: the lock-step twin of Java 4.12.27 (the dead-letter key, the registry client's
  retries, the schema caches and `libz-sys` 1.1.31, mercury #399).** Branch `release/4.12.27`: the CHANGELOG cut under a "Nothing
  yet" Unreleased, the sweep 13 manifests / 24, 27 lock lines; 142 suites / 737 passed / 0 failed / 9 ignored. Next, Eric's gates:
  open and merge the PR, tag `v4.12.27`, publish the GitHub release with the CHANGELOG section as its notes, `cargo publish
  --workspace`; then verify the tag commit and the twelve crates, rewrite `latest_release` and close this thread. The Java PR and
  the field's result are recorded in the Java repo.
  origin: 2026-10-30-230441.
  → serves: vision-mercury
  <!-- id: release-4-12-27 | created: 2026-10-30 | last_used: 2026-10-30 | uses: 1 | tier: working | origin: 2026-10-30-230441 -->
