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
import org.junit.Before;
import org.junit.Test;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.core.util.models.MockPubSub;

import java.io.File;
import java.io.IOException;
import java.util.*;
import java.util.concurrent.TimeoutException;

public class UtilityTests {
    
    private static final String HELLO_WORLD = "hello.world";

    @Before
    public void setup() {
        // temp directory should be available from the OS without access right restriction
        File temp = new File("/tmp");
        if (!temp.exists()) {
            temp.mkdir();
        }
    }

    @Test(expected = IllegalArgumentException.class)
    public void setServerPersonality() {
        ServerPersonality personality = ServerPersonality.getInstance();
        personality.setType(ServerPersonality.Type.APP);
        Assert.assertEquals(ServerPersonality.Type.APP, personality.getType());
        personality.setType(null);
    }

    @Test(expected = IOException.class)
    public void noPubSub() throws IOException {
        PubSub ps = PubSub.getInstance();
        ps.createTopic(HELLO_WORLD);
    }

    @Test
    public void mockPubSub() throws IOException, TimeoutException {
        Platform platform = Platform.getInstance();
        LambdaFunction f = (headers, body, instance) -> true;
        platform.registerPrivate(PubSub.PUBLISHER, f, 1);
        PubSub ps = PubSub.getInstance();
        ps.enableFeature(new MockPubSub());
        ps.waitForProvider(1);
        ps.createTopic(HELLO_WORLD);
        Assert.assertTrue(ps.exists(HELLO_WORLD));
        ps.deleteTopic(HELLO_WORLD);
        Assert.assertFalse(ps.exists(HELLO_WORLD));
        ps.createTopic(HELLO_WORLD, 10);
        Assert.assertTrue(ps.exists(HELLO_WORLD));
        Assert.assertTrue(ps.isStreamingPubSub());
        Assert.assertEquals(10, ps.partitionCount(HELLO_WORLD));
        Assert.assertTrue(ps.list().contains(HELLO_WORLD));
        ps.subscribe(HELLO_WORLD, f, "client100", "group100");
        ps.subscribe(HELLO_WORLD, 0, f, "client100", "group100");
        ps.publish(HELLO_WORLD, new HashMap<>(), "hello");
        ps.publish(HELLO_WORLD, 1, new HashMap<>(), "hello");
        ps.unsubscribe(HELLO_WORLD);
        ps.unsubscribe(HELLO_WORLD, 1);
        platform.release(PubSub.PUBLISHER);
    }

    @Test
    public void timestampTest() {
        Date now = new Date();
        Utility util = Utility.getInstance();
        String t = util.getTimestamp();
        Assert.assertTrue(util.isDigits(t));
        String ts = util.getTimestamp(now.getTime());
        long time = util.timestamp2ms(ts);
        Assert.assertEquals(now.getTime(), time);
        String awsTime = util.getAmazonDate(now);
        Assert.assertTrue(awsTime.contains("T") && awsTime.endsWith("Z"));
        String awsNumber = awsTime.replace("T", "").replace("Z", "");
        Assert.assertTrue(util.isDigits(awsNumber));
        String iso = util.date2str(now);
        java.sql.Date sql = new java.sql.Date(now.getTime());
        String sqlDate = util.getSqlDate(sql);
        Assert.assertEquals(iso.substring(0, iso.indexOf('T')), sqlDate);
        java.sql.Timestamp sqlTs = new java.sql.Timestamp(now.getTime());
        String sqlTime = util.getSqlTimestamp(sqlTs);
        Assert.assertEquals(iso.replace("T", " ").replace("Z", ""), sqlTime);
    }

    @Test
    public void base64Test() {
        Utility util = Utility.getInstance();
        String text = "hello world & good day";
        String b64 = util.bytesToBase64(util.getUTF(text));
        byte[] bytes = util.base64ToBytes(b64);
        Assert.assertEquals(text, util.getUTF(bytes));
        b64 = util.bytesToUrlBase64(util.getUTF(text));
        bytes = util.urlBase64ToBytes(b64);
        Assert.assertEquals(text, util.getUTF(bytes));
    }

    @Test
    public void dateConversion() {
        Utility util = Utility.getInstance();
        Date now = new Date();
        String s = util.date2str(now);
        Date restored = util.str2date(s);
        Assert.assertEquals(now, restored);
    }

    @Test
    public void ioTest() {
        Utility util = Utility.getInstance();
        File temp = new File("/tmp");
        File tempFile = new File(temp, "dummy");
        try {
            String HELLO_WORLD = "hello world";
            util.str2file(tempFile, HELLO_WORLD);
            String restored = util.file2str(tempFile);
            Assert.assertEquals(HELLO_WORLD, restored);
        } finally {
            tempFile.delete();
        }
    }

    @Test
    public void splitTest() {
        Utility util = Utility.getInstance();
        String TEST = "hello world this is | a |      test";
        List<String> parts = util.split(TEST, " |");
        for (String p: parts) {
            Assert.assertTrue(TEST.contains(p));
        }
    }

