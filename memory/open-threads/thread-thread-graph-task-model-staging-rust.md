- [x] (lock-step — **MERGED 2026-08-08 as mercury PR #197, merge `79212bc0` carrying
  `0530bd13`, CI green; rides the next release**; mirrors the Java engine's
  [PR #267](https://github.com/Accenture/mercury-composable/pull/267), squash `e16f4b40`)
  **graph.task `model.*` input staging (Event Script parity) + tutorial-13 as an HTTP
  client by configuration + the default-Accept client ruling.** stage_model_variable in
  skills.rs (guarded model.* RHS → state machine); tutorial-13/help byte-identical to
  Java (async.http.request, dynamic variables, ${...} load-time substitution, explicit
  headers.accept + headers.x-ttl); v1.hello.task mock retired; unit-test-task-6 gate
  negative; playground dry-run twin (ephemeral-port harness re-points the
  rest.server.port override at the bound port). **Eric's ruling: the async HTTP client
  sends a default `Accept: */*` when the caller gives none** (Java reactor-netty parity;
  both REST edges omit response content-type absent Accept, so the same model previously
  decoded JSON on Java and returned bytes here); explicit accept never overridden,
  wire-echo pinned both ways. Also repaired the INCREMENTS ledger (78/79 reconstructed,
  tail re-ordered 76→83, Overview extended). Increment 83.
  <!-- id: thread-graph-task-model-staging-rust | created: 2026-08-08 | last_used: 2026-08-09 | uses: 1 | tier: archive-candidate | origin: 2026-08-09-043233 -->
