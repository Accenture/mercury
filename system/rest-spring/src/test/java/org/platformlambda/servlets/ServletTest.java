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

package org.platformlambda.servlets;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.common.TestBase;
import org.platformlambda.core.exception.AppException;
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
        Assert.assertEquals("rest-spring", multi.getElement("app.name"));
        Assert.assertTrue(result.containsKey("library"));
    }

    @Test(expected = AppException.class)
    public void remoteLibEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/lib", headers);
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

    @Test(expected = AppException.class)
    public void remoteRouteEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/routes", headers);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void healthEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://localhost:"+port+"/health");
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("UP", result.get("status"));
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
        Assert.assertEquals("rest-spring", multi.getElement("app.name"));
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
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/shutdown");
        Assert.assertTrue(response instanceof String);
        SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
    }

    @Test(expected = AppException.class)
    public void suspendUsingGetWillFail() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/suspend");
        Assert.assertTrue(response instanceof String);
        SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
    }

    @Test(expected = AppException.class)
    public void resumeUsingGetWillFail() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/resume");
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
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend", new HashMap<>(), new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void suspendWithIncorrectAppInstanceWillFail() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend", headers, new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void resumeWithoutAppInstanceWillFail() throws AppException, IOException {
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume", new HashMap<>(), new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void resumeWithIncorrectAppInstanceWillFail() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume", headers, new HashMap<>());
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
    public void springRestExceptionHandling() throws IOException {
        try {
            SimpleHttpRequests.get("http://127.0.0.1:" + port + "/pojo?name=exception");
            throw new IOException("it should throw exception");
        } catch (AppException e) {
            Assert.assertEquals(400, e.getStatus());
            Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(e.getMessage(), Map.class);
            Assert.assertEquals(400, result.get("status"));
            Assert.assertEquals("IllegalArgumentException", result.get("message"));
            Assert.assertEquals("error", result.get("type"));
        }
    }

    @SuppressWarnings("unchecked")
    @Test
    public void springRestNullHandling() throws IOException {
        try {
            SimpleHttpRequests.get("http://127.0.0.1:" + port + "/pojo?name=null");
            throw new IOException("it should throw exception");
        } catch (AppException e) {
            Assert.assertEquals(500, e.getStatus());
            Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(e.getMessage(), Map.class);
            Assert.assertEquals(500, result.get("status"));
            Assert.assertEquals("Null pointer exception", result.get("message"));
            Assert.assertEquals("error", result.get("type"));
        }
    }

    @SuppressWarnings("unchecked")
    @Test
    public void springRestIOExceptionHandling() throws IOException {
        try {
            SimpleHttpRequests.get("http://127.0.0.1:" + port + "/pojo?name=io_exception");
            throw new IOException("it should throw exception");
        } catch (AppException e) {
            Assert.assertEquals(500, e.getStatus());
            Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(e.getMessage(), Map.class);
            Assert.assertEquals(500, result.get("status"));
            Assert.assertEquals("IOException", result.get("message"));
            Assert.assertEquals("error", result.get("type"));
        }
    }

    @SuppressWarnings("unchecked")
    @Test
    public void springRestAppExceptionHandling() throws IOException {
        try {
            SimpleHttpRequests.get("http://127.0.0.1:" + port + "/pojo?name=app_exception");
            throw new IOException("it should throw exception");
        } catch (AppException e) {
            Assert.assertEquals(401, e.getStatus());
            Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(e.getMessage(), Map.class);
            Assert.assertEquals(401, result.get("status"));
            Assert.assertEquals("AppException", result.get("message"));
            Assert.assertEquals("error", result.get("type"));
        }
    }

    @Test
    public void springRestAppExceptionHandlingHtml() throws IOException {
        try {
            SimpleHttpRequests.get("http://127.0.0.1:" + port + "/pojo?name=app_exception", "text/html");
            throw new IOException("it should throw exception");
        } catch (AppException e) {
            Assert.assertEquals(401, e.getStatus());
            String result = e.getMessage();
            Assert.assertTrue(result.startsWith("<!DOCTYPE html>"));
            Assert.assertTrue(result.contains("HTTP-401"));
        }
    }

    @Test
    public void springRestAppExceptionHandlingXml() throws IOException {
        try {
            SimpleHttpRequests.get("http://127.0.0.1:" + port + "/pojo?name=app_exception", "application/xml");
            throw new IOException("it should throw exception");
        } catch (AppException e) {
            Assert.assertEquals(401, e.getStatus());
            String result = e.getMessage();
            Assert.assertTrue(result.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
            Assert.assertTrue(result.contains("<status>401</status>"));
        }
    }

    @Test(expected = AppException.class)
    public void springRestNotFound() throws AppException, IOException {
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_such_page");
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
    public void websocketConnectionTest() {
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
                if (text.startsWith("{") && text.contains("keep-alive")) {
                    textBench.offer(true);
                } else {
                    Assert.assertEquals(MESSAGE, text);
                }
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

        try {
            textBench.poll(5000, TimeUnit.MILLISECONDS);
            bytesBench.poll(5000, TimeUnit.MILLISECONDS);
            client.close();
        } catch (InterruptedException e) {
            // ok to ignore
        }

    }

}
