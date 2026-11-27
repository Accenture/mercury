- [x] **A text value in a `graph.math` number expression is one number - Increment 220, on `main` — DONE
  2026-11-27.** Pull request #429 (commit `12317c38`; merge `bbbc6feb`): the twin of the Java engine's
  mercury-composable #683, from a field report - `math::expression_source` renders the expressions of `COMPUTE`,
  `DECIMAL`, `CONDITION` and `IF`, each value checked where it lands ([[math-text-value-one-value-rust]]). Lesson:
  mirror the Java tests' cases one for one and copy its fixtures byte for byte, so the twin's negative control is
  the Java engine's. origin: 2026-11-27-210952.
  → serves: vision-mercury
  <!-- id: math-text-value-one-number | created: 2026-11-27 | last_used: 2026-11-27 | uses: 1 | tier: working | origin: 2026-11-27-210952 -->
