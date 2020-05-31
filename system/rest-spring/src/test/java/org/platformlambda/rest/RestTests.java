package org.platformlambda.rest;

import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.util.SimpleHttpRequests;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.serializers.SimpleMapper;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class RestTests {

    private String port;

    @Before
    public void setup() {
        AppConfigReader config = AppConfigReader.getInstance();
        port = config.getProperty("server.port", "8080");
        String[] args = new String[0];
        RestServer.main(args);
    }

    @Test
    @SuppressWarnings("unchecked")
    public void contactHelloWorld() throws IOException, AppException {
        Map<String, Object> data = new HashMap<>();
        data.put("hello", "world");
        Object response = SimpleHttpRequests.put("http://127.0.0.1:"+port+"/api/hello/world", data);
        Assert.assertTrue(response instanceof String);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("Hello World", result.get("name"));
        Assert.assertEquals("100 World Blvd, Earth", result.get("address"));
        Assert.assertEquals("123-456-7890", result.get("telephone"));
        Assert.assertEquals("Congratulations! Hello world example endpoint is working fine", result.get("message"));
        Assert.assertEquals(data, result.get("data"));
    }

}
