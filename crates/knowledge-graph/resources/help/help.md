MiniGraph
---------
A mini-graph is a property graph designed to run entirely in memory
(default capacity: 750 nodes).

A graph model describes a business use case using graph methodology.
Optionally, you may give a node a special skill so it reacts to incoming
events. A skill is a property with the label "skill" whose value is a
composable function route name.

An instance model is an instance of a graph model used to process one
specific business use case or transaction. In the Playground you create it
with the "instantiate" command, optionally seeding mock input; in a deployed
application it is created when an incoming event arrives. Input data
attributes map to properties of one or more nodes.

Execution of an instance model starts from the root node and walks the graph
until it reaches the end node. The result of the end node is returned to the
calling party.

For a model to be meaningful, at least one node should have a skill to
process the data attributes of other nodes (the "data entities").

For more information about each feature, try the following help topics.

For graph model
---------------
- help create (node)
- help update (node)
- help edit (node)
- help delete (node, connection or cache)
- help connect (node-A to node-B)
- help list (nodes or connections)
- help export (graph model as JSON for deployment)
- help import (graph or node)
- help describe (graph, node, connection or skill)
- help data-dictionary
- help session (display, subscribe or reset session)

For instance model
------------------
- help instantiate (create an instance from the current graph model)
- help upload (mock data)
- help execute (the skill of one node in isolation, for functional testing)
- help inspect (state machine: node properties, input, output and model namespaces)
- help run (traverse a graph instance from the root node to the end node)
- help seen (display the nodes that have been seen or executed)

Built-in skills
---------------
1. graph.data.mapper - map data from one node or namespace to another
2. graph.math - compute and branch with a fast built-in math/boolean expression engine
3. graph.js - retired in this port; use graph.math or graph.task instead
4. graph.api.fetcher - make API calls to other systems via Dictionary and Provider nodes
5. graph.extension - delegate to another graph model or an Event Script flow
6. graph.island - marks the knowledge layer; the node leads to isolated nodes and traversal pauses there
7. graph.join - wait for completion of all nodes that connect to it (parallel-branch barrier)
8. graph.task - invoke a composable function through its route name

For skill details, use the hyphenated help topics, e.g. 'help graph-math',
'help graph-api-fetcher', or 'describe skill {route}'.

Tutorials
---------
- help tutorial 1 (your first 'hello world' graph model)
- help tutorial 2 (deploying a graph model)
- help tutorial 3 (data dictionary, provider and API fetcher)
- help tutorial 4 (decision-making with math and boolean expressions)
- help tutorial 5 (parallel processing with a join barrier)
- help tutorial 6 (iterative API fetching with the 'for_each' keyword)
- help tutorial 7 (data mapping)
- help tutorial 8 (JSON-Path key-value retrieval and search)
- help tutorial 9 (reusable 'modules')
- help tutorial 10 (graph extension)
- help tutorial 11 (flow extension)
- help tutorial 12 (custom error handling)
- help tutorial 13 (invoking a composable function with the graph.task skill)
