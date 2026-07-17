Tutorial 10
-----------
In this session, you will create a graph model with custom error handling.

Exercise
--------
You will import tutorial 3 and add an error-handler node to retry an API failure.

To clear the previous graph session, click the Tools button in the top-right corner and click the "Stop" and "Start"
toggle button. A new graph session will start.

Import tutorial 3 as a template
-------------------------------
Enter the following to import tutorial 3. Note that tutorial-3.json is preloaded into the main/resources/graph
folder.

```
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
Graph model imported as draft
```

Update the root node
--------------------
Enter the following to update the root node. It assigns the skill "graph.data.mapper" to the node and
maps the input parameter "exception" to the model variable with the same name.

The `f:defaultValue()` plugin function sets the variable "model.exception" to false when the input
parameter is not given.

We will use the model.exception parameter to trigger a simulated exception for the mdm-profile service.

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
For person-address, you will add the input parameter `exception:false` where ":false" is the default value of
the parameter if not given.

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
You will add the input data mapping `exception -> header.x-exception` to the mdm-profile node. The input parameter
"exception" is used to set the HTTP request header "X-Exception".

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
You will add the input data mapping `model.exception -> exception` to set the parameter exception to retrieve
the two data dictionary items (person-name and person-address).

You also add the property `exception=error-handler`. This tells the system to route the flow to the "error-handler"
node.

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

The mock endpoint contains this:

```java
@Override
public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) {
    if ("true".equals(input.getHeader("x-exception"))) {
        throw new AppException(401, "simulated exception");
    }
    // for simplicity, business logic not shown here
}
```

Create Error-Handler node
-------------------------
You will then create the error-handler node that is referenced in the fetcher node above.

When the "exception" property is configured in a fetcher, the system will not abort the graph traversal, it will
route it to the given error handler.

In the handler, you test the "fetcher.status" variable to see if it is HTTP-200. While an error status is always
a value equals or larger than 200, it is a good practice to do simple validation to avoid unintended configuration
error.

If it is not 200, the statement block will execute. The first 2 mapping statements increment the variable
"model.attempts". The next evaluation statement checks if the maximum attempts have reached, it will clear
the simulated exception by routing to the "clear-exception" node.

The "NEXT: fetcher" statement tells the system to connect to the fetcher again. Since a node cannot be executed twice,
you use the "RESET:" command to clear its states so that it can be executed again.

The "DELAY: 50" means that it will pause for 50 milliseconds before the next retry. This is a best practice because
it avoids very rapid retries that may contribute to a side effect called "recovery storm" or 
"unintended denial-of-service attack".

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
statement[]=MAPPING: f:defaultValue(model.attempts, int(0)) -> model.attempts
statement[]=MAPPING: f:add(model.attempts, int(1)) -> model.attempts
statement[]='''
IF: {model.attempts} >= 3
THEN: clear-exception
ELSE: next
'''
statement[]=RESET: fetcher, error-handler
statement[]=NEXT: fetcher
statement[]=DELAY: 50
```

Create the clear-exception node
-------------------------------
In the clear-exception node, you add statements to set the variable "model.exception" to false so that
the mock service will return normal response instead of an exception. You also clear the "model.attempts" to zero
and reset the fetcher and the clear-exception nodes so that the system can execute them again.

You will then create new connections to complete the exercise.

```
create node clear-exception
with type Decision
with properties
skill=graph.math
statement[]=MAPPING: boolean(false) -> model.exception
statement[]=MAPPING: int(0) -> model.attempts
statement[]=RESET: fetcher, clear-exception
```

Connections for error-handler and clear-exception nodes
-------------------------------------------------------

```
connect error-handler to fetcher with retry
connect clear-exception to fetcher with reset
```

Do a dry-run
------------
Enter the following to start the graph with mock input data. You are setting integer of 100 to person_id
and boolean value of "true" to exception in the input payload.

```
start graph
int(100) -> input.body.person_id
boolean(true) -> input.body.exception
```

Execute the run command

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

The graph traversal log shows that the "error-handler" node has been executed for 3 times before
the clear-exception node is executed. After clearing the exception, the mock service returns
a correct result set as "output".

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
To deploy the graph model, copy "/tmp/graph/tutorial-10.json" to your application's `main/resources/graph` folder.
You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-12 \
  -H "Content-Type: application/json" \
  -d '{ 
    "person_id": 100,
    "exception": true
}'
```

Summary
-------
In this session, you have used tutorial-3 as a template and enhanced it with custom error handling.

You have used the keywords "RESET", "NEXT" and "DELAY" to reset the states of the nodes visited, to tell the
graph traversal system to route to a specific node and to introduce an artificial delay to avoid overwhelming
the target service.

IMPORTANT: Graph traversal loops
--------------------------------
The graph traversal system is designed to allow a node to be executed only once.

When using the keyword "RESET: node-name", the "seen" status and all state information are cleared so that the node
can be executed again. This would create a potential endless loop in graph traversal.

Therefore, please pay attention to have some decision logic to stop looping or retries.

As a protection mechanism, the system has a built-in loop detection logic. When a node is executed too frequently,
the graph traversal will be aborted.

The default parameters in `application.properties` are 10 visits per second for the same node.

```properties
graph.max.loop.interval=1000
graph.node.high.frequency=10
```
