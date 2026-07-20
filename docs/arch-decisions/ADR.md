# Architectural Decision Records

A human-facing ledger of the **durable architecture decisions** behind mercury — one entry
per decision, capturing the *why* (context, alternatives, consequences) rather than the
*what that holds now*. The live constraints themselves are maintained in the project's
working memory (`memory/continuity.md` → *Architectural Invariants* / *Key Decisions*); each
ADR cross-links to the constraint it formalizes via a `formalizes:` pointer, and each such
constraint carries a matching `(ADR-NNNN)` tag. This ledger is read **on demand** — it is not
part of any per-session read path.

Entries are listed **newest first**. Numbering is monotonic and entries are **never
deleted**: a decision that no longer holds is marked *Superseded* (replaced by a newer ADR)
or *Deprecated* (no longer relevant), with its text left in place.

> **Provenance — this is a faithful port.** mercury is a Rust port of `mercury-composable`
> (canonical Java, v4.8.6). These ADRs were **adapted from that repo's
> `docs/arch-decisions/ADR.md`** (ADR-0001…0007) when the port's ledger was seeded
> (2026-07-18). The architecture is deliberately unchanged, so **most decisions carry over
> verbatim in intent**; each entry keeps its original number for cross-referencing, and its
> original decision **Date** (when the decision was first made in the Java project). Where the
> Rust *realization* differs from the Java one, the entry says so explicitly — chiefly
> **ADR-0002** (tokio async/await rather than Java 21 virtual threads over Vert.x) and
> **ADR-0006** (the Kafka service mesh is out of scope for this port). Continuity invariants
> are distilled incrementally (only `inv-never-couple-functions` / ADR-0001 is formalized so
> far); each `formalizes:` names the intended constraint id.

---

## ADR-0007 — Event Script configuration is preferred over code for orchestration {#adr-0007}
**Status:** Accepted · **Date:** 2026-06-27T15:45:00.000Z · **Serves:** vision-mercury
<!-- id: adr-0007 | status: accepted | formalizes: inv-event-script-over-code -->

**Abstract.** When a step is **orchestration** — sequencing functions, branching on a condition,
handling a failure, or moving data between steps — express it as **Event Script YAML** (tasks,
`execution` types, input/output data mapping, exception handler), not as imperative code inside a
function. Code is reserved for the **unit of work** itself (the function body; ADR-0005). The
boundary holds in both directions: a genuinely in-function concern (a computation, a blocking
rendezvous) stays in code — not all code becomes YAML.

**Rationale.** Two properties make configuration the better home for orchestration. **(1) It
communicates intent.** The flow file is a single, legible statement of the event flow — a reviewer
sees the `begin → publish → await` sequence, the routes, the fail-fast path, and the branches
without reading Rust. **(2) It manages dependencies.** Event Script declares both control-flow
dependencies (task order, decision branches, exception routing) and data-flow dependencies
(field-level mapping through `model`), and the engine enforces them — so functions stay fully
decoupled (ADR-0001), never referencing one another, with the only wiring in the flow. Reusable
building blocks are composed **by reference, not duplicated in code**. Cross-cutting behavior
(failure handling, status policy, `ttl` timeouts, trace propagation) becomes an engine concern
expressed in config rather than repeated boilerplate, and orchestration changes (add a step, change
a route, re-route a branch) are reviewable config edits that need no recompile. The accepted
consequences are the cost of the abstraction, not reasons to avoid it: the unit of work stays in
code, and declarative routing has its own vocabulary to learn (the `decision` type selects a `next`
entry by value; `*` whole-body passthrough carries opaque payloads through `model` — ADR-0003). This
decision refines ADR-0001 and is bounded by ADR-0005. **Rust port:** unchanged — the Event Script
engine (layer 2, `crates/event-script`) is a faithful port validated against the canonical Java flow
fixtures.

---

## ADR-0006 — Cloud-native by default; service mesh for sync-over-async and service discovery only {#adr-0006}
**Status:** Accepted · **Date:** 2026-06-23T18:30:00.000Z · **Serves:** vision-mercury
<!-- id: adr-0006 | status: accepted | formalizes: inv-cloud-native-mesh-opt-in -->

**Abstract.** The Kafka service mesh (`cloud.connector=kafka` + presence-monitor) is an **opt-in
capability** that solves two specific problems: (1) synchronous request-response between different
application instances over Kafka, and (2) service discovery between running pods. Applications that do
not need either capability must be designed **cloud-native** — each instance self-contained,
stateless, and horizontally scaled without cross-instance coupling. `cloud.connector=none` is the
framework default.

