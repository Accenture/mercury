---
title: Test Report — Streaming Return Route, polyglot dry-run (Java ⇄ Rust)
summary: Permanent record of the R4 experiment - the wire-parity acceptance gate.
  Four pods, two engines, one Redis - a Rust producer streaming into a Java UI
  pod's rendezvous and a Java producer into a Rust UI pod's, both engines sharing
  single rendezvous channels, cross-engine chaos recovery, and a real LLM token
  stream produced by the Java pod and rendered by the Rust edge.
layer: reference
audience: [developer, architect]
keywords: [interop, streaming, sse, redis, return route, polyglot, wire parity, test report]
---

# Test Report — Streaming Return Route, polyglot dry-run (Java ⇄ Rust)

*Live cross-engine validation of the streaming return route's normative wire contract
([port spec §4](https://github.com/Accenture/mercury/blob/main/draft-design-specs/sync-over-async-port.md))
— experiment R4, the acceptance gate of the sync-over-async port — conducted 2026-09-13
(local time) on a single developer machine: one `redis-standalone`, **four pods across two
engines**, Node.js as the SSE witness. Follows the
[Rust cross-pod report](streaming-return-route-cross-pod.md) (R3) and the Java engine's
[E-series record](https://github.com/Accenture/mercury-composable/blob/main/docs/test-reports/streaming-return-route-cross-pod.md).*

## The claim under test

The Redis rendezvous contract is language-neutral: identical key shapes
(`request:{cid}` / `queue:{cid}`), one compact segment envelope, one
`{sync.return.channel.prefix}:{origin}` channel convention, the same configuration
defaults. If that holds, **a producer on either engine can stream into a rendezvous held
by a UI pod on either engine — with no engine change and no translation layer anywhere.**
R4 makes it literal:

| Process | Engine | Role | Port |
|---------|--------|------|------|
| `redis-standalone` 4.12.8 (helper jar) | — | the rendezvous | 6379 |
| `sync-over-async-demo` 4.12.8, profile `stream-ui` | **Java** | holds an SSE connection | 8600 |
| `sync-over-async-demo` 4.12.8, profile `stream-producer` | **Java** | posts events (`StreamResponder`, Lua append) | 8601 |
| `sync-over-async-demo`, profile `stream-ui` | **Rust** | holds an SSE connection | 8602 |
| `sync-over-async-demo`, profile `stream-producer` | **Rust** | posts events (`StreamResponder`, MULTI/EXEC append) | 8603 |

Both coordinators' startup lines show the shared channel convention side by side:

```text
Java: Return-route subscriber listening on svc-return:202609147bd6f561cccf42cc92715d66431faede
Rust: Return-route subscriber listening on svc-return:504ae8e5bd8345d68be73758c957805e
```

## P1 — Rust producer → Java UI pod (the chat shape)

The Java facade announces the cid; the **Rust** responder posts five tokens + `eof`; the
**Java** coordinator drains and the Java edge renders:

```text
18:03:30.503  event: cid
18:03:30.503  data: 20a9873ed80d47529ec93a30325b8883
18:03:30.698  data: Streaming
18:03:30.698  data:  across
18:03:30.698  data:  pods
18:03:30.699  data:  by
18:03:30.699  data:  design
18:03:30.700  event: done
18:03:30.701  data: {"tokens":5}
```

Rust producer's view: `{"live":true,"posted":6}`. **Exact order, terminal metadata,
across engines.** ✅

## P2 — Java producer → Rust UI pod (the reverse)

The Rust facade announces; the **Java** responder posts (its atomic append is the Lua
script); the **Rust** coordinator drains (its append vehicle is MULTI/EXEC — the §5 item 1
delta proving interop-neutral, since consumers never see the vehicle):

```text
18:03:33.242  event: cid
18:03:33.242  data: be79fed03ada4a2ea68a8eb8f0ab7d1c
18:03:33.439  data: Streaming
...
18:03:33.445  event: done
18:03:33.445  data: {"tokens":5}
```

Java producer's view: `{"live":true,"posted":6}`. ✅

## P3 — two engines, ONE rendezvous (both UI pods)

The fullest form of the claim: a **Java** producer and a **Rust** producer post named
events into the *same* channel, and either engine may close it. On the Rust UI pod
(Java posts `orders`, Rust posts `billing`, Java closes):

```text
18:03:52.051  event: orders
18:03:52.051  data: order 42 shipped (posted by the Java pod)
18:03:53.133  event: billing
18:03:53.133  data: invoice 7 ready (posted by the Rust pod)
18:03:54.224  event: done
```

And mirrored on the Java UI pod (Rust posts, Java posts, **Rust** closes):

```text
18:04:12.695  event: orders
18:04:12.695  data: order 43 packed (posted by the Rust pod)
18:04:13.797  event: billing
18:04:13.797  data: invoice 8 sent (posted by the Java pod)
18:04:14.917  event: done
```

Posting order preserved, any-producer close honored, in both directions. Both edges
rendered the metadata-less terminal identically as `data: {}` — settling the R3 report's
"presentation nuance" note: **it is parity, not a divergence** (the R3 observation
compared against an elided line in the Java E3 evidence, not against the Java wire). ✅

## P4 — cross-engine chaos recovery

The **Rust** producer's `lost` mode stores a data segment *and* the terminal `eof` with
every wake-up suppressed, against a channel held by the **Java** UI pod (idle allowance
6s). Nothing wakes the pod; the **Java** watchdog's single final drain recovers both
Rust-stored segments and completes the render normally:

```text
18:04:38.758  event: cid
              (data + eof stored by the RUST producer, all wake-ups suppressed)
18:04:44.763  event: orders
18:04:44.763  data: stored by the Rust pod, wake-up lost
18:04:44.764  event: done
18:04:44.764  data: {"recovered":"cross-engine"}
```

Recovery at idle + 5 ms. **The recovery machinery itself crosses engines** — design D4's
cornerstone with the store and the drain on different runtimes. ✅

## P5 — the real-LLM leg (the Java E4 analog, cross-engine)

The blueprint payoff, now polyglot: the **Java** producer's `llm` mode pulls Gemini's own
SSE stream through the platform's SSE consumer and bridges each token batch into the
rendezvous — and the **Rust** edge renders it progressively, terminal `done` carrying the
provider's usage metadata (one short generation, quota-polite):

```text
18:05:05.572  event: cid
18:05:07.040  data: It enables diverse systems
18:05:07.063  data:  to seamlessly communicate and evolve independently without being
              locked into a single programming language or technology stack.
18:05:07.071  event: done
18:05:07.071  data: {"provider":"gemini","model":"gemini-3.8-flash","finishReason":"STOP",
              "promptTokenCount":15,"candidatesTokenCount":22,"totalTokenCount":37}
```

Real tokens, produced on a Java pod, rendered by a Rust pod — the model's own answer is a
fair summary of the gate. ✅

## Findings and round notes

1. **The wire-parity gate is MET.** Every scenario ran on the normative contract alone —
   no shims, no translation, no engine change. The port spec's §4 table is now
   live-verified in both directions, including the mixed-producer form.
2. **Mid-round server bounce — both engines healed their subscriptions on camera.** An
   operator mistake (see note 4) forced a full Redis restart with all four pods live. The
   Rust coordinator's explicit reconnect loop logged
   `Return-route subscriber re-subscribed to svc-return:…` and Java's Lettuce logged
   `Reconnected to 127.0.0.1:6379` — the §5 item 2 delta (hand-rolled resubscribe where
   Lettuce auto-resubscribes) proving equivalent live.
3. **One behavioral delta found on the bounce-recovery path (command lane):** after a
   server restart, the Rust pods' **first** Redis command fails (`broken pipe`) and heals
   on the next call — redis-rs's `ConnectionManager` arms an asynchronous reconnect but
   returns the failed command's error to the caller, and its retry configuration governs
   connection attempts, not command replay. Lettuce requeues not-yet-written commands
   across a reconnect, so the same first call on the Java pod simply works. The failure
   surfaces cleanly (a 500 on the producer endpoint; a pre-head 500 on a facade open) and
   the design already treats transport errors as retryable, but the engines differ in
   who retries: the library (Java) vs the application (Rust). **Maintainer decision
   requested:** accept and document, or harden the Rust store with a retry-once on its
   *idempotent* operations only (`save_route`/`get_route`/`cleanup`/`queue_length` —
   `SETEX`/`GET`/`DEL`/`LLEN`); the append deliberately stays fail-fast either way, since
   replaying an ambiguous `RPUSH` risks a duplicate segment (at-least-once) that the
   no-sequence-number design cannot detect.
4. **Operator note for the helper:** `kill -9` on the `redis-standalone` **wrapper JVM**
   orphans its `redis-server` subprocess, which keeps port 6379 and — once a later helper
   start wipes `/tmp/soa-redis` — fails its next background save and disables writes
   (`MISCONF … stop-writes-on-bgsave-error`). A plain SIGTERM/Ctrl-C shuts the subprocess
   down cleanly (verified at round close). The helper README's Ctrl-C guidance is the
   contract; this note records what the hard-kill failure looks like from the pods.

## What remains

Nothing in the R-series — **R1 → R4 complete; the port is functionally in lock-step** with
the Java engine's ratified design. Outside the series, recorded for their own gates: the
`mercury-sync-over-async` crates.io publication rides the next release (with the
path-only dev-dependency check), the one-shot facade tasks stay deferred until a transport
worth demonstrating lands (ruling Q1), and note 3 is **closed**: the store took the
idempotent-only retry-once first (spec §5 item 6), and the maintainer's ruling of 2026-09-22 —
retry intelligently, only when the broken pipe is a Redis restart or reconnection, on the strength of
simple lifecycle monitoring — moved the mechanism into the `redis-connection` foundation: a heartbeat
that notices the lost connection and makes the client reconnect *ahead* of the next command (so the
producer's first `RPUSH` after a restart, this note's symptom, now finds a fresh connection), one
retry per lost connection for idempotent commands only, and a single fast-failing attempt while the
connection is known down. `RPUSH`/`LPOP` stay unreplayed by design (D7).
