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

package org.platformlambda.services;

import org.platformlambda.MainApp;
import org.platformlambda.cloud.ConnectorConfig;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.models.PendingRsvp;
import org.platformlambda.ws.MonitorService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class TopicController implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(TopicController.class);

    private static final String MONITOR_PARTITION = MainApp.MONITOR_PARTITION;
    private static final String TYPE = "type";
    private static final String ORIGIN = "origin";
    private static final String NAME = "name";
    private static final String MONITOR = "monitor";
    private static final String ALIVE = "keep-alive";
    private static final String JOIN = "join";
    private static final String READY = "ready";
    private static final String VERSION = "version";
    private static final String GET_TOPIC = "get_topic";
    private static final String TX_PATH = "tx_path";
    private static final String RSVP = "rsvp";
    private static final String RSVP_TIME = "rsvp_time";
    private static final String RSVP_START = "rsvp_start";
    private static final String RSVP_COMPLETE = "rsvp_complete";
    private static final String CONFIRM_TOPIC = "confirm_topic";
    private static final String RELEASE_TOPIC = "release_topic";
    private static final String TOPIC = "topic";
    private static final String AVAILABLE = "*";
    private static final int TRY_AGAIN_LATER = 1013;
    private static final long INTERVAL = 5 * 1000L;
    private static final long EXPIRY = 60 * 1000L;
    // topic+partition -> origin | AVAILABLE(*)
    private static final ConcurrentMap<String, String> topicStore = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, Long> activeTopics = new ConcurrentHashMap<>();
    // topic RSVP protocol
    private static RsvpProcessor rsvpProcessor;
    private static final ConcurrentMap<String, PendingRsvp> bids = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, String> pendingRsvp = new ConcurrentHashMap<>();
    private static final long RSVP_TIMEOUT = 20 * 1000L;
    private static final long RSVP_GRACE_PERIOD = 3 * 1000L;
    private static List<String> allTopics;
    private final int partitionCount;
    private final int maxVirtualTopics;
    private final boolean topicSubstitution;
    private final Map<String, String> preAllocatedTopics;

    public TopicController() throws IOException {
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
        if (rsvpProcessor == null) {
            rsvpProcessor = new RsvpProcessor();
            rsvpProcessor.start();
        }
    }

    private String getMonitorType(String origin) {
        return Platform.getInstance().getOrigin().equals(origin) ? "me" : "peer";
    }

    public static String getTopic(String origin) {
        for (String t: allTopics) {
            String appOrigin = topicStore.get(t);
            if (appOrigin.equals(origin)) {
                return t;
            }
        }
        return null;
    }

    private void rsvpBidding(String origin, long rsvpTime) {
        PendingRsvp holder = bids.get(origin);
        if (holder != null) {
            bids.put(origin, holder.touch());
        } else {
            bids.put(origin, new PendingRsvp(origin, rsvpTime));
        }
    }

    private boolean hasRsvpRights() {
        Utility util = Utility.getInstance();
        long now = System.currentTimeMillis();
        String me = Platform.getInstance().getOrigin();
        // remove expired bids
        List<String> expired = new ArrayList<>();
        for (PendingRsvp bidder: bids.values()) {
            if (now - bidder.lastSeen > RSVP_TIMEOUT) {
                expired.add(bidder.origin);
            }
        }
        for (String bidder: expired) {
            bids.remove(bidder);
            log.warn("RSVP bid from {} expired", bidder);
        }
        if (bids.containsKey(me)) {
            if (bids.size() == 1) {
                PendingRsvp rsvp = bids.get(me);
                // provide a grace period to avoid racing condition
                return now - rsvp.lastSeen > RSVP_GRACE_PERIOD;
            } else {
                List<String> rsvpBidders = new ArrayList<>();
                for (PendingRsvp r: bids.values()) {
                    rsvpBidders.add(util.getTimestamp(r.created) + "|" + r.origin);
                }
                Collections.sort(rsvpBidders);
                if (!rsvpBidders.isEmpty()) {
                    String ticket = rsvpBidders.get(0);
                    String winner = ticket.substring(ticket.indexOf('|')+1);
                    return me.equals(winner);
                }
            }
        }
        return false;
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        Utility util = Utility.getInstance();
        PostOffice po = PostOffice.getInstance();
        String myOrigin = Platform.getInstance().getOrigin();
        if (headers.containsKey(TYPE)) {
            String type = headers.get(TYPE);
            if (ALIVE.equals(type) && headers.containsKey(TOPIC) && headers.containsKey(ORIGIN) &&
                    headers.containsKey(NAME)) {
                String topic = headers.get(TOPIC);
                if (!activeTopics.containsKey(topic)) {
                    log.info("{} -> {}, {}", headers.get(TOPIC), headers.get(ORIGIN), headers.get(NAME));
                }
                activeTopics.put(topic, System.currentTimeMillis());
                topicStore.put(topic, headers.get(ORIGIN));
                return true;
            }
            if (RSVP.equals(type) && headers.containsKey(MONITOR) && headers.containsKey(RSVP_TIME)) {
                String bidder = headers.get(MONITOR);
                long rsvpTime = util.str2date(headers.get(RSVP_TIME)).getTime();
                rsvpBidding(bidder, rsvpTime);
                return true;
            }
            if (RSVP_START.equals(type) && headers.containsKey(MONITOR)) {
                String holder = headers.get(MONITOR);
                if (bids.containsKey(holder)) {
                    log.info("Monitor ({}) {} starts RSVP", getMonitorType(holder), holder);
                }
                return true;
            }
            if (RSVP_COMPLETE.equals(type) && headers.containsKey(MONITOR)) {
                String holder = headers.get(MONITOR);
                bids.remove(holder);
                log.info("Monitor ({}) {} finished RSVP", getMonitorType(holder), holder);
                return true;
            }
            if (GET_TOPIC.equals(type) && headers.containsKey(ORIGIN) && headers.containsKey(TX_PATH)) {
                String appOrigin = headers.get(ORIGIN);
                pendingRsvp.put(appOrigin, headers.get(TX_PATH));
                po.send(MainApp.TOPIC_CONTROLLER + MONITOR_PARTITION, new Kv(TYPE, RSVP),
                        new Kv(RSVP_TIME, util.date2str(new Date())), new Kv(MONITOR, myOrigin));
                return true;
            }
            if (CONFIRM_TOPIC.equals(type) && headers.containsKey(ORIGIN) && headers.containsKey(TOPIC)) {
                if (headers.containsKey(MONITOR)) {
                    String monitor = headers.get(MONITOR);
                    PendingRsvp holder = bids.get(monitor);
                    if (holder != null) {
                        bids.put(monitor, holder.touch());
                    }
                }
                String topic = headers.get(TOPIC);
                String appOrigin = headers.get(ORIGIN);
                Object appName = MonitorService.getInfo(appOrigin, NAME);
                Object version = MonitorService.getInfo(appOrigin, VERSION);
                topicStore.put(topic, appOrigin);
                activeTopics.put(topic, System.currentTimeMillis());
                if (appName != null) {
                    log.info("{} assigned to {}, {}, {}", topic, appOrigin, appName, version);
                } else {
                    log.warn("{} reserved by {} but not reachable", topic, appOrigin);
                }
                return true;
            }
            if (RELEASE_TOPIC.equals(type) && headers.containsKey(ORIGIN)) {
                String appOrigin = headers.get(ORIGIN);
                String appName = headers.get(NAME);
                String version = headers.get(VERSION);
                String prevTopic = getTopic(appOrigin);
                if (prevTopic != null && appOrigin.equals(topicStore.get(prevTopic))) {
                    topicStore.put(prevTopic, AVAILABLE);
                    activeTopics.remove(prevTopic);
                    log.info("{} released by {}, {}, {}", prevTopic, appOrigin, appName, version);
                    return true;
                }
            }
        }
        return false;
    }

    private String nextTopic(String appOrigin) throws IOException {
        for (String t: allTopics) {
            String value = topicStore.get(t);
            if (value.equals(AVAILABLE)) {
                topicStore.put(t, appOrigin);
                return t;
            }
        }
        throw new IOException("All virtual topics ("+ maxVirtualTopics +") are busy");
    }

    public static Map<String, String> getAssignedTopics() {
        Map<String, String> assigned = new HashMap<>();
        for (String t: allTopics) {
            String value = topicStore.get(t);
            if (!AVAILABLE.equals(value)) {
                assigned.put(t, value);
            }
        }
        return assigned;
    }

    private class RsvpProcessor extends Thread {

        private boolean normal = true;

        @Override
        public void run() {
            log.info("RSVP processor started");
            Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));

            long t0 = System.currentTimeMillis();
            Utility util = Utility.getInstance();
            PubSub ps = PubSub.getInstance();
            PostOffice po = PostOffice.getInstance();
            String myOrigin = Platform.getInstance().getOrigin();
            List<String> currentTopics = new ArrayList<>();
            while (normal) {
                long now = System.currentTimeMillis();
                // any RSVP requests for me?
                if (hasRsvpRights()) {
                    // start RSVP
                    try {
                        po.send(MainApp.TOPIC_CONTROLLER + MONITOR_PARTITION,
                                new Kv(TYPE, RSVP_START), new Kv(MONITOR, myOrigin));
                    } catch (IOException e) {
                        // ok to ignore
                    }
                    List<String> requests = new ArrayList<>(pendingRsvp.keySet());
                    for (String appOrigin: requests) {
                        String txPath = pendingRsvp.get(appOrigin);
                        pendingRsvp.remove(appOrigin);
                        try {
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
                            if (!currentTopics.contains(topic)) {
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
                                currentTopics.add(topic);
                            }
                            po.send(MainApp.TOPIC_CONTROLLER+MONITOR_PARTITION,
                                    new Kv(TYPE, CONFIRM_TOPIC), new Kv(MONITOR, myOrigin),
                                    new Kv(TOPIC, topicPartition), new Kv(ORIGIN, appOrigin));
                            po.send(txPath, new EventEnvelope().setTo(READY)
                                    .setHeader(TOPIC, topicPartition)
                                    .setHeader(VERSION, util.getVersionInfo().getVersion()).toBytes());
                            po.send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, JOIN),
                                    new Kv(ORIGIN, appOrigin), new Kv(TOPIC, topicPartition));
                        } catch (IOException e) {
                            try {
                                MonitorService.closeConnection(txPath, TRY_AGAIN_LATER, e.getMessage());
                            } catch (IOException ioe) {
                                // ok to ignore
                            }
                        }
                    }
                    // finished RSVP
                    try {
                        po.send(MainApp.TOPIC_CONTROLLER + MONITOR_PARTITION,
                                new Kv(TYPE, RSVP_COMPLETE), new Kv(MONITOR, myOrigin));
                    } catch (IOException e) {
                        // ok to ignore
                    }
                }
                if (now - t0 > INTERVAL) {
                    t0 = now;
                    // remove stalled connections
                    MonitorService.clearStalledConnection();
                    // remove inactive topics
                    Map<String, String> topics = getAssignedTopics();
                    for (String t : topics.keySet()) {
                        if (activeTopics.containsKey(t)) {
                            if (now - activeTopics.get(t) > EXPIRY) {
                                activeTopics.remove(t);
                                topicStore.put(t, AVAILABLE);
                                log.info("{} expired", t);
                            }
                        } else {
                            // to be evaluated in next cycle
                            activeTopics.put(t, now);
                        }
                    }
                }
                try {
                    Thread.sleep(1000);
                } catch (InterruptedException e) {
                    // ok to ignore
                }
            }
            log.info("RSVP processor stopped");
        }

        private void shutdown() {
            normal = false;
        }
    }

}
