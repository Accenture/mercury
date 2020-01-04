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

import org.platformlambda.MainApp;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.models.AppInfo;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.TimeoutException;

public class AdditionalInfo implements LambdaFunction {

    private static final String MANAGER = MainApp.MANAGER;
    private static final String QUERY = "query";
    private static final String TYPE = "type";
    private static final String LIST = "list";
    private static final String ID = "id";
    private static final String PUB_SUB = "pub_sub";
    private static final long EXPIRY = 60 * 1000;

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
            List<String> topics = getTopics(false);
            Utility util = Utility.getInstance();
            long now = System.currentTimeMillis();
            List<String> liveTopicList = new ArrayList<>();
            List<String> expiredTopicList = new ArrayList<>();
            for (String t: topics) {
                AppInfo info = HouseKeeper.getAppInfo(t);
                if (info != null) {
                    if (now - info.lastSeen > EXPIRY) {
                        expiredTopicList.add(info.appName+"|"+
                                util.date2str(new Date(info.lastSeen), true)+"|"+t+"|"+info.source);
                    } else {
                        liveTopicList.add(info.appName+"|"+
                                util.date2str(new Date(info.lastSeen), true)+"|"+t+"|"+info.source);
                    }
                }
            }
            List<String> liveTopics = sortedTopicList(liveTopicList);
            List<String> expiredTopics = sortedTopicList(expiredTopicList);
            result.put("live_topics", liveTopics);
            result.put("expired_topics", expiredTopics);
            List<String> pubSub = getTopics(true);
            result.put("pub_sub", pubSub);
            // totals
            Map<String, Object> counts = new HashMap<>();
            counts.put("connections", connections.size());
            counts.put("pub_sub", pubSub.size());
            counts.put("live_topics", liveTopics.size());
            counts.put("expired_topics", expiredTopics.size());
            result.put("total", counts);
            return result;
        } else {
            throw new IllegalArgumentException("Usage: type=query");
        }
    }

    private List<String> sortedTopicList(List<String> list) {
        Utility util = Utility.getInstance();
        if (list.size() > 1) {
            Collections.sort(list);
        }
        List<String> result = new ArrayList<>();
        for (String item: list) {
            List<String> parts = util.split(item, "|");
            if (parts.size() == 4) {
                result.add(parts.get(2)+", "+parts.get(1)+", "+parts.get(0)+" via "+parts.get(3));
            } else {
                result.add(item);
            }
        }
        return result;
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
    private List<String> getTopics(boolean pubSub) throws TimeoutException, IOException, AppException {
        PostOffice po = PostOffice.getInstance();
        EventEnvelope res = pubSub? po.request(MANAGER, 15000, new Kv(TYPE, LIST), new Kv(PUB_SUB, true)) :
                                    po.request(MANAGER, 15000, new Kv(TYPE, LIST));
        return res.getBody() instanceof List? (List<String>) res.getBody() : new ArrayList<>();
    }

}
