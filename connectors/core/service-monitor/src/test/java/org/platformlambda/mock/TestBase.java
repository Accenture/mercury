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

package org.platformlambda.mock;

import io.vertx.core.Future;
import org.junit.BeforeClass;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.AutoStart;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;

public class TestBase {
    protected static final String HTTP_CLIENT = "async.http.request";
    protected static int port;

    private static final AtomicInteger startCounter = new AtomicInteger(0);

    @BeforeClass
    public static void setup() {
        if (startCounter.incrementAndGet() == 1) {
            Utility util = Utility.getInstance();
            AppConfigReader config = AppConfigReader.getInstance();
            port = util.str2int(config.getProperty("server.port", "8080"));
            AutoStart.main(new String[0]);
        }
    }

    protected EventEnvelope httpGet(String host, String path, Map<String, String> headers)
            throws IOException, InterruptedException {
        // BlockingQueue should only be used in unit test
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest().setMethod("GET").setTargetHost(host).setUrl(path);
        if (headers != null) {
            for (Map.Entry<String, String> kv: headers.entrySet()) {
                req.setHeader(kv.getKey(), kv.getValue());
            }
        }
        EventEnvelope event = new EventEnvelope().setTo(HTTP_CLIENT).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(event, 10000);
        res.onSuccess(bench::offer);
        return bench.poll(10, TimeUnit.SECONDS);
    }

    protected EventEnvelope httpPost(String host, String path,
                                          Map<String, String> headers, Map<String, Object> body)
            throws IOException, InterruptedException {
        // BlockingQueue should only be used in unit test
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        AsyncHttpRequest req = new AsyncHttpRequest().setMethod("POST")
                .setTargetHost(host).setUrl(path).setBody(body);
        if (headers != null) {
            for (Map.Entry<String, String> kv: headers.entrySet()) {
                req.setHeader(kv.getKey(), kv.getValue());
            }
        }
        EventEnvelope event = new EventEnvelope().setTo(HTTP_CLIENT).setBody(req);
        Future<EventEnvelope> res = po.asyncRequest(event, 10000);
        res.onSuccess(bench::offer);
        return bench.poll(10, TimeUnit.SECONDS);
    }
}