    @Test
    public void numberConversionTest() {
        Utility util = Utility.getInstance();
        // test integer value
        int n1 = 12345;
        String s1 = String.valueOf(n1);
        byte[] b1 = util.int2bytes(n1);
        Assert.assertEquals(4, b1.length);
        int restored1 = util.bytes2int(b1);
        Assert.assertEquals(n1, restored1);
        int r1 = util.str2int(s1);
        Assert.assertEquals(n1, r1);
        // test long value
        long n2 = 1000000000L;
        String s2 = String.valueOf(n2);
        byte[] b2 = util.long2bytes(n2);
        Assert.assertEquals(8, b2.length);
        long restored2 = util.bytes2long(b2);
        Assert.assertEquals(n2, restored2);
        long r2 = util.str2long(s2);
        Assert.assertEquals(n2, r2);
        // test float value
        float n3 = 12345.20f;
        String s3 = String.valueOf(n3);
        float c3 = util.str2float(s3);
        Assert.assertEquals(n3, c3, 0);
        // test double value
        double n4 = 12345.20123456789d;
        String s4 = String.valueOf(n4);
        double c4 = util.str2double(s4);
        Assert.assertEquals(n4, c4, 0);
    }

    @Test
    public void numberTest() {
        Utility util = Utility.getInstance();
        // digits
        String CORRECT_DIGITS = "12345";
        String INCORRECT_DIGITS = "123a45";
        Assert.assertTrue(util.isDigits(CORRECT_DIGITS));
        Assert.assertFalse(util.isDigits(INCORRECT_DIGITS));
        // numeric
        String CORRECT_NUMBER = "-12345";
        String INCORRECT_NUMBER = "$12345";
        Assert.assertTrue(util.isNumeric(CORRECT_NUMBER));
        Assert.assertFalse(util.isNumeric(INCORRECT_NUMBER));
    }

    @Test
    public void utfTest() {
        Utility util = Utility.getInstance();
        String HELLO_WORLD = "hello world";
        byte[] b = util.getUTF(HELLO_WORLD);
        String restored = util.getUTF(b);
        Assert.assertEquals(HELLO_WORLD, restored);
    }

    @Test
    public void zeroFillTest() {
        Utility util = Utility.getInstance();
        int n = 20;
        String result = util.zeroFill(n, 10000);
        Assert.assertEquals("00020", result);
    }

    @Test
    public void flatMapTest() {
        Utility util = Utility.getInstance();
        Map<String, Object> map = new HashMap<>();
        Map<String, Object> inner = new HashMap<>();
        List<Object> list = new ArrayList<>();
        list.add(1);
        list.add(2);
        inner.put("world", "data");
        inner.put("number", list);
        map.put("hello", inner);
        Map<String, Object> map2 = new HashMap<>();
        Map<String, Object> inner2 = new HashMap<>();
        List<Integer> list2 = new ArrayList<>();
        list2.add(1);
        list2.add(2);
        inner2.put("world", "data");
        inner2.put("number", list2);
        map2.put("hello", inner2);
        list.add(map2);
        list.add(list2);
        Map<String, Object> flatMap = util.getFlatMap(map);
        Assert.assertEquals("data", flatMap.get(HELLO_WORLD));
        Assert.assertEquals(1, flatMap.get("hello.number[0]"));
        Assert.assertEquals(2, flatMap.get("hello.number[1]"));
        Assert.assertEquals("data", flatMap.get("hello.number[2].hello.world"));
        Assert.assertEquals(1, flatMap.get("hello.number[2].hello.number[0]"));
        Assert.assertEquals(2, flatMap.get("hello.number[2].hello.number[1]"));
        /*
         * flatmap's keys are composite keys
         * We will create a multi-level map and set the elements with the key-values from the flatmap.
         * The original map (non-flatten) must be the same as the multi-level map key-values.
         */
        MultiLevelMap mm = new MultiLevelMap();
        for (String k: flatMap.keySet()) {
            mm.setElement(k, flatMap.get(k));
        }
        Assert.assertEquals(map, mm.getMap());
        /*
         * retrieval using composite keys from the multi-level map must match the original map's values
         */
        Assert.assertEquals("data", mm.getElement(HELLO_WORLD));
        Assert.assertEquals(1, mm.getElement("hello.number[0]"));
        Assert.assertEquals(2, mm.getElement("hello.number[1]"));
        Assert.assertEquals("data", mm.getElement("hello.number[2].hello.world"));
        Assert.assertEquals(1, mm.getElement("hello.number[2].hello.number[0]"));
        Assert.assertEquals(2, mm.getElement("hello.number[2].hello.number[1]"));
        // really a lot of nested levels
        String NESTED_PATH = "hello[5][4][3][2]";
        String SIMPLE_VALUE = "world";
        MultiLevelMap m2 = new MultiLevelMap();
        m2.setElement(NESTED_PATH, SIMPLE_VALUE);
        Map<String, Object> m2flat = util.getFlatMap(m2.getMap());
        Assert.assertEquals(SIMPLE_VALUE, m2flat.get(NESTED_PATH));
        Assert.assertEquals(m2flat.get(NESTED_PATH), m2.getElement(NESTED_PATH));
        // alternate map and list
        String MIX_PATH = "hello.world[0].headers[0]";
        MultiLevelMap m3 = new MultiLevelMap();
        m3.setElement(MIX_PATH, SIMPLE_VALUE);
        Map<String, Object> m3flat = util.getFlatMap(m3.getMap());
        Assert.assertEquals(SIMPLE_VALUE, m3flat.get(MIX_PATH));
        Assert.assertEquals(m3flat.get(MIX_PATH), m3.getElement(MIX_PATH));
    }

}
