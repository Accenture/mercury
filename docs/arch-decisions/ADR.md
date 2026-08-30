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

## ADR-0017 — Route pools: numbered singleton lanes as a first-class platform registration {#adr-0017}
**Status:** Proposed · **Date:** 2026-08-30 · **Serves:** vision-mercury · **Formalizes:** route-pool-registration-design
<!-- id: adr-0017 | status: proposed -->

**Abstract.** `Platform::register_route_pool(prefix, function, count)` registers a set
of private singleton routes `{prefix}.{n}` for n = 0 to count-1 and returns the member
names in order; `release_route_pool(prefix)` removes the set symmetrically. Each member
is a strict FIFO lane (one worker, one shared stateless function), so a caller that
checks out a lane gets per-conversation event ordering while other lanes serve
concurrent traffic — the pattern the HTTP edge's streaming reply lanes
(`async.http.response.stream.{n}`) introduced, promoted from an open-coded loop in the
server start to a platform API with registry-level identity (a pool registry mapping
prefix to lane count). Pools are always private: lane checkout is an in-process
rendezvous, so publishing members over Event-over-HTTP would be meaningless.
Registering an existing pool reloads it (the previous member set is released first,
house reload semantics); individual register/release calls that touch a pool member are
warned, never refused — range-checked, so a neighbor route such as `{prefix}.10` beside
a count-3 pool is never misclassified. Pool mutations serialize on a dedicated mutex
(the Java twin uses a ReentrantLock). The pool registry is lifecycle metadata only —
`routes()` remains the truthful registry view and the actuator's compact pool rendering
stays display-only. Lock-step twin of the Java engine's ADR-0020.

**Rationale.** The v4.12.0 streaming milestone shipped the lane-pool pattern without an
abstraction: 500 `register_private` calls at server start, no release counterpart, and
no way for the platform to tell a pool from 500 coincidentally numbered routes; upcoming
consumers (graph-run streaming, wrapper relay pools under the AI SDLC work) would each
re-open-code it. Naming and semantics follow the ratified cross-engine design (the Java
repo's draft-design-specs/register-route-pool.md, D1–D10): `registerStreams` was
rejected there for colliding with the Java engine's `registerStream(StreamFunction)`
API, and the pool abstraction is ordered lanes, not streams; an `is_private` flag was
rejected because no remote use case exists. Port nuance: this engine registers the lane
pool on EVERY server start (the per-test-runtime rebind idiom), so the reload path is
routine here rather than exceptional — one "Reloading route pool" warning replaces five
hundred per-lane reload warnings.

---

## ADR-0016 — The HTTP client consumes SSE progressively; Event-over-HTTP streams on the same call {#adr-0016}
**Status:** Accepted · **Date:** 2026-08-29 · **Serves:** vision-mercury · **Formalizes:** async-http-client-sse-streaming-design
<!-- id: adr-0016 | status: accepted -->

**Abstract.** `async.http.request` consumes a `text/event-stream` response progressively
and relays it as the platform's own streaming protocol: one `x-event-stream: data`
envelope per upstream SSE event to the caller's reply route (the event's data as body,
`event:` name as `x-event-name`, head control - upstream status plus the SSE content
type - on the first envelope), `eof` on a clean end, and an in-band `exception` on idle
expiry or a mid-stream disconnect. Activation is explicit and standard: the request must
declare `Accept: text/event-stream`, the response must actually be SSE, and the request
must carry a `reply_to`; anything else keeps the buffered single-shot behavior. For a
stream, the request timeout becomes the per-read idle allowance (any upstream bytes,
keep-alive comments included, reset it). Payloads are never interpreted - provider
conventions such as `data: [DONE]` forward verbatim, keeping the client vendor-neutral.
Because the streaming producer contract is the one the HTTP edge already consumes, a
streaming endpoint's function can forward its own `reply_to` and correlation id into the
client call and the application becomes an SSE-to-SSE relay by configuration. The same
enhancement is the transport for Event-over-HTTP peer streaming (python/node wrapper
functions and engine⇄engine, the next phases): the peer's `/api/event` answers the SAME
call with an SSE response using a hybrid control/data framing - control signals ride
base64-encoded MsgPack envelope frames under the reserved SSE event name `envelope`,
while token segments ride raw SSE frames - negotiated by the same Accept contract.
This is the Java engine's ADR-0019 twin (Java PR #300): activation contract, event
mapping, idle semantics, and in-band messages are engine-identical; the port-idiomatic
difference is that the relay runs as a spawned tokio task reading the hyper body
frame-by-frame (each read bounded by the idle allowance), so a long stream never holds
a client worker instance.

