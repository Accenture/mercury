/*

    Copyright 2018-2019 Accenture Technology

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

package org.platformlambda.service;

import org.platformlambda.MainApp;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;

import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeoutException;

public class InfoService implements LambdaFunction {

    private static final String MANAGER = MainApp.MANAGER;
    private static final String QUERY = "query";
    private static final String TYPE = "type";
    private static final String LIST = "list";
    private static final String ID = "id";

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
        PostOffice po = PostOffice.getInstance();
        EventEnvelope res1 = po.request(MANAGER, 30000, new Kv(TYPE, LIST));
        return res1.getBody() instanceof List? (List<String>) res1.getBody() : new ArrayList<>();
    }

}
