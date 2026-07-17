Skill: Graph Island
-------------------
The purpose of a node with this skill is to tell the system to block graph traversal.

In this way, we can use this node as a connector to data entities and other things that are used to
represent some knowledge. We don't want to system to actively executing the nodes on the "isolated island".

Execution will start when the GraphExecutor reaches the node containing this skill.

Route name
----------
"graph.island"

Setup
-----
To enable this skill for a node, set "skill=graph.island" as a property in a node.
This node does not require additional properties.

Properties
----------
```
skill=graph.island
```

Execution
---------
Upon successful execution, a node with this skill will return ".sink" to tell the system
that there is no need for further traversal.
