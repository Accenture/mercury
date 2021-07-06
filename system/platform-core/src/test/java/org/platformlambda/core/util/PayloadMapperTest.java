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
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedPayload;
import org.platformlambda.core.serializers.PayloadMapper;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.models.PoJo;
import org.platformlambda.core.util.unsafe.models.UnauthorizedObj;

import java.io.IOException;
import java.util.*;

import org.junit.Assert;

public class PayloadMapperTest {

    private static final PayloadMapper converter = PayloadMapper.getInstance();

    @Test
    @SuppressWarnings("unchecked")
    public void optionalTransport() throws IOException {
        String text = "hello world";
        Optional<Object> hello = Optional.of(text);
        EventEnvelope event1 = new EventEnvelope();
        event1.setBody(hello);
        byte[] b = event1.toBytes();
        EventEnvelope event2 = new EventEnvelope();
        event2.load(b);
        Assert.assertTrue(event2.getBody() instanceof Optional);
        Optional<Object> value = (Optional<Object>) event2.getBody();
        Assert.assertTrue(value.isPresent());
        Assert.assertEquals(text, value.get());
    }

    @Test
    public void pojoTransport() throws IOException {
        String name = "hello";
        PoJo pojo = new PoJo();
        pojo.setName(name);
        EventEnvelope event1 = new EventEnvelope();
        event1.setBody(pojo);
        byte[] b = event1.toBytes();
        EventEnvelope event2 = new EventEnvelope();
        event2.load(b);
        Assert.assertEquals(PoJo.class, event2.getBody().getClass());
        // try again with pojo disabled
        event1.setPoJoEnabled(false);
        b = event1.toBytes();
        event2.load(b);
        Assert.assertEquals(HashMap.class, event2.getBody().getClass());
    }

    @Test
    public void rejectUnauthorizedClass() throws IOException {
        UnauthorizedObj input = new UnauthorizedObj();
        input.setName("hello world");
        input.setNumber(12345);

        EventEnvelope event1 = new EventEnvelope();
        event1.setBody(input);
        byte[] b = event1.toBytes();

        EventEnvelope event2 = new EventEnvelope();
        event2.load(b);
        /*
         * Since the object is not in the safe.data.models white-list, the data is decoded as a simple HashMap.
         * Deserialization to the UnauthorizedObj is not performed.
         */
        Assert.assertEquals(HashMap.class, event2.getBody().getClass());
    }

    @Test
    public void acceptSafeJavaDefaultClasses() {
        SimpleMapper.getInstance().getSafeMapper(String.class);
        SimpleMapper.getInstance().getSafeMapper(byte[].class);
        SimpleMapper.getInstance().getSafeMapper(Date.class);
        SimpleMapper.getInstance().getSafeMapper(Integer.class);
        SimpleMapper.getInstance().getSafeMapper(Map.class);
        SimpleMapper.getInstance().getSafeMapper(HashMap.class);
        SimpleMapper.getInstance().getSafeMapper(List.class);
        SimpleMapper.getInstance().getSafeMapper(ArrayList.class);
        SimpleMapper.getInstance().getSafeMapper(Number.class);
    }

    @Test(expected=IllegalArgumentException.class)
    public void rejectUnsafeClasses() {
        SimpleMapper.getInstance().getSafeMapper(UnauthorizedObj.class);
    }

    @Test
    public void pojoInEvent() throws IOException {
        int len1 = pojoInEventUsingMsgPack();
        int len2 = pojoInEventUsingGson();
        // transport size is larger when using JSON
        Assert.assertTrue(len2 > len1);
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
        Assert.assertEquals(input.getClass(), event2.getBody().getClass());
        Assert.assertTrue(event2.isBinary());

        PoJo o = (PoJo) event2.getBody();
        Assert.assertEquals(input.getName(), o.getName());
        Assert.assertEquals(input.getNumber(), o.getNumber());
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
        Assert.assertEquals(input.getClass(), event2.getBody().getClass());
        Assert.assertFalse(event2.isBinary());

        PoJo o = (PoJo) event2.getBody();
        Assert.assertEquals(input.getName(), o.getName());
        Assert.assertEquals(input.getNumber(), o.getNumber());
        return b.length;
    }

    @Test
    public void convertPoJoUsingMsgPack() throws ClassNotFoundException {
        PoJo input = new PoJo();
        input.setName("hello world");
        input.setNumber(12345);

        TypedPayload typed = converter.encode(input, true);
        Assert.assertEquals(input.getClass().getName(), typed.getType());
        Assert.assertTrue(typed.getPayload() instanceof Map);
        Object converted = converter.decode(typed);
        Assert.assertTrue(converted instanceof PoJo);
        PoJo o = (PoJo) converted;
        Assert.assertEquals(input.getName(), o.getName());
        Assert.assertEquals(input.getNumber(), o.getNumber());
    }

