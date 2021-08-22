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

package org.platformlambda.util;

import com.google.api.client.http.*;
import com.google.api.client.http.javanet.NetHttpTransport;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.io.InputStream;
import java.util.HashMap;
import java.util.Map;

public class SimpleHttpRequests {

    private static final HttpTransport HTTP_TRANSPORT = new NetHttpTransport();
    private static final HttpRequestFactory factory = HTTP_TRANSPORT.createRequestFactory();
    private static final String APPLICATION_JSON = "application/json";

    public static Object get(String url) throws IOException, AppException {
        return get(url, APPLICATION_JSON);
    }

    public static Object get(String url, String accept) throws IOException, AppException {
        return get(url, accept, new HashMap<>());
    }

    public static Object get(String url, Map<String, String> reqHeaders) throws IOException, AppException {
        return get(url, APPLICATION_JSON, reqHeaders);
    }

    public static Object get(String url, String accept, Map<String, String> reqHeaders)
            throws IOException, AppException {
        GenericUrl target = new GenericUrl(url);
        HttpHeaders headers = new HttpHeaders();
        headers.setAccept(accept);
        for (String h: reqHeaders.keySet()) {
            String v = reqHeaders.get(h);
            headers.put(h, v);
        }
        HttpRequest request = factory.buildGetRequest(target).setHeaders(headers);
        try {
            HttpResponse response = request.execute();
            InputStream in = response.getContent();
            return Utility.getInstance().stream2str(in);
        } catch (HttpResponseException e) {
            throw new AppException(e.getStatusCode(), e.getContent());
        }
    }

    public static Object post(String url, Map<String, String> reqHeaders, Map<String, Object> data)
            throws IOException, AppException {
        SimpleMapper mapper = SimpleMapper.getInstance();
        String json = mapper.getMapper().writeValueAsString(data);
        ByteArrayContent content = ByteArrayContent.fromString(APPLICATION_JSON, json);
        GenericUrl target = new GenericUrl(url);
        HttpHeaders headers = new HttpHeaders();
        headers.setAccept(APPLICATION_JSON);
        for (String h: reqHeaders.keySet()) {
            String v = reqHeaders.get(h);
            headers.put(h, v);
        }
        HttpRequest request = factory.buildPostRequest(target, content).setHeaders(headers);
        try {
            HttpResponse response = request.execute();
            InputStream in = response.getContent();
            return Utility.getInstance().stream2str(in);
        } catch (HttpResponseException e) {
            throw new AppException(e.getStatusCode(), e.getContent());
        }
    }

}
