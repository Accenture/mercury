# Knowledge Graph as Application

Layer 3 — the **active knowledge graph**, the semantic layer at the top of the platform. At
this altitude the model stops *describing* the application and **becomes** it: intent,
knowledge, and behavior live in one executable graph, and running the application means
**traversing the model**.

[Layer 1](../event-driven/index.md) pushed coupling out of code — functions are addressed
only by route name, wired by events. [Layer 2](../event-script/index.md) pushed orchestration
out of code and into YAML flows. The knowledge graph is the next step on that ascent: instead
of writing a flow that *calls* functions, you describe what the system *knows* — entities,
relationships, data sources, decisions — as a graph, and attach a **skill** to each node that
needs to *do* something. Changing behavior means editing the model, validating it in a
dry run, and deploying the updated JSON — not rewriting and redeploying code.

## The property graph underneath

The engine builds on a deliberately **minimalist, in-memory property graph** — the
*mini-graph*. Three concepts carry the whole data model:

**Node**
:   A data entity, addressable by a unique **alias** name. Every node carries at least one
    **type** — a descriptive label such as `Root`, `Person`, or `Provider` — and an optional
    set of properties.

**Connection**
:   A **unidirectional** link between two nodes (a bidirectional link is simply two
    connections). A connection may carry zero or more **relations**, each with exactly one
    relationship type and its own properties.

**Properties**
:   Untyped key-values on a node or a relation — primitives or whole objects, as the model
    needs.

The minimalism is the point. A knowledge graph sized in the hundreds of nodes (default
capacity: 750) fits entirely in memory, so traversal, comparison, and decision-making run at
memory speed — none of the network latency of a graph database — and the whole model stays
small enough to render on one screen in the Playground and to hold in one reader's head. A
few hundred well-named nodes represent a surprisingly rich knowledge base. This is *not* a
general-purpose graph database: the graph drives execution and decisioning, not
storage-scale querying.

## Skills make nodes active

A traditional property graph only holds data. The active knowledge graph adds one thing: a
node may carry a `skill` property naming a **composable function**. That node becomes an
**active node**.

- Nodes **without** a skill are *passive* — they hold knowledge and are traversed but never
  executed.
- Nodes **with** a skill are *active* — when traversal reaches them, the engine invokes the
  skill.

Because a skill is just a composable function addressed by route name, the platform's
defining invariant holds at this layer too: the graph couples to its behavior through
**names and events**, never direct references. A node has at most **one** skill. Eight
skills ship with the engine:

