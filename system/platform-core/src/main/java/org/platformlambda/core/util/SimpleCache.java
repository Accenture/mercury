/*

    Copyright 2018-2024 Accenture Technology

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

import org.platformlambda.core.models.TimedItem;
import org.platformlambda.core.system.Platform;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * IMPORTANT: Reserved for internal uses.
 * e.g. MultipartPayload class for segmentation of large event payload
 * <p>
 * SimpleCache is a simple ConcurrentHashMap with automatic removal of inactive entries
 * <b>
 * Please use the ManagedCache for regular caching use cases
 */
public class SimpleCache {
    private static final Logger log = LoggerFactory.getLogger(SimpleCache.class);
    private static final long MIN_EXPIRY = 1000L;
    private static final ConcurrentMap<String, SimpleCache> COLLECTION = new ConcurrentHashMap<>();
    private static final AtomicInteger INIT_COUNTER = new AtomicInteger(0);
    private static final AtomicBoolean NOT_RUNNING = new AtomicBoolean(true);
    private static final long HOUSEKEEPING_INTERVAL = 30 * 1000L;    // 30 seconds
    private final String name;
    private final long expiry;
    private final ConcurrentMap<String, TimedItem> cache = new ConcurrentHashMap<>();

    private SimpleCache(String name, long expiryMs) {
        this.name = name;
        this.expiry = expiryMs;
        if (INIT_COUNTER.incrementAndGet() == 1) {
            Platform.getInstance().getVertx().setPeriodic(HOUSEKEEPING_INTERVAL, t -> removeExpiredCache());
            log.info("Housekeeper started");
        }
        if (INIT_COUNTER.get() > 10000) {
            INIT_COUNTER.set(10);
        }
    }

    /**
     * Create a simple cache with expiry timer
     *
     * @param name cache label
     * @param expiryMs timer
     * @return simple cache object
     */
    public static synchronized SimpleCache createCache(String name, long expiryMs) {
        SimpleCache simpleCache = getInstance(name);
        if (simpleCache != null) {
            return simpleCache;
        }
        long expiryTimer = Math.max(expiryMs, MIN_EXPIRY);
        simpleCache = new SimpleCache(name, expiryTimer);
        COLLECTION.put(name, simpleCache);
        String timer = Utility.getInstance().elapsedTime(expiryTimer);
        log.info("Created cache ({}), expiry {}", name, timer);
        return simpleCache;
    }

    public static SimpleCache getInstance(String name) {
        return COLLECTION.get(name);
    }

    public void put(String key, Object o) {
        cache.put(key, new TimedItem(System.currentTimeMillis(), o));
    }

    public void remove(String key) {
        TimedItem item = cache.get(key);
        cache.remove(key);
        if (item != null) {
            // remove references
            item.time = null;
            item.payload = null;
        }
    }

    public Object get(String key) {
        TimedItem item = cache.get(key);
        if (item == null) {
            return null;
        } else {
            if (System.currentTimeMillis() - item.time > expiry) {
                cache.remove(key);
                return null;
            } else {
                return item.payload;
            }
        }
    }

    public boolean exists(String key) {
        return get(key) != null;
    }

    public String getName() {
        return name;
    }

    public long getExpiry() {
        return expiry;
    }

    public void clear() {
        cache.clear();
    }

    public void cleanUp() {
        log.debug("Cleaning up {}", this.getName());
        long now = System.currentTimeMillis();
        // clean up cache
        List<String> expired = new ArrayList<>();
        cache.keySet().forEach(k -> {
            TimedItem item = cache.get(k);
            if (now - item.time > expiry) {
                expired.add(k);
            }
        });
        for (String k : expired) {
            remove(k);
        }
    }

    public int size() {
        return cache.size();
    }

    private void removeExpiredCache() {
        if (NOT_RUNNING.compareAndSet(true, false)) {
            Platform.getInstance().getEventExecutor().submit(() -> {
                try {
                    COLLECTION.keySet().forEach(k -> {
                        SimpleCache c = COLLECTION.get(k);
                        c.cleanUp();
                    });
                } finally {
                    NOT_RUNNING.set(true);
                }
            });
        }
    }

}
