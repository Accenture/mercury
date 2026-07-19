Execute a single node
---------------------
Run one node's skill in isolation. Graph traversal is paused, so you can
functionally verify a node without walking the whole graph.

Syntax
------
```
execute node {name}
execute {name}
```

Example
-------
```
execute fetcher
```

Notes
-----
- Requires a graph instance (see 'help instantiate').
- The node must have a 'skill' property with exactly one skill route, and
  that route must exist at runtime.
- The node reads from and writes to the instance's state machine exactly as
  it would during a run; use 'inspect' to check the outcome (see
  'help inspect').
- On success the console reports the execution time and the node's exit
  path; the node is marked as seen (see 'help seen').
