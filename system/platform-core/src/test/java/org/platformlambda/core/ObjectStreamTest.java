/*

    Copyright 2018-2024 Accenture Technology

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

package org.platformlambda.core;

import io.vertx.core.Future;
import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.system.AsyncObjectStreamReader;
import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.ObjectStreamWriter;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class ObjectStreamTest {
    private static final Logger log = LoggerFactory.getLogger(ObjectStreamTest.class);

    @Test
    public void expiryTest() throws IOException, InterruptedException {
        final BlockingQueue<Boolean> bench = new ArrayBlockingQueue<>(1);
        Utility util = Utility.getInstance();
        String TEXT = "hello world";
        // The minimum timeout is one second if you set it to a smaller value
        ObjectStreamIO unused = new ObjectStreamIO(0);
        Assert.assertEquals(1, unused.getExpirySeconds());
        String unusedStream = unused.getInputStreamId().substring(0, unused.getInputStreamId().indexOf('@'));
        // create a stream with 5 second expiry
        ObjectStreamIO stream = new ObjectStreamIO(5);
        ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
        out.write(TEXT);
        Map<String, Object> info = ObjectStreamIO.getStreamInfo();
        Assert.assertNotNull(info.get("count"));
        int count = util.str2int(info.get("count").toString());
        Assert.assertTrue(count > 0);
        String id = stream.getInputStreamId().substring(0, stream.getInputStreamId().indexOf('@'));
        Assert.assertTrue(info.containsKey(id));
        AsyncObjectStreamReader in = new AsyncObjectStreamReader(stream.getInputStreamId(), 5000);
        in.get().onSuccess(data -> {
            Assert.assertEquals(TEXT, data);
            bench.offer(true);
        });
        bench.poll(10, TimeUnit.SECONDS);
        // The stream is intentionally left open
        Thread.sleep(1100);
        /*
         * The system will check expired streams every 30 seconds
         * To avoid waiting it in a unit test, we force it to remove expired streams
         */
        ObjectStreamIO.checkExpiredStreams();
        Map<String, Object> allStreams = ObjectStreamIO.getStreamInfo();
        // the unused stream has already expired
        Assert.assertFalse(allStreams.containsKey(unusedStream));
        log.info("{} has expired", unusedStream);
        // The stream has not yet expired
        Assert.assertTrue(allStreams.containsKey(id));
        log.info("{} is still active", id);
        // Sleep past the 5-second mark
        Thread.sleep(4000);
        ObjectStreamIO.checkExpiredStreams();
        allStreams = ObjectStreamIO.getStreamInfo();
        // It should be gone at this point
        Assert.assertFalse(allStreams.containsKey(id));
    }

    @Test
    public void asyncReadWrite() throws IOException, InterruptedException {
        int CYCLES = 10;
        String TEXT = "hello world";
        /*
         * Producer creates a new stream with 60 seconds inactivity expiry
         */
        ObjectStreamIO stream = new ObjectStreamIO(1);
        log.info("Using {}", stream.getInputStreamId());
        // Closing an output stream will send an EOF signal. The try auto-close will do this.
        try (ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId())) {
            for (int i = 0; i < CYCLES; i++) {
                out.write(TEXT + " " + i);
            }
        }
        BlockingQueue<Integer> bench = new ArrayBlockingQueue<>(1);
        AsyncObjectStreamReader in = new AsyncObjectStreamReader(stream.getInputStreamId(), 8000);
        // AsyncObjectStreamReader is non-blocking. Therefore, we must use a blocking queue in a unit test.
        log.info("Beginning of Stream");
        fetchNextBlock(in, 0, bench);
        Integer count = bench.poll(10, TimeUnit.SECONDS);
        Assert.assertNotNull(count);
        Assert.assertEquals(CYCLES, count.intValue());
        Assert.assertTrue(in.isStreamEnd());
        Assert.assertTrue(in.isClosed());
    }

    private void fetchNextBlock(AsyncObjectStreamReader in, int count, BlockingQueue<Integer> bench) {
        Future<Object> block = in.get();
        block.onSuccess(b -> {
            if (b != null) {
                log.info("{}", b);
                fetchNextBlock(in, count+1, bench);
            } else {
                try {
                    in.close();
                    log.info("End of Stream");
                } catch (IOException e) {
                    log.error("Unable to close stream - {}", e.getMessage());
                } finally {
                    bench.offer(count);
                }
            }
        });
    }

}
