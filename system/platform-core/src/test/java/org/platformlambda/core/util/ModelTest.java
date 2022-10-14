package org.platformlambda.core.util;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.models.AsyncHttpRequest;

import java.util.Map;

public class ModelTest {

    @Test
    public void asyncHttpRequestModel() {
        final String HELLO = "hello";
        final String WORLD = "world";
        final String PUT = "PUT";
        AsyncHttpRequest request = new AsyncHttpRequest();
        request.setContentLength(HELLO.length());
        request.setBody(HELLO);
        request.setHeader(HELLO, WORLD);
        request.setCookie(HELLO, WORLD);
        request.setMethod(PUT);
        request.setFileName("none");
        request.setPathParameter(HELLO, WORLD);
        request.setQueryString(HELLO+"="+WORLD);
        request.setQueryParameter(HELLO, WORLD);
        request.setRemoteIp("127.0.0.1");
        request.setSecure(false);
        request.setStreamRoute("none");
        request.setSessionInfo(HELLO, WORLD);
        request.setTrustAllCert(false);
        request.setTimeoutSeconds(10);
        request.setTargetHost("http://localhost");
        request.setUploadTag("file");
        request.setUrl("/api/hello");

        Map<String, Object> map = request.toMap();
        AsyncHttpRequest restored = new AsyncHttpRequest(map);
        Assert.assertEquals(HELLO, restored.getBody());
        Assert.assertEquals(WORLD, restored.getHeader(HELLO));
        Assert.assertEquals(WORLD, restored.getCookie(HELLO));
        Assert.assertEquals(WORLD, restored.getPathParameter(HELLO));
        Assert.assertEquals(WORLD, restored.getQueryParameter(HELLO));
        Assert.assertEquals(PUT, restored.getMethod());
        Assert.assertEquals("none", restored.getFileName());
        Assert.assertEquals("127.0.0.1", restored.getRemoteIp());
        Assert.assertFalse(restored.isSecure());
        Assert.assertEquals("none", restored.getStreamRoute());
        Assert.assertEquals(WORLD, restored.getSessionInfo(HELLO));
        Assert.assertFalse(restored.isTrustAllCert());
        Assert.assertEquals(10, restored.getTimeoutSeconds());
        Assert.assertEquals("http://localhost", restored.getTargetHost());
        Assert.assertEquals("file", restored.getUploadTag());
        Assert.assertEquals("/api/hello", restored.getUrl());
    }
}
