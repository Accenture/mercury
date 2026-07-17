Instantiate from a Graph Model
------------------------------
1. This command creates a graph instance with mock input from the current graph model for development and tests
2. You must do this before using "execute", "inspect" and "run" commands
3. The name does not require the ".json" extension
4. You can tell the system to mock one or more constants as input variables
5. The input namespace contains 'body' and 'header'
6. The model namespace is a state machine. It is optional unless you want to emulate some model variables.

Syntax
------
```
instantiate graph
{constant} -> input.body.{key}
```

Example
-------
```
instantiate graph
int(100) -> input.body.profile_id
text(application/json) -> input.header.content-type
text(world) -> model.hello
```

Alias
-----
`start` is an alias of `instantiate`
