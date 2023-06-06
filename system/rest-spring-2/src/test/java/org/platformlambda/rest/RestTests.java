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

package org.platformlambda.rest;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.common.TestBase;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlParser;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class RestTests extends TestBase {

    private static final SimpleXmlParser xml = new SimpleXmlParser();

    @Test
    @SuppressWarnings("unchecked")
    public void contactHelloWorld() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Content-Type", "application/json");
        headers.put("Accept", "application/json");
        Map<String, Object> data = new HashMap<>();
        data.put("hello", "world");
        EventEnvelope response = httpPut("http://127.0.0.1:"+port, "/api/hello/world", headers, data);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals("Hello World", result.get("name"));
        Assert.assertEquals("100 World Blvd, Earth", result.get("address"));
        Assert.assertEquals("123-456-7890", result.get("telephone"));
        Assert.assertEquals("Congratulations! Hello world example endpoint is working fine", result.get("message"));
        Assert.assertEquals(data, result.get("data"));
        // both json and xml are converted to map
        headers.put("Content-Type", "application/json");
        headers.put("Accept", "application/xml");
        EventEnvelope response2 = httpPut("http://127.0.0.1:"+port, "/api/hello/world", headers, data);
        assert response2 != null;
        Assert.assertTrue(response2.getBody() instanceof Map);
        Map<String, Object> result2 = (Map<String, Object>) response2.getBody();
        Assert.assertEquals("Hello World", result2.get("name"));
        Assert.assertEquals("100 World Blvd, Earth", result2.get("address"));
        Assert.assertEquals("123-456-7890", result2.get("telephone"));
        Assert.assertEquals("Congratulations! Hello world example endpoint is working fine", result2.get("message"));
        Assert.assertEquals(data, result2.get("data"));
        // xml are converted to map
        headers.put("Content-Type", "application/xml");
        headers.put("Accept", "application/xml");
        EventEnvelope response3 = httpPut("http://127.0.0.1:"+port, "/api/hello/world", headers, data);
        assert response3 != null;
        Assert.assertTrue(response3.getBody() instanceof Map);
        Map<String, Object> result3 = (Map<String, Object>) response3.getBody();
        Assert.assertEquals("Hello World", result3.get("name"));
        Assert.assertEquals("100 World Blvd, Earth", result3.get("address"));
        Assert.assertEquals("123-456-7890", result3.get("telephone"));
        Assert.assertEquals("Congratulations! Hello world example endpoint is working fine", result3.get("message"));
        Assert.assertEquals(data, result3.get("data"));
        headers.put("x-raw-xml", "true");
        EventEnvelope response4 = httpPut("http://127.0.0.1:"+port, "/api/hello/world", headers, data);
        assert response4 != null;
        // when x-raw-xml header is set to "true", xml is returned as a string
        Assert.assertTrue(response4.getBody() instanceof String);
        Map<String, Object> result4 = xml.parse((String) response4.getBody());
        Assert.assertEquals("Hello World", result4.get("name"));
        Assert.assertEquals("100 World Blvd, Earth", result4.get("address"));
        Assert.assertEquals("123-456-7890", result4.get("telephone"));
        Assert.assertEquals("Congratulations! Hello world example endpoint is working fine", result4.get("message"));
        Assert.assertEquals(data, result4.get("data"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void http404Json() throws IOException, InterruptedException {
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/no_path", null);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Not Found", result.get("message"));
    }

    @Test
    public void http404Xml() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/xml");
        headers.put("x-raw-xml", "true");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/no_path", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        String result = (String) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertTrue(result.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(result.contains("<status>404</status>"));
    }

    @Test
    public void http404Html() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "text/html");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/no_path", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        String result = (String) response.getBody();
        Assert.assertEquals(404, response.getStatus());
        Assert.assertTrue(result.startsWith("<!DOCTYPE html>"));
        Assert.assertTrue(result.contains("HTTP-404"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void http404Text() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "text/plain");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/no_path", headers);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof String);
        Assert.assertEquals(404, response.getStatus());
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response.getBody(), Map.class);
        Assert.assertEquals(404, result.get("status"));
        Assert.assertEquals("Not Found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void methodNotAllowedCase() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpPost("http://127.0.0.1:"+port, "/api/hello/world", headers, new HashMap<>());
        assert response != null;
        Assert.assertEquals(405, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(405, result.get("status"));
        Assert.assertEquals("Method Not Allowed", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void unsupportedMediaTypeCase() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Content-Type", "text/plain");
        headers.put("accept", "application/json");
        EventEnvelope response = httpPut("http://127.0.0.1:"+port, "/api/hello/world", headers, "test");
        assert response != null;
        Assert.assertEquals(415, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(415, result.get("status"));
        Assert.assertEquals("Unsupported Media Type", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void badRequestCase() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/hello/world?test=400", headers);
        assert response != null;
        Assert.assertEquals(400, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(400, result.get("status"));
        Assert.assertEquals("test", result.get("message"));
    }

    @Test
    public void unauthorizedCase() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "text/html");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/hello/world?test=401", headers);
        assert response != null;
        Assert.assertEquals(401, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof String);
        String result = (String) response.getBody();
        Assert.assertTrue(result.startsWith("<!DOCTYPE html>"));
        Assert.assertTrue(result.contains("HTTP-401"));
        Assert.assertTrue(result.contains("Unauthorized"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void forbiddenCase() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "text/plain");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/hello/world?test=403", headers);
        assert response != null;
        Assert.assertEquals(403, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response.getBody(), Map.class);
        Assert.assertEquals(403, result.get("status"));
        Assert.assertEquals("Forbidden", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void notAcceptableCase() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/hello/world?test=406", headers);
        assert response != null;
        Assert.assertEquals(406, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(406, result.get("status"));
        Assert.assertEquals("Not acceptable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void notAvailableCase() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/hello/world?test=503", headers);
        assert response != null;
        Assert.assertEquals(503, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(503, result.get("status"));
        Assert.assertEquals("System temporarily unavailable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void getJsonFromText2Map() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/json");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/hello/json", headers);
        assert response != null;
        Assert.assertEquals(200, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals("Hello World", result.get("name"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void getXmlFromText2Map() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/xml");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/hello/xml", headers);
        assert response != null;
        Assert.assertEquals(200, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals("xml", result.get("hello"));
    }

    @Test
    public void getRawXml() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/xml");
        headers.put("x-raw-xml", "true");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/hello/xml", headers);
        assert response != null;
        Assert.assertEquals(200, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof String);
        Map<String, Object> result = xml.parse((String) response.getBody());
        Assert.assertEquals("xml", result.get("hello"));
    }

    @Test
    public void getRawXmlInList() throws IOException, InterruptedException {
        Map<String, String> headers = new HashMap<>();
        headers.put("accept", "application/xml");
        headers.put("x-raw-xml", "true");
        EventEnvelope response = httpGet("http://127.0.0.1:"+port, "/api/hello/list", headers);
        assert response != null;
        Assert.assertEquals(200, response.getStatus());
        Assert.assertTrue(response.getBody() instanceof String);
        String text = (String) response.getBody();
        Assert.assertTrue(text.startsWith("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        Assert.assertTrue(text.contains("<item>three</item>"));
    }
}
