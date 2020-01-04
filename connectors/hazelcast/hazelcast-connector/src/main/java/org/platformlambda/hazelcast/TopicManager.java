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

package org.platformlambda.hazelcast;

import com.hazelcast.core.HazelcastInstance;
import com.hazelcast.core.IMap;
import com.hazelcast.core.ITopic;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.hazelcast.reporter.PresenceConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;

public class TopicManager implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(TopicManager.class);
    private static final MsgPack msgPack = new MsgPack();

    public static final String ORIGIN = "origin";
    public static final String CREATE_TOPIC = "create_topic";
    public static final String LIST_TIMESTAMP = "list_timestamp";
    public static final String TOUCH = "touch";

    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String LEAVE = "leave";
    private static final String LIST = "list";
    private static final String GET = "get";
    private static final String GET_ALL = "get_all";
    private static final String EXISTS = "exists";
    private static final String NODES = "nodes";
    private static final String JOIN = "join";
    private boolean isServiceMonitor;

    public TopicManager() {
        AppConfigReader reader = AppConfigReader.getInstance();
        isServiceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (headers.containsKey(TYPE)) {
            if (LIST.equals(headers.get(TYPE))) {
                return listTopics();
            }
            if (LIST_TIMESTAMP.equals(headers.get(TYPE))) {
                return getTopicsWithTimestamp();
            }
            if (EXISTS.equals(headers.get(TYPE)) && headers.containsKey(ORIGIN)) {
                String origin = headers.get(ORIGIN);
                return topicExists(origin);
            }
            if (TOUCH.equals(headers.get(TYPE))) {
                try {
                    touchTopic(Platform.getInstance().getOrigin());
                    return true;
                } catch (Exception e) {
                    if (e.getMessage() != null && e.getMessage().contains("offline")) {
                        log.error("Hazelcast connection problem - {}", e.getMessage());
                        // tell service registry to clear routing table
                        PostOffice.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, LEAVE),
                                new Kv(ORIGIN, Platform.getInstance().getOrigin()));
                    } else {
                        log.error("Unable to touch topic - {}", e.getMessage());
                    }
                }
            }
            if (GET.equals(headers.get(TYPE)) && headers.containsKey(ORIGIN)) {
                String origin = headers.get(ORIGIN);
                return getTopic(origin);
            }
            if (GET_ALL.equals(headers.get(TYPE))) {
                return getTopics();
            }
            // if origin is not specified, it will create the dedicated topic for a new application that is starting up
            if (CREATE_TOPIC.equals(headers.get(TYPE))) {
                createTopic(headers.containsKey(ORIGIN)? headers.get(ORIGIN) : Platform.getInstance().getOrigin());
                return true;
            }
            // delete topic when an application instance expires
            if (LEAVE.equals(headers.get(TYPE)) && headers.containsKey(ORIGIN)) {
                String origin = headers.get(ORIGIN);
                if (topicExists(origin)) {
                    deleteTopic(origin);
                }
                return true;
            }

        }
        return false;
    }

    private boolean topicExists(String topic) {
        String nodes = HazelcastSetup.getNamespace()+NODES;
        HazelcastInstance client = HazelcastSetup.getHazelcastClient();
        IMap<String, byte[]> map = client.getMap(nodes);
        return map.containsKey(topic);
    }

    private void createTopic(String topic) {
        if (!topicExists(topic)) {
            String now = Utility.getInstance().date2str(new Date(), true);
            String nodes = HazelcastSetup.getNamespace()+NODES;
            String realTopic = HazelcastSetup.getNamespace()+topic;
            HazelcastInstance client = HazelcastSetup.getHazelcastClient();
            IMap<String, byte[]> map = client.getMap(nodes);
            Map<String, String> metadata = new HashMap<>();
            metadata.put("node", topic);
            metadata.put("name", Platform.getInstance().getName());
            metadata.put("created", now);
            metadata.put("updated", now);
            try {
                map.put(topic, msgPack.pack(metadata));
                // create topic if not exists
                client.getReliableTopic(realTopic);
                log.info("Topic {} created", topic);
            } catch (IOException e) {
                // this does not happen
                log.error("Unable to create topic {} - {}", topic, e.getMessage());
            }
        }
    }

    private void touchTopic(String topic) throws IOException {
        String now = Utility.getInstance().date2str(new Date(), true);
        Map<String, String> metadata = getTopic(topic);
        if (metadata == null) {
            metadata = new HashMap<>();
            metadata.put("node", topic);
            metadata.put("name", Platform.getInstance().getName());
            metadata.put("recovered", now);
            log.info("Topic {} recovered", topic);
            if (!isServiceMonitor) {
                PresenceConnector connector = PresenceConnector.getInstance();
                if (connector.isConnected() && connector.isReady()) {
                    // tell peers that I have joined
                    try {
                        PostOffice.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, JOIN),
                                new Kv(ORIGIN, Platform.getInstance().getOrigin()));
                    } catch (IOException e) {
                        log.error("Unable to notify peers that I have joined - {}", e.getMessage());
                    }
                }
            }
        }
        metadata.put("updated", now);
        String nodes = HazelcastSetup.getNamespace()+NODES;
        HazelcastInstance client = HazelcastSetup.getHazelcastClient();
        IMap<String, byte[]> map = client.getMap(nodes);
        try {
            map.put(topic, msgPack.pack(metadata));
            log.debug("Topic {} touched", topic);
        } catch (IOException e) {
            // this does not happen
            log.error("Unable to touch topic {} - {}", topic, e.getMessage());
        }
    }

    @SuppressWarnings("unchecked")
    private Map<String, String> getTopic(String topic) throws IOException {
        if (topicExists(topic)) {
            String nodes = HazelcastSetup.getNamespace()+NODES;
            HazelcastInstance client = HazelcastSetup.getHazelcastClient();
            IMap<String, byte[]> map = client.getMap(nodes);
            return (Map<String, String>) msgPack.unpack(map.get(topic));
        } else {
            return null;
        }
    }

    private List<String> listTopics() {
        List<String> result = new ArrayList<>();
        String nodes = HazelcastSetup.getNamespace()+NODES;
        HazelcastInstance client = HazelcastSetup.getHazelcastClient();
        IMap<String, byte[]> map = client.getMap(nodes);
        String namespace = Platform.getInstance().getNamespace();
        for (String t: map.keySet()) {
            // skip topics that are not in this namespace
            if (namespace != null && !t.endsWith(namespace)) {
                continue;
            }
            if (regularTopicFormat(t)) {
                result.add(t);
            }
        }
        return result;
    }

    @SuppressWarnings("unchecked")
    private List<Map<String, String>> getTopics() throws IOException {
        List<Map<String, String>> result = new ArrayList<>();
        String nodes = HazelcastSetup.getNamespace()+NODES;
        HazelcastInstance client = HazelcastSetup.getHazelcastClient();
        IMap<String, byte[]> map = client.getMap(nodes);
        String namespace = Platform.getInstance().getNamespace();
        for (String t: map.keySet()) {
            // skip topics that are not in this namespace
            if (namespace != null && !t.endsWith(namespace)) {
                continue;
            }
            Map<String, String> metadata = (Map<String, String>) msgPack.unpack(map.get(t));
            result.add(metadata);
        }
        return result;
    }

    @SuppressWarnings("unchecked")
    private Map<String, String> getTopicsWithTimestamp() throws IOException {
        String nodes = HazelcastSetup.getNamespace()+NODES;
        HazelcastInstance client = HazelcastSetup.getHazelcastClient();
        IMap<String, byte[]> map = client.getMap(nodes);
        Map<String, String> result = new HashMap<>();
        String namespace = Platform.getInstance().getNamespace();
        for (String t: map.keySet()) {
            // skip topics that are not in this namespace
            if (namespace != null && !t.endsWith(namespace)) {
                continue;
            }
            Map<String, String> metadata = (Map<String, String>) msgPack.unpack(map.get(t));
            if (metadata.containsKey("updated")) {
                result.put(t, metadata.get("updated"));
            }
        }
        return result;
    }

    private void deleteTopic(String topic) {
        if (topicExists(topic)) {
            String nodes = HazelcastSetup.getNamespace()+NODES;
            String realTopic = HazelcastSetup.getNamespace()+topic;
            HazelcastInstance client = HazelcastSetup.getHazelcastClient();
            // remove topic from node list
            IMap<String, byte[]> map = client.getMap(nodes);
            map.remove(topic);
            // destroy topic from cluster
            ITopic<byte[]> iTopic = client.getReliableTopic(realTopic);
            iTopic.destroy();
            log.info("Topic {} deleted", topic);
        }
    }

    /**
     * Validate a topic ID for an application instance
     *
     * @param topic in format of yyyymmdd uuid
     * @return true if valid
     */
    public static boolean regularTopicFormat(String topic) {
        Platform platform = Platform.getInstance();
        if (topic.length() != platform.getOrigin().length()) {
            return false;
        }
        // first 8 digits is a date stamp
        String uuid = topic.substring(8);
        if (!Utility.getInstance().isDigits(topic.substring(0, 8))) {
            return false;
        }
        // drop namespace before validation
        if (platform.getNamespace() != null) {
            int dot = uuid.lastIndexOf('.');
            if (dot > 1) {
                uuid = uuid.substring(0, dot);
            }
        }
        // application instance ID should be hexadecimal
        for (int i=0; i < uuid.length(); i++) {
            if (uuid.charAt(i) >= '0' && uuid.charAt(i) <= '9') continue;
            if (uuid.charAt(i) >= 'a' && uuid.charAt(i) <= 'f') continue;
            return false;
        }
        return true;
    }

}
