Skill: Graph Suspend
--------------------
When a graph reaches the node with this skill, the workflow state of the graph instance is
persisted to an external state store and the graph run completes normally - the transaction
can resume later through the "graph.resume" skill using the same business correlation ID.

This skill is a superset of "graph.task": the "task" property names the pluggable store
function, but the persistence envelope is assembled by the skill itself, so the node needs
no input or output data mapping.

The node carrying this skill MUST be named "suspend" - a reserved alias like "root" and
"end" - because graph traversal jumps to it by name: when a node with the "suspend=true"
property completes normally, the walker routes to the "suspend" node instead of the node's
normal forward path. A plain connection into the "suspend" node is an unconditional
suspension point. There is exactly one suspend node per graph.

A suspension point must be the sole active branch - do not suspend between a fan-out and
its join; suspend after the join instead. Anything a later step needs must be mapped into
the "model" namespace before the suspension point, because a node's transient "result"
properties do not survive suspension - the model is the workflow's durable memory.

Unless the graph staged its own output before suspension, the skill stages a default
response body so the caller of the suspended run receives a meaningful reply:

{
  "type": "suspended",
  "cid": "<business correlation ID>"
}

Route name
----------
"graph.suspend"

Setup
-----
To enable this skill, create a node named "suspend" with "skill=graph.suspend".

The following parameters are required in the properties of the node:

1. task - the route name of the state-store function (e.g. "v1.redis.persist.model")
2. ttl - the record's time-to-live using duration syntax, e.g. 20s, 5m, 2h, 2d

The store function receives headers "type=put" and a body of:

{
  "cid":   "<business correlation ID - the retrieval key>",
  "node":  "<the suspension point - the node that routed here>",
  "ttl":   <seconds>,
  "model": { the model namespace minus the per-run reserved keys },
  "seen":  { traversal bookkeeping },
  "run":   { traversal bookkeeping }
}

The store must acknowledge with a 2xx reply before the graph completes - a failed store
call fails the node (the optional "exception" property routes it to a handler node).

Example
-------
create node suspend
with type Suspend
with properties
purpose=Persist workflow state to the external state store
skill=graph.suspend
task=v1.redis.persist.model
ttl=2d
