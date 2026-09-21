---
title: Interop Test Report — Minimalist Kafka, Java ⇄ Rust
summary: Permanent record of the live validation of the Rust minimalist-kafka port against the Java
  module on one real broker - the Rust kafka-demo alone, the two engines chained in both directions
  under one trace id, a mixed Java-plus-Rust consumer group, a hard-killed member's partitions taken
  over, and the defects the drive surfaced - kept as the K4 gate evidence for the port.
layer: reference
audience: [developer, architect, devops]
keywords: [interop, kafka, minimalist-kafka, flow adapter, rust, traceparent, dead-letter, rebalance, consumer group, test report]
---

# Interop Test Report — Minimalist Kafka, Java ⇄ Rust

*Live validation of the Rust `minimalist-kafka` port (gate K4 of
[the port spec](https://github.com/Accenture/mercury/blob/main/draft-design-specs/minimalist-kafka-port.md))
against the Java module, conducted 2026-09-21 on one real broker: the Rust `kafka-demo` on its own,
the Java and Rust engines chained in both directions, a consumer group with one member of each
engine, and a member killed hard while records were in flight.*

This report is a permanent record. It documents what was tested, the evidence collected, and — in
the interest of honest engineering — the defects the drive surfaced and how each was fixed or
recorded. Everything here is reproducible from the shipped examples by following the
[Rust kafka-demo runbook](https://github.com/Accenture/mercury/tree/main/examples/kafka-demo) and its
[Java twin](https://github.com/Accenture/mercury-composable/tree/main/examples/kafka-demo).

## Setup

| Piece | What ran |
|-------|----------|
| Broker | the Java repository's `helpers/kafka-standalone` 4.12.13 — Apache Kafka 4.3.1, one KRaft node on `127.0.0.1:9092`, started fresh (it deletes its topics on restart) |
| Java app | `examples/kafka-demo` 4.12.13, unchanged: `demo.inbound` → `kafka-demo-flow`, `demo.orders` → the second-level routing rules; groups `kafka-demo-group` and `kafka-demo-orders-group` |
| Rust app | `examples/kafka-demo` (this port's twin, new at K4) on the K3 engine (`mercury-minimalist-kafka`, mercury #299) — the same topics, flows, rule list and **the same group ids** as the Java app, deliberately, so the two can share a group; the `interop` profile turns it into a relay between `demo.interop.in`, `demo.inbound`, `demo.outbound` and `demo.interop.out` |
| Helpers | Node 22 with kafkajs 2.2.4 — the five programs copied from the Java demo (`create-topics`, `listen-outbound`, `listen-dlq`, `publish-inbound`, `publish-orders`); seven topics with 10 partitions each |
| Protocol | classic rebalance protocol on both clients during the drive — both bundled templates left `group.protocol` unset at the time (the Java line was commented out); re-ruled the same evening, see the [addendum](#addendum-groupprotocolauto-re-ruled-and-proven-live) |

Timestamps below are the machines' own: the Node programs and the Rust app print UTC (`Z`), the
Java app and the broker print local time (UTC−7).

## Scenario 1 — direct routing, Rust alone

One message published to `demo.inbound`; the Rust adapter runs `kafka-demo-flow`; the flow publishes
to `demo.outbound`.

```
publisher   [17:41:24.937Z] -> demo.inbound cid=d4782a35-… traceId=19f866087b99895b4a14ce03a7c2c4ba hello composable kafka
rust app    17:41:24.939Z INFO [kafka_demo] Received from KAFKA /demo.inbound (cid=d4782a35-…, traceId=19f866087b99895b4a14ce03a7c2c4ba, incoming span=6872b7906f8391ca): hello composable kafka
listener    [17:41:24.948Z] <- demo.outbound[p7] cid=d4782a35-… traceId=19f866087b99895b4a14ce03a7c2c4ba span=9e29a3ce17d28bcb {"processedAt":"2026-09-21T17:41:24.939Z","processedBy":"kafka-demo","received":"hello composable kafka","traceId":"19f866087b99895b4a14ce03a7c2c4ba"}
```

One trace id at the publisher, in the app and at the listener; the incoming span is the publisher's,
the outbound span is a new one stamped by `simple.kafka.notification`. Eleven milliseconds end to end.

## Scenario 2 — second-level routing, Rust alone

Four records on `demo.orders`, one per rule of the binding's `flows` list (`serializer: 'json'`,
`ttl: 15s`):

| Command | Rule that fired | Evidence |
|---------|-----------------|----------|
| `order {"item": "mobile-phone", "qty": 1}` | `input.header.type(order)` → `flow://demo-order-flow` | app: `Order event routed by rule type(order) … {"item":"mobile-phone","qty":1}`; listener: `"routedBy":"input.header.type(order)"`, `"order":{"item":"mobile-phone","qty":1}` — the decoded map went in and JSON bytes came out |
| `order-123 {"item": "laptop", "qty": 1}` | `input.header.type(order-*)` (wildcard) → the same flow | `"routedBy":"input.header.type(order-123)"` |
| `refund {"order_id": "order-123"}` | `input.body.event.kind(refund)` → `task://demo.refund.processor` | app: `Refund routed by rule input.body.event.kind(refund) (cid=88c67ed0-…, traceId=b67abc94…): {"event":{"kind":"refund"},"order_id":"order-123"}`; nothing on `demo.outbound` — a task target publishes nothing |
| `hello` | `default` → `flow://demo-catch-all-flow` | app: `Unmatched record caught by the default rule … raw bytes (not a JSON object/array)`; listener: `"shape":"raw bytes (not a JSON object/array)","received":"hello","routedBy":"default"` |

Every record kept its own publisher trace id through the app and onto the listener.

## Scenario 3 — the failure path, Rust alone

`order hello`: the `type=order` header matches, the payload is not JSON, so the map-typed
`demo.order.processor` cannot digest it.

```
17:41:26.007Z WARN [minimalist_kafka::consumer] flow 'demo-order-flow' failed for a 'demo.orders' message (attempt 1/3); retrying - flow 'demo-order-flow' returned status 400
17:41:26.510Z WARN … (attempt 2/3); retrying …
17:41:27.012Z WARN … (attempt 3/3); retrying …
17:41:27.515Z WARN … failed for a 'demo.orders' message after 4 attempt(s); routing to Some("demo.orders.dlq") - flow 'demo-order-flow' returned status 400

dlq listener [17:41:27.528Z] <- demo.orders.dlq[p9] cid=ba56c312-…
                origin: demo.orders
                error:  flow 'demo-order-flow' returned status 400
                body:   hello
```

Three retries at the configured 500 ms backoff, then the original record parked with
`dlq.origin.topic` and `dlq.error`, headers and body preserved, 1.5 s after publication. The
partition stayed live (the later scenarios ran on the same binding).

## Scenario 4 — interop leg 1: Rust producer → Java flow adapter

The Rust relay (`interop` profile) consumes `demo.interop.in`, processes, and publishes to
`demo.inbound` — the Java app's inbound topic. The Java app processes and publishes to
`demo.outbound`, which the Rust relay consumes again and forwards to `demo.interop.out`. One trace
id, three hops, two engines:

```
publisher   [17:42:39.987Z] -> demo.interop.in cid=e0b18dcf-… traceId=be9231bede94493dcf53662fed94151a across two engines
rust relay  17:42:39.987Z Received from KAFKA /demo.interop.in (cid=e0b18dcf-…, traceId=be9231be…, incoming span=47d6f645edfe6186): across two engines
java app    10:42:40.038 DemoProcessor - Received from demo.inbound (cid=e0b18dcf-…, traceId=be9231be…, incoming span=bd3138f4bce13eea): {"processedAt":"2026-09-21T17:42:39.987Z","processedBy":"kafka-demo","received":"across two engines","traceId":"be9231be…"}
java app    10:42:40.043 Telemetry - {trace={path=KAFKA /demo.inbound, parent_span_id=bd3138f4bce13eea, span_id=abb35e81d7085f1c, service=demo.processor, … id=be9231be…}}
java app    10:42:40.070 Telemetry - {trace={path=KAFKA /demo.inbound, parent_span_id=abb35e81d7085f1c, span_id=83e43d5ca8106df5, service=simple.kafka.notification, … id=be9231be…}}
listener    [17:42:40.058Z] <- demo.outbound[p6] cid=e0b18dcf-… traceId=be9231bede94493dcf53662fed94151a span=83e43d5ca8106df5 {…Java's output…}
rust relay  17:42:40.058Z Received from KAFKA /demo.outbound (cid=e0b18dcf-…, traceId=be9231be…, incoming span=83e43d5ca8106df5): {…Java's output…}
listener    [17:42:40.066Z] <- demo.interop.out[p4] cid=e0b18dcf-… traceId=be9231bede94493dcf53662fed94151a span=bcc6d2ddb4eeb4b1 {"processedBy":"kafka-demo","received":"{ \"received\": \"{\\\"received\\\":\\\"across two engines\\\",…}\", \"processedBy\": \"kafka-demo\" …}", "traceId":"be9231be…"}
```

The span chain is exact across the engine boundary in both directions: the Java `demo.processor`'s
parent span (`bd3138f4bce13eea`) is the span the Rust notification stamped, and the Rust relay's
incoming span on `demo.outbound` (`83e43d5ca8106df5`) is the Java notification's span. The final
record nests the three hops' outputs. 79 ms end to end.

## Scenario 5 — interop leg 2: Java producer → Rust flow adapter

The publisher writes to `demo.inbound` as in the plain demo; the Java app processes and publishes
to `demo.outbound`; the Rust relay consumes and forwards.

```
publisher   [17:42:40.497Z] -> demo.inbound cid=44c92fd4-… traceId=97314d7cb7632d822787de512b4a0111 java first
java app    10:42:40.498 DemoProcessor - Received from demo.inbound (cid=44c92fd4-…, traceId=97314d7c…, incoming span=35b194c63185b1c6): java first
java app    10:42:40.500 Telemetry - {trace={… parent_span_id=ac30454f41980a44, span_id=bbf11dd82bad9169, service=simple.kafka.notification, … id=97314d7c…}}
rust relay  17:42:40.507Z Received from KAFKA /demo.outbound (cid=44c92fd4-…, traceId=97314d7c…, incoming span=bbf11dd82bad9169): {…Java's output…}
listener    [17:42:40.514Z] <- demo.interop.out[p8] cid=44c92fd4-… traceId=97314d7cb7632d822787de512b4a0111 span=83bb59eb42f2caf5 {"processedBy":"kafka-demo","received":"{ … \"received\": \"java first\", \"processedBy\": \"kafka-demo\" }","traceId":"97314d7c…"}
```

## Scenario 6 — a mixed-engine consumer group

The Rust app (plain profile) was started while the Java app was running, joining the same
`kafka-demo-group` on `demo.inbound`:

```
broker  10:44:40.302 Preparing to rebalance group kafka-demo-group … (reason: Adding new member rdkafka-3aa7b8bf-…)
broker  10:44:40.914 Stabilized group kafka-demo-group generation 4 with 2 members.
```

A burst of 20 records: **10 processed by the Java member, 10 by the Rust member**, all 20 at the
listener within one second, 20 unique, none delivered twice. A Java client and a librdkafka client
share one classic group (the `range` assignor is common to both), so a consumer group can be
migrated between engines member by member.

## Scenario 7 — a member killed hard: takeover and in-flight redelivery

**7a — the timeout.** The Rust member was killed with `SIGKILL` at 17:44:45; the coordinator
declared it dead 45 s later, the client default `session.timeout.ms` on both engines:

```
broker  10:45:30.677 Member rdkafka-3aa7b8bf-… in group kafka-demo-group has failed, removing it from the group.
broker  10:45:30.677 Preparing to rebalance group kafka-demo-group … (reason: removing member … on heartbeat expiration.)
```

**7b — in flight at death.** With the mixed group re-formed (generation 6, 2 members), four poison
orders (`order poison-N`) were published to `demo.orders` and the Rust member killed 0.7 s later —
mid-retry: its log ends at `attempt 1/3` of one of them, nothing dead-lettered yet. After the
takeover the Java member retried and dead-lettered every one of them: the DLQ holds exactly four
records (17:46:57 to 17:47:02), each with the Java error text
`java.lang.IllegalStateException: flow 'demo-order-flow' returned status 500` — the record the Rust
member had in flight was redelivered, not lost, and not duplicated on the DLQ.

**7c — records published while the member is dead.** 20 records to `demo.inbound` at 17:46:10.6:

```
listener  [17:46:10.783Z] <- demo.outbound[p9]   … the 10 records on the live Java member's partitions arrive at once …
listener  [17:46:10.864Z] <- demo.outbound[p4]
broker    10:46:52.983 Member rdkafka-8a6b68d7-… in group kafka-demo-group has failed, removing it from the group.
broker    10:46:55.977 Stabilized group kafka-demo-group generation 7 with 1 members.
listener  [17:46:56.121Z] <- demo.outbound[p6]   … the 10 records on the dead member's partitions arrive after the takeover …
listener  [17:46:56.200Z] <- demo.outbound[p0]
```

20 of 20 unique at the listener, none delivered twice; the second half waited 45.5 s for the
coordinator to expire the dead member and hand its partitions to the Java member. That is the
at-least-once contract as deployed: nothing is lost on a pod death, and the recovery time is the
session timeout.

**7d — the graceful counterpart.** Stopping the Rust app with `SIGINT` drains each binding's
consumer after its in-flight record and leaves the group explicitly:

```
rust app  Kafka flow consumer for topic 'demo.inbound' stopping / stopped   (both bindings)
broker    10:42:25.634 Member rdkafka-6ed7af08-… has left group through explicit `LeaveGroup` request …
broker    10:42:25.636 Preparing to rebalance group kafka-demo-group … (reason: explicit `LeaveGroup` request …)
```

The rebalance is immediate, where the hard kill cost 45 s.

## Findings

1. **A headless Rust application exited right after boot — fixed on this branch.** The first live
   run processed nothing: the app compiled its flows, started both consumers and exited 200 ms later,
   while the broker held the six records the publishers had written. `AutoStart::run` parked the
   process only when serving HTTP or websockets; a Kafka-consuming app with no REST server had
   nothing else holding it open — on the JVM the module's consumer threads are non-daemon, so the
   Java app never had the problem, and the Rust e2e never saw it because a test function keeps the
   runtime alive. Fix: `Platform::keep_running(reason)`, declared once by the flow adapter when its
   consumers start and honoured by `AutoStart::run` exactly like HTTP serving
   (`Kafka flow adapter (2 binding(s)) keeps the application running until it is stopped`, then
   `Application running - press Ctrl-C to stop`). The adapter also registers its consumers' stop as
   a shutdown hook, which is what produced scenario 7d.
2. **`SIGTERM` is not handled by the Rust entry point — follow-up, platform-wide.** `AutoStart::run`
   waits on Ctrl-C (`SIGINT`) only. A Kubernetes pod stop sends `SIGTERM`, so a rolling restart of a
   Rust consumer today looks like scenario 7c (partitions held until the 45 s session timeout)
   rather than 7d (immediate `LeaveGroup`). The Java process runs its shutdown hooks on `SIGTERM`.
   This is not Kafka-specific — every headless or REST Rust application is affected — and was
   recorded for platform-core rather than patched in the K4 change. **Closed the same day**
   (branch `fix/sigterm-graceful-stop`, the maintainer's direction): `AutoStart::run` now stops on
   `SIGTERM` exactly as on Ctrl-C, and the flow adapter's shutdown hook waits — bounded by a 10 s
   grace — for every binding consumer to finish its in-flight record before the process goes on, so a
   pod stop takes the scenario 7d path: the record commits, the member leaves the group explicitly,
   the rebalance is immediate.
3. **Presentation differences, not defects.** (a) The dead-letter `dlq.error` text differs: the
   Rust engine reports `flow 'demo-order-flow' returned status 400` (a typed function's input that
   cannot be deserialized is a 400), the Java engine
   `java.lang.IllegalStateException: flow 'demo-order-flow' returned status 500` (a Java type
   mismatch is a 500). Both are the same contract — the original record parked with the failure
   named. (b) The Java demo's outbound JSON is pretty-printed (its mapper's default), the Rust
   demo's is compact; both are valid JSON and any consumer reads either. (c) The reused
   `demo.processor` logged a fixed topic name; the Rust demo now logs the injected trace path
   (`KAFKA /<topic>`), which is what made the relay hops in scenarios 4 and 5 legible.
4. **A new consumer group starts from `auto.offset.reset=earliest`** (the bundled template's
   default): when the relay's `kafka-demo-interop-out` group first joined, it consumed the four
   records already on `demo.outbound` from scenarios 1 and 2 and forwarded them to
   `demo.interop.out`. Expected, and worth knowing when reading an interop listener.

## Learnings, kept as the playbook

- **Wait for the broker's word before publishing.** A consumer's group join takes seconds; the
  drive polled the broker log for `Stabilized group <id>` before each publish. The kafkajs listeners
  (`fromBeginning: false`) likewise see nothing published before their assignment.
- **Distinguish engines by their logs**, not by the payload: both demos stamp
  `processedBy: kafka-demo`. The Rust log names the hop (`Received from KAFKA /<topic>`), the Java
  log is the DemoProcessor line plus the telemetry span path.
- **The wire contract that makes the trace continuous across engines** is the record-header pair
  `cid` + W3C `traceparent`, with the notification stamping a fresh span per hop. The nested
  `received` field on the interop listener shows the hop order for free.
- **Recovery time after a hard death is the session timeout**, identical on both clients (45 s);
  a graceful stop rebalances at once. Finding 2 is what stands between a Rust pod and the graceful
  path in Kubernetes.
- **Run the failure path against the real broker at least once**: the mock cluster proves the
  logic, the live broker proves the timing (retry spacing, DLQ latency, takeover) that operators
  will actually see.

## Addendum — `group.protocol=auto` re-ruled and proven live

The maintainer asked, after the drive, why the Rust port had not switched to the KIP-848 consumer
protocol on a broker that supports it. The evidence: the broker finalizes `group.version=1`; the Java
template shipped `group.protocol` commented out, so the Java demo had joined classic as well; the
Rust template set no key. The K3 delta (`auto` = `classic`, for want of a feature probe in librdkafka)
was replaced the same evening by an **optimistic `auto`** — start each binding's consumer with
`consumer`, and if the broker refuses it, rebuild the consumer once with `classic` — because librdkafka
reports the refusal as a fatal `ConsumerGroupHeartbeat` error (`UNSUPPORTED_VERSION` when the
coordinator has the protocol disabled, `_UNSUPPORTED_FEATURE` when the API is not advertised), and a
refused join never becomes a group member. **Both engines now ship `auto` uncommented in their bundled
consumer templates** (maintainer decision, 2026-09-21).

**Proof A — the default template against the KIP-848 broker** (`kafka-standalone`, `group.version=1`):

```
rust app  group.protocol=auto - trying the KIP-848 consumer rebalance protocol first; a broker without it resolves the binding to classic at its first join
rust app  Kafka flow adapter binding: topic 'demo.inbound' -> flow 'kafka-demo-flow' (consumer group 'kafka-demo-group', dlq-topic 'demo.inbound.dlq', protocol consumer (auto))
broker    [GroupId kafka-demo-group] Member NzRWAus/TUaUSqJb63ZcWg joins the consumer group using the consumer protocol.
broker    [GroupId kafka-demo-group] Bumped group epoch to 2 with metadata hash 2421248355301747518.
broker    [GroupId kafka-demo-orders-group] Computed a new target assignment for epoch 2 with 'uniform' assignor in 3ms.
```

A published message was consumed; `SIGTERM` produced `Member … left the consumer group` — the
KIP-848 explicit leave.

**Proof B — the same binary against a coordinator with the consumer protocol disabled**
(the standalone broker started with `group.coordinator.rebalance.protocols=classic` through the
Spring Boot `PropertiesLauncher` and a `loader.path` override of `server.properties`):

```
rust app  ERROR librdkafka: FATAL [thrd:main]: Fatal error: Broker: API version not supported: ConsumerGroupHeartbeat fatal error: Broker: API version not supported
rust app  WARN  group.protocol=auto resolved to classic for topic 'demo.inbound' - the broker does not support the consumer rebalance protocol (ConsumerGroupHeartbeat fatal error: Broker: API version not supported); rejoining with the classic protocol
broker    Dynamic member with unknown member id joins group kafka-demo-group in Empty state. Created a new member id rdkafka-0b6d4827-…
```

Both bindings rejoined classic and consumed the published message. The client logs the refusal at
`ERROR` (twice per binding, librdkafka's own "Fatal error" and "Global error" lines) before the
adapter's `WARN` states the resolution — loud, but the outcome is the Java resolver's.

The flow adapter's own e2e now runs `auto` against `MockCluster`, which accepts the consumer protocol:
every binding logs `protocol consumer (auto)` and none falls back. Spec §7 item 7 records the
re-ruling.
