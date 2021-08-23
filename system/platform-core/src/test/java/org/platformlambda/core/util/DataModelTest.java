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

package org.platformlambda.core.util;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.WsRegistry;
import org.platformlambda.core.util.models.PoJo;

import javax.websocket.*;
import java.io.IOException;
import java.net.URI;
import java.security.Principal;
import java.util.*;

public class DataModelTest {

    private static final WsRegistry registry = WsRegistry.getInstance();

    @Test
    public void websocketRegistryTest() throws IOException {
        final String SESSION_ID = "12345";
        final String IP_ADDRESS = "10.123.12.3";
        Session noOpSession = new Session() {
            @Override
            public WebSocketContainer getContainer() {
                return null;
            }

            @Override
            public void addMessageHandler(MessageHandler messageHandler) throws IllegalStateException {

            }

            @Override
            public Set<MessageHandler> getMessageHandlers() {
                return null;
            }

            @Override
            public void removeMessageHandler(MessageHandler messageHandler) {

            }

            @Override
            public String getProtocolVersion() {
                return null;
            }

            @Override
            public String getNegotiatedSubprotocol() {
                return null;
            }

            @Override
            public List<Extension> getNegotiatedExtensions() {
                return null;
            }

            @Override
            public boolean isSecure() {
                return false;
            }

            @Override
            public boolean isOpen() {
                return false;
            }

            @Override
            public long getMaxIdleTimeout() {
                return 0;
            }

            @Override
            public void setMaxIdleTimeout(long l) {

            }

            @Override
            public void setMaxBinaryMessageBufferSize(int i) {

            }

            @Override
            public int getMaxBinaryMessageBufferSize() {
                return 0;
            }

            @Override
            public void setMaxTextMessageBufferSize(int i) {

            }

            @Override
            public int getMaxTextMessageBufferSize() {
                return 0;
            }

            @Override
            public RemoteEndpoint.Async getAsyncRemote() {
                return null;
            }

            @Override
            public RemoteEndpoint.Basic getBasicRemote() {
                return null;
            }

            @Override
            public String getId() {
                return SESSION_ID;
            }

            @Override
            public void close() throws IOException {

            }

            @Override
            public void close(CloseReason closeReason) throws IOException {

            }

            @Override
            public URI getRequestURI() {
                return URI.create("/ws/hello");
            }

            @Override
            public Map<String, List<String>> getRequestParameterMap() {
                return null;
            }

            @Override
            public String getQueryString() {
                return "ip="+IP_ADDRESS;
            }

            @Override
            public Map<String, String> getPathParameters() {
                return null;
            }

            @Override
            public Map<String, Object> getUserProperties() {
                return null;
            }

            @Override
            public Principal getUserPrincipal() {
                return null;
            }

            @Override
            public Set<Session> getOpenSessions() {
                return null;
            }

            @Override
            public <T> void addMessageHandler(Class<T> aClass, MessageHandler.Partial<T> partial) throws IllegalStateException {

            }

            @Override
            public <T> void addMessageHandler(Class<T> aClass, MessageHandler.Whole<T> whole) throws IllegalStateException {

            }
        };
        try {
            LambdaFunction noOp = (headers, body, instance) -> true;
            registry.createHandler(noOp, noOpSession);
            String route = registry.getRoute(SESSION_ID);
            Assert.assertTrue(registry.exists(route));
            Assert.assertTrue(route.startsWith("websocket.in."));
            Assert.assertEquals(IP_ADDRESS, registry.get(route).ip);
            Assert.assertTrue(registry.size() > 0);
            Assert.assertTrue(registry.getTxPath(SESSION_ID).startsWith("websocket.out."));
            registry.release(route);
        } catch (Exception e) {
            e.printStackTrace();
            throw e;
        }
    }

