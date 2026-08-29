//
// Copyright 2018-2026 Accenture Technology
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Rust port of the Java `ManagedCache`
//! (`org.platformlambda.core.util.ManagedCache`) — a **named, self-expiring
//! (expire-after-write), size-bounded in-memory cache** with a process-wide
//! registry. Design record: `draft-design-specs/managed-cache-port.md`
//! (maintainer-approved 2026-07-27).
//!
//! Engine: [moka](https://docs.rs/moka) (the Caffeine-lineage Rust cache),
//! kept an internal detail behind this wrapper so it can be swapped without
//! touching any consumer. Deliberate, documented divergences from the Java
//! original (design §5):
//!
//! - **Deterministic eviction** (maintainer ruling, 2026-07-27): the store is
//!   built with `EvictionPolicy::lru()` — newcomers are always admitted and
//!   the least-recently-used entry is the victim — where Java's Caffeine uses
//!   approximate W-TinyLFU with frequency-based admission plus deliberate
//!   HashDoS jitter (no policy switch exists there; a refactoring note is
//!   filed with the Java team).
//! - The housekeeper is **lifecycle-wired** ([`start_housekeeping`], called by
//!   `AppStarter`'s essential-services phase) instead of lazily started on
//!   first create: `create_cache` legitimately runs where no Tokio runtime
//!   exists (static init, plain tests). Correctness never depends on the
//!   sweep in either engine — the store itself enforces expiry on access.
//! - Expiry is clamped to a ~100-year ceiling as well as the Java 1 s floor
//!   (moka's builder panics past 1000 years; Java accepts any `long`).
//! - `entries()` / [`ManagedCache::get_cache_collection`] return snapshots
//!   where Java hands out live `ConcurrentMap` views.
//!
//! Values are type-erased as [`CacheValue`] (`Arc<dyn Any + Send + Sync>`) —
//! the faithful Rust carrier of Java's `Object` reference semantics: the
//! `Arc` clone returned by [`ManagedCache::get`] is the analog of Java
//! handing back the same object reference. Convention: one named cache
//! stores one value shape; [`ManagedCache::get_as`] returns `None` on a type
//! mismatch, exactly where Java's cast would sit.
//!
//! Java's `SimpleCache` is deliberately NOT ported (maintainer ruling): any
//! Java `SimpleCache` call site ported later maps onto a `ManagedCache`
//! instance — bounded + self-expiring is a strict superset of `SimpleCache`'s
//! unbounded lazy expiry. State the parity note once at each adopted site.

use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use moka::policy::EvictionPolicy;
use moka::sync::Cache;

use crate::util::elapsed_time;

/// Type-erased cache value — the Rust carrier of Java's `Object`.
pub type CacheValue = Arc<dyn Any + Send + Sync>;

/// Default capacity of [`ManagedCache::create_cache`] (Java `DEFAULT_MAX_ITEMS`).
const DEFAULT_MAX_ITEMS: u64 = 2000;
/// Expiry floor in ms (Java `MIN_EXPIRY`) — clamped up, never rejected.
const MIN_EXPIRY_MS: u64 = 1000;
/// Expiry ceiling (~100 years): moka's builder panics past 1000 years, so the
/// clamp keeps `create_cache` total where Java accepts any `long` (design §5).
const MAX_EXPIRY_MS: u64 = 100 * 365 * 24 * 60 * 60 * 1000;
/// Housekeeper cadence (Java `HOUSEKEEPING_INTERVAL` — 10 minutes).
const HOUSEKEEPING_INTERVAL: Duration = Duration::from_secs(600);

/// A named, self-expiring, size-bounded cache (see the module doc).
pub struct ManagedCache {
    name: String,
    expiry_ms: u64,
    max_items: u64,
    store: Cache<String, CacheValue>,
    // telemetry stamps — Java uses plain (racy) longs; atomics with relaxed
    // ordering carry the same values soundly
    last_read: AtomicI64,
    last_write: AtomicI64,
    last_reset: AtomicI64,
}

/// The process-wide registry (Java `COLLECTION` + `SAFETY` lock, collapsed
/// into one mutex-guarded map).
fn registry() -> &'static Mutex<HashMap<String, Arc<ManagedCache>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<ManagedCache>>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or_default()
}

