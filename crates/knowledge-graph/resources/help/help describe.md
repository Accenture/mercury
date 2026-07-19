Describe graph, node, connection or skill
-----------------------------------------
Print the structure of the current graph model, the detail of a node or a
connection, or the documentation of a skill.

Syntax
------
```
describe graph
describe node {name}
describe connection {node-A} and {node-B}
describe skill {skill.route.name}
```

Example
-------
```
describe node fetcher
describe skill graph.api.fetcher
```

Notes
-----
- 'describe graph' shows the structure of the current draft graph model.
- 'describe node' prints a node's type and properties.
- 'describe connection' reports the connections between the two nodes in
  either direction, or that they are not connected.
- 'describe skill' prints the shipped documentation of a skill by its route
  name - the same content as the hyphenated help topic (e.g.
  'help graph-api-fetcher').
