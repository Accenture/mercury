Skill: Graph Extension
----------------------
When a node is configured with this skill of "graph extension", it will make an API call to another graph model
(or flow) and collect result set into the "result" property of the node. In case of exception, the "status" and
"result.error" fields will be set to the node's properties and the graph execution will stop.

Execution will start when the GraphExecutor reaches the node containing this skill.

Route name
----------
"graph.extension"

Setup
-----
To enable this skill for a node, set "skill=graph.extension" as a property in a node.

The following parameters are required in the properties of the node:

1. extension - this should be a valid graph model name or flow identifier in the same memory space
2. input - this should include one or more data mapping as input parameters to invoke the API call

A flow identifier is prefixed by a flow protocol signature "flow://". e.g. "flow://hello-world".

The system uses the same syntax of Event Script for data mapping.

Properties
----------
```
skill=graph.extension
extension=graph-id or flow-id
input[]={mapping of key-value from input or another node to input parameter(s) of the data dictionary item(s)}
output[]={optional mapping of result set to one or more variables in the 'model.' or 'output.' namespace}
```

Optional properties
-------------------
```
for_each[]={map an array parameter for iterative API execution}
concurrency={controls parallel API calls for an "iterative API request". Default 3, max 30}
exception={error-handler-node-name}
```

Result set
----------
Upon successful execution, the result set will be stored in the "result" parameter in the properties of
the node. A subsequent data mapper can then map the key-values in the result set to one or more nodes.

Input Data mapping
------------------
source.composite.key -> target.composite.key

For input data mapping, the source can use a key-value from the `input.` namespace or another node.
The target can be a key-value in the state machine (`model.` namespace) or an input parameter name of the
data dictionary.

Example
-------
```
create node performance-evaluator
with properties
skill=graph.extension
extension=evaluate-sales-performance
input[]=input.body.department_id -> id
output[]=result.sales_performance -> output.body.sales_performance
```

Iterative API call
------------------
Using the optional `for_each` statement, you can tell the "Extension" skill to do "fork-n-join" of API requests.

A "for_each" statement extracts the next array element from a node result set into a model variable.
You can then put the model variable in the "left-hand-side" of the mapping statement. The skill will then
issue multiple API calls using an iterative stream of the model variable.

If your API call needs more than one parameter, you can configure more than one "for_each" statement.

The concurrency property tells the skill to limit parallelism to avoid overwhelming the target service.

The "[]" syntax is used to create and append a list of one or more data mapping entries
The "->" signature indicates the direction of mapping where the left-hand-side is source and right-hand-side is target

Custom error handling
---------------------
By default, when an API request fails, the system will abort the graph execution and return the error code
and message to the caller.

If you want to handle the exception in your graph model, you can set the node-name of the error-handler in
the "exception" property to tell the system to traverse to the error-handler node.

To handle an exception, the error-handler node should be a decision-making node using the graph.math or graph.js skill.
It can evaluate the status code and error in the API fetcher node to determine the next step.
