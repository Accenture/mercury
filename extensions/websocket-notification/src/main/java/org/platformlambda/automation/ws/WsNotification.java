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

package org.platformlambda.automation.ws;

import org.platformlambda.automation.MainApp;
import org.platformlambda.automation.models.UserChannels;
import org.platformlambda.automation.models.UserSession;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.CloseReason;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;

public class WsNotification implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(WsNotification.class);

    private static final String X_USER = "x-user";  // this must be lower case
    private static final String TYPE = WsEnvelope.TYPE;
    private static final String OPEN = WsEnvelope.OPEN;
    private static final String CLOSE = WsEnvelope.CLOSE;
    private static final String PING = "ping";
    private static final String WS_OPEN = "_open";
    private static final String WS_CLOSE = "_close";
    private static final String WS_ALIVE = "alive";
    private static final String DOWNLOAD = "download";
    private static final String RESTORE = "restore";

    private static final String ORIGIN = "origin";
    private static final String ROUTE = WsEnvelope.ROUTE;
    private static final String IP = WsEnvelope.IP;
    private static final String TX_PATH = WsEnvelope.TX_PATH;
    private static final String QUERY = WsEnvelope.QUERY;
    private static final String BYTES = WsEnvelope.BYTES;
    private static final String TEXT = "text";
    private static final String APPLICATION = "application";
    private static final String GET_PATHS = "get_paths";
    private static final String USER_ID = "user_id";

    private static final String USER_NAMESPACE = "U:";
    private static final String SESSION_NAMESPACE = "S:";

    private static final long ONE_MINUTE = 60000;

    /*
     * For this example, it is using a local in-memory cache for storing the WebSocket sessions.
     *
     * For scalability, this example broadcasts websocket connection activities
     * so that each notification service has a complete picture of all websocket connections system-wide.
     * This feedback circuit avoids dependency on a distributed cache.
     *
     * To do this, this example app broadcast the open, close and keep-alive events to multiple instances of this app
     * so that their sessions are in-sync.
     *
     * However, for large scale deployment, you may use a distributed cache. e.g. Redis.
     * With a distributed cache, you do not need the feedback circuit in this example.
     *
     * Note that the DEFAULT_MAX_ITEMS is 2,000. Thus this session store will hold roughly 1,000 user sessions
     * You can adjust the default value by setting it in the createCache request.
     * i.e. ManagedCache sessions = ManagedCache.createCache("ws.sessions", 60000, maxUsers * 2);
     */
    private static final ManagedCache sessions = ManagedCache.createCache("ws.sessions", ONE_MINUTE);

    private List<String> getPaths(String userId) {
        UserChannels user = getUserChannels(userId);
        return user != null? user.getPaths() : new ArrayList<>();
    }

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        String me = Platform.getInstance().getOrigin();
        PostOffice po = PostOffice.getInstance();
        if (headers.containsKey(TYPE)) {
            String type = headers.get(TYPE);
            if (GET_PATHS.equals(type) && headers.containsKey(USER_ID)) {
                return getPaths(headers.get(USER_ID));
            }
            if (headers.containsKey(ROUTE)) {
                /*
                 * OPEN EVENT
                 *
                 * tx_path is available in headers
                 * application, query, ip and token are available in body as a map
                 *
                 * Sample input:
                 * headers = {route=api.in.21891.2, x-user=demo, tx_path=api.out.21891.2, type=open}
                 * body = {application=notification, query={a=b}, ip=127.0.0.1}
                 */
                if (type.equals(OPEN) && body instanceof Map && headers.containsKey(X_USER) &&
                        headers.containsKey(ROUTE) && headers.containsKey(TX_PATH)) {
                    // handle the request first
                    handleOpen(headers, body);
                    // then broadcast to peer
                    EventEnvelope forward = new EventEnvelope().setTo(MainApp.WS_NOTIFICATION_SERVICE);
                    for (String h : headers.keySet()) {
                        forward.setHeader(h, headers.get(h));
                    }
                    forward.setHeader(TYPE, WS_OPEN).setHeader(ORIGIN, me).setBody(body);
                    po.broadcast(forward);
                }
                /*
                 * OPEN EVENT from peers
                 *
                 * headers = {route=api.in.21891.2, x-user=demo, tx_path=api.out.21891.2, type=ws_open,
                 *            origin=2019091906d6c855c40241fdaf134c6b32f0d89a}
                 * body = {application=notification, query={a=b}, ip=127.0.0.1}
                 */
                if (type.equals(WS_OPEN) && headers.containsKey(ORIGIN)) {
                    String origin = headers.get(ORIGIN);
                    if (!origin.equals(me)) {
                        handleOpen(headers, body);
                    }
                }
                /*
                 * CLOSE EVENT
                 *
                 * Sample input:
                 * headers = {route=api.in.50344.2, type=close}
                 */
                if (type.equals(CLOSE) && headers.containsKey(ROUTE)) {
                    // handle the request
                    handleClose(headers);
                    // broadcast to peers
                    EventEnvelope forward = new EventEnvelope().setTo(MainApp.WS_NOTIFICATION_SERVICE);
                    for (String h : headers.keySet()) {
                        forward.setHeader(h, headers.get(h));
                    }
                    forward.setHeader(TYPE, WS_CLOSE).setHeader(ORIGIN, me);
                    po.broadcast(forward);
                }
                /*
                 * CLOSE EVENT from peers
                 *
                 * headers = {route=api.in.50344.2, type=ws_close, origin=2019091906d6c855c40241fdaf134c6b32f0d89a}
                 */
                if (type.equals(WS_CLOSE) && headers.containsKey(ORIGIN)) {
                    String origin = headers.get(ORIGIN);
                    if (!origin.equals(me)) {
                        handleClose(headers);
                    }
                }
                /*
                 * TEXT EVENT
                 *
                 * tx_path is available in headers
                 * body contains incoming message as text from browser
                 *
                 * Sample input:
                 * headers = {route=api.in.21891.2, tx_path=api.out.21891.2, type=text}
                 * body = {"hello":"world"}
                 *
                 * We recommend the browser sends a keep-alive every 20-30 seconds.
                 * The keep-alive message should be a JSON string with "type"="ping".
                 *
                 * With the exception of keep-alive, we do not take requests from a websocket connection.
                 * Therefore we can just drop the incoming message.
                 */
                if (type.equals(TEXT) && headers.containsKey(ROUTE) && headers.containsKey(TX_PATH)) {
                    boolean keepAlive = false;
                    String json = (String) body;
                    if (json != null) {
                        json = json.trim();
                        if (json.startsWith("{") && json.endsWith("}")) {
                            Map<String, Object> data = SimpleMapper.getInstance().getMapper().readValue(json, Map.class);
                            if (PING.equals(data.get(TYPE))) {
                                UserSession user = getUserSession(headers.get(ROUTE));
                                if (user != null) {
                                    po.broadcast(MainApp.WS_NOTIFICATION_SERVICE, user, new Kv(TYPE, WS_ALIVE));
                                    keepAlive = true;
                                } else {
                                    // session has expired
                                    log.warn("{} {} has expired", headers.get(ROUTE), headers.get(TX_PATH));
                                    Utility.getInstance().closeConnection(headers.get(TX_PATH),
                                            CloseReason.CloseCodes.GOING_AWAY, "Session expired");
                                }
                            }
                        }
                        if (!keepAlive) {
                            String txPath = headers.get(TX_PATH);
                            log.warn("Incoming events with {} characters dropped from {}", json.length(),
                                    txPath.contains("@") ? txPath.substring(0, txPath.indexOf('@')): txPath);
                        }
                    }
                }
                /*
                 * BYTES EVENT
                 *
                 * tx_path is available in headers
                 * body contains incoming message as bytes from browser
                 *
                 * Sample input:
                 * headers = {route=api.in.21891.2, tx_path=api.out.21891.2, type=bytes}
                 * body = byte array
                 *
                 * We do not take requests from a websocket connection.
                 * Therefore we can just drop the incoming message.
                 *
                 * You can add your custom logic
                 */
                if (type.equals(BYTES)) {
                    String txPath = headers.get(TX_PATH);
                    byte[] b = (byte[]) body;
                    log.warn("Incoming events with {} bytes dropped from {}", b.length,
                            txPath.contains("@") ? txPath.substring(0, txPath.indexOf('@')): txPath);
                }

            } else {
                /*
                 * KEEP-ALIVE EVENT
                 */
                if (type.equals(WS_ALIVE) && body instanceof UserSession) {
                    UserSession user = (UserSession) body;
                    UserSession record = getUserSession(user.getRoute());
                    if (record != null) {
                        touchUserChannel(record);
                        saveUserSession(record.touch());
                    } else {
                        // recover from record from peer
                        saveUserSession(user);
                        addUserChannel(user.getUserId(), user.getTxPath());
                        log.info("Recovered {}", user);
                    }
                }
                if (type.equals(DOWNLOAD) && headers.containsKey(ORIGIN)) {
                    String origin = headers.get(ORIGIN);
                    if (!origin.equals(me)) {
                        for (String key: sessions.getMap().keySet()) {
                            Object o = sessions.get(key);
                            po.send(MainApp.WS_NOTIFICATION_SERVICE +"@"+headers.get(ORIGIN), o, new Kv(TYPE, RESTORE));
                        }
                        // tell the new member that initial load is done
                        po.send(MainApp.INITIAL_LOAD+"@"+headers.get(ORIGIN), "done");
                    }
                }
                if (type.equals(RESTORE)) {
                    if (body instanceof UserSession) {
                        UserSession user = (UserSession) body;
                        UserSession existing = getUserSession(user.getRoute());
                        if (existing == null) {
                            saveUserSession(user);
                            log.info("Restored {}", user);
                        }
                    }
                    if (body instanceof UserChannels) {
                        UserChannels channels = (UserChannels) body;
                        UserChannels existing = getUserChannels(channels.getUserId());
                        if (existing == null) {
                            saveUserChannels(channels);
                            log.info("Restored {}", channels);
                        }
                    }
                }
            }
        }
        return null;
    }

    @SuppressWarnings("unchecked")
    private void addUserChannel(String userId, String txPath) {
        UserChannels channels = getUserChannels(userId);
        if (channels != null) {
            channels.addPath(txPath);
        } else {
            channels = new UserChannels(userId, txPath);
        }
        saveUserChannels(channels);
    }

    private void removeUserChannel(UserSession user) {
        UserChannels channels = getUserChannels(user.getUserId());
        if (channels != null) {
            channels.touch().removePath(user.getTxPath());
            if (channels.isEmpty()) {
                String key = USER_NAMESPACE +user.getUserId();
                sessions.remove(key);
            } else {
                saveUserChannels(channels);
            }
        }
    }

    private void touchUserChannel(UserSession user) {
        UserChannels channels = getUserChannels(user.getUserId());
        if (channels != null) {
            channels.touch();
        }
        saveUserChannels(channels);
    }

    @SuppressWarnings("unchecked")
    private void handleOpen(Map<String, String> headers, Object body) {
        String userId = headers.get(X_USER);
        String route = headers.get(ROUTE);
        String txPath = headers.get(TX_PATH);
        Map<String, Object> metadata = (Map<String, Object>) body;
        UserSession user = new UserSession(userId, route, txPath, metadata);
        saveUserSession(user);
        addUserChannel(userId, txPath);
        log.info("User {} connected to notification path {}", userId, txPath);
    }

    private void handleClose(Map<String, String> headers) {
        UserSession user = getUserSession(headers.get(ROUTE));
        if (user != null) {
            removeUserChannel(user);
            clearUserSession(user);
            log.info("User {} disconnected from notification path {}", user.getUserId(), user.getTxPath());
        }
    }

    private UserSession getUserSession(String route) {
        String key = SESSION_NAMESPACE +route;
        Object o = sessions.get(key);
        return o instanceof UserSession? (UserSession) o : null;
    }

    private void saveUserSession(UserSession user) {
        sessions.put(SESSION_NAMESPACE +user.getRoute(), user);
    }

    private void clearUserSession(UserSession user) {
        sessions.remove(SESSION_NAMESPACE +user.getRoute());
    }

    private UserChannels getUserChannels(String userId) {
        String key = USER_NAMESPACE +userId;
        Object o = sessions.get(key);
        return o instanceof UserChannels? (UserChannels) o : null;
    }

    private void saveUserChannels(UserChannels channels) {
        sessions.put(USER_NAMESPACE+channels.getUserId(), channels);
    }

}
