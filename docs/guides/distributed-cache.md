# Distributed Cache

*Guide: a generic Redis-backed L2 distributed cache, exposed as one composable function you call from
any layer.*

> **At a glance**
>
> - **What** — a shared key-value cache backed by Redis, exposed as a single composable action function at
>   route **`v1.cache.redis`**. One `action` header selects the operation; values are opaque bytes.
> - **How** — `PUT`/`GET`/`MGET`/`MPUT`/`DELETE`/`PUT_IF_NOT_PRESENT` plus FIFO `LIST_PUSH`/`LIST_POP`/
>   `LIST_LEN`. Every stored key carries a TTL from creation. One shared, multiplexed connection — no pool.
> - **Advanced & opt-in** — off by default; enable with `redis.cache.enabled=true`. Standalone or
>   Redis Cluster with no code change.
> - **For** developers who need a cache shared across pods, across the three layers (Platform Core,
>   Event Script, Knowledge Graph) — and across engines: a Java pod and a Rust pod share one cache.

Everything on this page describes this repository's engine (`extensions/distributed-cache`, crate
`mercury-distributed-cache`). It is the lock-step twin of the Java engine's module: the same route, the
same action names, the same configuration keys, the same key layout in Redis — so the two engines'
caches interoperate on the same keys with no wire change.

The cache is a thin composable module over the shared **`mercury-redis-connection`** foundation
(standalone/cluster selection, auth, TLS, health probe) that also powers the streaming return route
(`mercury-sync-over-async`). It adds only the cache *operations* and the composability surfaces; it is
**not** a rendezvous transport (that is sync-over-async) and **not** a cache-aside framework — your flow
orchestrates read-through / write-through, the module just stores and returns bytes.

> **Opt-in.** The cache function and its health check register only when `redis.cache.enabled=true`. The
> connection is built **lazily on first use**, so enabling it does not fail application start-up when Redis
> is briefly unreachable — a cache call then fails fast and the flow's exception handler decides the
> fallback.

## Operations {#operations}

The `action` header (case-insensitive) selects the operation. The key(s) and TTL ride in headers; the
value(s) ride in the body:

| `action` | Other headers | Body (input) | Result | Redis |
|----------|---------------|--------------|--------|-------|
| `PUT` | `key`, `ttl`? | value (bytes) | `true` | `SETEX` |
| `GET` | `key` | — | value (bytes), or `null` on a miss | `GET` |
| `DELETE` | `key` | — | count removed (integer) | `DEL` |
| `PUT_IF_NOT_PRESENT` | `key`, `ttl`? | value (bytes) | `boolean` — `true` if stored, `false` if the key existed | `SET NX EX` |
| `MGET` | — | keys (list of strings) | map key → bytes (misses omitted) | `MGET` |
| `MPUT` | `ttl`? | entries (map key → bytes) | `true` | pipelined per-entry `SETEX` |
| `LIST_PUSH` | `key`, `ttl`? | value (bytes) | new list length (integer) | `RPUSH` + `EXPIRE` (atomic) |
| `LIST_POP` | `key` | — | oldest value (bytes), or `null` if empty | `LPOP` |
| `LIST_LEN` | `key` | — | list length (integer) | `LLEN` |

Notes for callers:

