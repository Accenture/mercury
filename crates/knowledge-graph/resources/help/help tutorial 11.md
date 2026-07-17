Tutorial 11
-----------
In this session, you will create a graph model to use an "event flow" as an extension.

Pre-requisite
-------------
You would need some working knowledge with event script. For more details, please refer to
[Event Script Syntax](https://accenture.github.io/mercury-composable/guides/CHAPTER-4).

Assume you already know how to create an event flow (configuration and composable functions as tasks),
it is easy to use event flow as an extension.

What is a flow extension?
-------------------------
A flow extension is an event flow that is built to serve some logic that can be reused by a graph model.

Import graph model from Tutorial-10
-----------------------------------
In tutorial 10, you have created an extension in a main graph to call another graph.

You will update the graph model in tutorial 10 to call a flow as an extension.

```
> import graph from tutorial-10
Graph exported to /tmp/graph/tutorial-11.json
Described in /api/graph/model/tutorial-11/431-3
```

Edit the root node
------------------
Enter 'edit node root' and copy-n-paste the content into the inbox box. Change the name and purpose for
tutorial 11.

```
update node root
with type Root
with properties
name=tutorial-11
purpose=Demonstrate the use of flow extension
```

Edit the extension node
-----------------------
Enter 'edit node extension' and copy-n-paste the content into the inbox box. Update the extension to "flow://flow-11"
and change the input statements to pass "hello" and "message" as parameters. The flow protocol prefix tells the
system to execute the flow with the identifier "flow-11".

```
update node extension
with type Extension
with properties
extension=flow://flow-11
input[]=input.body.hello -> hello
input[]=input.body.message -> message
output[]=result -> output.body
skill=graph.extension
```

About flow 11
-------------
For your convenience, "flow-11" is preloaded. You can review the configuration files "flows.yaml" and "flow-11.yml"
in the resources folder. The event flow "flow-11" is an echo program. The task "no.op" will echo everything from
the input and pass it as output. Below is an extract of the event flow's first task.

```yaml
tasks:
  - input:
      # pass all input parameters as arguments
      - 'input.body -> *'
    process: 'no.op'
    output:
      - 'result -> output.body'
    description: 'echo everything in the input payload'
    execution: end
```

Perform a dry-run
-----------------
To test the updated graph model, you can instantiate the graph with the two input "hello" and "message" as follows:

```
instantiate graph
text(world) -> input.body.hello
text(this is a good day) -> input.body.message
```

Then enter 'run' to execute the graph.

```
> start graph...
Graph instance created. Loaded 2 mock entries, model.ttl = 30000 ms
> run
Walk to root
Walk to extension
Executed extension with skill graph.extension in 5.46 ms
Walk to end
{
  "output": {
    "body": {
      "hello": "world",
      "message": "this is a good day"
    }
  }
}
Graph traversal completed in 7 ms
```

You can also check the application log. Telemetry and tracing information are shown.

```
GraphExtension:202 - Call extension flow://flow-11, ttl=30000
Telemetry:81 - {trace={path=/graph/playground, service=graph.extension...
Telemetry:81 - {trace={path=/graph/playground, service=no.op...
Telemetry:81 - {trace={path=/graph/playground, service=task.executor...
Telemetry:81 - {trace={path=/graph/playground, service=event.script.manager...
```

This validates that the event flow instance for "flow-11" was executed by the graph instance for tutorial-11.

Export the graph model
----------------------
Now you may save the graph model by exporting it.

```
> export graph as tutorial-11
Graph exported to /tmp/graph/tutorial-11.json
Described in /api/graph/model/tutorial-11/794-6
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-11.json" to your application's `main/resources/graph` folder.
You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-11 \
  -H "Content-Type: application/json" \
  -d '{ 
    "hello": "world",
    "message": "this is a good day"
}'
```

Summary
-------
In this session, we have discussed the use of an event flow as an extension to a graph model and
the use of the flow protocol prefix "flow://".

Why extending a graph model with event flow?
--------------------------------------------
While graph extension discussed in tutorial 10 can create sophisticated and powerful graph models,
extending a graph with event flow allows us to do things beyond simple API fetching, data mapping, computation
and decision-making.

With event flow, you can model very complex transaction processing with "pro-code". The combined graph modeling
and event script programming provides the best of both worlds in no-code and pro-code to tackle the most
demanding use cases.
