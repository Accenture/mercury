---
title: Workflow Suspension (suspend/resume)
summary: Express a long-running business process as a sequence of short graph runs -
  suspend at a human checkpoint, persist the workflow state to a pluggable store, and
  resume later with the same business correlation ID without re-executing completed steps.
layer: knowledge-graph
audience: [architect, developer]
keywords: [suspend, resume, workflow, human-in-the-loop, approval, checkpoint, state store, redis, correlation id]
related:
  - guides/knowledge-graph/skills-reference.md
  - guides/knowledge-graph/build-your-first-graph.md
  - guides/reserved-names-and-headers.md
---

# Workflow Suspension (suspend/resume)

> **At a glance**
>
> - **What it is** — A graph run can **suspend** at a human checkpoint (approval,
>   intervention, inbox notification): its model is persisted to an external state store
>   and the run completes normally. A later request with the **same business correlation
>   ID resumes** past the checkpoint without re-executing it. A long-running business
>   process becomes a sequence of short runs — nothing stays in memory between them.
> - **Vocabulary** — the `graph.suspend` and `graph.resume` skills, the reserved node
>   alias `suspend`, and the node property `suspend=true`.
> - **The store is pluggable** — Redis ships as the `minigraph-state-redis` extension
>   crate; any composable function honoring the [store contract](#state-store-contract)
>   works.

## Why short runs

An approval may take minutes or days. Parking a live graph instance for that long would
pin memory, defeat timeouts, and not survive a restart. Suspension inverts the problem:
the run **ends** — the caller gets a `{"type": "suspended", "cid": ...}` reply — and the
workflow's durable memory (the `model` namespace) waits in the state store under the
business correlation ID with a time-to-live you choose. The resumed run is an ordinary
graph execution that happens to start with restored state. Because the record key is the
correlation ID, a workflow suspended on one application instance can resume on **any**
instance sharing the store.

## The three vocabulary pieces

**1. The `suspend` node** — exactly one per graph, and the alias `suspend` is **reserved**
(like `root` and `end`): traversal jumps to it *by name*. Its skill assembles and persists
the state envelope through the attached store function — no data mapping needed:

```text
create node suspend
with type Suspend
with properties
purpose=Persist workflow state to the external state store
skill=graph.suspend
task=v1.redis.persist.model
ttl=2d
```

`ttl` is **mandatory with no default** — a checkpoint may wait a minute or days, and only
the workflow designer knows. It uses duration syntax (`20s`, `5m`, `2h`, `2d`) and becomes
the store record's expiry. The `suspend` node also needs an **outgoing connection**
(normally to `end`): without one, the record would persist and the run would then stall
instead of completing — the compiler rejects the graph.

**2. A suspensible node** — any skilled node marked `suspend=true`. After its skill
completes and its output mapping runs, traversal routes to the `suspend` node instead of
its normal forward path. Draw **both** edges — the checkpoint edge to `suspend` and the
continuation edge — so the diagram tells the whole story (the compiler enforces this).
Routing skills (`graph.math`, and `graph.js` in engines that ship it — it is retired in
this Rust port) cannot be suspensible. A plain edge *into* the `suspend` node is an
unconditional suspension point — no property needed.

**3. The resume node** — conventionally named `resume`, placed right after `root` (or
after setup nodes). When the store has a record for `model.cid`, it restores the model,
re-arms the traversal bookkeeping (a downstream `graph.join` still sees branches that
completed before suspension), and jumps past the checkpoint. When there is no record —
a fresh transaction, the normal first-run case, or an expired one — traversal simply
continues along the resume node's own forward path.

Either way, the skill records the outcome in **`model.run`** — `resume` when a record was
restored, `fresh` when there was none. The engine deliberately does not distinguish
absent from expired (with several checkpoints in one graph, no single fallback node could
be right for all of them): whether an expired approval needs its own response is
**application logic**. Gate the resume node's forward path with a `graph.math`
IF-THEN-ELSE — on `model.run` or on the request shape, exactly as tutorial-14 does — to
reject the request, advise the UI, or jump to a recovery node.

```text
create node resume
with type Resume
with properties
purpose=Restore workflow state from the external state store
skill=graph.resume
task=v1.redis.retrieve.model
```

Types (`Suspend`, `Resume`, `Suspensible`) are **visual convention** — they pick the node
colors in the Playground; the skill defines the behavior.

## Walkthrough: the purchase workflow (tutorial-14)

`tutorial-14` (shipped with the engine, runnable in the `minigraph-playground` example
app) is the complete multi-checkpoint pattern — **three human checkpoints, four short
runs, one correlation ID**:

```text
root → resume → order (suspend=true) → check-approval → approval (suspend=true) → delivery (suspend=true) → ship → end
                                        ↑        ↘ manager-reject → end
                                        └─ await-decision (suspend=true)
```

A customer orders, the store manager approves **or rejects with a reason**, the delivery
department releases the shipment, and the parcel ships — one `suspend` node serves every
checkpoint, and each suspensible node captures its actor's input into the model and
stages its own stage-specific reply (overriding the default `suspended` response). The
manager's decision lands at a `graph.math` decision node on the `order` checkpoint's
continuation with three outcomes: an approved decision routes to the next suspension
point, an explicit rejection routes to a terminal node that reports the manager's reason
(the workflow ends), and anything else — a missing or unrecognized decision — re-suspends
through `await-decision`, whose continuation loops back to the decision, so an invalid
request can never end a long-running workflow by accident. The decision must sit
**before** the suspensible nodes: a suspensible node always suspends when its skill
completes — it cannot evaluate the input and choose not to — so the routing choice is
made first, and only an approved decision reaches the checkpoint that suspends for the
next actor. Run it with Redis (e.g. the Java repo's `helpers/redis-standalone` — a plain
`java -jar` real Redis for developer machines without Docker) and drive the four runs
with one correlation ID.

Run 1 — the customer orders a laptop; the run suspends at the `order` checkpoint and
replies with `"run": "fresh"` (a new transaction):

```bash
curl -s -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \
  -H 'content-type: application/json' -H 'x-correlation-id: order-1001' \
  -d '{"item": "laptop", "amount": 2000}'
```

```json
{"stage": "order-submitted; waiting for store manager approval", "run": "fresh", "cid": "order-1001"}
```

Run 2 — with the same `x-correlation-id`, the store manager approves. The `resume` node
restores the persisted state and continues past the `order` checkpoint without
re-executing it, into the `check-approval` decision — every reply from here on carries
`"run": "resume"`:

```bash
curl -s -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \
  -H 'content-type: application/json' -H 'x-correlation-id: order-1001' \
  -d '{"decision": "approved", "manager": "store-88"}'
```

```json
{"stage": "approved; waiting for the delivery department to release the shipment", "run": "resume", "cid": "order-1001"}
```

Run 3 — the delivery department releases the shipment:

```bash
curl -s -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \
  -H 'content-type: application/json' -H 'x-correlation-id: order-1001' \
  -d '{"release": true, "courier": "express"}'
```

```json
{"stage": "released; waiting for shipment confirmation", "run": "resume", "cid": "order-1001"}
```

Run 4 — shipment confirmation completes the workflow:

```bash
curl -s -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \
  -H 'content-type: application/json' -H 'x-correlation-id: order-1001' \
  -d '{"tracking": "TRK-12345"}'
```

```json
{
  "stage": "shipped",
  "run": "resume",
  "order": {"item": "laptop", "amount": 2000},
  "approval": {"decision": "approved", "manager": "store-88"},
  "delivery": {"release": true, "courier": "express"},
  "shipment": {"tracking": "TRK-12345"},
  "cid": "order-1001"
}
```

Every stage's input crossed every suspension — the model accumulated `order`, `approval`
and `delivery` across four separate runs, and a later checkpoint simply re-persisted the
grown state under the same correlation ID.

The manager may **reject** instead — the alternative run 2. An explicit
`"decision": "rejected"` routes to the terminal rejection, which reports the manager's
reason together with the original order, and the workflow ends — the record was already
consumed on resume and nothing re-suspends, so a further request under the same
correlation ID is a fresh 404 rejection:

```bash
curl -s -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \
  -H 'content-type: application/json' -H 'x-correlation-id: order-2002' \
  -d '{"decision": "rejected", "reason": "budget exceeded"}'
```

```json
{
  "stage": "rejected",
  "reason": "budget exceeded",
  "order": {"item": "laptop", "amount": 2000},
  "run": "resume",
  "cid": "order-2002"
}
```

A **missing or unrecognized decision** takes the third path: the reply is
`"stage": "awaiting-decision; supply decision approved or rejected for the store manager"`
and the workflow **re-suspends** — `await-decision`'s continuation loops back to
`check-approval`, so the next resume re-evaluates the decision and only an explicit
`approved` or `rejected` moves the workflow forward. This is also what a replay against a
leftover suspended record now yields — a self-explanatory "still waiting" instead of a
surprise.

The tutorial also validates its input: a request that is not an order submission, for a
correlation ID with no suspended record, is **rejected with HTTP 404** — the order must
come first. Three techniques worth stealing from its model:

- **Null-safe presence check.** The math expression engine has no null literal, but `{var}`
  substitution inside a `text()` constant is null-safe:
  `MAPPING: text(={input.body.item}) -> model.order_probe` always yields a present
  string (`=null` when the field is absent), which an `IF` can compare safely. The
  `check-approval` decision reuses the same idiom, so a missing decision safely routes to
  the wait loop rather than raising a runtime error.
- **A wait loop across suspensions needs `RESET`.** The traveler and executor never
  re-execute a node they have already seen — and the `seen` marks are part of the
  persisted state, so they survive suspension. A loop that revisits a decision on every
  resume must clear its own nodes first: `check-approval`'s
  `RESET: check-approval await-decision` runs before its `IF` statements (an `IF` that
  jumps to a node returns immediately), un-marking both loop nodes so the next resumed
  run can walk them again.
- **The run flag.** `graph.resume` sets `model.run` to `fresh` or `resume`, and the
  tutorial stages it into every reply (`model.run -> output.body.run`) — so the UI always
  knows whether it is looking at a new transaction or a resumed continuation, and a
  rejected later-stage request tells the caller *why* (`"run": "fresh"` on a
  decision-shaped body means the record expired or never existed).
- **Declarative response status.** A graph may stage its own HTTP status —
  `int(404) -> output.status` in the rejection node. A non-2xx status routes through the
  surrounding flow's exception handler, which passes a staged map body through (minus any
  `stack` key, with its `status` key corrected) — so give your rejection fields names other
  than `status`.

