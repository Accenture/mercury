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

import org.platformlambda.core.models.TimedItem;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * SimpleCache is a simple ConcurrentHashMap with automatic removal of inactive entries
 *
 * WARNING - You should not use this as a regular cache class because its memory consumption would expand without limit.
 * This class is mainly used by the MultipartPayload class for segmentation of large event payload.
 */
public class SimpleCache {
    private static final Logger log = LoggerFactory.getLogger(SimpleCache.class);

    private static final long HOUSEKEEPING_INTERVAL = 10000;
    private static final long MIN_EXPIRY = HOUSEKEEPING_INTERVAL;
    private static final ConcurrentMap<String, SimpleCache> cacheCollection = new ConcurrentHashMap<>();
    private static final AtomicInteger counter = new AtomicInteger(0);

    private String name;
    private long expiry;

    private ConcurrentMap<String, TimedItem> cache = new ConcurrentHashMap<>();

    private SimpleCache(String name, long expiryMs) {
        this.name = name;
        this.expiry = expiryMs;
        if (counter.incrementAndGet() == 1) {
            CleanUp cleanUp = new CleanUp();
            cleanUp.start();
        }
    }

    /**
     * Create a simple cache with expiry timer
     *
     * @param name cache label
     * @param expiryMs timer
     * @return simple cache object
     */
    public synchronized static SimpleCache createCache(String name, long expiryMs) {
        SimpleCache simpleCache = getInstance(name);
        if (simpleCache != null) {
            return simpleCache;
        }
        long expiryTimer = Math.max(expiryMs, MIN_EXPIRY);
        simpleCache = new SimpleCache(name, expiryTimer);
        cacheCollection.put(name, simpleCache);
        log.info("Created cache ({}), expiry {} ms", name, expiryTimer);
        return simpleCache;
    }

    public static SimpleCache getInstance(String name) {
        return cacheCollection.get(name);
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
        return item == null? null: item.payload;
    }

    public String getName() {
        return name;
    }

    public long getExpiry() {
        return expiry;
    }

    public void cleanUp() {
        long now = System.currentTimeMillis();
        log.debug("Cleaning up {}", this.getName());
        // clean up cache
        List<String> expired = new ArrayList<>();
        for (String k: cache.keySet()) {
            TimedItem item = cache.get(k);
            if (now - item.time > expiry) {
                expired.add(k);
            }
        }
        if (!expired.isEmpty()) {
            for (String k : expired) {
                remove(k);
            }
            log.info("Total {} item{} expired", expired.size(), expired.size() == 1? "" : "s");
        }
    }

    public int size() {
        return cache.size();
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
                if (now - t1 > HOUSEKEEPING_INTERVAL) {
                    t1 = now;
                    // clean up cache collection
                    for (String key : cacheCollection.keySet()) {
                        SimpleCache c = cacheCollection.get(key);
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
