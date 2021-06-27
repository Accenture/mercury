package org.platformlambda.mock;

import org.platformlambda.cloud.EventProducer;
import org.platformlambda.cloud.ServiceLifeCycle;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.PubSubProvider;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class MockPubSub implements PubSubProvider {
    private static final Map<String, Integer> topicStore = new HashMap<>();
    private static final Map<String, LambdaFunction> subscriptions = new HashMap<>();

    private static final ConcurrentMap<String, ServiceLifeCycle> lifeCycles = new ConcurrentHashMap<>();

    public static void stop() {
        for (String route: lifeCycles.keySet()) {
            ServiceLifeCycle c = lifeCycles.get(route);
            c.complete();
        }
    }

    @Override
    public boolean createTopic(String topic) throws IOException {
        if (topic.equals("exception")) {
            throw new IOException("demo");
        }
        topicStore.put(topic, 1);
        return true;
    }

    @Override
    public boolean createTopic(String topic, int partitions) throws IOException {
        if (topic.equals("exception")) {
            throw new IOException("demo");
        }
        topicStore.put(topic, partitions);
        return true;
    }

    @Override
    public void deleteTopic(String topic) throws IOException {
        if (topic.equals("exception")) {
            throw new IOException("demo");
        }
        topicStore.remove(topic);
    }

    @Override
    public void publish(String topic, Map<String, String> headers, Object body) throws IOException {
        if (topic.equals("exception")) {
            throw new IOException("demo");
        }
    }

    @Override
    public void publish(String topic, int partition, Map<String, String> headers, Object body) throws IOException {
        String route = topic+"."+partition;
        PostOffice po = PostOffice.getInstance();
        Map<String, String> eventHeaders = headers == null? new HashMap<>() : headers;
        if (eventHeaders.containsKey(EventProducer.EMBED_EVENT) && body instanceof byte[]) {
            EventEnvelope event = new EventEnvelope();
            event.load((byte[]) body);
            String to = event.getTo();
            int sep = to.indexOf("@monitor");
            po.send(sep > 1? event.setTo(to.substring(0, sep)) : event);
        } else {
            po.send(new EventEnvelope().setTo(route).setHeaders(headers).setBody(body));
        }
    }

    @Override
    public void subscribe(String topic, LambdaFunction listener, String... parameters) throws IOException {
        if (topic.equals("exception")) {
            throw new IOException("demo");
        }
        subscriptions.put(topic, listener);
    }

    @Override
    public void subscribe(String topic, int partition, LambdaFunction listener, String... parameters) throws IOException {
        String route = topic+"."+partition;
        Platform platform = Platform.getInstance();
        platform.registerPrivate(route, listener, 1);
        subscriptions.put(topic, listener);
        if (parameters.length == 3 && parameters[2].equals("-100")) {
            final ServiceLifeCycle initialLoad = new ServiceLifeCycle(topic, partition, UUID.randomUUID().toString());
            initialLoad.start();
            lifeCycles.put(route, initialLoad);
            LambdaFunction f = (headers, body, instance) -> {
                initialLoad.complete();
                return true;
            };
            platform.registerPrivate(route+".init", f, 1);
            PostOffice.getInstance().sendLater(new EventEnvelope().setTo(route+".init").setBody("done"),
                    new Date(System.currentTimeMillis()+8000));
        }
    }

    @Override
    public void unsubscribe(String topic) throws IOException {
        if (topic.equals("exception")) {
            throw new IOException("demo");
        }
        subscriptions.remove(topic);
    }

    @Override
    public void unsubscribe(String topic, int partition) throws IOException {
        if (topic.equals("exception")) {
            throw new IOException("demo");
        }
        subscriptions.remove(topic);
    }

    @Override
    public boolean exists(String topic) throws IOException {
        if (topic.equals("exception")) {
            throw new IOException("demo");
        }
        return topicStore.containsKey(topic);
    }

    @Override
    public int partitionCount(String topic) throws IOException {
        if (topic.equals("exception")) {
            throw new IOException("demo");
        }
        return topicStore.getOrDefault(topic, -1);
    }

    @Override
    public List<String> list() throws IOException {
        return new ArrayList<>(topicStore.keySet());
    }

    @Override
    public boolean isNativePubSub() {
        return true;
    }
}
