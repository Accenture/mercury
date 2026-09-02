# Changelog

## Release notes

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The full increment-by-increment record lives in [`docs/INCREMENTS.md`](docs/INCREMENTS.md);
the design rationale in [`draft-design-specs/`](draft-design-specs/).

---
## Unreleased

### Added

- New `json` simple plugin (lock-step with the Java engine) - `f:json(text([])) ->
  my_empty_list` parses JSON text (a constant or a model variable) into a live map or
  list in one data-mapping statement:
  `f:json(text({"hello": [1, 2, {"nested": "demo"}]})) -> my_nested_dataset`. Closes the
  field feedback gap where creating a simple dataset required a composable function.
  Engine difference documented: this parser is strict (no unquoted keys or trailing
  commas) while the Java engine's is lenient - portable flows use strict JSON.

### Breaking

- The fire-and-forget AI companion endpoint `POST /api/companion/{id}` is RETIRED
  (lock-step with the Java engine, its ADR-0021) - the bare URL now answers 404. Use the
  synchronous `POST /api/companion/{id}/sync`, unchanged, which returns the command
  outcome in-band and tees output to the session console. Driven by a field AI-agent
  exercise: the async form hid errors and forced sleep-padded drivers.

### Documentation

- AI-grammar feedback round: the companion guides document /sync as the only endpoint;
  "restart ends the session - export first" and "404 means the session is gone" warnings.
  (graph.js needed no action here - this port never carried it and its guides already
  say so; the Java engine formally deprecated it, ADR-0022.)

---
## Version 4.12.1, 9/1/2026

The first public-registry release: the seven library crates publish to crates.io
under mercury-prefixed package names, with lib (code-facing) names unchanged.
Functionally a small follow-up to v4.12.0 (reply-lane rotation + the route pool
registration API shipped after the tag).

### Added

