---
title: Test Report — Streaming Return Route, cross-pod dry-run (Rust)
summary: Permanent record of the R3 experiment - two Rust processes against a
  standalone Redis, the UI request landing on one pod and producers posting from
  another, with chaos checks for lost notifications, a lost close, a producer
  killed mid-stream, and a killed UI pod.
layer: reference
audience: [developer, architect]
keywords: [streaming, sse, redis, return route, cross-pod, chaos, test report]
---

# Test Report — Streaming Return Route, cross-pod dry-run (Rust)

*Live two-pod validation of the
[sync-over-async port](https://github.com/Accenture/mercury/blob/main/draft-design-specs/sync-over-async-port.md)
(experiment R3), conducted 2026-09-13 (local time) on a single developer machine with one JVM
(the Redis helper) plus two Rust processes, and Node.js as the SSE client. This report is a
permanent record in the tradition of the Java engine's
[cross-pod report](https://github.com/Accenture/mercury-composable/blob/main/docs/test-reports/streaming-return-route-cross-pod.md),
whose E3 scenarios it reproduces outcome-for-outcome: what was run, the evidence, and the
observations the round produced.*

## The scenario under test

The streaming return route exists for exactly one gap: in a horizontally scaled deployment,
the pod that *produces* progressive events is generally not the pod holding the user's HTTP
connection. R3 makes that literal — **two separate Rust processes**:

| Process | Role | Port | Notes |
|---------|------|------|-------|
| `redis-standalone` (Java helper jar 4.12.8) | the rendezvous | 6379 | the only shared infrastructure — **no broker anywhere** (design D3); language-neutral, no Docker |
| `sync-over-async-demo`, profile `stream-ui` (**pod A**) | holds the SSE connection | 8600 | return-route coordinator + `StreamBridge` facade; `-Dsync.stream.ttl.seconds=30` for this run so the TTL-bounded endings complete quickly |
| `sync-over-async-demo`, profile `stream-producer` (**pod B**) | posts the events | 8601 | `StreamResponder` only — **no coordinator**, no subscriber, `sync.over.async.enabled=false` |

The client opens `GET /api/notifications` on pod A (`stream: true`, SSE); the facade
announces the session's correlation-id as the first SSE event; the client then POSTs to
pod B's `/api/produce`, quoting that cid — the notification use case's exact shape. Pod B's
chaos mode (`"mode": "lost"`) stores a segment while suppressing its wake-up, simulating a
lost Pub/Sub notification. The build under test is R2's facade half on R1's rendezvous
engine (PRs [#266](https://github.com/Accenture/mercury/pull/266) and
[#268](https://github.com/Accenture/mercury/pull/268), pre-release), driven by the new
`examples/sync-over-async-demo`. Both pods' `/health` carried the `soa.redis.health` probe
live (`"Redis client is starting up"` placeholder during the grace window, then
`"Redis is reachable"`).

## Scenario 1 — ordered tokens across pods (the chat shape)

Pod B posts five tokens and `eof` sequentially over one connection; pod A renders them out
its SSE edge. **No sequence number exists anywhere in the pipeline** — order is carried by
posting discipline → Redis list order → serialized drain → the edge's ordered reply lane:

```text
17:28:03.707  event: cid
17:28:03.707  data: 50a04b3ae3464d82904f87d452c98095
17:28:05.937  data: Streaming
17:28:05.938  data:  across
17:28:05.938  data:  pods
17:28:05.939  data:  by
17:28:05.939  data:  design
17:28:05.940  event: done
17:28:05.940  data: {"tokens":5}
```

Producer's view: `{"mode":"chat","live":true,"posted":6}`. **Exact order preserved,
terminal metadata delivered, rendezvous closed.** ✅

## Scenario 2 — lost notification, healed by the next drain

Three notifications from pod B: `orders` (normal), `billing` (**wake-up suppressed**),
`payments` (normal). Mid-run check after the suppressed post: `billing` is stored in Redis
but **absent from the render** — then the `payments` wake-up's drain delivers both, in list
order, and a backend-side `close` ends the channel ("the end signal is also an event"):

```text
17:28:25.895  event: orders
17:28:25.895  data: order 42 shipped
              (billing posted with its wake-up suppressed - nothing renders)
17:28:29.103  event: billing
17:28:29.103  data: invoice 7 ready
17:28:29.104  event: payments
17:28:29.104  data: refund 9 done
17:28:30.179  event: done
```

**A dropped signal costs latency, never data.** ✅

## Scenario 3 — lost *close*, recovered by the final drain at idle expiry

The worst case design D4's nuance exists for: a data segment **and the terminal `eof`** are
stored with every wake-up suppressed. Nothing wakes pod A. At the channel's idle allowance
(6s, set per request via `x-stream-idle-seconds`) the facade watchdog performs its **single
final drain** — and the render completes successfully:

```text
17:29:36.629  event: cid
              (data + eof stored at ~17:29:37, all wake-ups suppressed)
17:29:42.628  event: orders
17:29:42.629  data: the last update
17:29:42.630  event: done
17:29:42.630  data: {"recovered":true}
```

The drain fired at **idle + 2 ms** (channel opened 17:29:36.626, drain at 17:29:42.628).
**The one-shot "final read before timeout" cornerstone, working as a stream, across pods.** ✅

## Scenario 4 — producer killed mid-stream (`kill -9`)

Pod B posts two tokens ("stall" mode) and is then killed. No terminal can ever arrive; the
render must fail **in-band** at idle expiry, exactly like the failure table says:

```text
17:30:01.323  event: cid
17:30:01.511  data: first
17:30:01.512  data: second
              (pod B kill -9)
17:30:07.519  event: error
17:30:07.520  data: {"message":"Stream idle timeout","status":408,"type":"error"}
```

408 at 6.007s after the last segment (the watchdog re-armed by the activity, then firing).
The rendezvous keys were deleted by the close; the queue remnant needs no sweeper. This is
also the port delta of spec §5 item 5 proving itself live: the **watchdog's** 408 reached
the client — not the edge backstop's — because the bridge grants the edge
`EDGE_GRACE_SECONDS` of headroom over the watchdog allowance. ✅

## Scenario 5 — UI pod killed (`kill -9`) → producer orphan stop, TTL-bounded

Pod A is killed with a channel open (route TTL 30s in this run). The route key is Redis
state, so it does **not** vanish with the pod — a post inside the TTL window is accepted
into the void (stored, published to a channel nobody subscribes to), and the producer stops
as soon as the route expires:

```text
17:30:33  channel opened; pod A kill -9 in the same second
17:30:35  post right after the kill: {"mode":"notify","live":true}    <- TTL-bounded void window
17:31:14  post after the 30s TTL:    {"mode":"notify","live":false}   <- orphan stop
```

**Bounded, deterministic stop with no liveness machinery** — the TTL is the crash backstop
doing exactly its job. The producer stops *within the route TTL* of a UI-pod crash, not
instantaneously, and the abandoned queue remnant ages out on its own TTL. ✅

## Observations and round notes

- **No mechanism defects found.** All five scenarios behaved per the ported design on the
  first complete run — the Java E3 outcomes reproduced by the Rust engine against the same
  helper Redis, through the same wire contract.
- **The wire is the Java wire.** `request:{cid}` / `queue:{cid}` key shapes, the compact
  segment envelope, and `{sync.return.channel.prefix}:{origin}` channel naming carried the
  whole round — the polyglot rendezvous (a Java pod and a Rust pod sharing one streaming
  return route) is experiment R4's acceptance gate, on this same topology.
- **One presentation nuance** (pre-existing, deliberate): a terminal `eof` with **no**
  metadata renders its `done` event's data line as the literal `{}` on this engine's SSE
  edge (scenario 2's backend `close`). Metadata-carrying closes are identical across
  engines (scenarios 1 and 3).
- **Sizing note for operators** (unchanged from the Java round):
  `sync.stream.ttl.seconds` bounds two things at once — how long a crashed UI pod's
  rendezvous accepts posts into the void, and how long a stalled consumer's queue survives.
  The 1800s default favors long-lived quiet notification channels; this run used 30s to
  make the bound observable.
- The demo's `stream-ui` / `stream-producer` profiles are permanent: the runbook in the
  [sync-over-async-demo README](https://github.com/Accenture/mercury/tree/main/examples/sync-over-async-demo)
  reproduces this report end-to-end with three terminals, `curl`, and the timestamped
  SSE client under `scripts/`.

## What remains

Experiment **R4 — the polyglot dry-run**: a Rust producer streaming into a Java UI pod's
rendezvous and a Java producer into a Rust UI pod's, on one `redis-standalone` — the
wire-parity acceptance gate (port spec §4), with the optional real-LLM leg (the Java E4
analog) on top.
