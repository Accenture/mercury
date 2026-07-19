Tutorial 9
----------
In this tutorial, you will create a reusable module — a formula authored once, then borrowed by
any node that needs it.

Exercise
--------
You will create a reusable "addition" module, call it from a compute node with the EXECUTE
statement, and organize the module under an island node.

To clear the previous graph session, click the Tools button in the top-right corner and click the
"Stop" and "Start" toggle button. A new graph session will start.

What is a reusable module?
--------------------------
A module is a node with the graph.math skill that stays off the execution path. For a frequently
used math formula or boolean operation, you can save the "common logic" in one or more module
nodes and export them as a common graph model. When you design a new graph model, you can import
the modules you need from that common model.

This is a best practice for common computation and decision logic: developers do not re-invent the
same formula, and the shared modules encourage quality control and governance.

For this tutorial, we will skip exporting a common graph model and focus on creating a reusable
module and using it in a graph model.

Create a root node and an end node
----------------------------------
Enter the following to create a root node and an end node.

```
create node root
with type Root
with properties
name=tutorial-9
purpose=Demonstrate use of modules
```

```
create node end
with type End
```

Create a reusable module
-------------------------
You will create a simple "addition" module that adds two numbers and saves the result in a
variable called "sum".

```
create node addition
with type Module
with properties
skill=graph.math
statement[]=COMPUTE: sum -> {model.a} + {model.b}
```

Test the module
---------------
Enter the following to start the graph model and set two numbers in the variables "a" and "b" of
the state machine's "model" namespace.

```
instantiate graph
int(10) -> model.a
int(20) -> model.b
```

You can then test the module using 'execute addition'.

```
> execute addition
node addition run for 0.312 ms with exit path 'next'
```

Then you can inspect the node.

```
> inspect addition
{
  "inspect": "addition",
  "outcome": {
    "result": {
      "sum": 30.0
    },
    "decision": "next"
  }
}
```

The module adds the two numbers and saves the result "30.0" into the variable "sum" in the node's
result set. (When executed directly, the result lands on the module itself — the next step shows
what changes when another node executes it.)

Using the new module
--------------------
You will create a new node that uses the module.

```
create node compute
with type Compute
with properties
skill=graph.math
statement[]=MAPPING: input.body.a -> model.a
statement[]=MAPPING: input.body.b -> model.b
statement[]=EXECUTE: addition
statement[]=MAPPING: compute.result.sum -> output.body.sum
```

This node maps the input parameters "a" and "b" into the model variables "a" and "b", executes the
module "addition", then maps the computed value to the output payload "output.body.sum". Note the
last statement reads compute.result.sum — not addition.result.sum — for the reason shown next.

Test the compute node
---------------------
You will instantiate the graph model like this:

```
instantiate graph
int(10) -> input.body.a
int(20) -> input.body.b
```

Then enter 'execute compute'. It maps the input parameters to the model variables and executes the
module "addition" that adds the two model variables together.

Inspect the result
------------------
The result is saved to the variable "sum" under the "compute" node instead of the module
"addition". EXECUTE runs the module's statements in the caller's context: any COMPUTE result lands
on the invoking node (compute.result.sum here), and the module's own namespace stays empty — the
compute node just borrows the logic from the module.

```
> inspect compute
{
  "inspect": "compute",
  "outcome": {
    "result": {
      "sum": 30.0
    },
    "decision": "next"
  }
}
> inspect model
{
  "inspect": "model",
  "outcome": {
    "a": 10,
    "b": 20
  }
}
> inspect addition
{
  "inspect": "addition",
  "outcome": {}
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "sum": 30.0
    }
  }
}
```

The module works as expected.

Connect the nodes
-----------------
You will connect the nodes with the following commands:

```
connect root to compute with calculate
connect compute to end with finish
```

Test the completed model
------------------------
You will enter the following to test the whole model ('start' is an alias of 'instantiate').

```
start graph
int(10) -> input.body.a
int(20) -> input.body.b
```

Then enter 'run' to do a 'dry-run' from the root to the end node.

```
> run
Walk to root
Walk to compute
Executed compute with skill graph.math in 0.387 ms
Walk to end
{
  "output": {
    "body": {
      "sum": 30.0
    }
  }
}
Graph traversal completed in 7 ms
```

Check the nodes and connections
-------------------------------
Enter the following to show the nodes and connections.

```
> list nodes
root [Root]
addition [Module]
compute [Compute]
end [End]
> list connections
root -[calculate]-> compute
compute -[finish]-> end
```

Note that the module "addition" is not part of the traversal path — the compute node that executes
it is. However, the convention is to leave no node unconnected: 'export' fails if any node is an
orphan, and off-path nodes belong in the graph's knowledge structure so the model documents
itself. The next step wires the module in.

Create an island to hold modules
--------------------------------
You will create an island node to organize one or more module nodes. An island is isolated from
graph traversal, so the execution path is unaffected.

```
create node modules
with type Island
with properties
skill=graph.island
```

Then connect the root to the island, and the island to the module.

```
> connect root to modules with contains
node root connected to modules
> connect modules to addition with contains
node modules connected to addition
> list connections
root -[calculate]-> compute
root -[contains]-> modules
modules -[contains]-> addition
compute -[finish]-> end
```

Export the graph model
----------------------
As a good practice, you may save the graph model by exporting it.

```
> export graph as tutorial-9
Graph exported to /tmp/graph/tutorial-9.json
Described in /api/graph/model/tutorial-9/359-15
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-9.json" to your application's
`resources/graph` folder. You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-9 \
  -H "Content-Type: application/json" \
  -d '{ 
    "a": 10,
    "b": 20
}'
```

Summary
-------
In this tutorial, you have created a graph model with a compute node that executes a reusable
module. You have seen that EXECUTE runs the module's statements in the caller's context — the
result lands on the invoking node — and you have organized the module under an island so that no
node is left unconnected.
