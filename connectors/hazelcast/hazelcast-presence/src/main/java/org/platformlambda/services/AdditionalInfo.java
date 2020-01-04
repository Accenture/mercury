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

package org.platformlambda.services;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.hazelcast.HazelcastSetup;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.TimeoutException;

public class AdditionalInfo implements LambdaFunction {

    private static final String MANAGER = HazelcastSetup.MANAGER;
    private static final String QUERY = "query";
    private static final String TYPE = "type";
    private static final String GET_ALL = "get_all";
    private static final String ID = "id";
    private static final String NODE = "node";
    private static final String NAME = "name";
    private static final String UPDATED = "updated";

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws TimeoutException, IOException, AppException {
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
            List<String> topics = getTopics();
            result.put("topics", topics);
            // totals
            Map<String, Object> counts = new HashMap<>();
            counts.put("connections", connections.size());
            counts.put("topics", topics.size());
            result.put("total", counts);
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

    @SuppressWarnings("unchecked")
    private List<String> getTopics() throws TimeoutException, IOException, AppException {
        // get topic list from hazelcast
        PostOffice po = PostOffice.getInstance();
        EventEnvelope res = po.request(MANAGER, 30000, new Kv(TYPE, GET_ALL));
        List<String> list = new ArrayList<>();
        List<Map<String, String>> dataset = res.getBody() instanceof List?
                                            (List<Map<String, String>>) res.getBody() : new ArrayList<>();

        for (Map<String, String> map: dataset) {
            if (map.containsKey(NAME) && map.containsKey(NODE) && map.containsKey(UPDATED)) {
                String name = map.get(NAME);
                String node = map.get(NODE);
                String time = map.get(UPDATED);
                list.add(name+"|"+time+"|"+node);
            }
        }
        return sortedTopicList(list);
    }

    private List<String> sortedTopicList(List<String> list) {
        Utility util = Utility.getInstance();
        if (list.size() > 1) {
            Collections.sort(list);
        }
        List<String> result = new ArrayList<>();
        for (String item: list) {
            List<String> parts = util.split(item, "|");
            if (parts.size() == 3) {
                result.add(parts.get(2)+", "+parts.get(1)+", "+parts.get(0));
            } else {
                result.add(item);
            }
        }
        return result;
    }

}
