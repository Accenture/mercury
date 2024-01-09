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

package org.platformlambda.automation;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.common.TestBase;
import org.platformlambda.core.mock.MockCloud;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.MultiLevelMap;

import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class MinimalistHttpTest extends TestBase {

    private static final int HTTP_PORT = MINIMALIST_HTTP_PORT;
    private static final String[] INFO_SERVICE = {"/info", "info"};
    private static final String[] INFO_LIB = {"/info/lib", "lib"};
    private static final String[] INFO_ROUTES = {"/info/routes", "routes"};
    private static final String[] HEALTH_SERVICE = {"/health", "health"};
    private static final String[] ENV_SERVICE = {"/env", "env"};
    private static final String[] LIVENESSPROBE = {"/livenessprobe", "livenessprobe"};
    private static final String[][] ADMIN_ENDPOINTS = {INFO_SERVICE, INFO_LIB, INFO_ROUTES,
            HEALTH_SERVICE, ENV_SERVICE, LIVENESSPROBE};

    @SuppressWarnings("unchecked")
    @Test
    public void homePageTest() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("Minimalist HTTP server supports these admin endpoints",
                            multi.getElement("message"));
        int n = 0;
        for (String[] service: ADMIN_ENDPOINTS) {
            Assert.assertEquals(service[0], multi.getElement("endpoints["+n+"]"));
            n++;
        }
        Assert.assertTrue(multi.exists("time"));
        Assert.assertEquals("platform-core", multi.getElement("name"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void infoEndpointTest() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/info", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("platform-core", multi.getElement("app.name"));
        Assert.assertEquals("REST", multi.getElement("personality"));
        Assert.assertEquals(Platform.getInstance().getOrigin(), multi.getElement("origin"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void nonExistRemoteInfoEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/info", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void libEndpointTest() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/info/lib", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("platform-core", multi.getElement("app.name"));
        Assert.assertTrue(result.containsKey("library"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteLibEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/info", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteRouteEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/info/routes", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void healthEndpointTest() throws IOException, InterruptedException {
        MockCloud.setSimulateException(false);
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/health", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        MultiLevelMap map = new MultiLevelMap(result);
        Assert.assertEquals("UP", map.getElement("status"));
        Assert.assertEquals("fine", map.getElement("upstream[0].message"));
        Assert.assertEquals(200, map.getElement("upstream[0].status_code"));
        Assert.assertEquals("mock.connector", map.getElement("upstream[0].service"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void simulateHealthCheckFailureTest() throws IOException, InterruptedException {
        MockCloud.setSimulateException(true);
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/health", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        // failed health check is returned as HTTP-400
        Assert.assertEquals(400, response.getStatus());
        MultiLevelMap map = new MultiLevelMap(result);
        Assert.assertEquals("DOWN", map.getElement("status"));
        Assert.assertEquals("just a test", map.getElement("upstream[0].message"));
        // original status code from upstream service is preserved
        Assert.assertEquals(500, map.getElement("upstream[0].status_code"));
        Assert.assertEquals("mock.connector", map.getElement("upstream[0].service"));
        // livenessProbe is linked to health check
        response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/livenessprobe", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        Assert.assertEquals(400, response.getStatus());
        Assert.assertEquals("Unhealthy. Please check '/health' endpoint.", response.getBody());
        MockCloud.setSimulateException(false);
        // try it again
        httpGet("http://127.0.0.1:"+ HTTP_PORT, "/health", null);
        response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/livenessprobe", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        Assert.assertEquals("OK", response.getBody());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteHealthEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/health", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @Test
    public void livenessEndpointTest() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/livenessprobe", null);
        assert response != null;
        Assert.assertEquals("OK", response.getBody());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteLivenessEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/livenessprobe", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void envEndpointTest() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/env", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("platform-core", multi.getElement("app.name"));
        Assert.assertTrue(multi.getElement("env") instanceof Map);
        Assert.assertTrue(multi.getElement("routing.private") instanceof List);
        Assert.assertTrue(multi.getElement("routing.public") instanceof List);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteEnvEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/env", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownUsingGetWillFail() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/shutdown", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendUsingGetWillFail() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/suspend/now", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeUsingGetWillFail() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/resume/now", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownWithoutAppInstanceWillFail() throws IOException, InterruptedException {
        EventEnvelope response = httpPost("http://127.0.0.1:"+ HTTP_PORT, "/shutdown", null, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownWithIncorrectAppInstanceWillFail() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        EventEnvelope response = httpPost("http://127.0.0.1:"+ HTTP_PORT, "/shutdown", headers, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendWithoutAppInstanceWillFail() throws IOException, InterruptedException {
        EventEnvelope response = httpPost("http://127.0.0.1:"+ HTTP_PORT, "/suspend/now", null, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendWithIncorrectAppInstanceWillFail() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        EventEnvelope response = httpPost("http://127.0.0.1:"+ HTTP_PORT, "/suspend/now", headers, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeWithoutAppInstanceWillFail() throws IOException, InterruptedException {
        EventEnvelope response = httpPost("http://127.0.0.1:"+ HTTP_PORT, "/resume/now", null, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeWithIncorrectAppInstanceWillFail() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        EventEnvelope response = httpPost("http://127.0.0.1:"+ HTTP_PORT, "/resume/now", headers, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        EventEnvelope response = httpPost("http://127.0.0.1:"+ HTTP_PORT, "/suspend/now", headers, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals("suspend", result.get("type"));
        Assert.assertEquals(200, result.get("status"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        EventEnvelope response = httpPost("http://127.0.0.1:"+ HTTP_PORT, "/resume/now", headers, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals("resume", result.get("type"));
        Assert.assertEquals(200, result.get("status"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void pageNotExists() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+ HTTP_PORT, "/no_such_page", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Resource not found", result.get("message"));
    }

}
