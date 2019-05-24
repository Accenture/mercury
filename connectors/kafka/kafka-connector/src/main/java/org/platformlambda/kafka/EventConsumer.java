/*

    Copyright 2018-2019 Accenture Technology

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

package org.platformlambda.kafka;

import org.apache.kafka.clients.consumer.ConsumerConfig;
import org.apache.kafka.clients.consumer.ConsumerRecord;
import org.apache.kafka.clients.consumer.ConsumerRecords;
import org.apache.kafka.clients.consumer.KafkaConsumer;
import org.apache.kafka.common.errors.WakeupException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.time.Duration;
import java.util.Collections;
import java.util.Properties;

public class EventConsumer extends Thread {
    private static final Logger log = LoggerFactory.getLogger(EventConsumer.class);

    private String topic;
    private KafkaConsumer<String, byte[]> consumer;
    private boolean normal = true;

    public EventConsumer(Properties base, String topic) {
        this.topic = topic;
        String origin = Platform.getInstance().getOrigin();
        Properties prop = new Properties();
        prop.putAll(base);
        // create unique ID from origin ID by dropping date prefix
        prop.put(ConsumerConfig.CLIENT_ID_CONFIG, origin.substring(8)); // drop yyyyMMdd
        prop.put(ConsumerConfig.GROUP_ID_CONFIG, origin.substring(6));  // drop yyyyMM
        prop.put(ConsumerConfig.AUTO_OFFSET_RESET_CONFIG, "latest");
        prop.put(ConsumerConfig.KEY_DESERIALIZER_CLASS_CONFIG, org.apache.kafka.common.serialization.StringDeserializer.class);
        prop.put(ConsumerConfig.VALUE_DESERIALIZER_CLASS_CONFIG, org.apache.kafka.common.serialization.ByteArrayDeserializer.class);
        this.consumer = new KafkaConsumer<>(prop);
    }

    @Override
    public void run() {
        PostOffice po = PostOffice.getInstance();
        consumer.subscribe(Collections.singletonList(topic), new ConsumerLifeCycle());
        log.info("Started. Listening to {}", topic);

        try {
            while (normal) {
                ConsumerRecords<String, byte[]> records = consumer.poll(Duration.ofMillis(5000));
                for (ConsumerRecord<String, byte[]> record : records) {
                    EventEnvelope message = new EventEnvelope();
                    try {
                        message.load(record.value());
                        message.setEndOfRoute();
                        if (message.getTo() != null) {
                            po.send(message);
                        } else {
                            MultipartPayload.getInstance().incoming(message);
                        }

                    } catch (IOException e) {
                        log.error("Unable to process incoming event - {}", e.getMessage());
                    }
                }
            }
        } catch (IllegalStateException e) {
            /*
             * We will let the cloud restarts the application instance automatically.
             * There is nothing we can do.
             */
            log.error("Unrecoverable event stream error - {}", e.getMessage());
            consumer.close();
            System.exit(-1);
        } catch (WakeupException e) {
            log.info("Stopping");
        } finally {
            consumer.close();
            log.info("Stopped");
        }
    }

    public void shutdown() {
        normal = false;
        consumer.wakeup();
    }

}
