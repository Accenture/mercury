Skill: Graph JS
---------------
When a node is configured with this skill of "graph js", it will execute a set of simple JavaScript statements
to return result. For example, doing mathematical calculation or boolean operation for decision-making.

Execution will start when the GraphExecutor reaches the node containing this skill.

Route name
----------
"graph.js"

Setup
-----
To enable this skill for a node, set "skill=graph.js" as a property in a node.
One or more statements can be added.

There are 5 types of statements:
1. "IF" statement for decision-making
2. "COMPUTE" statement to evaluate a mathematical formula
3. "MAPPING" statement to do data mapping from a source to a target variable
4. "EXECUTE" statement to execute another node with "graph.js" skill
5. "RESET" statement to reset one or more nodes from the state machine

You can configure one or more statements of these 3 types.

The system will reject execution if the node contains only "MAP" statements
because it is more efficient to use the "graph.data.mapper" skills for mapping
only operations.

Statements are executed orderly.

Properties
----------
```
skill=graph.js
statement[]=COMPUTE: variable -> mathematical statement
statement[]=IF: if-then-else statement
statement[]=MAPPING: source -> target
statement[]=EXECUTE: another-node
```

Node cannot be executed more than once
--------------------------------------
To avoid unintended looping, the system guarantees that a node, that has been "seen", is not executed again.

The `reset` command clears the "seen" status and erases its result from the state machine. This is reserved
for advanced use cases that require executing a node more than once. You should use this feature with care.

The following statement resets the node named "previous-node" so that the graph executor can run this node
again when conditional traversal points to the node.

```
statement[]=RESET: previous-node
```

Optional properties
-------------------
```
for_each[]={map an array parameter for iterative statement execution}
statement[]=BEGIN
statement[]=END
statement[]=NEXT: {next-node-name}
statement[]=DELAY: {milliseconds}
```

Execution
---------
Upon successful execution of a "COMPUTE" statement, the result set will be stored in the "result" namespace
of the node. A subsequent "MAPPING" statement can map the key-values in the result set to one or more nodes.

For an "IF" statement, the system will execute a boolean operation.
This process will override the natural graph traversal order and jump to a specific node.
If the function returns "next" after evaluation of all statements, the natural graph traversal order
will be preserved.

Iterative Execution and Begin-End
---------------------------------
Using the optional `for_each` statement, you can tell the skill module to execute the statements iteratively.

A "for_each" statement extracts the next array element from another array variable into a model variable.
You can then put the model variable in the "left-hand-side" of an input statement. The module will then
execute the statement block using an iterative stream of the model variable.

You can also use the `BEGIN` and `END` control statements to select a section of the statements for the
iterative execution based on the "for_each" criteria.

Syntax for COMPUTE statement
----------------------------
It will be a regular JavaScript statement with parameter substitution using the bracket syntax where
the enclosed parameter is a reference to a data attributes in the namespace of "input.", "model." or node name.

When you have more than one JavaScript statement, a subsequent statement can use the result of a prior statement
as its parameters.

Each parameter is wrapped by a set of curly brackets.

Override Graph Traversal
------------------------
Normally the next node is the one or more nodes that this node is connected to.
If you want to tell system to jump to a specific "next-node", you can use the "NEXT:" syntax and put the name
of the node to jump to.

Deferred completion
-------------------
You can add an artificial delay to defer completion of the execution of this node. This is useful to simulate
a slow service for performance test and to pause between retries.

Next and Delay statements
-------------------------
It is a good practice to place the next or delay statement, if any, as last one in the block.
However, the placement does not change the behavior because they will only be processed at the end.

Limitation
----------
This skill is designed to execute a simple inline JavaScript statement that uses standard JavaScript library.
Complex functions and variables are not recommended.

Example
-------
```
create node demo-js-runner
with properties
skill=graph.js
statement[]=COMPUTE: amount -> (1 - {input.body.discount}) * {book.price}
```

The syntax `{variable_name}` is used to resolve the value from the variable into the COMPUTE statement.

Syntax for IF statement
-----------------------
Each IF statement is a multiline command:
```
IF: JavaScript-statement
THEN: node-name | next
ELSE: node-name | next
```

The "next" keyword tells the system to execute the next statement.

The if-then-else is used to select two options after evaluation of the JavaScript statement.
If the JavaScript statement does not return a boolean value, the following resolution would apply:
1. numeric value - true is positive value and false is negative value
2. text value - "true", "yes", "T", "Y" are positive and all other values are false
3. other value will be converted to a text string first

Example
-------
```
statement[]='''
IF: (1 - {input.body.discount}) * {book.price} > 5000
THEN: high-price
ELSE: low-price
```

The syntax `{variable_name}` is used to resolve the value from the variable into the IF statement.

Syntax for MAPPING statement
----------------------------
MAPPING: source.composite.key -> target.composite.key

The source composite key can use the following namespaces:
1. "input." namespace to map key-values from the input header or body of an incoming request
2. Node name (aka 'alias') to map key-values of a node's properties
3. "model." namespace for holding intermediate key-values for simple data transformation

The target composite key can use the following namespaces:
1. "output." namespace to map key-values to the result set to be returned as response to the calling party
2. Node name (aka 'alias') to map key-values of a node's properties
3. "model." namespace for holding intermediate key-values for simple data transformation

Example
-------
```
statment[]=MAPPING: input.body.hr_id -> employee.id
statement[]=MAPPING: input.body.join_date -> employee.join_date
```

Note that the MAPPING statement operates exactly in the same way as a data-mapper so there is
no need to use curly braces to wrap around variables.

Syntax for EXECUTE statement
----------------------------
EXECUTE: another-node

Example
-------
```
statment[]=EXECUTE: js-3
```

The "[]" syntax is used to create and append a list of one or more statements
