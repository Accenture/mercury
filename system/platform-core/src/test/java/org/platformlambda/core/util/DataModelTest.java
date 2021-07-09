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
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.models.PoJo;

import java.util.*;

public class DataModelTest {

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

}
