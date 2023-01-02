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

package org.platformlambda.core.websocket.server;

import io.vertx.core.Handler;
import io.vertx.core.http.ServerWebSocket;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * Websocket request handler
 */
public class WsRequestHandler implements Handler<ServerWebSocket> {
    private static final Logger log = LoggerFactory.getLogger(WsRequestHandler.class);

    private static final String HOUSEKEEPER = "system.ws.server.cleanup";
    private static final String IN = ".in";
    private static final String OUT = ".out";

    private static final CryptoApi crypto = new CryptoApi();

    private static final ConcurrentMap<String, WsEnvelope> connections = new ConcurrentHashMap<>();
    private static final AtomicInteger counter = new AtomicInteger(0);

    private final ConcurrentMap<String, LambdaFunction> lambdas;
    private final List<String> urls = new ArrayList<>();

    public WsRequestHandler(ConcurrentMap<String, LambdaFunction> lambdas) {
        this.lambdas = lambdas;
        urls.addAll(lambdas.keySet());
        if (urls.size() > 1) {
            Collections.sort(urls, Comparator.reverseOrder());
        }
        IdleCheck idleChecker = new IdleCheck();
        idleChecker.start();
        Platform platform = Platform.getInstance();
        try {
            platform.registerPrivate(HOUSEKEEPER, new WsHousekeeper(), 1);
        } catch (IOException e) {
            log.error("Unable to register {} - {}", HOUSEKEEPER, e.getMessage());
        }
    }

    @Override
    public void handle(ServerWebSocket ws) {
        String uri = ws.path().trim();
        String path = findPath(uri);
        if (path == null) {
            ws.reject();
        } else {
            ws.accept();
            final Platform platform = Platform.getInstance();
            final PostOffice po = PostOffice.getInstance();
            final String ip = ws.remoteAddress().hostAddress();
            final String token = uri.substring(uri.lastIndexOf('/')+1);
            final String query = ws.query();
            // generate a 6-digit random number as identifier
            final int r = crypto.nextInt(100000, 1000000);
            // ensure uniqueness using a monotonically increasing sequence number
            final int n = counter.incrementAndGet();
            final String session = "ws."+r+"."+n;
            final String rxPath = session+IN;
            final String txPath = session+OUT;
            WsEnvelope md = new WsEnvelope(ws, path, rxPath, txPath);
            connections.put(session, md);
            log.info("Session {} connected", session);
            try {
                platform.registerPrivate(rxPath, lambdas.get(path), 1);
                platform.registerPrivate(txPath, new WsServerTransmitter(ws), 1);
            } catch (IOException e) {
                log.error("Unable to register websocket session", e);
            }
            try {
                po.send(rxPath, new Kv(WsEnvelope.TYPE, WsEnvelope.OPEN),
                        new Kv(WsEnvelope.ROUTE, rxPath), new Kv(WsEnvelope.TX_PATH, txPath),
                        new Kv(WsEnvelope.IP, ip), new Kv(WsEnvelope.PATH, path),
                        new Kv(WsEnvelope.QUERY, query),
                        new Kv(WsEnvelope.TOKEN, token));
            } catch (IOException e) {
                log.error("Unable to send 'open' signal to {} - {}", rxPath, e.getMessage());
            }
            ws.binaryMessageHandler(b -> {
                md.touch();
                try {
                    po.send(rxPath, b.getBytes(), new Kv(WsEnvelope.TYPE, WsEnvelope.BYTES),
                            new Kv(WsEnvelope.ROUTE, rxPath), new Kv(WsEnvelope.TX_PATH, txPath));
                } catch (IOException e) {
                    log.error("Unable to send binary message to {} - {}", rxPath, e.getMessage());
                }
            });
            ws.textMessageHandler(text -> {
                md.touch();
                try {
                    po.send(rxPath, text, new Kv(WsEnvelope.TYPE, WsEnvelope.STRING),
                            new Kv(WsEnvelope.ROUTE, rxPath), new Kv(WsEnvelope.TX_PATH, txPath));
                } catch (IOException e) {
                    log.error("Unable to send text message to {} - {}", rxPath, e.getMessage());
                }
            });
            ws.closeHandler(close -> {
                connections.remove(session);
                String reason = ws.closeReason() == null? "ok" : ws.closeReason();
                log.info("Session {} closed ({}, {})", session, ws.closeStatusCode(), reason);
                try {
                    EventEnvelope closing = new EventEnvelope().setTo(rxPath);
                    closing.setHeader(WsEnvelope.ROUTE, rxPath)
                            .setHeader(WsEnvelope.TOKEN, token)
                            .setHeader(WsEnvelope.CLOSE_CODE, ws.closeStatusCode())
                            .setHeader(WsEnvelope.CLOSE_REASON, reason)
                            .setHeader(WsEnvelope.TYPE, WsEnvelope.CLOSE)
                            .setReplyTo(HOUSEKEEPER).setCorrelationId(session);
                    po.send(closing);
                } catch (IOException e) {
                    log.error("Unable to send 'close' signal to {} - {}", rxPath, e.getMessage());
                }
            });
            ws.exceptionHandler(e -> {
                log.warn("Session {} exception - {}", session, e.getMessage());
                if (!ws.isClosed()) {
                    ws.close();
                }
            });
        }

    }

