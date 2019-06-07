/*

    Copyright 2018-2019 Accenture Technology

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

package org.platformlambda.pubsub;

import org.apache.kafka.clients.producer.KafkaProducer;
import org.apache.kafka.clients.producer.ProducerConfig;
import org.apache.kafka.clients.producer.ProducerRecord;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.PubSubProvider;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.platformlambda.core.websocket.common.WsConfigurator;
import org.platformlambda.kafka.EventConsumer;
import org.platformlambda.kafka.KafkaSetup;
import org.platformlambda.kafka.TopicManager;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.util.*;
import java.util.concurrent.*;

public class KafkaPubSub implements PubSubProvider {
    private static final Logger log = LoggerFactory.getLogger(KafkaPubSub.class);

    private static final String MANAGER = KafkaSetup.MANAGER;
    private static final String TYPE = "type";
    private static final String PRESENCE_MONITOR = KafkaSetup.PRESENCE_MONITOR;
    private static final String CREATE_TOPIC = "create_topic";
    private static final String PUB_SUB = "pub_sub";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private static final String LEAVE = "leave";
    private static final String ORIGIN = "origin";
    private static final int MAX_PAYLOAD = WsConfigurator.getInstance().getMaxBinaryPayload() - 256;
    private static final ConcurrentMap<String, EventConsumer> subscribers = new ConcurrentHashMap<>();

    private Properties properties = new Properties();
    private Properties baseProp;
    private KafkaProducer<String, byte[]> producer;

    public KafkaPubSub(Properties baseProp) {
        this.baseProp = baseProp;
        properties.putAll(baseProp);
        properties.put(ProducerConfig.ACKS_CONFIG, "1"); // Setting to "1" ensures that the message is received by the leader
        properties.put(ProducerConfig.KEY_SERIALIZER_CLASS_CONFIG, org.apache.kafka.common.serialization.StringSerializer.class);
        properties.put(ProducerConfig.VALUE_SERIALIZER_CLASS_CONFIG, org.apache.kafka.common.serialization.ByteArraySerializer.class);
        properties.put(ProducerConfig.MAX_BLOCK_MS_CONFIG, 15000);
        // clean up subscribers when application stops
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
    }

    private void validateTopicName(String route) throws IOException {
        if (route.equals(PRESENCE_MONITOR)) {
            throw new IOException(PRESENCE_MONITOR+" is reserved");
        }
        // guarantee that only valid service name is registered
        Utility util = Utility.getInstance();
        if (!util.validServiceName(route)) {
            throw new IOException("Invalid route name - use 0-9, a-z, period, hyphen or underscore characters");
        }
        String path = util.filteredServiceName(route);
        if (path.length() == 0) {
            throw new IOException("Invalid route name");
        }
        if (!path.contains(".")) {
            throw new IOException("Invalid route "+route+" because it is missing dot separator(s). e.g. hello.world");
        }
        if (util.reservedExtension(path)) {
            throw new IOException("Invalid route "+route+" because it cannot use a reserved extension");
        }
        if (util.reservedFilename(path)) {
            throw new IOException("Invalid route "+route+" which is a reserved Windows filename");
        }
    }

    @Override
    public boolean createTopic(String topic) throws IOException {
        validateTopicName(topic);
        try {
            EventEnvelope init = PostOffice.getInstance().request(MANAGER, 20000,
                                                                new Kv(TYPE, CREATE_TOPIC), new Kv(ORIGIN, topic));
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
        if (TopicManager.regularTopicFormat(topic)) {
            throw new IOException("Unable to delete topic because "+topic+" is reserved");
        }
        try {
            PostOffice.getInstance().request(MANAGER, 20000, new Kv(TYPE, LEAVE), new Kv(ORIGIN, topic));
        } catch (TimeoutException | AppException e) {
            throw new IOException(e.getMessage());
        }
    }

    @Override
    public void publish(String topic, Map<String, String> headers, Object body) throws IOException {
        if (producer == null) {
            // create unique ID from origin ID by dropping date prefix and adding a sequence suffix
            String id = Platform.getInstance().getOrigin()+"ps";
            properties.put(ProducerConfig.CLIENT_ID_CONFIG, id.substring(8));
            producer = new KafkaProducer<>(properties);
            log.info("Pub/Sub Producer {} ready", properties.getProperty(ProducerConfig.CLIENT_ID_CONFIG));
        }
        if (TopicManager.regularTopicFormat(topic)) {
            throw new IOException("Unable to publish because "+topic+" is reserved");
        }
        validateTopicName(topic);
        /*
         * Application can publish into any kafka topic, even those that are not created by applications using Mercury.
         * The format of the payload is a standard EventEnvelope serialized as a byte array.
         *
         * However, it is the responsibility of the user application to publish to an available topic
         * since this is a low level access to Kafka.
         *
         * Kafka is designed for high-performance event streaming.
         * Therefore, checking the validity of a kafka topic is not an option because it would be too slow.
         */
        String uuid = Utility.getInstance().getUuid();
        EventEnvelope event = new EventEnvelope();
        event.setTo(topic);
        if (headers != null) {
            for (String h: headers.keySet()) {
                event.setHeader(h, headers.get(h));
            }
        }
        event.setBody(body);
        byte[] payload = event.toBytes();
        try {
            // perform segmentation for large payload
            if (payload.length > MAX_PAYLOAD) {
                int total = (payload.length / MAX_PAYLOAD) + (payload.length % MAX_PAYLOAD == 0 ? 0 : 1);
                ByteArrayInputStream in = new ByteArrayInputStream(payload);
                for (int i = 0; i < total; i++) {
                    // To distinguish from a normal payload, the segmented block MUST not have a "TO" value.
                    EventEnvelope block = new EventEnvelope();
                    block.setHeader(MultipartPayload.ID, event.getId());
                    block.setHeader(MultipartPayload.COUNT, String.valueOf(i + 1));
                    block.setHeader(MultipartPayload.TOTAL, String.valueOf(total));
                    byte[] segment = new byte[MAX_PAYLOAD];
                    int size = in.read(segment);
                    block.setBody(size == MAX_PAYLOAD ? segment : Arrays.copyOfRange(segment, 0, size));
                    producer.send(new ProducerRecord<>(topic, uuid+i, block.toBytes())).get(10000, TimeUnit.MILLISECONDS);
                    log.info("Sending block {} of {} to {} via {}", i + 1, total, event.getTo(), topic);
                }
            } else {
                producer.send(new ProducerRecord<>(topic, uuid, payload)).get(10000, TimeUnit.MILLISECONDS);
            }
        } catch (InterruptedException | ExecutionException | TimeoutException e) {
            throw new IOException(e.getMessage());
        }
    }

    @Override
    public void subscribe(String topic, LambdaFunction listener, String... parameters) throws IOException {
        validateTopicName(topic);
        if (parameters.length == 2 || parameters.length == 3) {
            if (parameters.length == 3 && !Utility.getInstance().isDigits(parameters[2])) {
                throw new IOException("topic offset must be numeric");
            }
            if (subscribers.containsKey(topic)) {
                throw new IOException(topic+" is already subscribed by this application instance");
            }
            EventConsumer consumer = new EventConsumer(baseProp, topic, true, parameters);
            consumer.start();
            Platform.getInstance().register(topic, listener, 1);
            subscribers.put(topic, consumer);
        } else {
            throw new IOException("Parameters: clientId, groupId and optional offset pointer");
        }
    }

    @Override
    public void unsubscribe(String topic) throws IOException {
        validateTopicName(topic);
        Platform platform = Platform.getInstance();
        if (platform.hasRoute(topic) && subscribers.containsKey(topic)) {
            EventConsumer consumer = subscribers.get(topic);
            platform.release(topic);
            subscribers.remove(topic);
            consumer.shutdown();
        } else {
            throw new IOException(topic+" has not been subscribed by this application instance");
        }
    }

    @Override
    public boolean exists(String topic) throws IOException {
        validateTopicName(topic);
        try {
            EventEnvelope init = PostOffice.getInstance().request(MANAGER, 20000, new Kv(TYPE, EXISTS),
                                                                    new Kv(ORIGIN, topic), new Kv(PUB_SUB, true));
            if (init.getBody() instanceof Boolean) {
                return (Boolean) init.getBody();
            } else {
                return false;
            }
        } catch (TimeoutException | AppException e) {
            throw new IOException(e.getMessage());
        }
    }

    @Override
    @SuppressWarnings("unchecked")
    public List<String> list() throws IOException {
        try {
            EventEnvelope init = PostOffice.getInstance().request(MANAGER, 20000, new Kv(TYPE, LIST), new Kv(PUB_SUB, true));
            if (init.getBody() instanceof List) {
                return (List<String>) init.getBody();
            } else {
                return Collections.EMPTY_LIST;
            }
        } catch (TimeoutException | AppException e) {
            throw new IOException(e.getMessage());
        }
    }

    private void shutdown() {
        for (String topic: subscribers.keySet()) {
            EventConsumer consumer = subscribers.get(topic);
            consumer.shutdown();
        }
    }

}
