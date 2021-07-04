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

package org.platformlambda.hazelcast.services;

import com.hazelcast.core.HazelcastInstance;
import com.hazelcast.topic.ITopic;
import com.hazelcast.topic.Message;
import com.hazelcast.topic.MessageListener;
import org.platformlambda.cloud.EventProducer;
import org.platformlambda.cloud.ServiceLifeCycle;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.platformlambda.hazelcast.HazelcastConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.UUID;

public class EventConsumer {
    private static final Logger log = LoggerFactory.getLogger(EventConsumer.class);

    private static final MsgPack msgPack = new MsgPack();
    private static final String TYPE = ServiceLifeCycle.TYPE;
    private static final String INIT = ServiceLifeCycle.INIT;
    private static final String DONE = "done";
    private static final String TOKEN = ServiceLifeCycle.TOKEN;
    private static final long INITIALIZE = ServiceLifeCycle.INITIALIZE;
    private static final String HEADERS = "headers";
    private static final String BODY = "body";
    private static final String MONITOR = "monitor";
    private static final String TO_MONITOR = "@"+MONITOR;
    private static final String COMPLETION = "completion.";
    private static final String STOP = "stop";
    private final String INIT_TOKEN = UUID.randomUUID().toString();
    private final String topic;
    private final int partition;
    private int skipped = 0;
    private long offset = -1;
    private UUID registrationId;
    private ITopic<Map<String, Object>> iTopic;

    public EventConsumer(String topic, int partition, String... parameters) {
        this.topic = topic;
        this.partition = partition;
        Utility util = Utility.getInstance();
        /*
         * Ignore groupId and clientId as they are specific to Kafka only.
         * Just detect if INITIALIZE is provided.
         */
        if (parameters != null) {
            for (String p: parameters) {
                long offset = util.str2long(p);
                if (offset == INITIALIZE) {
                    this.offset = INITIALIZE;
                    break;
                }
            }
        }
    }

    public void start() throws IOException {
        if (offset == INITIALIZE) {
            ServiceLifeCycle initialLoad = new ServiceLifeCycle(topic, partition, INIT_TOKEN);
            initialLoad.start();
        }
        PostOffice po = PostOffice.getInstance();
        Platform platform = Platform.getInstance();
        HazelcastInstance client = HazelcastConnector.getClient();
        String realTopic = partition < 0? topic : topic+"."+partition;
        iTopic = client.getReliableTopic(realTopic);
        registrationId = iTopic.addMessageListener(new EventListener());
        String completionHandler = COMPLETION + realTopic;
        LambdaFunction f = (headers, body, instance) -> {
            iTopic.removeMessageListener(registrationId);
            platform.release(completionHandler);
            log.info("Event consumer for {} closed", realTopic);
            if (offset == INITIALIZE) {
                String INIT_HANDLER = INIT + "." + (partition < 0 ? topic : topic + "." + partition);
                if (Platform.getInstance().hasRoute(INIT_HANDLER)) {
                    try {
                        po.send(INIT_HANDLER, DONE);
                    } catch (IOException e) {
                        // ok to ignore
                    }
                }
            }
            return true;
        };
        platform.registerPrivate(completionHandler, f, 1);
        log.info("Event consumer {} for {} started", registrationId, realTopic);
    }

    public void shutdown() {
        String realTopic = partition < 0? topic : topic+"."+partition;
        String completionHandler = COMPLETION + realTopic;
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        if (platform.hasRoute(completionHandler)) {
            try {
                po.send(completionHandler, STOP);
            } catch (IOException e) {
                log.error("Unable to close consumer - {}", e.getMessage());
            }
        }
    }

    private class EventListener implements MessageListener<Map<String, Object>> {
        private final String topicPartition = partition < 0? topic : topic+"."+partition;

        @SuppressWarnings("unchecked")
        @Override
        public void onMessage(Message<Map<String, Object>> evt) {
            Utility util = Utility.getInstance();
            PostOffice po = PostOffice.getInstance();
            String origin = Platform.getInstance().getOrigin();
            Map<String, Object> event = evt.getMessageObject();
            Object h = event.get(HEADERS);
            Object p = event.get(BODY);
            if (h instanceof Map && p instanceof byte[]) {
                Map<String, String> originalHeaders = getSimpleHeaders((Map<String, Object>) h);
                byte[] data = (byte[]) p;
                String dataType = originalHeaders.getOrDefault(EventProducer.DATA_TYPE, EventProducer.BYTES_DATA);
                boolean embedEvent = originalHeaders.containsKey(EventProducer.EMBED_EVENT);
                String recipient = originalHeaders.get(EventProducer.RECIPIENT);
                if (recipient != null && recipient.contains(MONITOR)) {
                    recipient = null;
                }
                if (recipient != null && !recipient.equals(origin)) {
                    log.error("Skipping record because it belongs to {}", recipient);
                    return;
                }
                EventEnvelope message = new EventEnvelope();
                if (embedEvent) {
                    try {
                        message.load(data);
                        message.setEndOfRoute();
                    } catch (Exception e) {
                        log.error("Unable to decode incoming event for {} - {}", topicPartition, e.getMessage());
                        return;
                    }
                    try {
                        String to = message.getTo();
                        if (to != null) {
                            // remove special routing qualifier for presence monitor events
                            if (to.contains(TO_MONITOR)) {
                                message.setTo(to.substring(0, to.indexOf(TO_MONITOR)));
                            }
                            PostOffice.getInstance().send(message);
                        } else {
                            MultipartPayload.getInstance().incoming(message);
                        }
                    } catch (Exception e) {
                        log.error("Unable to process incoming event for {} - {}", topicPartition, e.getMessage());
                    }
                } else {
                    if (offset == INITIALIZE) {
                        if (INIT.equals(originalHeaders.get(TYPE)) &&
                                INIT_TOKEN.equals(originalHeaders.get(TOKEN))) {
                            offset = -1;
                            if (skipped > 0) {
                                log.info("Skipped {} outdated event{}", skipped, skipped == 1 ? "" : "s");
                            }
                        } else {
                            skipped++;
                            return;
                        }
                    }
                    // transport the headers and payload in original form
                    try {
                        if (EventProducer.TEXT_DATA.equals(dataType)) {
                            message.setHeaders(originalHeaders).setBody(util.getUTF(data));
                        } else if (EventProducer.MAP_DATA.equals(dataType) || EventProducer.LIST_DATA.equals(dataType)) {
                            message.setHeaders(originalHeaders).setBody(msgPack.unpack(data));
                        } else {
                            message.setHeaders(originalHeaders).setBody(data);
                        }
                        // mercury service name must be lower case
                        po.send(message.setTo(topicPartition.toLowerCase()));
                    } catch (Exception e) {
                        log.error("Unable to process incoming event for {} - {}", topicPartition, e.getMessage());
                    }
                }
            }
        }
    }

    private Map<String, String> getSimpleHeaders(Map<String, Object> headers) {
        Map<String, String> result = new HashMap<>();
        for (String h: headers.keySet()) {
            result.put(h, headers.get(h).toString());
        }
        return result;
    }

}
