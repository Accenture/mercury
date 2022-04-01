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

import com.google.api.client.http.*;
import com.google.api.client.http.javanet.NetHttpTransport;
import io.vertx.core.Vertx;
import io.vertx.core.buffer.Buffer;
import org.junit.Assert;
import org.junit.BeforeClass;
import org.junit.Test;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.websocket.client.PersistentWsClient;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class WebSocketTest {
    private static final Logger log = LoggerFactory.getLogger(WebSocketTest.class);

    private static final HttpTransport HTTP_TRANSPORT = new NetHttpTransport();
    private static final HttpRequestFactory factory = HTTP_TRANSPORT.createRequestFactory();
    private static final String HELLO_WORLD = "hello world";
    private static final String WELCOME = "welcome";
    private static final int PORT = 8086;

    @BeforeClass
    public static void setup() {
        Utility util = Utility.getInstance();
        Vertx vertx = Vertx.vertx();
        vertx.createHttpServer()
                .webSocketHandler(ws -> {
                    ws.accept();
                    ws.handler(data -> {
                        String text = util.getUTF(data.getBytes());
                        if (WELCOME.equals(text)) {
                            ws.writeBinaryMessage(Buffer.buffer(WELCOME.getBytes()));
                        } else {
                            ws.writeTextMessage(text);
                        }
                    });
                    ws.closeHandler(end -> log.info("socket closed"));
                })
                .requestHandler(request -> request.response().end(HELLO_WORLD))
                .listen(PORT)
                .onSuccess(server -> log.info("Listening to port {}", server.actualPort()))
                .onFailure(ex -> {
                    log.error("Unable to start - {}", ex.getMessage());
                    System.exit(-1);
                });

    }

    @Test
    public void httpTest() throws IOException {
        GenericUrl target = new GenericUrl("http://127.0.0.1:"+PORT);
        HttpRequest request = factory.buildGetRequest(target);
        HttpResponse response = request.execute();
        int rc = response.getStatusCode();
        String statusMessage = response.getStatusMessage();
        InputStream in = response.getContent();
        String result = Utility.getInstance().stream2str(in);
        Assert.assertEquals(200, rc);
        Assert.assertEquals("OK", statusMessage);
        Assert.assertEquals(HELLO_WORLD, result);
    }

    @Test
    public void wsTest() throws InterruptedException {
        final BlockingQueue<Boolean> bench = new ArrayBlockingQueue<>(1);
        String MESSAGE = "hello world";
        Utility util = Utility.getInstance();
        PostOffice po = PostOffice.getInstance();
        List<String> welcome = new ArrayList<>();
        LambdaFunction connector = (headers, body, instance) -> {
            if ("open".equals(headers.get("type"))) {
                String txPath = headers.get("tx_path");
                Assert.assertNotNull(txPath);
                po.send(txPath, WELCOME.getBytes());
                po.send(txPath, MESSAGE);
            }
            if ("string".equals(headers.get("type"))) {
                Assert.assertTrue(body instanceof String);
                String text = (String) body;
                if (text.startsWith("{") && text.contains("keep-alive")) {
                    bench.offer(true);
                } else {
                    Assert.assertEquals(MESSAGE, text);
                }
            }
            if ("bytes".equals(headers.get("type"))) {
                Assert.assertTrue(body instanceof byte[]);
                welcome.add(util.getUTF( (byte[]) body));
            }
            return true;
        };
        for (int i=0; i < 3; i++) {
            if (Utility.getInstance().portReady("127.0.0.1", PORT, 3000)) {
                break;
            } else {
                log.info("Waiting for websocket server at port-{} to get ready", PORT);
            }
        }
        PersistentWsClient client = new PersistentWsClient(connector,
                Collections.singletonList("ws://127.0.0.1:"+PORT+"/ws/test/hi"));
        // set condition to null or true means no startup condition
        client.setCondition(null);
        client.setCondition(() -> true);
        client.start();
        bench.poll(5, TimeUnit.SECONDS);
        Assert.assertEquals(1, welcome.size());
        Assert.assertEquals(WELCOME, welcome.get(0));
        client.close();
    }

}
