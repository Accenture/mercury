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

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.ServiceDiscovery;

import java.util.*;

public class ServiceQuery extends ServiceDiscovery implements LambdaFunction {

    private static final String DOWNLOAD = "download";

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        if (FIND.equals(headers.get(TYPE)) && headers.containsKey(ROUTE)) {
            String route = headers.get(ROUTE);
            if (route.equals("*")) {
                if (body instanceof List) {
                    return exists((List<String>) body);
                } else {
                    return false;
                }
            } else {
                return exists(route);
            }
        } else if (DOWNLOAD.equals(headers.get(TYPE))) {
            Map<String, Object> result = new HashMap<>();
            result.put("routes", routes);
            result.put("nodes", new ArrayList<>(origins.keySet()));
            if (headers.containsKey(ORIGIN)) {
                result.put("this", headers.get(ORIGIN));
            }
            result.put("time", new Date());
            return result;
        } else {
            throw new IllegalArgumentException("Usage: headers (type: find), (route: route_name)");
        }
    }

    private boolean exists(List<String> routes) {
        for (String r: routes) {
            if (!exists(r)) {
                return false;
            }
        }
        return true;
    }

    private boolean exists(String route) {
        if (route.contains(".")) {
            if (Platform.getInstance().hasRoute(route)) {
                return true;
            }
            return routes.containsKey(route);
        } else {
            return originExists(route);
        }
    }

}
