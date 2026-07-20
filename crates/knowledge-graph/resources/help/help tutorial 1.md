Tutorial 1
----------
Welcome to the MiniGraph Playground, the self-service user interface for creating
applications with the Active Knowledge Graph.

In this tutorial, you will create the simplest possible application: a graph model
that returns a "hello world" message.

Exercise
--------
If you can see this page, you have successfully started the MiniGraph Playground in a
browser and connected to a designer workbench session.

If your session is disconnected, select the "Tools" dropdown in the top-right corner,
click MiniGraph's start toggle and select "MiniGraph".

Create the starting point of a graph
------------------------------------
**Create a root node** — the starting point of every graph model.
Select multiline and enter the following command in the bottom-right input box.

```
create node root
with type Root
with properties
purpose=Tutorial one to return a 'hello world' message
```

The console displays:

```
> create node root...
Graph with 1 node described in /api/graph/model/ws-875677-2/165-1
```

A drawing appears on the right-hand side under the "Graph" tab: a graph with a single
node called "root" has been created.

`ws-875677-2` is the session ID of the workbench.
`165-1` is a random number for the session that you can ignore.

Create an end node
------------------
An end node is the exit point of a graph model. Enter the following to create one.

```
create node end
with type End
with properties
skill=graph.data.mapper
mapping[]=text(hello world) -> output.body
```

The console displays:

```
> create node end...
Graph with 2 nodes described in /api/graph/model/ws-875677-2/061-2
```

The `skill=graph.data.mapper` line assigns the data mapper skill to the end node.
A data mapper node performs data mapping when it executes.

The mapping statement `mapping[]=text(hello world) -> output.body` maps the constant
"hello world" to `output.body` — the response payload when the graph is executed.
The `[]` suffix means `mapping` is a list: each `mapping[]=` line appends one statement.

MiniGraph uses the same data mapping syntax as Event Script. For a quick reference,
enter "help graph-data-mapper" in the console.

First attempt to run the graph
------------------------------
To run a graph model, first create an instance of it with the `instantiate graph` command.

The console displays:

```
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
```

Now try to run the graph by entering the `run` command.

The console displays:

```
> run
Walk to root
```

The system starts graph traversal from the starting point, i.e. the root node —
and then nothing happens.

What is missing?
----------------
An Active Knowledge Graph is a "property graph" that contains one or more "active"
nodes. An active node carries a "skill" that is backed by a composable function.

The system traverses the graph from the root node. Nothing happened because there is
no further node to reach after the root node: the two nodes are not yet connected,
so traversal stops before it can reach the end node.

Connecting nodes
----------------
Enter the following command to connect the root node to the end node.

```
connect root to end with done
```

The console displays:

```
> connect root to end with done
node root connected to end
Graph with 2 nodes described in /api/graph/model/ws-875677-2/551-3
```

The graph drawing on the right panel is updated.

Running the graph
-----------------
You now have a graph with a starting point and an ending point, where one node
carries a skill — the end node with its data mapping statement.

Instantiate the graph again and run it by entering the following commands.

```
instantiate graph
run
```

The console displays:

```
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
> run
Walk to root
Walk to end
Executed end with skill graph.data.mapper in 1.736 ms
{
  "output": {
    "body": "hello world"
  }
}
Graph traversal completed in 9 ms
```

Congratulations — you have created your first working MiniGraph.
It returns "hello world" when it runs.

Export the graph
----------------
You may now export the graph so that you can deploy it later.

Enter the export command below:

```
export graph as tutorial-1
```

This exports the graph model in JSON format with the name `tutorial-1`
as "/tmp/graph/tutorial-1.json".

The console displays:

```
> export graph as tutorial-1
Added name=tutorial-1 to Root node
Graph exported to /tmp/graph/tutorial-1.json
Described in /api/graph/model/tutorial-1/436-4
```

Note that the system adds the graph name (its unique "id") to the root node.
This prevents you from accidentally overwriting a different graph model.

Help pages
----------
To learn more about each command used in this tutorial, enter:

```
help create
help connect
help instantiate
help run
help export
```

Summary
-------
In this tutorial, you created the simplest graph model — it returns a "hello world"
message when its graph API endpoint is called — exported it, and tried some help pages.

Well done. Let's move on to "Tutorial 2".
