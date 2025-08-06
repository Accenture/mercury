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

import org.junit.jupiter.api.Test;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleObjectMapper;

import java.math.BigDecimal;
import java.math.BigInteger;
import java.util.Date;
import java.util.Map;

import org.junit.jupiter.api.Assertions;
import org.platformlambda.core.util.Utility;

public class GsonTest {

    @Test
    public void objectToMap() {
        // test custom map serializer
        SimplePoJo obj = getSample();
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
        Map m = mapper.readValue(obj, Map.class);
        Assertions.assertEquals(String.class, m.get("date").getClass());
        // small long number will be converted to integer
        Assertions.assertEquals(Integer.class, m.get("number").getClass());
        Assertions.assertEquals(Integer.class, m.get("small_long").getClass());
        Assertions.assertEquals(Long.class, m.get("long_number").getClass());
        Assertions.assertEquals(Double.class, m.get("float_number").getClass());
        // small double number will be converted to float
        Assertions.assertEquals(Double.class, m.get("small_double").getClass());
        // small double number will be converted to float
        Assertions.assertEquals(Double.class, m.get("double_number").getClass());
        Assertions.assertEquals(obj.name, m.get("name"));
        // date is converted to ISO-8601 string
        Assertions.assertEquals(Utility.getInstance().date2str(obj.date), m.get("date"));
        // big integer and big decimal are converted as String to preserve math precision
        Assertions.assertEquals(String.class, m.get("big_integer").getClass());
        Assertions.assertEquals(String.class, m.get("big_decimal").getClass());
    }

    @Test
    public void twoWayConversion() {
        SimplePoJo obj = getSample();
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
        String s = mapper.writeValueAsString(obj);
        SimplePoJo po = mapper.readValue(s, SimplePoJo.class);
        Assertions.assertEquals(obj.number, po.number);
        Assertions.assertEquals(obj.smallLong, po.smallLong);
        Assertions.assertEquals(obj.longNumber, po.longNumber);
        Assertions.assertEquals(obj.floatNumber, po.floatNumber, 0.0);
        Assertions.assertEquals(obj.smallDouble, po.smallDouble, 0.0);
        Assertions.assertEquals(obj.doubleNumber, po.doubleNumber, 0.0);
        Assertions.assertEquals(obj.name, po.name);
        Assertions.assertEquals(obj.date, po.date);
        Assertions.assertEquals(obj.bigInteger, po.bigInteger);
        Assertions.assertEquals(obj.bigDecimal, po.bigDecimal);
    }

    private SimplePoJo getSample() {
        SimplePoJo sample = new SimplePoJo();
        sample.date = new Date();
        sample.number = 10;
        sample.smallLong = 200L;
        sample.longNumber = System.currentTimeMillis();
        sample.floatNumber = 13.3f;
        sample.smallDouble = 26.6d;
        sample.doubleNumber = 3.5E38d;
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
