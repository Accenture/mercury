/*

    Copyright 2018-2022 Accenture Technology

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

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.util.Utility;
import org.platformlambda.lang.websocket.server.LanguageConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Map;

public class PubSubController implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(PubSubController.class);

    private static final String TYPE = "type";
    private static final String TOPIC = "topic";
    private static final String CREATE = "create";
    private static final String DELETE = "delete";
    private static final String PUBLISH = "publish";
    private static final String SUBSCRIBE = "subscribe";
    private static final String UNSUBSCRIBE = "unsubscribe";
    private static final String FEATURE = "feature";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private static final String PARTITION = "partition";
    private static final String PARTITION_COUNT = "partition_count";
    private static final String HEADERS = "headers";
    private static final String BODY = "body";
    private static final String ROUTE = "route";

    private static final String DOMAIN = "domain";

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {

        Utility util = Utility.getInstance();
        if (headers.containsKey(TYPE) && headers.containsKey(DOMAIN)) {
            String domain = headers.get(DOMAIN);
            PubSub engine = PubSub.getInstance(domain);
            String type = headers.get(TYPE);
            if (FEATURE.equals(type)) {
                return engine.featureEnabled();
            }
            if (!engine.featureEnabled()) {
                throw new AppException(500, "Pub/sub domain ("+domain+") not enabled");
            }
            if (LIST.equals(type)) {
                return engine.list();
            }
            if (!headers.containsKey(TOPIC)) {
                return false;
            }
            String topic = headers.get(TOPIC);
            if (EXISTS.equals(type)) {
                return engine.exists(topic);
            }
            if (PARTITION_COUNT.equals(type)) {
                return engine.partitionCount(topic);
            }
            if (CREATE.equals(type)) {
                int partition = Math.max(-1, util.str2int(headers.get(PARTITION)));
                return engine.createTopic(topic, partition);
            }
            if (DELETE.equals(type)) {
                engine.deleteTopic(topic);
                return true;
            }
            if (PUBLISH.equals(type) && body instanceof Map) {
                int partition = Math.max(-1, util.str2int(headers.get(PARTITION)));
                Map<String, Object> map = (Map<String, Object>) body;
                if (map.containsKey(BODY) && map.containsKey(HEADERS)) {
                    if (partition < 0) {
                        engine.publish(topic, (Map<String, String>) map.get(HEADERS), map.get(BODY));
                    } else {
                        engine.publish(topic, partition, (Map<String, String>) map.get(HEADERS), map.get(BODY));
                    }
                    return true;
                } else {
                    return false;
                }
            }
            if (SUBSCRIBE.equals(type) && headers.containsKey(ROUTE)) {
                int partition = Math.max(-1, util.str2int(headers.get(PARTITION)));
                String route = headers.get(ROUTE);
                if (LanguageConnector.hasRoute(route)) {
                    List<String> para = body instanceof List? (List<String>) body : Collections.EMPTY_LIST;
                    boolean found = false;
                    boolean all = false;
                    List<Integer> subscribed = new ArrayList<>();
                    List<TopicListener> listeners = TopicListener.getListeners(topic);
                    for (TopicListener listener: listeners) {
                        found = true;
                        if (listener.getPartition() < 0) {
                            all = true;
                        } else {
                            subscribed.add(listener.getPartition());
                        }
                    }
                    /*
                     * Subscription to a pub/sub topic is either all available partitions or a specific partition.
                     * Otherwise, the READ offset checkpoint logic for an event stream system would break.
                     */
                    if (found) {
                        if (partition >= 0 && all) {
                            throw new IllegalArgumentException("Cannot subscribe "+route+" to partition "+partition+
                                    " because all partitions of "+topic+" have been subscribed");
                        }
                        if (partition < 0 && !subscribed.isEmpty()) {
                            throw new IllegalArgumentException("Cannot subscribe "+route+" to all partitions " +
                                    "because some partitions of topic "+topic+" has been subscribed");
                        }
                    }
                    TopicListener existing = TopicListener.getListener(topic, partition);
                    if (existing != null) {
                        existing.addRoute(route);
                        if (partition < 0) {
                            log.info("Topic {} attached to {}", topic, existing.getRoutes());
                        } else {
                            log.info("Topic {} partition {} attached to {}", topic, partition, existing.getRoutes());
                        }
                    } else {
                        TopicListener listener = new TopicListener(topic, partition, route);
                        if (partition < 0) {
                            engine.subscribe(topic, listener, getParameters(para));
                            log.info("Topic {} attached to {}", topic, route);
                        } else {
                            engine.subscribe(topic, partition, listener, getParameters(para));
                            log.info("Topic {} partition {} attached to {}", topic, partition, route);
                        }
                    }
                    return true;
                } else {
                    throw new AppException(404, "Route "+route+" not registered");
                }
            }
            if (UNSUBSCRIBE.equals(type) && headers.containsKey(ROUTE)) {
                boolean success = false;
                String route = headers.get(ROUTE);
                if (LanguageConnector.hasRoute(route)) {
                    List<TopicListener> listeners = TopicListener.getListeners(topic);
                    for (TopicListener listener: listeners) {
                        if (listener.hasRoute(route)) {
                            listener.removeRoute(route);
                            success = true;
                            int partition = listener.getPartition();
                            if (partition < 0) {
                                log.info("Topic {} detached from {}", topic, route);
                            } else {
                                log.info("Topic {} partition {} detached from {}", topic, partition, route);
                            }

                        }
                    }
                    return success;

                } else {
                    throw new AppException(404, "Route "+route+" not registered");
                }
            }
        }
        return false;
    }

    private String[] getParameters(List<String> parameters) {
        String[] result = new String[parameters.size()];
        for (int i=0; i < parameters.size(); i++) {
            result[i] = parameters.get(i);
        }
        return result;
    }
}
