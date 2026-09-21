# sync-over-async-demo — synchronous REST over an async Kafka backend, and cross-pod progressive rendering

The Rust twin of the Java `sync-over-async-demo`. One binary plays four roles, selected by an active
profile (`-Dapp.profiles.active=<role>`):

| Role | Port | What runs |
|------|------|-----------|
| `facade` | 8400 | `POST /api/sync-to-async` (+ `-json`, `-avro`) → the `sync-to-async` flows (`sync.prepare` → `simple.kafka.notification` → `sync.await`), the reply flows bound to the response topics, the Redis return-route coordinator |
| `backend` | — | the `system-of-record` flows bound to the request topics; no REST, no Redis |
| `stream-ui` (pod A) | 8600 | return-route coordinator + `StreamBridge` facade behind `GET /api/notifications` (`stream: true`, SSE) |
| `stream-producer` (pod B) | 8601 | `StreamResponder` only — no coordinator, no subscriber — behind `POST /api/produce` |

The first two are the **sync-over-async request/reply pattern**: a caller makes one **synchronous HTTP
request**, but under the hood the request travels over **Kafka** to a separate backend pod, the reply comes
back over Kafka, and a **Redis return route** delivers it to the exact facade pod that is waiting — so the
HTTP call returns the real backend response synchronously.

```
  curl  --HTTP-->  facade pod (8400)                                   backend pod
                     sync-to-async flow                                 system-of-record flow
                       sync.prepare  -> register return route (Redis)
                       simple.kafka.notification --(soa.request)-->     system.of.record (process)
                       sync.await  (parks) ........                     simple.kafka.notification
                                                .                              |
                     soa-reply flow  <--(soa.response)---------------------- --+
                       soa.reply -> coordinator.deliver -> Redis wakes sync.await
                     <--HTTP 200 (backend reply)
```

