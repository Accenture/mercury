Skill: Graph Island
-------------------
Marks an isolated node. A node with this skill always returns ".sink", so
graph traversal never continues through it. That isolation is the point: the
island anchors the graph's knowledge layer. Dictionary, Provider, data-entity,
and reusable Module nodes hang off the island, turning the graph into its own
entity-relationship diagram - living documentation of the enterprise knowledge
behind the execution path.

Route name
----------
"graph.island"

Properties
----------
```
skill=graph.island
```

No other properties are required or accepted.

Example
-------
```
create node dictionary
with type Island
with properties
skill=graph.island
```

Wire the knowledge layer under it:

```
connect root to dictionary with contains
connect dictionary to person-profile with data
connect dictionary to account-detail with data
connect person-profile to mdm-profile with provider
connect account-detail to account-api with provider
```

Notes
-----
- Required convention: leave no node unconnected. Whenever the graph has
  off-path nodes - Dictionary/Provider configuration, data-entity, or
  reusable Module nodes - wire every one of them into the island structure:
  root -[contains]-> island -[data]-> dictionary -[provider]-> provider,
  and island -[module]-> module for reusable graph.math modules.
- Encouraged even for graphs with no off-path nodes: data-entity nodes that
  document the domain model (entities, fields, which fields are
  internal-only) make even a small graph discoverable enterprise knowledge.
- Relation labels are free-form and descriptive; "contains", "data",
  "provider" and "module" are the shipped conventions - choose names that
  capture the real-world relationship.
- Traversal is unaffected: the island sinks, so the run log shows a single
  "Executed ... with skill graph.island" line and the execution path never
  enters the knowledge layer.
- Reusable modules are documented under 'help graph-math'; the Dictionary and
  Provider configuration nodes under 'help data-dictionary'.
