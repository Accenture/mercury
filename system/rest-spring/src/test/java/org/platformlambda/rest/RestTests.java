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

}
