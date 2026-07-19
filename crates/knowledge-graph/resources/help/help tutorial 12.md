Tutorial 12
-----------
In this tutorial, you will create a graph model with custom error handling.

Exercise
--------
You will import tutorial 3 and add an error-handler node that retries an API failure.

To clear the previous graph session, click the Tools button in the top-right corner and click the
"Stop" and "Start" toggle button. A new graph session will start.

Import tutorial 3 as a template
-------------------------------
Enter the following to import tutorial 3. Note that tutorial-3.json is preloaded into the
`resources/graph` folder.

```
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
Graph model imported as draft
```

Update the root node
--------------------
Enter the following to update the root node. It assigns the skill "graph.data.mapper" to the node
and maps the input parameter "exception" to the model variable with the same name.

The `f:defaultValue()` plugin function sets the variable "model.exception" to false when the input
parameter is not given.

We will use the model.exception parameter to trigger a simulated exception in the mdm-profile
service.

```
update node root
with type Root
with properties
mapping[]=f:defaultValue(input.body.exception, boolean(false)) -> model.exception
name=tutorial-12
purpose=Demonstrate custom error handling
skill=graph.data.mapper
```

Update the dictionary
---------------------
For person-address, you will add the input parameter `exception:false`, where ":false" is the
default value of the parameter when it is not given.

```
update node person-address
with type Dictionary
with properties
input[]=person_id
input[]=exception:false
output[]=response.profile.address -> result.address
provider=mdm-profile
purpose=address of a person
```

and do the same for person-name

```
update node person-name
with type Dictionary
with properties
input[]=person_id
input[]=exception:false
output[]=response.profile.name -> result.name
provider=mdm-profile
purpose=name of a person
```

Update the data provider
------------------------
You will add the input data mapping `exception -> header.x-exception` to the mdm-profile node. The
input parameter "exception" is used to set the HTTP request header "X-Exception".

```
update node mdm-profile
with type Provider
with properties
feature[]=log-request-headers
feature[]=log-response-headers
input[]=text(application/json) -> header.accept
input[]=exception -> header.x-exception
input[]=person_id -> path_parameter.id
method=GET
purpose=Master Data Management's profile management endpoint
url=http://127.0.0.1:${rest.server.port:8080}/api/mdm/profile/{id}
```

Update the fetcher node
-----------------------
You will add the input data mapping `model.exception -> exception` to set the parameter
"exception" when retrieving the two data dictionary items (person-name and person-address).

You also add the property `exception=error-handler`. This tells the system to route the flow to
the "error-handler" node when a call fails, instead of aborting the graph traversal.

```
update node fetcher
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
exception=error-handler
input[]=input.body.person_id -> person_id
input[]=model.exception -> exception
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
skill=graph.api.fetcher
```

The dev-mode mock endpoint contains this:

```rust
// extract of the dev mock endpoint (mock.mdm.profile)
if request["headers"]["x-exception"] == "true" {
    return Err(AppError::new(401, "simulated exception"));
}
// business logic not shown
```

Create the error-handler node
-----------------------------
You will now create the error-handler node referenced by the fetcher above.

When the "exception" property is configured on a fetcher, a failed call — an error status is
always a value of 400 or higher — does not abort the graph traversal: the engine sets the node's
"status" and "error" variables, skips its output mappings, and routes the flow to the named error
handler.

The handler's statements run in order:

1. The first IF tests "fetcher.status". It is good practice to test for exactly 200 so an
   unintended configuration error cannot slip through. On HTTP-200 the THEN branch jumps to the
   end node — a taken node-jump ends the statement list immediately. Otherwise the ELSE branch
   resolves to "next" and falls through to the following statements.
2. RESET comes **first among the action statements**: it clears the run-once guard and state of a
   comma-separated list of nodes so they can be executed again — here the fetcher and the
   error-handler itself (a node may reset itself because the run-once mark is set before its
   statements execute). Placing RESET early guarantees it runs on every path — a later taken IF
   jump would skip it — and everything the node stores afterwards (such as the pending DELAY)
   survives the self-wipe. Keep RESET **after** any check that reads state it would wipe: the
   status IF above must run first, because RESET clears "fetcher.status" and an IF on a wiped
   variable aborts the run.
3. The two MAPPING statements increment the retry counter "model.attempts" (`f:defaultValue()`
   seeds it to 0 on the first pass). The "model" namespace is not touched by RESET.
