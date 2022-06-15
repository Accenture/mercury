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

package org.platformlambda.activemq.services;

import org.apache.activemq.artemis.api.core.client.*;
import org.apache.activemq.artemis.api.core.management.ManagementHelper;
import org.apache.activemq.artemis.api.core.management.ResourceNames;
import org.platformlambda.activemq.ArtemisConnector;
import org.platformlambda.cloud.ConnectorConfig;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Properties;

public class TopicManager implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(TopicManager.class);

    private static final String TYPE = "type";
    private static final String PARTITIONS = "partitions";
    private static final String TOPIC = "topic";
    private static final String QUEUE = "queue";
    private static final String CREATE = "create";
    private static final String DELETE = "delete";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private static final String USER_ID = "admin.id";
    private static final String USER_PWD = "admin.password";
    private static final String ACTIVEMQ_MANAGEMENT = "activemq.management";
    private static final String[] ACTIVEMQ_RESERVED = {"DLQ", "ExpiryQueue"};
    private static final String ACTIVEMQ_PREFIX = "activemq.";
    private static final String TOPIC_SIGNATURE = "routingTypes={MULTICAST}";
    private static final String MULTICAST = "MULTICAST";
    private static final String ADDRESS = "Address";
    private static ClientSession session;
    private final boolean topicSubstitution;
    private final Map<String, String> preAllocatedTopics;

    public TopicManager(Properties properties) throws Exception {
        this.topicSubstitution = ConnectorConfig.topicSubstitutionEnabled();
        this.preAllocatedTopics = ConnectorConfig.getTopicSubstitution();
        if (!this.topicSubstitution && session == null) {
            String cluster = properties.getProperty(ArtemisConnector.BROKER_URL, "tcp://127.0.0.1:61616");
            String userId = properties.getProperty(USER_ID, "");
            String password = properties.getProperty(USER_PWD, "");
            ServerLocator locator = ActiveMQClient.createServerLocator(cluster);
            ClientSessionFactory factory2 = locator.createSessionFactory();
            session = factory2.createSession(userId, password,
                    false, true, true, false, 1);
            session.start();
        }
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
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

    private boolean topicExists(String topic) throws Exception {
        if (topicSubstitution) {
            return preAllocatedTopics.get(topic) != null;
        }
        try (ClientRequestor requestor = new ClientRequestor(session, ACTIVEMQ_MANAGEMENT)) {
            ClientMessage m = session.createMessage(false);
            ManagementHelper.putOperationInvocation(m, ResourceNames.BROKER,
                    "getAddressInfo", topic);
            ClientMessage reply = requestor.request(m);
            Object o = ManagementHelper.getResult(reply);
            return o instanceof String && ((String) o).startsWith(ADDRESS);
        }
    }
    
    private int topicPartitions(String topic) throws Exception {
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
            List<String> segments = util.split(topic, ".");
            try (ClientRequestor requestor = new ClientRequestor(session, ACTIVEMQ_MANAGEMENT)) {
                ClientMessage m = session.createMessage(false);
                ManagementHelper.putOperationInvocation(m, ResourceNames.BROKER,
                        "listAddresses", "|");
                ClientMessage reply = requestor.request(m);
                Object o = ManagementHelper.getResult(reply);
                if (o instanceof String) {
                    int n = 0;
                    List<String> topicList = util.split((String) o, "|");
                    for (String t : topicList) {
                        if (t.startsWith(topic + ".")) {
                            List<String> parts = util.split(t, ".");
                            if (parts.size() == segments.size() + 1) {
                                if (util.isDigits(parts.get(parts.size() - 1))) {
                                    n++;
                                }
                            }
                        }
                    }
                    return n == 0 ? -1 : n;
                }
            }
        }
        return -1;
    }

    private void createTopic(String topic) throws Exception {
        if (topicSubstitution) {
            if (preAllocatedTopics.get(topic) == null) {
                throw new IllegalArgumentException("Missing topic substitution for "+topic);
            }
            return;
        }
        createAddress(topic, true);
    }

    private void createTopic(String topic, int partitions) throws Exception {
        String firstPartition = topic + ".0";
        if (!topicExists(firstPartition) && partitions > 0) {
            for (int i=0; i < partitions; i++) {
                createTopic(topic + "." + i);
            }
        }
    }

    private void createQueue(String queue) throws Exception {
        createAddress(queue, false);
    }

    private void createAddress(String address, boolean isTopic) throws Exception {
        if (!topicExists(address)) {
            try (ClientRequestor requestor = new ClientRequestor(session, ACTIVEMQ_MANAGEMENT)) {
                ClientMessage m = session.createMessage(false);
                if (isTopic) {
                    ManagementHelper.putOperationInvocation(m, ResourceNames.BROKER,
                            "createAddress", address, MULTICAST);
                } else {
                    ManagementHelper.putOperationInvocation(m, ResourceNames.BROKER,
                            "createAddress", address);
                }
                ClientMessage reply = requestor.request(m);
                Object o = ManagementHelper.getResult(reply);
                if (o instanceof String) {
                    String result = (String) o;
                    if (result.startsWith(ADDRESS)) {
                        log.info("Created {} {}", isTopic? TOPIC : QUEUE, address);
                    } else {
                        log.warn("ActiveMQ exception when creating {} {}", address, o);
                    }
                }
            }
        }
    }

    private void deleteTopic(String topic) throws Exception {
        if (topicSubstitution) {
            if (preAllocatedTopics.get(topic) == null) {
                throw new IllegalArgumentException("Missing topic substitution for "+topic);
            }
            return;
        }
        deleteAddress(topic);
    }

    private void deleteQueue(String queue) throws Exception {
        deleteAddress(queue);
    }

    private void deleteAddress(String address) throws Exception {
        if (topicExists(address)) {
            try (ClientRequestor requestor = new ClientRequestor(session, ACTIVEMQ_MANAGEMENT)) {
                ClientMessage m = session.createMessage(false);
                ManagementHelper.putOperationInvocation(m, ResourceNames.BROKER,
                        "deleteAddress", address);
                ClientMessage reply = requestor.request(m);
                Object o = ManagementHelper.getResult(reply);
                if (o != null) {
                    log.warn("ActiveMQ exception when deleting {} {}", address, o);
                }
            }
        }
    }

    private List<String> listTopics() throws Exception {
        if (topicSubstitution) {
            return new ArrayList<>(preAllocatedTopics.keySet());
        }
        List<String> result = new ArrayList<>();
        Utility util = Utility.getInstance();
        try (ClientRequestor requestor = new ClientRequestor(session, ACTIVEMQ_MANAGEMENT)) {
            ClientMessage m = session.createMessage(false);
            ManagementHelper.putOperationInvocation(m, ResourceNames.BROKER,
                    "listAddresses", "|");
            ClientMessage reply = requestor.request(m);
            Object o = ManagementHelper.getResult(reply);
            if (o instanceof String) {
                List<String> topicList = util.split((String) o, "|");
                for (String t : topicList) {
                    if (!isReserved(t) && isMulticast(t)) {
                        result.add(t);
                    }
                }
            }
        }
        return result;
    }

    private boolean isMulticast(String topic) throws Exception {
        if (topicSubstitution) {
            return true;
        }
        try (ClientRequestor requestor = new ClientRequestor(session, ACTIVEMQ_MANAGEMENT)) {
            ClientMessage m = session.createMessage(false);
            ManagementHelper.putOperationInvocation(m, ResourceNames.BROKER,
                    "getAddressInfo", topic);
            ClientMessage reply = requestor.request(m);
            Object o = ManagementHelper.getResult(reply);
            if (o instanceof String) {
                String result = (String) o;
                return result.contains(TOPIC_SIGNATURE);
            }
            return false;
        }
    }

    private boolean isReserved(String topic) {
        if (topic.startsWith(ACTIVEMQ_PREFIX)) {
            return true;
        }
        for (String reserved: ACTIVEMQ_RESERVED) {
            if (reserved.equals(topic)) {
                return true;
            }
        }
        return false;
    }

}
