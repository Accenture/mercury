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

package org.platformlambda.servlets;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.common.TestBase;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.client.PersistentWsClient;

import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class ServletTest extends TestBase {

    private static final SimpleXmlWriter xml = new SimpleXmlWriter();

    @SuppressWarnings("unchecked")
    @Test
    public void infoEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/info", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("rest-spring", multi.getElement("app.name"));
        Assert.assertEquals("APP", multi.getElement("personality"));
        Assert.assertEquals(Platform.getInstance().getOrigin(), multi.getElement("origin"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteInfoEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/info", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void libEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/info/lib", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("rest-spring", multi.getElement("app.name"));
        Assert.assertTrue(result.containsKey("library"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteLibEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/info/lib", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void routeEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/info/routes", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("rest-spring", multi.getElement("app.name"));
        Object o = result.get("route_substitution");
        Assert.assertTrue(o instanceof Map);
        Map<String, Object> substitution = (Map<String, Object>) o;
        Assert.assertEquals("hello.world", substitution.get("hello.test"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteRouteEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/info/routes", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void healthEndpointTest() throws IOException, InterruptedException {
        String MESSAGE = "Did you forget to define mandatory.health.dependencies or optional.health.dependencies";
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/health", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals("UP", result.get("status"));
        Assert.assertEquals(MESSAGE, result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void protectedEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://localhost:"+port, "/health", headers);
        assert response != null;
        Assert.assertEquals(404, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteHealthEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/health", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void uriPathSecurityTest() throws IOException, InterruptedException {
        String url = "/api/hello/world moved to https://evil.site?hello world=abc";
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://localhost:"+port, url, headers);
        assert response != null;
        Assert.assertEquals(404, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> error = (Map<String, Object>) response.getBody();
        Assert.assertEquals("/api/hello/world moved to https", error.get("path"));
    }

    @Test
    public void livenessEndpointTest() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/livenessprobe", null);
        Assert.assertEquals("OK", response.getBody());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteLivenessEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/livenessprobe", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void envEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/env", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("rest-spring", multi.getElement("app.name"));
        Assert.assertTrue(result.get("env") instanceof Map);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteEnvEndpointTest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/env", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownUsingGetWillFail() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/shutdown", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Not Found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendUsingGetWillFail() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/suspend", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Not Found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeUsingGetWillFail() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/resume", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Not Found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownWithoutAppInstanceWillFail() throws IOException, InterruptedException {
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/shutdown", null, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(400, response.getStatus());
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownWithIncorrectAppInstanceWillFail() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/shutdown", headers, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendWithoutAppInstanceWillFail() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/suspend", headers, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(400, response.getStatus());
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendWithIncorrectAppInstanceWillFail() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/suspend", headers, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeWithoutAppInstanceWillFail() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/resume", headers, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(400, response.getStatus());
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeWithIncorrectAppInstanceWillFail() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        headers.put("x-app-instance", "does-not-exist");
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/resume", headers, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendAppInstanceOK() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/suspend", headers, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(200, result.get("status"));
        Assert.assertEquals("ok", result.get("type"));
        Assert.assertEquals("/suspend", result.get("path"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeAppInstanceOK() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/resume", headers, new HashMap<>());
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(200, result.get("status"));
        Assert.assertEquals("ok", result.get("type"));
        Assert.assertEquals("/resume", result.get("path"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void getPoJoFromSpringRest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/pojo?name=test", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(101, result.get("number"));
        Assert.assertEquals("test", result.get("name"));
    }

    @Test
    public void getPoJoFromSpringRestHtml() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "text/html");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/hello/html", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        Assert.assertEquals("<html><body><div>hello</div></body></html>", response.getBody());
    }

    @Test
    public void getListFromSpringRest() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/xml");
        headers.put("x-raw-xml", "true");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/hello/list", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        String text = (String) response.getBody();
        Assert.assertTrue(text.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(text.contains("<item>three</item>"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void springRestExceptionHandling() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/pojo?name=exception", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(400, response.getStatus());
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("IllegalArgumentException", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void springRestNullHandling() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/pojo?name=null", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(500, response.getStatus());
        Assert.assertEquals(500, result.get("status"));
        Assert.assertEquals("Null pointer exception", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void springRestIOExceptionHandling() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/pojo?name=io_exception", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(500, response.getStatus());
        Assert.assertEquals(500, result.get("status"));
        Assert.assertEquals("IOException", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void springRestAppExceptionHandling() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/pojo?name=app_exception", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(401, response.getStatus());
        Assert.assertEquals(401, result.get("status"));
        Assert.assertEquals("AppException", result.get("message"));
    }

    @Test
    public void springRestAppExceptionHandlingHtml() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "text/html");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/pojo?name=app_exception", headers);
        assert response != null;
        Assert.assertEquals(401, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof String);
        String result = (String) response.getBody();
        Assert.assertTrue(result.startsWith("<!DOCTYPE html>"));
        Assert.assertTrue(result.contains("HTTP-401"));
    }

    @Test
    public void springRestAppExceptionHandlingXml() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/xml");
        headers.put("x-raw-xml", "true");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/pojo?name=app_exception", headers);
        assert response != null;
        Assert.assertEquals(401, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof String);
        String result = (String) response.getBody();
        Assert.assertTrue(result.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(result.contains("<status>401</status>"));
    }

    @Test
    public void getTextResponseFromServlet() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/mock_servlet", null);
        assert response != null;
        Assert.assertEquals(200, response.getStatus());
        Assert.assertNotNull(response.getHeader("content-type"));
        String contentType = response.getHeader("content-type");
        Assert.assertTrue(contentType.startsWith("text/plain"));
        Assert.assertTrue(response.getBody() instanceof String);
        Assert.assertEquals("hello world from servlet", response.getBody());
    }

    @Test
    public void getIndexPage() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/", null);
        assert response != null;
        Assert.assertEquals(200, response.getStatus());
        Assert.assertNotNull(response.getHeader("content-type"));
        String contentType = response.getHeader("content-type");
        Assert.assertTrue(contentType.startsWith("text/html"));
        Assert.assertTrue(response.getBody() instanceof String);
        String text = (String) response.getBody();
        Assert.assertTrue(text.startsWith("<!DOCTYPE html>"));
        Assert.assertTrue(text.contains("Your application is running"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void pageNotExists() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/no_such_page", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Not Found", result.get("message"));
    }

    @Test
    public void pageNotExistsXml() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/xml");
        headers.put("x-raw-xml", "true");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/no_such_page", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        Assert.assertEquals(404, response.getStatus());
        String result = (String) response.getBody();
        Assert.assertTrue(result.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(result.contains("<status>404</status>"));
    }

    @Test
    public void pageNotExistsHtml() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "text/html");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/no_such_page", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        Assert.assertEquals(404, response.getStatus());
        String result = (String) response.getBody();
        Assert.assertTrue(result.startsWith("<!DOCTYPE html>"));
        Assert.assertTrue(result.contains("HTTP-404"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void getHelloJson() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/hello/map", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals("hello", result.get("name"));
    }

    @Test
    public void getHelloXml() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/xml");
        headers.put("x-raw-xml", "true");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/hello/map", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        Assert.assertEquals(200, response.getStatus());
        String result = (String) response.getBody();
        Assert.assertTrue(result.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(result.contains("<name>hello</name>"));
    }

    @Test
    public void getHelloHtml() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "text/html");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/hello/map", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        String result = (String) response.getBody();
        Assert.assertTrue(result.startsWith("<html>") && result.endsWith("</html>"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void sendJson() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Content-Type", "application/json");
        headers.put("accept", "application/json");
        Map<String, Object> body = new HashMap<>();
        body.put("hello", "world");
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/hello/map", headers, body);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("world", multi.getElement("data.hello"));
    }

    @Test
    public void sendXml() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "application/xml");
        headers.put("Content-Type", "application/xml");
        headers.put("x-raw-xml", "true");
        Map<String, Object> body = new HashMap<>();
        body.put("hello", "world");
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/hello/map", headers, xml.write(body));
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        String text = (String) response.getBody();
        Assert.assertTrue(text.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(text.contains("<hello>world</hello>"));
    }

    @Test
    public void sendXmlReceiveHtml() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "text/html");
        headers.put("Content-Type", "application/xml");
        headers.put("x-raw-xml", "true");
        Map<String, Object> body = new HashMap<>();
        body.put("hello", "world");
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/hello/map", headers, xml.write(body));
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        String text = (String) response.getBody();
        Assert.assertTrue(text.startsWith("<html>") && text.endsWith("</html>"));
    }

    @Test
    public void sendText() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "text/plain");
        headers.put("Content-Type", "text/plain");
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/hello/text", headers, "hello world");
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        String text = (String) response.getBody();
        Assert.assertTrue(text.contains("\"data\": \"hello world\""));
    }

    @Test
    public void websocketConnectionTest() throws InterruptedException {
        final BlockingQueue<Boolean> textBench = new ArrayBlockingQueue<>(1);
        final BlockingQueue<Boolean> bytesBench = new ArrayBlockingQueue<>(1);
        String MESSAGE = "hello world";
        EventEmitter po = EventEmitter.getInstance();
        Utility util = Utility.getInstance();
        LambdaFunction connector = (headers, input, instance) -> {
            if ("open".equals(headers.get("type"))) {
                String txPath = headers.get("tx_path");
                Assert.assertNotNull(txPath);
                po.send(txPath, MESSAGE);
                // also test sending bytes
                po.send(txPath, util.getUTF(MESSAGE));
            }
            if ("string".equals(headers.get("type"))) {
                String text = (String) input;
                textBench.offer(true);
                Assert.assertEquals(MESSAGE, text);
            }
            if ("bytes".equals(headers.get("type"))) {
                byte[] text = (byte[]) input;
                bytesBench.offer(true);
                Assert.assertEquals(MESSAGE, util.getUTF(text));
            }

            return true;
        };
        boolean ready = Utility.getInstance().portReady("127.0.0.1", wsPort, 5000);
        Assert.assertTrue(ready);
        PersistentWsClient client = new PersistentWsClient(connector,
                Collections.singletonList("ws://127.0.0.1:"+wsPort+"/ws/mock"));
        client.start();

        textBench.poll(5000, TimeUnit.MILLISECONDS);
        bytesBench.poll(5000, TimeUnit.MILLISECONDS);
        client.close();
    }

}