4. The second IF bounds the retry loop: after 3 attempts it jumps to the "clear-exception" node.
5. "NEXT: fetcher" tells the traversal system to jump to the fetcher node. Unlike a taken IF jump,
   NEXT does not stop the statement list — the jump is applied after the whole list completes.
6. "DELAY: 50" pauses for 50 milliseconds after this node completes, before the next retry. Pacing
   retries is a best practice: it avoids very rapid retries that can cause a "recovery storm" — an
   unintended denial-of-service attack on the target service.

```
create node error-handler
with type Decision
with properties
skill=graph.math
statement[]='''
IF: {fetcher.status} == 200
THEN: end
ELSE: next
'''
statement[]=RESET: fetcher, error-handler
statement[]=MAPPING: f:defaultValue(model.attempts, int(0)) -> model.attempts
statement[]=MAPPING: f:add(model.attempts, int(1)) -> model.attempts
statement[]='''
IF: {model.attempts} >= 3
THEN: clear-exception
ELSE: next
'''
statement[]=NEXT: fetcher
statement[]=DELAY: 50
```

Create the clear-exception node
-------------------------------
In the clear-exception node, the RESET comes first (nothing before it reads node state), clearing
the fetcher and the clear-exception node itself so that the system can execute them again. You
then set the variable "model.exception" to false so that the mock service returns a normal
response instead of an exception, and clear "model.attempts" to zero.

```
create node clear-exception
with type Decision
with properties
skill=graph.math
statement[]=RESET: fetcher, clear-exception
statement[]=MAPPING: boolean(false) -> model.exception
statement[]=MAPPING: int(0) -> model.attempts
```

Connections for error-handler and clear-exception nodes
-------------------------------------------------------
Create the connections to complete the retry loop.

```
connect error-handler to fetcher with retry
connect clear-exception to fetcher with reset
```

Do a dry-run
------------
Enter the following to start the graph with mock input data. You are setting the integer 100 to
person_id and the boolean value "true" to exception in the input payload.

```
start graph
int(100) -> input.body.person_id
boolean(true) -> input.body.exception
```

Execute the run command.

```
> run
Walk to root
Executed root with skill graph.data.mapper in 0.231 ms
Walk to fetcher
Walk to dictionary
Executed dictionary with skill graph.island in 0.014 ms
Executed fetcher with skill graph.api.fetcher in 21.83 ms
Walk to error-handler
Executed error-handler with skill graph.math in 52.242 ms
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 8.025 ms
Walk to error-handler
Executed error-handler with skill graph.math in 51.824 ms
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 8.264 ms
Walk to error-handler
Executed error-handler with skill graph.math in 51.837 ms
Walk to clear-exception
Executed clear-exception with skill graph.math in 0.132 ms
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 0.547 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
Graph traversal completed in 201 ms
```

The graph traversal log shows that the "error-handler" node executed 3 times before the
clear-exception node ran. After the exception is cleared, the mock service returns a correct
result set as "output".

Export the graph model
----------------------
Now you may save the graph model by exporting it.

```
> export graph as tutorial-12
Graph exported to /tmp/graph/tutorial-12.json
Described in /api/graph/model/tutorial-12/591-5
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-12.json" to your application's
`resources/graph` folder. You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-12 \
  -H "Content-Type: application/json" \
  -d '{ 
    "person_id": 100,
    "exception": true
}'
```

Summary
-------
In this tutorial, you have used tutorial-3 as a template and enhanced it with custom error
handling.

You have used the keywords "RESET", "NEXT" and "DELAY" to clear the state of visited nodes, to
tell the graph traversal system to route to a specific node, and to introduce an artificial delay
that avoids overwhelming the target service.

IMPORTANT: Graph traversal loops
--------------------------------
The graph traversal system is designed to allow a node to be executed only once per run.

When you use the keyword "RESET: node-name", the "seen" status and all state information are
cleared so that the node can be executed again. This creates the potential for an endless loop in
graph traversal.

Therefore, always include decision logic that bounds the looping or retries — like the
"model.attempts" counter in this tutorial.

As a protection mechanism, the system has built-in loop detection. When a node is executed too
frequently, the graph traversal is aborted.

The default parameters in `application.properties` allow 10 visits per second for the same node.

```properties
graph.max.loop.interval=1000
graph.node.high.frequency=10
```
