package org.platformlambda.core;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
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
        Assertions.assertEquals("", event.getTag(TAG_WITH_NO_VALUE));
        Assertions.assertEquals(WORLD, event.getTag(HELLO));
        Assertions.assertEquals(DATA, event.getTag(ROUTING));
        event.removeTag(HELLO);
        event.removeTag(ROUTING);
        Assertions.assertNull(event.getTag(HELLO));
        Assertions.assertNull(event.getTag(ROUTING));
        Assertions.assertEquals(TAG_WITH_NO_VALUE, event.getExtra());
        event.removeTag(TAG_WITH_NO_VALUE);
        Assertions.assertNull(event.getExtra());
    }
}
