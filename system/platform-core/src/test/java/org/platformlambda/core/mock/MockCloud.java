/*

    Copyright 2018-2022 Accenture Technology

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

package org.platformlambda.core.mock;

import org.platformlambda.core.annotations.CloudConnector;
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.VersionInfo;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentMap;

@CloudConnector(name="mock.cloud")
public class MockCloud implements CloudSetup {
    private static final String CLOUD_CONNECTOR_HEALTH = "cloud.connector.health";
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String ROUTE = ServiceDiscovery.ROUTE;
    private static final String FIND = ServiceDiscovery.FIND;
    private static final String SEARCH = ServiceDiscovery.SEARCH;
    private static final String DOWNLOAD = "download";
    private static final String INFO = "info";
    private static final String HEALTH = "health";
    private static final PostOffice po = PostOffice.getInstance();
    private static final ConcurrentMap<String, ConcurrentMap<String, String>> cloudRoutes = po.getCloudRoutes();
    private static final ConcurrentMap<String, String> cloudOrigins = po.getCloudOrigins();

    @SuppressWarnings("unchecked")
    @Override
    public void initialize() {
        Platform platform = Platform.getInstance();
        // usually a cloud connector will automatically start cloud services
        platform.startCloudServices();

        LambdaFunction query = (headers, body, instance) -> {
            String type = headers.get(TYPE);
            if (INFO.equals(type)) {
                Map<String, Object> result = new HashMap<>();
                result.put("personality", ServerPersonality.getInstance().getType());
                VersionInfo info = Utility.getInstance().getVersionInfo();
                result.put("version", info.getVersion());
                result.put("name", platform.getName());
                result.put("origin", platform.getOrigin());
                return result;

            } else if (DOWNLOAD.equals(type)) {
                Utility util = Utility.getInstance();
                String me = platform.getName()+", v"+util.getVersionInfo().getVersion();
                Map<String, Object> result = new HashMap<>();
                result.put("routes", cloudRoutes);
                result.put("nodes", cloudOrigins);
                result.put("name", me);
                result.put("origin", platform.getOrigin());
                result.put("group", 1);
                return result;

            } else if (FIND.equals(type) && headers.containsKey(ROUTE)) {
                String route = headers.get(ROUTE);
                if (route.equals("*")) {
                    if (body instanceof List) {
                        List<String> list = (List<String>) body;
                        for (String item : list) {
                            if (platform.hasRoute(item)) {
                                return true;
                            }
                        }
                    }
                } else {
                    return platform.hasRoute(route);
                }
            } else if (SEARCH.equals(headers.get(TYPE)) && headers.containsKey(ROUTE)) {
                return Collections.emptyList();
            }
            return false;
        };
        LambdaFunction connector = (headers, body, instance) -> {
            // emulate a cloud connector to handle broadcast
            if ("1".equals(headers.get("broadcast")) && body instanceof byte[]) {
                EventEnvelope event = new EventEnvelope((byte[]) body);
                PostOffice.getInstance().send(event.setBroadcastLevel(0));
            }
            return null;
        };
        LambdaFunction health = (headers, body, instance) -> {
            if (INFO.equals(headers.get(TYPE))) {
                Map<String, Object> result = new HashMap<>();
                result.put("service", "mock.connector");
                result.put("href", "mock://127.0.0.1");
                result.put("topics", "mock.topic");
                return result;
            }
            if (HEALTH.equals(headers.get(TYPE))) {
                return "fine";
            }
            return false;
        };
        /*
         * dummy registry service - in real cloud connector, it is responsible for service registration
         * where cloudRoutes is the routing store.
         */
        LambdaFunction registry = (headers, body, instance) -> true;
        try {
            platform.registerPrivate(PostOffice.CLOUD_CONNECTOR, connector, 1);
            platform.registerPrivate(ServiceDiscovery.SERVICE_QUERY, query, 10);
            platform.registerPrivate(ServiceDiscovery.SERVICE_REGISTRY, registry, 10);
            platform.registerPrivate(CLOUD_CONNECTOR_HEALTH, health, 2);
        } catch (IOException e) {
            // nothing to worry
        }

    }

}
