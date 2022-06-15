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

package org.platformlambda.hazelcast.services;

import com.hazelcast.core.HazelcastInstance;
import com.hazelcast.topic.ITopic;
import org.platformlambda.cloud.ConnectorConfig;
import org.platformlambda.cloud.EventProducer;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.PubSubProvider;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.hazelcast.HazelcastConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;

public class PubSubManager implements PubSubProvider {
    private static final Logger log = LoggerFactory.getLogger(PubSubManager.class);

    private static final MsgPack msgPack = new MsgPack();
    private static final String TYPE = "type";
    private static final String PARTITIONS = "partitions";
    private static final String HEADERS = "headers";
    private static final String BODY = "body";
    private static final String CREATE = "create";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private static final String DELETE = "delete";
    private static final String TOPIC = "topic";
    private static final ConcurrentMap<String, EventConsumer> subscribers = new ConcurrentHashMap<>();

    private final String domain;
    private final Properties properties;
    private final String cloudManager;

    public PubSubManager(String domain, Properties properties, String cloudManager) {
        this.domain = domain;
        this.properties = properties;
        this.cloudManager = cloudManager;
        try {
            // start Topic Manager
            log.info("Starting {} pub/sub manager - {}", domain, cloudManager);
            Platform.getInstance().registerPrivate(cloudManager, new TopicManager(domain, properties), 1);
        } catch (IOException e) {
            log.error("Unable to start producer - {}", e.getMessage());
        }
        // clean up subscribers when application stops
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
    }

