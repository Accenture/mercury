---
title: Built-in skills reference
summary: The ten graph.* skills that make graph nodes active — data mapping, math and
  JavaScript evaluation, API fetching, composable-function tasks, sub-graph/flow extension,
  workflow suspend/resume, join, and island — with syntax, worked examples, and gotchas.
layer: knowledge-graph
audience: [developer, reference]
keywords: [graph.data.mapper, graph.math, graph.js, graph.api.fetcher, graph.task, graph.extension, graph.suspend, graph.resume, graph.join, graph.island, skill]
related:
  - guides/knowledge-graph/command-reference.md
  - guides/knowledge-graph/ai-agent-guide.md
  - guides/knowledge-graph/minigraph-commands.json
---

# Built-in skills reference

> **At a glance**
>
> - **What** — the ten skills shipped with the engine. Attach one to a node (`skill=<route>`)
>   to make it *active*: it runs when traversal reaches the node.
> - **They share** — the `source -> target` mapping syntax with its
>   [constant set](command-reference.md#constants), and the same state-machine
>   [namespaces](command-reference.md#namespaces) (`input.*`, `model.*`, `output.*`,
>   `{node}.result`).
> - **One skill per node.** A node returns a **decision** to the engine — `next` (follow the
>   connection), a **node name** (jump), or `.sink` (pause this path).

| Skill | Use it to… |
|---|---|
| [`graph.data.mapper`](#data-mapper) | copy/transform data between namespaces |
| [`graph.math`](#math) | compute and branch with fast inline math/boolean |
| [`graph.js`](#js) | ⚠️ **retired in this Rust port** (security) — use `graph.math` or `graph.task` |
| [`graph.api.fetcher`](#api-fetcher) | call external HTTP APIs declaratively |
| [`graph.task`](#task) | invoke a composable function through its route name |
| [`graph.extension`](#extension) | delegate to a sub-graph or an Event Script flow |
| [`graph.suspend`](#suspend) | persist workflow state at a human checkpoint and complete the run |
| [`graph.resume`](#resume) | restore persisted state and continue past the checkpoint without re-executing it |
| [`graph.join`](#join) | synchronize parallel paths |
| [`graph.island`](#island) | link the knowledge layer (dictionaries, providers, data entities) — isolated from traversal |

## graph.data.mapper {#data-mapper}

Copies and transforms data between state-machine namespaces. The workhorse for shaping inputs and
building the response.

```
skill=graph.data.mapper
mapping[]=source -> target
```

Sources/targets use `input.*`, `model.*`, `output.*`, or a node name (its properties); `text(...)`,
`int(...)` etc. inject constants. **`mapping[]` entries apply in order** within the node, so a
later entry may read an earlier entry's target — the chain idiom (ingest → transform → publish
inside one mapper). Example:

```
create node my-mapper
with properties
skill=graph.data.mapper
mapping[]=input.body.hr_id -> employee.id
mapping[]=input.body.join_date -> employee.join_date
```

Targets take **numeric list indices** too — the idiom for assembling a JSON list deterministically
(e.g. an `end` mapper after a fork/join):

```
mapping[]=fetch-one.result.profile -> output.body.profile[0]
mapping[]=fetch-two.result.profile -> output.body.profile[1]
```

## graph.math {#math}

Fast inline math and boolean evaluation for computation and decision-making. This is **the** skill
for inline compute/branch in this Rust port ([`graph.js`](#js) is retired). Statements run in order;
five types:

| Statement | Purpose |
|---|---|
| `COMPUTE` | evaluate a math expression → the node's `result` |
| `IF` | boolean decision → jump to a node (`THEN`/`ELSE`) |
| `MAPPING` | data-map source → target (no curly braces) |
| `EXECUTE` | run another `graph.math` node inline — results land on the **caller** (`{invoker}.result.*`), making this the module-reuse mechanism ([details](command-reference.md#math-statements)) |
| `RESET` | forget a node completely (guard, completion mark, state) so it can run again |

```
skill=graph.math
statement[]=COMPUTE: amount -> (1 - {input.body.discount}) * {book.price}
statement[]='''
IF: (1 - {input.body.discount}) * {book.price} > 5000
THEN: high-price
ELSE: low-price
'''
```

`{variable}` resolves a value from `input.*`, `model.*`, or a node property into the expression.
An `IF` returning a node name **overrides** natural traversal; returning `next` keeps it.
`NEXT:`/`DELAY:` control flow and timing.

**Iterating lists (`for_each[]`):** each `source -> model.{var}` entry whose source is a **list**
becomes an iteration array (parallel lists advance in lockstep and must agree on length; scalars
bind once; an unresolvable source removes the key). `BEGIN`/`END` split the statements into
pre-block (once) / each-block (per element) / post-block (once) — **without `BEGIN` the whole
list is the loop body**. Iteration is strictly sequential in list order, inside one node
execution; a taken `IF` jump breaks the loop and skips the post-block. Numeric accumulators work
with either `f:add` (numeric promotion: all-whole stays exact long, any decimal promotes to
double) or a pure-`COMPUTE` read-back. Full rules + worked example:
[for_each](command-reference.md#math-for-each).

**Gotchas:** a node runs **once** (guard against loops) unless you `RESET` it — an advanced,
use-with-care feature; a node may not contain only `MAPPING` statements (use the data mapper). The
expression dialect is a **narrow** JS-like subset — arithmetic, comparison and boolean operators
only: **no bitwise operators, no function calls** (e.g. `parseInt(...)`), no variables. `COMPUTE`
returns a double, so an integer result serializes as e.g. `8.0` (numerically exact — there is no
in-grammar integer coercion). For anything richer, use `graph.task` (a composable function).

## graph.js {#js}

> ⚠️ **Retired in this Rust port.** In the Java engine `graph.js` runs full JavaScript on GraalVM;
> **this Rust port disables it for security reasons.** Using it fails at runtime with:
> *"Skill graph.js is retired for security reasons - use graph.math or graph.task instead."*
>
> Use **`graph.math`** for inline compute/branch, or **`graph.task`** to invoke a composable function
> for anything a narrow expression can't express. Do not author `graph.js` nodes.

## graph.api.fetcher {#api-fetcher}

Calls external HTTP APIs declaratively, driven by **Dictionary and Provider config nodes** — the
full authoring rules (Provider URL `{name}` placeholders, the Dictionary's bare `input[]`
parameters with `:default`, `response.* -> result.*` output mapping) are in
[Provider & Dictionary](command-reference.md#provider-dictionary). Supports response deduplication
and bounded fork-join concurrency.

```
skill=graph.api.fetcher
dictionary[]=<data-dictionary-node>     # one or more (required)
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name   # optional: result always lands at {node}.result
for_each[]=<array-source> -> model.<var>   # optional: iterate a runtime list (see below)
concurrency=3                            # optional: 1–30, default 3
ttl=8s                                   # optional: per-call deadline override (see below)
exception=<error-handler-node>           # optional
```

**Iterating a runtime list (`for_each`):** the array source is typically a **prior fetcher's
result** (`{fetcher}.result.{key}`); wire the current element into each call with
`input[]=model.<var> -> {dictionary-parameter}`. Each iteration's `result.{key}` values are
**appended into one array** on this node's result set, and the order **deterministically follows
the source list** (batches of `concurrency` run in order; responses join in request order). Full
rules: [Iterative fetching](command-reference.md#for-each).

Worked example (fetch a person's name and address):

```
create node fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
```

The result lands at `{node}.result`. **Gotchas:** identical requests (same provider + input
parameters) are **deduplicated within the graph instance** — the cache holds **successful
responses only** (a failed call is never cached, so a retry after `RESET:` makes a real call;
an identical *successful* call reuses the cached response); the `input[]` targets must **match
the dictionary parameter names** exactly, or execution fails. The dictionary/provider setup this
skill depends on is specified in [Provider & Dictionary](command-reference.md#provider-dictionary).

**HTTP semantics:** one Provider call is exactly **one HTTP request** — redirects are never
followed (a `3xx` is a non-failure: status and body are captured and traversal proceeds).
`{node}.status` **always** carries the HTTP status of the fetch, success included.
`response.*` in Dictionary `output[]` addresses the **body only** (the bare root
`response -> result.{key}` captures a whole non-JSON body); response headers are available
via `feature[]=log-response-headers` at `{node}.header.response.{name}`.

**Failure routing:** with `exception={handler-node}`, a failed call (HTTP ≥ 400) sets
`{node}.status`/`{node}.error`, skips the `output[]` mappings, and **jumps to the handler**
instead of aborting — the building block for bounded retry loops
([full pattern](command-reference.md#failure-routing)). Without it, the run aborts.

**Deadline:** each call is bounded by the propagated `model.ttl` (default 30 s); the optional node
`ttl` (duration syntax `<digits>` + `s`/`m`/`h`/`d`, e.g. `8s`) overrides it for this node's calls
only. A deadline shorter than the graph's own budget makes a slow provider time out **first**, so
`exception=` routing handles the timeout instead of the whole run aborting on `model.ttl` — the
time-boxed half of a bounded retry loop. The same effective deadline is stamped as the outbound
`x-ttl` request header, aligning the HTTP client's wire-level read timeout (deadline + a one-second
grace) with the graph-side deadline, so a hung upstream's socket self-cancels instead of lingering
after the 408. When the target is another Mercury application, its ingress honors the inbound
`x-ttl` over the endpoint's configured timeout — the caller's deadline propagates end-to-end.
Details and gotchas: [`graph.task`](#task).

## graph.extension {#extension}

Delegates to another **graph model** or an **Event Script flow**, so you can compose larger
capabilities and reuse logic. **Discover the valid targets with `list graphs` / `list flows`**
([discovery commands](command-reference.md#describe)) — no out-of-band brief needed.

```
skill=graph.extension
extension=<graph-id>          # a sub-graph …
extension=flow://<flow-id>    # … or an Event Script flow (note the flow:// prefix)
input[]=input.body.person_id -> person_id
output[]=result -> output.body
```

Sub-graph example (reuse a deployed graph):

```
create node performance-evaluator
with type Extension
with properties
skill=graph.extension
extension=evaluate-sales-performance
input[]=input.body.department_id -> id
output[]=result.sales_performance -> output.body.sales_performance
```

**The delegation contract (rules, not just the example):**

- `extension={graph-id}` resolves among the **deployed graph models** (compiled at startup from
  the app's `resources/graph` folder — the same ids callable at `POST /api/graph/{graph-id}`).
  A session draft is **not** addressable — export and deploy it first. A missing id fails the
  node fast at run time.
- Each `input[]` **target** is a bare key that becomes the sub-graph's `input.body.{key}` (e.g.
  `input[]=input.body.person_id -> person_id` feeds the sub-graph's `input.body.person_id`).
  There is **no whole-body `*` target** on `graph.extension` — map named keys (the `*` merge idiom
  is [`graph.task`](#task)-only).
- The node's **`result.*` namespace is the sub-graph's `output.body`**: `result` (bare) is the
  whole response body; `result.{key}` a field of it.
- The same contract applies to a **flow** target (`extension=flow://{flow-id}`): the named keys
  feed the flow's `input.body`, and `result.*` is the flow's `output.body`.
- The optional node `ttl` (duration syntax, e.g. `10s`) overrides the propagated `model.ttl` as
  the **deadline for the delegated call** — a shorter child deadline lets the sub-graph or flow
  time out first, so this node's `exception=` route catches the timeout and can retry within the
  parent graph's remaining budget (same parameter as [`graph.api.fetcher`](#api-fetcher) and
  [`graph.task`](#task)).

This is the seam between the semantic layer and the composable (Event Script) layer beneath it —
authoring the target flow: [Event Script AI agent guide](../event-script/ai-agent-guide.md) +
[flow grammar](../event-script/flow-grammar.md).

## graph.task {#task}

Invokes a **composable function** — a `TypedLambdaFunction` registered with `@PreLoad` — through its
route name. The lightweight way to plug a small piece of custom business logic into a graph: your
own function becomes, in effect, a custom skill.

```
skill=graph.task
task=<function-route>
input[]=input.body -> *                  # '*' merges the mapped value into the request body
input[]=text(minigraph) -> header.x-app  # 'header.{name}' sets a request header
input[]=input.body.id -> model.id        # 'model.{key}' stages a state-machine variable
output[]=result -> output.body
```

Worked example (tutorial-13 — any registered route is callable, so `async.http.request` turns
the node into an HTTP client by configuration):

```
create node hello-task
with type Task
with properties
skill=graph.task
task=async.http.request
input[]=input.body.person_id -> model.person_id
input[]=text(http://127.0.0.1:${rest.server.port:8080}) -> host
input[]=text(/api/mdm/profile/{model.person_id}) -> url
input[]=text(GET) -> method
input[]=text(application/json) -> headers.accept
input[]=text(5000) -> headers.x-ttl
output[]=result -> output.body
```

`input[]` entries apply **in order**, so field mappings after a `*` merge into the request body,
and the body auto-converts when the function declares a PoJo input. A `model.{key}` target stages
a **state-machine variable** instead of a body field, and later entries can reference it as a
**dynamic variable** — `{model.person_id}` above resolves inside the `text(...)` constant, the
same idiom as Event Script. The `${rest.server.port:8080}` reference is **environment/config
substitution**, resolved when the model is loaded (at deployment compile, and at `instantiate
graph` for a dry-run) while the authored and exported model keeps the placeholder. The result lands at
`{node}.result` and response headers at `{node}.header` — in `output[]` mappings, **`result`
(bare) is the function's whole result** and `result.{key}` a field of it (same rule as
`graph.extension`). Optional `for_each[]` with `concurrency`
(1–30, default 3) iterates with bounded fork-join; `exception=<node>` routes failures
([failure routing](command-reference.md#failure-routing)).

**Gotchas:** the `task` route must exist at runtime or the node fails fast; a call is bounded by
`model.ttl` (default 30 s) — or by the node's optional `ttl` property (duration syntax, e.g.
`10s`), which overrides the propagated value for this node only, the same deadline override as
[`graph.api.fetcher`](#api-fetcher) and [`graph.extension`](#extension). That deadline bounds the
**event call** only — it cannot reach inside a generic function, so a function with its own
downstream timeout contract takes it from the input mapping: the AsyncHttpClient reads
`headers.x-ttl` in **milliseconds** as its HTTP timeout (default 30 s when absent) and propagates
it on the wire as the `X-TTL` header. For multi-step
orchestration, prefer [`graph.extension`](#extension) — `graph.task` is for a single function
call. Writing the function itself:
[function AI agent guide](../event-driven/ai-agent-guide.md) (`#[preload]` + `ComposableFunction`).

**`ttl` — one grammar, two meanings.** On the three calling skills (`graph.task`,
`graph.api.fetcher`, `graph.extension`) the node `ttl` is a **child-call deadline**; on the
[suspend node](#suspend) the same `<digits>` + `s`/`m`/`h`/`d` grammar sets the **store-record
expiry** — a persistence timer, not a deadline. On any other skill the property is rejected by
the CompileGraph gate and the playground pre-run check. (The Java engine has a third meaning —
a script execution deadline on `graph.js` — which does not exist here because
[`graph.js` is retired in this Rust port](#js); the validator's rejection message accordingly
names three skills, not four.) Note also that model metadata
(`model.cid`/`instance`/`flow`/`ttl`/`trace`/`parent`/`root`/`none`/`run`) is engine-managed and
**immutable** — a data mapping that writes to it is rejected at compile time (the CompileGraph
gate and the pre-run check) and again at runtime in both walker lanes. The per-node `ttl` is the
sanctioned deadline mechanism, not rewriting `model.ttl`.

## graph.suspend {#suspend}

Persists the workflow state of the running graph to an external state store and lets the run
complete — the transaction resumes later through [`graph.resume`](#resume) with the same business
correlation ID. A **superset of `graph.task`**: the `task` property names the pluggable store
function, but the persistence envelope (`{cid, node, ttl, model, seen, run}`) is assembled by the
skill itself — **no input/output mapping on the node**.

```
skill=graph.suspend
task=v1.redis.persist.model
ttl=2d
```

The node carrying this skill **must be named `suspend`** — a reserved alias like `root`/`end`.
Two ways in: a **working node with a drawn edge** to `suspend` suspends when its skill completes
(edge mode — the edge is the declaration; a continuation edge is mandatory and the node is never
re-executed on resume), and a **decision jumps** to it by returning `suspend` from its
IF-THEN-ELSE (jump mode — the decision re-executes against the new input on every resume; it
must not draw an edge to `suspend`). The retired `suspend=true` property is ignored (deprecation
WARN). `ttl` is **mandatory with no default** (duration syntax `20s/5m/2h/2d`) — it becomes the
store record's expiry. Unless the graph staged its own `output.*`, the caller of the suspended
run receives `{"type": "suspended", "cid": ...}`.

**Gotchas:** the store must acknowledge (2xx) before the graph completes — a failed persist fails
the node (`exception=` routes it); a suspension point must be the sole active branch (never
between a fan-out and its join); only `model.*` survives — map what later steps need into the
model **before** the checkpoint. Full story: [Workflow Suspension](workflow-suspension.md).

## graph.resume {#resume}

Restores the workflow state persisted by [`graph.suspend`](#suspend) and continues traversal at
the recorded suspension point — **past an edge-mode checkpoint without re-executing it**, or by
**re-executing a jump-mode decision** against the new request input. Also a superset of
`graph.task` — the `task` property names the store function (`type=get`, body `{cid}`),
restoration is encapsulated, no mapping on the node.

```
skill=graph.resume
task=v1.redis.retrieve.model
```

Place it early — conventionally named `resume`, right after `root` or after setup nodes. Found:
the persisted model merges into the state machine (the current run's reserved keys always win),
traversal bookkeeping is restored (downstream joins still see pre-suspension branches), and the
walker jumps past the checkpoint onto its normal path. Not found — a **fresh transaction (the
normal first-run case)** or an expired record: traversal continues along the node's own forward
path. Either way the skill sets **`model.run`** to `resume` or `fresh`; the engine does not
distinguish absent from expired, so handling that condition is application logic — gate the
forward path with a `graph.math` IF-THEN-ELSE on `model.run` (or a `graph.task`) to reject,
advise the UI, or jump to a recovery node.

**Gotchas:** the record is consumed on retrieval (a duplicate resume behaves as a fresh run, never
a double execution); `model.cid` is the retrieval key, so resume-bearing endpoints deserve
rest.yaml `authentication`. Full story: [Workflow Suspension](workflow-suspension.md).

## graph.join {#join}

A synchronization barrier for parallel branches. It returns `next` **only when all** connected
upstream nodes have completed, and `.sink` (pause) until then. **Completion is success-only and
current**: a branch that failed into its `exception=` route does not count while it retries, and
a `RESET` node stops counting until it re-executes successfully — so a retry loop feeding a join
holds the barrier instead of firing it prematurely. A **chained upstream join** counts only once
it actually **fired** (an evaluation that sank does not count), so multi-stage joins compose
safely.

```
skill=graph.join
```

```
connect fetch-name to join with done
connect fetch-address to join with done
connect join to combine with proceed
```

**Gotchas:** needs at least two predecessors to be meaningful; it is the explicit fork-join
mechanism — without it, traversal proceeds as branches complete. The fork side needs no special
node: **multiple outgoing connections from one node run their branches in parallel** (see
[connect](command-reference.md#connect)). Data mapping is thread-safe, but branches should not
overwrite the **same scalar key** (last writer wins) — use per-branch `model.*` keys, or the
race-free `[]` **list append** (element order then follows completion order; use numeric indices
after the join when order must be deterministic).

## graph.island {#island}

Marks an **isolated** node: it always returns `.sink`, so traversal does not continue through it.
That isolation is the point — an island is **not executable, but it is required knowledge
structure**: linking Dictionary, Provider, and data-entity nodes under it gives the graph its
**entity-relationship diagram**. The graph is living documentation of enterprise knowledge — a
new joiner (or an agent) discovers the domain model by reading the connected dictionaries and
entities, not just the execution path.

**Convention (required): leave no node unconnected** — wire every config node into the island
structure (see [Island — the knowledge layer](command-reference.md#island)):

```
skill=graph.island
```

```
connect root to dictionary with contains
connect dictionary to person-name with data
connect dictionary to person-address with data
connect person-name to mdm-profile with provider
connect person-address to mdm-profile with provider
```

## See also {#see-also}

- [MiniGraph command grammar](command-reference.md) — the full command language, the
  [constant set](command-reference.md#constants), and
  [Provider & Dictionary authoring](command-reference.md#provider-dictionary).
- [AI agent guide](ai-agent-guide.md) — driving the Playground via the companion endpoint.
- [`minigraph-commands.json`](minigraph-commands.json) — the machine-readable command catalog.
