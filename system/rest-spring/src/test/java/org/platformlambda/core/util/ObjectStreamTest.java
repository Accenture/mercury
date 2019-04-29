/*

    Copyright 2018-2019 Accenture Technology

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
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.ObjectStreamReader;
import org.platformlambda.core.system.ObjectStreamWriter;
import org.platformlambda.core.system.PostOffice;

import java.io.IOException;
import java.util.Map;
import java.util.concurrent.TimeoutException;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;
import static org.platformlambda.core.system.Platform.STREAM_MANAGER;

public class ObjectStreamTest {

    @Test
    @SuppressWarnings("unchecked")
    public void readWrite() throws IOException, TimeoutException, AppException {

        String messageOne = "hello world";
        String messageTwo = "it is great";

        PostOffice po = PostOffice.getInstance();
        EventEnvelope response = po.request(STREAM_MANAGER, 5000, new Kv("type", "create"));
        assertTrue(response.getBody() instanceof String);

        String fqPath = (String) response.getBody();
        assertTrue(fqPath.startsWith("stream."));
        // fully qualified path = streamId @ origin
        assertTrue(fqPath.contains("@"));

        ObjectStreamWriter out = new ObjectStreamWriter(fqPath);
        out.write(messageOne);
        out.write(messageTwo);
        // do not close output stream to demonstrate that the iterator will timeout during read
        // out.close();

        /*
         * get a list of all open streams and verify that the new I/O stream exists
         * e.g. {total=1, streams={stream.97c7d66f9a0f43a685ab01b7fcae00a4=2018-05-30T20:30:14.853Z}}
         */
        String path = fqPath.substring(0, fqPath.indexOf('@'));
        EventEnvelope query = po.request(STREAM_MANAGER, 5000, new Kv("type", "query"));
        assertTrue(query.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) query.getBody();
        assertTrue(result.containsKey("streams"));
        Map<String, Object> map = (Map<String, Object>) result.get("streams");
        assertTrue(map.containsKey(path));

        ObjectStreamReader in = new ObjectStreamReader(fqPath, 1000);
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
        // must close input stream to release resources
        in.close();
    }

}
