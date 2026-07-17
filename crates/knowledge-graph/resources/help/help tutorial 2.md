Tutorial 2
----------
In this session, you will deploy the graph model 'hello world' that you created in tutorial 1.

Exercise
--------
To deploy the graph model from tutorial 1, copy the 'tutorial-1.json' file that was exported earlier.

```
cp /tmp/tutorial-1.json ~/sandbox/{your_minigraph_project}/src/main/resources/graph
```

The default locations for the temp graph folder and the deployed graph folder are shown in the application.properties
file.

```properties
#
# temp graph working location
# (temp graph location must use "file:/" prefix because of READ/WRITE requirements
#
location.graph.temp=file:/tmp/graph
#
# deployed graph model location
# (deployed graph location may use "file:/" or "classpath:/" because it is READ only
#
location.graph.deployed=classpath:/graph
```

Invoke the graph API REST endpoint
----------------------------------
The generic graph API endpoint is `POST /api/graph/{graph_id}` where 'graph_id' is the name of the graph model.

To make a request to the 'tutorial-1' graph model, please enter the following curl command.

```
> curl -X POST http://127.0.0.1:8085/api/graph/tutorial-1
hello world
```

It will return 'hello world'.

Since the "hello world" graph model does not require any input parameter, you can also use HTTP-GET to execute
the graph.

```
> curl http://127.0.0.1:8085/api/graph/tutorial-1
hello world
```

In the application log, you will see the 'telemetry' of the event flow. The HTTP-POST request is received
by the 'http.flow.adapter' that executes a flow called 'graph-executor'.

The Graph Executor creates an instance of the graph, traverses from the "root" node and comes to the "end" node
that contains the "graph.data.mapper" skill. The data mapper sets the output as "hello world" that routes the
result to the "async.http.response" and the curl command receives.

```
2026-03-31 15:19:08.052 INFO  org.platformlambda.core.services.Telemetry:81 - 
    {trace={path=POST /api/graph/tutorial-1, service=http.flow.adapter, success=true, 
     origin=20260331aa0d11b425ce44c79f00afa8947885fc, start=2026-03-31T22:19:08.051Z, exec_time=0.12, 
     from=http.request, id=2cc56126d544483abcdbc523f486a232, status=200}}
2026-03-31 15:19:08.055 INFO  org.platformlambda.core.services.Telemetry:81 - 
    {trace={path=POST /api/graph/tutorial-1, service=graph.data.mapper, success=true, 
     origin=20260331aa0d11b425ce44c79f00afa8947885fc, start=2026-03-31T22:19:08.054Z, exec_time=0.074, 
     from=graph.executor, id=2cc56126d544483abcdbc523f486a232, status=200}, annotations={node=end}}
2026-03-31 15:19:08.056 INFO  com.accenture.minigraph.services.GraphHousekeeper:44 - 
    Graph instance 2c1a00d63f7d4ec2b657db4a75021068 for model 'tutorial-1' cleared
2026-03-31 15:19:08.056 INFO  org.platformlambda.core.services.Telemetry:81 - 
    {trace={path=POST /api/graph/tutorial-1, service=task.executor, success=true, 
     origin=20260331aa0d11b425ce44c79f00afa8947885fc, exec_time=4.0, start=2026-03-31T22:19:08.051Z, 
     from=event.script.manager, id=2cc56126d544483abcdbc523f486a232, status=200}, 
     annotations={execution=Run 1 task in 4 ms, tasks=[{spent=3.477, name=graph.executor}], flow=graph-executor}}
2026-03-31 15:19:08.056 INFO  org.platformlambda.core.services.Telemetry:81 - 
    {trace={path=POST /api/graph/tutorial-1, service=async.http.response, success=true, 
    origin=20260331aa0d11b425ce44c79f00afa8947885fc, start=2026-03-31T22:19:08.055Z, exec_time=0.224, 
    from=task.executor, id=2cc56126d544483abcdbc523f486a232, status=200}}
2026-03-31 15:19:08.057 INFO  org.platformlambda.core.services.Telemetry:81 - 
    {trace={path=POST /api/graph/tutorial-1, service=graph.housekeeper, success=true, 
    origin=20260331aa0d11b425ce44c79f00afa8947885fc, start=2026-03-31T22:19:08.056Z, exec_time=0.241, 
    from=task.executor, id=2cc56126d544483abcdbc523f486a232, status=200}}
```

Let's enhance the graph model to echo input.

Import the graph model
----------------------
You can import the tutorial-1 graph model like this:

```
> import graph from tutorial-1
Graph model imported as draft
```

The graph diagram is shown in the right panel under the "Graph" tab.

Edit the nodes
--------------
Enter an "edit node" command to print out the root node content.

```
> edit node root
update node root
with type Root
with properties
name=tutorial-1
purpose=Tutorial one to return a 'hello world' message
```

You can copy-n-paste the "update node" block into the input box and modify it as:

```
update node root
with type Root
with properties
name=tutorial-2
purpose=Tutorial two to echo a user message
```

Click enter and you will see:

```
> update node root...
node root updated
```

Then you will update the end root in the same fashion. Modify its content like this:

```
update node end
with type End
with properties
mapping[]=input.body -> output.body
skill=graph.data.mapper
```

Perform a Dry-Run
-----------------

To run the updated graph model, you can use the `instantiate graph` command with some mock input content.

```
> instantiate graph
  text(it works) -> input.body.message
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
```

In the above command, you insert the constant value "it works" into the "message" key in the "input.body"
namespace.

Enter "run" to do a dry-run and you will see this:

```
> run
Walk to root
Walk to end
Executed end with skill graph.data.mapper in 0.43 ms
{
  "output": {
    "body": {
      "message": "it works"
    }
  }
}
Graph traversal completed in 2 ms
```

Export the updated graph model
------------------------------
You may export the updated model graph as "tutorial 2".

```
> export graph as tutorial-2
Graph exported to /tmp/graph/tutorial-2.json
Described in /api/graph/model/tutorial-2/235-7
```

Deploy the graph model
----------------------
Repeat the deployment step in the beginning of this tutorial and apply it to 'tutorial-2'.

Test the deployed graph model
-----------------------------
Restart your application to load the deployed graphs into memory.

Send the following curl command

```
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-2 \
  -H "Content-Type: application/json" \
  -d '{
    "greeting": "Hello",
    "message": "it is a wonderful day"
  }'
```

It will response with:

```json
{
  "greeting": "Hello",
  "message": "it is a wonderful day"
}
```

Summary
-------
In this session, you have completed the following exercise:

1. deploy the graph model 'tutorial-1' and invoke the API that executes the graph model as an instance
2. enhance the graph model from a simple 'hello world' application to an echo program
3. perform a dry-run with mock input to test the response
4. export the updated graph model as 'tutorial-2'
5. deploy 'tutorial-2' graph model
6. test the 'tutorial-2' graph model using a HTTP-POST command with some input payload
