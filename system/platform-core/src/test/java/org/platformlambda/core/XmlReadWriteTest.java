/*

    Copyright 2018-2024 Accenture Technology

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

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.serializers.SimpleXmlParser;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.io.InputStream;
import java.util.*;

public class XmlReadWriteTest {

    private static final SimpleXmlParser parser = new SimpleXmlParser();
    private static final SimpleXmlWriter writer = new SimpleXmlWriter();

    @SuppressWarnings("unchecked")
    @Test
    public void readWriteTest() throws IOException {
        Utility util = Utility.getInstance();
        Date now = new Date();
        Map<String, Object> inner = new HashMap<>();
        inner.put("inner", "internal");
        inner.put("time", now);
        List<Object> list = new ArrayList<>();
        list.add(1);
        list.add(2);
        list.add("");   // empty element will be converted to null
        list.add(inner);
        list.add(3);
        list.add("test");
        list.add("");   // empty element at the end of the array will be dropped
        Map<String, Object> data = new HashMap<>();
        data.put("hello", "world");
        data.put("lists", list);
        data.put("single", Collections.singletonList("one"));
        String basic = writer.write(data);
        List<String> basicLines = util.split(basic, "\r\n");
        Assert.assertTrue(basicLines.size() > 2);
        Assert.assertEquals("<root>", basicLines.get(1));
        // set root as "result"
        String xml = writer.write("result", data);
        List<String> raw = util.split(xml, "\r\n");
        List<String> lines = new ArrayList<>();
        raw.forEach(line -> lines.add(line.trim()));
        Assert.assertTrue(lines.size() > 2);
        Assert.assertEquals("<?xml version=\"1.0\" encoding=\"UTF-8\"?>", lines.get(0));
        Assert.assertEquals("<result>", lines.get(1));
        Assert.assertTrue(lines.contains("<single>one</single>"));
        Map<String, Object> result = parser.parse(xml);
        Assert.assertEquals("one", result.get("single"));
        Assert.assertTrue(result.get("lists") instanceof List);
        List<Object> mixedList = (List<Object>) result.get("lists");
        // the 7th element in the array is dropped
        Assert.assertEquals(6, mixedList.size());
        MultiLevelMap multi = new MultiLevelMap(result);
        // empty array element is saved as null
        Assert.assertNull(multi.getElement("lists[2]"));
        Assert.assertEquals("internal", multi.getElement("lists[3].inner"));
        Assert.assertEquals(util.date2str(now), multi.getElement("lists[3].time"));
        Assert.assertEquals("3", multi.getElement("lists[4]"));
        Assert.assertEquals("test", multi.getElement("lists[5]"));
        // xml without array
        try (InputStream in = this.getClass().getResourceAsStream("/log4j2.xml")) {
            MultiLevelMap mm = new MultiLevelMap(parser.parse(in));
            Assert.assertEquals("console", mm.getElement("Appenders.name"));
            Assert.assertEquals("false", mm.getElement("Loggers.additivity"));
            Assert.assertEquals("console", mm.getElement("Loggers.Root.ref"));
        }
    }

    @Test
    public void adlsWithArrayTest() throws IOException {
        try (InputStream in = this.getClass().getResourceAsStream("/sample_adls_response.xml")) {
            MultiLevelMap mm = new MultiLevelMap(parser.parse(in));
            Assert.assertEquals("\"0x8D90F50C8DD6E2A\"",
                    mm.getElement("Containers.Container[0].Properties.Etag"));
            Assert.assertEquals("\"0x8D9934CF1AD9D12\"",
                    mm.getElement("Containers.Container[1].Properties.Etag"));
            Assert.assertEquals("hello", mm.getElement("Containers.Container[0].Name"));
            Assert.assertEquals("test", mm.getElement("Containers.Container[1].Name"));
        }
    }

    @Test
    public void adlsWithoutArrayTest() throws IOException {
        try (InputStream in = this.getClass().getResourceAsStream("/sample_adls_with_one_container.xml")) {
            MultiLevelMap mm = new MultiLevelMap(parser.parse(in));
            // an array of one element will be rendered as a regular element instead of an array
            Assert.assertEquals("\"0x8D90F50C8DD6E2A\"",
                    mm.getElement("Containers.Container.Properties.Etag"));
            Assert.assertEquals("hello", mm.getElement("Containers.Container.Name"));
        }
    }

}
