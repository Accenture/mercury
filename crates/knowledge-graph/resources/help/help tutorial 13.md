Tutorial 13
-----------
In this session, you will create a graph model that invokes a composable function using the
"graph.task" skill.

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
task=v1.hello.task
input[]=input.body -> *
input[]=text(minigraph) -> header.x-app
output[]=result -> output.body
skill=graph.task
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

1. `input.body -> *` maps the whole request body as the request body of the composable function.
   Since data mapping entries are processed in order, later entries can merge additional key-values
   into a request body that was seeded with `*`.
2. `text(minigraph) -> header.x-app` sets a request header of the function call. You can also map
   individual fields, e.g. `input.body.amount -> amount` would set one key-value in the request body.

If the composable function is declared with a PoJo input class, the request body map is automatically
converted to the PoJo at the function boundary.

About v1.hello.task
-------------------
For your convenience, the composable function "v1.hello.task" is preloaded in dev mode. It composes a
greeting from the "name" field, doubles the "amount" field and echoes the "x-app" request header.
Below is an extract of the function:

```java
@PreLoad(route = "v1.hello.task", instances = 50)
public class HelloTask implements TypedLambdaFunction<Map<String, Object>, Object> {

    @Override
    public Object handleEvent(Map<String, String> headers, Map<String, Object> input, int instance) {
        var result = new HashMap<String, Object>();
        result.put("greeting", "Hello, " + input.getOrDefault("name", "stranger"));
        if (input.get("amount") instanceof Number n) {
            result.put("doubled", n.doubleValue() * 2);
        }
        if (headers.containsKey("x-app")) {
            result.put("app", headers.get("x-app"));
        }
        return result;
    }
}
```

Perform a dry-run
-----------------
To test the graph model, you can instantiate the graph with mock input as follows:

```
instantiate graph
text(world) -> input.body.name
int(21) -> input.body.amount
```

Then enter 'run' to execute the graph.

```
> start graph...
Graph instance created. Loaded 2 mock entries, model.ttl = 30000 ms
> run
Walk to root
Walk to hello-task
Executed hello-task with skill graph.task in 4.12 ms
Walk to end
{
  "output": {
    "body": {
      "greeting": "Hello, world",
      "doubled": 42.0,
      "app": "minigraph"
    }
  }
}
Graph traversal completed in 6 ms
```

You can also check the application log. Telemetry and tracing information are shown, proving that the
composable function was executed by the graph instance with full trace propagation.

```
GraphTask:144 - Call task v1.hello.task, ttl=30000
Telemetry:81 - {trace={path=/graph/playground, service=graph.task...
Telemetry:81 - {trace={path=/graph/playground, service=v1.hello.task...
```

Error handling
--------------
If the composable function throws an exception (e.g. AppException with a status code) or the call times
out, the "error" and "status" parameters of the node are set. You can add an "exception" property to the
task node to route the error to a handler node, e.g. `exception=on-error`.

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

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-13.json" to your application's `main/resources/graph`
folder. You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-13 \
  -H "Content-Type: application/json" \
  -d '{
    "name": "world",
    "amount": 21
}'
```

Summary
-------
In this session, we have discussed the use of the "graph.task" skill to invoke a composable function
through its route name, with Event Script style input and output data mapping.

Why invoke a composable function from a graph?
----------------------------------------------
The built-in skills cover data mapping, decision-making, computation and API fetching without writing
any code, and flow extensions or subgraphs handle complex orchestration. A task node completes the
picture - any custom business logic can now be packaged as a composable function and plugged into a
graph as if it were a custom skill. This means you can extend a knowledge graph's capability with the
full power of the Mercury Composable programming model, one small function at a time.
