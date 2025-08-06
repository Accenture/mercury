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

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.math.BigDecimal;
import java.time.LocalDateTime;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import org.platformlambda.core.models.PoJo;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleObjectMapper;
import org.platformlambda.core.util.Utility;

import com.google.gson.JsonPrimitive;


public class SimpleMapperTest {

    @Test
    public void returnOriginalClassIfSameTargetClass() {
        PoJo pojo = new PoJo();
        pojo.setName("hello");
        pojo.setNumber(123);
        Object o = SimpleMapper.getInstance().getMapper().readValue(pojo, PoJo.class);
        Assertions.assertEquals(pojo, o);
    }

    @Test
    public void typedNumberShouldMapDouble() {
        final JsonPrimitive number = new JsonPrimitive("1.12345678");
        Object result = SimpleMapper.getInstance().typedNumber(number);
        assertEquals(1.12345678d, result);
    }

    @Test
    public void typedNumberShouldMapFloat() {
        final JsonPrimitive number = new JsonPrimitive("1.12");
        Object result = SimpleMapper.getInstance().typedNumber(number);
        assertEquals(1.12d, result);
    }

    @Test
    public void primitiveDataTest() {
        final boolean bol = true;
        Object bolString = SimpleMapper.getInstance().getMapper().writeValueAsString(bol);
        Assertions.assertEquals("true", bolString);
        final int n = 1;
        Object intString = SimpleMapper.getInstance().getMapper().writeValueAsString(n);
        Assertions.assertEquals("1", intString);
    }

    @Test
    @SuppressWarnings("unchecked")
    public void mapperSerializationTest() {
        Utility util = Utility.getInstance();
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
        Date now = new Date();
        LocalDateTime time = LocalDateTime.now();
        String iso8601 = util.date2str(now);
        String iso8601NoTimeZone = time.toString();
        Map<String, Object> map = new HashMap<>();
        map.put("integer", 100);
        map.put("date", now);
        map.put("time", time);
        map.put("sql_date", new java.sql.Date(now.getTime()));
        map.put("sql_timestamp", new java.sql.Timestamp(now.getTime()));
        Map<String, Object> converted = mapper.readValue(mapper.writeValueAsString(map), Map.class);
        // verify that java.util.Date, java.sql.Date and java.sql.Timestamp can be serialized to ISO-8601 string format
        Assertions.assertEquals(iso8601, converted.get("date"));
        // LocalDateTime string will drop the "T" separator
        Assertions.assertEquals(iso8601NoTimeZone.replace('T', ' '), converted.get("time"));
        // sql date is yyyy-mm-dd
        Assertions.assertEquals(new java.sql.Date(now.getTime()).toString(), converted.get("sql_date"));
        Assertions.assertEquals(iso8601, converted.get("sql_timestamp"));
        Assertions.assertEquals(Integer.class, converted.get("integer").getClass());
        String name = "hello world";
        Map<String, Object> input = new HashMap<>();
        input.put("full_name", name);
        input.put("date", iso8601);
        input.put("time", iso8601NoTimeZone);
        PoJo pojo = mapper.readValue(input, PoJo.class);
        // verify that the time is restored correctly
        Assertions.assertEquals(now.getTime(), pojo.getDate().getTime());
        Assertions.assertEquals(time, pojo.getTime());
        // verify that snake case is deserialized correctly
        Assertions.assertEquals(name, pojo.getFullName());
        // verify input timestamp can be in milliseconds too
        input.put("date", now.getTime());
        pojo = mapper.readValue(input, PoJo.class);
        Assertions.assertEquals(now.getTime(), pojo.getDate().getTime());
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
        Assertions.assertEquals(ONE, mapOne.get(NUMBER));
        // ensure that ZERO is converted to "0"
        Map<String, Object> mapZero = mapper.getMapper().readValue(zero, Map.class);
        Assertions.assertEquals("0", mapZero.get(NUMBER));
        // verify PoJo class conversion behavior - this will return the original object because the class is the same
        SimpleNumber numberOne = mapper.getMapper().readValue(one, SimpleNumber.class);
        Assertions.assertEquals(numberOne.number, one.number);
        // this will pass thru the serializer
        String zeroValue = mapper.getMapper().writeValueAsString(zero);
        SimpleNumber numberZero = mapper.getMapper().readValue(zeroValue, SimpleNumber.class);
        // the original number has the zero number with many zeros after the decimal
        Assertions.assertEquals("0E-8", zero.number.toString());
        Assertions.assertEquals(ZERO, zero.number.toPlainString());
        // the converted BigDecimal gets a zero number without zeros after the decimal
        Assertions.assertEquals("0", numberZero.number.toString());
        // verify map to PoJo serialization behavior
        SimpleNumber number0 = mapper.getMapper().readValue(mapZero, SimpleNumber.class);
        Assertions.assertTrue(mapper.isZero(number0.number));
        Assertions.assertTrue(mapper.isZero(zero.number));
        // the two zero objects are different because of precision
        Assertions.assertNotEquals(number0.number, zero.number);
        SimpleNumber number1 = mapper.getMapper().readValue(mapOne, SimpleNumber.class);
        // non-zero numbers are exactly the same
        Assertions.assertEquals(number1.number, one.number);
    }

    @Test
    public void bigDecimalTests() {
        String ZERO = "0.00000000";
        BigDecimal zero = new BigDecimal("0");
        BigDecimal zeroes = new BigDecimal(ZERO);
        BigDecimal result = zero.multiply(zeroes);
        // precision is preserved after multiplication
        Assertions.assertEquals(zeroes, result);
        Assertions.assertEquals(ZERO, result.toPlainString());
        // test zero values
        Assertions.assertTrue(SimpleMapper.getInstance().isZero(zero));
        Assertions.assertTrue(SimpleMapper.getInstance().isZero(zeroes));
        Assertions.assertTrue(SimpleMapper.getInstance().isZero(result));
        Assertions.assertTrue(SimpleMapper.getInstance().isZero(result.toPlainString()));
        Assertions.assertTrue(SimpleMapper.getInstance().isZero(result.toString()));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void caseMappingTest() {
        SimpleObjectMapper snakeMapper = SimpleMapper.getInstance().getSnakeCaseMapper();
        SimpleObjectMapper camelMapper = SimpleMapper.getInstance().getCamelCaseMapper();
        String NUMBER = "1.234567890";
        CaseDemo sn = new CaseDemo(NUMBER);
        Map<String, Object> snakeMap = snakeMapper.readValue(sn, Map.class);
        Assertions.assertEquals(NUMBER, snakeMap.get("case_demo"));
        Map<String, Object> camelMap = camelMapper.readValue(sn, Map.class);
        Assertions.assertEquals(NUMBER, camelMap.get("caseDemo"));
        CaseDemo restoredFromSnake = snakeMapper.readValue(snakeMap, CaseDemo.class);
        Assertions.assertEquals(NUMBER, restoredFromSnake.caseDemo.toPlainString());
        CaseDemo restoredFromCamel = camelMapper.readValue(camelMap, CaseDemo.class);
        Assertions.assertEquals(NUMBER, restoredFromCamel.caseDemo.toPlainString());
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
