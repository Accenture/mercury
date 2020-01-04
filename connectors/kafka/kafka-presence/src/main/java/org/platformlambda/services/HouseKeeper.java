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
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.models.AppInfo;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;

public class HouseKeeper implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(HouseKeeper.class);

    private static final String MANAGER = MainApp.MANAGER;
    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;
    private static final String TYPE = "type";
    private static final String LIST = "list";
    private static final String DOWNLOAD = "download";
    private static final String TO = "to";
    private static final String ORIGIN = "origin";
    private static final String NAME = "name";
    private static final String MONITOR_ALIVE = KeepAlive.MONITOR_ALIVE;
    private static final String APP_ALIVE = "app_alive";
    private static final String LEAVE = "leave";
    private static final String STOP = "stop";
    private static final String TIMESTAMP = "timestamp";
    private static final String EVENT = "event";
    private static final String WS = "ws";
    private static final String TEMP = "?";
    private static final String RESTART = "restart";
    private static final long ONE_MINUTE = 60 * 1000;
    // Topic expiry is 60 seconds, deletion is 5 minutes
    private static final long EXPIRY = 5 * ONE_MINUTE;

    private static final ConcurrentMap<String, Long> monitors = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, AppInfo> topics = new ConcurrentHashMap<>();

    private long start;

    public HouseKeeper() {
        AppConfigReader reader = AppConfigReader.getInstance();
        start = System.currentTimeMillis() +
                        getStartTime(reader.getProperty("start.housekeeping", "5m"));
        log.info("Housekeeping will start at {}", Utility.getInstance().date2str(new Date(start), true));
    }

    private long getStartTime(String time) {
        if (time.length() > 1 && (time.endsWith("s") || time.endsWith("m") || time.endsWith("h"))) {
            long t = Utility.getInstance().str2long(time.substring(0, time.length()-1));
            if (t > 0) {
                if (time.endsWith("s")) {
                    return t * 1000;
                }
                if (time.endsWith("m")) {
                    return t * 60 * 1000;
                }
                if (time.endsWith("h")) {
                    return t * 60 * 60 * 1000;
                }
            }
        }
        log.error("start.housekeeping timer must be a number ends with s, m or h");
        return 0;
    }

    public static AppInfo getAppInfo(String origin) {
        return topics.get(origin);
    }

    public static Map<String, Date> getMonitors() {
        Map<String, Date> result = new HashMap<>();
        for (String m: monitors.keySet()) {
            result.put(m, new Date(monitors.get(m)));
        }
        return result;
    }

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (APP_ALIVE.equals(headers.get(TYPE))) {
            if (headers.containsKey(TIMESTAMP) && headers.containsKey(ORIGIN) && headers.containsKey(NAME)) {
                long timestamp = Utility.getInstance().timestamp2ms(headers.get(TIMESTAMP));
                if (System.currentTimeMillis() - timestamp < EXPIRY) {
                    topics.put(headers.get(ORIGIN), new AppInfo(headers.get(NAME), timestamp, EVENT));
                }
            }
        }
        if (MONITOR_ALIVE.equals(headers.get(TYPE))) {
            if (headers.containsKey(TIMESTAMP) && headers.containsKey(ORIGIN)) {
                String origin = headers.get(ORIGIN);
                String timestamp = headers.get(TIMESTAMP);
                long time = Utility.getInstance().timestamp2ms(timestamp);
                long now = System.currentTimeMillis();
                if (time > now) {
                    time = now;
                } else {
                    if (now - time > EXPIRY) {
                        return false;
                    }
                }
                String me = Platform.getInstance().getOrigin();
                if (!monitors.containsKey(origin)) {
                    log.info("Registered monitor {} {}", origin, me.equals(origin) ? "(me)" : "(peer)");
                }
                monitors.put(origin, time);
                removeExpiredMonitors();
                if (me.equals(origin)) {
                    log.debug("Found {} monitor{}", monitors.size(), monitors.size() == 1 ? "" : "s");
                    /*
                     * Skip signals from other presence monitor.
                     * Check only when it is my turn.
                     */
                    List<String> expired = findExpiredTopics();
                    if (now > start) {
                        boolean restartMyProducer = false, restartAppProducers = false;
                        String leader = getLeader(me);
                        boolean myTurn = leader.equals(me);
                        PostOffice po = PostOffice.getInstance();
                        for (String e : expired) {
                            restartMyProducer = true;
                            // delete the expired topic from Kafka
                            if (myTurn) {
                                restartAppProducers = true;
                                log.info("Removing expired topic {}", e);
                                po.send(MANAGER, new Kv(TYPE, LEAVE), new Kv(ORIGIN, e));
                            } else {
                                log.info("Detected expired topic {}", e);
                            }
                            // remove from memory
                            topics.remove(e);
                        }
                        if (restartAppProducers) {
                            List<String> peers = getAppPeers();
                            // and broadcast the restart event to all live application instances
                            for (String p : peers) {
                                po.send(ServiceDiscovery.SERVICE_REGISTRY + "@" + p, new Kv(TYPE, RESTART));
                            }
                        }
                        if (restartMyProducer) {
                            // when a topic is deleted, we should reset the producer and admin clients
                            po.send(CLOUD_CONNECTOR, new Kv(TYPE, STOP));
                            po.send(MANAGER, new Kv(TYPE, STOP));
                        }
                    }

                } else if (body instanceof List) {
                    // compare connection list of myself with a peer
                    Map<String, Object> connections = MonitorService.getConnections();
                    List<String> myConnections = new ArrayList<>(connections.keySet());
                    List<String> peerConnections = (List<String>) body;
                    if (!sameList(myConnections, peerConnections)) {
                        log.warn("Sync up because my list ({}) does not match peer ({})",
                                myConnections.size(), peerConnections.size());
                        // download current connections from peers
                        EventEnvelope event = new EventEnvelope();
                        event.setTo(org.platformlambda.MainApp.PRESENCE_HANDLER);
                        event.setHeader(TYPE, DOWNLOAD);
                        event.setHeader(ORIGIN, me);
                        PostOffice.getInstance().send(PostOffice.CLOUD_CONNECTOR, event.toBytes(), new Kv(TO, "*"));
                    }
                }
            }
        }
        return null;
    }

    private boolean sameList(List<String> a, List<String> b) {
        if (a.size() != b.size()) {
            return false;
        }
        if (a.size() > 1) {
            Collections.sort(a);
        }
        if (b.size() > 1) {
            Collections.sort(b);
        }
        return a.toString().equals(b.toString());
    }

    @SuppressWarnings("unchecked")
    private List<String> findExpiredTopics() {
        long now = System.currentTimeMillis();
        List<String> expired = new ArrayList<>();
        Map<String, Object> connections = MonitorService.getConnections();
        for (String key: connections.keySet()) {
            Object o = connections.get(key);
            if (o instanceof Map) {
                Map<String, Object> info = (Map<String, Object>) o;
                if (info.containsKey(NAME)) {
                    topics.put(key, new AppInfo(info.get(NAME).toString(), now, WS));
                }
            }
        }
        try {
            List<String> registered = getTopics();
            for (String t: registered) {
                if (!topics.containsKey(t)) {
                    // this gives an unknown topic to wait for a complete alive cycle
                    topics.put(t, new AppInfo("Unknown", now, TEMP));
                }
            }
            for (String k: topics.keySet()) {
                AppInfo appInfo = topics.get(k);
                if (now - appInfo.lastSeen > EXPIRY) {
                    expired.add(k);
                }
            }
        } catch (TimeoutException | IOException | AppException e) {
            log.error("Unable to scan for expired topics - {}", e.getMessage());
        }
        return expired;
    }

    private void removeExpiredMonitors() {
        long now = System.currentTimeMillis();
        List<String> expired = new ArrayList<>();
        for (String k: monitors.keySet()) {
            long time = monitors.get(k);
            if (now - time > EXPIRY) {
                expired.add(k);
            }
        }
        if (!expired.isEmpty()) {
            for (String k: expired) {
                monitors.remove(k);
                log.info("Removed monitor {}", k);
            }
        }
    }

    private String getLeader(String me) {
        // the smallest origin ID wins
        List<String> list = new ArrayList<>(monitors.keySet());
        if (list.size() > 1) {
            Collections.sort(list);
            return list.get(0);
        } else {
            return me;
        }
    }

    @SuppressWarnings("unchecked")
    private List<String> getTopics() throws TimeoutException, IOException, AppException {
        PostOffice po = PostOffice.getInstance();
        EventEnvelope res1 = po.request(MANAGER, 20000, new Kv(TYPE, LIST));
        return res1.getBody() instanceof List? (List<String>) res1.getBody() : new ArrayList<>();
    }

    @SuppressWarnings("unchecked")
    private List<String> getAppPeers() throws IOException, TimeoutException, AppException {
        List<String> peers = new ArrayList<>();
        PostOffice po = PostOffice.getInstance();
        // check topic list
        EventEnvelope list = po.request(MANAGER, 20000, new Kv(TYPE, LIST));
        if (list.getBody() instanceof List) {
            List<String> eventTopics = (List<String>) list.getBody();
            if (!eventTopics.isEmpty()) {
                long now = System.currentTimeMillis();
                for (String t : eventTopics) {
                    // check if topic is active
                    AppInfo info = getAppInfo(t);
                    if (info != null) {
                        if (now - info.lastSeen < EXPIRY) {
                            peers.add(t);
                        }
                    }
                }
            }
        }
        return peers;
    }

}
