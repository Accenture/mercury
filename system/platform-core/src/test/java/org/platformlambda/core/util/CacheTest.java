package org.platformlambda.core.util;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.util.ManagedCache;

public class CacheTest {

    // note that the cache expiry has a minimum value of 1000 ms
    private static final ManagedCache cache = ManagedCache.createCache("hello.world", 1000, 100);

    @Test
    public void cacheBehavior() {
        String KEY = "key1";
        String DATA = "hello";
        cache.put(KEY, DATA);

        Object o = cache.get(KEY);
        Assert.assertEquals(DATA, o);

        try {
            Thread.sleep(1010);
        } catch (InterruptedException e) {
            // ok to ignore
        }
        // cached item will disappear in one second
        Object o2 = cache.get(KEY);
        Assert.assertNull(o2);
    }

}
