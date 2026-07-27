# Design — `ManagedCache` → Rust (self-expiring in-memory cache)

> **Status:** DRAFT v2 for maintainer review (v1 hardened by a 3-lens adversarial critique
> against the Java source, the moka docs/changelog, and the Rust consumer sites) ·
> **Realizes:** `ot-managedcache-port` (backlog) · **Serves:** `vision-mercury` ·
> **Author:** Claude Code · **Date:** 2026-07-27
> **Canonical source:** `org.platformlambda.core.util.ManagedCache` (mercury-composable, Java).
> This is a *Design-altitude* artifact in the VBDI loop. **Implementation waits on approval.**
>
> **Maintainer rulings already applied (2026-07-27):**
> 1. **Do NOT port `SimpleCache`.** One cache type only.
> 2. **"Adopt a proper self-expiring in-memory cache"** — use a maintained implementation
>    rather than hand-rolling expiry/eviction.

## 1. Goal & scope

Give the Rust port the named, self-managing, **self-expiring and size-bounded** in-memory
cache that Java applications get from `ManagedCache` — for engine internals that need it
today, for Java-API-surface completeness, and as the prerequisite for the future
connectors port (`bp-kafka-connectors-backlog`: minimalist-kafka's schema-registry client
is a heavy `ManagedCache` user).

