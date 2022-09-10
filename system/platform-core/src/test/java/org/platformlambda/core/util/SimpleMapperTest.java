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

package org.platformlambda.core.util;

import com.google.gson.JsonPrimitive;
import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleObjectMapper;
import org.platformlambda.core.util.models.PoJo;

import java.math.BigDecimal;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;

import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.MatcherAssert.assertThat;

public class SimpleMapperTest {

    @Test
    public void returnOriginalClassIfSameTargetClass() {
        PoJo pojo = new PoJo();
        pojo.setName("hello");
        pojo.setNumber(123);
        Object o = SimpleMapper.getInstance().getMapper().readValue(pojo, PoJo.class);
        Assert.assertEquals(pojo, o);
    }

    @Test
    public void typedNumberShouldMapDouble() {
        final JsonPrimitive number = new JsonPrimitive("1.12345678");
        Object result = SimpleMapper.getInstance().typedNumber(number);
        assertThat(result, is(1.12345678d));
    }

    @Test
    public void typedNumberShouldMapFloat() {
        final JsonPrimitive number = new JsonPrimitive("1.12");
        Object result = SimpleMapper.getInstance().typedNumber(number);
        assertThat(result, is(1.12d));
    }

    @Test
    public void primitiveDataTest() {
        final boolean bol = true;
        Object bolString = SimpleMapper.getInstance().getMapper().writeValueAsString(bol);
        Assert.assertEquals("true", bolString);
        final int n = 1;
        Object intString = SimpleMapper.getInstance().getMapper().writeValueAsString(n);
        Assert.assertEquals("1", intString);
    }

    @Test
    @SuppressWarnings("unchecked")
    public void mapperSerializationTest() {
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
        Date now = new Date();
        String iso8601 = Utility.getInstance().date2str(now);
        Map<String, Object> map = new HashMap<>();
        map.put("integer", 100);
        map.put("date", now);
        map.put("sql_date", new java.sql.Date(now.getTime()));
        map.put("sql_timestamp", new java.sql.Timestamp(now.getTime()));
        Map<String, Object> converted = mapper.readValue(mapper.writeValueAsString(map), Map.class);
        // verify that java.util.Date, java.sql.Date and java.sql.Timestamp can be serialized to ISO-8601 string format
        Assert.assertEquals(iso8601, converted.get("date"));
        // sql date is yyyy-mm-dd
        Assert.assertEquals(new java.sql.Date(now.getTime()).toString(), converted.get("sql_date"));
        Assert.assertEquals(iso8601, converted.get("sql_timestamp"));
        Assert.assertEquals(Integer.class, converted.get("integer").getClass());
        String name = "hello world";
        Map<String, Object> input = new HashMap<>();
        input.put("full_name", name);
        input.put("date", iso8601);
        PoJo pojo = mapper.readValue(input, PoJo.class);
        // verify that the time is restored correctly
        Assert.assertEquals(now.getTime(), pojo.getDate().getTime());
        // verify that snake case is deserialized correctly
        Assert.assertEquals(name, pojo.getFullName());

        // verify input timestamp can be in milliseconds too
        input.put("date", now.getTime());
        pojo = mapper.readValue(input, PoJo.class);
        Assert.assertEquals(now.getTime(), pojo.getDate().getTime());
    }

    @Test
    @SuppressWarnings("unchecked")
    public void bigDecimalSerializationTests() {
        SimpleMapper mapper = SimpleMapper.getInstance();
        String NUMBER = "number";
        String ONE  = "0.00000001";
        String ZERO = "0.00000000";
        SimpleNumber one  = new SimpleNumber(ONE);
        SimpleNumber zero = new SimpleNumber(ZERO);
        // verify hash map result
        Map<String, Object> mapOne = mapper.getMapper().readValue(one, Map.class);
        // numeric value is preserved
        Assert.assertEquals(ONE, mapOne.get(NUMBER));
        // ensure that ZERO is converted to "0"
        Map<String, Object> mapZero = mapper.getMapper().readValue(zero, Map.class);
        Assert.assertEquals("0", mapZero.get(NUMBER));
        // verify PoJo class conversion behavior - this will return the original object because the class is the same
        SimpleNumber numberOne = mapper.getMapper().readValue(one, SimpleNumber.class);
        Assert.assertEquals(numberOne.number, one.number);
        // this will pass thru the serializer
        String zeroValue = mapper.getMapper().writeValueAsString(zero);
        SimpleNumber numberZero = mapper.getMapper().readValue(zeroValue, SimpleNumber.class);
        // the original number has the zero number with many zeros after the decimal
        Assert.assertEquals("0E-8", zero.number.toString());
        Assert.assertEquals(ZERO, zero.number.toPlainString());
        // the converted BigDecimal gets a zero number without zeros after the decimal
        Assert.assertEquals("0", numberZero.number.toString());
        // verify map to PoJo serialization behavior
        SimpleNumber number0 = mapper.getMapper().readValue(mapZero, SimpleNumber.class);
        Assert.assertTrue(mapper.isZero(number0.number));
        Assert.assertTrue(mapper.isZero(zero.number));
        // the two zero objects are different because of precision
        Assert.assertNotEquals(number0.number, zero.number);
        SimpleNumber number1 = mapper.getMapper().readValue(mapOne, SimpleNumber.class);
        // non-zero numbers are exactly the same
        Assert.assertEquals(number1.number, one.number);
    }

    @Test
    public void bigDecimalTests() {
        String ZERO = "0.00000000";
        BigDecimal zero = new BigDecimal("0");
        BigDecimal zeroes = new BigDecimal(ZERO);
        BigDecimal result = zero.multiply(zeroes);
        // precision is preserved after multiplication
        Assert.assertEquals(zeroes, result);
        Assert.assertEquals(ZERO, result.toPlainString());
        // test zero values
        Assert.assertTrue(SimpleMapper.getInstance().isZero(zero));
        Assert.assertTrue(SimpleMapper.getInstance().isZero(zeroes));
        Assert.assertTrue(SimpleMapper.getInstance().isZero(result));
        Assert.assertTrue(SimpleMapper.getInstance().isZero(result.toPlainString()));
        Assert.assertTrue(SimpleMapper.getInstance().isZero(result.toString()));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void caseMappingTest() {
        SimpleObjectMapper snakeMapper = SimpleMapper.getInstance().getSnakeCaseMapper();
        SimpleObjectMapper camelMapper = SimpleMapper.getInstance().getCamelCaseMapper();
        String NUMBER = "1.234567890";
        CaseDemo sn = new CaseDemo(NUMBER);
        Map<String, Object> snakeMap = snakeMapper.readValue(sn, Map.class);
        Assert.assertEquals(NUMBER, snakeMap.get("case_demo"));
        Map<String, Object> camelMap = camelMapper.readValue(sn, Map.class);
        Assert.assertEquals(NUMBER, camelMap.get("caseDemo"));
        CaseDemo restoredFromSnake = snakeMapper.readValue(snakeMap, CaseDemo.class);
        Assert.assertEquals(NUMBER, restoredFromSnake.caseDemo.toPlainString());
        CaseDemo restoredFromCamel = camelMapper.readValue(camelMap, CaseDemo.class);
        Assert.assertEquals(NUMBER, restoredFromCamel.caseDemo.toPlainString());
    }

    private static class SimpleNumber {
        public BigDecimal number;

        public SimpleNumber(String number) {
            this.number = new BigDecimal(number);
        }
    }

    private static class CaseDemo {
        public BigDecimal caseDemo;

        public CaseDemo(String caseDemo) {
            this.caseDemo = new BigDecimal(caseDemo);
        }
    }

}