    private String findPath(String path) {
        for (String u: urls) {
            String prefix = u + "/";
            if (path.startsWith(prefix) && !path.equals(prefix)) {
                return u;
            }
        }
        return null;
    }

    private static class IdleCheck extends Thread {
        private boolean normal = true;

        public IdleCheck() {
            Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
            log.info("Started");
        }

        @Override
        public void run() {
            final AppConfigReader config = AppConfigReader.getInstance();
            final Utility util = Utility.getInstance();
            final long timeout = util.str2long(config.getProperty("websocket.idle.timeout", "60"));
            log.info("Websocket server idle timeout = {} seconds", timeout);
            final long expiry = Math.min(30000, timeout * 1000);
            long t1 = System.currentTimeMillis();
            while (normal) {
                long now = System.currentTimeMillis();
                if (now - t1 > 5000) {
                    t1 = now;
                    List<String> sessions = new ArrayList<>();
                    for (String conn : connections.keySet()) {
                        WsEnvelope md = connections.get(conn);
                        if (now - md.getLastAccess() > expiry) {
                            sessions.add(conn);
                        }
                    }
                    for (String conn : sessions) {
                        WsEnvelope md = connections.get(conn);
                        if (md != null) {
                            connections.remove(conn);
                            log.warn("Websocket {} expired ({}, {})", md.getPath(), md.getRxPath(), md.getTxPath());
                            // drop connection due to inactivity
                            if (!md.getWebSocket().isClosed()) {
                                md.getWebSocket().close((short) 1003, "Idle for " + timeout + " seconds");
                            }
                        }
                    }
                }
                try {
                    Thread.sleep(1000);
                } catch (InterruptedException e) {
                    // ok to ignore
                }
            }
        }

        public void shutdown() {
            normal = false;
            log.info("Stopped");
        }
    }

    @EventInterceptor
    private static class WsHousekeeper implements LambdaFunction {

        @Override
        public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
            if (body instanceof EventEnvelope) {
                EventEnvelope event = (EventEnvelope) body;
                String session = event.getCorrelationId();
                if (session != null) {
                    String rxPath = session + IN;
                    String txPath = session + OUT;
                    silentRelease(rxPath);
                    silentRelease(txPath);
                }
            }
            return null;
        }

        private void silentRelease(String route) {
            try {
                Platform.getInstance().release(route);
            } catch (IOException e) {
                log.error("Unable to release {} - {}", route, e.getMessage());
            }
        }
    }

}
