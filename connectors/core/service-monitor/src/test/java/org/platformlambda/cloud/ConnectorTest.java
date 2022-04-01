/*

    Copyright 2018-2022 Accenture Technology

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
import org.junit.Test;
import org.platformlambda.MainApp;
import org.platformlambda.cloud.reporter.PresenceConnector;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.websocket.client.PersistentWsClient;
import org.platformlambda.mock.MockCloud;
import org.platformlambda.mock.TestBase;
import org.platformlambda.models.WsMetadata;
import org.platformlambda.util.SimpleHttpRequests;
import org.platformlambda.ws.MonitorService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeoutException;

public class ConnectorTest extends TestBase {
    private static final Logger log = LoggerFactory.getLogger(ConnectorTest.class);

    private static final String TYPE = "type";
    private static final String DOWNLOAD = "download";
    private static final String ORIGIN = "origin";
    private static final String PUT = "put";
    private static final String MULTIPLES = "multiples";
    private static final String ALIVE = "keep-alive";
    private static final String TOPIC = "topic";
    private static final String ROUTE = ServiceDiscovery.ROUTE;
    private static final String SUBSCRIBE_LIFE_CYCLE = "subscribe_life_cycle";
    private static final String CONNECTION_LIFE_CYCLE = "connection.life.cycle";

    @SuppressWarnings("unchecked")
    @Test
    public void connectivityTest() throws AppException, IOException, TimeoutException {
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        platform.waitForProvider(ServiceDiscovery.SERVICE_REGISTRY, 10000);
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
        n = 0;
        while (n++ < 30) {
            Map<String, String> headers = new HashMap<>();
            headers.put("x-app-instance", Platform.getInstance().getOrigin());
            Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info",
                    "application/json", headers);
            Assert.assertTrue(response instanceof String);
            Map<String, Object> map = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
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
        po.request(MainApp.PRESENCE_HOUSEKEEPER, 5000,
                new Kv(TYPE, MainApp.MONITOR_ALIVE), new Kv(ORIGIN, origin));
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/health");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> map = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("UP", map.get("status"));
        response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info");
        Assert.assertTrue(response instanceof String);
        map = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
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
        // stop current session
        MockCloud.stopWsClient();
        log.info("Waiting for member to close");
        n = 0;
        while (MonitorService.getSessionCount() > 0 && ++n < 30) {
            log.info("Waiting for member to close ... {}", n);
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                // ok to ignore
            }
        }
        MockCloud.restartWsClient();
        log.info("Waiting for member to re-connect");
        n = 0;
        while (MonitorService.getSessionCount() == 0 && ++n < 30) {
            log.info("Waiting for member to re-connect ... {}", n);
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                // ok to ignore
            }
        }
        log.info("Member count = {}", MonitorService.getSessionCount());
    }

}
