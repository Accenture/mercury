Run a graph instance
--------------------
Traverse the current graph instance from the root node to the end node,
executing every node that has a skill along the way.

Syntax
------
```
run
```

Notes
-----
- Requires a graph instance (see 'help instantiate').
- Traversal starts at the root node. Multiple outgoing connections fork into
  parallel branches (synchronize them with graph.join); each node executes
  at most once per run (loop guard).
- Every run ends with either "Graph traversal completed in N ms" or
  "Graph traversal aborted"; on failure, the reason is printed before the
  aborted line.
- 'run' may be repeated on the same instance: each run clears the visited
  set and the output namespace, but model values persist across runs -
  instantiate again for a completely fresh state.
- Use 'seen' to list the nodes visited by the last run, and 'inspect' to
  read the results (e.g. 'inspect output.body').
