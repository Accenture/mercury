/*

    Copyright 2018-2021 Accenture Technology

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

package org.platformlambda.core.util;

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

public class UserNotification {
    private static final String NOTIFICATION_MANAGER = "notification.manager";
    private static final String TYPE = "type";
    private static final String TOPIC = "topic";
    private static final String PUBLISH = "publish";
    private static final String LIST = "list";
    private static final long TIMEOUT = 12000;
    private static final UserNotification instance = new UserNotification();

    private UserNotification() {
        // singleton
    }

    public static UserNotification getInstance() {
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
            throw new IllegalArgumentException("Notification service not reachable - " +
                    "did you forget to deploy REST automation");
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
            throw new IllegalArgumentException("Notification service not reachable - " +
                    "did you forget to deploy REST automation");
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
            throw new IllegalArgumentException("Notification service not reachable - " +
                    "did you forget to deploy REST automation");
        }
    }

}
