# kafka-demo — minimalist-kafka worked example

A hands-on, end-to-end demonstration of the **minimalist-kafka** consumer + producer pattern on the Rust
engine — including both **routing styles** of the Kafka flow adapter, side by side. You publish a message
from a terminal, it travels through Kafka into a composable Rust app, gets processed, is published to
another topic, and shows up in a second terminal — with the whole path visible in the app's log.

This is the twin of the Java [`examples/kafka-demo`](https://github.com/Accenture/mercury-composable/tree/main/examples/kafka-demo):
the same topics, the same flows, the same Node helper programs, the same outbound JSON — so the two apps
are interchangeable consumers of the same broker, which the [interop runbook](#interop) at the end uses.

```
  publish-inbound.js  --(demo.inbound)-->  kafka-demo (Rust)  --(demo.outbound)-->  listen-outbound.js
   (program-2, you type)                    |  flow adapter            ^                (program-1, logs it)
                                            |  -> demo.processor       |
                                            |  -> simple.kafka.notification
                                            +--------------------------+

  publish-orders.js   --(demo.orders)--->  kafka-demo (Rust)  --(demo.outbound)-->  listen-outbound.js
   (program-3, you type)                    |  flow adapter: SECOND-LEVEL ROUTING (per-record rules)
                                            |  -> flow://demo-order-flow       (type=order / order-*)
                                            |  -> task://demo.refund.processor (body event.kind=refund)
                                            |  -> flow://demo-catch-all-flow   (default)
```

The app is pure minimalist-kafka: the **Kafka flow adapter** binds `demo.inbound` to the `kafka-demo-flow`
(**direct routing** — every message goes to one flow), whose `demo.processor` task wraps the message with
processing metadata, and `simple.kafka.notification` publishes the result to `demo.outbound`. The
`demo.orders` binding uses **second-level routing** instead — a rule list inspects each record and picks
the target per message. No code wires any of those steps together — the flow YAML does.

## Prerequisites

- The Rust toolchain (this workspace builds the example: `cargo build -p kafka-demo`).
- Node.js 18+ (for the helper programs in [`node/`](node)).
- The local Kafka broker from the Java repository's
  [`helpers/kafka-standalone`](https://github.com/Accenture/mercury-composable/tree/main/helpers/kafka-standalone)
  — a real single-node KRaft broker, no Docker needed.

Install the Node dependencies once:

```shell
cd examples/kafka-demo/node
npm install
```

## Run it — five terminals

Run each step in its own terminal, from this repository's root unless noted.

### Terminal A — start the local Kafka broker

> **Note**: `x.y.z` denotes the current Mercury version of the Java repository.

```shell
cd <mercury-composable>/helpers/kafka-standalone
mvn clean package
java -jar target/kafka-standalone-x.y.z-exec.jar
```
Wait for it to report the broker is up on `127.0.0.1:9092`.

### Terminal B — create the demo topics (10 partitions each)
```shell
cd examples/kafka-demo/node
node create-topics.js
# -> created (10 partitions each): demo.inbound, demo.orders, demo.outbound,
#    demo.inbound.dlq, demo.orders.dlq, demo.interop.in, demo.interop.out
```
The two DLQ topics are created up front because dead-letter topics must be **pre-provisioned**
(Kafka auto-creation is off in production) — and creating them makes the failure path a
first-class part of the demo (see [the failure path](#failure-path)). The two interop topics serve
the [interop runbook](#interop).

### Terminal C — start the kafka-demo app
```shell
cargo run -p kafka-demo
```
It compiles the flows, starts the producer, and starts the flow adapter (one consumer per binding).
Each binding announces itself:
```
... INFO  [minimalist_kafka::consumer] Kafka flow adapter binding: topic 'demo.inbound' -> flow 'kafka-demo-flow' (consumer group 'kafka-demo-group', dlq-topic 'demo.inbound.dlq')
... INFO  [minimalist_kafka::consumer] Kafka flow adapter binding: topic 'demo.orders' -> second-level routing (3 rules + default) (consumer group 'kafka-demo-orders-group', serializer 'json', task ttl 15s, dlq-topic 'demo.orders.dlq')
```

### Terminal D — listen on the outbound topic (program-1)
```shell
cd examples/kafka-demo/node
node listen-outbound.js
# -> listening on 'demo.outbound' ...
```

### Terminal E — publish from the console (program-2)
```shell
cd examples/kafka-demo/node
node publish-inbound.js
> hello composable kafka
```

## What you should see

**Terminal E (publisher)** sends a `traceparent`, so it prints the `traceId` it started:
```
[2026-09-21T17:41:24.937Z] -> demo.inbound cid=d4782a35-... traceId=19f866087b99895b4a14ce03a7c2c4ba hello composable kafka
```
**Terminal C (the app)** logs the receipt — the hop it came in on (the trace path `KAFKA /<topic>`), the
trace id, and the incoming span it chained onto:
```
2026-09-21T17:41:24.939Z INFO  [kafka_demo] Received from KAFKA /demo.inbound (cid=d4782a35-..., traceId=19f866087b99895b4a14ce03a7c2c4ba, incoming span=6872b7906f8391ca): hello composable kafka
```
**Terminal D (listener)** receives the processed message, carrying the **same trace id** but a **new span**:
```
[2026-09-21T17:41:24.948Z] <- demo.outbound[p7] cid=d4782a35-... traceId=19f866087b99895b4a14ce03a7c2c4ba span=9e29a3ce17d28bcb {"processedAt":"2026-09-21T17:41:24.939Z","processedBy":"kafka-demo","received":"hello composable kafka","traceId":"19f866087b99895b4a14ce03a7c2c4ba"}
```

The `cid` is preserved end-to-end, and the **`traceId` is identical** at the publisher, in the app's log,
and at the listener — proof the trace stays continuous across both Kafka hops. (Each hop gets a new
`span_id`; `simple.kafka.notification`'s span becomes the parent of the next hop, while the trace id is
carried unchanged.) If the publisher sends no `traceparent`, the flow simply starts a fresh trace instead.

## Second-level routing — one topic, many targets

The `demo.orders` binding shows the adapter's **second-level routing**: instead of one `flow`, a `flows`
rule list inspects a key-value of each record and picks the target per message — the common Kafka pattern
of one topic carrying mixed event types. See the rule grammar in
[`kafka-flow-adapter.yaml`](resources/kafka-flow-adapter.yaml); the first matching rule wins, in
declaration order, and the mandatory `default` catches the rest. `serializer: 'json'` decodes each JSON
record to a map before routing, so the `input.body` rule can match — a non-JSON record keeps its raw bytes
and falls through to the default.

### Terminal F — publish mixed events (program-3)
```shell
cd examples/kafka-demo/node
node publish-orders.js
```
One command per routing rule:

**1. Exact header rule (`type=order`) routes to `flow://demo-order-flow`**
```
command: order [json]
example: order {"item": "mobile-phone", "qty": 1}
```

**2. Wildcard header rule (`type=order-*`) also routes to `flow://demo-order-flow`**
```
command: order-<id> [json]
example: order-123 {"item": "laptop", "qty": 1}
```

**3. Body rule (`event.kind=refund`) routes to `task://demo.refund.processor`**
```
command: refund [json]
example: refund {"order_id": "order-123"}
```
The optional json holds the refund **details** only — the `{"event":{"kind":"refund"}}` envelope
the body rule matches on is added by the script, so the example above routes to the task.

**4. When no rule matches, `default` routes to `flow://demo-catch-all-flow`**
```
command: <anything else>
example: hello
```

**What you should see per command:**

- `order` / `order-42` — the app log shows `Order event routed by rule type(...)`, and **Terminal D**
  receives the processed order on `demo.outbound` with `"routedBy"` naming the matched key. The flow
  publishes the processor's map straight through `simple.kafka.notification`, which **auto-serializes it
  to JSON bytes** — the outbound symmetry of `serializer: 'json'` (a map in the function, JSON on the wire).
- `refund` — the **app log** shows `Refund routed by rule input.body.event.kind(refund)` with the same
  `cid`/`traceId` the publisher printed. Nothing arrives on `demo.outbound`: a `task://` target invokes
  the function **directly** — all record headers copied verbatim, the whole payload as the body, no flow
  and no data mapping. Use it for processing simple enough that a flow is overweight; anything needing
  orchestration (like publishing onward) belongs in a `flow://` target.
- anything else — **Terminal D** receives the annotated record from `demo-catch-all-flow` with
  `"routedBy": "default"` and a `"shape"` field showing whether the body arrived as a decoded map/list
  or as raw bytes (`serializer: 'json'` is best-effort: an unparseable record passes through unchanged,
  and the default handler deals with it — the pattern a production catch-all should follow).

Each published record carries its own `traceparent`, so every routed message — flow or task — shows full
trace continuity in the app log, exactly like the direct-routing path.

### The failure path — retries then dead-letter {#failure-path}

`serializer: 'json'` is **best-effort**: an unparseable record keeps its raw bytes and simply passes to
whichever target the rules select. Start the dead-letter listener in its own terminal (program-4):

```shell
cd examples/kafka-demo/node
node listen-dlq.js
```

Then send an order whose payload is plain text:

```
> order hello
```

The `type=order` header matches, so the record reaches `demo-order-flow` — whose map-typed processor
cannot digest raw bytes and **fails the flow back to the Kafka flow adapter**. Watch the app terminal:
the flow fails, the adapter **retries 3 times** (`kafka.flow.max.retries`), then **dead-letters the
original record to `demo.orders.dlq`** and moves on — the partition never stalls. The DLQ listener
prints the intercepted record — headers and body preserved verbatim, plus the `dlq.error` (why it
failed) and `dlq.origin.topic` (where it came from) headers:

```
[...] <- demo.orders.dlq[p9] cid=ba56c312-...
[...]    origin: demo.orders
[...]    error:  flow 'demo-order-flow' returned status 400
[...]    body:   hello
```

(The Java demo dead-letters the same record with `java.lang.IllegalStateException: flow
'demo-order-flow' returned status 500` — a type mismatch is a 500 there, while this engine reports a
typed function's undeserializable input as a 400. Same contract, different status text.)

This is the production contract for a malformed event, demonstrated live. (The default flow, by
contrast, accepts raw bytes by design — compare with `hello` above, which has no matching rule and
lands in the catch-all normally.)

## Interop with the Java kafka-demo {#interop}

The Java and Rust apps speak the same wire contract — record headers (`cid`, W3C `traceparent`),
dataset shape, DLQ headers — so a record can cross engines. The `interop` profile turns this app into a
**relay** between two interop topics and the Java demo, which runs unchanged on the same broker, so one
trace crosses both engines in each direction (chained hops, not side-by-side consumption):

```
  Rust producer -> Java flow adapter
  publish-inbound.js demo.interop.in --> kafka-demo (Rust, interop) --(demo.inbound)--> kafka-demo (Java) --(demo.outbound)--> listen-outbound.js

  Java producer -> Rust flow adapter
  publish-inbound.js --(demo.inbound)--> kafka-demo (Java) --(demo.outbound)--> kafka-demo (Rust, interop) --(demo.interop.out)--> listen-outbound.js demo.interop.out
```

Run the Java demo (its README's Terminal C) and this app in the interop profile:

```shell
cargo run -p kafka-demo -- -Dapp.profiles.active=interop
```

**Leg 1 — Rust first, then Java.** Listen on `demo.outbound` (`node listen-outbound.js`) and publish into
the interop inbound topic:

```shell
node publish-inbound.js demo.interop.in
> across two engines
```

The Rust log shows `Received from KAFKA /demo.interop.in ...` (the relay flow reuses `demo.processor`),
the Java log shows its own receipt under the **same trace id** with the Rust notification's span as its
incoming span, and the listener prints a record processed by both: the Java output's `received` field
is the Rust output JSON. The relay also consumes `demo.outbound`, so the same record then appears a
third time on `demo.interop.out` — three hops, two engines, one trace id.

**Leg 2 — Java first, then Rust.** Listen on the interop outbound topic
(`node listen-outbound.js demo.interop.out`) and publish to `demo.inbound` as usual
(`node publish-inbound.js`). The Java demo processes it and publishes to `demo.outbound`; the Rust relay
consumes that, processes it again and publishes to `demo.interop.out` — the listener prints a record
whose `received` field is the Java output JSON, under the publisher's trace id.

Both apps can also share one consumer group: start this app in its plain profile next to the Java demo
and the coordinator splits `demo.inbound`'s partitions between a Java member and a Rust member. Stop
this app with Ctrl-C and it leaves the group explicitly (an immediate rebalance); kill it hard and the
coordinator hands its partitions over after the 45 s session timeout, with every record still delivered.

The permanent record of this drive — evidence, defects found and their fixes — is
[`docs/test-reports/minimalist-kafka-interop.md`](../../docs/test-reports/minimalist-kafka-interop.md).

## How it maps to minimalist-kafka

| Piece | What it shows |
|-------|---------------|
| [`kafka-flow-adapter.yaml`](resources/kafka-flow-adapter.yaml) | the **consumer** side, both styles: direct routing (`flow`) and second-level routing (`flows` + `serializer` + `ttl`) |
| [`kafka-demo-flow.yml`](resources/flows/kafka-demo-flow.yml) | orchestration as config: `demo.processor` → `simple.kafka.notification` |
| [`demo-order-flow.yml`](resources/flows/demo-order-flow.yml) | a rule-selected **specific flow**; publishes a map that `simple.kafka.notification` auto-serializes |
| [`demo-catch-all-flow.yml`](resources/flows/demo-catch-all-flow.yml) | the mandatory **default** flow; its task handles every body shape (map, list or raw bytes) |
| `DemoProcessor` in [`main.rs`](src/main.rs) | a self-contained function (the unit of work); it derives its trace context from the injected `my_*` headers, so it is directly unit-tested |
| `RefundProcessor` in [`main.rs`](src/main.rs) | a **`task://` routing target**: invoked directly by the adapter — headers copied verbatim, payload as body, no flow |
| [`listen-dlq.js`](node/listen-dlq.js) | the **dead-letter side**: prints intercepted records with their `dlq.error` / `dlq.origin.topic` headers (see [the failure path](#failure-path)) |
| [`application-interop.yml`](resources/application-interop.yml) + [`kafka-flow-adapter-interop.yaml`](resources/kafka-flow-adapter-interop.yaml) | the **interop relay** profile (see [interop](#interop)) |
| `simple.kafka.notification` | the **producer** side: publish to a topic via data mapping (`text(demo.outbound) -> header.topic`) |

## Notes

- Point at a different broker with `export KAFKA_BOOTSTRAP_SERVERS=host:port` (both the app and the
  Node programs honor it).
- On repeated processing failure, a message is dead-lettered to the binding's configured `dlq-topic`
  (`demo.inbound.dlq` / `demo.orders.dlq` in `kafka-flow-adapter.yaml`); `create-topics.js` pre-creates
  both, and the failure handling applies identically to `flow://` and `task://` targets — see
  [the failure path](#failure-path). The happy path never touches them.
- The Java repository's [Minimalist Kafka guide](https://accenture.github.io/mercury-composable/guides/minimalist-kafka/)
  is the canon for the adapter YAML, the routing rule grammar (selectors, the three matcher modes,
  targets, `serializer`, `ttl`) and the reliability contract; the Rust port's deliberate deltas are in
  [`draft-design-specs/minimalist-kafka-port.md`](../../draft-design-specs/minimalist-kafka-port.md) §7.
