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

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.serializers.SimpleXmlParser;
import org.platformlambda.core.serializers.SimpleXmlWriter;

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
        list.add(inner);
        list.add(3);
        list.add("test");
        Map<String, Object> data = new HashMap<>();
        data.put("hello", "world");
        data.put("list", list);
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
        Assert.assertTrue(result.get("list") instanceof List);
        List<Object> mixedList = (List<Object>) result.get("list");
        Assert.assertEquals(5, mixedList.size());
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("internal", multi.getElement("list[2].inner"));
        Assert.assertEquals(util.date2str(now), multi.getElement("list[2].time"));
        // xml without array
        try (InputStream in = this.getClass().getResourceAsStream("/log4j2.xml")) {
            Map<String, Object> log4j = parser.parse(in);
            MultiLevelMap m2 = new MultiLevelMap(log4j);
            Assert.assertEquals("console", m2.getElement("Appenders.name"));
            Assert.assertEquals("false", m2.getElement("Loggers.additivity"));
            Assert.assertEquals("console", m2.getElement("Loggers.Root.ref"));
        }



    }
}