    @Test
    public void convertPoJoUsingJackson() throws ClassNotFoundException {
        PoJo input = new PoJo();
        input.setName("hello world");
        input.setNumber(12345);

        TypedPayload typed = converter.encode(input, false);
        Assert.assertEquals(input.getClass().getName(), typed.getType());
        Assert.assertTrue(typed.getPayload() instanceof byte[]);
        Object converted = converter.decode(typed);
        Assert.assertTrue(converted instanceof PoJo);
        PoJo o = (PoJo) converted;
        Assert.assertEquals(input.getName(), o.getName());
        Assert.assertEquals(input.getNumber(), o.getNumber());
    }

    @Test
    public void convertMap() throws ClassNotFoundException {
        Map<String, Object> input = new HashMap<>();
        input.put("hello", "world");
        TypedPayload typed = converter.encode(input, true);
        Assert.assertEquals(PayloadMapper.MAP, typed.getType());
        Assert.assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        Assert.assertEquals(input, converted);
    }

    @Test
    public void convertString() throws ClassNotFoundException {
        String input = "hello world";
        TypedPayload typed = converter.encode(input, true);
        Assert.assertEquals(PayloadMapper.PRIMITIVE, typed.getType());
        Assert.assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        Assert.assertEquals(input, converted);
    }

    @Test
    public void convertBytes() throws ClassNotFoundException {
        byte[] input = "hello world".getBytes();
        TypedPayload typed = converter.encode(input, true);
        Assert.assertEquals(PayloadMapper.PRIMITIVE, typed.getType());
        Assert.assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        Assert.assertEquals(input, converted);
    }

    @Test
    public void convertNull() throws ClassNotFoundException {
        TypedPayload typed = converter.encode(null, true);
        Assert.assertEquals(PayloadMapper.NOTHING, typed.getType());
        Assert.assertNull(typed.getPayload());
        Object converted = converter.decode(typed);
        Assert.assertNull(converted);
    }

    @Test
    public void convertBoolean() throws ClassNotFoundException {
        TypedPayload typed = converter.encode(true, true);
        Assert.assertEquals(PayloadMapper.PRIMITIVE, typed.getType());
        Assert.assertEquals(true, typed.getPayload());
        Object converted = converter.decode(typed);
        Assert.assertEquals(true, converted);
    }

    @Test
    public void convertInteger() throws ClassNotFoundException {
        Integer input = 12345;
        TypedPayload typed = converter.encode(input, true);
        Assert.assertEquals(PayloadMapper.PRIMITIVE, typed.getType());
        Assert.assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        Assert.assertEquals(input, converted);
    }

    @Test
    public void convertLong() throws ClassNotFoundException {
        Long input = 123456L;
        TypedPayload typed = converter.encode(input, true);
        Assert.assertEquals(PayloadMapper.PRIMITIVE, typed.getType());
        Assert.assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        Assert.assertEquals(input, converted);
    }

    @Test
    public void convertFloat() throws ClassNotFoundException {
        Float input = 12.34f;
        TypedPayload typed = converter.encode(input, true);
        Assert.assertEquals(PayloadMapper.PRIMITIVE, typed.getType());
        Assert.assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        Assert.assertEquals(input, converted);
    }

    @Test
    public void convertDouble() throws ClassNotFoundException {
        Double input = 12.34d;
        TypedPayload typed = converter.encode(input, true);
        Assert.assertEquals(PayloadMapper.PRIMITIVE, typed.getType());
        Assert.assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        Assert.assertEquals(input, converted);
    }

    @Test
    public void convertDate() throws ClassNotFoundException {
        Date input = new Date();
        TypedPayload typed = converter.encode(input, true);
        Assert.assertEquals(PayloadMapper.PRIMITIVE, typed.getType());
        Assert.assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        Assert.assertEquals(input, converted);
    }

    @Test
    public void convertList() throws ClassNotFoundException {
        List<String> input = new ArrayList<>();
        input.add("hello");
        input.add("world");
        TypedPayload typed = converter.encode(input, true);
        Assert.assertEquals(PayloadMapper.LIST, typed.getType());
        Assert.assertEquals(input, typed.getPayload());
        Object converted = converter.decode(typed);
        Assert.assertEquals(input, converted);
    }

    @Test
    public void convertArray() throws ClassNotFoundException {
        String[] input = {"hello", "world"};
        TypedPayload typed = converter.encode(input, true);
        Assert.assertEquals(PayloadMapper.ARRAY, typed.getType());
        Object converted = converter.decode(typed);
        Assert.assertTrue(sameArrays(input, converted));
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

    @Test(expected = IllegalArgumentException.class)
    public void listOfPoJo() {
        List<PoJo> input = new ArrayList<>();
        PoJo a = new PoJo();
        a.setName("hello");
        a.setDate(new Date());
        input.add(a);
        PoJo b = new PoJo();
        b.setName("world");
        b.setDate(new Date());
        input.add(b);
        /*
         * For simplicity, list of PoJo is not supported.
         * This will throw IllegalArgumentException when trying to serialize.
         *
         * Please use a parent class to wrap it for transport over the network
         * between sender and recipient service.
         */
        converter.encode(input, true);
    }

}
