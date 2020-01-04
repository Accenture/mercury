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

package org.platformlambda.kafka;

import org.platformlambda.MainApp;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
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

    public static final String INIT_TOKEN = UUID.randomUUID().toString();
    private static final String TYPE = "type";
    private static final String INIT = "init";
    private static final String DOWNLOAD = "download";
    private static final String TO = "to";
    private static final String PUT = "put";
    private static final String DELETE = "del";
    private static final String ORIGIN = "origin";

    private static boolean ready = false;
    private int ignored = 0;

    public static boolean isReady() {
        return PresenceHandler.ready;
    }

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        String myOrigin = Platform.getInstance().getOrigin();
        if (headers.containsKey(INIT)){
            /*
             * Ignore all incoming event until the correct initialization message is created.
             */
            if (PresenceHandler.ready) {
                if (INIT_TOKEN.equals(headers.get(INIT))) {
                    log.info("Running");
                }
            } else if (INIT_TOKEN.equals(headers.get(INIT))) {
                PresenceHandler.ready = true;
                if (ignored > 0) {
                    log.warn("Skipping {} outdated events", ignored);
                }
                log.info("Ready");
                // download current connections from peers
                EventEnvelope event = new EventEnvelope();
                event.setTo(MainApp.PRESENCE_HANDLER);
                event.setHeader(TYPE, DOWNLOAD);
                event.setHeader(ORIGIN, myOrigin);
                PostOffice.getInstance().send(PostOffice.CLOUD_CONNECTOR, event.toBytes(), new Kv(TO, "*"));
                return true;
            }
            return false;
        }
        if (PresenceHandler.ready) {
            /*
             * Handle incoming events
             */
            if (headers.containsKey(ORIGIN)) {
                if (headers.containsKey(TYPE)) {
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
                                event.setTo(MainApp.PRESENCE_HANDLER);
                                event.setHeader(ORIGIN, node);
                                event.setHeader(TYPE, PUT);
                                event.setBody(info);
                                PostOffice.getInstance().send(PostOffice.CLOUD_CONNECTOR, event.toBytes(), new Kv(TO, "*"));
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