The last two are the [streaming return route](../../draft-design-specs/sync-over-async-port.md) — the
same rendezvous used for progressive rendering, broker-free by design; see
[Streaming return route](#streaming-return-route-cross-pod-progressive-rendering) below.

## What's app-specific vs. reused

Only [`src/soa.rs`](src/soa.rs) is written here: the backend business logic (`system.of.record` and its
JSON Schema / Avro variants), the two schema-typed reply variants, and the HTTP status policy
(`sync.error.handler`). Everything else is reused: `sync.prepare`, `sync.await`, `soa.reply` and the Redis
coordinator from `mercury-sync-over-async`; `simple.kafka.notification`, the Kafka flow adapter and the
Schema Registry codec from `mercury-minimalist-kafka`; `http.flow.adapter` from the flow engine. The
libraries start themselves — the binary only links them, and the roles differ by configuration alone.

## Prerequisites

- A Rust toolchain (stable). Build once from this repository's root: `cargo build -p sync-over-async-demo`.
- The Java repository's `redis-standalone` and `kafka-standalone` helper jars — plain TCP dev servers,
  no Docker (`x.y.z` = the current version in that repository).
- The Java repository's `schema-registry-standalone` helper jar — only for the
  [JSON Schema](#json-schema-variant-confluent-wire-format) and [Avro](#avro-variant-confluent-wire-format)
  variants below (Protobuf is [not supported](#protobuf-not-supported)).
- Node.js 18+ — only for the one-time topic-creation helper in `node/` (the app client is `curl`).

## Run it

### Terminal A — Redis
```shell
java -jar redis-standalone-x.y.z.jar
```
### Terminal B — Kafka
```shell
java -jar kafka-standalone-x.y.z-exec.jar
```

### Create the topics (10 partitions each) — once
```shell
cd examples/sync-over-async-demo/node
npm install        # once
node create-topics.js
```
This creates six topics with **10 partitions** each: `soa.request` / `soa.response` (the raw path),
`json-topic-1` / `json-topic-2` (the [JSON Schema path](#json-schema-variant-confluent-wire-format)), and
`avro-topic-1` / `avro-topic-2` (the [Avro path](#avro-variant-confluent-wire-format)). Don't rely on Kafka
auto-creation here: an auto-created topic has a single partition, so the `soa-reply-group` consumer group
can only place it on **one** facade — a second facade would stay idle. Multiple partitions let the group
spread across facades, which is what makes the multi-facade test below meaningful.

> Run this **before** the apps (Terminals C/D) — otherwise the flow adapter auto-creates the topics at 1
> partition first, and `create-topics.js` will then skip them as already existing. If they already exist at
> 1 partition from an earlier run, restart `kafka-standalone` first (it wipes all topics on restart), then
> create them here.

### Terminal C — backend pod
```shell
cargo run -p sync-over-async-demo -- -Dapp.profiles.active=backend
```
### Terminal D — facade pod (REST on :8400)
```shell
cargo run -p sync-over-async-demo -- -Dapp.profiles.active=facade
```
### Terminal E — call it synchronously
```shell
curl -sS -X POST http://127.0.0.1:8400/api/sync-to-async \
     -H 'content-type: application/json' \
     -d '{"order":"A-100","item":"widget","qty":3}'
```
You get the backend's reply **synchronously**:
```json
{"status":"processed","processedBy":"system-of-record","processedAt":"2026-...Z",
 "traceId":"...","request":{"order":"A-100","item":"widget","qty":3}}
```
The facade and backend logs show the same `traceId` across the Kafka hops.

## Two things worth trying

- **Multiple facade pods on one machine** (requires the multi-partition topics created above). Start a
  second facade on another port and call it — the Redis return route delivers each reply to the pod that
  originated the request, even when the *other* facade is the one that consumed the reply off `soa.response`
  (watch which facade logs `soa.reply` vs. which one returns the HTTP response):
  ```shell
  cargo run -p sync-over-async-demo -- -Dapp.profiles.active=facade -Drest.server.port=8401
  curl -sS -X POST http://127.0.0.1:8401/api/sync-to-async -H 'content-type: application/json' -d '{"order":"B-200"}'
  ```
- **Timeout → HTTP 408.** Stop the backend (Terminal C) and call again with a short budget; with no reply,
  the facade times out and `sync.error.handler` returns 408:
  ```shell
  curl -sS -i -X POST http://127.0.0.1:8400/api/sync-to-async \
       -H 'content-type: application/json' -H 'x-sync-timeout: 2000' -d '{"order":"C-300"}'
  ```

## Mixing engines

The Java demo runs the same flows on the same six topics, so any pairing works — a Rust facade with a
Java backend, a Java facade with a Rust backend, or one consumer group holding a Java and a Rust backend.
The [interop test report](../../docs/test-reports/minimalist-kafka-interop.md) records the drive.

## JSON Schema variant (Confluent wire format)

The same end-to-end pattern, but the Kafka legs carry **Confluent JSON Schema** instead of raw bytes. It runs
over a parallel pair of topics — `json-topic-1` (request) and `json-topic-2` (reply) — so you can compare
the two side by side; the raw path (`soa.request` / `soa.response`) is untouched.

**Seed the registry, then start it.** Schemas are governed artifacts registered out-of-band, so the demo
ships pre-registered schemas rather than self-registering at runtime. The [`registry/`](registry/) folder
holds one file per schema id, each carrying its `subject` + `version` — [`1.json`](registry/1.json)
(subject `sync-demo-json`, a permissive JSON Schema covering both JSON legs), and
[`2.json`](registry/2.json) (subject `sync-demo-avro`, the [Avro variant](#avro-variant-confluent-wire-format)).
Copy them into the registry's store (default `/tmp/schema-registry`), then start the registry — one registry
serves both variants:
```shell
# Terminal F — seed + start the local Confluent-compatible Schema Registry (port 8081)
mkdir -p /tmp/schema-registry
cp examples/sync-over-async-demo/registry/*.json /tmp/schema-registry/
java -jar schema-registry-standalone-x.y.z.jar
```
The schema (the escaped `schema` string in the seed) is:
```json
{ "$schema": "http://json-schema.org/draft-07/schema#", "title": "SyncDemoMessage",
  "type": "object",
  "properties": { "action": {"type":"string"}, "status": {"type":"string"}, "processedBy": {"type":"string"} },
  "additionalProperties": true }
```
Now call the JSON endpoint (same request shape + synchronous reply as the raw path):
```shell
curl -sS -X POST http://127.0.0.1:8400/api/sync-to-async-json \
     -H 'content-type: application/json' -d '{"action":"create","order":"A-100"}'
```

What's different from the raw path (everything else — Redis return route, `sync.prepare`/`sync.await`,
trace continuity — is identical):

| Aspect | How |
|--------|-----|
| **Producer is subject-driven** | the flows publish with a `subject: sync-demo-json` header (no explicit `version`, so it defaults to `latest`); `simple.kafka.notification` resolves the global schema id and type from that subject, then frames the body in the Confluent wire format using the **pre-registered** schema. The producer never needs a naming strategy — see [the Minimalist Kafka guide](../../docs/guides/minimalist-kafka.md#schema). |
| **Schema registered out-of-band** | the registry is seeded from `registry/1.json` (subject `sync-demo-json`, version 1); the producer resolves the subject to an id and fetches the schema. Mirrors enterprise reality, where schemas are governed artifacts — no runtime registration in the app. |
| **Consumer decodes by id** | the `json-topic-1` / `json-topic-2` adapter bindings set `schema.enabled: true`, so the adapter reads the embedded id, fetches the schema, and hands the flow a decoded **map**. |
| **Map-input task variants** | because the decoded body is a map, `system.of.record.json` and `soa.reply.json` take a map (vs the bytes `system.of.record` / `soa.reply`); they reuse the same logic. |

## Avro variant (Confluent wire format)

The same end-to-end pattern again, now over **Confluent Avro** on a third pair of topics — `avro-topic-1`
(request) and `avro-topic-2` (reply). Nothing in the producer/consumer machinery changes from the JSON path:
the flows publish with a `subject: sync-demo-avro` header, and `simple.kafka.notification` resolves that
subject to the Avro schema id + type and dispatches to the Avro codec. The registry seed already includes
**subject `sync-demo-avro`** (`registry/2.json`, copied in the step above), so the same Terminal F registry
serves both variants.

Call the Avro endpoint:
```shell
curl -sS -X POST http://127.0.0.1:8400/api/sync-to-async-avro \
     -H 'content-type: application/json' -d '{"action":"create","order":"A-100"}'
```
You get the backend's reply synchronously — the record's four fields, with `traceId` continuous across the
Kafka hops:
```json
{ "action": "create", "status": "processed", "processedBy": "system-of-record", "traceId": "..." }
```
> Note `order` is **not** a field of the Avro `SyncDemoMessage` record, so it is **dropped on the wire** —
> Avro records are closed-shape, and the codec's map→record conversion only carries declared fields. (The
> JSON variant's `additionalProperties: true` would carry it through.) Send a field the schema declares to
> see it round-trip.

The Avro `SyncDemoMessage` schema (id 2 in the seed) is:
```json
{ "type": "record", "name": "SyncDemoMessage", "namespace": "com.accenture.soa.demo",
  "fields": [ {"name":"action","type":"string","default":""}, {"name":"status","type":"string","default":""},
              {"name":"processedBy","type":"string","default":""}, {"name":"traceId","type":"string","default":""} ] }
```

The one instructive difference from JSON Schema — **Avro records are closed-shape**:

| Aspect | How |
|--------|-----|
| **Closed record, not open** | the JSON Schema is `additionalProperties: true`, so the raw/JSON reply can be an open, nested object. An Avro record declares its fields exactly, so `system.of.record.avro` builds a **flat** reply (`action`, `status`, `processedBy`, `traceId`) matching the record. |
| **Defaults fill partial input** | the request carries only `action`; the codec's map→record conversion applies each field's **schema default** (`""`) for the rest, so a partial input serializes cleanly (Avro's own strict JSON decoder would reject it). |
| **Generic, no codegen** | decode yields a generic record (no generated types), rendered back to a map — so the flow tasks stay map-in/map-out, exactly like the JSON variant. |

Everything else — subject-driven produce, out-of-band registration, `schema.enabled` decode-by-id, the Redis
return route, and trace continuity — is identical to the JSON path.

## Protobuf (not supported)

The Schema Registry integration supports **JSON Schema and Avro only** on both engines. The Java module
unwired Protobuf before its first release because Confluent's `kafka-protobuf-provider` depends on a
discontinued artifact carrying an unpatched denial-of-service CVE
([CVE-2026-45799 / GHSA-7xpr-hc2w-34m9](https://github.com/square/wire/security/advisories/GHSA-7xpr-hc2w-34m9));
this engine keeps parity. `PROTOBUF` is still recognized by the codec so an attempt to use it fails clearly
(a 501 naming the type), never silently — see the
[Minimalist Kafka guide](../../docs/guides/minimalist-kafka.md#schema).

## Streaming return route (cross-pod progressive rendering)

In a horizontally scaled deployment, the pod that *produces* progressive events is generally not the pod
holding the user's HTTP connection. The [streaming return route](../../draft-design-specs/sync-over-async-port.md)
closes that gap with **Redis alone — no broker anywhere**: the client opens the SSE channel on pod A; the
facade announces the session's correlation id as the first SSE event (`event: cid`); the client quotes that
cid in POSTs to pod B, which posts segments into the rendezvous purely through Redis. Both roles register
`soa.redis.health`, so each pod's `/health` shows the Redis probe. The two Kafka clients are switched off
in these profiles (`kafka.producer.enabled` / `kafka.consumer.enabled: false`), so the roles stay
broker-free although the one binary links the Kafka crate.

### Run it (three terminals, Redis only)

```shell
java -jar redis-standalone-x.y.z.jar                                        # Terminal A - Redis
cargo run -p sync-over-async-demo -- -Dapp.profiles.active=stream-ui        # pod A, port 8600
cargo run -p sync-over-async-demo -- -Dapp.profiles.active=stream-producer  # pod B, port 8601
```

Open a channel (the timestamped SSE client; the optional second argument sets the idle allowance in
seconds via `x-stream-idle-seconds`):

```shell
node scripts/sse-client.mjs http://127.0.0.1:8600/api/notifications
```

Copy the announced cid, then drive the producer:

```shell
curl -s -X POST http://127.0.0.1:8601/api/produce \
     -H 'content-type: application/json' \
     -d '{"cid": "<cid>", "mode": "chat"}'
```

Five tokens and the `done` terminal render progressively on the SSE side — **no sequence number exists
anywhere in the pipeline**: order is posting discipline → Redis list order → serialized drain → the edge's
ordered reply lane.

### Produce modes

| Body | Behavior |
|------|----------|
| `{"cid", "mode": "chat"}` | five ordered tokens + `eof` (the AI-chat shape) |
| `{"cid", "mode": "notify", "name": "orders", "body": "order 42 shipped"}` | one named event, as an uncoordinated backend service would emit |
| `{"cid", "mode": "close"}` | terminal `eof` — any producer may close the channel |
| `{"cid", "mode": "stall"}` | two tokens, NO terminal (producer-death chaos) |
| `{"cid", "mode": "lost", "type": "data\|eof", "name", "body"}` | **chaos**: store the segment but suppress its wake-up (a lost notification) |

Every response reports `live`: `false` means the rendezvous is over (the UI pod closed, timed out, or died —
the orphan contract), which is the producer's signal to stop.

### The chaos runbook

The five scenarios of the cross-pod test report
([docs/test-reports/streaming-return-route-cross-pod.md](../../docs/test-reports/streaming-return-route-cross-pod.md)),
reproducible end to end:

1. **Ordered tokens across pods** — open a channel, produce `chat`: exact order, terminal metadata,
   rendezvous closed.
2. **Lost notification, healed by the next drain** — produce `notify` (renders), then `lost` (stored,
   nothing renders), then `notify` again: the second wake-up's drain delivers both queued segments in list
   order. A dropped signal costs latency, never data.
3. **Lost close, recovered by the final drain** — open with a short idle allowance
   (`node scripts/sse-client.mjs <url> 6`), then `lost` a data segment AND a `lost` `eof`: nothing wakes
   pod A, and at idle expiry the facade watchdog's single final drain completes the render normally.
4. **Producer killed mid-stream** — produce `stall`, then `kill -9` pod B: no terminal can ever arrive, so
   the watchdog fails the render in-band (`event: error`, 408 `Stream idle timeout`) and deletes the
   rendezvous keys.
5. **UI pod killed** — run pod A with a short route TTL (`-Dsync.stream.ttl.seconds=30`), open a channel,
   `kill -9` pod A: a post inside the TTL window is accepted into the void (`live: true` — the TTL-bounded
   crash backstop), and the producer stops (`live: false`) as soon as the route expires. Bounded,
   deterministic, no liveness machinery.

## How it maps to the pattern

| Concern | Where |
|---------|-------|
| The synchronous edge | `rest.yaml` → `http.flow.adapter` → the `sync-to-async` flow; `x-sync-timeout` sets the per-request await budget (default 10 s) |
| Return-route registration | `sync.prepare` (`begin(cid)` in Redis before the publish — fail-fast if Redis is down) |
| The async hop | `simple.kafka.notification` publishes the request; a publish failure fails the flow (→ `sync.error.handler` → 503) |
| The wait | `sync.await` parks on the correlation id; 408 on timeout |
| The backend | `system.of.record` (+ `.json` / `.avro`) — the only business logic; its flow publishes the reply |
| Reply delivery | the facade's flow adapter consumes the response topic → `soa.reply` (+ `.json` / `.avro`) → `deliver(cid)` → the Redis return route wakes the right pod |
| Status policy | `sync.error.handler`: 408 passes through, 5xx → 503, and the pending entry is aborted |
