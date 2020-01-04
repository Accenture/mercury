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

package org.platformlambda.automation.config;

import org.platformlambda.automation.models.WsInfo;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class WsEntry {
    private static final Logger log = LoggerFactory.getLogger(WsEntry.class);

    private static final String WEB_SOCKET = "websocket";
    private static final String SERVICE = "service";
    private static final String APPLICATION = "application";
    private static final String AUTH = "authentication";

    private static final Map<String, WsInfo> routingTable = new HashMap<>();

    private static final WsEntry instance = new WsEntry();

    private WsEntry() {}

    public static WsEntry getInstance() {
        return instance;
    }

    public boolean isEmpty() {
        return routingTable.isEmpty();
    }

    public WsInfo getRoute(String app) {
        return routingTable.get(app);
    }

    @SuppressWarnings("unchecked")
    public void load(Map<String, Object> config) {
        if (config.containsKey(WEB_SOCKET)) {
            Utility util = Utility.getInstance();
            Object o = config.get(WEB_SOCKET);
            if (o instanceof List) {
                List<Object> entries = (List<Object>) o;
                for (Object e: entries) {
                    if (e instanceof Map) {
                        Map<String, Object> entry = (Map<String, Object>) e;
                        if (entry.containsKey(SERVICE) && entry.containsKey(APPLICATION) && entry.containsKey(AUTH)) {
                            String app = entry.get(APPLICATION).toString().trim();
                            String auth = entry.get(AUTH).toString().trim();
                            String service = entry.get(SERVICE).toString().trim();
                            if (util.validServiceName(service) && util.validServiceName(auth) &&
                                    service.contains(".") && auth.contains(".")){
                                if (app.contains("*")) {
                                    log.warn("Wild card not allowed in application name in WebSocket entry - {}", e);
                                } else {
                                    if (routingTable.containsKey(app)) {
                                        log.warn("Skipping duplicated WebSocket entry {}", e);
                                    } else {
                                        routingTable.put(app, new WsInfo(app, auth, service));
                                        log.info("WebSocket /ws/api/{}:{} -> {} -> {}", app, "{token}", auth, service);
                                    }
                                }
                            } else {
                                log.warn("Invalid user or authentication service name in WebSocket entry - {}", e);
                            }
                        } else {
                            log.warn("WebSocket entry is missing {}, {} or {} - skipping {}",
                                    SERVICE, APPLICATION, AUTH, e);
                        }

                    } else {
                        log.error("Skipping invalid WebSocket entry {}", e);
                    }
                }
            } else {
                log.error("Invalid WebSocket configuration - input must be a list of maps");
            }
        }

    }

}
