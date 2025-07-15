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

package org.platformlambda.kafka;

import org.junit.Assert;
import org.junit.BeforeClass;
import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.AppStarter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class KafkaAdapterTest {
    private static final Logger log = LoggerFactory.getLogger(KafkaAdapterTest.class);

    private static final String X_CONTENT_TYPE = "x-content-type";
    private static final String JSON = "json";
    private static final String EVENT = "event";
    private static final String TOPIC = "topic";
    private static boolean ready = false;

    @BeforeClass
    public static void setup() {
        ready = Utility.getInstance().portReady("127.0.0.1", 9092, 5000);
        if (ready) {
            AppStarter.main(new String[0]);
        }
    }

    @Test
    public void pubSubTest() throws IOException, InterruptedException {
        if (!ready) {
            log.error("Did you start a standalone Kafka server in this machine?");
            log.error("Unit test aborted");
            return;
        }
        Map<String, Object> map = new HashMap<>();
        map.put("hello", "world");
        Map<String, Object> map1 = new HashMap<>();
        map1.put("test", "message");
        BlockingQueue<Object> bench1 = new ArrayBlockingQueue<>(10);
        BlockingQueue<Object> bench2 = new ArrayBlockingQueue<>(10);
        LambdaFunction f1 = (headers, input, instance) -> {
            PostOffice po1 = new PostOffice(headers, instance);
            log.info("F1 received {} - {}", headers, input);
            Map<String, Object> content1 = new HashMap<>();
            content1.put("content", map1);
            po1.send("kafka.notification", content1, new Kv(TOPIC, "hello.notice"), new Kv(X_CONTENT_TYPE, JSON));
            bench1.offer(input);
            return null;
        };
        LambdaFunction f2 = (headers, input, instance) -> {
            log.info("F2 received {} - {}", headers, input);
            bench2.offer(input);
            return null;
        };
        Platform platform = Platform.getInstance();
        platform.registerPrivate("demo.listener", f1, 10);
        platform.registerPrivate("notice.listener", f2, 10);
        // give Kafka client a little bit of time to get ready
        Thread.sleep(2000);
        // case 1 - send JSON data to Kafka topic 'hello.world'
        PostOffice po = new PostOffice("unit.test", "T10000", "TEST /json");
        Map<String, Object> content = new HashMap<>();
        content.put("content", map);
        for (int i=0; i < 10; i++) {
            po.send("kafka.notification", content, new Kv(TOPIC, "hello.world"), new Kv(X_CONTENT_TYPE, JSON));
            Object result1 = bench1.poll(20, TimeUnit.SECONDS);
            log.info("From topic hello.world --> {}", result1);
            Assert.assertEquals(map, result1);
            Object result2 = bench2.poll(20, TimeUnit.SECONDS);
            log.info("From topic hello.notice --> {}", result2);
            Assert.assertEquals(map1, result2);
        }
        // try it again with x-content-type set to "event" so we can send an event envelope
        String message = "this is the message inside an event envelope";
        EventEnvelope req = new EventEnvelope();
        req.setHeader("hello", "world");
        req.setBody(message);
        content.put("content", req.toBytes());
        po.send("kafka.notification", content, new Kv(TOPIC, "hello.world"), new Kv(X_CONTENT_TYPE, EVENT));
        Object result3 = bench1.poll(20, TimeUnit.SECONDS);
        log.info("From topic hello.world --> {}", result3);
        Assert.assertEquals(message, result3);
        Object result4 = bench2.poll(20, TimeUnit.SECONDS);
        log.info("From topic hello.notice --> {}", result4);
        Assert.assertEquals(map1, result4);
    }
}
