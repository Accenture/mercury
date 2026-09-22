# Minimalist Kafka

*Guide: the opt-in `mercury-minimalist-kafka` crate — route Kafka topics into Event Script flows,
publish events to Kafka, speak the Confluent Schema Registry wire format, and health-check the
cluster.*

> **At a glance**
>
> - **What** — `minimalist-kafka` is an opt-in library with two composable building blocks: an **inbound**
>   Kafka Flow Adapter that routes each topic (or regex-matched set of topics) into an Event Script flow (the
>   Kafka counterpart of `rest.yaml`), and an **outbound** notification function that publishes an event to
>   a topic.
> - **Config, not code** — Kafka client connection/security comes from external `kafka-producer.yml` /
>   `kafka-consumer.yml` templates (librdkafka parameter names); the YAML binds topics (literal or regex) to
>   flows with optional consumer group, partition pinning, a per-binding dead-letter topic, and a per-binding
>   delivery mode. Enterprise SASL/OAuth2/mTLS is configured, never coded.
> - **Reliable, with a throughput escape hatch** — at-least-once consume (commit-after-process) by default,
>   bounded retry then a per-binding dead-letter topic, and continuous W3C trace context across the Kafka
>   hop; a binding may opt into Kafka-native auto-commit for higher throughput instead.
> - **Typed payloads, opt-in** — the Confluent Schema Registry wire format for JSON Schema and Avro values,
>   subject-driven on the way out and decoded by embedded id on the way in.
> - **For** developers and operators triggering flows from Kafka, or emitting Kafka events from a flow.

