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

package org.platformlambda.core;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import org.platformlambda.core.models.AsyncHttpRequest;

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

        AsyncHttpRequest restored = new AsyncHttpRequest(request.toMap());
        Assertions.assertEquals(HELLO, restored.getBody());
        Assertions.assertEquals(WORLD, restored.getHeader(HELLO));
        Assertions.assertEquals(WORLD, restored.getCookie(HELLO));
        Assertions.assertEquals(WORLD, restored.getPathParameter(HELLO));
        Assertions.assertEquals(WORLD, restored.getQueryParameter(HELLO));
        Assertions.assertEquals(PUT, restored.getMethod());
        Assertions.assertEquals("none", restored.getFileName());
        Assertions.assertEquals("127.0.0.1", restored.getRemoteIp());
        Assertions.assertFalse(restored.isSecure());
        Assertions.assertEquals("none", restored.getStreamRoute());
        Assertions.assertEquals(WORLD, restored.getSessionInfo(HELLO));
        Assertions.assertFalse(restored.isTrustAllCert());
        Assertions.assertEquals(10, restored.getTimeoutSeconds());
        Assertions.assertEquals("http://localhost", restored.getTargetHost());
        Assertions.assertEquals("file", restored.getUploadTag());
        Assertions.assertEquals("/api/hello", restored.getUrl());
    }
}