**Rationale.** Progressive delivery had reached the HTTP edge (ADR-0015) but not the two
consumption paths AI-era workloads need: an engine function consuming an LLM provider's
token stream, and a polyglot wrapper function streaming results back to the engine. One
mechanism closes both because the Event-over-HTTP relay already flows through the HTTP
client. Alternatives rejected in the shared design round: an on-demand WebSocket channel
(scope-fence growth across four codebases, gateway-hostile, drifts toward the
standing-connection mesh the framework keeps opt-in), per-segment POSTs back to the
reply lane (FIFO forces serialized posts - one round trip per token - and opens an
inbound path to reply lanes), gRPC/HTTP-2 push, and long-polling. Streaming on the
response of the engine's own request adds no inbound surface, is ordered by TCP for
free, and rides the wire shape gateways already accommodate for LLM traffic.

---

## ADR-0015 — HTTP response streaming rides the multi-shot reply route; the wire stays standards-only {#adr-0015}
**Status:** Accepted · **Date:** 2026-08-28 · **Serves:** vision-mercury · **Formalizes:** http-response-streaming-design
<!-- id: adr-0015 | status: accepted -->

**Abstract.** A function streams an HTTP response (LLM token segments, agent progress
events, live updates) by exercising the platform's native streaming pattern: the callee
sends a sequence of events to the caller-provided reply route until an
end-of-transmission signal. Each event carries the reserved **envelope** header
`x-event-stream: data | eof | exception`; the marker is internal protocol consumed by
the REST automation edge — it never appears on the wire. The public HTTP surface is
standards-only: Server-Sent Events framing when the content type is `text/event-stream`
(typed events, a terminal `done` event carrying trailing metadata, in-band `error`
events, keep-alive comments), chunked transfer with JSON Lines otherwise. A streaming
endpoint is declared with `stream: true` in rest.yaml, which checks out a dedicated
ordered reply lane for the request's lifetime — a single-instance route
(`async.http.response.stream.{n}`) drawn LIFO from a pool of 500 (the
`async.http.response` concurrency), returned when the request ends — the "ready" signal
pattern of the reactive manager/worker design. One request's segments ride its own lane
(strict FIFO) while different requests stream concurrently; an exhausted pool rejects
further streaming requests immediately with HTTP-503 (deterministic back-pressure, no
configuration knob). The first event commits the response head; each arrival extends
the idle allowance; stalls fail in-band with status 408; client disconnects turn late
segments into no-op drops; the response header transform applies to the streamed head
with single-shot parity. Responses without the marker are single-shot, exactly as
before.

