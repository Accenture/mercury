Tutorial 13
-----------
In this tutorial, you will create a graph model that invokes a composable function using the
"graph.task" skill.

Pre-requisite
-------------
You would need some working knowledge of composable functions. A composable function is a struct
implementing ComposableFunction, registered with the #[preload] attribute. For more details, see
docs/guides/event-driven/ai-agent-guide.md in this repository (the composable-function authoring
guide).

What is a task?
---------------
A task is a node that invokes a composable function through its route name. MiniGraph is designed
to be zero-code with built-in skills for data mapping, decision-making and API fetching. More
complex business logic is delegated to a flow extension or a subgraph (tutorials 10 and 11). A
task node sits in between: it provides a lightweight method to extend a knowledge graph's
capability with a small piece of business logic, without writing a new skill.

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
   Since data mapping entries are processed in order, later entries can merge additional
   key-values into a request body that was seeded with `*`.
2. `text(minigraph) -> header.x-app` sets a request header of the function call. You can also map
   individual fields, e.g. `input.body.amount -> amount` would set one key-value in the request
   body.

If the composable function declares a typed input, the request body is automatically converted at
the function boundary.

About v1.hello.task
-------------------
For your convenience, the composable function "v1.hello.task" is preloaded in dev mode. It
composes a greeting from the "name" field, doubles the "amount" field and echoes the "x-app"
request header. Below is an extract of the function:

```rust
/// The tutorial-13 demo task invoked through the graph.task skill.
#[preload(route = "v1.hello.task", instances = 50)]
#[optional_service("app.env=dev")]
pub struct HelloTask;

#[async_trait]
impl ComposableFunction for HelloTask {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: JsonValue = input.body_as().unwrap_or(JsonValue::Null);
        let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("stranger");
        let mut result = serde_json::json!({"greeting": format!("Hello, {name}")});
        if let Some(amount) = body.get("amount").and_then(|v| v.as_f64()) {
            result["doubled"] = serde_json::json!(amount * 2.0);
        }
        if let Some(app) = headers.get("x-app") {
            result["app"] = serde_json::json!(app);
        }
        EventEnvelope::new().set_body(result)
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

You can also check the application log. Telemetry and tracing information are shown, proving that
the composable function was executed by the graph instance with full trace propagation.

```
Call task v1.hello.task, ttl=30000
{trace={path=/graph/playground, service=graph.task...
{trace={path=/graph/playground, service=v1.hello.task...
```

Error handling
--------------
If the composable function returns an error (e.g. an AppError with a status code) or the call
times out, the "error" and "status" parameters of the node are set. You can add an "exception"
property to the task node to route the error to a handler node, e.g. `exception=on-error`
(tutorial 12 shows the full retry pattern).

Iterative execution
-------------------
Like the API fetcher and the flow extension, a task node supports iterative fork-join execution
with the "for_each" and "concurrency" properties. Please enter 'describe skill graph.task' for
details.

Why invoke a composable function from a graph?
----------------------------------------------
The built-in skills cover data mapping, decision-making, computation and API fetching without
writing any code, and flow extensions or subgraphs handle complex orchestration. A task node
completes the picture: any custom business logic can be packaged as a composable function and
plugged into a graph as if it were a custom skill. You can extend a knowledge graph's capability
with the full power of the Mercury Composable programming model, one small function at a time.

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
To deploy the graph model, copy "/tmp/graph/tutorial-13.json" to your application's
`resources/graph` folder. You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-13 \
  -H "Content-Type: application/json" \
  -d '{
    "name": "world",
    "amount": 21
}'
```

Summary
-------
In this tutorial, you have used the "graph.task" skill to invoke a composable function through its
route name, with Event Script style input and output data mapping.
