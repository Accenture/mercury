# Design — minimalist-kafka → Rust (Kafka flow adapter + notification)

> **Status:** APPROVED 2026-09-14 (Eric: Q1–Q5 ruled — §9). Next gate: experiment K1 ·
> **Realizes:** `ot-minimalist-kafka-port` · **Serves:** `vision-mercury` ·
> **Author:** Claude Code · **Date:** 2026-09-14
>
> **Canonical source:** `system/minimalist-kafka` (mercury-composable, Java) and its guide
> `docs/guides/minimalist-kafka.md` — the opt-in library with two composable building blocks:
> an **inbound** Kafka Flow Adapter (topics → Event Script flows, the Kafka counterpart of
> `rest.yaml`) and an **outbound** notification function — plus `kafka.health`. This document
> maps that canon to Rust; the Java module is the reference implementation.
>
> **Maintainer input driving this draft (Eric, 2026-09-14):**
> 1. **Hold** the `mercury-sync-over-async` crates.io publication and the deferred one-shot
>    facade tasks (sync-over-async ruling Q1) **until minimalist-kafka is implemented** — this
>    module is exactly the transport those tasks were waiting for, so one arc closes both.
> 2. **Investigate `mockforge-kafka 0.3.221`** as the unit-test double (§5).
> 3. **Integration and interop tests run against the Java repo's standalone helpers**
>    (`kafka-standalone`, `schema-registry-standalone` — plain TCP dev servers, no Docker;
>    already staged locally under `helpers/`).

## 1. Goal & scope

Give Rust applications the same opt-in Kafka building blocks the Java engine ships: consume
topics into Event Script flows and publish events to topics — **config, not code** — with the
same reliability contract (at-least-once by default, bounded retry, per-binding dead-letter),
the same trace/correlation continuity, and a health check. Completing this unlocks two held
items: the sync-over-async facade tasks (their request leg finally has a transport) and the
crates.io publication of both new crates together.

**In scope (the port target, staged by the K-series in §8)**

- A new workspace crate (`minimalist-kafka`; §9 Q1 settles its home and package name) with:
  the **outbound** `simple.kafka.notification` function; the **inbound** flow adapter driven
  by `yaml.kafka.flow.adapter` (literal topics and regex patterns, per-binding consumer
  group, partition pinning, second-level routing, per-binding delivery mode, bounded retry +
  per-binding DLQ); **`kafka.health`** with the waiting-vs-outage semantics the platform's
  health checks share; externalized client templates (`kafka-producer.properties` /
  `kafka-consumer.properties`, `${ENV_VAR:default}` substitution); the opt-out flags
  (`kafka.producer.enabled` / `kafka.consumer.enabled`) including the dlq-needs-producer
  startup guard; W3C trace continuity and the correlation-id auto-stamp fallback.
- The in-process unit-test story (§5) and the live dry-run/interop story against the Java
  helpers (§8 K4) — including cross-engine interop: a Rust producer feeding a Java flow
  adapter and vice versa (the polyglot discipline the streaming return route established).

**Out of scope (each with its own gate)**

- **Schema Registry support** (Confluent framing, Avro/JSON-Schema serdes, OAuth2 to the
  registry, and CSFLE field-level encryption) — a large, self-contained surface on top of the
  raw-`byte[]` core. Proposed: defer to its own follow-up spec once the K-series lands
  (§9 Q2). The core is designed so the schema path bolts on (the Java shape: `schema.enabled`
  per binding, `subject` header on publish).
- **`twin-kafka`** (the second-cluster bridge) — a separate module in Java for the same
  reason it would be here; follows the same pattern once the primary module exists (§9 Q3).
- **The service mesh** (`cloud.connector=kafka`) — a different concern entirely, per the
  standing Java invariant (mesh is opt-in and unrelated to this library); not part of the
  Rust port's roadmap here.

## 2. What the Rust engine already has

