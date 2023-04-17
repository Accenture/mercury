package com.accenture.tests;

import com.accenture.common.TestBase;
import io.vertx.core.Future;
import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.MultiLevelMap;

import java.io.IOException;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class GreetingTest extends TestBase {

    @SuppressWarnings("unchecked")
    @Test
    public void helloWorld() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        PostOffice po = new PostOffice("unit.test", "12345", "TEST /helloworld");
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setHeader("accept", "application/json");
        req.setUrl("/greeting");
        req.setTargetHost("http://127.0.0.1:"+springPort);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(RPC_TIMEOUT, TimeUnit.MILLISECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
        Assert.assertEquals("Hello, World", map.getElement("content.greeting"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void helloUser() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        PostOffice po = new PostOffice("unit.test", "24680", "TEST /helloworld");
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setHeader("accept", "application/json");
        req.setUrl("/greeting");
        req.setQueryParameter("name", "user");
        req.setTargetHost("http://127.0.0.1:"+springPort);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(RPC_TIMEOUT, TimeUnit.MILLISECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
        Assert.assertEquals("Hello, user", map.getElement("content.greeting"));
    }

}
