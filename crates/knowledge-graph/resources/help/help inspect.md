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

Examples
--------
```
inspect {input.body.user_id}
inspect {book.price}
inpsect {model.some_variable}
inspect {output.body.some_key}
```
