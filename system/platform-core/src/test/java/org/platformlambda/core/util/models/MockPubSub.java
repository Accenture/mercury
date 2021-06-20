package org.platformlambda.core.util.models;

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.PubSubProvider;

import java.io.IOException;
import java.util.*;

public class MockPubSub implements PubSubProvider {

    private static final Map<String, Integer> topicStore = new HashMap<>();
    private static final Map<String, LambdaFunction> subscriptions = new HashMap<>();

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
        if (topic.equals("exception")) {
            throw new IOException("demo");
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
        if (topic.equals("exception")) {
            throw new IOException("demo");
        }
        subscriptions.put(topic, listener);
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
