Instantiate a graph instance
----------------------------
Create a runnable instance of the current graph model, optionally seeded
with mock input for development and testing. Required before the 'run',
'execute' and 'inspect' commands. This is a multi-line command: enter all
lines as one block.

Syntax
------
```
instantiate graph
{constant} -> input.body.{key}
{constant} -> input.header.{key}
{constant} -> model.{key}
```

Example
-------
```
instantiate graph
int(100) -> input.body.profile_id
text(application/json) -> input.header.content-type
text(world) -> model.hello
```

Notes
-----
- The seed lines are optional. Each line assigns a constant (text(...),
  int(...), boolean(...), etc.) to the input.body, input.header or model
  namespace - no other targets are accepted. The model namespace is the
  state machine; seed it only to emulate model variables.
- Seed keys may be composite (dot-bracket), so nested mock payloads seed
  directly:

```
instantiate graph
text(Peter) -> input.body.profile.name
text(100 World Blvd) -> input.body.profile.address1
```

- The graph must have a root node and an end node.
- Instantiating replaces any previous instance of your session.
- The reply reports the number of mock entries loaded and the instance's
  model.ttl (default 30000 ms), the execution time budget - seed model.ttl
  to change it.
- 'start' is an alias of 'instantiate'.
- To mock a large input.body with a JSON payload, see 'help upload'.
