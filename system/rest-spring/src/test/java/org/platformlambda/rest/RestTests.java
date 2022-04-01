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

    @Test(expected = AppException.class)
    public void http404Json() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "application/json");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/no_path", headers);
    }

    @Test(expected = AppException.class)
    public void http404Xml() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "application/xml");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/no_path", headers);
    }

    @Test(expected = AppException.class)
    public void http404Html() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "text/html");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/no_path", headers);
    }

    @Test(expected = AppException.class)
    public void http404Text() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "text/plain");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/no_path", headers);
    }

    @Test(expected = AppException.class)
    public void methodNotAllowedCase() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Accept", "application/json");
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/api/hello/world", headers, new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void unsupportedMediaTypeCase() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("Content-Type", "text/plain");
        headers.put("Accept", "application/json");
        SimpleHttpRequests.putText("http://127.0.0.1:"+port+"/api/hello/world", "application/json",
                headers, "test");
    }

    @Test(expected = AppException.class)
    public void badRequestCase() throws AppException, IOException {
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world?test=400", "application/json");
    }

    @Test(expected = AppException.class)
    public void unauthorizedCase() throws AppException, IOException {
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world?test=401", "application/json");
    }

    @Test(expected = AppException.class)
    public void forbiddenCase() throws AppException, IOException {
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world?test=403", "application/json");
    }

    @Test(expected = AppException.class)
    public void notAcceptableCase() throws AppException, IOException {
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world?test=406", "application/json");
    }

    @Test(expected = AppException.class)
    public void notAvailableCase() throws AppException, IOException {
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/api/hello/world?test=503", "application/json");
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
