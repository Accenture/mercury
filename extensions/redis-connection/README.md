# mercury-redis-connection

The shared Redis client foundation of Mercury's Redis-backed extensions — the Rust twin of the Java
engine's `extensions/redis-connection`. It carries no feature of its own; `mercury-sync-over-async`
(the streaming return route) and `mercury-distributed-cache` (the L2 cache) depend on it so that one
Redis client layer serves both, in the same dependency direction as the Java engine.

| Type | Role |
|---|---|
| `RedisConfig` | the discrete connection parameters, read from a **configurable key prefix** — `soa.redis.*` for sync-over-async, plain `redis.*` for the cache — each key falling back to the un-prefixed `redis.*` form |
| `RedisBackend` | the **standalone-or-cluster** seam: two-key selection (`cluster.detect` / `cluster.mode`) with an `INFO` auto-detect, one long-lived multiplexed connection, every command through one `query` |
| `RedisHealthProbe` | the reusable `/health` PING probe (lazy config, waiting-vs-outage boundary, start-up grace); a module binds it to its own route |

Configuration keys and defaults are identical to the Java engine's, so a Java and a Rust pod
configured alike reach the same Redis. See the engine's configuration reference for the full list.
