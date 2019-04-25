package org.platformlambda.core.util;

import org.junit.Test;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleObjectMapper;

import java.util.Date;
import java.util.Map;

import static org.junit.Assert.assertEquals;

public class GsonTest {

    @Test
    public void objectToMap() {

        SimplePoJo obj = new SimplePoJo();
        obj.date = new Date();
        obj.number = 10;
        obj.longNumber = 200L;
        obj.floatNumber = 13.3f;
        obj.doubleNumber = 26.6d;
        obj.name = "hello world";

        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();

        Map m = mapper.readValue(obj, Map.class);
        assertEquals(String.class, m.get("date").getClass());
        assertEquals(Integer.class, m.get("number").getClass());
        // small long number will be converted to integer
        assertEquals(Integer.class, m.get("long_number").getClass());
        assertEquals(Float.class, m.get("float_number").getClass());
        // small double number will be converted to float
        assertEquals(Float.class, m.get("double_number").getClass());
        assertEquals(obj.name, m.get("name"));
        // date is converted to ISO-8601 string
        assertEquals(Utility.getInstance().date2str(obj.date), m.get("date"));
    }


    @Test
    public void mapToObject() {
        String s = "{\n" +
                "  \"date\": \"2019-04-25T03:16:49.686Z\",\n" +
                "  \"number\": 10,\n" +
                "  \"long_number\": 200,\n" +
                "  \"double_number\": 26.6,\n" +
                "  \"float_number\": 13.3,\n" +
                "  \"name\": \"hello world\"\n" +
                "}";

        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();

        SimplePoJo po = mapper.readValue(s, SimplePoJo.class);
        assertEquals(10, po.number);
        assertEquals(200, po.longNumber);
        assertEquals(13.3f, po.floatNumber, 0.0);
        assertEquals(26.6d, po.doubleNumber, 0.0);
        assertEquals("hello world", po.name);
    }

    private class SimplePoJo {
        int number;
        long longNumber;
        float floatNumber;
        double doubleNumber;
        String name;
        Date date;
    }
}
