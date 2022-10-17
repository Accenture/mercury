package org.platformlambda.core;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;

public class ExtraEventEnvelopeTest {

    @Test
    public void taggingTest() {
        final String HELLO = "hello";
        final String WORLD = "world";
        final String ROUTING = "routing";
        final String DATA = "a->b";
        final String TAG_WITH_NO_VALUE = "tag-with-no-value";
        EventEnvelope event = new EventEnvelope();
        event.addTag(HELLO, WORLD);
        event.addTag(ROUTING, DATA);
        event.addTag(TAG_WITH_NO_VALUE);
        Assert.assertEquals("", event.getTag(TAG_WITH_NO_VALUE));
        Assert.assertEquals(WORLD, event.getTag(HELLO));
        Assert.assertEquals(DATA, event.getTag(ROUTING));
        event.removeTag(HELLO);
        event.removeTag(ROUTING);
        Assert.assertNull(event.getTag(HELLO));
        Assert.assertNull(event.getTag(ROUTING));
        Assert.assertEquals(TAG_WITH_NO_VALUE, event.getExtra());
        event.removeTag(TAG_WITH_NO_VALUE);
        Assert.assertNull(event.getExtra());
    }
}
