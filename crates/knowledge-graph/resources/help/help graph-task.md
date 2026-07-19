Skill: Graph Task
-----------------
Invokes a composable function through its route name and collects the
function's response into the node's "result" property. This is the
lightweight way to plug a small piece of custom business logic into a graph:
your own function becomes, in effect, a custom skill. For multi-step
orchestration, prefer a sub-graph or an Event Script flow via graph.extension
(see 'help graph-extension').

Route name
----------
"graph.task"

Properties
----------
```
skill=graph.task
task={function-route}
input[]={source} -> {target}
output[]={source} -> {target}
```

- task (required) - the route name of a composable function registered in
  this application.
- input[] (optional) - maps values into the function's request; entries
  apply IN ORDER (see below).
- output[] (optional) - maps the function's result onward; the result always
  lands at {node}.result regardless.

Optional:

```
for_each[]={array-source} -> model.{var}   (iterate over a runtime list)
concurrency={1-30}                         (parallel fan-out, default 3)
exception={error-handler-node}             (jump on failure instead of abort)
```

Input data mapping
------------------
input[] entries apply in order. The target addresses the function's request:

- "*" - the mapped value is MERGED into the whole request body. Because
  entries apply in order, later entries can merge additional fields into a
  body seeded with "*".
- header.{name} - sets a request header of the function call.
- any other key - sets that field in the request body (composite dot-bracket
  keys supported).

Sources are the usual mapping sources: input.*, model.*, another node,
constants such as text(hello), or f:plugin(...) calls. If the function
declares a typed (PoJo) input, the request body converts automatically at
the function boundary.

Output data mapping
-------------------
The function's response body is stored at {node}.result, the response status
at {node}.status, and response headers at {node}.header. In output[]
mappings, bare "result" is the WHOLE function result; result.{key} is a
field of it (same rule as graph.extension):

```
output[]=result -> output.body
output[]=result.total -> model.total
```

Example
-------
```
create node hello-task
with type Task
with properties
skill=graph.task
task=v1.hello.task
input[]=input.body -> *
input[]=text(minigraph) -> header.x-app
output[]=result -> output.body
```

Notes
-----
- The task route must exist at runtime, or the node fails fast.
- A call is bounded by the graph instance's time-to-live (model.ttl,
  default 30000 ms).
- Failure routing: on a function error (or timeout), {node}.status and
  {node}.error are set and the output[] mappings are skipped. With
  exception={handler-node}, traversal jumps to the handler instead of
  aborting; without it, the run aborts. The bounded-retry pattern is shown
  under 'help graph-api-fetcher'.
- for_each[]={array-source} -> model.{var} invokes the function once per
  element of a runtime list (the source must resolve to a list; wire the
  element in with an ordinary input[] mapping from model.{var}), with
  bounded parallel fan-out (concurrency 1-30, default 3). The shared
  iteration rules are under 'help graph-api-fetcher'.
- Writing the composable function itself is a development task done in Rust
  with the #[preload] attribute; from the Playground you only reference its
  route name.
