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
import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.ObjectStreamReader;
import org.platformlambda.core.system.ObjectStreamWriter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Map;

public class ObjectStreamTest {
    private static final Logger log = LoggerFactory.getLogger(ObjectStreamTest.class);

    @Test
    public void expiryTest() throws IOException, InterruptedException {
        Utility util = Utility.getInstance();
        String TEXT = "hello world";
        ObjectStreamIO stream = new ObjectStreamIO(1);
        ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
        ObjectStreamReader in = new ObjectStreamReader(stream.getInputStreamId(), 5000);
        Thread.sleep(500);
        out.write(TEXT);
        Map<String, Object> info = ObjectStreamIO.getStreamInfo();
        Assert.assertNotNull(info.get("count"));
        int count = util.str2int(info.get("count").toString());
        Assert.assertTrue(count > 0);
        String id = stream.getInputStreamId().substring(0, stream.getInputStreamId().indexOf('@'));
        Assert.assertTrue(info.containsKey(id));
        int n = 0;
        for (Object data : in) {
            n++;
            Assert.assertEquals(TEXT, data);
            break;
        }
        Assert.assertEquals(1, n);
        // the stream will expire after one second of inactivity
        Thread.sleep(1200);
        // the system will check expired streams every 20 seconds so we don't want to wait in a unit test
        ObjectStreamIO.removeExpiredStreams();
        Map<String, Object> infoAfterExpiry = ObjectStreamIO.getStreamInfo();
        Assert.assertFalse(infoAfterExpiry.containsKey(id));
    }

    @Test
    public void readWrite() throws IOException {
        int CYCLES = 100;
        String TEXT = "hello world";
        /*
         * Producer creates a new stream with 60 seconds inactivity expiry
         */
        ObjectStreamIO stream = new ObjectStreamIO(60);
        log.info("Using {}", stream.getInputStreamId());
        ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
        for (int i=0; i < CYCLES; i++) {
            out.write(TEXT+" "+i);
        }
        // closing an output stream will send an EOF signal
        out.close();
        // ObjectStreamReader is auto closeable. Remember to close it to release resources.
        int n = 0;
        try (ObjectStreamReader in = new ObjectStreamReader(stream.getInputStreamId(), 8000)) {
            for (Object data : in) {
                n++;
                // it is an EOF signal when null is received
                if (data == null) {
                    log.info("Got {} items where the last one is an EOF signal", n);
                    break;
                }
            }
        }
        Assert.assertEquals(CYCLES + 1, n);
    }

    @Test
    public void mixedTypeStream() throws IOException {

        try {
            timeoutTest();
        } catch(Exception e) {
            log.error("{}", e.getMessage());
        }

        String TEXT = " hello world";
        ObjectStreamIO stream = new ObjectStreamIO();

        log.info("Using {}", stream.getInputStreamId());

        Assert.assertEquals(ObjectStreamIO.DEFAULT_TIMEOUT, stream.getExpirySeconds());
        ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
        byte[] b = (1+TEXT).getBytes();
        out.write(b, 0, b.length);
        out.write(2+TEXT);
        out.close();
        int n = 0;
        try (ObjectStreamReader in = new ObjectStreamReader(stream.getInputStreamId(), 8000)) {
            for (Object d : in) {
                n++;
                if (n == 1) {
                    Assert.assertTrue(d instanceof byte[]);
                    String firstOne = Utility.getInstance().getUTF((byte[]) d);
                    Assert.assertTrue(firstOne.startsWith("1"));
                    log.info("Got 1st item as bytes - {}", d);
                }
                if (n == 2) {
                    Assert.assertTrue(d instanceof String);
                    String secondOne = (String) d;
                    Assert.assertTrue(secondOne.startsWith("2"));
                    log.info("Got 2nd item as string '{}'", d);
                }
                // the last item is an EOF signal of null value
                if (n == 3) {
                    Assert.assertNull(d);
                    log.info("Got 3nd item as an EOF signal");
                }
            }
        }
        Assert.assertEquals(3, n);
    }

    @Test(expected = RuntimeException.class)
    public void timeoutTest() throws IOException {
        String TEXT = "hello world";
        ObjectStreamIO stream = new ObjectStreamIO();

        log.info("Using {}", stream.getInputStreamId());

        ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
        out.write(TEXT);
        // stop writing and do not close output stream so there are no more item to come
        int n = 0;
        try (ObjectStreamReader in = new ObjectStreamReader(stream.getInputStreamId(), 2000)) {
            for (Object d : in) {
                n++;
                if (n == 1) {
                    Assert.assertEquals(TEXT, d);
                    log.info("Got the 1st item '{}'. The 2nd item is designed to timeout in 2 seconds.", d);
                }
            }
        }
    }

}
