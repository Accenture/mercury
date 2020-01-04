/*

    Copyright 2018-2020 Accenture Technology

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
import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.ObjectStreamReader;
import org.platformlambda.core.system.ObjectStreamWriter;

import java.io.IOException;
import java.util.Map;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

public class ObjectStreamTest {

    @Test
    @SuppressWarnings("unchecked")
    public void readWrite() throws IOException {

        String messageOne = "hello world";
        String messageTwo = "it is great";
        /*
         * Producer creates a new stream with 60 seconds inactivity expiry
         */
        ObjectStreamIO producer = new ObjectStreamIO(60);
        ObjectStreamWriter out = producer.getOutputStream();
        out.write(messageOne);
        out.write(messageTwo);
        /*
         * If output stream is closed, it will send an EOF signal so that the input stream reader will detect it.
         * Otherwise, input stream reader will see a RuntimeException of timeout.
         *
         * For this test, we do not close the output stream to demonstrate the timeout.
         */
        //  out.close();

        /*
         * See all open streams in this application instance and verify that the new stream is there
         */
        String streamId = producer.getRoute();
        // remove the node-ID from the fully qualified route name
        String path = streamId.substring(0, streamId.indexOf('@'));
        Map<String, Object> localStreams = producer.getLocalStreams();
        assertTrue(localStreams.containsKey(path));

        /*
         * Producer should send the streamId to the consumer.
         * The consumer can then open the existing stream with the streamId.
         */
        ObjectStreamIO consumer = new ObjectStreamIO(streamId);
        /*
         * read object from the event stream
         * (minimum timeout value is one second)
         */
        ObjectStreamReader in = consumer.getInputStream(1000);
        int i = 0;
        while (!in.isEof()) {
            try {
                for (Object data : in) {
                    i++;
                    if (i == 1) {
                        assertEquals(messageOne, data);
                    }
                    if (i == 2) {
                        assertEquals(messageTwo, data);
                    }
                }
            } catch (RuntimeException e) {
                // iterator will timeout since the stream was not closed
                assertTrue(e.getMessage().contains("timeout"));
                assertTrue(in.isPending());
                break;
            }
        }
        // ensure that it has read the two messages
        assertEquals(2, i);
        // must close input stream to release resources
        in.close();

    }

}