**In scope:** one `ManagedCache` type + its by-name registry + lifecycle housekeeping +
the adopters in §6. **Out of scope:** `SimpleCache` (ruling #1); the fetcher provider
cache (per-graph-instance state in BOTH engines — not a `ManagedCache` use);
`system.log.cache` and `payload.segmentation` (their owning features — `logRecently`
dedup and mesh websocket segmentation — do not exist in the Rust port); the remaining
Java call sites are all connectors/mesh (`app.presence.list`, `sticky.destinations`,
`service.load.balancer`, `member.life.cycle.events`) — out of scope with their layer.

## 2. Canonical semantics (what must hold, from the Java source)

- **Named caches in a process-wide registry**: `createCache(name, expiryMs[, maxItems])`
  is idempotent by name — a second call returns the existing instance unchanged; the
  **first** creation's parameters win, later parameters are ignored
  (ManagedCache.java:92–97). `getInstance(name)` resolves from any module;
  `getCacheCollection()` enumerates for ops introspection.
- **Self-expiring**: entries expire `expiryMs` after their last **write**
  (Caffeine `expireAfterWrite`; a `put` to an existing key resets its clock). Minimum
  expiry **1000 ms** (clamped up via `Math.max`, never rejected).
- **Size-bounded**: at most `maxItems` entries (default **2000**); the store evicts to
  stay within bound. `size()` is an **estimate** (Java documents `estimatedSize`).
- **Untyped values, reference semantics**: Java stores `Object` — callers share the same
  instance through the cache, no copying. (Java does NOT guard null *values* — Caffeine
  throws; the Rust carrier makes that case unrepresentable.)
- **Guarded no-ops**: `put`/`get`/`remove` with a null/empty key do nothing; `exists()`
  guards transitively by delegating to `get`.
- **Telemetry stamps** (epoch ms), mapped precisely — silent micro-divergence here is
  easy, so the map is normative:
  - `put` → `last_write`; `remove` → `last_write` (stamped even when the key is absent);
  - `get`/`exists` → `last_read` (stamped **before** the lookup — hit or miss);
  - `clear` → `last_reset`;
  - constructor seeds `last_reset = now`, `last_read = last_write = 0`.
- **`clear()` is three operations**: stamp `lastReset` + `invalidateAll()` + `cleanUp()`
  (ManagedCache.java:163–167) — the cleanup keeps `size()` honest immediately after a
  clear. `clear()` itself emits no log; only `cleanUp()` logs (debug).
- **Housekeeping**: a single process-wide 10-minute sweep calls `cleanUp()` on every
  registered cache — it reclaims memory in caches that see **no subsequent activity at
  all** (any read *or* write elsewhere in a cache triggers its amortized maintenance);
  **correctness never depends on it** — expiry is enforced by the store itself. The Java
  sweep is non-overlapping (a `NOT_RUNNING` compare-and-set); the Rust interval task is
  serialized by construction.
- Logs (exact wordings): `Created cache ({name}), expiry {elapsed}, maxItems={n}` at
  create; `Housekeeper started` once; `Cleaning up {name}` at debug on sweep.

## 3. Decisions (proposed — the gate)

| # | Decision | Rationale |
|---|---|---|
| MC1 | **Engine: `moka = { version = "0.12", features = ["sync"] }`** (`moka::sync::Cache`), wrapped — never exposed | Ruling #2 ("adopt a proper self-expiring cache") + the repo's canonical-analog precedent (chrono-tz = `ZoneId`): moka is the Caffeine-lineage Rust cache — `max_capacity` = `maximumSize`, `time_to_live` = `expireAfterWrite` (reset-on-update proven by moka's own TTL unit test), `entry_count()` = `estimatedSize`, `run_pending_tasks()` = `cleanUp()`. Since v0.12 it spawns **no threads of its own** (README/CHANGELOG v0.12.0: background threads removed; re-verify against the pinned version's changelog at implementation) — and unlike Caffeine, which offloads cleanup to the JVM common pool by default, maintenance runs strictly on calling threads, which *strengthens* the no-runtime-entanglement argument. Health check (2026-07): v0.12.15 released 2026-07-04, steady patch cadence, no RUSTSEC advisories, MSRV 1.71.1 vs our 1.95.0; the old `quanta` dependency concern is stale (optional/non-default since v0.12.10) — the sync tree is modest and mainstream (crossbeam-\*, parking_lot, smallvec, …; uuid already in our tree). Honest caveat: effectively one maintainer (bus factor 1) — the wrapper keeps the engine an internal detail, so it can be swapped (incl. hand-rolled) without touching any consumer. *(Alternative considered: ~150-line hand-rolled TTL+bound map — zero new deps; rejected under ruling #2 and for the future schema-registry hot path, where moka's concurrent design wins. Revisit only via the engine-swap hatch.)* |
| MC2 | **Value carrier: `pub type CacheValue = Arc<dyn Any + Send + Sync>`** with a typed `get_as::<T>()` helper (`Arc::downcast`, stable std) | The faithful Rust carrier of Java `Object`: reference semantics (`Arc` clone = Java reference handout — moka's `get` returns a clone of V, i.e. a refcount bump), zero serialization tax, holds arbitrary structs; compile-verified against moka's `V: Clone + Send + Sync + 'static` bounds on rustc 1.95.0. This is load-bearing for the schema-registry consumer, which caches **parsed-schema handles** precisely to avoid re-parsing — `rmpv::Value`/`serde_json::Value` would force a serde round-trip per hit and exclude non-Serialize handles. Convention (documented): one named cache stores one value shape; `get_as` returns `None` on a type mismatch, exactly where Java's cast would sit. **Double-wrap trap (doc'd + tested):** `put(key, some_arc)` compiles — it stores TypeId `Arc<T>`, so `get_as::<T>()` misses; the `put` doc warns "never pass a pre-wrapped `Arc`/`CacheValue` to `put` — use `put_arc`", and §7 pins a wrong-wrap regression. Same move as the registration-metadata contract: canonical semantics fixed, carrier idiomatic per language. |
| MC3 | **One type** (`ManagedCache`), no `SimpleCache` | Ruling #1. Bounded + self-expiring is a strict superset of SimpleCache's unbounded lazy expiry; any Java `SimpleCache` call site ported later becomes a `ManagedCache` instance (default bound), with a one-line parity note at that site. The mapping rule is stated once, here. |
| MC4 | **Registry: monomorphic statics** — `get_instance(name) -> Option<Arc<ManagedCache>>`; `get_cache_collection() -> BTreeMap<String, Arc<ManagedCache>>` (sorted **snapshot**) | Because `ManagedCache` is one non-generic type (MC2 erases values, not caches), cross-module `getInstance` works with no type parameter — exactly Java. Snapshot instead of Java's live `ConcurrentMap` view (Rust hands out no live guard); BTreeMap-sorted per the repo's determinism convention (`/info/routes`, actuator.rs). Registry lock: `OnceLock<Mutex<HashMap<..>>>`; create-idempotency = `entry().or_insert_with()` under that one lock (Java's `ReentrantLock` + `ConcurrentMap` collapse into one; first creation's params win, §2). Schema-registry note: `create_cache` returns the `Arc` handle, so caches can also be **passed** into constructors (the Java client receives two handles) — the registry is a convenience, not the only access path. |
| MC5 | **Housekeeper: lifecycle-wired, never spawned from `create_cache`** — `managed_cache::start_housekeeping()`, idempotent (`OnceLock`), 10-min `tokio` interval sweeping `run_pending_tasks()` over the registry; called from `app_starter`'s essential-services phase (the `elastic_queue::start_housekeeping()` precedent, app_starter.rs:148) | `create_cache` legitimately runs where no Tokio runtime exists (`OnceLock` static init, plain `#[test]`); `tokio::spawn` there panics. The repo's *other* housekeeping precedent — knowledge-graph `commands::start_housekeeping()`, started lazily from the request handler — is the closer analog to Java's lazy first-create trigger but is rejected here for exactly that reason: the KG handler always runs inside the runtime; `create_cache` does not. Java starts its sweeper lazily on first create — the trigger point differs (documented divergence), the observable behavior does not: correctness never depends on the sweep in either engine (moka expires on access like Caffeine; the sweep only freshens idle caches' memory/introspection). Bare tests call `clean_up()` explicitly. Logs `Housekeeper started` once (Java wording). |
| MC6 | **API naming**: snake_case, bare-noun getters (repo style, e.g. `envelope.id()`) — see §4 | Matches `EventEnvelope` and the port's conventions; Java's overload becomes a second constructor fn (`create_cache` / `create_cache_with_limit`). |
| MC7 | **Module**: `crates/platform-core/src/util/managed_cache.rs` (`pub mod managed_cache;` in util/mod.rs); `//!` doc names `org.platformlambda.core.util.ManagedCache`; **`ManagedCache` + `CacheValue` join the lib.rs `pub use` list** (the AppConfigReader/ConfigReader precedent — this is a public application-facing utility in Java, not an internal like `elastic_queue`) | Repo convention (Apache header; module doc names the Java class ported; parity notes inline). The knowledge-graph adopter consumes the lib.rs re-export. |
| MC8 | **Testability**: public constructors keep the 1000 ms floor clamp (plus a documented ~100-year ceiling — moka's builder panics past 1000 years, a divergence Java's any-`long` API doesn't have; clamping keeps `create_cache` total); one **crate-private** unclamped-floor constructor (`pub(crate)`, doc'd test-only) enables fast TTL tests (~50 ms expiry + ~200 ms sleeps) | This is a **new pattern for the repo** — no `pub(crate)` test-only constructor exists today; the nearest precedent is the built-in test affordance "port 0 = ephemeral, for tests" (automation/server.rs). moka has an injectable `Clock`, but it is `pub(crate)` in moka itself — not available to us; and the repo's TTL-ish tests all sleep with tolerances (42 `sleep(from_millis)` calls across `crates/*/tests`). Without the seam, every TTL test sleeps >1 s. The clamp stays the only *public* path, so Java semantics hold for applications. Boundary rule for all TTL tests: assert presence only immediately after `put`, absence only well past TTL (≥ TTL + 300 ms) — never near the boundary. |

