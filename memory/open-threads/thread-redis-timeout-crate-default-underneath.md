- [x] **`redis.timeout.ms` is the one deadline of a Redis command - Increment 218, on `main` — DONE 2026-11-27.**
  Pull request #427 (commit `9121c43e`; merge `0b7d10e1`): every connection of the foundation and of the state store
  carries the configured timeout as the `redis` crate's own, whose 500 ms default had answered first, and a timeout
  the crate reports is this engine's own failure ([[redis-timeout-one-deadline-rust]]). The red run's cause on the
  runner was not established; the `rust` run on the merge commit is green. Lesson: a client library's defaults are
  part of the contract when a setting is meant to be the only deadline. origin: 2026-11-27-003936.
  → serves: vision-mercury
  <!-- id: redis-timeout-crate-default-underneath | created: 2026-11-27 | last_used: 2026-11-27 | uses: 1 | tier: working | origin: 2026-11-27-003936 -->
