- [x] (fix — **MERGED 2026-08-11 as mercury PR #206, merge `3bdcd3b3` carrying `a901b1b7`
  (tree verified), CI green (test 2m17s); Java twin
  [PR #280](https://github.com/Accenture/mercury-composable/pull/280) squash `68cd9d28`;
  rides the next release.**) **Dry-run graph identity simplified: an
  unnamed draft is scoped `untitled` instead of rejected.** The store contract needs the dry-run
  identity to be STABLE ACROSS INSTANTIATIONS, not derived from the model name — so v4.11.8's
  rejection guard was only defending against its own ephemeral `playground-{uuid}` fallback.
  `stable_graph_identity` → root name or `const UNTITLED`; guard, `root_name`, `uses_suspension`
  and the orphaned mirror test deleted. New twin pin
  (`companion_unnamed_draft_resumes_across_instantiations`, edge-mode draft sketched via companion
  commands) is **mutation-proven** against a per-instantiation handle. CHANGELOG `## Unreleased`
  (published v4.11.8 section untouched). Gates: 58/305 + clippy 0 + fmt clean.
  Relates [[thread-dry-run-graph-scope-fix-rust]].
  <!-- id: thread-untitled-dry-run-identity-rust | created: 2026-08-11 | last_used: 2026-08-11 | uses: 1 | tier: archive-candidate | origin: 2026-08-11-220612 -->
