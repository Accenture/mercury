Tutorial 2
----------
In this tutorial, you will deploy the 'hello world' graph model that you created in
tutorial 1, then enhance it into an echo application.

Exercise
--------
To deploy the graph model from tutorial 1, copy the 'tutorial-1.json' file that was
exported earlier into your application's resources/graph folder.

```
cp /tmp/graph/tutorial-1.json ~/sandbox/{your_project}/resources/graph
```

The default locations of the temp graph folder and the deployed graph folder are set
in the application configuration file (application.properties or application.yml):

```properties
#
# temp graph working location
# (temp graph location must use "file:/" prefix because of READ/WRITE requirements)
#
location.graph.temp=file:/tmp/graph
#
# deployed graph model location
# (deployed graph location may use "file:/" or "classpath:/" because it is READ only)
#
location.graph.deployed=classpath:/graph
```

Invoke the graph API REST endpoint
----------------------------------
The generic graph API endpoint is `POST /api/graph/{graph_id}`, where 'graph_id' is
the name of the graph model.

To make a request to the 'tutorial-1' graph model, enter the following curl command.

```
> curl -X POST http://127.0.0.1:8100/api/graph/tutorial-1
hello world
```

It returns 'hello world'.

Since the "hello world" graph model does not require any input parameter, you can also
use HTTP GET to execute the graph.

```
> curl http://127.0.0.1:8100/api/graph/tutorial-1
hello world
```

In the application log, you will see the 'telemetry' of the event flow. The HTTP POST
request is received by the 'http.flow.adapter' that executes a flow called
'graph-executor'.

The Graph Executor creates an instance of the graph, traverses from the "root" node
and comes to the "end" node that contains the "graph.data.mapper" skill. The data
mapper sets the output to "hello world", which is routed to "async.http.response"
and returned to the curl command.

The telemetry entries look like this:

```
2026-03-31T22:19:08.052Z INFO  [platform_core::telemetry] {"trace":{"path":"POST /api/graph/tutorial-1",
    "service":"http.flow.adapter","success":true,"from":"http.request","exec_time":0.12,"status":200}}
2026-03-31T22:19:08.055Z INFO  [platform_core::telemetry] {"trace":{"path":"POST /api/graph/tutorial-1",
    "service":"graph.data.mapper","success":true,"from":"graph.executor","exec_time":0.074,"status":200},
    "annotations":{"node":"end"}}
2026-03-31T22:19:08.056Z INFO  [knowledge_graph::services] Graph instance 2c1a00d63f7d4ec2b657db4a75021068
    for model 'tutorial-1' cleared
2026-03-31T22:19:08.056Z INFO  [platform_core::telemetry] {"trace":{"path":"POST /api/graph/tutorial-1",
    "service":"task.executor","success":true,"from":"event.script.manager","exec_time":4.0,"status":200},
    "annotations":{"execution":"Run 1 task in 4 ms","flow":"graph-executor"}}
2026-03-31T22:19:08.057Z INFO  [platform_core::telemetry] {"trace":{"path":"POST /api/graph/tutorial-1",
    "service":"async.http.response","success":true,"from":"task.executor","exec_time":0.224,"status":200}}
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

Copy-and-paste the "update node" block into the input box and modify it as:

```
update node root
with type Root
with properties
name=tutorial-2
purpose=Tutorial two to echo a user message
```

Press enter and you will see:

```
> update node root...
node root updated
```

Then update the end node in the same fashion. Modify its content like this:

```
update node end
with type End
with properties
mapping[]=input.body -> output.body
skill=graph.data.mapper
```

Perform a dry-run
-----------------
To run the updated graph model, use the `instantiate graph` command with some
mock input content.

```
> instantiate graph
  text(it works) -> input.body.message
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
```

In the above command, you insert the constant value "it works" into the "message"
key of the "input.body" namespace.

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
You may export the updated graph model as "tutorial-2".

```
> export graph as tutorial-2
Graph exported to /tmp/graph/tutorial-2.json
Described in /api/graph/model/tutorial-2/235-7
```

Deploy the graph model
----------------------
Repeat the deployment step at the beginning of this tutorial: copy
"/tmp/graph/tutorial-2.json" into your application's resources/graph folder.

Test the deployed graph model
-----------------------------
Restart your application to load the deployed graphs into memory.

Send the following curl command:

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-2 \
  -H "Content-Type: application/json" \
  -d '{
    "greeting": "Hello",
    "message": "it is a wonderful day"
  }'
```

It responds with:

```json
{
  "greeting": "Hello",
  "message": "it is a wonderful day"
}
```

Summary
-------
In this tutorial, you have completed the following exercise:

1. deployed the graph model 'tutorial-1' and invoked the API that executes the graph model as an instance
2. enhanced the graph model from a simple 'hello world' application to an echo program
3. performed a dry-run with mock input to test the response
4. exported the updated graph model as 'tutorial-2'
5. deployed the 'tutorial-2' graph model
6. tested the 'tutorial-2' graph model using an HTTP POST request with an input payload
