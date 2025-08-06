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

package org.platformlambda.http;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.platformlambda.cloud.reporter.PresenceConnector;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;
import org.platformlambda.mock.TestBase;
import org.platformlambda.util.SimpleHttpRequests;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.io.InputStream;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeoutException;
import java.util.concurrent.atomic.AtomicBoolean;

public class AdminEndpointTest extends TestBase {
    private static final Logger log = LoggerFactory.getLogger(AdminEndpointTest.class);

    private static final String CLOUD_CONNECTOR_HEALTH = "cloud.connector.health";

    private static final AtomicBoolean firstRun = new AtomicBoolean(true);

    @BeforeEach
    public void waitForMockCloud() {
        if (firstRun.get()) {
            firstRun.set(false);
            Platform platform = Platform.getInstance();
            try {
                platform.waitForProvider(CLOUD_CONNECTOR_HEALTH, 20);
                log.info("Mock cloud ready");
                waitForConnector();
            } catch (TimeoutException e) {
                log.error("{} not ready - {}", CLOUD_CONNECTOR_HEALTH, e.getMessage());
            }
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
    public void infoEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info");
        Assertions.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assertions.assertEquals("presence-monitor", multi.getElement("app.name"));
        Assertions.assertEquals("RESOURCES", multi.getElement("personality"));
        String origin = Platform.getInstance().getOrigin();
        Assertions.assertEquals(origin, multi.getElement("origin"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteInfoEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info", headers));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void libEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/lib");
        Assertions.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assertions.assertEquals("presence-monitor", multi.getElement("app.name"));
        Assertions.assertTrue(result.containsKey("library"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteLibEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/lib", headers));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void routeEndpointNotAvailableTest() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/routes"));
        Assertions.assertEquals(400, ex.getStatus());
        Assertions.assertEquals("Routing table is not visible from a presence monitor - " +
                "please try it from a regular application instance", ex.getMessage());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteRouteEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/routes", headers));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void healthEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/health");
        Assertions.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assertions.assertEquals("UP", result.get("status"));
        MultiLevelMap map = new MultiLevelMap(result);
        Assertions.assertEquals("mock-cloud", map.getElement("upstream[0].service"));
        log.info("health report: {}", result);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteHealthEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/health", headers));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @Test
    public void livenessEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/livenessprobe", "text/plain");
        Assertions.assertEquals("OK", response);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteLivenessEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/livenessprobe", headers));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void envEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/env");
        Assertions.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assertions.assertEquals("presence-monitor", multi.getElement("app.name"));
        Assertions.assertTrue(result.get("env") instanceof Map);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteEnvEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/env", headers));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownUsingGetWillFail() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/shutdown/now"));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendUsingGetWillFail() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/suspend/now"));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeUsingGetWillFail() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/resume/now"));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownWithoutAppInstanceWillFail() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.post("http://127.0.0.1:"+port+"/shutdown", new HashMap<>(), new HashMap<>()));
        Assertions.assertEquals(400, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(400, result.get("status"));
        Assertions.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownWithIncorrectAppInstanceWillFail() {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.post("http://127.0.0.1:"+port+"/shutdown", headers, new HashMap<>()));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendWithoutAppInstanceWillFail() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend/now", new HashMap<>(), new HashMap<>()));
        Assertions.assertEquals(400, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(400, result.get("status"));
        Assertions.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendWithIncorrectAppInstanceWillFail() {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend/now", headers, new HashMap<>()));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeWithoutAppInstanceWillFail() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume/now", new HashMap<>(), new HashMap<>()));
        Assertions.assertEquals(400, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(400, result.get("status"));
        Assertions.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeWithIncorrectAppInstanceWillFail() {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume/now", headers, new HashMap<>()));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendAppInstanceOK() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        Object response = SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend/now", headers, new HashMap<>());
        Assertions.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assertions.assertEquals(200, result.get("status"));
        Assertions.assertEquals("suspend", result.get("type"));
        Assertions.assertEquals("/suspend/now", result.get("path"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeAppInstanceOK() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        Object response = SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume/now", headers, new HashMap<>());
        Assertions.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assertions.assertEquals(200, result.get("status"));
        Assertions.assertEquals("resume", result.get("type"));
        Assertions.assertEquals("/resume/now", result.get("path"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void pageNotExists() throws AppException, IOException {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_such_page"));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void pageNotExistsXml() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_such_page", "application/xml"));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void pageNotExistsHtml() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_such_page", "text/html"));
        Assertions.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        // text/html is not supported in minimalist HTTP server
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("Resource not found", result.get("message"));
    }

}
