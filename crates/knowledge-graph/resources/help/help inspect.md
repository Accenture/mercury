Inspect the state machine
-------------------------
Read a value from the current graph instance's state machine: node
properties, and the input, output and model namespaces.

Syntax
------
```
inspect {key}
```

`{key}` is a placeholder - substitute your key and do not type the braces.
A whole namespace (input | output | model) is also valid, e.g.
'inspect output'.

Example
-------
```
inspect output
inspect input.body.user_id
inspect model.some_variable
inspect output.body.some_key
inspect book.price
```

Notes
-----
- Requires a graph instance (see 'help instantiate').
- Keys may be composite (dot-bracket), e.g. output.body.profile[0].name.
- A node's properties and results are addressed by node name, e.g.
  book.price or fetcher.result.name.
- A value too large for the console is redirected: the reply prints a
  GET /api/inspect/... URL to download it instead.
