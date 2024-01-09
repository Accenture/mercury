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

package org.platformlambda.core;

import org.junit.Assert;
import org.junit.BeforeClass;
import org.junit.Test;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.Platform;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Map;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;

public class MulticastTest {
    private static final Logger log = LoggerFactory.getLogger(MulticastTest.class);
    private static final String MY_ROUTE = "my_route";
    private static final int WAIT_INTERVAL = 300;
    @BeforeClass
    public static void setup() throws InterruptedException {
        // The multicast.yaml configuration will be loaded when the EventEmitter singleton initializes
        EventEmitter po = EventEmitter.getInstance();
        log.info("Unit test loaded with {}. Multicast ready? {}", po, po.isMulticastEnabled());
        int n = 0;
        while (!po.isMulticastEnabled()) {
            Thread.sleep(WAIT_INTERVAL);
            n++;
            log.info("Waiting for multicast engine to get ready. Elapsed {} ms", n * WAIT_INTERVAL);
        }
    }

    @Test
    public void routingTest() throws IOException, InterruptedException {
        final EventEmitter po = EventEmitter.getInstance();
        final String[] targets = {"v1.hello.service.1", "v1.hello.service.2"};
        final String TEXT = "ok";
        final AtomicInteger counter = new AtomicInteger(0);
        final BlockingQueue<Boolean> completion = new ArrayBlockingQueue<>(1);
        final ConcurrentMap<String, Object> result = new ConcurrentHashMap<>();
        LambdaFunction f = (headers, input, instance) -> {
            String myRoute = headers.get(MY_ROUTE);
            result.put(myRoute, input);
            if (counter.incrementAndGet() == 2) {
                completion.offer(true);
            }
            return true;
        };
        Platform platform = Platform.getInstance();
        final BlockingQueue<Boolean> bench = new ArrayBlockingQueue<>(1);
        platform.waitForProvider("v1.hello.world", 5).onSuccess(bench::offer);
        boolean available = Boolean.TRUE.equals(bench.poll(5, TimeUnit.SECONDS));
        Assert.assertTrue(available);
        for (String t: targets) {
            platform.registerPrivate(t, f, 1);
        }
        // Event targeted to v1.hello.world will be multicasted to v1.hello.service.1 and v1.hello.service.2
        po.send("v1.hello.world", TEXT);
        completion.poll(5, TimeUnit.SECONDS);
        Assert.assertEquals(2, result.size());
        for (Map.Entry<String, Object> kv: result.entrySet()) {
            Assert.assertEquals(TEXT, kv.getValue());
            log.info("Result from {} is correct", kv.getKey());
        }
    }

}
