package org.platformlambda.services;

import org.platformlambda.MainApp;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.models.TopicRequest;
import org.platformlambda.tibco.TibcoSetup;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.CloseReason;
import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class TopicController implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(TopicController.class);

    private static final String MONITOR_PARTITION = TibcoSetup.MONITOR_PARTITION;
    private static final String TYPE = "type";
    private static final String ORIGIN = "origin";
    private static final String NAME = "name";
    private static final String MONITOR = "monitor";
    private static final String DOWNLOAD = "download";
    private static final String ALIVE = "keep-alive";
    private static final String PUT = "put";
    private static final String JOIN = "join";
    private static final String READY = "ready";
    private static final String VERSION = "version";
    private static final String GET_TOPIC = "get_topic";
    private static final String TX_PATH = "tx_path";
    private static final String RSVP = "rsvp";
    private static final String RSVP_COMPLETE = "rsvp_done";
    private static final String CONFIRM_TOPIC = "confirm_topic";
    private static final String RELEASE_TOPIC = "release_topic";
    private static final String TOPIC = "topic";
    private static final String AVAILABLE = "*";
    private static final long INTERVAL = 5 * 1000;
    private static final long EXPIRY = 60 * 1000;
    // topic+partition -> origin | AVAILABLE(*)
    private static final ConcurrentMap<String, String> topicStore = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, Long> activeTopics = new ConcurrentHashMap<>();
    // topic RSVP protocol
    private static RsvpProcessor rsvpProcessor;
    private static final ConcurrentMap<String, TopicRequest> rsvpMap = new ConcurrentHashMap<>();
    private static final long RSVP_TIMEOUT = 12 * 1000;
    private static String rsvpHolder;
    private static long rsvpLock = System.currentTimeMillis();

    private static List<String> allTopics;
    private final int partitionCount, maxVirtualTopics;

    public TopicController() {
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

    private String getTopic(String origin) {
        for (String t: allTopics) {
            String appOrigin = topicStore.get(t);
            if (appOrigin.equals(origin)) {
                return t;
            }
        }
        return null;
    }

    private void eliminateDuplicate(String topic, String origin) {
        List<String> topicList = new ArrayList<>(topicStore.keySet());
        for (String t: topicList) {
            String appOrigin = topicStore.get(t);
            if (appOrigin.equals(origin) && !t.equals(topic)) {
                topicStore.put(t, AVAILABLE);
                activeTopics.remove(t);
                log.info("{} expired", t);
            }
        }
    }

    @SuppressWarnings("unchecked")
    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        long now = System.currentTimeMillis();
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
                eliminateDuplicate(topic, headers.get(ORIGIN));
                return true;
            }
            if (RSVP.equals(type) && headers.containsKey(MONITOR)) {
                rsvpHolder = headers.get(MONITOR);
                rsvpLock = now;
                log.info("Monitor ({}) {} begins RSVP", getMonitorType(headers.get(MONITOR)), headers.get(MONITOR));
                return true;
            }
            if (RSVP_COMPLETE.equals(type) && headers.containsKey(MONITOR)) {
                rsvpHolder = null;
                rsvpLock = 0;
                log.info("Monitor ({}) {} finished RSVP", getMonitorType(headers.get(MONITOR)), headers.get(MONITOR));
                return true;
            }
            if (GET_TOPIC.equals(type) && headers.containsKey(ORIGIN) && headers.containsKey(TX_PATH)) {
                String appOrigin = headers.get(ORIGIN);
                rsvpMap.put(appOrigin, new TopicRequest(appOrigin, headers.get(TX_PATH)));
                if (now - rsvpLock > RSVP_TIMEOUT) {
                    rsvpLock = now;
                    po.send(MainApp.TOPIC_CONTROLLER + MONITOR_PARTITION, new Kv(TYPE, RSVP),
                            new Kv(MONITOR, myOrigin));
                }
                return true;
            }
            if (CONFIRM_TOPIC.equals(type) && headers.containsKey(ORIGIN) && headers.containsKey(TOPIC)) {
                String topic = headers.get(TOPIC);
                String appOrigin = headers.get(ORIGIN);
                Object appName = MonitorService.getInfo(appOrigin, NAME);
                Object version = MonitorService.getInfo(appOrigin, VERSION);
                topicStore.put(topic, appOrigin);
                activeTopics.put(topic, System.currentTimeMillis());
                if (appName != null) {
                    log.info("{} assigned to {} {}, {}", topic, appOrigin, appName, version);
                } else {
                    log.warn("{} reserved by {} but not reachable", topic, appOrigin);
                }
                return true;
            }
            if (RELEASE_TOPIC.equals(type) && headers.containsKey(ORIGIN)) {
                String appOrigin = headers.get(ORIGIN);
                String prevTopic = getTopic(appOrigin);
                if (prevTopic != null && appOrigin.equals(topicStore.get(prevTopic))) {
                    topicStore.put(prevTopic, AVAILABLE);
                    activeTopics.remove(prevTopic);
                    log.info("{} released by {}", prevTopic, appOrigin);
                    return true;
                }
            }
            if (PUT.equals(type) && body instanceof Map) {
                if (!myOrigin.equals(headers.get(ORIGIN))) {
                    restoreAssignedTopics((Map<String, String>) body);
                }
                return true;
            }
            // download request from a new presence monitor
            if (DOWNLOAD.equals(type)) {
                if (!myOrigin.equals(headers.get(ORIGIN))) {
                    po.send(MainApp.TOPIC_CONTROLLER + MONITOR_PARTITION, getAssignedTopics(),
                            new Kv(TYPE, PUT), new Kv(ORIGIN, myOrigin));
                }
                return true;
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

    private void restoreAssignedTopics(Map<String, String> assigned) {
        for (String item: assigned.keySet()) {
            if (item.contains("-")) {
                topicStore.put(item, assigned.get(item));
                log.debug("Saving {}", item);
            }
        }
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
                // are there RSVP requests?
                if (!rsvpMap.isEmpty()) {
                    if (myOrigin.equals(rsvpHolder)) {
                        List<String> requests = new ArrayList<>(rsvpMap.keySet());
                        for (String appOrigin: requests) {
                            TopicRequest topicRequest = rsvpMap.get(appOrigin);
                            String txPath = topicRequest.txPath;
                            rsvpMap.remove(appOrigin);
                            try {
                                // check if appOrigin has a topic in store
                                String topicPartition = nextTopic(appOrigin);
                                String topic = topicPartition.substring(0, topicPartition.lastIndexOf('-'));
                                // automatic create topic
                                if (!currentTopics.contains(topic)) {
                                    String firstPartition = topic + ".0";
                                    if (!ps.exists(firstPartition)) {
                                        ps.createTopic(topic, partitionCount);
                                    }
                                    currentTopics.add(topic);
                                }
                                po.send(MainApp.TOPIC_CONTROLLER+MONITOR_PARTITION, new Kv(TYPE, CONFIRM_TOPIC),
                                        new Kv(TOPIC, topicPartition), new Kv(ORIGIN, appOrigin));
                                po.send(txPath, new EventEnvelope().setTo(READY)
                                        .setHeader(TOPIC, topicPartition)
                                        .setHeader(VERSION, util.getVersionInfo().getVersion()).toBytes());
                                po.send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, JOIN),
                                        new Kv(ORIGIN, appOrigin), new Kv(TOPIC, topicPartition));
                            } catch (IOException e) {
                                try {
                                    util.closeConnection(txPath, CloseReason.CloseCodes.TRY_AGAIN_LATER,e.getMessage());
                                } catch (IOException ioe) {
                                    // ok to ignore
                                }
                            }
                        }
                        // finished RSVP
                        try {
                            po.send(MainApp.TOPIC_CONTROLLER + MONITOR_PARTITION, new Kv(TYPE, RSVP_COMPLETE),
                                    new Kv(MONITOR, myOrigin));
                        } catch (IOException e) {
                            // ok to ignore
                        }
                        rsvpHolder = null;
                        rsvpLock = 0;

                    } else {
                        if (now - rsvpLock > RSVP_TIMEOUT) {
                            try {
                                po.send(MainApp.TOPIC_CONTROLLER + MONITOR_PARTITION, new Kv(TYPE, RSVP),
                                        new Kv(MONITOR, myOrigin));
                            } catch (IOException e) {
                                log.error("Unable to send RSVP - {}", e.getMessage());
                            }
                        }
                    }
                    // defer topic validation to the next interval
                    t0 = now;
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
