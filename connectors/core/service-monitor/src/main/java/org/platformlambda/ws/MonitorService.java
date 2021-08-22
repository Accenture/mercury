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

package org.platformlambda.ws;

import io.vertx.core.Handler;
import io.vertx.core.buffer.Buffer;
import io.vertx.core.http.ServerWebSocket;
import org.platformlambda.MainApp;
import org.platformlambda.cloud.services.ServiceRegistry;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;
import org.platformlambda.models.PendingConnection;
import org.platformlambda.models.WsMetadata;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicInteger;

public class MonitorService implements Handler<ServerWebSocket> {
    private static final Logger log = LoggerFactory.getLogger(MonitorService.class);

    private static final CryptoApi crypto = new CryptoApi();
    private static final AtomicInteger counter = new AtomicInteger(0);
    private static final String WS_PRESENCE = "/ws/presence/";
    private static final String MONITOR_PARTITION = MainApp.MONITOR_PARTITION;
    private static final String CLOUD_MANAGER = MainApp.CLOUD_MANAGER;
    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;
    private static final String APP_GROUP = ServiceRegistry.APP_GROUP;
    private static final String PUT = "put";
    private static final String ALIVE = "keep-alive";
    private static final String INFO = "info";
    private static final String TYPE = "type";
    private static final String NAME = "name";
    private static final String LEAVE = "leave";
    private static final String DELETE = "del";
    private static final String CLOSE = "close";
    private static final String CODE = "code";
    private static final String REASON = "reason";
    private static final String ORIGIN = "origin";
    private static final String ID = "id";
    private static final String SEQ = "seq";
    private static final String GROUP = "group";
    private static final String CREATED = "created";
    private static final String UPDATED = "updated";
    private static final String MONITOR = "monitor";
    private static final String TOPIC = "topic";
    private static final String GET_TOPIC = "get_topic";
    private static final String TX_PATH = "tx_path";
    private static final String STARTING_UP = "Starting up";
    private static final long ONE_SECOND = 1000;
    private static final long EXPIRY = 60 * ONE_SECOND;
    private static final long GRACE_PERIOD = 40 * ONE_SECOND;
    private static final long STALLED_CONNECTION = 20 * ONE_SECOND;
    // session -> info
    private static final ConcurrentMap<String, WsMetadata> connections = new ConcurrentHashMap<>();
    // pending connections of sessions to monitor if topic is assigned within a reasonable time
    private static final ConcurrentMap<String, PendingConnection> pendingConnections = new ConcurrentHashMap<>();
    // connection list of user applications to this presence monitor instance
    private static final ConcurrentMap<String, Map<String, Object>> myConnections = new ConcurrentHashMap<>();
    // user application connections for the whole system
    private static final ManagedCache connectionInfo = ManagedCache.createCache("app.presence.list", EXPIRY);
    private static boolean ready = false;
    private static IdleCheck idleChecker;

    public MonitorService() {
        if (idleChecker == null) {
            idleChecker = new IdleCheck();
            idleChecker.start();
        }
    }

    public static void setReady() {
        ready = true;
    }

    public static int getSessionCount() {
        return connections.size();
    }

    public static Map<String, WsMetadata> getSessions() {
        return connections;
    }

    public static Map<String, Object> getConnections() {
        return new HashMap<>(connectionInfo.getMap());
    }

    public static void clearStalledConnection() {
        List<String> pendingList = new ArrayList<>(pendingConnections.keySet());
        long now = System.currentTimeMillis();
        for (String route: pendingList) {
            PendingConnection conn = pendingConnections.get(route);
            if (PendingConnection.PendingType.CONNECTED == conn.type) {
                if (now - conn.created > STALLED_CONNECTION) {
                    closeStalledConnection(conn.session, "handshake not completed");
                }
            }
            if (PendingConnection.PendingType.HANDSHAKE == conn.type) {
                Map<String, Object> metadata = myConnections.getOrDefault(conn.origin, new HashMap<>());
                if (metadata.containsKey(TOPIC)) {
                    pendingConnections.remove(route);
                    log.info("Connection verified {} -> {}", route, metadata.get(TOPIC));
                } else {
                    if (now - conn.created > GRACE_PERIOD) {
                        closeStalledConnection(conn.session, "topic not assigned");
                    }
                }
            }
        }
    }

    private static void closeStalledConnection(String session, String reason) {
        PostOffice po = PostOffice.getInstance();
        try {
            log.info("Closing connection {} because {}", session, reason);
            pendingConnections.remove(session);
            po.send(session, new Kv(TYPE, CLOSE), new Kv(CODE, 1003), new Kv(REASON, reason));
        } catch (IOException e) {
            // ok to ignore
        }
    }

