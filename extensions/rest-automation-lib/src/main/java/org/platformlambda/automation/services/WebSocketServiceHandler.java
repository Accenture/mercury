/*

    Copyright 2018-2021 Accenture Technology

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

package org.platformlambda.automation.services;

import io.vertx.core.Handler;
import io.vertx.core.http.ServerWebSocket;
import org.platformlambda.automation.MainModule;
import org.platformlambda.automation.config.WsEntry;
import org.platformlambda.automation.models.WsInfo;
import org.platformlambda.automation.models.WsMetadata;
import org.platformlambda.automation.util.SimpleHttpUtility;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;

public class WebSocketServiceHandler implements Handler<ServerWebSocket> {
    private static final Logger log = LoggerFactory.getLogger(WebSocketServiceHandler.class);

    private static final AtomicInteger counter = new AtomicInteger(0);
    private static final CryptoApi crypto = new CryptoApi();
    private static final String TYPE = WsEnvelope.TYPE;
    private static final String OPEN = WsEnvelope.OPEN;
    private static final String CLOSE = WsEnvelope.CLOSE;
    private static final String IP = WsEnvelope.IP;
    private static final String ORIGIN = "origin";
    private static final String TX_PATH = WsEnvelope.TX_PATH;
    private static final String QUERY = WsEnvelope.QUERY;
    private static final String MESSAGE = "message";
    private static final String TOKEN = WsEnvelope.TOKEN;
    private static final String APPLICATION = "application";
    private static final String CLEAR = "clear";
    private static final String HELLO = "hello";
    private static final String TIME = "time";
    private static final String TOPIC = "topic";
    private static final String SUBSCRIBE = "subscribe";
    private static final String UNSUBSCRIBE = "unsubscribe";
    private static final String PUBLISH = "publish";
    private static final String WS_API = "/ws/api/";
    // txPath -> info
    private static final ConcurrentMap<String, WsMetadata> connections = new ConcurrentHashMap<>();
    private static IdleCheck idleChecker;

    public WebSocketServiceHandler() {
        if (idleChecker == null) {
            idleChecker = new IdleCheck();
            idleChecker.start();
        }
    }

    @Override
    public void handle(ServerWebSocket ws) {
        String path = ws.path();
        if (!path.startsWith(WS_API)) {
            ws.reject();
            return;
        }
        final AtomicBoolean accepted = new AtomicBoolean(false);
        final Utility util = Utility.getInstance();
        final SimpleHttpUtility httpUtil = SimpleHttpUtility.getInstance();
        final Platform platform = Platform.getInstance();
        final PostOffice po = PostOffice.getInstance();
        final String origin = platform.getOrigin();
        if (!NotificationManager.isReady()) {
            log.warn("Service temporarily unavailable - notification manager not ready");
            ws.reject();
            return;
        }
        WsEntry wsEntry = WsEntry.getInstance();
        if (wsEntry.isEmpty()) {
            log.warn("Service temporarily unavailable - WebSocket service not enabled");
            ws.reject();
            return;
        }
        String ip = ws.remoteAddress().hostAddress();
        String name = path.substring(WS_API.length());
        if (!name.contains(":")) {
            ws.reject();
            return;
        }
        int colon = name.lastIndexOf(':');
        String app = name.substring(0, colon);
        String token = name.substring(colon+1);
        WsInfo info = wsEntry.getInfo(app);
        if (info == null) {
            ws.accept();
            ws.close((short) 1003, "Invalid application - "+app);
            return;
        }
        String permitted = NotificationManager.getApplication(token);
        if (!app.equals(permitted)) {
            ws.accept();
            ws.close((short) 1003, "Invalid access token");
            return;
        }
        ws.accept();
        accepted.set(true);
        LambdaFunction f = (headers, body, instance) -> {
            if (ws.isClosed()) {
                return false;
            }
            if (body instanceof String) {
                ws.writeTextMessage((String) body);
            }
            if (body instanceof byte[]) {
                ws.writeTextMessage(util.getUTF((byte[]) body));
            }
            if (body instanceof Map) {
                String json = SimpleMapper.getInstance().getMapper().writeValueAsString(body);
                ws.writeTextMessage(json);
            }
            return true;
        };
        // generate a 5-digit random number as identifier
        int r = crypto.nextInt(10000, 100000000);
        // ensure uniqueness using a monotonically increasing sequence number
        int n = counter.incrementAndGet();
        final String txPath = "ws."+r+"."+n;
        try {
            platform.registerPrivate(txPath, f, 1);
        } catch (IOException e) {
            log.error("Unable to register {} - {}", txPath, e.getMessage());
            ws.close((short) 1003, "System temporarily unavailable");
            return;
        }
        log.info("Session {} started, ip={}, app={}", txPath, ip, app);
        WsMetadata md = new WsMetadata(ws, info.application, info.recipient, txPath, info.publish, info.subscribe);
        connections.put(txPath, md);
        try {
            po.broadcast(MainModule.NOTIFICATION_MANAGER, new Kv(TYPE, CLEAR), new Kv(TOKEN, token));
        } catch (IOException e) {
            log.error("Unable to inform {} when starting {} - {}",
                    MainModule.NOTIFICATION_MANAGER, txPath, e.getMessage());
        }
        if (!WsEntry.NONE_PROVIDED.equals(info.recipient)) {
            Map<String, Object> event = new HashMap<>();
            event.put(QUERY, httpUtil.decodeQueryString(ws.query()));
            event.put(APPLICATION, info.application);
            event.put(IP, ip);
            event.put(ORIGIN, origin);
            event.put(TX_PATH, txPath + "@" + origin);
            // broadcast to multiple instance of the recipient service
            try {
                po.broadcast(new EventEnvelope().setTo(info.recipient).setBody(event).setHeader(TYPE, OPEN));
            } catch (IOException e) {
                log.error("Unable to send open event to {} - {}", info.recipient, e.getMessage());
            }
        }
        ws.handler(block -> {
                if (accepted.get()) {
                    String message = util.getUTF(block.getBytes());
                    try {
                        handleMessage(txPath, message);
                    } catch (IOException e) {
                        log.warn("Exception happen when processing incoming message for {} - {}",
                                txPath, e.getMessage());
                    }
                }
            })
            .endHandler(end -> log.info("Session {} closed {}", txPath, ws.closeStatusCode()))
            .closeHandler(close -> {
                try {
                    handleClose(txPath);
                } catch (IOException e) {
                    log.error("Unable to inform {} when closing {} - {}",
                            MainModule.NOTIFICATION_MANAGER, txPath, e.getMessage());
                }
                connections.remove(txPath);
                try {
                    platform.release(txPath);
                } catch (IOException e) {
                    log.error("Unable to release {} - {}", txPath, e.getMessage());
                }
            });
    }

    public static void closeAllConnections() {
        List<ServerWebSocket> sockets = new ArrayList<>();
        for (String txPath: connections.keySet()) {
            WsMetadata md = connections.get(txPath);
            sockets.add(md.ws);
        }
        for (ServerWebSocket w: sockets) {
            w.close();
        }
    }

    @SuppressWarnings("unchecked")
    private void handleMessage(String txPath, String body) throws IOException {
        String origin = Platform.getInstance().getOrigin();
        PostOffice po = PostOffice.getInstance();
        WsMetadata md = connections.get(txPath);
        if (md != null) {
            md.touch();
            if (!WsEntry.NONE_PROVIDED.equals(md.recipient)) {
                try {
                    po.send(md.recipient, body, new Kv(TYPE, MESSAGE), new Kv(APPLICATION, md.application),
                            new Kv(ORIGIN, origin), new Kv(TX_PATH, md.txPath + "@" + origin));
                } catch (IOException e) {
                    log.warn("Message not delivered to {} - {}", md.recipient, e.getMessage());
                }
            }
            if (body.startsWith("{") && body.endsWith("}")) {
                Map<String, Object> message = SimpleMapper.getInstance().getMapper().readValue(body, Map.class);
                if (HELLO.equals(message.get(TYPE))) {
                    po.send(txPath, body);
                }
                if (SUBSCRIBE.equals(message.get(TYPE)) || UNSUBSCRIBE.equals(message.get(TYPE))) {
                    if (!md.subscribe) {
                        sendResponse(txPath, "error", "Subscribe feature not enabled for this connection");
                    } else {
                        if (message.containsKey(TOPIC)) {
                            String topic = message.get(TOPIC).toString();
                            subscribeTopic(message.get(TYPE).toString(), txPath, topic);
                        } else {
                            sendResponse(txPath, "error", "Missing topic");
                        }
                    }
                }
                if (PUBLISH.equals(message.get(TYPE))) {
                    if (!md.publish) {
                        sendResponse(txPath, "error", "Publish feature not enabled for this connection");
                    } else {
                        if (message.containsKey(TOPIC) && message.containsKey(MESSAGE)) {
                            publishTopic(txPath, message.get(TOPIC).toString(), message.get(MESSAGE).toString());
                        } else {
                            sendResponse(txPath, "error", "Input format should be topic:message");
                        }
                    }
                }
            }
        }
    }

    private void handleClose(String txPath) throws IOException {
        String origin = Platform.getInstance().getOrigin();
        PostOffice po = PostOffice.getInstance();
        log.info("Stopping {}", txPath);
        WsMetadata md = connections.get(txPath);
        if (md != null) {
            String target = md.txPath + "@" + origin;
            // tell notification manager to clear routing entries
            po.broadcast(MainModule.NOTIFICATION_MANAGER, new Kv(TYPE, CLOSE), new Kv(TX_PATH, target));
            if (!WsEntry.NONE_PROVIDED.equals(md.recipient)) {
                Map<String, Object> event = new HashMap<>();
                event.put(APPLICATION, md.application);
                event.put(ORIGIN, origin);
                event.put(TX_PATH, target);
                // broadcast to multiple instance of the recipient service
                po.broadcast(new EventEnvelope().setTo(md.recipient).setBody(event).setHeader(TYPE, CLOSE));
            }
        }
    }

    private void publishTopic(String txPath, String topic, String message) throws IOException {
        PostOffice po = PostOffice.getInstance();
        Utility util = Utility.getInstance();
        if (util.validServiceName(topic)) {
            po.send(MainModule.NOTIFICATION_MANAGER, message, new Kv(TYPE, PUBLISH), new Kv(TOPIC, topic));
            sendResponse(txPath, "publish", "sending message to "+topic);

        } else {
            sendResponse(txPath, "error", "Invalid topic");
        }
    }

    private void subscribeTopic(String type, String txPath, String topic) throws IOException {
        String origin = Platform.getInstance().getOrigin();
        PostOffice po = PostOffice.getInstance();
        Utility util = Utility.getInstance();
        if (util.validServiceName(topic)) {
            po.broadcast(MainModule.NOTIFICATION_MANAGER,
                    new Kv(TYPE, type), new Kv(ORIGIN, origin),
                    new Kv(TOPIC, topic), new Kv(TX_PATH, txPath));
            sendResponse(txPath, type, "topic "+topic);

        } else {
            sendResponse(txPath, "error", "Invalid topic");
        }
    }

    private void sendResponse(String txPath, String type, String message) {
        Map<String, Object> response = new HashMap<>();
        response.put(TYPE, type);
        response.put(MESSAGE, message);
        response.put(TIME, new Date());
        try {
            PostOffice.getInstance().send(txPath, response);
        } catch (IOException e) {
            // ok to ignore
        }
    }

    private static class IdleCheck extends Thread {
        private boolean normal = true;

        public IdleCheck() {
            Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
            log.info("Started");
        }

        @Override
        public void run() {
            final long EXPIRY = 60000;
            long t1 = System.currentTimeMillis();
            while (normal) {
                long now = System.currentTimeMillis();
                if (now - t1 > 5000) {
                    t1 = now;
                    List<ServerWebSocket> sockets = new ArrayList<>();
                    for (String txPath : connections.keySet()) {
                        WsMetadata md = connections.get(txPath);
                        if (now - md.lastAccess > EXPIRY) {
                            sockets.add(md.ws);
                            log.warn("{} expired", md.txPath);
                        }
                    }
                    for (ServerWebSocket w : sockets) {
                        w.close((short) 1003, "Idle for "+(EXPIRY / 1000)+" seconds");
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
}
