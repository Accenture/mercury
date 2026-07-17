Tutorial 9
----------
In this session, we will discuss the 'reusable module' use case.

Exercise
--------
You will create a reusable module and put it in a common graph model. Then create another graph model and
import the reusable module into the graph model to reuse it.

To clear the previous graph session, click the Tools button in the top-right corner and click the "Stop" and "Start"
toggle button. A new graph session will start.

What is a reusable module?
--------------------------
A module is a node that contains either the graph.js or graph.math skill. For frequently used math formula
or boolean operation, you can save the "common logic" in one or more module nodes and export it as a common graph model.

When you design a new graph model, you can import one or more reusable modules from the common graph model.

This is a best practice for graph modeling of common computation and decision logic so that developers do not need
to re-invent the same logic. This also encourages quality control and governance.

For this tutorial, we will skip the export of the common graph model and focus in creation of a reusable module
and illustration of how to use it in a graph model.

Create a root node and an end node
----------------------------------
Enter the following to create a root node and an end node

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
You will create a simple "addition" module by adding two numbers and save the result in a variable called "sum".

```
create node addition
with type Module
with properties
skill=graph.math
statement[]=COMPUTE: sum -> {model.a} + {model.b}
```

Test the module
---------------
Enter the following to start the graph model and set two numbers in variable "a" and "b" in the state machine
"model".

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

You can see the module adds the two numbers and save the result "30.0" into the variable "sum" in the result set
of the node.

Using the new module
--------------------
You will create a new node to use the module.

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

In this node, it maps the input parameter "a" and "b" into the model variable "a" and "b".
Then it executes the module "addition". The computed result is saved in the "compute" node.
The last statement maps the computed value to the output payload "output.body.sum".

Test the compute node
---------------------
You will instantiate the graph model like this:

```
instantiate graph
int(10) -> input.body.a
int(20) -> input.body.b
```

Then you enter 'execute compute'. It will invoke the node 'compute' and it maps the input parameters to the model
variables. Then it executes the module "addition" that adds the two model variables together.

Inspect the result
------------------
The result is saved to the variable "sum" under the "compute" node instead of the module "addition".
It is because the compute node is the one that executes the statements.
It just borrows the logic from the module "addition".

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

Now the module works as expected.

Connect the nodes
-----------------
You will connect the nodes with the following commands:

```
connect root to compute with calculate
connect compute to end with finish
```

Test the completed model
------------------------
You will enter the following to test the whole model.

```
start graph
int(10) -> input.body.a
int(20) -> input.body.b
```

and enter 'run' to do a 'dry-run' from the root to the end node.

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
Enter the following to show the nodes and connections

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

Note that the module "addition" does not need to be connected because it is a reusable module. The node that executes
it must be connected so that the graph executor can execute it when the graph traversal starts.

Create an island to hold modules
--------------------------------
You will create an island node to organize one or more module nodes.

```
create node modules
with type Island
with properties
skill=graph.island
```

Then you can connect the data dictionary nodes and provider node to it.

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
To deploy the graph model, copy "/tmp/graph/tutorial-9.json" to your application's `main/resources/graph` folder.
You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-9 \
  -H "Content-Type: application/json" \
  -d '{ 
    "a": 10,
    "b": 20
}'
```

Summary
-------
In this session, you have created a graph model that contains a compute node that executes a reusable module.