- Route pool registration API `register_route_pool` / `release_route_pool` - a set of
  private singleton routes `{prefix}.{n}` (strict FIFO lanes sharing one stateless
  function) with registry-level identity, symmetric release, reload semantics and
  range-checked integrity warnings for individual member updates. The streaming
  reply-lane pool now registers through it (lock-step with the Java engine's
  `registerRoutePool`, Java ADR-0020 / this repo's ADR-0017).

### Changed

- The public constant `ASYNC_HTTP_RESPONSE_STREAM_PREFIX` (dotted lane-name prefix) is
  replaced by `ASYNC_HTTP_RESPONSE_STREAM_POOL` (the un-dotted pool base) - lane route
  names on the wire and in telemetry are unchanged.
- Streaming reply-lane checkout now rotates through the pool (FIFO round-robin)
  instead of re-picking the most recently released lane (LIFO): the first stream
  takes `async.http.response.stream.0`, the next `.1`, and so on, with a released
  lane rejoining at the tail - predictable lane selection in telemetry and maximal
  rest before a lane is reused (lock-step with the Java engine).
- Library package names are mercury-prefixed for crates.io publication:
  `mercury-platform-core`, `mercury-platform-macros`, `mercury-event-script`,
  `mercury-event-script-macros`, `mercury-knowledge-graph`,
  `mercury-knowledge-graph-macros`, `mercury-minigraph-state-redis`. The lib
  (code-facing) names are unchanged, so source code and `use` paths are unaffected;
  an application consuming these crates by path updates only its Cargo.toml
  dependency keys. Application/example members are fenced with `publish = false`,
  and the workspace `repository` metadata now points at this repository.

---
## Version 4.12.0, 8/30/2026

The progressive-rendering milestone: token/event streaming end to end - HTTP edge,
HTTP client, and Event-over-HTTP peers - with full OpenTelemetry lineage and
business-correlation continuity across all four runtimes (Java and Rust engines,
Python and Node.js language packs at the same version). Useful on its own, and
the foundation for the AI SDLC (agent, MCP and tool adapters as wrapper-side
functions with complete observability).

### Added

1. **HTTP response streaming** - a function can stream an HTTP response progressively
   (token segments, progress events) by sending a sequence of events to the caller's
   reply route until an end-of-transmission signal, using the reserved envelope header
   `x-event-stream: data | eof | exception` (internal protocol - the wire carries only
   standard HTTP). Server-Sent Events framing for `text/event-stream` (typed events via
   `x-event-name`, terminal `done` event with trailing metadata, in-band `error` events,
   configurable keep-alive pings via `event.stream.keep.alive`); chunked transfer with
   JSON Lines for other content types. Declare a streaming endpoint with `stream: true`
   in rest.yaml - the request checks out a dedicated ordered reply lane
   (`async.http.response.stream.{n}`) from a pool of 500 for its lifetime: per-request
   strict FIFO with cross-request concurrency, and HTTP-503 back-pressure when the pool
   is exhausted. New `EventStreamWriter` producer API; idle timeouts fail streams
   in-band; client disconnects drop late segments as no-ops. A runnable demo ships in
   examples/hello-world (`GET /api/hello/sse` plus the `scripts/sse-client.mjs`
   progressive-rendering client). Engine-identical with the Java engine's feature
   (Java PR #299). See the HTTP Response Streaming guide (ADR-0015).

2. **Progressive SSE consumption in the HTTP client** - `async.http.request` can
   consume a `text/event-stream` response progressively (an LLM provider's token
   stream, another engine's streaming endpoint, any SSE API), relaying one
   `x-event-stream: data` envelope per upstream event to the caller's reply route,
   then `eof` - the same streaming protocol the HTTP edge consumes, which makes an
   SSE-to-SSE relay a pure configuration exercise. Activation is explicit: the
   request must declare `Accept: text/event-stream`, the response must be SSE, and
   the request must carry a reply route; anything else keeps the buffered single-shot
   behavior. For streams, the request timeout acts as the idle allowance between
   reads (keep-alive comments reset it); idle expiry and mid-stream disconnects fail
   the stream in-band. Engine-identical with the Java engine's feature
   (Java PR #300). See the HTTP Response Streaming guide (ADR-0016).

3. **Event-over-HTTP peer streaming (envelope mode)** - a remote streaming function
   reached through `/api/event` can stream its segments back to the caller's reply
   route on the same HTTP call. The caller opts in per event: a send with a
   `reply_to` and the `accept: text/event-stream` event header; the `x-ttl` event
   header (ms, default 60s) is the idle allowance between stream events on both
   hops. On the wire, the peer answers with SSE in a hybrid dialect - control
   signals (the head, the eof/exception terminals, and any segment that cannot
   round-trip as plain text) ride base64-encoded serialized envelopes under the
   reserved SSE event name `envelope`, while text segments ride raw frames - so
   segment types and terminal metadata survive the hop exactly, and token relays
   stay near-zero overhead. Compatibility is explicit: a non-streaming target
   answers byte-identical to the classic callback reply; a streaming function
   invoked without the opt-in (RPC, or no accept header) receives
   `406 Streaming function requires a caller that accepts text/event-stream`;
   server-side lane exhaustion answers the same 503 back-pressure as a local
   streaming endpoint (plain RPC traffic never consumes a lane). The consuming
   client guards the dialect (a missing envelope head or a transport end without a
   terminal fail in-band), and every in-band exception body now carries the
   standard error key-values (type/status/message). Engine-identical with the Java
   engine's feature (Java PR #301). See the HTTP Response Streaming guide (ADR-0016).

### Changed

1. The `/info/routes` actuator endpoint renders pool-style route families compactly:
   routes differing only by a trailing numeric suffix, with uniform instances and
   contiguous numbering, collapse into one display entry (e.g. the 500 streaming reply
   lanes appear as `"async.http.response.stream.0 - 499": 1`), and the rendered routing
   view is cached for 10 minutes. Display-only - the routing table is unchanged.
   (Java engine parity.)

2. Polyglot Functions documentation chapter (`guides/polyglot-functions.md`) - writing
   composable functions in Python and Node.js with the official Event-over-HTTP wrappers
   (docs: accenture.github.io/mercury-python and accenture.github.io/mercury-nodejs),
   with the zero-code demo extended to the wrapper demo apps. ADR-0014 (Proposed) records
   the peers-not-subprocesses design; the interop test report gains the polyglot wrapper
   round (2026-08-22 acceptance drives).

### Changed

1. Documentation home page unified on the Java reference presentation (the layered-ascent
   table) and the docs site renamed "Composable for Rust" for family consistency with
   Composable for Java / Python / Node.js; engine and wrapper documentation now
   cross-link. The navigation gains the Java site's "Operate & integrate" tab
   (Observability, Event over HTTP, Polyglot Functions) and an Orientation tab, so the
   tab row reads identically across the two engines' documentation sites.

4. **Engine-to-wrapper streaming demo and interop test report.** The hello-world
   example gains `GET /api/hello/remote` (`hello.remote.relay`): a streaming endpoint
   whose function forwards its caller's reply lane into an event-over-http mapped
   remote streaming function (the Python/Node.js demo apps' `hello.tokens`),
   rendering a remote peer's tokens progressively out this application's HTTP edge
   with zero imperative streaming code; a sample `resources/event-over-http.yaml`
   ships with the example (auto-loaded). `hello.sse` is now public, so wrapper
   clients can consume it over `/api/event`. A permanent interop test report
   (`docs/test-reports/progressive-rendering-interop.md`, packaged with the AI
   contract provider) records the live ten-combination cross-runtime matrix, the
   OpenTelemetry span-lineage and business-correlation-id verification, and a
   captured end-to-end telemetry + application-log-context example.

---
## Version 4.11.11, 8/23/2026

### Added

1. `graph.task` accepts a task route declared as a declarative Event-over-HTTP target
   (`yaml.event.over.http`) - a knowledge graph can invoke remote engine instances and
   polyglot (python/node.js) function hosts as graph tasks. Lock-step with the Java
   engine (which also adds a `PostOffice.getEventHttpTarget(route)` API).

---
## Version 4.11.10, 8/21/2026

### Added

1. **Event Script `f:setConfig` plugin — set or override a configuration parameter at
   run-time (lock-step with the Java engine).** The new built-in simple plugin stores a
   key-value pair in the process override registry (this port's analog of a Java system
   property), which takes precedence over the base configuration on every subsequent read.
   Typical use: a start-up flow hydrates secrets from a cloud secret manager before dependent
   components consume them. The key must be a non-empty string; the value can be any object
   and is converted to text. Invalid input returns false without side effect.

2. **`system/ai-contract-provider` — a standalone composable app serving Mercury's
   version-matched operational contract for AI discovery (lock-step with the Java engine's
   `system/ai-contract-provider`).** Read-only REST endpoints on port 8999
   (`/api/discovery`, `/api/contracts`, `/api/contracts/{id}`, `/api/skill`,
   `/api/references?path=...`, `/api/manifest`), each wired `rest.yaml` →
   `http.flow.adapter` → Event Script flow → function — the flow YAML files are
   byte-identical to the Java app's. `--export <directory>` writes the offline
   `mercury-platform` Agent Skill (SKILL.md, the packaged documentation closure including
   `references/llms.txt`, the installed contract inventory, per-file SHA-256 manifest
   written last as the completion marker); two exports of the same build are byte-identical,
   and the exporter never overwrites an existing snapshot. Contract behavior anchors are
   fully-qualified Rust paths resolved at compile time by the anchor test, and the snapshot
   is embedded at build time from `files.list`, so neither the catalog nor the packaged
   inventory can drift from the tree. The repo also gains `system/AGENTS.md` (the consumer
   starting point, same path convention as the Java repo) and the root `AGENTS.md`
   contributor/consumer fork.

---
## Version 4.11.9, 8/11/2026

### Changed

1. **A dry-run graph whose root node has no `name` is scoped as `untitled` instead of being
   rejected.** v4.11.8 required a suspend/resume model to carry a root `name` before the playground
   would instantiate it, because the fallback identity was a per-instantiation handle that no later
   run could read. What the state store actually needs is an identity that is *stable across
   instantiations*, not one derived from the model name — so an unnamed draft now falls back to the
   stable constant `untitled` and simply works: it suspends and resumes like any named model,
   sharing the `graph:untitled:{cid}` scope. Naming the root node (which the export path does
   automatically, keeping it in sync with the graph's file/deployment id) still gives a draft its
   own scope and remains the recommendation for anything you intend to deploy.

---
## Version 4.11.8, 8/10/2026

> Version 4.11.7 was a Java-only release (the Kafka flow adapter's `group.protocol=auto`),
> so this engine steps from 4.11.6 directly to 4.11.8 to stay in version lock-step.

### Fixed

1. **Dry-run suspend/resume: a suspended workflow now resumes across `instantiate graph`
   commands.** The graph-scoped store contract introduced in v4.11.6 keys records by
   `graph:{graph_id}:{cid}`, but the playground dry-run lane stamped each instantiation with an
   ephemeral `playground-{uuid}` handle — so a suspension was persisted under a key no later
   instantiation could ever read, and every resume silently restarted fresh (surfaced by a
   tutorial-14 regression drive: the manager-approval step rejected with "Transaction not
   found"). The dry-run instance now carries the model's stable identity — the root node's
   `name` property, which the export path keeps in sync with the graph's file/deployment id —
   matching the production executor's scope, so a dry-run suspension is resumable in a later
   instantiation. A suspend/resume model whose root has no name is REJECTED at instantiation
   with a teaching message (a silent ephemeral fallback would break the resume mechanism
   invisibly); a nameless draft without suspension keeps a unique playground handle. The
   production executor lane (`POST /api/graph/{graph-id}`) was never affected.

---
## Version 4.11.6, 8/10/2026

### Changed

1. **BREAKING: the suspend/resume state-store contract is scoped by graph + cid.** The
   persistence envelope gains a `graph` field (`{cid, graph, node, ttl, model, seen,
   run}`), the retrieve body becomes `{cid, graph}`, and the Redis store keys records
   `graph:{graph_id}:{cid}` (formerly `graph:state:{cid}`) — so the same business
   correlation ID suspends independently in each domain's graph and in each subgraph, and
   a resume only ever sees records written by its own graph. **Upgrade note:** records
   persisted under the old key are not visible after the upgrade — a resume behaves as a
   fresh transaction (the same observable behavior as an expired record). Let suspended
   workflows complete before upgrading, or accept the fresh start; custom store
   implementations must adopt the `graph` field (the shipped stores reject a request
   without it). (ADR-0012)

### Added

1. **Generic exception context for `exception=` handlers — one handler can serve every
   node.** When a failed node routes to its exception handler, the engine now stages
   `error.source` (the failing node's alias), `error.code`, `error.message`, and
   `error.stack` (when the failure carries one — this engine has no native stack-trace
   transport, so the key appears on cross-engine records only) in the state machine —
   the same `error.code/message/stack` vocabulary as Event Script's flow exception
   handlers, plus `source` — so one island-anchored handler node serves the whole graph
   without naming any failing node in its data mapping. `graph.api.fetcher`,
   `graph.task` (including `task=async.http.request`), `graph.extension` and the
   suspend/resume store calls all route through the same staging. The per-node record
   (`{node}.status`/`{node}.error`) is unchanged, so existing handlers keep working.
   The `error` namespace is inspectable in a dry-run session via `inspect error` — the
   node alias `error` has always been reserved by the graph model. The context also
   **reports recovery**: when the failing `error.source` is later retried successfully,
   `error.code` becomes 200 with the source kept and the failure details removed (the
   source match keeps parallel branches safe) — empty means nothing failed,
   `{source, code: 200}` means recovered, a full context means an outstanding failure.
   (ADR-0013)

2. **The orchestrator pattern: a parent graph delegates independently resumable subgraph
   paths.** `graph.extension` now stamps the caller's business correlation ID
   (`model.cid`) on the delegated call — both graph and `flow://` targets — exactly
   as an Event Script sub-flow launch already did. A delegated subgraph that suspends
   persists its record under the shared business correlation ID scoped by its own graph
   ID, so re-invoking the parent (or the subgraph directly) with the same correlation ID
   resumes it. Documented in the workflow-suspension guide with the reference model pair
   `unit-test-orchestrator` / `unit-test-sub-suspend`. (ADR-0012)

3. **Dynamic variables now resolve in every statement command — completing the generic
   error handler.** `NEXT:` and `THEN:`/`ELSE:` jump targets, `RESET:` list entries and
   `DELAY:` values in `graph.math` statements resolve `{namespace.key}` references at
   execution time, so a generic handler can retry *whichever node routed to it*:
   `RESET: {error.source}, error-handler` + `NEXT: {error.source}` (and pace with a
   computed `DELAY: {model.backoff}`). Previously only `MAPPING`/`COMPUTE` expressions and
   `IF` conditions substituted. An unresolved variable renders `null`: a `RESET:` entry is
   a safe no-op, a `DELAY:` is skipped, and a jump target fails the run loudly.
   tutorial-12's error-handler and clear-exception nodes now teach the generic idiom.

---
## Version 4.11.5, 8/9/2026

### Added

1. **The async HTTP client sends a default `Accept: */*` when the caller gives none**
   (maintainer ruling — best practice and Java parity: the Java engine's reactor-netty
   client does this implicitly). The REST edge negotiates the response content-type from
   the Accept header, so without the default the same request decoded JSON on the Java
   engine and returned raw bytes here. An explicit accept is never overridden; pinned by
   a wire-echo test for both cases.

### Fixed

1. **`graph.task` input mapping now supports `model.{key}` staging targets, restoring Event
   Script parity for dynamic variables** (lock-step mirror of the Java reference engine's
   PR #267). An `input[]` entry whose RHS starts with `model.` stages a variable in the
   graph's state machine — instead of silently landing in the request body under a literal
   `model` key — so a later entry can reference it as a dynamic variable, e.g.
   `input.body.person_id -> model.person_id` followed by
   `text(/api/mdm/profile/{model.person_id}) -> url`. This matches the `graph.api.fetcher`
   and `graph.extension` input mappings, which already supported it. Engine-managed model
   metadata remains immutable: the CompileGraph gate and the playground pre-run check reject
   a reserved target at validation time, and the new runtime path calls the same shared
   guard.

### Changed

1. **The suspend/resume documentation now teaches two suspension patterns — the checkpoint
   node and the decision node — named after the node that pauses.** The workflow-suspension
   guide opens with the two situations the feature serves — waiting for another system (a
   long batch job that calls back on completion, fully automatic) and waiting for a person
   (an approval, missing information requested by email) — and describes each pattern in
   plain language with a small diagram: a checkpoint does its work and pauses (never
   re-executed on resume; a decision may sit in front of it when the pause is conditional),
   while a decision chooses — continue, branch, or pause — and re-decides against the new
   input on every resume. The former *edge mode* / *jump mode* names remain in the ADRs and
   are noted once as engineering aliases. tutorial-14 help, the `graph.suspend` skill help
   and the AI grammar (`minigraph-commands.json`) tell the same story; behavior is
   unchanged.

2. **tutorial-13 now invokes the platform's AsyncHttpClient (`async.http.request`) instead of
   a throwaway demo function.** The `graph.task` tutorial becomes a worked example of an HTTP
   client by configuration: the input mapping stages `model.person_id` from the request body,
   resolves it as a dynamic variable inside the `url`, uses `${rest.server.port:8080}`
   environment/config substitution in the `host` — resolved when the model is loaded (at
   deployment compile and at `instantiate graph` for a dry-run) while the authored and
   exported model keeps the placeholder — and sets the HTTP timeout explicitly with
   `text(5000) -> headers.x-ttl` (milliseconds): the graph's ttl bounds only the event call
   to the function, so the `X-TTL` request header is the sanctioned way to give the HTTP
   operation its own budget, and it propagates the deadline on the wire. The tutorial also
   declares `headers.accept` explicitly as best practice — the probe that surfaced the
   client-default divergence resolved by the new default Accept above. The `v1.hello.task`
   demo mock is removed; the tutorial help, `graph.task` skill help, skills reference, the
   AI grammar (`minigraph-commands.json`) and the HTTP-client guide are updated to teach the
   staging, substitution and X-TTL idioms.

---
## Version 4.11.4, 8/7/2026

> Versions 4.11.2 and 4.11.3 were Java-only releases (the Kafka client/dependency
> surface this port does not carry); this port's version tracks the Java line.

### Changed

1. **Suspension is a destination: edge-mode checkpoints and jump-mode decisions replace the
   `suspend=true` property (ADR-0011; mirrors the Java reference engine's ADR-0012 in
   lock-step).** A suspension point is now declared by **how a node reaches the reserved
   `suspend` node**, discriminated purely by graph shape. **Edge mode** — a working node with a
   drawn edge to `suspend` redirects there when its skill completes; the edge is the
   declaration, a continuation edge is mandatory (compiler-enforced, shape-only — no statement
   inspection), and a resumed run continues along it without re-executing the node: byte-for-
   byte the prior suspensible behavior, so **every valid earlier model deploys and behaves
   identically unmodified** (the prior gate already required the checkpoint edge). **Jump
   mode** — a `graph.math` decision returns `suspend` from its IF-THEN-ELSE and is
   **re-executed against the new request input on every resume**, so it re-decides each time:
   "keep waiting on an invalid decision" is now one jump with no auxiliary wait nodes and no
   `RESET` bookkeeping (this replaces the wait-loop mechanics described in the entry below;
   tutorial-14 is remodeled accordingly and its `await-decision` node is gone). The retired
   `suspend=true` property is accepted and **ignored** for one deprecation window with a
   compiler WARN. New compiler rules teach the grammar: a routing-skill node must not draw an
   edge to `suspend` (a decision jumps instead), and the `suspend` node cannot be an exception
   handler (`exception=suspend` rejected). A jump-only `suspend` node is anchored behind an
   island (`root → island → suspend`) to satisfy the no-orphan rule — an island's outgoing
   edges are never traversed. Also fixes a latent divergence: the documentation always said a
   plain edge into `suspend` is a suspension point, but the walker previously fanned out
   (checkpoint and continuation in parallel); the walker now honors the documented contract.
   The persisted record contract and the store put/get contract are unchanged — records
   written by earlier releases resume correctly. The graph model, gate rules, teaching errors
   and AI-catalog suspend entries are identical to the Java engine's; covered by engine tests
   for both modes, the deprecation-compat shapes, the new gate rules and a jump-mode wait loop
   across consecutive suspensions.

2. **tutorial-14's manager approval step is now a real decision with three outcomes** (mirrored
   from the Java reference engine, same-day). The purchase workflow's store manager can approve —
   the graph suspends for the delivery department as before — or **reject with a reason**: a
   `graph.math` decision at the manager's resumption point routes an explicit rejection to a
   terminal node that reports the reason together with the original order, and the workflow ends
   (the record was consumed on resume, so a later request under the same correlation ID is a
   fresh 404). **Anything else — a missing or unrecognized decision — re-suspends** through a
   wait node whose continuation loops back to the decision, so an invalid request (or a replay
   against a leftover record) can never end a long-running workflow by accident; the loop uses
   `RESET` to clear both loop nodes' seen marks on every pass, since seen marks survive
   suspension and a seen node never re-executes. The graph model is byte-identical to the Java
   engine's; the end-to-end suite covers all three outcomes including loop stability across two
   suspensions; the tutorial help and the workflow-suspension guide walk every path. The
   decide-before-you-suspend rule, the suspensible node's capability envelope, and the wait-loop
   RESET pattern are stated everywhere an author learns the grammar — the guide's design rules,
   the tutorial help, the AI grammar (`minigraph-commands.json` and the `graph.suspend` skill
   help) — and the validator/runtime error for `suspend=true` on a routing skill now explains
   the why and the fix instead of only the restriction (same wording as the Java engine).

---
## Version 4.11.1, 8/1/2026

### Added

1. **Task-level TTL override for catchable sub-flow timeouts.** A new optional `ttl` on
   a sub-flow task (`process: 'flow://...'`, duration syntax, must be less than
   `flow.ttl`, compile-validated with whole-flow rejection) overrides the default TTL
   propagation at the invocation site: a SHORTER child deadline makes the sub-flow time
   out FIRST, so its 408 is catchable by the task-level or flow-level exception handler —
   enabling budgeted retries within the parent's remaining budget. A runtime WARN flags a
   task ttl (plus any delay) that is not less than the effective flow ttl. Lock-step twin
   of the Java v4.11.1 feature.

### Fixed

1. **The `delay` task parameter now defers a sub-flow launch (previously a silent
   no-op).** `delay` was accepted with full validation on a `flow://` task but discarded
   at dispatch — the sub-flow always launched immediately. It now defers the launch
   exactly as it defers a function task; the child's TTL timer starts on delivery, and a
   pending deferred dispatch (sub-flow or function) is cancelled at flow teardown so it
   cannot fire after the flow has ended. *Migration note: a flow that carried a
   (hitherto ignored) `delay` on a sub-flow task will now actually delay.*

2. **Graph state store works on Redis servers older than 6.2 (field report).** The
   suspend/resume Redis store consumed records with `GETDEL`, a Redis 6.2+ command —
   unavailable on older managed enterprise servers and on the community Windows binary
   (5.0.14) bundled by the Java repo's `redis-standalone` helper, where a resume failed
   with `ERR unknown command GETDEL`. The consume strategy is now **version-aware**: the
   store reads the server version from `INFO server` once per connection (stated in the
   startup log) and uses native `GETDEL` on 6.2+ or an equally atomic `MULTI/EXEC`
   `GET`+`DEL` transaction on older servers — the at-most-once resume guarantee holds on
   both paths, and an undetectable version selects the transactional fallback, which
   works everywhere. Lock-step twin of the Java engine's v4.11.1 fix.

---
## Version 4.11.0, 7/30/2026

### Added

1. **`ManagedCache`** — the Java `org.platformlambda.core.util.ManagedCache` ported
   (`platform_core::ManagedCache` / `CacheValue`; design record
   `draft-design-specs/managed-cache-port.md`, maintainer-gated): a named, self-expiring
   (expire-after-write, 1 s floor), size-bounded (default 2000) in-memory cache with a
   process-wide registry and a lifecycle-wired 10-minute housekeeper. Engine: moka
   (Caffeine's Rust lineage), wrapped as an internal detail and built with
   **deterministic LRU eviction** (`EvictionPolicy::lru`, maintainer ruling) — a
   documented divergence from Java Caffeine's approximate W-TinyLFU (which also carries
   deliberate HashDoS randomness; a refactoring note was handed to the Java team).
   Java's `SimpleCache` is deliberately NOT ported: one cache type — any Java
   `SimpleCache` site maps onto a `ManagedCache` instance.
2. **`/health` per-dependency info cache** (Java parity, previously deferred): the
   `type=info` lookup is now cached 5 s per dependency (`health.info`, map bodies only —
   Java `isServiceUnhealthy` semantics); `type=health` still runs on every probe and the
   `/health` result itself is never cached.

3. **Workflow suspension for the Active Knowledge Graph (suspend/resume), in lock-step
   with the Java engine.** A graph run can suspend at a human checkpoint — approval,
   intervention, inbox notification — and resume later with the same business
   correlation ID, without re-executing completed steps. A long-running business process
   becomes a sequence of short runs; nothing stays in memory between them.
   - New skills **`graph.suspend`** / **`graph.resume`** (supersets of `graph.task`: the
     `task` property names a pluggable state-store function; envelope assembly and
     restoration are fully encapsulated — no node data mapping). The node alias
     **`suspend`** is reserved (the `root`/`end` pattern — traversal jumps to it by
     name); a skilled node marked with the new reserved property **`suspend=true`**
     routes there after executing. `graph.resume` records the run condition in
     **`model.run`** (`resume` | `fresh`) so the graph's own logic decides how a
     fresh-or-expired request is handled — the engine deliberately does not distinguish
     absent from expired. `model.run` joins the read-only flow-metadata family
     (`model.cid`/`instance`/`flow`/`ttl`/`trace`): the flow compiler and the runtime
     dynamic-target guard reject any data mapping that overwrites it. The suspend node's
     **`ttl`** is mandatory with no default (duration syntax, e.g. `2d`).
   - Traversal bookkeeping is persisted and restored, so a `graph.join` after resume
     still sees branches completed before suspension. Reserved model keys never persist
     — nor restore: a forged store record cannot overwrite the current run's identity.
   - New optional extension crate **`extensions/minigraph-state-redis`**:
     `v1.redis.persist.model` (SETEX, native expiry) and `v1.redis.retrieve.model`
     (atomic GETDEL consume, Redis 6.2+) register automatically when an application
     links the crate (the minigraph-playground example app now does); lazy connection
     (`redis` crate `ConnectionManager` — the Lettuce analog), `redis.*` config keys
     shared with the Java sync-over-async family. Any composable function honoring the
     documented store contract can replace it.
   - New tutorial **`tutorial-14`** (a purchase workflow with THREE human checkpoints —
     order, approval, delivery release — as four short runs, copied verbatim from the
     Java engine) with an end-to-end test driving the real Redis client wire path —
     including input validation: a later-stage request without a suspended record is
     rejected with HTTP 404 (a null-safe presence check via `{var}` substitution inside
     a `text()` constant) — and every stage reply carries the `run` flag. The
     **Workflow Suspension** guide chapter (incl. the state-store contract),
     skills-reference entries, `help.md` + `help tutorial 14.md` Playground pages.
   - **Business correlation-id fidelity in graph telemetry and logs**: the graph walkers
     stamp the engine's business-cid tag from the graph's `model.cid` on every skill
     invocation (walkers are event interceptors, so PostOffice does not auto-propagate
     it), and the suspend/resume skills annotate their trace spans with `cid` — the
     application log context's `cid` on a traced store/skill line is always the business
     ID, never internal routing metadata.
   - **Declarative response status**: a graph can stage its HTTP status
     (e.g. `int(404) -> output.status`); `graph.executor` applies it to the graph's
     reply at completion.

### Changed

1. **CompileGraph is now the deployment gate for graph models (CompileFlows parity).**
   A deployed graph model is executable at `POST /api/graph/{graph-id}` only when it is
   listed in the graph manifest (`graph.model.automation`, e.g. `classpath:/graphs.yaml`)
   AND passes the CompileGraph quality gate at startup. A graph that fails the gate, or
   is not listed, answers **HTTP-404** as if the model does not exist — the lazy,
   per-request loading of deployed models is removed, so an unvalidated JSON file in the
   deployed folder can no longer be executed. This mirrors CompileFlows, where an invalid
   flow never becomes executable. The playground dry-run workspace
   (`location.graph.temp`) is a separate surface and is unaffected. Completing the
   CompileFlows symmetry, the manifest now carries the location of its own models:
   the optional **`location`** entry in `graphs.yaml` (default `classpath:/graph`)
   replaces the `location.graph.deployed` application property, exactly like the
   `location` entry in `flows.yaml` — the default preserves existing deployed-folder
   layouts, and a leftover `location.graph.deployed` property logs an obsolete-key
   warning at startup.
   With the gate mandatory, the graph executor no longer re-validates gate-guaranteed
   rules per request (root/end existence, the suspend-node contract) — data-driven
   runtime guards (store-record contents, dynamic jump targets, loop detection) remain.
   The two lanes are now explicit: production = models → CompileGraph → deployed graphs
   → graph executor; dry-run = drafts in the temp workspace → UI CLI validation at node
   create/update → graph traveler with full runtime validation. The gate's whole-graph
   rules are modularized (`model_validator`) and reused by the playground's `run`
   command as a pre-run quality check: draft authoring still allows partial models,
   but the moment the author asks to run, the suspend/resume contract must hold —
   a violation reports `Unable to run - <reason>` before traversal starts.
   **Migration:** applications that relied on lazy loading must set
   `graph.model.automation` and list their deployed graph IDs in the manifest — the
   shipped example already does; set the manifest's `location` only if your deployed
   folder is not `classpath:/graph`.

### Fixed

1. **WS duplicate-command suppression window is now ANCHORED like Java's** (knowledge
   graph): the previous stand-in re-put on every duplicate — a sliding window under
   which a continuous duplicate stream never let a command through; migrated to the Java
   `last.ws.message` 1 s `ManagedCache` (no re-put on duplicate, one command per ~1 s,
   Java's `Duplicated message` debug log — emitted once, at the comparison, like Java),
   and the dedup memory is now bounded and self-expiring (entries previously survived
   session close forever).
2. **`/info` `up_time` renders exactly like Java's `Utility.elapsedTime`**: zero
   components are omitted ("2 minutes", not "2 minutes 0 seconds"), Java's strict
   boundary quirks are kept (exactly 1 minute → "60 seconds"), and a sub-second value
   renders "N ms". The same shared formatter renders the `ManagedCache` create log, so
   that new Java-parity log line is exact from day one.

---
## Version 4.10.6, 7/26/2026

Feature release. Version number re-aligned with the Java repo's 4.10.6 (whose contents —
a Sonar-remediation patch — differ by design). Two arcs: the **annotation→macro
consistency P2 leftovers already on main** (yaml.preload.override, the
registration-metadata contract with golden conformance vectors, Unicode-scalar string
plugins) and the **typed-AsyncHttpRequest arc** (typed HTTP functions with no engine
special case, the single-source request dataset, pretty-printed JSON responses, the
/info/routes actuator with ops-tunable worker instances) — merged as PR #183 after six
review rounds on one commit.

### Added

1. **The engine dogfoods its extension points (annotation→macro consistency, in lock-step
   with the Java reference).** All 46 built-in mapping plugins are now `#[simple_plugin]`
   declarations and both built-in fetch features are `#[fetch_feature]` declarations —
   discovered through the same link-time inventory as user code, exactly like Java's 47
   `@SimplePlugin` and 2 `@FetchFeature` classes. The hard-wired
   `builtin_registrations()` / `register_builtins()` seedings are gone, and the loaders
   assert the registered counts at startup so a linker-elision regression fails the boot
   loudly. Behavior is identical: same names, same bodies, same error messages (the
   existing suites prove it). `#[simple_plugin]` now accepts the positional string form
   (`#[simple_plugin("getFirst")]`), mirroring `#[fetch_feature]`'s grammar — the string
   is the registered name, `name = "..."` remains an equivalent alias, and omitting both
   keeps the camelCase-of-fn-name derivation.
2. **Marker stacking is order-insensitive, matching Java annotation semantics.**
   `#[zero_tracing]` and `#[event_interceptor]` are now real attribute macros using the
   `#[optional_service]` self-reattachment pattern: written above or below `#[preload]`,
   the behavior is identical (previously they were consumed only when written below — an
   order requirement Java does not have). The inline-args form
   (`#[preload(..., zero_tracing, interceptor)]`) is unchanged; a marker with no primary
   attribute on the item is a compile error with a helpful message.
3. **Compile-fail guards for the macro surfaces** (`tests/ui`, trybuild): eleven fixtures
   across the three runtime crates pin every deliberate macro compile error — unknown
   parameters, missing required names/routes, empty route segments and conditions, and
   markers with no primary attribute — so error messages are part of the tested contract.
4. **`#[fetch_feature]` accepts a stacked `#[optional_service("condition")]` marker**
   (Java `@OptionalService` parity, same grammar as on the platform macros): a declared
   feature loads only when the configuration condition holds, logged as
   `Skip optional FetchFeature - {name}` otherwise. `#[simple_plugin]` deliberately takes
   no such marker — plugins are Event Script capabilities (flow vocabulary), never
   conditionally on/off.

5. **`yaml.preload.override` is ported (P2 of the annotation→macro consistency arc, D4).**
   The Java operational surface with identical semantics: config files named by the key
   rename, fan out, or re-tune the `instances` of any `#[preload]` route at deploy time
   without recompiling — `original` / `routes` / optional `instances` / optional
   `keep-original`, comma-separated locations with missing files logged and skipped,
   multi-file merge (route-set union; the first file to set `instances` wins), applied at
   boot between inventory collection and registration (after `env_instances` resolution,
   Java's order). An override entry matches when ANY of a function's declared
   comma-separated routes appears as an `original`, and the declared list is replaced by
   the override's sorted route set. Seven-scenario regression suite mirrors Java's
   `PreloadOverrideTest`.
6. **The registration-metadata contract, with golden conformance vectors (P2/D5).** The
   cross-language contract behind `#[preload]` and its family is now a spec page
   (`docs/guides/registration-metadata-contract.md`, adapted from the Java reference) with
   **golden vectors shared verbatim** between repositories
   (`registration-vectors/{core,plugin,feature}.json`, byte-identical to the Java copies)
   and three conformance suites that declare the same fixture set through the Rust macro
   carrier and assert declared metadata + boot-resolved registration against the golden
   entries — including env-instances resolution, marker order-freedom, name derivation
   (`fn vector_derived` and Java's `class VectorDerived` register the same
   "vectorDerived") and optional-service gating. ADR-0008 records the decision (the twin
   of the Java ledger's ADR-0009).

7. **Typed `AsyncHttpRequest` functions** (field gap report — the Rust port could not type
   a function's input as `AsyncHttpRequest`): the request model now implements
   `Serialize` / `Deserialize` as thin delegates onto its own map-shape builder/parser,
   so `#[preload(..., typed)]` + `TypedFunction<AsyncHttpRequest, O>` flows through the
   ordinary typed adapter with no engine special case (Java, by contrast, special-cases
   the class inside `WorkerHandler.getMapBody`) — the template rule for Python/Node
   ports: a request class constructible from the request map makes typed signatures just
   work. `from_value` now parses the full SERVER-side dataset (`ip`, `https`, `timeout`,
   raw `query`) with round-trip integrity, and the accessor surface reaches Java parity
   (`path_parameter`, `query_parameter`/`query_parameters`, `cookie`, `session_info`,
   `remote_ip`, `is_secure`, `query_string`, `body_as` — with full fluent-builder
   symmetry: `set_remote_ip`, `set_secure`, `set_query_string`, `set_cookie`,
   `set_session_info`, `set_query_parameter_values`, `set_route_timeout_seconds`). The
   REST automation server now constructs the request dataset THROUGH the struct at both
   assembly sites (the main dispatch and the static-content filter), so the wire shape
   has exactly one definition — `AsyncHttpRequest::to_value()` — and server↔struct drift
   is impossible by construction. The hello-world `greeting.api` is rewritten in the
   typed form as the living example.

8. **The `/info/routes` actuator is ported** (`routes.actuator.service`, a default
   rest.yaml endpoint like its siblings): the app block plus the local routing table
   split by visibility — `routing.public` / `routing.private`, route → instance count,
   sorted for deterministic output. Java's optional `journal` / `route_substitution` /
   mesh `network` blocks are omitted when empty, and those subsystems do not exist in
   this port, so the response is `{app, routing}`. Only `/info/lib` remains deferred
   (a Rust binary has no runtime dependency manifest). The endpoint's first payoff: the
   actuator family now runs 5 workers each (rule of thumb) with ONE ops knob —
   `worker.instances.actuator.services`, the same key as the Java engine — and the
   hello-world demo sizes its I/O-shaped fixtures accordingly (`event.api.auth` 30 with
   `worker.instances.event.api.auth` — a real deployment verifies bearer tokens against
   an OAuth2 authority; `http.request.filter` 20 with
   `worker.instances.http.request.filter`), so operations teams can fine-tune in QA/Perf
   before promoting to production.

### Fixed

1. **One conflict policy across every registry** (matching the Java engine's actual
   `Platform.register` behavior): explicit registration wins over declarative, and a
   duplicate name warns + last-wins — simple plugins
   (`Reloading SimplePlugin {name} - please check duplicated plugin name`, including a
   user plugin shadowing a built-in), fetch features
   (`Reloading FetchFeature {name} - please check duplicated feature name`, previously
   first-wins on the declarative path), and websocket services (previously silent
   replace). Preload routes already warned + reloaded.
2. **Two stale doc claims corrected**: `#[preload]` DOES accept a comma-separated route
   list (aliases are compile-validated and used in production), and the public/private
   function distinction IS ported (private by default; a private function is not
   reachable through `/api/event`).

### Changed

1. **JSON response bodies render pretty-printed** (presentation parity with the Java
   engine, whose `async.http.response` writes through SimpleMapper's default —
   pretty-printing — mapper): one shared render rule for every JSON response the REST
   automation server writes, function responses and the `/info` / `/health` / `/env`
   actuators alike (2-space indentation; the HTML `<pre>` shell wraps the same pretty
   text). The Event-over-HTTP `/api/event` path is unaffected — it responds with a
   MsgPack envelope, not rendered JSON.
2. **The `length` and `substring` string plugins use Unicode scalar values** (maintainer
   ruling, superseding the earlier UTF-16 retrofit): indexes and counts address whole
   characters (code points) — modern ports use Unicode-native semantics, and Java's
   UTF-16 code units are a JVM legacy that must not propagate to future Python/Node/Go
   ports. Identical for English/Chinese/BMP text (`f:length` of `你好` is 2 in both
   engines, never the UTF-8 byte count 6); emoji and other supplementary-plane
   characters count 1 here, 2 in Java. This also retires the retrofit's
   surrogate-split micro-divergence: with scalar indexing there is no lossy case at all
   (Rust strings cannot hold unpaired surrogates, so exact Java parity was structurally
   impossible). Out-of-bounds semantics and error messages unchanged.

---
## Version 4.10.5, 7/24/2026

Security patch in lock-step with the Java engine's v4.10.5.

### Security

1. **Playground webapp migrated to react-router 8.3.0
   ([dependabot #16](https://github.com/Accenture/mercury/security/dependabot/16)).**
   Remediates the React Router **RSC Mode CSRF Bypass** advisory (follow-up to
   CVE-2026-22030; affected `>= 7.12.0, < 8.3.0`). The webapp depended on
   `react-router-dom` ^7.18.1, which ends at 7.18.1 and pins the vulnerable
   `react-router` exactly — v8 consolidated everything into the single `react-router`
   package, so the migration swaps the dependency and updates the import specifiers in
   the four source files that referenced `react-router-dom`. Validation: `npm audit`
   clean (0 vulnerabilities), all 124 webapp tests pass, and the served bundle in
   `crates/knowledge-graph/resources/public` was rebuilt and redeployed via
   `npm run release`.

---
## Version 4.10.4, 7/24/2026

Patch release in lock-step with the Java engine's v4.10.4. Two arcs, one drive: the
**configurable traceparent carrier** (standards-first — the optional
`http.traceparent.header` name exists for backward compatibility with legacy systems
only, and the standard `traceparent` always wins inbound) and the **interop hygiene
round** (clean delivered envelope view, wire alignment of the outbound `/api/event`
request, x-ttl ingress parity), validated end to end by the `ce_traceparent` four-way
interop drive with **all eight echoes identical**
([test report](docs/test-reports/event-over-http-interop.md)).

### Fixed

1. **The delivered envelope view is scrubbed of engine metadata (interop hygiene round).**
   The pre-release `ce_traceparent` interop drive's four-combination matrix (see
   `docs/test-reports/event-over-http-interop.md`) found that a peer-transported or
   edge-merged `my_*` / `x-event-api` header could surface in a function's input
   **envelope** header view (the injected input copy was already clean). The worker now
   scrubs the five engine keys from the delivered envelope for non-interceptor functions —
   whatever a peer transported can never masquerade as application data — while the legacy
   `my_correlation_id` compat carrier remains honored into the injected view before the
   scrub, and event interceptors keep raw transport fidelity. Two regressions added
   (entry-side twins of the exit sanitization), mirrored from the Java reference.
2. **The programmatic Event-over-HTTP demo no longer copies its injected metadata onto the
   outgoing event.** The hello-flow `EventOverHttpRpc` task forwards business headers only —
   the injected `my_*` view describes the local function's own context and is never
   transported.
3. **Wire hygiene of the outbound `/api/event` request, aligned to the Java reference.**
   The engine's trace stamps (`x-trace-id`, `traceparent`, a custom
   `http.traceparent.header` name) now use insert semantics — one value each on the wire,
   never duplicated with the transport leg's own copies; the HTTP-level correlation-id
   header is no longer stamped on the engine's Event-over-HTTP transport leg (the business
   cid rides inside the envelope on the `my_cid` tag, exactly like Java); the request
   carries `accept: */*` and `x-small-payload-as-bytes: true` (Java's header set); and
   REST automation announces the resolved correlation-id / trace-id / traceparent header
   names at startup with the Java engine's wording.
4. **The endpoint timeout rides the request dataset as the `x-ttl` header (Java parity).**
   Java's `AsyncHttpRequest.setTimeoutSeconds` stores the REST endpoint timeout AS the
   `x-ttl` header (milliseconds), so a flow's `input.header` view naturally carries the
   key; the Rust ingress modeled the timeout outside the header map, making the two
   engines' echoed header sets differ. REST automation now stamps the endpoint timeout
   under `x-ttl` on the request dataset — a caller-sent value wins (Java copies inbound
   headers after the stamp), which is exactly how the Event-over-HTTP client's own TTL
   rides through the `/api/event` endpoint.

### Added

1. **Configurable traceparent header name (field request).** A new header-name key completes
   the observability impedance-matching surface: `http.traceparent.header` (default
   `traceparent`), plus a per-entry `traceparent.header` override in a rest.yaml endpoint.
   An escape hatch for an intermediary (e.g. an API-gateway header allow-list) that strips
   the standard W3C header: outbound calls (async HTTP client and Event-over-HTTP) stamp the
   same W3C value under **both** names, and inbound resolution (REST automation) honors the
   **standard `traceparent` first** — the custom name is read only when the standard header
   is absent, because a well-formed standard traceparent means the caller already speaks
   W3C/OTel and a residual proprietary header is safely ignored. Unlike the trace-id
   conflation workaround, the
   full W3C context (trace-id, parent span-id, flags) crosses the intermediary, so
   cross-application **span parenting** survives. Default behavior unchanged (`traceparent`).
   **The standard W3C/OpenTelemetry `traceparent` remains the project's position** — the
   optional key is for backward compatibility with legacy systems only, and departure from
   the standard is discouraged (a renamed carrier is invisible to OTel-compliant tooling);
   treat a custom name as a temporary bridge and plan the migration back to the standard
   header. Mirrors the Java engine's feature of
   the same name in lock-step (identical config keys and precedence; the Java engine's
   `kafka.traceparent.header` / `secondary.kafka.traceparent.header` twins have no Rust
   surface — the Kafka service mesh is not ported).

---
## Version 4.10.3, 7/23/2026

Patch release for **field deployment**, in lock-step with the Java engine's v4.10.3 —
releases are immutable, so the post-4.10.2 fixes roll up into a new patch. No engine
behavior changes: demo hygiene and refreshed playground webapp dependencies.

### Changed

1. **The demo echo displays the clean envelope-header view
   ([#175](https://github.com/Accenture/mercury/pull/175)).** The hello-world echo now
   reflects the function's ENVELOPE headers (the transported view) rather than the
   injected input copy — making the demo a live proof that the reserved `my_*` metadata
   never rides the wire: the injected keys are visible in the function (and documented),
   but the echoed transport view is clean. The hello-flow example adopts
   `log.format=json`, so a side-by-side java/rust demo run reads identically in both
   terminals (presentation parity down to the demo experience).
2. **Playground webapp dependencies refreshed from npm
   ([#174](https://github.com/Accenture/mercury/pull/174)).** Fresh resolution of the
   knowledge-graph Playground webapp lockfile within the declared semver ranges
   (including the dependabot react-router advisory bump,
   [#173](https://github.com/Accenture/mercury/pull/173)): `npm audit` clean, package
   registry and integrity checksums verified. Webapp-only — no Rust crate dependency
   changes.

---
## Version 4.10.2, 7/23/2026

Patch release: **the metadata contract, hardened in lock-step with the Java engine**. A
composable function has exactly three inputs — headers, body and instance; the headers
are a **copy** of the envelope headers with read-only metadata **injected by the worker
at entry and sanitized at exit** — metadata is never transported in the event itself.
The business correlation-id rides an engine-managed envelope tag (`my_cid`) and is echoed
on every HTTP response, and the RPC reply path is aligned with the Java design: a
**single reserved `temporary.inbox` route** keyed by correlation id — the `inbox.*`
namespace belongs to applications — registered with deterministic essential-service
sequencing, with the mesh-era `route@origin` syntax never generated. Re-verified live in
all four direction combinations at an **empty trace-signature diff** — the full battery
(functionality, correlation echo, injected-metadata parity, authentication, signature) is
recorded in the [Interop Test Report](docs/test-reports/event-over-http-interop.md).
The release also mirrors the team-contributed Event Script **collection plugins**
(`isEmpty`, `getFirst`, `getLast` — flows are engine-portable, so plugins must exist on
both engines). Metadata/reply-path changes in PR
[#171](https://github.com/Accenture/mercury/pull/171).

### Added

1. **The HTTP response echoes the business correlation-id
   ([#171](https://github.com/Accenture/mercury/pull/171)).** REST automation returns the
   request's correlation-id (inbound or edge-generated) on the response under the
   configured header name (default `X-Correlation-Id`), so an edge caller can correlate
   without parsing the body. A response header of the same name set by the function takes
   precedence. The edge also stamps the resolved value onto the request dataset headers,
   so the function, the flow engine (`model.cid`) and the response all see the SAME id.

2. **Collection plugins for Event Script: `isEmpty`, `getFirst`, `getLast`.** Contributed
   to the Java engine in mercury-composable
   [PR #220](https://github.com/Accenture/mercury-composable/pull/220) and mirrored here
   for flow portability — Event Script flows are engine-portable YAML, so
   `f:isEmpty(...)` behaves identically on both engines, including error text (which
   reads the same in aggregated logs). `isEmpty`: a single Collection/Map/String/array —
   true when it has no elements (use `isNull`/`notNull` for null checks; null or an
   unsupported type is an error). `getFirst`/`getLast`: a single non-empty List — its
   first/last element.

3. **The `inbox.*` route namespace belongs to applications
   ([#171](https://github.com/Accenture/mercury/pull/171)).** RPC replies now resolve
   through the ONE reserved reply-listener route, `temporary.inbox` (Java `TemporaryInbox`
   parity: private, zero-tracing, 500 instances, registered by the essential-service step
   at the highest startup priority and present on every platform from construction), keyed
   by correlation id — no per-request pseudo-route and no reserved prefix. A workflow
   application is free to register routes like `inbox.approval` (a human-operator staging
   area); they are reachable and traced like any user function. The worker's RPC-served
   detection now uses the reserved `rpc` envelope tag (Java `EventEmitter.RPC`) instead of
   a reply-address prefix. Per Eric's ruling, the legacy `route@origin` addressing syntax
   (meaningful only under the Kafka service mesh) is never generated; an inbound `@origin`
   suffix is parsed away.

### Fixed

1. **Protected metadata is never transported in the event
   ([#171](https://github.com/Accenture/mercury/pull/171)).** The business correlation-id
   now rides an engine-managed envelope tag (`tags` wire field — wire-compatible with the
   Java engine) instead of a `my_correlation_id` envelope header, and the worker injects
   the `my_*` read-only keys (`my_route`, `my_trace_id`, `my_trace_path`,
   `my_correlation_id`) into the function's input header copy at delivery — this port now
   injects the same four keys as the Java engine, so the echo demos are replicas. At exit
   the worker sanitizes a returned envelope's headers symmetrically: the `my_*` keys and
   the engine-internal `x-event-api` relay guard never leave a function as response
   headers, and neither reaches a function's view on the way in (tags are engine-visible
   only). A callee still honors the legacy header from a pre-4.10.2 peer (injected, then
   stripped), but no longer sends it — business-cid continuity in mixed fleets requires
   both sides on this version, the same upgrade-together posture as the wire format.

---
## Version 4.10.1, 7/23/2026

Patch release: **telemetry presentation parity with the Java reference engine**. Field
installations stay polyglot — DevSecOps teams aggregate both engines' telemetry and logs
in one place — so the trace-record topology and log presentation of this port must be an
**exact structural replica** of the Java engine's. After this release they are: the
normalized-signature diff is **empty in all four direction combinations** (java→java,
rust→rust, java→rust, rust→java; both calling patterns, incl. authentication). The
`/api/event` edge is a visible span aligned with the Java reference, the application log
context appears only on lines with a real request trace, the declarative demo endpoint is
renamed for symmetry, the new `event.api.auth` demo shows endpoint protection with
session-info forwarding, and the full interop story — evidence, defects, fixes, and the
learnings kept as a playbook for future language ports — is deposited in this repo's docs
as the [Interop Test Report](docs/test-reports/event-over-http-interop.md). All changes
in PR [#169](https://github.com/Accenture/mercury/pull/169).

### Added

1. **Event-over-HTTP authentication demo
   ([#169](https://github.com/Accenture/mercury/pull/169)).** The hello-world example overrides the default
   `/api/event` endpoint with a demo authentication service (`event.api.auth`) that
   validates the caller's `authorization` header against a shared secret resolved from
   the environment (`demo.peer.token: ${DEMO_PEER_TOKEN:demo}` on both peers — no
   hard-coded credential). The hello-flow example presents the token declaratively (a
   `headers` block in `event-over-http.yaml`) and programmatically (the request API's
   security headers), and session info injected by the auth service rides to the target
   function as read-only headers — REST automation now forwards auth-verdict headers as
   the request's `session` map (Java parity). The echo also forwards to a new
   `hello.pojo` function so span propagation is visible in the trace (lambda-example
   parity).

### Changed

1. **The declarative demo endpoint is renamed for symmetry with its programmatic twin
   ([#169](https://github.com/Accenture/mercury/pull/169)):**
   `/api/event/http/demo` → `/api/event/http/declarative` and flow id
   `event-over-http-demo` → `event-over-http-declarative` in the hello-flow example.
2. **REST automation dispatches the endpoint service as a CALLBACK
   ([#169](https://github.com/Accenture/mercury/pull/169))** (Java `HttpRouter`
   parity): the event carries `reply_to = async.http.response` and its `cid` is the HTTP
   context id, while the business correlation-id rides the `my_correlation_id` envelope
   header (the worker's trace bracket prefers it, so `po.my_correlation_id()` is
   unchanged). The endpoint service's worker now self-records its span — the first leg
   of every trace is a real span record — and the response leg (`async.http.response`)
   is itself a visible function span parenting onto the replying function's span. The
   telemetry topology of a two-app Event-over-HTTP call is now an exact structural
   replica of the Java engine's — verified record-for-record against the Java reference
   signature (both patterns, incl. the deliberate cross-pattern asymmetry of the
   caller-side response leg).

### Fixed

1. **The application log context no longer leaks onto context-less lines
   ([#169](https://github.com/Accenture/mercury/pull/169)).** The
   `context` block appears ONLY on log lines emitted inside a traced function execution
   with a real request trace (Java parity: the log context registers per worker
   execution in lockstep with the trace bracket). Telemetry records and framework/system
   lines carry no context block at all — previously they carried a partial block with
   constants and a timestamp.
2. **Reserved `my_*` metadata is stripped from HTTP response headers
   ([#169](https://github.com/Accenture/mercury/pull/169))** (Java
   `copyResponseHeaders` protected-metadata parity): `my_route`, `my_trace_id`,
   `my_trace_path` and `my_correlation_id` never reach the wire.
3. **The Event-over-HTTP client returns a non-envelope response as-is
   ([#169](https://github.com/Accenture/mercury/pull/169))** (e.g. an
   authentication-layer 401 in the REST error shape) with its HTTP status, instead of
   failing with "Invalid event-over-http response" (Java `handleFutureResponse` parity).

---
## Version 4.10.0, 7/22/2026

Feature release: cross-language Event-over-HTTP interoperability with the canonical
[Java implementation](https://github.com/Accenture/mercury-composable) (its v4.10.0
shipped the same day — one version, two languages). The language-neutral wire format, the
`/api/event` service and client, declarative routing, a ready-to-run demo pair covering
both calling patterns, application log context on by default, and RPC span-lineage
telemetry. Validated by live bidirectional Java ⇄ Rust interop drives — see the
[Interop Test Report](https://accenture.github.io/mercury-composable/test-reports/event-over-http-interop/)
on the Java docs site.

### Added

1. **Language-neutral event envelope wire format
   ([#166](https://github.com/Accenture/mercury/pull/166)).** The envelope's MsgPack
   map with descriptive string keys is now a cross-language contract shared verbatim
   with the Java engine (normative spec: the Java repo's
   [Event Envelope Wire Format](https://accenture.github.io/mercury-composable/guides/event-envelope-wire-format/)
   reference), proven by golden conformance vectors kept byte-identical in both repos.
   Decoders treat an absent and a nil field alike and ignore unknown keys; the v1
   service accepts the **standard** format only — a legacy Java *compact* envelope
   (single-character keys) is rejected with a clear 400 (Java 4.10+ defaults to
   standard).
2. **Event over HTTP: the `/api/event` service + client
   ([#166](https://github.com/Accenture/mercury/pull/166)).** `POST /api/event` ships in
   the default `rest.yaml` (merged like the actuators — zero configuration): RPC and
   async dispatch with `x-ttl`/`x-async` semantics, 403 for private targets, in-band
   404/400/408, and trace propagation via `x-trace-id` + W3C `traceparent`. Preloaded
   functions are now **private by default** with the `is_private = false` opt-out (Java
   `@PreLoad` parity) and every engine internal is registered private — an application
   instance is a closed world unless a function is deliberately published. The
   `event_over_http` client posts a serialized envelope to a peer and returns the reply.
3. **Declarative Event over HTTP — `yaml.event.over.http`
   ([#166](https://github.com/Accenture/mercury/pull/166)).** Routes listed in
   `event-over-http.yaml` (with optional per-target security headers and `${...}`
   substitution) forward transparently: `po.request` returns the peer's reply,
   `po.send` with a `reply_to` runs the callback dance, a plain `po.send` is
   drop-n-forget with the 202 ack, and `send_later` honors the map. The `x-event-api`
   marker is the recursion guard — a forwarded event crosses the wire exactly once.
4. **Comma-separated route aliases in `#[preload]`
   ([#167](https://github.com/Accenture/mercury/pull/167))** — Java `@PreLoad` parity:
   `route = "hello.world, hello.declarative"` registers the same function object under
   every listed name with the same instance count and visibility. Empty segments are a
   compile error.
5. **Application log context is now on by default
   ([#167](https://github.com/Accenture/mercury/pull/167)).** platform-core ships a
   built-in `default-log-context.yaml` (embedded at compile time) so the structured JSON
   formats (`log.format=json` or `compact`) stamp the standard trace context (`cid`,
   `traceId`, `tracePath`, `spanId`, `parentSpanId`, `service`, `timestamp`) into every
   log line a traced function emits — no setup required. An application can replace the
   template with its own `app-log-context.yaml`, or opt out with the new
   `app.log.context=false` key. Applications already providing an `app-log-context.yaml`
   are unaffected. Plain-text logging (`log.format=text`, the default) is unaffected.
6. **RPC telemetry records — exactly one record per span
   ([#167](https://github.com/Accenture/mercury/pull/167)).** The caller now emits the
   `round_trip` trace record for each traced RPC response (Java `InboxBase.recordTrace`
   parity) while the worker suppresses its own record for an RPC-served execution whose
   reply reached the caller (Java `WorkerHandler.sendTracingInfo` gate) — so each span
   reports once, with full lineage: `parent_span_id` (the caller's span, unconditional)
   and `span_id` (the callee's span, adopted only from a **direct responder**; a relayed
   reply — e.g. a flow answering on behalf of the flow-adapter route — keeps the parent
   but omits the span, Java `spanIdFromResponder` parity). Callback-style invocations
   keep self-recording. The callee's trace annotations now ride the reply envelope (also
   on the Event-over-HTTP wire) and fold into the span's single record; the RPC reply
   itself carries the measured `round_trip` value. The programmatic `event_over_http`
   client stamps the calling function's trace context (incl. its span) onto the wire
   envelope, so remote functions parent onto the caller's span in both the declarative
   and the programmatic pattern.
7. **Event-over-HTTP demo endpoints in `hello-flow` — both patterns
   ([#167](https://github.com/Accenture/mercury/pull/167))** (the structural parallel of
   the Java composable-example, now on port **8100**): `/api/event/http/demo`
   (declarative — the flow's task is the foreign route `hello.declarative`, resolved
   through `event-over-http.yaml`) and `/api/event/http/programmatic` (the task passes
   the peer's `/api/event` URL directly to the request API). The `hello-world` echo
   registers the `hello.declarative` alias and is interchangeable with the Java
   lambda-example — same port 8085, same routes — so the demo doubles as a
   cross-language interop demo with zero configuration changes; see the walk-through in
   the [Event over HTTP](docs/guides/event-over-http.md) guide.

### Fixed

1. **HTTP client read timeout no longer truncates a sub-second TTL to 1 second
   ([#166](https://github.com/Accenture/mercury/pull/166)).**
   `AsyncHttpRequest::timeout_seconds()` rounds the TTL up, the response-timeout site
   adds a one-second wire grace, and the `event_over_http` client waits 100 ms beyond
   the remote TTL — so a peer that spends its whole TTL still delivers its in-band 408
   instead of losing to a local transport abort.
2. **The `hello-world` echo no longer drops MsgPack-binary bodies
   ([#166](https://github.com/Accenture/mercury/pull/166))** — it reflects the raw
   value instead of taking a JSON detour (JSON has no byte type; found by the
   cross-language interop matrix).
3. **A zero-traced hop no longer leaks a nested reply's span id
   ([#167](https://github.com/Accenture/mercury/pull/167))** as its own on the
   response envelope (Java parity: its reply carries no span).

---
## Version 4.9.0, 7/20/2026

**Graduation release — and the adoption of the canonical version line.** The version jumps
from 0.1.0 to **4.9.0** to track `mercury-composable` (Java), with which this engine is
behavior-synced: the companion REST contract is byte-identical, the graph/flow DSLs are
shared, and every engine fix since the port began landed in both implementations
(mercury-composable PRs #187–#204, released there as 4.9.0 the same day). One version, two
languages.

Everything since 0.1.0, in brief (increments 30–49 — the full record in
[`docs/INCREMENTS.md`](docs/INCREMENTS.md)):

### Added

1. **The synchronous AI-companion endpoint** `POST /api/companion/{id}/sync` (ADR-0008) —
   command outcomes in-band (`{ok, output, error, result}`, whole-traversal capture, WS tee
   for real-time human+AI collaboration), with a truthful contract: whole-output-aware `ok`
   classification, no silent dedup for RPC callers, `Syntax:` usage hints classify as
   failures.
2. **Discovery + contract commands**: `list graphs` / `list flows` / `describe graph
   {graph-id}` — self-service delegation (list → contract → delegate) with the root
   `purpose` enforced at compile as living documentation.
3. **Outbound HTTPS for the async HTTP client** (rustls + OS trust store, per-request
   `trust_all_cert`) — field-validated end-to-end against a live CA chain. Redirects are
   deliberately not followed (backend design; documented decision record in
   `draft-design-specs/platform-core-port.md` §5j).
4. **Numeric promotion + `f:round`** for the simple-plugin arithmetic family.
5. **The battle-tested AI-agent documentation**: hardened by 25 fresh-agent exercises
   across both engines (the last thirteen passing with zero documentation lookups) and
   kept in lock-step with the Java repo (back-port #203 there).
6. **The human documentation site** — 20 pages, published at
   [accenture.github.io/mercury](https://accenture.github.io/mercury/) (automated via
   `mkdocs gh-deploy` on pushes to main).
7. **Rust CI quality gates** — fmt + clippy (zero warnings) + the full workspace test
   suite on every PR.

### Fixed

1. **Join barrier: only valid completions count** — success-only completion marks cleared
   by `RESET`, and chained joins judged by recorded outcome (latent data-loss bugs found by
   probe, fixed in both engines).
2. **Companion `session` limited to the read-only status query** (topology subcommands are
   a WebSocket-session privilege).
3. **HTTP-boundary content-type dispatch** mirrors the Java `handlePayload` rules exactly.
4. Spring-named configuration keys retired: `app.profiles.active` / `application.name`.

**Verified:** 206 workspace tests green, `clippy` zero warnings, `fmt` clean — enforced in
CI from this release on.

---
## Version 0.1.0, 7/18/2026

The first end-to-end port of `mercury-composable` (Java, canonical v4.8.6) to Rust: the three
foundational layers, ported bottom-up (foundation → UI) across 29 verified increments and
validated against the canonical Java fixture suite. **This is the first port ready for manual
end-to-end testing** — 181 workspace tests green, `clippy` clean, `fmt` clean.

Out of scope by design: the Kafka service mesh (`minimalist-kafka`, `twin-kafka`, connectors)
and Spring (`rest-spring-3/-4`). `graph.js` is deliberately **retired** — an interpreter running
arbitrary user code is an attack surface the port does not carry; `graph.math` (typed, bounded)
and `graph.task` (reviewed, compiled functions) cover its use cases.

### Added

**platform-core (layer 1) — the actor-model event bus and operable runtime**

1. **Configuration management, event bus, and reactive back-pressure** — the `-D`/YAML config
   reader, the route-addressed event bus (functions coupled only by route name + `EventEnvelope`),
   and the FIFO ElasticQueue with manager–worker back-pressure (disk spill under overload).
2. **Application lifecycle and annotation macros** — `#[preload]` / `#[before_application]` /
   `#[main_application]` / `#[zero_tracing]` with link-time `inventory` registration (the Java
   classpath-scan analog), plus the one-line `auto_start_main!()` entry point.
3. **Observability** — OpenTelemetry-style distributed tracing, business correlation-id, and
   app-log-context (three-format logger).
4. **REST automation and operability** — `rest.yaml` as the router on a hyper HTTP edge,
   actuators (`/info`, `/env`, `/health`, `/livenessprobe`), and the static-content protocol
   (SHA-256 etag / HTTP-304, no-cache pages, the `static-content.filter` request interceptor).
5. **RPC inbox, HTTP client, and WebSocket server** — the lightweight RPC inbox (`AsyncInbox`
   parity), the async HTTP client (`async.http.request`), and the WebSocket server on the HTTP
   upgrade path with the declarative `#[websocket_service]` macro.

**event-script (layer 2) — the composable-flow engine**

6. **Flow model, compiler, and data-mapping engine** — the full `CompileFlows` port (all Java
   fixtures reused verbatim) and the runtime MultiLevelMap mapping engine over `rmpv::Value`
   (direct composite-key access primary; JSON-Path `$.…` for complex queries).
7. **The complete flow runtime** — sequential / response / decision / sink, parallel and
   fork/join (pipe-map barrier), pipelines with for/while loops and break/continue, `flow://`
   sub-flows with shared parent state, and the external state machine — plus TTL abort, metrics,
   and the flow-summary span.
8. **Plugins, HTTP, and resilience** — all 42 built-in plugins with the `#[simple_plugin]` macro,
   the HTTP flow adapter, the resilience handler, and the event-script mock.

**active knowledge graph (layer 3) — the MiniGraph Playground**

9. **MiniGraph and the graph toolchain** — the MiniGraph property graph (a platform-core
   built-in), the math expression engine, and the graph compiler + registry (13 tutorial
   fixtures compiled verbatim; the engine crate ships its own bundled resources).
10. **The graph runtime and core skills** — the executor state machine (composite `{flow}@{node}`
    correlation, decision routing, loop detection) and the core skills `graph.data.mapper`,
    `graph.math`, `graph.task`, `graph.join`, `graph.island`, `graph.api.fetcher`,
    `graph.extension`, plus the declarative `#[fetch_feature]` macro (OAuth 2.0 bearer
    injection). `graph.js` is retired.
11. **The Playground** — the command grammar (`GraphCommandService` port), the graph traveler,
    the WebSocket UI (`/ws/graph`, `/ws/json`), the AI-companion REST hop
    (`POST /api/companion/{id}`), and dev-gating (`app.env=dev`); the React webapp
    (`@xyflow/react`) served as static content by REST automation.

**Examples**

12. `examples/hello-world` (layer 1), `examples/hello-flow` (layer 2 — a YAML flow over HTTP),
    and `examples/minigraph-playground` (layer 3 — the runnable Playground app at
    `http://127.0.0.1:8100/`).
