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
