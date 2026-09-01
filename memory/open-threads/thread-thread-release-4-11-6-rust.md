- [x] (release — SHIPPED AND PUBLISHED 2026-08-10, **both repos in lock-step at
  v4.11.6**) **v4.11.6 — the field-review follow-ups release.** Rust: release PR #203 merge `c008d11b` carrying
  `a3ae466f` (merge tree verified identical to the gated commit), CI green (test 2m18s +
  authoritative recheck), workspace Cargo.toml + Cargo.lock + CHANGELOG cut, gate =
  58 suites / 305 tests + clippy 0 + `cargo fmt --check` (exit codes verified unpiped),
  tag `v4.11.6` on the merge, dereference-verified. Java:
  [PR #275](https://github.com/Accenture/mercury-composable/pull/275) squash `c29915ee`,
  tag on the squash. Contents: graph-scoped workflow state (BREAKING store key
  `graph:{graph_id}:{cid}` — the CHANGELOG's `### Changed` LEADS with the upgrade note)
  + generic exception context incl. recovery + orchestrator pattern + dynamic statement
  variables. Both GitHub releases PUBLISHED by Eric 2026-08-10.
  <!-- id: thread-release-4-11-6-rust | created: 2026-08-10 | last_used: 2026-08-10 | uses: 1 | tier: archive-candidate | origin: 2026-08-10-224037 -->
