# Design — sync-over-async streaming return route → Rust (cross-pod progressive rendering)

> **Status:** APPROVED 2026-09-13 (Eric: Q1–Q5 ruled — §9; "proceed with the Q1–Q5 rulings").
> **R1 IMPLEMENTED 2026-09-13** (§8) · **R2 IMPLEMENTED 2026-09-13** (§8 — the facade half:
> `StreamBridge` + `soa.redis.health`, and the single-process SSE e2e) ·
> **R3 CONDUCTED 2026-09-13** (§8 — the cross-pod dry-run; permanent record:
> `docs/test-reports/streaming-return-route-cross-pod.md`) ·
> **R4 CONDUCTED 2026-09-13 — THE WIRE-PARITY ACCEPTANCE GATE IS MET** (§8; permanent
> record: `docs/test-reports/streaming-return-route-polyglot.md`). The R-series is
> complete: the port is functionally in lock-step. Bounce-path retry: ruled and implemented
> (§5 item 6). **Crates.io publication and the Q1 facade tasks are HELD until
> minimalist-kafka lands** (Eric, 2026-09-14 — see
> `draft-design-specs/minimalist-kafka-port.md`, whose K5 closes both) ·
> **Realizes:** `ot-sync-over-async-port` · **Serves:** `vision-mercury` ·
> **Author:** Claude Code · **Date:** 2026-09-13
>
> **Canonical source:** `extensions/sync-over-async` (mercury-composable, Java) and its ratified
> design spec `draft-design-specs/streaming-return-route.md` (D1–D8 decided, E1–E4 executed,
> final-drain nuance confirmed 2026-09-12 — no open flags), with the live cross-pod evidence in
> `docs/test-reports/streaming-return-route-cross-pod.md`. This document maps that canon to Rust;
> it does not re-open the D-series.
>
> **Maintainer input driving this draft (Eric, 2026-09-13):**
> 1. The Rust port has **no Kafka module** — sync-over-async never shipped here.
> 2. There is **no embedded Redis binary** for Rust tests; the hand-rolled in-process RESP double
>    is acceptable **if it fully covers the use cases**; Redis Pub/Sub was flagged as the
>    potentially demanding part to emulate.
> 3. **Docker is intentionally not an option** for tests — VDI-class dev machines cannot
>    virtualize. The test story must stay pure in-process (requirement **R-NoDocker**).

## 1. Goal & scope

Give Rust applications the same **cross-pod progressive rendering** capability the Java engine
shipped 2026-09-12: a UI pod opens a streaming rendezvous keyed by a business correlation id
(`cid`), any pod posts ordered segments into it purely through Redis, and the UI pod renders them
progressively out its SSE edge — AI-chat token streams and multi-producer event notification
channels, with the same recovery, orphan, and capacity contracts.

**In scope**

- A new workspace crate `extensions/sync-over-async` with the rendezvous engine:
  segment envelope, Redis store, coordinator (both rendezvous patterns), producer API, and the
  SSE-facade bridge (§6 component map).
- The **one-shot pattern as the degenerate stream** (Java D8): `begin` / `await_response` /
  `deliver` are the same mechanism (a one-shot response is a stream whose first entry is
  terminal), so the coordinator ships both patterns — sharing one wire contract and one test
  surface.
- The RESP test-double extension (Lists, `EXPIRE`, Pub/Sub) and the full ported unit/e2e suites
  (§7).
- A Redis health check registered with the actuator, mirroring Java's `soa.redis.health`
  (ruled in — Q3).
- A progressive-rendering demo under `examples/` driving the R3/R4 dry-runs (ruled in — Q5).

**Out of scope**

