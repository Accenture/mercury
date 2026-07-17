Tutorial 4
----------
In this session, you will setup simple mathematics and boolean operations in a graph model to make decision.

Exercise
--------
You will create a root node, an end node, a decision node as an exercise.

To clear the previous graph session, click the Tools button in the top-right corner and click the "Stop" and "Start"
toggle button. A new graph session will start.

Create root and end nodes
-------------------------
Enter the "create node" command for "root" and "end" nodes first.

```
create node root
with type Root
with properties
name=tutorial-4
purpose=Demonstrate decision making using mathematics and boolean operations
```

Assume there are two input parameters (a and b) and the 'decision' node will add the two numbers,
the end node will echo the input parameters and the sum of the two numbers.

```
create node end
with type End
with properties
skill=graph.data.mapper
mapping[]=input.body.a -> output.body.a
mapping[]=input.body.b -> output.body.b
mapping[]=decision.result.c -> output.body.sum
```

Create a decision node
----------------------
You may create node with skill 'graph.math' to do decision-making.

```
create node decision
with type Decision
with properties
skill=graph.math
statement[]=COMPUTE: c -> {input.body.a} + {input.body.b}
statement[]='''
IF: {input.body.a} >= {input.body.b}
THEN: next
ELSE: less-than
'''
statement[]=MAPPING: text(a >= b) -> output.body.message
statement[]=MAPPING: boolean(false) -> output.body.less_than
```

The skill "graph.math" supports statements for:

| Type         | Operation                                                    |
|--------------|--------------------------------------------------------------|
| COMPUTE      | to generate a value (LHS) from a mathematics operation (RHS) |
| IF-THEN-ELSE | to evaluate a condition with a boolean operation             |
| MAPPING      | to perform a data mapping operation                          |
| RESET        | to reset the current state of one or more nodes              |

We will discuss 'reset' feature in a more advanced tutorial chapter later.

You can use the 'triple single quote' syntax to create the IF-THEN-ELSE statement.

The IF statement is a boolean operation.
The THEN is the next step or another node when the IF statement is true.
The ELSE is the next step or another node when the IF statement is false.

Statements are evaluated in order. The 'next' statement refers to the one after the current IF-THEN-ELSE.
In the above example, the next statements are doing data mapping to set output key-values.

Create a node to handle the negative case
-----------------------------------------
Let's create a node called "less-than" to handle the negative case from the decision node.

```
create node less-than
with type Reject
with properties
mapping[]=text(a < b) -> output.body.message
mapping[]=boolean(true) -> output.body.less_than
skill=graph.data.mapper
```

The curly brace syntax `{}` is used to tell the system to get the value from the bracketed key.

A mapping statement does not need the curly brace syntax because it is designed for data mapping only where
the left-hand-side is a constant, an input parameter or a model variable and the right-hand-side is a model
variable or an output variable.

Connect the nodes
-----------------

```
connect root to decision with evaluate
connect less-than to end with negative
connect decision to end with positive
```

The "less-than" node is invoked by the decision node if "a < b". Therefore, it does not need to connect to the "root".
When it finishes execution, it will hand off to the "end" node. If you do a "list connections" command, you will see:

```
> list connections
root -[evaluate]-> decision
decision -[positive]-> end
less-than -[negative]-> end
```

You can also use the "describe node" command to see connections:

```
> describe node decision
{
  "node": {
    "types": [
      "Decision"
    ],
    "alias": "decision",
    "id": "c9b30d7d8a6c4d49a88b5a9254fe44e2",
    "properties": {
      "skill": "graph.math",
      "statement": [
        "COMPUTE: c -> {input.body.a} + {input.body.b}",
        "IF: {input.body.a} > {input.body.b}
         THEN: next
         ELSE: less-than",        
        "MAPPING: text(a >= b) -> output.body.message",
        "MAPPING: boolean(false) -> output.body.less_than"
      ]
    }
  },
  "from": [
    "root"
  ],
  "to": [
    "end"
  ]
}
```

Test positive case
------------------
To test a positive case, you can mock input value and instantiate the graph model. 
Note that "start" is an alias of "instantiate".

```
start graph
int(100) -> input.body.a
int(50) -> input.body.b
```

Then you can test the graph model with the "run" command:

```
> run
Walk to root
Walk to decision
Executed decision with skill graph.math in 0.824 ms
Walk to end
Executed end with skill graph.data.mapper in 0.099 ms
{
  "output": {
    "body": {
      "a": 100,
      "b": 50,
      "less_than": false,
      "sum": 150.0,
      "message": "a >= b"
    }
  }
}
Graph traversal completed in 7 ms
```

Test negative case
------------------

```
start graph
int(180) -> input.body.a
int(250) -> input.body.b
```

When you do a dry-run, it shows the following:

```
> run
Walk to root
Walk to decision
Executed decision with skill graph.math in 0.394 ms
Walk to less-than
Executed less-than with skill graph.data.mapper in 0.054 ms
Walk to end
Executed end with skill graph.data.mapper in 0.051 ms
{
  "output": {
    "body": {
      "a": 180,
      "b": 250,
      "less_than": true,
      "sum": 430.0,
      "message": "a < b"
    }
  }
}
Graph traversal completed in 2 ms
```

Export the graph model
----------------------
You may save the graph model by exporting it.

```
> export graph as tutorial-4
Graph exported to /tmp/graph/tutorial-4.json
Described in /api/graph/model/tutorial-4/804-24
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-4.json" to your application's `main/resources/graph` folder.
You can then test the deployed model with a curl command.

Summary
-------
In this session, you have created a graph model to add two numbers together, compare the two numbers and return
a decision.

While this is a trivial example, it demonstrates that you can create very useful computation and evaluation
logic using an Active Knowledge Graph that contains just simple mathematics and boolean operation statements.
