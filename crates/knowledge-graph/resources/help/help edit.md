Edit a node
-----------
A convenience command: prints an existing node as a complete 'update node'
command so you can copy it, edit the text, and submit the update.

Syntax
------
```
edit node {name}
```

Example
-------
```
edit node demo-node
```

Sample output
-------------
```
update node demo-node
with type Demo
with properties
hello=world
test='''
this is a sample multi-line value
line two
line three
'''
good=day
```

Notes
-----
- The printed command carries the node's current type and all properties,
  flattened to one key per line; list properties print one key[]=entry line
  per element, in order.
- Multi-line values are wrapped in triple single quotes.
- Edit the printed text and submit it as-is to apply the change (see
  'help update'). The node must exist, or the command reports an error.