- The Java demo's **Kafka request leg** and the Event Script facade tasks (`sync.prepare`,
  `sync.await`, `soa.reply`). The tasks are transport-neutral by design (the module's own `cid`
  contract — Java PR #364), but they presuppose a transport wiring the one-shot request leg;
  without a Kafka module they have nothing to demonstrate. They follow when
  `thread-bp-kafka-connectors-backlog` lands, or earlier over Event-over-HTTP
  (ruled: deferred — Q1).
- Redis Cluster/Sentinel (the Java module is standalone-Redis; unchanged here).
- List caps / `MAXLEN` trimming (Java D6: destructive pops keep the queue near-empty; the TTL
  bounds the stall window) and sequence numbers (Java D7: ordering is posting discipline).

## 2. Why this port is smaller than it looks — the substrate is already in lock-step

The 2026-08 progressive-rendering interop work shipped the whole HTTP half of the feature in this
engine. What is missing is exactly the sync-over-async extension; nothing under it.

| Prerequisite (Java) | Rust today |
|---|---|
| `x-event-stream` protocol (`data`/`eof`/`exception`, `x-event-name`, `envelope`) | `platform-core/src/event_stream.rs` — same constants, same semantics |
| `EventStreamWriter` (head commit on first write, idle-allowance override, writes-after-close dropped) | `event_stream.rs` `EventStreamWriter` — explicit Java parity |
| `stream: true` REST endpoints rendering the reply sequence as SSE, with idle allowance + keep-alives | `automation/routing.rs` (`stream_response`) + `automation/server.rs` |
| SSE **consumer** (`Accept: text/event-stream` + `reply_to` → relayed segment envelopes) | `automation/http_client.rs` — present (enables the LLM-leg composition, Java E4) |
| Redis client | `redis` crate 1.5 (tokio, rustls) — already a dependency of `minigraph-state-redis`, whose `redis.*` config family and connection idioms this crate reuses |
| Per-pod identity for the return channel | `Platform::origin()` |

## 3. Canonical semantics (what must hold, from the Java source)

The Java spec §4–§6 is the authority; this section is the port checklist. Each item is pinned by
the ported test suite (§7.4).

1. **Redis keys** (Java spec §4.2): `request:{cid}` — a string holding the originating pod's
   return channel, TTL-bounded, eagerly deleted on close; `queue:{cid}` — a Redis List of
   segment JSON entries, `EXPIRE` refreshed on every append, drained destructively (`LPOP`).
   Cleanup deletes both keys. `LLEN` supports the lost-wakeup re-check.
2. **Segment envelope**: compact JSON `{"type":"data|eof|exception","name":?,"body":?}` with
   absent fields omitted. `eof`/`exception` are terminal; `eof` may carry trailing metadata as
   its body. Parity is **parse-level, not byte-level** (JSON field order is irrelevant), but the
   field names, the three type values, and the omit-when-absent convention are normative.
3. **Atomic append**: a segment append and its `EXPIRE` are one atomic step — a producer crash
   must never strand a TTL-less `queue:{cid}` (Java E1 note, spec §4.7). Vehicle differs in
   Rust (§5.1); the contract does not.
4. **No sequence numbers (D7)**: ordering = producer posting discipline (one poster per ordered
   stream, awaiting each post) + Redis per-connection command order + serialized drains.
5. **Coordinator invariants** (Java `ReturnRouteCoordinator`):
   - one Pub/Sub subscription on `{prefix}:{origin}` serves both patterns — a wake-up carries a
     bare cid, checked against open streams first, then pending one-shot requests;
   - **complete-in-place**: a one-shot wake-up completes the pending entry without removing it;
     removal happens solely in the awaiting/aborting path, so an await-by-cid that starts after
     the response arrived still finds it (destructive pops leave no second copy to re-read);
   - **per-cid drain exclusivity**: concurrent wake-ups collapse onto one forwarding loop; the
     drain flag is held **through terminal cleanup** so no segment can be forwarded after the
     sink saw the terminal one; after release the queue is re-checked once (`LLEN`) — a producer
     that appended during the hold is not left stranded;
   - **await timeout recovery**: on timeout, one final drain of the queue, and if that is empty a
     last non-blocking check of the pending future — a wake-up racing the timeout may have popped
     the only copy (Java E1's race lesson);
   - wake-up handling never runs on the Pub/Sub event loop (Java dispatches to virtual threads;
     Rust spawns a task per wake-up — the drain is a sequence of blocking round trips).
6. **Facade semantics** (Java `StreamBridge`, spec §4.5): one idle number drives both the SSE
   head's idle allowance and a watchdog; every drained segment resets the watchdog; the terminal
   segment disarms it. At idle expiry the watchdog performs **one final drain** (never a periodic
   sweeper — D4's confirmed nuance): if it completes the stream the render ends normally;
   otherwise fail in-band (408) and close the rendezvous. Capacity exhaustion answers a real 503
   **before** the SSE head is committed. The watchdog is also what reclaims a disconnected
   client's stream.
7. **Sink failure closes the stream**: if the segment consumer throws (edge gone), the drain
   closes the rendezvous; producers stop on their next post.
8. **Orphan contract**: posts are **store-first**; `post` returns `false` when no route exists
   (rendezvous over or never opened) — the producer's signal to stop; the orphan remnant ages out
   under its own TTL. **A terminal post's answer is racy by construction** (R1 finding): the
   segment is appended first, so an in-flight drain may deliver it and close the rendezvous —
   deleting the route — before the producer's own route check runs, and `post` then answers
   `false` although the segment *was* delivered. The contract still holds (`false` = stop
   producing, and a producer has nothing to send after a terminal segment), but **liveness must
   not be asserted on a closing post** — in either engine.
9. **UI-pod death**: the route key survives until its TTL, so in-window posts are accepted into
   the void (`live: true`), then orphan-stop after — TTL-bounded, by design (Java spec §6).
10. **Wake-ups are best-effort by contract**: a lost notification costs latency only (healed by
    the next post's drain or the final drain); a duplicate pops nothing (destructive reads).
    This is what makes an emulated Pub/Sub honest (§7.3).
11. **The producer contract binds forwarders**: any component that relays an ordered stream into
    the rendezvous must post sequentially — a concurrent relay can sequence a data batch behind
    the terminal, and it is then silently discarded by design (Java E4's `instances = 1`
    lesson). In Rust: the bridge/relay function runs single-worker, or serializes per cid.

## 4. Wire-format parity is normative — the polyglot prize

The Redis contract is language-neutral. With the identical key shapes, envelope, channel naming,
and TTL semantics, **a Rust producer streams into a Java UI pod's rendezvous and vice versa** —
one rendezvous shared across engines, no engine change on either side. This is the reason to
port in lock-step rather than merely "similar", and it gets its own acceptance gate (R4, §8).

| Surface | Normative value (identical to Java) |
|---|---|
| Route key / queue key | `request:{cid}` / `queue:{cid}` |
| Return channel | `{sync.return.channel.prefix}:{origin}` |
| Segment envelope | `{"type":"data|eof|exception","name":?,"body":?}`, absent fields omitted |
| Config keys (defaults) | `sync.return.channel.prefix` (`svc-return`), `sync.route.ttl.seconds` (90), `sync.response.ttl.seconds` (30), `sync.max.pending.requests` (10000), `sync.stream.ttl.seconds` (1800), `sync.max.pending.streams` (1000) |
| Redis connectivity | the `redis.*` family already used by `minigraph-state-redis` (`redis.host`, `redis.port`, `redis.password` via `${REDIS_PASSWORD:}`, `redis.ssl`, `redis.database`, `redis.timeout.ms`) — mirroring the Java arrangement where one `redis.*` config (and one health probe) serves both modules |
| Terminal / orphan / TTL semantics | §3 items 2, 8, 9 |

## 5. Deliberate deltas from the Java implementation

Each delta is behavior-preserving at the contract level and exists for a stated reason; anything
not listed here ports faithfully (`port-bottom-up-faithful`: map, don't mirror).

1. **Atomic append via `MULTI`/`EXEC` pipeline, not Lua.** Java uses a two-line `EVAL` script
   (`RPUSH` + `EXPIRE`, single round trip). Rust uses `redis::pipe().atomic()` — a pipelined
   `MULTI … EXEC` block: the same atomicity (no TTL-less-key crash window), still one write /
   one round trip. Rationale: the RESP test double already implements `MULTI`/`EXEC` with
   per-connection queueing, so no Lua interpreter is ever emulated (R-NoDocker keeps the double
   as the only test server). Wire-visible but interop-neutral: both vehicles leave identical key
   states, and producers on different engines never need to match vehicles.
2. **Explicit Pub/Sub resubscribe loop.** Lettuce (Java) auto-reconnects and resubscribes; the
   `redis` crate's dedicated `aio::PubSub` connection does not (verified against the locked
   1.5.0 source — and Pub/Sub cannot ride the multiplexed `ConnectionManager`). The Rust
   coordinator owns a reconnect-and-resubscribe loop; after every resubscribe it runs one
   recovery pass — a final drain per open stream and a queue check per pending request — so any
   wake-up lost during the gap is healed by the machinery that already exists (§3 item 10).
3. **Concurrency vehicle mapping** (idiom, not behavior): virtual threads → `tokio::spawn`;
   `CompletableFuture` await → `oneshot`/`Notify` with `tokio::time::timeout`; the per-entry
   drain flag → `AtomicBool`; the bounded registries → mutex-guarded maps with the same
   capacity-rejection semantics (`IllegalStateException` → typed `AppError`, deterministic at
   capacity, slot released on close, rejected registration consumes nothing).
4. **Lazy client construction** stays (Java's preload-before-bootstrap lesson): connections are
   built on first use from live configuration — the `OnceCell` idiom `minigraph-state-redis`
   already uses — never at static-init time.
5. **The edge backstop gets explicit headroom** (R2 finding). Java's between-segment edge
   timeout is enforced by the AppStarter housekeeper — a 10-second sweep with a +100 ms buffer
   ("HTTP timeout does not need to be very accurate") — so the facade's exact idle watchdog
   always acts first by at least the sweep slack; that ordering is what lets the idle-expiry
   final drain (or the in-band 408) reach the client instead of the edge's own timeout. The
   Rust edge enforces its idle allowance **precisely per await**, which turns the same two
   timers into a photo finish the watchdog loses (observed: both idle-expiry E2 scenarios
   rendered the edge's `Timeout for 2 seconds` instead of the watchdog's outcome). The bridge
   therefore commits the SSE head with `EDGE_GRACE_SECONDS = 10` of headroom over the watchdog
   allowance — the housekeeper-interval analog. Observable contract unchanged: the watchdog
   owns idle expiry at exactly `idle_seconds`; the edge backstop reclaims a dead-facade stream
   within the same envelope as Java's sweep.
6. **Idempotent-only retry-once on the store's command lane** (R4 finding, note 3 of the
   polyglot report; maintainer ruling 2026-09-14). After a server bounce, redis-rs's
   `ConnectionManager` arms an asynchronous reconnect but returns the failed command's
   error (its retry configuration governs connection attempts, not command replay), where
   Lettuce requeues commands it had not yet written — so the first Rust call after a
   bounce failed where the Java call healed invisibly. The store now retries its
   idempotent operations exactly once (`save_route`/`get_route`/`cleanup`/`queue_length`);
   `append_segment`, `pop_segment` and `publish` stay fail-fast (duplicate/loss risk under
   D7's no-sequence-number design; wake-ups are best-effort by contract). **The retry is
   lifecycle-aware without extra machinery** (the maintainer's precision question): its
   condition is the manager's own reconnect trigger (`RedisError::is_unrecoverable_error`,
   plus `is_io_error` from the connect path), so it fires exactly when the manager has
   swapped in its reconnection future — and because the manager stores the connection as a
   shared future, the second attempt *awaits* that reconnection rather than racing or
   polling it, under its own timeout. (An explicit event feed exists —
   `ConnectionManagerConfig::set_push_sender` receives disconnection pushes — but adds a
   channel and task without more precision than the failed call's own classification.) A
   timed-out attempt is never retried: a timeout is not a connection-death signal.

## 6. Crate layout & component map

`extensions/sync-over-async`, package `mercury-sync-over-async`, lib `sync_over_async` —
workspace member and the eighth crates.io crate (ruled — Q4).

| Java (`org.platformlambda.sync/support`) | Rust module | Notes |
|---|---|---|
| `StreamSegment` | `segment.rs` | serde struct, `skip_serializing_if` for absent fields; `is_terminal()` |
| `ReturnRouteStore` | `store.rs` | atomic append (§5.1), `LPOP`/`LLEN`/`DEL`, route save/get |
| `ReturnRouteCoordinator` + `PendingRequests`/`PendingStreams` | `coordinator.rs`, `pending.rs` | both rendezvous patterns; §3 invariants; §5.2 resubscribe loop |
| `StreamResponder` | `responder.rs` | producer-side: own connection, post = atomic append + route check + `PUBLISH` |
| `StreamBridge` + `EventStreamSink` | `bridge.rs` | coordinator sink → `EventStreamWriter`; idle watchdog per §3 item 6 |
| `SyncOverAsyncConfig` | `config.rs` | same six keys + defaults (§4) |
| `SyncRuntime` | `runtime.rs` | process-wide holder; exposes operations, never the closeable coordinator (Java PR #376 lesson) |
| `soa.redis.health` | `health.rs` | actuator registration; auth rejections classify as "waiting", never restart-worthy. The `soa.` prefix is normative (Eric, 2026-09-13): the plain `redis.health` route name is reserved for the planned generic Redis distributed-cache module's check, so both features coexist on one server |

## 7. Test strategy — in-process only (R-NoDocker)

### 7.1 The RESP double today

`minigraph-state-redis/tests/common/mod.rs` (~240 lines): real TCP, real RESP2 frames through
the real `redis` crate — only the server is simulated. Already implements
`SETEX`/`GET`/`GETDEL`/`TTL`/`DEL`/`PING`/`INFO`, handshake chatter, **`MULTI`/`EXEC`/`DISCARD`
with per-connection queueing**, expiry honored on read, and a command journal that can prove
which strategy ran on the wire.

### 7.2 Required extension (estimate ~225 lines)

| Addition | Shape | Size |
|---|---|---|
| Lists | stored value becomes `String(Vec<u8>) \| List(VecDeque<Vec<u8>>)`; `RPUSH`, `LPOP`, `LLEN` (+ wrong-type error) | ~60 |
| `EXPIRE` | set `expires_at` on an existing key of either type; reply `:1`/`:0` | ~15 |
| Pub/Sub | per-connection writer task (`mpsc` + `select!` — the one structural change: replies become out-of-band-capable); shared `channel → subscribers` registry; `SUBSCRIBE`/`UNSUBSCRIBE` confirmations and `["message", channel, payload]` push frames (plain RESP2 arrays); `PUBLISH` returns the receiver count and drops dead senders | ~150 |

The Pub/Sub client only ever sends `SUBSCRIBE`/`UNSUBSCRIBE`/`PING` on its dedicated connection,
so no subscriber-mode enforcement is needed. The journal keeps recording — suites can pin that
the atomic `MULTI` block, not bare commands, performed every append.

**Home of the double (ruled — Q2):** lift it unchanged out of `minigraph-state-redis/tests` into
a small internal dev-only crate (`publish = false`) both crates dev-depend on, so two copies never
drift.

### 7.3 Why an emulated Pub/Sub is honest here

The design never depends on Pub/Sub delivery (§3 item 10): every invariant that needs *proof*
lives in the deterministic List/TTL/atomic-append surface, which the double implements exactly.
The Java chaos scenarios simulate lost notifications by **bypassing** publish (append via the
store, never wake) — expressible against the double verbatim. Duplicate and spurious wake-ups
are plain `PUBLISH` calls. Nothing in the suite requires Pub/Sub timing guarantees.

### 7.4 Suite mapping

- The Java E1 suite (13 scenarios: exact ordering, three-producer interleave without loss or
  duplication, any-producer close, orphan, consumer-side close, missed wake-up healed, final
  drain recovers a dropped close, quiet-stream final drain keeps it open, duplicate wake-ups,
  capacity bound/release, sink failure, D8 degenerate one-shot, shared return channel) ports
  1:1 against the double.
- The Java E2 suite (4 scenarios through the real HTTP edge: ordered chat render, multi-producer
  notification channel with backend close, final-drain recovery at idle expiry, in-band 408 with
  nothing queued) ports 1:1 — this engine's `stream: true` SSE edge and SSE consumer are native,
  so the single-process full circle is broker-free by construction.
- Registry/unit pins (pending maps, segment envelope, config parsing) port directly.

### 7.5 What the double cannot prove — and the answer

Real-server protocol corners, AUTH/TLS handshakes, expiry precision under load, performance.
Mitigation is the Java E3 discipline: a **live cross-pod dry-run** against `redis-standalone` —
the Java repo's helper is a plain TCP dev server, language-neutral and Docker-free, so it serves
the Rust dry-run unchanged (R3/R4, §8). Unit and CI suites never require it.

## 8. Experiment plan (lock-step E-series, Rust)

| # | Experiment | Gate |
|---|---|---|
| R1 ✅ | Crate + ported E1 suite against the extended double | **DONE 2026-09-13** — `extensions/sync-over-async` ships the rendezvous engine (segment, store, both registries, coordinator, responder, config, connection); the double moved to `crates/redis-test-double` and gained Lists, `EXPIRE` and Pub/Sub; all 13 E1 scenarios plus 21 unit pins green, workspace `fmt`/`clippy -D warnings`/`test` clean, no Docker. Finding: §3 item 8 (a terminal post's answer is racy in both engines) |
| R2 ✅ | Single-process e2e: `stream: true` endpoint + facade + responder + SSE-consumer collector | **DONE 2026-09-13** — `bridge.rs` (`StreamBridge` + `EventStreamSink` + the shared writer), `runtime.rs`, `health.rs` (`soa.redis.health`, waiting-vs-outage boundary incl. the vault pattern e2e against the double's new `requirepass` mode); all four Java E2 scenarios green through the real Rust HTTP edge, consumed by the shipped SSE client. Finding: §5 item 5 (the edge backstop needs explicit headroom over the watchdog) |
| R3 ✅ | Cross-pod dry-run: two Rust processes against `redis-standalone` (chaos: kill producer, kill UI pod, suppressed wake-ups, short-TTL orphan stop) | **DONE 2026-09-13** — all five Java E3 scenario outcomes reproduced on the first complete run (exact cross-pod ordering; lost notification healed by the next drain; lost close recovered by the final drain at idle + 2 ms; in-band 408 after `kill -9` of the producer — the §5 item 5 headroom proving itself live; TTL-bounded orphan stop after `kill -9` of the UI pod). Driven by the new `examples/sync-over-async-demo` (Q5 — `stream-ui` / `stream-producer` profiles); permanent record: `docs/test-reports/streaming-return-route-cross-pod.md` |
| R4 ✅ | **Polyglot dry-run**: Rust producer → Java UI pod and Java producer → Rust UI pod on one `redis-standalone` | **DONE 2026-09-13 — GATE MET.** Tokens rendered in exact order across engines both ways; both engines shared single rendezvous channels (mixed producers, any-engine close, both UI pods); the Java watchdog's final drain recovered Rust-stored lost segments; the real-LLM leg ran cross-engine (Java pod pulls Gemini SSE, Rust edge renders, usage metadata on `done`). Bonus live evidence: a mid-round Redis restart exercised both engines' resubscribe paths (§5 item 2 equivalent in practice). One delta recorded for a maintainer ruling: redis-rs's `ConnectionManager` fails the first command after a bounce where Lettuce requeues — report note 3. Permanent record: `docs/test-reports/streaming-return-route-polyglot.md` |

## 9. Maintainer rulings (Eric, 2026-09-13)

- **Q1 — one-shot facade tasks: DEFERRED.** The coordinator still ships both rendezvous patterns
  (D8 makes that free), but the Event Script facade tasks (`sync.prepare`/`sync.await`/
  `soa.reply`) wait for a transport worth demonstrating — they follow with
  `thread-bp-kafka-connectors-backlog`, or earlier over Event-over-HTTP on demand.
- **Q2 — the RESP double's home: shared dev-only crate ACCEPTED.** Lift the double unchanged into
  a small internal `publish = false` crate that `minigraph-state-redis` and this crate both
  dev-depend on; two copies never drift.
- **Q3 — health check: YES.** The `soa.redis.health` actuator analog ships in this increment
  (every critical infrastructure component needs a health check).
- **Q4 — naming/publication: ADD `mercury-sync-over-async`** as the eighth published crate.
- **Q5 — demo: ADD a sync-over-async (progressive rendering) demo under `examples/`** to drive
  the R3/R4 dry-runs. Shape (one crate with run-profile role selection, as the Java demo does
  with its stream-ui / stream-producer property files, or a crate pair) is decided when it is
  built, per the standalone-examples convention.

## 10. Relation to the blueprint

Serves `vision-mercury` directly (foundation → user interface: progressive rendering is a
user-facing capability of the engine's HTTP edge, already half-shipped here). The polyglot
rendezvous (§4) is the cross-engine contract made load-bearing at run time — the same field
reality (`inv-telemetry-presentation-parity`: installations are polyglot) that binds telemetry
presentation binds a shared wire protocol. The deferred facade tasks connect this thread to
`thread-bp-kafka-connectors-backlog` when that lands.
