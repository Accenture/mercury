package org.platformlambda.lang.services;

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.util.AppConfigReader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class TopicListener implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(TopicListener.class);

    private static final ConcurrentMap<String, TopicListener> listeners = new ConcurrentHashMap<>();

    private ConcurrentMap<String, Boolean> distributionList = new ConcurrentHashMap<>();
    private String topic;
    private static Boolean broadcast;

    public TopicListener(String topic, String route) {
        this.topic = topic;
        listeners.put(topic, this);
        addRoute(route);
        AppConfigReader reader = AppConfigReader.getInstance();
        if (broadcast == null) {
            broadcast = "true".equals(reader.getProperty("pubsub.broadcast", "false"));
            if (broadcast) {
                log.info("Pub/Sub is set to broadcast mode");
            }
        }
    }

    public static TopicListener getListener(String topic) {
        return listeners.get(topic);
    }

    public static void releaseRoute(String route) throws IOException {
        List<String> listenerList = new ArrayList<>(listeners.keySet());
        for (String topic: listenerList) {
            TopicListener t = listeners.get(topic);
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

    public List<String> getRoutes() {
        return new ArrayList<>(distributionList.keySet());
    }

    public boolean addRoute(String route) {
        if (distributionList.containsKey(route)) {
            return false;
        } else {
            distributionList.put(route, true);
            return true;
        }
    }

    public void removeRoute(String route) throws IOException {
        distributionList.remove(route);
        if (distributionList.isEmpty()) {
            listeners.remove(topic);
            PubSub engine = PubSub.getInstance();
            engine.unsubscribe(topic);
        }
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        for (String route: distributionList.keySet()) {
            EventEnvelope event = new EventEnvelope();
            event.setTo(route).setBody(body);
            for (String h : headers.keySet()) {
                event.setHeader(h, headers.get(h));
            }
            if (broadcast) {
                // broadcast to application instances that have the target route
                PostOffice.getInstance().broadcast(event);
            } else {
                // load balance among application instances
                PostOffice.getInstance().send(event);
            }
        }
        // return nothing since this is asynchronous
        return null;
    }

}
