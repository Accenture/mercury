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

package org.platformlambda.node.services;

import org.platformlambda.core.models.LambdaClient;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.system.EventNodeConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Date;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;


public class ServiceRegistry extends ServiceDiscovery implements LambdaFunction  {
    private static final Logger log = LoggerFactory.getLogger(ServiceRegistry.class);

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        if (ADD.equals(headers.get(TYPE)) && headers.containsKey(ROUTE) && headers.containsKey(ORIGIN)
                && headers.containsKey(EventNodeConnector.PERSONALITY)) {
            String route = headers.get(ROUTE);
            String origin = headers.get(ORIGIN);
            String personality = headers.get(EventNodeConnector.PERSONALITY);
            ConcurrentMap<String, Boolean> originEntries = origins.get(origin);
            if (originEntries == null) {
                originEntries = new ConcurrentHashMap<>();
                origins.put(origin, originEntries);
            }
            originEntries.put(route, true);
            ConcurrentMap<String, Date> routeEntries = routes.get(route);
            if (routeEntries == null) {
                routeEntries = new ConcurrentHashMap<>();
                routes.put(route, routeEntries);
            }
            routeEntries.put(origin, new Date());
            log.info("{} {}.{} registered", route, personality, origin);
            return true;
        }

        if (UNREGISTER.equals(headers.get(TYPE)) && headers.containsKey(ROUTE) && headers.containsKey(ORIGIN)) {
            removeEntry(headers.get(ORIGIN), headers.get(ROUTE));
            return true;
        }

        // remove a disconnected lambda
        if (REMOVE.equals(headers.get(TYPE)) && headers.containsKey(ORIGIN)) {
            String origin = headers.get(ORIGIN);
            List<String> routes = getRoutes(origin);
            if (routes != null) {
                for (String r: routes) {
                    removeEntry(origin, r);
                }
            }
            LambdaClient client = clients.get(origin);
            origins.remove(origin);
            clients.remove(origin);
            if (client != null) {
                log.info("{}.{} disconnected", client.personality, origin);
            }
            return true;
        }

        return false;
    }

    private void removeEntry(String origin, String route) {
        ConcurrentMap<String, Date> entries = routes.get(route);
        if (entries != null) {
            if (entries.containsKey(origin)) {
                entries.remove(origin);
                LambdaClient client = clients.get(origin);
                log.info("{}.{} removed from {}", client.personality, origin, route);
                if (entries.isEmpty()) {
                    routes.remove(route);
                    log.info("{} cleared", route);
                }
            }
        }
    }

}
