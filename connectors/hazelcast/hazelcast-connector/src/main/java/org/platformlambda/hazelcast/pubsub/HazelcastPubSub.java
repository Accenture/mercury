package org.platformlambda.hazelcast.pubsub;

import com.hazelcast.core.HazelcastInstance;
import com.hazelcast.topic.ITopic;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.*;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.platformlambda.core.websocket.common.WsConfigurator;
import org.platformlambda.hazelcast.HazelcastSetup;
import org.platformlambda.hazelcast.services.TopicManager;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.util.*;
import java.util.concurrent.*;

public class HazelcastPubSub implements PubSubProvider {
    private static final Logger log = LoggerFactory.getLogger(HazelcastPubSub.class);

    private static final String MANAGER = HazelcastSetup.MANAGER;
    private static final String TYPE = "type";
    private static final String PARTITIONS = "partitions";
    private static final String HEADERS = "headers";
    private static final String BODY = "body";
    private static final String CREATE = "create";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private static final String DELETE = "delete";
    private static final String ORIGIN = "origin";
    private static final int MAX_PAYLOAD = WsConfigurator.getInstance().getMaxBinaryPayload() - 256;
    private static final ConcurrentMap<String, EventConsumer> subscribers = new ConcurrentHashMap<>();
    private static long seq = 0, totalEvents = 0;

    public HazelcastPubSub() {
        try {
            // start Topic Manager
            Platform.getInstance().registerPrivate(MANAGER, new TopicManager(), 1);
        } catch (IOException e) {
            log.error("Unable to start producer - {}", e.getMessage());
        }
        // clean up subscribers when application stops
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
    }

    private void validateTopicName(String route) throws IOException {
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
        return createTopic(topic, 1);
    }

    @Override
    public boolean createTopic(String topic, int partitions) throws IOException {
        validateTopicName(topic);
        try {
            EventEnvelope init = PostOffice.getInstance().request(MANAGER, 20000,
                    new Kv(TYPE, CREATE), new Kv(ORIGIN, topic), new Kv(PARTITIONS, partitions));
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
            PostOffice.getInstance().request(MANAGER, 20000, new Kv(TYPE, DELETE), new Kv(ORIGIN, topic));
        } catch (TimeoutException | AppException e) {
            throw new IOException(e.getMessage());
        }
    }

    @Override
    public void publish(String topic, Map<String, String> headers, Object body) throws IOException {
        publish(topic, -1, headers, body);
    }

    @Override
    public void publish(String topic, int partition, Map<String, String> headers, Object body) throws IOException {
        validateTopicName(topic);
        EventEnvelope event = new EventEnvelope();
        if (headers.containsKey(EventProducer.EMBED_EVENT) && body instanceof byte[]) {
            sendEvent(topic, partition, event.getId(), (byte[]) body, headers.get(EventProducer.RECIPIENT));
        } else {
            PostOffice po = PostOffice.getInstance();
            event.setTo(topic);
            event.setHeaders(headers);
            event.setBody(body);
            // propagate trace info
            TraceInfo trace = po.getTrace();
            if (trace != null) {
                if (trace.route != null && event.getFrom() == null) {
                    event.setFrom(trace.route);
                }
                if (trace.id != null && trace.path != null) {
                    event.setTrace(trace.id, trace.path);
                }
            }
            sendEvent(topic, partition, event.getId(), event.toBytes(), null);
        }
    }

