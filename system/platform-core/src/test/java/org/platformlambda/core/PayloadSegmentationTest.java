/*

    Copyright 2018-2023 Accenture Technology

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

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;

import java.io.IOException;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class PayloadSegmentationTest {
    private static final String TEST_STRING = "123456789.";
    private static final int CYCLE = 30000;

    @Test
    public void multiPart() throws IOException, InterruptedException {
        /*
         * Generate large payload of over 64 KB
         * (for this test, we generate 500,000 bytes)
         */
        StringBuilder sb = new StringBuilder();
        for (int i=0; i < CYCLE; i++) {
            sb.append(TEST_STRING);
        }
        BlockingQueue<Integer> bench = new ArrayBlockingQueue<>(1);
        MultipartPayload multipart = MultipartPayload.getInstance();
        String RECEIVER = "large.payload.receiver";
        Platform platform = Platform.getInstance();
        // create function to receive large payload
        LambdaFunction f = (headers, input, instance) -> {
            if (input instanceof byte[]) {
                byte[] b = (byte[]) input;
                if (headers.containsKey("to")) {
                    EventEnvelope e = new EventEnvelope();
                    e.load(b);
                    if (e.getTo() != null) {
                        // reconstructed event
                        EventEmitter.getInstance().send(e);
                    } else {
                        // segmented payload
                        multipart.incoming(e);
                    }
                } else {
                    bench.offer(b.length);
                    Assert.assertEquals(b.length, sb.length());
                }
            }
            return true;
        };
        platform.registerPrivate(RECEIVER, f, 1);
        EventEnvelope event = new EventEnvelope();
        event.setTo(RECEIVER).setBody(Utility.getInstance().getUTF(sb.toString()));
        multipart.outgoing(RECEIVER, event);
        // wait for receiver to acknowledge message
        Integer size = bench.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(size);
        Assert.assertEquals((int) size, sb.length());
    }

}
