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

package org.platformlambda.ws;

import org.platformlambda.MainApp;
import org.platformlambda.cloud.services.ServiceRegistry;
import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.server.WsEnvelope;
import org.platformlambda.models.PendingConnection;
import org.platformlambda.models.WsMetadata;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

@WebSocketService("presence")
public class MonitorService implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(MonitorService.class);

    private static final String CLOUD_MANAGER = ServiceRegistry.CLOUD_MANAGER;
    private static final String CLOUD_CONNECTOR = EventEmitter.CLOUD_CONNECTOR;
    private static final String MONITOR_PARTITION = MainApp.MONITOR_PARTITION;
    private static final String APP_GROUP = ServiceRegistry.APP_GROUP;
    private static final String TYPE = "type";
    private static final String ALIVE = "keep-alive";
    private static final String INFO = "info";
    private static final String DELETE = "del";
    private static final String PUT = "put";
    private static final String NAME = "name";
    private static final String TOPIC = "topic";
    private static final String GET_TOPIC = "get_topic";
    private static final String TX_PATH = "tx_path";
    private static final String LEAVE = "leave";
    private static final String ORIGIN = "origin";
    private static final String CREATED = "created";
    private static final String UPDATED = "updated";
    private static final String MONITOR = "monitor";
    private static final String STATUS = "status";
    private static final String MESSAGE = "message";
    private static final String ID = "id";
    private static final String SEQ = "seq";
    private static final String STARTING_UP = "Starting up";
    private static final String TOPIC_ASSIGNED = "Topic already assigned";
    private static final String INVALID_PROTOCOL = "Invalid handshake";
    private static final String GROUP = "group";
    private static final int TRY_AGAIN_LATER = 1013;
    private static final long ONE_SECOND = 1000;
    private static final long EXPIRY = 60 * ONE_SECOND;
    private static final long GRACE_PERIOD = 40 * ONE_SECOND;
    private static final long STALLED_CONNECTION = 20 * ONE_SECOND;
    // pending connections of sessions to monitor if topic is assigned within a reasonable time
    private static final ConcurrentMap<String, PendingConnection> pendingConnections = new ConcurrentHashMap<>();
    // connection list of user applications to this presence monitor instance
    private static final ConcurrentMap<String, Map<String, Object>> myConnections = new ConcurrentHashMap<>();
    // session -> info
    private static final ConcurrentMap<String, WsMetadata> connections = new ConcurrentHashMap<>();
    // user application connections for the whole system
    private static final ManagedCache connectionInfo = ManagedCache.createCache("app.presence.list", EXPIRY);

    private static boolean ready = false;

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

    public static void updateNodeInfo(String origin, Map<String, Object> info) {
        if (connectionInfo.exists(origin)) {
            Object o = connectionInfo.get(origin);
            if (o instanceof Map && !info.equals(o)) {
                connectionInfo.put(origin, info);
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

    public static void clearStalledConnection() {
        List<String> pendingList = new ArrayList<>(pendingConnections.keySet());
        long now = System.currentTimeMillis();
        for (String route: pendingList) {
            WsMetadata md = connections.get(route);
            PendingConnection conn = pendingConnections.get(route);
            if (PendingConnection.PendingType.CONNECTED == conn.type && now - conn.created > STALLED_CONNECTION) {
                closeStalledConnection(md, "handshake not completed");
            }
            if (PendingConnection.PendingType.HANDSHAKE == conn.type) {
                Map<String, Object> metadata = myConnections.getOrDefault(conn.origin, new HashMap<>());
                if (metadata.containsKey(TOPIC)) {
                    pendingConnections.remove(route);
                    log.info("Connection verified {} -> {}", route, metadata.get(TOPIC));
                } else {
                    if (now - conn.created > GRACE_PERIOD) {
                        closeStalledConnection(md, "topic not assigned");
                    }
                }
            }
        }
    }

    private static void closeStalledConnection(WsMetadata md, String reason) {
        if (md != null) {
            try {
                log.info("Closing connection {} because {}", md.session, reason);
                pendingConnections.remove(md.session);
                closeConnection(md.txPath, TRY_AGAIN_LATER, reason);
            } catch (IOException e) {
                // ok to ignore
            }
        }
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object input, int instance) throws Exception {
        if (headers.containsKey(WsEnvelope.TYPE)) {
            String type = headers.get(WsEnvelope.TYPE);
            if (WsEnvelope.OPEN.equals(type)) {
                handleOpen(headers);
            } else if (WsEnvelope.CLOSE.equals(type)) {
                handleClose(headers);
            } else if (WsEnvelope.BYTES.equals(type) && input instanceof byte[]) {
                handleBytes(headers, (byte[]) input);
            }
            return true;
        } else {
            return false;
        }
    }

    private void handleOpen(Map<String, String> headers) throws IOException {
        String route = headers.get(WsEnvelope.ROUTE);
        String txPath = headers.get(WsEnvelope.TX_PATH);
        String token = headers.get(WsEnvelope.TOKEN);
        String ip = headers.get(WsEnvelope.IP);
        log.info("Session {} started, {}, {}", route, ip, token);
        Platform platform = Platform.getInstance();
        Utility util = Utility.getInstance();
        if (ready && platform.hasRoute(CLOUD_CONNECTOR) &&
                platform.hasRoute(CLOUD_MANAGER) && platform.hasRoute(MainApp.PRESENCE_HANDLER)) {
            Map<String, Object> info = new HashMap<>();
            String time = util.getLocalTimestamp(new Date().getTime());
            info.put(CREATED, time);
            info.put(UPDATED, time);
            info.put(MONITOR, platform.getOrigin());
            info.put(ID, route);
            info.put(SEQ, 0);
            myConnections.put(token, info);
            pendingConnections.put(route, new PendingConnection(token, route));
            WsMetadata md = new WsMetadata(route, txPath, token);
            connections.put(route, md);
        } else {
            log.info("Session {} closed - {}", route, STARTING_UP);
            closeConnection(txPath, TRY_AGAIN_LATER, STARTING_UP);
        }
    }

    private void handleBytes(Map<String, String> headers, byte[] payload) throws IOException {
        String route = headers.get(WsEnvelope.ROUTE);
        EventEmitter po = EventEmitter.getInstance();
        WsMetadata md = connections.get(route);
        if (md != null) {
            md.touch();
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
                            PendingConnection pc = pendingConnections.get(route);
                            if (pc != null) {
                                pendingConnections.put(route, pc.setType(PendingConnection.PendingType.HANDSHAKE));
                                log.info("Member registered {} {}", md.origin, info.get(NAME));
                                po.send(MainApp.TOPIC_CONTROLLER, new Kv(TYPE, GET_TOPIC),
                                        new Kv(TX_PATH, md.txPath), new Kv(ORIGIN, md.origin));
                            }
                        } else {
                            // this guarantees that a topic is used exclusively by a single app instance
                            if (isTopicAssigned(md.origin, info)) {
                                closeConnection(md.txPath, TRY_AGAIN_LATER, TOPIC_ASSIGNED);
                            } else {
                                log.debug("Member {} is alive {}", md.origin, info.get(SEQ));
                            }
                        }
                    }
                }

            } catch (Exception e) {
                closeConnection(md.txPath, TRY_AGAIN_LATER, INVALID_PROTOCOL);
            }

        }
    }

    @SuppressWarnings("unchecked")
    private void handleClose(Map<String, String> headers) throws IOException {
        String route = headers.get(WsEnvelope.ROUTE);
        EventEmitter po = EventEmitter.getInstance();
        WsMetadata md = connections.get(route);
        connections.remove(route);
        if (md != null) {
            myConnections.remove(md.origin);
            pendingConnections.remove(route);
            log.info("Session {} closed, {}", route, md.origin);
            if (connectionInfo.exists(md.origin)) {
                Object o = connectionInfo.get(md.origin);
                if (o instanceof Map) {
                    Map<String, Object> info = (Map<String, Object>) o;
                    if (route.equals(info.get(ID)) && info.get(GROUP) instanceof Integer) {
                        // broadcast to presence monitors
                        po.send(MainApp.PRESENCE_HANDLER + MONITOR_PARTITION,
                                new Kv(TYPE, DELETE), new Kv(ORIGIN, md.origin));
                        // tell all nodes to drop this node
                        leaveGroup(md.origin, (int) info.get(GROUP));
                    }
                }
            }
        } else {
            log.info("Session {} closed", route);
        }
    }

    private void leaveGroup(String closedApp, int groupId) {
        try {
            if (groupId < 1) {
                throw new IllegalArgumentException("Invalid closed user group ("+groupId+")");
            }
            // send leave event to the closed user group
            EventEmitter.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY + APP_GROUP + groupId,
                    new Kv(TYPE, LEAVE), new Kv(ORIGIN, closedApp));
            log.info("tell group {} that {} has left", groupId, closedApp);

        } catch (Exception e) {
            log.error("Unable to send leave event to group {} - {}", closedApp, e.getMessage());
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
        info.put(UPDATED, util.getLocalTimestamp(new Date().getTime()));
    }

    @SuppressWarnings("unchecked")
    private boolean isTopicAssigned(String origin, Map<String, Object> info) {
        if (info.containsKey(TOPIC)) {
            String myTopic = info.get(TOPIC).toString();
            Map<String, Object> allConnections = getConnections();
            for (String peer : allConnections.keySet()) {
                if (!origin.equals(peer)) {
                    Object o = allConnections.get(peer);
                    if (o instanceof Map) {
                        Map<String, Object> map = (Map<String, Object>) o;
                        if (map.containsKey(TOPIC)) {
                            String peerTopic = map.get(TOPIC).toString();
                            if (myTopic.equals(peerTopic)) {
                                log.warn("{} rejected because {} already assigned to {}", origin, peer, peerTopic);
                                return true;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    public static void closeConnection(String txPath, int status, String message) throws IOException {
        if (txPath != null && message != null) {
            EventEnvelope error = new EventEnvelope();
            error.setTo(txPath);
            error.setHeader(STATUS, status);
            error.setHeader(MESSAGE, message);
            error.setHeader(WsEnvelope.TYPE, WsEnvelope.CLOSE);
            EventEmitter.getInstance().send(error);
        }
    }

}
