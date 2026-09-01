- [x] (fix — **MERGED 2026-08-11 as mercury PR #204, merge `f5256ecc` carrying `8fc45b94`
  (tree verified identical), CI green (test 2m8s + authoritative recheck); Java twin
  [PR #278](https://github.com/Accenture/mercury-composable/pull/278) `573c62aa`;
  rides the next release.**) **v4.11.6 regression: dry-run suspend/resume
  never resumed — the playground lane's ephemeral `playground-{uuid}` graph id broke the
  graph-scoped store key** (`graph:{graph_id}:{cid}` never matched across instantiations;
  executor lane unaffected — stable deployed id). Fix: dry-run identity = root node's `name`
  property; **unnamed root + suspend/resume model REJECTED at instantiation with a teaching
  message (Eric's ruling — a silent fallback would break resume invisibly)**; guard-first (no
  side effects on rejection). Pins: resume-across-instantiations (store_file key pin + step
  counters + consume-on-retrieve) + the rejection; pre-run-check scratch graph gained a root
  name. Gates: 58/305 + clippy 0 + fmt clean. Porting gotcha recorded: scripted guard landed at
  the wrong `remove_instance` occurrence first — anchor on unique context.
  Relates [[thread-graph-scoped-state-and-error-context-rust]].
  <!-- id: thread-dry-run-graph-scope-fix-rust | created: 2026-08-11 | last_used: 2026-08-11 | uses: 2 | tier: archive-candidate | origin: 2026-08-11-051701 -->
