Tutorial 10
-----------
In this tutorial, you will create a graph model that uses another graph model as an extension.

Exercise
--------
You will use an existing graph model as an extension, then create a new graph model that calls it.

To clear the previous graph session, click the Tools button in the top-right corner and click the
"Stop" and "Start" toggle button. A new graph session will start.

What is a graph extension?
--------------------------
A graph extension is a graph model built to serve some logic that another graph model can reuse.

The `extension` property of a graph.extension node names a **deployed** graph model — one compiled
at application startup from the `resources/graph` folder (the same ids callable at
POST /api/graph/{graph-id}). A session draft is not addressable as an extension: export and deploy
it first.

Import tutorial 3 as an extension
---------------------------------
Enter the following to import tutorial 3. Note that tutorial-3.json is preloaded into the
`resources/graph` folder.

```
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
Graph model imported as draft
```

Once the graph model is imported, start the graph with mock data.

```
start graph
int(100) -> input.body.person_id
```

Then do a 'dry-run'.

```
> run
Walk to root
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 0.982 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
Graph traversal completed in 2 ms
```

You can see that it fetches data using the input parameter (person_id=100) and returns the name
and address of the person. This is the behavior your new graph will reuse.

Restart playground session
--------------------------
You will clear the current graph session — click the Tools button in the top-right corner and
click the "Stop" and "Start" toggle button. A new graph session will start.

Create a root node and an end node
----------------------------------
You will create a new graph model with a root node and an end node.

```
create node root
with type Root
with properties
name=tutorial-10
purpose=Demonstrate the use of graph extension
```

```
create node end
with type End
```

Create a node to use an extension
---------------------------------
Enter the following to create an extension node. The skill is 'graph.extension' and the
'extension' property names the deployed graph model 'tutorial-3'.

The input mapping sets the input parameter(s) of the extension, which is itself a graph model.
The output mapping sets the result from the extension to the output payload.

```
create node extension
with type Extension
with properties
skill=graph.extension
extension=tutorial-3
input[]=input.body.person_id -> person_id
output[]=result -> output.body
```

Connect the nodes to complete the graph model
---------------------------------------------

```
connect root to extension with run
connect extension to end with finish
```

Test the graph model
--------------------
Enter the following to instantiate the graph model with mock input.

```
instantiate graph
int(100) -> input.body.person_id
```

Then do a 'dry-run'.

```
> run
Walk to root
Walk to extension
Executed extension with skill graph.extension in 19.013 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
Graph traversal completed in 20 ms
```

The input of the current graph instance is mapped as an input parameter to the extension
'tutorial-3', and the result is mapped as the output of the graph.

If you inspect the extension node, you will see:

```
> inspect extension
{
  "inspect": "extension",
  "outcome": {
    "result": {
      "address": "100 World Blvd",
      "name": "Peter"
    },
    "live": true,
    "target": "tutorial-3",
    "status": 200
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
```

Check the application log
-------------------------
Complete telemetry information is shown in the application log. You will see that 'tutorial-3' is
invoked as an extension and that it fetches data from the data provider with the input parameter
'person_id'.

```
Call extension tutorial-3, ttl=30000
GET http://127.0.0.1:8100/api/mdm/profile/100, with [person_id], ttl=30000
```

This is a small example, but it demonstrates the pattern: a typical main graph model uses one or
more extensions for API data fetching, then performs decision-making using the retrieved data.

Reusability
-----------
Graph extension promotes reusability. Common use cases can be built as graph models and made
available as "extensions" for other graph models to use.

Export the graph model
----------------------
Now you may save the graph model by exporting it.

```
> export graph as tutorial-10
Graph exported to /tmp/graph/tutorial-10.json
Described in /api/graph/model/tutorial-10/286-8
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-10.json" to your application's
`resources/graph` folder. You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-10 \
  -H "Content-Type: application/json" \
  -d '{ 
    "person_id": 100
}'
```

Summary
-------
In this tutorial, you have created a graph model that uses a graph extension: the 'extension'
property names a deployed graph model, input mappings feed its input.body, and the extension's
output.body comes back as the node's result.
