/*

    Copyright 2018-2024 Accenture Technology

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

package org.platformlambda.kafka.adapter;

import org.apache.kafka.clients.admin.*;
import org.apache.kafka.clients.producer.KafkaProducer;
import org.apache.kafka.clients.producer.ProducerConfig;
import org.apache.kafka.common.errors.UnknownTopicOrPartitionException;
import org.platformlambda.core.annotations.CloudConnector;
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.kafka.KafkaConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ExecutionException;

@CloudConnector(name="kafka-adapter")
public class KafkaAdapter implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(KafkaAdapter.class);

    private static final String CREATE_PREFIX = "create[";
    private static final String CONSUMER_PREFIX = "consumer[";
    private static KafkaProducer<String, byte[]> producer = null;

    @Override
    public void initialize() {
        ConfigReader config = new ConfigReader();
        try {
            config.load("classpath:/kafka-adapter.yaml");
            Properties baseProp = KafkaConnector.getKafkaProperties("kafka.adapter");
            createTopics(config, baseProp);
            createProducer(config, baseProp);
            createConsumers(config, baseProp);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public static KafkaProducer<String, byte[]> getProducer() {
        return producer;
    }

    @SuppressWarnings("unchecked")
    private void createConsumers(ConfigReader config, Properties baseProp) {
        Object required = config.get("consumer");
        if (required instanceof List) {
            List<Object> entries = (List<Object>) required;
            if (!entries.isEmpty()) {
                for (int i=0; i < entries.size(); i++) {
                    String topic = config.getProperty(CONSUMER_PREFIX+i+"].topic");
                    String target = config.getProperty(CONSUMER_PREFIX+i+"].target");
                    String group = config.getProperty(CONSUMER_PREFIX+i+"].group");
                    boolean tracing = "true".equals(config.getProperty(CONSUMER_PREFIX+i+"].tracing", "false"));
                    if (topic == null || target == null || group == null) {
                        log.error("Missing topic, target or group in the 'consumer' section - entry #{}", i+1);
                    }
                    KafkaListener listener = new KafkaListener(baseProp, topic, target, group, tracing);
                    listener.start();
                }
            }
        }
    }

    private static void createProducer(ConfigReader config, Properties baseProp) {
        if ("true".equalsIgnoreCase(config.getProperty("producer.enabled", "false"))) {
            Properties properties = new Properties();
            properties.putAll(baseProp);
            properties.put(ProducerConfig.ACKS_CONFIG, "1"); // Setting to "1" ensures that the message is received by the leader
            properties.put(ProducerConfig.KEY_SERIALIZER_CLASS_CONFIG, org.apache.kafka.common.serialization.StringSerializer.class);
            properties.put(ProducerConfig.VALUE_SERIALIZER_CLASS_CONFIG, org.apache.kafka.common.serialization.ByteArraySerializer.class);
            properties.put(ProducerConfig.MAX_BLOCK_MS_CONFIG, 15000);
            properties.put(ProducerConfig.CLIENT_ID_CONFIG, "producer-"+ Platform.getInstance().getOrigin());
            producer = new KafkaProducer<>(properties);
            log.info("Kafka producer ready - ({})", producer);
        }
    }

    @SuppressWarnings("unchecked")
    private void createTopics(ConfigReader config, Properties baseProp) {
        Utility util = Utility.getInstance();
        Object required = config.get("create");
        if (required instanceof List) {
            List<Object> entries = (List<Object>) required;
            if (!entries.isEmpty()) {
                Properties properties = new Properties();
                properties.putAll(baseProp);
                properties.put(AdminClientConfig.CLIENT_ID_CONFIG, "admin-"+ Platform.getInstance().getOrigin());
                try (AdminClient admin = AdminClient.create(properties)) {
                    log.info("AdminClient started");
                    for (int i=0; i < entries.size(); i++) {
                        String topic = config.getProperty(CREATE_PREFIX+i+"].topic");
                        String p = config.getProperty(CREATE_PREFIX+i+"].partition", "1");
                        String r = config.getProperty(CREATE_PREFIX+i+"].replication", "1");
                        if (topic != null) {
                            int partition = Math.min(32, Math.max(1, util.str2int(p)));
                            int replication = Math.min(3, Math.max(1, util.str2int(r)));
                            ensureOneTopic(admin, topic, partition, replication);
                        } else {
                            log.error("Missing 'topic' in the 'create' section - entry #{}", i+1);
                        }
                    }
                } catch (ExecutionException | InterruptedException e) {
                    throw new IllegalArgumentException(e);
                } finally {
                    log.info("AdminClient stopped");
                }
            }
        }
    }

    private void ensureOneTopic(AdminClient admin, String topic, int partition, int replication)
                                throws ExecutionException, InterruptedException {
        if (!topicExists(admin, topic)) {
            CreateTopicsResult createTask = admin.createTopics(
                    Collections.singletonList(new NewTopic(topic, partition, (short) replication)));
            createTask.all().get();
            log.info("Created {} with {} partition{}, replication factor of {}",
                    topic, partition, partition == 1? "" : "s", replication);
            waitForTopic(admin, topic);
        }
    }

    private void waitForTopic(AdminClient admin, String topic) throws InterruptedException {
        // check if creation is successful
        boolean found = false;
        // try a few times due to eventual consistency
        for (int i=0; i < 20; i++) {
            Thread.sleep(500);
            found = topicExists(admin, topic);
            if (found) {
                return;
            } else {
                log.warn("Waiting for {} to be visible", topic);
            }
        }
        log.error("Unable to create {} after a few attempts", topic);
        System.exit(-1);
    }

    private boolean topicExists(AdminClient admin, String topic) {
        DescribeTopicsResult topicMetadata = admin.describeTopics(Collections.singletonList(topic));
        try {
            Map<String, TopicDescription> result = topicMetadata.allTopicNames().get();
            if (!result.isEmpty()) {
                Collection<TopicDescription> topics = result.values();
                for (TopicDescription desc: topics) {
                    if (desc.name().equals(topic)) {
                        int n = desc.partitions().size();
                        log.info("Found {} with {} partition{}", topic, n, n == 1? "" : "s");
                        return true;
                    }
                }
            }
        } catch (UnknownTopicOrPartitionException | InterruptedException | ExecutionException e) {
            // move on because topic does not exist
        }
        return false;
    }
}
