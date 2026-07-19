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
| **Node name** | lowercase letters and hyphen only | `person-name`, `mdm-profile` |
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
- `{name}` and `{type}` are lowercase-and-hyphen; `root`/`end` are reserved (see [lexical](#lexical)).

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
branch completes. Parallel branches must write to **disjoint** state keys (e.g. per-branch
`model.*` variables) to avoid stepping on each other.

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
| `Root` / `End` | entry / exit | — / often `graph.data.mapper` |
| data-entity | passive data holder | none |
| `Dictionary` | external attribute definition | none (config) |
| `Provider` | external endpoint definition | none (config) |
| `Island` | groups configuration / isolated nodes (not executed) | `graph.island` |
| (active node) | does work during traversal | a `graph.*` skill |

## Skill → property matrix {#skill-matrix}

Which properties each skill accepts. See the [skills reference](skills-reference.md) for semantics
and examples; this is the at-a-glance contract.

| Skill (`skill=`) | Required | Optional |
|---|---|---|
| `graph.data.mapper` | `mapping[]` | — |
| `graph.math` | `statement[]` (`COMPUTE`/`IF`/`MAPPING`/`EXECUTE`/`RESET`) | `for_each[]`, `NEXT:`, `DELAY:`, `BEGIN`/`END` |
| ~~`graph.js`~~ | ⚠️ **retired in this Rust port** (security) — use `graph.math` or `graph.task` | — |
| `graph.api.fetcher` | `dictionary[]`, `input[]`, `output[]` | `for_each[]`, `concurrency` (1–30, def 3), `exception` |
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
Both are **config nodes**: they never execute, are referenced **by name**, and need **no**
connections (group them under a `graph.island` node purely for visual organization if you wish).

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
| `EXECUTE` | `EXECUTE: {node-name}` | run another `graph.math` node inline |
| `RESET` | `RESET: {node-name}` | clear a node's run-once guard so it can execute again (advanced) |

Expressions use `{namespace.key}` substitution (`{input.body.a}`, `{book.price}`, `{model.x}`). A node
with **only** `MAPPING` statements is rejected — use `graph.data.mapper` instead. Statements run in order.

**`IF` is a multi-line statement — this is the decision construct.** An `IF` **must** be paired with
`THEN:` and `ELSE:`, or the engine aborts the run (`node {name} does not have if:, then: or else:`):

```
IF: <boolean expression>
THEN: <node-name> | next
ELSE: <node-name> | next
```

- `THEN:` / `ELSE:` each name the **node to jump to**, or the keyword **`next`** (fall through to the
  natural graph traversal / next statement).
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

**Traversal-control keywords** (optional; conventionally placed last):

- `NEXT: {node-name}` — unconditionally jump to a node **by name** (it takes a *node name*, **not** a
  connection/relation label).
- `BEGIN` / `END` — delimit a statement block for **`for_each[]`** iterative execution; they are
  **not** `IF`-block braces.
- `DELAY: {milliseconds}` — defer completion (e.g. to simulate a slow service).

## Invariants {#invariants}

Hard rules the engine enforces — violate them and generation fails:

1. The root node is named `root`; the end node is named `end`.
2. A node has **0 or 1** skill.
3. Node **names** are **lowercase + hyphen** only (`root`/`end` reserved). Node **types** are
   descriptive labels — shipped examples capitalize structural types (see [lexical](#lexical)).
4. Every node **in the traversal path** must connect to ≥1 node, or `export` fails. **Exception:**
   `Dictionary` and `Provider` configuration nodes are referenced *by name* (`dictionary[]=`,
   `provider=`) — not traversed — and need **no** connections; optionally group them under a
   `graph.island` node purely for organization.
5. A node is **executed once** per run (loop guard); a `graph.math` `RESET` statement is the only
   escape, for advanced re-execution.
6. `instantiate graph` must precede `run` / `execute` / `inspect`.

## See also {#see-also}

- [`minigraph-commands.json`](minigraph-commands.json) — the machine-readable form of this grammar.
- [AI agent guide](ai-agent-guide.md) — driving the Playground via the companion endpoint.
- [Built-in skills reference](skills-reference.md) — per-skill semantics and examples.
- The mapping syntax (`source -> target`) and [constant set](#constants) are shared with Event
  Script; this page is self-contained — the constants above are the full set.
