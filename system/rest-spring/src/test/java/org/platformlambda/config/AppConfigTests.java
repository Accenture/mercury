package org.platformlambda.config;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.util.AppConfigReader;

import java.util.List;

public class AppConfigTests {

    @Test
    public void verifyThatJaxRsExampleIsThere() {
        AppConfigReader reader = AppConfigReader.getInstance();
        String value = reader.getProperty("jaxrs.example.enabled");
        Assert.assertEquals("true", value);
        Object helloWorld = reader.get("hello.world.array");
        Assert.assertTrue(helloWorld instanceof List);
    }
    
}
