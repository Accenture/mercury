- [x] (port — sync-over-async streaming return route, 2026-09-13) **Cross-pod progressive
  rendering ported to the Rust engine in lock-step — R-series complete, wire-parity
  acceptance gate MET.**
  Outcome: `mercury-sync-over-async` (engine R1 #266, facade+health R2 #268), demo +
  cross-pod dry-run (R3 #269), polyglot dry-run R4 (this branch) — a Rust⇄Java rendezvous
  with mixed producers, cross-engine recovery, and a real-LLM leg, on one `redis-standalone`.
  Durable lesson: a language-neutral wire contract (§4: keys, envelope, channel naming,
  config defaults) plus map-don't-mirror deltas (§5: MULTI/EXEC for Lua, explicit
  resubscribe, edge-grace headroom) is what makes two engines drop-in peers — and porting
  the suites is a test review of the canon (two latent Java flakes found, #378/#379).
  Deferred: crates.io publication AND the Q1 facade tasks are HELD until minimalist-kafka
  lands (Eric 2026-09-14; [[ot-minimalist-kafka-port]] K5 closes both — the path-only
  dev-dep check rides that release). Bounce-path retry: RULED 2026-09-14 and implemented —
  idempotent-only retry-once keyed on the manager's own reconnect trigger (spec §5 item 6);
  append/pop/publish stay fail-fast.
  origin: 2026-09-13-161430 (spec) → R4 close: 2026-09-14-010743.
  <!-- id: ot-sync-over-async-port | created: 2026-09-13 | last_used: 2026-09-14 | uses: 10 | tier: active | origin: 2026-09-13-161430 -->