| Prerequisite (Java) | Rust today |
|---|---|
| Event Script engine + flow launch (`event.script.manager`) | `crates/event-script` — same routes, same flow input dataset discipline |
| The flow-adapter shape (mint the input dataset, launch the flow, error contract) | `event-script/src/adapter.rs` (`http.flow.adapter`) — the pattern to mirror |
| `${ENV_VAR:default}` config + external file templates | platform-core `ConfigReader`/`AppConfigReader` (profiles, overrides) |
| Health-check contract (`type=info`/`type=health`, waiting-vs-outage, `{text, code}` 503) | shipped and field-tested via `soa.redis.health` (sync-over-async §6) |
| Lazy client construction (preload-before-bootstrap lesson) | the established `OnceCell`/supplier idiom |
| Startup lifecycle for an opt-in module | `#[before_application]` hooks (the sync-over-async bootstrap shape) |
| Per-pod identity, tracing, telemetry parity | `Platform::origin()`, W3C trace plumbing in the worker bracket |

What is missing is exactly the Kafka client and the module around it.

## 3. Canonical semantics (the port checklist, from the guide + Java source)

Each item is pinned by the ported suite (§5.4).

1. **Adapter YAML** (`kafka-flow-adapter.yaml`): `consumer:` bindings with
   `topic` | `topic-pattern` (exactly one), `flow` | `flows` (exactly one), optional `group`
   (default `kafka-flow-adapter.<topic>`; required for a pattern), `partition` pinning
   (literal topics only), `serializer: 'json'` (mutually exclusive with the schema flag),
   `ttl` for `task://` targets, `dlq-topic`, `auto-commit`, `max-poll-records`, and the
   per-binding header-name overrides (`correlation.id.header`, `trace.id.header`,
   `traceparent.header`). Every value supports `${ENV_VAR:default}`. **Malformed entries fail
   startup fast and loud** — the Java guide's whole validation list, including the
   dlq-equals-source check.
