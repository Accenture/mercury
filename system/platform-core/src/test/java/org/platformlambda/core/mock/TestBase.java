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

package org.platformlambda.core.mock;

import io.vertx.core.Vertx;
import io.vertx.core.http.HttpServer;
import io.vertx.core.http.HttpServerOptions;
import org.junit.BeforeClass;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.server.MinimalistHttpHandler;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.concurrent.TimeoutException;
import java.util.concurrent.atomic.AtomicInteger;

public class TestBase {
    private static final Logger log = LoggerFactory.getLogger(TestBase.class);

    protected static final String HELLO_WORLD = "hello.world";
    protected static final String HELLO_MOCK = "hello.mock";
    protected static final String CLOUD_CONNECTOR_HEALTH = "cloud.connector.health";
    protected static final int MINIMALIST_HTTP_PORT = 8020;
    private static final String SERVICE_LOADED = "http.service.loaded";
    protected static int port;

    private static final AtomicInteger startCounter = new AtomicInteger(0);

    @BeforeClass
    public static void setup() throws IOException {
        if (startCounter.incrementAndGet() == 1) {
            Utility util = Utility.getInstance();
            AppConfigReader config = AppConfigReader.getInstance();
            port = util.str2int(config.getProperty("server.port", "8100"));
            AppStarter.runAsSpringBootApp();
            AppStarter.main(new String[0]);
            AppStarter.runMainApp();
            ServerPersonality.getInstance().setType(ServerPersonality.Type.REST);
            Platform platform = Platform.getInstance();
            try {
                platform.waitForProvider(AppStarter.ASYNC_HTTP_RESPONSE, 20);
                platform.waitForProvider(CLOUD_CONNECTOR_HEALTH, 20);
                // you can convert a private function to public when needed
                platform.waitForProvider(HELLO_WORLD, 5);
                log.info("Mock cloud ready");
            } catch (TimeoutException e) {
                log.error("{} not ready - {}", CLOUD_CONNECTOR_HEALTH, e.getMessage());
            }
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
                } else if (input.getBody() instanceof byte[]) {
                    return new EventEnvelope().setBody(input.getBody())
                            .setHeader("content-type", "application/octet-stream");
                } else {
                    return body;
                }
            };
            platform.registerPrivate(HELLO_MOCK, f, 10);
            platform.makePublic(HELLO_MOCK);
            // load minimalist HTTP server
            Vertx vertx = Vertx.vertx();
            HttpServerOptions options = new HttpServerOptions().setTcpKeepAlive(true);
            HttpServer server = vertx.createHttpServer(options);
            server.requestHandler(new MinimalistHttpHandler());
            server.listen(MINIMALIST_HTTP_PORT)
                    .onSuccess(service -> {
                        try {
                            platform.registerPrivate(SERVICE_LOADED, (headers, body, instance) -> true, 1);
                        } catch (IOException e) {
                            throw new RuntimeException(e);
                        }
                    })
                    .onFailure(ex -> {
                        log.error("Unable to start - {}", ex.getMessage());
                        System.exit(-1);
                    });
            try {
                platform.waitForProvider(SERVICE_LOADED, 20);
            } catch (TimeoutException e) {
                throw new RuntimeException(e);
            }
        }
    }
}
