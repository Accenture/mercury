package org.platformlambda.core.util;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.AsyncHttpRequest;

import java.util.Collections;
import java.util.Map;

public class DataModelTest {

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