    private void sendEvent(String topic, int partition, String id, byte[] payload, String dest) throws IOException {
        Utility util = Utility.getInstance();
        HazelcastInstance client = HazelcastSetup.getClient();
        String realTopic = partition < 0? topic : topic+"-"+partition;
        ITopic<Map<String, Object>> iTopic = client.getReliableTopic(realTopic);
        // perform segmentation for large payload
        if (payload.length > MAX_PAYLOAD) {
            int total = (payload.length / MAX_PAYLOAD) + (payload.length % MAX_PAYLOAD == 0 ? 0 : 1);
            ByteArrayInputStream in = new ByteArrayInputStream(payload);
            for (int i = 0; i < total; i++) {
                // To distinguish from a normal payload, the segmented block MUST not have a "TO" value.
                EventEnvelope block = new EventEnvelope();
                block.setHeader(MultipartPayload.ID, id);
                block.setHeader(MultipartPayload.COUNT, String.valueOf(i + 1));
                block.setHeader(MultipartPayload.TOTAL, String.valueOf(total));
                byte[] segment = new byte[MAX_PAYLOAD];
                int size = in.read(segment);
                block.setBody(size == MAX_PAYLOAD ? segment : Arrays.copyOfRange(segment, 0, size));
                Map<String, Object> headers = new HashMap<>();
                if (dest != null) {
                    headers.put(EventProducer.RECIPIENT, dest);
                }
                Map<String, Object> event = new HashMap<>();
                event.put(HEADERS, headers);
                event.put(BODY, block.toBytes());
                iTopic.publish(event);
                totalEvents++;
                log.info("Sending block {} of {} to {}", i + 1, total, topic);
            }
        } else {
            Map<String, Object> headers = new HashMap<>();
            if (dest != null) {
                headers.put(EventProducer.RECIPIENT, dest);
            }
            Map<String, Object> event = new HashMap<>();
            event.put(HEADERS, headers);
            event.put(BODY, payload);
            iTopic.publish(event);
            totalEvents++;
        }
    }

    @Override
    public void subscribe(String topic, LambdaFunction listener, String... parameters) throws IOException {
        subscribe(topic, -1, listener, parameters);
    }

    @Override
    public void subscribe(String topic, int partition, LambdaFunction listener, String... parameters) throws IOException {
        validateTopicName(topic);
        if (parameters.length == 2 || parameters.length == 3) {
            if (parameters.length == 3 && !Utility.getInstance().isNumeric(parameters[2])) {
                throw new IOException("topic offset must be numeric");
            }
            if (Platform.getInstance().hasRoute(topic)) {
                throw new IOException(topic+" is already used");
            }
            if (subscribers.containsKey(topic)) {
                throw new IOException(topic+" is already subscribed");
            }
            EventConsumer consumer = new EventConsumer(topic, partition, parameters);
            consumer.start();
            Platform.getInstance().registerPrivate(topic, listener, 1);
            subscribers.put(topic + (partition < 0? "" : "-" + partition), consumer);
        } else {
            throw new IOException("Check parameters: clientId, groupId and optional offset pointer");
        }
    }

    @Override
    public void unsubscribe(String topic) throws IOException {
        unsubscribe(topic, -1);
    }

    @Override
    public void unsubscribe(String topic, int partition) throws IOException {
        String topicPartition = topic + (partition < 0? "" : "-" + partition);
        validateTopicName(topic);
        Platform platform = Platform.getInstance();
        if (platform.hasRoute(topic) && subscribers.containsKey(topicPartition)) {
            EventConsumer consumer = subscribers.get(topicPartition);
            platform.release(topic);
            subscribers.remove(topicPartition);
            consumer.shutdown();
        } else {
            if (partition > -1) {
                throw new IOException(topic + " at partition-" + partition +
                        " has not been subscribed by this application instance");
            } else {
                throw new IOException(topic + " has not been subscribed by this application instance");
            }
        }
    }

    @Override
    public boolean exists(String topic) throws IOException {
        validateTopicName(topic);
        try {
            EventEnvelope response = PostOffice.getInstance().request(MANAGER, 20000,
                    new Kv(TYPE, EXISTS), new Kv(ORIGIN, topic));
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
        validateTopicName(topic);
        try {
            EventEnvelope response = PostOffice.getInstance().request(MANAGER, 20000,
                    new Kv(TYPE, PARTITIONS), new Kv(ORIGIN, topic));
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
            EventEnvelope init = PostOffice.getInstance().request(MANAGER, 20000, new Kv(TYPE, LIST));
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
        // TODO
    }
}
