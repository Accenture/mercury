---
title: Interop Test Report — Event over HTTP, Java ⇄ Rust
summary: Permanent record of the live bidirectional interoperability validation between the
  Java and Rust implementations - wire format, calling patterns, and telemetry presentation
  parity - kept as a playbook for future language ports.
layer: reference
audience: [developer, architect]
keywords: [interop, event over http, rust, wire format, test report, tracing]
---

# Interop Test Report — Event over HTTP, Java ⇄ Rust

*Live bidirectional interoperability validation between the Java engine
([mercury-composable](https://github.com/Accenture/mercury-composable)) and the official
Rust implementation ([mercury](https://github.com/Accenture/mercury)): the wire-format and
calling-pattern drives conducted 2026-07-22 as the release gate for the v4.10 line, and the
telemetry presentation-parity drive conducted 2026-07-23.*

This report is a permanent record. It documents what was tested, the evidence collected,
and — in the interest of honest engineering — the defects the drives surfaced and how each
was fixed and re-verified. Everything here is reproducible from the shipped examples by
following the [Event over HTTP](../guides/event-over-http.md#zero-code-demo-hello-flow-to-hello-world) walk-through.
It is also kept as a **playbook for future language ports** (e.g. Python) — see the
learnings section at the end.

## Background: the wire-format validation drive

The first interop drive (2026-07-22, morning) validated the language-neutral
[standard event envelope wire format](https://accenture.github.io/mercury-composable/guides/event-envelope-wire-format/) end-to-end:
Java→Rust 7/7 and Rust→Java 7/7 test cases, including binary payloads, in-band error
semantics (403/404/408 as envelope status), async acknowledgement, and W3C trace
continuity in both directions. That drive found and fixed three defects — a Java HTTP
client read-timeout truncation
([PR #214](https://github.com/Accenture/mercury-composable/pull/214)) and its Rust mirror
plus an example echo binary drop
([mercury PR #166](https://github.com/Accenture/mercury/pull/166)). Golden conformance
vectors are shared verbatim between the two repositories.

## This drive: the two calling patterns, both directions

The second drive validated the two Event-over-HTTP usage patterns — **programmatic** (the
PostOffice request API with an explicit Event API endpoint URL) and **declarative** (a
foreign route resolved through `event-over-http.yaml`) — using the shipped example
applications as-is.

| Role | Drive A | Drive B |
|------|---------|---------|
| Caller (port 8100) | Java composable-example | Rust hello-flow |
| Callee (port 8085) | Rust hello-world | Java lambda-example |

Endpoints exercised on each caller: `POST /api/event/http/demo` (declarative; renamed to
`/api/event/http/declarative` after v4.10.0), `POST /api/event/http/programmatic`, and
`GET /api/event/http/demo`. **Zero configuration
changes between drives** — the callees are drop-in counterparts of each other (same port,
same public routes `hello.world` / `hello.declarative`), which is the point of the demo.

**Versions under test:** Java at
[PR #215](https://github.com/Accenture/mercury-composable/pull/215) (pre-4.10, CI green);
Rust at [PR #167](https://github.com/Accenture/mercury/pull/167).

## Results — functionality: 6/6 pass

- Every request returned HTTP 200 with `Content-Type: application/json` and the correct
  echo: the request body round-tripped intact, and the `origin` field identified the
  application instance that actually executed the function — in the other language.
- Java callee: the echoed `my_route` header discriminates the pattern —
  `hello.declarative` (declarative) vs `hello.world` (programmatic) — demonstrating why
  the echo function registers two route names.

## Results — trace continuity

Distributed traces were inspected at the individual span level in both applications' logs.

**Java → Rust: a fully connected cross-language span tree in both patterns.** For the
declarative call (trace `a6b8fa67…`): Java `http.flow.adapter` (span `9fbd1bdc`) → Rust
`event.api.service` (span `a092275c`, parent `9fbd1bdc` — chained across the wire) → Rust
`hello.declarative` (span `8e505921`) — and back on the Java side, the response callback
span parents onto the **Rust** function's span, closing the loop. The programmatic call
chains the same way through the Java `v1.event.over.http.rpc` task span.

**Rust → Java:** the declarative call chained fully (Java `hello.declarative` parented
onto the Rust flow-adapter span, matching Java-caller behavior exactly; even the echo's
internal fire-and-forget `hello.pojo` call chained onto the echo's span). The programmatic
call initially joined by trace id only — see finding I3 below.

With the [default-on application log context](../guides/observability.md#the-application-log-context),
every structured log line on both sides carried the same trace id — logs and spans join up
across the language boundary with zero setup.

## Findings and resolution

Span-level analysis surfaced three telemetry-fidelity defects, all in the Rust port and
none affecting functionality. Recording them here is deliberate: the drives exist to find
exactly this class of issue.

- **I1 — duplicate span records.** The Rust callee emitted two records per RPC-served span
  (its worker record plus the caller-side round-trip record). The Java engine emits exactly
  one: the worker suppresses its own record when serving an RPC and the caller's inbox
  record — carrying `exec_time`, `round_trip`, and span lineage — is *the* record for that
  span. The Rust port now applies the same suppression, and the callee's annotations ride
  the reply envelope (a wire-compatible field that survives Event-over-HTTP hops in either
  direction).
- **I2 — foreign span adoption on relayed replies.** A round-trip record adopted the span
  id of whatever function produced the reply — for a flow, that is the flow's final task,
  not the requested route — misattributing and duplicating a span reported elsewhere. Both
  engines now adopt the reply's span id only when the reply comes from the requested route
  itself (Java: `InboxBase.spanIdFromResponder`, commit `140640d8`; Rust: commit
  `8328d720`). The same defect had been caught on the Java side by the event-script
  engine's span-uniqueness regression suite during this same release cycle.
- **I3 — missing caller span on the programmatic wire envelope.** The Rust programmatic
  client did not stamp the calling function's span id onto the outbound envelope, so the
  remote callee's record lost its `parent_span_id` (the declarative path was correct). Now
  fixed; both patterns parent identically.

**Re-drive after the fixes (both directions, both patterns):** exactly one record per span
(zero duplicates), no foreign span ids on round-trip records, and callee records parent
onto the caller's task span in both patterns — every parent id cross-checked as a real
span in the caller's log.

## Verdict

**Interop validation passed in full.** Both Event-over-HTTP patterns are functionally
proven Java ⇄ Rust with zero configuration changes, and distributed-trace telemetry is
span-accurate across the language boundary in both directions. The shipped examples are
drop-in cross-language counterparts, and the walk-through in the
[Event over HTTP guide](../guides/event-over-http.md#zero-code-demo-hello-flow-to-hello-world) reproduces this
validation with a single `curl`.

## The presentation-parity drive (2026-07-23)

After v4.10.0 shipped, a manual review of all four direction combinations
(java-to-java, rust-to-rust, java-to-rust, rust-to-java) side by side — the way a
DevSecOps operator reads an aggregated log stream — raised the bar from "trace continuity
works" to a stronger requirement:

> **Field installations stay polyglot for a long time. Operators aggregate both engines'
> telemetry and logs in one place, and any presentation difference is a support burden.
> The same-language logs of the two engines must be exact structural replicas of each
> other — then cross-language runs are symmetric by construction.**

That review surfaced loose ends the per-direction drives had not:

- **Java:** the `/api/event` edge connected to no span — `event.api.service` was a
  zero-tracing relay, so the remote target function parented onto a span from another
  application with nothing in between, and the HTTP response leg floated unparented.
- **Rust:** an incomplete application log-context block (constants only) appeared on
  telemetry and system lines that carry no request trace; the first-leg
  `http.flow.adapter` span was never recorded (its RPC-style dispatch suppressed the
  worker record), leaving `parent_span_id` values that pointed at spans no record
  reported; and reserved `my_*` metadata could leak into HTTP response headers.

### Method: a normalized reference signature

The Java engine is the reference implementation. After fixing the Java `/api/event` edge
(the service is now traced: its span parents onto the remote caller's span, the target
function parents onto it, and both response legs chain onto real spans), a live
java-to-java run was distilled into a **normalized trace signature**: per calling pattern,
the exact set of telemetry records — service name, parent edge (expressed symbolically so
span-id values don't matter), record kind (`round_trip` for RPC-served spans vs
`exec-only` for callback-served worker records), and path. Volatile fields (ids, origins,
timestamps, durations, thread/instance numbers, ordering) are exempt; everything else must
match record-for-record.

**Declarative pattern — 8 records:**

| side | service | parent (span owner) | kind | path |
|------|---------|---------------------|------|------|
| caller | http.flow.adapter | (root) | exec-only | POST /api/event/http/declarative |
| caller | task.executor | http.flow.adapter@caller | exec-only | POST /api/event/http/declarative |
| callee | event.api.auth | http.flow.adapter@caller | round_trip | POST /api/event |
| callee | event.api.service | http.flow.adapter@caller | exec-only | POST /api/event |
| callee | hello.declarative | event.api.service@callee | round_trip | POST /api/event/http/declarative |
| callee | hello.pojo | hello.declarative@callee | exec-only | POST /api/event/http/declarative |
| callee | async.http.response | event.api.service@callee | exec-only | POST /api/event |
| caller | async.http.response | hello.declarative@callee | exec-only | POST /api/event/http/declarative |

**Programmatic pattern — 9 records:** identical shape with the caller's
`v1.event.over.http.rpc` task span between the flow adapter and the callee (the callee's
`event.api.auth` / `event.api.service` parent onto it), the callee function being
`hello.world`, and one deliberate asymmetry: the caller's `async.http.response` parents
onto the **callee's function span** in the declarative pattern (the flow task's reply *is*
the remote reply) but onto the **local task span** in the programmatic one (the flow's
reply is produced locally by that task).

The signature also pins the invariants: one record per span, no dangling parents, HTTP
response headers free of reserved `my_*` metadata, and the log-context gating rule — a
`context` block appears **only** on log lines emitted inside a traced function execution;
telemetry and system lines carry none at all.

### Reaching the bar required refactoring, not patching

Matching the signature exposed genuine low-level variance in the Rust port's RPC
implementation, resolved structurally: its REST automation now dispatches endpoint
services as **callbacks** through a registered `async.http.response` service (the Java
twin) instead of RPC through a oneshot inbox — which is what made the first-leg and
response-leg spans real — and the business correlation-id moved to the reserved
envelope-header channel for exact `PostOffice` parity. The log context was re-gated to
render only inside a traced, non-zero-traced worker execution. The same drive added the
`event.api.auth` authentication demo to both engines (a shared token resolved from the
`DEMO_PEER_TOKEN` environment variable — authentication appears in the trace as a real
span, and its session info rides to the target function as proof).

### Result: empty diff in all four directions

| Direction | Declarative (8 records) | Programmatic (9 records) |
|-----------|-------------------------|--------------------------|
| java → java (reference) | — | — |
| rust → rust | **empty diff** | **empty diff** |
| java → rust | **empty diff** | **empty diff** |
| rust → java | **empty diff** | **empty diff** |

Exactly as predicted: once the same-language logs were replicas, the cross-language runs
were symmetric with no additional work. Authentication verified in every direction
(session proof in the echo; wrong or missing token → HTTP-401), response headers clean,
and zero context blocks on untraced lines in either engine.

## Learnings for future language ports

This validation arc is the playbook for bringing the next language (e.g. Python) into the
family:

1. **Golden conformance vectors first.** The wire format is proven byte-identically with
   vectors shared verbatim between repositories — no prose interpretation.
2. **Same-language baseline before cross-language testing.** Distill a live run of the
   reference engine (java-to-java) into a normalized trace signature; the new port must
   produce an **empty diff** on its own same-language run. Cross-language symmetry then
   follows by construction — do not chase cross-language differences directly.
3. **The signature is more than topology.** It pins record kinds (round_trip vs
   exec-only), path values, one-record-per-span, no dangling parents, log-context gating,
   and transport-header hygiene — the things an operator actually sees in an aggregated
   view. Presentation parity is a standing invariant, not a one-off acceptance test:
   polyglot installations put both engines' output in front of the same DevSecOps team.
4. **Expect refactoring, not configuration.** Matching the signature will surface genuine
   low-level differences (RPC vs callback dispatch, context scoping). Fix the structure;
   telemetry-layer workarounds recreate the drift later.
5. **The demo pair is the reusable test vehicle.** A new port implements the hello-world /
   hello-flow counterparts on the same ports and route names, and every drive in this
   report — functionality, trace continuity, authentication, signature diff — runs against
   it unchanged, with zero configuration.
6. **Live drives find what unit tests do not.** Every drive in this story surfaced real
   defects (timeout truncation, span misattribution, context leakage, an invisible relay
   edge) that per-repository test suites had not caught.
