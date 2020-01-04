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
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleObjectMapper;

import java.math.BigDecimal;
import java.math.BigInteger;
import java.util.Date;
import java.util.Map;

import static org.junit.Assert.assertEquals;

public class GsonTest {

    @Test
    public void objectToMap() {
        // test custom map serializer
        SimplePoJo obj = getSample();
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
        Map m = mapper.readValue(obj, Map.class);
        assertEquals(String.class, m.get("date").getClass());
        assertEquals(Integer.class, m.get("number").getClass());
        // small long number will be converted to integer
        assertEquals(Integer.class, m.get("small_long").getClass());
        assertEquals(Long.class, m.get("long_number").getClass());
        assertEquals(Float.class, m.get("float_number").getClass());
        // small double number will be converted to float
        assertEquals(Float.class, m.get("small_double").getClass());
        // small double number will be converted to float
        assertEquals(Double.class, m.get("double_number").getClass());
        assertEquals(obj.name, m.get("name"));
        // date is converted to ISO-8601 string
        assertEquals(Utility.getInstance().date2str(obj.date), m.get("date"));
        // big integer and big decimal are converted as String to preserve math precision
        assertEquals(String.class, m.get("big_integer").getClass());
        assertEquals(String.class, m.get("big_decimal").getClass());
    }

    @Test
    public void twoWayConversion() {
        SimplePoJo obj = getSample();
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
        String s = mapper.writeValueAsString(obj);
        SimplePoJo po = mapper.readValue(s, SimplePoJo.class);
        assertEquals(obj.number, po.number);
        assertEquals(obj.smallLong, po.smallLong);
        assertEquals(obj.longNumber, po.longNumber);
        assertEquals(obj.floatNumber, po.floatNumber, 0.0);
        assertEquals(obj.smallDouble, po.smallDouble, 0.0);
        assertEquals(obj.doubleNumber, po.doubleNumber, 0.0);
        assertEquals(obj.name, po.name);
        assertEquals(obj.date, po.date);
        assertEquals(obj.bigInteger, po.bigInteger);
        assertEquals(obj.bigDecimal, po.bigDecimal);
    }

    private SimplePoJo getSample() {
        SimplePoJo sample = new SimplePoJo();
        sample.date = new Date();
        sample.number = 10;
        sample.smallLong = 200L;
        sample.longNumber = 2147483648L; // a number larger than Integer.MAX_VALUE (2147483647)
        sample.floatNumber = 13.3f;
        sample.smallDouble = 26.6d;
        sample.doubleNumber = 3.5E38d; // a number larger than Float.MAX_VALUE (3.4028235E38)
        sample.name = "hello world";
        sample.bigInteger = new BigInteger("36210000122335678901234002030");
        sample.bigDecimal = new BigDecimal("123456789012345890201231.1416");
        return sample;
    }

    private class SimplePoJo {
        int number;
        long longNumber;
        long smallLong;
        float floatNumber;
        double smallDouble;
        double doubleNumber;
        String name;
        Date date;
        BigInteger bigInteger;
        BigDecimal bigDecimal;
    }
}
