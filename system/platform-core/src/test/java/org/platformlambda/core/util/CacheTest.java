/*

    Copyright 2018-2021 Accenture Technology

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

import org.junit.Assert;
import org.junit.Test;

public class CacheTest {

    // note that the cache expiry has a minimum value of 1000 ms
    private static final ManagedCache cache1 = ManagedCache.createCache("hello.world", 1000, 100);
    private static final SimpleCache cache2 = SimpleCache.createCache("simple.cache", 500);

    @Test
    public void cacheBehavior() throws InterruptedException {
        String KEY = "key1";
        String DATA = "hello";
        cache1.put(KEY, DATA);
        Object o = cache1.get(KEY);
        Assert.assertEquals(DATA, o);
        long n = cache1.size();
        Assert.assertEquals(1, n);
        // test expiry
        Thread.sleep(1010);
        // cached item will disappear in one second
        Object o2 = cache1.get(KEY);
        Assert.assertNull(o2);
        // test removal
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
    public void simpleCacheTest() throws InterruptedException {
        String KEY = "key1";
        String DATA = "hello";
        long expiry = cache2.getExpiry();
        // test minimum expiry to be one second
        Assert.assertEquals(1000, expiry);
        cache2.put(KEY, DATA);
        Object o = cache2.get(KEY);
        Assert.assertEquals(DATA, o);
        long n = cache2.size();
        Assert.assertEquals(1, n);
        cache2.remove(KEY);
        Object o2 = cache2.get(KEY);
        Assert.assertNull(o2);
        cache2.put(KEY, DATA);
        cache2.remove(KEY);
        Assert.assertFalse(cache2.exists(KEY));
        cache2.put(KEY, DATA);
        Assert.assertTrue(cache2.exists(KEY));
        Thread.sleep(1010);
        // test clean up
        cache2.cleanUp();
        Assert.assertFalse(cache2.exists(KEY));
        cache2.clear();
    }

}
