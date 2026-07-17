Skill: Graph API Fetcher
------------------------
When a node is configured with this skill of "graph API fetcher", it will make an API call to a backend service
and collect result set into the "result" property of the node. In case of exception, the "status" and "error"
fields will be set to the node's properties and the graph execution will stop.

Execution will start when the GraphExecutor reaches the node containing this skill.

Pre-requisite
-------------
Please refer to the "data dictionary" documentation using "help data-dictionary" before creating an API fetcher node.

Route name
----------
"graph.api.fetcher"

Setup
-----
To enable this skill for a node, set "skill=graph.api.fetcher" as a property in a node.
It will find out the data provider from a given data dictionary item to make an outgoing API call.

The following are required in the properties of the node:

1. dictionary - this is a list of valid data dictionary node names configured in the same graph model
2. input - one or more data mapping as input parameters to invoke the API call
3. output - one of more data mapping to map result set to another node or the 'output.' namespace

The parameter name in each mapping statement must match that in the data dictionary item.
Otherwise, execution will fail.

The system uses the same syntax of Event Script for data mapping.

Properties
----------
```
skill=graph.api.fetcher
dictionary[]={data dictionary item}
input[]={mapping of key-value from input or another node to input parameter(s) of the data dictionary item(s)}
output[]={optional mapping of result set to one or more variables in the 'model.' or 'output.' namespace}
```

Optional properties
-------------------
```
for_each[]={map an array parameter for iterative API execution}
concurrency={controls parallel API calls for an "iterative API request". Default 3, max 30}
exception={exception-handler-node-name}
```

Dictionary
----------
This list contains one or more data dictionary item (aka 'data attribute')

Feature
-------
This API fetcher supports features configured in a data provider's node.

There are 2 built-in features that are convenience for development and tests:
- log-request-headers
- log-response-headers

When either or both of these features are added to a data provider's node, 
the fetcher will log request/response headers into the "header" section
of its properties.

Input/Output Data mapping
-------------------------
source.composite.key -> target.composite.key

For input data mapping, the source can use a key-value from the `input.` namespace or another node.
The target can be a key-value in the state machine (`model.` namespace) or an input parameter name of the
data dictionary.

For output data mapping, the source can be a key-value from the result set and the target can use
the `output.` or `model.` namespace.

Output data mapping is optional because you can use another data mapper to map result set of the fetcher
to another node.

Result set
----------
Upon successful execution, the result set will be stored in the "result" parameter in the properties of
the node. A subsequent data mapper can then map the key-values in the result set to one or more nodes.

Example
-------
```
create node fetcher-1
with properties
skill=graph.api.fetcher
dictionary[]=person_name
dictionary[]=person_address
dictionary[]=person_accounts
input[]=input.body.person_id -> person_id
output[]=result.person_name -> output.body.name
output[]=result.person_address -> output.body.address
```

Iterative API call
------------------
Using the optional `for_each` statement, you can tell the API fetcher to do "fork-n-join" of API requests.

A "for_each" statement extracts the next array element from result set of a prior API call into a model variable.
You can then put the model variable in the "left-hand-side" of an input statement. The API fetcher will then
issue multiple API calls using an iterative stream of the model variable.

If your API call needs more than one parameter, you can configure more than one "for_each" statement.

Example
-------
In this example, the "for_each" statement extracts the "person_accounts" from the result of a prior API call
by "fetcher-1" and map the array into an iterative stream of elements using the model variable "account_id".

The concurrency property tells the API fetcher to limit parallelism to avoid overwhelming the target service.
```
create node fetcher-2
with properties
skill=graph.api.fetcher
dictionary[]=person_id
dictionary[]=account_id
for_each[]=fetcher-1.result.person_accounts -> model.account_id
concurrency=3
input[]=input.body.person_id -> person_id
input[]=model.account_id -> account_id
output[]=result.person_name -> output.body.name
output[]=result.person_address -> output.body.address
```

- The "[]" syntax is used to create and append a list of one or more data mapping entries
- The "->" signature indicates the direction of mapping where the left-hand-side is a source
  and right-hand-side is a target

Deprecated syntax
-----------------
Event Script's "simple type matching" syntax (e.g. `model.someKey:text`) is deprecated. Use "simple plugin"
syntax instead (e.g. `f:text(model.someKey)`). If you (or an AI agent) submit a "create node" or "update node"
command that still uses the deprecated colon-type syntax, the system will automatically convert it to the
simple plugin syntax and return a deprecation notice - it will not silently fail, but please switch to the
new syntax going forward.

Custom error handling
---------------------
By default, when an API request fails, the system will abort the graph execution and return the error code
and message to the caller.

If you want to handle the exception in your graph model, you can set the node-name of the error-handler in
the "exception" property to tell the system to traverse to the error-handler node.

To handle an exception, the error-handler node should be a decision-making node using the graph.math or graph.js skill.
It can evaluate the status code and error in the API fetcher node to determine the next step.

Caution
-------
API fetchers can be chained together to make multiple API calls. 
However, you should design the API chain to be minimalist.

An overly complex chain of API requests would mean slow performance. Just take the minimal set of data that are
required by your application. Don't abuse the flexibility of the API fetcher.
