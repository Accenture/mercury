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
3. any other composite key - a key-value in the request body

Example:
```
input[]=input.body -> *
input[]=input.header.hello -> header.hello
input[]=input.body.amount -> amount
```

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

Exception handling
------------------
If the function throws an exception (e.g. AppException with a status code) or the call times out,
the "error" and "status" parameters of the node are set. When the node has an "exception" property,
the graph jumps to that error handler node. Otherwise, the error is returned as the graph output.

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