2. **Message dataset**: the flow receives `input.body` (raw bytes, or a map when
   `serializer: 'json'` parses one), `input.header` (the record's Kafka headers), and
   `input.metadata` — `topic`, `partition`, `offset`, `timestamp` (epoch ms), `key` (omitted
   when absent) — the record's ACTUAL facts, which is what makes pattern bindings and DLQ
   reprocessing flows workable.
3. **Second-level routing**: `flows:` rule list — `input.header.X(value) -> flow://id` /
   `task://route`, first match wins, `default ->` catch-all; rules validated at startup
   against known flows and routes; `task://` targets honor the binding `ttl`.
4. **Delivery modes**: default **at-least-once** (manual commit-after-process, poll batch 1);
   per-binding `auto-commit: true` (Kafka-native commits, batch 500, overridable via
   `max-poll-records`) trades pod-death redelivery for throughput.
5. **Retry + DLQ**: `kafka.flow.max.retries` (3) attempts with `kafka.flow.retry.backoff.ms`
   (500) pause; exhausted messages publish to the binding's `dlq-topic` through this
   cluster's own producer with the origin headers (`dlq.origin.topic` et al.), then commit;
   no `dlq-topic` = drop with an ERROR log, then commit. `kafka.dlq.timeout.ms` (10000)
   bounds the dead-letter confirm-write. A binding with `dlq-topic` while
   `kafka.producer.enabled=false` **fails deployment at startup** naming both settings.
6. **Outbound contract** (`simple.kafka.notification`): `topic` header required; optional
   `partition`; body `byte[]` verbatim or `Map`/`List` auto-serialized to JSON bytes
   (non-schema topics only); any other body type rejected loudly; `null` = tombstone;
   remaining headers forwarded as Kafka headers; the correlation-id header auto-stamped from
   the flow's own `model.cid` when unmapped (an explicit value always wins); publishing is
   drop-n-forget with async delivery failures logged.
7. **Partitioning precedence**: explicit `partition` header → keyed record (Kafka's own
   hashing) → the configured partitioner (the module ships the Java module's
   round-robin-random default; content-based partitioning stays a template concern).
8. **Trace continuity**: the W3C `traceparent` record header is stamped outbound and consumed
   inbound (standard name always wins; per-binding legacy overrides only fall back), so one
   trace crosses the Kafka hop — presentation parity with the Java engine is normative
   (polyglot field reality).
9. **Health** (`kafka.health`): the platform health contract with probe config resolved
   lazily at client build time (the vault pattern) and a **waiting** status while the client
   cannot even be built; only a real round-trip failure fails `/health` 503 `{text, code}`.
   With the consumer switched off it probes through the producer template.
10. **Opt-out flags are vetoes, not triggers** — and disabling both leaves the module inert
    with a startup WARN.
11. **Threading**: the Java module pins Kafka-driving functions to kernel threads because
    Java clients block and pin virtual-thread carriers. librdkafka runs its **own native
    threads** and the Rust binding exposes async handles — callers only await — so the
    correct mapping is the `soa.redis.health` stance (no special executor), not a literal
    kernel-thread port. Confluent-serde thread-safety constraints return with the schema
    phase, not before.

## 4. The Kafka client decision: `rdkafka` (librdkafka bindings)

The inbound adapter needs the full consumer-group protocol (group management, rebalancing —
the Java guide even documents KIP-848 behavior), regex subscription, partition pinning,
headers, and manual commits. Among Rust clients:

- **`rdkafka`** (librdkafka bindings) — the de-facto production client: complete group
  protocol (librdkafka tracks KIP-848), pattern subscription, assign/commit APIs, headers,
  async streams over its own native threads. **Chosen.**
- **`rskafka`** — pure Rust but *deliberately* excludes consumer groups (a low-level
  partition client by design). Disqualifying for this module.
- **`kafka` (kafka-rust)** — effectively unmaintained; incomplete group support. Disqualified.

**The C-build consideration** (VDI-class machines are a standing constraint): `rdkafka-sys`
compiles a vendored librdkafka with the standard C toolchain (`cc` + `make` — **no CMake in
the default build**, verified on a machine without CMake; §5 spike). The workspace already
requires a C compiler (`ring` via rustls), so the incremental toolchain demand is `make`,
which ships with every developer toolchain the repo already assumes. No Docker anywhere.
SSL/SASL feature flags are chosen at K1 (the enterprise templates need SASL_SSL; librdkafka
vendors or links per feature — decided with the template work, matching what the field
actually configures).

## 5. Test strategy — the `mockforge-kafka` investigation and the verdict

The Java module's suites run against an **embedded in-JVM KRaft broker**; Rust has no
embedded broker, which is the same gap the RESP double filled for Redis. The maintainer asked
whether `mockforge-kafka 0.3.221` answers it for unit tests.

### 5.1 What mockforge-kafka is

Part of the MockForge mocking suite (SaaSy-Solutions/mockforge; MIT OR Apache-2.0). The
`mockforge-kafka` crate implements a **Kafka broker simulation over the real wire protocol**
(real clients connect via `bootstrap.servers`): 10+ APIs, topic management, multi-partition
offsets, consumer-group coordination and rebalancing, plus fixtures/replay and metrics.
Actively developed — 178 releases in ~11 months (0.3.221 published 2026-09-13), which also
means a fast-moving 0.3.x API to pin.

### 5.2 What it costs

Its dependency tree is the suite, not a double: `mockforge-core` alone declares **63 normal
dependencies** — axum, sqlx, reqwest, a QuickJS engine (`rquickjs`), protobuf
(`prost`/`prost-reflect`), JWT and password-hashing crates, websockets — plus
`mockforge-recorder`, compression codecs with C code (`zstd-sys`), **and `rdkafka` itself**
(so librdkafka's C build arrives with the mock too). As a dev-dependency this puts hundreds
of crates and several native builds into every `cargo test` of the workspace — the opposite
of the RESP double's philosophy (one file, tokio-only), and a material cost on VDI-class
machines and CI.

### 5.3 The lighter candidate the client brings for free: `rdkafka::mocking::MockCluster`

librdkafka ships a built-in **mock cluster** (`rd_kafka_mock_*`), exposed by the `rdkafka`
crate as `MockCluster`: an in-process, protocol-real broker (configurable broker count,
"reasonable subset of Kafka protocol operations, error injection") maintained by the same
codebase that implements the client protocol. Two ways in: explicit
(`MockCluster::new(n)` → `bootstrap_servers()`), or **pure configuration**
(`test.mock.num.brokers` in an ordinary client config — no test wiring at all, which suits
this module's config-not-code philosophy exactly). Zero additional dependencies beyond the
client the module already uses.

### 5.4 Head-to-head spike (this round, same produce/consume scenario on both)

One scratch project, two binaries — produce 5 headed records, consume them in a consumer
group with manual commits, assert order/headers/offsets:

| Measure | `mockforge-kafka 0.3.221` + `KafkaMockBroker` | `rdkafka 0.38` `MockCluster` (control) |
|---|---|---|
| Round trip (produce 5 → group-consume 5, manual commits) | **Fails as documented**: the README quickstart times out on the FIRST produce (`MessageTimedOut`, partition -1). Root cause read from its source: topic auto-create lives in the **Produce** handler, but librdkafka never produces to a topic that **Metadata** does not list — and the Metadata handler does not auto-create (answered `1 brokers, 0 topics`). Passes only after pre-creating the topic through the crate's Rust-side test API (`test_create_topic`) or fixtures — then order, headers, offsets and commits are all correct (1.65s) | **Passes as documented** (`create_topic` on the cluster handle, or none needed with client-side auto-topic): exact order, headers, offsets 0–4, group + manual commits (3.2s incl. client bootstrap) |
| Protocol fidelity observed | ApiVersions/Metadata/Produce/Fetch/groups work once topics exist; on shutdown librdkafka logs a **malformed `LeaveGroup v1` response** (protocol read buffer underflow) | clean — the mock is maintained inside librdkafka itself, the same codebase that implements the client protocol |
| Dependency weight (scratch project, this machine) | **338 crates**, ~2m05s cold build — `mockforge-core` alone declares 63 direct deps (axum, sqlx, reqwest, a QuickJS engine, protobuf, JWT/crypto suites, websockets), plus `mockforge-recorder`, `zstd-sys`, **and `rdkafka` itself** | **zero additional crates** — ships inside the client the module needs anyway |
| Toolchain | both candidates build librdkafka from vendored source with `cc` + `make` only — verified on a machine **without CMake** (the VDI-relevant datapoint) | same |
| Test-wiring shape | Rust API (`KafkaMockBroker::new(config)` + fixture YAML) | explicit `MockCluster::new(n)` → `bootstrap_servers()`, **or pure config**: `test.mock.num.brokers` in an ordinary client config — no test wiring at all |
| Release cadence | 178 releases in ~11 months on 0.3.x (churn risk for a pinned dev-dep); MIT OR Apache-2.0 | rides `rdkafka`/librdkafka releases |

### 5.5 Verdict and the suite plan

**Verdict — `rdkafka::mocking::MockCluster` for the unit suites; `mockforge-kafka` not
adopted** (recorded 2026-09-14 with the evidence above; revisitable as the project matures —
its core protocol demonstrably works, and the auto-create placement and LeaveGroup encoding
are fixable upstream). The deciding factors, in order: the mock the module's own client
ships is **protocol-authoritative and dependency-free**, its config-only activation
(`test.mock.num.brokers`) matches this module's config-not-code philosophy exactly (unit
tests exercise the SAME template pipeline, pointed at the mock), and it passed the head-to-
head unchanged while the candidate needed its private test API to complete the same
scenario. The 338-crate / several-native-builds cost of the candidate would also be paid on
every `cargo test` of the workspace — the opposite of the one-file RESP-double precedent.

Suite plan on that foundation: unit suites per K-increment against `MockCluster` (groups,
commits, headers, retry/DLQ via error injection where the mock supports it, plus the
adapter's startup-validation table which needs no broker at all); anything the mock cannot
express honestly (broker restarts mid-poll, rebalance storms, real DLQ topics under load)
belongs to the K4 live lane against `kafka-standalone` — the same two-lane split the
sync-over-async port used (in-process double + live dry-run).

Integration and interop (the maintainer's direction, unchanged): the **Java helpers** —
`kafka-standalone` (a real single-node KRaft broker, no Docker) for live dry-runs, plus
`schema-registry-standalone` when the schema phase arrives; the K4 interop leg runs a Rust
producer into a Java flow adapter and a Java producer into a Rust one, like the streaming
return route's R4.

## 6. Crate layout & component map

| Java (`org.platformlambda.mini.kafka`) | Rust module | Notes |
|---|---|---|
| `KafkaClientConfig` (producer/consumer templates) | `client_config.rs` | template file loading + `${ENV}` substitution + per-key pass-through to rdkafka `ClientConfig` |
| `KafkaFlowAutoStart` | `bootstrap.rs` | `#[before_application]` shape: build producer, start consumers, register functions; opt-out flags; the dlq-needs-producer guard |
| `KafkaFlowAdapter` + `KafkaConsumerBinding` | `adapter.rs` | YAML parse + startup validation (§3 item 1) |
| `KafkaFlowConsumer` | `consumer.rs` | one consumer task per binding: poll → dataset → flow launch → commit/retry/DLQ |
| `RoutingRuleSet` | `routing.rs` | second-level rules, first-match-wins, startup validation |
| `RetryPolicy` | `retry.rs` | bounded retry + backoff + DLQ publish with origin headers |
| `SimpleKafkaNotification` | `notification.rs` | §3 item 6 contract |
| `SimpleRandomPartitioner` | — (config, not code) | librdkafka's built-in `murmur2_random` partitioner is defaulted by `client_config` — §7 item 4 |
| `KafkaHealthCheck` | `health.rs` | §3 item 9; the `soa.redis.health` pattern |
| `KafkaHeaders` / `KafkaRuntime` | `headers.rs` / `runtime.rs` | constants; process-wide holder (operations, never the closeable client) |
| Schema classes (`SchemaCodec`, serdes, registry client, CSFLE) | — | deferred with the schema phase (§1) |

## 7. Deliberate deltas (expected; each documented when it lands)

1. **Threading** (§3 item 11): librdkafka native threads + async awaits replace
   `@KernelThreadRunner`.
2. **Client configuration pass-through**: the same two template files, parsed by this
   engine's ConfigReader and handed to rdkafka's `ClientConfig` — librdkafka property names
   are the same `bootstrap.servers`-style keys the Java templates already use, so field
   templates port nearly verbatim; genuinely JVM-only keys are ignored with a startup log
   line naming them.
3. **KIP-848**: inherited from librdkafka's `group.protocol` support rather than the Java
   client's — behavior documented against the guide's rebalance section at K2.
4. **The default partitioner is config, not code** (K1). Java ships a custom
   `SimpleRandomPartitioner` class (keyless → uniform random; keyed → Java murmur2).
   librdkafka's built-in `murmur2_random` partitioner has exactly those semantics — including
   the Java-producer-compatible key hash, so a key maps to the SAME partition from either
   engine (interop-relevant) — and `client_config` defaults it (a template that sets
   `partitioner` wins, the `putIfAbsent` parity). Nothing needs pinning either: this client
   is byte-native, so the Java module's serializer pinning has no analog.
5. **`max.poll.records` has no librdkafka analog and needs none** (K2; the mapping decided at
   K3): the Java module pins `max.poll.records=1` to enforce one-at-a-time
   commit-after-process; this client's per-record async `recv()` IS a poll batch of one by
   construction. **K3:** an EXPLICIT per-binding `max-poll-records` maps to
   `queued.min.messages` — the client's per-partition prefetch depth, the nearest
   fetch-tuning analog — and the mapping is stated in the startup log. The Java mode
   defaults (1 manual / 500 auto-commit) are NOT applied: manual mode's batch-of-one is
   inherent, and auto-commit mode keeps the client's own prefetch defaults. The
   poll-interval envelope (Java `applyPollInterval` — `max.poll.interval.ms` derived from
   `(retries + 1) x` the slowest target ttl `+ retries x backoff + headroom`, never below the
   client default, an explicit template value respected with a WARN) is ported with a
   per-record multiplier of one for the same reason.
6. **Library activation needs one `use` line** (K2 finding): inventory entries register at
   link time, and the Rust linker drops an rlib no symbol references — so an application
   that activates the module purely by configuration must carry a single
   `use mercury_minimalist_kafka as _;` where the Java jar needs only the dependency.
   Documented in the crate docs; the K4 demo shows it.
7. **`group.protocol=auto` is resolved OPTIMISTICALLY — `consumer` first, `classic` on refusal**
   (K3 shipped `auto` = `classic`; re-ruled by Eric at K4, 2026-09-21, after the live check that the
   `kafka-standalone` broker finalizes `group.version=1` and that `consumer` works on this engine).
   The Java module probes the cluster's finalized `group.version` feature through the Admin
   client's `describeFeatures`; librdkafka exposes no feature probe, and every side-effect-free
   alternative was rejected (a broker config is not the feature flag; a trial join with a synthetic
   group hits ACLs). What librdkafka DOES give is a **fatal `ConsumerGroupHeartbeat` error** when a
   broker lacks the protocol — `_UNSUPPORTED_FEATURE` when the API is not advertised (before 4.0),
   `UNSUPPORTED_VERSION` when the coordinator has it disabled — so `auto` starts each binding's
   consumer with `consumer` and, on that error at the first join, rebuilds it once with `classic`
   (`consumer_protocol_rejected` in `consumer.rs`; other fatal errors such as a group
   authorization failure are the consumer's own and are not masked). Same semantics as the Java
   probe, resolved per binding, with no synthetic group and no extra member; the Java conflict
   guard is kept (`session.timeout.ms`, `heartbeat.interval.ms`, `partition.assignment.strategy`
   with `auto` → `classic` naming the keys). **Both engines ship `auto` uncommented in their
   bundled consumer templates** (`${KAFKA_GROUP_PROTOCOL:auto}`, Eric 2026-09-21), so a KIP-848
   cluster gets the incremental protocol by default and an older one still works.
8. Anything else discovered at implementation joins this list; wire-visible behavior
   (headers, DLQ headers, dataset shape) is normative parity, never a delta.
9. **Regex subscriptions are anchored with a capturing group** (K3): a `topic-pattern` is
   handed to the client as `^(<pattern>)$` — full-string, like the Java
   `subscribe(Pattern)`. The client's regex engine (minilibs, JavaScript-flavoured) rejects
   the `(?:` non-capturing form in a subscription (probed against librdkafka 2.12), so the
   group captures — harmless. The pattern is validated at startup with Rust's `regex`
   (fail-fast, and the dlq-must-not-match-pattern check), then compiled again by the client;
   a dialect difference between the three engines (Java, Rust `regex`, librdkafka) surfaces
   as a subscribe error at startup, never silently. A newly created matching topic joins on
   the next full metadata refresh (`topic.metadata.refresh.interval.ms`, default 5 minutes)
   — the client's analog of the Java `metadata.max.age.ms`.
10. **`${ENV:default}` substitution in the adapter YAML needs an application base config**
    (K3, a testing note rather than a behaviour delta): `ConfigReader::get` substitutes only
    when the `AppConfigReader` is initialised (the Java `baseConfig != null` parity). Every
    application satisfies it — the auto-start runs after the configuration snapshot — and
    the e2e proves the group substitution; only a bare unit test sees the literal.
11. **A headless application must declare that it keeps running** (K4 finding, fixed in
    platform-core): the JVM stays up on the Java module's non-daemon consumer threads; a Rust
    process has no such implicit hold, and `AutoStart::run` parked only for HTTP/websocket
    serving — the first live run of the Rust `kafka-demo` booted, started its consumers and
    exited 200 ms later. `Platform::keep_running(reason)` is the explicit analog, declared once
    by the flow adapter when its consumers start (and it registers their stop as a shutdown
    hook). A REST application is unaffected. Recorded with the drive in
    `docs/test-reports/minimalist-kafka-interop.md`, together with the platform-wide follow-up
    it exposed — the Rust entry point stopped on `SIGINT` only — closed the same day: the entry
    point now stops on `SIGTERM` too, and the flow adapter's shutdown hook waits (10 s grace) for
    its consumers to finish their in-flight record and leave the group explicitly.

## 8. Experiment plan (K-series)

| # | Experiment | Gate |
|---|---|---|
| K1 ✅ | Crate + client templates + **outbound** (`simple.kafka.notification`, partitioner) + `kafka.health` + the unit suite against the §5 double | **DONE 2026-09-14** — `crates/minimalist-kafka` ships the outbound contract (body shapes incl. tombstones and Java-parity rejects, header propagation with the reserved-header exclusions, cid auto-stamp fallback, fresh traceparent from the hop's own span, explicit-partition routing), `kafka.health` (placeholder/warm-up, waiting-vs-outage, `{text, code}` 503, topics count, produce-only-leg probe rule), the template pipeline (embedded defaults + app-classpath shadowing + override chains + the JVM-only-key filter), the opt-out flags, and library auto-activation via inventory (`#[preload]` + `#[main_application]` — Java classpath-scan parity). 16 tests green against `MockCluster`, incl. a traced RPC through a registered worker. Platform-core gained `PostOffice::my_span_id()` and the public `w3c_trace` export (Java `W3cTrace` parity) |
| K2 ✅ | **Inbound core**: literal-topic bindings, groups, manual commit-after-process, dataset (§3 item 2), retry + DLQ, opt-out flags + startup guards | **DONE 2026-09-14** — `adapter` (YAML parse + the fail-fast validation table, incl. rejecting later-increment fields BY NAME, the dlq-equals-source check, the group default, and the dead-letter-needs-producer deployment guard) and `consumer` (one tokio task per binding: async recv → dataset → real flow launch via `event.script.manager` → sync commit under `block_in_place` after the flow finishes; bounded retry + backoff; the confirmed DLQ write with `dlq.origin.topic`/`dlq.error` and original headers/body preserved; DATA-LOSS drop for partition liveness; escalating error backoff keeps a binding alive). E2e through the REAL Event Script engine against `MockCluster` — configuration only: dataset shape (actual topic/partition/offset/timestamp/key, raw byte body), W3C traceparent chaining + `KAFKA /<topic>` trace path + business-cid resolution, exactly-one-retry success, retry-exhaustion parking with origin facts, and no-DLQ drop with the binding still live after |
| K3 ✅ | Inbound completions: second-level routing, `topic-pattern`, partition pinning, auto-commit + `max-poll-records`, per-binding header overrides | **DONE 2026-09-21** — `routing` (the `flows` rule grammar: `input.header.<name>` / `input.body...` selectors, exact / wildcard / explicit-regex matchers anchored full-string, first match wins, the mandatory `default`, `flow://` / `task://` targets validated against the live flow registry and platform routes — `task://event.script.manager` refused); `adapter` (the full Java validation table: source and destination exclusivity, an invalid regex, pattern + `partition`, pattern without `group`, a `dlq-topic` that equals or matches its source, `serializer` only `json`, positive `ttl` / `partition` / `max-poll-records`, the header overrides in nested and flat form; `schema.enabled` the one field still deferred by name — §9 Q2); `consumer` (group `subscribe`, manual `assign` for a pinned partition, anchored-regex `subscribe` for a pattern; best-effort JSON decode BEFORE routing; `task://` dispatch with every record header copied, the whole payload as body, the business cid on the `my_cid` tag and the binding `ttl` as deadline; per-binding header names; auto-commit skips the manual commit); `client_config` (the delivery-mode overlay, the derived `max.poll.interval.ms`, `group.protocol=auto` → classic — §7 items 5, 7, 9). E2e against `MockCluster` through the REAL flow engine, configuration only: pattern binding with actual-topic metadata, header exact and wildcard rules → flow with a decoded map body, body rule → task (headers copied, cid via the tag, trace continuous), list body → task, non-JSON → default with raw bytes, the three header-override legs (trace-id fallback, custom traceparent adopted, standard traceparent wins), partition pinning (the unpinned partition never arrives), auto-commit delivery, live-registry startup validation. 34 unit tests + the 14-scenario e2e; whole crate green |
| K4 ✅ | **Live dry-run + interop** against `kafka-standalone`: Rust↔Java flow adapters both ways, DLQ and rebalance chaos; report kept as permanent record | **DONE 2026-09-21** — `examples/kafka-demo` ported (the four functions with the Java tests one to one, the three flows, the two-binding adapter YAML, the Node helpers copied, an `interop` relay profile), driven live against the Java `kafka-standalone` (Kafka 4.3.1): direct and second-level routing, the poison-order DLQ path (400 here vs 500 on Java in `dlq.error`), both chained interop legs under one trace id with the span chain exact across the engine boundary, a mixed Java+Rust consumer group (20 records split 10/10), a `SIGKILL`ed member's partitions taken over at the 45 s session timeout with 20/20 delivered once and an in-flight poison order redelivered and dead-lettered by Java, and the `SIGINT` graceful `LeaveGroup`. Finding fixed: the headless keep-alive (§7 item 11). Follow-up recorded: `SIGTERM` handling in the Rust entry point. Report: `docs/test-reports/minimalist-kafka-interop.md` |
| K5 | **The held items close**: sync-over-async facade tasks (`sync.prepare`/`sync.await`/`soa.reply`) over this transport + the demo's Kafka request leg; then the release gate publishes `mercury-sync-over-async` + this crate together | the Java sync-over-async MVP flow (`RestFlowMvpTest` analog) green in Rust; publication un-holds |

## 9. Maintainer rulings (Eric, 2026-09-14)

- **Q1 — crate home and name: AGREED, with a layout clarification (Eric, 2026-09-14).**
  Package `mercury-minimalist-kafka`, joining the publication set (the K5 release publishes
  it together with `mercury-sync-over-async`). Home: `crates/minimalist-kafka` — in this
  repository **`crates/` is the Java repo's `system/` analog** (all system modules live
  there), while the Rust `system/` folder carries the scope-specific `AGENTS.md` consumer
  surface. The same convention moves the dev-only RESP double to
  `extensions/redis-test-double` (it is an optional add-on's test double, not an engine
  crate).
- **Q2 — Schema Registry scope: DEFERRED.** The whole schema surface (Confluent framing,
  serdes, registry auth, CSFLE) gets its own follow-up spec after K5; the K-series ships the
  raw-`byte[]` core with the bolt-on points intact (`schema.enabled` per binding, `subject`
  header on publish).
- **Q3 — twin-kafka: DEFERRED** until a bridge need exists.
- **Q4 — demo: PORT `kafka-demo`** (the Java example) at K4, as the dry-run runbook's
  vehicle.
- **Q5 — unit-test double: `rdkafka::mocking::MockCluster` RATIFIED** ("the simplicity /
  minimalist principle"); `mockforge-kafka` not adopted, with §5's dated evidence kept for a
  future revisit.

## 10. Relation to the blueprint

Serves `vision-mercury` directly: the flow adapter is Layer-2 composability meeting the
dominant enterprise event backbone, config-not-code. It is also the declared unblocking
dependency for two held items (the sync-over-async facade tasks and the crates.io
publication), and the K4 interop leg extends the polyglot discipline — one wire, two
engines — from the Redis rendezvous to Kafka records.
