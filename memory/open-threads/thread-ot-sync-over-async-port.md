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
  **R1 DONE 2026-09-13** (origin 2026-09-13-183453): `mercury-sync-over-async` ships the
  rendezvous engine, the double moved to `crates/redis-test-double` with Lists + EXPIRE +
  Pub/Sub, 13 E1 scenarios + 21 unit pins green, workspace fmt/clippy/test clean, no Docker.
  Finding (both engines): a terminal post's return value is racy by construction — never
  assert liveness on a closing post (spec §3 item 8).
  **R2 DONE 2026-09-14** (origin 2026-09-14-000411): the facade half — `StreamBridge` +
  `EventStreamSink` + shared writer, `runtime` holder, `soa.redis.health` with the
  waiting-vs-outage boundary (vault pattern e2e'd against the double's new `requirepass`
  mode); all four Java E2 scenarios green through the real `stream: true` SSE edge.
  Finding: the Rust edge times idle precisely per await where Java's backstop is a 10s-sweep
  housekeeper — the bridge grants the edge `EDGE_GRACE_SECONDS = 10` headroom so the watchdog
  owns idle expiry (spec §5 item 5). Java follow-up reported: StreamingRestE2eTest still
  asserts liveness on a closing post (PR #378 fixed E1 only).
  **R3 DONE 2026-09-13** (origin 2026-09-14-003459): all five Java E3 outcomes reproduced
  live — two Rust processes + the `redis-standalone` helper jar; drain recovery at idle + 2 ms;
  the watchdog's in-band 408 after a producer `kill -9` (§5 item 5 headroom proven live);
  TTL-bounded orphan stop after a UI-pod `kill -9`. New `examples/sync-over-async-demo`
  (Q5: one crate, `stream-ui` / `stream-producer` profiles) + permanent report
  `docs/test-reports/streaming-return-route-cross-pod.md`.
  Next: R4 — the polyglot dry-run (Rust producer → Java UI pod and vice versa on one
  `redis-standalone`) — the wire-parity acceptance gate; optional real-LLM leg (Java E4 analog).
  <!-- id: ot-sync-over-async-port | created: 2026-09-13 | last_used: 2026-09-13 | uses: 1 | tier: working | origin: 2026-09-13-161430 -->