**Rationale.** Superimposing synchronous request-response over Kafka (an inherently asynchronous
transport) is technically feasible but architecturally expensive: cross-instance synchronous RPC
creates latency dependencies between otherwise independent scaling units, propagates errors across
instance boundaries, and erodes the isolation horizontal scaling is meant to provide — overuse
degrades a cloud application into a **distributed monolith**. Cloud-native design avoids these risks:
inbound load is distributed at the infrastructure layer (load balancer / Kubernetes ingress) and each
instance handles its share independently. The mesh should be adopted only for (a) cross-application
synchronous RPC that cannot be decoupled further, or (b) distributed resilience patterns that require
peer awareness. **Rust port:** the principle is retained, but the **Kafka service mesh itself is out
of scope for this port** (`minimalist-kafka`, `twin-kafka`, `connectors/` are not ported — see the
non-goals in `README.md` and the Vision). Consequence: cloud-native single-instance deployment is
currently the *only* model; if the mesh is ever ported, this ADR governs it as an opt-in capability.

---

## ADR-0005 — One atom, four roles {#adr-0005}
**Status:** Accepted · **Date:** 2026-06-22T22:47:23.000Z · **Serves:** vision-mercury
<!-- id: adr-0005 | status: accepted | formalizes: inv-one-atom-four-roles -->

**Abstract.** The sole building block of an application is the **route-addressed function** — in the
Rust port, a struct annotated `#[preload]` implementing `ComposableFunction` (or the typed
`TypedFunction`), with Map/struct I/O, private by default. There is no second primitive; the same unit
is **named by how it is wired**:

- **function** — the atom itself (registered in the `Platform` registry by route name);
- **service** — a function mapped straight to HTTP via `service:` in `rest.yaml` (a narrow REST role,
  distinct from `flow:`; see `automation/routing.rs`);
- **task** — a step in an Event Script flow carrying an `execution` type (one of
  `decision, response, end, sequential, parallel, pipeline, fork, sink`);
- **skill** — a function attached to an Active Knowledge Graph node via that node's `skill:` property.

**Rationale.** One primitive means one mental model and one programming model regardless of which
paradigm layer you are working in — learning to write a function transfers to every role, and a
function can be promoted from a flow task to a graph skill without being rewritten. The alternative —
distinct primitives per layer — would fragment the model and break the decoupling guarantee the whole
framework rests on (ADR-0001). Consequence: the role-names are kept precise — "function" is the
general atom, "service" is the narrow REST role (not a synonym for it), and a task is a role of the
atom, never a separate kind of thing. **Rust port:** unchanged in structure — the Java `@PreLoad` /
`LambdaFunction` / `TypedLambdaFunction` become `#[preload]` / `ComposableFunction` / `TypedFunction`;
the four roles are identical.

---

## ADR-0004 — Three-paradigm-layer architecture {#adr-0004}
**Status:** Accepted · **Date:** 2026-06-22T22:47:23.000Z · **Serves:** vision-mercury
<!-- id: adr-0004 | status: accepted | formalizes: inv-three-paradigm-layers -->

**Abstract.** The framework is organized as **three ascending paradigm layers**, each building on the
one below:

1. **Event-driven foundation** — platform-core: decoupled functions over the in-memory event bus
   (ADR-0001, ADR-0002).
2. **Composable orchestration** — event-script: a YAML DSL choreographing those functions into
   transactions.
3. **Semantic — Active Knowledge Graph** — MiniGraph: graph models that *execute* behavior through
   skills embedded on nodes.

These conceptual layers are **distinct from the runtime request pipeline** — whose *stages* run
outside in: user / calling application → protocol boundary (REST automation for HTTP) → flow adapter →
Event Manager / flow engine → in-memory event bus → composable functions. The word "layers" is
reserved for the three paradigms; the request flow is a *pipeline* with *stages*, never a layering.

**Rationale.** A single coherent ascent gives users both a mental model and an on-ramp: begin
event-driven, compose with Event Script, model semantically with the Active Knowledge Graph. Naming is
locked to remove a recurring source of confusion: *Active Knowledge Graph* is the model, *MiniGraph*
the engine, *semantic* an adjective only. Human–AI collaboration is a **cross-cutting capability**
across all three layers (agent-ready DSL specs + a companion endpoint), **not** a fourth layer.
**Rust port:** unchanged — the port delivers exactly these three layers bottom-up
(`crates/platform-core` → `crates/event-script` → `crates/knowledge-graph`); the lineage
(Scala/Akka actor model → Eclipse Vert.x event bus → Java 21 virtual threads) continues into this port
as **tokio** (ADR-0002).

---

## ADR-0003 — Function I/O contract: Map-or-struct over an immutable EventEnvelope {#adr-0003}
**Status:** Accepted · **Date:** 2026-06-22T22:47:23.000Z · **Serves:** vision-mercury
<!-- id: adr-0003 | status: accepted | formalizes: inv-typed-io-map-or-struct -->

