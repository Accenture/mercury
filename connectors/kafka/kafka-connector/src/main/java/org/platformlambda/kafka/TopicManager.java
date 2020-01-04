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

import org.apache.kafka.clients.admin.*;
import org.apache.kafka.common.Node;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;
import java.util.concurrent.ExecutionException;

public class TopicManager implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(TopicManager.class);

    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String ORIGIN = "origin";
    private static final String CREATE_TOPIC = "create_topic";
    private static final String LEAVE = "leave";
    private static final String LIST = "list";
    private static final String PUB_SUB = "pub_sub";
    private static final String EXISTS = "exists";
    private static final String STOP = "stop";
    private static long lastStarted = 0;
    private static long lastActive = System.currentTimeMillis();
    private static Integer replicationFactor;

    private Properties properties;
    private AdminClient admin;
    private int count = 0;

    public TopicManager(Properties properties) {
        Properties prop = new Properties();
        prop.putAll(properties);
        prop.put(AdminClientConfig.CLIENT_ID_CONFIG, "admin-"+ Platform.getInstance().getOrigin());
        this.properties = prop;
    }

    public static long getLastStarted() {
        return lastStarted == 0? System.currentTimeMillis() : lastStarted;
    }

    public static long getLastActive() {
        return lastActive;
    }

    private void startAdmin() {
        if (admin == null) {
            admin = AdminClient.create(properties);
            lastStarted = System.currentTimeMillis();
            log.info("AdminClient ready");
        }
        lastActive = System.currentTimeMillis();
    }

    private void stopAdmin() {
        if (admin != null) {
            try {
                admin.close();
                log.info("AdminClient closed, processed: {}", count);

            } catch (Exception e) {
                // ok to ignore
            }
            lastStarted = 0;
            count = 0;
            admin = null;
        }
        lastActive = System.currentTimeMillis();
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        if (headers.containsKey(TYPE)) {
            if (STOP.equals(headers.get(TYPE))) {
                stopAdmin();
                return true;
            }
            if (LIST.equals(headers.get(TYPE))) {
                return listTopics(!headers.containsKey(PUB_SUB));
            }
            if (EXISTS.equals(headers.get(TYPE)) && headers.containsKey(ORIGIN)) {
                String origin = headers.get(ORIGIN);
                // direct pub-sub topic does not use regular node topic name
                boolean valid = headers.containsKey(PUB_SUB) || regularTopicFormat(origin);
                return valid && topicExists(origin);
            }
            // if origin is not specified, it will create the dedicated topic for a new application that is starting up
            if (CREATE_TOPIC.equals(headers.get(TYPE))) {
                createTopic(headers.containsKey(ORIGIN)? headers.get(ORIGIN) : Platform.getInstance().getOrigin());
                return true;
            }
            // delete topic when an application instance expires
            if (headers.containsKey(ORIGIN) && LEAVE.equals(headers.get(TYPE))) {
                String origin = headers.get(ORIGIN);
                if (topicExists(origin)) {
                    deleteTopic(origin);
                }
                return true;
            }
        }
        return false;
    }

    private boolean topicExists(String topic) {
        startAdmin();
        DescribeTopicsResult topicMetadata = admin.describeTopics(Arrays.asList(topic));
        boolean found = false;
        try {
            Map<String, TopicDescription> result = topicMetadata.all().get();
            count++;
            if (!result.isEmpty()) {
                for (String k: result.keySet()) {
                    TopicDescription desc = result.get(k);
                    if (desc.name().equals(topic)) {
                        found = true;
                        break;
                    }
                }
            }
            return found;
        } catch (Exception e) {
            // InvalidTopicException is a RunTimeException so it is better to catch all exceptions
            return false;
        }
    }

    private int getReplicationFactor() {
        if (replicationFactor == null) {
            AppConfigReader reader = AppConfigReader.getInstance();
            int factor = Utility.getInstance().str2int(reader.getProperty("kafka.replication.factor", "3"));
            if (factor > 3) {
                factor = 3;
                log.warn("Default kafka replication factor reset to 3");
            }
            DescribeClusterResult cluster = admin.describeCluster();
            try {
                Collection<Node> nodes = cluster.nodes().get();
                log.info("Kafka cluster information");
                for (Node n : nodes) {
                    log.info("Broker-Id: {}, Host: {}", n.id(), n.host());
                }
                replicationFactor = Math.min(factor, nodes.size());
                log.info("Kafka replication factor set to {}", replicationFactor);

            } catch (InterruptedException | ExecutionException e) {
                log.error("Unable to read cluster information - {}", e.getMessage());
                replicationFactor = 1;
            }
        }
        return replicationFactor;
    }

    private void createTopic(String topic) {
        startAdmin();
        try {
            if (topicExists(topic)) {
                log.info("Topic {} already exists", topic);
            } else {
                int replication = getReplicationFactor();
                CreateTopicsResult createTask = admin.createTopics(
                        Collections.singletonList(new NewTopic(topic, 1, (short) replication)));
                createTask.all().get();
                count++;
                // check if creation is successful
                boolean found = false;
                // try 3 times due to latency
                for (int i=0; i < 5; i++) {
                    found = topicExists(topic);
                    if (found) {
                        break;
                    } else {
                        Thread.sleep(1000);
                        log.warn("Newly created topic {} not found. Scanning it again.", topic);
                    }
                }
                if (found) {
                    log.info("Created topic {} with replication factor of {}", topic, replication);
                } else {
                    log.error("Unable to create topic {}", topic);
                    System.exit(-1);
                }
            }
        } catch (InterruptedException  | ExecutionException e) {
            log.error("Unable to create topic {} - {}", topic, e.getMessage());
            System.exit(-1);
        }
    }

    private void deleteTopic(String topic) {
        if (topicExists(topic)) {
            startAdmin();
            DeleteTopicsResult deleteTask = admin.deleteTopics(Collections.singletonList(topic));
            try {
                deleteTask.all().get();
                count++;
                // check if removal is successful
                // try 3 times due to eventual consistency latency
                boolean found = false;
                for (int i = 0; i < 5; i++) {
                    found = topicExists(topic);
                    if (found) {
                        Thread.sleep(1000);
                        log.warn("Newly deleted topic {} still exists. Scanning it again.", topic);
                    } else {
                        break;
                    }
                }
                if (!found) {
                    log.info("Deleted topic {}", topic);
                } else {
                    log.error("Unable to delete topic {}", topic);
                }

            } catch (InterruptedException | ExecutionException e) {
                log.error("Unable to delete topic {} - {}", topic, e.getMessage());
            }
        }
    }

    private List<String> listTopics(boolean regular) {
        startAdmin();
        List<String> result = new ArrayList<>();
        ListTopicsResult list = admin.listTopics();
        try {
            Set<String> topics = list.names().get();
            count++;
            String namespace = Platform.getInstance().getNamespace();
            for (String t: topics) {
                // skip topics that are not in this namespace
                if (namespace != null && !t.endsWith(namespace)) {
                    continue;
                }
                if (regular) {
                    if (regularTopicFormat(t)) {
                        result.add(t);
                    }
                } else {
                    if (!regularTopicFormat(t)) {
                        result.add(t);
                    }
                }
            }
        } catch (InterruptedException | ExecutionException e) {
            log.error("Unable to list topics - {}", e.getMessage());
        }
        return result;
    }

    /**
     * Validate a topic ID for an application instance
     *
     * @param topic in format of yyyymmdd uuid
     * @return true if valid
     */
    public static boolean regularTopicFormat(String topic) {
        Platform platform = Platform.getInstance();
        if (topic.length() != platform.getOrigin().length()) {
            return false;
        }
        // first 8 digits is a date stamp
        String uuid = topic.substring(8);
        if (!Utility.getInstance().isDigits(topic.substring(0, 8))) {
            return false;
        }
        // drop namespace before validation
        if (platform.getNamespace() != null) {
            int dot = uuid.lastIndexOf('.');
            if (dot > 1) {
                uuid = uuid.substring(0, dot);
            }
        }
        // application instance ID should be hexadecimal
        for (int i=0; i < uuid.length(); i++) {
            if (uuid.charAt(i) >= '0' && uuid.charAt(i) <= '9') continue;
            if (uuid.charAt(i) >= 'a' && uuid.charAt(i) <= 'f') continue;
            return false;
        }
        return true;
    }

}
