Skill: Graph JS (deprecated)
----------------------------
The graph.js skill evaluates inline JavaScript statements for computation and
decision-making. It is DEPRECATED: do not author new graph.js nodes. Use
graph.math for inline computation and IF/THEN/ELSE decisions - its expression
dialect covers the same ground without a script engine (see 'help graph-math') -
and graph.task to invoke a composable function for any logic an inline
expression cannot express (see 'help graph-task').

Status by engine
----------------
- Java engine: still registered so that existing graph models keep running;
  the module is scheduled for removal once field installations have migrated.
  Runtime JavaScript is an injection surface, and an equality comparison with a
  quoted string literal was measured silently evaluating to false - the reasons
  are recorded in the skills reference (graph.js section).
- Rust engine: never registered. A node with skill=graph.js fails at execution
  time with:

```
Skill graph.js is retired for security reasons - use graph.math or graph.task instead
```

Migrating a graph model
-----------------------
When importing an older graph model that contains graph.js nodes, replace
skill=graph.js with graph.math (compute/branch) or graph.task (custom logic)
before running it. The statement grammar has the same shape - COMPUTE, IF,
MAPPING, EXECUTE, RESET, NEXT, DELAY and for_each with BEGIN/END - so most
nodes migrate by changing the skill and rewriting each expression in the
graph.math dialect (a closed set of operators and functions, listed under
'help graph-math').

Reference for existing graph.js nodes (Java engine only)
--------------------------------------------------------
Route name: "graph.js"

Properties:

```
skill=graph.js
statement[]=COMPUTE: variable -> JavaScript statement
statement[]=IF: if-then-else statement
statement[]=MAPPING: source -> target
statement[]=EXECUTE: another-node
statement[]=RESET: node-name
```

Optional properties:

```
for_each[]={map an array parameter for iterative statement execution}
statement[]=BEGIN
statement[]=END
statement[]=NEXT: {next-node-name}
statement[]=DELAY: {milliseconds}
```

- Statements execute in order. A node with only MAPPING statements is rejected -
  use graph.data.mapper for mapping-only work.
- A COMPUTE result is stored in the node's "result" namespace; a later MAPPING
  statement can map it onward.
- An IF statement evaluates a boolean operation and may override the natural
  traversal order by jumping to a named node; when every statement resolves to
  "next", natural traversal is preserved.
- A node executes once per run (the run-once guard). RESET: clears a node's
  "seen" status and its result so conditional traversal can run it again;
  use it with care.
- for_each[] with BEGIN/END iterates a statement block over a runtime array,
  NEXT: jumps to a named node and DELAY: pauses before the next node - the same
  rules as graph.math. Every statement command resolves {dynamic variables}, so
  NEXT:/THEN:/ELSE: targets, RESET: entries and DELAY: values may each be a
  {namespace.key} reference (e.g. NEXT: {error.source} in a generic error
  handler).
- The skill is designed for a simple inline JavaScript statement using the
  standard JavaScript library; complex functions and variables are not
  supported.

COMPUTE statement:

```
create node demo-js-runner
with properties
skill=graph.js
statement[]=COMPUTE: amount -> (1 - {input.body.discount}) * {book.price}
```

The syntax {variable_name} resolves a value from the "input." or "model."
namespace or from a node's properties into the statement. A later statement
can use the result of a prior statement as its parameter.

IF statement - a multi-line command:

```
statement[]='''
IF: (1 - {input.body.discount}) * {book.price} > 5000
THEN: high-price
ELSE: low-price
'''
```

THEN: and ELSE: each name the node to jump to, or the keyword "next". When the
JavaScript statement does not return a boolean, the result is coerced: a
positive number is true and a negative number false; the text values "true",
"yes", "T" and "Y" are true and any other text is false; any other value is
converted to text first. (graph.math never coerces - a boolean is never a
number there.)

MAPPING statement - identical to the data mapper, so no curly braces:

```
statement[]=MAPPING: input.body.hr_id -> employee.id
statement[]=MAPPING: input.body.join_date -> employee.join_date
```

EXECUTE statement - runs another graph.js node's statements:

```
statement[]=EXECUTE: js-3
```

The "[]" suffix appends one statement per line to the node's statement list.
