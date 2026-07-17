Create a new node
-----------------
1. Root node must use the name 'root' and end node must use 'end'.
2. Skill is a property with the name 'skill'. A node has zero or one skill.
3. The 'create node' is a multi-line command 
4. Properties are optional for a graph model. If present, they are used as default value. 
5. For each property, you can use the "triple single quotes" to enter a multi-line value if needed. 
6. Node name and type should use lower case characters and hyphen only
7. Type and key-values will be used and validated by the node's skill function if any
8. The key of a property can be a composable key using the dot-bracket format.
   The value may use Event Script's constant syntax.

Syntax
------
```
create node {name}
with type {type}
with properties
{key1}={value1}
{key2}={value2}
...
```

Best practice
-------------
For root node, we recommend adding a "name" property as the graph name and "purpose" property to describe
the use case as a one-liner.

Example
-------
```
create node root
with type Root
with properties
name=helloworld
purpose=Demo graph
...
```
