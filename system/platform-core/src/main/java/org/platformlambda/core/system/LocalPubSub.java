/*

    Copyright 2018-2024 Accenture Technology

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

package org.platformlambda.core.system;

import org.platformlambda.core.annotations.CoroutineRunner;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class LocalPubSub {
    private static final Logger log = LoggerFactory.getLogger(LocalPubSub.class);

    private static final ConcurrentMap<String, List<String>> topics = new ConcurrentHashMap<>();
    private static final String MY_ROUTE = "my_route";

    private static final LocalPubSub INSTANCE = new LocalPubSub();

    private LocalPubSub() {
        // singleton
    }

    public static LocalPubSub getInstance() {
        return INSTANCE;
    }

    public void createTopic(String topic) throws IOException {
        Utility util = Utility.getInstance();
        if (util.validServiceName(topic)) {
            topics.computeIfAbsent(topic, d -> {
                log.info("Topic {} created", topic);
                return new ArrayList<>();
            });
            Platform platform = Platform.getInstance();
            platform.registerPrivate(topic, new LocalPublisher(), 1);
        } else {
            throw new IllegalArgumentException("Invalid topic name - use 0-9, a-z, period, hyphen or underscore characters");
        }
    }

    public void deleteTopic(String topic) {
        if (topic != null && topics.containsKey(topic)) {
            Platform platform = Platform.getInstance();
            platform.release(topic);
            topics.remove(topic);
            log.info("Topic {} deleted", topic);
        }
    }

    public List<String> getTopics() {
        return new ArrayList<>(topics.keySet());
    }

    public boolean topicExists(String topic) {
        return topic != null && topics.containsKey(topic);
    }

    public List<String> getSubscribers(String topic) {
        return topicExists(topic)? topics.get(topic) : Collections.emptyList();
    }

    public boolean subscribe(String topic, String memberRoute) {
        Utility util = Utility.getInstance();
        if (!util.validServiceName(topic)) {
            throw new IllegalArgumentException("Invalid topic name");
        }
        if (!util.validServiceName(memberRoute)) {
            throw new IllegalArgumentException("Invalid member route");
        }
        if (topicExists(topic)) {
            List<String> members = topics.getOrDefault(topic, new ArrayList<>());
            if (members.contains(memberRoute)) {
                log.warn("{} already subscribed to {}", memberRoute, topic);
                return false;
            } else {
                members.add(memberRoute);
                topics.put(topic, members);
                log.info("{} subscribed to {}", memberRoute, topic);
                return true;
            }
        } else {
            throw new IllegalArgumentException("Topic "+topic+" does not exist");
        }
    }

    public void unsubscribe(String topic, String memberRoute) {
        if (topicExists(topic)) {
            List<String> members = topics.get(topic);
            if (members.contains(memberRoute)) {
                members.remove(memberRoute);
                topics.put(topic, members);
                log.info("{} unsubscribed from {}", memberRoute, topic);
            }
        }
    }

    @CoroutineRunner
    @EventInterceptor
    public static class LocalPublisher implements TypedLambdaFunction<EventEnvelope, Void> {

        @Override
        public Void handleEvent(Map<String, String> headers, EventEnvelope input, int instance) throws Exception {
            String myRoute = headers.getOrDefault(MY_ROUTE, "?");
            List<String> members = topics.get(myRoute);
            if (members != null && !members.isEmpty()) {
                PostOffice po = new PostOffice(headers, instance);
                for (String target: members) {
                    if (po.exists(target)) {
                        try {
                            po.send(input.copy().setTo(target));
                        } catch (Exception e) {
                            log.debug("Unable to relay {} -> {} - {}", myRoute, target, e.getMessage());
                        }
                    }
                }
            }
            return null;
        }
    }
}


