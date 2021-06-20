package org.platformlambda.core.util;

import org.junit.Assert;
import org.junit.Test;

public class CacheTest {

    // note that the cache expiry has a minimum value of 1000 ms
    private static final ManagedCache cache1 = ManagedCache.createCache("hello.world", 1000, 100);
    private static final SimpleCache cache2 = SimpleCache.createCache("simple.cache", 15000);

    @Test
    public void cacheBehavior() {
        String KEY = "key1";
        String DATA = "hello";
        cache1.put(KEY, DATA);
        Object o = cache1.get(KEY);
        Assert.assertEquals(DATA, o);
        long n = cache1.size();
        Assert.assertEquals(1, n);
        try {
            Thread.sleep(1010);
        } catch (InterruptedException e) {
            // ok to ignore
        }
        // cached item will disappear in one second
        Object o2 = cache1.get(KEY);
        Assert.assertNull(o2);
        // test clean up
        cache1.put(KEY, DATA);
        cache1.remove(KEY);
        Assert.assertFalse(cache1.exists(KEY));
        cache1.cleanUp();
        cache1.clear();
    }

    /**
     * SimpleCache is reserved for internal use
     * <p>
     * Please DO NOT use it at application level
     */
    @Test
    public void simpleCacheTest() {
        String KEY = "key1";
        String DATA = "hello";
        long expiry = cache2.getExpiry();
        Assert.assertEquals(15000, expiry);
        cache2.put(KEY, DATA);
        Object o = cache2.get(KEY);
        Assert.assertEquals(DATA, o);
        long n = cache2.size();
        Assert.assertEquals(1, n);
        cache2.remove(KEY);
        Object o2 = cache2.get(KEY);
        Assert.assertNull(o2);
        // test clean up
        cache2.put(KEY, DATA);
        cache2.remove(KEY);
        Assert.assertFalse(cache2.exists(KEY));
        cache2.cleanUp();
        cache2.clear();
    }

}
