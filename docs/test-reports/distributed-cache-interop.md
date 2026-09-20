---
title: Interop Test Report — Distributed Cache, Java ⇄ Rust
summary: Permanent record of the live cross-engine distributed-cache validation - the Java and Rust
  distributed-cache examples sharing one Redis, every layer of either engine reading, deleting and
  missing what the other wrote, through a Redis outage and back - kept as the certification evidence
  for the v1.cache.redis lock-step.
layer: reference
audience: [developer, architect, devops]
keywords: [interop, distributed cache, redis, v1.cache.redis, rust, msgpack, outage, recovery, test report]
---

# Interop Test Report — Distributed Cache, Java ⇄ Rust

*Live cross-engine validation between the Java engine
([mercury-composable](https://github.com/Accenture/mercury-composable)) and the official Rust
implementation ([mercury](https://github.com/Accenture/mercury)) of the
[Distributed Cache](../guides/distributed-cache.md) — conducted 2026-09-20 UTC (the evening of
2026-09-19 Pacific) as the certification evidence for the `v1.cache.redis` lock-step: Java shipped the
module in v4.12.9, the Rust port merged as mercury PR #285.* The same report is kept in the Java repository's `docs/test-reports/`; the Java engine is the reference implementation.

This report is a permanent record. It validates the deployment shape the module was built for: a
**mixed fleet** — some pods on the Java engine, some on the Rust engine — sharing **one Redis cache**,
where a value written by any layer of either engine must read back, and be evicted, through any layer
of the other. Everything here is reproducible from the two shipped `distributed-cache-example`
applications and the Java repository's `helpers/redis-standalone` server.

## Why this drive exists

The cache contract is a **cross-language wire contract**: the key is `{redis.cache.key.prefix}{key}`
(here `cache-demo:` plus the profile id), the value is opaque bytes the *application* owns, and every
key carries its TTL from birth (`SETEX`). Nothing in the key or the value is engine-specific — so by
design, which engine wrote a value and which engine reads it must be invisible to the caller. Four
details are load-bearing for that guarantee, and each was pinned before the drive:

- **One action vocabulary.** Both engines' `v1.cache.redis` take the same actions, the same headers
  (`action` / `key` / `ttl`), the same configuration keys and return the same error messages — a mixed
  fleet is configured once (Rust port spec §2–§3, byte-compatible by construction).
- **The example's value is a plain MsgPack map** — Java `MsgPack.packMapOrList`, Rust `rmpv` — not a
  Java `EventEnvelope.toBytes()` holder. The Java example was refactored to this form (PR #395)
  precisely because the Rust port cannot read the envelope encoding.
- **One model, two engines.** The Layer 2 flow (`l2-profile.yml`) and the Layer 3 graph
  (`profile-cache.json`) are byte-identical files in the two repositories.
- **One error shape.** A REST client sees `{status, message, type: error}` from either engine whether
  the failure came from routing or from a function (Rust aligned in PR #285).

## Setup

| Component | Detail |
|-----------|--------|
| Java app | `examples/distributed-cache-example` from mercury-composable main (`c74ee6be`) plus the Layer 1 fix below; `java -jar distributed-cache-example-4.12.12.jar`, port **8305**; `/info` reports `4.12.12` |
| Rust app | `examples/distributed-cache-example` from mercury main (`5ac55eaf`, PR #285) plus the Layer 1 fix below; `distributed-cache-example -Drest.server.port=8306`, port **8306**; `/info` reports `4.12.7` (the Rust engine adopts the Java number at its catch-up release) |
| Redis | `helpers/redis-standalone` 4.12.12 — a real `redis-server` as a plain `java -jar`, no Docker — port 6379, shared by both apps |
| Cache settings (both apps) | `redis.cache.key.prefix=cache-demo:`, `redis.cache.default.ttl=1h`, `mandatory.health.dependencies=redis.health`, `app.env=dev` |
| Driver | a Python script over plain HTTP (`urllib`), with `redis-py` and `msgpack` for raw key inspection; run three times — discovery, fix verification, certification (this record) |

The two apps stand in for the mixed fleet; a request to one port is "the load balancer picked a Java
pod", to the other "a Rust pod". Layer 1 is the `v1.profile.l1` function driving the cache in code,
Layer 2 the `l2-profile` Event Script flow, Layer 3 the `profile-cache` graph behind the standard
`POST /api/graph/{graph_id}` endpoint.

## Method

**Main phase.** (A) `/info` and `/health` on both engines. (B) Each engine writes one profile through
each of its three layers — six writers — and every one of the six readers (three layers × two
engines) reads all six: a 6 × 6 matrix, 18 of whose cells cross the engine boundary. (C) The raw keys
are read straight from Redis: layout, TTL, and the bytes decoded as MsgPack; the same payload is then
written by both engines to compare bytes. (D) Every layer of each engine deletes a profile the *other*
engine wrote, the writer confirms the miss on yet another layer, and Redis confirms the key is gone.
(E) The miss bodies of all three layers and the Layer 3 unknown-action rejection are compared
field-by-field between the engines.

**Outage phase.** `redis-standalone` is stopped. Each engine's `/health` is read, then five probes —
Layer 1 GET, POST and DELETE, Layer 2 GET, Layer 3 get — must each fail as an *error*: never a 404
miss, never a `stored` acknowledgement.

**Recovery phase.** `redis-standalone` is restarted (empty — no persistence). Each engine must return
to `/health` 200 **without an application restart**, serve a Layer 1 write again within 60 s, and the
other engine must read that write on Layers 2 and 3. Two negative controls close the run: the earlier
keys are gone (a real restart, not a reconnect illusion) and the POST attempted during the outage
never landed.

## Results — certification run: 112/112 checks passed

Every hard check passed; one further *informational* comparison (§C) is recorded as expected. The
discovery run had exposed one defect in both engines (Finding 1), fixed and re-verified before this
run.

### A. Identity and health

Both `/info` endpoints answer 200 with the application block; both `/health` answer 200 with
`redis.health` listed as the mandatory dependency.

### B. The 6 × 6 matrix — every reader reads every writer

| Writer \ Reader | Java L1 | Java L2 | Java L3 | Rust L1 | Rust L2 | Rust L3 |
|------------------|---------|---------|---------|---------|---------|---------|
| **Java L1** writes `java-l1` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Java L2** writes `java-l2` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Java L3** writes `java-l3` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Rust L1** writes `rust-l1` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Rust L2** writes `rust-l2` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Rust L3** writes `rust-l3` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

36/36 reads returned HTTP 200 with the exact profile map; the **18 cross-engine cells** are the interop
proof, the 18 same-engine cells the control.

### C. The wire — key layout, value format, TTL

| Key | Written by | Raw value (hex) | Decoded (MsgPack) | TTL at read |
|-----|-----------|------------------|-------------------|-------------|
| `cache-demo:java-l1` | Java L1 | `82a46e616d65ac4a617661204c617965722031a5656d61696cb36a6176612d6c31406578616d706c652e636f6d` | `{"name":"Java Layer 1","email":"java-l1@example.com"}` | 3600 s |
| `cache-demo:java-l2` | Java L2 | `82a46e616d65ac4a617661204c617965722032a5656d61696cb36a6176612d6c32406578616d706c652e636f6d` | `{"name":"Java Layer 2","email":"java-l2@example.com"}` | 3600 s |
| `cache-demo:java-l3` | Java L3 | `82a46e616d65ac4a617661204c617965722033a5656d61696cb36a6176612d6c33406578616d706c652e636f6d` | `{"name":"Java Layer 3","email":"java-l3@example.com"}` | 3600 s |
| `cache-demo:rust-l1` | Rust L1 | `82a5656d61696cb3727573742d6c31406578616d706c652e636f6da46e616d65ac52757374204c617965722031` | `{"email":"rust-l1@example.com","name":"Rust Layer 1"}` | 3600 s |
| `cache-demo:rust-l2` | Rust L2 | `82a5656d61696cb3727573742d6c32406578616d706c652e636f6da46e616d65ac52757374204c617965722032` | `{"email":"rust-l2@example.com","name":"Rust Layer 2"}` | 3600 s |
| `cache-demo:rust-l3` | Rust L3 | `82a5656d61696cb3727573742d6c33406578616d706c652e636f6da46e616d65ac52757374204c617965722033` | `{"email":"rust-l3@example.com","name":"Rust Layer 3"}` | 3600 s |

Every value is a plain MsgPack map (`0x82` = a two-entry map) that decodes to the profile; every TTL is
the 1 h default counting from the write; the keyspace held exactly the eight `cache-demo:` keys and
nothing else.

**Informational — the same payload, written by both engines, is not byte-identical:**

```text
Java  L1 → 82a46e616d65a45477696ea5656d61696cb07477696e406578616d706c652e636f6d
Rust  L1 → 82a5656d61696cb07477696e406578616d706c652e636f6da46e616d65a45477696e
```

Both decode to `{"name": "Twin", "email": "twin@example.com"}`. The Java packer writes the map in
insertion order (`name`, `email`); the Rust packer serialises `serde_json`'s map, whose keys are sorted
(`email`, `name`). MsgPack map order carries no meaning and is not part of the contract — the 36/36
matrix above is the proof — but a byte-level comparison of stored values across engines must not be
used as an equality test.

### D. Cross-engine deletes

| Profile | Deleted by | Miss confirmed by | Redis key |
|---------|-----------|-------------------|-----------|
| `java-l1` (written by Java L1) | **Rust L2** → HTTP 200 | **Java L3** → HTTP 404 `Profile not found` | gone |
| `rust-l1` (written by Rust L1) | **Java L3** → HTTP 200 | **Rust L2** → HTTP 404 `Profile not found` | gone |
| `java-l2` (written by Java L2) | **Rust L3** → HTTP 200 | **Java L2** → HTTP 404 `Profile not found` | gone |
| `rust-l2` (written by Rust L2) | **Java L2** → HTTP 200 | **Rust L3** → HTTP 404 `Profile not found` | gone |
| `java-l3` (written by Java L3) | **Rust L1** → HTTP 200 | **Java L3** → HTTP 404 `Profile not found` | gone |
| `rust-l3` (written by Rust L3) | **Java L1** → HTTP 200 | **Rust L1** → HTTP 404 `Profile not found` | gone |

6/6 deletes crossed the engine boundary, each layer of each engine deleting once; every writer then
missed on a third layer with the same body, and every key was gone from Redis.

### E. One error shape

| Case | Java body | Rust body |
|------|-----------|-----------|
| Layer 1 miss | `{"message": "Profile not found", "status": 404, "type": "error"}` | identical |
| Layer 2 miss | `{"message": "Profile not found", "status": 404, "type": "error"}` | identical |
| Layer 3 miss | `{"message": "Profile not found", "status": 404, "target": "v1.profile.decode", "type": "error"}` | identical |
| Layer 3 unknown action | `{"message": "Invalid action. Use get, save or delete", "status": 400}` | identical |

A client sees the same status, the same keys and the same values from either engine, with
`content-type: application/json` on both.

### Outage — Redis stopped

Both `/health` endpoints answered **HTTP 400** with `"status": "DOWN"` and the dependency block
reporting `redis.health` at 503 with a `{code, text}` message — the same contract on both engines,
with the cause text engine-specific: Java `Redis is not reachable - Command timed out after 5 second(s)`, Rust `Redis is not reachable - broken pipe`.

| Engine | Probe | Result | Latency | Message |
|--------|-------|--------|---------|---------|
| Java | L1 GET | HTTP 500 | 5.02 s | `Timeout for 5000 ms` |
| Java | L1 POST | HTTP 500 | 5.01 s | `Timeout for 5000 ms` |
| Java | L1 DELETE | HTTP 500 | 5.01 s | `Timeout for 5000 ms` |
| Java | L2 GET | HTTP 500 | 5.01 s | `Command timed out after 5 second(s)` |
| Java | L3 get | HTTP 500 | 5.02 s | `Command timed out after 5 second(s)` |
| Rust | L1 GET | HTTP 500 | 0.0 s | `Redis error - broken pipe` |
| Rust | L1 POST | HTTP 408 | 5.01 s | `Request timeout for 5000 ms` |
| Rust | L1 DELETE | HTTP 500 | 4.46 s | `Redis error - Connection refused (os error 61)` |
| Rust | L2 GET | HTTP 500 | 5.01 s | `Redis request timed out after 5000ms` |
| Rust | L3 get | HTTP 500 | 4.46 s | `Redis error - Connection refused (os error 61)` |

Every probe failed as an error — none answered a 404 miss, none acknowledged `stored`, and the POST
never landed in Redis (verified after the restart). The latencies are Finding 2; the statuses Finding 3.

> **Amended after the run:** with the Java Finding 3 fix in place, the three Java Layer 1 rows read **HTTP 408**
> `Timeout for 5000 ms` (re-probed 2026-09-20 01:17 UTC); the Java Layer 2 and 3 rows and every Rust row are unchanged.

### Recovery — Redis restarted, no application restart

The outage lasted 54 s (Redis stopped at 00:37:07 UTC, listening again at 00:38:01). Both `/health`
endpoints answered 200 on the first read after the restart.

| Engine | Layer 1 write attempts after the restart | Recovered |
|--------|------------------------------------------|-----------|
| Java | attempt 1 at +0 s → HTTP 500 (`Timeout for 5000 ms` — the shared Lettuce connection was still on its reconnect backoff); attempt 2 → HTTP 201 at **+10.2 s**, the moment Lettuce's scheduled reconnect fired (`Reconnected` logged at +10.5 s) | yes, without a restart |
| Rust | attempt 1 → HTTP 201 (its first probe, at +10.2 s, since the driver probes the engines in sequence); the `ConnectionManager` reconnects on demand, so it was ready as soon as Redis was | yes, without a restart |

Each engine's post-recovery write was read by the other on Layers 2 and 3 (4/4); the restarted store
had lost the earlier keys and held no key from the outage POST. The asymmetry is Finding 4.

## Findings

**1. Layer 1 turned a cache failure into a miss — on both engines. Fixed.** In the discovery run,
with Redis stopped, the Rust Layer 1 GET answered HTTP 404 `Profile not found` while the
Rust log showed the cache function failing (`Redis error - broken pipe`); the Java Layer 1 answered
HTTP 500 only because its RPC timed out first. Both `ProfileCacheL1` functions awaited the
cache reply and tested its *body* — `byte[]`/`Value::Binary`, or `null` for a miss — without checking
the reply's **status**. A function that throws replies with the error status and the message as a
string body, so a fast failure (connection refused, an auth rejection) read as "nothing cached", and a
POST would have acknowledged `stored`. Layers 2 and 3 never had the gap: the flow and graph engines
check a task's status for the author — the *Event Script over code* principle, observed in the wild.
The fix is one guard in each Layer 1 function (an error-status reply is rethrown as the function's own
error), pinned on both sides by a test that swaps `v1.cache.redis` for a stub that fails fast, so the
assertion depends neither on a Redis outage nor on which of two timeouts fires first. Both fixes ship with this report — the Rust one in the same change as this page (Increment 120), the Java one in the companion change in the mercury-composable repository (its twin of this page).

**2. Failure latency differs by client library — timing, not semantics.** During the outage the Java
probes each took ~5 s: Lettuce buffers commands on a disconnected connection until its command timeout
(`redis.timeout`, 5 s), and the Layer 1 RPC timeout (5 s) races it. The Rust probes were mixed: the
first failed in 0 s (`broken pipe` on the dead multiplexed connection), the next ones waited on the
`ConnectionManager`'s reconnect attempts (~4.5 s of exponential retries) or the RPC timeout. Same
category on both sides — every probe an error — with different latency. Operators of a mixed fleet
should expect the Java pods to fail slow (bounded by `redis.timeout`) and the Rust pods to fail faster.

**3. An in-function RPC timeout surfaced as 500 on Java and 408 on Rust — the Java side was a
platform-core mapping gap, since fixed.** Java's `po.request(...).get()` rethrows the timeout inside an
`ExecutionException`, and both places that turn a function's exception into a reply read the status off
the *outermost* exception while taking the message from the root cause — so the wrapper's default 500
shipped with the cause's message. The Java engine now resolves the status from the **cause chain** (the
first `AppException`, `TimeoutException` → 408 or `IllegalArgumentException` → 400 wins; 500 only when
none is present), one rule shared by its three mappers. Re-probed live after that fix with Redis stopped
(2026-09-20 01:17 UTC): the Java Layer 1 GET, POST and DELETE answer **408 `Timeout for 5000 ms`** — the
code this engine already returned (`po.request` yields an `AppError` 408 as a `Result`, and the worker
renders an `AppError`'s status faithfully; there is no wrapper class to hide it). Java's Layers 2 and 3
keep answering 500 `Command timed out after 5 second(s)`, Lettuce's own timeout exception, which carries
no status. Fixed in the Java engine on 2026-09-20 (mercury-composable, the follow-up to PR #426).

**4. Recovery is bounded by the client library's reconnect policy, and health can be green first.**
The Rust `ConnectionManager` reconnects on the first command after Redis is back. Lettuce reconnects on
its own **exponential backoff, capped at 30 s**: in this run its attempts fell at +0, +9, +17 and +34 s
into the 54 s outage, and the next one — 30 s later, at +64 s — was the one that succeeded, 10.5 s
after Redis had returned. In the fix-verification run, whose recovery window was only 10 s, that same
backoff made every Java cache call fail for the whole window while `/health` already reported UP,
because `redis.health` probes on a **fresh** connection rather than the cache's shared one. So after
an outage longer than ~30 s a Java pod may serve up to ~30 s of cache errors while its health is
green; the window closes by itself and no request is misreported (they fail as errors, Finding 1),
but an operator watching `/health` alone would call the pod healthy before it served a cache call.
Worth a follow-up ruling: reset the shared Lettuce connection when a command times out (so recovery
is bounded by `redis.timeout` instead of the backoff), lower the reconnect cap, or document the
window. Not changed here.

**5. What matched exactly.** The health contract (`UP`/`DOWN`, HTTP 200/400, the dependency block with
`{code, text}`), the miss and rejection bodies on all three layers, the key layout, the TTL semantics,
and — the point of the drive — 36/36 reads and 6/6 deletes across the engine boundary.

## How to reproduce

```shell
# 1. one Redis (the Java repository)
java -jar helpers/redis-standalone/target/redis-standalone-x.y.z.jar

# 2. the Java example (port 8305) and the Rust example (moved to 8306)
java -jar examples/distributed-cache-example/target/distributed-cache-example-x.y.z.jar
cargo run -p distributed-cache-example -- -Drest.server.port=8306

# 3. write on one engine and layer, read on the other engine's three layers
curl -s -X POST http://127.0.0.1:8305/api/l1/profile/alice -H 'content-type: application/json' \
     -d '{"name":"Alice","email":"alice@example.com"}'
curl -s http://127.0.0.1:8306/api/l1/profile/alice
curl -s http://127.0.0.1:8306/api/l2/profile/alice
curl -s -X POST http://127.0.0.1:8306/api/graph/profile-cache -H 'content-type: application/json' \
     -d '{"action":"get","id":"alice"}'

# 4. evict from the Rust side, confirm the miss on the Java side
curl -s -X DELETE http://127.0.0.1:8306/api/l2/profile/alice
curl -s -i http://127.0.0.1:8305/api/l1/profile/alice        # 404 Profile not found

# 5. the outage leg: stop redis-standalone, repeat step 3 (every call is an error, never a 404),
#    restart it and repeat step 3 again (both apps recover without a restart)
```

`x.y.z` is the current version in each repository's build file. Both examples read `REDIS_HOST` /
`REDIS_PORT` from the environment when Redis is not on `127.0.0.1:6379`.