    @SuppressWarnings("unchecked")
    @Test
    public void eventEnvelopeSpecialAttributeTest() throws IOException {
        final String TEXT = "hello world";
        final Date now = new Date();
        final String ID = "12345";
        PoJo pojo = new PoJo();
        pojo.setName(TEXT);
        EventEnvelope event = new EventEnvelope();
        event.setId(ID);
        // getError will return null if status is not set
        Assert.assertNull(event.getError());
        event.setStatus(400);
        event.setHeader("date", now);
        event.setHeader("empty", null);
        event.setCorrelationId(ID);
        event.setTraceId(ID);
        event.setExtra(ID);
        event.setBody(pojo);
        Assert.assertEquals("", event.getHeaders().get("empty"));
        Assert.assertTrue(event.hasError());
        Assert.assertEquals(ID, event.getId());
        Assert.assertEquals(ID, event.getExtra());
        Assert.assertEquals(ID, event.getCorrelationId());
        Assert.assertEquals(ID, event.getTraceId());
        byte[] b = event.toBytes();
        EventEnvelope restored = new EventEnvelope(b);
        Assert.assertTrue(restored.getBody() instanceof PoJo);
        Map<String, Object> map = (Map<String, Object>) restored.getRawBody();
        Assert.assertEquals(TEXT, map.get("name"));
        Assert.assertEquals(Integer.valueOf(400), event.getStatus());
        Assert.assertEquals(map.toString(), restored.getError());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void asyncHttpRequestBodyRestoreTest() {
        PoJo pojo = new PoJo();
        pojo.setName("hello");
        pojo.setNumber(123);
        AsyncHttpRequest request = new AsyncHttpRequest();
        request.setBody(pojo);
        Map<String, Object> map = request.toMap();
        AsyncHttpRequest restored = new AsyncHttpRequest(map);
        Assert.assertEquals(PoJo.class, restored.getBody().getClass());
        PoJo restoredBody = (PoJo) restored.getBody();
        Assert.assertEquals(pojo.getName(), restoredBody.getName());
        Assert.assertEquals(pojo.getNumber(), restoredBody.getNumber());
        int NUMBER = 12345;
        AsyncHttpRequest request2 = new AsyncHttpRequest();
        request2.setBody(NUMBER);
        Map<String, Object> map2 = request2.toMap();
        AsyncHttpRequest restored2 = new AsyncHttpRequest(map2);
        Assert.assertEquals(NUMBER, restored2.getBody());
        Map<String, Object> restoredNumberInMap = SimpleMapper.getInstance().getMapper().readValue(NUMBER, Map.class);
        Assert.assertEquals(1, restoredNumberInMap.size());
        Assert.assertEquals(NUMBER, restoredNumberInMap.get("result"));

        List<Object> restoredNumberInList = SimpleMapper.getInstance().getMapper().readValue(NUMBER, List.class);
        Assert.assertEquals(1, restoredNumberInList.size());
        Assert.assertEquals(NUMBER, restoredNumberInList.get(0));
    }

    @Test(expected = IllegalArgumentException.class)
    public void rejectCastingOfPrimitiveToPoJo() {
        SimpleMapper.getInstance().getMapper().readValue(true, PoJo.class);
    }

    @Test
    public void asyncHttpModelTest() {
        AsyncHttpRequest request = new AsyncHttpRequest();
        request.setUrl("http://127.0.0.1:8080/api/hello/world");
        request.setMethod("POST");
        request.setFileName("test.txt");
        request.setContentLength(12345);
        request.setPathParameter("a", "b");
        request.setQueryString("x=y");
        request.setQueryParameter("x", "y");
        request.setRemoteIp("127.0.0.1");
        request.setSecure(false);
        request.setBody("test message");
        request.setSessionInfo("user", "someone");
        request.setTrustAllCert(false);
        request.setStreamRoute("test.101010");
        request.setUploadTag("file");
        request.setTimeoutSeconds(30);
        request.setCookie("hi", "there");
        request.setHeader("x-api-key", "hello");
        request.setTargetHost("http://localhost:8085");

        Map<String, Object> data = request.toMap();

        AsyncHttpRequest restored = new AsyncHttpRequest(data);
        Assert.assertEquals(request.getUrl(), restored.getUrl());
        Assert.assertEquals(request.getMethod(), restored.getMethod());
        Assert.assertEquals(request.getHeaders(), restored.getHeaders());
        Assert.assertEquals(request.getHeader("x-api-key"), restored.getHeader("x-api-key"));
        Assert.assertEquals(request.getPathParameters(), restored.getPathParameters());
        Assert.assertEquals(request.getPathParameter("a"), restored.getPathParameter("a"));
        Assert.assertEquals(request.getFileName(), restored.getFileName());
        Assert.assertEquals(request.getContentLength(), restored.getContentLength());
        Assert.assertEquals(request.getPathParameter("a"), restored.getPathParameter("a"));
        Assert.assertEquals(request.getQueryString(), restored.getQueryString());
        Assert.assertEquals(request.getQueryParameters(), restored.getQueryParameters());
        Assert.assertEquals(request.getQueryParameter("x"), restored.getQueryParameter("x"));
        Assert.assertEquals(request.getRemoteIp(), restored.getRemoteIp());
        Assert.assertEquals(request.isSecure(), restored.isSecure());
        Assert.assertEquals(request.isFile(), restored.isFile());
        Assert.assertEquals(request.getBody(), restored.getBody());
        Assert.assertEquals(request.getRawBody(), restored.getBody());
        Assert.assertEquals(request.getSessionInfo(), restored.getSessionInfo());
        Assert.assertEquals(request.getSessionInfo("user"), restored.getSessionInfo("user"));
        Assert.assertEquals(request.isTrustAllCert(), restored.isTrustAllCert());
        Assert.assertEquals(request.isStream(), restored.isStream());
        Assert.assertEquals(request.getStreamRoute(), restored.getStreamRoute());
        Assert.assertEquals(request.getUploadTag(), restored.getUploadTag());
        Assert.assertEquals(request.getTimeoutSeconds(), restored.getTimeoutSeconds());
        Assert.assertEquals(request.getCookies(), restored.getCookies());
        Assert.assertEquals(request.getCookie("hi"), restored.getCookie("hi"));
        Assert.assertEquals(request.getTargetHost(), restored.getTargetHost());
        Assert.assertNull(restored.getClassType());

        request.removeCookie("hi");
        Assert.assertTrue(request.getCookies().isEmpty());
        request.removePathParameter("a");
        Assert.assertTrue(request.getPathParameters().isEmpty());
        request.removeQueryParameter("x");
        Assert.assertTrue(request.getQueryParameters().isEmpty());
        request.removeSessionInfo("user");
        Assert.assertTrue(request.getSessionInfo().isEmpty());

        request.setQueryParameter("hello", Collections.singletonList("test"));
        Assert.assertEquals(request.getQueryParameter("hello"), "test");
        Assert.assertEquals(request.getQueryParameters("hello"), Collections.singletonList("test"));
    }

    @Test
    public void appExceptionTest() {
        AppException ex = new AppException(400, "demo");
        Assert.assertEquals(400, ex.getStatus());
        Assert.assertEquals("demo", ex.getMessage());
    }

    @Test(expected = IllegalArgumentException.class)
    public void envelopeAsInputNotAllowed() {
        EventEnvelope one = new EventEnvelope();
        EventEnvelope two = new EventEnvelope();
        one.setBody(two);
    }

}
