package org.platformlambda.core;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.PoJo;

import java.io.IOException;

public class ExtraEventEnvelopeTest {

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
}
