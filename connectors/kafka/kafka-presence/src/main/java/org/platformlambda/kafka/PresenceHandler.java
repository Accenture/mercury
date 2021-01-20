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

package org.platformlambda.kafka;

import org.platformlambda.MainApp;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.services.MonitorService;

import java.io.IOException;
import java.util.Map;

public class PresenceHandler implements LambdaFunction {
    private static final String TYPE = "type";
    private static final String DOWNLOAD = "download";
    private static final String TO = "to";
    private static final String PUT = "put";
    private static final String DELETE = "del";
    private static final String ORIGIN = "origin";

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        String myOrigin = Platform.getInstance().getOrigin();
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
    }

}
