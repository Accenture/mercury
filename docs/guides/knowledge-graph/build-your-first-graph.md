# Build Your First Graph

A hands-on walkthrough: build a tiny service that returns a message, enhance it to echo its
input, then deploy it and call it over REST — **no application code, only a graph model**.
You will use the MiniGraph Playground and one skill, `graph.data.mapper`, in about ten
minutes. New to the idea? Read [Knowledge Graph as Application](index.md) first.

## Open the Playground

```bash
cargo run -p minigraph-playground
```

Open <http://127.0.0.1:8100> in a browser. The **console** on the left is where you type
commands (Enter sends; paste a multi-line block, or use Shift+Enter for new lines); the
**Graph** tab on the right renders the model live as you build it. The console's
welcome message shows your session id — you will meet it again in
[Playground & AI Companion](playground-and-companion.md).

!!! note "Rust port"
    The shipped `minigraph-playground` example runs on port **8100** (the Java guide's
    examples use 8085). Like the Java original, the Playground is dev-only — it registers
    only when `app.env=dev`.

## Step 1 — a root and an end

Every graph has a **root** (entry) and an **end** (exit) — those names are reserved. Create
the root first, entering the whole block as one command:

```
create node root
with type Root
with properties
purpose=My first graph
```

```
> create node root...
Graph with 1 node described in /api/graph/model/ws-100001-1/165-1
```

A single node named `root` appears in the Graph tab. Now create the end node and give it a
**skill** — `graph.data.mapper` — so it actually does something. The `mapping[]` entry copies
the constant `hello world` into `output.body`, the response payload (the `[]` suffix means
`mapping` is a list; each `mapping[]=` line appends one entry):

```
create node end
with type End
with properties
skill=graph.data.mapper
mapping[]=text(hello world) -> output.body
```

Connect them so traversal can flow root → end (the label, here `done`, names the
connection):

```
connect root to end with done
```

```
> connect root to end with done
node root connected to end
Graph with 2 nodes described in /api/graph/model/ws-100001-1/551-3
```

## Step 2 — instantiate and run

A graph *model* doesn't run directly — `instantiate graph` creates a runnable instance from
it, and `run` traverses that instance:

```
instantiate graph
```

```
run
```

```
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
> run
Walk to root
Walk to end
Executed end with skill graph.data.mapper in 1.7 ms
{
  "output": {
    "body": "hello world"
  }
}
Graph traversal completed in 9 ms
```

That is a complete, working active knowledge graph: a root, an active node carrying a skill,
and an end — behavior expressed entirely as a model.

## Step 3 — inspect the state machine

After a run, `inspect` reads any value from the instance's state machine — a whole namespace
or a composite key:

```
inspect output
inspect output.body
```

The console prints the value stored at that key. `inspect` works on everything the run
touched — `input.*`, `model.*`, a node's properties, or a skill's `{node}.result` — which
makes it the primary debugging tool as graphs grow. See the
[namespaces](command-reference.md#namespaces) for the full addressing scheme.

## Step 4 — make it echo the input

A real service reacts to input. `update node` has the same shape as `create node` and
replaces the node's definition — map the *request* body to the *response* body instead of a
constant:

```
update node end
with type End
with properties
skill=graph.data.mapper
mapping[]=input.body -> output.body
```

This time, **dry-run** with mock input: `instantiate graph` accepts seed lines, here placing
the constant `it works` into `input.body.message`:

```
instantiate graph
text(it works) -> input.body.message
```

```
run
```

```
> instantiate graph
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
> run
Walk to root
Walk to end
Executed end with skill graph.data.mapper in 0.4 ms
{
  "output": {
    "body": {
      "message": "it works"
    }
  }
}
Graph traversal completed in 2 ms
```

A dry run needs no live dependencies — it is how you validate a model before deploying it.

## Step 5 — export the model

Export the graph to JSON so it can be deployed. The engine writes it to the temp graph
location and stamps the graph's name onto the root node (this prevents accidentally
overwriting a different model later):

```
export graph as my-first-graph
```

```
> export graph as my-first-graph
Added name=my-first-graph to Root node
Graph exported to /tmp/graph/my-first-graph.json
Described in /api/graph/model/my-first-graph/436-4
```

Export fails if any node is an orphan — every node must connect to at least one other
(see [the invariants](command-reference.md#invariants)).

## Step 6 — deploy it and call it over REST

Deployment is a file copy: move the exported JSON into your application's deployed-graph
folder and restart the app (deployed models are compiled at startup). The two locations come
from application configuration:

```yaml
# temp working location (must be file:/ — read/write)
location.graph.temp: file:/tmp/graph
# deployed model location (file:/ or classpath:/ — read-only)
location.graph.deployed: classpath:/graph
```

```bash
cp /tmp/graph/my-first-graph.json resources/graph/
```

A deployed graph is reachable at the generic endpoint `POST /api/graph/{graph-id}`, where the
id is the name you exported:

```bash
curl -X POST http://127.0.0.1:8100/api/graph/my-first-graph \
  -H "Content-Type: application/json" \
  -d '{"message": "it is a wonderful day"}'
```

```json
{"message": "it is a wonderful day"}
```

The graph is not invoked directly — an Event Script flow (`graph-executor`) wraps it, which
is how it gets a REST endpoint without coupling execution to the protocol. You can see the
whole chain in the application's telemetry log; [Composing the Layers](composing-the-layers.md#exposure)
walks through the wiring.

## Where to go next

**The thirteen in-app tutorials.** The Playground ships a complete, progressive course —
type `help tutorial 1` through `help tutorial 13` in the console. They take you from the
hello-world graph you just built (tutorial 1) and deployment (2), through data dictionaries,
providers, and the API fetcher (3), decisions with `graph.math` (4), parallel processing
with a join barrier (5), iterative fetching with `for_each` (6), data mapping in depth (7),
JSON-Path retrieval (8), reusable modules (9), graph extension (10), flow extension (11),
and custom error handling (12), to invoking a composable function with `graph.task` (13).

**Discovery.** Every tutorial also ships as a deployed graph model, so `list graphs` shows
them alongside your own — each with its root node's `purpose`, so the listing reads as living
documentation:

```
> list graphs
Deployed graph models - extension={graph-id} targets:
my-first-graph - My first graph
tutorial-1 - Tutorial one to return a 'hello world' message
...
Total 14 graph models
```

**The reference material.** `help {command}` prints the shipped help for every command used
here (`help create`, `help connect`, `help instantiate`, `help run`, `help export`), and the
[command grammar](command-reference.md) states the full language precisely. When you want an
AI agent to build the next graph *for* you, hand it the
[AI agent guide](ai-agent-guide.md) — and read
[Playground & AI Companion](playground-and-companion.md) to see how you both work on the same
live model.

---

*Adapted from the mercury-composable guide `knowledge-graph/build-your-first-graph.md`; behavior verified against this repository's engine-verified AI documentation.*
