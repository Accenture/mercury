# Distributed cache — Rust lock-step port spec

**Status:** IMPLEMENTED (D1–D4) 2026-09-19 under the §9 recommendations, pending the maintainer's PR
gate; the open questions in §9 remain his to confirm or redirect. Two engine findings surfaced on the
way and are fixed in the same change (§11). Java shipped this feature for v4.12.9 (`extensions/redis-connection` +
`extensions/distributed-cache` + `examples/distributed-cache-example`); its design spec is the
canonical one — `draft-design-specs/distributed-cache.md` in the Java repository, Q1–Q8 ruled by Eric
2026-09-14 — and this document records only the Rust realization: what maps, what deliberately
differs, and how it is proven without a Redis binary.

**Repo scope:** the Rust engine (`mercury`), lock-step with the Java engine (Java Q8: "the Java module
and the Rust port ship together"). Cache keys are plain Redis keys, so the two caches interoperate on
the same keys with no wire change; the Java worked example doubles as the interop harness once this
lands.

---

## 1. Goal & scope

Bring the three Java deliverables to the Rust engine, in the same dependency shape:

| Java | Rust (this port) | Crate |
|---|---|---|
| `extensions/redis-connection` — `RedisConfig` (prefix-parameterised, `redis.*` fallback), `RedisBackend<V>` (standalone / cluster seam), `RedisBackendFactory` (two-key cluster selection + `INFO` auto-detect), `RedisHealthProbe` (reusable, un-annotated) | `extensions/redis-connection` — `RedisConfig`, `RedisBackend`, `RedisHealthProbe` | `mercury-redis-connection` |
| `extensions/distributed-cache` — `v1.cache.redis` action function, `CacheConfig`, `RedisCacheStore`, `CacheRuntime`, `redis.health` binding | `extensions/distributed-cache` | `mercury-distributed-cache` |
| `examples/distributed-cache-example` — profile CRUD on all three layers over one cache | `examples/distributed-cache-example` | `distributed-cache-example` (publish = false) |
| sync-over-async refactored onto the foundation (`soa.redis.*` namespace with `redis.*` fallback; `soa.redis.health` as a thin binding) | `extensions/sync-over-async` refactored the same way | `mercury-sync-over-async` |

Out of scope, as in Java: a cache-aside framework, a general Redis client, the rendezvous protocol
(sync-over-async), a typed consumer helper (Java Q7, deferred).

## 2. Canonical semantics (from the Java source — normative for the port)

### 2.1 The action function `v1.cache.redis`

`action` header (case-insensitive) selects the operation; key(s) and TTL ride in headers, value(s) in
the body. Values are opaque bytes — the caller owns serialisation.

| `action` | headers | body (input) | result | Redis |
|---|---|---|---|---|
| `PUT` | `key`, `ttl`? | value bytes | `true` | `SETEX key ttl value` |
| `GET` | `key` | — | value bytes, or null on a miss | `GET` |
| `DELETE` | `key` | — | count removed (integer) | `DEL` |
| `PUT_IF_NOT_PRESENT` | `key`, `ttl`? | value bytes | `true` stored / `false` existed | `SET key value NX EX ttl` (atomic) |
| `MGET` | — | list of keys | map key → bytes, **misses omitted**, request order | `MGET` (cluster: per-slot scatter-gather) |
| `MPUT` | `ttl`? | map key → bytes | `true` | pipelined per-entry `SETEX` (one round trip; non-atomic across the map; never `MSET`, which sets no TTL) |
| `LIST_PUSH` | `key`, `ttl`? | value bytes | new length (integer) | `RPUSH` + `EXPIRE` as ONE atomic step |
| `LIST_POP` | `key` | — | oldest value bytes, or null if empty | `LPOP` |
| `LIST_LEN` | `key` | — | length (integer) | `LLEN` |

Rules the port keeps byte-for-byte: `ttl` is a duration string (`30s`/`5m`/`1h`; a bare number is
seconds); a write that omits it uses `redis.cache.default.ttl` (default `1h`); **every stored key
carries a TTL from creation**; a `String` body is accepted as a UTF-8 convenience; the optional
`redis.cache.key.prefix` is prepended to every key and stripped again from `MGET` results; `PING` is
NOT an action (it backs `redis.health`).

Error contract (Java `IllegalArgumentException` → surfaced to the caller as the event's error; here
`AppError(400)` with the same messages): `Missing 'action' - one of PUT, GET, MGET, MPUT, DELETE,
PUT_IF_NOT_PRESENT, LIST_PUSH, LIST_POP, LIST_LEN`; `Unsupported action '<x>' - one of …`;
`Missing 'key'`; `A value (byte[] or String) is required in the body`; `MGET requires a List of keys in
the body`; `MPUT requires a Map of key -> value in the body`. A Redis failure surfaces as the store's
error (500) — cache-aside fallback is the flow's exception handler's concern, never the module's.

### 2.2 Configuration

Plain `redis.*` namespace for the cache (the base namespace of the shared foundation), `soa.redis.*`
for sync-over-async, **each `soa.redis.*` key falling back to the un-prefixed `redis.*` form**:

```properties
redis.cache.enabled=true            # master switch: registers v1.cache.redis + redis.health
redis.cache.instances=20            # worker instances (function concurrency), NOT connections
redis.cache.default.ttl=1h          # default TTL when a write omits 'ttl'
redis.cache.key.prefix=             # optional app namespace prepended to every key
redis.host / redis.port             # 127.0.0.1 / 6379
redis.username / redis.password     # blank = default user / no auth
redis.ssl=false  redis.database=0  redis.timeout.ms=5000
redis.cluster.detect=auto           # auto = probe INFO at start-up; else decide by cluster.mode
redis.cluster.mode=false            # true = cluster client (also the inconclusive-probe fallback)
redis.cluster.nodes=                # host:port,host:port seeds (blank = redis.host:redis.port)
redis.health.timeout=5s  redis.health.startup.grace=30s
```

### 2.3 Health check `redis.health`

The reserved plain-named counterpart of `soa.redis.health` (the Rust sync-over-async doc already says
the name is reserved for exactly this module). Same probe, same semantics: `type=info` → `{service:
redis, href: host:port}`; `type=health` → `{status: "Redis is reachable"}`, or a passing `Waiting for
Redis connection` while the configuration is unusable or the credential is rejected (`NOAUTH` /
`WRONGPASS` / `ERR Client sent AUTH`), or **503** `{text, code}` on a genuine outage; a start-up
placeholder with background warm-up until the first success or the grace deadline; config resolved
lazily on every (re)build, never at construction. Both the cache function and its check register only
when `redis.cache.enabled=true`.

### 2.4 Connection model

One shared, multiplexed connection per process — no pool. Built **lazily on first use** and
re-resolved from live configuration on every failed attempt, so the application starts even if Redis
is down or a vault-published credential has not landed yet; once built it is reused (the client owns
reconnection). Released on shutdown through the platform's lifecycle hook.

### 2.5 The worked example's value format (interop-critical)

The Java example stores a profile as a **plain MsgPack-packed map** (`MsgPack.packMapOrList` — no
`EventEnvelope` wrapper, no `_T`/`_D` type tags) precisely so the Rust port reads and writes the same
bytes. The Rust example packs with `rmpv::encode::write_value` of a `Value::Map` and unpacks with
`rmpv::decode::read_value` — standard MsgPack, string keys, no engine framing. A profile written on
one engine's Layer 1 must read back on the other engine's Layer 2 or 3 unchanged; that is the interop
prize and the reason the example is part of the lock-step.

## 3. Wire-format parity is normative

- **Keys**: `{redis.cache.key.prefix}{key}` — plain Redis keys, identical on both engines.
- **Values**: the bytes the caller supplied, stored as-is (`SETEX`/`SET NX EX`/`RPUSH`).
- **List ops**: FIFO on a Redis List — `RPUSH` to append, `LPOP` to pop, `LLEN` — the same machinery
  the return route already runs on both engines.
- **Route names**: `v1.cache.redis`, `redis.health`, `soa.redis.health`.
- **Configuration keys and defaults**: as §2.2, identical names — a Java and a Rust pod configured
  alike address the same cache.

## 4. Deliberate deltas (map, don't mirror)

| # | Java | Rust | Why |
|---|---|---|---|
| 1 | `LIST_PUSH` = `EVAL` Lua (`RPUSH` + `EXPIRE`, returning the length) | `MULTI`/`EXEC` pipeline of `RPUSH` + `EXPIRE` (`redis::pipe().atomic()`), returning the `RPUSH` reply | Same single atomic server-side step; this is the delta the sync-over-async port already ruled for its `append_segment` (port spec §5), and the in-process RESP double implements `MULTI`/`EXEC` but cannot run Lua. |
| 2 | `RedisBackend<V>` generic in the value codec (`String` vs `byte[]`) | one `RedisBackend` over raw `redis::Value`; the cache reads `Value::BulkString(bytes)` and sync-over-async reads text | The `redis` crate is codec-free; typing lives at the call site. |
| 3 | Lettuce standalone `RedisClient` / `RedisClusterClient` behind one command type | an enum over `redis::aio::ConnectionManager` (standalone) and `redis::cluster_async::ClusterConnection` (cluster, feature `cluster-async`); both implement `aio::ConnectionLike`, so every command is issued through one `query` seam | Same seam, same two-key selection, same `INFO` auto-detect. `MGET` is routed per slot by the cluster client (redis-rs `routing.rs` treats `MGET`/`DEL` as multi-slot). |
| 4 | Cluster Pub/Sub connection from the cluster client | Pub/Sub on a standalone client to the first seed node (classic Pub/Sub crosses the cluster bus) | redis-rs cluster Pub/Sub is not a drop-in; only sync-over-async uses Pub/Sub, and the cache never does. Documented, not silent. |
| 5 | `@PreLoad(instances=20, envInstances="redis.cache.instances")` + `@OptionalService("redis.cache.enabled")` | `#[preload(route = "v1.cache.redis", instances = 20, env_instances = "redis.cache.instances")]` + `#[optional_service("redis.cache.enabled")]` | Direct macro equivalents; the inventory registers the function when the application links the crate (the minigraph-state-redis deployment story). |
| 6 | `Platform.onShutdown(Runnable)` closes the cache backend | `Platform::on_shutdown(...)` — **new** platform-core lifecycle API (§9 Q2); hooks run in reverse registration order from `AutoStart::run` after Ctrl-C, before the elastic-queue cleanup | Java added the hook in the same release (v4.12.9); the Java fact says "Rust parity is a lockstep follow-up (internal lifecycle API, not a wire contract)". |
| 7 | `ReentrantLock` (Java-21 carrier pinning) | `tokio::sync::Mutex` around the lazy build; `ArcSwap`-free `RwLock<Option<Arc<_>>>` for the fast path | No pinning concern on tokio; the shape (lock the slow build, lock-free read) is the same. |
| 8 | health `IllegalArgument`/`IllegalState` = "unbuildable config" | an `AppError` from building the client (bad address / unresolved placeholder) classifies as waiting, exactly as the sync-over-async port already does | Same boundary, already ported once. |

## 5. Crate layout & component map

```
extensions/redis-connection/            mercury-redis-connection (new)
  src/lib.rs        RedisConfig (prefix-parameterised loader, soa.redis.* -> redis.* fallback,
                    username, cluster keys, duration helper), RedisBackend (+ connect/detect),
                    RedisHealthProbe (reusable; a module binds route + config prefix)
extensions/distributed-cache/           mercury-distributed-cache (new)
  src/lib.rs        CacheAction, CacheConfig, RedisCacheStore, CacheRuntime,
                    RedisCache (#[preload] v1.cache.redis), CacheRedisHealthCheck (#[preload] redis.health)
examples/distributed-cache-example/     distributed-cache-example (new, publish = false)
  src/main.rs       v1.profile.l1, v1.profile.encode/decode, v1.http.method.action, v1.profile.exception
  resources/        application.yml, rest.yaml, flows.yaml + flows/l2-profile.yml (byte-identical to Java),
                    graphs.yaml + graph/profile-cache.json (byte-identical), graph-executor.yml
  tests/            profile_cache.rs — L1/L2/L3 CRUD, the L3 reject, cross-layer interop (RESP double)
extensions/sync-over-async/             refactor: RedisSettings -> foundation RedisConfig (alias kept),
                                        soa.redis.* namespace with redis.* fallback, health = thin binding
extensions/redis-test-double/           + MGET, SET with NX/EX/PX options (the cache's two missing commands)
crates/platform-core/                   + Platform::on_shutdown (Q2)
```

## 6. Test strategy — in-process only (R-NoDocker, as every Redis suite here)

The RESP double already serves `SET`, `SETEX`, `GET`, `GETDEL`, `DEL`, `EXPIRE`, `TTL`, `EXISTS`,
`RPUSH`, `LPUSH`, `LPOP`, `LLEN`, `MULTI`/`EXEC`, `PING`, `INFO`, `AUTH`, Pub/Sub. It gains:
`MGET` (bulk array with nulls for misses) and `SET` option parsing (`NX`/`XX`, `EX`/`PX`) so
`PUT_IF_NOT_PRESENT` runs as the real single command. `INFO cluster` answers the server block, so
auto-detect resolves to standalone, as it would against a real single node.

Suites mirror the Java test methods one to one (names in the Rust idiom):

- `redis-connection`: `RedisConfigTest` twins (defaults, discrete properties, two-key cluster selection,
  RBAC username, `soa.*` → `redis.*` fallback, `soa.*` wins when both present, base prefix reads plain
  keys, explicit prefix isolation, seeds from `cluster.nodes` and from host:port), `RedisBackendFactory`
  twins (auto-detect resolves standalone and round-trips; explicit standalone skips detection;
  pipelining over the one connection; opaque bytes round-trip; explicit cluster mode routes to the
  cluster branch), the health probe suites (moved from sync-over-async and generalised).
- `distributed-cache`: `CacheActionTest` (case-insensitive; missing/unsupported name the set),
  `CacheConfigTest` (defaults; tunables + plain namespace; the `soa.*` namespace is ignored),
  `RedisCacheStoreTest` (every op, TTL-from-birth via the double's `TTL`, prefix namespacing stripped
  from `MGET`, FIFO list semantics, empty `MGET`/`MPUT`), `RedisCacheTest` (the function contract:
  `true`/bytes/null/integer/boolean/map results, String body as UTF-8, the four rejections),
  `CacheRedisHealthCheckTest` (info href from the plain namespace; grace placeholder).
- `sync-over-async`: its existing suites unchanged in intent — the refactor is behaviour-preserving,
  which those suites prove.
- example: `ProfileCacheTest` twins — one booted server, sequential scenarios (repo convention).

What the double cannot prove: cluster routing and a live `MOVED`/`ASK` storm. Same stance as the Java
suite, which also pins only the branch selection; live-cluster verification is a certification run
against a real cluster.

## 7. Increment plan

- **D1 — foundation.** `mercury-redis-connection` (config, backend seam with standalone + cluster
  branches, health probe, duration helper) + sync-over-async refactored onto it (`soa.redis.*` with
  fallback; `RedisSettings` kept as an alias; `soa.redis.health` reads `soa.redis.health.timeout` with
  the `redis.health.timeout` fallback) + the double's `MGET` and `SET` options + `Platform::on_shutdown`.
- **D2 — the cache module.** `mercury-distributed-cache`: action set, config, store, lazy runtime with
  the shutdown hook, the two `#[preload]` functions, the five suites.
- **D3 — the worked example.** `examples/distributed-cache-example` with the Java flow and graph files
  byte-identical, the four Rust functions, dev-mode wiring as the Java example, `profile_cache.rs`.
- **D4 — documentation and contract.** `docs/guides/distributed-cache.md` (adapted from the Java guide),
  `configuration-reference.md` (`redis.cache.*`, the `soa.redis.*` namespace, `redis.*` as the
  foundation's base), `reserved-names-and-headers.md` (`v1.cache.redis`, `redis.health`), `llms.txt`,
  mkdocs nav, the AI-contract `files.list`, `docs/INCREMENTS.md`.

Each increment is its own `cargo fmt` + `clippy -D warnings` + `cargo test --workspace` gate; the PR
carries all four (Eric may ask to split).

## 8. Failure analysis (port-specific)

- **Redis down at start-up** — the function registers, the runtime holds no connection, the first call
  fails fast (500) and retries the build next call; `redis.health` reports waiting/503 per §2.3.
- **A late credential** — `CacheRuntime` re-reads `RedisConfig` on every failed build; the probe
  re-resolves on every rebuild. No restart needed on either path.
- **Shutdown** — the hook closes the shared connection once; a process that never touched the cache
  registered no hook.
- **Pub/Sub on a cluster** — delta 4; sync-over-async only.

## 9. Open questions for the maintainer (each with the recommendation the work proceeds under)

- **Q1 — Cluster support now or deferred?** *Recommendation: now, as the seam plus the `cluster-async`
  branch, tested at the selection level.* The Java foundation made cluster a first-class property of
  the config surface (`cluster.detect` / `cluster.mode` / `cluster.nodes`), a mixed fleet must accept
  the same keys, and the redis crate's cluster client routes the cache's only multi-key op (`MGET`) per
  slot. Live-cluster behaviour is a certification run, exactly as it was for Java. The alternative —
  accept the keys and fail `cluster.mode=true` with "not yet supported" — is a declared delta that would
  split the config contract.
- **Q2 — `Platform::on_shutdown` now or deferred?** *Recommendation: now.* It is small (a registry of
  boxed hooks run in reverse order from `AutoStart::run`), it is what Java shipped in the same release,
  and without it the cache's connection has no orderly close. Internal lifecycle API, not a wire
  contract.
- **Q3 — crates.io publication.** *Assumption:* the new crates join the existing hold — sync-over-async's
  publication waits for minimalist-kafka K5 (Eric, 2026-09-14) — so nothing publishes from this PR; the
  release that catches the Rust engine up decides.
- **Q4 — the example's port.** *Assumption:* `8305`, the Java example's port, so the two examples are
  interchangeable in a side-by-side interop run.
- **Q5 — `RedisSettings` name in sync-over-async.** *Assumption:* kept as a type alias of the foundation's
  `RedisConfig` so the demo and tests compile unchanged; the alias is documented as compatibility, and
  the foundation's name is the one new code uses.

## 11. Findings on the way (fixed in the same change)

- **The REST layer rendered a function's failure as plain text.** The example's Layer 1 miss —
  `Err(AppError::new(404, "Profile not found"))` — reached the HTTP client as `text/plain`, while the
  Java engine renders that shape as the standard error body `{status, message, type: error}`
  (`AsyncHttpResponse.handleException`: error status, no headers, a string body that does not look
  like JSON or XML). The Rust server had the Java shape only for its own routing errors. Mirrored in
  `automation/server.rs` with the same guard, pinned by `function_failure_is_java_shaped_error_body`.
  A REST client now sees one error shape whether the failure came from routing or from the service.
- **The example is the interop harness the Java repo asked for**: the Java flow and graph files run
  unchanged, and the stored value is a plain MsgPack map under the `cache-demo:` namespace — the same
  bytes the Java example writes — so the two examples pointed at one Redis read each other's profiles.

## 10. Relation to the blueprint

An infrastructure module, not a Blueprint gap item (the Java spec §9 says the same). It serves
`vision-mercury` indirectly — lean, decoupled, composable modules — and it closes the last Rust
lock-step item of the Java v4.12.9 release, so the Rust engine's catch-up release can adopt the Java
number with the distributed cache included.
