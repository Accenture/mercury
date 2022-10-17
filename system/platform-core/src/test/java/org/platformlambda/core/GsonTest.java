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
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleObjectMapper;

import java.math.BigDecimal;
import java.math.BigInteger;
import java.util.Date;
import java.util.Map;

import org.junit.Assert;
import org.platformlambda.core.util.Utility;

public class GsonTest {

    @Test
    public void objectToMap() {
        // test custom map serializer
        SimplePoJo obj = getSample();
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
        Map m = mapper.readValue(obj, Map.class);
        Assert.assertEquals(String.class, m.get("date").getClass());
        // small long number will be converted to integer
        Assert.assertEquals(Integer.class, m.get("number").getClass());
        Assert.assertEquals(Integer.class, m.get("small_long").getClass());
        Assert.assertEquals(Long.class, m.get("long_number").getClass());
        Assert.assertEquals(Double.class, m.get("float_number").getClass());
        // small double number will be converted to float
        Assert.assertEquals(Double.class, m.get("small_double").getClass());
        // small double number will be converted to float
        Assert.assertEquals(Double.class, m.get("double_number").getClass());
        Assert.assertEquals(obj.name, m.get("name"));
        // date is converted to ISO-8601 string
        Assert.assertEquals(Utility.getInstance().date2str(obj.date), m.get("date"));
        // big integer and big decimal are converted as String to preserve math precision
        Assert.assertEquals(String.class, m.get("big_integer").getClass());
        Assert.assertEquals(String.class, m.get("big_decimal").getClass());
    }

    @Test
    public void twoWayConversion() {
        SimplePoJo obj = getSample();
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
        String s = mapper.writeValueAsString(obj);
        SimplePoJo po = mapper.readValue(s, SimplePoJo.class);
        Assert.assertEquals(obj.number, po.number);
        Assert.assertEquals(obj.smallLong, po.smallLong);
        Assert.assertEquals(obj.longNumber, po.longNumber);
        Assert.assertEquals(obj.floatNumber, po.floatNumber, 0.0);
        Assert.assertEquals(obj.smallDouble, po.smallDouble, 0.0);
        Assert.assertEquals(obj.doubleNumber, po.doubleNumber, 0.0);
        Assert.assertEquals(obj.name, po.name);
        Assert.assertEquals(obj.date, po.date);
        Assert.assertEquals(obj.bigInteger, po.bigInteger);
        Assert.assertEquals(obj.bigDecimal, po.bigDecimal);
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
