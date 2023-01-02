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
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.client.PersistentWsClient;
import org.platformlambda.util.SimpleHttpRequests;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class ServletTest extends TestBase {

    private static final SimpleXmlWriter xml = new SimpleXmlWriter();

    @SuppressWarnings("unchecked")
    @Test
    public void infoEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("rest-spring", multi.getElement("app.name"));
        Assert.assertEquals("APP", multi.getElement("personality"));
        String origin = Platform.getInstance().getOrigin();
        Assert.assertEquals(origin, multi.getElement("origin"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteInfoEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info", headers));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void libEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/lib");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("rest-spring", multi.getElement("app.name"));
        Assert.assertTrue(result.containsKey("library"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteLibEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/lib", headers));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void routeEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/routes");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("rest-spring", multi.getElement("app.name"));
        Object o = result.get("route_substitution");
        Assert.assertTrue(o instanceof Map);
        Map<String, Object> substitution = (Map<String, Object>) o;
        Assert.assertEquals("hello.world", substitution.get("hello.test"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteRouteEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/routes", headers));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void healthEndpointTest() throws AppException, IOException {
        String MESSAGE = "Did you forget to define mandatory.health.dependencies or optional.health.dependencies";
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/health");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("UP", result.get("status"));
        Assert.assertEquals(MESSAGE, result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void protectedEndpointTest() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://localhost:"+port+"/health"));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteHealthEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/health", headers));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void uriPathSecurityTest() {
        String url = "http://127.0.0.1:"+port+"/api/hello/world moved to https://evil.site?hello world=abc";
        AppException ex = Assert.assertThrows(AppException.class, () -> SimpleHttpRequests.get(url, new HashMap<>()));
        Assert.assertEquals(404, ex.getStatus());
        Map<String, Object> error = SimpleMapper.getInstance().getMapper().readValue(ex.getMessage(), Map.class);
        Assert.assertEquals("/api/hello/world...", error.get("path"));
    }

    @Test
    public void livenessEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/livenessprobe", "text/plain");
        Assert.assertEquals("OK", response);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteLivenessEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/livenessprobe", headers));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void envEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/env");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("rest-spring", multi.getElement("app.name"));
        Assert.assertTrue(result.get("env") instanceof Map);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteEnvEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/env", headers));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownUsingGetWillFail() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/shutdown"));
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Not Found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendUsingGetWillFail() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/suspend"));
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Not Found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeUsingGetWillFail() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/resume"));
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Not Found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownWithoutAppInstanceWillFail() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.post("http://127.0.0.1:"+port+"/shutdown", new HashMap<>(), new HashMap<>()));
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownWithIncorrectAppInstanceWillFail() {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.post("http://127.0.0.1:"+port+"/shutdown", headers, new HashMap<>()));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendWithoutAppInstanceWillFail() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend", new HashMap<>(), new HashMap<>()));
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendWithIncorrectAppInstanceWillFail() {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend", headers, new HashMap<>()));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeWithoutAppInstanceWillFail() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume", new HashMap<>(), new HashMap<>()));
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeWithIncorrectAppInstanceWillFail() {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume", headers, new HashMap<>()));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendAppInstanceOK() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        Object response = SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend", headers, new HashMap<>());
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals(200, result.get("status"));
        Assert.assertEquals("ok", result.get("type"));
        Assert.assertEquals("/suspend", result.get("path"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeAppInstanceOK() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        Object response = SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume", headers, new HashMap<>());
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals(200, result.get("status"));
        Assert.assertEquals("ok", result.get("type"));
        Assert.assertEquals("/resume", result.get("path"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void getPoJoFromSpringRest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/pojo?name=test");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals(101, result.get("number"));
        Assert.assertEquals("test", result.get("name"));
    }

    @Test
    public void getPoJoFromSpringRestHtml() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/hello/html", "text/html");
        Assert.assertTrue(response instanceof String);
        Assert.assertEquals("<html><body><div>hello</div></body></html>", response);
    }

    @Test
    public void getListFromSpringRest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/hello/list", "application/xml");
        Assert.assertTrue(response instanceof String);
        String text = (String) response;
        Assert.assertTrue(text.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(text.contains("<item>three</item>"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void springRestExceptionHandling() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:" + port + "/pojo?name=exception"));
        Assert.assertEquals(400, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("IllegalArgumentException", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void springRestNullHandling() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:" + port + "/pojo?name=null"));
        Assert.assertEquals(500, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(500, result.get("status"));
        Assert.assertEquals("Null pointer exception", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void springRestIOExceptionHandling() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:" + port + "/pojo?name=io_exception"));
        Assert.assertEquals(500, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(500, result.get("status"));
        Assert.assertEquals("IOException", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void springRestAppExceptionHandling() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:" + port + "/pojo?name=app_exception"));
        Assert.assertEquals(401, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(401, result.get("status"));
        Assert.assertEquals("AppException", result.get("message"));
    }

    @Test
    public void springRestAppExceptionHandlingHtml() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:" + port + "/pojo?name=app_exception", "text/html"));
        Assert.assertEquals(401, ex.getStatus());
        String result = ex.getMessage();
        Assert.assertTrue(result.startsWith("<!DOCTYPE html>"));
        Assert.assertTrue(result.contains("HTTP-401"));
    }

    @Test
    public void springRestAppExceptionHandlingXml() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:" + port + "/pojo?name=app_exception", "application/xml"));
        Assert.assertEquals(401, ex.getStatus());
        String result = ex.getMessage();
        Assert.assertTrue(result.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(result.contains("<status>401</status>"));
    }

    @Test
    public void getResponseFromServlet() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/mock_servlet");
        Assert.assertTrue(response instanceof String);
        Assert.assertEquals("hello world from servlet", response);
    }

    @Test
    public void getIndexPage() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/");
        Assert.assertTrue(response instanceof String);
        String text = (String) response;
        Assert.assertTrue(text.startsWith("<!DOCTYPE html>"));
        Assert.assertTrue(text.contains("Your application is running"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void pageNotExists() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_such_page"));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Not Found", result.get("message"));
    }

    @Test
    public void pageNotExistsXml() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_such_page", "application/xml"));
        Assert.assertEquals(404, ex.getStatus());
        String result = ex.getMessage();
        Assert.assertTrue(result.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(result.contains("<status>404</status>"));
    }

    @Test
    public void pageNotExistsHtml() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_such_page", "text/html"));
        Assert.assertEquals(404, ex.getStatus());
        String result = ex.getMessage();
        Assert.assertTrue(result.startsWith("<!DOCTYPE html>"));
        Assert.assertTrue(result.contains("HTTP-404"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void getHelloJson() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/hello/map", "application/json");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("hello", result.get("name"));
    }

    @Test
    public void getHelloXml() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/hello/map", "application/xml");
        Assert.assertTrue(response instanceof String);
        String text = (String) response;
        Assert.assertTrue(text.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(text.contains("<name>hello</name>"));
    }

    @Test
    public void getHelloHtml() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/hello/map", "text/html");
        Assert.assertTrue(response instanceof String);
        String text = (String) response;
        Assert.assertTrue(text.startsWith("<html") && text.endsWith("</html>"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void sendJson() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "application/json");
        headers.put("Content-Type", "application/json");
        Map<String, Object> body = new HashMap<>();
        body.put("hello", "world");
        Object response = SimpleHttpRequests.postText("http://127.0.0.1:"+port+"/hello/map",
                "application/json", headers,
                SimpleMapper.getInstance().getMapper().writeValueAsString(body));
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("world", multi.getElement("data.hello"));
    }

    @Test
    public void sendXml() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "application/xml");
        headers.put("Content-Type", "application/xml");
        Map<String, Object> body = new HashMap<>();
        body.put("hello", "world");
        Object response = SimpleHttpRequests.postText("http://127.0.0.1:"+port+"/hello/map",
                "application/xml", headers, xml.write(body));
        Assert.assertTrue(response instanceof String);
        String text = (String) response;
        Assert.assertTrue(text.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(text.contains("<hello>world</hello>"));
    }

    @Test
    public void sendHtml() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "text/html");
        headers.put("Content-Type", "text/html");
        Map<String, Object> body = new HashMap<>();
        body.put("hello", "world");
        Object response = SimpleHttpRequests.postText("http://127.0.0.1:"+port+"/hello/text",
                "text/html", headers, xml.write(body));
        Assert.assertTrue(response instanceof String);
        String text = (String) response;
        Assert.assertTrue(text.startsWith("<html") && text.endsWith("</html>"));
    }

    @Test
    public void sendText() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "text/plain");
        headers.put("Content-Type", "text/plain");
        Object response = SimpleHttpRequests.postText("http://127.0.0.1:"+port+"/hello/text",
                "text/plain", headers, "hello world");
        Assert.assertTrue(response instanceof String);
        String text = (String) response;
        Assert.assertTrue(text.contains("\"data\": \"hello world\""));
    }

    @Test
    public void websocketConnectionTest() throws InterruptedException {
        final BlockingQueue<Boolean> textBench = new ArrayBlockingQueue<>(1);
        final BlockingQueue<Boolean> bytesBench = new ArrayBlockingQueue<>(1);
        String MESSAGE = "hello world";
        PostOffice po = PostOffice.getInstance();
        Utility util = Utility.getInstance();
        LambdaFunction connector = (headers, body, instance) -> {
            if ("open".equals(headers.get("type"))) {
                String txPath = headers.get("tx_path");
                Assert.assertNotNull(txPath);
                po.send(txPath, MESSAGE);
                // also test sending bytes
                po.send(txPath, util.getUTF(MESSAGE));
            }
            if ("string".equals(headers.get("type"))) {
                String text = (String) body;
                textBench.offer(true);
                Assert.assertEquals(MESSAGE, text);
            }
            if ("bytes".equals(headers.get("type"))) {
                byte[] text = (byte[]) body;
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
