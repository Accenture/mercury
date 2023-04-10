/*

    Copyright 2018-2023 Accenture Technology

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

package org.platformlambda.mock;

import org.platformlambda.cloud.EventProducer;
import org.platformlambda.cloud.ServiceLifeCycle;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.PubSubProvider;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.util.*;

public class MockPubSub implements PubSubProvider {
    private static final Map<String, Integer> topicStore = new HashMap<>();
    private static final Map<String, LambdaFunction> subscriptions = new HashMap<>();

    public MockPubSub() {
        Utility util = Utility.getInstance();
        ConfigReader reader = new ConfigReader();
        try {
            reader.load("classpath:/topic-substitution.yaml");
        } catch (IOException e) {
            // this should not happen
        }
        Map<String, Object> map = util.getFlatMap(reader.getMap());
        for (String item: map.keySet()) {
            topicStore.put(item, 1);
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
    public boolean createQueue(String queue) throws IOException {
        return false;
    }

    @Override
    public void deleteQueue(String queue) throws IOException {
        // no-op
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
        EventEmitter po = EventEmitter.getInstance();
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
        EventEmitter po = EventEmitter.getInstance();
        Platform platform = Platform.getInstance();
        platform.registerPrivate(route, listener, 1);
        subscriptions.put(topic, listener);
        if (parameters.length == 3 && parameters[2].equals("-100")) {
            final ServiceLifeCycle initialLoad = new ServiceLifeCycle(topic, partition, UUID.randomUUID().toString());
            initialLoad.start();
        }
    }

    @Override
    public void send(String queue, Map<String, String> headers, Object body) throws IOException {
        // no-op
    }

    @Override
    public void listen(String queue, LambdaFunction listener, String... parameters) throws IOException {
        // no-op
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
    public boolean isStreamingPubSub() {
        return true;
    }

    @Override
    public void cleanup() {
        // no-op
    }
}
