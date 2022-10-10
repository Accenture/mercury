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

package org.platformlambda.automation.mock;

import org.junit.BeforeClass;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.util.concurrent.TimeoutException;
import java.util.concurrent.atomic.AtomicInteger;

public class TestBase {
    protected static final String HTTP_REQUEST = "async.http.request";
    protected static int port;

    private static final AtomicInteger startCounter = new AtomicInteger(0);

    @BeforeClass
    public static void setup() throws IOException, TimeoutException {
        if (startCounter.incrementAndGet() == 1) {
            Utility util = Utility.getInstance();
            AppConfigReader config = AppConfigReader.getInstance();
            port = util.str2int(config.getProperty("rest.server.port",
                                config.getProperty("server.port", "8100")));
            AppStarter.main(new String[0]);
            AtomicInteger count = new AtomicInteger(0);
            LambdaFunction f = (headers, body, instance) -> {
                AsyncHttpRequest input = new AsyncHttpRequest(body);
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
                } else

                if (input.getBody() instanceof byte[]) {
                    return new EventEnvelope().setBody(input.getBody())
                                .setHeader("content-type", "application/octet-stream");
                } else {
                    return body;
                }
            };
            Platform platform = Platform.getInstance();
            platform.registerPrivate("hello.world", f, 10);
            // wait for service to be ready
            platform.waitForProvider("notification.manager", 20);
        }
    }
}
