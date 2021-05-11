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

import com.google.api.client.http.*;
import com.google.api.client.http.javanet.NetHttpTransport;
import org.junit.Test;

import java.io.IOException;
import java.io.InputStream;

import org.junit.Assert;

public class HttpClientTest {

    private static final HttpTransport HTTP_TRANSPORT = new NetHttpTransport();
    private static final HttpRequestFactory factory = HTTP_TRANSPORT.createRequestFactory();

    @Test
    public void askGoogle() throws IOException {
        GenericUrl target = new GenericUrl("https://www.google.com");
        HttpRequest request = factory.buildGetRequest(target);
        HttpResponse response = request.execute();
        int rc = response.getStatusCode();
        String statusMessage = response.getStatusMessage();
        InputStream in = response.getContent();
        String result = Utility.getInstance().stream2str(in);
        Assert.assertTrue(result.startsWith("<!doctype html>"));
        Assert.assertEquals(200, rc);
        Assert.assertEquals("OK", statusMessage);
    }
}