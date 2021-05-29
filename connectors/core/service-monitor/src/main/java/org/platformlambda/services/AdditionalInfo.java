/*

    Copyright 2018-2021 Accenture Technology

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

package org.platformlambda.services;

import org.platformlambda.cloud.ConnectorConfig;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.util.*;

public class AdditionalInfo implements LambdaFunction {

    private static final String QUERY = "query";
    private static final String TYPE = "type";
    private static final String ID = "id";
    private static final String NAME = "name";
    private static final String VERSION = "version";
    private final String appPrefix, monitorPrefix;
    private final boolean topicSubstitution;
    private final Map<String, String> preAllocatedTopics;

    public AdditionalInfo() throws IOException {
        AppConfigReader config = AppConfigReader.getInstance();
        appPrefix = config.getProperty("app.topic.prefix", "multiplex") + ".";
        monitorPrefix = config.getProperty("monitor.topic", "service.monitor") + ".";
        topicSubstitution = ConnectorConfig.topicSubstitutionEnabled();
        preAllocatedTopics = ConnectorConfig.getTopicSubstitution();
    }

    @SuppressWarnings("unchecked")
    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        if (QUERY.equals(headers.get(TYPE))) {
            Map<String, Object> result = new HashMap<>();
            // connection list
            Map<String, Object> connections = MonitorService.getConnections();
            List<String> keys = new ArrayList<>(connections.keySet());
            for (String k: keys) {
                connections.put(k, filterInfo((Map<String, Object>) connections.get(k)));
            }
            result.put("connections", connections);
            result.put("monitors", HouseKeeper.getMonitors());
            // topic list
            List<String> pubSub = getTopics();
            result.put("topics", pubSub);
            // totals
            Map<String, Object> counts = new HashMap<>();
            counts.put("connections", connections.size());
            counts.put("topics", pubSub.size());
            result.put("total", counts);
            List<String> vTopics = getVirtualTopics(connections);
            result.put("virtual.topics", vTopics);
            counts.put("virtual.topics", vTopics.size());
            return result;
        } else {
            throw new IllegalArgumentException("Usage: type=query");
        }
    }

    private String getTopicReplacement(String virtualTopic) {
        String replacement = preAllocatedTopics.get(virtualTopic);
        if (replacement != null) {
            return replacement;
        }
        Utility util = Utility.getInstance();
        if (virtualTopic.contains("-")) {
            int hyphen = virtualTopic.lastIndexOf('-');
            if (hyphen > 0) {
                String topic = virtualTopic.substring(0, hyphen);
                if (preAllocatedTopics.containsKey(topic)) {
                    return preAllocatedTopics.get(topic);
                } else {
                    int partition = util.str2int(virtualTopic.substring(hyphen + 1));
                    return preAllocatedTopics.get(topic + "." + partition);
                }
            }
        }
        return null;
    }

    @SuppressWarnings("unchecked")
    private List<String> getVirtualTopics(Map<String, Object> connections) {
        Map<String, String> topics = TopicController.getAssignedTopics();
        Map<String, List<String>> members = new HashMap<>();
        for (String t: topics.keySet()) {
            String member = topics.get(t);
            List<String> memberTopics = members.getOrDefault(member, new ArrayList<>());
            String topicName = t;
            if (topicSubstitution) {
                String replacement = getTopicReplacement(t);
                if (replacement != null) {
                    topicName += " ("+replacement+")";
                }
            }
            memberTopics.add(topicName);
            members.put(member, memberTopics);
        }
        List<String> vTopics = new ArrayList<>();
        for (String m: members.keySet()) {
            String topicList = list2str(members.get(m));
            String signature = m;
            Object c = connections.get(m);
            if (c instanceof Map) {
                Map<String, Object> cm = (Map<String, Object>) c;
                if (cm.containsKey(NAME)) {
                    signature += ", " + cm.get(NAME);
                }
                if (cm.containsKey(VERSION)) {
                    signature += " v" + cm.get(VERSION);
                }
            }
            vTopics.add(topicList+" -> "+signature);
        }
        if (vTopics.size() > 1) {
            Collections.sort(vTopics);
        }
        return vTopics;
    }

    private String list2str(List<String> list) {
        if (list.isEmpty()) {
            return "?";
        }
        if (list.size() == 1) {
            return list.get(0);
        }
        Collections.sort(list);
        StringBuilder sb = new StringBuilder();
        for (String item: list) {
            sb.append(item);
            sb.append(", ");
        }
        return sb.substring(0, sb.length()-2);
    }

    private Map<String, Object> filterInfo(Map<String, Object> info) {
        Map<String, Object> result = new HashMap<>();
        for (String key : info.keySet()) {
            if (!key.equals(ID)) {
                result.put(key, info.get(key));
            }
        }
        return result;
    }

    private List<String> getTopics() throws IOException {
        Utility util = Utility.getInstance();
        PubSub ps = PubSub.getInstance();
        List<String> topics = ps.list();
        if (topics.size() > 1) {
            Collections.sort(topics);
        }
        List<String> regularTopics = new ArrayList<>();
        Map<String, Integer> compositeTopics = new HashMap<>();
        List<String> result = new ArrayList<>();
        for (String topic: topics) {
            if (ps.isNativePubSub()) {
                result.add(topic + " (" + ps.partitionCount(topic) + ")");
            } else {
                // simulated topic partitioning
                if (topic.startsWith(appPrefix) || topic.startsWith(monitorPrefix)) {
                    List<String> parts = util.split(topic, ".");
                    if (parts.size() == 3) {
                        int dot = topic.lastIndexOf('.');
                        String topicName = topic.substring(0, dot);
                        String partition = topic.substring(dot+1);
                        if (util.isDigits(partition)) {
                            Integer n = compositeTopics.getOrDefault(topicName, 0) + 1;
                            compositeTopics.put(topicName, n);

                        } else {
                            regularTopics.add(topic);
                        }
                    } else {
                        regularTopics.add(topic);
                    }

                } else {
                    regularTopics.add(topic);
                }
            }
        }
        if (compositeTopics.isEmpty()) {
            return result;
        } else {
            List<String> consolidated = new ArrayList<>();
            List<String> topicList = new ArrayList<>(compositeTopics.keySet());
            if (topicList.size() > 1) {
                Collections.sort(topicList);
            }
            for (String t: topicList) {
                consolidated.add(t+" ("+compositeTopics.get(t)+")");
            }
            consolidated.addAll(regularTopics);
            return consolidated;
        }
    }

}
