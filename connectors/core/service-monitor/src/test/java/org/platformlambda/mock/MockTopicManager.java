package org.platformlambda.mock;

import org.platformlambda.cloud.ConnectorConfig;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.List;
import java.util.Map;

public class MockTopicManager implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(MockTopicManager.class);

    private static final String TYPE = "type";
    private static final String PARTITIONS = "partitions";
    private static final String TOPIC = "topic";
    private static final String CREATE = "create";
    private static final String DELETE = "delete";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private static final String STOP = "stop";
    private final PubSub ps;

    public MockTopicManager() {
        this.ps = PubSub.getInstance();
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
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
                int partitions = headers.containsKey(PARTITIONS)?
                        Math.max(1, Utility.getInstance().str2int(headers.get(PARTITIONS))) : 1;
                createTopic(headers.get(TOPIC), partitions);
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

    private boolean topicExists(String topic) throws IOException {
        return ps.exists(topic);
    }

    private int topicPartitions(String topic) throws IOException {
        return ps.partitionCount(topic);
    }

    private void createTopic(String topic, int partitions) throws IOException {
        ps.createTopic(topic, partitions);
    }

    private void deleteTopic(String topic) throws IOException {
        ps.deleteTopic(topic);
    }

    private List<String> listTopics() throws IOException {
        return ps.list();
    }
}
