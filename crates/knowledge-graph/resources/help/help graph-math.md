Skill: Graph Math
-----------------
Fast inline math and boolean evaluation for computation and decision-making.
A node with this skill runs an ordered list of statement[] lines. This is THE
skill for inline compute/branch in this Rust port (graph.js is retired - see
'help graph-js'). For anything richer than the narrow expression dialect
described below, invoke a composable function instead (see 'help graph-task').

Route name
----------
"graph.math"

Properties
----------
```
skill=graph.math
statement[]=COMPUTE: {var} -> {expression}
statement[]=IF: / THEN: / ELSE:              (multi-line - see below)
statement[]=MAPPING: {source} -> {target}
statement[]=EXECUTE: {node-name}
statement[]=RESET: {node-name}[, {node-name} ...]
```

- statement[] (required) - at least one statement; statements run in order.

Optional:

```
for_each[]={array-source} -> model.{var}     (iterate a statement block)
statement[]=BEGIN / statement[]=END          (delimit the for_each block)
statement[]=NEXT: {node-name}
statement[]=DELAY: {milliseconds}
```

Statements
----------
- COMPUTE: {var} -> {expression} - evaluate the expression; the result is
  stored in THIS node's result namespace, readable as
  {this-node}.result.{var} or moved onward with a MAPPING statement.
- IF - a boolean decision that can redirect traversal (see below).
- MAPPING: {source} -> {target} - data mapping, identical to the data mapper
  (see 'help graph-data-mapper'). Do NOT wrap source/target in curly braces.
  A node with ONLY MAPPING statements is rejected - use graph.data.mapper.
- EXECUTE: {node-name} - run another graph.math node's statements inline, IN
  THE CALLING NODE'S CONTEXT: any COMPUTE results land on the INVOKING node
  ({invoker}.result.{var}); the executed module's own namespace stays empty.
  This is the module-reuse mechanism - author a formula once in an off-path
  Module node reading neutral model.* operands, and any node borrows it.
- RESET: {node-name}[, ...] - clear the run-once guard and state of one or
  MORE nodes (comma/space-separated list). Resetting a never-executed node
  is a safe no-op. Advanced - see Notes.

Expressions
-----------
{namespace.key} substitutes a value from input.*, model.*, or a node's
properties/result into a COMPUTE or IF expression, e.g.
{input.body.discount}, {book.price}, {model.x}. Substitution is robust to
hyphenated names - {unit-price} is the value of "unit-price", never parsed
as a subtraction - so use communicative hyphenated names freely.

The dialect is a NARROW JavaScript-like subset: arithmetic, comparison and
boolean operators only. No bitwise operators, no function calls (e.g. no
parseInt), no variables inside the expression. COMPUTE yields a double, so
an integer result serializes as e.g. 8.0 (numerically exact).

IF / THEN / ELSE
----------------
IF is the decision construct. It is a multi-line statement - enter it as one
statement[] value wrapped in triple single quotes. THEN: and ELSE: are both
REQUIRED, or the engine aborts the run.

```
statement[]='''
IF: {input.body.a} >= {input.body.b}
THEN: ge-path
ELSE: lt-path
'''
```

- THEN: / ELSE: each name the node to jump to, or the keyword "next".
- A taken node-jump ENDS the statement list immediately - later statements
  do not run. A branch resolving to "next" FALLS THROUGH: processing
  continues with the following statements, and natural traversal is
  preserved if nothing else redirects it. Order the list accordingly (e.g.
  an early-exit check first, retry logic after).

Traversal control
-----------------
- NEXT: {node-name} - unconditionally jump to a node BY NAME (a node name,
  not a connection label). Unlike a taken IF jump, NEXT: does not stop
  processing: the remaining statements still run, and the jump applies after
  the whole list completes (the last NEXT: wins).
- DELAY: {milliseconds} - pause after this node completes, before the walk
  continues to the next node. Paces retries; simulates a slow service.
- RESET enables retry loops. A node may reset ITSELF - the run-once mark is
  set before execution, so a self-reset survives and the node can run again.
  Placement rule: put RESET FIRST among the action statements - it then runs
  on every path (a later taken IF jump would skip it) and everything the node
  stores afterwards (such as DELAY's pending pause) survives the self-wipe.
  The one exception: keep RESET after any statement that reads state it would
  wipe - an IF on a just-wiped variable (e.g. {fetcher.status} after
  RESET: fetcher) aborts the run, so a defensive status check goes before it.

Example
-------
```
create node price-check
with type Decision
with properties
skill=graph.math
statement[]=COMPUTE: amount -> (1 - {input.body.discount}) * {book.price}
statement[]='''
IF: (1 - {input.body.discount}) * {book.price} > 5000
THEN: high-price
ELSE: low-price
'''
```

Reusable module - author the formula once, borrow it anywhere:

```
create node addition
with type Module
with properties
skill=graph.math
statement[]=COMPUTE: sum -> {model.a} + {model.b}
```

```
create node calculate
with type Compute
with properties
skill=graph.math
statement[]=MAPPING: input.body.a -> model.a
statement[]=MAPPING: input.body.b -> model.b
statement[]=EXECUTE: addition
statement[]=MAPPING: calculate.result.sum -> output.body.sum
```

Note "calculate.result.sum", not "addition.result.sum" - the caller borrows
the logic, so the result belongs to the caller. Keep the module off the
execution path and hang it under the island knowledge layer
(island -[module]-> addition) - see 'help graph-island'.

Notes
-----
- A node executes ONCE per run (the run-once guard); a RESET statement is
  the only escape, for advanced re-execution. Use it with care.
- Loop guard: a node executed too frequently (default: more than 10 times
  per second) aborts the traversal - bound every retry loop and pace it
  with DELAY:.
- for_each[]={array-source} -> model.{var} iterates a statement block over a
  runtime array; BEGIN / END delimit the block to iterate (they are for_each
  delimiters, not IF braces).
- The bounded-retry pattern (RESET the failing node and itself first, count
  attempts with f:defaultValue + f:add, exit at the bound via a taken IF
  jump, NEXT: back, DELAY: to pace) is shown under 'help graph-api-fetcher'.
