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

package org.platformlambda.automation.service;

import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.ObjectStreamReader;
import org.platformlambda.core.system.ObjectStreamWriter;

import java.io.IOException;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;

public class MockHelloWorld implements TypedLambdaFunction<AsyncHttpRequest, Object> {

    private static final AtomicInteger count = new AtomicInteger(0);
    @Override
    public Object handleEvent(Map<String, String> headers, AsyncHttpRequest body, int instance) throws IOException {
        AsyncHttpRequest input = new AsyncHttpRequest(body); // test AsyncHttpRequest clone initializer
        if ("HEAD".equals(input.getMethod())) {
            EventEnvelope result = new EventEnvelope().setHeader("X-Response", "HEAD request received")
                    .setHeader("Content-Length", 100);
            if (count.incrementAndGet() == 1) {
                result.setHeader("Set-Cookie", "first=cookie|second=one");
            } else {
                result.setHeader("Set-Cookie", "single=cookie");
            }
            return result;
        }
        if (input.getStreamRoute() != null) {
            ObjectStreamIO stream = new ObjectStreamIO();
            ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
            ObjectStreamReader in = new ObjectStreamReader(input.getStreamRoute(), 10000);
            for (Object o: in) {
                out.write(o);
            }
            out.close();
            return new EventEnvelope().setBody(input.getBody()).setHeader("stream", stream.getInputStreamId())
                    .setHeader("content-type", "application/octet-stream");
        } else if (input.getBody() instanceof byte[]) {
            return new EventEnvelope().setBody(input.getBody())
                    .setHeader("content-type", "application/octet-stream");
        } else {
            // returning AsyncHttpRequest is an edge case
            return body;
        }
    }
}
