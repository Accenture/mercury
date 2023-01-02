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

package org.platformlambda.lang.services;

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.PubSub;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class TopicListener implements LambdaFunction {
    private static final ConcurrentMap<String, TopicListener> listeners = new ConcurrentHashMap<>();

    private final ConcurrentMap<String, Boolean> distributionList = new ConcurrentHashMap<>();
    private final String topic, composite;
    private final int partition;

    public TopicListener(String topic, int partition, String route) {
        if (topic == null || route == null) {
            throw new IllegalArgumentException("Missing topic or route");
        }
        this.topic = topic;
        this.partition = partition;
        this.composite = getComposite(topic, partition);
        listeners.put(this.composite, this);
        addRoute(route);
    }

    private static String getComposite(String topic, int partition) {
        return topic + '#' + Math.max(-1, partition);
    }

    public static List<TopicListener> getListeners(String topic) {
        List<TopicListener> result = new ArrayList<>();
        for (String k: listeners.keySet()) {
            TopicListener listener = listeners.get(k);
            if (listener.getTopic().equals(topic)) {
                result.add(listener);
            }
        }
        return result;
    }

    public static TopicListener getListener(String topic, int partition) {
        return listeners.get(getComposite(topic, partition));
    }

    public static void releaseRoute(String route) throws IOException {
        List<String> listenerList = new ArrayList<>(listeners.keySet());
        // composite key = topic#partition
        for (String composite: listenerList) {
            TopicListener t = listeners.get(composite);
            if (t.hasRoute(route)) {
                t.removeRoute(route);
            }
        }
    }

    public boolean hasRoute(String route) {
        return distributionList.containsKey(route);
    }

    public String getTopic() {
        return topic;
    }

    public int getPartition() {
        return partition;
    }

    public List<String> getRoutes() {
        return new ArrayList<>(distributionList.keySet());
    }

    public void addRoute(String route) {
        distributionList.put(route, true);
    }

    public void removeRoute(String route) throws IOException {
        distributionList.remove(route);
        if (distributionList.isEmpty()) {
            listeners.remove(composite);
            PubSub engine = PubSub.getInstance();
            if (partition < 0) {
                engine.unsubscribe(topic);
            } else {
                engine.unsubscribe(topic, partition);
            }
        }
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        for (String route: distributionList.keySet()) {
            EventEnvelope event = new EventEnvelope();
            event.setTo(route).setBody(body);
            event.setHeaders(headers);
            // Send the event to the next available application instance that provides the target service
            PostOffice.getInstance().send(event);
        }
        // return nothing since this is asynchronous
        return null;
    }

}
