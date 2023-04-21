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

package org.platformlambda.automation;

import io.vertx.core.Future;
import io.vertx.core.Promise;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.platformlambda.common.TestBase;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.*;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class RestEndpointTest extends TestBase {

    private static final String MULTIPART_FORM_DATA = "multipart/form-data";
    private static final String HTTP_REQUEST = "async.http.request";
    private static final long RPC_TIMEOUT = 10000;

    @Before
    public void setupAuthenticator() throws IOException {
        Platform platform = Platform.getInstance();
        if (!platform.hasRoute("v1.api.auth")) {
            LambdaFunction f = (headers, input, instance) -> {
                PostOffice po = new PostOffice(headers, instance);
                po.annotateTrace("hello", "world");
                return true;
            };
            platform.registerPrivate("v1.api.auth", f, 1);
        }
        if (!platform.hasRoute("v1.demo.auth")) {
            LambdaFunction f = (headers, input, instance) -> {
                PostOffice po = new PostOffice(headers, instance);
                po.annotateTrace("demo", "will show 'unauthorized'");
                return false;
            };
            platform.registerPrivate("v1.demo.auth", f, 1);
        }
    }

    @Test
    public void remoteEventApiAccessControlTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        long TIMEOUT = 3000;
        String DEMO_FUNCTION = "demo.private.function";
        LambdaFunction f = (headers, input, instance) -> true;
        Platform platform = Platform.getInstance();
        platform.registerPrivate(DEMO_FUNCTION, f, 1);
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope event = new EventEnvelope();
        event.setTo(DEMO_FUNCTION).setBody("ok").setHeader("hello", "world");
        Future<EventEnvelope> response = po.asyncRequest(event, TIMEOUT, Collections.emptyMap(),
                "http://127.0.0.1:"+port+"/api/event", true);
        response.onSuccess(bench::offer);
        EventEnvelope result = bench.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(403, result.getStatus());
        Assert.assertEquals(DEMO_FUNCTION+" is private", result.getError());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void serviceTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setHeader("accept", "application/json");
        req.setUrl("/api/hello/world?hello world=abc");
        req.setQueryParameter("x1", "y");
        List<String> list = new ArrayList<>();
        list.add("a");
        list.add("b");
        req.setQueryParameter("x2", list);
        req.setTargetHost("http://127.0.0.1:"+port);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
        Assert.assertEquals("application/json", map.getElement("headers.accept"));
        Assert.assertEquals(false, map.getElement("https"));
        Assert.assertEquals("/api/hello/world", map.getElement("url"));
        Assert.assertEquals("GET", map.getElement("method"));
        Assert.assertEquals("127.0.0.1", map.getElement("ip"));
        Assert.assertEquals(10, map.getElement("timeout"));
        Assert.assertEquals("y", map.getElement("parameters.query.x1"));
        Assert.assertEquals(list, map.getElement("parameters.query.x2"));
    }

    @Test
    public void uriPathSecurityTest() throws IOException, InterruptedException {
        uriPathSecurity("/api/hello/world moved to https://evil.site?hello world=abc",
                "/api/hello/world moved to https");
        uriPathSecurity("/api/hello/world <div>test</div>",
                "/api/hello/world ");
        uriPathSecurity("/api/hello/world > something",
                "/api/hello/world ");
        uriPathSecurity("/api/hello/world &nbsp;",
                "/api/hello/world ");
    }

    @SuppressWarnings("unchecked")
    private void uriPathSecurity(String uri, String expected) throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setHeader("accept", "application/json");
        req.setUrl(uri);
        req.setQueryParameter("x1", "y%20");
        List<String> list = new ArrayList<>();
        list.add("a");
        list.add("b");
        req.setQueryParameter("x2", list);
        req.setTargetHost("http://127.0.0.1:"+port);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        System.out.println(response.getBody());

        Assert.assertEquals(404, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> map = (Map<String, Object>) response.getBody();
        Assert.assertEquals("Resource not found", map.get("message"));
        Assert.assertEquals(expected, map.get("path"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void authRoutingTest1() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        Utility util = Utility.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setHeader("accept", "application/json");
        String credentials = "Basic " + util.bytesToBase64(util.getUTF("hello:world"));
        req.setHeader("authorization", credentials);
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(503, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> map = (Map<String, Object>) response.getBody();
        Assert.assertEquals("Service v1.basic.auth not reachable", map.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void authRoutingTest2() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setHeader("accept", "application/json");
        req.setHeader("x-app-name", "demo");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(401, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> map = (Map<String, Object>) response.getBody();
        Assert.assertEquals("Unauthorized", map.get("message"));
    }

    @Test
    public void uploadSmallBlockWithPut() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench1 = new ArrayBlockingQueue<>(1);
        final BlockingQueue<Boolean> bench2 = new ArrayBlockingQueue<>(1);
        final Utility util = Utility.getInstance();
        final EventEmitter po = EventEmitter.getInstance();
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
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench1::offer);
        EventEnvelope response = bench1.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertNull(response.getBody());
        // async.http.request returns a stream
        String streamId = response.getHeaders().get("stream");
        Assert.assertNotNull(streamId);
        ByteArrayOutputStream result = new ByteArrayOutputStream();
        AsyncObjectStreamReader in = new AsyncObjectStreamReader(streamId, RPC_TIMEOUT);
        stream2bytes(in, result).onSuccess(done -> bench2.offer(done));
        Boolean done = bench2.poll(10, TimeUnit.SECONDS);
        Assert.assertEquals(Boolean.TRUE, done);
        Assert.assertArrayEquals(b, result.toByteArray());
    }

    private Future<Boolean> stream2bytes(AsyncObjectStreamReader in, ByteArrayOutputStream out) {
        return Future.future(promise -> {
            fetchNextBlock(promise, in, out);
        });
    }

    private void fetchNextBlock(Promise<Boolean> promise,
                                AsyncObjectStreamReader in, ByteArrayOutputStream out) {
        Utility util = Utility.getInstance();
        Future<Object> block = in.get();
        block.onSuccess(data -> {
            try {
                if (data != null) {
                    if (data instanceof byte[]) {
                        byte[] b = (byte[]) data;
                        if (b.length > 0) {
                            out.write(b);
                        }
                    }
                    if (data instanceof String) {
                        String text = (String) data;
                        if (!text.isEmpty()) {
                            out.write(util.getUTF((String) data));
                        }
                    }
                    fetchNextBlock(promise, in, out);
                } else {
                    try {
                        in.close();
                        promise.complete(true);
                    } catch (IOException e) {
                        promise.fail(e);
                    }
                }
            } catch (IOException e) {
                promise.fail(e);
            }
        });
    }

    @Test
    public void uploadBytesWithPut() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        Utility util = Utility.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        int len = 0;
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        for (int i=0; i < 100; i++) {
            byte[] line = util.getUTF("hello world "+i+"\n");
            bytes.write(line);
            len += line.length;
        }
        byte[] b = bytes.toByteArray();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("PUT");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        req.setBody(b);
        req.setContentLength(len);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof byte[]);
        Assert.assertArrayEquals(b, (byte[]) response.getBody());
    }

    @Test
    public void uploadLargePayloadWithPut() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench1 = new ArrayBlockingQueue<>(1);
        final BlockingQueue<Boolean> bench2 = new ArrayBlockingQueue<>(1);
        Utility util = Utility.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        int len = 0;
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        for (int i=0; i < 4000; i++) {
            byte[] line = util.getUTF("hello world "+i+"\n");
            bytes.write(line);
            len += line.length;
        }
        /*
         * The payload size is 66890 which is larger than default threshold of 50000,
         * the rest automation system will upload it as a stream to the target service.
         */
        byte[] b = bytes.toByteArray();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("PUT");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        req.setBody(b);
        req.setContentLength(len);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench1::offer);
        EventEnvelope response = bench1.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertNull(response.getBody());
        // async.http.request returns a stream
        String streamId = response.getHeaders().get("stream");
        Assert.assertNotNull(streamId);
        ByteArrayOutputStream result = new ByteArrayOutputStream();
        AsyncObjectStreamReader in = new AsyncObjectStreamReader(streamId, RPC_TIMEOUT);
        stream2bytes(in, result).onSuccess(bench2::offer);
        Boolean done = bench2.poll(10, TimeUnit.SECONDS);
        Assert.assertEquals(Boolean.TRUE, done);
        Assert.assertArrayEquals(b, result.toByteArray());
    }

    @Test
    public void uploadStreamWithPut() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench1 = new ArrayBlockingQueue<>(1);
        final BlockingQueue<Boolean> bench2 = new ArrayBlockingQueue<>(1);
        Utility util = Utility.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        int len = 0;
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        ObjectStreamIO stream = new ObjectStreamIO();
        ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
        for (int i=0; i < 600; i++) {
            String line = "hello world "+i+"\n";
            byte[] d = util.getUTF(line);
            out.write(d);
            bytes.write(d);
            len += d.length;
        }
        out.close();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("PUT");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        req.setHeader("accept", "application/octet-stream");
        req.setHeader("content-type", "application/octet-stream");
        req.setContentLength(len);
        req.setStreamRoute(stream.getInputStreamId());
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench1::offer);
        EventEnvelope response = bench1.poll(10, TimeUnit.SECONDS);
        Assert.assertNotNull(response.getHeaders().get("stream"));
        String streamId = response.getHeaders().get("stream");
        ByteArrayOutputStream result = new ByteArrayOutputStream();
        AsyncObjectStreamReader in = new AsyncObjectStreamReader(streamId, RPC_TIMEOUT);
        stream2bytes(in, result).onSuccess(bench2::offer);
        Boolean done = bench2.poll(10, TimeUnit.SECONDS);
        Assert.assertEquals(Boolean.TRUE, done);
        Assert.assertArrayEquals(bytes.toByteArray(), result.toByteArray());
    }

    @Test
    public void uploadMultipartWithPost() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench1 = new ArrayBlockingQueue<>(1);
        final BlockingQueue<Boolean> bench2 = new ArrayBlockingQueue<>(1);
        Utility util = Utility.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        int len = 0;
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        ObjectStreamIO stream = new ObjectStreamIO();
        ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
        for (int i=0; i < 600; i++) {
            String line = "hello world "+i+"\n";
            byte[] d = util.getUTF(line);
            out.write(d);
            bytes.write(d);
            len += d.length;
        }
        out.close();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("POST");
        req.setUrl("/api/upload/demo");
        req.setTargetHost("http://127.0.0.1:"+port);
        req.setHeader("accept", "application/json");
        req.setHeader("content-type", MULTIPART_FORM_DATA);
        req.setContentLength(len);
        req.setFileName("hello-world.txt");
        req.setStreamRoute(stream.getInputStreamId());
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench1::offer);
        EventEnvelope response = bench1.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertNotNull(response.getHeaders().get("stream"));
        String streamId = response.getHeaders().get("stream");
        ByteArrayOutputStream result = new ByteArrayOutputStream();
        AsyncObjectStreamReader in = new AsyncObjectStreamReader(streamId, RPC_TIMEOUT);
        stream2bytes(in, result).onSuccess(bench2::offer);
        Boolean done = bench2.poll(10, TimeUnit.SECONDS);
        Assert.assertEquals(Boolean.TRUE, done);
        Assert.assertArrayEquals(bytes.toByteArray(), result.toByteArray());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void postJson() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
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
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
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
    public void postXml() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        SimpleXmlWriter xmlWriter = new SimpleXmlWriter();
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
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
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

    @SuppressWarnings("unchecked")
    @Test
    public void postJsonMap() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("POST");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        Map<String, Object> data = new HashMap<>();
        data.put("hello", "world");
        data.put("test", "message");
        req.setBody(data);
        req.setHeader("accept", "application/json");
        req.setHeader("content-type", "application/json");
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
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
    public void testJsonResultList() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("POST");
        req.setUrl("/api/hello/list");
        req.setTargetHost("http://127.0.0.1:"+port);
        Map<String, Object> data = new HashMap<>();
        data.put("hello", "world");
        data.put("test", "message");
        req.setBody(data);
        req.setHeader("accept", "application/json");
        req.setHeader("content-type", "application/json");
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof List);
        List<Map<String, Object>> list = (List<Map<String, Object>>) response.getBody();
        Assert.assertEquals(1, list.size());
        Assert.assertEquals(HashMap.class, list.get(0).getClass());
        MultiLevelMap map = new MultiLevelMap(list.get(0));
        Assert.assertEquals("application/json", map.getElement("headers.content-type"));
        Assert.assertEquals("application/json", map.getElement("headers.accept"));
        Assert.assertEquals(false, map.getElement("https"));
        Assert.assertEquals("/api/hello/list", map.getElement("url"));
        Assert.assertEquals("POST", map.getElement("method"));
        Assert.assertEquals("127.0.0.1", map.getElement("ip"));
        Assert.assertEquals(15, map.getElement("timeout"));
        Assert.assertTrue(map.getElement("body") instanceof Map);
        Map<String, Object> received = (Map<String, Object>) map.getElement("body");
        Assert.assertEquals(data, received);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void testXmlResultList() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        SimpleXmlWriter xmlWriter = new SimpleXmlWriter();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("POST");
        req.setUrl("/api/hello/list");
        req.setTargetHost("http://127.0.0.1:"+port);
        Map<String, Object> data = new HashMap<>();
        data.put("hello", "world");
        data.put("test", "message");
        String xml = xmlWriter.write(data);
        req.setBody(xml);
        req.setHeader("accept", "application/xml");
        req.setHeader("content-type", "application/xml");
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
        Assert.assertEquals("application/xml", map.getElement("result.headers.content-type"));
        Assert.assertEquals("application/xml", map.getElement("result.headers.accept"));
        // xml key-values are parsed as text
        Assert.assertEquals("false", map.getElement("result.https"));
        Assert.assertEquals("/api/hello/list", map.getElement("result.url"));
        Assert.assertEquals("POST", map.getElement("result.method"));
        Assert.assertEquals("127.0.0.1", map.getElement("result.ip"));
        Assert.assertEquals("15", map.getElement("result.timeout"));
        Assert.assertTrue(map.getElement("result.body") instanceof Map);
        Map<String, Object> received = (Map<String, Object>) map.getElement("result.body");
        Assert.assertEquals(data, received);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void sendHttpDelete() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("DELETE");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        req.setHeader("accept", "application/json");
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
        Assert.assertEquals("application/json", map.getElement("headers.accept"));
        Assert.assertEquals(false, map.getElement("https"));
        Assert.assertEquals("/api/hello/world", map.getElement("url"));
        Assert.assertEquals("DELETE", map.getElement("method"));
        Assert.assertEquals("127.0.0.1", map.getElement("ip"));
        Assert.assertEquals(10, map.getElement("timeout"));
        Assert.assertNull(map.getElement("body"));
    }

    @Test
    public void sendHttpHeadWithCID() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        String traceId = Utility.getInstance().getDateUuid();
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("HEAD");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        req.setHeader("accept", "application/json");
        req.setHeader("x-correlation-id", traceId);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Map<String, String> headers = response.getHeaders();
        // HTTP head response may include custom headers and content-length
        Assert.assertEquals("HEAD request received", headers.get("X-Response"));
        Assert.assertEquals("100", headers.get("Content-Length"));
        // the same correlation-id is returned to the caller
        Assert.assertEquals(traceId, headers.get("X-Correlation-Id"));
    }

    @Test
    public void sendHttpHeadWithTraceId() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        String traceId = Utility.getInstance().getDateUuid();
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("HEAD");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        req.setHeader("accept", "application/json");
        req.setHeader("x-trace-id", traceId);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Map<String, String> headers = response.getHeaders();
        // HTTP head response may include custom headers and content-length
        Assert.assertEquals("HEAD request received", headers.get("X-Response"));
        Assert.assertEquals("100", headers.get("Content-Length"));
        // the same correlation-id is returned to the caller
        Assert.assertEquals(traceId, headers.get("X-Trace-Id"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void postXmlMap() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("POST");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        Map<String, Object> data = new HashMap<>();
        data.put("hello", "world");
        data.put("test", "message");
        req.setBody(data);
        req.setHeader("accept", "application/json");
        req.setHeader("content-type", "application/xml");
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
        Assert.assertEquals("application/xml", map.getElement("headers.content-type"));
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
    public void postList() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("POST");
        req.setUrl("/api/hello/world");
        req.setTargetHost("http://127.0.0.1:"+port);
        Map<String, Object> data = new HashMap<>();
        data.put("hello", "world");
        data.put("test", "message");
        req.setBody(Collections.singletonList(data));
        req.setHeader("accept", "application/json");
        req.setHeader("content-type", "application/json");
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
        Assert.assertEquals("application/json", map.getElement("headers.content-type"));
        Assert.assertEquals("application/json", map.getElement("headers.accept"));
        Assert.assertEquals(false, map.getElement("https"));
        Assert.assertEquals("/api/hello/world", map.getElement("url"));
        Assert.assertEquals("POST", map.getElement("method"));
        Assert.assertEquals("127.0.0.1", map.getElement("ip"));
        Assert.assertEquals(10, map.getElement("timeout"));
        Assert.assertTrue(map.getElement("body") instanceof List);
        List<Map<String, Object>> received = (List<Map<String, Object>>) map.getElement("body");
        Assert.assertEquals(1, received.size());
        Assert.assertEquals(data, received.get(0));
    }

    @Test
    public void getIndexPage() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        Utility util = Utility.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setUrl("/");
        req.setTargetHost("http://127.0.0.1:"+port);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals("text/html", response.getHeaders().get("Content-Type"));
        Assert.assertTrue(response.getBody() instanceof String);
        String html = (String) response.getBody();
        InputStream in = this.getClass().getResourceAsStream("/public/index.html");
        String content = util.stream2str(in);
        Assert.assertEquals(content, html);
    }

    @Test
    public void getCssPage() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        Utility util = Utility.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setUrl("/sample.css");
        req.setTargetHost("http://127.0.0.1:"+port);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals("text/css", response.getHeaders().get("Content-Type"));
        Assert.assertTrue(response.getBody() instanceof String);
        String html = (String) response.getBody();
        InputStream in = this.getClass().getResourceAsStream("/public/sample.css");
        String content = util.stream2str(in);
        Assert.assertEquals(content, html);
    }

    @Test
    public void getJsPage() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        Utility util = Utility.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("GET");
        req.setUrl("/sample.js");
        req.setTargetHost("http://127.0.0.1:"+port);
        EventEnvelope request = new EventEnvelope().setTo(HTTP_REQUEST).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
        res.onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals("text/javascript", response.getHeaders().get("Content-Type"));
        Assert.assertTrue(response.getBody() instanceof String);
        String html = (String) response.getBody();
        InputStream in = this.getClass().getResourceAsStream("/public/sample.js");
        String content = util.stream2str(in);
        Assert.assertEquals(content, html);
    }

}