```bash
curl -s -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \
  -H 'content-type: application/json' -H 'x-correlation-id: order-9999' \
  -d '{"decision": "approved"}'
```

```json
{"type": "rejected", "message": "Transaction not found. Submit the order first", "run": "fresh", "status": 404}
```

## Design rules

- **The model is the workflow's durable memory.** Only the `model` namespace persists —
  a node's `{node}.result` scratch does not survive suspension. Map anything a later step
  needs into `model.*` **before** the checkpoint.
- **A suspensible node is a complete working node — only its exit changes.** It executes
  its skill in full (input mapping → skill → output mapping) before routing to the
  `suspend` node, so it may carry any non-routing skill (`graph.data.mapper`,
  `graph.task`, `graph.api.fetcher`, `graph.extension`), capture the actor's input into
  `model.*`, and stage the caller's reply in `output.*` — and its non-checkpoint edge
  defines exactly where the next run continues after resume. What it never does is
  choose: it cannot route, and it cannot decline to suspend.
- **Decide before you suspend.** A suspensible node always suspends — when its skill
  completes, traversal routes to the `suspend` node unconditionally; it cannot inspect
  the input and opt out. Place a routing node (e.g. `graph.math`) on the resume
  continuation, **before** the next suspensible node, to decide whether the workflow
  continues to that checkpoint, branches, or ends — as tutorial-14's `check-approval`
  does with the manager's decision.
