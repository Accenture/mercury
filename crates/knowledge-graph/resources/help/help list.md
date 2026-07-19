List nodes or connections
-------------------------
Show all nodes or all connections of the current graph model.

Syntax
------
```
list nodes
list connections
```

Notes
-----
- 'list nodes' prints each node with its type: the root node first, the end
  node last, and the other nodes in alphabetical order. A missing root or
  end node is flagged with "(does not exist)".
- 'list connections' prints one line per connection with its relation
  label(s).
- Use 'describe node {name}' for the full detail of a single node (see
  'help describe').
