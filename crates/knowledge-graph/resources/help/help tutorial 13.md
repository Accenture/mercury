Tutorial 13
-----------
In this session, you will create a graph model that invokes a composable function using the
"graph.task" skill. The composable function is the AsyncHttpClient (route "async.http.request")
provided by the platform-core module, turning the task node into an HTTP client by configuration.

Pre-requisite
-------------
You would need some working knowledge of composable functions. A composable function is a
TypedLambdaFunction registered with the PreLoad annotation. For more details, please refer to the
[Developer Guide](https://accenture.github.io/mercury-composable/).

What is a task?
---------------
A task is a node that invokes a composable function through its route name. MiniGraph is designed to be
zero-code with built-in skills for data mapping, decision-making and API fetching. More complex business
logic is delegated to a flow extension or a subgraph (tutorials 10 and 11). A task node sits in between -
it provides a lightweight method to extend a knowledge graph's capability with a small piece of business
logic, without writing a new skill.

In this tutorial, the "small piece of business logic" is not custom code at all - it is the framework's
own AsyncHttpClient. Any function registered in the platform is callable by route name, so the task node
can drive an HTTP call purely by configuration.

Create the graph model
----------------------
Create the root node:

```
create node root
with type Root
with properties
name=tutorial-13
purpose=Demonstrate the graph.task skill - invoking a composable function through its route name
```

Create the task node. The "task" property is the route name of the composable function:

```
create node hello-task
with type Task
with properties
input[]=input.body.person_id -> model.person_id
input[]=text(http://127.0.0.1:${rest.server.port:8080}) -> host
input[]=text(/api/mdm/profile/{model.person_id}) -> url
input[]=text(GET) -> method
input[]=text(application/json) -> headers.accept
input[]=text(5000) -> headers.x-ttl
output[]=result -> output.body
purpose=Invoke AsyncHttpClient with route 'async.http.request' to fetch a user profile
skill=graph.task
task=async.http.request
```

Create the end node and connect the three nodes:

```
create node end
with type End
```

```
connect root to hello-task with run
connect hello-task to end with finish
```

For your convenience, this graph model is also preloaded. You can import it with
'import graph from tutorial-13' instead of creating the nodes manually.

About the input data mapping
----------------------------
The input data mapping follows the Event Script syntax and is applied in declaration order:

1. `input.body.person_id -> model.person_id` stages a variable in the graph's state machine
   (the `model.` namespace). It does not become part of the function's request - it is kept for
   later entries to reference.
2. `text(/api/mdm/profile/{model.person_id}) -> url` demonstrates a **dynamic variable**: the
   `{model.person_id}` reference inside the text constant is substituted with the model value
   staged by the earlier entry. This is the same idiom as Event Script's
   `text(Bearer {model.token}) -> headers.Authorization`.
3. `text(http://127.0.0.1:${rest.server.port:8080}) -> host` demonstrates **environment variable
   substitution**. The `${name:default}` reference is resolved by the configuration system when the
   model is loaded - at 'instantiate graph' for a dry-run and at deployment time for a deployed
   model - so both lanes behave the same. The authored model (and any export) keeps the `${...}`
   placeholder, making the model portable across environments.
4. `text(application/json) -> headers.accept` declares the response type this client accepts.
   Always declare it instead of relying on an HTTP library's implicit default - with an explicit
   accept, the profile service replies with `content-type: application/json` and the
   AsyncHttpClient decodes the response body into a map.
5. `text(5000) -> headers.x-ttl` sets the **HTTP timeout** of the AsyncHttpClient. The graph's
   regular ttl propagation (a node's optional `ttl` property, else `model.ttl`) bounds only the
   event call to the composable function - it cannot reach inside a generic function, so the
   HTTP client would otherwise run on its own 30-second default. The X-TTL value is expressed
   in **milliseconds**. It also rides the wire as the `X-TTL` request header, so a downstream
   Mercury service adopts it as its processing deadline (end-to-end deadline propagation).
6. Any other RHS such as `url` and `method` is a composite key path in the function's request body.
   RHS `*` would map the LHS value as the whole request body, and `header.{name}` would set a
   request header of the function call.

About async.http.request
------------------------
The input data mapping above builds a map of key-values. The AsyncHttpRequest class in platform-core
renders that map into an HTTP request through its "fromMap" method at the function boundary. The
commonly used keys are:

```
host              target host, e.g. http://127.0.0.1:8080
url               URI path, e.g. /api/mdm/profile/100
method            GET, POST, PUT, DELETE, etc.
headers.{name}    an HTTP request header
headers.x-ttl     HTTP timeout in milliseconds (default 30000); also propagates on the wire
body              the HTTP request body (for POST/PUT)
parameters.query.{name}   a query parameter
```

The mock MDM profile service (GET /api/mdm/profile/{id}) is preloaded in dev mode, serving
person IDs 100 and 200.

Perform a dry-run
-----------------
To test the graph model, you can instantiate the graph with mock input as follows:

```
instantiate graph
int(100) -> input.body.person_id
```

Then enter 'run' to execute the graph.

```
> start graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
> run
Walk to root
Walk to hello-task
Executed hello-task with skill graph.task in 18.5 ms
Walk to end
{
  "output": {
    "body": {
      "profile": {
        "id": "100",
        "name": "Peter",
        "address": "100 World Blvd"
      },
      "accounts": ["a101", "b202", "c303", "d400", "e500"],
      "observed_ttl": "5000"
    }
  }
}
Graph traversal completed in 21 ms
```

Note that 'instantiate graph' resolved `${rest.server.port:8080}` to the application's actual port,
and the task node resolved `{model.person_id}` to 100 before calling the HTTP endpoint. The
"observed_ttl" field is the mock service echoing the `X-TTL` request header it received - proof
that the 5000 ms deadline arrived on the wire.

You can also check the application log. Telemetry and tracing information are shown, proving that the
composable function was executed by the graph instance with full trace propagation.

```
GraphTask:144 - Call task async.http.request, ttl=30000
Telemetry:81 - {trace={path=/graph/playground, service=graph.task...
Telemetry:81 - {trace={path=/graph/playground, service=async.http.request...
Telemetry:81 - {trace={path=/graph/playground, service=mock.mdm.profile...
```

Error handling
--------------
If the composable function throws an exception (e.g. AppException with a status code) or the call times
out, the "error" and "status" parameters of the node are set. You can add an "exception" property to the
task node to route the error to a handler node, e.g. `exception=on-error`.

You can see this without any extra configuration - instantiate with an unknown person ID such as
`int(999) -> input.body.person_id` and the HTTP error from the profile service becomes the graph
output.

Iterative execution
-------------------
Like the API fetcher and the flow extension, a task node supports iterative fork-join execution with the
"for_each" and "concurrency" properties. Please enter 'describe skill graph.task' for details.

Export the graph model
----------------------
Now you may save the graph model by exporting it.

```
> export graph as tutorial-13
Graph exported to /tmp/graph/tutorial-13.json
Described in /api/graph/model/tutorial-13/431-3
```

The exported file keeps the `${rest.server.port:8080}` placeholder, so the same model resolves to
the correct port in each environment it is deployed to.

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-13.json" to your application's `main/resources/graph`
folder. You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-13 \
  -H "Content-Type: application/json" \
  -d '{
    "person_id": 100
}'
```

Summary
-------
In this session, we have discussed the use of the "graph.task" skill to invoke a composable function
through its route name, with Event Script style input and output data mapping. Along the way you used
a model variable as a dynamic variable in a later data mapping entry, and an environment variable
reference that resolves when the model is loaded.

Why invoke a composable function from a graph?
----------------------------------------------
The built-in skills cover data mapping, decision-making, computation and API fetching without writing
any code, and flow extensions or subgraphs handle complex orchestration. A task node completes the
picture - any custom business logic can now be packaged as a composable function and plugged into a
graph as if it were a custom skill. As this tutorial shows, that includes functions the framework
already provides: the AsyncHttpClient became an HTTP client by configuration, with no code at all.
