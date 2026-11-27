- [x] **A `COMPUTE` with a ternary computes its branch's value, as a `DECIMAL` does - Increment 221, on `main` —
  DONE 2026-11-27.** Pull request #431 (commit `4ffc216a`; merge `a400233f`), the twin of the Java engine's
  mercury-composable #686 (the maintainer's ruling of the same day): `expression_source::has_ternary` makes a
  statement with a ternary a number statement in a `COMPUTE` as in a `DECIMAL`
  ([[math-text-value-one-value-rust]]); PR #430 (merge `e9f01715`) put the rules on the AI agent guide's
  checklist. Lesson: a bundle deployed from an unmerged Java branch is safe only when that branch merges
  unchanged - compare the chunk, the help page and the fixtures after both merges, as was done here.
  origin: 2026-11-27-210952.
  → serves: vision-mercury
  <!-- id: compute-ternary-as-decimal | created: 2026-11-27 | last_used: 2026-11-27 | uses: 1 | tier: working | origin: 2026-11-27-210952 -->
