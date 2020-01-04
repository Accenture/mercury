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

import org.platformlambda.MainApp;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.services.MonitorService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Map;
import java.util.UUID;

public class PresenceHandler implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(PresenceHandler.class);

    private static final String TYPE = "type";
    private static final String INIT = "init";
    private static final String DOWNLOAD = "download";
    private static final String RESET = "reset";
    private static final String PUT = "put";
    private static final String DELETE = "del";
    private static final String ORIGIN = "origin";
    private final static String initToken = UUID.randomUUID().toString();

    private static boolean ready = false;
    private int ignored = 0;

    public static boolean isReady() {
        return ready;
    }

    public static String getInitToken() {
        return initToken;
    }

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        String myOrigin = Platform.getInstance().getOrigin();
        if (headers.containsKey(INIT)){
            /*
             * Ignore all incoming event until the correct initialization message is created.
             */
            if (ready) {
                if (getInitToken().equals(headers.get(INIT))) {
                    log.info("System is healthy");
                }
            } else {
                if (getInitToken().equals(headers.get(INIT))) {
                    ready = true;
                    if (ignored > 0) {
                        log.warn("Skipping {} outdated events", ignored);
                    }
                    log.info("Ready");
                    // download current connections from peers
                    EventEnvelope event = new EventEnvelope();
                    event.setTo(org.platformlambda.MainApp.PRESENCE_HANDLER);
                    event.setHeader(TYPE, DOWNLOAD);
                    event.setHeader(ORIGIN, myOrigin);
                    PostOffice.getInstance().send(MainApp.PRESENCE_MONITOR, event.toBytes());
                    return true;
                }
            }
            return false;
        }
        if (ready) {
            /*
             * Handle incoming events
             */
            if (headers.containsKey(ORIGIN)) {
                if (headers.containsKey(TYPE)) {
                    if (RESET.equals(headers.get(TYPE))) {
                        // reset all connections because Hazelcast is offline
                        log.warn("Reset application connection because Hazelcast is offline");
                        MonitorService.closeAllConnections();
                    }
                    if (PUT.equals(headers.get(TYPE)) && body instanceof Map) {
                        MonitorService.updateNodeInfo(headers.get(ORIGIN), (Map<String, Object>) body);
                    }
                    if (DELETE.equals(headers.get(TYPE))) {
                        MonitorService.deleteNodeInfo(headers.get(ORIGIN));
                    }
                    if (DOWNLOAD.equals(headers.get(TYPE))) {
                        if (!myOrigin.equals(headers.get(ORIGIN))) {
                            // download request from a new presence monitor
                            Map<String, Object> connections = MonitorService.getConnections();
                            for (String node: connections.keySet()) {
                                Object info = connections.get(node);
                                EventEnvelope event = new EventEnvelope();
                                event.setTo(org.platformlambda.MainApp.PRESENCE_HANDLER);
                                event.setHeader(ORIGIN, node);
                                event.setHeader(TYPE, PUT);
                                event.setBody(info);
                                PostOffice.getInstance().send(MainApp.PRESENCE_MONITOR, event.toBytes());
                            }
                        }
                    }
                }
            }
            return true;
        } else {
            ignored++;
            return false;
        }
    }

}
