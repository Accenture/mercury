package org.platformlambda.core.util;

import org.junit.Assert;
import org.junit.BeforeClass;
import org.junit.Test;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;

import java.io.IOException;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;

public class ExceptionTransportTest {

    private static final String ROUTE = "exception.test";
    private static final String CALLBACK = "callback.function";
    private static final String DEMO = "demo";
    private static final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);

    @BeforeClass
    public static void setup() throws IOException {
        Platform platform = Platform.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            throw new IllegalArgumentException("demo");
        };
        platform.registerPrivate(ROUTE, f, 1);
    }

    @Test
    public void transportTest() throws IOException, TimeoutException, InterruptedException {
        PostOffice po = PostOffice.getInstance();
        try {
            po.request(ROUTE, 5000, "demo");
            throw new IOException("Event exception transport failed");
        } catch (AppException e) {
            Throwable cause = e.getCause();
            Assert.assertTrue(cause instanceof IllegalArgumentException);
            Assert.assertEquals(DEMO, cause.getMessage());
        }
    }

    @Test
    public void callbackExceptionTest() throws IOException, InterruptedException {
        Platform platform = Platform.getInstance();
        platform.registerPrivate(CALLBACK, new MyCallBack(), 1);
        EventEnvelope request = new EventEnvelope();
        request.setTo(ROUTE);
        request.setReplyTo(CALLBACK);
        request.setBody("ok");
        PostOffice po = PostOffice.getInstance();
        po.send(request);
        EventEnvelope result = bench.poll(10, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertNotNull(result.getException());
        Assert.assertTrue(result.getException() instanceof IllegalArgumentException);
        Assert.assertEquals(DEMO, result.getException().getMessage());
    }

    private static class MyCallBack implements TypedLambdaFunction<EventEnvelope, Object> {

        @Override
        public Object handleEvent(Map<String, String> headers, EventEnvelope body, int instance) {
            bench.offer(body);
            return true;
        }
    }
}
