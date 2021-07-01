package org.platformlambda.core.util;

import com.google.api.client.http.*;
import com.google.api.client.http.javanet.NetHttpTransport;
import io.vertx.core.Vertx;
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
import java.util.Collections;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class WebSocketTest {
    private static final Logger log = LoggerFactory.getLogger(WebSocketTest.class);

    private static final HttpTransport HTTP_TRANSPORT = new NetHttpTransport();
    private static final HttpRequestFactory factory = HTTP_TRANSPORT.createRequestFactory();
    private static final String HELLO_WORLD = "hello world";

    @BeforeClass
    public static void setup() {
        Vertx vertx = Vertx.vertx();
        vertx.createHttpServer()
                .webSocketHandler(ws -> {
                    ws.accept();
                    ws.handler(data -> {
                        byte[] b = data.getBytes();
                        ws.writeTextMessage(Utility.getInstance().getUTF(b));
                    });
                    ws.closeHandler(end -> log.info("socket closed"));
                })
                .requestHandler(request -> request.response().end(HELLO_WORLD))
                .listen(8085)
                .onSuccess(server -> log.info("Listening to port {}", server.actualPort()))
                .onFailure(ex -> {
                    log.error("Unable to start - {}", ex.getMessage());
                    System.exit(-1);
                });

    }

    @Test
    public void httpTest() throws IOException {
        GenericUrl target = new GenericUrl("http://127.0.0.1:8085");
        HttpRequest request = factory.buildGetRequest(target);
        HttpResponse response = request.execute();
        int rc = response.getStatusCode();
        String statusMessage = response.getStatusMessage();
        InputStream in = response.getContent();
        String result = Utility.getInstance().stream2str(in);
        System.out.println(result);
        Assert.assertEquals(200, rc);
        Assert.assertEquals("OK", statusMessage);
        Assert.assertEquals(HELLO_WORLD, result);
    }

    @Test
    public void wsTest() {
        final BlockingQueue<Boolean> bench = new ArrayBlockingQueue<>(1);
        String MESSAGE = "hello world";
        PostOffice po = PostOffice.getInstance();
        LambdaFunction connector = (headers, body, instance) -> {
            log.info("{}", headers);
            if ("open".equals(headers.get("type"))) {
                String txPath = headers.get("tx_path");
                Assert.assertNotNull(txPath);
                po.send(txPath, MESSAGE);
            }
            if ("string".equals(headers.get("type"))) {
                String text = (String) body;
                if (text.startsWith("{") && text.contains("keep-alive")) {
                    bench.offer(true);
                } else {
                    Assert.assertEquals(MESSAGE, text);
                }
            }
            return true;
        };
        boolean ready = Utility.getInstance().portReady("127.0.0.1", 8085, 5000);
        Assert.assertTrue(ready);
        PersistentWsClient client = new PersistentWsClient(connector,
                Collections.singletonList("ws://127.0.0.1:8085/ws/test/hi"));
        // set condition to null or true means no startup condition
        client.setCondition(null);
        client.setCondition(() -> true);
        client.start();

        try {
            bench.poll(5000, TimeUnit.MILLISECONDS);
            client.close();
        } catch (InterruptedException e) {
            // ok to ignore
        }

    }

}
