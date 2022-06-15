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

package org.platformlambda.tibco.services;

import com.tibco.tibjms.admin.TibjmsAdmin;
import com.tibco.tibjms.admin.TibjmsAdminException;
import com.tibco.tibjms.admin.TopicInfo;
import com.tibco.tibjms.admin.QueueInfo;
import org.platformlambda.cloud.ConnectorConfig;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.util.Utility;
import org.platformlambda.tibco.TibcoConnector;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Properties;

public class TopicManager implements LambdaFunction {
    private static final String TYPE = "type";
    private static final String PARTITIONS = "partitions";
    private static final String TOPIC = "topic";
    private static final String QUEUE = "queue";
    private static final String CREATE = "create";
    private static final String DELETE = "delete";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private final String domain;
    private final Properties properties;
    private final boolean topicSubstitution;
    private final Map<String, String> preAllocatedTopics;

    public TopicManager(String domain, Properties properties) throws IOException {
        this.domain = domain;
        this.properties = properties;
        this.topicSubstitution = ConnectorConfig.topicSubstitutionEnabled();
        this.preAllocatedTopics = ConnectorConfig.getTopicSubstitution();
    }

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
            if (CREATE.equals(headers.get(TYPE))) {
                int partitions = Utility.getInstance().str2int(headers.getOrDefault(PARTITIONS, "-1"));
                if (headers.containsKey(TOPIC)) {
                    if (partitions > -1) {
                        createTopic(headers.get(TOPIC), partitions);
                    } else {
                        createTopic(headers.get(TOPIC));
                    }
                } else if (headers.containsKey(QUEUE)) {
                    createQueue(headers.get(QUEUE));
                }
                return true;
            }
            if (DELETE.equals(headers.get(TYPE))) {
                if (headers.containsKey(TOPIC)) {
                    String topic = headers.get(TOPIC);
                    if (topicExists(topic)) {
                        deleteTopic(topic);
                    }
                } else if (headers.containsKey(QUEUE)) {
                    String queue = headers.get(QUEUE);
                    if (topicExists(queue)) {
                        deleteQueue(queue);
                    }
                }
                return true;
            }
        }
        return false;
    }

    private boolean topicExists(String topic) throws TibjmsAdminException {
        if (topicSubstitution) {
            return preAllocatedTopics.get(topic) != null;
        }
        TibjmsAdmin admin = TibcoConnector.getAdminClient(domain, properties);
        try {
            return admin.getTopic(topic) != null || admin.getQueue(topic) != null;
        } catch (TibjmsAdminException e) {
            return false;
        }
    }

    private int topicPartitions(String topic) throws TibjmsAdminException {
        if (topicSubstitution) {
            int n = 0;
            while (preAllocatedTopics.containsKey(topic+"."+n)) {
                n++;
            }
            return n;
        }
        String firstPartition = topic + ".0";
        if (topicExists(firstPartition)) {
            Utility util = Utility.getInstance();
            TibjmsAdmin admin = TibcoConnector.getAdminClient(domain, properties);
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
        if (topicSubstitution) {
            if (preAllocatedTopics.get(topic) == null) {
                throw new IllegalArgumentException("Missing topic substitution for "+topic);
            }
            return;
        }
        if (!topicExists(topic)) {
            TibjmsAdmin admin = TibcoConnector.getAdminClient(domain, properties);
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
        if (topicSubstitution) {
            if (preAllocatedTopics.get(topic) == null) {
                throw new IllegalArgumentException("Missing topic substitution for "+topic);
            }
            return;
        }
        if (topicExists(topic)) {
            TibjmsAdmin admin = TibcoConnector.getAdminClient(domain, properties);
            admin.destroyTopic(topic);
        }
    }

    private void createQueue(String queue) throws TibjmsAdminException {
        if (!topicExists(queue)) {
            TibjmsAdmin admin = TibcoConnector.getAdminClient(domain, properties);
            QueueInfo info = new QueueInfo(queue);
            admin.createQueue(info);
        }
    }

    private void deleteQueue(String queue) throws TibjmsAdminException {
        if (topicExists(queue)) {
            TibjmsAdmin admin = TibcoConnector.getAdminClient(domain, properties);
            admin.destroyQueue(queue);
        }
    }

    private List<String> listTopics() throws TibjmsAdminException {
        if (topicSubstitution) {
            return new ArrayList<>(preAllocatedTopics.keySet());
        }
        List<String> result = new ArrayList<>();
        TibjmsAdmin admin = TibcoConnector.getAdminClient(domain, properties);
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
