/*

    Copyright 2018-2023 Accenture Technology

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

 */

package org.platformlambda.core.util;

import com.google.common.cache.Cache;
import com.google.common.cache.CacheBuilder;
import org.platformlambda.core.system.Platform;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;

public class ManagedCache {
    private static final Logger log = LoggerFactory.getLogger(ManagedCache.class);

    private static final long DEFAULT_MAX_ITEMS = 2000L;
    private static final long MIN_EXPIRY = 1000L;
    private static final ConcurrentMap<String, ManagedCache> COLLECTION = new ConcurrentHashMap<>();
    private static final AtomicInteger INIT_COUNTER = new AtomicInteger(0);
    private static final AtomicBoolean NOT_RUNNING = new AtomicBoolean(true);
    private static final long HOUSEKEEPING_INTERVAL = 10 * 60 * 1000L; // 10 minutes
    private final String name;
    private final long expiry;
    private final long maxItems;
    private final Cache<String, Object> cache;
    private long lastWrite = 0;
    private long lastRead = 0;
    private long lastReset = System.currentTimeMillis();

    private ManagedCache(Cache<String, Object> cache, String name, long expiryMs, long maxItems) {
        this.cache = cache;
        this.name = name;
        this.expiry = expiryMs;
        this.maxItems = maxItems;
        if (INIT_COUNTER.incrementAndGet() == 1) {
            Platform.getInstance().getVertx().setPeriodic(HOUSEKEEPING_INTERVAL, t -> removeExpiredCache());
            log.info("Housekeeper started");
        }
        if (INIT_COUNTER.get() > 10000) {
            INIT_COUNTER.set(10);
        }
    }

    /**
     * Obtain a ManagedCache instance
     *
     * @param name of cache store
     * @param expiryMs in milliseconds
     * @return cache instance
     */
    public static ManagedCache createCache(String name, long expiryMs) {
        return createCache(name, expiryMs, DEFAULT_MAX_ITEMS);
    }

    /**
     * Obtain a ManagedCache instance
     *
     * @param name of cache store
     * @param expiryMs in milliseconds
     * @param maxItems maximum number of cached objects
     * @return cache instance
     */
    public static synchronized ManagedCache createCache(String name, long expiryMs, long maxItems) {
        ManagedCache managedCache = getInstance(name);
        if (managedCache != null) {
            return managedCache;
        }
        long expiryTimer = Math.max(expiryMs, MIN_EXPIRY);
        Cache<String, Object> cache = CacheBuilder.newBuilder().maximumSize(maxItems).expireAfterWrite(expiryTimer, TimeUnit.MILLISECONDS).build();
        // create cache
        managedCache = new ManagedCache(cache, name, expiryTimer, maxItems);
        COLLECTION.put(name, managedCache);
        log.info("Created cache ({}), expiry {} ms, maxItems={}", name, expiryTimer, maxItems);
        return managedCache;
    }

    public static ManagedCache getInstance(String name) {
        return COLLECTION.get(name);
    }

    public static ConcurrentMap<String, ManagedCache> getCacheCollection() {
        return COLLECTION;
    }

    public String getName() {
        return name;
    }

    public long getExpiry() {
        return expiry;
    }

    public long getMaxItems() {
        return maxItems;
    }

    public void put(String key, Object value) {
        if (key != null && key.length() > 0) {
            lastWrite = System.currentTimeMillis();
            cache.put(key, value);
        }
    }

    public boolean exists(String key) {
        return get(key) != null;
    }

    public Object get(String key) {
        if (key != null && key.length() > 0) {
            lastRead = System.currentTimeMillis();
            return cache.getIfPresent(key);
        } else {
            return null;
        }
    }

    public void remove(String key) {
        if (key != null && key.length() > 0) {
            lastWrite = System.currentTimeMillis();
            cache.invalidate(key);
        }
    }

    public void clear() {
        lastReset = System.currentTimeMillis();
        cache.invalidateAll();
        cache.cleanUp();
    }

    public void cleanUp() {
        log.debug("Cleaning up {}", this.getName());
        cache.cleanUp();
    }

    public long size() {
        return cache.size();
    }

    public ConcurrentMap<String, Object> getMap() {
        return cache.asMap();
    }

    public long getLastRead() {
        return lastRead;
    }

    public long getLastWrite() {
        return lastWrite;
    }

    public long getLastReset() {
        return lastReset;
    }

    private void removeExpiredCache() {
        if (NOT_RUNNING.compareAndSet(true, false)) {
            Platform.getInstance().getEventExecutor().submit(() -> {
                try {
                    COLLECTION.keySet().forEach(k -> {
                        ManagedCache c = COLLECTION.get(k);
                        c.cleanUp();
                    });
                } finally {
                    NOT_RUNNING.set(true);
                }
            });
        }
    }

}
