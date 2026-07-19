Create a node
-------------
Add a node to the current graph model. This is a multi-line command: enter
all lines as one block.

Syntax
------
```
create node {name}
with type {type}
with properties
{key1}={value1}
{key2}={value2}
```

Example
-------
```
create node root
with type Root
with properties
name=helloworld
purpose=Demo graph
```

Notes
-----
- Node names use lowercase letters, digits and hyphen. The names 'root' and
  'end' are reserved: the root node must be named 'root' and the end node
  must be named 'end'.
- Types are descriptive labels, conventionally Capitalized (e.g. Root, End,
  Provider, Dictionary, Fetcher, Island). The type and properties are used
  and validated by the node's skill, if any.
- A node has zero or one skill, set with skill={route}.
- 'with properties' and the key lines are optional. Property values act as
  defaults for the instance model.
- A property key may be composite, using the dot-bracket format; a
  key[]=entry line appends one entry to the list "key" (repeat per entry).
  Values may use the Event Script constant syntax, e.g. text(hello),
  int(100), boolean(true).
- Wrap a multi-line value in triple single quotes (''').
- Best practice: give the root node a "name" property (the graph name) and a
  "purpose" property describing the use case as a one-liner.
