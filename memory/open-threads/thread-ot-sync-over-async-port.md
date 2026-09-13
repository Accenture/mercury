- [ ] (port — sync-over-async streaming return route, 2026-09-13) **Bring cross-pod
  progressive rendering to the Rust engine in lock-step with Java's ratified design**
  (mercury-composable `draft-design-specs/streaming-return-route.md`, D1–D8 + E1–E4, no
  open flags). Draft mapping spec: `draft-design-specs/sync-over-async-port.md` — the
  substrate (x-event-stream protocol, EventStreamWriter, `stream: true` SSE edge, SSE
  consumer) is already in lock-step; the missing piece is exactly the extension crate.
  Spec APPROVED 2026-09-13, Q1–Q5 ruled (spec §9): facade tasks deferred; the RESP
  double lifts into a shared dev-only crate; `soa.redis.health` ships in the
  increment; publish as `mercury-sync-over-async` (eighth crate); a progressive-
  rendering demo lands under `examples/`. Deliberate deltas: MULTI/EXEC atomic
  pipeline for Lua (keeps the test double Lua-free), explicit Pub/Sub resubscribe
  loop (redis-rs has no auto-resubscribe). Test story is in-process only (no Docker —
  VDI constraint): extend the RESP double with Lists + EXPIRE + Pub/Sub (~225 lines).
  The prize beyond parity: **polyglot rendezvous** — Rust and Java pods sharing one
  streaming return route (experiment R4).
  Next: R1 — the crate + the ported E1 suite against the extended double.
  <!-- id: ot-sync-over-async-port | created: 2026-09-13 | last_used: 2026-09-13 | uses: 1 | tier: working | origin: 2026-09-13-161430 -->
