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

package org.platformlambda.cloud;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.mock.TestBase;
import org.platformlambda.util.SimpleHttpRequests;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeoutException;

public class ConnectorTest extends TestBase {
    private static final Logger log = LoggerFactory.getLogger(ConnectorTest.class);

    private static final String NOTIFICATION_INTERNAL = "notification.manager.internal";

    @SuppressWarnings("unchecked")
    @Test
    public void connectivityTest() throws TimeoutException, AppException, IOException {
        LambdaFunction f1 = (headers, body, instance) -> {
            log.info("Notification receives {} {}", headers, body);
            return true;
        };
        String origin = "unit-test";
        Platform platform = Platform.getInstance();
        platform.waitForProvider("cloud.connector.health", 10);
        platform.register(NOTIFICATION_INTERNAL, f1, 1);
        PostOffice po = PostOffice.getInstance();
        String URL = "https://127.0.0.1";
        String SERVICE_NAME = "CloudConnector";
        ConnectorConfig.setDisplayUrl(URL);
        ConnectorConfig.setServiceName(SERVICE_NAME);
        String url = ConnectorConfig.getDisplayUrl();
        String name = ConnectorConfig.getServiceName();
        Map<String, String> topicSubstitution = ConnectorConfig.getTopicSubstitution();
        Assert.assertEquals(URL, url);
        Assert.assertEquals(SERVICE_NAME, name);
        Assert.assertEquals("user.topic.one", topicSubstitution.get("multiplex.0001.0"));
        po.request(ServiceDiscovery.SERVICE_REGISTRY, 5000,
                new Kv("type", "join"), new Kv("origin", origin), new Kv("topic", "multiplex.0001-001"));
        po.request(ServiceDiscovery.SERVICE_REGISTRY, 5000,
                new Kv("type", "join"), new Kv("origin", platform.getOrigin()), new Kv("topic", "multiplex.0001-000"));
        po.request(ServiceDiscovery.SERVICE_REGISTRY, 5000,
                new Kv("type", "add"), new Kv("origin", origin),
                new Kv("route", "hello.world"), new Kv("personality", "WEB"));
        Map<String, Object> routes = new HashMap<>();
        routes.put("hello.test", "WEB");
        routes.put("hello.demo", "WEB");
        routes.put("to.be.removed", "WEB");
        po.request(ServiceDiscovery.SERVICE_REGISTRY, 5000, routes,
                new Kv("type", "add"), new Kv("origin", origin),
                new Kv("personality", "WEB"));
        po.broadcast("hello.world", "something");
        po.send("hello.demo@"+origin, "something else");
        po.request(ServiceDiscovery.SERVICE_REGISTRY, 5000, routes,
                new Kv("type", "unregister"), new Kv("origin", origin),
                new Kv("route", "to.be.removed"));
        EventEnvelope queryResult = po.request(ServiceDiscovery.SERVICE_QUERY, 5000,
                new Kv("type", "search"), new Kv("route", "hello.world"));
        Assert.assertTrue(queryResult.getBody() instanceof List);
        List<String> instances = (List<String>) queryResult.getBody();
        Assert.assertTrue(instances.contains("unit-test"));
        Assert.assertTrue(instances.contains(platform.getOrigin()));
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/routes");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> info = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(info);
        Object nodes = multi.getElement("routing.nodes");
        Assert.assertTrue(nodes instanceof List);
        String nodeList = nodes.toString();
        Assert.assertTrue(nodeList.contains("unit-test"));
        Assert.assertTrue(po.exists("hello.demo"));
        queryResult = po.request(ServiceDiscovery.SERVICE_QUERY, 5000,
                Collections.singletonList("hello.world"), new Kv("type", "find"), new Kv("route", "*"));
        Assert.assertEquals(true, queryResult.getBody());
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        response = SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend/now", headers, new HashMap<>());
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals(200L, result.get("status"));
        Assert.assertEquals("ok", result.get("type"));
        Assert.assertEquals("/suspend/now", result.get("path"));
        response = SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume/now", headers, new HashMap<>());
        Assert.assertTrue(response instanceof String);
        result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals(200L, result.get("status"));
        Assert.assertEquals("ok", result.get("type"));
        Assert.assertEquals("/resume/now", result.get("path"));
        po.send(ServiceDiscovery.SERVICE_REGISTRY, new Kv("type", "leave"), new Kv("origin", origin));
    }

    @Test(expected = IOException.class)
    public void checkTopicNameWithoutDot() throws IOException {
        String name = "hello.world";
        ConnectorConfig.validateTopicName(name);
        String invalid = "helloworld";
        ConnectorConfig.validateTopicName(invalid);
    }

    @Test(expected = IOException.class)
    public void checkEmptyTopic() throws IOException {
        ConnectorConfig.validateTopicName("");
    }

    @Test(expected = IOException.class)
    public void reservedExtension() throws IOException {
        ConnectorConfig.validateTopicName("hello.com");
    }

    @Test(expected = IOException.class)
    public void reservedName() throws IOException {
        ConnectorConfig.validateTopicName("Thumbs.db");
    }

    @SuppressWarnings("unchecked")
    @Test
    public void healthTest() throws AppException, IOException, TimeoutException {
        Platform platform = Platform.getInstance();
        platform.waitForProvider("cloud.connector.health", 10);
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/health");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("UP", result.get("status"));
        Assert.assertEquals("cloud-connector", result.get("name"));
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals(200L, multi.getElement("upstream[0].status_code"));
    }
}
