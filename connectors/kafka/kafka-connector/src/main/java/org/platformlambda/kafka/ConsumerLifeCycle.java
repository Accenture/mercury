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
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Collection;

public class ConsumerLifeCycle implements ConsumerRebalanceListener {
    private static final Logger log = LoggerFactory.getLogger(ConsumerLifeCycle.class);

    private static final String TYPE = "type";
    private static final String INIT = "init";
    private final boolean pubSub, serviceMonitor;
    private final String topic;
    private boolean ready = false;

    public ConsumerLifeCycle(String topic, boolean pubSub) {
        this.topic = topic;
        this.pubSub = pubSub;
        AppConfigReader reader = AppConfigReader.getInstance();
        this.serviceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
    }

    public boolean isReady() {
        return ready;
    }

    @Override
    public void onPartitionsRevoked(Collection<TopicPartition> partitions) {
        if (!partitions.isEmpty()) {
            ready = false;
            log.warn("Topic {} with {} partition{} revoked", topic, partitions.size(),
                    partitions.size() == 1 ? "" : "s");
        }
    }

    @Override
    public void onPartitionsAssigned(Collection<TopicPartition> partitions) {
        if (!partitions.isEmpty()) {
            ready = true;
            log.info("Topic {} with {} partition{} is ready", topic, partitions.size(),
                    partitions.size() == 1 ? "" : "s");
            if (!pubSub) {
                // sending a loop-back message to tell the event consumer that the system is ready
                if (!serviceMonitor && TopicManager.regularTopicFormat(topic)) {
                    try {
                        PostOffice.getInstance().send(PostOffice.CLOUD_CONNECTOR, new Kv(TYPE, INIT));
                    } catch (IOException e) {
                        log.error("Unable to send initialization request to {} - {}",
                                PostOffice.CLOUD_CONNECTOR, e.getMessage());
                    }
                }
            }
        } else {
            log.warn("No partition for topic {} is available", topic);
        }
    }

}