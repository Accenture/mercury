Import a graph model or a node
------------------------------
Load an exported graph model into your session as a draft for review and
update, or copy a single node from another graph model.

Syntax
------
```
import graph from {name}
import node {node-name} from {graph-name}
```

Example
-------
```
import graph from helloworld
import node fetcher from helloworld
```

Notes
-----
- The name uses letters, digits and hyphen; do not add a ".json" extension.
- 'import graph' looks in the Playground temp folder first (where
  'export graph' writes). When the file is not there, it falls back to the
  graph models deployed with the application. The message "Graph model not
  found in /tmp/graph/... Found deployed graph model" is this normal
  fallback, not an error - the deployed model is imported as your draft.
- 'import node' copies one node (its type and properties, not its
  connections) from an exported graph model in the temp folder - export the
  source graph first. If a node with the same name already exists in your
  draft, it is overwritten.
- Best practice: publish a common graph model holding reusable nodes
  (modules and skills) so team members can import them into their own
  graph models.
- In the Playground UI, the Graph view's "Import Graph" button (also on the
  empty canvas) and a graph JSON file dropped on the canvas import a model
  from your computer as your draft (POST /api/graph/import/{session-id}).
  The file must be a JSON object with a "nodes" section; "connections" is
  optional (a work in progress may have none); any other top-level section
  is refused by name, and a node without alias or types is refused too.
  CompileGraph remains the quality gate when the model is deployed.
- An import replaces the draft of every member of a shared session, like a
  command, and clears a graph instance; a corrupt model reports "Graph model
  not imported" with the reason. The UI asks before replacing a loaded graph.