impl ManagedCache {
    /// Obtain (or create) the named cache with the default 2000-item bound
    /// (Java `createCache(name, expiryMs)`). Idempotent by name: a later call
    /// returns the existing instance unchanged — the first creation's
    /// parameters win, later parameters are ignored (Java semantics).
    pub fn create_cache(name: &str, expiry_ms: u64) -> Arc<ManagedCache> {
        Self::create_cache_with_limit(name, expiry_ms, DEFAULT_MAX_ITEMS)
    }

    /// Obtain (or create) the named cache (Java `createCache(name, expiryMs,
    /// maxItems)`). Expiry is clamped to \[1 s, ~100 years\].
    pub fn create_cache_with_limit(
        name: &str,
        expiry_ms: u64,
        max_items: u64,
    ) -> Arc<ManagedCache> {
        Self::create_clamped(
            name,
            expiry_ms.clamp(MIN_EXPIRY_MS, MAX_EXPIRY_MS),
            max_items,
        )
    }

    /// Test seam (design MC8, maintainer-approved): bypasses the 1 s floor so
    /// TTL unit tests run in milliseconds. The public constructors above are
    /// the only application path and always clamp.
    #[cfg(test)]
    fn create_cache_unclamped(name: &str, expiry_ms: u64, max_items: u64) -> Arc<ManagedCache> {
        Self::create_clamped(name, expiry_ms.min(MAX_EXPIRY_MS), max_items)
    }

    fn create_clamped(name: &str, expiry_ms: u64, max_items: u64) -> Arc<ManagedCache> {
        let mut collection = registry().lock().expect("managed cache registry");
        if let Some(existing) = collection.get(name) {
            return existing.clone();
        }
        let store = Cache::builder()
            .max_capacity(max_items)
            .time_to_live(Duration::from_millis(expiry_ms))
            // deterministic eviction — maintainer ruling (module doc)
            .eviction_policy(EvictionPolicy::lru())
            .build();
        let cache = Arc::new(ManagedCache {
            name: name.to_string(),
            expiry_ms,
            max_items,
            store,
            last_read: AtomicI64::new(0),
            last_write: AtomicI64::new(0),
            last_reset: AtomicI64::new(now_ms()),
        });
        collection.insert(name.to_string(), cache.clone());
        log::info!(
            "Created cache ({}), expiry {}, maxItems={}",
            name,
            elapsed_time(Duration::from_millis(expiry_ms)),
            max_items
        );
        cache
    }

    /// Resolve a cache by name from any module (Java `getInstance`).
    pub fn get_instance(name: &str) -> Option<Arc<ManagedCache>> {
        registry()
            .lock()
            .expect("managed cache registry")
            .get(name)
            .cloned()
    }

