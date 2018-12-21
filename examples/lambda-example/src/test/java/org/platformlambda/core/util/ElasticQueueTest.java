/*

    Copyright 2018 Accenture Technology

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

import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;

import java.io.File;
import java.io.IOException;

import static org.junit.Assert.assertTrue;
import static org.junit.Assert.assertEquals;

public class ElasticQueueTest {

    @Test
    public void normalPayload() throws IOException {
        readWrite("normal", 10);
    }

    @Test
    public void largePayload() throws IOException {
        readWrite("large", 90000);
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
        // temp queue directory
        File dir = new File("fifo");
        ElasticQueue spooler = new ElasticQueue(dir, path);
        // immediate read after write
        for (int i = 0; i < ElasticQueue.MEMORY_BUFFER * 3; i++) {
            String input = baseText+i;
            EventEnvelope event = new EventEnvelope();
            event.setTo(target);
            event.setBody(input);
            spooler.write(event);
            EventEnvelope data = spooler.read();
            assertTrue(data != null);
            assertTrue(input.equals(data.getBody()));
        }
        // write a large number of messages first so it will overflow to disk
        for (int i = 0; i < ElasticQueue.MEMORY_BUFFER * 3; i++) {
            String input = baseText+i;
            EventEnvelope event = new EventEnvelope();
            event.setTo(target);
            event.setBody(input);
            spooler.write(event);
        }
        // then read the messages
        for (int i = 0; i < ElasticQueue.MEMORY_BUFFER * 3; i++) {
            String input = baseText+i;
            EventEnvelope data = spooler.read();
            assertTrue(data != null);
            assertTrue(input.equals(data.getBody()));
        }
        // it should return null when there are no more messages to be read
        EventEnvelope nothing = spooler.read();
        assertEquals(null, nothing);
        // elastic queue should be automatically closed when all messages are consumed
        assertTrue(spooler.isClosed());
        // closing again has no effect
        spooler.close();
        // remove temp directory
        if (dir.exists()) {
            Utility.getInstance().cleanupDir(dir);
        }
    }

}
