Skill: Graph Extension
----------------------
Delegates to another graph model (a sub-graph) or to an Event Script flow,
so larger capabilities compose from smaller ones. The node passes named
inputs to the target, and the target's response body becomes this node's
result. This is the seam between the knowledge-graph layer and the Event
Script layer beneath it.

The delegated graph or flow inherits the caller's business correlation ID (model.cid), the same
way an Event Script sub-flow does. A delegated subgraph that suspends therefore persists its
state under the shared business correlation ID scoped by its own graph ID - re-invoking with the
same correlation ID resumes it. This makes a parent graph a natural orchestrator of independently
resumable subgraph paths (see the workflow-suspension guide's orchestrator pattern).

Route name
----------
"graph.extension"

Properties
----------
```
skill=graph.extension
extension={graph-id}           (a deployed sub-graph ...)
extension=flow://{flow-id}     (... or an Event Script flow)
input[]={source} -> {key}
output[]={source} -> {target}
```

- extension (required) - the target. A graph id resolves among DEPLOYED
  graph models only (compiled at startup from the app's resources/graph
  folder - the same ids callable at POST /api/graph/{graph-id}). A session
  draft is NOT addressable: export and deploy it first. A missing id fails
  the node fast at run time. A flow target takes the flow:// prefix, e.g.
  extension=flow://hello-world.
- input[] (required) - each entry's TARGET is a bare key that becomes the
  target's input.body.{key}. There is NO whole-body "*" target on this
  skill - map named keys (the "*" merge idiom is graph.task-only; see
  'help graph-task').
- output[] (optional) - maps the result onward; the result always lands at
  {node}.result regardless.

Optional:

```
for_each[]={array-source} -> model.{var}   (iterate over a runtime list)
concurrency={1-30}                         (parallel fan-out, default 3)
exception={error-handler-node}             (jump on failure instead of abort)
async=true                                 (start the target and continue at once)
progress={text}                            (a node.start event for a watching caller)
```

With progress, a synchronous subgraph returns its own progress events to the
watching caller through this node, naming itself; an asynchronous target never
inherits the caller's return path.

Result set
----------
This node's result namespace IS the target's output.body:

- bare "result" in an output[] mapping is the whole response body
- result.{key} is a field of it

The same contract applies to both target kinds: the named input keys feed
the sub-graph's or flow's input.body, and result.* is its output.body.

Example
-------
```
create node performance-evaluator
with type Extension
with properties
skill=graph.extension
extension=evaluate-sales-performance
input[]=input.body.department_id -> id
output[]=result.sales_performance -> output.body.sales_performance
```

Here input.body.department_id feeds the sub-graph's input.body.id, and the
sub-graph's output.body.sales_performance comes back as
result.sales_performance.

Asynchronous
------------
With async=true the node starts the target and continues to its next
node at once. Use it for work that runs longer than a caller waits: the
parent graph answers (an HTTP 202 advice, for example) while the target
runs to its end, and the target's last node notifies through a route
the use case chooses.

```
create node start-review
with type Extension
with properties
skill=graph.extension
extension=credit-review
async=true
input[]=input.body.application -> application
```

- Nothing comes back, so the node takes no output[]. The deployment gate
  refuses one, and refuses async on any other skill.
- The target inherits the business correlation ID, so its suspensions
  resume under that ID and its trace continues. It runs under its own
  TTL (a flow's ttl, or the TTL of the flow that runs graphs), not the
  parent's; the node's ttl, when declared, becomes the target's TTL.
- exception= covers only a failure to start: a target that does not
  exist (a graph id or a flow), or an input the target graph's root
  schema refuses. A failure inside the target routes to the target's
  own handlers, and the end of each run is logged with its status.
- With for_each, the node starts one run per element and continues at
  once; concurrency bounds how many runs are active, and the next starts
  as an earlier one ends. Each run has its own TTL and its own end, and
  a run that suspends keeps its own record, keyed by its position. Every
  element's input is checked before any run starts. Freeze a derived
  array into model before fanning out (inserting, removing or
  reordering breaks the positions); a suspending graph is never reached
  through a flow launched by for_each.
- One notification for the whole set: start ONE asynchronous subgraph,
  fan out inside it with a synchronous for_each, and notify at its end.
  A parent cannot join runs it does not wait for.

Notes
-----
- Failure routing: on failure, {node}.status and {node}.error are set and
  the output[] mappings are skipped. With exception={handler-node},
  traversal jumps to the handler instead of aborting; without it, the run
  aborts. The jump also stages the generic exception context
  (error.source/code/message and error.stack when available) so ONE
  island-anchored handler can serve every node - error.source is the
  extension node in THIS graph; failures inside the delegated subgraph or
  flow route to that graph's own handlers. The bounded-retry pattern and
  the full error-context contract are shown under 'help graph-api-fetcher'.
- for_each[]={array-source} -> model.{var} invokes the target once per
  element of a runtime list, with bounded parallel fan-out (concurrency
  1-30, default 3). The shared iteration rules are under
  'help graph-api-fetcher'.
- Use graph.extension for multi-step orchestration; use graph.task for a
  single composable-function call.
