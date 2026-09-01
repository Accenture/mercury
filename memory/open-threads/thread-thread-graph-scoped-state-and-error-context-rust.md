- [x] (feature — **MERGED 2026-08-10 as mercury PR #200, merge `283d41e2`** carrying
  `24eeef89` + the `cargo fmt` follow-up `7dadd1ff` — the first CI run FAILED the
  Format check because the scripted test edits weren't rustfmt-clean (this repo's gate
  is tests + clippy + FMT; run all three locally); CI green on re-run (test job 2m25s).
  Mirrors Java
  [PR #271](https://github.com/Accenture/mercury-composable/pull/271) squash `adfb2a0d`
  + polish PR #272 squash `0612ec6d`, both merged same day — **COMPLETE ON BOTH
  ENGINES; both ride the next release.** Rust ADR-0012/ADR-0013 accepted via the merge.)
  **Graph-scoped workflow state + generic exception context (field
  review follow-ups), Increment 84.** The suspend/resume store contract is scoped by
  graph + cid: envelope {cid, graph, node, ttl, model, seen, run}, get body {cid, graph},
  Redis key `graph:{graph_id}:{cid}` (formerly `graph:state:{cid}` — BREAKING, flag-day
  per Eric's R1; both store functions reject a missing graph; version-aware
  GETDEL/MULTI-EXEC consume unchanged, re-proven on the RESP double).
  `graph.extension`'s `build_forward` stamps the caller's model.cid as the
  `correlation_id` header (Event Script sub-flow parity) — the orchestrator pattern
  (parent delegating independently resumable subgraph paths) pinned by the
  byte-identical unit-test-orchestrator/unit-test-sub-suspend pair + per-graph isolation
  scenario. Generic exception context: `stage_error_context` in common.rs staged at both
  walker choke points (error.source/code/message; **error.stack only when a record
  carries one — this engine has NO native stack-trace transport, a documented port
  divergence**); 'error' was always reserved in the graph model (RESERVED_NAMES) so no
  gate change; `inspect error` works by construction; the alias fixture joins the
  compiled-or-404 negatives (compiler counts 47 valid / 14 invalid). Rust ADR-0012 +
  ADR-0013 proposed (Java twins ADR-0013/ADR-0014 — the numbering skew continues). Gate:
  58 suites / 305 tests green, clippy clean, webapp 212/212 (bundle regenerated for six
  help pages — three byte-copied, three adapted to port variants).
  <!-- id: thread-graph-scoped-state-and-error-context-rust | created: 2026-08-10 | last_used: 2026-08-11 | uses: 2 | tier: archive-candidate | origin: 2026-08-10-190550 -->
