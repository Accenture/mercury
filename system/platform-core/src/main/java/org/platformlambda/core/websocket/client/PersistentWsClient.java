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

package org.platformlambda.core.websocket.client;

import io.vertx.core.Future;
import io.vertx.core.Vertx;
import io.vertx.core.http.HttpClient;
import io.vertx.core.http.WebSocket;
import io.vertx.core.http.WebSocketConnectOptions;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.net.MalformedURLException;
import java.net.URL;
import java.util.*;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;

public class PersistentWsClient extends Thread {
    private static final Logger log = LoggerFactory.getLogger(PersistentWsClient.class);

    private static final String TYPE = "type";
    private static final String OPEN = "open";
    private static final String CLOSE = "close";
    private static final String STRING = "string";
    private static final String BYTES = "bytes";
    private static final String TIME = "time";

    private static final String MAP = "map";
    private static final String ROUTE = "route";
    private static final String TX_PATH = "tx_path";
    public static final String IP = "ip";
    public static final String PATH = "path";
    private static final String ALIVE_SEQ = "seq";
    private static final String PC = "pc.";
    private static final String IN = ".in";
    private static final String OUT = ".out";

    private final AtomicLong seq = new AtomicLong(1);
    private final AtomicLong aliveSeq = new AtomicLong(0);

    private static final String IDLE_TIMEOUT = "websocket.idle.timeout";
    private static final String DEFAULT_IDLE_TIMEOUT = "60";
    private static final String ALIVE = "keep-alive";
    private static final long RETRY_TIMER = 5000;
    private final List<String> urls = new ArrayList<>();
    private final List<String> targets = new ArrayList<>();
    private final String sessionId;
    private final String session;
    private final AtomicBoolean connected = new AtomicBoolean(false);
    private boolean running = true;

    private Vertx vertx = null;
    private HttpClient client = null;

    public PersistentWsClient(LambdaFunction connector, List<String> urls) {
        if (urls.isEmpty()) {
            throw new IllegalArgumentException("Missing target URL(s)");
        }
        this.urls.addAll(urls);
        Platform platform = Platform.getInstance();
        sessionId = Utility.getInstance().getUuid().substring(0, 8);
        session = PC+sessionId+IN;
        try {
            platform.registerPrivate(session, connector, 1);
        } catch (IOException e) {
            log.error("Unable to setup service {} - {}", session, e.getMessage());
        }
    }

    @Override
    public void run() {
        vertx = Vertx.vertx();
        client = vertx.createHttpClient();
        Runtime.getRuntime().addShutdownHook(new Thread(this::close));
        Utility util = Utility.getInstance();
        AppConfigReader reader = AppConfigReader.getInstance();
        int idleSeconds = Math.max(20, util.str2int(reader.getProperty(IDLE_TIMEOUT, DEFAULT_IDLE_TIMEOUT)));
        log.info("Started - {}, idle timeout = {} seconds", urls, idleSeconds);
        makeConnection(idleSeconds);
    }

