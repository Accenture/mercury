---
title: Interop Test Report — Workflow Suspend/Resume, Java ⇄ Rust
summary: Permanent record of the live cross-engine workflow-suspension validation - a mixed
  Java/Rust fleet sharing one Redis state store, with every resume decoding a record the
  other engine persisted - kept as the release evidence for the suspend/resume feature.
layer: reference
audience: [developer, architect]
keywords: [interop, suspend, resume, workflow suspension, rust, redis, state store, test report]
---

# Interop Test Report — Workflow Suspend/Resume, Java ⇄ Rust

*Live cross-engine validation between the Java engine
([mercury-composable](https://github.com/Accenture/mercury-composable)) and the official
Rust implementation ([mercury](https://github.com/Accenture/mercury)) of the
[Workflow Suspension](../guides/knowledge-graph/workflow-suspension.md) feature, conducted
2026-07-30 as the release evidence for the suspend/resume arc (Java PRs #238–#242; Rust
PR #186).*

This report is a permanent record. It validates the deployment shape the feature was
built for: a **load-balanced mixed fleet** — some pods on the Java engine, some on the
Rust engine, all sharing one Redis — where a workflow may suspend on one engine and
resume on the other. Everything here is reproducible from the shipped examples: both
`minigraph-playground` example apps, the `tutorial-14` purchase workflow they both
deploy, and the Java repo's `helpers/redis-standalone` server (a real Redis as a plain
`java -jar` — the standard local test server for this port).

## Why this drive exists

The suspend/resume state-store contract is a **cross-language wire contract**: the
persisted record is a plain MsgPack map `{cid, node, ttl, model, seen, run}` under the
Redis key `graph:state:<cid>`, consumed atomically on retrieval (`GETDEL`). Nothing in it
is engine-specific — so by design, which engine wrote a record and which engine resumes
from it should be invisible to the workflow. Three implementation details are
load-bearing for that guarantee, and each was pinned during the cross-engine consistency
review before this drive:

- **The correlation ID is normalized identically on both engines** — it is the store
  key, so any normalization difference would make one engine's records unreachable from
  the other. At drive time the shared rule was "use the value raw"; immediately after
  the drive both engines adopted **trimming** instead (a business correlation ID such
  as an order number may be entered by an operator in a web UI, and accidental padding
  would otherwise split the key space) — the invariant is the *identical* normalization,
  and both engines changed in lock-step.
- **Reserved model keys never cross the boundary in either direction** — excluded on
  persist and stripped on restore with a *literal* key-level merge, so a record is safe
  external input regardless of its writer.
- **The traversal bookkeeping (`seen`/`run`) is keyed by node alias** — identical on
  both engines because a mixed fleet deploys the same graph JSON through the same
  CompileGraph manifest gate.

## Setup

| Component | Detail |
|-----------|--------|
| Java app | `examples/minigraph-playground` from mercury-composable main (`1ed8732a`), port 8085 |
| Rust app | `examples/minigraph-playground` from mercury main (post PR #186 merge, `d2791b09`), port 8100 (its default has since been synced to **8085**, same as Java, so manual engine-swap tests reuse one browser URL — for side-by-side runs, override one app's `rest.server.port`) |
| State store | `helpers/redis-standalone` 4.10.6 (a real embedded Redis as a plain `java -jar` — no Docker), port 6379, shared by both apps |
| Workflow | `tutorial-14` — three human checkpoints (order → approval → delivery release), four short runs, one correlation ID |
| Store functions | `v1.redis.persist.model` (SETEX) / `v1.redis.retrieve.model` (GETDEL) — this repo's `extensions/minigraph-state-redis` crate and its Java extension-module twin |

The two apps stand in for the mixed fleet; alternating requests between their ports
simulates the load balancer sending consecutive runs of one workflow to different pods.

## Method

Two interleavings of tutorial-14's four runs, each under one distinctive correlation ID,
so every consecutive pair of runs crosses the engine boundary:

| Run | Payload | `ce-java-first-1001` | `ce-rust-first-2001` |
|-----|---------|----------------------|----------------------|
| 1 — customer orders | `{"item": "laptop", "amount": 2000}` | **Java** | **Rust** |
| 2 — manager approves | `{"decision": "approved", "manager": "store-88"}` | **Rust** | **Java** |
| 3 — delivery releases | `{"release": true, "courier": "express"}` | **Java** | **Rust** |
| 4 — shipment confirms | `{"tracking": "TRK-12345"}` | **Rust** | **Java** |

Plus one rejection probe per engine (`{"decision": "approved"}` under a correlation ID
that never ordered) to confirm the input-validation contract is identical.

Per-run assertions: HTTP status, the `stage` reply, the **`run` flag** (`fresh` on run 1,
`resume` thereafter), and the echoed `cid`. Run 4 additionally asserts the **full
four-stage history** — order, approval, delivery, shipment — accumulated across three
suspensions, including the MsgPack integer-width probe (`amount: 2000` must survive
cross-engine round-trips as an integer).

## Results — all 50 checks passed

Both interleavings completed with every assertion green. The final reply, identical in
shape from either engine:

```json
{
  "stage": "shipped",
  "run": "resume",
  "order": {"item": "laptop", "amount": 2000},
  "approval": {"decision": "approved", "manager": "store-88"},
  "delivery": {"release": true, "courier": "express"},
  "shipment": {"tracking": "TRK-12345"},
  "cid": "ce-java-first-1001"
}
```

**Every restore was a cross-engine decode.** By construction of the interleavings, no
engine ever read a record it wrote itself:

| Restore | Record written by | Restored by |
|---------|-------------------|-------------|
| `ce-java-first-1001` run 2 | Java (run 1) | **Rust** |
| `ce-java-first-1001` run 3 | Rust (run 2) | **Java** |
| `ce-java-first-1001` run 4 | Java (run 3) | **Rust** |
| `ce-rust-first-2001` run 2 | Rust (run 1) | **Java** |
| `ce-rust-first-2001` run 3 | Java (run 2) | **Rust** |
| `ce-rust-first-2001` run 4 | Rust (run 3) | **Java** |

6/6 record handoffs crossed the engine boundary; the workflow state (model key-values
and the join-relevant traversal bookkeeping) survived every crossing byte-faithfully.

The rejection probes matched exactly on both engines: HTTP-404 with
`{"type": "rejected", "message": "Transaction not found. Submit the order first",
"run": "fresh"}` — the `run` flag telling the caller *why* (no record: never submitted,
or expired) on Java and Rust alike.

## Observability evidence

Presentation parity held across the fleet — the polyglot-operations requirement that a
DevSecOps team watching one log aggregation sees the same story from both engines:

- **Business correlation ID, business-or-nothing**: every traced store-function log line
  on both engines carried the business ID (`ce-java-first-1001` / `ce-rust-first-2001`)
  in its log context — never an internal routing ID. Sample, one from each engine:

    ```text
    Java: org.platformlambda.graph.redis.RetrieveModel:65 - Restored workflow state for cid ce-java-first-1001
    Rust: [minigraph_state_redis] Restored workflow state for cid ce-java-first-1001
    ```

- **Span topology**: on both engines the store-function spans are parented on their
  skill spans (`from=graph.resume` / `from=graph.suspend` on the Java telemetry records;
  span-linked service records on Rust), and a resumed run shows **no spans for
  checkpoints it did not re-execute** — the no-re-execution guarantee is visible in
  trace topology on either side of the fleet.

## Notes for operators

- **Same graph, same store, any engine.** The only fleet requirements are the ones any
  deployment already has: both engines deploy the same graph JSON through their
  CompileGraph manifest, and share the `redis.*` configuration. Engine mix is otherwise
  invisible to the workflow.
- **At-most-once resume is fleet-wide.** `GETDEL` consumes the record atomically in
  Redis itself, so a duplicate resume — even racing across two engines — finds nothing
  and behaves as a fresh transaction.
- **The correlation ID is a resume capability across the whole fleet**: protect
  resume-bearing endpoints with `rest.yaml` authentication regardless of engine.
- One first-request artifact worth knowing: the store connection is lazy on both
  engines, so the fleet's very first suspend/resume call pays the connect cost
  (~200 ms in this drive); subsequent calls are single-digit milliseconds.
