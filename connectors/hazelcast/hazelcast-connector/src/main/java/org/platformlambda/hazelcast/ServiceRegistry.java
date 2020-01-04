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

package org.platformlambda.hazelcast;

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
    private static final String MANAGER = HazelcastSetup.MANAGER;
    private static final String PERSONALITY = EventNodeConnector.PERSONALITY;
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String ROUTE = ServiceDiscovery.ROUTE;
    private static final String ORIGIN = ServiceDiscovery.ORIGIN;
    private static final String UNREGISTER = ServiceDiscovery.UNREGISTER;
    private static final String ADD = ServiceDiscovery.ADD;

    private static final String RESET = "reset";
    private static final String JOIN = "join";
    private static final String LEAVE = "leave";
    private static final String PING = "ping";
    private static final String PEERS = "peers";
    private static final String CHECKSUM = "checksum";
    private static final long APP_EXPIRY = 60 * 1000;
    // static because this is a shared lambda function
    private static boolean isServiceMonitor;
    /*
     * routes: route_name -> (origin, personality)
     * origins: origin -> last seen
     */
    private static final ManagedCache origins = ManagedCache.createCache("peer.origins", 10000);
    private static final ConcurrentMap<String, ConcurrentMap<String, String>> routes = new ConcurrentHashMap<>();
    private static final ManagedCache cache = ManagedCache.createCache("discovery.log.cache", 2000);
    private static List<String> peers = new ArrayList<>();
    private static String me;

    public ServiceRegistry() {
        AppConfigReader reader = AppConfigReader.getInstance();
        isServiceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
        me = Platform.getInstance().getOrigin();
    }

    public static Map<String, Map<String, String>> getAllRoutes() {
        return new HashMap<>(routes);
    }

    public static Set<String> getAllOrigins() {
        Set<String> result = new HashSet<>(peers);
        result.addAll(origins.getMap().keySet());
        return result;
    }

    public static ConcurrentMap<String, String> getDestinations(String route) {
        return routes.get(route);
    }

    public static boolean destinationExists(String origin) {
        return origin.equals(me) || origins.exists(origin) || peers.contains(origin);
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
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (isServiceMonitor) {
            // service monitor does not use global routing table
            return false;
        }
        String type = headers.get(TYPE);
        // peer events from presence monitor
        if (PEERS.equals(type) && body instanceof List) {
            PostOffice po = PostOffice.getInstance();
            String me = Platform.getInstance().getOrigin();
            List<String> updated = (List<String>) body;
            List<String> additions = getAdditions(updated);
            List<String> removal = getRemoval(updated);
            peers = updated;
            for (String p: peers) {
                origins.put(p, Utility.getInstance().date2str(new Date(), true));
            }
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
        return processEvent(headers);
    }

    private Object processEvent(Map<String, String> headers) throws IOException, TimeoutException, AppException {
        String type = headers.get(TYPE);
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
        // when a node leaves
        if (LEAVE.equals(type) && headers.containsKey(ORIGIN)) {
            // remove corresponding entries from routing table
            String origin = headers.get(ORIGIN);
            if (origin.equals(Platform.getInstance().getOrigin())) {
                // this happens when the service-monitor is down
                Set<String> all = getAllOrigins();
                for (String o : all) {
                    if (!o.equals(origin)) {
                        log.info("{} disconnected", o);
                        removeRoutesFromOrigin(o);
                    }
                }
                origins.clear();
                peers.clear();
                peers.add(origin);
                log.warn("Peer connections cleared");
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

    private void broadcast(String origin, String route, String personality, String type) throws IOException, TimeoutException, AppException {
        PostOffice po = PostOffice.getInstance();
        Utility util = Utility.getInstance();
        if (origin.equals(Platform.getInstance().getOrigin())) {
            // broadcast to peers
            List<String> peers = getPeers();
            for (String p : peers) {
                if (!p.equals(Platform.getInstance().getOrigin())) {
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

    private void addNode() throws IOException, TimeoutException, AppException {
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

    @SuppressWarnings("unchecked")
    private List<String> getPeers() throws IOException, TimeoutException, AppException {
        PostOffice po = PostOffice.getInstance();
        EventEnvelope response = po.request(MANAGER, 10000, new Kv(TYPE, TopicManager.LIST_TIMESTAMP));
        if (response.getBody() instanceof Map) {
            Map<String, String> peers = (Map<String, String>) response.getBody();
            List<String> result = new ArrayList<>();
            for (String p: peers.keySet()) {
                if (!isExpired(peers.get(p))) {
                    result.add(p);
                }
            }
            return result;

        } else {
            return Collections.EMPTY_LIST;
        }
    }

    private boolean isExpired(String isoTimestamp) {
        Date time = Utility.getInstance().str2date(isoTimestamp);
        return System.currentTimeMillis() - time.getTime() > APP_EXPIRY;
    }

}
