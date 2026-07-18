Inspect state machine
---------------------
This command inspects the state machine containing properties of nodes, input, output and model namespaces.

Pre-requisite
-------------
A graph instance is created with the "instantiate" command

Syntax
------
```
inspect {variable_name}
```
`{variable_name}` is a placeholder — substitute your key and do **not** type the
braces (see the examples). A whole namespace (`input` | `output` | `model`) is
also valid, e.g. `inspect output`.

Examples
--------
```
inspect input.body.user_id
inspect book.price
inspect model.some_variable
inspect output.body.some_key
```
