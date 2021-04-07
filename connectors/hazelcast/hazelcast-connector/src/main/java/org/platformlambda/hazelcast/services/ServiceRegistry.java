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

package org.platformlambda.hazelcast.services;

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.hazelcast.HazelcastSetup;
import org.platformlambda.hazelcast.reporter.PresenceConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class ServiceRegistry implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(ServiceRegistry.class);

    private static final String APP_GROUP = HazelcastSetup.APP_GROUP;
    private static final String NOTIFICATION_INTERNAL = "notification.manager.internal";
    private static final String PERSONALITY = "personality";
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String ROUTE = ServiceDiscovery.ROUTE;
    private static final String ORIGIN = ServiceDiscovery.ORIGIN;
    private static final String IS_FINAL = "final";
    private static final String TOPIC = "topic";
    private static final String UNREGISTER = ServiceDiscovery.UNREGISTER;
    private static final String ADD = ServiceDiscovery.ADD;
    private static final String EXCHANGE = "exchange";
    private static final String VERSION = "version";
    private static final String JOIN = "join";
    private static final String LEAVE = "leave";
    private static final String MONITOR = "monitor-";

    // static because this is a shared lambda function
    private final boolean presenceMonitor;
    private final int closedUserGroup;
    /*
     * routes: route_name -> (origin, personality)
     * origins: origin -> last seen
     * originTopic: origin -> topic and partition
     */
    private static final ConcurrentMap<String, ConcurrentMap<String, String>> routes = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, String> origins = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, String> originTopic = new ConcurrentHashMap<>();
    private static String monitorTopic;
    private long lastBroadcastAdd = 0;

    public ServiceRegistry() {
        Utility util = Utility.getInstance();
        AppConfigReader config = AppConfigReader.getInstance();
        presenceMonitor = "true".equals(config.getProperty("service.monitor", "false"));
        monitorTopic = config.getProperty("monitor.topic", "service.monitor");
        int maxGroups = Math.min(30,
                Math.max(3, util.str2int(config.getProperty("max.closed.user.groups", "30"))));
        closedUserGroup = util.str2int(config.getProperty("closed.user.group", "1"));
        if (closedUserGroup < 1 || closedUserGroup > maxGroups) {
            log.error("closed.user.group is invalid. Please select a number from 1 to "+maxGroups);
            System.exit(-1);
        }
    }

    public static Map<String, Map<String, String>> getAllRoutes() {
        return new HashMap<>(routes);
    }

    public static List<String> getInstances(String route) {
        if (routes.containsKey(route)) {
            return new ArrayList<>(routes.get(route).keySet());
        } else {
            return Collections.emptyList();
        }
    }

    public static Map<String, String> getAllOrigins() {
        Map<String, String> allOrigins = new HashMap<>(origins);
        for (String origin: allOrigins.keySet()) {
            String lastSeen = allOrigins.get(origin);
            String topic = originTopic.get(origin);
            if (topic != null) {
                allOrigins.put(origin, topic + ", "+lastSeen);
            }
        }
        return allOrigins;
    }

    public static ConcurrentMap<String, String> getDestinations(String route) {
        return routes.get(route);
    }

    public static boolean destinationExists(String origin) {
        if (origin == null) {
            return false;
        } else {
            return  origin.startsWith(MONITOR) || origin.equals(Platform.getInstance().getOrigin()) ||
                    origins.containsKey(origin);
        }
    }

    public static String getTopic(String dest) {
        return dest.startsWith(MONITOR)? monitorTopic+"-"+dest.substring(MONITOR.length()) : originTopic.get(dest);
    }

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        String myOrigin = platform.getOrigin();
        String type = headers.get(TYPE);
        // when a node joins
        if (JOIN.equals(type) && headers.containsKey(ORIGIN) && headers.containsKey(TOPIC)) {
            String origin = headers.get(ORIGIN);
            String topic = headers.get(TOPIC);
            origins.put(origin, Utility.getInstance().date2str(new Date(), true));
            originTopic.put(origin, topic);
            if (!presenceMonitor) {
                if (origin.equals(myOrigin)) {
                    if (headers.containsKey(VERSION)) {
                        log.info("Presence monitor v"+headers.get(VERSION)+" detected");
                    }
                    registerMyRoutes();
                    if (platform.hasRoute(NOTIFICATION_INTERNAL)) {
                        po.send(NOTIFICATION_INTERNAL, new Kv(TYPE, JOIN), new Kv(ORIGIN, origin));
                    }
                } else {
                    // send routing table of this node to the newly joined node
                    sendMyRoutes(true);
                }
            }
        }
        // when a node leaves
        if (LEAVE.equals(type) && headers.containsKey(ORIGIN)) {
            String origin = headers.get(ORIGIN);
            originTopic.remove(origin);
            if (presenceMonitor) {
                origins.remove(origin);
            } else {
                // remove corresponding entries from routing table
                if (origin.equals(platform.getOrigin())) {
                    // this happens when the service-monitor is down
                    List<String> all = new ArrayList<>(origins.keySet());
                    for (String o : all) {
                        if (!o.equals(origin)) {
                            log.info("{} disconnected", o);
                            removeRoutesFromOrigin(o);
                        }
                    }
                } else {
                    log.info("Peer {} left", origin);
                    removeRoutesFromOrigin(origin);
                }
                if (platform.hasRoute(NOTIFICATION_INTERNAL)) {
                    po.send(NOTIFICATION_INTERNAL, new Kv(TYPE, LEAVE), new Kv(ORIGIN, headers.get(ORIGIN)));
                }
            }
        }
        // add route
        if (ADD.equals(type) && headers.containsKey(ORIGIN)) {
            String origin = headers.get(ORIGIN);
            if (headers.containsKey(ROUTE) && headers.containsKey(PERSONALITY)) {
                // add a single route
                String route = headers.get(ROUTE);
                String personality = headers.get(PERSONALITY);
                // add to routing table
                addRoute(origin, route, personality);
                if (origin.equals(myOrigin) && !headers.containsKey(IS_FINAL)) {
                    // broadcast to peers
                    EventEnvelope request = new EventEnvelope();
                    request.setTo(ServiceDiscovery.SERVICE_REGISTRY + APP_GROUP + closedUserGroup)
                            .setHeaders(headers).setHeader(IS_FINAL, true);
                    po.send(request);
                }

            } else if (body instanceof Map) {
                if (!origin.equals(myOrigin)) {
                    // add a list of routes
                    Map<String, String> routeMap = (Map<String, String>) body;
                    int count = routeMap.size();
                    int n = 0;
                    for (String route : routeMap.keySet()) {
                        String personality = routeMap.get(route);
                        if (addRoute(origin, route, personality)) n++;
                    }
                    if (n > 0) {
                        log.info("Loaded {} route{} from {}", count, count == 1 ? "" : "s", origin);
                    }
                    if (headers.containsKey(TOPIC)) {
                        originTopic.put(origin, headers.get(TOPIC));
                    }
                    if (headers.containsKey(EXCHANGE)) {
                        sendMyRoutes(false);
                    }
                }
            }
        }
        // clear a route
        if (UNREGISTER.equals(type) && headers.containsKey(ROUTE) && headers.containsKey(ORIGIN)) {
            String route = headers.get(ROUTE);
            String origin = headers.get(ORIGIN);
            // remove from routing table
            removeRoute(origin, route);
            if (origin.equals(myOrigin) && !headers.containsKey(IS_FINAL)) {
                // broadcast to peers
                EventEnvelope request = new EventEnvelope();
                request.setTo(ServiceDiscovery.SERVICE_REGISTRY + APP_GROUP + closedUserGroup)
                        .setHeaders(headers).setHeader(IS_FINAL, true);
                po.send(request);
            }
        }
        return true;
    }

    private void sendMyRoutes(boolean exchange) throws IOException {
        long now = System.currentTimeMillis();
        if (!exchange && now - lastBroadcastAdd < 100) {
            log.debug("Duplicated broadcast add ignored");
            return;
        }
        lastBroadcastAdd = now;
        PostOffice po = PostOffice.getInstance();
        String myOrigin = Platform.getInstance().getOrigin();
        Map<String, String> routeMap = new HashMap<>();
        for (String r : routes.keySet()) {
            ConcurrentMap<String, String> originMap = routes.get(r);
            if (originMap.containsKey(myOrigin)) {
                routeMap.put(r, originMap.get(myOrigin));
            }
        }
        EventEnvelope request = new EventEnvelope();
        request.setTo(ServiceDiscovery.SERVICE_REGISTRY + APP_GROUP + closedUserGroup)
                .setHeader(TOPIC, PresenceConnector.getInstance().getTopic())
                .setHeader(TYPE, ADD).setHeader(ORIGIN, myOrigin).setBody(routeMap);
        if (exchange) {
            request.setHeader(EXCHANGE, true);
        }
        po.send(request);
    }

    private boolean addRoute(String origin, String route, String personality) {
        if (!routes.containsKey(route)) {
            routes.put(route, new ConcurrentHashMap<>());
        }
        ConcurrentMap<String, String> originMap = routes.get(route);
        if (originMap.containsKey(origin)) {
            return false;
        } else {
            originMap.put(origin, personality);
            origins.put(origin, Utility.getInstance().date2str(new Date(), true));
            log.info("{} {}.{} registered", route, personality, origin);
            return true;
        }
    }

    private void removeRoute(String origin, String route) {
        boolean deleted = false;
        if (routes.containsKey(route)) {
            ConcurrentMap<String, String> originMap = routes.get(route);
            if (originMap.containsKey(origin)) {
                originMap.remove(origin);
                deleted = true;
            }
            if (originMap.isEmpty()) {
                routes.remove(route);
            }
        }
        if (deleted) {
            log.info("{} {} unregistered", route, origin);
        }
    }

    private void removeRoutesFromOrigin(String origin) {
        List<String> routeList = new ArrayList<>(routes.keySet());
        for (String r: routeList) {
            removeRoute(origin, r);
        }
        origins.remove(origin);
        originTopic.remove(origin);
    }

    private void registerMyRoutes() {
        Platform platform = Platform.getInstance();
        String origin = platform.getOrigin();
        // copy local registry to global registry
        ConcurrentMap<String, ServiceDef> routingTable = platform.getLocalRoutingTable();
        String personality = ServerPersonality.getInstance().getType().toString();
        for (String r: routingTable.keySet()) {
            ServiceDef def = routingTable.get(r);
            if (!def.isPrivate()) {
                addRoute(origin, def.getRoute(), personality);
            }
        }
    }

}
