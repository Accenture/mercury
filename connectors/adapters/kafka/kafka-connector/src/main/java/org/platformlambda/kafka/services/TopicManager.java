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

package org.platformlambda.kafka.services;

import org.apache.kafka.clients.admin.*;
import org.apache.kafka.common.Node;
import org.apache.kafka.common.errors.UnknownTopicOrPartitionException;
import org.platformlambda.cloud.services.ServiceRegistry;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.kafka.KafkaConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ExecutionException;

public class TopicManager implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(TopicManager.class);

    private static final String TYPE = "type";
    private static final String PARTITIONS = "partitions";
    private static final String TOPIC = "topic";
    private static final String CREATE = "create";
    private static final String DELETE = "delete";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private static final String STOP = "stop";
    private static Integer replicationFactor;

    private static boolean startMonitor = true;
    private AdminClient admin;
    private long lastAccess = 0;
    private int count = 0, seq = 0;

    public TopicManager() {
        Runtime.getRuntime().addShutdownHook(new Thread(this::stopAdmin));
        if (startMonitor) {
            startMonitor = false;
            InactivityMonitor monitor = new InactivityMonitor();
            monitor.start();
        }
    }

    private synchronized void startAdmin() {
        if (admin == null) {
            seq++;
            Properties properties = new Properties();
            properties.putAll(KafkaConnector.getKafkaProperties());
            properties.put(AdminClientConfig.CLIENT_ID_CONFIG, "admin-"+ Platform.getInstance().getOrigin()+"-"+seq);
            admin = AdminClient.create(properties);
            log.info("AdminClient-{} ready", seq);
            lastAccess = System.currentTimeMillis();
        }
    }

    private synchronized void stopAdmin() {
        if (admin != null) {
            try {
                admin.close();
                log.info("AdminClient-{} closed, processed: {}", seq, count);
            } catch (Exception e) {
                // ok to ignore
            }
            count = 0;
            admin = null;
        }
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        if (headers.containsKey(TYPE)) {
            if (LIST.equals(headers.get(TYPE))) {
                return listTopics();
            }
            if (EXISTS.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                String origin = headers.get(TOPIC);
                return topicExists(origin);
            }
            if (PARTITIONS.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                String origin = headers.get(TOPIC);
                return topicPartitions(origin);
            }
            // if origin is not specified, it will create the dedicated topic for a new application that is starting up
            if (CREATE.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                int partitions = headers.containsKey(PARTITIONS)?
                                    Math.max(1, Utility.getInstance().str2int(headers.get(PARTITIONS))) : 1;
                createTopic(headers.get(TOPIC), partitions);
                return true;
            }
            // delete topic when an application instance expires
            if (DELETE.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                String origin = headers.get(TOPIC);
                if (topicExists(origin)) {
                    deleteTopic(origin);
                }
                return true;
            }
            if (STOP.equals(headers.get(TYPE))) {
                stopAdmin();
            }
        }
        return false;
    }

    private boolean topicExists(String topic) {
        return topicPartitions(topic) != -1;
    }

    private int topicPartitions(String topic) {
        startAdmin();
        lastAccess = System.currentTimeMillis();
        DescribeTopicsResult topicMetadata = admin.describeTopics(Collections.singletonList(topic));
        try {
            Map<String, TopicDescription> result = topicMetadata.all().get();
            count++;
            if (!result.isEmpty()) {
                for (String k: result.keySet()) {
                    TopicDescription desc = result.get(k);
                    if (desc.name().equals(topic)) {
                        return desc.partitions().size();
                    }
                }
            }
        } catch (UnknownTopicOrPartitionException | InterruptedException | ExecutionException e) {
            // move on because topic does not exist
        }
        return -1;
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

    private void createTopic(String topic, int partitions) {
        startAdmin();
        lastAccess = System.currentTimeMillis();
        try {
            int currentPartitions = topicPartitions(topic);
            if (currentPartitions == -1) {
                int replication = getReplicationFactor();
                int partitionCount = Math.max(1, partitions);
                CreateTopicsResult createTask = admin.createTopics(
                        Collections.singletonList(new NewTopic(topic, partitionCount, (short) replication)));
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
                        log.warn("Newly created {} not found. Scanning it again.", topic);
                    }
                }
                if (found) {
                    log.info("Created {} with {} partition{}, replication factor of {}",
                            topic, partitionCount, partitionCount == 1? "" : "s", replication);
                } else {
                    log.error("Unable to create {}", topic);
                    System.exit(-1);
                }
            } else {
                log.warn("{} with {} partition{} already exists", topic, currentPartitions,
                        currentPartitions == 1? "" : "s");
            }
        } catch (Exception e) {
            log.error("Unable to create {} - {}", topic, e.getMessage());
            System.exit(-1);
        }
    }

    private void deleteTopic(String topic) {
        if (topicExists(topic)) {
            startAdmin();
            lastAccess = System.currentTimeMillis();
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
                        log.warn("Newly deleted {} still exists. Scanning it again.", topic);
                    } else {
                        break;
                    }
                }
                if (!found) {
                    log.info("Deleted {}", topic);
                } else {
                    log.error("Unable to delete {}", topic);
                }
            } catch (InterruptedException | ExecutionException e) {
                log.error("Unable to delete {} - {}", topic, e.getMessage());
                stopAdmin();
            }
        }
    }

    private List<String> listTopics() {
        startAdmin();
        List<String> result = new ArrayList<>();
        ListTopicsResult list = admin.listTopics();
        try {
            count++;
            return new ArrayList<>(list.names().get());
        } catch (InterruptedException | ExecutionException e) {
            log.error("Unable to list topics - {}", e.getMessage());
            stopAdmin();
        }
        return result;
    }

    private class InactivityMonitor extends Thread {

        private boolean normal = true;

        public InactivityMonitor() {
            Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
        }

        @Override
        public void run() {
            final long INTERVAL = 20 * 1000;
            final long IDLE = 60 * 1000;
            long t0 = System.currentTimeMillis();
            while (normal) {
                long now = System.currentTimeMillis();
                if (now - t0 > INTERVAL) {
                    t0 = now;
                    if (admin != null && now - lastAccess > IDLE) {
                        try {
                            PostOffice.getInstance().send(ServiceRegistry.CLOUD_MANAGER, new Kv(TYPE, STOP));
                        } catch (IOException e) {
                            // ok to ignore
                        }
                    }
                }
                try {
                    Thread.sleep(1000);
                } catch (InterruptedException e) {
                    // ok to ignore
                }
            }
            log.info("Stopped");
        }

        private void shutdown() {
            normal = true;
        }

    }

}
