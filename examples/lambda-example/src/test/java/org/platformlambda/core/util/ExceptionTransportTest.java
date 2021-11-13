package org.platformlambda.core.util;

import org.junit.Assert;
import org.junit.BeforeClass;
import org.junit.Test;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.services.HelloPoJo;

import java.io.IOException;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;

public class ExceptionTransportTest {

    private static final String ROUTE = "exception.test";
    private static final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);

    @BeforeClass
    public static void setup() throws IOException {
        String ROUTE = "exception.test";
        Platform platform = Platform.getInstance();
        platform.registerPrivate(ROUTE, new HelloPoJo(), 1);
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
            Assert.assertEquals("Missing parameter 'id'", cause.getMessage());
        }
    }

    @Test
    public void callbackExceptionTest() throws IOException, InterruptedException {
        Platform platform = Platform.getInstance();
        String CALLBACK = "callback.function";
        platform.registerPrivate(CALLBACK, new MyCallBack(), 1);
        EventEnvelope request = new EventEnvelope();
        request.setTo(ROUTE);
        request.setReplyTo(CALLBACK);
        request.setBody("ok");
        PostOffice po = PostOffice.getInstance();
        po.send(request);
        EventEnvelope result = bench.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertNotNull(result.getException());
        Assert.assertTrue(result.getException() instanceof IllegalArgumentException);
        Assert.assertEquals("Missing parameter 'id'", result.getException().getMessage());
    }

    private static class MyCallBack implements TypedLambdaFunction<EventEnvelope, Object> {

        @Override
        public Object handleEvent(Map<String, String> headers, EventEnvelope body, int instance) throws Exception {
            bench.offer(body);
            return true;
        }
    }
}
