package org.platformlambda.activemq.pubsub;

import org.platformlambda.activemq.reporter.PresenceConnector;
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
import org.platformlambda.activemq.ActiveMqSetup;
import org.platformlambda.activemq.services.TopicManager;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.jms.*;
import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;

public class ActiveMqPubSub implements PubSubProvider {
    private static final Logger log = LoggerFactory.getLogger(ActiveMqPubSub.class);

    private static final MsgPack msgPack = new MsgPack();
    private static final String MANAGER = ActiveMqSetup.MANAGER;
    private static final String PUBLISHER = PresenceConnector.PUBLISHER;
    private static final String TYPE = "type";
    private static final String STOP = "stop";
    private static final String PARTITIONS = "partitions";
    private static final String PARTITION = "partition";
    private static final String BODY = "body";
    private static final String CREATE = "create";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private static final String DELETE = "delete";
    private static final String TOPIC = "topic";
    private static final ConcurrentMap<String, EventConsumer> subscribers = new ConcurrentHashMap<>();
    private Session primarySession, secondarySession;

    @SuppressWarnings("unchecked")
    public ActiveMqPubSub() throws JMSException {
        LambdaFunction publisher = (headers, body, instance) -> {
            if (STOP.equals(headers.get(TYPE))) {
                stopConnection();
            } else if (body instanceof Map) {
                Map<String, Object> data = (Map<String, Object>) body;
                Object topic = data.get(TOPIC);
                Object partition = data.get(PARTITION);
                Object payload = data.get(BODY);
                if (topic instanceof String && partition instanceof Integer && payload instanceof byte[]) {
                    sendEvent(false, (String) topic, (int) partition, headers, (byte[]) payload);
                }
            }
            return true;
        };
        Platform platform = Platform.getInstance();
        try {
            // start Topic Manager
            platform.registerPrivate(MANAGER, new TopicManager(), 1);
            // start publisher
            platform.registerPrivate(PUBLISHER, publisher, 1);
        } catch (Exception e) {
            log.error("Unable to start producer - {}", e.getMessage());
        }
        // clean up subscribers when application stops
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
    }

    private void startSession(boolean primary) throws JMSException {
        if (primary) {
            if (primarySession == null) {
                Connection connection = ActiveMqSetup.getConnection();
                primarySession = connection.createSession(Session.AUTO_ACKNOWLEDGE);
                log.debug("Primary session started");
            }
        } else {
            if (secondarySession == null) {
                Connection connection = ActiveMqSetup.getConnection();
                secondarySession = connection.createSession(Session.AUTO_ACKNOWLEDGE);
                log.debug("Secondary session started");
            }
        }
    }

    private void stopConnection() throws JMSException {
        if (primarySession != null) {
            primarySession.close();
            primarySession = null;
            log.debug("Primary session stopped");
        }

        if (secondarySession != null) {
            secondarySession.close();
            secondarySession = null;
            log.debug("Secondary session stopped");
        }
        ActiveMqSetup.stopConnection();
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
            PostOffice.getInstance().request(MANAGER, 20000, new Kv(TYPE, DELETE), new Kv(TOPIC, topic));
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
        Utility util = Utility.getInstance();
        Map<String, String> eventHeaders = headers == null? new HashMap<>() : headers;
        if (eventHeaders.containsKey(EventProducer.EMBED_EVENT) && body instanceof byte[]) {
            // embedded events are sent by the EventPublisher thread
            sendEvent(true, topic, partition, eventHeaders, (byte[]) body);
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
            /*
             * for thread safety, tell the singleton publisher to send event
             */
            Map<String, Object> data = new HashMap<>();
            data.put(TOPIC, topic);
            data.put(PARTITION, partition);
            data.put(BODY, payload);
            EventEnvelope event = new EventEnvelope();
            event.setHeaders(eventHeaders).setBody(data).setTo(PUBLISHER);
            PostOffice.getInstance().send(event);
        }
    }

    private void sendEvent(boolean primary, String topic, int partition, Map<String, String> headers, byte[] payload) {
        String realTopic = partition < 0 ? topic : topic + "." + partition;
        try {
            startSession(primary);
            Session session = primary? primarySession : secondarySession;
            Topic destination = session.createTopic(realTopic);
            MessageProducer producer = session.createProducer(destination);
            BytesMessage message = session.createBytesMessage();
            for (String h: headers.keySet()) {
                message.setStringProperty(h, headers.get(h));
            }
            message.writeBytes(payload);
            producer.send(message);
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
        validateTopicName(topic);
        String topicPartition = topic + (partition < 0? "" : "." + partition);
        if (parameters.length == 2 || parameters.length == 3) {
            if (parameters.length == 3 && !Utility.getInstance().isNumeric(parameters[2])) {
                throw new IOException("topic offset must be numeric");
            }
            if (subscribers.containsKey(topicPartition) || Platform.getInstance().hasRoute(topicPartition)) {
                throw new IOException(topicPartition+" is already subscribed");
            }
            EventConsumer consumer = new EventConsumer(topic, partition, parameters);
            consumer.start();
            Platform.getInstance().registerPrivate(topicPartition, listener, 1);
            subscribers.put(topicPartition, consumer);
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
        validateTopicName(topic);
        String topicPartition = topic + (partition < 0? "" : "." + partition);
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
        validateTopicName(topic);
        try {
            EventEnvelope response = PostOffice.getInstance().request(MANAGER, 20000,
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
        validateTopicName(topic);
        try {
            EventEnvelope response = PostOffice.getInstance().request(MANAGER, 20000,
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
        for (String topic: subscribers.keySet()) {
            EventConsumer consumer = subscribers.get(topic);
            consumer.shutdown();
        }
    }
}
