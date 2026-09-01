- [x] (lock-step — **SHIPPED 2026-08-01: feature merged as PR #191 (`5db06a8f`),
  release merged as PR #192 (`7358f1a2`), tag `v4.11.1` on the verified merge commit;
  both engines in lock-step at 4.11.1**)
  **The Java v4.11.1 lock-step arc: version-aware Redis consume (GETDEL / atomic
  MULTI/EXEC below 6.2, field report), Event Script per-task ttl + honored sub-flow
  delay with teardown cancellation, minigraph node ttl + model-metadata immutability
  (the previously UNGUARDED model.* RHS closed), the traveler run-level watcher with
  exactly-one-terminal CAS arbitration, honest companion drain, fetcher x-ttl stamp,
  and end-to-end deadline propagation (the flow adapter now derives the budget from
  the delivered x-ttl, Java-exact ceil-to-seconds).** Adversarial review round: 14
  confirmed findings resolved (the Java-parity lens caught the raw-ms budget, the
  35s drain fallback, the gate message wording); three exact Java-parity residuals
  documented as shared follow-ups. Workspace 58 suites green / clippy 0 / fmt.
  Increments 76-80. Java reference: mercury-composable v4.11.1 (tag on `410e03bb`).
  <!-- id: thread-v4-11-1-lockstep | created: 2026-08-01 | last_used: 2026-08-01 | uses: 1 | tier: working | origin: 2026-08-01-233448 -->
