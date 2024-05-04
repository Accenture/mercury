package org.platformlambda.core;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.PoJo;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.math.BigDecimal;
import java.util.*;

public class EventEnvelopeTest {
    private static final Logger log = LoggerFactory.getLogger(EventEnvelope.class);
    private static final String SET_COOKIE = "set-cookie";

    @Test
    public void cookieTest() {
        EventEnvelope event = new EventEnvelope();
        event.setHeader(SET_COOKIE, "a=100");
        event.setHeader(SET_COOKIE, "b=200");
        // set-cookie is a special case for composite value
        Assert.assertEquals("a=100|b=200", event.getHeader(SET_COOKIE));
    }

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
        int VALUE = 100;
        EventEnvelope source = new EventEnvelope();
        source.setBody(VALUE);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assert.assertEquals(VALUE, target.getRawBody());
        Assert.assertEquals(VALUE, target.getBody());
    }

    @Test
    public void longTest() throws IOException {
        Long VALUE = 100L;
        EventEnvelope source = new EventEnvelope();
        source.setBody(VALUE);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        // long will be compressed to integer by MsgPack
        Assert.assertEquals(VALUE.intValue(), target.getRawBody());
        Assert.assertEquals(VALUE.intValue(), target.getBody());
    }

    @Test
    public void floatTest() throws IOException {
        float VALUE = 1.23f;
        EventEnvelope source = new EventEnvelope();
        source.setBody(VALUE);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assert.assertEquals(VALUE, target.getRawBody());
        Assert.assertEquals(VALUE, target.getBody());
    }

    @Test
    public void doubleTest() throws IOException {
        double VALUE = 1.23d;
        EventEnvelope source = new EventEnvelope();
        source.setBody(VALUE);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assert.assertEquals(VALUE, target.getRawBody());
        Assert.assertEquals(VALUE, target.getBody());
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
        Date NOW = new Date();
        EventEnvelope source = new EventEnvelope();
        source.setBody(NOW);
        byte[] b = source.toBytes();
        EventEnvelope target = new EventEnvelope(b);
        Assert.assertEquals(util.date2str(NOW), target.getRawBody());
        Assert.assertEquals(util.date2str(NOW), target.getBody());
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

    @Test
    public void taggingTest() {
        final String HELLO = "hello";
        final String WORLD = "world";
        final String ROUTING = "routing";
        final String DATA = "a->b";
        final String TAG_WITH_NO_VALUE = "tag-with-no-value";
        EventEnvelope event = new EventEnvelope();
        event.addTag(TAG_WITH_NO_VALUE).addTag(HELLO, WORLD).addTag(ROUTING, DATA);
        // When a tag is created with no value, the system will set a "*" as a filler.
        Assert.assertEquals("*", event.getTag(TAG_WITH_NO_VALUE));
        Assert.assertEquals(WORLD, event.getTag(HELLO));
        Assert.assertEquals(DATA, event.getTag(ROUTING));
        event.removeTag(HELLO).removeTag(ROUTING);
        Assert.assertNull(event.getTag(HELLO));
        Assert.assertNull(event.getTag(ROUTING));
        Assert.assertEquals(TAG_WITH_NO_VALUE+"=*", event.getExtra());
        event.removeTag(TAG_WITH_NO_VALUE);
        Assert.assertNull(event.getExtra());
    }

    @Test
    public void fluentTest() throws IOException {
        EventEnvelope event = new EventEnvelope().setRoundTrip(1.1236f).setEndOfRoute().setBinary(true)
                // the second one will replace the value set by the first one
                .setParametricType("com.accenture.SomePoJo").setParametricType(PoJo.class);
        byte[] b = event.toBytes();
        EventEnvelope restored = new EventEnvelope(b);
        Assert.assertEquals(PoJo.class.getName(), restored.getParametricType());
        // verify that the system will save up to 3 decimal points
        Assert.assertEquals(1.124f, restored.getRoundTrip(), 0);
    }

    @Test
    public void mapSerializationTest() {
        String HELLO = "hello";
        PoJo pojo = new PoJo();
        pojo.setName(HELLO);
        pojo.setNumber(10);
        EventEnvelope source = new EventEnvelope();
        source.setBody(pojo);
        source.setFrom("unit.test");
        source.setTo("hello.world");
        source.setReplyTo("my.callback");
        source.setTrace("101", "PUT /api/unit/test");
        // use JSON instead of binary serialization
        source.setBinary(false);
        source.setException(new IllegalArgumentException("hello"));
        source.setBroadcastLevel(1);
        source.setCorrelationId("121");
        source.setExtra("x=y");
        source.setEndOfRoute();
        source.setHeader("a", "b");
        source.setStatus(400);
        source.setExecutionTime(1.23f);
        source.setRoundTrip(2.0f);
        EventEnvelope target = new EventEnvelope(source.toMap());
        MultiLevelMap map = new MultiLevelMap(target.toMap());
        Assert.assertEquals(HELLO, map.getElement("body.name"));
        Assert.assertEquals(10, map.getElement("body.number"));
        Assert.assertEquals(0, map.getElement("body.long_number"));
        Assert.assertEquals("y", target.getTag("x"));
        Assert.assertEquals(1.23f, target.getExecutionTime(), 0f);
        Assert.assertEquals(2.0f, target.getRoundTrip(), 0f);
        Assert.assertEquals("121", target.getCorrelationId());
        Assert.assertEquals(400, target.getStatus());
        Assert.assertEquals(1, target.getBroadcastLevel());
        Assert.assertEquals(source.getId(), target.getId());
        Assert.assertEquals(source.getFrom(), target.getFrom());
        Assert.assertEquals(source.getReplyTo(), target.getReplyTo());
        Assert.assertEquals("101", target.getTraceId());
        Assert.assertEquals("PUT /api/unit/test", target.getTracePath());
        Assert.assertEquals("b", map.getElement("headers.a"));
        Assert.assertEquals(true, map.getElement("json"));
        Assert.assertTrue(target.getBody() instanceof PoJo);
        PoJo output = (PoJo) target.getBody();
        Assert.assertEquals(HELLO, output.getName());
        Assert.assertEquals(HELLO, target.getException().getMessage());
        Assert.assertEquals(IllegalArgumentException.class, target.getException().getClass());
        Assert.assertTrue(map.getElement("exception") instanceof byte[]);
        byte[] b = (byte[]) map.getElement("exception");
        log.info("Stacktrace binary payload size for {} = {}", IllegalArgumentException.class.getName(), b.length);
    }

    @Test
    public void optionalTransportTest() {
        EventEnvelope source = new EventEnvelope();
        source.setBody(Optional.of("hello"));
        EventEnvelope target = new EventEnvelope(source.toMap());
        Assert.assertEquals(Optional.of("hello"), target.getBody());
    }

}
