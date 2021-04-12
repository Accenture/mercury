package org.platformlambda.hazelcast.services;

import com.hazelcast.core.HazelcastInstance;
import com.hazelcast.map.IMap;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.util.Utility;
import org.platformlambda.hazelcast.HazelcastSetup;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

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

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
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
            // if origin is not specified, it will create the dedicated topic for a new application that is starting up
            if (CREATE.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                createTopic(headers.get(TOPIC),
                        headers.containsKey(PARTITIONS)? Utility.getInstance().str2int(headers.get(PARTITIONS)) : 1);
                return true;
            }
            // delete topic when an application instance expires
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
        HazelcastInstance client = HazelcastSetup.getClient();
        IMap<String, byte[]> map = client.getMap(TOPIC_STORE);
        return map.containsKey(topic);
    }

    @SuppressWarnings("unchecked")
    private int topicPartitions(String topic) {
        if (topicExists(topic)) {
            try {
                Utility util = Utility.getInstance();
                HazelcastInstance client = HazelcastSetup.getClient();
                IMap<String, byte[]> map = client.getMap(TOPIC_STORE);
                Map<String, Object> topicMap = (Map<String, Object>) msgPack.unpack(map.get(topic));
                return util.str2int(topicMap.getOrDefault(PARTITIONS, 1).toString());
            } catch (IOException e) {
                log.error("Unable to retrieve {} - {}", topic, e.getMessage());
            }
        }
        return -1;
    }

    private void createTopic(String topic, int partitions) {
        if (!topicExists(topic)) {
            HazelcastInstance client = HazelcastSetup.getClient();
            IMap<String, byte[]> map = client.getMap(TOPIC_STORE);
            Map<String, Object> topicMap = new HashMap<>();
            topicMap.put(PARTITIONS, partitions);
            topicMap.put(CREATED, System.currentTimeMillis());
            try {
                map.put(topic, msgPack.pack(topicMap));
            } catch (IOException e) {
                log.error("Unable to create {} - {}", topic, e.getMessage());
            }
        }
    }

    private void deleteTopic(String topic) {
        if (topicExists(topic)) {
            HazelcastInstance client = HazelcastSetup.getClient();
            IMap<String, byte[]> map = client.getMap(TOPIC_STORE);
            map.remove(topic);
        }
    }

    private List<String> listTopics() {
        HazelcastInstance client = HazelcastSetup.getClient();
        IMap<String, byte[]> map = client.getMap(TOPIC_STORE);
        return new ArrayList<>(map.keySet());
    }

}
