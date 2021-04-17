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

package org.platformlambda.activemq.pubsub;

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.platformlambda.activemq.ActiveMqSetup;
import org.platformlambda.activemq.InitialLoad;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.jms.*;
import java.util.Enumeration;
import java.util.HashMap;
import java.util.Map;
import java.util.UUID;

public class EventConsumer extends Thread {
    private static final Logger log = LoggerFactory.getLogger(EventConsumer.class);

    private static final MsgPack msgPack = new MsgPack();
    private static final String TYPE = InitialLoad.TYPE;
    private static final String INIT = InitialLoad.INIT;
    private static final String TOKEN = InitialLoad.TOKEN;
    private static final long INITIALIZE = InitialLoad.INITIALIZE;
    private static final String MONITOR = "monitor";
    private static final String TO_MONITOR = "@"+MONITOR;
    private final String INIT_TOKEN = UUID.randomUUID().toString();
    private final String topic;
    private final int partition;
    private InitialLoad initialLoad;
    private int skipped = 0;
    private long offset = -1;
    private Session session;
    private MessageConsumer messageConsumer;

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

    @Override
    public void run() {
        if (offset == INITIALIZE) {
            initialLoad = new InitialLoad(topic, partition, INIT_TOKEN);
            initialLoad.start();
        }
        String realTopic = partition < 0 ? topic : topic + "." + partition;
        try {
            Connection connection = ActiveMqSetup.getConnection();
            session = connection.createSession(Session.AUTO_ACKNOWLEDGE);
            Topic destination = session.createTopic(realTopic);
            messageConsumer = session.createConsumer(destination);
            messageConsumer.setMessageListener(new EventListener());

        } catch (JMSException e) {
            log.error("Unable to start - {}", e.getMessage());
            System.exit(-1);
        }
        log.info("Event consumer for {} started", realTopic);
    }

    public void shutdown() {
        if (messageConsumer != null && session != null) {
            try {
                messageConsumer.close();
                session.close();
                log.info("Event consumer for {} closed", partition < 0 ? topic : topic + "." + partition);
            } catch (JMSException e) {
                log.error("Unable to close consumer - {}", e.getMessage());
            }
            messageConsumer = null;
            session = null;
        }
    }

    private class EventListener implements MessageListener {

        private final String consumerTopic = topic + (partition < 0? "" : "." + partition);

        @SuppressWarnings("unchecked")
        @Override
        public void onMessage(Message evt) {
            Utility util = Utility.getInstance();
            PostOffice po = PostOffice.getInstance();
            String origin = Platform.getInstance().getOrigin();
            try {
                Enumeration<String> headerNames = evt.getPropertyNames();
                Map<String, String> originalHeaders = new HashMap<>();
                while (headerNames.hasMoreElements()) {
                    String h = headerNames.nextElement();
                    String value = evt.getStringProperty(h);
                    originalHeaders.put(h, value);
                }
                if (evt instanceof BytesMessage) {
                    BytesMessage b = (BytesMessage) evt;
                    int len = (int) b.getBodyLength();
                    byte[] data = new byte[len];
                    b.readBytes(data);
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
                            log.error("Unable to decode incoming event for {} - {}", topic, e.getMessage());
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
                            log.error("Unable to process incoming event for {} - {}", topic, e.getMessage());
                        }
                    } else {
                        if (offset == INITIALIZE) {
                            if (INIT.equals(originalHeaders.get(TYPE)) &&
                                    INIT_TOKEN.equals(originalHeaders.get(TOKEN))) {
                                initialLoad.complete();
                                initialLoad = null;
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
                            po.send(message.setTo(consumerTopic));
                        } catch (Exception e) {
                            log.error("Unable to process incoming event for {} - {}", topic, e.getMessage());
                        }
                    }
                }

            } catch (JMSException e) {
                log.error("Unable to process incoming event - {}", e.getMessage());
            }
        }
    }

}