    private void makeConnection(int idleSeconds) {
        if (!running) {
            return;
        }
        if (urls.isEmpty()) {
            log.error("No valid target URLs are available");
            return;
        }
        if (connected.get()) {
            log.error("Session is still connected");
            return;
        }
        // get next URL using a round-robin
        if (targets.isEmpty()) {
            targets.addAll(urls);
        }
        String u = targets.get(0);
        targets.remove(0);
        final boolean secure;
        final String url;
        if (u.startsWith("ws://")) {
            secure = false;
            url = u.replace("ws://", "http://");
        } else if (u.startsWith("wss://")) {
            secure = true;
            url = u.replace("wss://", "https://");
        } else {
            log.error("Invalid target URL {} - protocol must be ws:// or wss://", u);
            urls.remove(u);
            vertx.setTimer(RETRY_TIMER, n -> makeConnection(idleSeconds));
            return;
        }
        final Platform platform = Platform.getInstance();
        final EventEmitter po = EventEmitter.getInstance();
        final WebSocketConnectOptions options;
        final URL target;
        try {
            int defaultPort = secure? 443 : 80;
            target = new URL(url);
            options = new WebSocketConnectOptions();
            options.setHost(target.getHost());
            options.setPort(target.getPort() > 0? target.getPort() : defaultPort);
            options.setURI(target.getPath() + "/" + platform.getOrigin());
            options.setSsl(secure);
            options.setTimeout(idleSeconds * 1000L);

        } catch (MalformedURLException e) {
            log.error("Invalid target URL {} - {}", u, e.getMessage());
            urls.remove(0);
            vertx.setTimer(RETRY_TIMER, n -> makeConnection(idleSeconds));
            return;
        }
        String txPath = PC+sessionId+"."+seq.get()+OUT;
        Future<WebSocket> connection = client.webSocket(options);
        connection.onSuccess(ws -> {
            seq.incrementAndGet();
            connected.set(true);
            log.info("Session {} connected to {}", session, options.getURI());
            WsClientTransmitter tx = new WsClientTransmitter(ws);
            try {
                platform.registerPrivate(txPath, tx, 1);
            } catch (IOException e1) {
                // this should never happen
                log.error("Unable to setup session {} - {}", session, e1.getMessage());
            }
            try {
                po.send(session, new Kv(TYPE, OPEN), new Kv(IP, target.getHost()+":"+options.getPort()),
                        new Kv(PATH, options.getURI()), new Kv(ROUTE, session), new Kv(TX_PATH, txPath));
            } catch (IOException e) {
                log.error("Unable to send open signal to {} - {}", session, e.getMessage());
            }
            long poll = Math.max(10000, idleSeconds * 1000L / 2 - 3000);
            long keepAlive = vertx.setPeriodic(poll, n -> {
                Map<String, Object> data = new HashMap<>();
                data.put(TYPE, ALIVE);
                data.put(ALIVE_SEQ, aliveSeq.incrementAndGet());
                data.put(TIME, new Date());
                try {
                    // inform primary handler
                    po.send(session, data, new Kv(TYPE, MAP), new Kv(ROUTE, session), new Kv(TX_PATH, txPath));
                    // send keep-alive to the remote websocket server
                    po.send(txPath, data);
                } catch (IOException e) {
                    log.warn("Unable to send keep-alive to {} - {}", session, e.getMessage());
                }
            });
            ws.textMessageHandler(text -> {
                try {
                    po.send(session, text, new Kv(TYPE, STRING),
                            new Kv(ROUTE, session), new Kv(TX_PATH, txPath));
                } catch (IOException e) {
                    log.warn("Unable to send incoming message to {} - {}", session, e.getMessage());
                }
            });
            ws.binaryMessageHandler(b -> {
                try {
                    po.send(session, b.getBytes(), new Kv(TYPE, BYTES),
                            new Kv(ROUTE, session), new Kv(TX_PATH, txPath));
                } catch (IOException e) {
                    log.warn("Unable to send incoming message to {} - {}", session, e.getMessage());
                }
            });
            ws.closeHandler(empty -> {
                connected.set(false);
                tx.close();
                vertx.cancelTimer(keepAlive);
                String reason = ws.closeReason() == null? "ok" : ws.closeReason();
                log.info("Session {} closed ({}, {})", session, ws.closeStatusCode(), reason);
                try {
                    po.send(session, new Kv(TYPE, CLOSE), new Kv(ROUTE, session), new Kv(TX_PATH, txPath));
                } catch (IOException e) {
                    // this happens when PersistentWsClient is closed
                    log.info("Client {} stopped", session);
                }
                platform.release(txPath);
                vertx.setTimer(RETRY_TIMER, n -> makeConnection(idleSeconds));
            });
            ws.exceptionHandler(e -> {
                if (connected.get()) {
                    log.warn("Session {} - {}", session, e.getMessage());
                    connected.set(false);
                    ws.close((short) 1000, e.getMessage());
                }
            });
        });
        connection.onFailure(e -> {
            String error = e.getMessage();
            if (error.toLowerCase().contains("connection refused")) {
                log.warn("Unreachable {}:{}{}", options.getHost(), options.getPort(), options.getURI());
            } else {
                log.warn("Unable to connect - {}", error);
            }
            vertx.setTimer(RETRY_TIMER, n -> makeConnection(idleSeconds));
        });
    }

    public void close() {
        running = false;
        if (client != null) {
            boolean success = Platform.getInstance().release(session);
            if (!success) {
                log.debug("{} already released", session);
            }
            client.close();
            vertx.close();
        }
    }

}
