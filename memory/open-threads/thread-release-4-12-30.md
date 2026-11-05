- [ ] **Release v4.12.30 — the lock-step twin of Java 4.12.30, catching up from 4.12.27 (Eric chose lock-step 4.12.30,
  2026-11-05).** Branch `release/4.12.30`: the CHANGELOG cut (#401-#408, Increments 194-201: the agent loop as the new
  crate `mercury-agent-loop`, the asynchronous `graph.extension`, progress events and `/ws/graph-run`, the abort record,
  the panel name with the bundle and help, the AI companion broker), 14 manifests / 25 occurrences, 28 lock lines. Eric's
  gates: open and merge the PR, tag `v4.12.30`, `cargo publish --workspace` - thirteen crates, `mercury-agent-loop`'s
  first publish (the dry run packaged all thirteen). Then verify the tag commit and the crates on crates.io, rewrite
  `latest_release`, update `memory/instructions.md`'s "twelve published crates", and close this thread.
  origin: 2026-11-05-205957.
  → serves: vision-mercury
  <!-- id: release-4-12-30 | created: 2026-11-05 | last_used: 2026-11-05 | uses: 1 | tier: working | origin: 2026-11-05-205957 -->
