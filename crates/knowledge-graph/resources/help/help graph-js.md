Skill: Graph JS (retired)
-------------------------
The graph.js skill is RETIRED in this Rust port for security reasons - the
engine deliberately ships no embedded JavaScript runtime.

Any node configured with skill=graph.js is rejected at execution time with:

```
Skill graph.js is retired for security reasons - use graph.math or graph.task instead
```

What to use instead
-------------------
- graph.math - inline computation and IF/THEN/ELSE decision-making with a
  narrow math/boolean expression dialect. See 'help graph-math'.
- graph.task - invoke a composable function by route name for any logic an
  inline expression cannot express. See 'help graph-task'.

Notes
-----
- Do not author new graph.js nodes.
- When importing an older graph model that contains graph.js nodes, replace
  skill=graph.js with graph.math (compute/branch) or graph.task (custom
  logic) before running it.