## 4. API surface (Java → Rust)

```rust
// crates/platform-core/src/util/managed_cache.rs
//! Rust port of org.platformlambda.core.util.ManagedCache — a named,
//! self-expiring (expire-after-write), size-bounded in-memory cache with a
//! process-wide registry. Engine: moka (Caffeine's Rust lineage), wrapped as
//! an internal detail. [parity + divergence notes per §5]

pub type CacheValue = Arc<dyn Any + Send + Sync>;

impl ManagedCache {
    pub fn create_cache(name: &str, expiry_ms: u64) -> Arc<ManagedCache>;              // maxItems = 2000
    pub fn create_cache_with_limit(name: &str, expiry_ms: u64, max_items: u64) -> Arc<ManagedCache>;
    pub fn get_instance(name: &str) -> Option<Arc<ManagedCache>>;
    pub fn get_cache_collection() -> BTreeMap<String, Arc<ManagedCache>>;              // sorted snapshot

    pub fn put<V: Any + Send + Sync>(&self, key: &str, value: V);                      // stamps last_write; empty key = no-op;
                                                                                       //   NEVER pass a pre-wrapped Arc — use put_arc
    pub fn put_arc(&self, key: &str, value: CacheValue);                               // stamps last_write
    pub fn get(&self, key: &str) -> Option<CacheValue>;                                // stamps last_read on any non-empty-key
                                                                                       //   call, hit or miss (Java parity)
    pub fn get_as<T: Any + Send + Sync>(&self, key: &str) -> Option<Arc<T>>;           // None on type mismatch
    pub fn exists(&self, key: &str) -> bool;                                           // = get(key).is_some() (inherits the stamp)
    pub fn remove(&self, key: &str);                                                   // stamps last_write even if key absent (Java)
    pub fn clear(&self);                                                               // stamps last_reset + invalidate_all
                                                                                       //   + run_pending_tasks (Java's 3 ops); no log
    pub fn clean_up(&self);                                                            // run_pending_tasks ("Cleaning up {}" debug)

    pub fn name(&self) -> &str;
    pub fn expiry_ms(&self) -> u64;      // clamped value (>= 1000)
    pub fn max_items(&self) -> u64;
    pub fn size(&self) -> u64;           // estimated (moka entry_count = Java estimatedSize)
    pub fn entries(&self) -> Vec<(String, CacheValue)>;   // unexpired snapshot via moka iter (skips expired
                                                          //   pre-eviction — verified); Java getMap = live view, §5
    pub fn last_read(&self) -> i64;      // epoch ms; AtomicI64 (Java's plain-long races made sound, same values)
    pub fn last_write(&self) -> i64;
    pub fn last_reset(&self) -> i64;
}

pub fn start_housekeeping();             // idempotent; requires a Tokio runtime; wired in app_starter
```

