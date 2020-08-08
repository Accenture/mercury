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

import org.apache.kafka.clients.consumer.ConsumerRebalanceListener;
import org.apache.kafka.common.TopicPartition;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Collection;
import java.util.concurrent.TimeoutException;

public class ConsumerLifeCycle implements ConsumerRebalanceListener {
    private static final Logger log = LoggerFactory.getLogger(ConsumerLifeCycle.class);

    private static final String TYPE = "type";
    private static final String INIT = "init";
    private final boolean pubSub;
    private final String topic;
    private static boolean ready = false, eventStreamReady = false;

    public ConsumerLifeCycle(String topic, boolean pubSub) {
        this.topic = topic;
        this.pubSub = pubSub;
    }

    public static void setEventStreamReady() {
        eventStreamReady = true;
    }

    public static boolean isReady() {
        return ready && eventStreamReady;
    }

    @Override
    public void onPartitionsRevoked(Collection<TopicPartition> partitions) {
        if (!partitions.isEmpty()) {
            log.warn("Topic {} with {} partition{} revoked", topic, partitions.size(),
                    partitions.size() == 1 ? "" : "s");
            if (!pubSub) {
                ready = false;
            }
        }
    }

    @Override
    public void onPartitionsAssigned(Collection<TopicPartition> partitions) {
        if (!partitions.isEmpty()) {
            log.info("Topic {} with {} partition{} is ready", topic, partitions.size(),
                    partitions.size() == 1 ? "" : "s");
            if (!pubSub) {
                ready = true;
                // send an initialization event to make sure the send/receive paths are ready
                if (KafkaSetup.PRESENCE_MONITOR.equals(topic) || TopicManager.regularTopicFormat(topic)) {
                    try {
                        Platform.getInstance().waitForProvider(PostOffice.CLOUD_CONNECTOR, 20);
                        PostOffice.getInstance().send(PostOffice.CLOUD_CONNECTOR, new Kv(TYPE, INIT));
                    } catch (IOException |TimeoutException e) {
                        log.error("Unable to initialize topic {} - {}", topic, e.getMessage());
                    }
                }
            }
        } else {
            log.warn("No partition for topic {} is available", topic);
        }
    }

}