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

package org.platformlambda.automation.tests;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.automation.mock.TestBase;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeoutException;

public class RestEndpointTest extends TestBase {
    private static final Logger log = LoggerFactory.getLogger(RestEndpointTest.class);

    private static final String HTTP_REQUEST = "async.http.request";

    @SuppressWarnings("unchecked")
    @Test
    public void serviceTest() throws AppException, IOException, TimeoutException {
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            po.annotateTrace("hello", "world");
            return true;
        };
        if (!platform.hasRoute("v1.api.auth")) {
            platform.registerPrivate("v1.api.auth", f, 1);
        }
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setHeader("accept", "application/json");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);

        EventEnvelope res = po.request(HTTP_REQUEST, 5000, req.toMap());
        Assert.assertTrue(res.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) res.getBody());
        Assert.assertEquals("application/json", map.getElement("headers.accept"));
        Assert.assertEquals(false, map.getElement("https"));
        Assert.assertEquals("/api/hello/world", map.getElement("url"));
        Assert.assertEquals("GET", map.getElement("method"));
        Assert.assertEquals("127.0.0.1", map.getElement("ip"));
        Assert.assertEquals(10, map.getElement("timeout"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void authRoutingTest1() {
        Utility util = Utility.getInstance();
        PostOffice po = PostOffice.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setHeader("accept", "application/json");
        String credentials = "Basic " + util.bytesToBase64(util.getUTF("hello:world"));
        req.setHeader("authorization", credentials);
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        try {
            po.request(HTTP_REQUEST, 5000, req.toMap());
            throw new IllegalArgumentException("Test is excepted to throw AppException");
        } catch (Exception e) {
            Assert.assertTrue(e instanceof AppException);
            AppException ex = (AppException) e;
            Assert.assertEquals(503, ex.getStatus());
            String message = ex.getMessage();
            Map<String, Object> eMap = SimpleMapper.getInstance().getMapper().readValue(message, Map.class);
            Assert.assertEquals("Service v1.basic.auth not reachable", eMap.get("message"));
        }
    }

    @SuppressWarnings("unchecked")
    @Test
    public void authRoutingTest2() throws IOException {
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            po.annotateTrace("demo", "will show 'unauthorized'");
            return false;
        };
        if (!platform.hasRoute("v1.app.auth")) {
            platform.registerPrivate("v1.app.auth", f, 1);
        }
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setHeader("accept", "application/json");
        req.setHeader("x-app-name", "demo");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        try {
            po.request(HTTP_REQUEST, 5000, req.toMap());
            throw new IllegalArgumentException("Test is excepted to throw AppException");
        } catch (Exception e) {
            Assert.assertTrue(e instanceof AppException);
            AppException ex = (AppException) e;
            Assert.assertEquals(401, ex.getStatus());
            String message = ex.getMessage();
            Map<String, Object> eMap = SimpleMapper.getInstance().getMapper().readValue(message, Map.class);
            Assert.assertEquals("Unauthorized", eMap.get("message"));
        }
    }

    @Test
    public void uploadSmallBlockWithPut() throws AppException, IOException, TimeoutException {
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            po.annotateTrace("hello", "world");
            return true;
        };
        if (!platform.hasRoute("v1.api.auth")) {
            platform.registerPrivate("v1.api.auth", f, 1);
        }
        int len = 0;
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        ObjectStreamIO stream = new ObjectStreamIO();
        ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
        for (int i=0; i < 100; i++) {
            byte[] line = util.getUTF("hello world "+i+"\n");
            out.write(line);
            bytes.write(line);
            len += line.length;
        }
        out.close();
        byte[] b = bytes.toByteArray();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("PUT");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        req.setStreamRoute(stream.getInputStreamId());
        req.setContentLength(len);
        EventEnvelope res = po.request(HTTP_REQUEST, 5000, req.toMap());
        Assert.assertTrue(res.getBody() instanceof byte[]);
        Assert.assertArrayEquals(b, (byte[]) res.getBody());
    }

    @Test
    public void uploadLargeBlockWithPut() throws AppException, IOException, TimeoutException {
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            po.annotateTrace("hello", "world");
            return true;
        };
        if (!platform.hasRoute("v1.api.auth")) {
            platform.registerPrivate("v1.api.auth", f, 1);
        }
        int len = 0;
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        ObjectStreamIO stream = new ObjectStreamIO();
        ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
        for (int i=0; i < 520; i++) {
            String line = "hello world "+i+"\n";
            byte[] d = util.getUTF(line);
            out.write(d);
            byte[] d2 = util.getUTF(line);
            bytes.write(d2);
            len += d2.length;
        }
        out.close();
        byte[] b = bytes.toByteArray();

        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("PUT");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        req.setHeader("accept", "application/json");
        req.setHeader("content-type", "application/octet-stream");
        req.setContentLength(len);
        req.setStreamRoute(stream.getInputStreamId());
        EventEnvelope res = po.request(HTTP_REQUEST, 5000, req.toMap());
        Assert.assertNotNull(res.getHeaders().get("stream"));
        String resultStream = res.getHeaders().get("stream");
        ObjectStreamReader in = new ObjectStreamReader(resultStream, 30000);
        ByteArrayOutputStream body = new ByteArrayOutputStream();
        for (Object o: in) {
            if (o instanceof byte[]) {
                body.write((byte[]) o);
            }
        }
        byte[] bodyBytes = body.toByteArray();
        Assert.assertArrayEquals(b, bodyBytes);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void postJson() throws AppException, IOException, TimeoutException {
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            po.annotateTrace("hello", "world");
            return true;
        };
        if (!platform.hasRoute("v1.api.auth")) {
            platform.registerPrivate("v1.api.auth", f, 1);
        }
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("POST");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        Map<String, Object> data = new HashMap<>();
        data.put("hello", "world");
        data.put("test", "message");
        String json = SimpleMapper.getInstance().getMapper().writeValueAsString(data);
        req.setBody(json);
        req.setHeader("accept", "application/json");
        req.setHeader("content-type", "application/json");
        EventEnvelope res = po.request(HTTP_REQUEST, 5000, req.toMap());
        Assert.assertTrue(res.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) res.getBody());
        Assert.assertEquals("application/json", map.getElement("headers.content-type"));
        Assert.assertEquals("application/json", map.getElement("headers.accept"));
        Assert.assertEquals(false, map.getElement("https"));
        Assert.assertEquals("/api/hello/world", map.getElement("url"));
        Assert.assertEquals("POST", map.getElement("method"));
        Assert.assertEquals("127.0.0.1", map.getElement("ip"));
        Assert.assertEquals(10, map.getElement("timeout"));
        Assert.assertTrue(map.getElement("body") instanceof Map);
        Map<String, Object> received = (Map<String, Object>) map.getElement("body");
        Assert.assertEquals(data, received);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void postXml() throws AppException, IOException, TimeoutException {
        SimpleXmlWriter xmlWriter = new SimpleXmlWriter();
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            po.annotateTrace("hello", "world");
            return true;
        };
        if (!platform.hasRoute("v1.api.auth")) {
            platform.registerPrivate("v1.api.auth", f, 1);
        }
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("POST");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        Map<String, Object> data = new HashMap<>();
        data.put("hello", "world");
        data.put("test", "message");
        String xml = xmlWriter.write(data);
        req.setBody(xml);
        req.setHeader("accept", "application/xml");
        req.setHeader("content-type", "application/xml");
        EventEnvelope res = po.request(HTTP_REQUEST, 5000, req.toMap());
        Assert.assertTrue(res.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) res.getBody());
        Assert.assertEquals("application/xml", map.getElement("headers.content-type"));
        Assert.assertEquals("application/xml", map.getElement("headers.accept"));
        // xml key-values are parsed as text
        Assert.assertEquals("false", map.getElement("https"));
        Assert.assertEquals("/api/hello/world", map.getElement("url"));
        Assert.assertEquals("POST", map.getElement("method"));
        Assert.assertEquals("127.0.0.1", map.getElement("ip"));
        Assert.assertEquals("10", map.getElement("timeout"));
        Assert.assertTrue(map.getElement("body") instanceof Map);
        Map<String, Object> received = (Map<String, Object>) map.getElement("body");
        Assert.assertEquals(data, received);
    }

    @Test
    public void getIndexPage() throws AppException, IOException, TimeoutException {
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            po.annotateTrace("hello", "world");
            return true;
        };
        if (!platform.hasRoute("v1.api.auth")) {
            platform.registerPrivate("v1.api.auth", f, 1);
        }
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setUrl("/");
        req.setTargetHost("http://127.0.0.1:"+port);
        EventEnvelope res = po.request(HTTP_REQUEST, 5000, req.toMap());
        Assert.assertEquals("text/html", res.getHeaders().get("content-type"));
        Assert.assertTrue(res.getBody() instanceof String);
        String html = (String) res.getBody();
        InputStream in = this.getClass().getResourceAsStream("/public/index.html");
        String content = util.stream2str(in);
        Assert.assertEquals(content, html);
    }

    @Test
    public void getCssPage() throws AppException, IOException, TimeoutException {
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            po.annotateTrace("hello", "world");
            return true;
        };
        if (!platform.hasRoute("v1.api.auth")) {
            platform.registerPrivate("v1.api.auth", f, 1);
        }
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setUrl("/sample.css");
        req.setTargetHost("http://127.0.0.1:"+port);
        EventEnvelope res = po.request(HTTP_REQUEST, 5000, req.toMap());
        Assert.assertEquals("text/css", res.getHeaders().get("content-type"));
        Assert.assertTrue(res.getBody() instanceof String);
        String html = (String) res.getBody();
        InputStream in = this.getClass().getResourceAsStream("/public/sample.css");
        String content = util.stream2str(in);
        Assert.assertEquals(content, html);
    }

    @Test
    public void getJsPage() throws AppException, IOException, TimeoutException {
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            po.annotateTrace("hello", "world");
            return true;
        };
        if (!platform.hasRoute("v1.api.auth")) {
            platform.registerPrivate("v1.api.auth", f, 1);
        }
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setUrl("/sample.js");
        req.setTargetHost("http://127.0.0.1:"+port);
        EventEnvelope res = po.request(HTTP_REQUEST, 5000, req.toMap());
        Assert.assertEquals("text/javascript", res.getHeaders().get("content-type"));
        Assert.assertTrue(res.getBody() instanceof String);
        String html = (String) res.getBody();
        InputStream in = this.getClass().getResourceAsStream("/public/sample.js");
        String content = util.stream2str(in);
        Assert.assertEquals(content, html);
    }

}
