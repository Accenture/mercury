Skill: Graph Data Mapper
------------------------
When a node is configured with this skill of "data mapping", it will execute a set of data mapping entries
to populate data attributes into one or more nodes where each node represents a data entity.

Execution will start when the GraphExecutor reaches the node containing this skill.

Route name
----------
"graph.data.mapper"

Setup
-----
To enable this skill for a node, set "skill=graph.data.mapper" as a property in a node.
One or more data mapping entries can be added to the property "mapping".

Properties
----------
```
skill=graph.data.mapper
mapping[]=source -> target
```

The system uses the same syntax of Event Script for data mapping.

Execution
---------
Upon successful execution, key-values will be populated to one or more nodes.

Syntax for mapping
------------------
source.composite.key -> target.composite.key

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
create node my-simple-mapper
with properties
skill=graph.data.mapper
mapping[]=input.body.hr_id -> employee.id
mapping[]=input.body.join_date -> employee.join_date
```

The "[]" syntax is used to create and append a list of one or more data mapping entries
The "->" signature indicates the direction of mapping where the left-hand-side is source and right-hand-side is target

Deprecated syntax
-----------------
Event Script's "simple type matching" syntax (e.g. `model.someKey:text`) is deprecated. Use "simple plugin"
syntax instead (e.g. `f:text(model.someKey)`). If you (or an AI agent) submit a "create node" or "update node"
command that still uses the deprecated colon-type syntax, the system will automatically convert it to the
simple plugin syntax and return a deprecation notice - it will not silently fail, but please switch to the
new syntax going forward.
