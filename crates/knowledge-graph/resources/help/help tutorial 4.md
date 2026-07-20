Tutorial 4
----------
In this tutorial, you will set up simple mathematics and boolean operations in a
graph model to make a decision.

Exercise
--------
You will create a root node, an end node and a decision node.

To clear the previous graph session, click the Tools button in the top-right corner
and click the "Stop" and "Start" toggle button. A new graph session will start.

Create root and end nodes
-------------------------
Enter the "create node" command for the "root" and "end" nodes first.

```
create node root
with type Root
with properties
name=tutorial-4
purpose=Demonstrate decision making using mathematics and boolean operations
```

Assume there are two input parameters (a and b). The 'decision' node will add the two
numbers, and the end node will echo the input parameters and the sum.

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
Create a node with the skill 'graph.math' to do decision-making.

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

The skill "graph.math" supports these statement types:

| Type         | Operation                                                       |
|--------------|-----------------------------------------------------------------|
| COMPUTE      | generate a value (LHS) from a mathematics expression (RHS)      |
| IF-THEN-ELSE | evaluate a boolean condition and select the next node           |
| MAPPING      | perform a data mapping operation                                |
| EXECUTE      | run another graph.math node's statements inline (module reuse)  |
| RESET        | reset the current state of one or more nodes                    |

The 'RESET' and 'EXECUTE' features are covered in more advanced tutorials.
Enter "help graph-math" for the full statement grammar.

Use the 'triple single quote' syntax to enter the IF-THEN-ELSE statement as one
multi-line value.

The IF line is a boolean expression. THEN names the next step when the expression is
true; ELSE names the next step when it is false. Each may be a node name or the
keyword 'next'.

Statements are evaluated in order. A branch that resolves to 'next' falls through to
the statements after the IF-THEN-ELSE — in this example, the two MAPPING statements
that set the positive-case output key-values. A branch that jumps to a named node
(here 'less-than') ends the statement list immediately, so those mappings do not run.

The curly brace syntax `{key}` substitutes the value of the bracketed key inside a
COMPUTE or IF expression. A MAPPING statement does not use curly braces — it is data
mapping only, where the left-hand side is a constant, an input parameter or a model
variable, and the right-hand side is a model or output variable.

Create a node to handle the negative case
-----------------------------------------
Create a node called "less-than" to handle the negative case from the decision node.

```
create node less-than
with type Reject
with properties
mapping[]=text(a < b) -> output.body.message
mapping[]=boolean(true) -> output.body.less_than
skill=graph.data.mapper
```

Connect the nodes
-----------------

```
connect root to decision with evaluate
connect less-than to end with negative
connect decision to end with positive
```

The "less-than" node is reached only when the decision node evaluates "a < b", so it
does not need a connection from the root. When it finishes, it hands off to the "end"
node. A "list connections" command shows:

```
> list connections
root -[evaluate]-> decision
decision -[positive]-> end
less-than -[negative]-> end
```

You can also use the "describe node" command to see a node's content and connections:

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
        "IF: {input.body.a} >= {input.body.b}
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

Test the positive case
----------------------
To test the positive case, mock the input values and instantiate the graph model.
Note that "start" is an alias of "instantiate".

```
start graph
int(100) -> input.body.a
int(50) -> input.body.b
```

Then test the graph model with the "run" command:

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

Note that "sum" is 150.0 — a COMPUTE statement evaluates to a floating-point number,
so an integer result serializes with a decimal point (it is numerically exact).

Test the negative case
----------------------

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
Save the graph model by exporting it.

```
> export graph as tutorial-4
Graph exported to /tmp/graph/tutorial-4.json
Described in /api/graph/model/tutorial-4/804-24
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-4.json" to your application's
resources/graph folder. You can then test the deployed model with a curl command.

Summary
-------
In this tutorial, you created a graph model that adds two numbers, compares them and
returns a decision.

While this is a trivial example, it demonstrates that you can build useful computation
and evaluation logic in an Active Knowledge Graph using just simple mathematics and
boolean operation statements.