Everything on this page describes this repository's engine (`crates/minimalist-kafka`, crate
`mercury-minimalist-kafka`). It is the lock-step twin of the Java engine's `system/minimalist-kafka`: the
same adapter YAML, the same flow dataset, the same record headers, the same Confluent frame — so a Rust
flow adapter consumes what a Java flow publishes and vice versa (the two-engine drive is recorded in the
[interop test report](../test-reports/minimalist-kafka-interop.md)). Where this engine's Kafka client
forces a difference, the [deltas](#deltas) section states it.

The built-in HTTP flow adapter routes HTTP requests into flows. `minimalist-kafka` does the same for Kafka:
a topic listener mints an `EventEnvelope` and hands it to the Event Script engine, so the flow's tasks — not
the I/O layer — do the work. It is **not** a service mesh (the Java engine's `cloud.connector=kafka`, which
this port does not carry); this library is an application-level building block you opt into.

> **This is an opt-in library.** Add the `mercury-minimalist-kafka` dependency and set
> `yaml.kafka.flow.adapter` to activate the inbound adapter. The outbound `simple.kafka.notification`
> function and the `kafka.health` check register automatically.

## Enabling the library {#enable}

1. Depend on the crate (it depends on `mercury-event-script` and `mercury-platform-core`):

    ```toml
    [dependencies]
    mercury-minimalist-kafka = "x.y.z"   # the current Mercury version
    ```

2. Link it. The library registers its functions and its start-up hook through the same annotation
   inventory the rest of the engine uses, and inventory entries register at link time — so an application
   that activates the module **purely by configuration** must reference the crate once, or the Rust linker
   drops it as an unused dependency (the one line a Rust application needs where the Java jar needs only the
   dependency):

    ```rust
    use mercury_minimalist_kafka as _;
    ```

3. Point `yaml.kafka.flow.adapter` at your adapter config (inbound). Without it, no consumer starts.
4. Provide the Kafka client templates (see [client config](#client-config)) — the bundled defaults work
   for local dev.

```yaml
# application.yml
yaml.kafka.flow.adapter: classpath:/kafka-flow-adapter.yaml
```

The library autoloads at startup (a main-application hook, after every composable function is registered):
it builds the [Schema Registry codec](#schema) when `schema.registry.url` is set, the shared producer, and,
if `yaml.kafka.flow.adapter` is set, starts one consumer task per topic binding. A headless application —
one with no REST endpoint — stays running on its consumers: the adapter declares itself the process's
reason to exist, and on Ctrl-C or `SIGTERM` the consumers finish their in-flight record and leave the
group before the process exits. Either client can be [switched off](#opt-out) when the cluster has no
credentials for it.

### Switching off a client you do not use {#opt-out}

Both clients start by default. When the cluster grants credentials for only **one** of them — the usual case
for one leg of a bridge, where a Confluent console issues an API key for producing *or* consuming — switch
the unused one off:

```yaml
# a consume-only leg: this cluster issues no producer credentials
kafka.producer.enabled: false
```

| Setting | Effect when `false` |
|---|---|
| `kafka.producer.enabled` | No producer is built. `simple.kafka.notification` stays registered but fails with a message naming this key, so a flow that publishes anyway points at the config rather than at a missing route. |
| `kafka.consumer.enabled` | No adapter consumer starts, even with `yaml.kafka.flow.adapter` set, and [`kafka.health`](#health) probes through the producer template instead. |

Two rules worth knowing:

- **The flag is a veto, not a trigger.** Leaving it at the default starts nothing that is not otherwise
  configured — an inbound adapter still needs `yaml.kafka.flow.adapter`. Only the literal `false`
  switches a client off; any other value leaves it on.
- **A dead-letter topic needs a producer.** Dead letters are published through this cluster's own
  producer, so a binding that declares `dlq-topic` while `kafka.producer.enabled=false` **fails the
  deployment at startup**, naming both settings. It is the contradiction that matters: without the
  guard an exhausted message would be dropped with a `DATA LOSS` log and its offset committed. Enable
  the producer, or drop the `dlq-topic`.

Disabling both is allowed — the module goes inert and says so with a startup `WARN` — which makes a
"Kafka off in this profile" switch possible without removing the dependency (the `sync-over-async-demo`
does exactly that for its broker-free streaming roles).

## Inbound: the adapter YAML {#adapter-yaml}

`kafka-flow-adapter.yaml` lists `topic -> flow` bindings:

```yaml
consumer:
  - topic: 'incoming-orders'
    flow: 'process-order'
    group: 'sales-order-group'        # optional
    dlq-topic: 'incoming-orders-dlq'  # optional; no DLQ if omitted (failed messages dropped w/ ERROR)
  - topic: 'incoming-payments'
    flow: 'process-payment'
    partition: 0                      # optional
  - topic-pattern: 'events\.[a-z]{2}' # optional; regex subscribe instead of a literal 'topic'
    flow: 'process-region-event'
    group: 'region-events-group'      # required for topic-pattern bindings
  - topic: 'clickstream'
    flow: 'ingest-clickstream'
    auto-commit: true                 # optional; trades pod-death redelivery for throughput
    max-poll-records: 500             # optional; the client's prefetch depth on this engine
  - topic: 'mixed-events'             # second-level routing: pick the target per record
    serializer: 'json'                # optional; best-effort JSON decode on a non-schema topic
    flows:
      - 'input.header.type(order) -> flow://order-flow'
      - 'input.body.event.kind(refund) -> task://v1.refund.processor'
      - 'default -> flow://catch-all-flow'
```

| Field | Required | Description |
|-------|----------|-------------|
| `topic` | one of `topic`/`topic-pattern` | Literal source Kafka topic. |
| `topic-pattern` | one of `topic`/`topic-pattern` | Regex subscription instead of a literal topic (see [pattern subscription](#pattern)). |
| `flow` | one of `flow`/`flows` | Event Script flow id every message of this binding is routed into (direct routing). |
| `flows` | one of `flow`/`flows` | Second-level routing rule list — inspect a key-value of each record to pick the target flow or function per message (see [second-level routing](#routing)). |
| `group` | no (required for `topic-pattern`) | Consumer group id (see [consumer group](#group)). Defaults to `kafka-flow-adapter.<topic>` for a literal topic; no default exists for a pattern. |
| `partition` | no | Pins a single partition (see [partition pinning](#pinning)). Omit for group-managed assignment. Cannot be combined with `topic-pattern`. |
| `schema.enabled` | no | When `true`, decode the Confluent-framed value into a map before routing it into the flow (see [Schema Registry](#schema)). Default `false` (raw bytes). Flat (`schema.enabled: true`) or nested (`schema:` / `enabled: true`) spelling. |
| `serializer` | no | `'json'` = best-effort JSON decode of the record value on a non-schema topic (see [payload prerequisites](#routing-payload)). Mutually exclusive with `schema.enabled`. |
| `ttl` | no | Deadline for `task://` routing targets (duration syntax, e.g. `30s`, `5m`; default 30s) — a bare function has no flow ttl. Flow targets always use their own flow `ttl`. |
| `dlq-topic` | no | Pre-provisioned topic for exhausted messages (see [reliability](#reliability)). No DLQ if omitted. |
| `auto-commit` | no | When `true`, use Kafka-native auto-commit instead of the default manual commit-after-process (see [delivery mode](#delivery-mode)). Default `false`. |
| `max-poll-records` | no | On this engine, the client's per-partition prefetch depth (`queued.min.messages`) — see [delivery mode](#delivery-mode). |
| `correlation.id.header` | no | Per-binding override of the global `kafka.correlation.id.header` (default `cid`) — impedance matching for an upstream that publishes its own correlation-id header name (e.g. `X-Correlation-ID`). |
| `trace.id.header` | no | Per-binding override of the global `kafka.trace.id.header` — a fallback trace-id source for an upstream that does not send a W3C `traceparent` (which always takes precedence). |
| `traceparent.header` | no | Per-binding override of the global `kafka.traceparent.header` (default `traceparent`) — the header carrying the **full W3C trace context**, for **backward compatibility with a legacy upstream only** (departure from the W3C/OTel standard is discouraged). The standard `traceparent` always wins; the custom name is read only when the standard is absent. |

The file is read by the configuration reader, so **every value supports `${ENV_VAR:default}`
substitution** — e.g. `group: '${KAFKA_CONSUMER_GROUP:sales-order-group}'`. A malformed entry (missing
`topic`/`topic-pattern`, missing or duplicated `flow`/`flows`, a malformed routing rule or one referencing an
unknown flow or task route, `serializer` combined with `schema.enabled`, `schema.enabled` without a
`schema.registry.url`, an invalid regex, a `dlq-topic` that equals or matches its own source, etc.) fails
startup fast and loud rather than being silently skipped.

### Message dataset {#dataset}

Every message hands the flow a map with three top-level objects — `input.body`, `input.header`, and
`input.metadata`:

| Field | Type | Description |
|-------|------|--------------|
| `body` | bytes or map | The message payload; a map when [`schema.enabled`](#schema) decodes a Confluent-framed value or [`serializer: 'json'`](#routing-payload) parses a JSON object (a list for a JSON array), raw bytes otherwise. |
| `header` | map of strings | The record's Kafka headers, including `traceparent` (consumed for [trace continuity](#tracing)) and `cid` (correlation id) when the producer set them. |
| `metadata` | map | The record's own envelope facts — `topic`, `partition`, `offset`, `timestamp` (epoch milliseconds), and `key` (omitted when the record carries no key). |

`metadata.topic` and `metadata.partition` are the record's **actual** topic and partition — not the
binding's configured `topic`/`topic-pattern`. For a literal `topic` binding this is redundant (the flow
already knows the topic from its own YAML), but for a [`topic-pattern`](#pattern) binding it is the *only*
way a flow recovers which of the many matched topics a given message came from, since every matched topic
shares one `flow`. It is equally useful for a reprocessing flow bound to a `dlq-topic`: `metadata.topic`
there is the DLQ topic itself, while the `dlq.origin.topic` header (see [reliability](#reliability)) carries
the original source topic — together they let a reprocessor recover both "where this landed" and "where it
came from" without any framework-side rule/schema code.

Because `metadata` is just another field on `input`, a task's own `input:` mapping can pass it straight to
a composable function's parameter — no `model.*` relay needed. This is what makes
[`topic-pattern`](#pattern) practical for a "serving" function that must vary its behavior by the concrete
topic a message arrived on, even though every matched topic shares one `flow`:

```yaml
# in the first task of a topic-pattern flow, passed straight to the composable function
input:
  - 'input.metadata.topic -> topic'      # e.g. 'events.de' - the function decides per-topic behavior
  - 'input.metadata.partition -> partition'
  - 'input.body -> body'
process: 'topic.aware.dispatcher'
```

`model.*` is only needed when a *later* task (not the one receiving the message) needs the value — store it
once (`'input.metadata.topic -> model.source_topic'`) and reference `model.source_topic` from there on.

### Second-level routing {#routing}

Direct routing sends every record of a binding to one flow. When one topic carries mixed event types
(a common Kafka pattern — e.g. a `type` header distinguishing orders from shipments), **second-level
routing** picks the target per record instead: replace `flow` with a `flows` rule list (exactly one of
the two, never both):

```yaml
consumer:
  - topic: 'mixed-events'
    serializer: 'json'                 # optional; enables the input.body rule below
    ttl: '30s'                         # optional; deadline for task:// targets (default 30s)
    flows:
      - 'input.header.type(order) -> flow://order-flow'
      - 'input.header.type(order-*) -> flow://order-variant-flow'
      - 'input.header.type(regex: ^shipment-(eu|us)$) -> flow://shipment-flow'
      - 'input.body.event.kind(refund) -> task://v1.refund.processor'
      - 'default -> flow://catch-all-flow'
```

Each rule is `<selector>(<matcher>) -> <target>`, plus the **mandatory** `default -> <target>` fallback.

**Selectors** inspect one key-value of the inbound record:

- `input.header.<name>` — a Kafka record header. The header **name** lookup is case-insensitive
  (Kafka preserves the producer's wire casing, so a rule must not depend on it); the value comparison
  stays case-sensitive.
- `input.body` followed by a dot-bracket composite path — a map body via `input.body.order.type`, a
  **top-level list body** via `input.body[0].type`, and any nesting of the two
  (`input.body.items[1].kind`). Body rules match only when the body is a map or list — see
  [payload prerequisites](#routing-payload) below.

**Matchers** — three modes, explicit over sniffing:

| Form | Mode | Notes |
|------|------|-------|
| `type(order)` | exact | case-sensitive value comparison |
| `type(order-*)` | wildcard | the presence of `*` makes it one; each `*` matches any run of characters |
| `type(regex: <expr>)` | regex | always explicit — the exception, not the norm |

Wildcard and regex matchers use **full-string** matching (the `topic-pattern` precedent), so
`regex: shipment` does not match `my-shipment-1`.

**Evaluation.** Order matters: the **first matching rule wins**, in declaration order — put the most
specific rule first. A missing header/key, a non-map body for an `input.body` rule, or a non-text
value is a **non-match, never an error**; when no rule matches, `default` decides.

**Targets:**

- `flow://<flow-id>` — dispatch to an Event Script flow exactly as direct routing does: same
  [dataset](#dataset), same `model.cid` seeding, same [trace continuity](#tracing), same flow `ttl`.
- `task://<route>` — invoke a registered composable function **directly**, for processing simple enough
  that a flow is overweight. No input/output data mapping: all inbound record headers are copied to the
  function's input headers, the whole payload (bytes or decoded map) is the body, and trace context
  plus the business correlation-id propagate exactly as on the flow path (the function reads
  `my_correlation_id` from its input headers, or `PostOffice::my_correlation_id()`, as usual). There is
  no `metadata` map on this path — a function that needs the record's envelope facts should be fronted by
  a flow instead. A bare function has no flow `ttl`, so the binding's optional `ttl` (duration syntax:
  `30s`, `5m`; default 30s) is the invocation deadline.

Both target kinds sit in the **unchanged reliability envelope**: unless `auto-commit` is on, the offset
commits only after the selected flow or task finishes successfully, and a failure follows the same
bounded-retry-then-[`dlq-topic`](#reliability) path. A routing non-match is not a failure — it selects
`default`.

All rules are validated at startup, fail-fast: every rule must parse (regexes compile; body keys use
the dot-bracket composite-path convention), exactly one `default` is required, every `flow://` target
must be a compiled flow, and every `task://` target must be a registered route (functions preload
before the adapter starts) other than the flow engine itself — dispatch flows with `flow://`, never
`task://event.script.manager`.

#### Payload prerequisites and `serializer: 'json'` {#routing-payload}

`input.body.*` rules need a map body. On a [`schema.enabled`](#schema) binding the Confluent decode
already yields one. For a registry-less topic (not every installation uses a schema registry), the
optional per-binding `serializer: 'json'` tells the adapter to **try** deserializing each record value as
JSON before routing:

- a JSON **object** becomes a map — `input.body.<key>` rules match, and the selected flow/task
  receives the decoded map;
- a JSON **array** becomes a list — addressable by bracket rules (`input.body[0].type(order)`) and
  delivered as decoded;
- anything else — a scalar, or **malformed text** — keeps the **raw bytes**, which simply pass to the
  selected target. There is no special poison handling in the adapter: a target that cannot digest
  the bytes fails normally into the retry/DLQ path, while a `default` target designed for raw bytes
  handles them directly.

`serializer` is mutually exclusive with `schema.enabled` (the registry owns that decode) and is useful
on a plain `flow` binding too — the flow receives a map body without a schema registry. The parameter
is open-ended for later extension; `json` is the only supported value today. Numbers keep their JSON
width in the dynamic body (an integer stays an integer, a decimal a float).

### Consumer group {#group}

`group` is the Kafka consumer group id, used **exactly as given**. Enterprise DevSecOps teams typically
provision topics, ACLs, and consumer groups administratively, so the library never decorates the value. For
a literal `topic` it defaults to `kafka-flow-adapter.<topic>` for convenience in dev/test; a `topic-pattern`
binding has no sensible default (a regex string is not a group id) and must set `group` explicitly. All
instances that share a group load-balance that binding's partitions; set it explicitly to your assigned
group in production.

### Partition pinning {#pinning}

When `partition` is present, the consumer **manually assigns** that single topic-partition instead of joining
the consumer group for dynamic assignment. This bypasses group rebalancing — the pinned consumer reads
exactly that partition — so you own the deployment model (one consumer per partition, or each pod pinning a
distinct partition via `partition: ${POD_PARTITION}`). Offsets still commit under the configured group.
Omit `partition` for normal group-managed consumption. Mutually exclusive with `topic-pattern` (below), since
manual assignment needs concrete topic-partitions up front.

### Pattern subscription {#pattern}

Set `topic-pattern` instead of `topic` to subscribe to every topic matching a regex, using the client's
native regex subscription: the client tracks which topics currently match and adds/removes them from the
subscription automatically as matching topics are created — no adapter-side polling of topic metadata, no
restart needed when a new matching topic appears (it joins at the next full metadata refresh,
`topic.metadata.refresh.interval.ms`, five minutes by default). All messages from every matched topic
route into the same `flow`.

```yaml
  - topic-pattern: 'events\.[a-z]{2}'   # matches events.de, events.fr, events.us, ...
    flow: 'process-region-event'
    group: 'region-events-group'        # required - no sensible default for a regex string
```

The pattern is **full-string**, as on the Java engine: the adapter hands the client `^(<pattern>)$`. It is
validated at startup by this engine's regex library and compiled again by the client, so a dialect
difference surfaces as a subscribe error at startup, never silently.

Two rules follow from this: `topic-pattern` cannot be combined with `partition` (manual assignment needs
concrete topic-partitions up front, which a pattern does not provide), and `group` must be set explicitly.
`dlq-topic`, if configured, must not itself match the pattern (see [reliability](#reliability)).

## Kafka client configuration {#client-config}

Connection and security settings live in **template files**, not code, because enterprise Kafka varies
widely (on-prem, cloud, SaaS, Confluent; SASL/PLAIN, SASL/SCRAM, OAuth2, mTLS):

- `kafka-producer.yml` — used by the publisher and the dead-letter writer.
- `kafka-consumer.yml` — base config for every adapter consumer.
- `schema-registry.yml` — the Confluent Schema Registry client (see
  [registry authentication](#schema-auth)).

By default, each is loaded from the bundled template (the application's own `resources/` copy shadows it
— `classpath:/kafka-producer.yml`, with a `.properties` twin also accepted). Set
`kafka.producer.properties`, `kafka.consumer.properties`, or `schema.registry.properties` only when you
want a different location. A single location is normal; a comma-separated list is an optional fallback
chain, useful when CI/CD renders an external file into a deployment volume and you still want to fall back
to the bundled template. All template values support `${ENV_VAR:default}` substitution. The keys are
**librdkafka parameter names** — the same `bootstrap.servers`-style keys the Java templates use, so a
field template ports nearly verbatim; the genuinely JVM-only keys (`key.serializer`, `sasl.jaas.config`,
`partitioner.class`, …) are ignored with a startup log line naming each one. The library **pins** only
the parameters its contract depends on and lets the template own everything else:

| Concern | Pinned by the library | From the template |
|---------|-----------------------|-------------------|
| Serialization | — (the client is byte-native; nothing to pin) | — |
| Delivery semantics (consumer) | `enable.auto.commit` / `queued.min.messages` — per-binding overlay (see [delivery mode](#delivery-mode)) | `auto.offset.reset` |
| Partitioning (producer) | `partitioner` **defaulted** (not pinned) to `murmur2_random` | any `partitioner` set here wins |
| Connection / security | — | `bootstrap.servers`, `security.protocol`, `sasl.*`, `ssl.*`, `acks` |

`bootstrap.servers` is template-only via `${KAFKA_BOOTSTRAP_SERVERS:127.0.0.1:9092}`, and the shipped
consumer template sets `auto.offset.reset: ${KAFKA_AUTO_OFFSET_RESET:earliest}` — a brand-new consumer
group starts from the beginning of the topic; committed offsets govern thereafter. The bytes wire contract
keeps the building blocks serializer-free; richer encodings layer on top via the
[Schema Registry integration](#schema) (JSON Schema / Avro), opt-in per binding.

> **Why a random partitioner?** Kafka's default is a *sticky* partitioner — throughput-friendly, but at
> low volume it lands everything on one partition, leaving a multi-instance consumer group mostly idle.
> The library defaults the client's built-in `murmur2_random` partitioner: keyless records spread
> uniformly at random, keyed records keep the **Java-producer-compatible murmur2 key hash** — so a key
> maps to the same partition from either engine, which is what makes a mixed Java+Rust consumer group
> behave. Records with an explicit `partition` header bypass it. Set `partitioner` in
> `kafka-producer.yml` to override.

> **Enterprise security is a template block.** SASL_SSL, SCRAM and OAUTHBEARER (`sasl.oauthbearer.method:
> oidc` with the token endpoint, client id and secret) are librdkafka settings the bundled templates carry
> commented out. They need the crate's `ssl` feature (`mercury-minimalist-kafka = { version = "x.y.z",
> features = ["ssl"] }`), which builds the client with its vendored OpenSSL. There is no JVM allow-list
> for OAuth token URLs on this engine — nothing to register.

## Reliability: delivery mode, retry, and dead-letter {#reliability}

### Delivery mode {#delivery-mode}

By default (`auto-commit: false`, or omitted) the consumer commits offsets **only after** the flow finishes
a message, one message at a time — this engine's consumer receives records one at a time by construction,
so the Java module's `max.poll.records=1` pin has nothing to pin. If the instance crashes before the commit,
Kafka redelivers to a surviving instance in the group — the deliberate resilience-over-throughput
trade-off.

Set `auto-commit: true` on a binding to trade that guarantee for throughput: Kafka commits offsets on its
own periodic timer regardless of processing outcome. A message being processed when a pod dies may already
be considered committed and is **not** redelivered. Retry/dead-letter handling on flow failure is unaffected
either way — auto-commit only changes *when* Kafka considers the offset committed, not whether a failure is
retried or dead-lettered. Choose this per binding for high-volume topics (e.g. clickstream/telemetry) that
can tolerate occasional loss on crash in exchange for throughput; leave strict topics on the default.

An explicit `max-poll-records` maps to the client's per-partition **prefetch depth**
(`queued.min.messages`) — the nearest fetch-tuning analog on this engine; the mapping is stated in the
startup log. The Java module's mode defaults (1 manual / 500 auto-commit) are not applied: manual mode's
batch-of-one is inherent here, and auto-commit mode keeps the client's own prefetch defaults.

A flow **succeeds** when it replies with a status below `400` (any 2xx/3xx). A `4xx`/`5xx` status — or a
thrown error, including a **timeout** when the flow does not reply within its own `ttl` — is a **failure**.
(Kafka is asynchronous, so unlike an HTTP entry the adapter has no inherent request timeout: the flow's
`ttl` *is* the processing deadline. There is no separate flow-timeout knob.)

### Retry and dead-letter

On a **failure**, the message is retried up to `kafka.flow.max.retries` times (with
`kafka.flow.retry.backoff.ms` between attempts), then written to the binding's configured `dlq-topic`:

- **One DLQ topic per binding**, not per concrete topic — a `topic-pattern` binding that matches many topics
  still has a single `dlq-topic` (or none). The same flow that consumes a matched topic can reprocess a
  dead-lettered message later regardless of which concrete topic it originated from; that provenance is
  preserved via the `dlq.origin.topic` header, so a shared DLQ is not the "mixing source schemas"
  anti-pattern it would be for unrelated topics. `dlq-topic` must not equal the source `topic`, nor match
  `topic-pattern`, or a dead-lettered message would be re-consumed by the same binding and fail forever —
  the adapter rejects that configuration at startup.
- `dlq-topic` is **optional**. When omitted, a message that exhausts retries is dropped with a logged
  `ERROR` instead of being dead-lettered — the same fallback used when the DLQ write itself fails (below).
- The DLQ write is **confirmed** (it awaits the broker acknowledgement, bounded by `kafka.dlq.timeout.ms`);
  on success the offset commits (or, in auto-commit mode, is left to Kafka's own timer as usual).
- **DLQ topics must be pre-provisioned** (Kafka auto-creation is off in production). The original record's
  headers are preserved, plus `dlq.origin.topic` and `dlq.error`.
- **A schema decode failure is a poison message** and skips the retry loop: on a
  [`schema.enabled`](#schema-consume) binding, a record that is not Confluent-framed or whose embedded id
  cannot be resolved is dead-lettered at once with the raw record — retrying cannot help.

> **When there is no DLQ, or the DLQ write itself fails (data loss).** A failed write to the DLQ is an
> *exception of an exception* with no further fallback. Blocking the partition to retry forever would re-run
> the failing flow and re-attempt the failing DLQ write indefinitely — a self-sustaining **recovery storm**
> (a known cause of prolonged outages). So the adapter instead logs a loud `ERROR` and **commits** (in
> manual-commit mode), deliberately **dropping that one message** to keep the partition live. This is a
> conscious data-loss trade-off; a planned improvement is a classic resilience **alternative path** —
> persisting the record to a durable store for later replay instead of dropping it.

**Reprocessing** (read the DLQ topic → fix → replay) is business-domain logic and is intentionally out of
scope: the library guarantees durable capture (when a `dlq-topic` is configured and reachable), not replay.

### Consumer liveness: rebalances and the processing deadline {#liveness}

Two robustness behaviors keep a binding alive through the realities of consumer-group life:

- **The poll loop survives transient consumer errors.** A group rebalance (scale-out, pod churn)
  routinely fails an in-flight offset commit. The loop logs a `WARN` and continues — the uncommitted
  records simply redeliver to whichever consumer owns the partitions after the rejoin, preserving
  at-least-once delivery (flows must be idempotent, as always). Any *other* unexpected error keeps the
  binding alive too, with an escalating pause (1s doubling to 30s) and an `ERROR` per occurrence — loud
  but alive, instead of a consumer task that dies silently until the pod restarts.
- **`max.poll.interval.ms` is derived from the binding's worst-case processing time.** Message processing
  happens between two polls (the flow's `ttl` is the deadline), so the worst case is the full retry
  envelope — `(kafka.flow.max.retries + 1) ×` the slowest reachable flow/task `ttl` `+ retries × backoff`
  — plus headroom. If that exceeds Kafka's `max.poll.interval.ms` (default 5 minutes), the group
  coordinator evicts the consumer mid-processing and the subsequent commit fails. The adapter therefore
  computes the envelope per binding at startup and raises `max.poll.interval.ms` to cover it (never
  lowering it below the Kafka default; the derivation is logged). An explicit `max.poll.interval.ms` in the
  consumer template is an operator decision and is respected as-is — with a `WARN` when the computed
  envelope exceeds it. Raising the interval is low-risk: a crashed pod is still detected by heartbeats
  (`session.timeout.ms`; broker-side group configuration under the
  [KIP-848 consumer protocol](#rebalance-protocol)); this setting only bounds time between polls.

### Consumer rebalance protocol (KIP-848) {#rebalance-protocol}

Kafka's *classic* rebalance protocol is client-driven with a group-wide synchronization barrier: when
cloud infrastructure interrupts one pod, **every** member of the group stops, rejoins, and re-syncs —
and a flapping member repeats that storm. The **KIP-848 consumer rebalance protocol** (GA since Apache
Kafka 4.0) moves coordination to the broker's group coordinator and makes reassignment fully
incremental: only the interrupted member's partitions move, survivors keep consuming. On clusters that
support it, this materially reduces rebalance time and the CPU churn of unscheduled rebalances.

The protocol is selected per cluster in `kafka-consumer.yml` via `group.protocol`:

| Value | Behavior |
|-------|----------|
| *(unset)* / `classic` | Kafka's classic protocol — works on every broker. |
| `consumer` | The KIP-848 protocol, unconditionally. Fails at runtime if the cluster does not support it. |
| `auto` | Start with `consumer`; if the broker refuses the protocol at the first join, rebuild the binding's consumer once with `classic`. |

The bundled `kafka-consumer.yml` sets `group.protocol: ${KAFKA_GROUP_PROTOCOL:auto}`, so **`auto` is the
default**: a KIP-848 cluster gets the incremental protocol without configuration, an older cluster keeps
`classic`. Override it with the `KAFKA_GROUP_PROTOCOL` environment variable or in your own template. The
Java engine ships the same default.

**How `auto` decides.** The Java module probes the cluster's finalized `group.version` feature once at
startup; this engine's client exposes no feature probe, so `auto` is **optimistic**: each binding's
consumer starts with the consumer protocol, and a broker that lacks it reports the refused join as a fatal
`ConsumerGroupHeartbeat` error (`UNSUPPORTED_VERSION` where the coordinator has the protocol disabled, an
unsupported-feature error before Kafka 4.0). The adapter recognizes exactly that error, logs a `WARN`, and
rejoins the binding with `classic`. A refused join never becomes a group member, so there is no probe
group and no extra member; other fatal errors (a group authorization failure, say) are the consumer's own
and are not masked. The decision is per binding and stated in the log.

**Client tuning that conflicts.** Under the consumer protocol, `session.timeout.ms`,
`heartbeat.interval.ms` and `partition.assignment.strategy` move to broker-side group configuration — a
client that sets them together with `group.protocol=consumer` is refused. When the template sets any of
them, `auto` therefore resolves to `classic` with a `WARN` naming the conflicting keys: remove them to let
`auto` upgrade. Everything the adapter itself manages — `group.id`, the delivery-mode overlay, the
[derived `max.poll.interval.ms`](#liveness) — is valid under both protocols.

**Prerequisites and managed services.** The cluster must run Apache Kafka 4.0+ with the
`group.version` feature enabled (new 4.0+ clusters enable it at format time; upgraded clusters enable
it explicitly — check with `kafka-features.sh describe`). Confluent Platform 8.x carries the Apache
4.x core; for other managed or Kafka-compatible services (Confluent Cloud, AWS MSK, Azure Event Hubs),
verify against your actual cluster — wherever the protocol is refused, `auto` simply keeps `classic`.
Migration is online: a group converts when members join with the consumer protocol (mixed members
interoperate during a rolling deploy) and reverts if all new-protocol members leave. To force the classic
protocol regardless of cluster support, set `group.protocol: classic` explicitly.

### Shutdown: leaving the group {#shutdown}

On process shutdown — `SIGTERM` from an orchestrator's rolling restart, or Ctrl-C — the flow adapter stops
every binding's consumer before the process exits, and the consumer's close sends the group coordinator a
**LeaveGroup**: the member's partitions are reassigned to the surviving members at once. Without that,
the broker only notices the dead member when its session expires — 45 seconds by default under the
KIP-848 consumer protocol — and every partition it held sits unread for that long, which on a rolling
deploy is a pause of the same length for the pod's share of the traffic. The adapter registers the stop on
the platform's shutdown lifecycle (`Platform::on_shutdown`) when its consumers start, next to the
[keep-running declaration](#enable), and the entry point runs the hooks once the signal arrives. Each
consumer finishes the record in hand first — the stop is honoured between records, never mid-flow — and the
hook waits up to ten seconds for all of them, so a stuck flow cannot hold the shutdown hostage (a record
still in flight after the grace redelivers: at-least-once). The log confirms each step — `Kafka flow
consumer for topic '<topic>' stopping`, then `stopped`, then `Kafka flow consumers stopped` — and the
broker's own log shows the member leaving instead of being fenced. After the consumers, the shared producer
is **flushed** — every record already accepted by `simple.kafka.notification` or a dead-letter write is
delivered before the process exits — and then forgotten, so a late caller is told the producer is not
started; the log line is `Kafka producer flushed and closed`. The flush waits through the client's linger
(`linger.ms`, 5 ms by default) for the acknowledgements and is bounded by the same ten seconds:
a stopping pod must not wait on a dead broker past its termination grace, so when the broker cannot take
the records in time the log names how many were left undelivered (`Kafka producer flush incomplete after
10 s - N message(s) undelivered`). The Java engine shuts down in the same order; its producer close waits
without bound.

## Outbound: publishing to Kafka {#outbound}

`simple.kafka.notification` is a composable function that publishes an event to a topic. Send it an
`EventEnvelope` with a `topic` header (required), an optional `partition` header (see
[partitioning strategies](#partitioning)), a body, and any other headers (forwarded as Kafka headers):

```rust
po.send(
    EventEnvelope::new()
        .set_to("simple.kafka.notification")
        .set_header("topic", "outgoing-events")
        .set_header("cid", &business_correlation_id)
        .set_raw_body(rmpv::Value::Binary(payload_bytes)),
)
.await?;
```

The body is bytes (published verbatim — the minimalist default), or a **map/list, automatically
serialized to JSON bytes** — the outbound symmetry of the inbound [`serializer: 'json'`](#routing-payload):
the producing application writes a map, the wire carries JSON bytes, and a consuming binding with
`serializer: 'json'` hands its flow a map again. This JSON convenience applies to **non-schema-registry
topics only**; `null` stays `null` (a Kafka tombstone).

Publishing is **drop-n-forget** (Kafka's commit log is the durable buffer), but async delivery failures are
logged rather than silently masked.

Two contract details worth knowing: any **other body type** (a string, a number) is rejected loudly with a
400 — convert to bytes or a map/list first. And the **correlation-id header is auto-stamped as a
fallback**: when the flow maps no value under the configured header (default `cid`), the publisher stamps
the flow's own business correlation id (`model.cid`); an explicitly mapped value always wins. With a
customized `kafka.correlation.id.header`, map to the configured name — a `header.cid` mapping under a
custom name is forwarded as a literal `cid` record header, never renamed.

One header opts a publish into the Confluent wire format instead of raw bytes: `subject` (with an optional
`version`; see [Schema Registry](#schema)). It is an encoding directive — consumed by the function, not
forwarded as a Kafka header. On this schema path **the body must be bytes (a pre-serialized JSON document)
— passing a map or list is rejected with a 400**; the map/list JSON convenience applies to non-schema
topics only.

> **The codec is shared.** On this engine the Schema Registry codec is thread-safe, so the notification
> function's default pool of five workers shares one codec — there is no per-worker encoder and no
> kernel-thread constraint (the Java page's "keep the worker pool small" note has no analog here). Raise
> `instances` only if profiling shows the publishing path is the genuine bottleneck.

### Partitioning strategies {#partitioning}

Three mechanisms decide which partition an outbound message lands on, in precedence order:

1. **Explicit `partition` header** — the caller (usually a flow's data mapping) names the target
   partition and every partitioner is bypassed. This is the building block for **content-based
   partitioning** (below).
2. **A partitioner in the producer template** — the externalized `kafka-producer.yml` may set the
   client's `partitioner` to any of its built-ins (`murmur2_random`, `murmur2`, `consistent_random`,
   `consistent`, `random`, `fnv1a`, …); the library defaults it and never overrides a template's value.
   A partitioner sees the record's key, not its headers or payload — header-based or content-based
   partitioning belongs in the flow (below), on this engine as on Java.
3. **`murmur2_random` — the library default**: keyless records spread uniformly at random (Kafka's own
   sticky default batches onto one partition, which starves multi-instance consumer groups at low
   volume); keyed records keep the Java-compatible murmur2 hashing.

**Content-based partitioning — the composable pattern.** When the partition must be derived from a
record header or a payload key-value (a tenant, an entity id), compute it in the flow — where the
whole record is visible — and pass the explicit `partition` header. A tiny selector function plus
data mapping, no client plumbing:

```yaml
tasks:
  - input:
      # any header can drive the decision - including ones a Kafka partitioner could never see
      - 'input.header.x-routing-value -> header.routing-value'
      - 'input.body -> *'
    process: 'partition.selector'          # e.g. hash(routing-value) % partition count
    output:
      - 'result.partition -> model.partition'
      - 'result.payload -> model.payload'
    description: 'Derive the target partition from the record content'
    execution: sequential
    next:
      - 'simple.kafka.notification'

  - input:
      - 'text(outgoing-events) -> header.topic'
      - 'model.partition -> header.partition'   # explicit partition bypasses all partitioners
      - 'model.payload -> *'
    process: 'simple.kafka.notification'
    output: []
    description: 'Publish to the selected partition'
    execution: end
```

A deterministic selector (same value → same partition) gives per-entity ordering — the classic reason
for content-based placement. This is the outbound mirror of [second-level routing](#routing): content
inspection expressed in the application layer, where it is legible and governable and can see the
whole record, rather than buried in client configuration.

### Trace continuity across Kafka {#tracing}

Rather than forwarding the caller's stale `traceparent`, the notification function stamps a **fresh** W3C
`traceparent` from its own current span; the adapter parses it on the way in and chains the flow onto that
span. The result is one continuous distributed trace across the asynchronous Kafka boundary — the two
notification hops are the bridge spans, and the span chain is exact across the engine boundary too (the
interop report shows a Java span parenting a Rust one). See [Observability](observability.md).

## Health check {#health}

The library ships a ready-made health-check function at route **`kafka.health`** (registered automatically
when the crate is linked). Opt in by listing it as a health dependency in `application.yml`:

```yaml
mandatory.health.dependencies: kafka.health
# or, when Kafka should be reported but not fail /health:
# optional.health.dependencies: kafka.health
```

The probe is deliberately minimal: one Kafka **Metadata** request using the module's
[consumer template](#client-config) — it joins no consumer group, commits no offsets, and needs no admin
privileges. The Metadata request itself requires **no ACL**: brokers filter the response to the topics the
principal may Describe rather than rejecting the request, so under a fully locked-down principal the probe
still succeeds (with a visible topic count of 0) — the successful round trip proves connectivity, TLS/SASL
authentication, and a served API request. A reachable cluster reports a status map (`Kafka cluster is
reachable`, with the visible topic count); an unreachable one fails the check with a **503** status and a
key-value message (`text` for the DevOps reader, `code` for the status code), so `/health` marks the
dependency down and the endpoint answers non-2xx while the application is DOWN.

During application start-up the check returns a **placeholder healthy** status (`Kafka client is starting
up`) while the client warms up in the background — `/health` neither fails nor blocks before the client and
the rest of the start-up sequence complete. After the first successful probe, or once the grace period
expires, every check is live. Two keys tune the behavior: `kafka.health.timeout` (default `5s`) and
`kafka.health.startup.grace` (default `30s`).

The probe's client configuration is resolved **lazily** — when the probe client is built, and again
whenever a failed probe forces a rebuild — never at construction time. `kafka.health` is registered before
your main application runs, so a bootstrap that fetches secrets and publishes them as configuration
overrides (the vault pattern) has not executed yet — a template frozen at construction would interpolate
such a credential as *missing* and fail every probe from then on. With lazy resolution the first probe
after the credential lands simply succeeds; nothing needs a restart. While the template is still incomplete
— the client cannot even be built from it — the check reports a **passing** `Waiting for Kafka connection`
status rather than a failure: failing `/health` would invite the container orchestrator to restart the pod,
and a restart cannot produce the credential. A real connectivity failure (client built, cluster
unreachable) fails the check with status 503.

> **On a produce-only leg the probe uses the producer template.** With
> [`kafka.consumer.enabled=false`](#opt-out) there are no consumer credentials to build a probe from,
> yet a bridge is healthy only when both clusters are reachable. So the probe follows whichever client
> the deployment configured, reading connection and security settings (`bootstrap.servers`,
> `security.protocol`, `sasl.*`, `ssl.*` — named identically in both client surfaces) from
> `kafka-producer.yml`. Producer-only settings such as `acks` are filtered out. Nothing else about the
> probe changes; it still joins no group and needs no ACL.

The Java page's notes on JVM classloaders and class-valued settings describe a JVM concern with no analog
on this engine.

## Schema Registry: typed payloads (opt-in) {#schema}

The default wire contract is raw bytes, which keeps the building blocks serializer-free. To interoperate
with existing Confluent client projects, the library can also speak the **Confluent Schema Registry wire
format** — `[magic 0x00][4-byte global schema id][payload]` — for **JSON Schema and Avro** values. The Java
module uses Confluent's own serializers as a library; there is no Confluent client for Rust, so this engine
implements the frame, the subject/version resolution and the two codecs itself, on the Apache Avro
reference crate (`apache-avro`) and a JSON Schema validator (`jsonschema`). The wire format is byte-for-byte
the same: a JSON Schema payload is the JSON document, an Avro payload is the binary datum against the
registered writer schema, and the two engines' demos exchange both over one topic pair (see the
[interop report](../test-reports/minimalist-kafka-interop.md)).

> **Protobuf is not supported.** The Java module unwired it before its first release because Confluent's
> `kafka-protobuf-provider` depends on a discontinued artifact carrying an unpatched denial-of-service CVE
> ([CVE-2026-45799 / GHSA-7xpr-hc2w-34m9](https://github.com/square/wire/security/advisories/GHSA-7xpr-hc2w-34m9));
> this engine keeps parity. `PROTOBUF` is still recognized as a schema type so a misconfigured attempt
> fails clearly (a 501 naming the type), not silently.

Set `schema.registry.url` to turn the feature on (point it at a real Confluent registry or the Java
repository's local [`schema-registry-standalone`](https://accenture.github.io/mercury-composable/guides/schema-registry-mock/)
mock, a plain dev server with no Docker). When it is unset, schema features stay off and the library keeps
its raw-bytes behavior.

```yaml
schema.registry.url: '${SCHEMA_REGISTRY_URL:http://127.0.0.1:8081}'
schema.registry.cache.ttl: 30m                         # TTL for the in-memory schema cache (by id)
```

### Registry authentication (OAuth 2.0 / basic) {#schema-auth}

The registry client's authentication parameters live in the **`schema-registry.yml` template** (same
mechanics as the producer/consumer templates: bundled by default, optionally relocated with
`schema.registry.properties`). The keys and values are the Confluent Schema Registry client's own, so a
Java-side `schema-registry.properties` ports by renaming the file — with one difference to know: the Java
module passes the template **verbatim** to the Confluent client, while this engine **interprets** it. The
keys below are honoured by name; any other key is logged at startup as ignored, never silently.

| Key | Meaning |
|-----|---------|
| `bearer.auth.credentials.source` | `OAUTHBEARER` (client credentials), `STATIC_TOKEN`, or `SASL_OAUTHBEARER_INHERIT` (reuse the Kafka client template's `sasl.oauthbearer.*` settings — one credential for broker and registry). |
| `bearer.auth.issuer.endpoint.url`, `bearer.auth.client.id`, `bearer.auth.client.secret`, `bearer.auth.scope` | The client-credentials grant. |
| `bearer.auth.token` | The fixed token for `STATIC_TOKEN` (dev/test). |
| `bearer.auth.cache.expiry.buffer.seconds` | Refresh the token this long before it expires (default 300). |
| `bearer.auth.logical.cluster`, `bearer.auth.identity.pool.id` | Confluent Cloud: sent as the `target-sr-cluster` and `Confluent-Identity-Pool-Id` headers. |
| `basic.auth.credentials.source` | `USER_INFO` (`basic.auth.user.info` = `user:password`), `URL` (credentials carried in `schema.registry.url`), or `SASL_INHERIT` (the Kafka template's `sasl.username` / `sasl.password`). |

OAuth 2.0 client-credentials (e.g. Azure AD / Entra ID):

```yaml
bearer.auth.credentials.source: OAUTHBEARER
bearer.auth.issuer.endpoint.url: '${SCHEMA_REGISTRY_OAUTH_TOKEN_URL:}'
bearer.auth.client.id: '${SCHEMA_REGISTRY_CLIENT_ID:}'
bearer.auth.client.secret: '${SCHEMA_REGISTRY_CLIENT_SECRET:}'
bearer.auth.scope: '${SCHEMA_REGISTRY_OAUTH_SCOPE:}'
```

The client fetches the bearer token from the issuer endpoint exactly as Kafka's own retriever does (the
client id and secret as HTTP Basic on the token request, `grant_type=client_credentials` and the scope in
the form body), sends it as `Authorization: Bearer` on every registry request, and **caches it**, refreshing
before expiry (`expires_in`, else the token's own `exp` claim). Keep secrets in environment variables via
the `${ENV_VAR}` substitution; the shipped template is fully commented out, so an unauthenticated registry
(like the local mock) keeps working with zero configuration.

**TLS trust comes from the operating system's trust store** — the registry is reached through the
platform's own HTTP client, which trusts what the OS trusts (the same rule as the JDK default). A private
CA is installed in the OS store; the Java template's `schema.registry.ssl.*` truststore keys have no analog
here and are reported as ignored.

### Produce: subject-driven {#schema-produce}

`simple.kafka.notification` serializes the body into the wire format when you supply a `subject` header:

| Header | Description |
|--------|-------------|
| `subject` | The registry **subject** to serialize against. The schema must be **pre-registered**; the producer resolves the subject to a global schema id (and its type — `JSON` or `AVRO`) from the registry and never registers. |
| `version` | Optional. The subject version to resolve: a positive integer to **pin** a specific version, or `latest` to track the current version. Defaults to `latest`. |

```yaml
# in a flow task that publishes via simple.kafka.notification
input:
  - 'text(orders) -> header.topic'
  - 'text(orders-value) -> header.subject'    # version omitted → latest
  - 'model.payload -> *'        # the body: must be bytes (a JSON document) on the schema path
process: 'simple.kafka.notification'
```

The producer resolves the subject (+ version) to a **global schema id** and its **schema type** from the
registry, converts the document with that type's codec and frames it with the id. The wire format carries
only the global id, and the consumer (id-from-wire) is unchanged; only the producer's *input* is a subject
rather than an explicit id+type. Whoever registers the schema — CI, a client project, an admin tool — owns
the subject naming strategy (TopicName / RecordName / TopicRecordName are all fine, and a topic can carry
many record types). This assumes schemas are **governed artifacts registered out-of-band**, as they are in
practice; the producer never auto-registers. The `subject` and `version` headers are encoding directives:
they never reach Kafka as record headers.

### CSFLE (Client-Side Field Level Encryption) {#csfle}

**Not supported on this engine — and refused rather than degraded.** The Java module supports
[Confluent CSFLE](https://docs.confluent.io/cloud/current/security/encrypt/csfle/overview.html) by
*delegation*: a schema's `ruleSet` (its `ENCRYPT` rules tagging fields) travels with the schema, and
Confluent's own serializer/deserializer run those rules — with a KMS driver on the classpath — during the
same serialize/deserialize call the library already makes. This engine has no Confluent serializer to
delegate to, and the alternative — writing plaintext where the schema declares encryption — would be a
silent security regression. So a fetched schema that carries a `ruleSet` (an `ENCRYPT` rule or any other
Confluent data-contract rule) fails its lookup with a 501 naming the rules: the producer refuses to publish,
and a consumer dead-letters the record. The same holds for **schema references** (`references` on a
registered schema): register self-contained schemas for the topics a Rust pod produces or consumes.

The Java module's `schema.registry.serde.*` pass-through is read for the one setting with an analog here —
`schema.registry.serde.json.fail.invalid.schema` (see the [notes](#schema-notes)); any other key under that
prefix is logged as unsupported.

### Consume: decode by embedded id {#schema-consume}

Set `schema.enabled: true` on a consumer binding. The adapter reads the magic byte + embedded id, looks up
the registered schema's type, dispatches to the matching decoder, and hands the flow a **map** as
`input.body` (instead of bytes). No flow-YAML change is needed (`input.body -> *` is type-neutral); a
schema-fed flow task simply takes a map instead of bytes. The decoded document also feeds the binding's
[routing rules](#routing), so `input.body.*` selectors work on a schema topic.

```yaml
consumer:
  - topic: 'orders'
    flow: 'process-order'
    group: 'order-group'
    schema.enabled: true
```

A **decode failure is a poison message** (retrying will not help), so the raw record is dead-lettered
immediately via the [DLQ path](#reliability) rather than retried — the `dlq.error` header names the cause
(an unframed payload, an unresolvable id, a document that does not match its schema).

### Notes {#schema-notes}

- **One subject-driven path, two formats.** The producer and consumer are type-generic; only the `subject`
  (and the registered schema behind it) differ — the producer reads the schema type from the registry, so
  the flow never names it. JSON Schema is *open* (`additionalProperties`), while Avro records are
  *closed-shape* — a message must match the declared fields, and a non-schema field is dropped on the
  wire. The JSON→Avro conversion walks the writer schema, so an absent field takes its **schema default**
  and a missing field with no default fails fast (as Avro requires); unions are tried in declaration
  order, `bytes`/`fixed` values are written as Avro's byte-string text or a list of byte values, and a
  decoded record keeps its `bytes` binary. Avro decodes generically (no generated types), rendered to a
  map.
- **JSON validation is opt-in.** Like Confluent's serializer, a JSON document is validated against its
  schema only when `json.fail.invalid.schema=true` — in the registry template or as
  `schema.registry.serde.json.fail.invalid.schema` — on both produce and consume.
- **Schema cache.** Lookups by id are cached **in memory** (the platform's `ManagedCache`, TTL
  `schema.registry.cache.ttl`, default `30m`) to cut registry round-trips, and the parsed schema (the Avro
  schema with its named types, or the compiled JSON validator) is cached with the text, so per-record
  encode/decode never re-parses. A global schema id is immutable, so a cache hit is always the right
  schema. **Positive results only** — a not-found id is never cached, so a schema registered while the app
  is running becomes visible on the next lookup. The TTL lets schema changes be picked up without
  restarting pods (handy in dev / lower environments); lengthen it in production where schemas change
  rarely. The cache is rebuildable and **cleared at startup**.
- **Subject→id resolution cache.** The producer also caches the *subject (+ version) → schema id*
  resolution, and how long depends on the version. A **pinned numeric version** (`subject` + `version: N`)
  maps to one immutable schema id, so it is cached **long** (`schema.registry.version.cache.ttl`, default
  `10d`, bounded to 3000 entries). `latest` (the default) can change when a new version is registered, so
  it shares the short-TTL id cache and is re-resolved frequently, picking up a new current version without
  a restart. Pin a version in production paths where the schema must not shift underneath you; use
  `latest` in dev / lower environments where tracking the newest schema is convenient.
- **A second registry is a second codec.** The default codec reads the `schema.registry.*` keys; a
  library that talks to another cluster's registry builds its own under another key prefix, with its own
  caches — global ids are only unique within one registry.
- **Worked example.** The [sync-over-async demo](https://github.com/Accenture/mercury/tree/main/examples/sync-over-async-demo)
  runs the same end-to-end flow over both formats (`json-topic-1/2`, `avro-topic-1/2`) alongside the raw
  bytes path, against the Java demo on the same topics.

## Configuration keys {#config}

All keys are documented in the [Configuration Reference](configuration-reference.md#kafka-flow-adapter).
The essentials:

| Key | Default | Description |
|-----|---------|-------------|
| `yaml.kafka.flow.adapter` | — | Adapter config location; unset = inbound adapter off. |
| `kafka.producer.enabled` | `true` | Set `false` on a consume-only leg to build no producer — see [switching off a client](#opt-out). A binding with `dlq-topic` then fails startup. |
| `kafka.consumer.enabled` | `true` | Set `false` on a produce-only leg to start no adapter consumer; [`kafka.health`](#health) then probes through the producer template. |
| `kafka.producer.properties` | `classpath:/kafka-producer.yml` | Producer template location (a `.properties` twin is also accepted). Set to an external file path (or explicit fallback list) to externalize. |
| `kafka.consumer.properties` | `classpath:/kafka-consumer.yml` | Consumer template location. Set to an external file path (or explicit fallback list) to externalize. |
| `kafka.dlq.timeout.ms` | `10000` | Confirm-write timeout for the dead-letter publish. (Flow processing has no timeout knob — the flow's own `ttl` is the deadline.) |
| `kafka.flow.max.retries` | `3` | Retry attempts before dead-lettering. |
| `kafka.flow.retry.backoff.ms` | `500` | Pause between retry attempts. |
| `kafka.correlation.id.header` | `cid` | The record header carrying the business correlation id, both directions (per-binding override: `correlation.id.header`). |
| `kafka.trace.id.header` | — | An optional legacy trace-id header stamped outbound and read inbound when no `traceparent` is present. |
| `kafka.traceparent.header` | `traceparent` | The header carrying the W3C trace context; when customized, the context is stamped under both names. |
| `kafka.health.timeout` | `5s` | The probe's round-trip deadline. |
| `kafka.health.startup.grace` | `30s` | How long `/health` reports the placeholder status while the client warms up. |
| `schema.registry.url` | — | Confluent Schema Registry URL; unset = [schema features](#schema) off (raw bytes). |
| `schema.registry.properties` | `classpath:/schema-registry.yml` | Registry client template location — authentication parameters interpreted by name (see [registry authentication](#schema-auth)). Set to an external file path (or explicit fallback list) to externalize. |
| `schema.registry.cache.ttl` | `30m` | TTL for the in-memory schema cache (by id); positive results only; cleared at startup. |
| `schema.registry.version.cache.ttl` | `10d` | TTL for the pinned subject+version resolutions (bounded to 3000 entries). |
| `schema.registry.serde.json.fail.invalid.schema` | `false` | Validate JSON documents against their schema on produce and consume. |

## Differences from the Java engine {#deltas}

The behaviour above is the Java module's; these are the places where this engine's client (librdkafka
through `rdkafka`) or the Rust ecosystem forces a difference — each is also recorded in the port's design
spec (`draft-design-specs/minimalist-kafka-port.md`, §7):

| Area | Java engine | This engine |
|------|-------------|-------------|
| Activation | the jar on the classpath is enough | one `use mercury_minimalist_kafka as _;` line when nothing else references the crate |
| Client templates | Kafka Java client keys | librdkafka keys (the same `bootstrap.servers`-style names; JVM-only keys ignored with a log line); SASL_SSL/OAUTHBEARER need the crate's `ssl` feature |
| Default partitioner | the `SimpleRandomPartitioner` class | the client's built-in `murmur2_random` (same semantics, Java-compatible key hash) |
| `max-poll-records` | the poll batch size (mode defaults 1 / 500) | the client's prefetch depth (`queued.min.messages`); one record at a time is inherent |
| `group.protocol=auto` | one `group.version` feature probe per cluster | optimistic: `consumer` first, rebuilt once as `classic` when the broker refuses the join |
| `topic-pattern` | `subscribe(Pattern)` | the anchored `^(<pattern>)$` handed to the client's regex subscription; a new matching topic joins at the next metadata refresh |
| Threading | Kafka-driving functions on kernel threads; Confluent serdes owner-confined | native client threads + async tasks; one shared, thread-safe codec |
| Headless app | the JVM stays up on non-daemon consumer threads | the adapter declares that it keeps the process running; `SIGTERM`/Ctrl-C stop the consumers gracefully ([shutdown](#shutdown)) |
| Shutdown | closes every consumer, then flushes and closes the producer, waiting without bound | stops every consumer (each leaves its group on close), then flushes the producer within the 10 s grace and forgets it; records the grace could not deliver are counted in the log |
| Schema codecs | Confluent's serializers as a library | this engine's own JSON Schema and Avro codecs on `jsonschema` / `apache-avro`; the frame and payloads are byte-identical |
| Registry template | passed verbatim to the Confluent client | interpreted by name (unknown keys logged); TLS trust from the OS store |
| CSFLE, rules, references | delegated to the Confluent serdes and KMS drivers | refused with a 501 (never plaintext); register self-contained schemas |
| Avro conversion | the first non-null union branch; a string for `bytes` fails at the serializer | unions tried in declaration order; `bytes` from a byte-string or a byte list |
| `twin-kafka` | the second-cluster bridge module | not ported (deferred until a bridge need exists); the codec and template loader carry the key-prefix seam it would use |

## See also

- [Interop test report — Minimalist Kafka](../test-reports/minimalist-kafka-interop.md) — the two-engine drive
  on the Java `kafka-standalone` helper: routing, DLQ, a mixed Java+Rust consumer group, rebalance chaos,
  the Schema Registry legs.
- [Observability](observability.md) — how trace context stays continuous across the Kafka hop.
- [Configuration Reference](configuration-reference.md#kafka-flow-adapter) — every Kafka flow-adapter key.
- [Event Script syntax](event-script/syntax.md) — the flow YAML the adapter dispatches into.
- The Java engine's [Minimalist Kafka guide](https://accenture.github.io/mercury-composable/guides/minimalist-kafka/)
  (the canon this page twins), its [Sync-over-Async guide](https://accenture.github.io/mercury-composable/guides/sync-over-async/)
  and [Twin Kafka](https://accenture.github.io/mercury-composable/guides/twin-kafka/) (not ported).
- The worked examples: [`kafka-demo`](https://github.com/Accenture/mercury/tree/main/examples/kafka-demo)
  (both routing styles + notification) and
  [`sync-over-async-demo`](https://github.com/Accenture/mercury/tree/main/examples/sync-over-async-demo)
  (synchronous REST over Kafka with the Redis return route, raw / JSON Schema / Avro legs).
