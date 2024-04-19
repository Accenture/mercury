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

package org.platformlambda.cloud;

import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.platformlambda.MainApp;
import org.platformlambda.cloud.reporter.PresenceConnector;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.mock.TestBase;
import org.platformlambda.models.WsMetadata;
import org.platformlambda.ws.MonitorService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;

public class ConnectorTest extends TestBase {
    private static final Logger log = LoggerFactory.getLogger(ConnectorTest.class);

    private static final String TYPE = "type";
    private static final String DOWNLOAD = "download";
    private static final String ORIGIN = "origin";
    private static final String PUT = "put";
    private static final String MULTIPLES = "multiples";
    private static final String ALIVE = "keep-alive";
    private static final String TOPIC = "topic";
    private static final String CLOUD_CONNECTOR_HEALTH = "cloud.connector.health";
    private static final AtomicBoolean firstRun = new AtomicBoolean(true);

    @Before
    public void waitForMockCloud() throws InterruptedException {
        if (firstRun.get()) {
            firstRun.set(false);
            final int WAIT = 20;
            final BlockingQueue<Boolean> bench = new ArrayBlockingQueue<>(1);
            Platform platform = Platform.getInstance();

            List<String> providers = new ArrayList<>();
            providers.add(CLOUD_CONNECTOR_HEALTH);
            providers.add(ServiceDiscovery.SERVICE_REGISTRY);
            platform.waitForProviders(providers, WAIT).onSuccess(bench::offer);
            Boolean success = bench.poll(WAIT, TimeUnit.SECONDS);
            if (Boolean.TRUE.equals(success)) {
                log.info("Mock cloud ready");
            }
            waitForConnector();
        }
    }

    private void waitForConnector() {
        boolean ready = false;
        PresenceConnector connector = PresenceConnector.getInstance();
        for (int i=0; i < 20; i++) {
            if (connector.isConnected() && connector.isReady()) {
                ready = true;
                break;
            }
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        }
        if (ready) {
            log.info("Cloud connection ready");
        } else {
            log.error("Cloud connection not ready");
        }
    }

    @SuppressWarnings("unchecked")
    @Test
    public void connectivityTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        int n = 0;
        while (!PresenceHandler.isReady() || MonitorService.getConnections().isEmpty() && ++n < 20) {
            log.info("Waiting for member connection... {}", n);
            try {
                Thread.sleep(5000);
            } catch (InterruptedException e) {
                // ok to ignore
            }
        }
        Map<String, Object> connections = MonitorService.getConnections();
        if (connections.isEmpty()) {
            throw new IllegalArgumentException("Should have at least one member connection");
        }
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        headers.put("x-app-instance", Platform.getInstance().getOrigin());
        n = 0;
        while (n++ < 30) {
            EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/info", headers);
            Assert.assertTrue(response.getBody() instanceof Map);
            Map<String, Object> map = (Map<String, Object>) response.getBody();
            Object info = map.get("additional_info");
            Assert.assertTrue(info instanceof Map);
            Map<String, Object> infoMap = (Map<String, Object>) info;
            Object tt = infoMap.get("topics");
            Assert.assertTrue(tt instanceof List);
            List<String> topicList = (List<String>) tt;
            Assert.assertEquals(2, topicList.size());
            Object vt = infoMap.get("virtual_topics");
            Assert.assertTrue(vt instanceof List);
            List<String> vtList = (List<String>) vt;
            if (vtList.isEmpty()) {
                log.info("Waiting for first active member... {}", n);
                try {
                    Thread.sleep(3000);
                } catch (InterruptedException e) {
                    // ok to ignore
                }
            } else {
                Assert.assertTrue(vtList.get(0).startsWith("multiplex.0001-000 (user.topic.one) ->"));
                po.send(ServiceDiscovery.SERVICE_REGISTRY,
                        new Kv(TYPE, ALIVE), new Kv(TOPIC, "multiplex.0001-000"),
                        new Kv(ORIGIN, Platform.getInstance().getOrigin()));
                break;
            }
        }
        PubSub ps = PubSub.getInstance();
        ps.createTopic("multiplex.0001", 32);
        ps.createTopic("multiplex.0002", 32);
        ps.createTopic("service.monitor", 10);
        // simulate a peer presence monitor
        String origin = "test-monitor";
        po.send(MainApp.PRESENCE_HANDLER,
                new Kv(TYPE, DOWNLOAD), new Kv(ORIGIN, origin));
        po.send(MainApp.PRESENCE_HANDLER, connections,
                new Kv(TYPE, PUT), new Kv(ORIGIN, origin), new Kv(MULTIPLES, true));
        // use RPC to wait for completion
        EventEnvelope req = new EventEnvelope().setTo(MainApp.PRESENCE_HOUSEKEEPER)
                .setHeader(TYPE, MainApp.MONITOR_ALIVE).setHeader(ORIGIN, origin);
        po.asyncRequest(req, 5000).onSuccess(bench::offer);
        bench.poll(10, TimeUnit.SECONDS);
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/health", headers);
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> map = (Map<String, Object>) response.getBody();
        Assert.assertEquals("UP", map.get("status"));
        response = httpGet("http://127.0.0.1:"+port, "/info", headers);
        Assert.assertTrue(response.getBody() instanceof Map);
        map = (Map<String, Object>) response.getBody();
        Object info = map.get("additional_info");
        Assert.assertTrue(info instanceof Map);
        Map<String, Object> infoMap = (Map<String, Object>) info;
        Object monitorList = infoMap.get("monitors");
        Assert.assertTrue(monitorList instanceof List);
        String monitors = monitorList.toString();
        Assert.assertTrue(monitors.contains("test-monitor"));
        Assert.assertTrue(monitors.contains(Platform.getInstance().getOrigin()));
        Map<String, WsMetadata> sessions = MonitorService.getSessions();
        Assert.assertEquals(1, sessions.size());
    }

}
