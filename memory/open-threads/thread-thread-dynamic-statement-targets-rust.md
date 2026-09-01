- [x] (feature+fix — **MERGED 2026-08-10 as mercury PR #201, merge `354c1134`** carrying
  `7d2da900`, CI green on the first run incl. the Format check (test job 2m9s); mirrors
  the Java engine's same-day fix, merged as
  [PR #273](https://github.com/Accenture/mercury-composable/pull/273) squash `96d9c35f`
  — **COMPLETE ON BOTH ENGINES**; rides v4.11.6. Increment 85.) **Dynamic
  variables in every statement command — completing the generic error handler — PLUS the
  recovery semantics follow-up (**MERGED as PR #202, merge `213b739a`** carrying
  `6c7cf134`, CI green; Java twin
  [PR #274](https://github.com/Accenture/mercury-composable/pull/274) squash `5a01c0c6`;
  Increment 86 — shipped in v4.11.6 same day, see [[thread-release-4-11-6-rust]]): a successful retry of error.source
  RESOLVES the virtual 'error' node (code=200, source kept, details removed; source match
  keeps parallel branches safe) — pinned by unit-test-error-recovery + a tutorial-12
  companion dry-run; 58/305 + clippy + fmt.** Eric's
  regression pass found RESET:/NEXT: took targets literally; now NEXT:/THEN:/ELSE:
  targets, RESET: entries and DELAY: values resolve {namespace.key} at execution time
  (`get_next_tag_resolved` + per-tag substitution in skills.rs; unresolved → "null":
  RESET no-op, DELAY skipped, jump fails loudly). No graph.js on this engine (retired) —
  math executor only. tutorial-12 genericized (fixture byte-identical; help adapted in
  the port's structure); unit-test-dynamic-jump pins THEN:/DELAY:. Gate: 58 suites /
  305 tests, clippy clean, fmt clean (verified by real exit code — a piped `head` masked
  the first check), webapp 212/212.
  Relates [[thread-graph-scoped-state-and-error-context-rust]].
  <!-- id: thread-dynamic-statement-targets-rust | created: 2026-08-10 | last_used: 2026-08-11 | uses: 2 | tier: archive-candidate | origin: 2026-08-10-224037 -->
