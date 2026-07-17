Skill: Graph Join
-----------------
A node with this skill will wait for all connected nodes that join to this node to complete.

Execution will start when the GraphExecutor reaches the node containing this skill.

Route name
----------
"graph.join"

Setup
-----
To enable this skill for a node, set "skill=graph.join" as a property in a node.
This node does not require additional properties.

Properties
----------
```
skill=graph.join
```

Execution
---------
Upon successful execution, a node with this skill will return "next" if all connected nodes to finish
processing. Otherwise, it will return ".sink" to tell the system that it is not ready.
