Update a node
-------------
Replace the definition of an existing node. This multi-line command has the
same shape as 'create node' (see 'help create'): enter all lines as one
block, and the node takes the type and properties you provide.

Syntax
------
```
update node {name}
with type {type}
with properties
{key1}={value1}
{key2}={value2}
```

Example
-------
```
update node greeting
with type Task
with properties
skill=graph.task
task=v1.hello.task
input[]=input.body -> *
output[]=result -> output.body
```

Notes
-----
- Node names use lowercase letters, digits and hyphen ('root' and 'end' are
  reserved for the root and end nodes).
- Types are descriptive labels, conventionally Capitalized; the type and
  properties are validated by the node's skill, if any.
- A node has zero or one skill, set with skill={route}.
- 'with properties' and the key lines are optional; a key[]=entry line
  appends one entry to the list "key"; wrap a multi-line value in triple
  single quotes ('''). Values may use the Event Script constant syntax.
- Tip: 'edit node {name}' prints an existing node as a ready-to-edit
  'update node' command (see 'help edit').