    public static void updateNodeInfo(String origin, Map<String, Object> info) {
        if (connectionInfo.exists(origin)) {
            Object o = connectionInfo.get(origin);
            if (o instanceof Map) {
                if (!info.equals(o)) {
                    connectionInfo.put(origin, info);
                }
            }
        } else {
            connectionInfo.put(origin, info);
            log.info("Member {} joins", origin);
        }
    }

    public static void deleteNodeInfo(String origin) {
        if (connectionInfo.exists(origin)) {
            connectionInfo.remove(origin);
            log.info("Member {} left", origin);
        }
    }

    @SuppressWarnings("unchecked")
    public static Object getInfo(String origin, String key) {
        if (connectionInfo.exists(origin)) {
            Object data = connectionInfo.get(origin);
            if (data instanceof Map) {
                Map<String, Object> metadata = (Map<String, Object>) data;
                return metadata.get(key);
            }
        }
        return null;
    }

    @Override
    public void handle(ServerWebSocket ws) {
        Platform platform = Platform.getInstance();
        Utility util = Utility.getInstance();
        // generate a random number as identifier
        int r = crypto.nextInt(10000, 100000000);
        // ensure uniqueness using a monotonically increasing sequence number
        int n = counter.incrementAndGet();
        String session = "ws."+r+"."+n;
        String ip = ws.remoteAddress().hostAddress();
        String path = ws.path();
        log.info("Session {} started, {}, {}", session, ip, path);
        List<String> segments = util.split(path, "/");
        if (!path.startsWith(WS_PRESENCE) || segments.size() < 3) {
            ws.reject();
            log.warn("Session {} rejected, {}, {} - invalid path", session, ip, path);
            return;
        }
        ws.accept();
        String app = segments.get(2);
        LambdaFunction f = (headers, body, instance) -> {
            if (ws.isClosed()) {
                return false;
            }
            if (CLOSE.equals(headers.get(TYPE)) && headers.containsKey(CODE) && headers.containsKey(REASON)) {
                ws.close((short) util.str2int(headers.get(CODE)), headers.get(REASON));
                return true;
            }
            if (body instanceof String) {
                ws.writeTextMessage((String) body);
            }
            if (body instanceof byte[]) {
                ws.writeBinaryMessage(Buffer.buffer((byte[]) body));
            }
            if (body instanceof Map) {
                String json = SimpleMapper.getInstance().getMapper().writeValueAsString(body);
                ws.writeTextMessage(json);
            }
            return true;
        };
        try {
            platform.registerPrivate(session, f, 1);
        } catch (IOException e) {
            ws.close((short) 1013, "System temporarily unavailable");
            log.error("Unable to accept connection - {}", e.getMessage());
            return;
        }
        if (ready && platform.hasRoute(CLOUD_CONNECTOR) &&
                platform.hasRoute(CLOUD_MANAGER) && platform.hasRoute(MainApp.PRESENCE_HANDLER)) {
            Map<String, Object> info = new HashMap<>();
            String time = util.date2str(new Date(), true);
            info.put(CREATED, time);
            info.put(UPDATED, time);
            info.put(MONITOR, platform.getOrigin());
            info.put(ID, session);
            info.put(SEQ, 0);
            myConnections.put(app, info);
            pendingConnections.put(session, new PendingConnection(app, session));
        } else {
            ws.close((short) 1013, STARTING_UP);
            log.info("Session {} closed - {}", session, STARTING_UP);
            return;
        }
        WsMetadata md = new WsMetadata(ws, app, session);
        connections.put(session, md);
        ws.handler(block -> {
                try {
                    handleMessage(ws, session, block.getBytes());
                } catch (IOException e) {
                    log.warn("Exception happen when processing incoming message for {} - {}", session, e.getMessage());
                }
            })
            .endHandler(end -> log.debug("Session {} completed - {}", session, ws.closeStatusCode()))
            .closeHandler(close -> {
                try {
                    handleClose(session);
                } catch (IOException e) {
                    log.warn("Exception happen when closing {} - {}", session, e.getMessage());
                }
                connections.remove(session);
                try {
                    platform.release(session);
                } catch (IOException e) {
                    log.error("Unable to release {} - {}", session, e.getMessage());
                }
            }).exceptionHandler(e -> {
                if (!ws.isClosed()) {
                    ws.close();
                }
                log.error("Unhandled exception - {}", e.getMessage());
            });
    }

