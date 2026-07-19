---
title: MiniGraph command grammar
summary: The authoritative, rule-based reference for the MiniGraph Playground command language —
  lexical rules, namespaces, every command's exact syntax, node types, and the skill→property
  matrix. Designed so a human or an AI agent can generate commands deterministically without
  inferring the grammar from examples.
layer: knowledge-graph
audience: [developer, architect, ai-agent, reference]
keywords: [minigraph, dsl, grammar, command syntax, companion endpoint, deterministic, data mapping]
related:
  - guides/knowledge-graph/skills-reference.md
  - guides/knowledge-graph/ai-agent-guide.md
  - guides/knowledge-graph/minigraph-commands.json
---

# MiniGraph command grammar

> **At a glance**
>
> - This is the **source of truth** for the MiniGraph command language — the rules, not a tour.
>   Tutorials teach by example; this page states the grammar so you (or an AI agent) generate
>   commands **deterministically**, without inferring.
> - A machine-readable form of this grammar lives at
>   [`minigraph-commands.json`](minigraph-commands.json) (ingest it; validate against it).
> - Driving the Playground from an agent? See the [AI agent guide](ai-agent-guide.md).

## Lexical rules {#lexical}

| Element | Rule | Example |
|---|---|---|
| **Node name** | lowercase letters, digits and hyphen | `person-name`, `mdm-profile`, `fetcher-1` |
| **Node type** | a descriptive label; shipped examples **capitalize** structural types | `Root`, `End`, `Provider`, `Dictionary`, `Fetcher`, `Island` |
| **Reserved names** | the root node **must** be named `root`; the end node **must** be `end` | — |
| **Property** | `key=value`; keys may be composite (dot-bracket) | `url=http://...`, `mapping[]=a -> b` |
| **List property** | a `key[]=entry` line *appends* one entry to the list `key` | repeat `mapping[]=...` per entry |
| **Multi-line value** | wrap the value in triple single quotes | `statement[]='''` … `'''` |
| **Constant** | `type(value)` — the closed set in [Constants](#constants) | `text(hello)`, `int(100)`, `boolean(true)` |
| **Mapping operator** | `source -> target` (left = source, right = target) | `input.body.id -> person_id` |
| **Variable substitution** | `{namespace.key}` inside `COMPUTE`/`IF` expressions | `{book.price}`, `{input.body.discount}` |

## Namespaces {#namespaces}

The per-execution **state machine** is addressed through these namespaces (sources and targets in
mappings):

| Namespace | Meaning | Read | Write |
|---|---|---|---|
| `input.body` / `input.header` | the incoming request | ✓ | (seeded at instantiate) |
| `model.*` | intermediate working state | ✓ | ✓ |
| `output.body` / `output.header` | the response returned to the caller | ✓ | ✓ |
| `{node-name}` | a node's own properties | ✓ | ✓ |
| `{node-name}.result` | a skill's output | ✓ | (set by the skill) |
| `{node-name}.status` / `.error` | a skill's execution status | ✓ | (set by the engine) |
| `response.*` | a **data Provider's** raw HTTP response — used in a **Dictionary** node's `output[]` | ✓ (in a dictionary mapping) | (set by the fetch) |
| `result.*` | a **Dictionary/Fetcher** result set | ✓ | (set by the skill) |

**Composite keys & arrays.** Every mapping source and target addresses nested data with the
**dot-bracket** convention — including **numeric list indices**, on the target side too. A numeric
index in a target creates/sets that list slot, which is the natural idiom for assembling a JSON
list deterministically (e.g. after a parallel fork/join):

```
mapping[]=fetch-one.result.profile -> output.body.profile[0]
mapping[]=fetch-two.result.profile -> output.body.profile[1]
```

An **empty index appends**: `[]` in a target adds one element to the end of the list — and
creates the list with that first element when it does not yet exist:

```
output[]=model.fetcher-one -> output.body.profile[]
output[]=model.fetcher-two -> output.body.profile[]
```

Data mapping is **thread-safe** (state-machine operations are serialized), so concurrent `[]`
appends from parallel branches carry **no racing risk** — but their **element order follows
completion order**, which is undetermined across parallel branches. Use `[]` when order does not
matter; use numeric indices (above) when the order must be deterministic.

A **non-leaf (interior) path maps the entire subtree**, not just scalars: a source like
`fetch-one.result.profile` above carries the whole profile object, and `response.accounts` in a
Dictionary mapping carries the whole array.

**An unresolvable source skips the entry** — when a mapping's source key does not exist, the
target is left **untouched** (not nulled). Two idioms for defaults follow from this:
`f:defaultValue(input.body.flag, boolean(false)) -> model.flag`, or default-then-overlay
(`boolean(false) -> model.flag` followed by `input.body.flag -> model.flag`).

## Constants {#constants}

A constant is valid wherever a mapping **source** is (a `mapping[]`/`input[]`/`output[]` source, an
`instantiate graph` seed line). This is the **closed set** — no other constant or coercion form
exists (in particular, the legacy `:type` suffix — "simple type matching" — is **deprecated**; never
generate it. A `:` inside a **Dictionary** node's `input[]` entry is a **default value**, nothing
else — see [Provider & Dictionary](#provider-dictionary)):

| Form | Produces |
|---|---|
| `text(hello world)` | string (verbatim, no quoting needed) |
| `int(100)` / `long(10000000000)` | integer (non-numeric input → `-1`; a decimal part is dropped) |
| `float(1.5)` / `double(1.5)` | floating-point number |
| `boolean(true)` | boolean — `true` only for case-insensitive `true`; anything else is `false` |
| `map(k1=v1, k2=v2)` | inline map literal (values are strings) |
| `map(config.key)` | the value of an application-configuration key |
| `file(text:/tmp/f.txt)` / `file(json:…)` / `file(binary:…)` | file content as text / parsed JSON / bytes |
| `classpath(text:/data/f.txt)` | like `file()`, resolved against the app's resource roots |

Beyond constants, two further **non-constant source forms** are valid in mappings (the graph
skills share Event Script's mapping engine):

| Form | Meaning |
|---|---|
| `f:plugin(args…)` | a [simple-plugin](../event-script/syntax.md#simple-plugins) invocation — the modern replacement for the deprecated `:type` suffixes. Examples: `f:concat(model.a, text(!))`, generators `f:uuid()` and `f:now(text(local))` (current date-time at execution: `iso`/`local`/`ms`), arithmetic `f:add(...)`, logic `f:ternary(...)`, and **list/map reshapers** `f:removeKey(list, text(key))` (strip fields from every map in a list) and `f:listOfMap(...)` (maps-of-lists → list-of-maps). **Full catalog** in the [Event Script syntax page](../event-script/syntax.md#simple-plugins) |
| `$.…` | a JSONPath expression over the state machine (prefer plain dot-bracket keys; JSONPath only when the query needs it) |

## Commands {#commands}

Each command's exact form. Lines shown stacked are a single **multi-line** command (enter as one
block); one-line commands are self-contained.

### create node / update node {#create}

Multi-line. `update node` has the identical shape and replaces a node's definition.

```
create node {name}
with type {type}
with properties
{key}={value}
{key}={value}
```

- `with properties` and the key lines are **optional** (properties act as defaults).
- A node has **zero or one** skill, set with `skill={route}`.
- `{name}` is lowercase letters, digits and hyphen (`root`/`end` reserved); `{type}` is a descriptive label,
  conventionally **Capitalized** (`Root`, `Fetcher`, `Module` — see [lexical](#lexical)).

### connect {#connect}

One-line. **Directional** — `connect a to b` differs from `connect b to a`.

```
connect {node-a} to {node-b} with {relation}
```

The `{relation}` is a **descriptive label** (e.g. `done`, `fetch`, `provider`) — free-form, not
interpreted for skill routing. For data-entity nodes, meaningful relationship names capture
enterprise knowledge.

A node may have **multiple outgoing connections** — traversal **forks into parallel branches**, one
per connection, and the branches execute **concurrently**. Synchronize them with a
[`graph.join`](skills-reference.md#join) barrier node; without one, traversal proceeds as each
branch completes. Data mapping is **thread-safe**, but parallel branches should not write the
**same scalar key** (the last writer wins, nondeterministically) — write to disjoint keys (e.g.
per-branch `model.*` variables), or **append to a shared list with `[]`**, which is race-free
(element order then follows completion order — see
[Composite keys & arrays](#namespaces)).

### delete {#delete}

```
delete node {name}
delete connection {node-a} and {node-b}
```

### instantiate graph {#instantiate}

Multi-line. Creates a runnable instance with optional seeded mock input. **Required before**
`run`, `execute`, or `inspect`. Alias: `start`.

```
instantiate graph
{constant} -> input.body.{key}
{constant} -> input.header.{key}
{constant} -> model.{key}
```

```
instantiate graph
int(100) -> input.body.profile_id
text(application/json) -> input.header.content-type
```

Seed keys may be **composite** (dot-bracket), so nested mock payloads seed directly:

```
instantiate graph
text(Peter) -> input.body.profile.name
text(100 World Blvd) -> input.body.profile.address1
```

### run / execute / inspect {#run}

```
run                        # traverse from root to end
execute {node}             # run a single node (after instantiate)
inspect {namespace.key}    # read a value from the state machine
```

```
inspect output               # a whole namespace: input | output | model
inspect output.body.name     # a specific composite key
```

> **Placeholder convention:** `{…}` in the syntax lines above (e.g. `{node}`,
> `{namespace.key}`) marks a value you substitute — **do not type the braces**.
> Write `inspect output.body.name`, not `inspect {output.body.name}` (a literal
> `{output.body}` is treated as the key `{output` → `body}` and resolves to nothing).

### describe / list / seen {#describe}

```
describe graph
describe node {name}
describe connection {node-a} and {node-b}
describe skill {skill.route}     # prints the shipped help for a skill
list nodes
list connections
seen                             # nodes visited in the last run
```

### export / import {#export}

```
export graph as {name}
import graph from {name}
import node {node} from {name}
```

- `export` writes JSON to `location.graph.temp`; it adds `name={name}` to the root node and
  **fails if any node is an orphan** (every node must connect to ≥1 other).
- The export reply includes `Described in /api/graph/model/{name}/{token}` — a read-only HTTP
  view of the exported model.

### session {#session}

```
session                    # show this session id + subscribers
session subscribe {id}     # mirror another (primary) session's commands into yours
session unsubscribe
session reset
```

The topology subcommands (`subscribe`/`unsubscribe`/`reset`) work only from a
**WebSocket-connected session**. The companion REST endpoints reject them (a companion is an
assistant to a session, not a session of its own) — only the read-only `session` status query is
available there.

### help {#help}

One-line. Prints the engine's shipped help page for a command, a concept, or a skill — the same
content `describe skill {route}` returns for a skill. Useful for in-band self-service when a
detail is not in this grammar (and file an issue when that happens — this page is meant to be
sufficient).

```
help                       # overview
help {command}             # e.g. help connect, help instantiate
help {topic}               # e.g. help data-dictionary, help session
help {skill-topic}         # hyphenated skill form: help graph-api-fetcher, help graph-math
```

- Aliases: `help start` → `help instantiate`; `help clear` → `help delete`.
- An unknown topic returns "not found" — topic names are lowercase (commands by name, skills as
  `graph-…` hyphenated).

## Node types {#node-types}

`root` and `end` are **structural** (entry/exit). All other types are **descriptive labels**
validated by the node's skill, if any. Common conventional types:

| Type | Role | Typical skill |
|---|---|---|
| `Root` / `End` | entry / exit | — / often `graph.data.mapper`; a **skill-less `End`** is valid when an upstream node already shaped `output.body` |
| data-entity | passive data holder | none |
| `Dictionary` | external attribute definition | none (config) |
| `Provider` | external endpoint definition | none (config) |
| `Island` | the **knowledge layer**: links config/data-entity nodes into an entity-relationship view (isolated from traversal, never executed) — see [Island](#island) | `graph.island` |
| (active node) | does work during traversal | a `graph.*` skill |

## Skill → property matrix {#skill-matrix}

Which properties each skill accepts. See the [skills reference](skills-reference.md) for semantics
and examples; this is the at-a-glance contract.

| Skill (`skill=`) | Required | Optional |
|---|---|---|
| `graph.data.mapper` | `mapping[]` | — |
| `graph.math` | `statement[]` (`COMPUTE`/`IF`/`MAPPING`/`EXECUTE`/`RESET`) | `for_each[]`, `NEXT:`, `DELAY:`, `BEGIN`/`END` |
| ~~`graph.js`~~ | ⚠️ **retired in this Rust port** (security) — use `graph.math` or `graph.task` | — |
| `graph.api.fetcher` | `dictionary[]` (+ `input[]` whenever its dictionaries declare parameters — the usual case) | `output[]` (the result set always lands at `{node}.result` for a later mapper), `for_each[]` (see [Iterative fetching](#for-each)), `concurrency` (1–30, def 3), `exception` |
| `graph.extension` | `extension` (`{graph-id}` or `flow://{flow-id}`), `input[]` | `output[]`, `for_each[]`, `concurrency`, `exception` |
| `graph.task` | `task` (a composable function's route name) | `input[]`, `output[]`, `for_each[]`, `concurrency`, `exception` |
| `graph.join` | — | — |
| `graph.island` | — | — |

Configuration nodes used by `graph.api.fetcher` — full authoring rules in
[Provider & Dictionary](#provider-dictionary):

| Node | Properties |
|---|---|
| **Provider** | `url`, `method`, `feature[]`, `input[]` (targets: `header.*`, `query.*`, `path_parameter.*`, `body.*`) |
| **Dictionary** | `provider`, `input[]` (**bare** parameter names), `output[]` (`response.*` → `result.*`) |

## Provider & Dictionary — the data-dictionary method {#provider-dictionary}

`graph.api.fetcher` never holds a URL itself. It names one or more **Dictionary** nodes (data
attributes); each Dictionary names the **Provider** node (endpoint definition) that supplies it.
Both are **config nodes**: they never execute and are referenced **by name** — but they must
**not** be left floating. Wire them into the knowledge structure under a `graph.island` node
(see [Island — the knowledge layer](#island)): the island subgraph is the graph's
entity-relationship diagram, and **no node is left unconnected**.

**Provider** — defines the HTTP call:

```
create node {name}
with type Provider
with properties
purpose={description}
url={target url}
method={GET | POST | PUT | PATCH | DELETE | HEAD}
feature[]={feature flag}
input[]={source} -> {target}
```

- The `url` may embed **`{name}` path placeholders** — each one is filled by an `input[]` line
  targeting `path_parameter.{name}`. Standard `${config.key:default}` substitution also applies
  (e.g. `url=http://127.0.0.1:${rest.server.port:8080}/api/mdm/profile/{id}`).
- `input[]` **source**: a [constant](#constants), a Dictionary **parameter name** (bare), or a
  state-machine value (`model.*`). **Target**: `header.{name}`, `query.{name}`,
  `path_parameter.{name}`, `body.{key}` — or the whole `body` (e.g. to send a string or array as
  the request body).
- `feature[]` entries declare capabilities the calling fetcher must support (e.g. an auth
  mechanism). Built-ins: `log-request-headers` / `log-response-headers` — the fetcher logs the
  request/response headers into its node's `header` section. An unsupported feature produces a
  warning from `graph.api.fetcher` (a custom fetcher may enforce it).

**Dictionary** — defines one data attribute retrievable through a Provider:

```
create node {name}
with type Dictionary
with properties
purpose={description}
provider={provider-node-name}
input[]={parameter}
input[]={parameter}:{default}
output[]=response.{path} -> result.{key}
```

- `input[]` entries are **bare parameter names**, *not* `source -> target` mappings (the one
  exception to the mapping rule). An optional `:{default}` suffix supplies a default value
  (`input[]=detail:true`) — that is the **only** meaning of `:` here.
- `output[]` maps the Provider's raw HTTP response body (the **`response.*`** namespace) into the
  **result set** (`result.{key}`). The result set is what a fetcher exposes: as the `result.*`
  source inside its own `output[]` mappings, and as `{fetcher-node}.result` to later nodes.
  The source path may be a **leaf or an interior node** — an interior path maps the **whole
  subtree**: `response.profile.name -> result.name` extracts one field, while
  `response.profile -> result.profile` captures the entire profile object and
  `response.accounts -> result.account_numbers` an entire array.
- A fetcher's `input[]` **targets must match the dictionary parameter names** exactly, or execution
  fails. Several Dictionary nodes may share one Provider; identical calls (same provider + same
  input values) are **deduplicated** into a single HTTP request.

Worked example — fetch a person's profile by id (path parameter + JSON accept header), then expose
name and address:

```
create node mdm-profile
with type Provider
with properties
purpose=MDM profile endpoint
url=http://127.0.0.1:${rest.server.port:8080}/api/mdm/profile/{id}
method=GET
input[]=text(application/json) -> header.accept
input[]=person_id -> path_parameter.id
```

```
create node person-profile
with type Dictionary
with properties
purpose=full profile record of a person
provider=mdm-profile
input[]=person_id
output[]=response.profile.name -> result.name
output[]=response.profile.address -> result.address
```

```
create node fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=person-profile
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
```

Second worked example — a **POST** Provider: `body.{key}` targets build the JSON request body
(set `content-type`; there is no URL placeholder — the parameters travel in the body):

```
create node account-api
with type Provider
with properties
purpose=account management endpoint
url=http://127.0.0.1:${rest.server.port:8080}/api/account/details
method=POST
input[]=text(application/json) -> header.accept
input[]=text(application/json) -> header.content-type
input[]=person_id -> body.person_id
input[]=account_id -> body.account_id
```

## Iterative fetching — `for_each` {#for-each}

A `graph.api.fetcher` node can execute **once per element of a runtime array** — the mechanism for
"fetch details for each item in a list obtained from a previous call":

```
create node accounts-fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=account-detail
for_each[]=profile-fetcher.result.accounts -> model.account_id
concurrency=3
input[]=input.body.person_id -> person_id
input[]=model.account_id -> account_id
output[]=result.detail -> model.account_details
```

- `for_each[]={array-source} -> model.{var}` — the source **must resolve to a list**; it is
  typically a **prior fetcher's result** (`{fetcher}.result.{key}` — the cross-node `.result`
  namespace) or any `model.*` array. Multiple `for_each[]` lines iterate multiple parameters in
  lock-step.
- Wire the current element into each call with an ordinary input mapping:
  `input[]=model.{var} -> {dictionary-parameter}`. Non-iterated inputs (like `person_id` above)
  are passed unchanged to every call.
- `concurrency` bounds the parallel fan-out (1–30, default 3): the calls run in batches of that
  size.
- **Aggregation (guaranteed):** each iteration's `result.{key}` values are **appended into a
  single array** on this node's result set — after N iterations, `result.detail` above is an
  array of N. **Order is deterministic**: batches execute in input-list order and responses join
  in request order, so the aggregated array preserves the source array's order regardless of
  `concurrency`.
- Identical requests are still deduplicated into one HTTP call.

## Failure routing — `exception=` {#failure-routing}

`graph.api.fetcher`, `graph.task`, and `graph.extension` accept an optional
`exception={handler-node}` property. On a **failed** call (HTTP status ≥ 400, or a task error):

- the node's `{node}.status` and `{node}.error` are set (the engine's error record);
- the node's `output[]` mappings are **skipped**;
- traversal **jumps to the named handler node** instead of aborting. Without `exception=`, the
  run **aborts** on failure.

Wire the handler back explicitly (e.g. `connect error-handler to fetcher with retry`) — no node
left unconnected. The canonical **bounded-retry** pattern combines the pieces (see the
[statement grammar](#math-statements)):

```
create node error-handler
with type Decision
with properties
skill=graph.math
statement[]=RESET: fetcher, error-handler
statement[]=MAPPING: f:defaultValue(model.attempts, int(0)) -> model.attempts
statement[]=MAPPING: f:add(model.attempts, int(1)) -> model.attempts
statement[]='''
IF: {model.attempts} >= 3
THEN: recovery-node
ELSE: next
'''
statement[]=NEXT: fetcher
statement[]=DELAY: 50
```

The handler **resets the fetcher and itself first** (the placement rule above — it then runs on
every path, including the recovery jump), counts attempts (`f:defaultValue` + `f:add` on the
`model.*` namespace, which `RESET` never touches), exits to a recovery node at the bound (a taken
`IF` jump ends the list), otherwise jumps back and paces the retry with a delay — staying under
the engine's loop guard. If the handler also carries a defensive check on the failed node's
status, that check must come **before** the `RESET` (it reads state the reset wipes).

## Island — the knowledge layer (required) {#island}

A `graph.island` node is **isolated from graph traversal** — it executes only to sink (the run
log shows one `Executed … with skill graph.island` line), so traversal never continues through it.
It is **not optional decoration**: the island subgraph is the graph's
**entity-relationship diagram**. Connecting Dictionary, Provider, data-entity, and reusable
**module** nodes under an island turns the graph into **living documentation of enterprise
knowledge** — a new joiner (or an agent) reads the connected dictionaries, entities, and modules
to discover the domain model, not just the execution path.

**Convention: leave no node unconnected.** The island is **required** whenever the graph has
off-path nodes — config (Dictionary, Provider), data-entity, or reusable
[module](#math-statements) nodes — wire every one of them into the knowledge structure. For a
graph with none (e.g. a pure transformation), an island is **encouraged**: adding data-entity
nodes that document the domain model (entities, fields, which fields are internal-only) turns
even a small graph into discoverable enterprise knowledge.

```
create node dictionary
with type Island
with properties
skill=graph.island
```

```
connect root to dictionary with contains
connect dictionary to person-profile with data
connect dictionary to account-detail with data
connect person-profile to mdm-profile with provider
connect account-detail to account-api with provider
```

The relation labels are descriptive (free-form) — choose names that capture the real-world
relationship; `contains` / `data` / `provider` are the shipped conventions. Traversal is
unaffected: the island sinks, so the execution path never enters the knowledge layer.

## `graph.math` statement grammar {#math-statements}

> **`graph.js` is retired in this Rust port** (disabled for security — the runtime rejects it with
> *"Skill graph.js is retired for security reasons - use graph.math or graph.task instead."*). Use
> `graph.math` for inline compute/branch, or `graph.task` for anything richer. Its expression dialect
> is a narrow JS-like subset — arithmetic/comparison/boolean operators only, **no bitwise ops, no
> function calls, no variables**; `COMPUTE` yields a double (integers serialize as e.g. `8.0`).

A `graph.math` node runs an ordered list of `statement[]` lines. Five statement types:

| Statement | Form | Purpose |
|---|---|---|
| `COMPUTE` | `COMPUTE: {var} -> {expr}` | evaluate a JS-like math/boolean expression; the result is stored in **this node's `result` namespace** — read it back as `{this-node}.result.{var}` or move it with `MAPPING` |
| `IF` | multi-line (see below) | a boolean **decision** that redirects traversal to a named node |
| `MAPPING` | `MAPPING: source -> target` | data mapping, identical to `graph.data.mapper` (**no** `{}` around source/target) |
| `EXECUTE` | `EXECUTE: {node-name}` | run another `graph.math` node's statements inline, **in the calling node's context** — any `COMPUTE` results land in the **invoking** node's result namespace (`{invoker}.result.{var}`); the executed module's own namespace stays empty. This is the **module-reuse mechanism**: author a formula once in an off-path module node, and any executing node borrows it (see the note below) |
| `RESET` | `RESET: {node-name}` | forget a node completely — guard, completion mark, state — so it can execute again (advanced; see the rules below) |

Expressions use `{namespace.key}` substitution (`{input.body.a}`, `{book.price}`, `{model.x}`) —
the `{…}` substitution syntax is robust to hyphenated names (`{unit-price}` is the value of
`unit-price`, never a subtraction), so use communicative hyphenated names freely. A node
with **only** `MAPPING` statements is rejected — use `graph.data.mapper` instead. Statements run in order.

**Reusable modules.** A `graph.math` node can serve as a governed **library module**: author the
formula once, reading neutral `model.*` operands, keep the node **off the execution path**, and
let any traversal node borrow it with `EXECUTE`. The caller marshals inputs into the module's
expected `model.*` keys, executes, then maps **its own** result out:

```
create node addition             # the library — authored once, not traversed
with type Module
with properties
skill=graph.math
statement[]=COMPUTE: sum -> {model.a} + {model.b}
```

```
create node compute              # the execution-path caller
with type Compute
with properties
skill=graph.math
statement[]=MAPPING: input.body.a -> model.a
statement[]=MAPPING: input.body.b -> model.b
statement[]=EXECUTE: addition
statement[]=MAPPING: compute.result.sum -> output.body.sum
```

Note `compute.result.sum` — **not** `addition.result.sum`: the caller borrows the logic, so the
result belongs to the caller. Hang the module under the [Island knowledge layer](#island)
(`island -[module]-> addition`) so it is documented and no node is left unconnected.

**`IF` is a multi-line statement — this is the decision construct.** An `IF` **must** be paired with
`THEN:` and `ELSE:`, or the engine aborts the run (`node {name} does not have if:, then: or else:`):

```
IF: <boolean expression>
THEN: <node-name> | next
ELSE: <node-name> | next
```

- `THEN:` / `ELSE:` each name the **node to jump to**, or the keyword **`next`** (fall through to the
  natural graph traversal / next statement).
- **A taken node-jump ends the statement list immediately** — statements after it do not run. A
  branch that resolves to `next` **falls through**: processing continues with the following
  statements. Order the list accordingly (e.g. an early-exit check first; the retry logic after).
- Append the whole triad as one multi-line value with `'''` … `'''` (see [lexical](#lexical)).

Worked example — compute the sum, then branch on a comparison so each branch fills the response:

```
skill=graph.math
statement[]=COMPUTE: sum -> {input.body.a} + {input.body.b}
statement[]='''
IF: {input.body.a} >= {input.body.b}
THEN: ge-path
ELSE: lt-path
'''
```

**Traversal-control keywords** — these are ordinary `statement[]` lines (e.g.
`statement[]=NEXT: fetcher`, `statement[]=DELAY: 500`), conventionally placed last:

- `NEXT: {node-name}` — unconditionally jump to a node **by name** (it takes a *node name*, **not** a
  connection/relation label). Unlike a taken `IF` jump, `NEXT:` does **not** stop processing: the
  remaining statements still run, and the jump is applied **after the whole list completes**
  (the last `NEXT:` wins).
- `RESET: {node-name}[, {node-name} …]` — clear the run-once guard, the **completion mark**, and
  the **state** of one or **more** nodes (comma/space-separated list). The completion mark matters
  for [join barriers](skills-reference.md#join): a reset (retrying) branch stops satisfying the
  barrier until it re-executes successfully — and a branch that **failed** into its `exception=`
  route never satisfies it in the first place (completion is success-only). Resetting a never-executed node is a safe no-op. A node may
  reset **itself** — the run-once mark is set *before* execution, so a self-reset survives and the
  node can run again. **Placement rule: put `RESET` first among the action statements** — it then
  runs on every path (a later taken `IF` jump would skip it) and everything the node stores
  afterwards (such as `DELAY:`'s pending pause) survives the self-wipe. The one exception: keep it
  **after** any statement that reads state it would wipe — an `IF` on a just-wiped variable (e.g.
  `{fetcher.status}` after `RESET: fetcher`) **aborts the run**, so a defensive status check goes
  before the `RESET`. This enables retry loops — see [Failure routing](#failure-routing) — but
  mind the engine's **loop guard**: a node executed too frequently (default >10 visits/second)
  aborts the traversal, so bound every retry loop and pace it with `DELAY:`.
- `BEGIN` / `END` — delimit the loop body for **`for_each[]`** iterative execution (they are
  **not** `IF`-block braces) — see [for_each](#math-for-each).
- `DELAY: {milliseconds}` — pause **after this node completes**, deferring the walk to the next
  node (paces retries; simulates a slow service).

### `for_each[]` — iterate a statement block over lists {#math-for-each}

The optional node property `for_each[]` turns part of the statement list into a **loop**. Each
entry has the mapping form `source -> model.{var}` — the RHS **must** be a `model.*` key:

- a **list-valued** source becomes an **iteration array**: its `model.{var}` is rebound to
  element *i* on each pass. Multiple list entries advance **in lockstep** (parallel arrays) and
  must all have the **same length** (engine error otherwise). At least one entry must resolve
  to a list, or the node aborts (*"No data mapping resolved from 'for_each' entries. LHS must
  be a list."*).
- a **scalar** source binds its `model.{var}` **once**, at resolution time — before the loop
  runs, even when the lists are empty.
- an **unresolvable** source **removes** the `model.{var}` key (not left stale, not nulled).

`BEGIN` / `END` lines split `statement[]` into three blocks:

```
statement[]=…      ← pre-block: runs ONCE, before the loop
statement[]=BEGIN
statement[]=…      ← each-block: runs once PER ELEMENT
statement[]=END
statement[]=…      ← post-block: runs ONCE, after the loop
```

Rules (engine-verified):

- **No `BEGIN` ⇒ the whole statement list is the loop body.** Seed accumulators in a pre-block,
  or the seeding re-runs on every iteration.
- Iteration is **strictly sequential, in list order** — element 0 completes before element 1
  starts. (Contrast: the [fetcher's `for_each`](#for-each) fans HTTP calls out concurrently and
  only *aggregates* in order.) The whole loop is **one node execution**, so a long list does not
  trip the traversal loop guard.
- **A taken `IF` jump breaks the loop:** it ends the current iteration immediately, skips the
  remaining elements **and the post-block**, and routes traversal to the named node. In the
  pre-block it skips the loop and post-block the same way. (A `NEXT:` exits too, after its block
  completes.) An `ELSE: next` falls through to the rest of the iteration.
- **Empty lists are fine:** the each-block runs zero times; the pre- and post-blocks still run.
- **Number dialect:** `COMPUTE` yields **doubles**; the `f:add`/`f:subtract`/… simple plugins
  use **numeric promotion** — all-whole-number inputs keep exact long arithmetic (including
  integer division and the classic integer counters of
  [Failure routing](#failure-routing)), while **any** floating-point argument promotes the
  whole computation to a double. So `f:add` composes directly with `COMPUTE` results and
  decimal API data; accumulate with either `f:add` or a pure-`COMPUTE` read-back (both shown
  below). Tame floating-point artifacts with **`f:round(value, int(2))`** — half-up rounding
  applied to the number's decimal representation (`1.005` → `1.01` at 2 places).
- `EXECUTE:` inlining happens **before** the blocks are split, so an executed module's
  statements land at the `EXECUTE:` position and may contribute to (or delimit) the loop body.
- Without `for_each[]`, `BEGIN`/`END` lines are accepted and ignored.

Worked example (engine-verified) — line totals with a running sum:

```
create node totaler
with type Loop
with properties
skill=graph.math
for_each[]=input.body.prices -> model.price
for_each[]=input.body.quantities -> model.qty
statement[]=MAPPING: int(0) -> model.total
statement[]=BEGIN
statement[]=COMPUTE: total -> {model.total} + {model.price} * {model.qty}
statement[]=MAPPING: totaler.result.total -> model.total
statement[]=END
statement[]=MAPPING: model.total -> output.body.total
```

With `prices=[10,20,30]` and `quantities=[7,8,9]` the run yields `total: 500.0`: the pre-block
seeds the accumulator once, each pass computes `total + price*qty` and writes it back to
`model.total`, and the post-block maps the final value out. The plugin form is equivalent —
`COMPUTE: line -> {model.price} * {model.qty}` then
`MAPPING: f:add(model.total, totaler.result.line) -> model.total` — numeric promotion carries
the `COMPUTE` doubles through `f:add` (both forms are engine-verified).

## Invariants {#invariants}

Hard rules the engine enforces — violate them and generation fails:

1. The root node is named `root`; the end node is named `end`.
2. A node has **0 or 1** skill.
3. Node **names** are **lowercase letters, digits and hyphen** (`root`/`end` reserved). Node **types** are
   descriptive labels — shipped examples capitalize structural types (see [lexical](#lexical)).
4. Every node **in the traversal path** must connect to ≥1 node, or `export` fails.
   `Dictionary` and `Provider` configuration nodes are referenced *by name* (`dictionary[]=`,
   `provider=`) and are **not traversed** — but the convention is still **no node left
   unconnected**: wire them under a `graph.island` node
   (`root -[contains]-> island -[data]-> dictionary -[provider]-> provider`) so the graph carries
   the entity-relationship knowledge — see [Island](#island).
5. A node is **executed once** per run (loop guard); a `graph.math` `RESET` statement is the only
   escape, for advanced re-execution.
6. `instantiate graph` must precede `run` / `execute` / `inspect`.

## See also {#see-also}

- [`minigraph-commands.json`](minigraph-commands.json) — the machine-readable form of this grammar.
- [AI agent guide](ai-agent-guide.md) — driving the Playground via the companion endpoint.
- [Built-in skills reference](skills-reference.md) — per-skill semantics and examples.
- The mapping syntax (`source -> target`) and [constant set](#constants) are shared with
  [Event Script](../event-script/syntax.md#tasks-and-data-mapping); this page is self-contained —
  the constants above are the full set.
- [Event Script flow grammar](../event-script/flow-grammar.md) — authoring the flows a
  `graph.extension` node calls via `flow://{flow-id}`; [function AI agent guide](../event-driven/ai-agent-guide.md)
  — the composable functions a `graph.task` node calls.
