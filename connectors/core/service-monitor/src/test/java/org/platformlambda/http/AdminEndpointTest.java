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

package org.platformlambda.http;

import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
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

    @Before
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
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("presence-monitor", multi.getElement("app.name"));
        Assert.assertEquals("RESOURCES", multi.getElement("personality"));
        String origin = Platform.getInstance().getOrigin();
        Assert.assertEquals(origin, multi.getElement("origin"));
    }

    @Test(expected = AppException.class)
    public void remoteInfoEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info", headers);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void libEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/lib");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("presence-monitor", multi.getElement("app.name"));
        Assert.assertTrue(result.containsKey("library"));
    }

    @Test(expected = AppException.class)
    public void remoteLibEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/lib", headers);
    }

    @Test(expected = AppException.class)
    public void routeEndpointNotAvailableTest() throws AppException, IOException {
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/routes");
    }

    @Test(expected = AppException.class)
    public void protectedRoutesEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://localhost:"+port+"/info/routes");
        log.error("This Route Report should not show because the it is expecting an exception - {}", response);
    }

    @Test(expected = AppException.class)
    public void remoteRouteEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/routes", headers);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void healthEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/health");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("UP", result.get("status"));
        MultiLevelMap map = new MultiLevelMap(result);
        Assert.assertEquals("mock-cloud", map.getElement("upstream[0].service"));
        log.info("health report: {}", result);
    }

    @Test(expected = AppException.class)
    public void remoteHealthEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/health", headers);
    }

    @Test
    public void livenessEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/livenessprobe", "text/plain");
        Assert.assertEquals("OK", response);
    }

    @Test(expected = AppException.class)
    public void remoteLivenessEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/livenessprobe", headers);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void envEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/env");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("presence-monitor", multi.getElement("app.name"));
        Assert.assertTrue(result.get("env") instanceof Map);
    }

    @Test(expected = AppException.class)
    public void remoteEnvEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/env", headers);
    }

    @Test(expected = AppException.class)
    public void shutdownUsingGetWillFail() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/shutdown/now");
        Assert.assertTrue(response instanceof String);
        SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
    }

    @Test(expected = AppException.class)
    public void suspendUsingGetWillFail() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/suspend/now");
        Assert.assertTrue(response instanceof String);
        SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
    }

    @Test(expected = AppException.class)
    public void resumeUsingGetWillFail() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/resume/now");
        Assert.assertTrue(response instanceof String);
        SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
    }

    @Test(expected = AppException.class)
    public void shutdownWithoutAppInstanceWillFail() throws AppException, IOException {
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/shutdown", new HashMap<>(), new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void shutdownWithIncorrectAppInstanceWillFail() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/shutdown", headers, new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void suspendWithoutAppInstanceWillFail() throws AppException, IOException {
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend/now", new HashMap<>(), new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void suspendWithIncorrectAppInstanceWillFail() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend/now", headers, new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void suspendWithOutAppInstanceWillFail() throws AppException, IOException {
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend/now", new HashMap<>(), new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void resumeWithoutAppInstanceWillFail() throws AppException, IOException {
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume/now", new HashMap<>(), new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void resumeWithIncorrectAppInstanceWillFail() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume/now", headers, new HashMap<>());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendAppInstanceOK() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        Object response = SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend/now", headers, new HashMap<>());
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals(200L, result.get("status"));
        Assert.assertEquals("ok", result.get("type"));
        Assert.assertEquals("/suspend/now", result.get("path"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeAppInstanceOK() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        Object response = SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume/now", headers, new HashMap<>());
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals(200L, result.get("status"));
        Assert.assertEquals("ok", result.get("type"));
        Assert.assertEquals("/resume/now", result.get("path"));
    }

    @Test
    public void getCssPage() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/sample.css");
        Assert.assertTrue(response instanceof String);
        String text = (String) response;
        InputStream in = this.getClass().getResourceAsStream("/public/sample.css");
        String css = Utility.getInstance().stream2str(in);
        Assert.assertEquals(css, text);
    }

    @Test
    public void getTextPage() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/sample.txt");
        Assert.assertTrue(response instanceof String);
        String text = (String) response;
        InputStream in = this.getClass().getResourceAsStream("/public/sample.txt");
        String txt = Utility.getInstance().stream2str(in);
        Assert.assertEquals(txt, text);
    }

    @Test
    public void getJsPage() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/sample.js");
        Assert.assertTrue(response instanceof String);
        String text = (String) response;
        InputStream in = this.getClass().getResourceAsStream("/public/sample.js");
        String js = Utility.getInstance().stream2str(in);
        Assert.assertEquals(js, text);
    }

    @Test(expected = AppException.class)
    public void pageNotExists() throws AppException, IOException {
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_such_page");
    }

    @Test(expected = AppException.class)
    public void pageNotExistsXml() throws AppException, IOException {
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_such_page", "application/xml");
    }

    @Test(expected = AppException.class)
    public void pageNotExistsHtml() throws AppException, IOException {
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_such_page", "text/html");
    }

}
