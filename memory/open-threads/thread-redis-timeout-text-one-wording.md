- [x] **A Redis command without an answer in time reads `Redis request timed out after N ms` - Increment 219, on
  `main` — DONE 2026-11-27.** Pull request #428 (commit `6ebf425c`; merge `79aae2d7`): one text for the one failure,
  built by `command_timeout`, whichever of the two timers reports it, where the `redis` crate's own `timed out` had
  been appended to this engine's words; the classification is given the deadline, which the crate's error does not
  carry ([[redis-timeout-one-deadline-rust]]). The maintainer ruled `after N ms`, for consistency with the Java
  engine's Lettuce text, and accepted the difference of unit. Lesson: put the other engine's text beside a new one
  before settling its wording - the ruling came from that. origin: 2026-11-27-013938.
  → serves: vision-mercury
  <!-- id: redis-timeout-text-one-wording | created: 2026-11-27 | last_used: 2026-11-27 | uses: 1 | tier: working | origin: 2026-11-27-013938 -->
