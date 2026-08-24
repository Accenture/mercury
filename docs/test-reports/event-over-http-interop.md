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
It is also kept as a **playbook for future language ports** — see the learnings section
at the end; the [polyglot wrapper round](#the-polyglot-wrapper-round-2026-08-22) records
the playbook's first execution, bringing Python and Node.js into the family.

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
  the echo function registers two route names. (The demo echo has since been changed to
  display the clean envelope-header view, so the injected metadata no longer appears in
  its output; the two route names remain.)

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

## The metadata-hardening round (2026-07-23)

A further manual four-direction review — after v4.10.1 — found two remaining issues: protected
metadata visible to user functions and responses, and the business correlation-id missing from
HTTP response headers. The trace signature was re-verified as an exact replica in all four
directions **before** any fix, isolating the issues to pure header hygiene. The round produced
a design ruling that is now the cross-engine contract:

> **A composable function has exactly three inputs — headers, body and instance. The headers
> are a COPY of the envelope headers with read-only metadata injected by the worker at entry
> and sanitized at exit. Metadata is never transported in the event itself.**

Root causes: the business correlation-id was the one metadata key transported as a real
envelope header (the envelope has fields for trace id and path, but none for the business
cid), so it leaked across the wire and into relayed events; the engine-internal `x-event-api`
relay guard reached function views; and neither engine echoed the correlation-id on HTTP
responses.

**Fixes (both engines):** the business correlation-id rides an engine-managed envelope tag —
`rpc` and `my_cid` are now documented reserved tag names in the
[wire-format specification](https://accenture.github.io/mercury-composable/guides/event-envelope-wire-format/) — with the worker
injecting the four `my_*` read-only keys at delivery and sanitizing them (plus `x-event-api`)
from returned envelopes at exit, covering the accidental-copy case; and REST automation echoes
the request's business correlation-id on every HTTP response. This also closed a day-one
injection asymmetry: the Rust callee now shows the identical `my_*` key set as Java, so the
echo demos are replicas again.

**The RPC reply path was aligned in the same round.** The Rust port had minted per-request
pseudo-routes under an `inbox.` prefix — reserving the entire `inbox.*` namespace, which
belongs to applications (workflow systems use routes like `inbox.approval` for human-approval
staging). It now mirrors the Java design: one reserved private route `temporary.inbox` with a
correlation-id-keyed pending registry, registered as an **essential service** at the highest
startup priority (with the deliberate concurrency triple: HTTP client 500, temporary.inbox
500, telemetry 1 — a singleton to preserve record ordering); the RPC marker is the reserved
`rpc` tag; and the mesh-era `route@origin` syntax is never generated (inbound-tolerant only —
instance addressing is a Kafka-service-mesh concern the port does not carry). A regression
proves a user function registered as `inbox.approval` is reachable, RPC-able, and traced
normally. The rework also surfaced a genuine pre-existing defect in the Rust HTTP client
service (replying through the global platform instead of its own).

### Regression re-test: the refactor is observably invisible

The working hypothesis — that the rework would make interop more robust *and predictable* —
was tested by re-running the full battery in both cross-language directions:

| Check | Java → Rust | Rust → Java |
|-------|-------------|-------------|
| Functionality (both patterns) | pass | pass |
| `X-Correlation-Id` response echo | pass | pass |
| Generated-cid identity across the wire | pass | pass (one value end to end) |
| Identical injected `my_*` key set | pass | pass |
| `x-event-api` in any function view | absent | absent |
| Authentication (session proof / 401) | pass | pass |
| **Trace signature** | **empty diff** (8+9 records) | **empty diff** (8+9 records) |

Same-language: Java full reactor and the Rust workspace suites green, with the rust-to-rust
signature also an empty diff. The internals hardened — a static route table, one cid-keyed
correlation concept local and remote, deterministic essential-service startup — while
observable behavior stayed byte-for-byte unchanged, which is exactly what the empty diff is
designed to prove.

## The configurable-traceparent round (2026-07-24)

A field installation's API gateway strips the standard W3C `traceparent` from its header
allow-list, so both engines gained a configurable traceparent **carrier name** —
`http.traceparent.header` (Java adds the `kafka.*` / `secondary.kafka.*` twins), with per-entry
overrides — shipped in lock-step (Java PR #227, Rust PR #177). Before release, this round
validated the feature live across the language boundary with the custom name `ce_traceparent`,
every application launched with `-Dhttp.traceparent.header=ce_traceparent` (the Rust port
honors the same `-D` override syntax).

The drive deliberately simulated the field scenario: the client supplied the W3C value **only
under `ce_traceparent`** — the standard header never sent — so any trace continuity observed
had to come from the custom carrier.

### Results — the feature is correct and symmetric

| Check | java→rust | rust→java |
|-------|-----------|-----------|
| Functionality (declarative + programmatic) | pass | pass |
| Edge adopts trace context from `ce_traceparent` alone | pass | pass |
| Caller's REST span parents onto the client's span | pass | pass |
| Callee `/api/event` spans parent onto the caller's span | pass | pass |
| Declarative reply-span asymmetry preserved cross-language | pass | pass |
| Authentication (`event.api.auth`) | pass | pass |
| `X-Correlation-Id` response echo | pass | pass |

Wire-level captures (a header-dumping listener standing in for the peer) confirmed the **dual
stamp** on both engines' outbound `/api/event` calls: `traceparent` and `ce_traceparent`
carrying the identical W3C value, alongside the trace-id header. A four-combination echo
matrix (java→java, java→rust, rust→rust, rust→java, both patterns, all eight runs under the
custom name) confirmed the feature introduces no cross-engine differences: every echoed
`ce_traceparent` and trace behavior was symmetric.

### Findings — pre-existing surfaces exposed by the matrix (not caused by the feature)

The deeper matrix inspection surfaced header-hygiene asymmetries that predate this feature:

1. **Both programmatic demo tasks transport their injected metadata.** Java's
   `EventOverHttpRpc` (`headers.forEach(req::setHeader)`) and the Rust twin (its `set_header`
   loop) copy the function's **injected** header view — including the four `my_*` keys — onto
   the outgoing envelope: the accidental-copy anti-pattern of learning #7, on the request side.
2. **The two engines sanitize different subsets at the receiving `/api/event` door.** With
   inbound `my_*` on the wire, a Java callee delivers `my_correlation_id` (the legacy-cid
   compat carrier) but strips `my_route`/`my_trace_id`/`my_trace_path`; a Rust callee strips
   `my_correlation_id` and `x-event-api` but delivers the other three. On the declarative
   path, `x-event-api: callback` is visible in a Java callee's envelope view, absent in a
   Rust callee's.
3. **Wire hygiene nits.** The Rust HTTP client emits each trace header twice (same value);
   it stamps `x-correlation-id` on the `/api/event` request where Java carries the business
   cid only in the envelope tag; Java sends `accept` and `x-small-payload-as-bytes` where
   Rust does not; and the Rust port does not log the resolved traceparent header name at
   startup (Java: `Traceparent HTTP header is 'ce_traceparent'`).

### Verdict

The configurable traceparent carrier works end to end in both directions: with the standard
header never supplied, the custom name alone carried the full W3C context — trace identity
**and** span parenting — through both engines' edges, app-to-app hops, and `/api/event`
doors. The feature is release-ready. The findings above were fixed in the hygiene round
below.

### Resolution — the hygiene round (2026-07-24, ships in v4.10.4)

All findings were fixed in lock-step and the matrix re-run:

1. **The delivered envelope view is scrubbed on both engines.** The worker removes the five
   engine keys (`my_route`, `my_trace_id`, `my_trace_path`, `my_correlation_id`,
   `x-event-api`) from the delivered envelope's own headers for non-interceptor functions —
   whatever a peer transported or a local edge merged can never masquerade as application
   data. The legacy `my_correlation_id` compat carrier remains honored into the injected
   view before the scrub, and event interceptors keep raw transport fidelity (on the Rust
   port this also *restored* interceptor fidelity that its earlier partial scrub had
   violated). Entry-side regression twins added on both engines. The investigation settled
   the attribution: the Rust port never injected metadata into the envelope view — the
   matrix leaks were pure transport (both programmatic demos' header-copy loops) plus, on
   the Java side, the declarative relay guard and the auth session-info merge reaching the
   envelope view.
2. **Both programmatic demo tasks forward business headers only** — the injected `my_*`
   view describes the local function's own context and is never transported.
3. **Wire hygiene aligned to the Java reference.** The Rust client now stamps each trace
   header exactly once, no longer adds `x-correlation-id` to the event-over-HTTP transport
   leg (the business cid rides the envelope tag), sends `accept` and
   `x-small-payload-as-bytes` like Java, and logs the resolved
   `Correlation-id / Trace-id / Traceparent HTTP header` names at startup.
4. **The endpoint timeout rides the request dataset as `x-ttl` on both engines.** Java's
   REST ingress represents the route timeout as the request's `x-ttl` header (caller-sent
   value wins), so a flow's `input.header` view carries it; the Rust ingress now does the
   same — with a regression pinning both the default and the caller-wins precedence.

**Final verification:** the four-combination matrix re-ran with both fixed engines under
`ce_traceparent` — **all eight echoes are identical** after normalizing volatile fields
(same header set, same values including `x-ttl` milliseconds), wire captures show the same
header set from both engines' clients, and cross-language span parenting held in both
directions. Gates: Java full reactor green (422 platform-core tests, 2 new); Rust workspace
260 passed (3 new), clippy 0, fmt clean.

**Post-round refinement — inbound precedence finalized before release.** In line with the
standards position, the inbound rule was flipped from the drive's original
custom-name-first semantics: the **standard `traceparent` always wins**, and the custom
name is read only when the standard header is absent. Rationale: a well-formed standard
traceparent means the legacy system already upgraded to the W3C/OTel standard, so a
proprietary header alongside it is residual and safely ignored. This also makes the family
self-consistent — the `trace.id.header` fallback already yields to the standard. Both
engines flipped in lock-step with their precedence regressions inverted, and note that the
drive's ce_traceparent-only scenario (the gateway simulation) is unaffected: the custom
name remains the carrier whenever the standard header does not survive the intermediary.

## The polyglot wrapper round (2026-08-22)

The playbook below was executed for real when the **polyglot initiative** brought Python
and Node.js into the family — not as engine ports but as lightweight **Event-over-HTTP
wrappers** ([mercury-python](https://github.com/Accenture/mercury-python),
[mercury-nodejs](https://github.com/Accenture/mercury-nodejs); see the
[Polyglot Functions](../guides/polyglot-functions.md) guide). Evidence from the
acceptance drives, conducted 2026-08-22 against engine v4.11.10:

1. **Golden conformance vectors first — item 1 of the playbook, literally.** Both
   wrappers' standard-format codecs are verified against the same golden vectors the
   Java and Rust engines share, carried verbatim in each wrapper's test suite. The
   classic compact format is detected and rejected with a teaching error (the wrappers
   are standard-only by design).
2. **Cross-wrapper drive.** Python ⇄ Node.js live in both directions — RPC replies,
   trace context, and reply annotations intact across both hops.
3. **The flagship drive: an unchanged engine flow executed a Python function.** The
   Java composable-example's shipped `event-over-http-declarative` Event Script flow —
   the zero-code demo of this report's earlier rounds (this engine's parallel:
   [hello-flow to hello-world](../guides/event-over-http.md#zero-code-demo-hello-flow-to-hello-world))
   — ran against the Python demo host in the callee's slot (same port 8085, same public
   `hello.declarative` route), with **zero engine change and zero engine configuration
   change**. The reply carried `"language": "python"`; the flow's business correlation
   id arrived injected as the read-only `my_correlation_id` header (the `my_cid` tag
   contract); `x-flow-id` and the flow's `x-ttl` were delivered; and the Python host's
   log line showed the **engine's trace id** in the engine-consistent log format —
   presentation parity extending to the wrapper family (playbook items 3 and 5).
4. **The one engine change shipped separately, in lock-step.** `graph.task`'s
   route-existence guard now consults the `yaml.event.over.http` map on both engines
   (Rust [PR #212](https://github.com/Accenture/mercury/pull/212), Java
   [PR #292](https://github.com/Accenture/mercury-composable/pull/292)), released in
   **v4.11.11** — deployed knowledge graphs can name polyglot routes directly.

Per the initiative's conformance ruling, this report gains a round like this one with
each wrapper release: the golden vectors are the per-release gate, and a live
engine-flow-to-wrapper drive is the acceptance proof.

## Learnings for future language ports

This validation arc is the playbook that brought Python and Node.js into the family (the
wrapper round above) and remains the checklist for the next language:

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
7. **Reserve the minimum; inject the rest.** Engine metadata is injected into the function's
   input copy at entry and sanitized at exit — never transported in the event — and the
   engine reserves the smallest possible surface (one RPC listener route, two tag names).
   Route namespaces and header space belong to applications; a port that borrows them for
   engine plumbing will collide with real-world naming sooner or later.
