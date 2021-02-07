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

package org.platformlambda.automation.ws;

import org.platformlambda.automation.MainModule;
import org.platformlambda.automation.config.WsEntry;
import org.platformlambda.automation.models.WsInfo;
import org.platformlambda.automation.models.WsMetadata;
import org.platformlambda.automation.services.NotificationManager;
import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.CloseReason;
import java.io.IOException;
import java.io.UnsupportedEncodingException;
import java.net.URLDecoder;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

@WebSocketService("api")
public class WsGateway implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(WsGateway.class);

    private static final String TYPE = WsEnvelope.TYPE;
    private static final String OPEN = WsEnvelope.OPEN;
    private static final String CLOSE = WsEnvelope.CLOSE;
    private static final String ROUTE = WsEnvelope.ROUTE;
    private static final String IP = WsEnvelope.IP;
    private static final String ORIGIN = "origin";
    private static final String TX_PATH = WsEnvelope.TX_PATH;
    private static final String QUERY = WsEnvelope.QUERY;
    private static final String BYTES = WsEnvelope.BYTES;
    private static final String STRING = WsEnvelope.STRING;
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

    // route -> info
    private static final ConcurrentMap<String, WsMetadata> connections = new ConcurrentHashMap<>();

    public static void closeAllConnections() {
        Utility util = Utility.getInstance();
        List<String> keys = new ArrayList<>(connections.keySet());
        for (String k: keys) {
            WsMetadata md = connections.get(k);
            try {
                util.closeConnection(md.txPath, CloseReason.CloseCodes.GOING_AWAY,
                        "Service temporarily unavailable");
            } catch (IOException e) {
                // ok to ignore
            }
        }
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (headers.containsKey(TYPE)) {
            String type = headers.get(TYPE);
            if (OPEN.equals(type)) {
                handleOpen(headers);
            }
            if (CLOSE.equals(type)) {
                handleClose(headers);
            }
            if (BYTES.equals(type)) {
                log.warn("Websocket byte message ignored - {}", headers);
            }
            if (STRING.equals(type) && body instanceof String) {
                handleMessage(headers, ((String) body).trim());
            }
        }
        // nothing to return because this is asynchronous
        return null;
    }

    private void handleOpen(Map<String, String> headers) throws IOException {
        Utility util = Utility.getInstance();
        String route = headers.get(ROUTE);
        String txPath = headers.get(TX_PATH);
        if (!NotificationManager.isReady()) {
            util.closeConnection(txPath, CloseReason.CloseCodes.GOING_AWAY,
                    "Service temporarily unavailable");
            return;
        }
        String origin = Platform.getInstance().getOrigin();
        PostOffice po = PostOffice.getInstance();
        WsEntry wsEntry = WsEntry.getInstance();
        if (wsEntry.isEmpty()) {
            util.closeConnection(txPath, CloseReason.CloseCodes.CANNOT_ACCEPT,
                    "WebSocket service not enabled");
            return;
        }
        // the open event contains route, txPath, ip, path, query and token
        String ip = headers.get(IP);
        String identifier = headers.get(TOKEN).toLowerCase();
        int colon = identifier.indexOf(':');
        if (colon == -1) {
            util.closeConnection(txPath, CloseReason.CloseCodes.CANNOT_ACCEPT,
                    "URL must end with {application}:{token}");
            return;
        }
        String app = identifier.substring(0, colon).trim();
        String token = identifier.substring(colon+1).trim();
        WsInfo info = wsEntry.getInfo(app);
        if (info == null) {
            util.closeConnection(txPath, CloseReason.CloseCodes.CANNOT_ACCEPT,
                    "WebSocket application "+app+" not found");
            return;
        }
        String permitted = NotificationManager.getApplication(token);
        if (!app.equals(permitted)) {
            util.closeConnection(txPath, CloseReason.CloseCodes.CANNOT_ACCEPT, "Invalid access token");
            return;
        }
        log.info("Started route={}, ip={}, app={}", route, ip, app);
        WsMetadata md = new WsMetadata(info.application, info.recipient, txPath, info.publish, info.subscribe);
        connections.put(route, md);
        po.broadcast(MainModule.NOTIFICATION_MANAGER, new Kv(TYPE, CLEAR), new Kv(TOKEN, token));
        if (!WsEntry.NONE_PROVIDED.equals(info.recipient)) {
            Map<String, Object> event = new HashMap<>();
            event.put(QUERY, getQueryParameters(headers.get(QUERY)));
            event.put(APPLICATION, info.application);
            event.put(IP, ip);
            event.put(ORIGIN, origin);
            event.put(TX_PATH, txPath + "@" + origin);
            // broadcast to multiple instance of the recipient service
            po.broadcast(new EventEnvelope().setTo(info.recipient).setBody(event).setHeader(TYPE, OPEN));
        }
    }

    private void handleClose(Map<String, String> headers) throws IOException {
        String origin = Platform.getInstance().getOrigin();
        PostOffice po = PostOffice.getInstance();
        String route = headers.get(ROUTE);
        log.info("Stopping {}", route);
        WsMetadata md = connections.get(route);
        if (md != null) {
            String target = md.txPath + "@" + origin;
            // tell notification manager to clear routing entries
            po.broadcast(MainModule.NOTIFICATION_MANAGER, new Kv(TYPE, CLOSE), new Kv(TX_PATH, target));
            if (!WsEntry.NONE_PROVIDED.equals(md.recipient)) {
                connections.remove(route);
                Map<String, Object> event = new HashMap<>();
                event.put(APPLICATION, md.application);
                event.put(ORIGIN, origin);
                event.put(TX_PATH, target);
                // broadcast to multiple instance of the recipient service
                po.broadcast(new EventEnvelope().setTo(md.recipient).setBody(event).setHeader(TYPE, CLOSE));
            }
        }
    }

    @SuppressWarnings("unchecked")
    private void handleMessage(Map<String, String> headers, String body) throws IOException {
        String origin = Platform.getInstance().getOrigin();
        PostOffice po = PostOffice.getInstance();
        String route = headers.get(ROUTE);
        String txPath = headers.get(TX_PATH);
        WsMetadata md = connections.get(route);
        if (md != null) {
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

    private Map<String, String> getQueryParameters(String query) throws UnsupportedEncodingException {
        Map<String, String> result = new HashMap<>();
        Utility util = Utility.getInstance();
        List<String> parts = util.split(query, "&");
        for (String p: parts) {
            int sep = p.indexOf('=');
            if (sep > 0) {
                String key = URLDecoder.decode(p.substring(0, sep), "UTF-8");
                String value = URLDecoder.decode(p.substring(sep+1), "UTF-8");
                result.put(key, value);
            } else {
                result.put(URLDecoder.decode(p, "UTF-8"), "");
            }
        }
        return result;
    }

}
