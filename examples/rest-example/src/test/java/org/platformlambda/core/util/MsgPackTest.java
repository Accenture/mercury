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
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.serializers.PayloadMapper;
import org.platformlambda.core.util.models.PoJo;

import java.io.IOException;
import java.util.*;

import static org.junit.Assert.*;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

public class MsgPackTest {

    private static final MsgPack msgPack = new MsgPack();

    @SuppressWarnings("unchecked")
    @Test
    public void dataIsMap() throws IOException {
        Map<String, Object> input = new HashMap<>();
        input.put("hello", "world");
        input.put("boolean", true);
        input.put("integer", 12345);
        input.put(PayloadMapper.NOTHING, null);
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        assertTrue(o instanceof Map);
        Map<String, Object> result = (Map<String, Object>) o;
        // MsgPack does not transport null elements in a map
        assertFalse(result.containsKey(PayloadMapper.NOTHING));
        result.remove(PayloadMapper.NOTHING);
        assertEquals(o, result);
    }

    @Test
    public void dataIsInteger() throws IOException {
        int input = 10;
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        assertEquals(input, o);
    }

    @Test
    public void dataIsNull() throws IOException {
        Object input = null;
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        assertEquals(input, o);
    }

    @Test
    public void dataIsBoolean() throws IOException {
        boolean input = true;
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        assertEquals(input, o);
    }

    @Test
    public void dataIsFloat() throws IOException {
        Float input = 3.2f;
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        assertEquals(input, o);
    }

    @Test
    public void dataIsDate() throws IOException {
        Date input = new Date();
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        // date object is serialized as UTC string
        assertEquals(Utility.getInstance().date2str(input), o);
    }

    @Test
    public void dataIsList() throws IOException {
        List<String> input = new ArrayList<>();
        input.add("hello");
        input.add("world");
        input.add(null);
        input.add("1");
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        // MsgPack transports null elements in an array list so that absolute sequencing can be preserved
        assertEquals(input, o);
    }

    @Test
    public void dataIsPoJo() throws IOException {
        PoJo input = new PoJo();
        input.setName("hello world");
        input.setNumber(12345);
        input.setAddress("123 Planet Earth");
        byte[] b = msgPack.pack(input);
        Object o = msgPack.unpack(b);
        // successfully restored to PoJo
        assertTrue(o instanceof PoJo);
        PoJo result = (PoJo) o;
        assertEquals(input.getNumber(), result.getNumber());
        assertEquals(input.getName(), result.getName());
        assertEquals(input.getAddress(), result.getAddress());
    }

}