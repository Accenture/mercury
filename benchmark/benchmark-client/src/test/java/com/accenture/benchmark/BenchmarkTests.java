/*

    Copyright 2018-2023 Accenture Technology

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

package com.accenture.benchmark;

import io.vertx.core.Future;
import org.junit.Assert;
import org.junit.BeforeClass;
import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.AppStarter;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.Platform;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.text.NumberFormat;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class BenchmarkTests {
    private static final Logger log = LoggerFactory.getLogger(BenchmarkTests.class);

    @BeforeClass
    public static void setup() throws InterruptedException {
        AppStarter.main(new String[0]);
        Platform.getInstance().connectToCloud();
        waitForProvider();
    }

    private static void waitForProvider() throws InterruptedException {
        int WAIT_INTERVAL = 5;
        int ATTEMPTS = 20;
        String RECEIVE_ONLY = "network.one.way";
        String TWO_WAY = "network.echo";
        EventEmitter po = EventEmitter.getInstance();
        int n1 = 0;
        while (!po.exists(RECEIVE_ONLY)) {
            if (++n1 > ATTEMPTS) {
                log.error("Benchmark server not ready after {} seconds - Did you start the server?", ATTEMPTS * 5);
                break;
            }
            log.info("Waiting for {} to get ready...{}", RECEIVE_ONLY, n1);
            Thread.sleep(WAIT_INTERVAL * 1000);

        }
        int n2 = 0;
        while (!po.exists(TWO_WAY)) {
            if (++n2 > ATTEMPTS) {
                log.error("Benchmark server not ready after {} seconds - Did you start the server?", ATTEMPTS * 5);
                break;
            }
            log.info("Waiting for {} to get ready...{}", TWO_WAY, n2);
            Thread.sleep(WAIT_INTERVAL * 1000);
        }
    }

    @Test
    public void smallPayloadOneWayTest() throws IOException, InterruptedException {
        String RECEIVE_ONLY = "network.one.way";
        Map<String, Object> payload = new HashMap<>();
        payload.put("key", "value");
        EventEnvelope event = new EventEnvelope().setTo(RECEIVE_ONLY).setBody(payload);
        byte[] data = event.toBytes();

        Integer CYCLE = 10;
        String TYPE = "Small payload one-way";
        float min = Float.MAX_VALUE;
        float max = 0;
        for (int i=0; i < 10; i++) {
            float timeSpent = oneCycle(RECEIVE_ONLY, CYCLE, TYPE, i, payload);
            if (timeSpent > -1.0f) {
                if (timeSpent < min) {
                    min = timeSpent;
                }
                if (timeSpent > max) {
                    max = timeSpent;
                }
            }
        }
        NumberFormat number = NumberFormat.getInstance();
        log.info("{} of {} bytes - min {}, max {} events/second", TYPE, number.format(data.length),
                number.format(min), number.format(max));
    }

    @Test
    public void mediumPayloadOneWayTest() throws IOException, InterruptedException {
        String RECEIVE_ONLY = "network.one.way";
        StringBuilder sb = new StringBuilder();
        for (int i=0; i < 1000; i++) {
            sb.append("123456789.");
        }
        Map<String, Object> payload = new HashMap<>();
        payload.put("key", sb.toString());
        EventEnvelope event = new EventEnvelope().setTo(RECEIVE_ONLY).setBody(payload);
        byte[] data = event.toBytes();

        Integer CYCLE = 10;
        String TYPE = "Medium payload one-way";
        float min = Float.MAX_VALUE;
        float max = 0;
        for (int i=0; i < 10; i++) {
            float timeSpent = oneCycle(RECEIVE_ONLY, CYCLE, TYPE, i, payload);
            if (timeSpent > -1.0f) {
                if (timeSpent < min) {
                    min = timeSpent;
                }
                if (timeSpent > max) {
                    max = timeSpent;
                }
            }
        }
        NumberFormat number = NumberFormat.getInstance();
        log.info("{} of {} bytes - min {}, max {} events/second", TYPE, number.format(data.length),
                number.format(min), number.format(max));
    }

    @Test
    public void largePayloadOneWayTest() throws IOException, InterruptedException {
        String RECEIVE_ONLY = "network.one.way";
        StringBuilder sb = new StringBuilder();
        for (int i=0; i < 10000; i++) {
            sb.append("123456789.");
        }
        Map<String, Object> payload = new HashMap<>();
        payload.put("key", sb.toString());
        EventEnvelope event = new EventEnvelope().setTo(RECEIVE_ONLY).setBody(payload);
        byte[] data = event.toBytes();

        Integer CYCLE = 10;
        String TYPE = "Large payload one-way";
        float min = Float.MAX_VALUE;
        float max = 0;
        for (int i=0; i < 10; i++) {
            float timeSpent = oneCycle(RECEIVE_ONLY, CYCLE, TYPE, i, payload);
            if (timeSpent > -1.0f) {
                if (timeSpent < min) {
                    min = timeSpent;
                }
                if (timeSpent > max) {
                    max = timeSpent;
                }
            }
        }
        NumberFormat number = NumberFormat.getInstance();
        log.info("{} of {} bytes - min {}, max {} events/second", TYPE, number.format(data.length),
                number.format(min), number.format(max));
    }

    @Test
    public void smallPayloadTwoWayTest() throws IOException, InterruptedException {
        String TWO_WAY = "network.echo";
        Map<String, Object> payload = new HashMap<>();
        payload.put("key", "value");
        EventEnvelope event = new EventEnvelope().setTo(TWO_WAY).setBody(payload);
        byte[] data = event.toBytes();

        Integer CYCLE = 10;
        String TYPE = "Small payload 2-way";
        float min = Float.MAX_VALUE;
        float max = 0;
        for (int i=0; i < 10; i++) {
            float timeSpent = oneCycle(TWO_WAY, CYCLE, TYPE, i, payload);
            if (timeSpent > -1.0f) {
                if (timeSpent < min) {
                    min = timeSpent;
                }
                if (timeSpent > max) {
                    max = timeSpent;
                }
            }
        }
        NumberFormat number = NumberFormat.getInstance();
        log.info("{} of {} bytes - min {}, max {} events/second", TYPE, number.format(data.length),
                number.format(min), number.format(max));
    }

    @Test
    public void mediumPayloadTwoWayTest() throws IOException, InterruptedException {
        String TWO_WAY = "network.echo";
        StringBuilder sb = new StringBuilder();
        for (int i=0; i < 1000; i++) {
            sb.append("123456789.");
        }
        Map<String, Object> payload = new HashMap<>();
        payload.put("key", sb.toString());
        EventEnvelope event = new EventEnvelope().setTo(TWO_WAY).setBody(payload);
        byte[] data = event.toBytes();

        Integer CYCLE = 10;
        String TYPE = "Medium payload 2-way";
        float min = Float.MAX_VALUE;
        float max = 0;
        for (int i=0; i < 10; i++) {
            float timeSpent = oneCycle(TWO_WAY, CYCLE, TYPE, i, payload);
            if (timeSpent > -1.0f) {
                if (timeSpent < min) {
                    min = timeSpent;
                }
                if (timeSpent > max) {
                    max = timeSpent;
                }
            }
        }
        NumberFormat number = NumberFormat.getInstance();
        log.info("{} of {} bytes - min {}, max {} events/second", TYPE, number.format(data.length),
                number.format(min), number.format(max));
    }

    @Test
    public void largePayloadTwoWayTest() throws IOException, InterruptedException {
        String TWO_WAY = "network.echo";
        StringBuilder sb = new StringBuilder();
        for (int i=0; i < 10000; i++) {
            sb.append("123456789.");
        }
        Map<String, Object> payload = new HashMap<>();
        payload.put("key", sb.toString());
        EventEnvelope event = new EventEnvelope().setTo(TWO_WAY).setBody(payload);
        byte[] data = event.toBytes();

        Integer CYCLE = 10;
        String TYPE = "Large payload 2-way";
        float min = Float.MAX_VALUE;
        float max = 0;
        for (int i=0; i < 10; i++) {
            float timeSpent = oneCycle(TWO_WAY, CYCLE, TYPE, i, payload);
            if (timeSpent > -1.0f) {
                if (timeSpent < min) {
                    min = timeSpent;
                }
                if (timeSpent > max) {
                    max = timeSpent;
                }
            }
        }
        NumberFormat number = NumberFormat.getInstance();
        log.info("{} of {} bytes - min {}, max {} events/second", TYPE, number.format(data.length),
                number.format(min), number.format(max));
    }

    private float oneCycle(String target, Integer cycle, String type, int n, Map<String, Object> payload)
            throws IOException, InterruptedException {
        long start = System.currentTimeMillis();
        long TIMEOUT = 30000;
        BlockingQueue<Integer> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope event = new EventEnvelope().setTo(target).setBody(payload);
        List<EventEnvelope> requests = new ArrayList<>();
        for (int i=0; i < cycle; i++) {
            requests.add(event);
        }
        Future<List<EventEnvelope>> responses = po.asyncRequest(requests, TIMEOUT);
        responses.onSuccess(items -> bench.offer(items.size()));

        Integer value = bench.poll(TIMEOUT, TimeUnit.MILLISECONDS);
        long diff = System.currentTimeMillis() - start;
        Assert.assertEquals(cycle, value);
        // ignore the first 2 cycles as JVM needs time to load objects
        NumberFormat number = NumberFormat.getInstance();
        if (n > 1) {
            float timeSpent = (float) 1000 / diff * cycle;
            log.info("{} - {} events in {} ms or {} events/second", type, number.format(cycle), number.format(diff),
                    number.format(timeSpent));
            return timeSpent;
        } else {
            return -1.0f;
        }

    }

}
