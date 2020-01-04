/*

    Copyright 2018-2020 Accenture Technology

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
import org.platformlambda.core.util.models.ListOfObjects;
import org.platformlambda.core.util.models.ObjectWithGenericType;
import org.platformlambda.core.util.models.PoJo;

import java.io.IOException;
import java.util.Collections;
import java.util.Map;

import static org.junit.Assert.*;

public class GenericTypeTest {

    @Test
    public void testListOfObjects() throws IOException {
        int number = 100;
        String name = "hello world";
        PoJo pojo = new PoJo();
        pojo.setNumber(number);
        pojo.setName(name);
        ListOfObjects list = new ListOfObjects();
        list.setPoJoList(Collections.singletonList(pojo));

        EventEnvelope event = new EventEnvelope();
        event.setBody(list);
        byte[] b = event.toBytes();

        EventEnvelope result = new EventEnvelope();
        result.load(b);

        assertTrue(result.getBody() instanceof ListOfObjects);
        ListOfObjects o = (ListOfObjects) result.getBody();
        assertTrue(result.getBody() instanceof ListOfObjects);
        PoJo restored = o.getPoJoList().get(0);
        assertEquals(name, restored.getName());
        assertEquals(number, restored.getNumber());
    }

    @Test
    @SuppressWarnings("unchecked")
    public void correctParametricType() throws IOException {

        int id = 100;
        String name = "hello world";
        ObjectWithGenericType<PoJo> genericObject = new ObjectWithGenericType<>();
        PoJo pojo = new PoJo();
        pojo.setName(name);
        genericObject.setContent(pojo);
        genericObject.setId(id);

        EventEnvelope event = new EventEnvelope();
        event.setBody(genericObject);
        event.setParametricType(PoJo.class);
        byte[] b = event.toBytes();

        EventEnvelope result = new EventEnvelope();
        result.load(b);

        Object o = result.getBody();
        assertTrue(o instanceof ObjectWithGenericType);
        ObjectWithGenericType<PoJo> gs = (ObjectWithGenericType<PoJo>) o;
        assertEquals(id, gs.getId());
        PoJo content = gs.getContent();
        assertNotNull(content);
        assertEquals(name, content.getName());
    }

    @Test(expected=ClassCastException.class)
    @SuppressWarnings("unchecked")
    public void missingTypingInfo() throws IOException {

        int id = 100;
        String name = "hello world";
        ObjectWithGenericType<PoJo> genericObject = new ObjectWithGenericType<>();
        PoJo pojo = new PoJo();
        pojo.setName(name);
        genericObject.setContent(pojo);
        genericObject.setId(100);

        EventEnvelope event = new EventEnvelope();
        event.setBody(genericObject);
        byte[] b = event.toBytes();

        EventEnvelope result = new EventEnvelope();
        result.load(b);

        Object o = result.getBody();
        assertTrue(o instanceof ObjectWithGenericType);
        ObjectWithGenericType<PoJo> gs = (ObjectWithGenericType<PoJo>) o;
        // all fields except the ones with generic types can be deserialized correctly
        assertEquals(id, gs.getId());
        /*
         * without parametricType defined, this will throw ClassCastException because the value is a HashMap.
         *
         * Note that Java class with generic types is not type-safe.
         * You therefore can retrieve a copy of the HashMap by this:
         * Object content = gs.getContent();
         */
        PoJo content = gs.getContent();
        assertNotNull(content);
    }

    @Test
    @SuppressWarnings("unchecked")
    public void invalidParametricType() throws IOException {
        Utility util = Utility.getInstance();
        int id = 123;
        String name = "hello world";
        ObjectWithGenericType<PoJo> genericObject = new ObjectWithGenericType<>();
        PoJo pojo = new PoJo();
        pojo.setNumber(id);
        pojo.setName(name);
        genericObject.setContent(pojo);
        genericObject.setId(id);

        EventEnvelope event = new EventEnvelope();
        event.setBody(genericObject);
        event.setParametricType(String.class);  // setting an incorrect type
        byte[] b = event.toBytes();

        EventEnvelope result = new EventEnvelope();
        result.load(b);

        // When parametricType is incorrect, it will fall back to a map.
        Object o = result.getBody();
        assertTrue(o instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map) o);
        assertEquals(name, map.getElement("content.name"));
        // numbers are encoded as string in map
        assertEquals(id, util.str2int(map.getElement("id").toString()));
        assertEquals(id, util.str2int(map.getElement("content.number").toString()));
    }

}
