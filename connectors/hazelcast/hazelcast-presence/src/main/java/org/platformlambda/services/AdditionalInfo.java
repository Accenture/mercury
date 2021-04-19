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

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PubSub;

import java.io.IOException;
import java.util.*;

public class AdditionalInfo implements LambdaFunction {

    private static final String QUERY = "query";
    private static final String TYPE = "type";
    private static final String ID = "id";
    private static final String NAME = "name";
    private static final String VERSION = "version";

    @Override
    @SuppressWarnings("unchecked")
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
            List<String> vTopics = new ArrayList<>();
            Map<String, String> topics = TopicController.getAssignedTopics();
            for (String t: topics.keySet()) {
                String value = topics.get(t);
                Object c = connections.get(value);
                if (c instanceof Map) {
                    Map<String, Object> cm = (Map<String, Object>) c;
                    if (cm.containsKey(NAME)) {
                        value += ", " + cm.get(NAME);
                    }
                    if (cm.containsKey(VERSION)) {
                        value += " v" + cm.get(VERSION);
                    }
                }
                vTopics.add(t+" -> "+value);
            }
            if (vTopics.size() > 1) {
                Collections.sort(vTopics);
            }
            result.put("virtual.topics", vTopics);
            counts.put("virtual.topics", vTopics.size());
            return result;
        } else {
            throw new IllegalArgumentException("Usage: type=query");
        }
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
        PubSub ps = PubSub.getInstance();
        List<String> topics = ps.list();
        List<String> result = new ArrayList<>();
        for (String t: topics) {
            result.add(t+" ("+ps.partitionCount(t)+")");
        }
        return result;
    }

}