- **A suspension point must be the sole active branch.** Do not suspend between a fan-out
  and its join — branches in flight cannot be persisted (the engine logs a warning);
  suspend *after* the join instead. Joins whose predecessors completed before suspension
  work: their completion marks are part of the persisted state.
- **One resume per transaction.** The shipped stores consume the record atomically on
  retrieval (`GETDEL` on Redis 6.2+, an atomic `MULTI/EXEC` `GET`+`DEL` transaction on
  older servers), so a duplicate resume — a double click, a retried message —
  finds nothing and behaves as a fresh run instead of double-executing the continuation.
  A later checkpoint in the resumed run simply persists a new record under the same ID.
- **The correlation ID is a resume capability.** Whoever presents it continues the
  workflow: protect resume-bearing endpoints with rest.yaml `authentication`, and use
  non-guessable IDs (the engine generates UUIDs when the caller supplies none).
- **Suspension does not cross `graph.extension`.** The business correlation ID does not
  propagate into delegated sub-graphs or flows today — design resumable workflows as
  top-level graphs.
- Reserved model keys (`model.cid`, `model.instance`, `model.flow`, `model.ttl`,
  `model.trace`, `model.run`) are never persisted — the resumed run's own identity is
  authoritative. `model.run` is part of the read-only flow metadata family: `graph.resume`
  is its only writer, and the flow compiler rejects any data mapping that targets it
  (like the other reserved keys).

