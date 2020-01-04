/*

    Copyright 2018-2020 Accenture Technology

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

package org.platformlambda.services;

import org.platformlambda.MainApp;
import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;
import org.platformlambda.hazelcast.PresenceHandler;
import org.platformlambda.hazelcast.TopicLifecycleListener;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.CloseReason;
import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;

@WebSocketService("presence")
public class MonitorService implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(MonitorService.class);

    private static final String MANAGER = MainApp.MANAGER;
    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;
    private static final String READY = "ready";

    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private static final String PUT = "put";
    private static final String ALIVE = "keep-alive";
    private static final String INFO = "info";
    private static final String TYPE = "type";
    private static final String LEAVE = "leave";
    private static final String DELETE = "del";
    private static final String ORIGIN = "origin";
    private static final String ID = "id";
    private static final String SEQ = "seq";
    private static final String CREATED = "created";
    private static final String UPDATED = "updated";
    private static final String MONITOR = "monitor";
    private static final String PEERS = "peers";
    private static final long EXPIRY = 60 * 1000;

    // websocket route to user application origin-ID. Websocket routes for this presence monitor instance only
    private static final ConcurrentMap<String, String> route2token = new ConcurrentHashMap<>();
    // connection list of user applications to this presence monitor instance
    private static final ConcurrentMap<String, Map<String, Object>> token2info = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, String> token2txPath = new ConcurrentHashMap<>();
    // user application connections for the whole system
    private static final ManagedCache connectionInfo = ManagedCache.createCache("app.ws.info", EXPIRY);

    public static Map<String, Object> getConnections() {
        return new HashMap<>(connectionInfo.getMap());
    }

    public static void closeAllConnections() {
        Utility util = Utility.getInstance();
        for (String token: token2txPath.keySet()) {
            String txPath = token2txPath.get(token);
            log.warn("Reset connection {}", txPath);
            try {
                util.closeConnection(txPath, CloseReason.CloseCodes.TRY_AGAIN_LATER,"Starting up");
            } catch (IOException e) {
                log.warn("Unable to close connection {}", txPath);
            }
        }
    }

    public static void updateNodeInfo(String origin, Map<String, Object> info) {
        if (connectionInfo.exists(origin)) {
            Object o = connectionInfo.get(origin);
            if (o instanceof Map) {
                if (!info.equals(o)) {
                    connectionInfo.put(origin, info);
                    log.debug("Updating {}", origin);
                }
            }
        } else {
            connectionInfo.put(origin, info);
            log.info("Adding {}", origin);
            notifyConnectedApps();
        }
    }

    public static void deleteNodeInfo(String origin) {
        if (connectionInfo.exists(origin)) {
            connectionInfo.remove(origin);
            log.info("Removing {}", origin);
            notifyConnectedApps();
        }
    }

    public static List<String> getOrigins() {
        return new ArrayList<>(route2token.values());
    }

    private static void notifyConnectedApps() {
        if (!token2txPath.isEmpty()) {
            PostOffice po = PostOffice.getInstance();
            Map<String, Object> connections = connectionInfo.getMap();
            List<String> list = new ArrayList<>(connections.keySet());
            for (String token: token2txPath.keySet()) {
                String txPath = token2txPath.get(token);
                EventEnvelope event = new EventEnvelope();
                event.setTo(ServiceDiscovery.SERVICE_REGISTRY);
                event.setHeader(ORIGIN, token);
                event.setHeader(TYPE, PEERS);
                event.setBody(list);
                try {
                    po.send(txPath, event.toBytes());
                } catch (IOException e) {
                    log.error("Unable to update peer list to {} - {}", token, e.getMessage());
                }
            }
        }
    }

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {

        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        String route, token, txPath;

        if (headers.containsKey(WsEnvelope.TYPE)) {
            switch (headers.get(WsEnvelope.TYPE)) {
                case WsEnvelope.OPEN:
                    // the open event contains route, txPath, ip, path, query and token
                    route = headers.get(WsEnvelope.ROUTE);
                    txPath = headers.get(WsEnvelope.TX_PATH);
                    token = headers.get(WsEnvelope.TOKEN);
                    String ip = headers.get(WsEnvelope.IP);
                    log.info("Started {}, {}, node={}", route, ip, token);
                    // check if dependencies are ready
                    if (PresenceHandler.isReady() && TopicLifecycleListener.isReady() &&
                            platform.hasRoute(CLOUD_CONNECTOR) && platform.hasRoute(MANAGER) &&
                            platform.hasRoute(MainApp.PRESENCE_MONITOR) &&
                            platform.hasRoute(MainApp.PRESENCE_HANDLER)) {
                        // check if there is a corresponding message hub is available
                        boolean exists = false;
                        if (validTopicId(token)) {
                            try {
                                EventEnvelope response = po.request(MANAGER, 10000, new Kv(TYPE, EXISTS), new Kv(ORIGIN, token));
                                if (response.getBody() instanceof Boolean) {
                                    exists = (Boolean) response.getBody();
                                }
                            } catch (TimeoutException | AppException e) {
                                log.error("Unable to check if topic {} exists - {}", token, e.getMessage());
                            }
                        }
                        if (exists) {
                            Map<String, Object> info = new HashMap<>();
                            String time = Utility.getInstance().date2str(new Date(), true);
                            info.put(CREATED, time);
                            info.put(MONITOR, platform.getOrigin());
                            info.put(ID, route);
                            route2token.put(route, token);
                            token2info.put(token, info);
                            token2txPath.put(token, txPath);
                        } else {
                            Utility.getInstance().closeConnection(txPath, CloseReason.CloseCodes.PROTOCOL_ERROR, "Unauthorized");
                        }
                    } else {
                        Utility.getInstance().closeConnection(txPath, CloseReason.CloseCodes.TRY_AGAIN_LATER,"Starting up");
                    }
                    break;
                case WsEnvelope.CLOSE:
                    // the close event contains only the route for this websocket
                    route = headers.get(WsEnvelope.ROUTE);
                    token = headers.get(WsEnvelope.TOKEN);
                    route2token.remove(route);
                    token2info.remove(token);
                    token2txPath.remove(token);
                    log.info("Stopped {}, node={}", route, token);
                    if (connectionInfo.exists(token)) {
                        Object o = connectionInfo.get(token);
                        if (o instanceof Map) {
                            Map<String, Object> info = (Map<String, Object>) o;
                            if (route.equals(info.get("id"))) {
                                /*
                                 * broadcast to all presence monitors
                                 */
                                EventEnvelope event = new EventEnvelope();
                                event.setTo(MainApp.PRESENCE_HANDLER);
                                event.setHeader(ORIGIN, token);
                                event.setHeader(TYPE, DELETE);
                                po.send(MainApp.PRESENCE_MONITOR, event.toBytes());
                                // tell all nodes to drop this node
                                broadcastLeaveEvent(token);
                            }
                        }
                    }
                    break;
                case WsEnvelope.BYTES:
                    // the data event for byteArray payload contains route and txPath
                    route = headers.get(WsEnvelope.ROUTE);
                    txPath = headers.get(WsEnvelope.TX_PATH);
                    token = route2token.get(route);
                    if (body instanceof byte[] && token != null && token2info.containsKey(token)) {
                        EventEnvelope command = new EventEnvelope();
                        command.load((byte[]) body);
                        boolean isInfo = INFO.equals(command.getTo());
                        boolean isAlive = ALIVE.equals(command.getTo());
                        if (token2info.containsKey(token)) {
                            Map<String, Object> info = token2info.get(token);
                            if (isInfo || isAlive) {
                                updateInfo(info, command.getHeaders());
                                /*
                                 * broadcast to all presence monitors
                                 */
                                EventEnvelope event = new EventEnvelope();
                                event.setTo(MainApp.PRESENCE_HANDLER);
                                event.setHeader(ORIGIN, token);
                                event.setHeader(TYPE, PUT);
                                event.setBody(info);
                                po.send(MainApp.PRESENCE_MONITOR, event.toBytes());
                                if (isInfo) {
                                    log.info("Member registered {}", info);
                                    po.send(txPath, new EventEnvelope().setTo(READY).toBytes());
                                } else {
                                    log.debug("Member {} is alive {}", token, info.get(SEQ));
                                    po.send(txPath, ALIVE + " " + info.get(SEQ));
                                }
                            }
                        }
                    }
                    break;
                default:
                    // this should not happen
                    break;
            }
        }
        // nothing to return because this is asynchronous
        return null;
    }

    private void updateInfo(Map<String, Object> info, Map<String, String> headers) {
        Utility util = Utility.getInstance();
        for (String key : headers.keySet()) {
            if (!key.equals(ID) && !key.equals(CREATED) && !key.equals(MONITOR)) {
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
        info.put(UPDATED, Utility.getInstance().date2str(new Date(), true));
    }

    private void broadcastLeaveEvent(String closedApp) {
        try {
            PostOffice po = PostOffice.getInstance();
            List<String> peers = getPeers();
            // and broadcast the leave event to all nodes
            for (String p : peers) {
                if (!p.equals(closedApp)) {
                    po.send(ServiceDiscovery.SERVICE_REGISTRY + "@" + p, new Kv(TYPE, LEAVE), new Kv(ORIGIN, closedApp));
                    log.info("tell {} that {} has left", p, closedApp);
                }
            }

        } catch (Exception e) {
            log.error("Unable to broadcast leave event for {} - {}", closedApp, e.getMessage());
        }
    }

    @SuppressWarnings("unchecked")
    private List<String> getPeers() throws IOException, TimeoutException, AppException {
        List<String> peers = new ArrayList<>();
        PostOffice po = PostOffice.getInstance();
        // check topic list
        EventEnvelope list = po.request(MANAGER, 20000, new Kv(TYPE, LIST));
        if (list.getBody() instanceof List) {
            List<String> topics = (List<String>) list.getBody();
            if (!topics.isEmpty()) {
                for (String t : topics) {
                    // check if topic is active
                    if (connectionInfo.exists(t)) {
                        peers.add(t);
                    }
                }
            }
        }
        return peers;
    }

    /**
     * Validate a topic ID
     *
     * @param topic in format of yyyymmdd uuid
     * @return true if valid
     */
    private boolean validTopicId(String topic) {
        Platform platform = Platform.getInstance();
        if (topic.length() != platform.getOrigin().length()) {
            return false;
        }
        String uuid = topic.substring(8);
        if (!Utility.getInstance().isDigits(topic.substring(0, 8))) {
            return false;
        }
        // drop namespace before validation
        if (platform.getNamespace() != null) {
            int dot = uuid.lastIndexOf('.');
            if (dot > 1) {
                uuid = uuid.substring(0, dot);
            }
        }
        // application instance ID should be hexadecimal
        for (int i=0; i < uuid.length(); i++) {
            if (uuid.charAt(i) >= '0' && uuid.charAt(i) <= '9') continue;
            if (uuid.charAt(i) >= 'a' && uuid.charAt(i) <= 'f') continue;
            return false;
        }
        return true;
    }

}