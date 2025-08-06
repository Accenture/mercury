package org.platformlambda.core;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;

import java.io.IOException;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class MulticastTest {
    private static final Logger log = LoggerFactory.getLogger(MulticastTest.class);

    private static final int WAIT_INTERVAL = 300;
    @BeforeAll
    public static void setup() throws InterruptedException {
        // The multicast.yaml configuration will be loaded when the EventEmitter singleton initializes
        PostOffice po = PostOffice.getInstance();
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
        final String[] targets = {"v1.hello.service.1", "v1.hello.service.2"};
        final PostOffice po = PostOffice.getInstance();
        final String TEXT = "ok";
        final AtomicInteger counter = new AtomicInteger(0);
        final BlockingQueue<Boolean> completion = new ArrayBlockingQueue<>(1);
        final ConcurrentMap<String, String> result = new ConcurrentHashMap<>();
        LambdaFunction f = (headers, body, instance) -> {
            result.put(po.getRoute(), (String) body);
            if (counter.incrementAndGet() == 2) {
                completion.offer(true);
            }
            return true;
        };
        Platform platform = Platform.getInstance();
        for (String t: targets) {
            platform.registerPrivate(t, f, 1);
        }
        po.send("v1.hello.world", TEXT);
        completion.poll(5, TimeUnit.SECONDS);
        Assertions.assertEquals(2, result.size());
        for (String t: targets) {
            Assertions.assertEquals(TEXT, result.get(t));
        }
    }

}
