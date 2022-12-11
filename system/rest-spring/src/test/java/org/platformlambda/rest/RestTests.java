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

package org.platformlambda.rest;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.common.TestBase;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.util.SimpleHttpRequests;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class RestTests extends TestBase {

    @Test
    @SuppressWarnings("unchecked")
    public void contactHelloWorld() throws IOException, AppException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Content-Type", "application/json");
        headers.put("Accept", "application/json");
        Map<String, Object> data = new HashMap<>();
        data.put("hello", "world");
        Object response = SimpleHttpRequests.put("http://127.0.0.1:"+port+"/api/hello/world", headers, data);
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("Hello World", result.get("name"));
        Assert.assertEquals("100 World Blvd, Earth", result.get("address"));
        Assert.assertEquals("123-456-7890", result.get("telephone"));
        Assert.assertEquals("Congratulations! Hello world example endpoint is working fine", result.get("message"));
        Assert.assertEquals(data, result.get("data"));
        headers.put("Content-Type", "application/json");
        headers.put("Accept", "application/xml");
        response = SimpleHttpRequests.put("http://127.0.0.1:"+port+"/api/hello/world", headers, data);
        Assert.assertTrue(response instanceof String);
        Assert.assertTrue(((String) response).startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        headers.put("Content-Type", "application/xml");
        headers.put("Accept", "application/xml");
        response = SimpleHttpRequests.putXml("http://127.0.0.1:"+port+"/api/hello/world", headers, data);
        Assert.assertTrue(response instanceof String);
        Assert.assertTrue(((String) response).startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(((String) response).contains("<hello>world</hello>"));
        response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world", "text/html");
        Assert.assertTrue(response instanceof String);
        Assert.assertTrue(((String) response).startsWith("<html><body><pre>"));
        response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world", "application/xml");
        Assert.assertTrue(response instanceof String);
        Assert.assertTrue(((String) response).startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world", "text/plain");
        Assert.assertTrue(response instanceof String);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void http404Json() {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "application/json");
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/no_path", headers));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Not Found", result.get("message"));
    }

    @Test
    public void http404Xml() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_path", "application/xml"));
        Assert.assertEquals(404, ex.getStatus());
        String result = ex.getMessage();
        Assert.assertTrue(result.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(result.contains("<status>404</status>"));
    }

    @Test
    public void http404Html() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_path", "text/html"));
        Assert.assertEquals(404, ex.getStatus());
        String result = ex.getMessage();
        Assert.assertTrue(result.startsWith("<!DOCTYPE html>"));
        Assert.assertTrue(result.contains("HTTP-404"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void http404Text() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_path", "text/plain"));
        Assert.assertEquals(404, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Not Found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void methodNotAllowedCase() {
        AppException ex = Assert.assertThrows(AppException.class, () -> {
            Map<String, String> headers = new HashMap<>();
            headers.put("Accept", "application/json");
            SimpleHttpRequests.post("http://127.0.0.1:"+port+"/api/hello/world", headers, new HashMap<>());
        });
        Assert.assertEquals(405, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(405, result.get("status"));
        Assert.assertEquals("Method Not Allowed", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void unsupportedMediaTypeCase() {
        AppException ex = Assert.assertThrows(AppException.class, () -> {
            Map<String, String> headers = new HashMap<>();
            headers.put("Content-Type", "text/plain");
            headers.put("Accept", "application/json");
            SimpleHttpRequests.putText("http://127.0.0.1:"+port+"/api/hello/world", "application/json",
                    headers, "test");
        });
        Assert.assertEquals(415, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(415, result.get("status"));
        Assert.assertEquals("Unsupported Media Type", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void badRequestCase() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world?test=400", "application/json"));
        Assert.assertEquals(400, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("test", result.get("message"));
    }

    @Test
    public void unauthorizedCase() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world?test=401", "text/html"));
        Assert.assertEquals(401, ex.getStatus());
        String result = ex.getMessage();
        Assert.assertTrue(result.startsWith("<!DOCTYPE html>"));
        Assert.assertTrue(result.contains("HTTP-401"));
        Assert.assertTrue(result.contains("Unauthorized"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void forbiddenCase() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world?test=403", "text/plain"));
        Assert.assertEquals(403, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(403, result.get("status"));
        Assert.assertEquals("Forbidden", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void notAcceptableCase() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world?test=406", "application/json"));
        Assert.assertEquals(406, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(406, result.get("status"));
        Assert.assertEquals("Not acceptable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void notAvailableCase() {
        AppException ex = Assert.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world?test=503", "application/json"));
        Assert.assertEquals(503, ex.getStatus());
        String error = ex.getMessage();
        Assert.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assert.assertEquals(503, result.get("status"));
        Assert.assertEquals("System temporarily unavailable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void getJsonInText() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/json",
                "application/json");
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("Hello World", result.get("name"));
    }

    @Test
    public void getXmlInText() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/xml",
                "application/xml");
        Assert.assertTrue(response instanceof String);
        String text = (String) response;
        Assert.assertTrue(text.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(text.contains("<hello>xml</hello>"));
    }

    @Test
    public void getXmlInLists() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/list",
                "application/xml");
        Assert.assertTrue(response instanceof String);
        String text = (String) response;
        Assert.assertTrue(text.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(text.contains("<item>three</item>"));
    }

}
