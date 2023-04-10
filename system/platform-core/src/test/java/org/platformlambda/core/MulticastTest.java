package org.platformlambda.core;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.PostOffice;

import java.io.IOException;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;

public class MulticastTest {

    @Test
    public void routingTest() throws IOException, InterruptedException {
        final String[] targets = {"v1.hello.service.1", "v1.hello.service.2"};
        final EventEmitter emitter = EventEmitter.getInstance();
        final String TEXT = "ok";
        final AtomicInteger counter = new AtomicInteger(0);
        final BlockingQueue<Boolean> completion = new ArrayBlockingQueue<>(1);
        final ConcurrentMap<String, String> result = new ConcurrentHashMap<>();
        LambdaFunction f = (headers, input, instance) -> {
            PostOffice po = new PostOffice(headers, instance);
            result.put(po.getRoute(), (String) input);
            if (counter.incrementAndGet() == 2) {
                completion.offer(true);
            }
            return true;
        };
        Platform platform = Platform.getInstance();
        for (String t: targets) {
            platform.registerPrivate(t, f, 1);
        }
        emitter.send("v1.hello.world", TEXT);
        completion.poll(5, TimeUnit.SECONDS);
        Assert.assertEquals(2, result.size());
        for (String t: targets) {
            Assert.assertEquals(TEXT, result.get(t));
        }
    }

}