**Rationale.** The prerequisite for AI-era workloads is progressive delivery over plain
HTTP — SSE is the de facto wire for chat token streams, agent progress protocols, and
the live-watch window of long-running workflows. The multi-shot reply route adds no new
substrate — anything that can send an envelope to a route can stream, which keeps the
mechanism language-neutral by construction: flow tasks, graph nodes, and Event-over-HTTP
peers join by sending the same envelopes. This is the Java engine's ADR-0018 twin
(Java PR #299): the envelope vocabulary, the rest.yaml surface, the SSE framing, the
503 message, and the `/info/routes` family compression are engine-identical; the
internal execution differs idiomatically (a tokio renderer task enforces the idle
allowance directly instead of Java's housekeeper sweep — the wire behavior is the same).
The Java-side `x-stream-id` relay remains a documented deferral of this port.

---

## ADR-0014 — Polyglot functions are Event-over-HTTP peers, not subprocesses or ports {#adr-0014}
**Status:** Proposed · **Date:** 2026-08-22 · **Serves:** vision-mercury · **Formalizes:** polyglot-event-over-http-design
<!-- id: adr-0014 | status: proposed -->

**Abstract.** Functions written in Python and Node.js join Event Script flows and
MiniGraph knowledge graphs as long-lived **Event API peers**: each official wrapper
([mercury-python](https://github.com/Accenture/mercury-python),
[mercury-nodejs](https://github.com/Accenture/mercury-nodejs)) hosts `POST /api/event`
with the engines' exact semantics and speaks the standard envelope wire format, verified
against the golden conformance vectors this engine shares with the Java engine. The
engine addresses a polyglot route through the existing declarative
`yaml.event.over.http` map, so a flow task or `graph.task` node calls a Python or
Node.js function exactly as if it were local, with trace context, the `my_cid` →
`my_correlation_id` injection, and the portable error contract (handler errors ride
HTTP 200 with envelope status; transport errors keep 400/403/404/408 with
engine-identical messages) intact. The wrapper scope is fenced: envelope codec, Event
API host, `preload` registry, thin `PostOffice` client, a primitive in-process event bus
(per-route FIFO mailboxes with faithful `instances`; no spill tier, no queue cap), the
engines' actuator endpoints, the minimalist utilities (configuration with the
`resources/` convention and `-Dkey=value` overrides, engine-format logging with
`log.format=text|json|compact`, trace context), and a dev runner — **no orchestration**:
flows, graphs, persistence, and pub/sub stay on the engines. **Rust realization:** the
single engine change is the `graph.task` route-existence guard consulting the
declarative map (`event_api::get_event_http_target` in `skills.rs`), shipped lock-step
with the Java engine in v4.11.11. The initiative's design record
(`polyglot-event-over-http-design`) lives in the Java repository's shared memory; the
wire conformance vectors are the acceptance gate for every wrapper, and each wrapper
release extends the interop test report.

**Rationale.** The alternative designs were a full language port and an engine-managed
subprocess runner, and both were investigated. A full port re-implements the composable
core (event bus, flow engine, graph engine) per language — this repository *is* such a
port and demonstrates the cost profile: staying current is a sustained engineering
commitment, justified for Rust as a first-class engine, unjustifiable per scripting
language (the legacy Node.js port fell ~2 years behind and was retired). A subprocess
runner (functions as child processes over stdio) was prototyped on the Java engine and
shelved: kernel-thread isolation per in-flight call, process-tree lifecycle management,
and per-call interpreter startup create an operational stability surface the peer model
does not have. The peer model reuses what already works: the function contract is
route-name + envelope (nothing in it names a language), the Event API endpoint already
carries it across instances and across engines, and the declarative map already
abstracts location. Keeping orchestration out of the wrappers preserves the
architecture's one boundary — the engine tier owns sequencing, retries, and
back-pressure (a leaf host fails fast by deadline instead of hoarding work) — and keeps
each wrapper small enough to stay in lock-step through a conformance suite rather than a
porting effort. Engine-consistent utilities and actuator endpoints are part of the
decision, not convenience: polyglot installations put every language's telemetry, logs,
probes, and dashboards in front of one DevSecOps team, so presentation parity is a field
requirement extended from the two engines to the wrapper family. The Node.js wrapper is
also the sanctioned answer to the retired legacy port — a fresh re-port was never going
to stay current; a thin protocol wrapper can.

## ADR-0013 — Generic exception context: one handler serves every `exception=` route {#adr-0013}
**Status:** Proposed · **Date:** 2026-08-10 · **Serves:** vision-mercury · **Formalizes:** field-graph-scoped-state-and-error-context-rust
<!-- id: adr-0013 | status: proposed -->

**Abstract.** When a failed node routes to its `exception=` handler, the walkers
(executor and traveler — one staging site each, covering every skill) stage a **generic
exception context** in the state machine: `error.source` (the failing node's alias),
`error.code`, `error.message`, and `error.stack` when the failing record carries one
(this engine has no native stack-trace transport — a documented port divergence; the
"when available" contract holds on both engines). The names follow **Event Script's flow
exception contract** (`error.code/message/stack`) so both orchestration layers share one
vocabulary; `source` is the graph-side addition, since a graph handler — unlike a flow's
— is a node that many other nodes may name. The per-node error record
(`{node}.status`/`{node}.error`) is unchanged, so existing handlers keep working. `error`
is a **reserved node alias** (it always was, in the graph model's reserved-name list) —
the namespace is a first-class state-machine citizen like `model`, inspectable in a
dry-run with `inspect error`. A shared handler is island-anchored
(`root → island → handler`, the ADR-0011 anchoring idiom) because exception routing is a
jump; it is entered at most once per run unless RESET, and it may connect onward to
further processing nodes.

**Rationale.** The field demo showed graphs cluttered with per-fetcher handler clones —
structurally forced, because the engine staged failures only under the failing node's own
scratch (`{node}.status`/`{node}.error`), so a handler's data mapping had to name its
failing node statically. Staging at the walkers' exception choke points (rather than in
each skill) covers `graph.api.fetcher`, `graph.task`, `graph.extension` and the
suspend/resume store failures in one move, with no per-skill drift. For an extension
node, `error.source` is the extension node in the parent graph — failures inside a
delegated subgraph route to that subgraph's own handlers, composing with ADR-0012's
self-containment. Event Script parity naming was chosen over graph-local naming
(`error.status`/`error.error`) for cross-layer consistency and because `error.error`
reads badly. The context is transient per run and never persisted across suspension.

## ADR-0012 — A business transaction spans graphs: workflow-state records are scoped by graph + cid, and delegation inherits the correlation ID {#adr-0012}
**Status:** Proposed · **Date:** 2026-08-10 · **Serves:** vision-mercury · **Formalizes:** field-graph-scoped-state-and-error-context-rust · **Amends:** ADR-0009, ADR-0011
<!-- id: adr-0012 | status: proposed -->

**Abstract.** The suspend/resume state-store contract is scoped by **graph + cid**: the
persistence envelope gains a `graph` field (`{cid, graph, node, ttl, model, seen, run}`),
the retrieve body becomes `{cid, graph}`, and the Redis reference implementation keys
records `graph:{graph_id}:{cid}` (formerly `graph:state:{cid}`). Key composition remains
store-internal, but every store MUST scope by both — a cid-only key collapses all of a
transaction's suspensions into one record. Complementarily, **`graph.extension` stamps
the parent's business correlation ID** (`model.cid`) on the delegated call — both a graph
id and a `flow://` target, both branches (single and `for_each`) — exactly as an Event
Script sub-flow launch already did, closing the asymmetry where a subgraph's `model.cid`
was an unusable per-call random UUID. Together these make suspension **self-contained per
graph by construction**: one business transaction may suspend independently in each
domain's graph and in each subgraph, a resume only ever sees its own graph's records, and
an orchestrator parent can delegate independently resumable subgraph paths (the
documented orchestrator pattern, pinned by the `unit-test-orchestrator` /
`unit-test-sub-suspend` reference models). **Breaking change** (accepted, dev-phase — no
legacy fallback): records persisted under the old key are invisible after upgrade and
resume behaves as fresh; custom stores must adopt the `graph` field.

**Rationale.** The field demo drove both halves: the same business correlation ID is used
across multiple domains, so `graph:state:{cid}` collided across domains sharing a Redis —
and their orchestrator use case (a parent graph delegating suspend/resume per processing
path) was structurally impossible, for two independent reasons the code study confirmed:
the key collision, and `graph.extension` minting a random UUID per call with no
business-cid header, so a subgraph's resume could never find its record even with scoped
keys. The graph ID is unique per domain, so `graph + cid` addresses both the domain and
the subgraph requirement with one convention. Scoping is enforced in the contract (the
reference stores fail fast on a missing `graph`) rather than by engine-composed opaque
keys, keeping key layout a store concern. The `for_each` caveat is documented, not
enforced: one graph × one cid = one record, so a suspendable subgraph is invoked once per
cid per run. ADR-0009's cid-as-capability note extends naturally: one cid now unlocks
resume in every graph that suspended under it, scoped per graph id, with endpoint
authentication unchanged.

## ADR-0011 — Suspension is a destination: edge-mode checkpoints and jump-mode decisions replace the `suspend=true` property {#adr-0011}
**Status:** Accepted · **Date:** 2026-08-07T22:50:34.000Z · **Serves:** vision-mercury · **Formalizes:** suspend-resume-rationalization-rust · **Amends:** ADR-0009
<!-- id: adr-0011 | status: accepted | formalizes: suspend-resume-rationalization-rust -->

**Abstract.** A suspension point is declared by **how a node reaches the reserved
`suspend` node**, not by a node property — the `suspend=true` property is retired
(accepted and ignored for one deprecation window, with a compiler WARN). Two modes,
discriminated **purely by graph shape**: in **edge mode**, a working node with a drawn
edge to `suspend` redirects there when its skill completes normally — the drawn edge is
the declaration; the node must keep at least one continuation edge, where a resumed run
continues **without re-executing** the node (byte-for-byte the prior suspensible
behavior). In **jump mode**, a decision (`graph.math`) returns `suspend` from its
IF-THEN-ELSE; on resume the decision is **re-executed** against the new request input, so
it re-decides on every resume — a wait-on-invalid-input loop is one jump with no
auxiliary nodes. A routing-skill node must **not** draw an edge to `suspend` (its drawn
edges are outcome alternatives; the gate rejects the shape with a teaching error), the
`suspend` node cannot be an exception handler (`exception=suspend` rejected —
checkpoint-on-failure would smuggle in retry semantics), and a jump-only `suspend` node
is anchored behind an island (`root → island → suspend`) to satisfy the no-orphan export
rule — an island's outgoing edges are never traversed. The persisted record contract
(`{cid, node, ttl, model, seen, run}`) and the store put/get contract are **unchanged**.
This mirrors the Java reference engine's ADR-0012 in lock-step; both engines carry the
identical grammar, gate rules and teaching errors.

**Rationale.** Two independent field teams hit the same conceptual wall within days: a
suspensible node ignores IF-THEN-ELSE routing and suspends unconditionally, which reads
as an inconsistency between decision-making and suspension — documenting the
decide-before-you-suspend rule (the first team's remedy) did not stop the second report,
so the constraint itself was the defect. The reference engine's code study showed the
property was only a walker-level routing trigger — persistence was already
predecessor-agnostic and resume already continued along the persisted node's forward
links — and it was papering over two latent divergences (documented plain-edge
suspension vs actual fan-out; a half-working jump whose resume fanned out a decision's
alternatives). Shape discrimination was chosen over skill-class discrimination because it
is cheaper — one forward-link probe at resume, no statement inspection at the gate — and
because the classes coincide **by construction**: working skills always return `next` so
they can only be edge mode, and only routing skills can jump, so re-execution can never
touch a working node. Back-compat is structural: every valid pre-change model necessarily
drew its checkpoint edge (the prior gate required it), so edge-inferred redirect
reproduces the old behavior exactly and pre-change store records replay correctly under
the new engine — mixed-version fleets are safe in both directions.

---

## ADR-0010 — CompileGraph is the mandatory deployment gate for graph models (CompileFlows parity) {#adr-0010}
**Status:** Accepted · **Date:** 2026-07-30T01:52:00.000Z · **Serves:** vision-mercury · **Formalizes:** compilegraph-mandatory-gate-rust
<!-- id: adr-0010 | status: accepted | formalizes: compilegraph-mandatory-gate-rust -->
<!-- accepted via the PR #186 merge (d2791b09), 2026-07-30 - the ledger-gate precedent -->

*The twin of the Java engine's ADR-0011 — the decision was made in the reference
repository and this port adopts it in lock-step; the Rust realization notes are inline.*

**Abstract.** A deployed graph model is executable at `POST /api/graph/{graph-id}` **only**
when it is listed in the graph manifest (`graph.model.automation`) **and** passes the
CompileGraph quality gate at startup — a graph that fails the gate, or is not listed,
answers **HTTP-404 as if the model does not exist**, and the lazy, per-request loading of
deployed models is removed. Like `flows.yaml`, the manifest carries the location of its
own models (an optional `location` entry, default `classpath:/graph`) — there is no
separate application property. Validation follows **two explicit lanes**: *production* =
models → CompileGraph (`compiler::compile_graphs`) → deployed graphs → the graph executor,
which **trusts the gate** and drops per-request re-validation of gate-guaranteed rules,
keeping only data-driven runtime guards (store-record contents, dynamic jump targets, loop
detection); *dry-run* = drafts in the temp workspace → UI CLI input validation at node
create/update → the graph traveler with full runtime validation. The gate's whole-graph
rules live in a reusable `model_validator`, which the playground's `run` command also
invokes as a **pre-run quality check** — draft authoring deliberately allows partial
models, but the moment the author asks to run, the contract must hold.

**Rationale.** This is the `CompileFlows` precedent applied to Layer 3: an invalid flow
never becomes executable, and the graph engine now gives the same guarantee — previously a
manifest graph that *failed* validation could still be resurrected by the lazy-load
fallback and executed unvalidated, which is untenable for field production. Compiled-or-404
(identical for failed and unlisted models) leaks nothing about why a model is absent, and
turning the deploy folder into a pure data directory removes it as a direct execution
vector. Startup-time rejection converts an entire class of runtime stalls and mid-run
errors (missing `end` node, checkpoint without a continuation edge, dead-end suspend node)
into immediate, logged deployment failures — while the same rules surface to graph authors
at dry-run `run` time, so the deployment contract is learned in the playground, not
discovered in the field. The consequences are accepted deliberately: the manifest is now a
**requirement** (a one-line migration for installations that relied on lazy loading, with
the `classpath:/graph` default preserving existing layouts and an obsolete-key warning for
the retired `location.graph.deployed` property), hot-dropping a JSON file into the deploy
folder no longer works (deployment is an explicit, restart-scoped act — consistent with
the governance lifecycle the Vision calls for), and the walkers' suspend/resume guards are
now exercised end-to-end only on the dry-run lane (the static validator carries the
per-rule coverage). Rust realization note: the port surfaced a corollary worth keeping —
compiled-or-404 makes the manifest **deployment intent**, so every graph a runtime test
executes must be listed in the test manifest, and the playground example app carries its
own `graphs.yaml` (it previously ran tutorials purely through the now-deleted lazy path).

---

## ADR-0009 — Graph workflow suspension: short runs + an external state store, encapsulated in skills {#adr-0009}
**Status:** Accepted (amended by ADR-0011: the `suspend=true` declaration vocabulary is retired in favor of edge/jump modes; short runs, the store contract and the record envelope stand) · **Date:** 2026-07-30T01:52:00.000Z · **Serves:** vision-mercury · **Formalizes:** graph-suspend-resume-rust
<!-- id: adr-0009 | status: accepted | amended-by: adr-0011 | formalizes: graph-suspend-resume-rust -->
<!-- accepted via the PR #186 merge (d2791b09), 2026-07-30 - the ledger-gate precedent -->

*The twin of the Java engine's ADR-0010 — the decision was made in the reference
repository and this port adopts it in lock-step; the Rust realization notes are inline.*

**Abstract.** A long-running business process with human checkpoints (approval,
intervention, inbox notification) is expressed as a **sequence of short graph runs**: at a
suspension point the run persists its workflow state — the `model` namespace plus
traversal bookkeeping — to an **external state store** keyed by the business correlation
ID with a designer-chosen TTL, then completes normally; a later request with the same
correlation ID restores that state and continues past the checkpoint without re-executing
it. The mechanics are **encapsulated in two skills** — `graph.suspend` and `graph.resume`,
supersets of `graph.task` that invoke a pluggable store function named by the node's
`task` property with a fixed put/get contract — so suspension nodes carry **no data
mapping**. The node alias `suspend` is **reserved** (the `root`/`end` pattern): traversal
routes to it by name when a node marked with the reserved property `suspend=true`
completes; node *types* (`Suspend`/`Resume`/`Suspensible`) remain visual convention —
**the skill defines behavior**. Store retrieval **consumes the record atomically**
(at-most-once resume); reserved model keys never persist — nor restore, so a forged store
record cannot overwrite the current run's identity; a suspension point must be the sole
active branch.

**Rationale.** Parking a live graph instance for a multi-day approval would pin memory,
defeat the flow ttl, and not survive a restart — the short-run model keeps the engine's
in-memory instance lifecycle untouched and makes cross-instance resume free (any pod
sharing the store can continue the workflow). Skill encapsulation was chosen over
node-level data mapping because the mapping variant required special-casing the mapping
grammar per node type and left the resume jump-target with no channel; a fixed store
contract also makes the persistence seam documentable and replaceable (Redis ships as the
optional `extensions/minigraph-state-redis` crate — never an engine dependency; engine
tests use a temp-file store). The reserved-alias routing reuses the existing jump-by-name
directive vocabulary instead of introducing edge classification, at the accepted cost of
one suspend node per graph. Consume-on-retrieve was preferred over keep-until-TTL so a
duplicate resume cannot double-execute a continuation; workflows needing stronger crash
guarantees may implement keep-until-ack semantics in a custom store. Alternatives
rejected: engine-managed timers or parked instances (memory + restart fragility); reusing
the Event Script `ext:` fire-and-forget external-state contract (durability requires a
synchronous acknowledgement); persisting `{node}.result` scratch (the model is the
workflow's single durable memory — an explicit, teachable rule). Rust realization notes:
Java's Mono-wrapped eager store request (issued on the worker thread for the thread-keyed
trace context) maps to a plain `await` — task-scoped trace context yields the same
observable store-call-under-skill-span topology with no workaround; and porting the
walkers surfaced a genuine two-lock seen-check race the Java atomicity fix also covered,
now a single atomic insert-if-absent. This decision **supersedes** the
knowledge-graph port design record's "session persistence across restarts is out of
scope" default for *workflow* state (Playground UI sessions remain in-memory).

---

## ADR-0008 — Registration metadata is a cross-language contract; carriers are per-language idioms {#adr-0008}
**Status:** Accepted · **Date:** 2026-07-26T01:38:18.000Z · **Serves:** vision-mercury · **Formalizes:** registration-metadata-contract
<!-- id: adr-0008 | status: accepted | formalizes: registration-metadata-contract -->

**Abstract.** Declarative registration — `#[preload]` and its family (entry points,
websocket services, Event Script plugins, graph fetch features) — is governed by **one
canonical metadata model with fixed semantics**, specified in
`docs/guides/registration-metadata-contract.md` and proven by **golden vectors shared
verbatim** between engine repositories. How each language *carries* the metadata is an
idiom — Java annotations discovered by runtime classpath scan, Rust attribute macros
collected by link-time inventory, Python/Node decorators discovered by explicit
package/module walks — but the model and its semantics are the contract: attach at
definition / resolve at boot (`env_instances`); the optional-service condition grammar;
order-free marker stacking; one conflict policy (explicit wins over declarative;
duplicates WARN + last-wins); extension-point naming (an explicit positional name, or
derivation from the declaration such that idiomatic declarations in every language yield
the same registered name); plugins are Event Script capabilities (flow vocabulary) and are
never conditionally gated, while features honor gating; the boot sequence
(discover → register → override → resolve → validate → route table); explicit
loud-failure discovery; and misuse as a first-class, tested error surface.

**Rationale.** This port's first annotation pass proved that porting the *mechanism*
without fixing the *semantics* produces drift invisible to any single repository: built-ins
bypassing the extension points they exemplify, conflict policies diverging (skip-first-wins
vs last-wins), gating support absent where the reference has it, and an attribute
stack-order requirement Java never had. Each was individually small; together they meant a
developer — or an AI agent — could not transfer knowledge between engines, and every future
port would re-diverge independently. The same problem was already solved once for the wire
format (spec + golden vectors, v4.10.0): fixing the contract in a language-neutral artifact
with executable conformance is what made the four-way interop matrix provable. This ADR
applies that method to the declaration surface. The maintainer's two governing directives
are part of the decision: developers must see **consistent, decoupled** registration in
every language, and this Rust port is the **best-practice template** for the Python and
Node ports.

**Alternatives.** (a) *Per-port judgment calls documented in each repo* — rejected: that is
the drift this ADR eliminates; N-of-1 documentation cannot be conformance-tested.
(b) *A shared runtime registry service* (as an external blueprint's open item suggested for
multi-process parity) — rejected: registration is process-local by design in a
self-contained composable application; cross-process discovery is the service mesh's
concern and stays opt-in (out of scope for this port — ADR-0006). (c) *Exporting the full
live registry for byte comparison* — rejected in favor of a fixed fixture set: engines
legitimately differ in framework built-ins (no Spring in Rust, no Kafka mesh), so
whole-registry comparison would pin incidental surface, not contract.

**Consequences.** New ports implement the carrier idiomatically, then pass the three
golden-vector suites (`registration-vectors/core.json`, `plugin.json`, `feature.json`)
before their declaration surface is considered done; every capability field a port cannot
honor is documented as N/A where developers would meet it, never silently dropped. The
engines accept a small ongoing cost: vector files are maintained verbatim in every
repository, and semantic changes to registration must update the contract page, the
vectors, and all engines in lock-step — which is precisely the point.

> **Cross-ledger note.** This entry is the twin of the Java ledger's **ADR-0009** (the
> reference repository allocated 0008 to its companion-sync decision, which has no Rust
> counterpart); the numbering differs, the decision is one.

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