## 5. Divergences (documented, none silent)

| Divergence | Direction | Note |
|---|---|---|
| Eviction policy under capacity pressure | moka TinyLFU vs Caffeine W-TinyLFU | Same lineage, both approximate; TinyLFU admission may reject a newcomer rather than evict. moka ≥0.12.5 also offers `EvictionPolicy::lru` if deterministic eviction is ever wanted. No in-scope consumer approaches `max_items`. |
| `getMap()` live view → `entries()` snapshot | safe | Rust hands out no live guard; introspection-only surface. |
| Housekeeper start: first-create (Java) → lifecycle (`app_starter`) | behavioral no-op | Correctness never depends on the sweep in either engine; bare tests call `clean_up()`. |
| `getCacheCollection()` live map → sorted snapshot | safe | Determinism convention. |
| Expiry ceiling clamp (~100 years) | wrapper-added | Java accepts any `long`; moka's builder panics past 1000 years — the clamp keeps `create_cache` total. No real workload is affected. |
| `SimpleCache` call sites (if ever ported) → `ManagedCache` instances | ruling #1 | Bounded + self-expiring ⊃ unbounded lazy-expiry; one-line parity note per adopted site. |

## 6. In-repo adopters (this increment, separate commits)

1. **WS duplicate-command suppression** — `crates/knowledge-graph/src/commands.rs`
   `is_duplicate()`: replace the unbounded `Mutex<HashMap<String,(String,i64)>>` with
   `ManagedCache::create_cache("last.ws.message", 1000)` (Java
   `GraphCommandService` name + TTL, GraphCommandService.java:58). Two improvements,
   both **toward** Java: bounded, self-expiring memory (today: one entry per WS session
   route, never removed — the session-close arm cleans everything *except* this map); and
   a **window-semantics parity fix** — the current code re-`put`s on every duplicate,
   creating a *sliding* window (a continuous duplicate stream never lets a command
   through), while Java's expire-after-write is *anchored* (one command passes per ~1 s;
   Java puts only in the not-equal branch). Migrated form: `get_as::<String>` → equal ⇒
   duplicate — log `Duplicated message - {command} for {in_route}` at debug (Java's exact
   wording; log-presentation parity is a standing invariant) — return true with **no
   re-put**; else `put`. The existing WS double-submit regression —
   `graph_runtime_end_to_end` → `companion_sync_contract_gaps_closed`, WS-guard section
   (graph_runtime.rs:337–391) — passes unchanged under anchored semantics (verified
   during critique: two back-to-back identical sends still yield exactly one echo);
   `is_duplicate` has no other callers in the workspace. Add an anchored-window test
   (duplicate within 1 s dropped; same command re-accepted after ~1.3 s).
