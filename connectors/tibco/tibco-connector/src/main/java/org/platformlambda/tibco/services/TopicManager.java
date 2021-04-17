package org.platformlambda.tibco.services;

import com.tibco.tibjms.admin.TibjmsAdmin;
import com.tibco.tibjms.admin.TibjmsAdminException;
import com.tibco.tibjms.admin.TopicInfo;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.util.Utility;
import org.platformlambda.tibco.TibcoSetup;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

public class TopicManager implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(TopicManager.class);

    private static final String TYPE = "type";
    private static final String PARTITIONS = "partitions";
    private static final String TOPIC = "topic";
    private static final String CREATE = "create";
    private static final String DELETE = "delete";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws TibjmsAdminException {
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
                if (headers.containsKey(PARTITIONS)) {
                    int partitions = Math.max(1, Utility.getInstance().str2int(headers.get(PARTITIONS)));
                    createTopic(headers.get(TOPIC), partitions);
                } else {
                    createTopic(headers.get(TOPIC));
                }
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

    private boolean topicExists(String topic) throws TibjmsAdminException {
        TibjmsAdmin admin = TibcoSetup.getAdminClient();
        try {
            return admin.getTopic(topic) != null;
        } catch (TibjmsAdminException e) {
            return false;
        }
    }

    @SuppressWarnings("unchecked")
    private int topicPartitions(String topic) throws TibjmsAdminException {
        String firstPartition = topic + ".0";
        if (topicExists(firstPartition)) {
            Utility util = Utility.getInstance();
            TibjmsAdmin admin = TibcoSetup.getAdminClient();
            TopicInfo[] topics = admin.getTopics();
            String prefix = topic+".";
            int n = 0;
            for (TopicInfo t: topics) {
                String name = t.getName();
                if (name.startsWith(prefix)) {
                    int partition = util.str2int(name.substring(prefix.length()));
                    if (partition >=0) {
                        n++;
                    }
                }
            }
            return n == 0? -1 : n;
        }
        return -1;
    }

    private void createTopic(String topic) throws TibjmsAdminException {
        if (!topicExists(topic)) {
            TibjmsAdmin admin = TibcoSetup.getAdminClient();
            TopicInfo info = new TopicInfo(topic);
            admin.createTopic(info);
        }
    }

    private void createTopic(String topic, int partitions) throws TibjmsAdminException {
        String firstPartition = topic + ".0";
        if (!topicExists(firstPartition) && partitions > 0) {
            for (int i=0; i < partitions; i++) {
                createTopic(topic + "." + i);
            }
        }
    }

    private void deleteTopic(String topic) throws TibjmsAdminException {
        if (topicExists(topic)) {
            TibjmsAdmin admin = TibcoSetup.getAdminClient();
            admin.destroyTopic(topic);
        }
    }

    private List<String> listTopics() throws TibjmsAdminException {
        List<String> result = new ArrayList<>();
        TibjmsAdmin admin = TibcoSetup.getAdminClient();
        TopicInfo[] topics = admin.getTopics();
        for (TopicInfo t: topics) {
            String name = t.getName();
            if (!name.startsWith(">")) {
                result.add(name);
            }
        }
        return result;
    }

}
