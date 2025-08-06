package org.platformlambda.core;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.PoJo;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.math.BigDecimal;
import java.util.Collections;
import java.util.Date;
import java.util.List;
import java.util.Map;

public class EventEnvelopeRawBodyTest {

    @Test
    public void booleanTest() throws IOException {
        boolean HELLO = true;
        EventEnvelope source = new EventEnvelope();
        source.setBody(HELLO);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assertions.assertEquals(true, target.getRawBody());
        Assertions.assertEquals(true, target.getBody());
    }

    @Test
    public void integerTest() throws IOException {
        int VALUE = 100;
        EventEnvelope source = new EventEnvelope();
        source.setBody(VALUE);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assertions.assertEquals(VALUE, target.getRawBody());
        Assertions.assertEquals(VALUE, target.getBody());
    }

    @Test
    public void longTest() throws IOException {
        Long VALUE = 100L;
        EventEnvelope source = new EventEnvelope();
        source.setBody(VALUE);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        // long will be compressed to integer by MsgPack
        Assertions.assertEquals(VALUE.intValue(), target.getRawBody());
        Assertions.assertEquals(VALUE.intValue(), target.getBody());
    }

    @Test
    public void floatTest() throws IOException {
        float VALUE = 1.23f;
        EventEnvelope source = new EventEnvelope();
        source.setBody(VALUE);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assertions.assertEquals(VALUE, target.getRawBody());
        Assertions.assertEquals(VALUE, target.getBody());
    }

    @Test
    public void doubleTest() throws IOException {
        double VALUE = 1.23d;
        EventEnvelope source = new EventEnvelope();
        source.setBody(VALUE);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assertions.assertEquals(VALUE, target.getRawBody());
        Assertions.assertEquals(VALUE, target.getBody());
    }

    @Test
    public void bigDecimalTest() throws IOException {
        String VALUE = "1.23";
        BigDecimal HELLO = new BigDecimal(VALUE);
        EventEnvelope source = new EventEnvelope();
        source.setBody(HELLO);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        // big decimal is converted to string if it is not encoded in a PoJo
        Assertions.assertEquals(VALUE, target.getRawBody());
        Assertions.assertEquals(VALUE, target.getBody());
    }

    @Test
    public void dateTest() throws IOException {
        Utility util = Utility.getInstance();
        Date NOW = new Date();
        EventEnvelope source = new EventEnvelope();
        source.setBody(NOW);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assertions.assertEquals(util.date2str(NOW), target.getRawBody());
        Assertions.assertEquals(util.date2str(NOW), target.getBody());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void pojoTest() throws IOException {
        String HELLO = "hello";
        PoJo pojo = new PoJo();
        pojo.setName(HELLO);
        EventEnvelope source = new EventEnvelope();
        source.setBody(pojo);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assertions.assertTrue(target.getRawBody() instanceof Map);
        Map<String, Object> map = (Map<String, Object>) target.getRawBody();
        Assertions.assertEquals(HELLO, map.get("name"));
        Assertions.assertTrue(target.getBody() instanceof PoJo);
        PoJo output = (PoJo) target.getBody();
        Assertions.assertEquals(HELLO, output.getName());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void pojoListTest() throws IOException {
        String HELLO = "hello";
        PoJo pojo = new PoJo();
        pojo.setName(HELLO);
        List<PoJo> list = Collections.singletonList(pojo);
        EventEnvelope source = new EventEnvelope();
        source.setBody(list);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        // raw body is encoded as a map containing a list of map
        // e.g. {list=[{number=0, long_number=0, name=hello}]}
        Assertions.assertTrue(target.getRawBody() instanceof Map);
        Map<String, Object> map = (Map<String, Object>) target.getRawBody();
        MultiLevelMap multi = new MultiLevelMap(map);
        Assertions.assertEquals(HELLO, multi.getElement("list[0].name"));
        Assertions.assertTrue(target.getBody() instanceof List);
        List<PoJo> output = (List<PoJo>) target.getBody();
        Assertions.assertEquals(1, output.size());
        Assertions.assertEquals(HELLO, output.get(0).getName());
    }

}
