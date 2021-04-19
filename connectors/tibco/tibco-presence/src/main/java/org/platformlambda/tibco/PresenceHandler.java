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

package org.platformlambda.tibco;

import org.platformlambda.MainApp;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.services.MonitorAlive;
import org.platformlambda.services.MonitorService;
import org.platformlambda.services.TopicController;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;

public class PresenceHandler implements LambdaFunction {
    private static final String MONITOR_PARTITION = TibcoSetup.MONITOR_PARTITION;
    private static final String TYPE = "type";
    private static final String NAME = "name";
    private static final String TOPIC = "topic";
    private static final String ALIVE = "keep-alive";
    private static final String DOWNLOAD = "download";
    private static final String PUT = "put";
    private static final String MULTIPLES = "multiples";
    private static final String DELETE = "del";
    private static final String RELEASE_TOPIC = "release_topic";
    private static final String ORIGIN = "origin";
    private static final String INIT = "init";
    private static final String TIMESTAMP = "timestamp";
    private static boolean ready = false;

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        Utility util = Utility.getInstance();
        PostOffice po = PostOffice.getInstance();
        String myOrigin = Platform.getInstance().getOrigin();
        if (headers.containsKey(ORIGIN) && headers.containsKey(TYPE)) {
            String type = headers.get(TYPE);
            if (!ready) {
                if (INIT.equals(type) && myOrigin.equals(headers.get(ORIGIN))) {
                    MonitorService.setReady();
                    MonitorAlive.setReady();
                    // download monitor list
                    po.send(MainApp.PRESENCE_HOUSEKEEPER + MONITOR_PARTITION, new ArrayList<String>(),
                            new Kv(ORIGIN, myOrigin),
                            new Kv(TYPE, INIT), new Kv(TIMESTAMP, util.getTimestamp()));
                    // download connection list
                    po.send(MainApp.PRESENCE_HANDLER + MONITOR_PARTITION,
                            new Kv(TYPE, DOWNLOAD), new Kv(ORIGIN, myOrigin));
                    ready = true;
                } else {
                    // ignore other event until consumer is initialized
                    return false;
                }
            }
            if (PUT.equals(type) && headers.containsKey(ORIGIN) && body instanceof Map) {
                if (headers.containsKey(MULTIPLES)) {
                    String monitorOrigin = headers.get(ORIGIN);
                    if (!myOrigin.equals(monitorOrigin)) {
                        Map<String, Map<String, Object>> connections = (Map<String, Map<String, Object>>) body;
                        for (String appOrigin: connections.keySet()) {
                            Map<String, Object> metadata = connections.get(appOrigin);
                            MonitorService.updateNodeInfo(appOrigin, metadata);
                            if (metadata.containsKey(TOPIC)) {
                                po.send(MainApp.TOPIC_CONTROLLER, new Kv(TYPE, ALIVE), new Kv(NAME, metadata.get(NAME)),
                                        new Kv(TOPIC, metadata.get(TOPIC)), new Kv(ORIGIN, appOrigin));
                            }
                        }
                    }
                } else {
                    String appOrigin = headers.get(ORIGIN);
                    Map<String, Object> metadata = (Map<String, Object>) body;
                    MonitorService.updateNodeInfo(appOrigin, metadata);
                    if (metadata.containsKey(TOPIC)) {
                        po.send(MainApp.TOPIC_CONTROLLER, new Kv(TYPE, ALIVE), new Kv(NAME, metadata.get(NAME)),
                                new Kv(TOPIC, metadata.get(TOPIC)), new Kv(ORIGIN, appOrigin));
                    }
                }
                return true;
            }
            if (DELETE.equals(type)) {
                MonitorService.deleteNodeInfo(headers.get(ORIGIN));
                po.send(MainApp.TOPIC_CONTROLLER, new Kv(ORIGIN, headers.get(ORIGIN)), new Kv(TYPE, RELEASE_TOPIC));
                return true;
            }
            if (DOWNLOAD.equals(type)) {
                // download request from a new presence monitor
                if (!myOrigin.equals(headers.get(ORIGIN))) {
                    Map<String, Object> connections = MonitorService.getConnections();
                    if (!connections.isEmpty()) {
                        List<String> connectionList = new ArrayList<>(connections.keySet());
                        for (String appOrigin: connectionList) {
                            Object o = connections.get(appOrigin);
                            String topic = TopicController.getTopic(appOrigin);
                            if (topic != null && o instanceof Map) {
                                Map<String, Object> metadata = (Map<String, Object>) o;
                                metadata.put(TOPIC, topic);
                                connections.put(appOrigin, metadata);
                            }
                        }
                        po.send(MainApp.PRESENCE_HANDLER + MONITOR_PARTITION, connections,
                                new Kv(TYPE, PUT), new Kv(ORIGIN, myOrigin), new Kv(MULTIPLES, true));
                    }
                }
                return true;
            }
        }
        return false;
    }

}
