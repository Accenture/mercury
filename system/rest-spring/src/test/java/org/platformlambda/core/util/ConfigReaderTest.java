package org.platformlambda.core.util;

import org.junit.Test;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.UUID;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

public class ConfigReaderTest {

    @Test
    public void environmentVarSubstitution() throws IOException {
        ConfigReader reader = new ConfigReader();
        reader.load("classpath:/test.properties");
        /*
         * the original value in test.properties for "path" is "hello world".
         * However, it should be replaced by the environment variable "PATH" automatically.
         */
        String path = System.getenv("PATH");
        assertEquals(path, reader.getProperty("path"));
        /*
         * test environment variable mapping
         * env.variables=PATH:path.too
         * path.too=hello world
         */
        assertEquals(path, reader.getProperty("path.too"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void dotFormatterTest() throws IOException {
        ConfigReader reader = new ConfigReader();
        reader.load("classpath:/test.yaml");
        MultiLevelMap formatter = new MultiLevelMap(reader.getMap());
        Object o = formatter.getElement("hello.world");
        assertEquals("some value", o);
        o = formatter.getElement("hello.multiline");
        assertTrue(o instanceof String);
        assertTrue(o.toString().contains("\n"));
        o = formatter.getElement("hello.array");
        assertTrue(o instanceof ArrayList);
        List<String> elements = (List<String>) o;
        assertEquals(2, elements.size());
        assertEquals("hi", elements.get(0));
        assertEquals("this is great", elements.get(1));
        o = formatter.getElement("hello.array[0]");
        assertEquals("hi", o);
        o = formatter.getElement("hello.array[1]");
        assertEquals("this is great", o);
    }

    @Test
    public void flattenMapTest() throws IOException {
        ConfigReader reader = new ConfigReader();
        reader.load("classpath:/test.yaml");
        Map<String, Object> map = Utility.getInstance().getFlatMap(reader.getMap());
        assertEquals("some value", map.get("hello.world"));
        assertEquals("hi", map.get("hello.array[0]"));
        assertEquals("this is great", map.get("hello.array[1]"));
        /*
         * Unlike properties file that converts values into strings,
         * YAML and JSON preserve original objects.
         */
        Object o = map.get("hello.number");
        assertTrue(o instanceof Integer);
        assertEquals(12345, o);
    }

    @Test
    public void appConfigTest() {
        AppConfigReader reader = AppConfigReader.getInstance();
        assertEquals("rest-spring", reader.getProperty("spring.application.name"));
    }

    @Test
    public void parameterSubstitutionTest() throws IOException {
        ConfigReader reader = new ConfigReader();
        reader.load("classpath:/test.yaml");
        reader.flattenMap();
        assertEquals("rest-spring", reader.getProperty("hello.name"));
        assertEquals("/tmp/lambda/apps", reader.getProperty("hello.location[1]"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void dotFormatterSetTest() throws IOException {
        // generate random top level key
        String uuid = UUID.randomUUID().toString().replace("-", "");
        String goodDay = uuid+".great.day";
        String goodArray = uuid+".array";
        String message = "test message";
        Integer input = 123456789;
        ConfigReader reader = new ConfigReader();
        reader.load("classpath:/test.yaml");
        int size = reader.getMap().size();
        MultiLevelMap formatter = new MultiLevelMap(reader.getMap());
        formatter.setElement(goodDay, input).setElement(goodArray+"[1]", message);
        Object o = formatter.getElement(goodDay);
        assertEquals(input, o);
        // confirm added only one key at the top level
        assertEquals(size+1, formatter.getMap().size());
        assertEquals(null, formatter.getElement(goodArray+"[0]"));
        assertEquals(message, formatter.getElement(goodArray+"[1]"));

        o = formatter.getElement(uuid);
        assertTrue(o instanceof Map);
        Map<String, Object> submap = (Map<String, Object>) o;
        assertEquals(2, submap.size());
        assertTrue(submap.containsKey("great"));
        assertTrue(submap.containsKey("array"));
    }

    @Test(expected=IOException.class)
    public void resourceNotFound() throws IOException {
        ConfigReader reader = new ConfigReader();
        reader.load("classpath:/notfound.yaml");
    }

    @Test(expected=IOException.class)
    public void fileNotFound() throws IOException {
        ConfigReader reader = new ConfigReader();
        reader.load("file:/notfound.yaml");
    }

    @Test
    public void jsonReadTest() throws IOException {
        ConfigReader reader = new ConfigReader();
        reader.load("classpath:/test.json");
        assertEquals(2, reader.getMap().size());
        assertEquals("world", reader.get("hello"));
        assertEquals("message", reader.get("test"));
    }

}