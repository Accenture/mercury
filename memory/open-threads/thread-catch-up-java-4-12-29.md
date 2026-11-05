- [ ] **Catch up the Java engine's common features since 4.12.27, the mini-scheduler excepted (Eric, 2026-11-05).** Java
  4.12.28 and 4.12.29 were Java-only releases because their headline work was the mini-scheduler, which this port does
  not have (no number without content). Eric: everything else the Java repo shipped since comes across this sprint.
  Items, one PR and Increment each, in Java's order: (1) #610's abort record (Increment 196, PR #403
  MERGED `46a62855`); (2) #605/#606, the agent loop `llm.run` with the ask row, as `extensions/agent-loop`, passing
  the shared `llm-run-vectors.json` (Increment 198, branch `feat/agent-loop`; a new published crate); (3) #609, the asynchronous `graph.extension` and its CompileGraph rule (Increment 197, PR #404
  MERGED `a121210e`, done before (2) because the two are independent); (4) #611,
  progress events, the `x-return-path` relays, the graph-run WebSocket entry, `llm.run`'s progress events, the
  streaming single-shot reply and the Kafka progress publisher (Increment 199, branch `feat/progress-events` stacked on
  `feat/agent-loop`; Eric's ruling: an engine-opened stream allows the flow's TTL plus 5 seconds); (5) #617, `playground.panel.name` in `get_index_html`,
  with the Playground bundle and help mirror deployed last, once the help's async and progress sections are true here
  (Increment 200, branch `feat/panel-name-bundle` stacked on `feat/progress-events`).
  Not ported: #607 (this engine's mock peer already binds port 0), the RFC notes #602/#608, the Java Sonar round #614.
  Next: the release at the Java number once the items are merged.
  origin: 2026-11-05-181529.
  → serves: vision-mercury
  <!-- id: catch-up-java-4-12-29 | created: 2026-11-05 | last_used: 2026-11-05 | uses: 1 | tier: working | origin: 2026-11-05-181529 -->