    @Override
    public void waitForProvider(int seconds) {
        try {
            Platform.getInstance().waitForProvider(cloudManager, seconds);
        } catch (TimeoutException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public boolean createTopic(String topic) throws IOException {
        return createTopic(topic, 1);
    }

    @Override
    public boolean createTopic(String topic, int partitions) throws IOException {
        ConnectorConfig.validateTopicName(topic);
        try {
            EventEnvelope init = PostOffice.getInstance().request(cloudManager, 20000,
                    new Kv(TYPE, CREATE), new Kv(TOPIC, topic), new Kv(PARTITIONS, partitions));
            if (init.getBody() instanceof Boolean) {
                return(Boolean) init.getBody();
            } else {
                return false;
            }
        } catch (TimeoutException | AppException e) {
            throw new IOException(e.getMessage());
        }
    }

    @Override
    public void deleteTopic(String topic) throws IOException {
        try {
            PostOffice.getInstance().request(cloudManager, 20000, new Kv(TYPE, DELETE), new Kv(TOPIC, topic));
        } catch (TimeoutException | AppException e) {
            throw new IOException(e.getMessage());
        }
    }

    @Override
    public boolean createQueue(String queue) throws IOException {
        throw new IOException("Not implemented");
    }

    @Override
    public void deleteQueue(String queue) throws IOException {
        throw new IOException("Not implemented");
    }

    @Override
    public void publish(String topic, Map<String, String> headers, Object body) throws IOException {
        publish(topic, -1, headers, body);
    }

    @Override
    public void publish(String topic, int partition, Map<String, String> headers, Object body) throws IOException {
        ConnectorConfig.validateTopicName(topic);
        Utility util = Utility.getInstance();
        Map<String, String> eventHeaders = headers == null? new HashMap<>() : headers;
        if (eventHeaders.containsKey(EventProducer.EMBED_EVENT) && body instanceof byte[]) {
            sendEvent(topic, partition, eventHeaders, (byte[]) body);
        } else {
            final byte[] payload;
            if (body instanceof byte[]) {
                payload = (byte[]) body;
                eventHeaders.put(EventProducer.DATA_TYPE, EventProducer.BYTES_DATA);
            } else if (body instanceof String) {
                payload = util.getUTF((String) body);
                eventHeaders.put(EventProducer.DATA_TYPE, EventProducer.TEXT_DATA);
            } else if (body instanceof Map) {
                payload = msgPack.pack(body);
                eventHeaders.put(EventProducer.DATA_TYPE, EventProducer.MAP_DATA);
            } else if (body instanceof List) {
                payload = msgPack.pack(body);
                eventHeaders.put(EventProducer.DATA_TYPE, EventProducer.LIST_DATA);
            } else {
                // other primitive and PoJo are serialized as JSON string
                payload = SimpleMapper.getInstance().getMapper().writeValueAsBytes(body);
                eventHeaders.put(EventProducer.DATA_TYPE, EventProducer.TEXT_DATA);
            }
            sendEvent(topic, partition, eventHeaders, payload);
        }
    }

    private void sendEvent(String topic, int partition, Map<String, String> headers, byte[] payload) {
        String realTopic = partition < 0? topic : topic+"."+partition;
        try {
            HazelcastInstance client = HazelcastConnector.getClient(domain, properties);
            ITopic<Map<String, Object>> iTopic = client.getReliableTopic(realTopic);
            Map<String, Object> event = new HashMap<>();
            event.put(HEADERS, headers);
            event.put(BODY, payload);
            iTopic.publish(event);
        } catch (Exception e) {
            log.error("Unable to publish event to {} - {}", realTopic, e.getMessage());
            // just let the platform such as Kubernetes to restart the application instance
            System.exit(12);
        }
    }

    @Override
    public void subscribe(String topic, LambdaFunction listener, String... parameters) throws IOException {
        subscribe(topic, -1, listener, parameters);
    }

    @Override
    public void subscribe(String topic, int partition, LambdaFunction listener, String... parameters) throws IOException {
        ConnectorConfig.validateTopicName(topic);
        String topicPartition = (topic + (partition < 0? "" : "." + partition)).toLowerCase();
        if (parameters.length == 2 || parameters.length == 3) {
            if (parameters.length == 3 && !Utility.getInstance().isNumeric(parameters[2])) {
                throw new IOException("topic offset must be numeric");
            }
            if (subscribers.containsKey(topicPartition) || Platform.getInstance().hasRoute(topicPartition)) {
                throw new IOException(topicPartition+" is already subscribed");
            }
            EventConsumer consumer = new EventConsumer(domain, properties, topic, partition, parameters);
            consumer.start();
            Platform.getInstance().registerPrivate(topicPartition, listener, 1);
            subscribers.put(topicPartition, consumer);
        } else {
            throw new IOException("Check parameters: clientId, groupId and optional offset pointer");
        }
    }

    @Override
    public void send(String queue, Map<String, String> headers, Object body) throws IOException {
        throw new IOException("Not implemented");
    }

    @Override
    public void listen(String queue, LambdaFunction listener, String... parameters) throws IOException {
        throw new IOException("Not implemented");
    }

    @Override
    public void unsubscribe(String topic) throws IOException {
        unsubscribe(topic, -1);
    }

    @Override
    public void unsubscribe(String topic, int partition) throws IOException {
        String topicPartition = (topic + (partition < 0? "" : "." + partition)).toLowerCase();
        Platform platform = Platform.getInstance();
        if (platform.hasRoute(topicPartition) && subscribers.containsKey(topicPartition)) {
            EventConsumer consumer = subscribers.get(topicPartition);
            platform.release(topicPartition);
            subscribers.remove(topicPartition);
            consumer.shutdown();
        } else {
            if (partition > -1) {
                throw new IOException(topicPartition +
                        " has not been subscribed by this application instance");
            } else {
                throw new IOException(topic + " has not been subscribed by this application instance");
            }
        }
    }

    @Override
    public boolean exists(String topic) throws IOException {
        try {
            EventEnvelope response = PostOffice.getInstance().request(cloudManager, 20000,
                    new Kv(TYPE, EXISTS), new Kv(TOPIC, topic));
            if (response.getBody() instanceof Boolean) {
                return (Boolean) response.getBody();
            } else {
                return false;
            }
        } catch (TimeoutException | AppException e) {
            throw new IOException(e.getMessage());
        }
    }

    @Override
    public int partitionCount(String topic) throws IOException {
        try {
            EventEnvelope response = PostOffice.getInstance().request(cloudManager, 20000,
                    new Kv(TYPE, PARTITIONS), new Kv(TOPIC, topic));
            if (response.getBody() instanceof Integer) {
                return (Integer) response.getBody();
            } else {
                return -1;
            }
        } catch (TimeoutException | AppException e) {
            throw new IOException(e.getMessage());
        }
    }

    @SuppressWarnings("unchecked")
    @Override
    public List<String> list() throws IOException {
        try {
            EventEnvelope init = PostOffice.getInstance().request(cloudManager, 20000, new Kv(TYPE, LIST));
            if (init.getBody() instanceof List) {
                return (List<String>) init.getBody();
            } else {
                return Collections.EMPTY_LIST;
            }
        } catch (TimeoutException | AppException e) {
            throw new IOException(e.getMessage());
        }
    }

    @Override
    public boolean isStreamingPubSub() {
        return false;
    }

    private void shutdown() {
        for (String topic: subscribers.keySet()) {
            EventConsumer consumer = subscribers.get(topic);
            consumer.shutdown();
        }
    }
}
