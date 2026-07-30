Tutorial 14
-----------
In this session, you will build a purchase workflow with THREE human checkpoints - a customer
orders, the store manager approves, the delivery department releases the shipment, and the
parcel ships to the customer. One graph model, four short runs, one correlation ID.

Pre-requisite
-------------
Workflow suspension persists state to an external store through two composable functions. This
tutorial uses the Redis store from the "minigraph-state-redis" extension - the playground
application already includes it, so "v1.redis.persist.model" and "v1.redis.retrieve.model" are
registered automatically. Start a Redis before you run the graph (the "redis-standalone" helper
application works out of the box).

What is workflow suspension?
----------------------------
An approval may take minutes or days. Instead of parking a live graph instance, the graph
persists its workflow state - the "model" namespace - under the business correlation ID and the
run completes normally. A later request with the same correlation ID restores that state and
continues past the checkpoint without re-executing it. Three vocabulary pieces make this work:

1. the "suspend" node - a reserved node name (like root and end) with the "graph.suspend" skill;
   traversal jumps to it by name. ONE suspend node serves every checkpoint in the graph.
2. a suspensible node - any skilled node with the "suspend=true" property; it routes to the
   suspend node after its skill completes
3. the resume node - the "graph.resume" skill placed right after root; it restores a persisted
   record and jumps past the LAST checkpoint, or lets a fresh transaction flow through - either
   way it sets "model.run" to "resume" or "fresh" so the graph's own logic can react

The graph navigation is:

```
root -> resume -> order (suspend=true) -> approval (suspend=true) -> delivery (suspend=true) -> ship -> end
```

Each suspensible node captures its actor's input into the model and suspends; each following
run resumes one checkpoint further. The model is the workflow's durable memory - anything a
later step needs must be mapped into "model.*" before the checkpoint.

Create the graph model
----------------------
Create the root node:

```
create node root
with properties
purpose=Purchase workflow with three human checkpoints
name=tutorial-14
```

Create the resume node. A resumed run jumps past its last checkpoint; a fresh transaction
(no record - never suspended, or expired) continues along the forward path into the
"check-fresh" validation gate with "model.run" set to "fresh":

```
create node resume
with type Resume
with properties
purpose=Restore workflow state if this transaction was suspended earlier
skill=graph.resume
task=v1.redis.retrieve.model
```

Create the input validation gate. The variable substitution inside the text() constant is
null-safe: when the request has no "item" field it is not an order submission, so a later-stage
request without a suspended record is rejected:

```
create node check-fresh
with type Decision
with properties
purpose=A fresh transaction must be an order submission
skill=graph.math
statement[]=MAPPING: text(={input.body.item}) -> model.order_probe
statement[]=IF: {model.order_probe} == '=null'
THEN: reject
ELSE: order
```

Create the three checkpoint nodes. Each captures its actor's input into the model, stages a
stage-specific reply for the caller (overriding the default suspended response), and carries
"suspend=true" so traversal routes to the suspend node when it completes:

```
create node order
with type Suspensible
with properties
purpose=Capture the customer order, then suspend for the store manager
skill=graph.data.mapper
suspend=true
mapping[]=input.body -> model.order
mapping[]=text(order-submitted; waiting for store manager approval) -> output.body.stage
mapping[]=model.run -> output.body.run
mapping[]=model.cid -> output.body.cid
```

```
create node approval
with type Suspensible
with properties
purpose=Capture the store manager approval, then suspend for the delivery department
skill=graph.data.mapper
suspend=true
mapping[]=input.body -> model.approval
mapping[]=text(approved; waiting for the delivery department to release the shipment) -> output.body.stage
mapping[]=model.run -> output.body.run
mapping[]=model.cid -> output.body.cid
```

```
create node delivery
with type Suspensible
with properties
purpose=Capture the shipment release, then suspend for shipment confirmation
skill=graph.data.mapper
suspend=true
mapping[]=input.body -> model.delivery
mapping[]=text(released; waiting for shipment confirmation) -> output.body.stage
mapping[]=model.run -> output.body.run
mapping[]=model.cid -> output.body.cid
```

Create the completion, rejection, suspend and end nodes:

```
create node ship
with type mapper
with properties
purpose=Ship to the customer with the full order history
skill=graph.data.mapper
mapping[]=text(shipped) -> output.body.stage
mapping[]=model.run -> output.body.run
mapping[]=model.order -> output.body.order
mapping[]=model.approval -> output.body.approval
mapping[]=model.delivery -> output.body.delivery
mapping[]=input.body -> output.body.shipment
mapping[]=model.cid -> output.body.cid
```

```
create node reject
with type mapper
with properties
purpose=Reject a request that has no suspended transaction and is not an order
skill=graph.data.mapper
mapping[]=int(404) -> output.status
mapping[]=text(rejected) -> output.body.type
mapping[]=text(Transaction not found. Submit the order first) -> output.body.message
mapping[]=model.run -> output.body.run
```

```
create node suspend
with type Suspend
with properties
purpose=Persist workflow state to Redis and wait for the next actor
skill=graph.suspend
task=v1.redis.persist.model
ttl=1h
```

```
create node end
```

Connect the nodes. Every suspensible node draws BOTH edges - the checkpoint edge to "suspend"
and the continuation edge to the next step - so the diagram tells the whole story:

```
connect root to resume with then
connect resume to check-fresh with fresh
connect check-fresh to order with submission
connect check-fresh to reject with no-transaction
connect order to suspend with checkpoint
connect order to approval with next
connect approval to suspend with checkpoint
connect approval to delivery with next
connect delivery to suspend with checkpoint
connect delivery to ship with next
connect ship to end with then
connect reject to end with then
connect suspend to end with then
```

For your convenience, this graph model is preloaded as "tutorial-14".

Dry-run the workflow interactively
----------------------------------
You can exercise all three checkpoints without leaving the playground. Two things to
remember: instantiate before every run so each round starts with a fresh state machine
(on this engine 'run' may repeat on one instance and model values persist across runs -
see 'help run' - which would pollute a short-run simulation); and the SAME model.cid
must be supplied each time - it is the resume key. Redis must be running.

Import the deployed model as a draft:

```
import graph from tutorial-14
```

Run 1 - the customer orders a laptop:

```
instantiate graph
text(order-1001) -> model.cid
text(laptop) -> input.body.item
int(2000) -> input.body.amount
```

```
run
```

The traversal walks root -> resume -> check-fresh -> order -> suspend -> end and the run
completes normally - the workflow state now lives in Redis, not in memory. Inspect the
staged reply:

```
inspect output.body
```

It shows stage=order-submitted..., run=fresh (a new transaction) and cid=order-1001.

Run 2 - the store manager approves. Instantiate again with the same correlation ID and
the manager's input:

```
instantiate graph
text(order-1001) -> model.cid
text(approved) -> input.body.decision
text(store-88) -> input.body.manager
```

```
run
```

Watch the console: the resume node restores the persisted state and the traversal
continues at the approval node - the order checkpoint is NOT re-executed. Now
"inspect output.body" shows stage=approved... and run=resume, and the "seen" command
lists the order node as visited even though this run never executed it - that is the
restored traversal bookkeeping.

Run 3 - the delivery department releases the shipment:

```
instantiate graph
text(order-1001) -> model.cid
boolean(true) -> input.body.release
text(express) -> input.body.courier
```

```
run
```

Run 4 - shipment confirmation completes the workflow:

```
instantiate graph
text(order-1001) -> model.cid
text(TRK-12345) -> input.body.tracking
```

```
run
```

Inspect the final reply - the model accumulated state across all four short runs:

```
inspect output.body
```

It shows stage=shipped, run=resume, and the full history: order (laptop/2000), approval
(approved/store-88), delivery (release/express) and shipment (TRK-12345).

To see the input validation, start over with a correlation ID that never ordered:

```
instantiate graph
text(order-9999) -> model.cid
text(approved) -> input.body.decision
```

```
run
```

"inspect output" shows status=404 with type=rejected and run=fresh - the order must come
first, and the run flag tells the caller why. Each record is consumed on resume, so
repeating any middle run behaves the same way: no record means a fresh transaction.

Test the workflow over REST
---------------------------
Run 1 - the customer orders a laptop:

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-14 \
  -H "Content-Type: application/json" \
  -H "X-Correlation-Id: order-1001" \
  -d '{"item": "laptop", "amount": 2000}'
```

The reply is {"stage": "order-submitted; waiting for store manager approval", "run": "fresh",
"cid": "order-1001"} and the run is over - nothing stays in memory. Every stage reply carries
the "run" flag ("fresh" on run 1, "resume" on runs 2 to 4) so the caller always knows whether
it is looking at a new transaction or a resumed continuation. Run 2 - the store manager approves:

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-14 \
  -H "Content-Type: application/json" \
  -H "X-Correlation-Id: order-1001" \
  -d '{"decision": "approved", "manager": "store-88"}'
```

Run 3 - the delivery department releases the shipment:

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-14 \
  -H "Content-Type: application/json" \
  -H "X-Correlation-Id: order-1001" \
  -d '{"release": true, "courier": "express"}'
```

Run 4 - shipment confirmation completes the workflow:

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-14 \
  -H "Content-Type: application/json" \
  -H "X-Correlation-Id: order-1001" \
  -d '{"tracking": "TRK-12345"}'
```

The final reply carries the whole history - the order from run 1, the approval from run 2, the
release from run 3 and the shipment from run 4 - proof that the workflow state crossed every
suspension. Now try a decision with a correlation ID that never ordered:

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-14 \
  -H "Content-Type: application/json" \
  -H "X-Correlation-Id: order-9999" \
  -d '{"decision": "approved"}'
```

The workflow rejects it with HTTP-404 - the order must come first - and the reply's
"run": "fresh" tells the UI why: the record expired or never existed. Each record is consumed on
resume, so a duplicated request at any stage behaves like a fresh transaction instead of
executing that stage twice.

Summary
-------
In this session, we expressed a purchase workflow with three human checkpoints as four short
graph runs keyed by one business correlation ID: one reserved "suspend" node served every
checkpoint, each suspensible node captured its actor's input into the model and staged its own
stage response, input validation enforced the order-before-decision sequence, and the
engine-managed "model.run" flag told every reply whether the run was fresh or resumed.

Why suspend and resume?
-----------------------
Real business processes wait on people - repeatedly. Suspension turns each wait into a durable
record instead of a parked runtime: any application instance sharing the state store can resume
the workflow, restarts lose nothing, and each run stays short and observable. The state store is
pluggable - Redis is the packaged implementation, and any composable function honoring the
documented store contract can replace it.
