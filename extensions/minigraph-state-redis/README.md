# minigraph-state-redis

Redis implementation of the knowledge graph's **suspend/resume state-store contract**
— the Rust twin of the Java `extensions/minigraph-state-redis` module:

| Route | Contract | Redis operation |
|---|---|---|
| `v1.redis.persist.model` | headers `type=put`; body `{cid, node, ttl, model, seen, run}` | `SETEX graph:state:<cid>` (MsgPack bytes, native expiry) |
| `v1.redis.retrieve.model` | headers `type=get`; body `{cid}` → record, or empty map when absent/expired | `GETDEL graph:state:<cid>` (atomic consume) |

Both functions are ordinary composable functions registered by `#[preload]` — **add this
crate as a dependency of a graph application (e.g. the `minigraph-playground` example
app), reference it from `main.rs` so the linker keeps its annotation inventory, and they
register automatically**; point the graph's `suspend`/`resume` nodes at them through the
`task` property:

```text
create node suspend
with type Suspend
with properties
purpose=Persist workflow state to the external state store
skill=graph.suspend
task=v1.redis.persist.model
ttl=2d
```

## Behavior

- **Durability ack**: the persist function replies only after Redis accepts the SETEX —
  the `graph.suspend` skill treats anything but 2xx as a failed suspension.
- **Consume-on-retrieve**: `GETDEL` removes the record atomically, so a duplicate resume
  request (a double click, a retried message) cannot execute the continuation twice.
  Requires **Redis 6.2+**.
- **Expiry is native**: the `ttl` from the suspend node becomes the Redis key TTL — no
  sweeper. An expired record simply reads as absent, which the resume skill treats as a
  fresh transaction.
- **Cross-instance by design**: any application instance sharing the same Redis can resume
  a workflow suspended by another (`graph:state:` is one shared namespace keyed by the
  business correlation ID).
- **Lazy connection**: the app boots normally without Redis; the first persist/retrieve
  connects (and fails loudly if Redis is unreachable). The `redis` crate's
  `ConnectionManager` reconnects automatically, and every round-trip is bounded by
  `redis.timeout.ms`.

## Configuration

The same keys as the sync-over-async family, so an application configures Redis once
(all support `${ENV_VAR:default}` substitution):

```properties
redis.host=127.0.0.1
redis.port=6379
redis.password=${REDIS_PASSWORD:}
redis.ssl=false
redis.database=0
redis.timeout.ms=5000
```

Worker instances default to 50 per function and are ops-tunable via
`worker.instances.v1.redis.persist.model` / `worker.instances.v1.redis.retrieve.model`.

## Custom stores

Any composable function honoring the same put/get contract can replace this crate —
implement your own for PostgreSQL, DynamoDB, MongoDB, etc., and name its route in the
node's `task` property. If the store has no native TTL, implement record expiry yourself.
The engine's unit tests ship the smallest possible reference implementation (a temp-file
store). Tests here drive the real `redis` client over TCP against an in-process RESP2
test double (this environment carries no `redis-server` binary or Docker; the Java suite
uses an embedded redis-server instead) — point the config at a real server and the same
functions run unchanged.