    private void handleMessage(ServerWebSocket ws, String session, byte[] payload) throws IOException {
        // input message must be a JSON string or EventEnvelope in bytes
        WsMetadata md = connections.get(session);
        if (md != null) {
            md.touch();
            Utility util = Utility.getInstance();
            if (payload.length > 3 && payload[0] == '{' && payload[payload.length - 1] == '}') {
                log.debug("text message {}", util.getUTF(payload));
            } else {
                PostOffice po = PostOffice.getInstance();
                try {
                    EventEnvelope command = new EventEnvelope(payload);
                    boolean register = INFO.equals(command.getTo());
                    boolean alive = ALIVE.equals(command.getTo());
                    if (myConnections.containsKey(md.origin)) {
                        Map<String, Object> info = myConnections.get(md.origin);
                        if (register || alive) {
                            updateInfo(info, command.getHeaders());
                            // broadcast to all presence monitors
                            po.send(MainApp.PRESENCE_HANDLER + MONITOR_PARTITION, info,
                                    new Kv(TYPE, PUT), new Kv(ORIGIN, md.origin));
                            if (register) {
                                // tell the connected application instance to proceed
                                PendingConnection pc = pendingConnections.get(session);
                                if (pc != null) {
                                    pendingConnections.put(session, pc.setType(PendingConnection.PendingType.HANDSHAKE));
                                    log.info("Member registered {} {}", md.origin, info.get(NAME));
                                    po.send(MainApp.TOPIC_CONTROLLER, new Kv(TYPE, GET_TOPIC),
                                            new Kv(TX_PATH, session), new Kv(ORIGIN, md.origin));
                                }
                            } else {
                                log.debug("Member {} is alive {}", md.origin, info.get(SEQ));
                            }
                        }
                    }

                } catch (Exception e) {
                    ws.close((short) 1003, "Invalid protocol");
                }
            }
        }
    }

    @SuppressWarnings("unchecked")
    private void handleClose(String session) throws IOException {
        WsMetadata md = connections.get(session);
        connections.remove(session);
        if (md != null) {
            myConnections.remove(md.origin);
            pendingConnections.remove(session);
            log.info("Session {} closed, {}", session, md.origin);
            if (connectionInfo.exists(md.origin)) {
                Object o = connectionInfo.get(md.origin);
                if (o instanceof Map) {
                    Map<String, Object> info = (Map<String, Object>) o;
                    if (session.equals(info.get(ID)) && info.get(GROUP) instanceof Integer) {
                        // broadcast to presence monitors
                        PostOffice.getInstance().send(MainApp.PRESENCE_HANDLER + MONITOR_PARTITION,
                                                        new Kv(TYPE, DELETE), new Kv(ORIGIN, md.origin));
                        // tell all nodes to drop this node
                        leaveGroup(md.origin, (int) info.get(GROUP));
                    }
                }
            }
        } else {
            log.info("Session {} closed", session);
        }
    }

    private void updateInfo(Map<String, Object> info, Map<String, String> headers) {
        Utility util = Utility.getInstance();
        for (String key : headers.keySet()) {
            if (!key.equals(ID) && !key.equals(MONITOR)) {
                // normalize numbers
                String value = headers.get(key);
                if (util.isNumeric(value)) {
                    long v = util.str2long(value);
                    info.put(key, v >= Integer.MAX_VALUE? v : (int) v);
                } else {
                    info.put(key, headers.get(key));
                }
            }
        }
        // save timestamp without milliseconds
        info.put(UPDATED, util.date2str(new Date(), true));
    }

    private void leaveGroup(String closedApp, int groupId) {
        try {
            if (groupId < 1) {
                throw new IllegalArgumentException("Invalid closed user group ("+groupId+")");
            }
            // send leave event to the closed user group
            PostOffice.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY + APP_GROUP + groupId,
                    new Kv(TYPE, LEAVE), new Kv(ORIGIN, closedApp));
            log.info("tell group {} that {} has left", groupId, closedApp);

        } catch (Exception e) {
            log.error("Unable to send leave event to group {} - {}", closedApp, e.getMessage());
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
                    List<String> sessions = new ArrayList<>();
                    for (String conn : connections.keySet()) {
                        WsMetadata md = connections.get(conn);
                        if (now - md.lastAccess > EXPIRY) {
                            sessions.add(conn);
                        }
                    }
                    for (String conn : sessions) {
                        WsMetadata md = connections.get(conn);
                        if (md != null) {
                            connections.remove(conn);
                            log.warn("{} expired", md.session);
                            if (!md.ws.isClosed()) {
                                md.ws.close((short) 1003, "Idle for " + (EXPIRY / 1000) + " seconds");
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
}
