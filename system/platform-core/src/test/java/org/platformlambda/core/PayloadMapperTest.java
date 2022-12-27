/*

    Copyright 2018-2022 Accenture Technology

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

import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedPayload;
import org.platformlambda.core.serializers.PayloadMapper;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.models.PoJo;
import com.unsafe.models.UnauthorizedObj;

import java.io.IOException;
import java.util.*;

import org.junit.Assert;
import org.platformlambda.core.util.Utility;

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
         * Since the object is not in the safe.data.models list, the data is decoded as a simple HashMap.
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

    @Test
    public void rejectUnsafeClasses() {
        String MESSAGE = "Class " + UnauthorizedObj.class.getName() + " not in safe.data.models";
        IllegalArgumentException ex = Assert.assertThrows(IllegalArgumentException.class,
                () ->  SimpleMapper.getInstance().getSafeMapper(UnauthorizedObj.class));
        Assert.assertEquals(MESSAGE, ex.getMessage());
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
    public void convertPoJoUsingJson() throws ClassNotFoundException {
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
    public void datePayloadTest() throws IOException {
        Utility util = Utility.getInstance();
        Date now = new Date();
        EventEnvelope event = new EventEnvelope();
        event.setBody(now);
        Object o = event.getBody();
        // date object is serialized as ISO-8601 timestamp when the setBody method is called
        Assert.assertEquals(util.date2str(now), o);
        byte[] b = event.toBytes();
        EventEnvelope restored = new EventEnvelope(b);
        Assert.assertEquals(util.date2str(now), restored.getBody());
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
        Utility util = Utility.getInstance();
        Date input = new Date();
        TypedPayload typed = converter.encode(input, true);
        Assert.assertEquals(PayloadMapper.PRIMITIVE, typed.getType());
        Assert.assertEquals(util.date2str(input), typed.getPayload());
        Object converted = converter.decode(typed);
        Assert.assertEquals(util.date2str(input), converted);
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
        Assert.assertEquals(PayloadMapper.LIST, typed.getType());
        Object converted = converter.decode(typed);
        Assert.assertEquals(Arrays.asList(input), converted);
    }

}
