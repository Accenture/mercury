- [x] (feature — **MERGED 2026-08-22 as
  [PR #212](https://github.com/Accenture/mercury/pull/212), merge `c49d6cd7` carrying
  `83b12c36` (tree verified), CI green (test 2m26s + the new agent-memory check); branches
  deleted both ends; rides the next release** via CHANGELOG Unreleased) **graph.task reaches declarative event-over-http targets — the polyglot
  initiative's only engine change (D5), lock-step with the Java engine (commit `10c53ca3`
  there, same day).** Guard in skills.rs consults `event_api::get_event_http_target`;
  unit-test-task-7 pin (fixture byte-identical to Java, stub /api/event peer, proven
  failing against unfixed code); compiled-set pin 49→50; workspace 63 suites + clippy 0 +
  fmt clean. The initiative's design record lives in the Java repo's memory
  (polyglot-event-over-http-design); wrapper repos mercury-python/mercury-nodejs carry
  their own memory. Full detail: origin log.
  <!-- id: thread-graph-task-event-over-http | created: 2026-08-22 | last_used: 2026-08-24 | uses: 1 | tier: archive-candidate | origin: 2026-08-22-185217 -->
