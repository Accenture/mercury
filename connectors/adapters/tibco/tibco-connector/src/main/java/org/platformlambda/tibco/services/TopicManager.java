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
import org.platformlambda.cloud.ConnectorConfig;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.tibco.TibcoConnector;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;

public class TopicManager implements LambdaFunction {
    private static final String TYPE = "type";
    private static final String PARTITIONS = "partitions";
    private static final String TOPIC = "topic";
    private static final String CREATE = "create";
    private static final String DELETE = "delete";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private final boolean topicSubstitution;
    private final Map<String, String> preAllocatedTopics;

    public TopicManager() throws IOException {
        topicSubstitution = ConnectorConfig.topicSubstitutionEnabled();
        preAllocatedTopics = ConnectorConfig.getTopicSubstitution();
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
        if (topicSubstitution) {
            return preAllocatedTopics.get(topic) != null;
        }
        TibjmsAdmin admin = TibcoConnector.getAdminClient();
        try {
            return admin.getTopic(topic) != null;
        } catch (TibjmsAdminException e) {
            return false;
        }
    }

    @SuppressWarnings("unchecked")
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
            TibjmsAdmin admin = TibcoConnector.getAdminClient();
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
            TibjmsAdmin admin = TibcoConnector.getAdminClient();
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
            TibjmsAdmin admin = TibcoConnector.getAdminClient();
            admin.destroyTopic(topic);
        }
    }

    private List<String> listTopics() throws TibjmsAdminException {
        if (topicSubstitution) {
            return new ArrayList<>(preAllocatedTopics.keySet());
        }
        List<String> result = new ArrayList<>();
        TibjmsAdmin admin = TibcoConnector.getAdminClient();
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
