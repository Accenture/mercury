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

package org.platformlambda.hazelcast.services;

import com.hazelcast.core.HazelcastInstance;
import com.hazelcast.map.IMap;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.util.Utility;
import org.platformlambda.hazelcast.HazelcastConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;

public class TopicManager implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(TopicManager.class);

    private static final MsgPack msgPack = new MsgPack();
    private static final String TYPE = "type";
    private static final String PARTITIONS = "partitions";
    private static final String CREATED = "created";
    private static final String TOPIC = "topic";
    private static final String CREATE = "create";
    private static final String DELETE = "delete";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private static final String TOPIC_STORE = "/topics";
    private final String domain;
    private final Properties properties;

    public TopicManager(String domain, Properties properties) {
        this.domain = domain;
        this.properties = properties;
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object input, int instance) throws IOException {
        if (headers.containsKey(TYPE)) {
            if (LIST.equals(headers.get(TYPE))) {
                return listTopics();
            }
            if (EXISTS.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                String origin = headers.get(TOPIC);
                return topicExists(origin);
            }
            if (PARTITIONS.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                String origin = headers.get(TOPIC);
                return topicPartitions(origin);
            }
            if (CREATE.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                int partitions = headers.containsKey(PARTITIONS)?
                                    Math.max(1, Utility.getInstance().str2int(headers.get(PARTITIONS))) : 1;
                createTopic(headers.get(TOPIC), partitions);
                return true;
            }
            if (DELETE.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                String origin = headers.get(TOPIC);
                if (topicExists(origin)) {
                    deleteTopic(origin);
                }
                return true;
            }
        }
        return false;
    }

    private boolean topicExists(String topic) {
        HazelcastInstance client = HazelcastConnector.getClient(domain, properties);
        IMap<String, byte[]> map = client.getMap(TOPIC_STORE);
        return map.containsKey(topic);
    }

    @SuppressWarnings("unchecked")
    private int topicPartitions(String topic) {
        if (topicExists(topic)) {
            try {
                Utility util = Utility.getInstance();
                HazelcastInstance client = HazelcastConnector.getClient(domain, properties);
                IMap<String, byte[]> map = client.getMap(TOPIC_STORE);
                Map<String, Object> topicMap = (Map<String, Object>) msgPack.unpack(map.get(topic));
                return util.str2int(topicMap.getOrDefault(PARTITIONS, 1).toString());
            } catch (IOException e) {
                log.error("Unable to retrieve {} - {}", topic, e.getMessage());
            }
        }
        return -1;
    }

    private void createTopic(String topic, int partitions) throws IOException {
        if (!topicExists(topic)) {
            HazelcastInstance client = HazelcastConnector.getClient(domain, properties);
            IMap<String, byte[]> map = client.getMap(TOPIC_STORE);
            Map<String, Object> topicMap = new HashMap<>();
            topicMap.put(PARTITIONS, partitions);
            topicMap.put(CREATED, System.currentTimeMillis());
            try {
                map.put(topic, msgPack.pack(topicMap));
            } catch (IOException e) {
                log.error("Unable to create {} - {}", topic, e.getMessage());
                throw e;
            }
        }
    }

    private void deleteTopic(String topic) {
        if (topicExists(topic)) {
            HazelcastInstance client = HazelcastConnector.getClient(domain, properties);
            IMap<String, byte[]> map = client.getMap(TOPIC_STORE);
            map.remove(topic);
        }
    }

    private List<String> listTopics() {
        HazelcastInstance client = HazelcastConnector.getClient(domain, properties);
        IMap<String, byte[]> map = client.getMap(TOPIC_STORE);
        return new ArrayList<>(map.keySet());
    }

}
