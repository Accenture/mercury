- [x] **A failing element of a `for_each` fan-out fails the node with its own status - Increments 216 and 217, on
  `main` — DONE 2026-11-27.** Pull request #426 (commits `3bd7e01d`, `16d4299e`, `0a79caaf`; merge `f8ac53f7`): the
  twin of the Java engine's mercury-composable #682 - the first failing element in list order decides and no batch
  starts after a failure ([[fan-out-first-failure-decides-rust]]) - then two differences from the Java engine rectified
  on the maintainer's word (an error body names its `target` once; a call without a reply reads `Timeout for N ms`),
  and a test race closed (`edge_relay_spans_are_connected` takes the suite guard). Lesson: a twin's first failing test
  is a cheap parity probe - both differences showed in its output before any fix. origin: 2026-11-26-233739.
  → serves: vision-mercury
  <!-- id: for-each-failure-masked | created: 2026-11-26 | last_used: 2026-11-26 | uses: 1 | tier: working | origin: 2026-11-26-233739 -->
