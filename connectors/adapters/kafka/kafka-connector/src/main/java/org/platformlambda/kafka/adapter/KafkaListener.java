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

package org.platformlambda.kafka.adapter;

import org.apache.kafka.clients.consumer.ConsumerConfig;
import org.apache.kafka.clients.consumer.ConsumerRecord;
import org.apache.kafka.clients.consumer.ConsumerRecords;
import org.apache.kafka.clients.consumer.KafkaConsumer;
import org.apache.kafka.common.errors.WakeupException;
import org.apache.kafka.common.header.Header;
import org.apache.kafka.common.header.Headers;
import org.apache.kafka.common.serialization.ByteArrayDeserializer;
import org.apache.kafka.common.serialization.StringDeserializer;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleObjectMapper;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.time.Duration;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Properties;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;

public class KafkaListener extends Thread {
    private static final Logger log = LoggerFactory.getLogger(KafkaListener.class);

    private static final AtomicInteger counter = new AtomicInteger(0);
    private static final String X_TRACE_ID = "x-trace-id";
    private static final String X_CONTENT_TYPE = "x-content-type";
    private static final String JSON = "json";
    private static final String EVENT = "event";
    private final boolean tracing;
    private final KafkaConsumer<String, byte[]> consumer;
    private final String topic;
    private final String target;
    private final AtomicBoolean normal = new AtomicBoolean(true);

    public KafkaListener(Properties baseProp, String topic, String target, String group, boolean tracing) {
        int n = counter.incrementAndGet();
        this.topic = topic;
        this.target = target;
        this.tracing = tracing;
        Properties prop = new Properties();
        prop.putAll(baseProp);
        prop.put(ConsumerConfig.CLIENT_ID_CONFIG, "client-"+Platform.getInstance().getOrigin()+"-"+n);
        prop.put(ConsumerConfig.GROUP_ID_CONFIG, group);
        prop.put(ConsumerConfig.KEY_DESERIALIZER_CLASS_CONFIG, StringDeserializer.class);
        prop.put(ConsumerConfig.VALUE_DESERIALIZER_CLASS_CONFIG, ByteArrayDeserializer.class);
        this.consumer = new KafkaConsumer<>(prop);
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
    }

    @Override
    public void run() {
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
        Utility util = Utility.getInstance();
        consumer.subscribe(Collections.singletonList(topic));
        log.info("Target {} subscribed to {}", target, topic);
        try {
            while (normal.get()) {
                ConsumerRecords<String, byte[]> records = consumer.poll(Duration.ofSeconds(30));
                for (ConsumerRecord<String, byte[]> rec : records) {
                    Map<String, String> headers = getSimpleHeaders(rec.headers());
                    String traceId = headers.get(X_TRACE_ID);
                    String contentType = headers.getOrDefault(X_CONTENT_TYPE, JSON);
                    byte[] data = rec.value();
                    boolean isEvent = false;
                    if (EVENT.equals(contentType)) {
                        EventEnvelope evt = new EventEnvelope();
                        try {
                            evt.load(data);
                            headers.forEach(evt::setHeader);
                            sendEvent(evt, traceId);
                            isEvent = true;
                        } catch (IOException e) {
                            log.error("Incoming message is not an event envelope");
                        }
                    }
                    if (!isEvent) {
                        EventEnvelope evt = new EventEnvelope();
                        String text = util.getUTF(data).trim();
                        if (text.startsWith("{") && text.endsWith("}")) {
                            evt.setBody(mapper.readValue(text, Map.class));
                        } else {
                            evt.setBody(data);
                        }
                        headers.forEach(evt::setHeader);
                        sendEvent(evt, traceId);
                    }
                }
            }
        } catch (WakeupException e) {
            log.info("Stopping listener for {}", topic);
        } catch (Exception e) {
            // when this happens, it is better to shut down so that infrastructure can restart the app instance.
            log.error("Event stream error for {} - {} {}", topic, e.getClass(), e.getMessage());
            System.exit(10);
        } finally {
            consumer.close();
            log.info("Unsubscribed {}", topic);
        }
    }

    private void sendEvent(EventEnvelope event, String traceId) {
        try {
            if (tracing) {
                String id = traceId == null? Utility.getInstance().getUuid() : traceId;
                PostOffice po = new PostOffice("kafka.adapter", id, "TOPIC "+topic);
                po.send(event.setTo(target));
            } else {
                EventEmitter po = EventEmitter.getInstance();
                po.send(event.setTo(target));
            }
        } catch (IOException e) {
            log.error("Unable to send event to {} - {}", target, e.getMessage());
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

    private void shutdown() {
        if (normal.get()) {
            normal.set(false);
            consumer.wakeup();
        }
    }
}
