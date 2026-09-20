Skill: Graph Task
-----------------
When a node is configured with this skill of "graph task", it will invoke a composable function
through its route name and collect the function's response into the "result" property of the node.
In case of exception, the "status" and "error" fields will be set to the node's properties and the
graph execution will stop unless an exception handler node is configured.

A composable function is a TypedLambdaFunction registered using the PreLoad annotation. This provides
a lightweight method to extend a knowledge graph's capability with a small piece of business logic,
without writing a new skill - more complex business logic should be delegated to a flow extension
or a subgraph using the "graph.extension" skill.

Execution will start when the GraphExecutor reaches the node containing this skill.

Route name
----------
"graph.task"

Setup
-----
To enable this skill for a node, set "skill=graph.task" as a property in a node.

The following parameters are required in the properties of the node:

1. task - the route name of the composable function to invoke
2. input - one or more data mapping entries as input to the composable function

The system uses the same syntax of Event Script for data mapping.

Properties
----------
```
skill=graph.task
task=route.name.of.composable.function
input[]={mapping of key-values from input, model or another node to the function's request}
output[]={optional mapping of result set to one or more variables in the 'model.' or 'output.' namespace}
```

Optional properties
-------------------
```
for_each[]={map an array parameter for iterative function execution}
concurrency={controls parallel function calls for an "iterative task request". Default 3, max 30}
exception={error-handler-node-name}
```

Input data mapping
------------------
source.composite.key -> target

The source (LHS) can use a key-value from the `input.` namespace, the `model.` namespace, another
node or a constant such as text(hello). The target (RHS) addresses the function's request:

1. `*` - the LHS value becomes the whole request body (same as Event Script). Data mapping entries
   are processed in order, so later entries can merge additional key-values into a request body
   that was seeded with `*`.
2. `header.{name}` - sets a request header of the function call
3. `model.{key}` - stages a variable in the graph's state machine instead of the request body, so
   that later entries can reference it as a **dynamic variable** (same as Event Script). e.g. after
   `input.body.token -> model.token`, the entry `text(Bearer {model.token}) -> auth` resolves the
   `{model.token}` reference. Engine-managed model metadata (model.cid, model.ttl, etc.) is
   immutable - a mapping that targets it is rejected.
4. any other composite key - a key-value in the request body

Example:
```
input[]=input.body -> *
input[]=input.header.hello -> header.hello
input[]=input.body.amount -> amount
input[]=input.body.person_id -> model.person_id
input[]=text(/api/mdm/profile/{model.person_id}) -> url
```

Static decision table
---------------------
A lookup table that changes with legislation rather than with each request belongs on a node, not
in the function. It is more readable - the product owner certifies the rules on the graph, in the
business vocabulary - and it replaces a ladder of IF-THEN-ELSE (a chain of graph.math decision nodes,
or conditionals inside a composable function) with one table that grows a row per rule instead of a
branch per rule. Do not hard-code the table in graph.math statements or in a function. Every node's
properties are copied into the state machine when the graph is instantiated, so a skill-less node's
alias is a mapping source and ONE input entry hands the whole table to a generic function:

```
create node state-rules
with type DecisionTable
with properties
keys=[ "community-property", "separate-property" ]
community-property=[ "CA", "TX" ]
separate-property=[ "NY" ]
```

```
input[]=state-rules -> table
input[]=input.body.state -> key
```

"keys" names the rules in priority order and each rule lists its member keys. Every value is a JSON
array written as text, so the node reads as a table in the Playground and the function reconstructs
the lists (SimpleMapper in Java, serde_json in Rust) and reads the rule names from "table.keys".
Variations: "key[]=member" lines build a real list property, and a nested table may be ONE JSON text
property (a multi-line '''...''' value) that f:json parses at mapping time:
input[]=f:json(state-rules.table) -> table. The product owner certifies the table on the graph and a
new table ships as a new graph version, never as a code change. Wire the table node under the
graph's island so that no node is left unconnected.

If the function is declared as a TypedLambdaFunction with a PoJo input class, the request body map
is automatically converted to the PoJo at the function boundary.

Result set
----------
Upon successful execution, the function's response body is stored in the "result" parameter, the
response status in "status" and the response headers in "header" in the properties of the node.
The optional output data mapping can copy them to the 'model.' or 'output.' namespace.

Example:
```
output[]=result -> model.soap_request_payload
```

Timeout
-------
The function call uses the graph instance's time-to-live from "model.ttl" (default 30000 ms).

This deadline bounds the event call to the composable function - it cannot reach inside a
generic function. When the function has its own downstream timeout contract, express it in the
input data mapping. For example, the AsyncHttpClient (async.http.request) takes its HTTP timeout
from the "x-ttl" key-value under "headers" in milliseconds, e.g. `text(5000) -> headers.x-ttl`
(see tutorial 13).

Exception handling
------------------
If the function throws an exception (e.g. AppException with a status code) or the call times out,
the "error" and "status" parameters of the node are set (plus "stack" when the failure carries a
stack trace). When the node has an "exception" property, the graph jumps to that error handler
node. Otherwise, the error is returned as the graph output.

When traversal jumps to the handler, the engine also stages a generic exception context that
does not name the failing node:

- error.source  - the failing node's alias
- error.code    - the status code
- error.message - the error message
- error.stack   - the stack trace, when the failure carries one

so ONE handler node can serve the "exception" route of every node in the graph - a graph.task
node, an API fetcher and an extension can all share the same handler, and error.source tells
them apart. Anchor a shared handler from an island (root -> island -> handler) because it is
reached by jumping, and note that a node is visited at most once per run unless RESET. The
alias 'error' is reserved for this namespace - probe it in a dry-run session with
"inspect error". See "describe skill graph.api.fetcher" for the canonical bounded-retry handler.

Example
-------
```
create node prepare-soap-request
with type Task
with properties
task=v1.prepare.soap.request
input[]=input.body -> *
input[]=input.header.hello -> header.hello
output[]=result -> model.soap_request_payload
skill=graph.task
```