## The state store contract {#state-store-contract}

The store is an ordinary composable function named by the suspend/resume nodes' `task`
property — the Redis crate below is one implementation; PostgreSQL, DynamoDB, MongoDB or
anything else plugs in the same way.

**Persist** — invoked by `graph.suspend`; headers `type=put`; request body:

```json
{
  "cid":   "<business correlation ID - the retrieval key>",
  "node":  "<the suspension point>",
  "ttl":   172800,
  "model": { "the model namespace minus reserved keys": "..." },
  "seen":  { "traversal bookkeeping": true },
  "run":   { "traversal bookkeeping": true }
}
```

Store the body **opaquely** (the reference implementations use MsgPack — binary values
round-trip; note the platform's [serialization gotchas](../api-overview.md)) and reply
2xx only when the record is durable — the reply is the acknowledgement `graph.suspend`
requires before the graph completes; any error fails the suspension.

**Retrieve** — invoked by `graph.resume`; headers `type=get`; body `{"cid": "..."}`.
Return the stored record as-is, or **null / an empty map** when absent or expired — an
absent record is the normal fresh-transaction case, never an error. Consume the record
atomically on retrieval (or document your replay semantics). If the store has no native
TTL, implement record expiry yourself.

The smallest possible reference implementation is the engine's test fixture — a temp-file
store (`FileStateStore` in the knowledge-graph crate's test sources).

## The Redis store crate

`extensions/minigraph-state-redis` ships `v1.redis.persist.model` (SETEX, native expiry)
and `v1.redis.retrieve.model` (atomic consume: `GETDEL` on Redis 6.2+, a `MULTI/EXEC`
transaction on older servers — detected per connection and stated in the startup log).
Add the crate as an
application dependency and reference it from `main.rs` (the linker keeps its annotation
inventory) — the two functions register automatically; the connection is lazy, so the
application boots normally without Redis until a workflow actually suspends.
Configuration uses the same `redis.*` keys as the sync-over-async family (`redis.host`,
`redis.port`, `redis.password`, `redis.ssl`, `redis.database`, `redis.timeout.ms`), and
the worker counts are ops-tunable via `worker.instances.v1.redis.persist.model` /
`worker.instances.v1.redis.retrieve.model`. See the crate README for details.

## See also

- [Built-in skills reference](skills-reference.md) — `graph.suspend` / `graph.resume` entries.
- [Build your first graph](build-your-first-graph.md) — graph authoring basics.
- [Reserved names & headers](../reserved-names-and-headers.md) — the extension routes.
