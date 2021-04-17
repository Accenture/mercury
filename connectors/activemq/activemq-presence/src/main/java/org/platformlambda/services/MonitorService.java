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

package org.platformlambda.services;

import org.platformlambda.MainApp;
import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;
import org.platformlambda.activemq.ActiveMqSetup;
import org.platformlambda.models.PendingConnection;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.CloseReason;
import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

@WebSocketService("presence")
public class MonitorService implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(MonitorService.class);

    private static final String MONITOR_PARTITION = ActiveMqSetup.MONITOR_PARTITION;
    private static final String MANAGER = MainApp.MANAGER;
    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;
    private static final String APP_GROUP = ActiveMqSetup.APP_GROUP;
    private static final String PUT = "put";
    private static final String ALIVE = "keep-alive";
    private static final String INFO = "info";
    private static final String TYPE = "type";
    private static final String LEAVE = "leave";
    private static final String DELETE = "del";
    private static final String ORIGIN = "origin";
    private static final String ID = "id";
    private static final String SEQ = "seq";
    private static final String GROUP = "group";
    private static final String CREATED = "created";
    private static final String UPDATED = "updated";
    private static final String MONITOR = "monitor";
    private static final String GET_TOPIC = "get_topic";
    private static final String TX_PATH = "tx_path";
    private static final long EXPIRY = 60 * 1000;
    private static final long STALLED_CONNECTION = 20 * 1000;
    private static boolean ready = false;

    // websocket route to user application origin-ID. Websocket routes for this presence monitor instance only
    private static final ConcurrentMap<String, String> route2origin = new ConcurrentHashMap<>();
    // pending connections of TxPaths to monitor if handshake is done within a reasonable time
    private static final ConcurrentMap<String, PendingConnection> pendingConnections = new ConcurrentHashMap<>();
    // connection list of user applications to this presence monitor instance
    private static final ConcurrentMap<String, Map<String, Object>> myConnections = new ConcurrentHashMap<>();
    // user application connections for the whole system
    private static final ManagedCache connectionInfo = ManagedCache.createCache("app.presence.list", EXPIRY);

    public static void setReady() {
        MonitorService.ready = true;
    }

    public static Map<String, Object> getConnections() {
        return new HashMap<>(connectionInfo.getMap());
    }

    public static void clearStalledConnection() {
        Utility util = Utility.getInstance();
        List<String> pendingList = new ArrayList<>(pendingConnections.keySet());
        long now = System.currentTimeMillis();
        for (String route: pendingList) {
            PendingConnection conn = pendingConnections.get(route);
            if (now - conn.created > STALLED_CONNECTION) {
                String txPath = conn.txPath;
                try {
                    log.info("Closing stalled connection {}", route);
                    pendingConnections.remove(route);
                    util.closeConnection(txPath, CloseReason.CloseCodes.CANNOT_ACCEPT, "Invalid protocol");
                } catch (IOException e) {
                    // ok to ignore
                }
            }
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
            log.info("Member {} joined", origin);
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
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        String route, appOrigin, txPath;
        if (headers.containsKey(WsEnvelope.TYPE)) {
            switch (headers.get(WsEnvelope.TYPE)) {
                case WsEnvelope.OPEN:
                    // the open event contains route, txPath, ip, path, query and token
                    route = headers.get(WsEnvelope.ROUTE);
                    txPath = headers.get(WsEnvelope.TX_PATH);
                    appOrigin = headers.get(WsEnvelope.TOKEN);
                    String ip = headers.get(WsEnvelope.IP);
                    log.info("Started {}, {}, {}", route, ip, appOrigin);
                    // check if dependencies are ready
                    if (MonitorService.ready && platform.hasRoute(CLOUD_CONNECTOR) &&
                            platform.hasRoute(MANAGER) && platform.hasRoute(MainApp.PRESENCE_HANDLER)) {
                        Map<String, Object> info = new HashMap<>();
                        String time = util.date2str(new Date(), true);
                        info.put(CREATED, time);
                        info.put(UPDATED, time);
                        info.put(MONITOR, platform.getOrigin());
                        info.put(ID, route);
                        info.put(SEQ, 0);
                        route2origin.put(route, appOrigin);
                        myConnections.put(appOrigin, info);
                        pendingConnections.put(route, new PendingConnection(route, txPath));

                    } else {
                        util.closeConnection(txPath, CloseReason.CloseCodes.TRY_AGAIN_LATER,"Starting up");
                    }
                    break;
                case WsEnvelope.CLOSE:
                    // the close event contains only the route for this websocket
                    route = headers.get(WsEnvelope.ROUTE);
                    appOrigin = headers.get(WsEnvelope.TOKEN);
                    route2origin.remove(route);
                    myConnections.remove(appOrigin);
                    pendingConnections.remove(route);
                    log.info("Stopped {}, {}", route, appOrigin);
                    if (connectionInfo.exists(appOrigin)) {
                        Object o = connectionInfo.get(appOrigin);
                        if (o instanceof Map) {
                            Map<String, Object> info = (Map<String, Object>) o;
                            if (route.equals(info.get(ID)) && info.get(GROUP) instanceof Integer) {
                                // broadcast to presence monitors
                                po.send(MainApp.PRESENCE_HANDLER + MONITOR_PARTITION,
                                        new Kv(TYPE, DELETE), new Kv(ORIGIN, appOrigin));
                                // tell all nodes to drop this node
                                leaveGroup(appOrigin, (int) info.get(GROUP));
                            }
                        }
                    }
                    break;
                case WsEnvelope.BYTES:
                    // the data event for byteArray payload contains route and txPath
                    route = headers.get(WsEnvelope.ROUTE);
                    txPath = headers.get(WsEnvelope.TX_PATH);
                    appOrigin = route2origin.get(route);
                    if (body instanceof byte[] && appOrigin != null && myConnections.containsKey(appOrigin)) {
                        EventEnvelope command = new EventEnvelope();
                        command.load((byte[]) body);
                        boolean register = INFO.equals(command.getTo());
                        boolean alive = ALIVE.equals(command.getTo());
                        if (myConnections.containsKey(appOrigin)) {
                            Map<String, Object> info = myConnections.get(appOrigin);
                            if (register || alive) {
                                updateInfo(info, command.getHeaders());
                                // broadcast to all presence monitors
                                po.send(MainApp.PRESENCE_HANDLER + MONITOR_PARTITION, info,
                                        new Kv(TYPE, PUT), new Kv(ORIGIN, appOrigin));
                                if (register) {
                                    // tell the connected application instance to proceed
                                    pendingConnections.remove(route);
                                    log.info("Member registered {}", getMemberInfo(appOrigin, info));
                                    po.send(MainApp.TOPIC_CONTROLLER, new Kv(TYPE, GET_TOPIC),
                                            new Kv(TX_PATH, txPath), new Kv(ORIGIN, appOrigin));
                                } else {
                                    log.debug("Member {} is alive {}", appOrigin, info.get(SEQ));
                                }
                            }
                        }

                    }
                    break;
                case WsEnvelope.STRING:
                    log.debug("{}", body);
                    break;
                default:
                    break;
            }
        }
        // nothing to return because this is asynchronous
        return null;
    }

    private String getMemberInfo(String origin, Map<String, Object> info) {
        StringBuilder sb = new StringBuilder();
        for (String k: info.keySet()) {
            String v = info.get(k).toString();
            sb.append(k);
            sb.append('=');
            sb.append(v);
            sb.append(", ");
        }
        sb.append(ORIGIN);
        sb.append('=');
        sb.append(origin);
        return sb.toString();
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

}