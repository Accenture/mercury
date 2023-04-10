package org.platformlambda.core;

import org.junit.Assert;
import org.junit.BeforeClass;
import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;

import java.io.IOException;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class ExceptionTransportTest {

    private static final String ROUTE = "exception.test";
    private static final String CALLBACK = "callback.function";
    private static final String DEMO = "demo";
    private static final BlockingQueue<EventEnvelope> callbackBench = new ArrayBlockingQueue<>(1);

    @BeforeClass
    public static void setup() throws IOException {
        Platform platform = Platform.getInstance();
        LambdaFunction f = (headers, input, instance) -> {
            throw new IllegalArgumentException("demo");
        };
        platform.registerPrivate(ROUTE, f, 1);
    }

    @Test
    public void transportTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope().setTo(ROUTE).setBody("demo");
        po.asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(400, response.getStatus());
        Assert.assertEquals(DEMO, response.getError());

    }

    @Test
    public void callbackExceptionTest() throws IOException, InterruptedException {
        Platform platform = Platform.getInstance();
        platform.registerPrivate(CALLBACK, new MyCallBack(), 1);
        EventEnvelope request = new EventEnvelope();
        request.setTo(ROUTE);
        request.setReplyTo(CALLBACK);
        request.setBody("ok");
        EventEmitter po = EventEmitter.getInstance();
        po.send(request);
        EventEnvelope result = callbackBench.poll(10, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertNotNull(result.getException());
        Assert.assertTrue(result.getException() instanceof IllegalArgumentException);
        Assert.assertEquals(DEMO, result.getException().getMessage());
    }

    private static class MyCallBack implements TypedLambdaFunction<EventEnvelope, Object> {

        @Override
        public Object handleEvent(Map<String, String> headers, EventEnvelope body, int instance) {
            callbackBench.offer(body);
            return true;
        }
    }
}
