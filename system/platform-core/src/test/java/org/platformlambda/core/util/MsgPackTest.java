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

import org.junit.Test;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.serializers.PayloadMapper;
import org.platformlambda.core.util.models.PoJo;

import java.io.IOException;
import java.math.BigDecimal;
import java.math.BigInteger;
import java.util.*;

import org.junit.Assert;

public class MsgPackTest {

    private static final MsgPack msgPack = new MsgPack();

    @SuppressWarnings("unchecked")
    @Test
    public void dataIsMap() throws IOException {
        PoJo pojo = new PoJo();
        pojo.setName("hello world");
        String pojoInstanceString = pojo.toString();
        String[] HELLO_WORLD = {"hello", "world"};
        Map<String, Object> input = new HashMap<>();
        input.put("hello", "world");
        input.put("boolean", true);
        input.put("array", HELLO_WORLD);
        input.put("integer", 12345);
        input.put("pojo", pojo);
        input.put(PayloadMapper.NOTHING, null);
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        Assert.assertTrue(o instanceof Map);
        Map<String, Object> result = (Map<String, Object>) o;
        // MsgPack does not transport null elements in a map
        Assert.assertFalse(result.containsKey(PayloadMapper.NOTHING));
        result.remove(PayloadMapper.NOTHING);
        Assert.assertEquals(o, result);
        // array is converted to list of objects
        Assert.assertEquals(Arrays.asList(HELLO_WORLD), result.get("array"));
        // embedded pojo in a map is converted to the pojo's instance string
        Assert.assertEquals(pojoInstanceString, result.get("pojo"));
    }

    @Test
    public void dataIsInteger() throws IOException {
        int input = 10;
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        Assert.assertEquals(input, o);
    }

    @Test
    public void smallLongBecomesInteger() throws IOException {
        // msgpack compresses number and data type information will be lost
        Long input = 10L;
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        Assert.assertEquals(input.intValue(), o);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void dataIsBigLong() throws IOException {
        long input = -5106534569952410475L;
        Map<String, Object> map = new HashMap<>();
        map.put("number", input);
        byte[] b = msgPack.pack(map);
        Object o = msgPack.unpack(b);
        Assert.assertTrue(o instanceof Map);
        Map<String, Object> restored = (Map<String, Object>) o;
        Assert.assertEquals(input, restored.get("number"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void dataIsBorderLineLong() throws IOException {
        Long input = Integer.MAX_VALUE + 1L;
        List<Long> value = new ArrayList<>();
        value.add(input);
        byte[] b = msgPack.pack(value);
        Object o = msgPack.unpack(b);
        Assert.assertTrue(o instanceof List);
        List<Long> restored = (List<Long>) o;
        Assert.assertEquals(input, restored.get(0));
    }

    @Test
    public void dataIsBigInteger() throws IOException {
        BigInteger input = new BigInteger("10");
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        Assert.assertEquals(input.toString(), o);
    }

    @Test
    public void dataIsBigDecimal() throws IOException {
        BigDecimal input = new BigDecimal("0.0000000000012345");
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        Assert.assertEquals(input.toPlainString(), o);
    }

    @Test
    public void dataIsNull() throws IOException {
        byte[] b = msgPack.pack(null);
        Object o = msgPack.unpack(b);
        Assert.assertNull(o);
    }

    @Test
    public void dataIsBoolean() throws IOException {
        byte[] b = msgPack.pack(true);
        Object o = msgPack.unpack(b);
        Assert.assertEquals(true, o);
    }

    @Test
    public void dataIsFloat() throws IOException {
        Float input = 3.2f;
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        Assert.assertEquals(input, o);
    }

    @Test
    public void dataIsSmallDouble() throws IOException {
        Double input = 3.2d;
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        Assert.assertEquals(input, o);
    }

    @Test
    public void dataIsDouble() throws IOException {
        Double input = Float.MAX_VALUE + 1.0d;
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        Assert.assertEquals(input, o);
    }

    @Test
    public void dataIsDate() throws IOException {
        Date input = new Date();
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        // date object is serialized as UTC string
        Assert.assertEquals(Utility.getInstance().date2str(input), o);
    }

    @Test
    public void dataIsList() throws IOException {
        List<String> input = new ArrayList<>();
        input.add("hello");
        input.add("world");
        input.add(null);    // prove that null value in a list can be transported
        input.add("1");
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        // MsgPack transports null elements in an array list so that absolute sequencing can be preserved
        Assert.assertEquals(input, o);
    }

    @Test
    public void dataIsArray() throws IOException {
        String[] input = {"hello", "world", null, "1"};
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        Assert.assertEquals(Arrays.asList(input), o);
    }

    @Test
    public void dataIsShortNumber() throws IOException {
        Short number = 10;
        byte[] b = msgPack.pack(number);
        Object o = msgPack.unpack(b);
        Assert.assertEquals((int) number, o);
    }

    @Test
    public void dataIsByte() throws IOException {
        byte number = 10;
        byte[] b = msgPack.pack(number);
        Object o = msgPack.unpack(b);
        // a single byte is converted to an integer
        Assert.assertEquals((int) number, o);
    }

    @Test
    public void dataIsPoJo() throws IOException {
        PoJo input = new PoJo();
        input.setName("testing Integer transport");
        input.setNumber(12345);
        input.setAddress("123 Planet Earth");
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        // successfully restored to PoJo
        Assert.assertTrue(o instanceof PoJo);
        PoJo result = (PoJo) o;
        Assert.assertEquals(input.getNumber(), result.getNumber());
        Assert.assertEquals(input.getName(), result.getName());
        Assert.assertEquals(input.getAddress(), result.getAddress());
    }

    @Test
    public void dataIsPoJoWithLong() throws IOException {
        PoJo input = new PoJo();
        input.setName("testing Long number transport");
        input.setLongNumber(10L);
        input.setAddress("100 Planet Earth");
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        // successfully restored to PoJo when the intermediate value becomes an integer
        Assert.assertTrue(o instanceof PoJo);
        PoJo result = (PoJo) o;
        Assert.assertEquals(input.getLongNumber(), result.getLongNumber());
        Assert.assertEquals(input.getName(), result.getName());
        Assert.assertEquals(input.getAddress(), result.getAddress());
    }

    @Test
    public void dataIsPoJoWithBigInteger() throws IOException {
        PoJo input = new PoJo();
        input.setName("testing BigInteger transport");
        input.setBigInteger(new BigInteger("10"));
        input.setAddress("100 Planet Earth");
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        // successfully restored to PoJo when the intermediate value becomes an integer
        Assert.assertTrue(o instanceof PoJo);
        PoJo result = (PoJo) o;
        Assert.assertEquals(input.getBigInteger(), result.getBigInteger());
        Assert.assertEquals(input.getName(), result.getName());
        Assert.assertEquals(input.getAddress(), result.getAddress());
    }

    @Test
    public void dataIsPoJoWithBigDecimal() throws IOException {
        PoJo input = new PoJo();
        input.setName("testing BigInteger transport");
        input.setBigDecimal(new BigDecimal("0.00000012345"));
        input.setAddress("100 Planet Earth");
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        // successfully restored to PoJo when the intermediate value becomes an integer
        Assert.assertTrue(o instanceof PoJo);
        PoJo result = (PoJo) o;
        Assert.assertEquals(input.getBigDecimal(), result.getBigDecimal());
        Assert.assertEquals(input.getName(), result.getName());
        Assert.assertEquals(input.getAddress(), result.getAddress());
    }

}