    /// Sorted snapshot of every registered cache for ops introspection
    /// (Java `getCacheCollection` returns the live map — design §5).
    pub fn get_cache_collection() -> BTreeMap<String, Arc<ManagedCache>> {
        registry()
            .lock()
            .expect("managed cache registry")
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Store a value (Java `put`) — stamps `last_write`; empty key = no-op.
    /// NEVER pass a pre-wrapped `Arc`/[`CacheValue`] here — it would nest
    /// (`Arc<Arc<T>>`) and `get_as::<T>` would miss; use [`Self::put_arc`]
    /// for values that are already reference-counted.
    pub fn put<V: Any + Send + Sync>(&self, key: &str, value: V) {
        self.put_arc(key, Arc::new(value));
    }

    /// Store an already-wrapped value — stamps `last_write`; empty key = no-op.
    pub fn put_arc(&self, key: &str, value: CacheValue) {
        if !key.is_empty() {
            self.last_write.store(now_ms(), Ordering::Relaxed);
            self.store.insert(key.to_string(), value);
        }
    }

    /// Fetch a value (Java `get`) — stamps `last_read` on any non-empty-key
    /// call, hit or miss (Java stamps before the lookup).
    pub fn get(&self, key: &str) -> Option<CacheValue> {
        if key.is_empty() {
            return None;
        }
        self.last_read.store(now_ms(), Ordering::Relaxed);
        self.store.get(key)
    }

    /// Typed fetch: `None` on absence OR type mismatch — the Rust analog of
    /// the cast at a Java call site.
    pub fn get_as<T: Any + Send + Sync>(&self, key: &str) -> Option<Arc<T>> {
        self.get(key).and_then(|value| value.downcast::<T>().ok())
    }

    /// Java `exists` — delegates to `get`, so it inherits the `last_read` stamp.
    pub fn exists(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Java `remove` — stamps `last_write` even when the key is absent.
    pub fn remove(&self, key: &str) {
        if !key.is_empty() {
            self.last_write.store(now_ms(), Ordering::Relaxed);
            self.store.invalidate(key);
        }
    }

    /// Java `clear` is three operations: stamp `lastReset`, `invalidateAll()`,
    /// `cleanUp()` — the cleanup keeps `size()` honest immediately after a
    /// clear. Emits no log (only `clean_up` logs).
    pub fn clear(&self) {
        self.last_reset.store(now_ms(), Ordering::Relaxed);
        self.store.invalidate_all();
        self.store.run_pending_tasks();
    }

    /// Java `cleanUp` — apply pending maintenance (expiry sweep, deferred
    /// evictions) now.
    pub fn clean_up(&self) {
        log::debug!("Cleaning up {}", self.name);
        self.store.run_pending_tasks();
    }

    /// Cache name (Java `getName`).
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The clamped expiry in ms (Java `getExpiry` returns the clamped value).
    pub fn expiry_ms(&self) -> u64 {
        self.expiry_ms
    }

    /// Capacity bound (Java `getMaxItems`).
    pub fn max_items(&self) -> u64 {
        self.max_items
    }

    /// Estimated entry count (Java `size()` = Caffeine `estimatedSize`;
    /// moka `entry_count` — call [`Self::clean_up`] first for freshness).
    pub fn size(&self) -> u64 {
        self.store.entry_count()
    }

    /// Snapshot of the unexpired entries (Java `getMap` returns a live
    /// `ConcurrentMap` view; Rust hands out no live guard — design §5).
    pub fn entries(&self) -> Vec<(String, CacheValue)> {
        self.store.iter().map(|(k, v)| ((*k).clone(), v)).collect()
    }

    /// Epoch ms of the last `get`/`exists` (Java `getLastRead`; 0 = never).
    pub fn last_read(&self) -> i64 {
        self.last_read.load(Ordering::Relaxed)
    }

    /// Epoch ms of the last `put`/`remove` (Java `getLastWrite`; 0 = never).
    pub fn last_write(&self) -> i64 {
        self.last_write.load(Ordering::Relaxed)
    }

    /// Epoch ms of construction or the last `clear` (Java `getLastReset`).
    pub fn last_reset(&self) -> i64 {
        self.last_reset.load(Ordering::Relaxed)
    }
}

/// Start the 10-minute housekeeper the lifecycle owns (idempotent). Java
/// starts its sweeper lazily inside the first constructor; here
/// `create_cache` may run where no Tokio runtime exists, so the lifecycle
/// wires this instead — a documented behavioral no-op (design §5): the sweep
/// only reclaims memory in caches with no subsequent activity; the store
/// itself enforces expiry on access. Must be called within a Tokio runtime
/// (`AppStarter::run` does — its "essential services" phase).
pub fn start_housekeeping() {
    static STARTED: OnceLock<()> = OnceLock::new();
    STARTED.get_or_init(|| {
        log::info!("Housekeeper started");
        tokio::spawn(async {
            loop {
                tokio::time::sleep(HOUSEKEEPING_INTERVAL).await;
                housekeeping();
            }
        });
    });
}

/// One housekeeping sweep over every registered cache (the Java
/// `removeExpiredCache` body; a single sequential task, so sweeps never
/// overlap — the intent of Java's `NOT_RUNNING` guard). The interval task
/// calls this; tests call it directly — never test the timer.
fn housekeeping() {
    for cache in ManagedCache::get_cache_collection().into_values() {
        cache.clean_up();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: the registry is process-wide and unit tests run in parallel —
    // every test uses its own unique cache name.

    #[test]
    fn round_trip_remove_clear_and_entries() {
        let cache = ManagedCache::create_cache("unit.round.trip", 60_000);
        cache.put("s", "text".to_string());
        cache.put("n", 7_i64);
        assert_eq!(cache.get_as::<String>("s").unwrap().as_str(), "text");
        assert_eq!(*cache.get_as::<i64>("n").unwrap(), 7);
        // wrong type -> None (the Java cast site)
        assert!(cache.get_as::<i64>("s").is_none());
        let mut keys: Vec<String> = cache.entries().into_iter().map(|(k, _)| k).collect();
        keys.sort();
        assert_eq!(keys, ["n", "s"]);
        cache.remove("s");
        assert!(!cache.exists("s"));
        cache.clear();
        // clear runs pending tasks (Java invalidateAll + cleanUp), so the
        // estimate is honest immediately
        assert_eq!(cache.size(), 0);
        assert!(cache.get("n").is_none());
    }

    #[test]
    fn pre_wrapped_arc_is_the_documented_trap() {
        let cache = ManagedCache::create_cache("unit.wrong.wrap", 60_000);
        let wrapped: Arc<String> = Arc::new("hello".to_string());
        cache.put("k", wrapped); // stores TypeId Arc<String>, NOT String
        assert!(cache.get_as::<String>("k").is_none());
        assert!(cache.get_as::<Arc<String>>("k").is_some());
        // the right way for a pre-wrapped value
        cache.put_arc("k2", Arc::new("hello".to_string()));
        assert_eq!(cache.get_as::<String>("k2").unwrap().as_str(), "hello");
    }

    #[test]
    fn empty_key_is_a_guarded_no_op() {
        let cache = ManagedCache::create_cache("unit.empty.key", 60_000);
        cache.put("", "x".to_string());
        cache.clean_up();
        assert_eq!(cache.size(), 0);
        assert!(cache.get("").is_none());
        assert!(!cache.exists(""));
        cache.remove("");
        // Java guards BEFORE stamping: none of the above touched the markers
        assert_eq!(cache.last_read(), 0);
        assert_eq!(cache.last_write(), 0);
    }

    #[test]
    fn expiry_clamps_at_both_ends() {
        let low = ManagedCache::create_cache("unit.clamp.low", 500);
        assert_eq!(low.expiry_ms(), 1000);
        // must not panic (moka's builder rejects > 1000 years — design §5)
        let high = ManagedCache::create_cache("unit.clamp.high", u64::MAX);
        assert_eq!(high.expiry_ms(), MAX_EXPIRY_MS);
    }

    #[test]
    fn create_is_idempotent_first_params_win() {
        let first = ManagedCache::create_cache_with_limit("unit.create.idempotent", 5_000, 10);
        let second = ManagedCache::create_cache_with_limit("unit.create.idempotent", 9_000, 99);
        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(second.expiry_ms(), 5_000);
        assert_eq!(second.max_items(), 10);
    }

    #[test]
    fn registry_lookup_and_sorted_collection() {
        ManagedCache::create_cache("unit.registry.zeta", 60_000);
        ManagedCache::create_cache("unit.registry.alpha", 60_000);
        assert!(ManagedCache::get_instance("unit.registry.alpha").is_some());
        assert!(ManagedCache::get_instance("no.such.cache").is_none());
        let all = ManagedCache::get_cache_collection();
        let names: Vec<&str> = all
            .keys()
            .map(String::as_str)
            .filter(|k| k.starts_with("unit.registry."))
            .collect();
        // BTreeMap keeps the snapshot deterministic (the /info/routes rule)
        assert_eq!(names, ["unit.registry.alpha", "unit.registry.zeta"]);
        assert_eq!(
            all.get("unit.registry.alpha").unwrap().name(),
            "unit.registry.alpha"
        );
    }

    #[test]
    fn ttl_expires_after_write_lazily() {
        // 200 ms TTL: presence is asserted immediately after the put (only a
        // >200 ms preemption between two adjacent statements could flake it);
        // the absence side sleeps far past the boundary (sleeps never
        // undershoot)
        let cache = ManagedCache::create_cache_unclamped("unit.ttl.expiry", 200, 100);
        cache.put("k", 42_i32);
        assert!(cache.exists("k"));
        std::thread::sleep(Duration::from_millis(600));
        // no housekeeper involved — the store enforces expiry on access
        assert!(cache.get_as::<i32>("k").is_none());
        assert!(!cache.exists("k"));
    }

    #[test]
    fn ttl_resets_on_update_expire_after_write() {
        // a reset test inherently needs one mid-window presence assert; the
        // 1200 ms TTL gives every boundary >= 400 ms of slow-CI headroom
        let cache = ManagedCache::create_cache("unit.ttl.reset", 1200);
        cache.put("k", "a".to_string());
        std::thread::sleep(Duration::from_millis(800));
        // a rewrite resets the clock (Caffeine expireAfterWrite parity) —
        // the primitive behind the WS-dedup anchored window
        cache.put("k", "b".to_string());
        std::thread::sleep(Duration::from_millis(800));
        // ~1.6 s after the first write (past its 1.2 s TTL — presence here
        // PROVES the reset) but only ~0.8 s after the rewrite
        assert_eq!(cache.get_as::<String>("k").unwrap().as_str(), "b");
        std::thread::sleep(Duration::from_millis(700));
        // ~1.5 s after the rewrite — expired
        assert!(cache.get("k").is_none());
    }

    #[test]
    fn lru_eviction_is_deterministic() {
        // maintainer ruling: EvictionPolicy::lru — newcomers always admitted,
        // the least-recently-used entry is the victim (design §5). Sequential
        // access, so the recorded order is exact.
        let cache = ManagedCache::create_cache_with_limit("unit.lru.eviction", 60_000, 3);
        cache.put("a", 1_i32);
        cache.put("b", 2_i32);
        cache.put("c", 3_i32);
        // flush the writes first: a maintenance pass applies the read log
        // BEFORE the write log, so a read recorded ahead of its entry's
        // write would be a recency no-op
        cache.clean_up();
        // touch a and b so c becomes the least recently used; flush the
        // recorded reads before inserting the 4th entry
        assert!(cache.exists("a"));
        assert!(cache.exists("b"));
        cache.clean_up();
        cache.put("d", 4_i32);
        cache.clean_up();
        assert!(!cache.exists("c"), "the LRU entry is the victim");
        assert!(cache.exists("a"));
        assert!(cache.exists("b"));
        assert!(cache.exists("d"), "the newcomer is always admitted");
        assert_eq!(cache.size(), 3);
    }

    #[test]
    fn telemetry_stamps_follow_the_java_map() {
        let cache = ManagedCache::create_cache("unit.telemetry.stamps", 60_000);
        assert_eq!(cache.last_read(), 0);
        assert_eq!(cache.last_write(), 0);
        assert!(cache.last_reset() > 0);
        // get on a miss stamps last_read (Java stamps before the lookup)
        assert!(cache.get("absent").is_none());
        assert!(cache.last_read() > 0);
        cache.put("k", "v".to_string());
        let first_write = cache.last_write();
        assert!(first_write > 0);
        std::thread::sleep(Duration::from_millis(10));
        // remove stamps last_write even when the key is absent (Java parity)
        cache.remove("no-such-key");
        assert!(cache.last_write() > first_write);
        let first_reset = cache.last_reset();
        std::thread::sleep(Duration::from_millis(10));
        cache.clear();
        assert!(cache.last_reset() > first_reset);
    }

    #[test]
    fn housekeeping_sweeps_idle_caches() {
        let cache = ManagedCache::create_cache_unclamped("unit.housekeeping", 50, 100);
        cache.put("k", 1_i32);
        std::thread::sleep(Duration::from_millis(250));
        // the sweep body the interval task runs — never test the timer
        housekeeping();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn concurrent_put_get_smoke() {
        let cache = ManagedCache::create_cache("unit.concurrent.smoke", 60_000);
        let mut handles = Vec::new();
        for t in 0..4 {
            let cache = cache.clone();
            handles.push(std::thread::spawn(move || {
                for i in 0..250 {
                    let key = format!("k{t}-{i}");
                    cache.put(&key, i);
                    assert_eq!(*cache.get_as::<i32>(&key).unwrap(), i);
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        cache.clean_up();
        assert_eq!(cache.size(), 1000);
    }
}
