/*

    Copyright 2018-2020 Accenture Technology

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
import org.apache.kafka.common.TopicPartition;
import org.apache.kafka.common.errors.WakeupException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.time.Duration;
import java.util.Collections;
import java.util.Properties;
import java.util.Set;

public class EventConsumer extends Thread {
    private static final Logger log = LoggerFactory.getLogger(EventConsumer.class);

    private static final long POLL_SECONDS = 60;
    private String topic;
    private KafkaConsumer<String, byte[]> consumer;
    private boolean normal = true, pubSub = false;
    private long offset = -1;

    public EventConsumer(Properties base, String topic) {
        initialize(base, topic, false);
    }

    public EventConsumer(Properties base, String topic, boolean pubSub, String... parameters) {
        initialize(base, topic, pubSub, parameters);
    }

    private void initialize(Properties base, String topic, boolean pubSub, String... parameters) {
        AppConfigReader reader = AppConfigReader.getInstance();
        boolean isServiceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
        this.pubSub = pubSub;
        this.topic = topic;
        String origin = Platform.getInstance().getOrigin();
        Properties prop = new Properties();
        prop.putAll(base);
        // create unique values for client ID and group ID
        if (pubSub) {
            if (parameters.length == 2 || parameters.length == 3) {
                prop.put(ConsumerConfig.CLIENT_ID_CONFIG, parameters[0]);
                prop.put(ConsumerConfig.GROUP_ID_CONFIG, parameters[1]);
                /*
                 * If offset is not given, the consumer will read from the latest when it is started for the first time.
                 * Subsequent restart of the consumer will resume read from the current offset.
                 */
                if (parameters.length == 3) {
                    Utility util = Utility.getInstance();
                    long value = util.str2int(parameters[2]);
                    if (value > -1) {
                        offset = value;
                    }
                }
            } else {
                throw new IllegalArgumentException("Unable to start consumer for topic "+topic+" - number of parameters must be 2 or 3");
            }
        } else {
            /*
             * For presence monitor, we want to skip offset to the latest.
             * For regular applications, we want to read from the beginning.
             */
            prop.put(ConsumerConfig.CLIENT_ID_CONFIG, origin.substring(8));
            prop.put(ConsumerConfig.GROUP_ID_CONFIG, origin.substring(6));
            prop.put(ConsumerConfig.AUTO_OFFSET_RESET_CONFIG, isServiceMonitor? "latest" : "earliest");
            log.info("{} = {}", ConsumerConfig.AUTO_OFFSET_RESET_CONFIG, prop.getProperty(ConsumerConfig.AUTO_OFFSET_RESET_CONFIG));
        }
        prop.put(ConsumerConfig.KEY_DESERIALIZER_CLASS_CONFIG, org.apache.kafka.common.serialization.StringDeserializer.class);
        prop.put(ConsumerConfig.VALUE_DESERIALIZER_CLASS_CONFIG, org.apache.kafka.common.serialization.ByteArrayDeserializer.class);
        this.consumer = new KafkaConsumer<>(prop);
    }

    @Override
    public void run() {
        PostOffice po = PostOffice.getInstance();
        consumer.subscribe(Collections.singletonList(topic), new ConsumerLifeCycle(topic));
        log.info("Subscribed topic {}", topic);

        if (pubSub && offset > -1) {
            consumer.poll(Duration.ofSeconds(POLL_SECONDS));
            Set<TopicPartition> p = consumer.assignment();
            for (TopicPartition tp: p) {
                consumer.seek(tp, offset);
                log.info("Setting read pointer for topic {}, partition-{} to {}", topic, tp.partition(), offset);
            }
        }
        try {
            while (normal) {
                ConsumerRecords<String, byte[]> records = consumer.poll(Duration.ofSeconds(POLL_SECONDS));
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
                        log.error("Unable to process incoming event for {} - {}", topic, e.getMessage());
                    }
                }
            }
        } catch (IllegalStateException e) {
            /*
             * We will let the cloud restarts the application instance automatically.
             * There is nothing we can do.
             */
            log.error("Unrecoverable event stream error for {} - {}", topic, e.getMessage());
            consumer.close();
            System.exit(-1);
        } catch (WakeupException e) {
            log.info("Stopping listener for {}", topic);
        } finally {
            consumer.close();
            log.info("Unsubscribed topic {}", topic);
        }
    }

    public void shutdown() {
        if (normal) {
            normal = false;
            consumer.wakeup();
        }
    }

}
