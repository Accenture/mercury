/*

    Copyright 2018-2019 Accenture Technology

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
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedPayload;
import org.platformlambda.core.serializers.PayloadMapper;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.models.PoJo;

import java.io.IOException;
import java.util.*;

import static org.junit.Assert.*;
import static org.junit.Assert.assertNull;
import static org.platformlambda.core.serializers.PayloadMapper.*;

public class PayloadMapperTest {

    private static final PayloadMapper converter = PayloadMapper.getInstance();

    @Test
    public void acceptSafeJavaDefaultClasses() {
        SimpleMapper.getInstance().getWhiteListMapper(String.class);
        SimpleMapper.getInstance().getWhiteListMapper(byte[].class);
        SimpleMapper.getInstance().getWhiteListMapper(Date.class);
        SimpleMapper.getInstance().getWhiteListMapper(Integer.class);
        SimpleMapper.getInstance().getWhiteListMapper(Map.class);
        SimpleMapper.getInstance().getWhiteListMapper(HashMap.class);
        SimpleMapper.getInstance().getWhiteListMapper(List.class);
        SimpleMapper.getInstance().getWhiteListMapper(ArrayList.class);
        SimpleMapper.getInstance().getWhiteListMapper(Number.class);
    }

    @Test
    public void pojoInEvent() throws IOException {
        int len1 = pojoInEventUsingMsgPack();
        int len2 = pojoInEventUsingGson();
        // transport size is larger when using JSON
        assertTrue(len2 > len1);
    }

    private int pojoInEventUsingMsgPack() throws IOException {
        PoJo input = new PoJo();
        input.setName("hello world");
        input.setNumber(12345);

        EventEnvelope event1 = new EventEnvelope();
        event1.setBody(input);
        byte[] b = event1.toBytes();

        EventEnvelope event2 = new EventEnvelope();
        event2.load(b);
        assertEquals(input.getClass(), event2.getBody().getClass());
        assertTrue(event2.isBinary());

        PoJo o = (PoJo) event2.getBody();
        assertEquals(input.getName(), o.getName());
        assertEquals(input.getNumber(), o.getNumber());
        return b.length;
    }

    private int pojoInEventUsingGson() throws IOException {
        PoJo input = new PoJo();
        input.setName("hello world");
        input.setNumber(12345);

        EventEnvelope event1 = new EventEnvelope();
        event1.setBody(input);
        event1.setBinary(false);
        byte[] b = event1.toBytes();

        EventEnvelope event2 = new EventEnvelope();
        event2.load(b);
        assertEquals(input.getClass(), event2.getBody().getClass());
        assertFalse(event2.isBinary());

        PoJo o = (PoJo) event2.getBody();
        assertEquals(input.getName(), o.getName());
        assertEquals(input.getNumber(), o.getNumber());
        return b.length;
    }

    @Test
    public void convertPoJoUsingMsgPack() throws ClassNotFoundException {
        PoJo input = new PoJo();
        input.setName("hello world");
        input.setNumber(12345);

        TypedPayload typed = converter.encode(input, true);
        assertEquals(input.getClass().getName(), typed.getType());
        assertTrue(typed.getPayload() instanceof Map);
        Object converted = converter.decode(typed);
        assertTrue(converted instanceof PoJo);
        PoJo o = (PoJo) converted;
        assertEquals(input.getName(), o.getName());
        assertEquals(input.getNumber(), o.getNumber());
    }

    @Test
    public void convertPoJoUsingJackson() throws ClassNotFoundException {
        PoJo input = new PoJo();
        input.setName("hello world");
        input.setNumber(12345);

        TypedPayload typed = converter.encode(input, false);
        assertEquals(input.getClass().getName(), typed.getType());
        assertTrue(typed.getPayload() instanceof byte[]);
        Object converted = converter.decode(typed);
        assertTrue(converted instanceof PoJo);
        PoJo o = (PoJo) converted;
        assertEquals(input.getName(), o.getName());
        assertEquals(input.getNumber(), o.getNumber());
    }

    @Test
    public void convertMap() throws ClassNotFoundException {
        Map<String, Object> input = new HashMap<>();
        input.put("hello", "world");
        TypedPayload typed = converter.encode(input, true);
        assertEquals(MAP, typed.getType());
        assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        assertEquals(input, converted);
    }

    @Test
    public void convertString() throws ClassNotFoundException {
        String input = "hello world";
        TypedPayload typed = converter.encode(input, true);
        assertEquals(PRIMITIVE, typed.getType());
        assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        assertEquals(input, converted);
    }

    @Test
    public void convertBytes() throws ClassNotFoundException {
        byte[] input = "hello world".getBytes();
        TypedPayload typed = converter.encode(input, true);
        assertEquals(PRIMITIVE, typed.getType());
        assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        assertEquals(input, converted);
    }

    @Test
    public void convertNull() throws ClassNotFoundException {
        TypedPayload typed = converter.encode(null, true);
        assertEquals(NOTHING, typed.getType());
        assertNull(typed.getPayload());
        Object converted = converter.decode(typed);
        assertNull(converted);
    }

    @Test
    public void convertBoolean() throws ClassNotFoundException {
        TypedPayload typed = converter.encode(true, true);
        assertEquals(PRIMITIVE, typed.getType());
        assertEquals(true, typed.getPayload());
        Object converted = converter.decode(typed);
        assertEquals(true, converted);
    }

    @Test
    public void convertInteger() throws ClassNotFoundException {
        Integer input = 12345;
        TypedPayload typed = converter.encode(input, true);
        assertEquals(PRIMITIVE, typed.getType());
        assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        assertEquals(input, converted);
    }

    @Test
    public void convertLong() throws ClassNotFoundException {
        Long input = 123456L;
        TypedPayload typed = converter.encode(input, true);
        assertEquals(PRIMITIVE, typed.getType());
        assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        assertEquals(input, converted);
    }

    @Test
    public void convertFloat() throws ClassNotFoundException {
        Float input = 12.34f;
        TypedPayload typed = converter.encode(input, true);
        assertEquals(PRIMITIVE, typed.getType());
        assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        assertEquals(input, converted);
    }

    @Test
    public void convertDouble() throws ClassNotFoundException {
        Double input = 12.34d;
        TypedPayload typed = converter.encode(input, true);
        assertEquals(PRIMITIVE, typed.getType());
        assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        assertEquals(input, converted);
    }

    @Test
    public void convertDate() throws ClassNotFoundException {
        Date input = new Date();
        TypedPayload typed = converter.encode(input, true);
        assertEquals(PRIMITIVE, typed.getType());
        assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        assertEquals(input, converted);
    }

    @Test
    public void convertList() throws ClassNotFoundException {
        List<String> input = new ArrayList<>();
        input.add("hello");
        input.add("world");
        TypedPayload typed = converter.encode(input, true);
        assertEquals(LIST, typed.getType());
        assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        assertEquals(input, converted);
    }

    @Test
    public void convertArray() throws ClassNotFoundException {
        String[] input = {"hello", "world"};
        TypedPayload typed = converter.encode(input, true);
        assertEquals(ARRAY, typed.getType());
        Object converted = converter.decode(typed);
        assertTrue(sameArrays(input, converted));
    }

    private boolean sameArrays(Object a, Object b) {
        if (a instanceof Object[] && b instanceof Object[]) {
            Object[] o1 = (Object[]) a;
            Object[] o2 = (Object[]) b;
            if (o1.length == o2.length) {
                for (int i=0; i < o1.length; i++) {
                    if (o1[i] != o2[i]) {
                        return false;
                    }
                }
                return true;
            }
        }
        return false;
    }

}