**Abstract.** A typed function's normal input and output type is a **Map or a serde struct** (the Java
"Map or PoJo"). **Key-by-key data mapping** in Event Script (layer 2) and the Knowledge Graph
(layer 3) maps fields individually, so a List cannot serve as the mapping contract there — use a Map or
a single struct. The **`*` whole-body passthrough** (`model.list -> *`) is the special escape from
key-by-key mapping: it passes the entire state-machine value as the event body. Functions exchange the
immutable `EventEnvelope` message container: headers are `Map<String,String>`, and the body is
**MsgPack**-serialized on the wire (`rmpv::Value` in memory; `rmp-serde` + serde for struct↔Map
conversion).

**Rationale.** Constraining key-by-key I/O to Map-or-struct keeps Event Script data mapping clean and
readable and avoids serialization edge cases. A struct enforces an interface contract; a Map gives
flexible structure — together they cover the spectrum without admitting ambiguous generic collections.
The `*` passthrough is the intentional escape hatch for opaque / List payloads. **Rust port:** the
contract is unchanged; the realization uses **serde + rmpv** instead of Java's customized Gson +
MsgPack. The accepted consequences follow from the wire format: MsgPack normalizes integer widths, so
pin a type with a struct (`body_as::<T>()`) when the exact width matters, and read scalars through the
typed accessors rather than assuming a Rust integer type; Map keys are strings. Field-by-field mapping
over the state machine is the primary tool; JSON-Path (`$.…`) is the escape for complex queries
(layer 2/3).

---

## ADR-0002 — Async event engine: sequential-reading RPC at reactive performance {#adr-0002}
**Status:** Accepted · **Date:** 2026-06-22T22:47:23.000Z · **Serves:** vision-mercury
<!-- id: adr-0002 | status: accepted | formalizes: inv-async-tokio-rpc -->

**Abstract.** Functions execute as **tokio tasks** over an **`async-channel`** in-memory event bus
(one MPMC queue per route, with FIFO back-pressure). A PostOffice RPC call (`po.request(...).await`)
reads as a straight-line request→reply to the caller, while the tokio runtime multiplexes many
in-flight requests over a small pool of OS threads — so sequential-style code performs at async
throughput.

**Rationale — this is the one entry whose *realization* the port changes.** The **goal is identical**
to the Java framework's: keep the **clarity of sequential code** (the code reads as the intent of the
application) without paying the throughput cost of blocking an OS thread per in-flight request. Java
achieves this with **Java 21 virtual threads over an Eclipse Vert.x event bus** — a virtual thread is
suspended and its carrier kernel thread released across an RPC. The Rust port achieves the same
property natively with **`async`/`await` on tokio**: an `.await` suspends the task and frees the worker
thread, no virtual-thread machinery required. The lineage is preserved and extended: Scala/Akka actor
model (Mercury v1) → Vert.x event bus (v2) → non-blocking engine (v3) → Java 21 virtual threads
(v3.1+) → **tokio async (this Rust port)**. Alternatives considered for the port were the same in
spirit: a hand-rolled thread-per-request pool (caps concurrency) or a fully callback-style API (harder
to read) — both rejected for the same reasons Java rejected them. Consequences specific to the port:
the framework requires the tokio runtime; the actor-model discipline is enforced by the borrow checker
plus the route-only coupling rule (ADR-0001) — notably **never hold a `MutexGuard` across an
`.await`**; and the port's platform-core benchmark measured this design **outperforming** the Java
virtual-thread baseline on the same RPC workloads (see `docs/INCREMENTS.md`, the platform-core
milestone).

---

## ADR-0001 — Decoupled functions wired by route names; orchestration as Event Script {#adr-0001}
**Status:** Accepted · **Date:** 2026-06-22T22:47:23.000Z · **Serves:** vision-mercury
<!-- id: adr-0001 | status: accepted | formalizes: inv-never-couple-functions -->

**Abstract.** All application logic is packaged as **self-contained functions** —
`#[preload]`-annotated structs implementing `ComposableFunction` / `TypedFunction`, registered in the
`Platform` registry and addressed **exclusively by a route-name string**. Functions hold no direct
reference to one another; they communicate only by exchanging immutable `EventEnvelope` messages over
the event bus. **Orchestration** — the sequencing of functions into a transaction — is declared in
**YAML Event Script**, not written in code; the only link between a flow and a function is the
route-name string.

**Rationale.** Full decoupling is the foundation the entire framework rests on: functions can be
developed, tested, deployed, relocated, and recomposed into new flows without recompiling or knowing
about each other. Moving orchestration out of code and into configuration makes the sequencing
reviewable and changeable on its own, and roughly halves application code. The alternatives — direct
calls or dependency-injection wiring between components, and imperative orchestration code — were
rejected because they reintroduce compile-time coupling and bury the transaction flow in control
logic. The accepted consequence is that the route-name string is the whole contract between a flow and
a function, so route-naming discipline matters and is enforced by convention. This decision is
elaborated by ADR-0005 (the one function atom plays four wiring roles) and realized on the runtime of
ADR-0002. **Rust port:** this is the defining invariant carried over unchanged — the actor-model
decoupling the whole three-layer design rests on (`inv-never-couple-functions`).
