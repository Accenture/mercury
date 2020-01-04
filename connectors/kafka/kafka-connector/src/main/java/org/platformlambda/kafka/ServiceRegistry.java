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

package org.platformlambda.kafka;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;

public class ServiceRegistry implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(ServiceRegistry.class);

    private static final CryptoApi crypto = new CryptoApi();
    private static final String MANAGER = KafkaSetup.MANAGER;
    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;
    private static final String PERSONALITY = EventNodeConnector.PERSONALITY;
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String ROUTE = ServiceDiscovery.ROUTE;
    private static final String ORIGIN = ServiceDiscovery.ORIGIN;
    private static final String UNREGISTER = ServiceDiscovery.UNREGISTER;
    private static final String ADD = ServiceDiscovery.ADD;
    private static final String PEERS = "peers";
    private static final String INIT = "init";
    private static final String JOIN = "join";
    private static final String LEAVE = "leave";
    private static final String RESTART = "restart";
    private static final String READY = "ready";
    private static final String CHECKSUM = "checksum";
    private static final String PING = "ping";
    private static final String STOP = "stop";
    // static because this is a shared lambda function
    private static boolean isServiceMonitor = false, ready = false;
    private static List<Map<String, String>> deferred = new ArrayList<>();
    /*
     * routes: route_name -> (origin, personality)
     * origins: origin -> last seen
     */
    private static final ConcurrentMap<String, ConcurrentMap<String, String>> routes = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, String> origins = new ConcurrentHashMap<>();
    private static final ManagedCache cache = ManagedCache.createCache("discovery.log.cache", 2000);
    private static List<String> peers = new ArrayList<>();

    public ServiceRegistry() {
        AppConfigReader reader = AppConfigReader.getInstance();
        if ("true".equals(reader.getProperty("service.monitor", "false"))) {
            isServiceMonitor = true;
            // service monitor does not have a kafka consumer so there is no need to check for readiness
            ready = true;
        }
    }

    public static Map<String, Map<String, String>> getAllRoutes() {
        return new HashMap<>(routes);
    }

    public static Map<String, String> getAllOrigins() {
        return new HashMap<>(origins);
    }

    public static ConcurrentMap<String, String> getDestinations(String route) {
        return routes.get(route);
    }

    public static boolean destinationExists(String origin) {
        if (origins.containsKey(origin)) {
            return true;
        } else {
            return peers.contains(origin);
        }
    }

    private String getChecksum() {
        StringBuilder sb = new StringBuilder();
        List<String> keys = new ArrayList<>(routes.keySet());
        if (keys.size() > 1) {
            Collections.sort(keys);
        }
        sb.append("*");
        for (String r: keys) {
            sb.append(r);
            sb.append(':');
            Map<String, String> instances = routes.get(r);
            List<String> innerKeys = new ArrayList<>(instances.keySet());
            if (innerKeys.size() > 1) {
                Collections.sort(innerKeys);
            }
            for (String i: innerKeys) {
                String v = instances.get(i);
                sb.append(i);
                sb.append('-');
                sb.append(v);
                sb.append('\n');
            }
        }
        Utility util = Utility.getInstance();
        return util.bytesToUrlBase64(crypto.getSHA1(util.getUTF(sb.toString())));
    }

    private List<String> getAdditions(List<String> updated) {
        List<String> additions = new ArrayList<>();
        for (String member: updated) {
            if (!peers.contains(member)) {
                additions.add(member);
            }
        }
        return additions;
    }

    private List<String> getRemoval(List<String> updated) {
        List<String> removal = new ArrayList<>();
        for (String member: peers) {
            if (!updated.contains(member)) {
                removal.add(member);
            }
        }
        return removal;
    }

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException, TimeoutException, AppException {
        if (isServiceMonitor) {
            // service monitor does not use global routing table
            return false;
        }
        String type = headers.get(TYPE);
        if (READY.equals(type)) {
            return ready;
        }
        // peer events from presence monitor
        if (PEERS.equals(type) && body instanceof List) {
            PostOffice po = PostOffice.getInstance();
            String me = Platform.getInstance().getOrigin();
            List<String> updated = (List<String>) body;
            List<String> additions = getAdditions(updated);
            List<String> removal = getRemoval(updated);
            peers = updated;
            List<String> members = new ArrayList<>(peers);
            members.remove(me);
            if (members.isEmpty()) {
                log.info("No peers are detected. I am running alone.");
            } else {
                log.info("Found {} peer{}", members.size(), members.size() == 1? "" : "s");
            }
            if (!additions.isEmpty()) {
                for (String member: additions) {
                    if (!member.equals(me)) {
                        po.send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, JOIN), new Kv(ORIGIN, member));
                    }
                }
            }
            if (!removal.isEmpty()) {
                for (String member: removal) {
                    if (!member.equals(me)) {
                        po.send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, LEAVE), new Kv(ORIGIN, member));
                    }
                }
            }
        }
        if (!ready) {
            // detect if kafka consumer has received the first event
            if (INIT.equals(type)) {
                ready = true;
                log.info("Event streams are ready");
                if (!deferred.isEmpty()) {
                    for (Map<String, String> request: deferred) {
                        processEvent(request);
                    }
                    deferred.clear();
                }
                return true;
            }
            // defer processing
            deferred.add(headers);
            return false;
        } else {
            // ignore all subsequent INIT request
            if (INIT.equals(type)) {
                return true;
            }
        }
        return processEvent(headers);
    }

    private Object processEvent(Map<String, String> headers) throws IOException {
        PostOffice po = PostOffice.getInstance();
        String type = headers.get(TYPE);
        if (PING.equals(type)) {
            broadcastChecksum(getChecksum());
            log.info("Routing table integrity check");
            return true;
        }
        if (CHECKSUM.equals(type) && headers.containsKey(CHECKSUM) && headers.containsKey(ORIGIN)) {
            String myOrigin = Platform.getInstance().getOrigin();
            String origin = headers.get(ORIGIN);
            String extChecksum = headers.get(CHECKSUM);
            String myChecksum = getChecksum();
            if (!origin.equals(myOrigin)) {
                if (extChecksum.equals(myChecksum)) {
                    log.debug("Routing table matches with {}, checksum {}", origin, extChecksum);
                } else {
                    log.warn("Routing table checksum not matched. Syncing with {}", origin);
                    sendMyRoutes(origin);
                }
            }
        }
        // when a node joins
        if (JOIN.equals(type) && headers.containsKey(ORIGIN)) {
            String myOrigin = Platform.getInstance().getOrigin();
            String origin = headers.get(ORIGIN);
            origins.put(origin, Utility.getInstance().date2str(new Date(), true));
            if (origin.equals(myOrigin)) {
                addNode();
                broadcast(origin, null, null, JOIN);
            } else {
                // send routing table of this node to the newly joined node
                String key = "join/"+origin;
                if (!cache.exists(key)) {
                    cache.put(key, true);
                    log.info("Peer {} joined", origin);
                }
                sendMyRoutes(origin);
            }
        }
        if (RESTART.equals(type)) {
            // just restart
            log.info("Restarting event producer");
            po.send(CLOUD_CONNECTOR, new Kv(TYPE, STOP));
            po.send(MANAGER, new Kv(TYPE, STOP));
        }
        // when a node leaves
        if (LEAVE.equals(type) && headers.containsKey(ORIGIN)) {
            // restart kafka producer when a node leaves
            po.send(CLOUD_CONNECTOR, new Kv(TYPE, STOP));
            po.send(MANAGER, new Kv(TYPE, STOP));
            // remove corresponding entries from routing table
            String origin = headers.get(ORIGIN);
            if (origin.equals(Platform.getInstance().getOrigin())) {
                // this happens when the service-monitor is down
                List<String> all = new ArrayList<>(origins.keySet());
                for (String o : all) {
                    if (!o.equals(origin)) {
                        log.info("{} disconnected", o);
                        removeRoutesFromOrigin(o);
                    }
                }
            } else {
                String key = "leave/"+origin;
                if (!cache.exists(key)) {
                    cache.put(key, true);
                    log.info("Peer {} left", origin);
                }
                removeRoutesFromOrigin(origin);
            }
        }
        // add a route
        if (ADD.equals(type) && headers.containsKey(ROUTE)
                && headers.containsKey(ORIGIN) && headers.containsKey(PERSONALITY)) {
            String route = headers.get(ROUTE);
            String origin = headers.get(ORIGIN);
            String personality = headers.get(PERSONALITY);
            if (origin.equals(Platform.getInstance().getOrigin())) {
                broadcast(origin, route, personality, ADD);
            }
            // add to routing table
            addRoute(origin, route, personality);
        }
        // clear a route
        if (UNREGISTER.equals(type) && headers.containsKey(ROUTE) && headers.containsKey(ORIGIN)) {
            String route = headers.get(ROUTE);
            String origin = headers.get(ORIGIN);
            if (origin.equals(Platform.getInstance().getOrigin())) {
                broadcast(origin, route, null, UNREGISTER);
            }
            // remove from routing table
            removeRoute(origin, route);
        }
        return true;
    }

    private void sendMyRoutes(String origin) throws IOException {
        PostOffice po = PostOffice.getInstance();
        String myOrigin = Platform.getInstance().getOrigin();
        for (String r : routes.keySet()) {
            ConcurrentMap<String, String> originMap = routes.get(r);
            if (originMap.containsKey(myOrigin)) {
                String personality = originMap.get(myOrigin);
                EventEnvelope request = new EventEnvelope();
                request.setTo(ServiceDiscovery.SERVICE_REGISTRY + "@" + origin)
                        .setHeader(TYPE, ADD)
                        .setHeader(ORIGIN, myOrigin)
                        .setHeader(ROUTE, r).setHeader(PERSONALITY, personality);
                po.send(request);
            }
        }
    }

    private void broadcast(String origin, String route, String personality, String type) throws IOException {
        PostOffice po = PostOffice.getInstance();
        Utility util = Utility.getInstance();
        String myOrigin = Platform.getInstance().getOrigin();
        if (origin.equals(myOrigin)) {
            // broadcast to peers
            for (String p : peers) {
                if (!p.equals(myOrigin)) {
                    EventEnvelope request = new EventEnvelope();
                    request.setTo(ServiceDiscovery.SERVICE_REGISTRY + "@" + p)
                            .setHeader(TYPE, type).setHeader(ORIGIN, origin);
                    if (route != null) {
                        request.setHeader(ROUTE, route);
                    }
                    if (personality != null) {
                        request.setHeader(PERSONALITY, personality);
                    }
                    origins.put(p, util.date2str(new Date(), true));
                    po.send(request);
                }
            }
        }
    }

    private void broadcastChecksum(String checkSum) throws IOException {
        PostOffice po = PostOffice.getInstance();
        String myOrigin = Platform.getInstance().getOrigin();
        // broadcast to peers
        for (String p : peers) {
            if (!p.equals(myOrigin)) {
                EventEnvelope request = new EventEnvelope();
                request.setTo(ServiceDiscovery.SERVICE_REGISTRY + "@" + p).setHeader(ORIGIN, myOrigin)
                        .setHeader(TYPE, CHECKSUM).setHeader(CHECKSUM, checkSum);
                po.send(request);
            }
        }
    }

    private void addRoute(String origin, String route, String personality) {
        if (!routes.containsKey(route)) {
            routes.put(route, new ConcurrentHashMap<>());
        }
        ConcurrentMap<String, String> originMap = routes.get(route);
        if (!originMap.containsKey(origin)) {
            originMap.put(origin, personality);
            origins.put(origin, Utility.getInstance().date2str(new Date(), true));
            log.info("{} {}.{} registered", route, personality, origin);
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
    }

    private void addNode() throws IOException {
        Platform platform = Platform.getInstance();
        String origin = platform.getOrigin();
        // copy local registry to global registry
        ConcurrentMap<String, ServiceDef> routingTable = platform.getLocalRoutingTable();
        String personality = ServerPersonality.getInstance().getType().toString();
        for (String r: routingTable.keySet()) {
            ServiceDef def = routingTable.get(r);
            if (!def.isPrivate()) {
                addRoute(origin, def.getRoute(), personality);
                broadcast(origin, def.getRoute(), personality, ADD);
            }
        }
    }

}
