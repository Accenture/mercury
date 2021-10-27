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

package org.platformlambda.automation.tests;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.automation.mock.TestBase;
import org.platformlambda.automation.services.SimpleNotification;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.websocket.client.PersistentWsClient;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;
import java.util.concurrent.atomic.AtomicBoolean;

public class WebSocketTests extends TestBase {
    private static final Logger log = LoggerFactory.getLogger(WebSocketTests.class);

    private static final String TYPE = "type";
    private static final String TOKEN = "token";
    private static final String TX_PATH = "tx_path";

    private static final Map<String, Object> store = new HashMap<>();

    @SuppressWarnings("unchecked")
    @Test
    public void userChannelTest() throws IOException, AppException, TimeoutException, InterruptedException {
        BlockingQueue<Boolean> matched = new ArrayBlockingQueue<>(1);
        BlockingQueue<Boolean> completion = new ArrayBlockingQueue<>(1);
        String PUBLISH_MESSAGE = "1234567890";
        String USER_MESSAGE = "user-message";
        AtomicBoolean subscribed = new AtomicBoolean(false);
        AtomicBoolean published = new AtomicBoolean(false);
        AtomicBoolean messaged = new AtomicBoolean(false);
        Platform platform = Platform.getInstance();
        LambdaFunction listener = (headers, body, instance) -> {
            if (USER_MESSAGE.equals(body)) {
                messaged.set(true);
            }
            return true;
        };
        platform.register("my.ws.handler", listener, 1);
        PostOffice po = PostOffice.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setUrl("/api/ws/token/notification");
        req.setTargetHost("http://127.0.0.1:"+port);
        req.setHeader("accept", "application/json");
        req.setHeader("content-type", "application/json");
        EventEnvelope res = po.request(HTTP_REQUEST, 5000, req.toMap());
        Assert.assertTrue(res.getBody() instanceof Map);
        Map<String, Object> data = (Map<String, Object>) res.getBody();
        Assert.assertTrue(data.containsKey(TOKEN));
        String token = (String) data.get(TOKEN);
        List<String> target = Collections.singletonList("ws://127.0.0.1:"+port+"/ws/notification/"+token);
        LambdaFunction f = (headers, body, instance) -> {
            log.info("GOT headers: {} payload: {}", headers, body);
            String type = headers.get(TYPE);
            if ("open".equals(type)) {
                String txPath = headers.get(TX_PATH);
                Map<String, Object> subscribe = new HashMap<>();
                subscribe.put("type", "subscribe");
                subscribe.put("topic", "hello.world");
                po.send(txPath, subscribe);
                Map<String, Object> publish = new HashMap<>();
                publish.put("type", "publish");
                publish.put("topic", "hello.world");
                publish.put("message", PUBLISH_MESSAGE);
                po.send(txPath, publish);
                po.send(txPath, USER_MESSAGE);
                Map<String, Object> subscribeMore = new HashMap<>();
                subscribeMore.put("type", "subscribe");
                subscribeMore.put("topic", "another.channel");
                po.send(txPath, subscribeMore);
            }
            if ("string".equals(type)) {
                if (PUBLISH_MESSAGE.equals(body)) {
                    matched.offer(true);
                } else {
                    String text = (String) body;
                    if (text.startsWith("{") && text.endsWith("}")) {
                        Map<String, Object> response = SimpleMapper.getInstance().getMapper().readValue(text, Map.class);
                        if ("subscribe".equals(response.get(TYPE))) {
                            subscribed.set(true);
                        }
                        if ("publish".equals(response.get(TYPE))) {
                            published.set(true);
                        }
                    }
                }
            }
            if ("close".equals(type)) {
                completion.offer(true);
            }
            return true;
        };
        PersistentWsClient client = new PersistentWsClient(f, target);
        client.start();
        Boolean result = matched.poll(20, TimeUnit.SECONDS);
        Assert.assertEquals(true, result);
        Assert.assertTrue(subscribed.get());
        Assert.assertTrue(published.get());
        Assert.assertTrue(messaged.get());
        AsyncHttpRequest check1 = new AsyncHttpRequest();
        check1.setMethod("GET");
        check1.setUrl("/api/notification");
        check1.setTargetHost("http://127.0.0.1:"+port);
        check1.setHeader("accept", "application/json");
        EventEnvelope resCheck1 = po.request(HTTP_REQUEST, 5000, check1.toMap());
        Assert.assertTrue(resCheck1.getBody() instanceof Map);
        Map<String, Object> checkResult1 = (Map<String, Object>) resCheck1.getBody();
        Assert.assertTrue(checkResult1.containsKey("topics"));
        Assert.assertTrue(checkResult1.get("topics") instanceof List);
        List<String> subscription = (List<String>) checkResult1.get("topics");
        Assert.assertTrue(subscription.contains("hello.world"));
        AsyncHttpRequest check2 = new AsyncHttpRequest();
        check2.setMethod("GET");
        check2.setUrl("/api/notification/hello.world");
        check2.setTargetHost("http://127.0.0.1:"+port);
        check2.setHeader("accept", "application/json");
        EventEnvelope resCheck2 = po.request(HTTP_REQUEST, 5000, check2.toMap());
        Assert.assertTrue(resCheck2.getBody() instanceof Map);
        Map<String, Object> checkResult2 = (Map<String, Object>) resCheck2.getBody();
        Assert.assertTrue(checkResult2.containsKey("hello.world"));
        // backend service can publish directly to browser event channel
        SimpleNotification notifier = SimpleNotification.getInstance();
        notifier.publish("another.channel", "test message");
        client.close();
        completion.poll(10, TimeUnit.SECONDS);
    }

}
