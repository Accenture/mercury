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

package org.platformlambda.kafka.services;

import org.apache.kafka.clients.consumer.ConsumerConfig;
import org.apache.kafka.clients.consumer.ConsumerRecord;
import org.apache.kafka.clients.consumer.ConsumerRecords;
import org.apache.kafka.clients.consumer.KafkaConsumer;
import org.apache.kafka.common.TopicPartition;
import org.apache.kafka.common.errors.WakeupException;
import org.apache.kafka.common.header.Header;
import org.apache.kafka.common.header.Headers;
import org.apache.kafka.common.serialization.ByteArrayDeserializer;
import org.apache.kafka.common.serialization.StringDeserializer;
import org.platformlambda.cloud.ConnectorConfig;
import org.platformlambda.cloud.EventProducer;
import org.platformlambda.cloud.ServiceLifeCycle;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.time.Duration;
import java.util.*;
import java.util.concurrent.atomic.AtomicBoolean;

public class EventConsumer extends Thread {
    private static final Logger log = LoggerFactory.getLogger(EventConsumer.class);

    private static final MsgPack msgPack = new MsgPack();
    private static final String OFFSET = "_offset_";
    private static final String PARTITION = "_partition_";
    private static final String KEY = "_key_";
    private static final String TIMESTAMP = "_timestamp_";
    private static final String TYPE = ServiceLifeCycle.TYPE;
    private static final String INIT = ServiceLifeCycle.INIT;
    private static final String DONE = "done";
    private static final String TOKEN = ServiceLifeCycle.TOKEN;
    private static final long INITIALIZE = ServiceLifeCycle.INITIALIZE;
    private static final String MONITOR = "monitor";
    private static final String TO_MONITOR = "@"+MONITOR;
    private final String INIT_TOKEN = UUID.randomUUID().toString();
    private final String topic, realTopic;
    private final int partition;
    private int realPartition;
    private final KafkaConsumer<String, byte[]> consumer;
    private final AtomicBoolean normal = new AtomicBoolean(true);
    private int skipped = 0;
    private long offset = -1;

    public EventConsumer(Properties base, String topic, int partition, String... parameters) throws IOException {
        Utility util = Utility.getInstance();
        boolean substitute = ConnectorConfig.topicSubstitutionEnabled();
        Map<String, String> preAllocatedTopics = ConnectorConfig.getTopicSubstitution();
        this.topic = topic;
        this.partition = partition;
        if (substitute) {
            String virtualTopic = topic + (partition < 0? "" : "." + partition);
            String topicPartition = topic + (partition < 0? "" : "#" + partition);
            topicPartition = preAllocatedTopics.getOrDefault(virtualTopic, topicPartition);
            int sep = topicPartition.lastIndexOf('#');
            if (sep == -1) {
                this.realTopic = topicPartition;
                this.realPartition = -1;
            } else {
                this.realTopic = topicPartition.substring(0, sep);
                this.realPartition = util.str2int(topicPartition.substring(sep+1));
            }
        } else {
            this.realTopic = topic;
            this.realPartition = partition;
        }
        Properties prop = new Properties();
        prop.putAll(base);
        // create unique values for client ID and group ID
        if (parameters.length == 2 || parameters.length == 3) {
            prop.put(ConsumerConfig.CLIENT_ID_CONFIG, parameters[0]);
            prop.put(ConsumerConfig.GROUP_ID_CONFIG, parameters[1]);
            /*
             * If offset is not given, the consumer will read from the latest when it is started for the first time.
             * Subsequent restart of the consumer will resume read from the current offset.
             */
            if (parameters.length == 3) {
                long v = util.str2long(parameters[2]);
                offset = INITIALIZE == v? v : Math.max(-1, v);
            }
        } else {
            throw new IllegalArgumentException("Unable to start consumer for " + realTopic +
                                                " - parameters must be clientId, groupId and an optional offset");
        }
        prop.put(ConsumerConfig.KEY_DESERIALIZER_CLASS_CONFIG, StringDeserializer.class);
        prop.put(ConsumerConfig.VALUE_DESERIALIZER_CLASS_CONFIG, ByteArrayDeserializer.class);
        this.consumer = new KafkaConsumer<>(prop);
    }

