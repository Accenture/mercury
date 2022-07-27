/*

    Copyright 2018-2022 Accenture Technology

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
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.SimpleCache;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.platformlambda.lang.services.*;
import org.platformlambda.websocket.WsConfigurator;
import org.platformlambda.websocket.WsEnvelope;
import org.platformlambda.websocket.WsTransmitter;
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
    private static final String STREAM_MANAGER = "object.streams.io";
    private static final String CREATE_STREAM = "create_stream";
    private static final String IN = "in";
    private static final String OUT = "out";
    private static final String EXPIRY = "expiry";
    private static final String ID = "id";
    private static final String TO = "to";
    private static final String FROM = "from";
    private static final String HEADERS = "headers";
    private static final String BODY = "body";
    private static final String STATUS = "status";
    private static final String REPLY_TO = "reply_to";
    private static final String CID = "cid";
    private static final String EXTRA = "extra";
    private static final String ROUTING = "routing";
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
    private static final String SYSTEM_ALERT = "system.alerts";
    private static final String SYSTEM_CONFIG = "system.config";
    private static final String MAX_PAYLOAD = "max.payload";

    private static final String TRACE_AGGREGATION = "trace.aggregation";

    private static final String MSG_ID = MultipartPayload.ID;
    private static final String COUNT = MultipartPayload.COUNT;
    private static final String TOTAL = MultipartPayload.TOTAL;
    private static final int OVERHEAD = MultipartPayload.OVERHEAD;
    private static final int MAX_PAYLOAD_SIZE = WsConfigurator.getInstance().getMaxBinaryPayload() - OVERHEAD;

    private static final SimpleCache cache = SimpleCache.createCache("payload.segmentation", 60000);
    private static String apiKey;
    private static String inboxRoute;

    private enum State {
        OPEN, AUTHENTICATED
    }
    // token -> ConnectionStatus
    private static final ConcurrentMap<String, ConnectionStatus> connections = new ConcurrentHashMap<>();
    // route -> token
    private static final ConcurrentMap<String, List<String>> routingTable = new ConcurrentHashMap<>();
    // rxPath -> token
    private static final ConcurrentMap<String, String> tokens = new ConcurrentHashMap<>();

    private static boolean aggregation;

    public LanguageConnector() {
        AppConfigReader config = AppConfigReader.getInstance();
        aggregation = "true".equalsIgnoreCase(config.getProperty("distributed.trace.aggregation", "true"));
    }

    public static String getTxPathFromToken(String token) {
        if (token == null || token.length() == 0) {
            return null;
        }
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
        EventEnvelope alert = new EventEnvelope();
        alert.setTo(SYSTEM_ALERT);
        alert.setHeader(STATUS, status);
        alert.setStatus(status);
        alert.setBody(message);
        response.put(EVENT, msgPack.pack(LanguageConnector.mapFromEvent(alert)));
        PostOffice.getInstance().send(txPath, msgPack.pack(response));
    }

    private static void sendServerConfig(String txPath, Map<String, Object> config) throws IOException {
        Map<String, Object> response = new HashMap<>();
        response.put(TYPE, EVENT);
        EventEnvelope alert = new EventEnvelope();
        alert.setTo(SYSTEM_CONFIG);
        alert.setHeader(TYPE, SYSTEM_CONFIG);
        alert.setBody(config);
        response.put(EVENT, msgPack.pack(LanguageConnector.mapFromEvent(alert)));
        PostOffice.getInstance().send(txPath, msgPack.pack(response));
    }

    private static void sendReady(String txPath) throws IOException {
        Map<String, Object> response = new HashMap<>();
        response.put(TYPE, EVENT);
        EventEnvelope alert = new EventEnvelope();
        alert.setTo(SYSTEM_CONFIG);
        alert.setHeader(TYPE, READY);
        response.put(EVENT, msgPack.pack(LanguageConnector.mapFromEvent(alert)));
        PostOffice.getInstance().send(txPath, msgPack.pack(response));
    }

    private static String getApiKey() {
        Utility util = Utility.getInstance();
        AppConfigReader reader = AppConfigReader.getInstance();
        String apiKeyInEnv = reader.getProperty("language.pack.key");
        if (apiKeyInEnv != null) {
            return apiKeyInEnv;
        }
        File tempConfigDir = new File("/tmp/config");
        if (!tempConfigDir.exists()) {
            tempConfigDir.mkdirs();
        }
        File apiKeyFile = new File(tempConfigDir, "lang-api-key.txt");
        if (apiKeyFile.exists()) {
            log.info("Reading language API key from {}", apiKeyFile);
            return util.file2str(apiKeyFile).trim();
        } else {
            log.warn("Generating new language API key in {}", apiKeyFile);
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
            LanguageConnector.inboxRoute = LANGUAGE_INBOX + "@" + platform.getOrigin();
            // setup stream manager
            Utility util = Utility.getInstance();
            LambdaFunction streamManager = (headers, body, instance) -> {
                if (CREATE_STREAM.equals(headers.get(TYPE)) && headers.containsKey(EXPIRY)) {
                    int expiry = Math.max(1, util.str2int(headers.get(EXPIRY)));
                    ObjectStreamIO stream = new ObjectStreamIO(expiry);
                    Map<String, Object> result = new HashMap<>();
                    result.put(IN, stream.getInputStreamId());
                    result.put(OUT, stream.getOutputStreamId());
                    result.put(EXPIRY, expiry);
                    return result;
                }
                return null;
            };
            platform.registerPrivate(STREAM_MANAGER, streamManager, 1);
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
                                        config.put(MAX_PAYLOAD, MAX_PAYLOAD_SIZE);
                                        config.put(TRACE_AGGREGATION, aggregation);
                                        sendServerConfig(txPath, config);
                                        log.info("{} authenticated", token);
                                    } else {
                                        // txPath, CloseReason.CloseCodes status, String message
                                        closeConnection(txPath, CloseReason.CloseCodes.CANNOT_ACCEPT,
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
                                        if (control.size() == 3 && control.containsKey(MSG_ID)
                                                && control.containsKey(COUNT) && control.containsKey(TOTAL)) {
                                            Utility util = Utility.getInstance();
                                            String msgId = control.get(MSG_ID);
                                            int count = util.str2int(control.get(COUNT));
                                            int total = util.str2int(control.get(TOTAL));
                                            byte[] data = (byte[]) block.getBody();
                                            if (count != -1 && total != -1) {
                                                ByteArrayOutputStream buffer = (ByteArrayOutputStream) cache.get(msgId);
                                                if (count == 1 || buffer == null) {
                                                    buffer = new ByteArrayOutputStream();
                                                }
                                                buffer.write(data);
                                                if (count == total) {
                                                    cache.remove(msgId);
                                                    byte[] b = buffer.toByteArray();
                                                    Map<String, Object> m = (Map<String, Object>) msgPack.unpack(b);
                                                    relayEvent(mapReplyTo(token, eventFromMap(m)));
                                                } else {
                                                    cache.put(msgId, buffer);
                                                }
                                            }
                                        }
                                    }
                                    // regular events
                                    if (EVENT.equals(type) && event.containsKey(EVENT)) {
                                        Object inner = event.get(EVENT);
                                        if (inner instanceof byte[]) {
                                            byte[] b = (byte[]) inner;
                                            EventEnvelope data = eventFromMap((Map<String, Object>) msgPack.unpack(b));
                                            relayEvent(mapReplyTo(token, data));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    break;
                case WsEnvelope.STRING:
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

    private void relayEvent(EventEnvelope relay) throws IOException {
        PostOffice po = PostOffice.getInstance();
        String target = relay.getTo();
        try {
            po.send(relay);
        } catch (IOException e) {
            String replyTo = relay.getReplyTo();
            if (replyTo != null) {
                relay.setTo(replyTo).setReplyTo(null)
                        .setBody("Route "+target+" not found")
                        .addTag("exception").setStatus(404);
                po.send(relay);
            } else {
                log.warn("Unable to relay event - Route {} not found", target);
            }
        }
    }

    private EventEnvelope mapReplyTo(String token, EventEnvelope request) {
        if (request.getReplyTo() != null) {
            String replyTo = request.getReplyTo();
            if (replyTo.startsWith("->")) {
                /*
                 * Encode the client token ID with reply to so that
                 * the language inbox can relay the service response correctly
                 */
                request.addTag(ROUTING, token + replyTo);
                request.setReplyTo(inboxRoute);
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
            // ensure key-values are strings
            Map<Object, Object> headers = (Map<Object, Object>) map.get(HEADERS);
            for (Object key: headers.keySet()) {
                event.setHeader(key.toString(), headers.get(key) == null? "" : headers.get(key).toString());
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

    public static void closeConnection(String txPath, CloseReason.CloseCodes status, String message) throws IOException {
        if (txPath != null && status != null && message != null) {
            EventEnvelope error = new EventEnvelope();
            error.setTo(txPath);
            error.setHeader(WsTransmitter.STATUS, String.valueOf(status.getCode()));
            error.setHeader(WsTransmitter.MESSAGE, message);
            error.setHeader(WsEnvelope.TYPE, WsEnvelope.CLOSE);
            PostOffice.getInstance().send(error);
        }
    }

    private static class ConnectionStatus {

        private State state = State.OPEN;
        private final String rxPath;
        private final String txPath;
        private final String token;
        private final Date created;

        public ConnectionStatus(String rxPath, String txPath, String token) {
            this.created = new Date();
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