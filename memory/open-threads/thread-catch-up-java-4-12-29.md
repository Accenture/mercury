- [x] **Catch up the Java engine's common features since 4.12.27, the mini-scheduler excepted (Eric, 2026-11-05) —
  DONE 2026-11-05.** Five items, one PR and Increment each: the abort record (#403 `46a62855`, 196), the asynchronous
  `graph.extension` (#404 `a121210e`, 197), `llm.run` in `mercury-agent-loop` (#405 `21c9f150`, 198), progress events and
  `/ws/graph-run` (#406 `9c0aa84a`, 199), `playground.panel.name` (#407 `826d550c`, 200); shipped in v4.12.30. Lesson: a
  thread edited on main conflicts with a stacked branch's edit of the same paragraph, so record a stacked item's merge in
  its session log and let the close carry it. origin: 2026-11-05-181529.
  → serves: vision-mercury
  <!-- id: catch-up-java-4-12-29 | created: 2026-11-05 | last_used: 2026-11-05 | uses: 5 | tier: active | origin: 2026-11-05-181529 -->
