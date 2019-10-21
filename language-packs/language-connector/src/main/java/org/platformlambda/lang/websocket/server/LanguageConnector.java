/*

    Copyright 2018-2019 Accenture Technology

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

package org.platformlambda.lang.websocket.server;

import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.SimpleCache;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.platformlambda.core.websocket.common.WsConfigurator;
import org.platformlambda.lang.services.*;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.CloseReason;
import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

@WebSocketService("lang")
public class LanguageConnector implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(LanguageConnector.class);
    private static final MsgPack msgPack = new MsgPack();

    public static final String TYPE = "type";
    public static final String EVENT = "event";
    public static final String ROUTE = "route";
    public static final String BLOCK = "block";

    private static final String ID = "id";
    private static final String TO = "to";
    private static final String FROM = "from";
    private static final String HEADERS = "headers";
    private static final String BODY = "body";
    private static final String STATUS = "status";
    private static final String REPLY_TO = "reply_to";
    private static final String CID = "cid";
    private static final String EXTRA = "extra";
    private static final String LOGIN = "login";
    private static final String ADD = "add";
    private static final String REMOVE = "remove";
    private static final String READY = "ready";
    private static final String DISCONNECT = "disconnect";
    private static final String TOKEN = "token";
    private static final String API_KEY = "api_key";
    private static final String BROADCAST = "broadcast";
    private static final String EXEC_TIME = "exec_time";
    private static final String TRACE_ID = "trace_id";
    private static final String TRACE_PATH = "trace_path";
    private static final String LANGUAGE_REGISTRY = "language.pack.registry";
    private static final String LANGUAGE_INBOX = "language.pack.inbox";
    private static final String PUB_SUB_CONTROLLER = "pub.sub.controller";
    private static final String DEFERRED_DELIVERY = "system.deferred.delivery";
    private static final String SYSTEM_ALERT = "system.alerts";
    private static final String SYSTEM_CONFIG = "system.config";
    private static final String MAX_PAYLOAD = "max.payload";
    private static final String COUNT = MultipartPayload.COUNT;
    private static final String TOTAL = MultipartPayload.TOTAL;
    private static final int OVERHEAD = MultipartPayload.OVERHEAD;

    private static final SimpleCache cache = SimpleCache.createCache("payload.segmentation", 60000);
    private static String apiKey, inboxRoute;

    private enum State {
        OPEN, AUTHENTICATED
    }
    // token -> ConnectionStatus
    private static final ConcurrentMap<String, ConnectionStatus> connections = new ConcurrentHashMap<>();
    // route -> token
    private static final ConcurrentMap<String, List<String>> routingTable = new ConcurrentHashMap<>();
    // rxPath -> token
    private static final ConcurrentMap<String, String> tokens = new ConcurrentHashMap<>();

    public static String getTxPathFromToken(String token) {
        ConnectionStatus client = connections.get(token);
        return client == null? null : client.getTxPath();
    }

    public static List<String> getDestinations(String route) {
        List<String> result = routingTable.get(route);
        return result == null? new ArrayList<>() : new ArrayList<>(result);
    }

    public static boolean hasRoute(String route) {
        return routingTable.containsKey(route);
    }

    private static void notifyClient(String txPath, int status, String message) throws IOException {
        Map<String, Object> response = new HashMap<>();
        response.put(TYPE, EVENT);
        response.put(ROUTE, SYSTEM_ALERT);
        EventEnvelope alert = new EventEnvelope();
        alert.setTo(SYSTEM_ALERT);
        alert.setHeader(STATUS, status);
        alert.setStatus(status);
        alert.setBody(message);
        response.put(EVENT, LanguageConnector.mapFromEvent(alert));
        PostOffice.getInstance().send(txPath, msgPack.pack(response));
    }

    private static void sendServerConfig(String txPath, Map<String, Object> config) throws IOException {
        Map<String, Object> response = new HashMap<>();
        response.put(TYPE, EVENT);
        response.put(ROUTE, SYSTEM_CONFIG);
        EventEnvelope alert = new EventEnvelope();
        alert.setTo(SYSTEM_CONFIG);
        alert.setHeader(TYPE, SYSTEM_CONFIG);
        alert.setBody(config);
        response.put(EVENT, LanguageConnector.mapFromEvent(alert));
        PostOffice.getInstance().send(txPath, msgPack.pack(response));
    }

    private static void sendReady(String txPath) throws IOException {
        Map<String, Object> response = new HashMap<>();
        response.put(TYPE, EVENT);
        response.put(ROUTE, SYSTEM_CONFIG);
        EventEnvelope alert = new EventEnvelope();
        alert.setTo(SYSTEM_CONFIG);
        alert.setHeader(TYPE, READY);
        response.put(EVENT, LanguageConnector.mapFromEvent(alert));
        PostOffice.getInstance().send(txPath, msgPack.pack(response));
    }

    private static String getApiKey() {
        Utility util = Utility.getInstance();
        AppConfigReader reader = AppConfigReader.getInstance();
        String envVar = reader.getProperty("api.key.location");
        String apiKeyInEnv = System.getenv(envVar);
        if (apiKeyInEnv != null) {
            log.info("Found API key in environment variable {}", envVar);
            return apiKeyInEnv;
        }
        File tempConfigDir = new File("/tmp/config");
        if (!tempConfigDir.exists()) {
            tempConfigDir.mkdirs();
        }
        File apiKeyFile = new File(tempConfigDir, "lang-api-key.txt");
        if (apiKeyFile.exists()) {
            log.info("Reading API key from {}", apiKeyFile);
            return util.file2str(apiKeyFile).trim();
        } else {
            log.warn("Generating new API key in {} because it is not found in environment variable {}", apiKeyFile, envVar);
            String key = util.getUuid();
            util.str2file(apiKeyFile, key+"\n");
            return key;
        }
    }

    public static void initialize() throws IOException {
        if (apiKey == null) {
            apiKey = getApiKey();
            log.info("Started");
            LambdaFunction registry = (headers, body, instance) -> {
                if (headers.containsKey(TYPE)) {
                    String type = headers.get(TYPE);
                    if (ADD.equals(type) && headers.containsKey(ROUTE) && headers.containsKey(TOKEN)) {
                        String route = headers.get(ROUTE);
                        String token = headers.get(TOKEN);
                        ConnectionStatus client = connections.get(token);
                        if (client != null) {
                            if (Platform.getInstance().hasRoute(route) && !routingTable.containsKey(route)) {
                                notifyClient(client.getTxPath(), 409,
                                        "Unable to advertise " + route + " because route is reserved");
                            } else {
                                if (!routingTable.containsKey(route)) {
                                    routingTable.put(route, new ArrayList<>());
                                    Platform.getInstance().register(route, LanguageRelay.getInstance(), 1);
                                }
                                List<String> clients = routingTable.get(route);
                                clients.add(token);
                                notifyClient(client.getTxPath(), 200, route+" advertised to network");
                                log.info("{} added to {}", token, route);
                            }
                        }
                    }
                    if (REMOVE.equals(type) && headers.containsKey(ROUTE) && headers.containsKey(TOKEN)) {
                        String route = headers.get(ROUTE);
                        String token = headers.get(TOKEN);
                        if (routingTable.containsKey(route)) {
                            removeClientFromRoute(route, token);
                            ConnectionStatus client = connections.get(token);
                            if (client != null) {
                                notifyClient(client.getTxPath(), 200, route+" removed from network");
                            }
                        }
                    }
                    if (DISCONNECT.equals(type) && headers.containsKey(TOKEN)) {
                        String token = headers.get(TOKEN);
                        List<String> routes = new ArrayList<>(routingTable.keySet());
                        for (String r: routes) {
                            removeClientFromRoute(r, token);
                        }
                    }
                    if (READY.equals(type) && headers.containsKey(READY)) {
                        sendReady(headers.get(READY));
                    }
                }
                return null;
            };
            Platform platform = Platform.getInstance();
            platform.registerPrivate(LANGUAGE_REGISTRY, registry, 1);
            platform.registerPrivate(LANGUAGE_INBOX, new LanguageInbox(), 1);
            platform.registerPrivate(PUB_SUB_CONTROLLER, new PubSubController(), 1);
            platform.registerPrivate(DEFERRED_DELIVERY, new DeferredDelivery(), 1);
            LanguageConnector.inboxRoute = LANGUAGE_INBOX + "@" + platform.getOrigin();
        }
    }

    private static void removeClientFromRoute(String route, String token) throws IOException {
        List<String> clients = routingTable.get(route);
        if (clients != null && clients.contains(token)) {
            clients.remove(token);
            log.info("{} removed from {}", token, route);
            if (clients.isEmpty()) {
                routingTable.remove(route);
                Platform.getInstance().release(route);
                TopicListener.releaseRoute(route);
                log.info("{} cleared", route);
            }
        }
    }

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        PostOffice po = PostOffice.getInstance();
        String rxPath, token, txPath;

        if (headers.containsKey(WsEnvelope.TYPE)) {
            switch (headers.get(WsEnvelope.TYPE)) {
                case WsEnvelope.OPEN:
                    // the open event contains rxPath, txPath, ip, path, query and token
                    rxPath = headers.get(WsEnvelope.ROUTE);
                    txPath = headers.get(WsEnvelope.TX_PATH);
                    token = headers.get(WsEnvelope.TOKEN);
                    String ip = headers.get(WsEnvelope.IP);
                    String path = headers.get(WsEnvelope.PATH);
                    String query = headers.get(WsEnvelope.QUERY);
                    connections.put(token, new ConnectionStatus(rxPath, txPath, token));
                    tokens.put(rxPath, token);
                    log.info("Started {}, {}, ip={}, path={}, query={}, token={}", rxPath, txPath, ip, path, query, token);
                    break;
                case WsEnvelope.CLOSE:
                    // the close event contains rxPath and token for this websocket
                    rxPath = headers.get(WsEnvelope.ROUTE);
                    token = headers.get(WsEnvelope.TOKEN);
                    log.info("Stopped {}, token={}", rxPath, token);
                    connections.remove(token);
                    tokens.remove(rxPath);
                    po.send(LANGUAGE_REGISTRY, new Kv(TYPE, DISCONNECT), new Kv(TOKEN, token));
                    break;
                case WsEnvelope.BYTES:
                    // the data event for byteArray payload contains rxPath and txPath
                    rxPath = headers.get(WsEnvelope.ROUTE);
                    txPath = headers.get(WsEnvelope.TX_PATH);
                    token = tokens.get(rxPath);
                    if (token != null) {
                        Map<String, Object> event = (Map<String, Object>) msgPack.unpack((byte[]) body);
                        if (event.containsKey(TYPE)) {
                            String type = (String) event.get(TYPE);
                            ConnectionStatus client = connections.get(token);
                            if (client != null) {
                                if (client.getState() == State.OPEN) {
                                    if (LOGIN.equals(type) && event.containsKey(API_KEY)
                                            && event.get(API_KEY).equals(apiKey)) {
                                        client.setState(State.AUTHENTICATED);
                                        Map<String, Object> config = new HashMap<>();
                                        config.put(MAX_PAYLOAD, WsConfigurator.getInstance().getMaxBinaryPayload() - OVERHEAD);
                                        sendServerConfig(txPath, config);
                                        log.info("{} authenticated", token);
                                    } else {
                                        // txPath, CloseReason.CloseCodes status, String message
                                        Utility.getInstance().closeConnection(txPath, CloseReason.CloseCodes.CANNOT_ACCEPT,
                                                "Requires login with "+API_KEY);
                                    }
                                } else if (client.getState() == State.AUTHENTICATED) {
                                    // handle registration of public routes
                                    if (ADD.equals(type) && event.containsKey(ROUTE)) {
                                        po.send(LANGUAGE_REGISTRY, new Kv(TYPE, ADD),
                                                new Kv(TOKEN, token), new Kv(ROUTE, event.get(ROUTE)));
                                    }
                                    if (REMOVE.equals(type) && event.containsKey(ROUTE)) {
                                        po.send(LANGUAGE_REGISTRY, new Kv(TYPE, REMOVE),
                                                new Kv(TOKEN, token), new Kv(ROUTE, event.get(ROUTE)));
                                    }
                                    /*
                                     * echo ready signal through language registry
                                     * so that the acknowledgement is done orderly
                                     */
                                    if (READY.equals(type)) {
                                        po.send(LANGUAGE_REGISTRY, new Kv(TYPE, READY), new Kv(READY, txPath));
                                    }
                                    // reconstruct large payload
                                    if (BLOCK.equals(type) && event.containsKey(BLOCK)) {
                                        EventEnvelope block = eventFromMap((Map<String, Object>) event.get(BLOCK));
                                        Map<String, String> control = block.getHeaders();
                                        if (control.size() == 3 && control.containsKey(ID)
                                                && control.containsKey(COUNT) && control.containsKey(TOTAL)) {
                                            Utility util = Utility.getInstance();
                                            String id = control.get(ID);
                                            int count = util.str2int(control.get(COUNT));
                                            int total = util.str2int(control.get(TOTAL));
                                            byte[] data = (byte[]) block.getBody();
                                            if (data != null && count != -1 && total != -1) {
                                                ByteArrayOutputStream buffer = (ByteArrayOutputStream) cache.get(id);
                                                if (count == 1 || buffer == null) {
                                                    buffer = new ByteArrayOutputStream();
                                                    cache.put(id, buffer);
                                                }
                                                buffer.write(data);
                                                if (count == total) {
                                                    cache.remove(id);
                                                    Map<String, Object> evt = (Map<String, Object>) msgPack.unpack(buffer.toByteArray());
                                                    EventEnvelope request = eventFromMap((Map<String, Object>) evt.get(EVENT));
                                                    po.send(mapReplyTo(token, request));
                                                }
                                            }
                                        }
                                    }
                                    // regular events
                                    if (EVENT.equals(type) && event.containsKey(EVENT)) {
                                        EventEnvelope request = eventFromMap((Map<String, Object>) event.get(EVENT));
                                        po.send(mapReplyTo(token, request));
                                    }
                                }
                            }
                        }
                    }
                    break;
                case WsEnvelope.STRING:
                    // this is likely a keep-alive message from the language pack client
                    // rxPath is not used as we just want to echo back the keep-alive message.
                    // rxPath = headers.get(WsEnvelope.ROUTE);
                    txPath = headers.get(WsEnvelope.TX_PATH);
                    String message = (String) body;
                    po.send(txPath, message);
                    break;
                default:
                    // this should not happen
                    log.error("Invalid event {} {}", headers, body);
                    break;
            }
        }
        // nothing to return because this is asynchronous
        return null;
    }

    private EventEnvelope mapReplyTo(String token, EventEnvelope request) {
        if (request.getReplyTo() != null) {
            String replyTo = request.getReplyTo();
            if (replyTo.startsWith("->")) {
                /*
                 * Encode the client token ID with reply to so the language inbox
                 * can relay the service response correctly.
                 *
                 * Change the replyTo to the language inbox which will intercept
                 * the service responses.
                 */
                request.setExtra(token+replyTo);
                request.setReplyTo(LanguageConnector.inboxRoute);
            }
        }
        return request;
    }

    @SuppressWarnings("unchecked")
    private EventEnvelope eventFromMap(Map<String, Object> map) {
        EventEnvelope event = new EventEnvelope();
        if (map.containsKey(ID)) {
            event.setId((String) map.get(ID));
        }
        if (map.containsKey(TO)) {
            event.setTo((String) map.get(TO));
        }
        if (map.containsKey(FROM)) {
            event.setFrom((String) map.get(FROM));
        }
        if (map.containsKey(STATUS)) {
            event.setStatus((Integer) map.get(STATUS));
        }
        if (map.containsKey(REPLY_TO)) {
            event.setReplyTo((String) map.get(REPLY_TO));
        }
        if (map.containsKey(TRACE_ID) && map.containsKey(TRACE_PATH)) {
            event.setTrace((String) map.get(TRACE_ID), (String) map.get(TRACE_PATH));
        }
        if (map.containsKey(BROADCAST)) {
            boolean broadcast = (Boolean) map.get(BROADCAST);
            if (broadcast) {
                event.setBroadcastLevel(1);
            }
        }
        if (map.containsKey(HEADERS)) {
            Map<String, Object> headers = (Map<String, Object>) map.get(HEADERS);
            for (String key: headers.keySet()) {
                event.setHeader(key, headers.get(key));
            }
        }
        if (map.containsKey(BODY)) {
            event.setBody(map.get(BODY));
        }
        if (map.containsKey(CID)) {
            event.setCorrelationId((String) map.get(CID));
        }
        if (map.containsKey(EXTRA)) {
            event.setExtra((String) map.get(EXTRA));
        }
        if (map.containsKey(EXEC_TIME)) {
            if (map.get(EXEC_TIME) instanceof Float) {
                event.setExecutionTime((Float) map.get(EXEC_TIME));
            } else {
                event.setExecutionTime(Utility.getInstance().str2float(map.get(EXEC_TIME).toString()));
            }
        }
        return event;
    }

    public static Map<String, Object> mapFromEvent(EventEnvelope event) {
        Map<String, Object> result = new HashMap<>();
        result.put(ID, event.getId());
        if (event.getTo() != null) {
            result.put(TO, event.getTo());
        }
        if (event.getFrom() != null) {
            result.put(FROM, event.getFrom());
        }
        if (event.hasError()) {
            result.put(STATUS, event.getStatus());
        }
        if (event.getReplyTo() != null) {
            result.put(REPLY_TO, event.getReplyTo());
        }
        if (event.getTraceId() != null) {
            result.put(TRACE_ID, event.getTraceId());
            result.put(TRACE_PATH, event.getTracePath());
        }
        result.put(HEADERS, event.getHeaders());
        if (event.getBody() != null) {
            result.put(BODY, event.getBody());
        }
        if (event.getCorrelationId() != null) {
            result.put(CID, event.getCorrelationId());
        }
        if (event.getExtra() != null) {
            result.put(EXTRA, event.getExtra());
        }
        if (event.getExecutionTime() >= 0) {
            result.put(EXEC_TIME, event.getExecutionTime());
        }
        return result;
    }

    private class ConnectionStatus {

        private String rxPath, txPath, token;
        private State state;
        private Date created;

        public ConnectionStatus(String rxPath, String txPath, String token) {
            this.created = new Date();
            this.state = State.OPEN;
            this.rxPath = rxPath;
            this.txPath = txPath;
            this.token = token;
        }

        public void setState(State state) {
            this.state = state;
        }

        public State getState() {
            return state;
        }

        public String getRxPath() {
            return rxPath;
        }

        public String getTxPath() {
            return txPath;
        }

        public String getToken() {
            return token;
        }

        public Date getCreated() {
            return created;
        }

    }

}