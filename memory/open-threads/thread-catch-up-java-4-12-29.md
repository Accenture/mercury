- [x] **Catch up the Java engine's common features since 4.12.27, the mini-scheduler excepted (Eric, 2026-11-05) —
  DONE 2026-11-05.** Five items, one PR and Increment each, all merged the same day: #610's abort record (PR #403
  `46a62855`, Increment 196), the asynchronous `graph.extension` (#404 `a121210e`, 197), the agent loop `llm.run` as the
  new crate `mercury-agent-loop` (#405 `21c9f150`, 198), progress events and `/ws/graph-run` (#406 `9c0aa84a`, 199;
  Eric's ruling: an engine-opened stream allows the flow's TTL plus 5 seconds), `playground.panel.name` with the bundle
  and help deployed (#407 `826d550c`, 200). Lesson: dependent PRs stack cleanly on a merge-commit repo, but a thread
  edited on main conflicts with a stacked branch's edit of the same paragraph, so record a stacked item's merge in its
  session log and let the close carry it. Next: the release at 4.12.29. origin: 2026-11-05-181529.
  → serves: vision-mercury
  <!-- id: catch-up-java-4-12-29 | created: 2026-11-05 | last_used: 2026-11-05 | uses: 1 | tier: working | origin: 2026-11-05-181529 -->
