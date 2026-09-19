# mercury-distributed-cache

An opt-in **distributed cache** for Mercury applications — the Rust twin of the Java engine's
`extensions/distributed-cache`: a generic Redis-backed L2 key-value cache exposed as **one composable
action function**, route `v1.cache.redis`, over opaque byte values. The same function is reachable from
all three layers — Platform Core (`po.request`), Event Script (a task with output data mapping) and the
Knowledge Graph (a `graph.task` node) — because all three are just "call a route".

| `action` header | other headers | body | result |
|---|---|---|---|
| `PUT` | `key`, `ttl`? | value bytes | `true` |
| `GET` | `key` | — | value bytes, or null on a miss |
| `DELETE` | `key` | — | count removed |
| `PUT_IF_NOT_PRESENT` | `key`, `ttl`? | value bytes | `true` stored / `false` existed |
| `MGET` | — | list of keys | map key → bytes (misses omitted) |
| `MPUT` | `ttl`? | map key → bytes | `true` |
| `LIST_PUSH` / `LIST_POP` / `LIST_LEN` | `key`, `ttl`? | value bytes (push) | length / oldest value / length |

Every stored key carries a TTL from creation. Enable with `redis.cache.enabled=true`; the connection
uses the plain `redis.*` namespace of the shared `mercury-redis-connection` foundation, one multiplexed
connection, built lazily on first use. `redis.health` is the module's `/health` check. Cache keys are
plain Redis keys, so a Java pod and a Rust pod share one cache with no wire change.

See the engine's *Distributed Cache* guide for configuration, the Layer 1/2/3 usage patterns and the
design rationale (the Java repository's `draft-design-specs/distributed-cache.md`, Q1–Q8).
