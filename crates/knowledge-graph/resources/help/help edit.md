Edit a node
-----------
This is a convenience feature to populate an "update node" command with raw input data.

Syntax
------
```
edit node {name}
with type {type}
with properties
{key1}={value1}
{key2}={value2}
...
```

Example
-------
```
edit node demo-node
...
```

The above command will print the raw input data of "demo-node" if it exists.
You can then edit the raw input data and submit the update.

Sample output
-------------
```
update node demo-node
with type Demo
with properties
hello=world
test='''
this is a sample multiple key-value
line two
line three
'''
good=day
...
```
