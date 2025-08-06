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

package org.platformlambda.core;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.platformlambda.automation.util.SimpleHttpUtility;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.core.models.MockPubSub;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;

import java.io.File;
import java.io.IOException;
import java.util.*;

public class UtilityTests {

    private static final String HELLO_WORLD = "hello.world";
    private static final long ONE_SECOND = 1000;
    private static final long ONE_MINUTE = 60 * ONE_SECOND;
    private static final long ONE_HOUR = 60 * ONE_MINUTE;
    private static final long ONE_DAY = 24 * ONE_HOUR;

    @BeforeEach
    public void setup() {
        // temp directory should be available from the OS without access right restriction
        File temp = new File("/tmp");
        if (!temp.exists()) {
            temp.mkdir();
        }
    }

    @Test
    public void setServerPersonality() {
        ServerPersonality personality = ServerPersonality.getInstance();
        String MESSAGE = "Personality cannot be null";
        IllegalArgumentException ex = Assertions.assertThrows(IllegalArgumentException.class,
                                                () -> personality.setType(null));
        Assertions.assertEquals(MESSAGE, ex.getMessage());
    }

    @Test
    public void mockPubSub() throws IOException {
        PubSub ps = PubSub.getInstance();
        ps.enableFeature(new MockPubSub());
        ps.waitForProvider(1);
        ps.createTopic(HELLO_WORLD);
        Assertions.assertTrue(ps.exists(HELLO_WORLD));
        ps.deleteTopic(HELLO_WORLD);
        Assertions.assertFalse(ps.exists(HELLO_WORLD));
        ps.createTopic(HELLO_WORLD, 10);
        Assertions.assertTrue(ps.exists(HELLO_WORLD));
        Assertions.assertTrue(ps.isStreamingPubSub());
        Assertions.assertEquals(10, ps.partitionCount(HELLO_WORLD));
        Assertions.assertTrue(ps.list().contains(HELLO_WORLD));
        LambdaFunction f = (headers, body, instance) -> true;
        ps.subscribe(HELLO_WORLD, f, "client100", "group100");
        ps.subscribe(HELLO_WORLD, 0, f, "client100", "group100");
        ps.publish(HELLO_WORLD, new HashMap<>(), "hello");
        ps.publish(HELLO_WORLD, 1, new HashMap<>(), "hello");
        ps.unsubscribe(HELLO_WORLD);
        ps.unsubscribe(HELLO_WORLD, 1);
        ps.cleanup();
    }

    @Test
    public void mockPubSubCreateQueue() {
        PubSub ps = PubSub.getInstance();
        ps.enableFeature(new MockPubSub());
        String MESSAGE = "Not implemented";
        IllegalArgumentException ex = Assertions.assertThrows(IllegalArgumentException.class,
                                            () -> ps.createQueue("demo.queue"));
        Assertions.assertEquals(MESSAGE, ex.getMessage());
    }

    @Test
    public void mockPubSubDeleteQueue() throws IOException {
        PubSub ps = PubSub.getInstance();
        ps.enableFeature(new MockPubSub());
        IllegalArgumentException ex = Assertions.assertThrows(IllegalArgumentException.class,
                () -> ps.deleteQueue("demo.queue"));
        Assertions.assertEquals("Not implemented", ex.getMessage());
    }

    @Test
    public void timestampTest() {
        Utility util = Utility.getInstance();
        String EXACT_SECOND = ".000";
        Date now = new Date();
        String t = util.getTimestamp();
        Assertions.assertTrue(util.isDigits(t));
        String ts = util.getTimestamp(now.getTime());
        long time = util.timestamp2ms(ts);
        Assertions.assertEquals(now.getTime(), time);
        String awsTime = util.getAmazonDate(now);
        Assertions.assertTrue(awsTime.contains("T") && awsTime.endsWith("Z"));
        String awsNumber = awsTime.replace("T", "").replace("Z", "");
        Assertions.assertTrue(util.isDigits(awsNumber));
        String iso = util.date2str(now);
        java.sql.Date sql = new java.sql.Date(now.getTime());
        String sqlDate = util.getSqlDate(sql);
        Assertions.assertEquals(iso.substring(0, iso.indexOf('T')), sqlDate);
        java.sql.Timestamp sqlTs = new java.sql.Timestamp(now.getTime());
        String sqlTime = util.getSqlTimestamp(sqlTs);
        if (sqlTime.endsWith(EXACT_SECOND)) {
            sqlTime = sqlTime.substring(0, sqlTime.length() - EXACT_SECOND.length());
        }
        Assertions.assertEquals(iso.replace("T", " ").replace("Z", ""), sqlTime);
    }

