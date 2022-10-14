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

package org.platformlambda.mock;

import org.platformlambda.cloud.ConnectorConfig;
import org.platformlambda.cloud.services.ServiceRegistry;
import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

@WebSocketService("presence")
public class MockPresenceMonitor implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(MockPresenceMonitor.class);

    private static final PubSub ps = PubSub.getInstance();
    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;
    private static final String APP_GROUP = ServiceRegistry.APP_GROUP;
    private static final String TYPE = "type";
    private static final String STATUS = "status";
    private static final String MESSAGE = "message";
    private static final String OPEN = "open";
    private static final String CLOSE = "close";
    private static final String BYTES = "bytes";
    private static final String STRING = "string";
    private static final String ROUTE = "route";
    private static final String TX_PATH = "tx_path";
    private static final String TOKEN = "token";
    private static final String IP = "ip";
    private static final String ALIVE = "keep-alive";
    private static final String INFO = "info";
    private static final String LEAVE = "leave";
    private static final String ORIGIN = "origin";
    private static final String ID = "id";
    private static final String SEQ = "seq";
    private static final String GROUP = "group";
    private static final String CREATED = "created";
    private static final String UPDATED = "updated";
    private static final String MONITOR = "monitor";
    private static final String TOPIC = "topic";
    private static final String VERSION = "version";
    private static final String JOIN = "join";
    private static final String READY = "ready";
    private static final int TRY_AGAIN_LATER = 1013;
    private static final long EXPIRY = 60 * 1000;
    private static final String AVAILABLE = "*";
    // topic+partition -> origin | AVAILABLE(*)
    private static final ConcurrentMap<String, String> topicStore = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, Long> activeTopics = new ConcurrentHashMap<>();
    private static List<String> allTopics;
    private final int partitionCount, maxVirtualTopics;
    private final boolean topicSubstitution;
    private final Map<String, String> preAllocatedTopics;

    // websocket route to user application origin-ID. Websocket routes for this presence monitor instance only
    private static final ConcurrentMap<String, String> route2origin = new ConcurrentHashMap<>();
    // connection list of user applications to this presence monitor instance
    private static final ConcurrentMap<String, Map<String, Object>> myConnections = new ConcurrentHashMap<>();
    // user application connections for the whole system
    private static final ManagedCache connectionInfo = ManagedCache.createCache("app.presence.list", EXPIRY);


    public MockPresenceMonitor() throws IOException {
        topicSubstitution = ConnectorConfig.topicSubstitutionEnabled();
        preAllocatedTopics = ConnectorConfig.getTopicSubstitution();
        Utility util = Utility.getInstance();
        AppConfigReader config = AppConfigReader.getInstance();
        String prefix = config.getProperty("app.topic.prefix", "multiplex");
        String topicPrefix = prefix.endsWith(".")? prefix : prefix + ".";
        partitionCount = Math.max(1, util.str2int(config.getProperty("app.partitions.per.topic", "32")));
        int tc = Math.max(1, util.str2int(config.getProperty("max.virtual.topics", "288")));
        if (tc % partitionCount > 0) {
            maxVirtualTopics = (tc / partitionCount) * partitionCount;
            log.warn("max.virtual.topics {} should be a multiple of partitions {} - reset to {}",
                    tc, partitionCount, maxVirtualTopics);
        } else {
            maxVirtualTopics = tc;
        }
        log.info("Topic prefix {}, partition count {}, max virtual topics {}",
                prefix, partitionCount, maxVirtualTopics);
        // prepare topic store
        int maxPhysicalTopics = maxVirtualTopics / partitionCount;
        for (int n=1; n <= maxPhysicalTopics; n++) {
            String topic = topicPrefix+util.zeroFill(n, 1000);
            for (int i=0; i < partitionCount; i++) {
                String topicPartition = topic+"-"+util.zeroFill(i, 100);
                topicStore.put(topicPartition, AVAILABLE);
            }
        }
        if (allTopics == null) {
            allTopics = new ArrayList<>(topicStore.keySet());
            Collections.sort(allTopics);
        }
    }

    public static Map<String, Object> getConnections() {
        return new HashMap<>(connectionInfo.getMap());
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
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        String route, appOrigin, txPath;
        if (headers.containsKey(TYPE)) {
            switch (headers.get(TYPE)) {
                case OPEN:
                    // the open event contains route, txPath, ip, path, query and token
                    route = headers.get(ROUTE);
                    txPath = headers.get(TX_PATH);
                    appOrigin = headers.get(TOKEN);
                    String ip = headers.get(IP);
                    log.info("Started {}, {}, {}", route, ip, appOrigin);
                    // check if dependencies are ready
                    if (platform.hasRoute(CLOUD_CONNECTOR)) {
                        Map<String, Object> info = new HashMap<>();
                        String time = util.date2str(new Date(), true);
                        info.put(CREATED, time);
                        info.put(UPDATED, time);
                        info.put(MONITOR, platform.getOrigin());
                        info.put(ID, route);
                        info.put(SEQ, 0);
                        route2origin.put(route, appOrigin);
                        myConnections.put(appOrigin, info);

                    } else {
                        EventEnvelope error = new EventEnvelope();
                        error.setTo(txPath);
                        error.setHeader(STATUS, TRY_AGAIN_LATER);
                        error.setHeader(MESSAGE, "Starting up");
                        error.setHeader(TYPE, CLOSE);
                        po.send(error);
                    }
                    break;
                case CLOSE:
                    // the close event contains only the route for this websocket
                    route = headers.get(ROUTE);
                    appOrigin = headers.get(TOKEN);
                    route2origin.remove(route);
                    myConnections.remove(appOrigin);
                    log.info("Stopped {}, {}", route, appOrigin);
                    if (connectionInfo.exists(appOrigin)) {
                        Object o = connectionInfo.get(appOrigin);
                        if (o instanceof Map) {
                            Map<String, Object> info = (Map<String, Object>) o;
                            if (route.equals(info.get(ID)) && info.get(GROUP) instanceof Integer) {
                                // tell all nodes to drop this node
                                leaveGroup(appOrigin, (int) info.get(GROUP));
                            }
                        }
                    }
                    break;
                case BYTES:
                    // the data event for byteArray payload contains route and txPath
                    route = headers.get(ROUTE);
                    txPath = headers.get(TX_PATH);
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
                                if (register) {
                                    // tell the connected application instance to proceed
                                    log.info("Member registered {}", getMemberInfo(appOrigin, info));
                                    // check if appOrigin has a topic in store
                                    String topicPartition = nextTopic(appOrigin);
                                    int hyphen = topicPartition.lastIndexOf('-');
                                    String topic = topicPartition.substring(0, hyphen);
                                    if (topicSubstitution) {
                                        int partition = util.str2int(topicPartition.substring(hyphen+1));
                                        String virtualTopic = topic + "." + partition;
                                        if (!preAllocatedTopics.containsKey(virtualTopic)) {
                                            throw new IOException("Missing topic substitution for "+virtualTopic);
                                        }
                                    }
                                    if (!topicSubstitution) {
                                        // automatically create topic if not exist
                                        if (ps.exists(topic)) {
                                            int actualPartitions = ps.partitionCount(topic);
                                            if (actualPartitions < partitionCount) {
                                                log.error("Insufficient partitions in {}, Expected: {}, Actual: {}",
                                                        topic, partitionCount, actualPartitions);
                                                log.error("SYSTEM NOT OPERATIONAL. Please setup topic {} and restart",
                                                        topic);
                                                throw new IOException("Insufficient partitions in " + topic);
                                            }
                                        } else {
                                            ps.createTopic(topic, partitionCount);
                                        }
                                    }
                                    po.send(txPath, new EventEnvelope().setTo(READY)
                                            .setHeader(TOPIC, topicPartition)
                                            .setHeader(VERSION, util.getVersionInfo().getVersion()).toBytes());
                                    po.send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, JOIN),
                                            new Kv(ORIGIN, appOrigin), new Kv(TOPIC, topicPartition));
                                } else {
                                    log.debug("Member {} is alive {}", appOrigin, info.get(SEQ));
                                }
                            }
                        }

                    }
                    break;
                case STRING:
                    log.debug("{}", body);
                    break;
                default:
                    break;
            }
        }
        // nothing to return because this is asynchronous
        return null;
    }

    private String nextTopic(String appOrigin) throws IOException {
        for (String t: allTopics) {
            String value = topicStore.get(t);
            if (value.equals(AVAILABLE)) {
                topicStore.put(t, appOrigin);
                return t;
            }
        }
        throw new IOException("All virtual topics are used");
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