2. **Actuator per-dependency info cache** — `crates/platform-core/src/actuator.rs`
   `check_services()` (line 307). **Corrected fact (verified in the Java source):**
   Java does **not** cache the `/health` result. It caches only the per-dependency
   `type=info` lookup (`SimpleCache("health.info", 5000)`, key `info/{route}`,
   ActuatorServices.java:39,387) — and only when the info body is a **Map**; a non-map
   info response is re-requested every call (:388–394). The `type=health` probe re-runs
   on **every** call (10 s timeout), and `/livenessprobe` reads an `AtomicBoolean` —
   never the cache. The Rust port currently re-sends the info request (3 s timeout,
   actuator.rs:319–322) on every `/health` call — the module doc itself lists "the Java
   per-route info cache" as deferred (actuator.rs:46). Adopt
   `ManagedCache::create_cache("health.info", 5000)` around the info lookup only, with
   the only-cache-map-bodies guard (the Rust merge site already merges only Object
   bodies) and a parity note that Java used SimpleCache here (ruling #1). Test: a
   counting mock dependency — two back-to-back `/health` calls ⇒ info counted **once**,
   health counted **twice** (proving health itself is never cached). **Isolation
   hazard (designed for):** the `health.info` cache is process-wide and the actuator
   test binary runs its `#[tokio::test]`s in parallel against the shared route
   `demo.health` — the counting test must register its dependency under a **unique
   route** (its cache key is then unshared) so cached info never bleeds across tests.
3. **(Optional) Event-script test fixture** — the Java `ExternalStateMachine` test task
   uses `ManagedCache.createCache("state.machine", 10000)` explicitly "so that memory
   will be released" (event-script-engine test source); the Rust twin
   (crates/event-script/tests/flow_runtime.rs:297–318) uses an unbounded
   `OnceLock<Mutex<HashMap>>` whose per-trace entries are never evicted. Harmless today
   (test-binary lifetime), but adopting it dogfoods the API from a second crate and
   restores fixture parity. Cheap; included unless you'd rather keep the increment
   minimal.

**Future consumer (validates the API, not in scope):** minimalist-kafka's
`ManagedCacheSchemaRegistryClient` — receives two cache **handles** in its constructor
(→ MC4's `Arc` returns), stores arbitrary parsed-schema structs (→ MC2's `Any` carrier),
uses `get`/`put` (client) + `clear` (codec) only.

## 7. Test plan

In-module unit tests: put/get/exists/remove/clear round-trip incl. `get_as` downcast +
wrong-type `None` + the **wrong-wrap regression** (`put(key, Arc<T>)` ⇒ `get_as::<T>()`
is `None`, `get_as::<Arc<T>>()` hits — pins the MC2 doc warning); empty-key no-ops;
floor clamp (`create_cache(n, 500)` ⇒ `expiry_ms() == 1000`) + ceiling clamp doesn't
panic (`u64::MAX`); TTL expiry via the crate-private fast constructor (present
immediately after put; `None` well past TTL — lazy, no housekeeper); TTL reset-on-update
(re-`put` extends life — the anchored-window primitive); bound holds (`max_items = 3`,
insert 4 ⇒ `size() <= 3` after `clean_up()`; **neither victim nor survivor set
asserted** — TinyLFU admission may reject the newcomer); create-idempotency
(`Arc::ptr_eq`, second call's params ignored — Java semantics); registry (`get_instance`
Some/None, sorted collection); telemetry stamps per the §2 map (incl. `remove` on an
absent key stamps `last_write`; `get` miss stamps `last_read`; `clear` seeds/stamps
`last_reset`); concurrent put/get smoke. Integration: the adopter tests in §6 (with the
§6.2 route-isolation rule). Never test the 10-min timer — test `clean_up()` directly
(the repo's housekeeping-fn-not-loop pattern, e.g. `scan_expired_stores()` in
lifecycle tests).

## 8. Open questions for the gate

1. **MC1 (engine = moka)** — confirm, given ruling #2's "adopt a proper implementation"
   reading and the bus-factor-1 caveat. The wrapper keeps it swappable either way.
2. **Adopter scope** — actuator `health.info` cache (§6.2) in this increment
   (recommended: yes, own commit + parity note)? The optional §6.3 test-fixture
   adoption too, or keep the increment minimal?
3. **MC8** — is the crate-private unclamped constructor acceptable for fast TTL tests?
   It is a new pattern for this repo (nearest precedent: "port 0 = ephemeral, for
   tests"). Rejecting it means >1 s sleeps in every TTL test.