| Skill | Use it to… |
|---|---|
| [`graph.data.mapper`](skills-reference.md#data-mapper) | copy and transform data between namespaces |
| [`graph.math`](skills-reference.md#math) | compute and branch with fast inline math/boolean expressions |
| [`graph.js`](skills-reference.md#js) | *(retired in this port — see below)* |
| [`graph.api.fetcher`](skills-reference.md#api-fetcher) | call external HTTP APIs declaratively |
| [`graph.extension`](skills-reference.md#extension) | delegate to a sub-graph or an Event Script flow |
| [`graph.task`](skills-reference.md#task) | invoke a composable function through its route name |
| [`graph.join`](skills-reference.md#join) | synchronize parallel traversal branches |
| [`graph.island`](skills-reference.md#island) | anchor the knowledge layer (see below) |

Per-skill syntax and worked examples live in the
[built-in skills reference](skills-reference.md); in the Playground, `describe skill {name}`
prints the same shipped help.

!!! note "Rust port"
    In the Java engine, `graph.js` evaluates full JavaScript on GraalVM. **This port retires
    it for security** — the runtime rejects it with *"Skill graph.js is retired for security
    reasons - use graph.math or graph.task instead."* Use `graph.math` for inline
    compute/branch and `graph.task` for anything richer.

## How a graph executes

Execution is event-driven, not a blocking tree-walk. The engine seeds a per-instance **state
machine** with the request, then walks from the `root` node toward the `end` node. At each
active node it invokes the skill, records the outcome at `{node}.result` (and `.status` /
`.error`), and routes by the skill's **decision**: `next` follows the outgoing connection, a
**node name** jumps — this is how branching works — and `.sink` pauses the path (joins and
islands). A node with multiple outgoing connections **forks parallel branches**, synchronized
by a `graph.join` barrier; built-in loop detection guards against runaway traversal. Reaching
`end` returns the response from the `output` namespace.

The state machine is the shared workspace across stateless skills:

| Namespace | Holds |
|---|---|
| `input.body` / `input.header` | the incoming request |
| `model.*` | intermediate working state |
| `{node}.result` / `.status` / `.error` | each skill's output and execution status |
| `output.body` / `output.header` | the final response |

The full addressing rules — composite keys, list indices, constants, mapping semantics — are
in the [command grammar](command-reference.md#namespaces).

## The island knowledge layer

Not everything in the model is on the execution path. `Dictionary` and `Provider` nodes
configure external data sources by *name*; data-entity nodes describe the domain; reusable
module nodes hold governed logic. The convention — enforced at export as *no node left
unconnected* — is to wire all of them under a [`graph.island`](skills-reference.md#island)
node, which is isolated from traversal by design.

That island subgraph is the graph's **entity-relationship diagram**: the same model that
executes the service also documents the entities, attributes, endpoints, and modules it
depends on, with meaningful relationship names. The graph becomes **living documentation of
enterprise knowledge** — a new team member (or an AI agent) reads the connected dictionaries
and entities to discover the domain model, not just the execution path. The wiring rules are
in [Island — the knowledge layer](command-reference.md#island).

## Composing the layers

An active knowledge graph is not an island itself — it composes the layers beneath it without
coupling. `graph.extension` delegates to another deployed graph (`extension={graph-id}`) or
crosses into Layer 2 with an Event Script flow (`extension=flow://{flow-id}`); `graph.task`
invokes a single composable function by route name; `graph.api.fetcher` reaches external
systems declaratively; and a deployed graph is exposed over REST at
`POST /api/graph/{graph-id}` by an Event Script flow, keeping protocol decoupled from
execution. [Composing the Layers](composing-the-layers.md) tells this story end to end.

## In this section

- **[Build Your First Graph](build-your-first-graph.md)** — the hands-on walkthrough: open
  the Playground, build a hello-world graph one command at a time, dry-run it, export it,
  deploy it, and call it over REST.
- **[Playground & AI Companion](playground-and-companion.md)** — the interactive workbench,
  collaborative sessions, and the companion endpoints that let an AI agent co-author a graph
  in a live session while humans watch in real time.
- **[Composing the Layers](composing-the-layers.md)** — sub-graph and flow delegation,
  composable-function tasks, external data sources, discovery commands, and REST exposure.
- **The AI documentation set** — the [AI agent guide](ai-agent-guide.md), the
  [command grammar](command-reference.md) (with its machine-readable form,
  [`minigraph-commands.json`](minigraph-commands.json)), and the
  [skills reference](skills-reference.md). These are **engine-verified**: in a
  thirteen-tutorial validation sweep, fresh AI agents given only these documents built and
  ran every tutorial graph — the last eight tutorials passing on the first attempt with zero
  out-of-band lookups. If an AI agent builds graphs for you, point it at
  [`docs/llms.txt`](../../llms.txt) and it will find its own way here.

## See also

- [Getting Started](../getting-started.md) — build the workspace and take the Playground for
  a spin.
- [Composable Orchestration](../event-script/index.md) — Layer 2, the flow engine that
  `graph.extension` bridges to and that exposes graphs over REST.
- [Event-Driven Foundation](../event-driven/index.md) — Layer 1, where the functions behind
  every skill live.

---

*Adapted from the mercury-composable guides `knowledge-graph/index.md` and `knowledge-graph/property-graph.md`; behavior verified against this repository's engine-verified AI documentation.*
