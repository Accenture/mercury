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

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.serializers.PayloadMapper;
import org.platformlambda.core.util.models.PoJo;

import java.io.IOException;
import java.util.concurrent.TimeoutException;

public class ElasticQueueTest {

    @Test
    public void peeking() throws IOException {
        String firstItem = "hello world 1";
        String secondItem = "hello world 2";
        ElasticQueue spooler = new ElasticQueue("unit.test");
        spooler.write(new EventEnvelope().setBody(firstItem).toBytes());
        spooler.write(new EventEnvelope().setBody(secondItem).toBytes());

        byte[] b = spooler.peek();
        EventEnvelope first = new EventEnvelope();
        first.load(b);
        Assert.assertEquals(firstItem, first.getBody());

        b = spooler.read();
        EventEnvelope firstAgain = new EventEnvelope();
        firstAgain.load(b);
        Assert.assertEquals(firstItem, firstAgain.getBody());

        b = spooler.read();
        EventEnvelope second = new EventEnvelope();
        second.load(b);
        Assert.assertEquals(secondItem, second.getBody());
        Assert.assertNull(spooler.peek());
        Assert.assertNull(spooler.read());
        // elastic queue should be automatically closed when all messages are consumed
        Assert.assertTrue(spooler.isClosed());
        // close elastic queue
        spooler.close();
    }

    @Test
    public void normalPayload() throws IOException, AppException, TimeoutException {
        readWrite("normal.payload.test", 10);
    }

    @Test
    public void largePayload() throws IOException, AppException, TimeoutException {
        readWrite("large.payload.test", 90000);
    }

    private void readWrite(String path, int size) throws IOException {
        String target = "hello.world";
        // create input
        StringBuilder sb = new StringBuilder();
        for (int i=0; i < size; i++) {
            sb.append("0123456789");
        }
        sb.append(": ");
        String baseText = sb.toString();
        ElasticQueue spooler = new ElasticQueue(path);
        // immediate read after write
        for (int i = 0; i < ElasticQueue.MEMORY_BUFFER * 3; i++) {
            String input = baseText + i;
            EventEnvelope event = new EventEnvelope();
            event.setTo(target);
            event.setBody(input);
            spooler.write(event.toBytes());
            byte[] b = spooler.read();
            Assert.assertNotNull(b);
            EventEnvelope data = new EventEnvelope();
            data.load(b);
            Assert.assertEquals(input, data.getBody());
        }
        /*
         * Test overflow to temporary storage
         * by writing a larger number of messages to force buffering to disk
         */
        for (int i = 0; i < ElasticQueue.MEMORY_BUFFER * 5; i++) {
            String input = baseText+i;
            PoJo pojo = new PoJo();
            EventEnvelope event = new EventEnvelope();
            event.setTo(target);
            pojo.setName(input);
            event.setBody(pojo);
            spooler.write(event.toBytes());
        }
        // then 4/5 of the messages
        for (int i = 0; i < ElasticQueue.MEMORY_BUFFER * 4; i++) {
            String input = baseText+i;
            byte[] b = spooler.read();
            EventEnvelope data = new EventEnvelope();
            data.load(b);
            Assert.assertNotNull(data);
            Assert.assertTrue(data.getBody() instanceof PoJo);
            PoJo o = (PoJo) data.getBody();
            Assert.assertEquals(input, o.getName());
        }
        // read one more
        byte[] someData = spooler.read();
        Assert.assertNotNull(someData);
        spooler.close();
        // it should return null when there are no more messages to be read
        byte[] nothing = spooler.read();
        Assert.assertNull(nothing);
        Assert.assertTrue(spooler.isClosed());
        // closing again has no effect
        spooler.close();
        // finally, verify if the PoJo class name is cached
        ManagedCache cache = ManagedCache.getInstance(PayloadMapper.JAVA_CLASS_CACHE);
        Assert.assertNotNull(cache);
        Assert.assertTrue(cache.exists(PoJo.class.getName()));
    }

    @Test
    public void cleanupTest() throws IOException {
        String HELLO_WORLD = "hello world ";
        try (ElasticQueue spooler = new ElasticQueue("unread.test")) {
            for (int i = 0; i < ElasticQueue.MEMORY_BUFFER * 3; i++) {
                String input = HELLO_WORLD + i;
                EventEnvelope event = new EventEnvelope();
                event.setTo("hello.world");
                event.setBody(input);
                spooler.write(event.toBytes());
                if (i < ElasticQueue.MEMORY_BUFFER) {
                    byte[] b = spooler.read();
                    Assert.assertNotNull(b);
                    EventEnvelope data = new EventEnvelope();
                    data.load(b);
                    Assert.assertEquals(input, data.getBody());
                }
            }
            Assert.assertFalse(spooler.isClosed());
        }
    }

}