    private long getEarliest(TopicPartition tp) {
        Map<TopicPartition, Long> data = consumer.beginningOffsets(Collections.singletonList(tp));
        return data.get(tp);
    }

    private long getLatest(TopicPartition tp) {
        Map<TopicPartition, Long> data = consumer.endOffsets(Collections.singletonList(tp));
        return data.get(tp);
    }

    @Override
    public void run() {
        final boolean init = offset == INITIALIZE;
        if (init) {
            /*
             * IMPORTANT
             * ---------
             * Kafka will do load balancing for different consumers of the same group.
             * Mercury system topics require direct assignment to an exact partition to
             * enable broadcast instead of load balancing.
             *
             * Therefore, we are setting partition to 0 if none is given.
             */
            if (ConnectorConfig.topicSubstitutionEnabled() && realPartition < 0) {
                realPartition = 0;
            }
            ServiceLifeCycle initialLoad = new ServiceLifeCycle(topic, partition, INIT_TOKEN);
            initialLoad.start();
        }
        final int INVALID_EVENT_THRESHOLD = 150;
        int invalidEvents = 0;
        boolean reset = true;
        String origin = Platform.getInstance().getOrigin();
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        String virtualTopic = (topic + (partition < 0? "" : "." + partition)).toLowerCase();
        String topicPartition = realTopic + (realPartition < 0? "" : "." + realPartition);
        if (realPartition < 0) {
            consumer.subscribe(Collections.singletonList(realTopic));
        } else {
            consumer.assign(Collections.singletonList(new TopicPartition(realTopic, realPartition)));
        }
        log.info("Subscribed {}", topicPartition);
        try {
            while (normal.get()) {
                long interval = reset? 15 : 30;
                ConsumerRecords<String, byte[]> records = consumer.poll(Duration.ofSeconds(interval));
                if (reset) {
                    Set<TopicPartition> p = consumer.assignment();
                    if (p.isEmpty()) {
                        // wait until a partition is assigned
                        continue;
                    }
                    reset = false;
                    if (offset != -1) {
                        if (p.size() == 1) {
                            for (TopicPartition tp : p) {
                                long earliest = getEarliest(tp);
                                long latest = getLatest(tp);
                                if (offset < 0) {
                                    consumer.seek(tp, latest);
                                    log.info("Setting '{}' READ offset, partition-{} to latest ({} - {})",
                                            realTopic, tp.partition(), earliest, latest);
                                } else if (offset < earliest) {
                                    consumer.seek(tp, earliest);
                                    log.warn("Setting '{}' READ offset, partition-{} to earliest instead of {} ({} - {})",
                                            realTopic, tp.partition(), offset, earliest, latest);
                                } else if (offset < latest) {
                                    consumer.seek(tp, offset);
                                    log.info("Setting '{}' READ offset, partition-{} to {} ({} - {})",
                                            realTopic, tp.partition(), offset, earliest, latest);
                                } else {
                                    consumer.seek(tp, latest);
                                    if (latest == offset) {
                                        log.info("Setting '{}' READ offset, partition-{} to latest ({} - {})",
                                                realTopic, tp.partition(), earliest, latest);
                                    } else {
                                        log.warn("Setting '{}' READ offset, partition-{} to latest instead of {} ({} - {})",
                                                realTopic, tp.partition(), offset, earliest, latest);
                                    }
                                }
                            }
                            continue;
                        } else if (partition == -1) {
                            log.warn("Unable to override READ offset to {} because there are more than one partitions." +
                                    " Number of partitions assigned: {}", offset, p.size());
                        }
                    }
                }
                for (ConsumerRecord<String, byte[]> record : records) {
                    Map<String, String> originalHeaders = getSimpleHeaders(record.headers());
                    String dataType = originalHeaders.getOrDefault(EventProducer.DATA_TYPE, EventProducer.BYTES_DATA);
                    boolean embedEvent = originalHeaders.containsKey(EventProducer.EMBED_EVENT);
                    String recipient = originalHeaders.get(EventProducer.RECIPIENT);
                    if (recipient != null && !recipient.contains(MONITOR) && !recipient.equals(origin)) {
                        /*
                         * this is an error case when two consumers listen to the same partition
                         * or when READ offset is incorrect
                         */
                        log.error("Skipping record {} because it belongs to {}", record.offset(), recipient);
                        if (++invalidEvents > INVALID_EVENT_THRESHOLD) {
                            throw new IOException("Too many outdated events - likely to be a READ offset error");
                        }
                        continue;
                    }
                    byte[] data = record.value();
                    EventEnvelope message = new EventEnvelope();
                    if (embedEvent) {
                        // payload is an embedded event
                        try {
                            message.load(data);
                            message.setEndOfRoute();
                        } catch (Exception e) {
                            log.error("Unable to decode incoming event for {} - {}", topicPartition, e.getMessage());
                            continue;
                        }
                        try {
                            String to = message.getTo();
                            if (to != null) {
                                // remove special routing qualifier for presence monitor events
                                if (to.contains(TO_MONITOR)) {
                                    message.setTo(to.substring(0, to.indexOf(TO_MONITOR)));
                                }
                                po.send(message);
                            } else {
                                MultipartPayload.getInstance().incoming(message);
                            }
                        } catch (Exception e) {
                            log.error("Unable to process incoming event for {} - {} {}",
                                    topicPartition, e.getClass().getSimpleName(), e.getMessage());
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
                                continue;
                            }
                        }
                        // transport the headers and payload in original form
                        try {
                            if (EventProducer.TEXT_DATA.equals(dataType)) {
                                message.setHeaders(originalHeaders).setBody(util.getUTF(data));
                            } else if (EventProducer.MAP_DATA.equals(dataType) ||
                                    EventProducer.LIST_DATA.equals(dataType)) {
                                message.setHeaders(originalHeaders).setBody(msgPack.unpack(data));
                            } else {
                                message.setHeaders(originalHeaders).setBody(data);
                            }
                            /*
                             * Offset is only meaningful when listening to a specific partition.
                             * This allows user application to reposition offset when required.
                             *
                             * For direct pub/sub use, kafka specific metadata are encoded in:
                             * _key_, _timestamp_, _partition_ and _offset_
                             */
                            message.setHeader(KEY, record.key());
                            message.setHeader(TIMESTAMP, record.timestamp());
                            message.setHeader(PARTITION, record.partition());
                            message.setHeader(OFFSET, record.offset());

                            po.send(message.setTo(virtualTopic));

                        } catch (Exception e) {
                            log.error("Unable to process incoming event for {} - {} {}",
                                    topicPartition, e.getClass().getSimpleName(), e.getMessage());
                        }
                    }
                }
            }
        } catch (Exception e) {
            if (e instanceof WakeupException) {
                log.info("Stopping listener for {}", virtualTopic);
            } else {
                // when this happens, it is better to shut down so that infrastructure can restart the app instance.
                log.error("Event stream error for {} - {} {}", topicPartition, e.getClass(), e.getMessage());
                System.exit(10);
            }
        } finally {
            consumer.close();
            log.info("Unsubscribed {}", topicPartition);
            String INIT_HANDLER = INIT + "." + (partition < 0 ? topic : topic + "." + partition);
            if (init && platform.hasRoute(INIT_HANDLER)) {
                try {
                    po.send(INIT_HANDLER, DONE);
                } catch (IOException e) {
                    // ok to ignore
                }
            }
        }
    }

    private Map<String, String> getSimpleHeaders(Headers headers) {
        Utility util = Utility.getInstance();
        Map<String, String> result = new HashMap<>();
        for (Header h: headers) {
            result.put(h.key(), util.getUTF(h.value()));
        }
        return result;
    }

    public void shutdown() {
        if (normal.get()) {
            normal.set(false);
            consumer.wakeup();
        }
    }

}
