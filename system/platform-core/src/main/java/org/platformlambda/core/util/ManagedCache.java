/*

    Copyright 2018-2020 Accenture Technology

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
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;

public class ManagedCache {
    private static final Logger log = LoggerFactory.getLogger(ManagedCache.class);

    private static final long DEFAULT_MAX_ITEMS = 2000;
    private static final long MIN_EXPIRY = 1000;
    private static final long FIVE_MINUTES = 5 * 60000;
    private static final ConcurrentMap<String, ManagedCache> cacheCollection = new ConcurrentHashMap<>();
    private static final AtomicInteger counter = new AtomicInteger(0);

    private String name;
    private long expiry, maxItems;
    private Cache<String, Object> cache;
    private long lastWrite = 0, lastRead = 0, lastReset = System.currentTimeMillis();

    private ManagedCache(Cache<String, Object> cache, String name, long expiryMs, long maxItems) {
        this.cache = cache;
        this.name = name;
        this.expiry = expiryMs;
        this.maxItems = maxItems;
        if (counter.incrementAndGet() == 1) {
            // clean up cache every 5 minutes to promote garbage collection
            CleanUp cleanUp = new CleanUp();
            cleanUp.start();
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
    public synchronized static ManagedCache createCache(String name, long expiryMs, long maxItems) {
        ManagedCache managedCache = getInstance(name);
        if (managedCache != null) {
            return managedCache;
        }
        long expiryTimer = Math.max(expiryMs, MIN_EXPIRY);
        Cache<String, Object> cache = CacheBuilder.newBuilder().maximumSize(maxItems).expireAfterWrite(expiryTimer, TimeUnit.MILLISECONDS).build();
        // create cache
        managedCache = new ManagedCache(cache, name, expiryTimer, maxItems);
        cacheCollection.put(name, managedCache);
        log.info("Created cache ({}), expiry {} ms, maxItems={}", name, expiryTimer, maxItems);
        return managedCache;
    }

    public static ManagedCache getInstance(String name) {
        return cacheCollection.get(name);
    }

    public static ConcurrentMap<String, ManagedCache> getCacheCollection() {
        return cacheCollection;
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

    private class CleanUp extends Thread {

        private boolean normal = true;

        private CleanUp() {
            Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
        }

        @Override
        public void run() {
            log.info("Started");
            long t1 = System.currentTimeMillis();
            while (normal) {
                long now = System.currentTimeMillis();
                // avoid scanning frequently
                if (now - t1 > FIVE_MINUTES) {
                    t1 = now;
                    // clean up cache collection
                    for (String key : cacheCollection.keySet()) {
                        ManagedCache c = cacheCollection.get(key);
                        c.cleanUp();
                    }
                }
                try {
                    Thread.sleep(1000);
                } catch (InterruptedException e) {
                    // ok to ignore
                }
            }
            log.info("Stopped");
        }

        private void shutdown() {
            normal = false;
        }

    }

}
