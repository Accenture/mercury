package org.platformlambda.automation.services;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.PostOffice;

import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeoutException;

public class SimpleNotification {
    private static final String NOTIFICATION_MANAGER = "notification.manager";
    private static final String TYPE = "type";
    private static final String TOPIC = "topic";
    private static final String PUBLISH = "publish";
    private static final String LIST = "list";
    private static final long TIMEOUT = 12000;
    private static final SimpleNotification instance = new SimpleNotification();

    private SimpleNotification() {
        // singleton
    }

    public static SimpleNotification getInstance() {
        return instance;
    }

    public void publish(String topic, String message) {
        PostOffice po = PostOffice.getInstance();
        if (po.exists(NOTIFICATION_MANAGER)) {
            try {
                po.send(NOTIFICATION_MANAGER, message, new Kv(TYPE, PUBLISH), new Kv(TOPIC, topic));
            } catch (IOException e) {
                throw new IllegalArgumentException(e.getMessage());
            }
        } else {
            throw new IllegalArgumentException("Notification service not reachable");
        }
    }

    @SuppressWarnings("unchecked")
    public List<String> listTopics() throws TimeoutException, AppException, IOException {
        PostOffice po = PostOffice.getInstance();
        if (po.exists(NOTIFICATION_MANAGER)) {
            EventEnvelope response = po.request(NOTIFICATION_MANAGER, TIMEOUT, new Kv(TYPE, LIST));
            Object result = response.getBody();
            return result instanceof List? (List<String>) result: new ArrayList<>();

        } else {
            throw new IllegalArgumentException("Notification service not reachable");
        }
    }

    @SuppressWarnings("unchecked")
    public Map<String, List<String>> getTopic(String topic) throws TimeoutException, AppException, IOException {
        if (topic == null) {
            throw new IllegalArgumentException("Missing topic");
        }
        PostOffice po = PostOffice.getInstance();
        if (po.exists(NOTIFICATION_MANAGER)) {
            EventEnvelope response = po.request(NOTIFICATION_MANAGER, TIMEOUT, new Kv(TYPE, LIST), new Kv(TOPIC, topic));
            Object result = response.getBody();
            return result instanceof Map? (Map<String, List<String>>) result: new HashMap<>();

        } else {
            throw new IllegalArgumentException("Notification service not reachable");
        }
    }

}