- **`ttl`** is a duration string (`30s`, `5m`, `1h`, or bare seconds). When a write omits it, the cache
  uses `redis.cache.default.ttl` (default `1h`). **Every stored key carries a TTL from creation** — there
  are no un-expiring keys; `PUT_IF_NOT_PRESENT` is atomic (`SET … NX EX`, never `SETNX` then `EXPIRE`),
  and `LIST_PUSH` appends and sets the expiry as **one atomic `MULTI`/`EXEC` step** (the Java engine uses a
  Lua `EVAL` for the same step — same guarantee, this engine's ruled equivalent).
- **`MPUT`** is a pipelined batch of single-key `SETEX` — one round trip, and each key keeps its own TTL
  (a raw `MSET` sets none). It is **not atomic** across the map (a partial failure is just cache misses),
  which is correct for a cache and unavoidable on a cluster.
- **`MGET`** keys may span cluster hash slots — the cluster client scatter-gathers them for you.
- An unknown `action`, a missing `key`, a missing value, or an unparseable `ttl` is rejected with the
  event's error (status 400 and a message naming the problem, e.g. `Unsupported action 'INCR' - one of
  PUT, GET, MGET, …`).
- **Redis failures are classified**, so a caller — or the flow's / graph's exception handler, which passes
  the status through — sees the failure for what it is: a command timeout replies **408** (`Redis request
  timed out after N ms`), a refused, dropped or unreachable connection replies **503** `Redis unavailable - …`
  (the `redis.health` vocabulary), and anything the server answered (a wrong type, an unknown command) keeps
  the default 500. The Java engine classifies the same way, so a mixed fleet fails alike.

## Enabling and configuring {#config}

Add the crate to the application and enable it. Linking the crate registers the two functions through the
preload inventory (reference the crate from `main.rs` so the linker keeps the inventory — the "include
the jar" deployment story). The connection uses the plain **`redis.*`** namespace (the base namespace of
the shared foundation), so it never collides with sync-over-async's `soa.redis.*`:

```toml
[dependencies]
mercury-distributed-cache = "4.12"
```

```properties
redis.cache.enabled=true              # master switch: registers v1.cache.redis + redis.health
redis.cache.default.ttl=1h            # default TTL when a write omits 'ttl'
redis.cache.key.prefix=app1:          # optional: prepended to every key (namespace apps sharing one Redis)

redis.host=${REDIS_HOST:127.0.0.1}
redis.port=${REDIS_PORT:6379}
redis.username=${REDIS_USERNAME:}     # blank = default user; set for an ACL/RBAC user
redis.password=${REDIS_PASSWORD:}     # blank = no auth; keep secrets in the environment
redis.cluster.detect=auto             # auto = detect at start-up; else use the boolean below
redis.cluster.mode=false              # true = cluster, false = standalone (when detect is not auto)
```

The essentials — see the [Configuration Reference](configuration-reference.md#redis-cache) for the full
list:

| Key | Default | Description |
|-----|---------|-------------|
| `redis.cache.enabled` | `false` | Master switch; `true` registers `v1.cache.redis` and `redis.health`. |
| `redis.cache.instances` | `20` | **Worker instances** of the function (concurrency), **not** a connection count — every instance shares the one multiplexed connection. |
| `redis.cache.default.ttl` | `1h` | TTL applied when a `PUT` / `MPUT` / `LIST_PUSH` omits `ttl`. |
| `redis.cache.key.prefix` | — (blank) | Prepended to every key; stripped again from `MGET` results. Isolate apps sharing one Redis. |
| `redis.host` / `redis.port` | `127.0.0.1` / `6379` | Redis connection (or the cluster configuration endpoint). |
| `redis.username` / `redis.password` | — (blank) | ACL/RBAC username / auth password. Source from the environment. |
| `redis.ssl` | `false` | Use TLS. |
| `redis.cluster.detect` / `redis.cluster.mode` / `redis.cluster.nodes` | `auto` / `false` / — | Standalone-or-cluster selection — the same two-key scheme as the Java engine. |
| `redis.timeout.ms` | `5000` | Default command timeout. |
| `redis.heartbeat.ms` | `1000` | Connection heartbeat (Rust engine only; `0` = off) — see *Redis restarts* below. |
| `redis.health.timeout` | `5s` | Timeout for the [`redis.health`](#health) probe. |
| `redis.health.startup.grace` | `30s` | Start-up grace for [`redis.health`](#health). |

### Separate Redis clients, by design {#separation}

**When an application runs both the cache and sync-over-async, give each its own Redis client — configure
the cache under `redis.*` and sync-over-async under `soa.redis.*`, fully.** That is the intended shape,
not merely a supported one. Every endpoint-defining key — `host`, `port`, `username`, `password`, `ssl`,
`database`, `timeout.ms`, and all three `cluster.*` keys — resolves per namespace, so the two can differ
in server, credentials, TLS, and even topology.

- **A cache evicts; a rendezvous must not.** A cache under memory pressure with an eviction policy will
  evict whatever fits its policy — including a `request:{cid}` rendezvous key, **mid-request**. Separate
  instances make that failure mode structurally impossible.
- **Independent operations.** Restarting, resizing, or failing over the cache should not disturb in-flight
  synchronous requests.

> **What the `redis.*` fallback is for.** Each `soa.redis.*` key falls back to the un-prefixed `redis.*`
> form when the prefixed one is absent. That is **backward compatibility** — sync-over-async predates the
> cache and was configured under plain `redis.*`, so those deployments keep working untouched. It is not
> an invitation to share one instance between the two modules; and because the fallback is *per key*, a
> partial `soa.redis.*` override silently mixes the two — when you decouple, set the whole connection set.

> **Both probes, both endpoints.** Two clients mean two health checks:
> `mandatory.health.dependencies=redis.health, soa.redis.health`.

## Using the cache {#usage}

The same function is reachable from all three layers — it is just "call a route".

### Layer 1 — PostOffice {#layer1}

Values are opaque bytes — send a MsgPack **binary** body (`rmpv::Value::Binary`), never a
`set_body(Vec<u8>)`, which serde would encode as a list of integers:

```rust
use platform_core::{EventEnvelope, Platform, PostOffice};
use rmpv::Value;

let po = PostOffice::new(&Platform::get_instance());   // the trace bracket is task-local: the call joins this request's trace

// PUT: the value is the body; ttl is optional (defaults to redis.cache.default.ttl)
let mut payload = Vec::new();
rmpv::encode::write_value(&mut payload, &profile)?;   // any bytes you like - here a plain MsgPack map
po.request(
    EventEnvelope::new().set_to("v1.cache.redis")
        .set_header("action", "PUT").set_header("key", "profile:42").set_header("ttl", "10m")
        .set_raw_body(Value::Binary(payload)),
    Duration::from_secs(5),
).await?;

// GET: a miss returns a null body
let reply = po.request(
    EventEnvelope::new().set_to("v1.cache.redis")
        .set_header("action", "GET").set_header("key", "profile:42"),
    Duration::from_secs(5),
).await?;
let cached: Option<&[u8]> = match reply.body() {
    Value::Binary(bytes) => Some(bytes),   // hit
    _ => None,                             // miss
};
```

### Layer 2 — Event Script task {#layer2}

Drive it from a flow with input/output data mapping — a constant sets the `action`, `model.*` supplies the
key, and the bytes value rides the whole-body `*` passthrough. The flow YAML is identical on both engines:

```yaml
tasks:
  # write-through: cache the serialized profile under a 10-minute TTL
  - input:
      - 'text(PUT) -> header.action'
      - 'model.cacheKey -> header.key'
      - 'text(10m) -> header.ttl'
      - 'model.profileBytes -> *'          # the bytes value rides in the body
    process: 'v1.cache.redis'
    output:
      - 'result -> model.stored'           # the PUT ack (true)
    description: 'Cache the profile'
    execution: sequential
    next:
      - 'read.back'

  # read: GET returns the value, or null on a miss
  - input:
      - 'text(GET) -> header.action'
      - 'model.cacheKey -> header.key'
    process: 'v1.cache.redis'
    output:
      - 'result -> model.cached'           # bytes value, or null on a miss (branch on it with a decision task)
    description: 'Read the cached profile'
    execution: sequential
```

### Layer 3 — Knowledge Graph node {#layer3}

A `graph.task` node calls the same route with the same mapping syntax:

```json
{
  "skill": "graph.task",
  "task": "v1.cache.redis",
  "input": [
    "text(GET) -> header.action",
    "model.cacheKey -> header.key"
  ],
  "output": [
    "result -> model.cached"
  ]
}
```

The worked example `examples/distributed-cache-example` runs the same profile CRUD on all three layers
over one cache; its flow and graph files are byte-identical to the Java example's.

## Value type & serialisation {#values}

Values are **opaque bytes** — the cache stores and returns exactly the bytes you supplied, and the caller
owns serialisation. This maximises interop: any layer, and either engine, reads and writes the same keys. A
string body is accepted as a UTF-8 convenience, but the canonical value type is bytes. For a value both
engines must read, pack **plain MsgPack** (`rmpv::encode::write_value` of a map; the Java
`MsgPack.packMapOrList`) — never an engine's envelope encoding, which is a per-engine wire format. A typed
convenience helper is deliberately **not** provided — the function plus the flow/graph surfaces already
cover all three layers.

## Standalone or cluster Redis {#cluster}

The cache runs against a **single-node** Redis or a **Redis Cluster** with no code change — the
foundation's two-key selection (`redis.cluster.detect` / `redis.cluster.mode` / `redis.cluster.nodes`)
picks the standalone manager or the cluster-aware connection (the `redis` crate's `cluster-async`
client), and authentication works identically. Every operation is **cluster-safe by construction**:
`PUT`/`GET`/`DELETE`/`PUT_IF_NOT_PRESENT` and the list ops are single-key; `MGET` keys may span slots and
are routed per slot by the cluster client; `MPUT` is a pipeline of independent single-key `SETEX`. The
module uses **one shared, multiplexed** connection — **no connection pool**: the client pipelines any
number of concurrent callers over one in-order connection, and this op set has no blocking commands
(`LPOP`, not `BLPOP`). `redis.cache.instances` is worker concurrency, not a connection count.

## Redis restarts {#restarts}

Lettuce (the Java client) requeues commands it has not yet written across a reconnect, so on the
Java engine the first command after a Redis restart simply works. The `redis` crate arms a reconnect
when a command fails but returns that command's error, so on this engine the first command after a
restart used to fail with `503 Redis unavailable - broken pipe` and the second heal. The shared
foundation now carries the maintainer's ruling — retry only when the failure is a restart or a
reconnection, which takes some simple lifecycle monitoring:

- **A heartbeat** (`redis.heartbeat.ms`, default one second) notices a lost connection within one
  interval and makes the client reconnect ahead of the next command, so a command that arrives after
  that finds a fresh connection — including the non-idempotent ones. The loss and the recovery are
  each logged once.
- **One retry per lost connection, idempotent commands only.** `GET`, `MGET`, `PUT`/`MPUT` (`SETEX`),
  `DELETE` and `LLEN` that meet the lost connection themselves are retried exactly once on the fresh
  connection. `PUT_IF_NOT_PRESENT` (`SET NX`), list push (`RPUSH`) and pop (`LPOP`) are never
  replayed — the client cannot prove whether a lost command reached the server — so they fail the
  caller for what it is (503) and the caller's own retry lands on the healed connection.
- **A known outage never doubles the deadline.** Once the connection is known down, a command makes one
  attempt, bounded by `redis.timeout.ms` — 408 while the client is still trying to reconnect, 503 when
  the connection is refused outright — and is not retried. A timeout is never treated as a lost
  connection.

## Health check {#health}

The module ships a health-check function at route **`redis.health`** (registered with the cache when
`redis.cache.enabled=true`). Opt in as a health dependency:

```properties
mandatory.health.dependencies=redis.health
# or, to report Redis without failing /health:
# optional.health.dependencies=redis.health
```

The probe is a single Redis **PING** on a dedicated connection built from the `redis.*` parameters — one
round trip proves connectivity, TLS, and authentication. Its semantics match `soa.redis.health` exactly
(both are bindings of the foundation's one probe): config is resolved **lazily** (so a vault-published
credential that lands after start-up is picked up), an unusable configuration or a rejected credential
(`NOAUTH` / `WRONGPASS`) reports a **passing** `Waiting for Redis connection` status rather than failing
`/health`, and only a genuine connectivity failure returns **503**. `redis.health.timeout` (default `5s`)
bounds the probe; `redis.health.startup.grace` (default `30s`) is the start-up placeholder window.

> `redis.health` is the plain-named counterpart to sync-over-async's `soa.redis.health`; the two coexist,
> each reporting on its own `redis.*` / `soa.redis.*` server.

## When to use it {#when}

Reach for the distributed cache when you need a **shared L2 key-value cache** — cross-pod, cross-instance,
cross-engine, and reachable from any of the three layers — with TTL'd entries and a small, cache-shaped
operation set. It is opt-in: if a per-instance in-memory cache suffices (`ManagedCache`), use that instead.
It is **not**:

- a **rendezvous / streaming transport** — that is sync-over-async;
- a **cache-aside framework** — there is no automatic DB read-through/write-through or invalidation; your
  flow orchestrates that (a cache-miss branch calling the source of truth);
- a **general Redis client** — the operation set is bounded and cache-shaped, not arbitrary `EVAL` /
  pub-sub / streams.

The design rationale and the ruled decisions (Q1–Q8) live in the Java repository's
[distributed-cache design spec](https://github.com/Accenture/mercury-composable/blob/main/draft-design-specs/distributed-cache.md);
this engine's realization — what maps and what deliberately differs — is
[`draft-design-specs/distributed-cache-port.md`](https://github.com/Accenture/mercury/blob/main/draft-design-specs/distributed-cache-port.md).

## See also

- [Configuration Reference](configuration-reference.md#redis-cache) — every `redis.cache.*` / `redis.*` / `soa.redis.*` key.
- [Event Script syntax](event-script/syntax.md) — the input/output data-mapping syntax the Layer 2 / Layer 3 examples use.
- [Observability](observability.md) — tracing a cache call end-to-end.
