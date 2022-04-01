package org.platformlambda.core.util;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.util.models.PoJo;

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
        Assert.assertEquals(true, target.getRawBody());
        Assert.assertEquals(true, target.getBody());
    }

    @Test
    public void integerTest() throws IOException {
        int HELLO = 100;
        EventEnvelope source = new EventEnvelope();
        source.setBody(HELLO);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assert.assertEquals(100, target.getRawBody());
        Assert.assertEquals(100, target.getBody());
    }

    @Test
    public void longTest() throws IOException {
        long HELLO = 100L;
        EventEnvelope source = new EventEnvelope();
        source.setBody(HELLO);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        // long will be compressed to integer by MsgPack
        Assert.assertEquals(100, target.getRawBody());
        Assert.assertEquals(100, target.getBody());
    }

    @Test
    public void floatTest() throws IOException {
        float HELLO = 1.23f;
        EventEnvelope source = new EventEnvelope();
        source.setBody(HELLO);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assert.assertEquals(1.23f, target.getRawBody());
        Assert.assertEquals(1.23f, target.getBody());
    }

    @Test
    public void doubleTest() throws IOException {
        double HELLO = 1.23d;
        EventEnvelope source = new EventEnvelope();
        source.setBody(HELLO);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assert.assertEquals(1.23d, target.getRawBody());
        Assert.assertEquals(1.23d, target.getBody());
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
        Assert.assertEquals(VALUE, target.getRawBody());
        Assert.assertEquals(VALUE, target.getBody());
    }

    @Test
    public void dateTest() throws IOException {
        Utility util = Utility.getInstance();
        Date HELLO = new Date();
        EventEnvelope source = new EventEnvelope();
        source.setBody(HELLO);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assert.assertEquals(util.date2str(HELLO), target.getRawBody());
        Assert.assertEquals(util.date2str(HELLO), target.getBody());
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
        Assert.assertTrue(target.getRawBody() instanceof Map);
        Map<String, Object> map = (Map<String, Object>) target.getRawBody();
        Assert.assertEquals(HELLO, map.get("name"));
        Assert.assertTrue(target.getBody() instanceof PoJo);
        PoJo output = (PoJo) target.getBody();
        Assert.assertEquals(HELLO, output.getName());
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
        Assert.assertTrue(target.getRawBody() instanceof Map);
        Map<String, Object> map = (Map<String, Object>) target.getRawBody();
        MultiLevelMap multi = new MultiLevelMap(map);
        Assert.assertEquals(HELLO, multi.getElement("list[0].name"));
        Assert.assertTrue(target.getBody() instanceof List);
        List<PoJo> output = (List<PoJo>) target.getBody();
        Assert.assertEquals(1, output.size());
        Assert.assertEquals(HELLO, output.get(0).getName());
    }

}