    @Test
    public void exactSecondTimestampTest() {
        Utility util = Utility.getInstance();
        String EXACT_SECOND = ".000";
        String exact = util.date2str(new Date(), true);
        Date now = util.str2date(exact);
        String iso = util.date2str(now);
        java.sql.Timestamp sqlTs = new java.sql.Timestamp(now.getTime());
        String sqlTime = util.getSqlTimestamp(sqlTs);
        Assertions.assertTrue(sqlTime.endsWith(EXACT_SECOND));
        sqlTime = sqlTime.substring(0, sqlTime.length() - EXACT_SECOND.length());
        Assertions.assertEquals(iso.replace("T", " ").replace("Z", ""), sqlTime);
    }

    @Test
    public void base64Test() {
        Utility util = Utility.getInstance();
        String text = "hello world & good day";
        String b64 = util.bytesToBase64(util.getUTF(text));
        byte[] bytes = util.base64ToBytes(b64);
        Assertions.assertEquals(text, util.getUTF(bytes));
        b64 = util.bytesToUrlBase64(util.getUTF(text));
        bytes = util.urlBase64ToBytes(b64);
        Assertions.assertEquals(text, util.getUTF(bytes));
    }

    @Test
    public void dateConversion() {
        Utility util = Utility.getInstance();
        Date now = new Date();
        String s = util.date2str(now);
        Date restored = util.str2date(s);
        Assertions.assertEquals(now, restored);
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
            Assertions.assertEquals(HELLO_WORLD, restored);
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
            Assertions.assertTrue(TEST.contains(p));
        }
    }

    @Test
    public void numberConversionTest() {
        Utility util = Utility.getInstance();
        // test integer value
        int n1 = 12345;
        String s1 = String.valueOf(n1);
        byte[] b1 = util.int2bytes(n1);
        Assertions.assertEquals(4, b1.length);
        int restored1 = util.bytes2int(b1);
        Assertions.assertEquals(n1, restored1);
        int r1 = util.str2int(s1);
        Assertions.assertEquals(n1, r1);
        // test long value
        long n2 = 1000000000L;
        String s2 = String.valueOf(n2);
        byte[] b2 = util.long2bytes(n2);
        Assertions.assertEquals(8, b2.length);
        long restored2 = util.bytes2long(b2);
        Assertions.assertEquals(n2, restored2);
        long r2 = util.str2long(s2);
        Assertions.assertEquals(n2, r2);
        // test float value
        float n3 = 12345.20f;
        String s3 = String.valueOf(n3);
        float c3 = util.str2float(s3);
        Assertions.assertEquals(n3, c3, 0);
        // test double value
        double n4 = 12345.20123456789d;
        String s4 = String.valueOf(n4);
        double c4 = util.str2double(s4);
        Assertions.assertEquals(n4, c4, 0);
    }

    @Test
    public void numberTest() {
        Utility util = Utility.getInstance();
        // digits
        String CORRECT_DIGITS = "12345";
        String INCORRECT_DIGITS = "123a45";
        Assertions.assertTrue(util.isDigits(CORRECT_DIGITS));
        Assertions.assertFalse(util.isDigits(INCORRECT_DIGITS));
        // numeric
        String CORRECT_NUMBER = "-12345";
        String INCORRECT_NUMBER = "$12345";
        Assertions.assertTrue(util.isNumeric(CORRECT_NUMBER));
        Assertions.assertFalse(util.isNumeric(INCORRECT_NUMBER));
    }

    @Test
    public void utfTest() {
        Utility util = Utility.getInstance();
        String HELLO_WORLD = "hello world";
        byte[] b = util.getUTF(HELLO_WORLD);
        String restored = util.getUTF(b);
        Assertions.assertEquals(HELLO_WORLD, restored);
    }

    @Test
    public void zeroFillTest() {
        Utility util = Utility.getInstance();
        int n = 20;
        String result = util.zeroFill(n, 10000);
        Assertions.assertEquals("00020", result);
    }

    @Test
    public void multiLevelMapTest() {
        String HELLO = "hello";
        String WORLD = "world";
        String HELLO_WORLD = "hello.world";
        String NULL_KEY_VALUE = "this.is.nil";
        String NOT_EXIST_KEY = "key.not.exist";

        MultiLevelMap mm = new MultiLevelMap();
        mm.setElement(HELLO, WORLD);
        mm.setElement(NULL_KEY_VALUE, null);
        Assertions.assertEquals(WORLD, mm.getElement(HELLO));
        Assertions.assertNull(mm.getElement(NULL_KEY_VALUE));
        // key exists but value is null
        Assertions.assertTrue(mm.keyExists(NULL_KEY_VALUE));
        Assertions.assertFalse(mm.exists(NULL_KEY_VALUE));
        // key does not exist
        Assertions.assertFalse(mm.keyExists(NOT_EXIST_KEY));
        // delete a key-value
        mm.removeElement(HELLO_WORLD);
        Assertions.assertEquals(Collections.EMPTY_MAP, mm.getElement(HELLO));
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
        Assertions.assertEquals("data", flatMap.get(HELLO_WORLD));
        Assertions.assertEquals(1, flatMap.get("hello.number[0]"));
        Assertions.assertEquals(2, flatMap.get("hello.number[1]"));
        Assertions.assertEquals("data", flatMap.get("hello.number[2].hello.world"));
        Assertions.assertEquals(1, flatMap.get("hello.number[2].hello.number[0]"));
        Assertions.assertEquals(2, flatMap.get("hello.number[2].hello.number[1]"));
        /*
         * flatmap's keys are composite keys
         * We will create a multi-level map and set the elements with the key-values from the flatmap.
         * The original map (non-flatten) must be the same as the multi-level map key-values.
         */
        MultiLevelMap mm = new MultiLevelMap();
        for (String k: flatMap.keySet()) {
            mm.setElement(k, flatMap.get(k));
        }
        Assertions.assertEquals(map, mm.getMap());
        /*
         * retrieval using composite keys from the multi-level map must match the original map's values
         */
        Assertions.assertEquals("data", mm.getElement(HELLO_WORLD));
        Assertions.assertEquals(1, mm.getElement("hello.number[0]"));
        Assertions.assertEquals(2, mm.getElement("hello.number[1]"));
        Assertions.assertEquals("data", mm.getElement("hello.number[2].hello.world"));
        Assertions.assertEquals(1, mm.getElement("hello.number[2].hello.number[0]"));
        Assertions.assertEquals(2, mm.getElement("hello.number[2].hello.number[1]"));
        // really a lot of nested levels
        String NESTED_PATH = "hello[5][4][3][2]";
        String SIMPLE_VALUE = "world";
        MultiLevelMap m2 = new MultiLevelMap();
        m2.setElement(NESTED_PATH, SIMPLE_VALUE);
        Map<String, Object> m2flat = util.getFlatMap(m2.getMap());
        Assertions.assertEquals(SIMPLE_VALUE, m2flat.get(NESTED_PATH));
        Assertions.assertEquals(m2flat.get(NESTED_PATH), m2.getElement(NESTED_PATH));
        // alternate map and list
        String MIX_PATH = "hello.world[0].headers[0]";
        MultiLevelMap m3 = new MultiLevelMap();
        m3.setElement(MIX_PATH, SIMPLE_VALUE);
        Map<String, Object> m3flat = util.getFlatMap(m3.getMap());
        Assertions.assertEquals(SIMPLE_VALUE, m3flat.get(MIX_PATH));
        Assertions.assertEquals(m3flat.get(MIX_PATH), m3.getElement(MIX_PATH));
    }

    @Test
    public void intranetIpTest() {
        final Utility util = Utility.getInstance();
        String[] IP_ADDRESSES = {"127.0.0.1:8080", "127.0.0.1", "10.1.2.3", "172.16.1.2", "192.168.1.30"};
        for (String ip: IP_ADDRESSES) {
            Assertions.assertTrue(util.isIntranetAddress(ip));
        }
        Assertions.assertFalse(util.isIntranetAddress("localhost"));
        Assertions.assertFalse(util.isIntranetAddress(null));
        Assertions.assertFalse(util.isIntranetAddress("128.1.2.3"));
        Assertions.assertFalse(util.isIntranetAddress("hello.world.com"));
        Assertions.assertFalse(util.isIntranetAddress("127.0001.1.1"));
    }

    @Test
    public void elapsedTimeTest() {
        long time = ONE_DAY + 40 * ONE_HOUR + 5 * ONE_MINUTE + 6 * ONE_SECOND;
        String expected = "2 days 16 hours 5 minutes 6 seconds";
        final Utility util = Utility.getInstance();
        Assertions.assertEquals(expected, util.elapsedTime(time));
    }

    @Test
    public void simpleHttpDecodeTest() {
        SimpleHttpUtility http = SimpleHttpUtility.getInstance();
        Map<String, String> result = http.decodeQueryString("a=b&x=y");
        Assertions.assertEquals("b", result.get("a"));
        Assertions.assertEquals("y", result.get("x"));
    }

    @Test
    public void urlRewriteTest() {
        SimpleHttpUtility http = SimpleHttpUtility.getInstance();
        List<String> rewrite = new ArrayList<>();
        rewrite.add("/api/");
        rewrite.add("/api/v2/");
        String url = "/api/hello/world";
        Assertions.assertEquals("/api/v2/hello/world", http.normalizeUrl(url, rewrite));
    }

}
