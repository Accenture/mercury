---
title: Built-in skills reference
summary: The eight graph.* skills that make graph nodes active — data mapping, math and
  JavaScript evaluation, API fetching, composable-function tasks, sub-graph/flow extension,
  join, and island — with syntax, worked examples, and gotchas.
layer: knowledge-graph
audience: [developer, reference]
keywords: [graph.data.mapper, graph.math, graph.js, graph.api.fetcher, graph.task, graph.extension, graph.join, graph.island, skill]
related:
  - guides/knowledge-graph/command-reference.md
  - guides/knowledge-graph/ai-agent-guide.md
  - guides/knowledge-graph/minigraph-commands.json
---

# Built-in skills reference

> **At a glance**
>
> - **What** — the eight skills shipped with the engine. Attach one to a node (`skill=<route>`)
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
`int(...)` etc. inject constants. Example:

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
| `RESET` | clear a node's "seen" flag so it can run again |

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
An `IF` returning a node name **overrides** natural traversal; returning `next` keeps it. Optional
`for_each[]` iterates a statement block; `NEXT:`/`DELAY:` control flow and timing.

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

**Failure routing:** with `exception={handler-node}`, a failed call (HTTP ≥ 400) sets
`{node}.status`/`{node}.error`, skips the `output[]` mappings, and **jumps to the handler**
instead of aborting — the building block for bounded retry loops
([full pattern](command-reference.md#failure-routing)). Without it, the run aborts.

## graph.extension {#extension}

Delegates to another **graph model** or an **Event Script flow**, so you can compose larger
capabilities and reuse logic.

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
output[]=result -> output.body
```

Worked example (Tutorial 13):

```
create node hello-task
with type Task
with properties
skill=graph.task
task=v1.hello.task
input[]=input.body -> *
output[]=result -> output.body
```

`input[]` entries apply **in order**, so field mappings after a `*` merge into the request body,
and the body auto-converts when the function declares a PoJo input. The result lands at
`{node}.result` and response headers at `{node}.header`. Optional `for_each[]` with `concurrency`
(1–30, default 3) iterates with bounded fork-join; `exception=<node>` routes failures.

**Gotchas:** the `task` route must exist at runtime or the node fails fast; a call is bounded by
`model.ttl` (default 30 s). For multi-step orchestration, prefer [`graph.extension`](#extension) —
`graph.task` is for a single function call. Writing the function itself:
[function AI agent guide](../event-driven/ai-agent-guide.md) (`#[preload]` + `ComposableFunction`).

## graph.join {#join}

A synchronization barrier for parallel branches. It returns `next` **only when all** connected
upstream nodes have completed, and `.sink` (pause) until then.

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
