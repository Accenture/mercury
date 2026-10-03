Export a graph model
--------------------
Write the current graph model as a JSON file for deployment or later
re-import.

Syntax
------
```
export graph as {name}
```

Example
-------
```
export graph as helloworld
```

Notes
-----
- The name uses letters, digits and hyphen; do not add a ".json" extension.
- The file is written to the Playground temp folder (configuration key
  location.graph.temp, default /tmp/graph).
- The export sets name={name} on the root node. If the root node's "name"
  property differs from {name} and the target file already exists, the
  export is refused - update the root node's name to overwrite the existing
  model. If no root node exists, one is created automatically.
- Export fails when the graph has orphan nodes: every node must connect to
  at least one other node (see 'help connect').
- The reply includes "Described in /api/graph/model/{name}/{token}", a
  read-only HTTP view of the exported model.
- In the Playground UI, the Download button beside Copy (Graph and Raw views)
  saves the model to your computer as {name}.json after asking for the graph
  id; the root node's name in the file becomes that id, as this command does.
  Browsers with a "save as" dialog let you pick the folder; others save to
  the download folder. Drop the file on the canvas, or use "Import Graph", to
  load it into a session again.
